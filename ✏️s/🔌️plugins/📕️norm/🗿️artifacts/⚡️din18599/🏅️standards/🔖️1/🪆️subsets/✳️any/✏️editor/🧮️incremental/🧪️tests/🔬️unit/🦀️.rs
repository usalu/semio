use crate::DinV18599Family;
use semio_s_artifact_norm_din18599_inferences::evaluate;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub(super) fn documents() -> Vec<Din18599Snapshot> {
    vec![Din18599Snapshot::default(), semio_s_artifact_norm_din18599_model::subjects::noncompliant_detached_house(), semio_s_artifact_norm_din18599_model::subjects::compliant_two_zone_house(), semio_s_artifact_norm_din18599_model::subjects::cooled_office_building()]
}

semio_s_plugin_norm_compliance::incremental_laws!(source = DinV18599Family, full = evaluate, live = semio_s_artifact_norm_din18599_inferences::live::report, documents = documents());
