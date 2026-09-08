mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn trig_canon_rewrites_tan_to_sin_over_cos() {
        let x = Expr::symbol("x");
        let e = Expr::func(FnKind::Tan, vec![x.clone()]);
        let result = trig_canon(&e);
        let expected = Expr::mul(vec![Expr::func(FnKind::Sin, vec![x.clone()]), Expr::pow(Expr::func(FnKind::Cos, vec![x]), Expr::integer(-1))]);
        assert_eq!(result, expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn trig_canon_applies_pythagorean_identity() {
        let x = Expr::symbol("x");
        let e = Expr::add(vec![Expr::pow(Expr::func(FnKind::Sin, vec![x.clone()]), Expr::integer(2)), Expr::pow(Expr::func(FnKind::Cos, vec![x]), Expr::integer(2))]);
        assert_eq!(trig_canon(&e), Expr::integer(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn trig_canon_pythagorean_with_extra_terms() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let e = Expr::add(vec![Expr::pow(Expr::func(FnKind::Sin, vec![x.clone()]), Expr::integer(2)), Expr::pow(Expr::func(FnKind::Cos, vec![x]), Expr::integer(2)), y.clone()]);
        assert_eq!(trig_canon(&e), Expr::add(vec![Expr::integer(1), y]));
    }

    #[semio_framework_async_macros::async_test]
    async fn expand_trig_sin_of_sum() {
        let a = Expr::symbol("a");
        let b = Expr::symbol("b");
        let e = Expr::func(FnKind::Sin, vec![Expr::add(vec![a.clone(), b.clone()])]);
        let expected = Expr::add(vec![Expr::mul(vec![Expr::func(FnKind::Sin, vec![a.clone()]), Expr::func(FnKind::Cos, vec![b.clone()])]), Expr::mul(vec![Expr::func(FnKind::Cos, vec![a]), Expr::func(FnKind::Sin, vec![b])])]);
        assert_eq!(expand_trig(&e), expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn expand_log_of_product_and_power() {
        let a = Expr::symbol("a");
        let b = Expr::symbol("b");
        let e = Expr::func(FnKind::Ln, vec![Expr::mul(vec![Expr::pow(a.clone(), Expr::integer(2)), b.clone()])]);
        let expected = Expr::add(vec![Expr::mul(vec![Expr::integer(2), Expr::func(FnKind::Ln, vec![a])]), Expr::func(FnKind::Ln, vec![b])]);
        assert_eq!(expand_log(&e), expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn logcombine_merges_positive_logs() {
        let a = Expr::symbol_with("a", crate::cas::assume::AssumeSet::POSITIVE);
        let b = Expr::symbol_with("b", crate::cas::assume::AssumeSet::POSITIVE);
        let e = Expr::add(vec![Expr::func(FnKind::Ln, vec![a.clone()]), Expr::func(FnKind::Ln, vec![b.clone()])]);
        let combined = logcombine(&e);
        assert_eq!(combined, Expr::func(FnKind::Ln, vec![Expr::mul(vec![a, b])]));
    }

    #[semio_framework_async_macros::async_test]
    async fn logcombine_skips_unknown_sign_arguments() {
        let a = Expr::symbol("a");
        let b = Expr::symbol("b");
        let e = Expr::add(vec![Expr::func(FnKind::Ln, vec![a]), Expr::func(FnKind::Ln, vec![b])]);
        assert_eq!(logcombine(&e), e);
    }

    #[semio_framework_async_macros::async_test]
    async fn powsimp_combines_same_exponent_factors() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let e = Expr::mul(vec![Expr::pow(x.clone(), Expr::integer(3)), Expr::pow(y.clone(), Expr::integer(3))]);
        let expected = Expr::pow(Expr::mul(vec![x, y]), Expr::integer(3));
        assert_eq!(powsimp(&e), expected);
    }
}
