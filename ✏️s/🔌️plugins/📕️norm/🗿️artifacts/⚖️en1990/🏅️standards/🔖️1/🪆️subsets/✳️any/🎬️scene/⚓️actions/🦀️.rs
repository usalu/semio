//! ⚓️ Actions: the load-glyph model of permanent, variable (by category), accidental and seismic actions on the members they affect. A glyph carries the design value γ·ψ·E of the active combination, so selecting another combination re-scales every glyph;
//! an action that does not take part in the active combination stays as a ghost at its characteristic value. The reliability, consequence and annex badges and the combination selector group belong here too.

use super::scene::projection_kit::{loc, mul, along, entity_ref, polyline};
use super::inferences::group_coefficient;
use super::checks::{k_fi_for_rc, psi_entry_of, psi_is_ordered, psi_lookup, resolve_psi_category, target_reliability_index_for};
use super::combinations::Column;
use super::kit::*;
use super::layout::Layout;
use super::model::{En1990Snapshot, LoadGroup};
use super::scene::*;
use super::{handles, norm, SceneInput};

const LANE: f64 = 0.35;
pub(crate) const GROUP_LANE: f64 = 1.2;
const BADGE_LANE: f64 = 0.4;

/// 🧾️ One action of the document as the scene sees it.
pub(crate) struct Row {
    pub id: String,
    pub collection: &'static str,
    pub kind: LoadKind,
    pub value: f64,
    pub category: String,
    pub name: LocalizedCopy,
    pub psi: Option<CombinationFactors>,
}

fn variable_kind(category: &str) -> LoadKind {
    if category.starts_with("wind") {
        LoadKind::Wind
    } else if category.starts_with("snow") {
        LoadKind::Snow
    } else if category.starts_with("temperature") {
        LoadKind::Thermal
    } else {
        LoadKind::Variable
    }
}

/// 🧾️ Every action in document order: permanents, variables, accidentals, seismics; the value is the characteristic magnitude (the design action for seismics).
pub(crate) fn rows(document: &En1990Snapshot) -> Vec<Row> {
    let mut out = Vec::new();
    for p in &document.permanents {
        let kind = if p.kind == semio_s_artifact_norm_en1990_model::PermanentKind::Prestress { LoadKind::Prestress } else { LoadKind::Permanent };
        let (en, de) = match p.kind {
            semio_s_artifact_norm_en1990_model::PermanentKind::GInf => ("Permanent action (favourable)", "Ständige Einwirkung (günstig)"),
            semio_s_artifact_norm_en1990_model::PermanentKind::Prestress => ("Prestress", "Vorspannung"),
            _ => ("Permanent action", "Ständige Einwirkung"),
        };
        out.push(Row { id: p.id.clone(), collection: "permanents", kind, value: p.gk, category: p.kind.as_str().to_string(), name: loc(&format!("{en} {}", p.id), &format!("{de} {}", p.id)), psi: None });
    }
    for v in &document.variables {
        let category = resolve_psi_category(document.annex, &v.category, document.altitude_m);
        let psi = psi_lookup(document.annex, &category).filter(|row| psi_is_ordered(*row)).map(|row| CombinationFactors { psi0: row.psi_0, psi1: row.psi_1, psi2: row.psi_2 });
        let label = psi_entry_of(&category).map_or((category.clone(), category.clone()), |e| (e.label_en.to_string(), e.label_de.to_string()));
        out.push(Row { id: v.id.clone(), collection: "variables", kind: variable_kind(&category), value: v.qk, category, name: loc(&format!("{} {}", label.0, v.id), &format!("{} {}", label.1, v.id)), psi });
    }
    for a in &document.accidentals {
        out.push(Row { id: a.id.clone(), collection: "accidentals", kind: LoadKind::Accidental, value: a.ad, category: "accidental".into(), name: loc(&format!("Accidental action {}", a.id), &format!("Außergewöhnliche Einwirkung {}", a.id)), psi: None });
    }
    for s in &document.seismics {
        out.push(Row { id: s.id.clone(), collection: "seismics", kind: LoadKind::Seismic, value: s.a_ed(), category: "seismic".into(), name: loc(&format!("Seismic action {}", s.id), &format!("Erdbebeneinwirkung {}", s.id)), psi: None });
    }
    out
}

