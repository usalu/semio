//! 🩹️ Minimal remedies: one click writes the smallest value of one typed member field that makes a failing check pass, found by re-evaluating the member (never a safety factor guess). See [`super`] and EN 1992-1-1 §6, §7 and §5.10.

use super::*;

const EXPANSIONS: usize = 7;
const MAX_BISECTIONS: usize = 24;
const JOINT_SCALES: [f64; 4] = [1.25, 1.5, 2.0, 3.0];

/// ⏱️ Most candidate members one single-field search evaluates: seven expansions, at most [`MAX_BISECTIONS`] bisections and the final snapped value.
pub(crate) const SEARCH_BOUND: usize = EXPANSIONS + MAX_BISECTIONS + 1;

/// ⏱️ Most uncached candidate evaluations one failing check may spend: three single-field searches (the longest plan, shear), so the single-field phase of every check always fits and only the joint phase can run dry (the check then carries a notice). Cached candidates cost nothing, and every check owns its budget, so no check can starve another.
pub(crate) const CHECK_BUDGET: usize = 3 * SEARCH_BOUND;

/// 🧭 Which way a field moves to make the check pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Direction {
    Raise,
    Lower,
}

/// 🔧 A typed member field a remedy may write; each has a set-field rule of the mutation crate.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Field {
    Height,
    Width,
    LayerCount(String),
    LayerDiameter(String),
    StirrupSpacing,
    StirrupDiameter,
    PunchingAsw,
    PrestressForce,
    BucklingLength,
    BearingWidth,
    StudRowSpacingRadial(String),
    StudRowDiameter(String),
}

impl Field {
    pub(super) fn get(&self, m: &RcMember) -> Option<f64> {
        match self {
            Field::Height => Some(m.height),
            Field::Width => Some(m.width),
            Field::LayerCount(id) => m.longitudinal.iter().find(|x| &x.id == id).map(|x| x.count as f64),
            Field::LayerDiameter(id) => m.longitudinal.iter().find(|x| &x.id == id).map(|x| x.diameter),
            Field::StirrupSpacing => m.stirrups.as_ref().map(|s| s.spacing),
            Field::StirrupDiameter => m.stirrups.as_ref().map(|s| s.diameter),
            Field::PunchingAsw => m.punching.as_ref().map(|p| p.asw),
            Field::PrestressForce => m.prestress.as_ref().map(|p| p.force),
            Field::BucklingLength => Some(m.buckling_length),
            Field::BearingWidth => Some(m.bearing_width),
            Field::StudRowSpacingRadial(id) => m.stud_rows.iter().find(|x| &x.id == id).map(|x| x.spacing_radial),
            Field::StudRowDiameter(id) => m.stud_rows.iter().find(|x| &x.id == id).map(|x| x.diameter),
        }
    }

    pub(super) fn set(&self, m: &mut RcMember, v: f64) {
        match self {
            Field::Height => {
                m.effective_depth += v - m.height;
                m.height = v;
            }
            Field::Width => m.width = v,
            Field::LayerCount(id) => m.longitudinal.iter_mut().filter(|x| &x.id == id).for_each(|x| x.count = v.round().max(0.0) as u32),
            Field::LayerDiameter(id) => m.longitudinal.iter_mut().filter(|x| &x.id == id).for_each(|x| x.diameter = v),
            Field::StirrupSpacing => m.stirrups.iter_mut().for_each(|s| s.spacing = v),
            Field::StirrupDiameter => m.stirrups.iter_mut().for_each(|s| s.diameter = v),
            Field::PunchingAsw => m.punching.iter_mut().for_each(|p| p.asw = v),
            Field::PrestressForce => m.prestress.iter_mut().for_each(|p| p.force = v),
            Field::BucklingLength => m.buckling_length = v,
            Field::BearingWidth => m.bearing_width = v,
            Field::StudRowSpacingRadial(id) => m.stud_rows.iter_mut().filter(|x| &x.id == id).for_each(|x| x.spacing_radial = v),
            Field::StudRowDiameter(id) => m.stud_rows.iter_mut().filter(|x| &x.id == id).for_each(|x| x.diameter = v),
        }
    }

