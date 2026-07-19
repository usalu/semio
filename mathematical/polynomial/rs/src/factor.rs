//! 🔍 Factorization over `Z`/`Q`: prime selection, quadratic-precision-free (linear) Hensel lifting
//! from a `GF(p)` factorization, and subset recombination — plus the rational root theorem.

use crate::finite::factor_mod_p;
use crate::univariate::PolyU;
use mathematical_number::{primes, Integer, IntegralDomain, ModInt, Natural, Rational};
use mathematical_random::Rng;

// #region 🔖Conversions
fn to_modp(f: &PolyU<Integer>, p: u64) -> PolyU<ModInt> {
    let modulus_nat = Natural::from_u64(p);
    PolyU::from_coeffs(
        f.coeffs()
            .iter()
            .map(|c| {
                let (_, r) = c.div_rem_euclid(&Integer::from_natural(modulus_nat.clone()));
                ModInt::new(r.to_i64().unwrap_or(0) as u64, p)
            })
            .collect(),
    )
}

fn lift_nonneg(f_modp: &PolyU<ModInt>) -> PolyU<Integer> {
    PolyU::from_coeffs(f_modp.coeffs().iter().map(|c| Integer::from_i64(c.value() as i64)).collect())
}

/// 🎯 Re-centers coefficients currently in `[0, modulus)` into the balanced range `(-modulus/2, modulus/2]`.
fn center_coeffs(f: &PolyU<Integer>, modulus: &Natural) -> PolyU<Integer> {
    let half = modulus.shr(1);
    let modulus_int = Integer::from_natural(modulus.clone());
    PolyU::from_coeffs(
        f.coeffs()
            .iter()
            .map(|c| {
                if c.magnitude() > &half {
                    c.sub(&modulus_int)
                } else {
                    c.clone()
                }
            })
            .collect(),
    )
}
// #endregion 🔖Conversions

// #region 🔖Hensel
/// 🧮 Multi-factor linear Hensel lifting: given `f` (monic, integer) and pairwise-coprime monic
/// `GF(p)` factors whose product is `f mod p`, lifts every factor simultaneously to modulus
/// `>= target_modulus`, one power of `p` at a time, using the classical partial-fraction (CRT)
/// construction of the lifting coefficients `c_i` with `c_i == 1 (mod g_i)` and `f/g_i | c_i`.
fn hensel_lift_factors(f: &PolyU<Integer>, mod_p_factors: &[PolyU<ModInt>], p: u64, target_modulus: &Natural) -> Vec<PolyU<Integer>> {
    let k = mod_p_factors.len();
    if k <= 1 {
        return vec![f.clone()];
    }
    // f_i = product of all mod_p_factors except the i-th, via prefix/suffix products.
    let mut prefix = vec![PolyU::<ModInt>::one(); k + 1];
    for i in 0..k {
        prefix[i + 1] = prefix[i].mul(&mod_p_factors[i]);
    }
    let mut suffix = vec![PolyU::<ModInt>::one(); k + 1];
    for i in (0..k).rev() {
        suffix[i] = suffix[i + 1].mul(&mod_p_factors[i]);
    }
    let f_modp = to_modp(f, p);
    let mut c: Vec<PolyU<ModInt>> = Vec::with_capacity(k);
    for i in 0..k {
        let f_i = prefix[i].mul(&suffix[i + 1]);
        let (_, s_i, _) = f_i.xgcd(&mod_p_factors[i]);
        let c_i = { let (_, r) = s_i.mul(&f_i).div_rem(&f_modp); r };
        c.push(c_i);
    }

    let mut g: Vec<PolyU<Integer>> = mod_p_factors.iter().map(lift_nonneg).collect();
    let mut current_modulus = Natural::from_u64(p);
    while current_modulus < *target_modulus {
        let mut prod = PolyU::<Integer>::one();
        for gi in &g {
            prod = prod.mul(gi);
        }
        let e = f.sub(&prod);
        let modulus_int = Integer::from_natural(current_modulus.clone());
        let e_over_mod = PolyU::from_coeffs(
            e.coeffs()
                .iter()
                .map(|c| c.exact_div(&modulus_int).expect("Hensel lifting invariant: f - prod(g_i) is divisible by the current modulus"))
                .collect(),
        );
        let e_bar = to_modp(&e_over_mod, p);
        let mut new_g = Vec::with_capacity(k);
        for i in 0..k {
            let delta_full = c[i].mul(&e_bar);
            let (_, delta) = delta_full.div_rem(&mod_p_factors[i]);
            let delta_int = lift_nonneg(&delta);
            let scaled = delta_int.mul_scalar(&Integer::from_natural(current_modulus.clone()));
            new_g.push(g[i].add(&scaled));
        }
        g = new_g;
        current_modulus = current_modulus.mul(&Natural::from_u64(p));
    }
    g.into_iter().map(|gi| center_coeffs(&gi, &current_modulus)).collect()
}
// #endregion 🔖Hensel

