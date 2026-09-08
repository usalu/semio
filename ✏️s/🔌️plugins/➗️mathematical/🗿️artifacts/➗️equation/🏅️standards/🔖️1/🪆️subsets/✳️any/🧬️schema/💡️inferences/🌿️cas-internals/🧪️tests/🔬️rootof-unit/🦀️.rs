mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn real_roots_of_quadratic_gives_two_rootofs() {
        // x^2 - 2, roots +-sqrt(2)
        let p = PolyU::from_coeffs(vec![Integer::from_i64(-2), Integer::from_i64(0), Integer::from_i64(1)]);
        let roots = real_roots_of(&p);
        assert_eq!(roots.len(), 2);
        let vals: Vec<f64> = roots.iter().map(|r| root_of_to_f64(r).unwrap()).collect();
        assert!(vals.iter().any(|v| (v - std::f64::consts::SQRT_2).abs() < 1e-9));
        assert!(vals.iter().any(|v| (v + std::f64::consts::SQRT_2).abs() < 1e-9));
    }

    #[semio_framework_async_macros::async_test]
    async fn root_of_sign_matches_isolation_interval() {
        let p = PolyU::from_coeffs(vec![Integer::from_i64(-2), Integer::from_i64(0), Integer::from_i64(1)]);
        let roots = real_roots_of(&p);
        let signs: Vec<_> = roots.iter().map(root_of_sign).collect();
        assert!(signs.contains(&Some(std::cmp::Ordering::Less)));
        assert!(signs.contains(&Some(std::cmp::Ordering::Greater)));
    }
}
