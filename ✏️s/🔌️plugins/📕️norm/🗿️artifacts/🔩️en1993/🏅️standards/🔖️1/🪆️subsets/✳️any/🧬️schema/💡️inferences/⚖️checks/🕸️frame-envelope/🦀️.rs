//! 🕸️ Frame results per load case, per EN 1990 combination and as envelope per bar: the `frameActions` of every load case are combined by the shared EN 1990 rules (6.10a/b, [`combine_scalar_forces`]), every ultimate combination is solved on its own (second order is not additive), every combination carries the global sway imperfection of EN 1993-1-1 5.3.2(3) as equivalent horizontal forces (5.3.2(7)), and the forces of the bars of a member feed its checks when the member force basis is `FrameDerived`. The bow imperfection e0/L of Table 5.1 is NOT part of these forces (method 5.2.2(3)b): the member checks of 6.3 carry it in the buckling curve, so it is never counted twice; the section rows of the frame check (5.2.2(3)c) add it in their own solve. See [EN 1990](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use semio_s_artifact_norm_en1993_frame_eigen::{self as fe, BarEffects, FrameFault, FrameLoads, Imperfections, SecondOrder, Station};
use semio_s_artifact_norm_en1993_model::{DesignAction, MemberForceBasis};
use semio_s_plugin_norm_mechanics as mech;
use std::{cell::RefCell, rc::Rc};

/// 📋️ One load vector of the frame with its combination id (`design` for the explicit node loads, `uls-6.10a-<lead>` / `uls-6.10b-<lead>` / `uls-g` for combinations).
#[derive(Clone, Debug, PartialEq)]
pub struct LoadSet {
    pub id: String,
    pub loads: FrameLoads,
}

/// 🔁️ The second-order solution of the frame with the semi-rigid joint springs of its members under one load vector.
pub fn second_order(document: &En1993Snapshot, loads: &FrameLoads, imperfections: &Imperfections) -> Result<SecondOrder, FrameFault> {
    fe::second_order_with(document, &mech::Control::none(), loads, imperfections, &|j| joint_row_model::joint_secant(document, j))
}

/// 🌀️ The buckling modes of the frame with the semi-rigid joint springs of its members under one load vector.
pub fn eigen(document: &En1993Snapshot, loads: &FrameLoads) -> Result<fe::FrameEigen, FrameFault> {
    fe::frame_eigen_with(document, &mech::Control::none(), &|j| joint_row_model::joint_secant(document, j), loads)
}

fn fault(en: String, de: String) -> FrameFault {
    FrameFault { en, de }
}

/// 🧮️ The ultimate load vectors of the frame. Without `frameActions` it is the explicit node loads alone; otherwise every EN 1990 ultimate combination of the load cases (factors from [`combine_scalar_forces`] applied to unit forces, so the shared rule is not repeated) with the explicit node loads added unfactored.
pub fn load_sets(document: &En1993Snapshot) -> Result<Vec<LoadSet>, FrameFault> {
    let design = FrameLoads::design(document);
    if document.frame_actions.is_empty() {
        return Ok(vec![LoadSet { id: "design".into(), loads: design }]);
    }
    if let Some(action) = document.frame_actions.iter().find(|a| !document.load_cases.iter().any(|c| c.id == a.load_case_id)) {
        return Err(fault(format!("Frame action '{}' refers to the unknown load case '{}'.", action.id, action.load_case_id), format!("Rahmeneinwirkung '{}' verweist auf den unbekannten Lastfall '{}'.", action.id, action.load_case_id)));
    }
    let mut cases: Vec<(String, FrameLoads)> = Vec::new();
    for case in &document.load_cases {
        let loads = FrameLoads::of_case(document, &case.id)?;
        if !loads.is_empty() {
            cases.push((case.id.clone(), loads));
        }
    }
    let unit = |k: usize| -> Vec<CombinedScalar> { combine_scalar_forces(document, &cases.iter().enumerate().map(|(i, (id, _))| (id.clone(), if i == k { 1.0 } else { 0.0 })).collect::<Vec<_>>()) };
    let factors: Vec<Vec<CombinedScalar>> = (0..cases.len()).map(unit).collect();
    let mut ids: Vec<String> = Vec::new();
    for combination in factors.iter().flatten().filter(|c| c.situation == "uls") {
        if !ids.contains(&combination.combination_id) {
            ids.push(combination.combination_id.clone());
        }
    }
    Ok(ids
        .into_iter()
        .map(|id| {
            let mut loads = design.clone();
            for ((_, case_loads), own) in cases.iter().zip(&factors) {
                let k = own.iter().find(|c| c.combination_id == id).map_or(0.0, |c| c.value);
                if k != 0.0 {
                    loads.add(&case_loads.scaled(k));
                }
            }
            LoadSet { id, loads }
        })
        .collect())
}