/// 📏️ The height above the member top at which action number `index` is drawn.
pub(crate) fn lift(index: usize) -> f64 {
    0.5 + LANE * index as f64
}

fn kn(value: f64) -> String {
    format!("{:.1} kN", value / 1e3)
}

pub(crate) fn affected(document: &En1990Snapshot, action: &str) -> Vec<String> {
    let direct: Vec<String> = document.members.iter().filter(|m| document.effects.iter().any(|e| e.action_id == action && e.member_id == m.id && e.influence != 0.0)).map(|m| m.id.clone()).collect();
    if direct.is_empty() { document.members.first().map(|m| m.id.clone()).into_iter().collect() } else { direct }
}

fn glyph_of(kind: LoadKind, frame: &Frame, span: f64, depth: f64, lift: f64, magnitude: f64, case: LoadCaseRef) -> (LoadGlyph, Vec3) {
    let force = Quantity::new(QuantityKind::Force, magnitude);
    let mid = along(&frame.origin, &frame.x, span / 2.0);
    match kind {
        LoadKind::Wind => {
            let at = along(&mid, &frame.y, -(depth / 4.0 + 0.4 + lift / 2.0));
            (LoadGlyph::Point { at: at.clone(), direction: frame.y.clone(), magnitude: force, case }, at)
        }
        LoadKind::Thermal | LoadKind::Seismic => {
            let at = along(&along(&frame.origin, &frame.x, span), &frame.z, lift / 2.0);
            (LoadGlyph::Point { at: at.clone(), direction: mul(&frame.x, -1.0), magnitude: force, case }, at)
        }
        LoadKind::Accidental => {
            let at = along(&mid, &frame.y, -(depth / 4.0 + 0.2));
            (LoadGlyph::Impact { at: at.clone(), direction: frame.y.clone(), energy_or_force: force, case }, at)
        }
        _ => {
            let top = depth / 2.0 + lift;
            let (a, b) = (along(&frame.origin, &frame.z, top), along(&along(&frame.origin, &frame.x, span), &frame.z, top));
            (LoadGlyph::Line { path: polyline(&[a.clone(), b]), direction: mul(&frame.z, -1.0), profile: LoadProfile::Uniform { value: force }, case }, a)
        }
    }
}

fn design_words(factor: Option<f64>, value: f64, column: Option<&Column>) -> (LocalizedCopy, f64) {
    match (factor, column) {
        (Some(f), Some(c)) if f.abs() > 1e-12 => (loc(&format!("γ·ψ = {f:.3}, design value {} in {} lead {}", kn(f * value), c.combination.equation.label(), c.leading()), &format!("γ·ψ = {f:.3}, Bemessungswert {} in {} führend {}", kn(f * value), c.combination.equation.label(), c.leading())), (f * value).abs()),
        (_, Some(c)) => (loc(&format!("not part of {} lead {}; characteristic value {}", c.combination.equation.label(), c.leading(), kn(value)), &format!("nicht Teil von {} führend {}; charakteristischer Wert {}", c.combination.equation.label(), c.leading(), kn(value))), value.abs()),
        (_, None) => (loc(&format!("characteristic value {}", kn(value)), &format!("charakteristischer Wert {}", kn(value))), value.abs()),
    }
}

fn factor_symbol(factor: &str) -> &'static str {
    match factor {
        "gamma" => "γ",
        "psi0" => "ψ₀",
        "psi1" => "ψ₁",
        _ => "ψ₂",
    }
}

