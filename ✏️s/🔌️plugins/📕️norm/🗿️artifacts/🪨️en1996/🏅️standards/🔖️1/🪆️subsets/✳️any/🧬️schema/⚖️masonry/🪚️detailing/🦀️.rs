//! 🪚️ Document-level detailing of EN 1996-1-1 Section 8 and the structural system of EN 1998-1 9.3: chases and recesses (8.6) and movement joints are rows of the typed collections `chases` and `movementJoints`, linked to a wall by `wallId`, so they are editable, drawn in the scene and verified against the wall they belong to. See [`super`].

use super::*;
use semio_s_artifact_norm_en1996_model::{BracingSystem, ChaseKind, MovementJoint, WallBar, WallChase, WallLintel, WallOpening};

/// 📐️ Extent `(x0, x1, y0, y1)` of a chase on the wall face: a vertical chase runs `length` up from `y`, a horizontal one `length` along from `x`, a recess spans width by length.
pub fn chase_extent(chase: &WallChase) -> (f64, f64, f64, f64) {
    match chase.kind {
        ChaseKind::Horizontal => (chase.x_m, chase.x_m + chase.length_m, chase.y_m, chase.y_m + chase.width_m),
        ChaseKind::Vertical | ChaseKind::Recess => (chase.x_m, chase.x_m + chase.width_m, chase.y_m, chase.y_m + chase.length_m),
    }
}

/// 🏷️ English and German name of a chase kind.
pub fn kind_words(kind: ChaseKind) -> (&'static str, &'static str) {
    match kind {
        ChaseKind::Vertical => ("Vertical chase", "Senkrechter Schlitz"),
        ChaseKind::Horizontal => ("Horizontal chase", "Waagerechter Schlitz"),
        ChaseKind::Recess => ("Recess", "Aussparung"),
    }
}

/// 🔑️ The input key of the M-95 catalogue entry for a bracing system.
pub fn system_key(system: BracingSystem) -> &'static str {
    match system {
        BracingSystem::Unreinforced => "unreinforced",
        BracingSystem::Confined => "confined",
        BracingSystem::Reinforced => "reinforced",
    }
}

fn rank(system: BracingSystem) -> u8 {
    match system {
        BracingSystem::Unreinforced => 0,
        BracingSystem::Confined => 1,
        BracingSystem::Reinforced => 2,
    }
}

const SYSTEMS: [(BracingSystem, &str); 3] = [(BracingSystem::Unreinforced, "Unreinforced"), (BracingSystem::Confined, "Confined"), (BracingSystem::Reinforced, "Reinforced")];

/// 🧱️ The bracing system of the building: the weakest system declared by a wall (`bracingSystem`), unreinforced when there is no wall.
pub fn structural_system(walls: &[MasonryWall]) -> BracingSystem {
    walls.iter().map(|w| w.bracing_system).min_by_key(|s| rank(*s)).unwrap_or(BracingSystem::Unreinforced)
}

/// 🌍️ The behaviour-factor case of the document (M-95) with the declared bracing system and the weakest declared diaphragm stiffness; its q range is information, an entered q is verified by the design case rows.
pub fn system_case(annex: AnnexChoice, walls: &[MasonryWall]) -> Option<Case> {
    let case = (!walls.is_empty()).then(|| Case::new("system", annex).bound("building", "walls", loc("Building", "Gebäude")).with_s("system", system_key(structural_system(walls))))?;
    let stiffness = walls.iter().map(|w| w.diaphragm_stiffness_n_per_m).filter(|k| *k > 0.0).fold(f64::INFINITY, f64::min);
    Some(if stiffness.is_finite() { case.with_n("k_diaphragm_n_m", stiffness) } else { case })
}

/// 🧱️ The bracing system the elements of a wall can provide: reinforced when the wall is reinforced, confined when it carries confining elements, else unreinforced.
pub fn provided_system(wall: &MasonryWall) -> BracingSystem {
    if wall.reinforced {
        BracingSystem::Reinforced
    } else if !wall.confined.is_empty() {
        BracingSystem::Confined
    } else {
        BracingSystem::Unreinforced
    }
}

fn chase_subject(chase: &WallChase, leaf: &str) -> SubjectRef {
    let (en, de) = kind_words(chase.kind);
    SubjectRef::new(&chase.wall_id, format!("chases[id={}].{leaf}", chase.id), loc(&format!("{en} {}", chase.id), &format!("{de} {}", chase.id)))
}

fn joint_subject(joint: &MovementJoint, leaf: &str) -> SubjectRef {
    SubjectRef::new(&joint.wall_id, format!("movementJoints[id={}].{leaf}", joint.id), loc(&format!("Movement joint {}", joint.id), &format!("Dehnfuge {}", joint.id)))
}

const CALCULATION_EN: &str = "outside the table, so a calculation is required (EN 1996-1-1 8.6.2)";
const CALCULATION_DE: &str = "außerhalb der Tabelle, daher ist ein Nachweis erforderlich (EN 1996-1-1 8.6.2)";

fn gap_m(a0: f64, a1: f64, b0: f64, b1: f64) -> f64 {
    (b0 - a1).max(a0 - b1)
}

