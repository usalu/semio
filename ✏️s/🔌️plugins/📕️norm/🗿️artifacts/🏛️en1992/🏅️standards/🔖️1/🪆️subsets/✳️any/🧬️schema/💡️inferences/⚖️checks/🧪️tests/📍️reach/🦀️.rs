//! 📍️ The route table of the catalogue entries and the entity cases.

use super::*;
use entity_cases::reach::{route_of, Route, ENTITY_FRAGMENTS, REACH};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

fn fixtures() -> Vec<En1992Snapshot> {
    vec![En1992Snapshot::compliant_office_frame(), En1992Snapshot::failing_under_reinforced(), En1992Snapshot::compliant_prestressed_beam(), En1992Snapshot::failing_prestressed_beam(), En1992Snapshot::liquid_retaining_fem_anchor(), En1992Snapshot::flat_slab_punching()]
}

fn edge_fixtures() -> Vec<En1992Snapshot> {
    let mut beyond_scope = En1992Snapshot::compliant_office_frame();
    beyond_scope.concrete_grades.iter_mut().for_each(|g| g.f_ck = 95.0e6);
    let mut bridge = En1992Snapshot::compliant_office_frame();
    bridge.members[0].bridge_sigma_c = 5.0e6;
    bridge.members[0].bridge_delta_sigma_s = 50.0e6;
    let mut missing_steel = En1992Snapshot::compliant_office_frame();
    missing_steel.members[0].reinforcement_grade_id = "missing".into();
    let mut dangling = En1992Snapshot::compliant_prestressed_beam();
    dangling.members[0].prestress_steel_id = "missing".into();
    let mut orphan = En1992Snapshot::compliant_office_frame();
    orphan.members[0].prestress = None;
    orphan.members[0].prestress_steel_id = "orphan".into();
    let mut unloaded = En1992Snapshot::compliant_office_frame();
    unloaded.members[0].actions.clear();
    let mut deep_beam = En1992Snapshot::compliant_office_frame();
    if let Some(beam) = deep_beam.members.iter_mut().find(|m| m.kind == semio_s_artifact_norm_en1992_model::MemberKind::Beam) {
        beam.span = 1.5;
        beam.height = 0.6;
        beam.effective_depth = 0.54;
    }
    let mut national = En1992Snapshot::liquid_retaining_fem_anchor();
    national.annex = semio_s_plugin_norm_compliance::AnnexChoice::De;
    vec![beyond_scope, bridge, missing_steel, dangling, orphan, unloaded, deep_beam, national]
}

pub(crate) fn report_ids() -> Vec<String> {
    fixtures().iter().chain(edge_fixtures().iter()).flat_map(|d| d.members.iter().flat_map(|m| evaluate_member(d, m)).chain(d.anchors.iter().flat_map(|a| evaluate_anchor(d, a))).chain(evaluate_material_cases(d))).map(|c| c.id).collect()
}

#[test]
fn every_registry_entry_has_exactly_one_route() {
    let ids: Vec<&str> = registry().into_iter().map(|e| e.id).collect();
    assert_eq!(REACH.len(), ids.len());
    assert!(REACH.windows(2).all(|w| w[0].0 < w[1].0));
    for id in &ids {
        assert!(route_of(id).is_some(), "{id} has no route");
    }
}

#[test]
fn the_check_routes_name_a_prefix_the_fixtures_produce() {
    let produced = report_ids();
    for (id, route) in REACH {
        if let Route::Check(prefix) = route {
            assert!(produced.iter().any(|p| p.starts_with(prefix)), "{id}: no check starts with {prefix}");
        }
    }
}

#[test]
fn the_entity_cases_emit_bond_and_prestress_limits_on_the_document_entities() {
    let produced = report_ids();
    for fragment in ENTITY_FRAGMENTS.iter().filter(|fragment| **fragment != "en1992.b-09.") {
        assert!(produced.iter().any(|p| p.starts_with(fragment)), "no entity case {fragment}");
    }
}

#[test]
fn the_entity_case_bond_strength_equals_the_catalogue_value() {
    let document = En1992Snapshot::compliant_office_frame();
    let member = document.members.iter().find(|m| !m.longitudinal.is_empty()).expect("a member with bars");
    let layer = &member.longitudinal[0];
    let fck = document.concrete(&member.concrete_grade_id).expect("grade").f_ck / 1.0e6;
    let expected = f_bd(document.annex, fck, layer.diameter * 1.0e3, layer.bond_condition != semio_s_artifact_norm_en1992_model::BondCondition::Poor);
    let results = evaluate_member(&document, member);
    let hit = results.iter().find(|c| c.id == format!("en1992.b-01.fbd.{}-{}", member.id, layer.id)).expect("bond case");
    assert!((hit.limit.value / 1.0e6 - expected).abs() < 1.0e-9 * expected.abs().max(1.0));
}

#[test]
fn the_registry_rows_of_the_constants_exist_unverified_and_keep_the_implemented_values() {
    let ids = ["psi-fire-lead", "fire-zones", "fire-zone-exponent", "ld-flange-factor", "ld-flange-ratio", "col-link-spacing-de", "col-link-spacing-max", "col-link-near-factor", "lq-04-wk-selfhealing", "lq-04-wk-above", "lq-04-head-ratio", "na-de-edition", "bridge-alpha-cc-lt"];
    for id in ids.map(|id| format!("ndp.en1992.{id}")) {
        assert!(!ndp_is_verified(&id), "{id} must stay unverified");
        assert!(ndp(AnnexChoice::En, &id).is_finite() && ndp(AnnexChoice::De, &id).is_finite());
    }
    assert_eq!((ndp(AnnexChoice::En, "ndp.en1992.col-link-spacing-de"), ndp(AnnexChoice::De, "ndp.en1992.col-link-spacing-de")), (20.0, 12.0));
    assert_eq!(ndp(AnnexChoice::De, "ndp.en1992.bridge-alpha-cc-lt"), 0.8);
    assert_eq!((fire_field::zones(), fire_field::zone_exponent()), (5, 1.0));
    assert_eq!(ndp(AnnexChoice::De, "ndp.en1992.na-de-edition"), 2013.0);
}
