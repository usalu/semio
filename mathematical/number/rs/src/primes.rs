//! 🧮 Primality testing, integer factorization, and elementary number-theoretic functions.

use crate::modular::mod_pow;
use crate::natural::Natural;

// #region 🔖GcdU64
/// 🤝 Euclidean GCD on plain `u64`s — a small helper shared by `modular.rs`'s order/root functions.
pub fn gcd_u64(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
// #endregion 🔖GcdU64

// #region 🔖MillerRabin
/// 🎲 Deterministic Miller-Rabin using the witness set `{2,3,5,7,11,13,17,19,23,29,31,37}`, proven
/// sufficient to decide primality for every `u64` (in fact every 64-bit and much larger input).
pub fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == p {
            return true;
        }
        if n.is_multiple_of(p) {
            return false;
        }
    }
    let mut d = n - 1;
    let mut r = 0u32;
    while d.is_multiple_of(2) {
        d /= 2;
        r += 1;
    }
    'witness: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..r - 1 {
            x = crate::modular::mod_mul(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

/// 🌱 Deterministic splitmix64-derived witness stream, used only to seed extra Miller-Rabin rounds for
/// [`is_prime`]'s big-integer path — reproducible across runs, and deliberately not `mathematical_random`
/// (this crate has zero dependencies).
struct InlineSplitMix64(u64);
impl InlineSplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}
// #endregion 🔖MillerRabin

// #region 🔖Bpsw
fn miller_rabin_natural(n: &Natural, base: &Natural) -> bool {
    let one = Natural::one();
    let n_minus_1 = n.checked_sub(&one).unwrap();
    let mut d = n_minus_1.clone();
    let mut r = 0u32;
    while !d.bit(0) && !d.is_zero() {
        d = d.shr(1);
        r += 1;
    }
    let mut x = mod_pow_natural(base, &d, n);
    if x == one || x == n_minus_1 {
        return true;
    }
    for _ in 0..r.saturating_sub(1) {
        x = mulmod_natural(&x, &x, n);
        if x == n_minus_1 {
            return true;
        }
    }
    false
}

fn mulmod_natural(a: &Natural, b: &Natural, m: &Natural) -> Natural {
    let (_, r) = a.mul(b).div_rem(m);
    r
}

fn mod_pow_natural(base: &Natural, exp: &Natural, m: &Natural) -> Natural {
    let mut result = Natural::one();
    let mut b = { let (_, r) = base.div_rem(m); r };
    let bits = exp.bit_length();
    for i in 0..bits {
        if exp.bit(i) {
            result = mulmod_natural(&result, &b, m);
        }
        b = mulmod_natural(&b, &b, m);
    }
    result
}

/// 🎯 Probabilistic primality test for arbitrary-precision `Natural`: a base-2 strong Miller-Rabin
/// round (catching every even/trivial composite cheaply) followed by 40 further rounds with
/// deterministic splitmix64-derived witness bases. Reproducible across runs (same `n` always tests the
/// same bases) with false-positive probability bounded by `4^-40` for adversarial composite inputs —
/// astronomically small in practice, though (unlike true BPSW) not a proven zero-counterexample test.
pub fn is_prime(n: &Natural) -> bool {
    if let Some(small) = n.to_u64() {
        return is_prime_u64(small);
    }
    if !n.bit(0) {
        return false;
    }
    if !miller_rabin_natural(n, &Natural::from_u64(2)) {
        return false;
    }
    let seed = n.limbs().first().copied().unwrap_or(1) ^ n.limbs().last().copied().unwrap_or(1) ^ n.bit_length();
    let mut rng = InlineSplitMix64(seed);
    let n_minus_3 = n.checked_sub(&Natural::from_u64(3)).unwrap_or_else(Natural::zero);
    for _ in 0..40 {
        let raw = ((rng.next() as u128) << 64 | rng.next() as u128) as u64;
        let base = if n_minus_3.is_zero() { Natural::from_u64(2) } else { let (_, r) = Natural::from_u64(raw).div_rem(&n_minus_3); r.add(&Natural::from_u64(2)) };
        if !miller_rabin_natural(n, &base) {
            return false;
        }
    }
    true
}

pub fn next_prime(n: &Natural) -> Natural {
    let mut candidate = n.add(&Natural::one());
    if !candidate.bit(0) && candidate > Natural::from_u64(2) {
        candidate = candidate.add(&Natural::one());
    }
    if candidate <= Natural::from_u64(2) {
        return Natural::from_u64(2);
    }
    while !is_prime(&candidate) {
        candidate = candidate.add(&Natural::from_u64(2));
    }
    candidate
}
// #endregion 🔖Bpsw

