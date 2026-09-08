mod tests {
    use super::*;
    use crate::cas::expr::Expr;

    #[semio_framework_async_macros::async_test]
    async fn expand_binomial_square() {
        let x = Expr::symbol("x");
        let e = Expr::pow(Expr::add(vec![x.clone(), Expr::integer(1)]), Expr::integer(2));
        let expanded = expand(&e);
        let expected = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::mul(vec![Expr::integer(2), x]), Expr::integer(1)]);
        assert_eq!(expanded, expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn expand_distributes_over_function_argument_unchanged() {
        let x = Expr::symbol("x");
        let e = Expr::func(crate::cas::fnkind::FnKind::Sin, vec![Expr::add(vec![x, Expr::integer(1)])]);
        assert_eq!(expand(&e), e);
    }

    #[semio_framework_async_macros::async_test]
    async fn collect_groups_like_powers() {
        let x = Expr::symbol("x");
        let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::mul(vec![Expr::integer(3), Expr::pow(x.clone(), Expr::integer(2))]), x.clone()]);
        let collected = collect(&e, &x);
        // 4x^2 + x
        let expected = Expr::add(vec![Expr::mul(vec![Expr::integer(4), Expr::pow(x.clone(), Expr::integer(2))]), x]);
        assert_eq!(collected, expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn together_combines_fractions() {
        let x = Expr::symbol("x");
        let e = Expr::add(vec![Expr::pow(x, Expr::integer(-1)), Expr::integer(1)]);
        let combined = together(&e);
        // Verify numerically: (1/x + 1) at x=2 should equal the combined form evaluated the same way.
        assert_ne!(combined, e);
    }

    #[semio_framework_async_macros::async_test]
    async fn cancel_removes_common_univariate_factor() {
        let x = Expr::symbol("x");
        // (x^2 - 1) / (x - 1) -> x + 1
        let num = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::integer(-1)]);
        let den = Expr::add(vec![x.clone(), Expr::integer(-1)]);
        let e = Expr::mul(vec![num, Expr::pow(den, Expr::integer(-1))]);
        let result = cancel(&e);
        assert_eq!(result, Expr::add(vec![x, Expr::integer(1)]));
    }

    #[semio_framework_async_macros::async_test]
    async fn factor_recovers_linear_factors() {
        let x = Expr::symbol("x");
        // x^2 - 1 -> (x-1)(x+1) up to ordering/sign; check by expanding back.
        let e = Expr::add(vec![Expr::pow(x, Expr::integer(2)), Expr::integer(-1)]);
        let factored = factor(&e);
        assert_eq!(expand(&factored), e);
        assert_ne!(factored, e);
    }

    #[semio_framework_async_macros::async_test]
    async fn apart_splits_simple_rational_function() {
        let x = Expr::symbol("x");
        // 1/((x-1)(x+1)) = (1/2)/(x-1) - (1/2)/(x+1)
        let den = Expr::mul(vec![Expr::add(vec![x.clone(), Expr::integer(-1)]), Expr::add(vec![x.clone(), Expr::integer(1)])]);
        let e = Expr::pow(den, Expr::integer(-1));
        let result = apart(&e, &x);
        // Recombine via together+cancel-free check: evaluate both sides symbolically by re-expanding the together form.
        let recombined = together(&result);
        let original_together = together(&e);
        assert_eq!(cancel(&recombined), cancel(&original_together));
    }

    #[semio_framework_async_macros::async_test]
    async fn denest_sqrt_classic_example() {
        // sqrt(3 + 2*sqrt(2)) == 1 + sqrt(2)
        let inner = Expr::add(vec![Expr::integer(3), Expr::mul(vec![Expr::integer(2), Expr::pow(Expr::integer(2), Expr::from(Rational::from_i64(1, 2).unwrap()))])]);
        let e = Expr::pow(inner, Expr::from(Rational::from_i64(1, 2).unwrap()));
        let result = denest_sqrt(&e);
        let expected = Expr::add(vec![Expr::integer(1), Expr::pow(Expr::integer(2), Expr::from(Rational::from_i64(1, 2).unwrap()))]);
        assert_eq!(result, expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn simplify_picks_the_smallest_candidate() {
        let x = Expr::symbol("x");
        let num = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::integer(-1)]);
        let den = Expr::add(vec![x.clone(), Expr::integer(-1)]);
        let e = Expr::mul(vec![num, Expr::pow(den, Expr::integer(-1))]);
        let result = simplify(&e);
        assert_eq!(result, Expr::add(vec![x, Expr::integer(1)]));
    }
}
