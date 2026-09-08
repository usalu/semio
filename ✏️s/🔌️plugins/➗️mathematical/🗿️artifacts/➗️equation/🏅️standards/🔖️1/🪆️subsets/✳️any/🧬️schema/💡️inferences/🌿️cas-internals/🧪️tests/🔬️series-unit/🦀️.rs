mod tests {
    use super::*;
    use crate::cas::fnkind::FnKind;

    #[semio_framework_async_macros::async_test]
    async fn taylor_series_of_exp_matches_known_coefficients() {
        let x = Expr::symbol("x");
        let e = Expr::func(FnKind::Exp, vec![x.clone()]);
        let s = taylor_series(&e, &x, &Expr::integer(0), 4).unwrap();
        // exp(x) = 1 + x + x^2/2 + x^3/6 + x^4/24
        assert_eq!(s.coeffs[0], Expr::integer(1));
        assert_eq!(s.coeffs[1], Expr::integer(1));
        assert_eq!(s.coeffs[2], Expr::from(number::Rational::from_i64(1, 2).unwrap()));
        assert_eq!(s.coeffs[3], Expr::from(number::Rational::from_i64(1, 6).unwrap()));
    }

    #[semio_framework_async_macros::async_test]
    async fn taylor_series_of_sin_around_zero_has_no_even_terms() {
        let x = Expr::symbol("x");
        let e = Expr::func(FnKind::Sin, vec![x.clone()]);
        let s = taylor_series(&e, &x, &Expr::integer(0), 4).unwrap();
        assert_eq!(s.coeffs[0], Expr::integer(0));
        assert_eq!(s.coeffs[1], Expr::integer(1));
        assert_eq!(s.coeffs[2], Expr::integer(0));
    }

    #[semio_framework_async_macros::async_test]
    async fn taylor_series_fails_at_a_pole() {
        let x = Expr::symbol("x");
        let e = Expr::pow(x.clone(), Expr::integer(-1));
        assert!(taylor_series(&e, &x, &Expr::integer(0), 2).is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn leading_term_skips_zero_coefficients() {
        let x = Expr::symbol("x");
        let e = Expr::func(FnKind::Sin, vec![x.clone()]);
        let s = taylor_series(&e, &x, &Expr::integer(0), 3).unwrap();
        let (order, coeff) = leading_term(&s).unwrap();
        assert_eq!(order, 1);
        assert_eq!(coeff, Expr::integer(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn series_to_expr_round_trips_a_polynomial() {
        let x = Expr::symbol("x");
        let s = Series { x: x.clone(), at: Expr::integer(0), coeffs: vec![Expr::integer(1), Expr::integer(2), Expr::integer(3)] };
        let e = series_to_expr(&s);
        let expected = Expr::add(vec![Expr::integer(1), Expr::mul(vec![Expr::integer(2), x.clone()]), Expr::mul(vec![Expr::integer(3), Expr::pow(x, Expr::integer(2))])]);
        assert_eq!(e, expected);
    }
}
