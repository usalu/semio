mod tests {
    use super::*;

    fn m(v: i64, p: u64) -> ModInt {
        ModInt::new(v.rem_euclid(p as i64) as u64, p)
    }

    fn poly(coeffs: Vec<i64>, p: u64) -> PolyU<ModInt> {
        PolyU::from_coeffs(coeffs.into_iter().map(|c| m(c, p)).collect())
    }

    #[semio_framework_async_macros::async_test]
    async fn poly_mod_pow_matches_repeated_squaring() {
        let p = 7;
        let base = poly(vec![1, 1], p); // x + 1
        let modulus = poly(vec![-1, 0, 0, 1], p); // x^3 - 1
        let via_fast = poly_mod_pow(&base, 5, &modulus);
        let mut via_slow = PolyU::one();
        for _ in 0..5 {
            via_slow = via_slow.mul(&base).div_rem(&modulus).1;
        }
        assert_eq!(via_fast, via_slow);
    }

    #[semio_framework_async_macros::async_test]
    async fn is_irreducible_hand_cases() {
        let p = 5;
        // x^2 + 1 is irreducible mod 5? -1 is not a QR mod 5 (5 % 4 == 1, so -1 IS a QR actually).
        // Use x^2 + 2, known irreducible mod 5 (2 is a non-residue mod 5).
        let f = poly(vec![2, 0, 1], p);
        assert!(is_irreducible(&f));
        let g = poly(vec![-1, 0, 1], p); // x^2 - 1 = (x-1)(x+1), reducible
        assert!(!is_irreducible(&g));
    }

    #[semio_framework_async_macros::async_test]
    async fn distinct_degree_factor_separates_degrees() {
        let p = 5;
        let deg1 = poly(vec![-1, 1], p); // x - 1
        let deg2 = poly(vec![2, 0, 1], p); // x^2 + 2, irreducible
        let f = deg1.mul(&deg2);
        let groups = distinct_degree_factor(&f);
        assert!(groups.iter().any(|(_, d)| *d == 1));
        assert!(groups.iter().any(|(_, d)| *d == 2));
    }

    #[semio_framework_async_macros::async_test]
    async fn equal_degree_factor_splits_product_of_two_linears() {
        let p = 7;
        let a = poly(vec![-1, 1], p); // x - 1
        let b = poly(vec![-2, 1], p); // x - 2
        let f = a.mul(&b);
        let mut rng = Rng::from_seed(42);
        let factors = equal_degree_factor(&f, 1, &mut rng);
        assert_eq!(factors.len(), 2);
        let product = factors[0].mul(&factors[1]);
        assert_eq!(product, f);
    }

    #[semio_framework_async_macros::async_test]
    async fn factor_mod_p_reconstructs_via_multiplication() {
        let p = 11;
        let a = poly(vec![-1, 1], p); // x - 1
        let b = poly(vec![-3, 1], p); // x - 3
        let f = a.mul(&b).mul(&a); // (x-1)^2 (x-3)
        let mut rng = Rng::from_seed(7);
        let (lc, factors) = factor_mod_p(&f, &mut rng);
        let mut product = PolyU::constant(lc);
        for (factor, mult) in &factors {
            product = product.mul(&factor.pow(*mult as u64));
        }
        assert_eq!(product, f);
    }
}