    fn path(&self, member: &str) -> String {
        match self {
            Field::Height => member_path(member, "height"),
            Field::Width => member_path(member, "width"),
            Field::LayerCount(id) => layer_path(member, id, "count"),
            Field::LayerDiameter(id) => layer_path(member, id, "diameter"),
            Field::StirrupSpacing => member_path(member, "stirrups.spacing"),
            Field::StirrupDiameter => member_path(member, "stirrups.diameter"),
            Field::PunchingAsw => member_path(member, "punching.asw"),
            Field::PrestressForce => member_path(member, "prestress.force"),
            Field::BucklingLength => member_path(member, "bucklingLength"),
            Field::BearingWidth => member_path(member, "bearingWidth"),
            Field::StudRowSpacingRadial(id) => member_path(member, &format!("studRows[id={id}].spacingRadial")),
            Field::StudRowDiameter(id) => member_path(member, &format!("studRows[id={id}].diameter")),
        }
    }

    fn quantity(&self, v: f64) -> Quantity {
        match self {
            Field::LayerCount(_) => Quantity::new(QuantityKind::Dimensionless, v),
            Field::PunchingAsw => area_m2(v),
            Field::PrestressForce => force_n(v),
            _ => length_m(v),
        }
    }

    fn step(&self) -> f64 {
        match self {
            Field::LayerCount(_) => 1.0,
            Field::PunchingAsw => 1.0e-6,
            Field::PrestressForce => 1.0e3,
            _ => 1.0e-3,
        }
    }

    fn name(&self) -> (&'static str, &'static str) {
        match self {
            Field::Height => ("section height", "Querschnittshöhe"),
            Field::Width => ("section width", "Querschnittsbreite"),
            Field::LayerCount(_) => ("bar count", "Stabanzahl"),
            Field::LayerDiameter(_) => ("bar diameter", "Stabdurchmesser"),
            Field::StirrupSpacing => ("stirrup spacing", "Bügelabstand"),
            Field::StirrupDiameter => ("stirrup diameter", "Bügeldurchmesser"),
            Field::PunchingAsw => ("punching reinforcement", "Durchstanzbewehrung"),
            Field::PrestressForce => ("prestressing force", "Vorspannkraft"),
            Field::BucklingLength => ("buckling length", "Knicklänge"),
            Field::BearingWidth => ("bearing plate width", "Auflagerplattenbreite"),
            Field::StudRowSpacingRadial(_) => ("stud row radial spacing", "Dübelreihe radialer Abstand"),
            Field::StudRowDiameter(_) => ("stud diameter", "Dübeldurchmesser"),
        }
    }

    fn display(&self, v: f64) -> String {
        match self {
            Field::LayerCount(_) => format!("{v:.0}"),
            Field::PunchingAsw => format!("{:.0} mm²", v * 1.0e6),
            Field::PrestressForce => format!("{:.0} kN", v / 1.0e3),
            _ => format!("{:.0} mm", v * 1.0e3),
        }
    }
}

fn tension_layer(member: &RcMember) -> Option<String> {
    member.longitudinal.iter().find(|x| x.position == semio_s_artifact_norm_en1992_model::BarPosition::Bottom || x.position == semio_s_artifact_norm_en1992_model::BarPosition::Tension).or_else(|| member.longitudinal.first()).map(|x| x.id.clone())
}

fn compression_layer(member: &RcMember) -> Option<String> {
    member.longitudinal.iter().find(|x| x.position == semio_s_artifact_norm_en1992_model::BarPosition::Top || x.position == semio_s_artifact_norm_en1992_model::BarPosition::Compression).map(|x| x.id.clone())
}

/// 🔩 Id of the active stud row with the smallest diameter, whose area the punching resistance reads.
fn weakest_row(member: &RcMember) -> Option<String> {
    member.stud_rows.iter().filter(|r| r.count > 0 && r.diameter > 0.0).min_by(|a, b| a.diameter.total_cmp(&b.diameter)).map(|r| r.id.clone())
}

/// 🔩 Id of the active stud row whose outermost stud reaches farthest, whose reach plus k d must cover the outer control perimeter u_out.
fn longest_row(member: &RcMember) -> Option<String> {
    member.stud_rows.iter().filter(|r| r.count > 0 && r.diameter > 0.0).max_by(|a, b| (a.spacing_first + f64::from(a.count - 1) * a.spacing_radial).total_cmp(&(b.spacing_first + f64::from(b.count - 1) * b.spacing_radial))).map(|r| r.id.clone())
}

/// 🔩 Id of the active stud row with the widest radial spacing, `None` without a row of two or more studs.
fn widest_row(member: &RcMember) -> Option<String> {
    member.stud_rows.iter().filter(|r| r.count > 1 && r.diameter > 0.0).max_by(|a, b| a.spacing_radial.total_cmp(&b.spacing_radial)).map(|r| r.id.clone())
}

