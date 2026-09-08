mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn laplace_of_t_to_the_n() {
        let t = Expr::symbol("t");
        let s = Expr::symbol("s");
        // L{t^2} = 2/s^3
        let e = Expr::pow(t.clone(), Expr::integer(2));
        let result = laplace_transform(&e, &t, &s).unwrap();
        assert_eq!(result, Expr::integer(2) * Expr::pow(s, Expr::integer(-3)));
    }

    #[semio_framework_async_macros::async_test]
    async fn laplace_of_exp() {
        let t = Expr::symbol("t");
        let s = Expr::symbol("s");
        let e = Expr::func(FnKind::Exp, vec![Expr::integer(3) * t.clone()]);
        let result = laplace_transform(&e, &t, &s).unwrap();
        assert_eq!(result, Expr::pow(s - Expr::integer(3), Expr::integer(-1)));
    }

    #[semio_framework_async_macros::async_test]
    async fn laplace_linearity() {
        let t = Expr::symbol("t");
        let s = Expr::symbol("s");
        let e = Expr::integer(2) * t.clone() + Expr::integer(3);
        let result = laplace_transform(&e, &t, &s).unwrap();
        let expected = Expr::integer(2) * Expr::pow(s.clone(), Expr::integer(-2)) + Expr::integer(3) * Expr::pow(s, Expr::integer(-1));
        assert_eq!(result, expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn laplace_and_inverse_round_trip_for_exp() {
        let t = Expr::symbol("t");
        let s = Expr::symbol("s");
        let e = Expr::func(FnKind::Exp, vec![Expr::integer(-2) * t.clone()]);
        let transformed = laplace_transform(&e, &t, &s).unwrap();
        let back = inverse_laplace_transform(&transformed, &s, &t).unwrap();
        assert_eq!(back, e);
    }
}
