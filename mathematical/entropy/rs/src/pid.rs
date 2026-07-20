//! 🧩 Williams-Beer Partial Information Decomposition: the two-source `I_min` redundancy
//! decomposition (`pid_two_sources`) and the full 18-node redundancy lattice for exactly three
//! sources (`PidLattice`). Every quantity here is computed by maximum-likelihood plug-in on
//! empirical counts (no bias correction — see [`pid_two_sources`]'s doc for why) and internally
//! in nats, converted to the caller's [`LogBase`] only at the API boundary.

use crate::counts::JointCounts;
use crate::numeric::{checked_state_count, clamp_near_zero, neumaier_sum};
use crate::{EntropyError, LogBase};

// #region 🔖Packing
/// 🧩 Packs several aligned symbol sequences into one joint symbol via mixed-radix encoding. A
/// local copy of `mutual::pack_symbols`'s approach: that helper is private to its own module, so
/// this module keeps its own small copy rather than reaching into `mutual`'s internals.
fn pack_symbols(parts: &[&[u32]], sizes: &[usize]) -> Result<(Vec<u32>, usize), EntropyError> {
    let total = checked_state_count(sizes).ok_or(EntropyError::InvalidConfig {
        field: "sizes",
        reason: "joint alphabet size overflows u128",
    })?;
    if total > u32::MAX as u128 {
        return Err(EntropyError::InvalidConfig { field: "sizes", reason: "joint alphabet size exceeds u32::MAX" });
    }
    let n = parts[0].len();
    let mut combined = vec![0u32; n];
    for i in 0..n {
        let mut acc: u64 = 0;
        for (part, &size) in parts.iter().zip(sizes.iter()) {
            acc = acc * size as u64 + part[i] as u64;
        }
        combined[i] = acc as u32;
    }
    Ok((combined, total as usize))
}
// #endregion 🔖Packing

// #region 🔖SpecificInformation
/// 🧩 Williams-Beer specific information `I_spec(A -> t) = sum_a p(a|t) ln(p(t|a)/p(t))` for
/// every target outcome `t`, alongside the target marginal `p(t)` used to weight it into a
/// mutual information (`I(A;T) = sum_t p(t) I_spec(A -> t)`) or an `I_min` redundancy term.
fn specific_information(a: &[u32], a_size: usize, t: &[u32], t_size: usize) -> Result<(Vec<f64>, Vec<f64>), EntropyError> {
    let joint = JointCounts::from_pairs(a, t, a_size, t_size)?;
    let total = joint.total();
    let p_a = joint.marginal_x();
    let p_t = joint.marginal_y();
    let mut i_spec = vec![0.0_f64; t_size];
    for tj in 0..t_size {
        if p_t[tj] <= 0.0 {
            continue;
        }
        i_spec[tj] = neumaier_sum((0..a_size).filter_map(|ai| {
            let p_at = joint.get(ai, tj) / total;
            if p_at <= 0.0 || p_a[ai] <= 0.0 {
                return None;
            }
            let p_a_given_t = p_at / p_t[tj];
            let p_t_given_a = p_at / p_a[ai];
            Some(p_a_given_t * (p_t_given_a / p_t[tj]).ln())
        }));
    }
    Ok((i_spec, p_t))
}

/// 🧩 Mutual information `I(A;T)`, computed via the same specific-information pathway used by
/// every PID atom below, so the total-vs-atoms consistency checks never compare two independently
/// derived formulas for "the same" quantity.
fn mutual_information_via_specific(a: &[u32], a_size: usize, t: &[u32], t_size: usize) -> Result<f64, EntropyError> {
    let (i_spec, p_t) = specific_information(a, a_size, t, t_size)?;
    Ok(clamp_near_zero(neumaier_sum(i_spec.iter().zip(p_t.iter()).map(|(&i, &p)| p * i)), 1e-9))
}
// #endregion 🔖SpecificInformation

