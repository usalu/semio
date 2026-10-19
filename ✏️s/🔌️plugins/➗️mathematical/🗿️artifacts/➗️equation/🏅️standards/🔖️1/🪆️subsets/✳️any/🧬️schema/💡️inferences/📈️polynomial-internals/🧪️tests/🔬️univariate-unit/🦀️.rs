mod tests {
    use super::*;
    use number::Rational;

    fn r(n: i64, d: i64) -> Rational {
        Rational::from_i64(n, d).unwrap()
    }

    fn poly(coeffs: Vec<i64>) -> PolyU<Rational> {
        PolyU::from_coeffs(coeffs.into_iter().map(|c| r(c, 1)).collect())
    }

    #[semio_framework_async_macros::async_test]
    async fn ring_axioms_on_small_polynomials() {
        let a = poly(vec![1, 2, 3]);
        let b = poly(vec![0, 1]);
        let sum = a.add(&b);
        assert_eq!(sum.coeff(1), r(3, 1));
        let prod = a.mul(&b);
        assert_eq!(prod.coeffs(), &[r(0, 1), r(1, 1), r(2, 1), r(3, 1)]);
    }

    #[semio_framework_async_macros::async_test]
    async fn div_rem_identity_holds() {
        let a = poly(vec![-1, 0, 1]);
        let b = poly(vec![-1, 1]);
        let (q, rem) = a.div_rem(&b);
        assert!(rem.is_zero());
        assert_eq!(q, poly(vec![1, 1]));
    }

    #[semio_framework_async_macros::async_test]
    async fn derivative_power_rule() {
        let f = poly(vec![0, 0, 0, 1]);
        assert_eq!(f.derivative(), poly(vec![0, 0, 3]));
    }

    #[semio_framework_async_macros::async_test]
    async fn eval_horner_matches_direct_computation() {
        let f = poly(vec![1, 2, 3]);
        assert_eq!(f.eval(&r(2, 1)), r(1 + 4 + 12, 1));
    }

    #[semio_framework_async_macros::async_test]
    async fn gcd_hand_case() {
        let common = poly(vec![-1, 0, 1]);
        let a = common.mul(&poly(vec![2, 1]));
        let b = common.mul(&poly(vec![-5, 1]));
        let g = a.gcd_monic(&b);
        let g_monic_common = common.make_monic();
        assert_eq!(g, g_monic_common);
    }

    #[semio_framework_async_macros::async_test]
    async fn resultant_of_coprime_linear_factors_is_nonzero() {
        let a = poly(vec![-1, 1]);
        let b = poly(vec![-2, 1]);
        let res = a.resultant(&b);
        assert_ne!(res, r(0, 1));
    }

    #[semio_framework_async_macros::async_test]
    async fn resultant_of_shared_root_is_zero() {
        let a = poly(vec![-1, 0, 1]);
        let b = poly(vec![-1, 1]);
        let res = a.resultant(&b);
        assert_eq!(res, r(0, 1));
    }

    #[semio_framework_async_macros::async_test]
    async fn factor_x2_minus_1_via_squarefree_and_roots() {
        let f = poly(vec![-1, 0, 1]);
        let decomposition = f.squarefree_decomposition();
        assert_eq!(decomposition.len(), 1);
        assert_eq!(decomposition[0].1, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn squarefree_decomposition_of_repeated_factor() {
        let base = poly(vec![-1, 1]);
        let f = base.mul(&base).mul(&base);
        let decomposition = f.squarefree_decomposition();
        assert!(decomposition.iter().any(|(factor, mult)| *factor == base.make_monic() && *mult == 3));
    }

    #[semio_framework_async_macros::async_test]
    async fn interpolate_reconstructs_quadratic() {
        let points = vec![(r(0, 1), r(1, 1)), (r(1, 1), r(6, 1)), (r(2, 1), r(15, 1))];
        let f = PolyU::interpolate(&points).unwrap();
        assert_eq!(f, poly(vec![1, 3, 2]));
    }

    #[semio_framework_async_macros::async_test]
    async fn rational_root_via_eval_hand_case() {
        let f = PolyU::from_coeffs(vec![r(1, 1), r(-5, 1), r(6, 1)]);
        assert_eq!(f.eval(&r(1, 2)), r(0, 1));
        assert_eq!(f.eval(&r(1, 3)), r(0, 1));
    }
}