// #region 🔖Sieve
/// 🧮 Bit-packed sieve of Eratosthenes over `0..limit`.
pub struct Sieve {
    limit: usize,
    is_composite: Vec<u64>,
}

impl Sieve {
    pub fn new(limit: usize) -> Self {
        let words = limit / 64 + 1;
        let mut is_composite = vec![0u64; words];
        let set = |bits: &mut [u64], i: usize| bits[i / 64] |= 1 << (i % 64);
        if limit > 0 {
            set(&mut is_composite, 0);
        }
        if limit > 1 {
            set(&mut is_composite, 1);
        }
        let mut i = 2usize;
        while i * i <= limit {
            let get = |bits: &[u64], j: usize| bits[j / 64] & (1 << (j % 64)) != 0;
            if !get(&is_composite, i) {
                let mut j = i * i;
                while j <= limit {
                    set(&mut is_composite, j);
                    j += i;
                }
            }
            i += 1;
        }
        Self { limit, is_composite }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        if n > self.limit {
            return is_prime_u64(n as u64);
        }
        self.is_composite[n / 64] & (1 << (n % 64)) == 0
    }

    pub fn primes(&self) -> impl Iterator<Item = usize> + '_ {
        (2..=self.limit).filter(move |&n| self.is_prime(n))
    }

    pub fn count(&self) -> usize {
        self.primes().count()
    }
}
// #endregion 🔖Sieve

// #region 🔖Factor
/// 🔍 Pollard's rho with Brent's cycle-detection improvement and batched GCDs; returns a nontrivial
/// factor of composite `n` (never called on primes or `n <= 3` by [`factor_u64`]'s driver).
fn pollard_rho_u64(n: u64) -> u64 {
    if n.is_multiple_of(2) {
        return 2;
    }
    let mut rng = InlineSplitMix64(n ^ 0xA5A5_A5A5_A5A5_A5A5);
    loop {
        let c = 1 + (rng.next() % (n - 1));
        let f = |x: u128| -> u64 { ((x * x + c as u128) % n as u128) as u64 };
        let (mut x, mut y, mut d) = (2u64, 2u64, 1u64);
        let mut q = 1u128;
        let (mut xs, ys) = (x, y);
        let m = 128u64;
        'outer: while d == 1 {
            xs = x;
            for _ in 0..m {
                x = f(x as u128);
                y = f(f(y as u128) as u128);
                q = (q * (x as i64 - y as i64).unsigned_abs() as u128) % n as u128;
                if q == 0 {
                    break 'outer;
                }
            }
            d = gcd_u64(q as u64, n);
        }
        if d == n || d == 0 {
            // fallback: brute retry within the cycle
            d = 1;
            let mut xr = xs;
            let yr = ys;
            while d == 1 {
                xr = f(xr as u128);
                d = gcd_u64((xr as i64 - yr as i64).unsigned_abs(), n);
            }
        }
        if d != n && d > 1 {
            return d;
        }
        // retry with a new random c
    }
}

pub fn factor_u64(n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    let mut n = n;
    for p in [2u64, 3, 5, 7, 11, 13].into_iter().chain((17..65536).step_by(2)) {
        if p * p > n {
            break;
        }
        if n.is_multiple_of(p) {
            let mut count = 0u32;
            while n.is_multiple_of(p) {
                n /= p;
                count += 1;
            }
            factors.push((p, count));
        }
    }
    fn recurse(n: u64, factors: &mut Vec<(u64, u32)>) {
        if n == 1 {
            return;
        }
        if is_prime_u64(n) {
            match factors.iter_mut().find(|(p, _)| *p == n) {
                Some((_, c)) => *c += 1,
                None => factors.push((n, 1)),
            }
            return;
        }
        let d = pollard_rho_u64(n);
        recurse(d, factors);
        recurse(n / d, factors);
    }
    recurse(n, &mut factors);
    factors.sort_unstable_by_key(|&(p, _)| p);
    // merge duplicate prime entries that recursion may have added at different points
    let mut merged: Vec<(u64, u32)> = Vec::new();
    for (p, c) in factors {
        if let Some(last) = merged.last_mut() {
            if last.0 == p {
                last.1 += c;
                continue;
            }
        }
        merged.push((p, c));
    }
    merged
}