/// 🎯️ The load vector the stability and the checks of the whole frame start from: the ultimate combination with the largest vertical load.
pub fn governing_loads(document: &En1993Snapshot) -> Result<LoadSet, FrameFault> {
    let weight = |set: &LoadSet| set.loads.vertical_at_nodes(document).iter().map(|(_, f)| f.abs()).sum::<f64>();
    load_sets(document)?.into_iter().max_by(|a, b| weight(a).total_cmp(&weight(b))).ok_or_else(|| fault("The frame has no ultimate load combination.".into(), "Der Rahmen hat keine Grenzzustandskombination der Tragfähigkeit.".into()))
}

/// 🏢️ One storey of the frame between two node levels with the loads above it for the exemption of EN 1993-1-1 5.3.2(4): the sway imperfection may be disregarded when H_Ed ≥ 0.15 V_Ed.
#[derive(Clone, Debug, PartialEq)]
pub struct Storey {
    pub level: f64,
    pub horizontal: f64,
    pub vertical: f64,
    pub exempt: bool,
}

/// 📐️ The global sway imperfection of one load vector: φ = φ0 α_h α_m (φ0 from `ndp.en1993.1-1.phi0`, α_h from the frame height, α_m from the compressed columns), the equivalent horizontal force φ·V per node without a rigid or pinned support (5.3.2(7)) and the storeys checked for the exemption. `forces` are magnitudes; the direction is chosen per combination.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sway {
    pub height: f64,
    pub columns: usize,
    pub phi: f64,
    pub forces: Vec<(String, f64)>,
    pub storeys: Vec<Storey>,
}

impl Sway {
    /// ➡️ The equivalent horizontal forces `(node, N)` with the given direction (+1 towards +x, -1 towards -x).
    pub fn signed(&self, sign: f64) -> Vec<(String, f64)> {
        self.forces.iter().map(|(node, force)| (node.clone(), sign * force)).collect()
    }
}

/// 🔁️ The second-order result of one ultimate combination with its unfavourable sway imperfection (`sign` is 0 when no imperfection applies).
#[derive(Clone, Debug, PartialEq)]
pub struct CombinationResult {
    pub id: String,
    pub result: SecondOrder,
    pub sway: Sway,
    pub sign: f64,
}

impl CombinationResult {
    /// ➡️ The equivalent horizontal imperfection forces `(node, N)` applied in this combination.
    pub fn imperfection_forces(&self) -> Vec<(String, f64)> {
        self.sway.signed(self.sign)
    }
}

/// 🔁️ The second-order result of one load case with its characteristic loads (no factors, no imperfections).
#[derive(Clone, Debug, PartialEq)]
pub struct CaseResult {
    pub case_id: String,
    pub result: SecondOrder,
}

/// 🎯️ The combination under which a bar has its largest bending moment and the peak forces of that combination.
#[derive(Clone, Debug, PartialEq)]
pub struct Governing {
    pub combination: String,
    pub effects: BarEffects,
}

/// 📈️ Envelope of one bar over all ultimate combinations: largest and smallest `n`, `v`, `m` at every station and the governing combination.
#[derive(Clone, Debug, PartialEq)]
pub struct BarEnvelope {
    pub bar_id: String,
    pub max: Vec<Station>,
    pub min: Vec<Station>,
    pub governing: Governing,
}