// #region 🔖TwoSourcePid
/// 🧩 The four non-negative Williams-Beer partial information atoms decomposing `I(S1,S2;T)`,
/// each expressed in the [`LogBase`] requested at the call site.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PidAtoms {
    pub redundancy: f64,
    pub unique_1: f64,
    pub unique_2: f64,
    pub synergy: f64,
}

/// 🧩 Williams-Beer two-source partial information decomposition via the `I_min` redundancy
/// measure, all four quantities estimated by maximum-likelihood plug-in on empirical counts.
/// Deliberately **not** bias-corrected: `Redundancy`/`Unique1`/`Unique2`/`Synergy` are a *linear
/// recombination* of several plug-in mutual informations sharing overlapping alphabets, and
/// naively bias-correcting each term independently (e.g. via `estimators::DiscreteMethod`) would
/// not cancel consistently across the recombination the way it does for a single MI estimate —
/// that would need its own dedicated derivation. This first implementation documents that
/// limitation rather than silently under- or over-correcting.
pub fn pid_two_sources(
    source1: &[u32],
    source2: &[u32],
    target: &[u32],
    sizes: (usize, usize, usize),
    base: LogBase,
) -> Result<PidAtoms, EntropyError> {
    base.validate()?;
    if source1.len() != source2.len() {
        return Err(EntropyError::LengthMismatch { expected: source1.len(), actual: source2.len() });
    }
    if source1.len() != target.len() {
        return Err(EntropyError::LengthMismatch { expected: source1.len(), actual: target.len() });
    }
    if source1.is_empty() {
        return Err(EntropyError::EmptyInput { what: "source1" });
    }
    let (s1_size, s2_size, t_size) = sizes;

    let (i_spec1, p_t) = specific_information(source1, s1_size, target, t_size)?;
    let (i_spec2, _) = specific_information(source2, s2_size, target, t_size)?;
    let (joint12, joint12_size) = pack_symbols(&[source1, source2], &[s1_size, s2_size])?;
    let i12_nats = mutual_information_via_specific(&joint12, joint12_size, target, t_size)?;

    let i1_nats = clamp_near_zero(neumaier_sum(i_spec1.iter().zip(p_t.iter()).map(|(&i, &p)| p * i)), 1e-9);
    let i2_nats = clamp_near_zero(neumaier_sum(i_spec2.iter().zip(p_t.iter()).map(|(&i, &p)| p * i)), 1e-9);

    let redundancy_nats = clamp_near_zero(
        neumaier_sum((0..t_size).map(|tj| p_t[tj] * i_spec1[tj].min(i_spec2[tj]))),
        1e-9,
    );
    let unique1_nats = clamp_near_zero(i1_nats - redundancy_nats, 1e-9);
    let unique2_nats = clamp_near_zero(i2_nats - redundancy_nats, 1e-9);
    let synergy_nats = clamp_near_zero(i12_nats - i1_nats - i2_nats + redundancy_nats, 1e-9);

    Ok(PidAtoms {
        redundancy: base.from_nats(redundancy_nats),
        unique_1: base.from_nats(unique1_nats),
        unique_2: base.from_nats(unique2_nats),
        synergy: base.from_nats(synergy_nats),
    })
}
// #endregion 🔖TwoSourcePid

// #region 🔖Lattice
/// 🧩 One node of the Williams-Beer redundancy lattice for `n = 3` sources: an antichain of
/// non-empty source-index subsets, each subset packed as a bitmask (bit `i` set iff source `i`,
/// 0-based, is a member). Kept sorted ascending for canonical/order-independent equality.
type LatticeNode = Vec<u32>;

/// 🧩 Every non-empty subset of the 3 source indices `{0, 1, 2}`, as a bitmask: `1..=7`.
const NON_EMPTY_SUBSET_MASKS: [u32; 7] = [1, 2, 3, 4, 5, 6, 7];

