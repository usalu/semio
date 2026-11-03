use crate::En1997Family;
use semio_s_artifact_norm_en1997_inferences::evaluate;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub(super) fn documents() -> Vec<En1997Snapshot> {
    vec![semio_s_artifact_norm_en1997_model::compliant_demo(), semio_s_artifact_norm_en1997_model::noncompliant_demo(), semio_s_artifact_norm_en1997_model::showcase_demo(), En1997Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)]
}

semio_s_plugin_norm_compliance::incremental_laws!(source = En1997Family, full = evaluate, live = semio_s_artifact_norm_en1997_inferences::live::report, documents = documents());
