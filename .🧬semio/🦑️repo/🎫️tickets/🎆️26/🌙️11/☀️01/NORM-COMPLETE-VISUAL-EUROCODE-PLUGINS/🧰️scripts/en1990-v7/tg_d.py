"""Traffic groups layer D: inference (effective group, registry coefficient, row checks, group combinations), integrity, units."""
from tg_lib import *
from tg_a import put

edit(INF + "/🦀️.rs", lambda t: put(put(t, "use semio_s_artifact_norm_en1990_model::{BridgeSls, FactorOverride, Member, MemberEffect};", "use semio_s_artifact_norm_en1990_model::{BridgeSls, FactorOverride, LoadGroup, Member, MemberEffect, TrafficGroup, VariableAction};"), "pub use report_helpers::{actions_with, context_for, member_actions};", "pub use report_helpers::{actions_with, context_for, effective_group, group_coefficient, group_coefficient_id, member_actions};"))

HELPERS = '''/// 🚦️ The exclusive group id of a variable action: its typed `trafficGroups` row (lower-case group id such as `gr1a`) wins over the free `group` text.
pub fn effective_group(document: &En1990Snapshot, variable: &VariableAction) -> String {
    document.traffic_groups.iter().find(|row| row.variable_id == variable.id).map_or_else(|| variable.group.clone(), |row| row.group.as_str().to_string())
}

/// 🆔️ The registry id of the coefficient of a component category in a traffic load group.
pub fn group_coefficient_id(group: LoadGroup, category: &str) -> String {
    format!("ndp.en1990.a2.group.{}.{category}", group.as_str())
}

/// 🔢️ The coefficient applied to the declared characteristic value of a traffic component: 1.0 without a table row, the registry value of the (group, category) pair with one, `None` when the registry holds no value for the pair.
pub fn group_coefficient(document: &En1990Snapshot, variable: &VariableAction) -> Option<f64> {
    document.traffic_groups.iter().find(|row| row.variable_id == variable.id).map_or(Some(1.0), |row| ndp_lookup(document.annex, &group_coefficient_id(row.group, &variable.category)))
}

'''
def helpers(t):
    if "pub fn effective_group" in t:
        return t
    t = put(t, "/// ⚡️ The document's actions as generator input", HELPERS + "/// ⚡️ The document's actions as generator input")
    t = rep(t, "        let value = influence(&variable.id) * variable.qk;\n", "        let value = influence(&variable.id) * variable.qk * group_coefficient(document, variable).unwrap_or(1.0);\n")
    return rep(t, ".group(variable.group.clone())", ".group(effective_group(document, variable))")
edit(INF + "/🧰️report-helpers/🦀️.rs", helpers)