/// 📋 The fields that can repair a check, by check id, in the order a designer would try them.
pub(super) fn plan(id: &str, member: &RcMember) -> Vec<(Field, Direction)> {
    use Direction::{Lower, Raise};
    let layer = tension_layer(member).unwrap_or_default();
    let (count, diameter) = (Field::LayerCount(layer.clone()), Field::LayerDiameter(layer.clone()));
    let has = |prefix: &str| id.starts_with(prefix);
    if has("en1992.6.1.flexure.") || has("en1992.7.2.") {
        vec![(count, Raise), (Field::Height, Raise)]
    } else if has("en1992.6.3.torsion-longitudinal-compression.") {
        compression_layer(member).map(|top| vec![(Field::LayerCount(top.clone()), Raise), (Field::LayerDiameter(top), Raise)]).unwrap_or_default()
    } else if has("en1992.6.3.torsion-longitudinal.") {
        vec![(count, Raise), (diameter, Raise)]
    } else if has("en1992.6.5.strut.") || has("en1992.6.5.node-") {
        vec![(Field::BearingWidth, Raise), (Field::Width, Raise), (Field::Height, Raise)]
    } else if has("en1992.6.2.shear.") || has("en1992.6.3.") {
        vec![(Field::StirrupSpacing, Lower), (Field::StirrupDiameter, Raise), (Field::Height, Raise)]
    } else if has("en1992.6.4.punching") {
        match weakest_row(member) {
            Some(ring) => vec![(Field::StudRowDiameter(ring), Raise), (Field::Height, Raise)],
            None => vec![(Field::PunchingAsw, Raise), (Field::Height, Raise)],
        }
    } else if has("en1992.6.4.uout.") {
        longest_row(member).map(|ring| vec![(Field::StudRowSpacingRadial(ring), Raise)]).unwrap_or_default()
    } else if has("en1992.9.4.3.studs.") {
        widest_row(member).map(|ring| vec![(Field::StudRowSpacingRadial(ring), Lower)]).unwrap_or_default()
    } else if has("en1992.7.3.crack.") || has("en1992.9.minas.") || has("en1992.6.5.tie.") {
        vec![(count, Raise), (diameter, Raise)]
    } else if has("en1992.9.maxas.") {
        vec![(count, Lower), (diameter, Lower)]
    } else if has("en1992.7.4.") {
        vec![(Field::Height, Raise)]
    } else if has("en1992.5.8.slender.") {
        vec![(Field::Height, Raise), (Field::Width, Raise), (Field::BucklingLength, Lower)]
    } else if has("en1992.1-2.nm.") || has("en1992.1-2.zone.") {
        vec![(Field::Height, Raise), (Field::Width, Raise)]
    } else if has("en1992.5.10.prestress.") {
        vec![(Field::PrestressForce, Lower)]
    } else if has("en1992.5.10.decompression.") {
        vec![(Field::PrestressForce, Raise)]
    } else if let Some(layer) = id.strip_prefix(&format!("en1992.8.4.anchorage.{}.", member.id)) {
        vec![(Field::LayerDiameter(layer.to_string()), Lower)]
    } else {
        Vec::new()
    }
}

/// 🧮 Candidate evaluations of one member, cached by the written field values so every check shares the results, and charged to the asking check against [`CHECK_BUDGET`].
struct Solver<'a> {
    doc: &'a En1992Snapshot,
    member: &'a RcMember,
    cache: std::cell::RefCell<std::collections::HashMap<(Vec<(String, u64)>, bool), std::collections::HashMap<String, bool>>>,
    used: std::cell::RefCell<std::collections::HashMap<String, usize>>,
}

impl<'a> Solver<'a> {
    fn new(doc: &'a En1992Snapshot, member: &'a RcMember) -> Self {
        Self { doc, member, cache: Default::default(), used: Default::default() }
    }

