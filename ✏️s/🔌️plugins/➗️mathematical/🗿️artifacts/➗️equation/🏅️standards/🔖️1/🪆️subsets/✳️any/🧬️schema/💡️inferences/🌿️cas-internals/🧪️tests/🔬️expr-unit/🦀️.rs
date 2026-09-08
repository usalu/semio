mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn structural_equality_ignores_sharing() {
        let a = Expr::integer(5);
        let b = Expr::integer(5);
        assert_eq!(a, b);
    }

    #[semio_framework_async_macros::async_test]
    async fn hash_is_deterministic() {
        let a = Expr::integer(42);
        let b = Expr::integer(42);
        assert_eq!(a.hash(), b.hash());
    }

    #[semio_framework_async_macros::async_test]
    async fn ord_is_consistent_and_total() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let one = Expr::integer(1);
        assert!(one < x);
        assert!(x < y || y < x);
        assert!(!(x < y && y < x));
    }

    #[semio_framework_async_macros::async_test]
    async fn symbols_with_different_assumptions_are_distinct() {
        let x1 = Expr::symbol("x");
        let x2 = Expr::symbol_with("x", AssumeSet::POSITIVE);
        assert_ne!(x1, x2);
    }
}
