mod inner {
    #[macro_export]
    macro_rules! made {
        () => {
            1
        };
    }
}
pub fn made_one() -> i32 {
    crate::made!()
}
