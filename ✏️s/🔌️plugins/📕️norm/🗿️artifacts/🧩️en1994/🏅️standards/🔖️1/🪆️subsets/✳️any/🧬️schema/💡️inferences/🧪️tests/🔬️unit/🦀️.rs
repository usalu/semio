use crate::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    assert_eq!(En1994Inference::infer(&snapshot).expect("valid materialized inference fixture"), En1994Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(En1994Inference::infer(&En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)).expect("valid materialized inference fixture"), En1994Inference::default());
}

#[semio_framework_async_macros::async_test]
async fn every_spec_id_is_traced_to_a_check_an_example_document_emits() {
    use semio_s_plugin_norm_compliance::spec_trace::{assert_traced, Emitted};
    let mut detailed = En1994Snapshot::bridge_girder();
    {
        let beam = &mut detailed.beams[0];
        beam.stud_transverse_spacing_m = 0.1;
        beam.stud_edge_distance_m = 0.03;
        beam.transverse_as_top_m2_per_m = 0.001;
        beam.end_plate_width_m = 0.3;
        beam.end_plate_height_m = 0.3;
        beam.end_zone_length_m = 0.5;
    }
    let mut punched = En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    punched.slabs[0].actions.push(semio_s_artifact_norm_en1994_model::SlabAction::from(&semio_s_artifact_norm_en1994_model::CharacteristicAction::point_force("P-slab", semio_s_artifact_norm_en1994_model::ActionStage::Composite, semio_s_artifact_norm_en1994_model::ActionKind::Imposed, "B", 20e3)));
    let documents = [En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), En1994Snapshot::unpropped_building(), En1994Snapshot::custom_plate_building(), En1994Snapshot::bridge_girder(), En1994Snapshot::fire_demanding(), detailed, punched, En1994Snapshot::full_showcase()];
    let emitted = documents.iter().fold(Emitted::default(), |emitted, doc| emitted.with_checks(evaluate(doc).checks.into_iter().map(|c| c.id)));
    assert_traced(&semio_s_artifact_norm_en1994_checks::spec_trace::TABLE, &emitted);
}

const SHOWCASE_UNREACHED: &[(&str, &str)] = &[];

#[semio_framework_async_macros::async_test]
async fn the_full_showcase_alone_reaches_every_check_fragment_of_the_trace() {
    use semio_s_plugin_norm_compliance::spec_trace::{assert_reaches_all_but, Emitted};
    let showcase = En1994Snapshot::full_showcase();
    for (collection, rows) in [("beams", showcase.beams.len()), ("columns", showcase.columns.len()), ("slabs", showcase.slabs.len()), ("placements", showcase.placements.len())] {
        assert!(rows > 0, "the showcase leaves {collection} empty");
    }
    let emitted = Emitted::default().with_checks(evaluate(&showcase).checks.into_iter().map(|c| c.id));
    assert_reaches_all_but(&semio_s_artifact_norm_en1994_checks::spec_trace::TABLE, &emitted, SHOWCASE_UNREACHED);
}