/// 📏️ True when the bar stands more vertical than horizontal.
pub fn is_column(document: &En1993Snapshot, bar: &semio_s_artifact_norm_en1993_model::FrameBar) -> bool {
    let at = |id: &str| document.frame_nodes.iter().find(|n| n.id == id).map_or((0.0, 0.0), |n| (n.x, n.z));
    let (a, b) = (at(&bar.start_node_id), at(&bar.end_node_id));
    (b.1 - a.1).abs() >= (b.0 - a.0).abs()
}

/// 🏛️ The number m of columns in a row for α_m (EN 1993-1-1 5.3.2(3)b): per storey (columns grouped by their lower level) only the columns whose axial force is at least half the average of that storey count, and the smallest count over all storeys governs because it gives the largest α_m; `compressed` holds (lower level, compression) of every compressed column.
pub fn columns_in_a_row(compressed: &[(f64, f64)]) -> usize {
    const TOLERANCE: f64 = 1e-6;
    let mut rows: Vec<(f64, Vec<f64>)> = Vec::new();
    for &(level, n) in compressed {
        match rows.iter_mut().find(|(l, _)| (l - level).abs() <= TOLERANCE) {
            Some((_, forces)) => forces.push(n),
            None => rows.push((level, vec![n])),
        }
    }
    rows.iter()
        .map(|(_, forces)| {
            let average = forces.iter().sum::<f64>() / forces.len() as f64;
            forces.iter().filter(|n| **n >= 0.5 * average).count()
        })
        .min()
        .unwrap_or(0)
}

/// 📐️ The sway imperfection of EN 1993-1-1 5.3.2(3) to (7) for the load vector `loads`; `base` is the solution without imperfections (compressed columns and their number m). The imperfection of a node is dropped when the storey below it is exempt by H_Ed ≥ 0.15 V_Ed (5.3.2(4)), H_Ed and V_Ed being the horizontal and vertical loads at and above the storey top.
pub fn sway_imperfection(document: &En1993Snapshot, loads: &FrameLoads, base: &SecondOrder) -> Sway {
    const TOLERANCE: f64 = 1e-6;
    let (low, high) = document.frame_nodes.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), n| (l.min(n.z), h.max(n.z)));
    let height = if high > low { high - low } else { 0.0 };
    let level_of = |id: &str| document.frame_nodes.iter().find(|n| n.id == id).map_or(0.0, |n| n.z);
    let compressed: Vec<(f64, f64)> = document
        .frame_bars
        .iter()
        .filter(|b| is_column(document, b))
        .filter_map(|b| base.bar(&b.id).map(|f| (level_of(&b.start_node_id).min(level_of(&b.end_node_id)), -f.first.n)))
        .filter(|(_, n)| *n > 0.0)
        .collect();
    let columns = columns_in_a_row(&compressed);
    let phi = if columns > 0 && height > 0.0 { clauses::global::sway_phi(height, columns as f64) } else { 0.0 };
    let (vertical, horizontal) = (loads.vertical_at_nodes(document), loads.horizontal_at_nodes(document));
    let at = |list: &[(String, f64)], id: &str| list.iter().filter(|(n, _)| n == id).map(|(_, f)| f).sum::<f64>();
    let mut levels: Vec<f64> = document.frame_nodes.iter().map(|n| n.z).collect();
    levels.sort_by(f64::total_cmp);
    levels.dedup_by(|a, b| (*a - *b).abs() <= TOLERANCE);
    let storeys: Vec<Storey> = levels
        .iter()
        .skip(1)
        .map(|&level| {
            let above = || document.frame_nodes.iter().filter(move |n| n.z >= level - TOLERANCE);
            let (h, v) = (above().map(|n| at(&horizontal, &n.id)).sum::<f64>(), -above().map(|n| at(&vertical, &n.id)).sum::<f64>());
            Storey { level, horizontal: h, vertical: v, exempt: v > 0.0 && clauses::global::sway_may_be_neglected(h.abs(), v) }
        })
        .collect();
    let exempt_below = |z: f64| levels.iter().position(|l| (l - z).abs() <= TOLERANCE).and_then(|i| i.checked_sub(1)).and_then(|i| storeys.get(i)).is_some_and(|s| s.exempt);
    let forces = document
        .frame_nodes
        .iter()
        .filter(|n| !matches!(n.support, semio_s_artifact_norm_en1993_model::FrameNodeSupport::Fixed | semio_s_artifact_norm_en1993_model::FrameNodeSupport::Pinned) && !exempt_below(n.z))
        .map(|n| (n.id.clone(), clauses::global::sway_equivalent_force(phi, -at(&vertical, &n.id))))
        .filter(|(_, force)| *force > 0.0)
        .collect();
    Sway { height, columns, phi, forces, storeys }
}