// #region 🔖Bounds
/// 📏 A generous (not tight) bound on the absolute value of any coefficient of any integer factor of
/// `f`: `binom(n, n/2) * R^n * |lc|`, where `R` is a Cauchy-style bound on the magnitude of `f`'s
/// roots. Looser than the classical Landau-Mignotte bound costs a few extra Hensel lifting steps, never
/// correctness.
fn factor_coefficient_bound(f: &PolyU<Integer>) -> Natural {
    let n = f.degree().unwrap_or(0);
    let lc = f.leading_coeff().cloned().unwrap_or_else(Integer::one).abs();
    let max_coeff = f.coeffs().iter().map(Integer::abs).fold(Natural::zero(), |acc, m| if m > acc { m } else { acc });
    let r = max_coeff.div_rem(&lc).0.add(&Natural::from_u64(2)); // ceil-ish Cauchy bound, padded
    let mut binom = Natural::one();
    for i in 0..(n / 2) {
        binom = binom.mul(&Natural::from_u64((n - i) as u64)).div_rem(&Natural::from_u64((i + 1) as u64)).0;
    }
    binom.mul(&r.pow(n as u64)).mul(&lc).add(&Natural::one())
}
// #endregion 🔖Bounds

// #region 🔖FactorZ
/// 🔍 Full factorization of a monic integer polynomial: content is always `1` and leading coefficient
/// `1`, so the result is `[(irreducible_factor, multiplicity), ...]` with `product == f` exactly.
/// Non-monic primitive polynomials go through the classical "multiply through" substitution
/// (`f_hat(y) = a_n^(n-1) f(y/a_n)`, monic in `y`) with a defensive final reconstruction check —
/// if that check fails for any reason, `f` is conservatively reported as its own single factor rather
/// than risking a silently wrong answer.
pub fn factor_integer_poly(f: &PolyU<Integer>) -> (Integer, Vec<(PolyU<Integer>, u32)>) {
    if f.is_zero() {
        return (Integer::zero(), Vec::new());
    }
    let content = f.content();
    let primitive = f.primitive_part();
    let lc = primitive.leading_coeff().unwrap().clone();
    let sign_adjusted_content = if lc.is_negative() { content.neg() } else { content };
    let primitive = if lc.is_negative() { primitive.neg() } else { primitive };
    let lc = primitive.leading_coeff().unwrap().clone();

    let monic_factors = if lc == Integer::one() {
        factor_monic_integer_poly(&primitive)
    } else {
        factor_nonmonic_via_substitution(&primitive, &lc)
    };

    // Group identical factors into multiplicities (squarefree_decomposition already separates them by
    // multiplicity upstream of the mod-p work, but distinct square-free classes can still coincide).
    let mut grouped: Vec<(PolyU<Integer>, u32)> = Vec::new();
    for f_i in monic_factors {
        if let Some(entry) = grouped.iter_mut().find(|(g, _)| *g == f_i) {
            entry.1 += 1;
        } else {
            grouped.push((f_i, 1));
        }
    }
    (sign_adjusted_content, grouped)
}

