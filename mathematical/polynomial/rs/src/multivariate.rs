//! 🕸️ Sparse multivariate polynomials over the `mathematical_number` ring hierarchy: monomial orders,
//! multivariate division, and Buchberger's Groebner-basis algorithm.

use mathematical_number::{Field, Ring};

// #region 🔖MonomialOrder
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MonomialOrder {
    Lex,
    GrLex,
    GrevLex,
}
// #endregion 🔖MonomialOrder

// #region 🔖Monomial
/// 🎛️ Exponent vector; fixed arity (`exps.len()`) per polynomial instance.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Monomial {
    exps: Vec<u32>,
}

impl Monomial {
    pub fn new(exps: Vec<u32>) -> Self {
        Self { exps }
    }

    pub fn var(index: usize, nvars: usize) -> Self {
        let mut exps = vec![0u32; nvars];
        exps[index] = 1;
        Self { exps }
    }

    pub fn one(nvars: usize) -> Self {
        Self { exps: vec![0u32; nvars] }
    }

    pub fn exps(&self) -> &[u32] {
        &self.exps
    }

    pub fn total_degree(&self) -> u32 {
        self.exps.iter().sum()
    }

    pub fn mul(&self, other: &Self) -> Self {
        Self { exps: self.exps.iter().zip(other.exps.iter()).map(|(a, b)| a + b).collect() }
    }

    /// ➗ `Some(self / other)` if `other`'s exponents are all `<= self`'s, else `None`.
    pub fn try_div(&self, other: &Self) -> Option<Self> {
        let mut result = Vec::with_capacity(self.exps.len());
        for (a, b) in self.exps.iter().zip(other.exps.iter()) {
            if b > a {
                return None;
            }
            result.push(a - b);
        }
        Some(Self { exps: result })
    }

    pub fn lcm(&self, other: &Self) -> Self {
        Self { exps: self.exps.iter().zip(other.exps.iter()).map(|(&a, &b)| a.max(b)).collect() }
    }

    pub fn cmp_by(&self, other: &Self, order: MonomialOrder) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match order {
            MonomialOrder::Lex => self.exps.cmp(&other.exps),
            MonomialOrder::GrLex => self.total_degree().cmp(&other.total_degree()).then_with(|| self.exps.cmp(&other.exps)),
            MonomialOrder::GrevLex => {
                match self.total_degree().cmp(&other.total_degree()) {
                    Ordering::Equal => {
                        // Reverse lex on the reversed exponent vector, with comparison sense flipped.
                        for (a, b) in self.exps.iter().rev().zip(other.exps.iter().rev()) {
                            match a.cmp(b) {
                                Ordering::Equal => continue,
                                ord => return ord.reverse(),
                            }
                        }
                        Ordering::Equal
                    }
                    ord => ord,
                }
            }
        }
    }
}
// #endregion 🔖Monomial

// #region 🔖PolyM
/// 🕸️ Sparse multivariate polynomial: terms sorted descending under `order`, no zero coefficients.
#[derive(Clone, Debug)]
pub struct PolyM<C> {
    nvars: usize,
    order: MonomialOrder,
    terms: Vec<(Monomial, C)>,
}

impl<C: Ring> PartialEq for PolyM<C> {
    fn eq(&self, other: &Self) -> bool {
        // Compare canonically regardless of stored order: both term lists are already order-sorted,
        // but two PolyM values with different `order` fields can still represent the same polynomial,
        // so compare as sets via a Lex-sorted clone.
        let a = self.with_order(MonomialOrder::Lex);
        let b = other.with_order(MonomialOrder::Lex);
        a.terms == b.terms && a.nvars == b.nvars
    }
}

impl<C: Ring> PolyM<C> {
    fn normalize(mut terms: Vec<(Monomial, C)>, nvars: usize, order: MonomialOrder) -> Self {
        terms.retain(|(_, c)| !c.is_zero());
        terms.sort_by(|a, b| b.0.cmp_by(&a.0, order));
        Self { nvars, order, terms }
    }

    pub fn zero(nvars: usize, order: MonomialOrder) -> Self {
        Self { nvars, order, terms: Vec::new() }
    }

    pub fn constant(c: C, nvars: usize, order: MonomialOrder) -> Self {
        Self::normalize(vec![(Monomial::one(nvars), c)], nvars, order)
    }

