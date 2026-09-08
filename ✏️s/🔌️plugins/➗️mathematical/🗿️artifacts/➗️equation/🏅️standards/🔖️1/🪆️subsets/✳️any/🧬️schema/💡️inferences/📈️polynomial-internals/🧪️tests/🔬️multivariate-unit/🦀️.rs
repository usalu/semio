mod tests {
    use super::*;
    use number::Rational;

    fn r(n: i64) -> Rational {
        Rational::from_i64(n, 1).unwrap()
    }

    fn mono(exps: Vec<u32>) -> Monomial {
        Monomial::new(exps)
    }

    #[semio_framework_async_macros::async_test]
    async fn monomial_ordering_lex() {
        let a = mono(vec![2, 0]);
        let b = mono(vec![1, 5]);
        assert_eq!(a.cmp_by(&b, MonomialOrder::Lex), std::cmp::Ordering::Greater);
    }

    #[semio_framework_async_macros::async_test]
    async fn monomial_ordering_grlex_uses_total_degree_first() {
        let a = mono(vec![1, 0]); // degree 1
        let b = mono(vec![0, 2]); // degree 2
        assert_eq!(a.cmp_by(&b, MonomialOrder::GrLex), std::cmp::Ordering::Less);
    }

    #[semio_framework_async_macros::async_test]
    async fn try_div_and_lcm() {
        let a = mono(vec![2, 3]);
        let b = mono(vec![1, 1]);
        assert_eq!(a.try_div(&b), Some(mono(vec![1, 2])));
        assert_eq!(mono(vec![3, 0]).try_div(&mono(vec![0, 1])), None);
        assert_eq!(a.lcm(&b), mono(vec![2, 3]));
    }

    #[semio_framework_async_macros::async_test]
    async fn ring_ops_hand_case() {
        // f = x + y, g = x - y ; f*g = x^2 - y^2
        let x = PolyM::<Rational>::var(0, 2, MonomialOrder::Lex);
        let y = PolyM::<Rational>::var(1, 2, MonomialOrder::Lex);
        let f = x.add(&y);
        let g = x.sub(&y);
        let prod = f.mul(&g);
        let expected = x.mul(&x).sub(&y.mul(&y));
        assert_eq!(prod, expected);
    }

    #[semio_framework_async_macros::async_test]
    async fn eval_hand_case() {
        let x = PolyM::<Rational>::var(0, 2, MonomialOrder::Lex);
        let y = PolyM::<Rational>::var(1, 2, MonomialOrder::Lex);
        let f = x.mul(&x).add(&y); // x^2 + y
        assert_eq!(f.eval(&[r(3), r(2)]), r(11));
    }

    #[semio_framework_async_macros::async_test]
    async fn groebner_basis_of_line_intersection() {
        // {x^2 + y^2 - 1, x - y} over Q: eliminating gives a univariate relation in y.
        let x = PolyM::<Rational>::var(0, 2, MonomialOrder::Lex);
        let y = PolyM::<Rational>::var(1, 2, MonomialOrder::Lex);
        let one = PolyM::<Rational>::constant(r(1), 2, MonomialOrder::Lex);
        let f1 = x.mul(&x).add(&y.mul(&y)).sub(&one);
        let f2 = x.sub(&y);
        let gb = PolyM::groebner_basis(&[f1, f2]);
        assert!(!gb.is_empty());
        // Every original generator must reduce to zero against the basis (membership check).
        for g in [x.mul(&x).add(&y.mul(&y)).sub(&one), x.sub(&y)] {
            let (_, rem) = g.reduce(&gb);
            assert!(rem.is_zero());
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn partial_derivative_hand_case() {
        let x = PolyM::<Rational>::var(0, 2, MonomialOrder::Lex);
        let y = PolyM::<Rational>::var(1, 2, MonomialOrder::Lex);
        let f = x.mul(&x).mul(&y); // x^2 y
        let df_dx = f.partial_derivative(0); // 2xy
        assert_eq!(df_dx, x.mul(&y).mul_scalar(&r(2)));
    }
}