/// 🎚️ The badge text and tone of the factor-override rows that reach one action: the project value beside the National Annex value of every deviating row, a refused row as an error; `None` when no row deviates or fails.
pub(crate) fn override_badge(d: &En1990Snapshot, action: &str) -> Option<(LocalizedCopy, Tone)> {
    use super::inferences::factor_overrides::{annex_for_action, deviations, rows_for, Verdict};
    let deviating = deviations(d);
    let (mut en, mut de, mut tone) = (Vec::new(), Vec::new(), Tone::Warning);
    for (index, verdict) in rows_for(d, action) {
        let row = &d.factor_overrides[index];
        let symbol = factor_symbol(&row.factor);
        if verdict != Verdict::Applied {
            tone = Tone::Error;
            en.push(format!("{symbol} row {} not applied", row.id));
            de.push(format!("{symbol}-Zeile {} nicht angewendet", row.id));
        } else if deviating[index] {
            let annex = annex_for_action(d, &row.factor, action).map_or_else(|| "-".to_string(), |a| format!("{a:.3}"));
            en.push(format!("{symbol} {:.3} (NA {annex})", row.value));
            de.push(format!("{symbol} {:.3} (NA {annex})", row.value));
        }
    }
    (!en.is_empty()).then(|| (LocalizedCopy::new(format!("Deviates from NA: {}", en.join(", ")), format!("Weicht vom NA ab: {}", de.join(", "))), tone))
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, active: Option<&Column>, out: &mut Out) {
    let d = input.document;
    let rows = rows(d);
    for (index, row) in rows.iter().enumerate() {
        if !out.wants(&row.id) {
            continue;
        }
        let factor = active.and_then(|c| c.factor(&row.id));
        let ghost = active.is_some() && factor.is_none_or(|f| f.abs() <= 1e-12);
        let (words, magnitude) = design_words(factor, row.value, active);
        let deviates = active.is_some_and(|c| c.deviates(&row.id));
        let override_badge = override_badge(d, &row.id);
        for member_id in affected(d, &row.id) {
            let (Some(pose), Some(member)) = (layout.member(&member_id), d.members.iter().find(|m| m.id == member_id)) else { continue };
            let lift = lift(index);
            let case = LoadCaseRef { id: row.id.clone(), kind: row.kind, psi: row.psi.clone() };
            let (geometry, anchor) = glyph_of(row.kind, &pose.frame, member.span, member_depth(member.span), lift, magnitude, case);
            out.push(
                Spec {
                    kind: EntityKind::Load,
                    id: format!("{}:{}", row.id, member_id),
                    collection: row.collection,
                    geometry: Geometry::Load(geometry),
                    role: MaterialRole::Load { kind: row.kind },
                    binding: None,
                    name: loc(&format!("{} on {}", row.name.en, member.label_en), &format!("{} auf {}", row.name.de, member.label_de)),
                    detail: loc(&format!("{}: {}{}", row.category, words.en, if deviates { " (deviates from NA)" } else { "" }), &format!("{}: {}{}", row.category, words.de, if deviates { " (weicht vom NA ab)" } else { "" })),
                    layer: LAYER_LOADS,
                    handles: handles::action_ids(d, &row.id),
                    deformation: None,
                    ghost,
                },
                &[anchor.clone()],
            );
            if let Some((text, tone)) = &override_badge {
                let at = along(&anchor, &pose.frame.z, BADGE_LANE);
                out.annotate(Annotation::Label { id: format!("badge:override:{}:{member_id}", row.id), anchor: at.clone(), text: text.clone(), tone: *tone }, &at);
            }
        }
    }
    badges(input, layout, active, rows.len(), out);
    selector(input, layout, active, out);
    traffic_group_components(input, layout, active, out);
    group_chips(input, layout, active, rows.len(), out);
}

/// 📍️ The frame of the first member and the height of the badge stack above its axis.
pub(crate) fn badge_anchor(document: &En1990Snapshot, layout: &Layout, actions: usize) -> Option<(Frame, f64)> {
    let first = document.members.first()?;
    let pose = layout.member(&first.id)?;
    Some((pose.frame.clone(), member_depth(first.span) / 2.0 + 1.2 + LANE * actions as f64))
}