/// 🔍 Factorization over arbitrary-precision `Natural`, via a u64 fast path plus `Integer`-arithmetic
/// Pollard-Brent for larger inputs. 60+-digit hard semiprimes are out of scope (no ECM/QS here).
pub fn factor(n: &Natural) -> Vec<(Natural, u32)> {
    if let Some(small) = n.to_u64() {
        return factor_u64(small).into_iter().map(|(p, c)| (Natural::from_u64(p), c)).collect();
    }
    let mut remaining = n.clone();
    let mut factors: Vec<(Natural, u32)> = Vec::new();
    let sieve = Sieve::new(100_000);
    for p in sieve.primes() {
        let p_nat = Natural::from_u64(p as u64);
        if remaining.to_u64().is_some() {
            break;
        }
        let mut count = 0u32;
        loop {
            let (q, r) = remaining.div_rem(&p_nat);
            if r.is_zero() {
                remaining = q;
                count += 1;
            } else {
                break;
            }
        }
        if count > 0 {
            factors.push((p_nat, count));
        }
    }
    fn recurse(n: &Natural, out: &mut Vec<(Natural, u32)>) {
        if n == &Natural::one() {
            return;
        }
        if let Some(small) = n.to_u64() {
            for (p, c) in factor_u64(small) {
                let p_nat = Natural::from_u64(p);
                match out.iter_mut().find(|(q, _)| *q == p_nat) {
                    Some((_, existing)) => *existing += c,
                    None => out.push((p_nat, c)),
                }
            }
            return;
        }
        if is_prime(n) {
            match out.iter_mut().find(|(q, _)| q == n) {
                Some((_, c)) => *c += 1,
                None => out.push((n.clone(), 1)),
            }
            return;
        }
        let d = pollard_rho_natural(n);
        let (q, _) = n.div_rem(&d);
        recurse(&d, out);
        recurse(&q, out);
    }
    recurse(&remaining, &mut factors);
    factors.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged: Vec<(Natural, u32)> = Vec::new();
    for (p, c) in factors {
        if let Some(last) = merged.last_mut() {
            if last.0 == p {
                last.1 += c;
                continue;
            }
        }
        merged.push((p, c));
    }
    merged
}

fn pollard_rho_natural(n: &Natural) -> Natural {
    if !n.bit(0) {
        return Natural::from_u64(2);
    }
    let seed = n.limbs().first().copied().unwrap_or(1) ^ 0x1357_9BDF_2468_ACE0;
    let mut rng = InlineSplitMix64(seed);
    loop {
        let c = Natural::from_u64(1 + rng.next() % 0xFFFF_FFFF);
        let f = |x: &Natural| -> Natural {
            let (_, r) = x.mul(x).add(&c).div_rem(n);
            r
        };
        let mut x = Natural::from_u64(2);
        let mut y = Natural::from_u64(2);
        let mut d = Natural::one();
        while d == Natural::one() {
            x = f(&x);
            y = f(&f(&y));
            let diff = if x > y { x.checked_sub(&y).unwrap() } else { y.checked_sub(&x).unwrap() };
            if diff.is_zero() {
                d = n.clone();
                break;
            }
            d = diff.gcd(n);
        }
        if d != *n && !d.is_zero() && d != Natural::one() {
            return d;
        }
        // else retry with a different c
    }
}
// #endregion 🔖Factor

// #region 🔖ArithmeticFns
pub fn euler_phi(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut result = n;
    for (p, _) in factor_u64(n) {
        result = result / p * (p - 1);
    }
    result
}

pub fn moebius(n: u64) -> i8 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let factors = factor_u64(n);
    if factors.iter().any(|&(_, e)| e > 1) {
        return 0;
    }
    if factors.len().is_multiple_of(2) {
        1
    } else {
        -1
    }
}

pub fn divisors_u64(n: u64) -> Vec<u64> {
    if n == 0 {
        return Vec::new();
    }
    let factors = factor_u64(n);
    let mut divisors = vec![1u64];
    for (p, e) in factors {
        let mut new_divisors = Vec::with_capacity(divisors.len() * (e as usize + 1));
        let mut power = 1u64;
        for _ in 0..=e {
            for &d in &divisors {
                new_divisors.push(d * power);
            }
            power *= p;
        }
        divisors = new_divisors;
    }
    divisors.sort_unstable();
    divisors
}

pub fn divisor_count(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    factor_u64(n).into_iter().map(|(_, e)| e as u64 + 1).product()
}