/// 🧩 The redundancy-lattice order: `alpha <= beta` iff every set in `beta` has some subset (or
/// itself) present in `alpha`. `I_min` is monotone non-decreasing along this order — verified by
/// [`tests::full_set_node_i_min_equals_total_joint_mi_and_singletons_node_is_smaller`] — which
/// places the all-singletons node `{{0},{1},{2}}` at the *bottom* (smallest `I_min`, since a
/// `min` over three independently-estimated specific informations can only be `<=` any one of
/// them) and the single-full-set node `{{0,1,2}}` at the *top* (largest `I_min`, exactly the
/// total joint mutual information, an upper bound for every other node by data processing).
// 🧩 `a & b == a` tests "is `a` a submask of `b`", not a fixed-value membership check — despite
// the closure's shape, this is not a `Vec::contains` rewrite (clippy's `manual_contains` lint
// pattern-matches too eagerly here since the target value is self-referential on the loop
// variable `a`).
#[allow(clippy::manual_contains)]
fn is_below(alpha: &LatticeNode, beta: &LatticeNode) -> bool {
    beta.iter().all(|&b| alpha.iter().any(|&a| (a & b) == a))
}

/// 🧩 Enumerates all antichains of non-empty subsets of `{0, 1, 2}` (no member is a subset of
/// another) by brute force over the `2^7` subsets of the 7-element ground set of non-empty
/// masks — exhaustively correct at this size and self-verifying against the known count of 18,
/// rather than a hand-typed (and hand-error-prone) list.
fn enumerate_antichain_nodes() -> Vec<LatticeNode> {
    let mut nodes = Vec::new();
    for bits in 1u32..(1u32 << NON_EMPTY_SUBSET_MASKS.len()) {
        let mut chosen: Vec<u32> = Vec::new();
        for (i, &mask) in NON_EMPTY_SUBSET_MASKS.iter().enumerate() {
            if bits & (1 << i) != 0 {
                chosen.push(mask);
            }
        }
        let is_antichain = chosen
            .iter()
            .enumerate()
            .all(|(ia, &a)| chosen.iter().enumerate().all(|(ib, &b)| ia == ib || (a & b) != a));
        if is_antichain {
            chosen.sort_unstable();
            nodes.push(chosen);
        }
    }
    nodes
}

/// 🧩 A computed Williams-Beer redundancy lattice for exactly `n = 3` sources: the `I_min` value
/// at every one of the 18 antichain nodes, their Mobius-inverted partial information atoms `Pi`,
/// and the total joint mutual information `I(S1,S2,S3;T)` those 18 atoms sum to.
pub struct PidLattice {
    nodes: Vec<LatticeNode>,
    i_min_nats: Vec<f64>,
    partial_info_nats: Vec<f64>,
    total_mi_nats: f64,
    base: LogBase,
}