    fn passes(&self, id: &str, moves: &[(&Field, f64)]) -> bool {
        let fire = id.starts_with("en1992.1-2.");
        let key = (moves.iter().map(|(field, value)| (field.path(&self.member.id), value.to_bits())).collect::<Vec<_>>(), fire);
        let hit = self.cache.borrow().get(&key).map(|statuses| statuses.get(id).copied().unwrap_or(false));
        if let Some(hit) = hit {
            return hit;
        }
        {
            let mut used = self.used.borrow_mut();
            let spent = used.entry(id.to_string()).or_insert(0);
            if *spent >= CHECK_BUDGET {
                return false;
            }
            *spent += 1;
        }
        let mut candidate = self.member.clone();
        moves.iter().for_each(|(field, value)| field.set(&mut candidate, *value));
        if !fire {
            candidate.fire = None;
        }
        let statuses: std::collections::HashMap<String, bool> = evaluate_member_unsolved(self.doc, &candidate).into_iter().map(|r| (r.id, r.status != CheckStatus::Fail)).collect();
        let hit = statuses.get(id).copied().unwrap_or(false);
        self.cache.borrow_mut().insert(key, statuses);
        hit
    }

    /// 🎯 The smallest value (largest for [`Direction::Lower`]) of `field` that makes check `id` pass with the `fixed` fields already written; `None` when no value within reach and budget does.
    fn minimal(&self, id: &str, field: &Field, direction: Direction, fixed: &[(&Field, f64)]) -> Option<f64> {
        search(field.get(self.member)?, field.step(), matches!(field, Field::LayerCount(_)), direction, |value| {
            let mut moves = fixed.to_vec();
            moves.push((field, value));
            self.passes(id, &moves)
        })
    }

    /// 🤝 Two fields written together: the secondary one is moved by growing scales in its direction, the primary one is solved for each until the check passes.
    fn joint(&self, id: &str, planned: &[(Field, Direction)]) -> Option<Vec<(Field, Direction, f64)>> {
        planned.iter().enumerate().find_map(|(first, (primary, primary_direction))| {
            planned.iter().enumerate().filter(|(second, _)| *second != first).find_map(|(_, (secondary, secondary_direction))| {
                let current = secondary.get(self.member)?;
                JOINT_SCALES.iter().find_map(|scale| {
                    let target = scale_on_grid(secondary, current, *scale, *secondary_direction);
                    let moved = (target - current).abs() > secondary.step() * 0.5;
                    let value = moved.then(|| self.minimal(id, primary, *primary_direction, &[(secondary, target)])).flatten()?;
                    Some(vec![(primary.clone(), *primary_direction, value), (secondary.clone(), *secondary_direction, target)])
                })
            })
        })
    }
}

/// 📏 `current` moved by `scale` in `direction` and snapped outward to the field grid (whole bars for counts).
fn scale_on_grid(field: &Field, current: f64, scale: f64, direction: Direction) -> f64 {
    let step = field.step();
    let raw = match direction {
        Direction::Raise => current * scale,
        Direction::Lower => current / scale,
    };
    let snapped = match direction {
        Direction::Raise => (raw / step - 1.0e-9).ceil() * step,
        Direction::Lower => (raw / step + 1.0e-9).floor() * step,
    };
    if matches!(field, Field::LayerCount(_)) && snapped == current {
        match direction {
            Direction::Raise => current + 1.0,
            Direction::Lower => current - 1.0,
        }
    } else {
        snapped.max(step)
    }
}

