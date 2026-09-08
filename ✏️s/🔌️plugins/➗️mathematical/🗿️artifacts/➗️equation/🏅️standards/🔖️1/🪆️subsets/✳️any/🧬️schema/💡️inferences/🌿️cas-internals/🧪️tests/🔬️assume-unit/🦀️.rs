mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn close_propagates_positive_to_real_nonnegative_nonzero() {
        let closed = AssumeSet::POSITIVE.close();
        assert!(closed.contains(AssumeSet::REAL));
        assert!(closed.contains(AssumeSet::NONNEGATIVE));
        assert!(closed.contains(AssumeSet::NONZERO));
        assert!(closed.contains(AssumeSet::COMPLEX));
    }

    #[semio_framework_async_macros::async_test]
    async fn close_propagates_even_to_integer_rational_real() {
        let closed = AssumeSet::EVEN.close();
        assert!(closed.contains(AssumeSet::INTEGER));
        assert!(closed.contains(AssumeSet::RATIONAL));
        assert!(closed.contains(AssumeSet::REAL));
    }

    #[semio_framework_async_macros::async_test]
    #[should_panic(expected = "contradictory POSITIVE and NEGATIVE")]
    async fn close_rejects_positive_and_negative() {
        (AssumeSet::POSITIVE | AssumeSet::NEGATIVE).close();
    }

    #[semio_framework_async_macros::async_test]
    #[should_panic(expected = "contradictory EVEN and ODD")]
    async fn close_rejects_even_and_odd() {
        (AssumeSet::EVEN | AssumeSet::ODD).close();
    }

    #[semio_framework_async_macros::async_test]
    async fn bound_for_deduces_sign_from_assumption_store() {
        use number::Rational;
        let mut assumptions = Assumptions::new();
        assumptions.assume_bound("x", RelationalOperator::Gt, Rational::from_i64(2, 1).unwrap());
        assert_eq!(assumptions.bound_for("x"), Some(true));
        assert_eq!(assumptions.bound_for("y"), None);
    }
}