fn push_chase(report: &mut CheckReport, annex: AnnexChoice, wall: &MasonryWall, chase: &WallChase, siblings: &[&WallChase]) {
    let part = "EN 1996-1-1";
    let (x0, x1, y0, y1) = chase_extent(chase);
    let limits = chase_limits(chase.kind, wall.thickness_m, chase.length_m, y1, wall.height_m, annex);
    let limit = limits.map_or(0.0, |l| l.depth_m);
    let table = limits.map_or("8.1/8.2", |l| l.table);
    let (en, de) = kind_words(chase.kind);
    let remaining = wall.thickness_m - chase.depth_m;
    let verdict = |ok: bool| (if ok { "within the table" } else { CALCULATION_EN }, if ok { "innerhalb der Tabelle" } else { CALCULATION_DE });
    let v = verdict(chase.depth_m <= limit + 1e-9);
    let mut depth = CheckResult::assess(format!("en1996.8.6.chase-depth.{}", chase.id), part, clause(part, &format!("§8.6.2 Table {table}"), "8.6.2"), chase_subject(chase, "depthM"), loc("Chase or recess depth within the table without calculation", "Schlitz- oder Aussparungstiefe ohne Nachweis zulässig"))
        .utilization(Quantity::length_m(chase.depth_m), Quantity::length_m(limit.max(1e-9)))
        .annex(annex)
        .status(CheckStatus::Pass)
        .warn_above(1.0)
        .explanation(loc(
            &format!("{en} in {}: depth {:.3} m against {limit:.3} m of Table {table} for t = {:.3} m, remaining thickness {remaining:.3} m: {}. Unverified registry rows ndp.en1996.chase.*.", wall.id, chase.depth_m, wall.thickness_m, v.0),
            &format!("{de} in {}: Tiefe {:.3} m gegenüber {limit:.3} m nach Tabelle {table} bei t = {:.3} m, Restdicke {remaining:.3} m: {}. Ungeprüfte Registerzeilen ndp.en1996.chase.*.", wall.id, chase.depth_m, wall.thickness_m, v.1),
        ))
        .ndp("ndp.en1996.chase.t-lower");
    if chase.depth_m > limit + 1e-9 {
        if limit > 0.0 {
            depth = depth.remedy(Remedy::at_most(chase_subject(chase, "depthM"), Quantity::length_m(chase.depth_m), Quantity::length_m(limit), loc("Reduce the chase depth to the tabulated value.", "Schlitztiefe auf den tabellierten Wert verringern.")));
        }
        if let Some(t) = chase_min_thickness_m(chase.kind, chase.depth_m, 0.0, chase.length_m, y1, wall.height_m, annex) {
            depth = depth.remedy(Remedy::at_least(SubjectRef::new(&wall.id, wall_path(wall, "thicknessM"), loc(&wall.label_en, &wall.label_de)), Quantity::length_m(wall.thickness_m), Quantity::length_m(t), loc(&format!("Thicken the wall to at least {:.0} mm so that this chase needs no calculation.", t * 1000.0), &format!("Wand auf mindestens {:.0} mm verdicken, damit dieser Schlitz keinen Nachweis braucht.", t * 1000.0))));
        }
    }
    report.push(depth.build());
    if let Some(max_width) = limits.and_then(|l| l.width_m) {
        let v = verdict(chase.width_m <= max_width + 1e-9);
        let mut width = CheckResult::assess(format!("en1996.8.6.chase-width.{}", chase.id), part, clause(part, "§8.6.2 Table 8.1", "8.6.2"), chase_subject(chase, "widthM"), loc("Chase or recess width within the table without calculation", "Schlitz- oder Aussparungsbreite ohne Nachweis zulässig"))
            .utilization(Quantity::length_m(chase.width_m), Quantity::length_m(max_width.max(1e-9)))
            .annex(annex)
            .status(CheckStatus::Pass)
            .warn_above(1.0)
            .explanation(loc(&format!("{en} in {}: width {:.3} m against {max_width:.3} m of Table 8.1 for t = {:.3} m: {}.", wall.id, chase.width_m, wall.thickness_m, v.0), &format!("{de} in {}: Breite {:.3} m gegenüber {max_width:.3} m nach Tabelle 8.1 bei t = {:.3} m: {}.", wall.id, chase.width_m, wall.thickness_m, v.1)))
            .ndp("ndp.en1996.chase.t-lower");
        if chase.width_m > max_width + 1e-9 {
            width = width.remedy(Remedy::at_most(chase_subject(chase, "widthM"), Quantity::length_m(chase.width_m), Quantity::length_m(max_width), loc("Reduce the chase width to the tabulated value.", "Schlitzbreite auf den tabellierten Wert verringern.")));
            if let Some(t) = chase_min_thickness_m(chase.kind, 0.0, chase.width_m, chase.length_m, y1, wall.height_m, annex) {
                width = width.remedy(Remedy::at_least(SubjectRef::new(&wall.id, wall_path(wall, "thicknessM"), loc(&wall.label_en, &wall.label_de)), Quantity::length_m(wall.thickness_m), Quantity::length_m(t), loc(&format!("Thicken the wall to at least {:.0} mm so that this width needs no calculation.", t * 1000.0), &format!("Wand auf mindestens {:.0} mm verdicken, damit diese Breite keinen Nachweis braucht.", t * 1000.0))));
            }
        }
        report.push(width.build());
    }
    let neighbours = siblings
        .iter()
        .filter(|s| s.id != chase.id && (s.kind == ChaseKind::Horizontal) == (chase.kind == ChaseKind::Horizontal))
        .map(|s| {
            let (s0, s1, _, _) = chase_extent(s);
            (format!("{} {}", kind_words(s.kind).0, s.id), format!("{} {}", kind_words(s.kind).1, s.id), chase_gap_m(chase.kind, s.kind, chase.width_m.max(s.width_m), chase.length_m.max(s.length_m), annex), s0, s1)
        })
        .chain(wall.openings.iter().map(|o| (format!("Opening {}", o.id), format!("Öffnung {}", o.id), chase_opening_gap_m(chase.kind, chase.width_m, annex), o.position_m, o.position_m + o.width_m)))
        .map(|(en_name, de_name, need, n0, n1)| (en_name, de_name, need, gap_m(x0, x1, n0, n1), n0, n1))
        .max_by(|a, b| (a.2 / a.3.max(1e-9)).total_cmp(&(b.2 / b.3.max(1e-9))));
    if let Some((en_name, de_name, need, gap, n0, n1)) = neighbours {
        let v = verdict(gap + 1e-9 >= need);
        let mut distance = CheckResult::assess(format!("en1996.8.6.chase-distance.{}", chase.id), part, clause(part, "§8.6.2 Tables 8.1, 8.2", "8.6.2"), chase_subject(chase, "xM"), loc("Distance of the chase or recess to neighbouring chases and openings", "Abstand des Schlitzes oder der Aussparung zu Nachbarschlitzen und Öffnungen"))
            .utilization(Quantity::length_m(need), Quantity::length_m(gap.max(1e-9)))
            .annex(annex)
            .status(CheckStatus::Pass)
            .warn_above(1.0)
            .explanation(loc(&format!("{en} {}: horizontal distance {gap:.3} m to {en_name} against the required {need:.3} m: {}.", chase.id, v.0), &format!("{de} {}: waagerechter Abstand {gap:.3} m zu {de_name} gegenüber geforderten {need:.3} m: {}.", chase.id, v.1)))
            .ndp("ndp.en1996.chase.v.spacing");
        if gap + 1e-9 < need {
            let (current, target, to_right) = if 0.5 * (n0 + n1) < 0.5 * (x0 + x1) { (chase.x_m, ceil_to(n1 + need, 1e-3), true) } else { (chase.x_m, floor_to((n0 - need - (x1 - x0)).max(0.0), 1e-3), false) };
            let (en_text, de_text) = ("Move the chase away from its neighbour.", "Schlitz vom Nachbarn wegschieben.");
            distance = distance.remedy(if to_right { Remedy::at_least(chase_subject(chase, "xM"), Quantity::length_m(current), Quantity::length_m(target), loc(en_text, de_text)) } else { Remedy::at_most(chase_subject(chase, "xM"), Quantity::length_m(current), Quantity::length_m(target), loc(en_text, de_text)) });
        }
        report.push(distance.build());
    }
    let (x0, x1, y0, y1) = chase_extent(chase);
    let (u_x, u_y) = (x1 / wall.length_m.max(1e-9), y1 / wall.height_m.max(1e-9));
    let mut extent = CheckResult::assess(format!("en1996.8.6.chase-extent.{}", chase.id), part, clause(part, "§8.6", "8.6"), chase_subject(chase, if u_x >= u_y { "xM" } else { "yM" }), loc("Chase or recess lies within the wall face", "Schlitz oder Aussparung liegt innerhalb der Wandfläche"))
        .utilization(Quantity::new(QuantityKind::Dimensionless, u_x.max(u_y).max(if x0 < 0.0 || y0 < 0.0 { 2.0 } else { 0.0 })), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .annex(annex)
        .explanation(loc(&format!("x from {x0:.3} to {x1:.3} m of {:.3} m, y from {y0:.3} to {y1:.3} m of {:.3} m.", wall.length_m, wall.height_m), &format!("x von {x0:.3} bis {x1:.3} m von {:.3} m, y von {y0:.3} bis {y1:.3} m von {:.3} m.", wall.length_m, wall.height_m)));
    if x1 > wall.length_m {
        extent = extent.remedy(Remedy::at_most(chase_subject(chase, "xM"), Quantity::length_m(chase.x_m), Quantity::length_m(floor_to((chase.x_m - (x1 - wall.length_m)).max(0.0), 1e-3)), loc("Move the chase into the wall.", "Schlitz in die Wand verschieben.")));
    }
    if y1 > wall.height_m {
        extent = extent.remedy(Remedy::at_most(chase_subject(chase, "yM"), Quantity::length_m(chase.y_m), Quantity::length_m(floor_to((chase.y_m - (y1 - wall.height_m)).max(0.0), 1e-3)), loc("Lower the chase into the wall.", "Schlitz in die Wand absenken.")));
    }
    if x0 < 0.0 {
        extent = extent.remedy(Remedy::at_least(chase_subject(chase, "xM"), Quantity::length_m(chase.x_m), Quantity::length_m(0.0), loc("Move the chase into the wall.", "Schlitz in die Wand verschieben.")));
    }
    if y0 < 0.0 {
        extent = extent.remedy(Remedy::at_least(chase_subject(chase, "yM"), Quantity::length_m(chase.y_m), Quantity::length_m(0.0), loc("Raise the chase into the wall.", "Schlitz in die Wand anheben.")));
    }
    report.push(extent.build());
}

fn push_chase_total(report: &mut CheckReport, annex: AnnexChoice, wall: &MasonryWall, chases: &[WallChase]) {
    let part = "EN 1996-1-1";
    let vertical: Vec<&WallChase> = chases.iter().filter(|c| c.wall_id == wall.id && c.kind != ChaseKind::Horizontal).collect();
    if vertical.is_empty() {
        return;
    }
    let sum: f64 = vertical.iter().map(|c| c.width_m).sum();
    let limit = chase_cumulative_width_m(wall.length_m, annex);
    let subject = SubjectRef::new(&wall.id, wall_path(wall, "lengthM"), loc(&wall.label_en, &wall.label_de));
    let ok = sum <= limit + 1e-9;
    let mut check = CheckResult::assess(format!("en1996.8.6.chase-cumulative.{}", wall.id), part, clause(part, "§8.6.2 Table 8.1", "8.6.2"), subject.clone(), loc("Cumulative width of vertical chases and recesses within the table without calculation", "Gesamtbreite senkrechter Schlitze und Aussparungen ohne Nachweis zulässig"))
        .utilization(Quantity::length_m(sum), Quantity::length_m(limit.max(1e-9)))
        .annex(annex)
        .status(CheckStatus::Pass)
        .warn_above(1.0)
        .explanation(loc(
            &format!("{} vertical chases and recesses in {}: total width {sum:.3} m against {limit:.3} m: {}.", vertical.len(), wall.id, if ok { "within the table" } else { CALCULATION_EN }),
            &format!("{} senkrechte Schlitze und Aussparungen in {}: Gesamtbreite {sum:.3} m gegenüber {limit:.3} m: {}.", vertical.len(), wall.id, if ok { "innerhalb der Tabelle" } else { CALCULATION_DE }),
        ))
        .ndp("ndp.en1996.chase.width-ratio");
    if !ok {
        check = check.remedy(Remedy::at_least(subject, Quantity::length_m(wall.length_m), Quantity::length_m(ceil_to(sum / chase_cumulative_width_m(1.0, annex), 1e-3)), loc("Lengthen the wall or narrow the chases so that their total width stays within the table.", "Wand verlängern oder Schlitze verschmälern, damit die Gesamtbreite in der Tabelle bleibt.")));
    }
    report.push(check.build());
}

fn push_joints(report: &mut CheckReport, annex: AnnexChoice, wall: &MasonryWall, joints: &[&MovementJoint]) {
    let part = "EN 1996-2";
    let limit = joint_spacing_limit_m(wall.unit_material);
    let mut at: Vec<(f64, bool, Option<usize>)> = vec![(0.0, false, None)];
    let mut ordered: Vec<usize> = (0..joints.len()).collect();
    ordered.sort_by(|a, b| joints[*a].position_m.total_cmp(&joints[*b].position_m));
    at.extend(ordered.iter().map(|i| (joints[*i].position_m.clamp(0.0, wall.length_m), true, Some(*i))));
    at.push((wall.length_m, false, None));
    let gaps: Vec<f64> = at.windows(2).map(|w| w[1].0 - w[0].0).collect();
    let widest = gaps.iter().copied().fold(0.0, f64::max);
    let subject = SubjectRef::new(&wall.id, wall_path(wall, "lengthM"), loc(&wall.label_en, &wall.label_de));
    let mut check = CheckResult::assess(format!("en1996.8.5.joint-spacing.{}", wall.id), part, clause(part, "§2.3", "2.3"), subject, loc("Movement joint spacing along the wall", "Dehnfugenabstand entlang der Wand"))
        .utilization(Quantity::length_m(widest), Quantity::length_m(limit))
        .annex(annex)
        .explanation(loc(
            &format!("{} joints in {}; widest panel {widest:.2} m against the recommended {limit:.0} m. Unverified NDP basis: m52.joints.", joints.len(), wall.id),
            &format!("{} Dehnfugen in {}; breitestes Feld {widest:.2} m gegenüber empfohlenen {limit:.0} m. Ungeprüfte NDP-Grundlage: m52.joints.", joints.len(), wall.id),
        ))
        .status(CheckStatus::Pass)
        .warn_above(1.0)
        .ndp("ndp.en1996.m52.joints");
    if widest > limit {
        let supports: Vec<numerical_models::Support> = at.iter().map(|(x, movable, _)| numerical_models::Support { at: *x, movable: *movable }).collect();
        match numerical_models::relocation(&supports, &vec![limit; gaps.len()]) {
            Some((k, to)) => {
                let joint = joints[at[k].2.unwrap_or(0)];
                let from = at[k].0;
                check = check.remedy(Remedy::exactly(joint_subject(joint, "positionM"), Quantity::length_m(from), Quantity::length_m(to), loc(&format!("Move the joint from x = {from:.3} m to x = {to:.3} m so that no panel exceeds {limit:.0} m."), &format!("Dehnfuge von x = {from:.3} m auf x = {to:.3} m verschieben, damit kein Feld {limit:.0} m überschreitet."))));
            }
            None => {
                let need = (wall.length_m / limit).ceil() as usize - 1;
                check = check.remedy(Remedy::at_least(SubjectRef::new(&wall.id, "movementJoints", loc(&wall.label_en, &wall.label_de)), Quantity::new(QuantityKind::Dimensionless, joints.len() as f64), Quantity::new(QuantityKind::Dimensionless, need as f64), loc(&format!("Add movement joints: at least {need} for a {:.1} m wall with panels of at most {limit:.0} m.", wall.length_m), &format!("Dehnfugen ergänzen: mindestens {need} für eine {:.1} m lange Wand mit Feldern von höchstens {limit:.0} m.", wall.length_m))));
            }
        }
    }
    report.push(check.build());
    for joint in joints {
        if joint.position_m <= 0.0 || joint.position_m >= wall.length_m {
            let inside = joint.position_m.clamp(0.001, (wall.length_m - 0.001).max(0.001));
            report.push(
                CheckResult::assess(format!("en1996.8.5.joint-position.{}", joint.id), part, clause(part, "§2.3", "2.3"), joint_subject(joint, "positionM"), loc("Movement joint lies inside the wall", "Dehnfuge liegt innerhalb der Wand"))
                    .utilization(Quantity::length_m(joint.position_m.abs().max(wall.length_m - joint.position_m).max(1e-9)), Quantity::length_m(wall.length_m))
                    .annex(annex)
                    .explanation(loc(&format!("x = {:.3} m on a {:.3} m wall.", joint.position_m, wall.length_m), &format!("x = {:.3} m an einer {:.3} m langen Wand.", joint.position_m, wall.length_m)))
                    .remedy(Remedy::exactly(joint_subject(joint, "positionM"), Quantity::length_m(joint.position_m), Quantity::length_m(inside), loc("Move the joint into the wall.", "Dehnfuge in die Wand verschieben.")))
                    .build(),
            );
        }
    }
}

fn push_bracing(report: &mut CheckReport, annex: AnnexChoice, wall: &MasonryWall) {
    let (declared, provided) = (wall.bracing_system, provided_system(wall));
    let part = "EN 1998-1";
    let mut check = CheckResult::assess(format!("en1996.9.3.bracing-system.{}", wall.id), part, clause(part, "§9.2.1", "9.2.1"), SubjectRef::new(&wall.id, wall_path(wall, "bracingSystem"), loc(&wall.label_en, &wall.label_de)), loc("Declared bracing system is provided by the wall", "Angegebenes Aussteifungssystem wird von der Wand geleistet"))
        .utilization(Quantity::new(QuantityKind::Dimensionless, f64::from(rank(declared) + 1)), Quantity::new(QuantityKind::Dimensionless, f64::from(rank(provided) + 1)))
        .annex(annex)
        .explanation(loc(
            &format!("{} declares {} masonry; the wall provides {} (reinforced: {}, confining elements: {}).", wall.id, system_key(declared), system_key(provided), wall.reinforced, wall.confined.len()),
            &format!("{} gibt {} Mauerwerk an; die Wand leistet {} (bewehrt: {}, Einfassungselemente: {}).", wall.id, system_key(declared), system_key(provided), wall.reinforced, wall.confined.len()),
        ));
    if rank(declared) > rank(provided) {
        check = check.remedy(Remedy::one_of(SubjectRef::new(&wall.id, wall_path(wall, "bracingSystem"), loc(&wall.label_en, &wall.label_de)), SYSTEMS.iter().filter(|(s, _)| rank(*s) <= rank(provided)).map(|(_, name)| (*name).to_string()).collect(), loc("Declare a system the wall provides, or add the confining elements or reinforcement.", "Ein von der Wand geleistetes System angeben oder Einfassungselemente bzw. Bewehrung ergänzen.")));
    }
    report.push(check.build());
}

/// 🪟️ The part of an opening `[x0, x1]` that no lintel clear span `[start, end]` covers.
pub fn uncovered_length_m(x0: f64, x1: f64, start: f64, end: f64) -> f64 {
    (start.min(x1) - x0).max(0.0) + (x1 - end.max(x0)).max(0.0)
}

/// 🛤️ The lintel that covers an opening best: the least uncovered length, then the longest span.
pub fn covering_lintel<'a>(wall: &'a MasonryWall, opening: &WallOpening) -> Option<&'a WallLintel> {
    let (x0, x1) = (opening.position_m, opening.position_m + opening.width_m);
    wall.lintels.iter().min_by(|a, b| {
        let gap = |l: &WallLintel| uncovered_length_m(x0, x1, l.position_m, l.position_m + l.span_m);
        gap(a).total_cmp(&gap(b)).then(b.span_m.total_cmp(&a.span_m))
    })
}