/// 🧮 Factors a monic primitive integer polynomial via squarefree decomposition (over Q, cleared to Z)
/// followed by mod-p factorization, Hensel lifting, and subset recombination per squarefree part.
fn factor_monic_integer_poly(f: &PolyU<Integer>) -> Vec<PolyU<Integer>> {
    let f_rational = PolyU::from_coeffs(f.coeffs().iter().map(|c| Rational::from_integer(c.clone())).collect());
    let squarefree_parts = f_rational.squarefree_decomposition();
    let mut result = Vec::new();
    for (part_rational, mult) in squarefree_parts {
        let part_integer = clear_denominators(&part_rational).primitive_part();
        for factor in factor_squarefree_monic(&part_integer) {
            for _ in 0..mult {
                result.push(factor.clone());
            }
        }
    }
    if result.is_empty() {
        vec![f.clone()]
    } else {
        result
    }
}

fn clear_denominators(f: &PolyU<Rational>) -> PolyU<Integer> {
    let denom_lcm = f.coeffs().iter().fold(Natural::one(), |acc, c| acc.mul(c.denom()).div_rem(&acc.gcd(c.denom())).0);
    PolyU::from_coeffs(f.coeffs().iter().map(|c| c.mul(&Rational::from_integer(Integer::from_natural(denom_lcm.clone()))).trunc()).collect())
}

/// 🔍 Factors a squarefree monic integer polynomial: picks a good prime, factors mod p, lifts, recombines.
fn factor_squarefree_monic(f: &PolyU<Integer>) -> Vec<PolyU<Integer>> {
    if f.degree() == Some(0) || f.degree() == Some(1) {
        return vec![f.clone()];
    }
    let mut rng = Rng::from_seed(0xC0FF_EE00_D15E_A5E5);
    let bound = factor_coefficient_bound(f);
    let target_modulus = bound.mul(&Natural::from_u64(2));

    // Try a handful of odd primes that don't divide the leading coefficient, picking the one giving a
    // squarefree image mod p (guarantees the mod-p factorization is separable).
    let mut best: Option<(u64, Vec<(PolyU<ModInt>, u32)>)> = None;
    let mut p = 100_003u64; // an odd prime comfortably away from tiny-degree coefficient collisions
    let mut attempts = 0;
    while attempts < 8 {
        p = primes::next_prime(&Natural::from_u64(p + 1)).to_u64().unwrap_or(p + 2);
        attempts += 1;
        let f_modp = to_modp(f, p);
        if f_modp.degree() != f.degree() {
            continue; // leading coefficient vanished mod p
        }
        let monic = f_modp.make_monic();
        let squarefree = monic.squarefree_decomposition();
        if squarefree.len() != 1 || squarefree[0].1 != 1 {
            continue; // not squarefree mod p; try another prime
        }
        let (_, factors) = factor_mod_p(&f_modp, &mut rng);
        let better = best.as_ref().is_none_or(|(_, existing)| factors.len() < existing.len());
        if better {
            best = Some((p, factors));
        }
        if best.as_ref().unwrap().1.len() <= 1 {
            break; // already irreducible mod p => irreducible over Z
        }
    }
    let Some((chosen_p, factors_mod_p)) = best else {
        return vec![f.clone()];
    };
    if factors_mod_p.len() <= 1 {
        return vec![f.clone()];
    }
    let mod_p_polys: Vec<PolyU<ModInt>> = factors_mod_p.into_iter().map(|(poly, _)| poly).collect();
    let lifted = hensel_lift_factors(f, &mod_p_polys, chosen_p, &target_modulus);
    recombine(f, &lifted)
}