    pub fn var(index: usize, nvars: usize, order: MonomialOrder) -> Self {
        Self::normalize(vec![(Monomial::var(index, nvars), C::one())], nvars, order)
    }

    pub fn from_terms(terms: Vec<(Monomial, C)>, nvars: usize, order: MonomialOrder) -> Self {
        Self::normalize(terms, nvars, order)
    }

    pub fn with_order(&self, order: MonomialOrder) -> Self {
        Self::normalize(self.terms.clone(), self.nvars, order)
    }

    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    pub fn nvars(&self) -> usize {
        self.nvars
    }

    pub fn order(&self) -> MonomialOrder {
        self.order
    }

    pub fn terms(&self) -> &[(Monomial, C)] {
        &self.terms
    }

    pub fn leading_term(&self) -> Option<&(Monomial, C)> {
        self.terms.first()
    }

    pub fn total_degree(&self) -> u32 {
        self.terms.iter().map(|(m, _)| m.total_degree()).max().unwrap_or(0)
    }

    pub fn add(&self, other: &Self) -> Self {
        assert_eq!(self.nvars, other.nvars, "PolyM::add: variable-count mismatch");
        let mut map: std::collections::BTreeMap<Vec<u32>, C> = std::collections::BTreeMap::new();
        for (m, c) in self.terms.iter().chain(other.terms.iter()) {
            map.entry(m.exps.clone()).and_modify(|acc| *acc = acc.add(c)).or_insert_with(|| c.clone());
        }
        let terms = map.into_iter().map(|(exps, c)| (Monomial::new(exps), c)).collect();
        Self::normalize(terms, self.nvars, self.order)
    }

    pub fn neg(&self) -> Self {
        Self { nvars: self.nvars, order: self.order, terms: self.terms.iter().map(|(m, c)| (m.clone(), c.neg())).collect() }
    }

    pub fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    pub fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.nvars, other.nvars, "PolyM::mul: variable-count mismatch");
        let mut map: std::collections::BTreeMap<Vec<u32>, C> = std::collections::BTreeMap::new();
        for (m1, c1) in &self.terms {
            for (m2, c2) in &other.terms {
                let m = m1.mul(m2);
                let c = c1.mul(c2);
                map.entry(m.exps).and_modify(|acc| *acc = acc.add(&c)).or_insert(c);
            }
        }
        let terms = map.into_iter().map(|(exps, c)| (Monomial::new(exps), c)).collect();
        Self::normalize(terms, self.nvars, self.order)
    }

    pub fn pow(&self, exp: u64) -> Self {
        let mut result = Self::constant(C::one(), self.nvars, self.order);
        let mut base = self.clone();
        let mut e = exp;
        while e > 0 {
            if e & 1 == 1 {
                result = result.mul(&base);
            }
            base = base.mul(&base);
            e >>= 1;
        }
        result
    }

    pub fn eval(&self, point: &[C]) -> C {
        self.terms.iter().fold(C::zero(), |acc, (m, c)| {
            let mut term_val = c.clone();
            for (var_idx, &exp) in m.exps.iter().enumerate() {
                if exp > 0 {
                    term_val = term_val.mul(&point[var_idx].pow(exp as u64));
                }
            }
            acc.add(&term_val)
        })
    }

    pub fn mul_scalar(&self, s: &C) -> Self {
        Self::normalize(self.terms.iter().map(|(m, c)| (m.clone(), c.mul(s))).collect(), self.nvars, self.order)
    }

    pub fn partial_derivative(&self, var: usize) -> Self {
        let mut terms = Vec::new();
        for (m, c) in &self.terms {
            let e = m.exps[var];
            if e == 0 {
                continue;
            }
            let mut new_exps = m.exps.clone();
            new_exps[var] = e - 1;
            let new_c = c.mul(&C::from_i64(e as i64));
            terms.push((Monomial::new(new_exps), new_c));
        }
        Self::normalize(terms, self.nvars, self.order)
    }
}
// #endregion 🔖PolyM

