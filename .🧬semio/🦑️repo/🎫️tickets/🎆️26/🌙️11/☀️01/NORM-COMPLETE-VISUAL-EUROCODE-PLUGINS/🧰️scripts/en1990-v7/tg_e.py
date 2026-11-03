"""Traffic groups layer E: scene (kit registry, deck projection of the active group's components, group chips, dependencies)."""
from tg_lib import *
from tg_a import put
SC = SUB + "/🎬️scene"

def kit(t):
    if "trafficGroup" in t:
        return t
    t = put(t, '    kind("combination", "combinations", "Combination selector", "Kombinationswahl", "list", EntityKind::Group),\n', '    kind("trafficGroup", "trafficGroups", "Traffic load group component", "Verkehrslastgruppen-Bestandteil", "route", EntityKind::Load),\n    kind("combination", "combinations", "Combination selector", "Kombinationswahl", "list", EntityKind::Group),\n')
    t = rep(t, "fn rows_of(d: &model::En1990Snapshot) -> [(&'static str, Vec<&str>); 12] {", "fn rows_of(d: &model::En1990Snapshot) -> [(&'static str, Vec<&str>); 13] {")
    t = put(t, '        ("combinations", vec![COMBINATION_ENTITY]),\n', '        ("trafficGroups", d.traffic_groups.iter().map(|e| e.id.as_str()).collect()),\n        ("combinations", vec![COMBINATION_ENTITY]),\n')
    return put(t, "    if is_action {\n", "    if let Some(row) = d.traffic_groups.iter().find(|r| r.id == base) {\n        out.insert(row.variable_id.clone());\n        out.extend(d.effects.iter().filter(|e| e.action_id == row.variable_id).map(|e| e.member_id.clone()));\n    }\n    out.extend(d.traffic_groups.iter().filter(|r| r.variable_id == base).map(|r| r.id.clone()));\n    if is_action {\n")
edit(SC + "/🧰️kit/🦀️.rs", kit)
edit(SC + "/📈️diagrams/🦀️.rs", lambda t: put(t, "        || document.factor_overrides.iter().any(|o| o.id == id)\n", "        || document.factor_overrides.iter().any(|o| o.id == id)\n        || document.traffic_groups.iter().any(|o| o.id == id)\n"))

PROJECT = '''
/// 🚦️ The group whose components act in the active combination (first group of the table without an active combination).
pub(crate) fn active_group(d: &En1990Snapshot, active: Option<&Column>) -> Option<LoadGroup> {
    active.and_then(|column| d.traffic_groups.iter().find(|row| column.factor(&row.variable_id).is_some_and(|f| f.abs() > 1e-12)).map(|row| row.group)).or_else(|| d.traffic_groups.first().map(|row| row.group))
}

/// 🚦️ The load components of every `trafficGroups` row on the deck of the member they act on: one lane per group across the deck width, the active group solid and the others ghosts; the magnitude is the declared value times the registry coefficient of the pair.
fn traffic_group_components(input: &SceneInput, layout: &Layout, active: Option<&Column>, out: &mut Out) {
    let d = input.document;
    let current = active_group(d, active);
    let groups: Vec<LoadGroup> = LoadGroup::ALL.into_iter().filter(|group| d.traffic_groups.iter().any(|row| row.group == *group)).collect();
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
                None => (format!("{} · {} · no registry coefficient (unverified) · {}", row.group.as_str(), variable.category, kn(magnitude)), format!("{} · {} · kein Registerbeiwert (ungeprüft) · {}", row.group.as_str(), variable.category, kn(magnitude))),
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
                    handles: Vec::new(),
                    deformation: None,
                    ghost: Some(row.group) != current,
                },
                &[a.clone()],
            );
        }
    }
}

/// 🚦️ One chip per traffic load group beside the combination badges: the active group is highlighted. The group follows the active combination; a free group selector needs the view-state routing requested from the shared owner.
fn group_chips(input: &SceneInput, layout: &Layout, active: Option<&Column>, actions: usize, out: &mut Out) {
    let d = input.document;
    let Some((frame, base)) = badge_anchor(d, layout, actions) else { return };
    let current = active_group(d, active);
    let groups: Vec<LoadGroup> = LoadGroup::ALL.into_iter().filter(|group| d.traffic_groups.iter().any(|row| row.group == *group)).collect();
    for (index, group) in groups.iter().enumerate() {
        let anchor = along(&along(&frame.origin, &frame.z, base - BADGE_LANE * 2.0), &frame.x, 1.0 + 1.2 * index as f64);
        let tone = if Some(*group) == current { Tone::Info } else { Tone::Neutral };
        out.annotate(Annotation::Label { id: format!("badge:traffic-group:{}", group.as_str()), anchor: anchor.clone(), text: loc(&format!("Group {}{}", group.as_str(), if Some(*group) == current { " (active)" } else { "" }), &format!("Gruppe {}{}", group.as_str(), if Some(*group) == current { " (aktiv)" } else { "" })), tone }, &anchor);
    }
}
'''
def actions(t):
    if "fn traffic_group_components" in t:
        return t
    t = rep(t, "    badges(input, layout, active, rows.len(), out);\n    selector(input, layout, active, out);\n}", "    badges(input, layout, active, rows.len(), out);\n    selector(input, layout, active, out);\n    traffic_group_components(input, layout, active, out);\n    group_chips(input, layout, active, rows.len(), out);\n}")
    t = rep(t, "const LANE: f64 = 0.35;", "const LANE: f64 = 0.35;\nconst GROUP_LANE: f64 = 1.2;")
    t = rep(t, "use super::model::En1990Snapshot;", "use super::model::{En1990Snapshot, LoadGroup};")
    t = rep(t, "use super::checks::{k_fi_for_rc,", "use super::inferences::group_coefficient;\nuse super::checks::{k_fi_for_rc,")
    return t.rstrip("\n") + "\n" + PROJECT
edit(SC + "/⚓️actions/🦀️.rs", actions)