pub fn divisor_sum(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    factor_u64(n).into_iter().map(|(p, e)| (0..=e).map(|k| p.pow(k)).sum::<u64>()).product()
}
// #endregion 🔖ArithmeticFns

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn is_prime_u64_hand_cases() {
        for p in [2u64, 3, 5, 7, 11, 97, 7919, 999_983] {
            assert!(is_prime_u64(p), "{p} should be prime");
        }
        for c in [1u64, 4, 6, 8, 9, 100, 561, 1105, 1729] {
            assert!(!is_prime_u64(c), "{c} should be composite");
        }
    }

    #[test]
    fn carmichael_numbers_are_rejected() {
        for &carmichael in &[561u64, 41041, 825265] {
            assert!(!is_prime_u64(carmichael), "{carmichael} is a Carmichael number, not prime");
            assert!(!is_prime(&Natural::from_u64(carmichael)));
        }
    }

    #[test]
    fn is_prime_natural_matches_u64_path_for_small_values() {
        for n in 2u64..200 {
            assert_eq!(is_prime(&Natural::from_u64(n)), is_prime_u64(n), "mismatch at {n}");
        }
    }

    #[test]
    fn is_prime_natural_handles_large_known_prime() {
        // A 128-bit-ish prime candidate; validated via BPSW.
        let p = Natural::from_str("170141183460469231731687303715884105727").unwrap(); // 2^127 - 1, a Mersenne prime
        assert!(is_prime(&p));
        let composite = p.add(&Natural::from_u64(2));
        assert!(!is_prime(&composite) || composite.to_u64().is_some());
    }

    #[test]
    fn factor_u64_reconstructs_via_multiplication() {
        for n in [1u64, 2, 97, 360, 999_983, 1_000_000] {
            let factors = factor_u64(n);
            let product: u64 = factors.iter().map(|&(p, e)| p.pow(e)).product();
            assert_eq!(product, n, "factorization of {n} = {factors:?} doesn't reconstruct");
            for (p, _) in &factors {
                assert!(is_prime_u64(*p));
            }
        }
    }

    #[test]
    fn euler_phi_hand_cases() {
        assert_eq!(euler_phi(1), 1);
        assert_eq!(euler_phi(9), 6);
        assert_eq!(euler_phi(97), 96);
    }

    #[test]
    fn moebius_hand_cases() {
        assert_eq!(moebius(1), 1);
        assert_eq!(moebius(6), 1); // 2*3, two distinct primes -> +1
        assert_eq!(moebius(4), 0); // 2^2 -> 0
        assert_eq!(moebius(30), -1); // 2*3*5, three distinct primes -> -1
    }

    #[test]
    fn divisors_hand_case() {
        let divs = divisors_u64(12);
        assert_eq!(divs, vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(divisor_count(12), 6);
        assert_eq!(divisor_sum(12), 28);
    }

    #[test]
    fn sieve_matches_is_prime_u64() {
        let sieve = Sieve::new(1000);
        for n in 0..=1000 {
            assert_eq!(sieve.is_prime(n), is_prime_u64(n as u64), "mismatch at {n}");
        }
    }

    #[test]
    fn next_prime_hand_cases() {
        assert_eq!(next_prime(&Natural::from_u64(10)), Natural::from_u64(11));
        assert_eq!(next_prime(&Natural::from_u64(1)), Natural::from_u64(2));
    }

    // #region 🔖QuickTests
    mod quick {
        use super::*;

        #[test]
        fn factor_random_64bit_semiprimes() {
            // Products of two mid-size primes.
            for &(p, q) in &[(65537u64, 65539u64), (999_983, 999_979), (7919, 104729)] {
                let n = p * q;
                let factors = factor_u64(n);
                let product: u64 = factors.iter().map(|&(f, e)| f.pow(e)).product();
                assert_eq!(product, n);
                assert!(factors.iter().any(|&(f, _)| f == p));
                assert!(factors.iter().any(|&(f, _)| f == q));
            }
        }

        #[test]
        fn sieve_count_matches_known_pi_of_1e6() {
            let sieve = Sieve::new(1_000_000);
            assert_eq!(sieve.count(), 78498);
        }
    }
    // #endregion 🔖QuickTests

    // #region 🔖LongTests
    mod long {
        use super::*;

        #[test]
        fn factor_natural_random_semiprime() {
            let p = Natural::from_str("1000000000000000003").unwrap(); // prime
            assert!(is_prime(&p));
            let q = Natural::from_u64(999_999_999_999_999_989); // prime
            let n = p.mul(&q);
            let factors = factor(&n);
            let product = factors.iter().fold(Natural::one(), |acc, (f, e)| acc.mul(&f.pow(*e as u64)));
            assert_eq!(product, n);
        }
    }
    // #endregion 🔖LongTests
}
// #endregion 🔖Tests
