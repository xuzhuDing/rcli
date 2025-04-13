fn main() {
    let x = 1;
    let y = 2;
    let z = add(x, y);
    print!("Hello, world!{}", z);
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