impl PidLattice {
    /// 🧩 Computes the full 18-node redundancy lattice for exactly 3 sources against `target`.
    /// `sizes` gives each source's alphabet size in the same order as `sources`; `target_size` is
    /// the target's alphabet size. Rejects `sources.len() != 3` — this first implementation does
    /// not attempt a general-`n` lattice (the antichain count grows combinatorially and the
    /// well-known closed enumeration only exists at small `n`).
    pub fn compute(
        sources: &[&[u32]],
        target: &[u32],
        sizes: &[usize],
        target_size: usize,
        base: LogBase,
    ) -> Result<Self, EntropyError> {
        base.validate()?;
        if sources.len() != 3 {
            return Err(EntropyError::InvalidConfig {
                field: "sources",
                reason: "PidLattice currently supports exactly 3 sources",
            });
        }
        if sizes.len() != 3 {
            return Err(EntropyError::ShapeMismatch { what: "sizes", expected: 3, actual: sizes.len() });
        }
        if target.is_empty() {
            return Err(EntropyError::EmptyInput { what: "target" });
        }
        for &s in sources {
            if s.len() != target.len() {
                return Err(EntropyError::LengthMismatch { expected: target.len(), actual: s.len() });
            }
        }

        // #region 🔖SubsetJoints
        // 🧩 packed joint symbols + alphabet size, and the resulting specific-information vector,
        // for every one of the 7 non-empty source subsets — indexed directly by bitmask (index 0
        // unused) rather than a `HashMap`, since the key space is fixed and tiny.
        let mut packed: [Option<(Vec<u32>, usize)>; 8] = [None, None, None, None, None, None, None, None];
        for &mask in NON_EMPTY_SUBSET_MASKS.iter() {
            let idxs: Vec<usize> = (0..3).filter(|i| mask & (1 << i) != 0).collect();
            let parts: Vec<&[u32]> = idxs.iter().map(|&i| sources[i]).collect();
            let part_sizes: Vec<usize> = idxs.iter().map(|&i| sizes[i]).collect();
            packed[mask as usize] = Some(pack_symbols(&parts, &part_sizes)?);
        }

        let mut spec: [Option<Vec<f64>>; 8] = [None, None, None, None, None, None, None, None];
        let mut p_t: Vec<f64> = Vec::new();
        for &mask in NON_EMPTY_SUBSET_MASKS.iter() {
            let (sym, size) = packed[mask as usize].as_ref().unwrap();
            let (i_spec, this_p_t) = specific_information(sym, *size, target, target_size)?;
            if p_t.is_empty() {
                p_t = this_p_t;
            }
            spec[mask as usize] = Some(i_spec);
        }
        // #endregion 🔖SubsetJoints

        let total_mi_nats = clamp_near_zero(
            neumaier_sum(spec[7].as_ref().unwrap().iter().zip(p_t.iter()).map(|(&i, &p)| p * i)),
            1e-9,
        );

        // #region 🔖IMin
        let nodes = enumerate_antichain_nodes();
        let i_min_nats: Vec<f64> = nodes
            .iter()
            .map(|node| {
                let value = neumaier_sum((0..target_size).map(|tj| {
                    let m = node
                        .iter()
                        .map(|&mask| spec[mask as usize].as_ref().unwrap()[tj])
                        .fold(f64::INFINITY, f64::min);
                    p_t[tj] * m
                }));
                clamp_near_zero(value, 1e-9)
            })
            .collect();
        // #endregion 🔖IMin

        // #region 🔖MobiusInversion
        // 🧩 `predecessors[i]` = every strictly-smaller node under [`is_below`]. Since the order
        // is transitive, a node's predecessor-set size is strictly greater than any of its own
        // predecessors' — sorting by that size ascending is therefore a valid topological order
        // without needing a dedicated Kahn's-algorithm pass.
        let predecessors: Vec<Vec<usize>> = (0..nodes.len())
            .map(|i| (0..nodes.len()).filter(|&j| j != i && is_below(&nodes[j], &nodes[i])).collect())
            .collect();
        let mut order: Vec<usize> = (0..nodes.len()).collect();
        order.sort_by_key(|&i| predecessors[i].len());

        let mut partial_info_nats = vec![0.0_f64; nodes.len()];
        for &i in &order {
            let sum_predecessors = neumaier_sum(predecessors[i].iter().map(|&j| partial_info_nats[j]));
            partial_info_nats[i] = i_min_nats[i] - sum_predecessors;
        }
        // #endregion 🔖MobiusInversion

        Ok(Self { nodes, i_min_nats, partial_info_nats, total_mi_nats, base })
    }

    /// 🧩 Number of lattice nodes — always 18 for `n = 3` sources.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// 🧩 Looks up `Pi(alpha)` (converted to this lattice's `base`, as passed to
    /// [`PidLattice::compute`]) for the antichain described by `node_sets`: each inner `Vec<usize>`
    /// is a 0-based source-index subset, and both the outer and inner order are irrelevant (the
    /// antichain is compared as a set of sets).
    pub fn partial_information(&self, node_sets: &[Vec<usize>]) -> Option<f64> {
        self.node_index(node_sets).map(|i| self.base.from_nats(self.partial_info_nats[i]))
    }