fn push_lintel_cover(report: &mut CheckReport, annex: AnnexChoice, wall: &MasonryWall) {
    let part = "EN 1996-1-1";
    for opening in &wall.openings {
        let (x0, x1) = (opening.position_m, opening.position_m + opening.width_m);
        let subject = SubjectRef::new(&wall.id, format!("walls[id={}].openings[id={}].widthM", wall.id, opening.id), loc(&wall.label_en, &wall.label_de));
        let id = format!("en1996.5.5.lintel-cover.{}.{}", wall.id, opening.id);
        let title = loc("Opening is spanned by a lintel", "Öffnung wird von einem Sturz überspannt");
        let Some(lintel) = covering_lintel(wall, opening) else {
            report.push(CheckResult::assess(id, part, clause(part, "§5.5", "5.5"), subject, title).annex(annex).not_applicable(loc(&format!("{} has no lintel row: declare the lintel that spans opening {}.", wall.id, opening.id), &format!("{} hat keine Sturzzeile: den Sturz deklarieren, der Öffnung {} überspannt.", wall.id, opening.id))).build());
            continue;
        };
        let (start, end) = (lintel.position_m, lintel.position_m + lintel.span_m);
        let missing = uncovered_length_m(x0, x1, start, end);
        let u = if missing <= 1e-9 { opening.width_m / lintel.span_m.max(1e-9) } else { 1.0 + missing / opening.width_m.max(1e-9) };
        let path = |leaf: &str| format!("walls[id={}].lintels[id={}].{leaf}", wall.id, lintel.id);
        let lintel_subject = |leaf: &str| SubjectRef::new(&lintel.id, path(leaf), loc(&format!("Lintel {}", lintel.id), &format!("Sturz {}", lintel.id)));
        let mut check = CheckResult::assess(id, part, clause(part, "§5.5", "5.5"), subject, title)
            .utilization(Quantity::new(QuantityKind::Dimensionless, u), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .annex(annex)
            .explanation(loc(
                &format!("Opening {} spans x = {x0:.3} to {x1:.3} m; the best lintel {} spans {start:.3} to {end:.3} m and leaves {missing:.3} m uncovered.", opening.id, lintel.id),
                &format!("Öffnung {} liegt bei x = {x0:.3} bis {x1:.3} m; der beste Sturz {} überspannt {start:.3} bis {end:.3} m und lässt {missing:.3} m unüberdeckt.", opening.id, lintel.id),
            ));
        if missing > 1e-9 {
            let from = start.min(x0);
            if start > x0 + 1e-9 {
                check = check.remedy(Remedy::exactly(lintel_subject("positionM"), Quantity::length_m(start), Quantity::length_m(floor_to(x0, 1e-3)), loc("Move the lintel to the start of the opening.", "Sturz an den Öffnungsbeginn verschieben.")));
            }
            check = check.remedy(Remedy::at_least(lintel_subject("spanM"), Quantity::length_m(lintel.span_m), Quantity::length_m(ceil_to(x1 - from, 1e-3)), loc("Lengthen the lintel span so that it spans the opening.", "Sturzspannweite so verlängern, dass sie die Öffnung überspannt.")));
        }
        report.push(check.build());
    }
}

fn push_bar_layout(report: &mut CheckReport, annex: AnnexChoice, wall: &MasonryWall, bar: &WallBar) {
    let part = "EN 1996-1-1";
    let reach = wall.length_m.max(wall.height_m);
    let (gap_u, fit_u) = (bar.diameter_m / bar.spacing_m.max(1e-9), bar.length_m / reach.max(1e-9));
    let path = |leaf: &str| format!("walls[id={}].bars[id={}].{leaf}", wall.id, bar.id);
    let subject = |leaf: &str| SubjectRef::new(&bar.id, path(leaf), loc(&format!("Bar {}", bar.id), &format!("Stab {}", bar.id)));
    let mut check = CheckResult::assess(format!("en1996.8.2.bar-layout.{}.{}", wall.id, bar.id), part, clause(part, "§8.2", "8.2"), subject(if gap_u >= fit_u { "spacingM" } else { "lengthM" }), loc("Bars fit the wall with a clear gap", "Stäbe passen in die Wand mit lichtem Abstand"))
        .utilization(Quantity::new(QuantityKind::Dimensionless, gap_u.max(fit_u)), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .annex(annex)
        .explanation(loc(
            &format!("Bar {} of {:.1} mm at {:.0} mm spacing (d/s = {gap_u:.3}), length {:.3} m of {reach:.3} m (ratio {fit_u:.3}); geometric fit only, no normative bar spacing is applied.", bar.id, bar.diameter_m * 1e3, bar.spacing_m * 1e3, bar.length_m),
            &format!("Stab {} mit {:.1} mm bei {:.0} mm Abstand (d/s = {gap_u:.3}), Länge {:.3} m von {reach:.3} m (Verhältnis {fit_u:.3}); nur geometrische Passung, kein Normwert für den Stababstand.", bar.id, bar.diameter_m * 1e3, bar.spacing_m * 1e3, bar.length_m),
        ));
    if gap_u > 1.0 {
        check = check.remedy(Remedy::at_least(subject("spacingM"), Quantity::length_m(bar.spacing_m), Quantity::length_m(ceil_to(bar.diameter_m, 1e-3)), loc("Space the bars at least one bar diameter apart.", "Stäbe mit mindestens einem Stabdurchmesser Abstand anordnen.")));
    }
    if fit_u > 1.0 {
        check = check.remedy(Remedy::at_most(subject("lengthM"), Quantity::length_m(bar.length_m), Quantity::length_m(floor_to(reach, 1e-3)), loc("Shorten the bar to the wall extent.", "Stab auf die Wandabmessung kürzen.")));
    }
    report.push(check.build());
}

/// 🪚️ The detailing checks of the document: the bracing system declared by each wall, the lintel that spans each opening, the layout of the reinforcing bars, the floor diaphragm classification and its shear distribution, chases and recesses against their wall, movement joints against the recommended spacing, and rows whose wall does not exist.
pub fn detailing_checks(annex: AnnexChoice, walls: &[MasonryWall], chases: &[WallChase], joints: &[MovementJoint]) -> CheckReport {
    let mut report = CheckReport::default();
    let by_id = |id: &str| walls.iter().find(|w| w.id == id);
    for wall in walls.iter().filter(|w| w.bracing_system != BracingSystem::Unreinforced) {
        push_bracing(&mut report, annex, wall);
    }
    for chase in chases {
        match by_id(&chase.wall_id) {
            Some(wall) => push_chase(&mut report, annex, wall, chase, &chases.iter().filter(|c| c.wall_id == wall.id).collect::<Vec<_>>()),
            None => report.push(unknown_wall(annex, "chase", &chase.id, chase_subject(chase, "wallId"), walls)),
        }
    }
    for wall in walls {
        push_chase_total(&mut report, annex, wall, chases);
    }
    for wall in walls {
        let own: Vec<&MovementJoint> = joints.iter().filter(|j| j.wall_id == wall.id).collect();
        if !own.is_empty() {
            push_joints(&mut report, annex, wall, &own);
        }
    }
    for joint in joints.iter().filter(|j| by_id(&j.wall_id).is_none()) {
        report.push(unknown_wall(annex, "joint", &joint.id, joint_subject(joint, "wallId"), walls));
    }
    for wall in walls {
        push_lintel_cover(&mut report, annex, wall);
        for bar in &wall.bars {
            push_bar_layout(&mut report, annex, wall, bar);
        }
    }
    for check in diaphragm_checks(annex, walls) {
        report.push(check);
    }
    report
}

fn unknown_wall(annex: AnnexChoice, what: &str, id: &str, subject: SubjectRef, walls: &[MasonryWall]) -> CheckResult {
    CheckResult::assess(format!("en1996.8.{what}-wall.{id}"), "EN 1996-1-1", clause("EN 1996-1-1", "§8", "8"), subject.clone(), loc("Detail refers to an existing wall", "Detail verweist auf eine vorhandene Wand"))
        .annex(annex)
        .status(CheckStatus::Fail)
        .explanation(loc("The wall id of this chase, recess or joint does not exist.", "Die Wand-Id dieses Schlitzes, dieser Aussparung oder Dehnfuge existiert nicht."))
        .remedy(Remedy::one_of(subject, walls.iter().map(|w| w.id.clone()).collect(), loc("Select an existing wall.", "Eine vorhandene Wand wählen.")))
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chase(id: &str, wall: &str, kind: ChaseKind, x: f64, y: f64, width: f64, length: f64, depth: f64) -> WallChase {
        WallChase { id: id.into(), wall_id: wall.into(), kind, x_m: x, y_m: y, width_m: width, length_m: length, depth_m: depth }
    }

    fn joint(id: &str, wall: &str, x: f64) -> MovementJoint {
        MovementJoint { id: id.into(), wall_id: wall.into(), position_m: x }
    }

    fn wall(length: f64, t: f64) -> MasonryWall {
        let mut w = semio_s_artifact_norm_en1996_model::En1996Snapshot::compliant_clay_wall().walls.remove(0);
        w.length_m = length;
        w.thickness_m = t;
        w
    }

    #[test]
    fn the_extent_of_a_chase_follows_its_kind() {
        assert_eq!(chase_extent(&chase("c", "w", ChaseKind::Horizontal, 1.0, 0.0, 0.05, 1.5, 0.03)), (1.0, 2.5, 0.0, 0.05));
        assert_eq!(chase_extent(&chase("c", "w", ChaseKind::Vertical, 1.0, 0.2, 0.05, 1.5, 0.03)), (1.0, 1.05, 0.2, 1.7));
        assert_eq!(chase_extent(&chase("c", "w", ChaseKind::Recess, 1.0, 0.2, 0.3, 0.4, 0.1)), (1.0, 1.3, 0.2, 0.6));
    }

    #[test]
    fn chase_and_joint_checks_are_owned_by_their_wall_and_keep_the_row_path() {
        let w = wall(5.0, 0.365);
        let report = detailing_checks(AnnexChoice::De, std::slice::from_ref(&w), &[chase("c1", &w.id, ChaseKind::Vertical, 0.0, 0.0, 0.05, 1.0, 0.06)], &[joint("j1", &w.id, 2.0)]);
        for (id, path) in [("en1996.8.6.chase-depth.c1", "chases[id=c1].depthM"), ("en1996.8.5.joint-position.j1", "movementJoints[id=j1].positionM")] {
            if let Some(check) = report.checks.iter().find(|c| c.id == id) {
                assert_eq!((check.subject.entity_id.as_str(), check.subject.path.as_str()), (w.id.as_str(), path), "{id}");
            }
        }
        assert!(report.checks.iter().any(|c| c.id == "en1996.8.6.chase-depth.c1"));
    }

    fn find<'a>(report: &'a CheckReport, id: &str) -> &'a CheckResult {
        report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("{id}"))
    }

    #[test]
    fn a_chase_beyond_table_8_1_needs_calculation_and_a_thin_wall_below_the_tables_is_thickened() {
        let w = wall(5.0, 0.365);
        let rows = [chase("c1", &w.id, ChaseKind::Vertical, 0.0, 0.0, 0.05, 1.0, 0.06)];
        let depth = find(&detailing_checks(AnnexChoice::De, std::slice::from_ref(&w), &rows, &[]), "en1996.8.6.chase-depth.c1").clone();
        assert!((depth.utilization - 2.0).abs() < 1e-9 && depth.status == CheckStatus::Warning && depth.remedies.len() == 1);
        let thin = wall(5.0, 0.06);
        let rows = [chase("c1", &thin.id, ChaseKind::Vertical, 0.0, 0.0, 0.05, 1.0, 0.03)];
        let depth = find(&detailing_checks(AnnexChoice::De, std::slice::from_ref(&thin), &rows, &[]), "en1996.8.6.chase-depth.c1").clone();
        assert!(depth.status == CheckStatus::Warning && depth.remedies.len() == 1);
        assert_eq!(chase_limits(ChaseKind::Vertical, 0.06, 1.0, 1.0, 2.75, AnnexChoice::De), None);
    }

    #[test]
    fn the_table_limits_follow_kind_thickness_length_and_the_lower_third() {
        let a = AnnexChoice::De;
        let v = |t: f64, top: f64| chase_limits(ChaseKind::Vertical, t, 1.0, top, 2.75, a).unwrap();
        assert_eq!((v(0.115, 2.0).depth_m, v(0.115, 2.0).width_m), (0.030, Some(0.100)));
        assert_eq!(v(0.175, 2.0).width_m, Some(0.125));
        assert_eq!(v(0.365, 2.0).width_m, Some(0.200));
        assert_eq!((v(0.365, 0.9).depth_m, v(0.365, 0.9).width_m), (0.080, Some(0.200)));
        assert_eq!(v(0.150, 0.9).depth_m, 0.030);
        let h = |t: f64, length: f64| chase_limits(ChaseKind::Horizontal, t, length, 0.0, 2.75, a).unwrap();
        assert_eq!((h(0.115, 1.0).depth_m, h(0.175, 1.0).depth_m, h(0.175, 2.0).depth_m, h(0.365, 2.0).depth_m, h(0.365, 1.25).depth_m), (0.0, 0.015, 0.0, 0.015, 0.030));
        assert_eq!(h(0.365, 1.0).width_m, None);
        assert!((chase_min_thickness_m(ChaseKind::Horizontal, 0.015, 0.0, 2.0, 0.0, 2.75, a).unwrap() - 0.301).abs() < 1e-12);
    }

    #[test]
    fn width_distance_and_cumulative_rules_warn_and_carry_moving_remedies() {
        let mut w = wall(5.0, 0.115);
        w.openings.clear();
        let rows = [chase("c1", &w.id, ChaseKind::Vertical, 1.0, 0.0, 0.15, 1.0, 0.03), chase("c2", &w.id, ChaseKind::Vertical, 1.2, 0.0, 0.05, 1.0, 0.03)];
        let report = detailing_checks(AnnexChoice::De, std::slice::from_ref(&w), &rows, &[]);
        assert!(find(&report, "en1996.8.6.chase-width.c1").utilization > 1.0 && find(&report, "en1996.8.6.chase-width.c2").utilization <= 0.5 + 1e-9);
        let distance = find(&report, "en1996.8.6.chase-distance.c2");
        assert!((distance.utilization - 0.225 / 0.05).abs() < 1e-6 && distance.status == CheckStatus::Warning && distance.remedies.len() == 1);
        let total = find(&report, &format!("en1996.8.6.chase-cumulative.{}", w.id));
        assert!((total.utilization - 0.20 / (0.13 * 5.0)).abs() < 1e-9 && total.status == CheckStatus::Pass);
    }

    #[test]
    fn a_chase_outside_the_wall_is_moved_back_and_an_unknown_wall_is_a_selectable_failure() {
        let w = wall(5.0, 0.365);
        let rows = [chase("c1", &w.id, ChaseKind::Horizontal, 4.5, 0.0, 0.05, 1.0, 0.03), chase("c2", "missing", ChaseKind::Vertical, 0.0, 0.0, 0.05, 1.0, 0.03)];
        let report = detailing_checks(AnnexChoice::De, std::slice::from_ref(&w), &rows, &[joint("j9", "missing", 1.0)]);
        let extent = report.checks.iter().find(|c| c.id == "en1996.8.6.chase-extent.c1").unwrap();
        assert!(extent.utilization > 1.0 && extent.remedies.len() == 1);
        for id in ["en1996.8.chase-wall.c2", "en1996.8.joint-wall.j9"] {
            let check = report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("{id}"));
            assert_eq!(check.remedies.len(), 1);
        }
    }

    #[test]
    fn joints_wider_than_the_limit_are_moved_by_the_single_relocation() {
        let w = wall(30.0, 0.365);
        let rows = [joint("j1", &w.id, 8.0), joint("j2", &w.id, 26.0)];
        let report = detailing_checks(AnnexChoice::De, std::slice::from_ref(&w), &[], &rows);
        let c = report.checks.iter().find(|c| c.id.starts_with("en1996.8.5.joint-spacing")).unwrap();
        assert!(c.utilization > 1.0, "{}", c.utilization);
        assert_eq!(c.remedies.len(), 1);
    }

    #[test]
    fn the_document_reports_the_behaviour_factor_range_of_the_declared_system() {
        let doc = semio_s_artifact_norm_en1996_model::En1996Snapshot::compliant_clay_wall();
        let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.reliability_class, doc.storeys, &doc.walls);
        for key in ["q-min", "q-max"] {
            let check = report.checks.iter().find(|c| c.id == format!("en1996.m-95.{key}.system")).unwrap_or_else(|| panic!("{key}"));
            assert!(check.ndps.iter().any(|id| id == "ndp.en1996.m95.q.unreinforced.min") && check.ndps.iter().any(|id| id == "ndp.en1996.m95.q.unreinforced.max"));
        }
    }

    #[test]
    fn the_structural_system_is_the_weakest_declared_wall_system_and_the_diaphragm_is_the_weakest_stiffness() {
        let mut a = wall(5.0, 0.365);
        assert_eq!(structural_system(std::slice::from_ref(&a)), BracingSystem::Unreinforced);
        a.bracing_system = BracingSystem::Reinforced;
        assert_eq!(structural_system(std::slice::from_ref(&a)), BracingSystem::Reinforced);
        let mut b = wall(5.0, 0.365);
        b.bracing_system = BracingSystem::Confined;
        a.diaphragm_stiffness_n_per_m = 8.0e7;
        b.diaphragm_stiffness_n_per_m = 5.0e7;
        assert_eq!(structural_system(&[a.clone(), b.clone()]), BracingSystem::Confined);
        let case = system_case(AnnexChoice::En, &[a, b]).unwrap();
        assert_eq!(case.opt("k_diaphragm_n_m"), Some(5.0e7));
        assert!(system_case(AnnexChoice::En, &[]).is_none());
    }

    #[test]
    fn a_wall_cannot_declare_a_bracing_system_its_elements_do_not_provide() {
        let mut w = wall(5.0, 0.365);
        w.bracing_system = BracingSystem::Reinforced;
        let report = detailing_checks(AnnexChoice::De, std::slice::from_ref(&w), &[], &[]);
        let check = report.checks.iter().find(|c| c.id == format!("en1996.9.3.bracing-system.{}", w.id)).unwrap();
        assert!(check.utilization > 1.0 && check.remedies.len() == 1);
        w.reinforced = true;
        let report = detailing_checks(AnnexChoice::De, std::slice::from_ref(&w), &[], &[]);
        assert!(report.checks.iter().find(|c| c.id == format!("en1996.9.3.bracing-system.{}", w.id)).unwrap().utilization <= 1.0);
    }

    #[test]
    fn an_opening_needs_a_lintel_that_spans_it_and_offers_the_move_and_the_span_as_remedies() {
        let doc = semio_s_artifact_norm_en1996_model::En1996Snapshot::members_wall_example();
        let report = detailing_checks(doc.annex, &doc.walls, &[], &[]);
        let id = |o: &str| format!("en1996.5.5.lintel-cover.wall-members.{o}");
        for opening in ["win-1", "door-1"] {
            let check = report.checks.iter().find(|c| c.id == id(opening)).unwrap();
            assert!(check.utilization <= 1.0 && check.remedies.is_empty(), "{opening}: {}", check.utilization);
        }
        let mut shifted = doc.walls.clone();
        shifted[0].lintels[0].position_m = 1.0;
        shifted[0].lintels[0].span_m = 0.9;
        let report = detailing_checks(doc.annex, &shifted, &[], &[]);
        let check = report.checks.iter().find(|c| c.id == id("win-1")).unwrap();
        assert!(check.utilization > 1.0 && check.remedies.len() == 2, "{:?}", check.remedies);
        let bare = detailing_checks(doc.annex, &[MasonryWall { lintels: Vec::new(), ..doc.walls[0].clone() }], &[], &[]);
        assert!(bare.checks.iter().filter(|c| c.id.starts_with("en1996.5.5.lintel-cover.")).all(|c| c.status == CheckStatus::NotApplicable));
    }

    #[test]
    fn the_uncovered_length_is_the_part_of_the_opening_outside_the_lintel_span() {
        assert_eq!(uncovered_length_m(1.0, 2.0, 0.5, 2.5), 0.0);
        assert!((uncovered_length_m(1.0, 2.0, 1.2, 2.5) - 0.2).abs() < 1e-12);
        assert!((uncovered_length_m(1.0, 2.0, 0.5, 1.7) - 0.3).abs() < 1e-12);
        assert!((uncovered_length_m(1.0, 2.0, 3.0, 4.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn bars_must_fit_the_wall_with_a_clear_gap() {
        let doc = semio_s_artifact_norm_en1996_model::En1996Snapshot::members_wall_example();
        let mut walls = doc.walls.clone();
        let ok = detailing_checks(doc.annex, &walls, &[], &[]);
        assert!(ok.checks.iter().filter(|c| c.id.starts_with("en1996.8.2.bar-layout.")).count() == 2 && ok.checks.iter().filter(|c| c.id.starts_with("en1996.8.2.bar-layout.")).all(|c| c.utilization < 1.0));
        walls[0].bars[0].spacing_m = 0.004;
        walls[0].bars[1].length_m = 50.0;
        let report = detailing_checks(doc.annex, &walls, &[], &[]);
        for bar in ["bar-shear", "bar-anchor"] {
            let check = report.checks.iter().find(|c| c.id == format!("en1996.8.2.bar-layout.wall-members.{bar}")).unwrap();
            assert!(check.utilization > 1.0 && check.remedies.len() == 1, "{bar}");
        }
    }
}
