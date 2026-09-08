mod tests {
    use super::*;

    fn diff_matches(e: &Expr, x: &Expr, antideriv: &Expr) -> bool {
        let d = crate::cas::diff::diff(antideriv, x).unwrap();
        crate::cas::simplify::simplify(&(d - e.clone())).is_zero_literal()
    }

    #[semio_framework_async_macros::async_test]
    async fn integrate_power_rule() {
        let x = Expr::symbol("x");
        let e = Expr::pow(x.clone(), Expr::integer(2));
        let result = integrate(&e, &x).unwrap();
        assert!(diff_matches(&e, &x, &result));
    }

    #[semio_framework_async_macros::async_test]
    async fn integrate_reciprocal_gives_ln() {
        let x = Expr::symbol("x");
        let e = Expr::pow(x.clone(), Expr::integer(-1));
        let result = integrate(&e, &x).unwrap();
        assert_eq!(result, Expr::func(FnKind::Ln, vec![Expr::func(FnKind::Abs, vec![x])]));
    }

    #[semio_framework_async_macros::async_test]
    async fn integrate_sin_and_cos() {
        let x = Expr::symbol("x");
        let sin_result = integrate(&Expr::func(FnKind::Sin, vec![x.clone()]), &x).unwrap();
        assert!(diff_matches(&Expr::func(FnKind::Sin, vec![x.clone()]), &x, &sin_result));
        let cos_result = integrate(&Expr::func(FnKind::Cos, vec![x.clone()]), &x).unwrap();
        assert!(diff_matches(&Expr::func(FnKind::Cos, vec![x.clone()]), &x, &cos_result));
    }

    #[semio_framework_async_macros::async_test]
    async fn integrate_polynomial_sum() {
        let x = Expr::symbol("x");
        let e = Expr::pow(x.clone(), Expr::integer(2)) + Expr::mul(vec![Expr::integer(3), x.clone()]) + Expr::integer(1);
        let result = integrate(&e, &x).unwrap();
        assert!(diff_matches(&e, &x, &result));
    }

    #[semio_framework_async_macros::async_test]
    async fn integrate_simple_partial_fraction() {
        let x = Expr::symbol("x");
        // 1/((x-1)(x+1)) integrates to (1/2)ln|x-1| - (1/2)ln|x+1| (up to grouping)
        let den = (x.clone() - Expr::integer(1)) * (x.clone() + Expr::integer(1));
        let e = Expr::pow(den, Expr::integer(-1));
        let result = integrate(&e, &x).unwrap();
        assert!(diff_matches(&e, &x, &result));
    }

    #[semio_framework_async_macros::async_test]
    async fn integrate_u_substitution() {
        let x = Expr::symbol("x");
        // 2x * cos(x^2) -> sin(x^2)
        let inner = Expr::pow(x.clone(), Expr::integer(2));
        let e = Expr::mul(vec![Expr::integer(2), x.clone(), Expr::func(FnKind::Cos, vec![inner])]);
        let result = integrate(&e, &x).unwrap();
        assert!(diff_matches(&e, &x, &result));
    }

    #[semio_framework_async_macros::async_test]
    async fn integrate_by_parts_x_times_exp() {
        let x = Expr::symbol("x");
        let e = Expr::mul(vec![x.clone(), Expr::func(FnKind::Exp, vec![x.clone()])]);
        let result = integrate(&e, &x).unwrap();
        assert!(diff_matches(&e, &x, &result));
    }

    #[semio_framework_async_macros::async_test]
    async fn integrate_ln_by_parts() {
        let x = Expr::symbol("x");
        let e = Expr::func(FnKind::Ln, vec![x.clone()]);
        let result = integrate(&e, &x).unwrap();
        assert!(diff_matches(&e, &x, &result));
    }

    #[semio_framework_async_macros::async_test]
    async fn integrate_irreducible_quadratic_denominator() {
        let x = Expr::symbol("x");
        // 1/(x^2+1) -> atan(x)
        let e = Expr::pow(Expr::pow(x.clone(), Expr::integer(2)) + Expr::integer(1), Expr::integer(-1));
        let result = integrate(&e, &x).unwrap();
        assert!(diff_matches(&e, &x, &result));
    }

    #[semio_framework_async_macros::async_test]
    async fn definite_integral_of_power() {
        let x = Expr::symbol("x");
        let e = Expr::pow(x.clone(), Expr::integer(2));
        let result = integrate_definite(&e, &x, &Expr::integer(0), &Expr::integer(3)).unwrap();
        assert_eq!(result, Expr::integer(9));
    }

    #[semio_framework_async_macros::async_test]
    async fn residue_at_simple_pole() {
        let x = Expr::symbol("x");
        // 1/(x-2) has residue 1 at x=2
        let e = Expr::pow(x.clone() - Expr::integer(2), Expr::integer(-1));
        assert_eq!(residue(&e, &x, &Expr::integer(2)), Some(Expr::integer(1)));
    }
}
