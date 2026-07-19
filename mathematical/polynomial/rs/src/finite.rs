//! 🧮 Polynomial arithmetic over `GF(p)` (`PolyU<ModInt>`): modular exponentiation, Rabin's
//! irreducibility test, distinct-degree factorization, and Cantor-Zassenhaus equal-degree splitting —
//! the modular layer that `factor.rs` lifts via Hensel's lemma to factor over `Z`/`Q`.

use crate::univariate::PolyU;
use mathematical_number::ModInt;
use mathematical_random::Rng;

// #region 🔖PolyModPow
pub fn poly_mod_pow(base: &PolyU<ModInt>, exp: u64, modulus: &PolyU<ModInt>) -> PolyU<ModInt> {
    let mut result = PolyU::one();
    let mut b = { let (_, r) = base.div_rem(modulus); r };
    let mut e = exp;
    while e > 0 {
        if e & 1 == 1 {
            let (_, r) = result.mul(&b).div_rem(modulus);
            result = r;
        }
        let (_, r) = b.mul(&b).div_rem(modulus);
        b = r;
        e >>= 1;
    }
    result
}
// #endregion 🔖PolyModPow

// #region 🔖Irreducibility
fn prime_factors_of_degree(n: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    let mut n = n;
    let mut p = 2usize;
    while p * p <= n {
        if n % p == 0 {
            factors.push(p);
            while n % p == 0 {
                n /= p;
            }
        }
        p += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

/// 🎯 Rabin's irreducibility test: `f` (monic, degree `n`) is irreducible over `GF(p)` iff
/// `x^(p^n) == x (mod f)` and `gcd(x^(p^(n/q)) - x, f) == 1` for every prime `q | n`.
pub fn is_irreducible(f: &PolyU<ModInt>) -> bool {
    let Some(n) = f.degree() else { return false };
    if n == 0 {
        return false;
    }
    let p = f.leading_coeff().unwrap().modulus();
    let x = PolyU::x();
    for q in prime_factors_of_degree(n) {
        let h = poly_mod_pow(&x, p.pow((n / q) as u32), f);
        let diff = h.sub(&x);
        let g = diff.gcd_monic(f);
        if g.degree() != Some(0) {
            return false;
        }
    }
    let h = poly_mod_pow(&x, p.pow(n as u32), f);
    h == x
}
// #endregion 🔖Irreducibility

// #region 🔖DistinctDegree
/// ✂️ Splits a squarefree `f` into groups of irreducible factors sharing the same degree:
/// `[(product_of_degree_i_factors, i), ...]`.
pub fn distinct_degree_factor(f: &PolyU<ModInt>) -> Vec<(PolyU<ModInt>, usize)> {
    let p = f.leading_coeff().expect("distinct_degree_factor: f must be nonzero").modulus();
    let mut result = Vec::new();
    let mut f_star = f.make_monic();
    let mut h = PolyU::x();
    let mut i = 0usize;
    while f_star.degree().unwrap_or(0) > 0 {
        i += 1;
        h = poly_mod_pow(&h, p, &f_star);
        let diff = h.sub(&PolyU::x());
        let g = diff.gcd_monic(&f_star);
        if g.degree() != Some(0) {
            result.push((g.clone(), i));
            f_star = f_star.div_rem(&g).0.make_monic();
            h = { let (_, r) = h.div_rem(&f_star); r };
        }
        if 2 * i > f_star.degree().unwrap_or(0) {
            break;
        }
    }
    if f_star.degree().unwrap_or(0) > 0 {
        let deg = f_star.degree().unwrap();
        result.push((f_star, deg));
    }
    result
}
// #endregion 🔖DistinctDegree

// #region 🔖EqualDegree
/// 🎲 Cantor-Zassenhaus equal-degree splitting: `f` is a product of `r` distinct monic irreducibles,
/// each of degree `d`; returns all `r` of them. Requires odd `p` (the driver in `factor.rs` never
/// selects `p == 2` for this reason).
pub fn equal_degree_factor(f: &PolyU<ModInt>, d: usize, rng: &mut Rng) -> Vec<PolyU<ModInt>> {
    let n = f.degree().unwrap_or(0);
    if n == d || n == 0 {
        return vec![f.make_monic()];
    }
    let p = f.leading_coeff().unwrap().modulus();
    loop {
        let deg_a = 1 + (rng.next_range(0, n as u64) as usize);
        let coeffs: Vec<ModInt> = (0..deg_a).map(|_| ModInt::new(rng.next_range(0, p), p)).collect();
        let a = PolyU::from_coeffs(coeffs);
        if a.is_zero() {
            continue;
        }
        let g0 = a.gcd_monic(f);
        let g = if g0.degree() != Some(0) {
            g0
        } else {
            let exp_pow = mod_pow_u64_via_natural(p, d as u64);
            let half = (exp_pow - 1) / 2;
            let b = poly_mod_pow(&a, half, f);
            let b_minus_1 = b.sub(&PolyU::one());
            b_minus_1.gcd_monic(f)
        };
        if g.degree() != Some(0) && g.degree() != Some(n) {
            let mut left = equal_degree_factor(&g, d, rng);
            let cofactor = f.div_rem(&g).0.make_monic();
            let mut right = equal_degree_factor(&cofactor, d, rng);
            left.append(&mut right);
            return left;
        }
    }
}

/// 🔢 `p^d` as a `u64`; degrees stay small in practice (factoring polys of reasonable size), so plain
/// `u64` exponentiation with an overflow-safety fallback via saturating multiplication is sufficient.
fn mod_pow_u64_via_natural(p: u64, d: u64) -> u64 {
    let mut result = 1u64;
    for _ in 0..d {
        result = result.saturating_mul(p);
    }
    result
}
// #endregion 🔖EqualDegree

// #region 🔖FactorModP
/// 🧮 Full factorization of `f` over `GF(p)`: `(leading_coeff, [(irreducible_factor, multiplicity), ...])`.
pub fn factor_mod_p(f: &PolyU<ModInt>, rng: &mut Rng) -> (ModInt, Vec<(PolyU<ModInt>, u32)>) {
    let lc = f.leading_coeff().expect("factor_mod_p: f must be nonzero").clone();
    let monic = f.make_monic();
    let squarefree = monic.squarefree_decomposition();
    let mut result = Vec::new();
    for (part, mult) in squarefree {
        for (irreducible_group, degree) in distinct_degree_factor(&part) {
            for irreducible in equal_degree_factor(&irreducible_group, degree, rng) {
                result.push((irreducible, mult));
            }
        }
    }
    (lc, result)
}
// #endregion 🔖FactorModP

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn m(v: i64, p: u64) -> ModInt {
        ModInt::new(v.rem_euclid(p as i64) as u64, p)
    }

    fn poly(coeffs: Vec<i64>, p: u64) -> PolyU<ModInt> {
        PolyU::from_coeffs(coeffs.into_iter().map(|c| m(c, p)).collect())
    }

    #[test]
    fn poly_mod_pow_matches_repeated_squaring() {
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

    #[test]
    fn is_irreducible_hand_cases() {
        let p = 5;
        // x^2 + 1 is irreducible mod 5? -1 is not a QR mod 5 (5 % 4 == 1, so -1 IS a QR actually).
        // Use x^2 + 2, known irreducible mod 5 (2 is a non-residue mod 5).
        let f = poly(vec![2, 0, 1], p);
        assert!(is_irreducible(&f));
        let g = poly(vec![-1, 0, 1], p); // x^2 - 1 = (x-1)(x+1), reducible
        assert!(!is_irreducible(&g));
    }

    #[test]
    fn distinct_degree_factor_separates_degrees() {
        let p = 5;
        let deg1 = poly(vec![-1, 1], p); // x - 1
        let deg2 = poly(vec![2, 0, 1], p); // x^2 + 2, irreducible
        let f = deg1.mul(&deg2);
        let groups = distinct_degree_factor(&f);
        assert!(groups.iter().any(|(_, d)| *d == 1));
        assert!(groups.iter().any(|(_, d)| *d == 2));
    }

    #[test]
    fn equal_degree_factor_splits_product_of_two_linears() {
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

    #[test]
    fn factor_mod_p_reconstructs_via_multiplication() {
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
// #endregion 🔖Tests
