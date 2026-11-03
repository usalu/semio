use crate::En1998Family;
use semio_s_artifact_norm_en1998_inferences::evaluate;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub(super) fn documents() -> Vec<En1998Snapshot> {
    vec![En1998Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), En1998Snapshot::compliant_de_office(), En1998Snapshot::noncompliant_de_office(), En1998Snapshot::compliant_de_multipart(), En1998Snapshot::noncompliant_de_multipart(), En1998Snapshot::compliant_en_office(), En1998Snapshot::noncompliant_en_office(), En1998Snapshot::compliant_de_torsion_regular(), En1998Snapshot::noncompliant_de_torsion_irregular()]
}

semio_s_plugin_norm_compliance::incremental_laws!(source = En1998Family, full = evaluate, live = semio_s_artifact_norm_en1998_inferences::live::report, documents = documents());