/// 📈️ The sum of the second-order peak moments of all bars: the measure that picks the unfavourable direction of an imperfection.
pub fn peak_moment(result: &SecondOrder) -> f64 {
    result.bars.iter().map(|b| b.second.m).sum()
}

/// 🔁️ The second-order result of one load vector with the sway imperfection of 5.3.2 in the unfavourable direction: both directions are solved and the one with the larger [`peak_moment`] is kept; without an applicable imperfection the plain result and sign 0.
pub fn solve_with_sway(document: &En1993Snapshot, loads: &FrameLoads) -> Result<(SecondOrder, Sway, f64), FrameFault> {
    let base = second_order(document, loads, &Imperfections::default())?;
    let sway = sway_imperfection(document, loads, &base);
    if sway.forces.is_empty() {
        return Ok((base, sway, 0.0));
    }
    let mut best: Option<(SecondOrder, f64)> = None;
    for sign in [1.0, -1.0] {
        let result = second_order(document, loads, &Imperfections { sway: sway.signed(sign), bow: Vec::new() })?;
        if best.as_ref().is_none_or(|(kept, _)| peak_moment(&result) > peak_moment(kept)) {
            best = Some((result, sign));
        }
    }
    let (result, sign) = best.unwrap_or((base, 0.0));
    Ok((result, sway, sign))
}

/// 🔁️ Every ultimate combination of the frame solved on its own with the unfavourable sway imperfection.
pub fn combination_results(document: &En1993Snapshot) -> Result<Vec<CombinationResult>, FrameFault> {
    load_sets(document)?
        .into_iter()
        .map(|set| solve_with_sway(document, &set.loads).map(|(result, sway, sign)| CombinationResult { id: set.id, result, sway, sign }))
        .collect()
}

/// 🔁️ Every load case with `frameActions` solved on its own with the characteristic loads.
pub fn case_results(document: &En1993Snapshot) -> Result<Vec<CaseResult>, FrameFault> {
    let mut out = Vec::new();
    for case in &document.load_cases {
        let loads = FrameLoads::of_case(document, &case.id)?;
        if !loads.is_empty() {
            out.push(CaseResult { case_id: case.id.clone(), result: second_order(document, &loads, &Imperfections::default())? });
        }
    }
    Ok(out)
}

/// 📈️ The envelope of every frame bar over the given combination results (second-order courses).
pub fn envelope(document: &En1993Snapshot, results: &[CombinationResult]) -> Vec<BarEnvelope> {
    document
        .frame_bars
        .iter()
        .filter_map(|bar| {
            let rows: Vec<(&CombinationResult, &fe::BarForces)> = results.iter().filter_map(|r| r.result.bar(&bar.id).map(|b| (r, b))).collect();
            let first = rows.first()?.1;
            let stations = first.second_profile.len();
            let pick = |keep: fn(f64, f64) -> f64, start: f64| -> Vec<Station> {
                (0..stations)
                    .map(|i| {
                        let at = |f: fn(&Station) -> f64| rows.iter().filter_map(|(_, b)| b.second_profile.get(i)).map(f).fold(start, keep);
                        Station { x: first.second_profile[i].x, n: at(|s| s.n), v: at(|s| s.v), m: at(|s| s.m) }
                    })
                    .collect()
            };
            let (combination, forces) = rows.iter().max_by(|a, b| (a.1.second.m, a.1.second.n.abs()).partial_cmp(&(b.1.second.m, b.1.second.n.abs())).unwrap_or(std::cmp::Ordering::Equal))?;
            Some(BarEnvelope { bar_id: bar.id.clone(), max: pick(f64::max, f64::NEG_INFINITY), min: pick(f64::min, f64::INFINITY), governing: Governing { combination: combination.id.clone(), effects: forces.second } })
        })
        .collect()
}

