use super::tests::documents;
use crate::Vdi3805Family;
use semio_s_artifact_norm_vdi3805_scene as scene;
use semio_s_plugin_norm_compliance::incremental_evaluation::evaluate_units;
use semio_s_plugin_norm_compliance::CheckStatus;

#[test]
fn every_check_is_drawn_on_its_row_or_the_badge_and_a_failure_colours_its_entity() {
    for document in documents() {
        let report = evaluate_units::<Vdi3805Family>(&document);
        let drawn = scene::live_scene_of(&document, &report);
        for check in &report.checks {
            assert!(scene::route_of(&document, check).is_some(), "{} has no route", check.id);
            if check.status == CheckStatus::Fail {
                if let Some(owner) = scene::owner_entity(&document, check) {
                    let entity = drawn.entities.iter().find(|e| e.entity.id == owner).unwrap_or_else(|| panic!("{owner} is not drawn"));
                    assert_eq!(entity.appearance.style, semio_s_plugin_norm_structural_scene::StyleState::Error, "{} does not colour {owner}", check.id);
                }
            }
        }
    }
}