/// 📍️ Where the combination selector grip stands.
pub(crate) fn selector_anchor(document: &En1990Snapshot, layout: &Layout) -> Option<Vec3> {
    let (frame, base) = badge_anchor(document, layout, rows(document).len())?;
    Some(along(&along(&frame.origin, &frame.x, 1.0), &frame.z, base - BADGE_LANE))
}

fn badges(input: &SceneInput, layout: &Layout, active: Option<&Column>, actions: usize, out: &mut Out) {
    let d = input.document;
    out.global();
    let Some((frame, base)) = badge_anchor(input.document, layout, actions) else { return };
    let target = target_reliability_index_for(d.reliability_class.number(), d.reference_period_years);
    let unverified = d.variables.iter().filter(|v| psi_entry_of(&v.category).is_some_and(|e| !e.verified())).count();
    let annex = d.annex.label();
    let mut lines: Vec<(&str, LocalizedCopy, Tone)> = vec![
        ("consequence", loc(&format!("Consequence class CC{}", d.consequence_class), &format!("Schadensfolgeklasse CC{}", d.consequence_class)), Tone::Neutral),
        (
            "reliability",
            loc(&format!("Reliability class RC{} · K_FI = {:.2} · {}/{}", d.reliability_class.number(), k_fi_for_rc(d.reliability_class.number()).unwrap_or(1.0), d.supervision_level, d.inspection_level), &format!("Zuverlässigkeitsklasse RC{} · K_FI = {:.2} · {}/{}", d.reliability_class.number(), k_fi_for_rc(d.reliability_class.number()).unwrap_or(1.0), d.supervision_level, d.inspection_level)),
            if d.reliability_class.number() == d.consequence_class { Tone::Info } else { Tone::Warning },
        ),
        (
            "beta",
            loc(
                &format!("β = {:.2}{} for {:.0} a", d.beta_computed, target.map_or(String::new(), |t| format!(" (target {t:.2})")), d.reference_period_years),
                &format!("β = {:.2}{} für {:.0} a", d.beta_computed, target.map_or(String::new(), |t| format!(" (Ziel {t:.2})")), d.reference_period_years),
            ),
            check_tone(input.report, "en1990.annex-c.beta"),
        ),
        ("annex", loc(&format!("Annex {annex}"), &format!("Anhang {annex}")), Tone::Neutral),
    ];
    if let Some(applied) = input.report.checks.iter().find(|c| c.id == "en1990.annex-b.kfi-applied") {
        lines.push(("kfi-applied", loc(&format!("Applied γ_G,sup = K_FI·γ = {:.3}", applied.computed.value), &format!("Angewendetes γ_G,sup = K_FI·γ = {:.3}", applied.computed.value)), Tone::Info));
    }
    if input.report.checks.iter().any(|c| c.id == "en1990.a2.traffic-groups" && c.status != norm::CheckStatus::NotApplicable) {
        lines.push(("traffic-groups", loc("Traffic load groups", "Verkehrslastgruppen"), check_tone(input.report, "en1990.a2.traffic-groups")));
    }
    let applied = super::inferences::factor_overrides::deviations(d).into_iter().filter(|deviates| *deviates).count();
    let refused = super::inferences::factor_overrides::verdicts(d).iter().filter(|v| **v != super::inferences::factor_overrides::Verdict::Applied).count();
    if applied > 0 {
        lines.push(("deviates-from-na", loc(&format!("Deviates from NA: {applied} project factor override(s)"), &format!("Weicht vom NA ab: {applied} projektbezogene Faktorüberschreibung(en)")), Tone::Warning));
    }
    if refused > 0 {
        lines.push(("override-refused", loc(&format!("{refused} factor override(s) not applied"), &format!("{refused} Faktorüberschreibung(en) nicht angewendet")), Tone::Error));
    }
    if unverified > 0 {
        lines.push(("unverified", loc(&format!("Provisional: {unverified} unverified ψ rows"), &format!("Vorläufig: {unverified} ungeprüfte ψ-Zeilen")), Tone::Warning));
    }
    if let Some(column) = active {
        let e = column.combination.total();
        lines.push(("combination", loc(&format!("Active {} · lead {} · Σ = {}", column.combination.equation.label(), column.leading(), kn(e)), &format!("Aktiv {} · führend {} · Σ = {}", column.combination.equation.label(), column.leading(), kn(e))), Tone::Info));
    }
    for (index, (key, text, tone)) in lines.into_iter().enumerate() {
        let anchor = along(&frame.origin, &frame.z, base + BADGE_LANE * index as f64);
        out.annotate(Annotation::Label { id: format!("badge:{key}"), anchor: anchor.clone(), text, tone }, &anchor);
    }
}

