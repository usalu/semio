//! 🧾️ Catalogue-gap checks: the formula-based items of the `V-xxx` catalogue that no earlier check family carried (material grade, ductility and constants, bow imperfections, shear area, torsional buckling, rolled and simplified LTB, floor frequency, block tearing, punching, flange welds, constant-amplitude fatigue, fire LTB and interaction, flange curling). See [EN 1993-1](https://eurocodes.jrc.ec.europa.eu/).

use super::*;

#[path = "🧱️member-vids/🦀️.rs"]
mod member_vids;
pub use member_vids::{torsional_summary, TorsionalSummary};

#[path = "🔩️joint-vids/🦀️.rs"]
mod joint_vids;

#[path = "🧪️fatigue-fire-vids/🦀️.rs"]
mod other_vids;
pub use other_vids::{fire_ltb_summary, FireLtbSummary};

//#region 🔖️ComplianceHelpers
/// 🧾️ Appends the catalogue-gap checks in the order of the incremental units `vidMaterials`, `vidMembers`, `vidJoints`, `vidFatigue`, `vidFire` and `vidColdFormed`.
pub(super) fn check_vids(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, scope: Scope<'_>, report: &mut CheckReport) {
    other_vids::check_materials(document, annex, scope, report);
    member_vids::check_members(document, annex, params, scope, report);
    joint_vids::check_joints(document, annex, params, scope, report);
    other_vids::check_fatigue(document, annex, scope, report);
    other_vids::check_fire(document, annex, scope, report);
    other_vids::check_cold_formed(document, annex, scope, report);
}
//#endregion 🔖️ComplianceHelpers
