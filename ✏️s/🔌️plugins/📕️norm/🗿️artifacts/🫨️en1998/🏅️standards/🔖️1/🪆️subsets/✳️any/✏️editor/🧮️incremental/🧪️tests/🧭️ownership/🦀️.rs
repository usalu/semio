use super::tests::documents;
use crate::En1998Family;
use semio_s_artifact_norm_en1998_scene as scene;
use semio_s_plugin_norm_compliance::incremental_evaluation::{evaluate_units, UnitSource};
use semio_s_plugin_norm_structural_scene::projection_kit::{audit_owners, ownership_audit};

#[test]
fn every_check_of_every_demo_document_has_an_owner_or_is_documented_global_and_a_failure_colours_its_entity() {
    for document in documents() {
        let report = evaluate_units::<En1998Family>(&document);
        let drawn = scene::live_scene_of(&document, &report);
        let owners = audit_owners(&document, &scene::entities_of(&document));
        let audit = ownership_audit(&report, &drawn, &owners, |id| En1998Family::OWNERS.is_document_level(id));
        assert!(audit.is_clean(), "{audit:?}");
    }
}
