mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn diff_of_constant_is_zero() {
        assert_eq!(diff(&Expr::integer(5), &Expr::symbol("x")), Some(Expr::integer(0)));
    }

    #[semio_framework_async_macros::async_test]
    async fn diff_of_x_is_one() {
        let x = Expr::symbol("x");
        assert_eq!(diff(&x, &x), Some(Expr::integer(1)));
    }

    #[semio_framework_async_macros::async_test]
    async fn diff_of_other_symbol_is_zero() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        assert_eq!(diff(&y, &x), Some(Expr::integer(0)));
    }

    #[semio_framework_async_macros::async_test]
    async fn power_rule() {
        let x = Expr::symbol("x");
        let e = Expr::pow(x.clone(), Expr::integer(3));
        let expected = Expr::mul(vec![Expr::integer(3), Expr::pow(x, Expr::integer(2))]);
        assert_eq!(diff(&e, &Expr::symbol("x")), Some(expected));
    }

    #[semio_framework_async_macros::async_test]
    async fn product_rule() {
        let x = Expr::symbol("x");
        let e = Expr::mul(vec![x.clone(), Expr::func(FnKind::Sin, vec![x.clone()])]);
        let expected = Expr::add(vec![Expr::func(FnKind::Sin, vec![x.clone()]), Expr::mul(vec![x.clone(), Expr::func(FnKind::Cos, vec![x])])]);
        assert_eq!(diff(&e, &Expr::symbol("x")), Some(expected));
    }

    #[semio_framework_async_macros::async_test]
    async fn chain_rule_sin_of_square() {
        let x = Expr::symbol("x");
        let e = Expr::func(FnKind::Sin, vec![Expr::pow(x.clone(), Expr::integer(2))]);
        let expected = Expr::mul(vec![Expr::integer(2), x.clone(), Expr::func(FnKind::Cos, vec![Expr::pow(x, Expr::integer(2))])]);
        assert_eq!(diff(&e, &Expr::symbol("x")), Some(expected));
    }

    #[semio_framework_async_macros::async_test]
    async fn exp_of_x_is_itself() {
        let x = Expr::symbol("x");
        assert_eq!(diff(&Expr::func(FnKind::Exp, vec![x.clone()]), &x), Some(Expr::func(FnKind::Exp, vec![x])));
    }

    #[semio_framework_async_macros::async_test]
    async fn ln_derivative() {
        let x = Expr::symbol("x");
        assert_eq!(diff(&Expr::func(FnKind::Ln, vec![x.clone()]), &x), Some(Expr::pow(x, Expr::integer(-1))));
    }

    #[semio_framework_async_macros::async_test]
    async fn general_power_logarithmic_differentiation() {
        // d/dx x^x = x^x * (ln(x) + 1)
        let x = Expr::symbol("x");
        let e = Expr::pow(x.clone(), x.clone());
        let result = diff(&e, &x).unwrap();
        let expected = Expr::mul(vec![Expr::pow(x.clone(), x.clone()), Expr::add(vec![Expr::func(FnKind::Ln, vec![x]), Expr::integer(1)])]);
        assert_eq!(result, expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn unknown_function_derivative_is_none() {
        let x = Expr::symbol("x");
        let e = Expr::func(FnKind::Zeta, vec![x.clone()]);
        assert_eq!(diff(&e, &x), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn bessel_j_recurrence_derivative() {
        let x = Expr::symbol("x");
        let n = Expr::integer(2);
        let e = Expr::func(FnKind::BesselJ, vec![n, x.clone()]);
        let expected = Expr::mul(vec![
            Expr::from(Rational::from_i64(1, 2).unwrap()),
            Expr::add(vec![Expr::func(FnKind::BesselJ, vec![Expr::integer(1), x.clone()]), Expr::mul(vec![Expr::integer(-1), Expr::func(FnKind::BesselJ, vec![Expr::integer(3), x.clone()])])]),
        ]);
        assert_eq!(diff(&e, &x), Some(expected));
    }

    #[semio_framework_async_macros::async_test]
    async fn gradient_computes_all_partials() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let e = Expr::mul(vec![x.clone(), y.clone()]);
        let grad = gradient(&e, &[x.clone(), y.clone()]).unwrap();
        assert_eq!(grad[0], y);
        assert_eq!(grad[1], x);
    }
}
