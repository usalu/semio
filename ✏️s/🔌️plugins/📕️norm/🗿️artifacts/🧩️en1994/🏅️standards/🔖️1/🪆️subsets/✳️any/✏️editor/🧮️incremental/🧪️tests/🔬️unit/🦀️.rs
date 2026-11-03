use crate::En1994Family;
use semio_s_artifact_norm_en1994_inferences::evaluate;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub(super) fn documents() -> Vec<En1994Snapshot> {
    vec![En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), En1994Snapshot::unpropped_building(), En1994Snapshot::custom_plate_building(), En1994Snapshot::bridge_girder(), En1994Snapshot::fire_demanding()]
}

semio_s_plugin_norm_compliance::incremental_laws!(source = En1994Family, full = evaluate, live = semio_s_artifact_norm_en1994_inferences::live::report, documents = documents());
