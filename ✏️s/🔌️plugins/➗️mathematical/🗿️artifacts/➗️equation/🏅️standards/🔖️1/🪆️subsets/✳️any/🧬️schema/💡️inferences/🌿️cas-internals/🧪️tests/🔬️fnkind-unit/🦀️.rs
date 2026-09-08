mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn arity_hand_cases() {
        assert_eq!(FnKind::Sin.arity(), Some(1));
        assert_eq!(FnKind::BesselJ.arity(), Some(2));
        assert_eq!(FnKind::UserFn("f".into()).arity(), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn parity_hand_cases() {
        assert!(FnKind::Cos.is_even());
        assert!(FnKind::Sin.is_odd());
        assert!(!FnKind::Exp.is_even() && !FnKind::Exp.is_odd());
    }

    #[semio_framework_async_macros::async_test]
    async fn name_hand_cases() {
        assert_eq!(FnKind::Sin.name(), "sin");
        assert_eq!(FnKind::UserFn("myFunc".into()).name(), "myFunc");
    }
}