fn selector(input: &SceneInput, layout: &Layout, active: Option<&Column>, out: &mut Out) {
    let d = input.document;
    let Some(anchor) = selector_anchor(d, layout) else { return };
    let members: Vec<EntityRef> = d.members.iter().map(|m| entity_ref(EntityKind::Member, &m.id, "members")).collect();
    let words = active.map_or_else(|| loc("no combination", "keine Kombination"), |c| loc(&format!("{} lead {}", c.combination.equation.label(), c.leading()), &format!("{} führend {}", c.combination.equation.label(), c.leading())));
    out.push(
        Spec {
            kind: EntityKind::Group,
            id: COMBINATION_ENTITY.into(),
            collection: "combinations",
            geometry: Geometry::Group { members },
            role: MaterialRole::Neutral,
            binding: None,
            name: loc(&format!("Combination selector: {}", words.en), &format!("Kombinationswahl: {}", words.de)),
            detail: loc("Steps through the generated combinations; the load glyphs follow.", "Schaltet durch die erzeugten Kombinationen; die Lastsymbole folgen."),
            layer: LAYER_ANNOTATIONS,
            handles: handles::combination_ids(d),
            deformation: None,
            ghost: false,
        },
        &[anchor],
    );
}

/// 🚦️ The load groups the table declares, in the fixed order of `LoadGroup::ALL`: the order of the group chips and of the `view.activeTrafficGroup` index.
pub fn present_groups(d: &En1990Snapshot) -> Vec<LoadGroup> {
    LoadGroup::ALL.into_iter().filter(|group| d.traffic_groups.iter().any(|row| row.group == *group)).collect()
}

/// 🚦️ The group the `view.activeTrafficGroup` index selects (`None` for no or an out-of-range index, which leaves the choice to the combination).
pub fn traffic_group_at(d: &En1990Snapshot, index: Option<u32>) -> Option<LoadGroup> {
    index.and_then(|at| present_groups(d).get(at as usize).copied())
}

/// 🚦️ The group whose components act: the chosen one when the table declares it, else the group of the active combination (first group of the table without an active combination).
pub(crate) fn active_group(d: &En1990Snapshot, active: Option<&Column>, chosen: Option<LoadGroup>) -> Option<LoadGroup> {
    chosen.filter(|group| d.traffic_groups.iter().any(|row| row.group == *group)).or_else(|| active.and_then(|column| d.traffic_groups.iter().find(|row| column.factor(&row.variable_id).is_some_and(|f| f.abs() > 1e-12)).map(|row| row.group)).or_else(|| d.traffic_groups.first().map(|row| row.group)))
}