fn same_inputs(a: &En1993Snapshot, b: &En1993Snapshot) -> bool {
    a.frame_nodes == b.frame_nodes && a.frame_bars == b.frame_bars && a.frame_actions == b.frame_actions && a.load_cases == b.load_cases && a.members == b.members && a.sections == b.sections && a.materials == b.materials && a.joints == b.joints && a.annex == b.annex && a.reliability_class == b.reliability_class
}

type Memo = Option<(En1993Snapshot, Result<Rc<Vec<CombinationResult>>, FrameFault>)>;

thread_local! {
    static MEMO: RefCell<Memo> = const { RefCell::new(None) };
}

/// ⚡️ The combination results of the frame, remembered for the last frame so the many member checks of one evaluation, the frame scene fragments and the design diagrams solve it once; the inputs are compared in place, no text is built per call.
pub fn combination_results_cached(document: &En1993Snapshot) -> Result<Rc<Vec<CombinationResult>>, FrameFault> {
    MEMO.with(|memo| {
        if let Some((known, r)) = &*memo.borrow() {
            if same_inputs(known, document) {
                return r.clone();
            }
        }
        let r = combination_results(document).map(Rc::new);
        *memo.borrow_mut() = Some((document.clone(), r.clone()));
        r
    })
}

/// 🧭️ The ultimate design effects of a member from the frame: per combination the action of its bar with the largest second-order bending moment (compression positive as in `memberActions`); `None` when the member force basis is not `FrameDerived` or the member has no bars. The forces carry the sway imperfection of the combination (5.3.2(3)); the bow imperfection stays in the member check of 6.3 (no double counting). A frame that cannot be solved gives no rows, the frame checks report the fault; the serviceability rows of the explicit member actions are kept.
pub fn frame_derived_effects(document: &En1993Snapshot, member_id: &str, explicit: Vec<CombinedEffects>) -> Vec<CombinedEffects> {
    let derived = document.frame_member_bases.iter().any(|b| b.member_id == member_id && b.basis == MemberForceBasis::FrameDerived);
    let bars: Vec<&str> = document.frame_bars.iter().filter(|b| b.member_id == member_id).map(|b| b.id.as_str()).collect();
    if !derived || bars.is_empty() {
        return explicit;
    }
    let mut out: Vec<CombinedEffects> = explicit.into_iter().filter(|e| e.situation != "uls").collect();
    let Ok(results) = combination_results_cached(document) else { return out };
    for combination in results.iter() {
        let governing = bars.iter().filter_map(|id| combination.result.bar(id)).map(|b| b.second).max_by(|a, b| a.m.total_cmp(&b.m));
        if let Some(effects) = governing {
            out.push(CombinedEffects { combination_id: combination.id.clone(), situation: "uls".into(), action: DesignAction { n: -effects.n, vy: 0.0, vz: effects.v, my: effects.m, mz: 0.0, t: 0.0 } });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_s_artifact_norm_en1993_model::{FrameAction, FrameActionKind, FrameMemberBasis};

    fn action(id: &str, case: &str, kind: FrameActionKind, target: &str, fx: f64, fz: f64) -> FrameAction {
        FrameAction { id: id.into(), load_case_id: case.into(), target_id: target.into(), kind, fx, fz, my: 0.0, start_ratio: 0.0, end_ratio: 1.0 }
    }

    fn portal() -> En1993Snapshot {
        let mut doc = En1993Snapshot::frame_portal_showcase();
        doc.frame_nodes.iter_mut().for_each(|n| (n.load_x, n.load_z) = (0.0, 0.0));
        doc.frame_actions = vec![
            action("g-beam", "g-permanent", FrameActionKind::BarUniform, "bar-2", 0.0, -20_000.0),
            action("q-beam", "q-imposed", FrameActionKind::BarUniform, "bar-2", 0.0, -15_000.0),
            action("w-eaves", "w-wind", FrameActionKind::NodalForce, "node-b", 12_000.0, 0.0),
        ];
        doc
    }

    #[test]
    fn the_combinations_are_the_en1990_rules_applied_to_the_load_cases() {
        let doc = portal();
        let sets = load_sets(&doc).unwrap();
        let ids: Vec<&str> = sets.iter().map(|s| s.id.as_str()).collect();
        assert!(ids.contains(&"uls-6.10a-q-imposed") && ids.contains(&"uls-6.10b-w-wind"), "{ids:?}");
        let by = |id: &str| sets.iter().find(|s| s.id == id).unwrap();
        let lead_q = by("uls-6.10a-q-imposed");
        let beam = lead_q.loads.line.iter().map(|l| l.fz).sum::<f64>();
        let direct: Vec<f64> = combine_scalar_forces(&doc, &[("g-permanent".into(), -20_000.0), ("q-imposed".into(), -15_000.0), ("w-wind".into(), 0.0)]).iter().filter(|c| c.combination_id == "uls-6.10a-q-imposed").map(|c| c.value).collect();
        assert!((beam - direct[0]).abs() < 1e-6, "beam line load {beam} vs combine_scalar_forces {}", direct[0]);
    }

    #[test]
    fn the_envelope_bounds_every_combination_and_names_the_governing_one() {
        let doc = portal();
        let results = combination_results(&doc).unwrap();
        assert!(results.len() >= 4);
        let envelope = envelope(&doc, &results);
        let bar = envelope.iter().find(|e| e.bar_id == "bar-2").unwrap();
        for r in &results {
            for (i, s) in r.result.bar("bar-2").unwrap().second_profile.iter().enumerate() {
                assert!(s.m <= bar.max[i].m + 1e-9 && s.m >= bar.min[i].m - 1e-9);
            }
        }
        let peak = results.iter().map(|r| r.result.bar("bar-2").unwrap().second.m).fold(0.0_f64, f64::max);
        assert!((bar.governing.effects.m - peak).abs() < 1e-9 && results.iter().any(|r| r.id == bar.governing.combination));
    }

    #[test]
    fn a_frame_derived_member_takes_the_envelope_forces_and_an_explicit_one_keeps_its_actions() {
        let mut doc = portal();
        let explicit = combine_member_actions(&doc, "member-b1");
        assert_eq!(frame_derived_effects(&doc, "member-b1", explicit.clone()).len(), explicit.len());
        doc.frame_member_bases.push(FrameMemberBasis { id: "basis-b1".into(), member_id: "member-b1".into(), basis: MemberForceBasis::FrameDerived });
        let derived = frame_derived_effects(&doc, "member-b1", explicit.clone());
        let uls: Vec<&CombinedEffects> = derived.iter().filter(|e| e.situation == "uls").collect();
        assert_eq!(uls.len(), combination_results(&doc).unwrap().len());
        assert!(uls.iter().all(|e| e.action.my > 0.0));
        assert_eq!(derived.iter().filter(|e| e.situation == "sls").count(), explicit.iter().filter(|e| e.situation == "sls").count());
    }

    #[test]
    fn a_load_case_alone_matches_the_first_order_statics_of_its_beam() {
        let doc = portal();
        let cases = case_results(&doc).unwrap();
        let g = cases.iter().find(|c| c.case_id == "g-permanent").unwrap().result.bar("bar-2").unwrap();
        let hogging = g.first_profile.first().unwrap().m.abs();
        let fixed_end = 20_000.0 * 36.0 / 12.0;
        assert!(hogging > 0.5 * fixed_end && hogging < 1.05 * fixed_end, "end moment {hogging} against wl²/12 {fixed_end} of a fully fixed beam");
    }

    #[test]
    fn a_combination_is_the_factored_sum_of_its_load_cases_in_first_order() {
        let doc = En1993Snapshot::frame_cases_showcase();
        let sets = load_sets(&doc).unwrap();
        let set = sets.iter().find(|s| s.id == "uls-6.10a-q-imposed").unwrap();
        let (kg, kq, kw) = (set.loads.line[0].fz / -20_000.0, set.loads.line[1].fz / -15_000.0, set.loads.nodal[0].1[0] / 12_000.0);
        assert!(kg > 1.0 && kq > 1.0 && kw > 0.0 && kw < kq, "factors {kg} {kq} {kw}");
        let cases = case_results(&doc).unwrap();
        let course = |case: &str| cases.iter().find(|c| c.case_id == case).unwrap().result.bar("bar-1").unwrap().first_profile.clone();
        let combined = &second_order(&doc, &set.loads, &Imperfections::default()).unwrap().bar("bar-1").unwrap().first_profile.clone();
        let (g, q, w) = (course("g-permanent"), course("q-imposed"), course("w-wind"));
        for (i, station) in combined.iter().enumerate() {
            let sum = kg * g[i].m + kq * q[i].m + kw * w[i].m;
            assert!((station.m - sum).abs() <= 1e-6 * sum.abs().max(1.0), "station {i}: {} vs {sum}", station.m);
        }
    }

    fn portal_with_horizontal(load_x: f64) -> En1993Snapshot {
        let mut doc = En1993Snapshot::frame_portal_showcase();
        doc.frame_nodes.iter_mut().filter(|n| n.id == "node-b").for_each(|n| n.load_x = load_x);
        doc
    }

    #[test]
    fn the_portal_sway_forces_are_phi_times_the_eaves_loads_of_5_3_2() {
        let doc = portal_with_horizontal(20_000.0);
        let loads = &load_sets(&doc).unwrap()[0].loads;
        let base = second_order(&doc, loads, &Imperfections::default()).unwrap();
        let sway = sway_imperfection(&doc, loads, &base);
        let phi = 0.005 * (2.0_f64 / 4.0_f64.sqrt()).clamp(2.0 / 3.0, 1.0) * (0.5 * (1.0 + 1.0 / 2.0_f64)).sqrt();
        assert!((phi - 4.330127e-3).abs() < 1e-8, "hand value {phi}");
        assert_eq!((sway.columns, sway.height), (2, 4.0));
        assert!((sway.phi - phi).abs() < 1e-12);
        let nodes: Vec<&str> = sway.forces.iter().map(|f| f.0.as_str()).collect();
        assert_eq!(nodes, vec!["node-b", "node-c"]);
        assert!(sway.forces.iter().all(|f| (f.1 - phi * 200_000.0).abs() < 1e-6 && (f.1 - 866.0254).abs() < 1e-3));
        assert!(!sway.storeys[0].exempt && (sway.storeys[0].horizontal - 20_000.0).abs() < 1e-9 && (sway.storeys[0].vertical - 400_000.0).abs() < 1e-6);
    }

    #[test]
    fn m_counts_the_columns_of_a_row_per_storey_and_the_smallest_row_governs() {
        let ground = [(0.0, 100.0e3), (0.0, 100.0e3), (0.0, 100.0e3), (0.0, 100.0e3)];
        let upper = [(3.0, 50.0e3), (3.0, 50.0e3), (3.0, 50.0e3), (3.0, 10.0e3)];
        let all: Vec<(f64, f64)> = ground.iter().chain(upper.iter()).copied().collect();
        assert_eq!(columns_in_a_row(&ground), 4);
        assert_eq!(columns_in_a_row(&upper), 3);
        assert_eq!(columns_in_a_row(&all), 3);
        assert_eq!(columns_in_a_row(&[]), 0);
        let alpha = |m: f64| (0.5 * (1.0 + 1.0 / m)).sqrt();
        assert!((alpha(3.0) - 0.816_496_580_927_726).abs() < 1e-12 && alpha(7.0) < alpha(3.0), "pooling the storeys (m = 7) would lower α_m from {} to {}", alpha(3.0), alpha(7.0));
    }

    #[test]
    fn the_combination_solve_adds_the_sway_forces_in_the_unfavourable_direction() {
        for (load_x, sign) in [(20_000.0, 1.0), (-20_000.0, -1.0)] {
            let doc = portal_with_horizontal(load_x);
            let result = &combination_results(&doc).unwrap()[0];
            assert_eq!(result.sign, sign);
            let opposite = second_order(&doc, &load_sets(&doc).unwrap()[0].loads, &Imperfections { sway: result.sway.signed(-sign), bow: Vec::new() }).unwrap();
            assert!(peak_moment(&result.result) >= peak_moment(&opposite));
            let by_hand = second_order(&doc, &load_sets(&doc).unwrap()[0].loads, &Imperfections { sway: vec![("node-b".into(), sign * 866.0254), ("node-c".into(), sign * 866.0254)], bow: Vec::new() }).unwrap();
            for bar in ["bar-1", "bar-2", "bar-3"] {
                let (a, b) = (result.result.bar(bar).unwrap().second, by_hand.bar(bar).unwrap().second);
                assert!((a.m - b.m).abs() <= 1e-3 * b.m.abs().max(1.0) && (a.n - b.n).abs() <= 1e-3 * b.n.abs().max(1.0), "{bar}: {a:?} vs {b:?}");
            }
            assert_eq!(result.imperfection_forces().iter().map(|f| f.1.signum()).collect::<Vec<_>>(), vec![sign, sign]);
        }
    }

    #[test]
    fn a_storey_with_h_ed_of_at_least_015_v_ed_gets_no_sway_imperfection() {
        let doc = portal_with_horizontal(61_000.0);
        let result = &combination_results(&doc).unwrap()[0];
        assert!(result.sway.storeys[0].exempt && result.sway.forces.is_empty() && result.sign == 0.0);
        let plain = second_order(&doc, &load_sets(&doc).unwrap()[0].loads, &Imperfections::default()).unwrap();
        assert!((peak_moment(&result.result) - peak_moment(&plain)).abs() <= 1e-9 * peak_moment(&plain).abs().max(1.0));
        assert!(!sway_imperfection(&portal_with_horizontal(59_000.0), &load_sets(&portal_with_horizontal(59_000.0)).unwrap()[0].loads, &plain).forces.is_empty());
    }

    fn timing_ms(mut work: impl FnMut(), rounds: usize) -> (f64, f64) {
        let mut ms: Vec<f64> = (0..rounds)
            .map(|_| {
                let started = std::time::Instant::now();
                work();
                started.elapsed().as_secs_f64() * 1000.0
            })
            .collect();
        ms.sort_by(f64::total_cmp);
        let at = |percent: usize| ms[(ms.len() * percent / 100).min(ms.len() - 1)];
        (at(50), at(95))
    }

    #[test]
    fn the_frame_envelope_fits_the_commit_budget_and_a_repaint_on_the_same_frame_is_free() {
        let scale = if cfg!(debug_assertions) { 20.0 } else { 1.0 };
        let mut doc = portal();
        let started = std::time::Instant::now();
        std::hint::black_box(combination_results_cached(&doc).unwrap().len());
        let cold = started.elapsed().as_secs_f64() * 1000.0;
        eprintln!("[latency] frame-envelope/cold p50_ms={cold:.3} p95_ms={cold:.3} budget_ms={:.1}", 1000.0 * scale);
        assert!(cold < 1000.0 * scale, "cold first frame solve {cold:.1} ms exceeds the 1000 ms cold budget");
        let edit = |doc: &mut En1993Snapshot| doc.frame_actions[0].fz -= 1.0;
        (0..3).for_each(|_| {
            edit(&mut doc);
            drop(combination_results_cached(&doc).unwrap());
        });
        let (solve_p50, solve) = timing_ms(
            || {
                edit(&mut doc);
                let results = combination_results_cached(&doc).unwrap();
                std::hint::black_box(envelope(&doc, &results).len());
            },
            10,
        );
        eprintln!("[latency] frame-envelope/commit p50_ms={solve_p50:.3} p95_ms={solve:.3} budget_ms={:.1}", 400.0 * scale);
        assert!(solve < 400.0 * scale, "frame combinations and envelope of an edited frame: p95 {solve:.1} ms exceeds the 400 ms commit budget");
        let (repeat_p50, repeat) = timing_ms(|| drop(std::hint::black_box(combination_results_cached(&doc).unwrap())), 50);
        eprintln!("[latency] frame-envelope/repaint p50_ms={repeat_p50:.3} p95_ms={repeat:.3} budget_ms={:.1}", 12.0 * scale);
        assert!(repeat < 12.0 * scale && repeat < solve, "a cached repaint takes {repeat:.3} ms against {solve:.1} ms solved");
    }
}
