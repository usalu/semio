use crate::En1993Family;
use semio_s_artifact_norm_en1993_checks::check_full_steel_structure;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
use semio_s_artifact_norm_en1993_inferences::live::report;

pub(super) fn documents() -> Vec<En1993Snapshot> {
    let mut stainless = En1993Snapshot::compliant_heb240_frame();
    stainless.materials[0].kind = semio_s_artifact_norm_en1993_model::MaterialKind::Stainless;
    vec![En1993Snapshot::compliant_heb240_frame(), En1993Snapshot::noncompliant_overloaded_frame(), En1993Snapshot { members: Vec::new(), joints: Vec::new(), fatigue_details: Vec::new(), fire_exposures: Vec::new(), cold_formed_members: Vec::new(), plated_panels: Vec::new(), silo_shells: Vec::new(), tension_components: Vec::new(), bridge_fatigue: Vec::new(), tower_legs: Vec::new(), piles: Vec::new(), crane_runways: Vec::new(), ..En1993Snapshot::compliant_heb240_frame() }, stainless, En1993Snapshot::frame_portal_showcase(), En1993Snapshot::special_parts_showcase()]
}

#[test]
fn every_checked_row_has_its_own_unit() {
    let units = <En1993Family as semio_s_plugin_norm_compliance::incremental_evaluation::UnitSource>::units(&En1993Snapshot::compliant_heb240_frame());
    let ids: std::collections::BTreeSet<_> = units.iter().map(|u| u.id.0.clone()).collect();
    assert_eq!(ids.len(), units.len());
    for expected in ["global:document", "members:member-b1", "joints:joint-j1", "fireExposures:fire-1", "memberProperties:member-b1", "piles:pile-1"] {
        assert!(ids.contains(expected), "{expected} in {ids:?}");
    }
}

semio_s_plugin_norm_compliance::incremental_laws!(source = En1993Family, full = check_full_steel_structure, live = report, documents = documents());
