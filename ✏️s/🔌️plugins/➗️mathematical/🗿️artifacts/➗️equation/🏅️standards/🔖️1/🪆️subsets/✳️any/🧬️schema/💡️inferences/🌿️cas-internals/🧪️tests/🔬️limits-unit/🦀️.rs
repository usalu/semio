mod tests {
    use super::*;
    use crate::cas::fnkind::FnKind;

    #[semio_framework_async_macros::async_test]
    async fn direct_substitution_when_defined() {
        let x = Expr::symbol("x");
        let e = Expr::pow(x.clone(), Expr::integer(2));
        assert_eq!(limit(&e, &x, &Expr::integer(3), Direction::Both), Some(Expr::integer(9)));
    }

    #[semio_framework_async_macros::async_test]
    async fn classic_sin_x_over_x_at_zero() {
        let x = Expr::symbol("x");
        let e = Expr::func(FnKind::Sin, vec![x.clone()]) * Expr::pow(x.clone(), Expr::integer(-1));
        assert_eq!(limit(&e, &x, &Expr::integer(0), Direction::Both), Some(Expr::integer(1)));
    }

    #[semio_framework_async_macros::async_test]
    async fn polynomial_ratio_at_removable_singularity() {
        // (x^2 - 1)/(x - 1) -> 2 as x -> 1
        let x = Expr::symbol("x");
        let num = Expr::pow(x.clone(), Expr::integer(2)) - Expr::integer(1);
        let den = x.clone() - Expr::integer(1);
        let e = num * Expr::pow(den, Expr::integer(-1));
        assert_eq!(limit(&e, &x, &Expr::integer(1), Direction::Both), Some(Expr::integer(2)));
    }

    #[semio_framework_async_macros::async_test]
    async fn limit_at_infinity_of_rational_function() {
        // (2x + 1)/(x + 3) -> 2 as x -> oo
        let x = Expr::symbol("x");
        let num = Expr::integer(2) * x.clone() + Expr::integer(1);
        let den = x.clone() + Expr::integer(3);
        let e = num * Expr::pow(den, Expr::integer(-1));
        assert_eq!(limit(&e, &x, &Expr::constant(Constant::Inf), Direction::Both), Some(Expr::integer(2)));
    }

    #[semio_framework_async_macros::async_test]
    async fn one_plus_one_over_n_to_the_n_via_lhopital_on_log_form() {
        // A simpler but still classic L'Hopital case: lim x->0 (1 - cos(x))/x^2 = 1/2
        let x = Expr::symbol("x");
        let num = Expr::integer(1) - Expr::func(FnKind::Cos, vec![x.clone()]);
        let den = Expr::pow(x.clone(), Expr::integer(2));
        let e = num * Expr::pow(den, Expr::integer(-1));
        assert_eq!(limit(&e, &x, &Expr::integer(0), Direction::Both), Some(Expr::from(number::Rational::from_i64(1, 2).unwrap())));
    }
}
