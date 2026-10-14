//! 🔺️ `change-cellar-ventilation` diff.
use crate::change_cellar_ventilation::ChangeCellarVentilation;
use semio_s_artifact_norm_din16798_diff::Din16798Diff;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn diff(payload: &ChangeCellarVentilation, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.cellar_ventilation_m3_h == payload.new_cellar_ventilation_m3_h {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { cellar_ventilation_m3_h: Some(payload.new_cellar_ventilation_m3_h), ..Default::default() })
}