    /// 🧩 Looks up the raw `I_min(alpha)` redundancy value (before Mobius inversion) at the node
    /// described by `node_sets`, converted to `base` — the diagnostic quantity every
    /// [`PidLattice::partial_information`] atom is derived from, useful for inspecting the
    /// lattice's intermediate state (e.g. confirming monotonicity along [`is_below`]) rather than
    /// only its final decomposition.
    pub fn i_min(&self, node_sets: &[Vec<usize>], base: LogBase) -> Option<f64> {
        self.node_index(node_sets).map(|i| base.from_nats(self.i_min_nats[i]))
    }

    fn node_index(&self, node_sets: &[Vec<usize>]) -> Option<usize> {
        let mut masks: Vec<u32> = node_sets
            .iter()
            .map(|subset| subset.iter().fold(0u32, |acc, &idx| acc | 1u32.checked_shl(idx as u32).unwrap_or(0)))
            .collect();
        masks.sort_unstable();
        masks.dedup();
        self.nodes.iter().position(|n| n == &masks)
    }

    /// 🧩 The total joint mutual information `I(S1,S2,S3;T)`, converted to `base` (independent of
    /// whatever `base` was passed to [`PidLattice::compute`]).
    pub fn total_mutual_information(&self, base: LogBase) -> f64 {
        base.from_nats(self.total_mi_nats)
    }
}
// #endregion 🔖Lattice

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::numeric::Xorshift64;

    // #region 🔖TwoSourceLogicGates
    #[test]
    fn copy_gate_shows_dominant_unique1_and_near_zero_synergy() {
        // 🔐 T = S1 exactly, S2 independent noise: all info about T is uniquely S1's.
        let mut rng = Xorshift64::new(101);
        let n = 3000;
        let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s2: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let target = s1.clone();
        let atoms = pid_two_sources(&s1, &s2, &target, (2, 2, 2), LogBase::Nats).unwrap();
        assert!(atoms.unique_1 > 0.5, "unique_1={}", atoms.unique_1);
        assert!(atoms.redundancy < 0.1, "redundancy={}", atoms.redundancy);
        assert!(atoms.unique_2 < 0.1, "unique_2={}", atoms.unique_2);
        assert!(atoms.synergy.abs() < 0.1, "synergy={}", atoms.synergy);
    }

    #[test]
    fn xor_gate_shows_dominant_synergy_near_one_bit() {
        // 🔐 T = S1 XOR S2, S1/S2 independent fair coins: the classic pure-synergy example.
        let mut rng = Xorshift64::new(202);
        let n = 4000;
        let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s2: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let target: Vec<u32> = s1.iter().zip(s2.iter()).map(|(&a, &b)| a ^ b).collect();
        let atoms = pid_two_sources(&s1, &s2, &target, (2, 2, 2), LogBase::Nats).unwrap();
        assert!((atoms.synergy - core::f64::consts::LN_2).abs() < 0.1, "synergy={}", atoms.synergy);
        assert!(atoms.redundancy < 0.1, "redundancy={}", atoms.redundancy);
        assert!(atoms.unique_1 < 0.1, "unique_1={}", atoms.unique_1);
        assert!(atoms.unique_2 < 0.1, "unique_2={}", atoms.unique_2);
    }
    // #endregion 🔖TwoSourceLogicGates

    // #region 🔖TwoSourceValidation
    #[test]
    fn pid_two_sources_rejects_length_mismatch() {
        assert!(matches!(
            pid_two_sources(&[0, 1], &[0], &[0, 1], (2, 1, 2), LogBase::Nats),
            Err(EntropyError::LengthMismatch { .. })
        ));
    }

    #[test]
    fn pid_two_sources_rejects_empty_input() {
        assert!(matches!(
            pid_two_sources(&[], &[], &[], (2, 2, 2), LogBase::Nats),
            Err(EntropyError::EmptyInput { .. })
        ));
    }
    // #endregion 🔖TwoSourceValidation

    // #region 🔖Lattice
    #[test]
    fn lattice_has_exactly_eighteen_nodes() {
        let mut rng = Xorshift64::new(303);
        let n = 1000;
        let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s2: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s3: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let target: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let lattice = PidLattice::compute(&[&s1, &s2, &s3], &target, &[2, 2, 2], 2, LogBase::Nats).unwrap();
        assert_eq!(lattice.node_count(), 18);
    }

    #[test]
    fn lattice_rejects_source_counts_other_than_three() {
        let s1 = [0u32, 1, 0, 1];
        let target = [0u32, 1, 1, 0];
        assert!(matches!(
            PidLattice::compute(&[&s1, &s1], &target, &[2, 2], 2, LogBase::Nats),
            Err(EntropyError::InvalidConfig { .. })
        ));
    }

    #[test]
    fn full_set_node_i_min_equals_total_joint_mi_and_singletons_node_is_smaller() {
        let mut rng = Xorshift64::new(404);
        let n = 2000;
        let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s2: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s3: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let target: Vec<u32> = s1.iter().zip(s2.iter()).map(|(&a, &b)| a ^ b).collect();
        let lattice = PidLattice::compute(&[&s1, &s2, &s3], &target, &[2, 2, 2], 2, LogBase::Nats).unwrap();
        let full_set_idx = lattice.nodes.iter().position(|node| node == &vec![7u32]).unwrap();
        let singletons_idx = lattice.nodes.iter().position(|node| node == &vec![1u32, 2, 4]).unwrap();
        assert!((lattice.i_min_nats[full_set_idx] - lattice.total_mi_nats).abs() < 1e-9);
        assert!(lattice.i_min_nats[singletons_idx] <= lattice.i_min_nats[full_set_idx] + 1e-9);
    }

    #[test]
    fn sum_of_all_partial_information_equals_total_mutual_information() {
        // 🔐 the critical Mobius/zeta consistency check: sum(Pi) over all 18 nodes must equal the
        // total joint MI exactly (an algebraic identity of the inversion, not a statistical
        // convergence property), on several independently seeded random datasets.
        for seed in [11u64, 22u64, 33u64] {
            let mut rng = Xorshift64::new(seed);
            let n = 1500;
            let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
            let s2: Vec<u32> = (0..n).map(|_| rng.next_below(3) as u32).collect();
            let s3: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
            let target: Vec<u32> = (0..n).map(|_| rng.next_below(3) as u32).collect();
            let lattice = PidLattice::compute(&[&s1, &s2, &s3], &target, &[2, 3, 2], 3, LogBase::Nats).unwrap();
            let sum_pi: f64 = lattice.partial_info_nats.iter().sum();
            let total_mi = lattice.total_mutual_information(LogBase::Nats);
            assert!((sum_pi - total_mi).abs() < 1e-6, "seed={seed} sum_pi={sum_pi} total_mi={total_mi}");
        }
    }

    #[test]
    fn partial_information_lookup_matches_internal_full_set_node() {
        let mut rng = Xorshift64::new(505);
        let n = 1200;
        let s1: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s2: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let s3: Vec<u32> = (0..n).map(|_| rng.next_below(2) as u32).collect();
        let target: Vec<u32> = s1.iter().zip(s3.iter()).map(|(&a, &b)| a ^ b).collect();
        let lattice = PidLattice::compute(&[&s1, &s2, &s3], &target, &[2, 2, 2], 2, LogBase::Nats).unwrap();
        let looked_up = lattice.partial_information(&[vec![0, 1, 2]]).unwrap();
        let full_set_idx = lattice.nodes.iter().position(|node| node == &vec![7u32]).unwrap();
        assert!((looked_up - lattice.partial_info_nats[full_set_idx]).abs() < 1e-12);
        // 🔐 an antichain referencing a source index that doesn't correspond to any lattice node
        // (99 is out of range for a 3-source lattice) must return `None`, not panic.
        assert!(lattice.partial_information(&[vec![0], vec![1], vec![99]]).is_none());
    }
    // #endregion 🔖Lattice
}
// #endregion 🔖Tests