/// 🔎️ The smallest value (largest for [`Direction::Lower`]) reachable from `current` on the grid `step` for which `passes` holds, found by doubling the move per try (at most seven tries, a reach of 128 times) and bisecting the last interval; `None` when nothing within reach passes. Integer fields (`discrete`) move in whole steps.
pub(crate) fn search(current: f64, step: f64, discrete: bool, direction: Direction, passes: impl Fn(f64) -> bool) -> Option<f64> {
    let next = |v: f64| match (direction, discrete) {
        (Direction::Raise, true) => (v * 2.0).ceil().max(v + 1.0),
        (Direction::Raise, false) => v * 2.0,
        (Direction::Lower, true) => (v / 2.0).floor().min(v - 1.0),
        (Direction::Lower, false) => v / 2.0,
    };
    let (mut fail, mut pass) = (current, current);
    let mut found = false;
    for _ in 0..EXPANSIONS {
        fail = pass;
        pass = next(pass);
        if pass <= step * 0.5 || (discrete && pass < 1.0) {
            return None;
        }
        if passes(pass) {
            found = true;
            break;
        }
    }
    if !found {
        return None;
    }
    for _ in 0..MAX_BISECTIONS {
        if (pass - fail).abs() <= step {
            break;
        }
        let mid = if discrete { ((fail + pass) / 2.0).floor() } else { (fail + pass) / 2.0 };
        if mid == fail || mid == pass {
            break;
        }
        if passes(mid) {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    let snapped = match direction {
        Direction::Raise => (pass / step - 1.0e-9).ceil() * step,
        Direction::Lower => (pass / step + 1.0e-9).floor() * step,
    };
    Some(if snapped > 0.0 && passes(snapped) { snapped } else { pass })
}

fn remedy_of(doc: &En1992Snapshot, member: &RcMember, id: &str, field: &Field, direction: Direction, value: f64) -> Option<Remedy> {
    let current = field.get(member)?;
    let (en, de) = field.name();
    let target = member_ref(doc, member, &field.path(&member.id));
    let (from, to) = (field.display(current), field.display(value));
    let action = |verb_en: &str, verb_de: &str| l(&format!("{verb_en} the {en} of {} from {from} to {to}, the smallest value that passes {id}.", member.id), &format!("{de} von {} von {from} {verb_de} {to}, der kleinste Wert, der {id} erfüllt.", member.id));
    Some(match direction {
        Direction::Raise => Remedy::at_least(target, field.quantity(current), field.quantity(value), action("Raise", "auf mindestens")),
        Direction::Lower => Remedy::at_most(target, field.quantity(current), field.quantity(value), action("Lower", "auf höchstens")),
    })
}

/// 🔔 The typed "no single change suffices" notice: a non-applicable choice remedy whose options are the governing field paths, with the explanation in English and German.
fn notice_of(member: &RcMember, result: &CheckResult, planned: &[(Field, Direction)]) -> Remedy {
    let names: Vec<(&str, &str)> = planned.iter().map(|(field, _)| field.name()).collect();
    let (en, de) = (names.iter().map(|n| n.0).collect::<Vec<_>>().join(", "), names.iter().map(|n| n.1).collect::<Vec<_>>().join(", "));
    let options = if planned.is_empty() { vec![result.subject.path.clone()] } else { planned.iter().map(|(field, _)| field.path(&member.id)).collect() };
    let governing = if planned.is_empty() { l("the inputs of this check", "die Eingaben dieser Prüfung") } else { l(&en, &de) };
    let action = l(&format!("No single change suffices for {}: revise {} of {} together or rethink the design.", result.id, governing.en, member.id), &format!("Keine einzelne Änderung genügt für {}: {} von {} gemeinsam überarbeiten oder den Entwurf überdenken.", result.id, governing.de, member.id));
    Remedy { applicable: false, ..Remedy::one_of(result.subject.clone(), options, action) }
}

/// 🩹️ Adds the minimal one-click remedy of every planned field to every failing check (a remedy that already targets the same field is replaced); when no single field passes, the planned fields are written together as a joint remedy (every `options` entry names the paths that belong together); when even that fails, a typed notice names the governing fields. Every failing check leaves with an applicable remedy or a notice.
pub(super) fn attach_minimal_remedies(doc: &En1992Snapshot, member: &RcMember, results: &mut [CheckResult]) {
    let solver = Solver::new(doc, member);
    let (solved, _) = record_ndp_reads(|| {
        results
            .iter()
            .enumerate()
            .filter(|(_, r)| r.status == CheckStatus::Fail)
            .map(|(index, r)| {
                let planned = plan(&r.id, member);
                let single: Vec<Remedy> = planned
                    .iter()
                    .filter_map(|(field, direction)| solver.minimal(&r.id, field, *direction, &[]).and_then(|value| remedy_of(doc, member, &r.id, field, *direction, value)))
                    .collect();
                let mut merged: Vec<Remedy> = r.remedies.iter().filter(|old| !single.iter().any(|new| new.target.path == old.target.path)).cloned().collect();
                merged.extend(single);
                if !merged.iter().any(|x| x.applicable) {
                    match solver.joint(&r.id, &planned) {
                        Some(together) => {
                            let paths: Vec<String> = together.iter().map(|(field, _, _)| field.path(&member.id)).collect();
                            merged.retain(|old| !paths.contains(&old.target.path));
                            merged.extend(together.iter().filter_map(|(field, direction, value)| {
                                let mut remedy = remedy_of(doc, member, &r.id, field, *direction, *value)?;
                                remedy.options = paths.clone();
                                remedy.action = l(&format!("{} Written together with {}.", remedy.action.en, paths.join(", ")), &format!("{} Gemeinsam mit {} zu schreiben.", remedy.action.de, paths.join(", ")));
                                Some(remedy)
                            }));
                        }
                        None => merged.push(notice_of(member, r, &planned)),
                    }
                }
                (index, merged)
            })
            .collect::<Vec<_>>()
    });
    for (index, remedies) in solved {
        results[index].remedies = remedies;
    }
}