// #region 🔖Reduction
impl<C: Field> PolyM<C> {
    /// ➗ Multivariate division of `self` by `divisors`: returns `(quotients, remainder)` such that
    /// `self == sum(q_i * divisors_i) + remainder` and no term of `remainder` is divisible by any
    /// divisor's leading term.
    pub fn reduce(&self, divisors: &[Self]) -> (Vec<Self>, Self) {
        let mut quotients = vec![Self::zero(self.nvars, self.order); divisors.len()];
        let mut remainder = Self::zero(self.nvars, self.order);
        let mut p = self.clone();
        'outer: while !p.is_zero() {
            let (lead_m, lead_c) = p.leading_term().unwrap().clone();
            for (i, d) in divisors.iter().enumerate() {
                let Some((d_lead_m, d_lead_c)) = d.leading_term() else { continue };
                if let Some(quot_m) = lead_m.try_div(d_lead_m) {
                    let quot_c = lead_c.div(d_lead_c).expect("nonzero divisor leading coefficient");
                    let quot_term = Self::from_terms(vec![(quot_m, quot_c)], self.nvars, self.order);
                    quotients[i] = quotients[i].add(&quot_term);
                    p = p.sub(&quot_term.mul(d));
                    continue 'outer;
                }
            }
            let lead_poly = Self::from_terms(vec![(lead_m, lead_c)], self.nvars, self.order);
            remainder = remainder.add(&lead_poly);
            p = p.sub(&lead_poly);
        }
        (quotients, remainder)
    }

    /// 🧮 `S(f,g) = (lcm/LT(f)) * f - (lcm/LT(g)) * g`, where `LT` is the leading term (monomial times
    /// coefficient) — each cofactor's coefficient is the inverse of *its own* polynomial's leading
    /// coefficient, since its job is to cancel that polynomial's own leading term exactly.
    pub fn s_polynomial(&self, other: &Self) -> Self {
        let (lm1, lc1) = self.leading_term().expect("s_polynomial: self must be nonzero").clone();
        let (lm2, lc2) = other.leading_term().expect("s_polynomial: other must be nonzero").clone();
        let lcm = lm1.lcm(&lm2);
        let factor1 = lcm.try_div(&lm1).expect("lcm is divisible by lm1 by construction");
        let factor2 = lcm.try_div(&lm2).expect("lcm is divisible by lm2 by construction");
        let term1 = Self::from_terms(vec![(factor1, lc1.inv().unwrap())], self.nvars, self.order);
        let term2 = Self::from_terms(vec![(factor2, lc2.inv().unwrap())], self.nvars, self.order);
        term1.mul(self).sub(&term2.mul(other))
    }
    // #endregion

    // #region 🔖Groebner
    /// 🧮 Buchberger's algorithm with the coprime-leading-term criterion and pairwise interreduction,
    /// producing the unique reduced monic Groebner basis of the ideal generated by `gens`.
    pub fn groebner_basis(gens: &[Self]) -> Vec<Self> {
        let mut basis: Vec<Self> = gens.iter().filter(|g| !g.is_zero()).map(Self::make_monic_lead).collect();
        let mut pairs: Vec<(usize, usize)> = (0..basis.len()).flat_map(|i| (0..i).map(move |j| (i, j))).collect();
        while let Some((i, j)) = pairs.pop() {
            let (lm_i, _) = basis[i].leading_term().unwrap();
            let (lm_j, _) = basis[j].leading_term().unwrap();
            // Coprimality criterion: if the leading monomials share no variables, the S-polynomial
            // reduces to zero and can be skipped (Buchberger's first criterion).
            let coprime = lm_i.exps().iter().zip(lm_j.exps().iter()).all(|(&a, &b)| a == 0 || b == 0);
            if coprime {
                continue;
            }
            let s = basis[i].s_polynomial(&basis[j]);
            let (_, remainder) = s.reduce(&basis);
            if !remainder.is_zero() {
                let new_poly = remainder.make_monic_lead();
                let new_idx = basis.len();
                for k in 0..new_idx {
                    pairs.push((new_idx, k));
                }
                basis.push(new_poly);
            }
        }
        Self::interreduce(basis)
    }

    fn make_monic_lead(&self) -> Self {
        let Some((_, lc)) = self.leading_term() else {
            return self.clone();
        };
        let inv = lc.inv().expect("nonzero leading coefficient has an inverse in a field");
        Self { nvars: self.nvars, order: self.order, terms: self.terms.iter().map(|(m, c)| (m.clone(), c.mul(&inv))).collect() }
    }

    /// 🧹 Reduces each basis element against the others and removes any that become redundant, giving
    /// the canonical reduced Groebner basis.
    fn interreduce(basis: Vec<Self>) -> Vec<Self> {
        let mut current = basis;
        loop {
            let mut changed = false;
            let mut next = Vec::with_capacity(current.len());
            for i in 0..current.len() {
                let others: Vec<Self> = current.iter().enumerate().filter(|&(j, _)| j != i).map(|(_, p)| p.clone()).collect();
                let (_, remainder) = current[i].reduce(&others);
                if remainder.is_zero() {
                    changed = true;
                    continue;
                }
                if remainder != current[i] {
                    changed = true;
                }
                next.push(remainder.make_monic_lead());
            }
            current = next;
            if !changed {
                break;
            }
        }
        current.sort_by(|a, b| a.leading_term().map(|t| t.0.clone()).cmp(&b.leading_term().map(|t| t.0.clone())));
        current
    }

    /// 🚮 Lex Groebner basis followed by dropping generators that involve any of the first `drop_vars`
    /// variables — the elimination-ideal extraction used by polynomial-system triangularization.
    pub fn eliminate(gens: &[Self], drop_vars: usize) -> Vec<Self> {
        let lex_gens: Vec<Self> = gens.iter().map(|g| g.with_order(MonomialOrder::Lex)).collect();
        let gb = Self::groebner_basis(&lex_gens);
        gb.into_iter().filter(|p| p.terms.iter().all(|(m, _)| m.exps()[..drop_vars].iter().all(|&e| e == 0))).collect()
    }
    // #endregion 🔖Groebner
}
// #endregion 🔖Reduction

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use mathematical_number::Rational;

    fn r(n: i64) -> Rational {
        Rational::from_i64(n, 1).unwrap()
    }

    fn mono(exps: Vec<u32>) -> Monomial {
        Monomial::new(exps)
    }

    #[test]
    fn monomial_ordering_lex() {
        let a = mono(vec![2, 0]);
        let b = mono(vec![1, 5]);
        assert_eq!(a.cmp_by(&b, MonomialOrder::Lex), std::cmp::Ordering::Greater);
    }

    #[test]
    fn monomial_ordering_grlex_uses_total_degree_first() {
        let a = mono(vec![1, 0]); // degree 1
        let b = mono(vec![0, 2]); // degree 2
        assert_eq!(a.cmp_by(&b, MonomialOrder::GrLex), std::cmp::Ordering::Less);
    }

    #[test]
    fn try_div_and_lcm() {
        let a = mono(vec![2, 3]);
        let b = mono(vec![1, 1]);
        assert_eq!(a.try_div(&b), Some(mono(vec![1, 2])));
        assert_eq!(mono(vec![3, 0]).try_div(&mono(vec![0, 1])), None);
        assert_eq!(a.lcm(&b), mono(vec![2, 3]));
    }

    #[test]
    fn ring_ops_hand_case() {
        // f = x + y, g = x - y ; f*g = x^2 - y^2
        let x = PolyM::<Rational>::var(0, 2, MonomialOrder::Lex);
        let y = PolyM::<Rational>::var(1, 2, MonomialOrder::Lex);
        let f = x.add(&y);
        let g = x.sub(&y);
        let prod = f.mul(&g);
        let expected = x.mul(&x).sub(&y.mul(&y));
        assert_eq!(prod, expected);
    }

    #[test]
    fn eval_hand_case() {
        let x = PolyM::<Rational>::var(0, 2, MonomialOrder::Lex);
        let y = PolyM::<Rational>::var(1, 2, MonomialOrder::Lex);
        let f = x.mul(&x).add(&y); // x^2 + y
        assert_eq!(f.eval(&[r(3), r(2)]), r(11));
    }

    #[test]
    fn groebner_basis_of_line_intersection() {
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

    #[test]
    fn partial_derivative_hand_case() {
        let x = PolyM::<Rational>::var(0, 2, MonomialOrder::Lex);
        let y = PolyM::<Rational>::var(1, 2, MonomialOrder::Lex);
        let f = x.mul(&x).mul(&y); // x^2 y
        let df_dx = f.partial_derivative(0); // 2xy
        assert_eq!(df_dx, x.mul(&y).mul_scalar(&r(2)));
    }
}
// #endregion 🔖Tests