ROWS = '''
fn table_subject(row: &TrafficGroup, field: &str) -> SubjectRef {
    SubjectRef::new(&row.id, format!("trafficGroups[id={}].{field}", row.id), copy(format!("Traffic group row {}", row.id), format!("Verkehrsgruppenzeile {}", row.id)))
}

/// 🚦️ One check per `trafficGroups` row (EN 1990 A2.2.2 with EN 1991-2 Table 4.4a): the component exists, belongs to a traffic category of the row's group and the registry holds its coefficient; every registry coefficient is unverified, so a consistent row reports an unverified basis, never a compliant verdict.
pub(super) fn push_traffic_group_rows(report: &mut CheckReport, document: &En1990Snapshot) {
    if !is_bridge(&document.structure_kind) {
        return;
    }
    for row in &document.traffic_groups {
        let clause = ClauseId::new("EN 1990", "A2.2.2", "Table A2.4, EN 1991-2 Table 4.4a");
        let title = copy("Traffic load group component", "Bestandteil der Verkehrslastgruppe");
        let builder = CheckResult::assess(format!("en1990.a2.traffic-group.{}", row.id), "EN 1990 Annex A2", clause, table_subject(row, "variableId"), title).annex(document.annex);
        let Some(variable) = document.variables.iter().find(|variable| variable.id == row.variable_id) else {
            let ids: Vec<String> = document.variables.iter().map(|variable| variable.id.clone()).collect();
            report.push(builder.status(CheckStatus::Fail).explanation(copy(format!("Component {:?} of {} is not a variable action.", row.variable_id, row.group.as_str()), format!("Bestandteil {:?} von {} ist keine veränderliche Einwirkung.", row.variable_id, row.group.as_str())))
                .remedy(Remedy::one_of(table_subject(row, "variableId"), ids, copy("Name an existing variable action.", "Eine vorhandene veränderliche Einwirkung benennen."))).build());
            continue;
        };
        let natural = traffic_group(&variable.category);
        let id = group_coefficient_id(row.group, &variable.category);
        let coefficient = ndp_lookup(document.annex, &id);
        let (en, de, status, utilisation) = match (natural, coefficient) {
            (None, _) => (format!("{} (category {}) is not a road traffic component.", variable.id, variable.category), format!("{} (Kategorie {}) ist kein Straßenverkehrsbestandteil.", variable.id, variable.category), CheckStatus::Warning, None),
            (Some(group), _) if group != row.group.as_str() => (format!("{} belongs to {group} by its category, not to {}.", variable.id, row.group.as_str()), format!("{} gehört nach seiner Kategorie zu {group}, nicht zu {}.", variable.id, row.group.as_str()), CheckStatus::Warning, None),
            (Some(_), None) => (format!("The registry holds no coefficient for {} in {}; the declared value is used unchanged.", variable.category, row.group.as_str()), format!("Das Register enthält keinen Beiwert für {} in {}; der angegebene Wert wird unverändert verwendet.", variable.category, row.group.as_str()), CheckStatus::Warning, None),
            (Some(_), Some(c)) => (format!("{} acts in {} with coefficient {c:.2} on its characteristic value.", variable.id, row.group.as_str()), format!("{} wirkt in {} mit dem Beiwert {c:.2} auf seinen charakteristischen Wert.", variable.id, row.group.as_str()), if c <= 1.0 { CheckStatus::Pass } else { CheckStatus::Warning }, Some(c)),
        };
        let mut builder = builder.status(status).explanation(copy(en, de));
        if let Some(c) = utilisation {
            builder = builder.utilization(q_dim(c), q_dim(1.0));
        }
        if status != CheckStatus::Pass {
            let target = natural.and_then(LoadGroup::from_id).map_or_else(|| vec![variable.id.clone()], |group| vec![format!("{group:?}")]);
            builder = builder.remedy(Remedy::one_of(table_subject(row, if natural.is_some() { "group" } else { "variableId" }), target, copy("Assign the component to its group.", "Den Bestandteil seiner Gruppe zuordnen.")));
        }
        report.push(builder.ndp(&id).build());
    }
}
'''
def criteria(t):
    if "push_traffic_group_rows" in t:
        return t
    t = rep(t, "fn group_subject(variable: &semio_s_artifact_norm_en1990_model::VariableAction) -> SubjectRef {", "fn group_subject(variable: &semio_s_artifact_norm_en1990_model::VariableAction) -> SubjectRef {", count=1)
    t = rep(t, "    let violates = |(variable, group): &(&semio_s_artifact_norm_en1990_model::VariableAction, &str)| {\n        variable.group.is_empty()\n            || traffic.iter().any(|(other, other_group)| other.id != variable.id && ((other.group == variable.group && other_group != group) || (other_group == group && other.group != variable.group)))\n    };", "    let effective = |variable: &semio_s_artifact_norm_en1990_model::VariableAction| effective_group(document, variable);\n    let violates = |(variable, group): &(&semio_s_artifact_norm_en1990_model::VariableAction, &str)| {\n        effective(variable).is_empty()\n            || traffic.iter().any(|(other, other_group)| other.id != variable.id && ((effective(other) == effective(variable) && other_group != group) || (other_group == group && effective(other) != effective(variable))))\n    };")
    return t.rstrip("\n").rsplit("//#endregion", 1)[0].rstrip("\n") + "\n" + ROWS + "//#endregion" + t.rstrip("\n").rsplit("//#endregion", 1)[1] + "\n" if "//#endregion" in t else t + ROWS