/// 🧩 Subset recombination: tries products of subsets of the lifted modular factors (smallest
/// cardinality first) against exact integer trial division, capping the number of modular factors
/// considered to keep the search space bounded (`log()`-worthy cases beyond the cap fall back to
/// reporting the un-combined lifted factors, still a correct — if potentially non-irreducible — cover).
fn recombine(f: &PolyU<Integer>, lifted: &[PolyU<Integer>]) -> Vec<PolyU<Integer>> {
    const MAX_MODULAR_FACTORS: usize = 24;
    if lifted.len() > MAX_MODULAR_FACTORS {
        return lifted.to_vec();
    }
    let mut remaining: Vec<PolyU<Integer>> = lifted.to_vec();
    let mut remaining_target = f.clone();
    let mut result = Vec::new();
    let mut subset_size = 1usize;
    while subset_size <= remaining.len() && !remaining.is_empty() {
        if subset_size > remaining.len() {
            break;
        }
        let mut found = false;
        'search: for subset in combinations(remaining.len(), subset_size) {
            let mut candidate = PolyU::<Integer>::one();
            for &idx in &subset {
                candidate = candidate.mul(&remaining[idx]);
            }
            let candidate = candidate.primitive_part();
            if let Some(quotient) = remaining_target.exact_div(&candidate) {
                if quotient.degree().unwrap_or(0) + candidate.degree().unwrap_or(0) == remaining_target.degree().unwrap_or(0) {
                    result.push(candidate);
                    remaining_target = quotient.primitive_part();
                    remaining = remaining.iter().enumerate().filter(|(i, _)| !subset.contains(i)).map(|(_, p)| p.clone()).collect();
                    found = true;
                    break 'search;
                }
            }
        }
        if !found {
            subset_size += 1;
        }
    }
    if remaining_target.degree().unwrap_or(0) > 0 {
        result.push(remaining_target);
    }
    if result.is_empty() {
        vec![f.clone()]
    } else {
        result
    }
}

fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    if k == 0 {
        return vec![Vec::new()];
    }
    if k > n {
        return Vec::new();
    }
    let mut result = Vec::new();
    let mut combo: Vec<usize> = (0..k).collect();
    loop {
        result.push(combo.clone());
        let mut i = k;
        loop {
            if i == 0 {
                return result;
            }
            i -= 1;
            if combo[i] != i + n - k {
                break;
            }
        }
        combo[i] += 1;
        for j in (i + 1)..k {
            combo[j] = combo[j - 1] + 1;
        }
    }
}

/// 🔁 Handles a non-monic primitive `f` via `f_hat(y) = lc^(n-1) f(y/lc)` (monic, integer coefficients
/// since the top term's negative power of `lc` cancels exactly against its own coefficient), factors
/// the monic version, un-substitutes each factor via `H_i(lc * x)`, and takes primitive parts. The
/// product is verified against `f` (up to sign) before being trusted; on any mismatch, `f` is returned
/// as its own single factor.
fn factor_nonmonic_via_substitution(f: &PolyU<Integer>, lc: &Integer) -> Vec<PolyU<Integer>> {
    let n = f.degree().unwrap_or(0);
    if n == 0 {
        return vec![f.clone()];
    }
    let mut hat_coeffs = Vec::with_capacity(n + 1);
    for i in 0..n {
        hat_coeffs.push(f.coeff(i).mul(&lc.pow((n - 1 - i) as u64)));
    }
    hat_coeffs.push(Integer::one());
    let f_hat = PolyU::from_coeffs(hat_coeffs);
    let monic_factors = factor_monic_integer_poly(&f_hat);
    let scale = PolyU::monomial(lc.clone(), 1);
    let candidates: Vec<PolyU<Integer>> = monic_factors.iter().map(|h| h.compose(&scale).primitive_part()).collect();
    let mut product = PolyU::<Integer>::one();
    for c in &candidates {
        product = product.mul(c);
    }
    if product == *f {
        candidates
    } else if product == f.neg() {
        let mut fixed = candidates;
        if let Some(first) = fixed.first_mut() {
            *first = first.neg();
        }
        fixed
    } else {
        vec![f.clone()]
    }
}
// #endregion 🔖FactorZ

