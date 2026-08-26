//! 🔺️ `change-residential-ventilation-m3-h` sparse diff construction — writes only `Din16798Diff.residential_ventilation_m3_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_residential_ventilation_m3_h::mutation::ChangeResidentialVentilationM3H;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeResidentialVentilationM3H, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_residential_ventilation_m3_h.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Residential ventilation air flow must be a finite number, got {}.", payload.new_residential_ventilation_m3_h), Vec::<String>::new());
    }
    if base.residential_ventilation_m3_h == payload.new_residential_ventilation_m3_h {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Residential ventilation air flow is already {}.", payload.new_residential_ventilation_m3_h));
    }
    protocol::MutationOutcome::new(Din16798Diff { residential_ventilation_m3_h: Some(payload.new_residential_ventilation_m3_h.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