edit(INF + "/📐️annex-criteria/🦀️.rs", criteria)
edit(INF + "/📋️compliance-evaluation/🦀️.rs", lambda t: t if "push_traffic_group_rows" in t else rep(t, "        push_traffic_groups(&mut report, document);\n", "        push_traffic_groups(&mut report, document);\n        push_traffic_group_rows(&mut report, document);\n"))
edit(INF + "/⛓️referential-integrity/🦀️.rs", lambda t: t if "trafficGroups" in t else rep(t, '    push_duplicate_ids(report, document, "placements",', '    push_duplicate_ids(report, document, "trafficGroups", &document.traffic_groups.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("trafficGroups[id={id}].id"));\n    push_duplicate_ids(report, document, "placements",'))
edit(INF + "/🧮️units/🦀️.rs", lambda t: t if "trafficGroups.*" in t else rep(t, '"fatigueSpectra.*", "factorOverrides.*"]', '"fatigueSpectra.*", "factorOverrides.*", "trafficGroups.*"]'))

COMBOS = '''
/// 🚦️ The governing STR combination of one member per traffic load group of the `trafficGroups` table: the components of every other group are removed from the generator input, so the group acts alone with the permanent, wind and thermal actions; the governing group is reported last (EN 1990 A2.2.2, EN 1991-2 Table 4.4a).
pub(super) fn push_traffic_group_combinations(report: &mut CheckReport, document: &En1990Snapshot, member: &Member, actions: &[Action]) {
    if !is_bridge(&document.structure_kind) || document.traffic_groups.is_empty() {
        return;
    }
    let Some(ctx) = context_for(document, FactorSet::StrB) else { return };
    let groups: std::collections::BTreeSet<LoadGroup> = document.traffic_groups.iter().filter(|row| document.variables.iter().any(|variable| variable.id == row.variable_id)).map(|row| row.group).collect();
    let mut governing: Option<(LoadGroup, f64)> = None;
    for group in &groups {
        let others: Vec<&str> = document.traffic_groups.iter().filter(|row| row.group != *group).map(|row| row.variable_id.as_str()).collect();
        let kept: Vec<Action> = actions.iter().filter(|action| !others.contains(&action.id.as_str())).cloned().collect();
        let Some(combination) = worst(&ctx, &kept, ultimate_equations(document.annex, ctx.domain)[0], false) else { continue };
        let total = combination.total();
        if governing.map_or(true, |(_, best)| total > best) {
            governing = Some((*group, total));
        }
        report.push(
            CheckResult::assess(format!("en1990.a2.traffic-group-combination.{}.{}", group.as_str(), member.id), "EN 1990 Annex A2", ClauseId::new("EN 1990", "A2.2.2", "Table 4.4a"), member_subject(member, "rdStr"), copy(format!("Traffic group {} – governing STR combination", group.as_str()), format!("Verkehrslastgruppe {} – maßgebende STR-Kombination", group.as_str())))
                .informative(q_force(total))
                .annex(document.annex)
                .explanation(copy(format!("E_d = {total:.1} N ({}) with {} as the only traffic load group", describe(&combination), group.as_str()), format!("E_d = {total:.1} N ({}) mit {} als einziger Verkehrslastgruppe", describe(&combination), group.as_str())))
                .build(),
        );
    }
    if let Some((group, total)) = governing.filter(|_| groups.len() > 1) {
        report.push(
            CheckResult::assess(format!("en1990.a2.traffic-group-governing.{}", member.id), "EN 1990 Annex A2", ClauseId::new("EN 1990", "A2.2.2", "Table 4.4a"), member_subject(member, "rdStr"), copy("Governing traffic load group", "Maßgebende Verkehrslastgruppe"))
                .informative(q_force(total))
                .annex(document.annex)
                .explanation(copy(format!("{} governs {} with E_d = {total:.1} N among {} groups", group.as_str(), member.id, groups.len()), format!("{} ist für {} maßgebend mit E_d = {total:.1} N unter {} Gruppen", group.as_str(), member.id, groups.len())))
                .build(),
        );
    }
}
'''
def ulscombos(t):
    if "push_traffic_group_combinations" in t:
        return t
    head, sep, tail = t.rpartition("//#endregion")
    return head.rstrip("\n") + "\n" + COMBOS + sep + tail
edit(INF + "/🏗️ultimate-limit-state/🦀️.rs", ulscombos)
edit(INF + "/📋️compliance-evaluation/🦀️.rs", lambda t: t if "push_traffic_group_combinations" in t else rep(t, "            push_geo_set_c(&mut report, document, member, &actions);\n", "            push_geo_set_c(&mut report, document, member, &actions);\n            push_traffic_group_combinations(&mut report, document, member, &actions);\n"))