/// 🚦️ The load components of every `trafficGroups` row on the deck of the member they act on: one lane per group across the deck width, the active group solid and the others ghosts; the magnitude is the declared value times the registry coefficient of the pair.
fn traffic_group_components(input: &SceneInput, layout: &Layout, active: Option<&Column>, out: &mut Out) {
    let d = input.document;
    let current = active_group(d, active, input.traffic_group);
    let groups = present_groups(d);
    for row in &d.traffic_groups {
        let Some(variable) = d.variables.iter().find(|variable| variable.id == row.variable_id) else { continue };
        if !out.wants(&row.id) {
            continue;
        }
        let lane = groups.iter().position(|group| *group == row.group).unwrap_or(0) as f64;
        let lateral = (lane - (groups.len() as f64 - 1.0) / 2.0) * GROUP_LANE;
        let coefficient = group_coefficient(d, variable);
        let magnitude = variable.qk * coefficient.unwrap_or(1.0);
        for member_id in affected(d, &variable.id).into_iter().take(1) {
            let (Some(pose), Some(member)) = (layout.member(&member_id), d.members.iter().find(|m| m.id == member_id)) else { continue };
            let top = member_depth(member.span) / 2.0 + 0.3;
            let origin = along(&pose.frame.origin, &pose.frame.y, lateral);
            let (a, b) = (along(&origin, &pose.frame.z, top), along(&along(&origin, &pose.frame.x, member.span), &pose.frame.z, top));
            let case = LoadCaseRef { id: row.id.clone(), kind: LoadKind::Variable, psi: None };
            let geometry = Geometry::Load(LoadGlyph::Line { path: polyline(&[a.clone(), b]), direction: mul(&pose.frame.z, -1.0), profile: LoadProfile::Uniform { value: Quantity::new(QuantityKind::Force, magnitude) }, case });
            let words = match coefficient {
                Some(c) => (format!("{} · {} · c = {c:.2} · {}", row.group.as_str(), variable.category, kn(magnitude)), format!("{} · {} · c = {c:.2} · {}", row.group.as_str(), variable.category, kn(magnitude))),
                None => (format!("{} · {} · awaiting value (registry cell not transcribed) · {}", row.group.as_str(), variable.category, kn(magnitude)), format!("{} · {} · Wert ausstehend (Registerzelle nicht übertragen) · {}", row.group.as_str(), variable.category, kn(magnitude))),
            };
            out.push(
                Spec {
                    kind: EntityKind::Load,
                    id: format!("{}:{}", row.id, member_id),
                    collection: "trafficGroups",
                    geometry,
                    role: MaterialRole::Load { kind: LoadKind::Variable },
                    binding: None,
                    name: loc(&format!("Traffic group {} component {} on {}", row.group.as_str(), variable.id, member.label_en), &format!("Verkehrslastgruppe {} Bestandteil {} auf {}", row.group.as_str(), variable.id, member.label_de)),
                    detail: loc(&words.0, &words.1),
                    layer: LAYER_LOADS,
                    handles: vec![format!("{}:{}", row.id, "group")],
                    deformation: None,
                    ghost: Some(row.group) != current,
                },
                &[a.clone()],
            );
        }
    }
}

/// 🚦️ One chip per traffic load group beside the combination badges: the active group is highlighted. The group follows the group selector (`view.activeTrafficGroup`) and, without a choice, the active combination.
fn group_chips(input: &SceneInput, layout: &Layout, active: Option<&Column>, actions: usize, out: &mut Out) {
    let d = input.document;
    let Some((frame, base)) = badge_anchor(d, layout, actions) else { return };
    let current = active_group(d, active, input.traffic_group);
    let groups = present_groups(d);
    for (index, group) in groups.iter().enumerate() {
        let anchor = along(&along(&frame.origin, &frame.z, base - BADGE_LANE * 2.0), &frame.x, 1.0 + 1.2 * index as f64);
        let tone = if Some(*group) == current { Tone::Info } else { Tone::Neutral };
        out.annotate(Annotation::Label { id: format!("badge:traffic-group:{}", group.as_str()), anchor: anchor.clone(), text: loc(&format!("Group {}{}", group.as_str(), if Some(*group) == current { " (active)" } else { "" }), &format!("Gruppe {}{}", group.as_str(), if Some(*group) == current { " (aktiv)" } else { "" })), tone }, &anchor);
    }
}
