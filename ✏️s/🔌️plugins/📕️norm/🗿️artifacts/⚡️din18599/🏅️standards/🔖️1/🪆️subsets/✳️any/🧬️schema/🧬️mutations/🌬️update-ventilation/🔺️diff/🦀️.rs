//! 🔺️ `update-ventilation` sparse diff.

use crate::mutations::update_ventilation::UpdateVentilation;
use crate::Din18599Snapshot;
use crate::diff::{Din18599Diff, Din18599VentilationPatch};

pub fn diff(payload: &UpdateVentilation, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.ventilation == payload.new_ventilation {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "ventilation already has this value.");
    }
    let (old, new) = (&base.ventilation, &payload.new_ventilation);
    protocol::MutationOutcome::new(Din18599Diff {
        ventilation: Some(Din18599VentilationPatch {
            airflow_m3_h: (old.airflow_m3_h != new.airflow_m3_h).then(|| new.airflow_m3_h.clone()),
            heat_recovery_eta: (old.heat_recovery_eta != new.heat_recovery_eta).then(|| new.heat_recovery_eta.clone()),
            fan_power_w: (old.fan_power_w != new.fan_power_w).then(|| new.fan_power_w.clone()),
        }),
        ..Default::default()
    })
}