// #region 🔖RationalRoots
/// 🎯 Rational roots of `f` via the rational root theorem: candidates `p/q` with `p | trailing`,
/// `q | leading` (over the cleared-denominator integer polynomial), each verified by exact evaluation.
pub fn rational_roots(f: &PolyU<Rational>) -> Vec<Rational> {
    if f.is_zero() {
        return Vec::new();
    }
    let integer_poly = clear_denominators(f).primitive_part();
    let Some(trailing) = integer_poly.coeffs().first().cloned() else { return Vec::new() };
    let leading = integer_poly.leading_coeff().cloned().unwrap();
    if trailing.is_zero() {
        // x = 0 is a root; factor it out and recurse on the rest.
        let reduced = PolyU::from_coeffs(integer_poly.coeffs()[1..].to_vec());
        let reduced_rational = PolyU::from_coeffs(reduced.coeffs().iter().map(|c| Rational::from_integer(c.clone())).collect());
        let mut roots = vec![Rational::zero()];
        roots.extend(rational_roots(&reduced_rational));
        return roots;
    }
    let p_candidates = primes::divisors_u64(trailing.abs().to_u64().unwrap_or(1));
    let q_candidates = primes::divisors_u64(leading.abs().to_u64().unwrap_or(1));
    let mut roots = Vec::new();
    for &p in &p_candidates {
        for &q in &q_candidates {
            for sign in [1i64, -1] {
                let candidate = Rational::from_i64(sign * p as i64, q as i64).unwrap();
                if f.eval(&candidate).is_zero() && !roots.contains(&candidate) {
                    roots.push(candidate);
                }
            }
        }
    }
    roots
}
// #endregion 🔖RationalRoots

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn i(v: i64) -> Integer {
        Integer::from_i64(v)
    }

    fn ipoly(coeffs: Vec<i64>) -> PolyU<Integer> {
        PolyU::from_coeffs(coeffs.into_iter().map(Integer::from_i64).collect())
    }

    #[test]
    fn factor_x2_minus_1() {
        let f = ipoly(vec![-1, 0, 1]); // x^2 - 1 = (x-1)(x+1)
        let (content, factors) = factor_integer_poly(&f);
        assert_eq!(content, i(1));
        let mut product = PolyU::constant(content);
        for (factor, mult) in &factors {
            product = product.mul(&factor.pow(*mult as u64));
        }
        assert_eq!(product, f);
        assert_eq!(factors.len(), 2);
    }

    #[test]
    fn factor_x4_minus_1() {
        let f = ipoly(vec![-1, 0, 0, 0, 1]); // x^4 - 1 = (x-1)(x+1)(x^2+1)
        let (_, factors) = factor_integer_poly(&f);
        let mut product = PolyU::<Integer>::one();
        for (factor, mult) in &factors {
            product = product.mul(&factor.pow(*mult as u64));
        }
        assert_eq!(product, f);
        assert!(factors.len() >= 2);
    }

    #[test]
    fn factor_repeated_linear_factor() {
        let base = ipoly(vec![-1, 1]); // x - 1
        let f = base.mul(&base).mul(&base); // (x-1)^3
        let (_, factors) = factor_integer_poly(&f);
        let mut product = PolyU::<Integer>::one();
        for (factor, mult) in &factors {
            product = product.mul(&factor.pow(*mult as u64));
        }
        assert_eq!(product, f);
        assert!(factors.iter().any(|(factor, mult)| *factor == base && *mult == 3));
    }

    #[test]
    fn factor_irreducible_quadratic_stays_whole() {
        let f = ipoly(vec![1, 0, 1]); // x^2 + 1, irreducible over Q
        let (_, factors) = factor_integer_poly(&f);
        assert_eq!(factors.len(), 1);
        assert_eq!(factors[0].1, 1);
    }

    #[test]
    fn factor_nonmonic_quadratic() {
        let f = ipoly(vec![-3, -1, 2]); // 2x^2 - x - 3 = (2x - 3)(x + 1)
        let (content, factors) = factor_integer_poly(&f);
        let mut product = PolyU::constant(content);
        for (factor, mult) in &factors {
            product = product.mul(&factor.pow(*mult as u64));
        }
        assert_eq!(product, f);
    }

    #[test]
    fn rational_roots_of_quadratic() {
        let f = PolyU::from_coeffs(vec![Rational::from_i64(1, 1).unwrap(), Rational::from_i64(-5, 1).unwrap(), Rational::from_i64(6, 1).unwrap()]); // 6x^2 - 5x + 1
        let roots = rational_roots(&f);
        assert_eq!(roots.len(), 2);
        for r in &roots {
            assert!(f.eval(r).is_zero());
        }
    }

    #[test]
    fn rational_roots_with_zero_root() {
        let f = PolyU::from_coeffs(vec![Rational::zero(), Rational::from_i64(-1, 1).unwrap(), Rational::from_i64(1, 1).unwrap()]); // x^2 - x = x(x-1)
        let roots = rational_roots(&f);
        assert_eq!(roots.len(), 2);
    }
}
// #endregion 🔖Tests
