mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn detect_gens_finds_symbols_and_functions() {
        let x = Expr::symbol("x");
        let s = Expr::func(crate::cas::fnkind::FnKind::Sin, vec![x.clone()]);
        let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), s.clone()]);
        let gens = detect_gens(&e);
        assert!(gens.contains(&x));
        assert!(gens.contains(&s));
    }

    #[semio_framework_async_macros::async_test]
    async fn as_poly_roundtrips_through_from_poly() {
        let x = Expr::symbol("x");
        let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::mul(vec![Expr::integer(3), x]), Expr::integer(1)]);
        let (poly, map) = as_poly_auto(&e).unwrap();
        let rebuilt = from_poly(&poly, &map);
        assert_eq!(rebuilt, e);
    }

    #[semio_framework_async_macros::async_test]
    async fn as_poly_uni_extracts_univariate_polynomial() {
        let x = Expr::symbol("x");
        let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(3)), Expr::integer(2)]);
        let p = as_poly_uni(&e, &x).unwrap();
        assert_eq!(p.coeff(3), Rational::one());
        assert_eq!(p.coeff(0), Rational::from_i64(2, 1).unwrap());
    }

    #[semio_framework_async_macros::async_test]
    async fn as_poly_uni_fails_for_other_generators() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let e = Expr::add(vec![x.clone(), y]);
        assert!(as_poly_uni(&e, &x).is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn as_ratfunc_auto_recovers_together_form() {
        let x = Expr::symbol("x");
        // 1/x + 1 -> (x + 1)/x  (structurally: num has x-degree-1 term, den has x^1 term)
        let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(-1)), Expr::integer(1)]);
        let (num, den, map) = as_ratfunc_auto(&e).unwrap();
        assert!(poly_uses_var(&den, gen_index(&x, &map).unwrap()));
        assert!(!num.is_zero());
    }

    #[semio_framework_async_macros::async_test]
    async fn factor_poly_u_recombines_to_the_original() {
        // (2x - 1)(x + 3) = 2x^2 + 5x - 3, with a rational (non-integer) leading structure once made monic.
        let f = PolyU::from_coeffs(vec![Rational::from_i64(-3, 1).unwrap(), Rational::from_i64(5, 1).unwrap(), Rational::from_i64(2, 1).unwrap()]);
        let (overall, factors) = factor_poly_u(&f);
        let mut recombined = PolyU::constant(overall);
        for (factor, mult) in &factors {
            recombined = recombined.mul(&factor.pow(*mult as u64));
        }
        assert_eq!(recombined, f);
    }

    #[semio_framework_async_macros::async_test]
    async fn build_ratio_folds_constant_denominator() {
        let x = Expr::symbol("x");
        let (num, _map) = as_poly(&x, std::slice::from_ref(&x)).unwrap();
        let den = PolyM::constant(Rational::from_i64(2, 1).unwrap(), 1, MonomialOrder::Lex);
        let map = PolyMap { gens: vec![x.clone()] };
        let result = build_ratio(&num, &den, &map);
        assert_eq!(result, Expr::mul(vec![Expr::from(Rational::from_i64(1, 2).unwrap()), x]));
    }
}
