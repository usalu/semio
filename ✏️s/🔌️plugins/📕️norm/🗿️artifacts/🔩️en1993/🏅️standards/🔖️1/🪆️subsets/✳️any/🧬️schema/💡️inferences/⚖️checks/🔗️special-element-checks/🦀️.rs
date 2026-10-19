//! 🔗️ EN 1993 special element checks: joints, fatigue, fire, cold-formed members, plated panels, shells, tension components, bridges, towers, piles and crane runways.

use super::*;

#[path = "🔩️joint-checks/🦀️.rs"]
mod joint_checks;
use joint_checks::check_joints;

#[path = "⏳️fatigue-fire-and-thin-walled-checks/🦀️.rs"]
mod fatigue_fire_and_thin_walled_checks;
use fatigue_fire_and_thin_walled_checks::{check_fatigue_and_fire, check_cold_formed_stainless_and_plated};

#[path = "🏛️shell-and-structure-checks/🦀️.rs"]
mod shell_and_structure_checks;
use shell_and_structure_checks::{check_shell_and_member_properties, check_bridge_tower_pile_and_crane};

//#region 🔖️ComplianceHelpers
/// 🔩 The joint, fatigue, fire, cold-formed, stainless, plated, shell, through-thickness, tension, high-strength, bridge, tower, pile and crane checks of `check_full_steel_structure`, appended to the same report.
pub(super) fn check_special_elements(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, report: &mut CheckReport) {
    check_joints(document, annex, params, report);
    check_fatigue_and_fire(document, annex, params, report);
    check_cold_formed_stainless_and_plated(document, annex, params, report);
    check_shell_and_member_properties(document, annex, params, report);
    check_bridge_tower_pile_and_crane(document, annex, params, report);
}
//#endregion 🔖️ComplianceHelpers
