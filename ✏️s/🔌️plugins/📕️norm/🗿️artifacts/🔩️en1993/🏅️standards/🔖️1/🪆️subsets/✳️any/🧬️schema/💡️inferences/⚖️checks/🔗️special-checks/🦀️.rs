//! 🔗️ EN 1993 special element checks: joints, fatigue, fire, cold-formed members, plated panels, shells, tension components, bridges, towers, piles, crane runways and the special parts of EN 1993-1-3 to 6.

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

#[path = "🔭️special-parts-checks/🦀️.rs"]
mod special_parts_checks;

#[path = "🏛️annex-block-checks/🦀️.rs"]
mod annex_block_checks;
use annex_block_checks::check_annex_blocks;

pub use semio_s_artifact_norm_en1993_clauses_annex as clauses_annex;
pub use semio_s_artifact_norm_en1993_checks_special_frames::*;

#[path = "🔩️joint-row-checks/🦀️.rs"]
pub mod joint_row_checks;
use joint_row_checks::check_joint_rows;
pub use joint_row_checks::{joint_row_summary, JointRowSummary};

//#region 🔖️ComplianceHelpers
/// 🔩 The joint, fatigue, fire, cold-formed, stainless, plated, shell, through-thickness, tension, high-strength, bridge, tower, pile and crane checks of `check_full_steel_structure`, appended to the same report.
pub fn check_special_elements(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, scope: Scope<'_>, report: &mut CheckReport) {
    check_joints(document, annex, params, scope, report);
    check_fatigue_and_fire(document, annex, params, scope, report);
    check_cold_formed_stainless_and_plated(document, annex, params, scope, report);
    check_shell_and_member_properties(document, annex, params, scope, report);
    check_bridge_tower_pile_and_crane(document, annex, params, scope, report);
    check_storeys(document, annex, scope, report);
    check_frames(document, annex, scope, report);
    check_restraints(document, annex, params, scope, report);
    check_built_up_members(document, annex, params, scope, report);
    check_weld_groups(document, annex, params, scope, report);
    check_joint_plates(document, annex, params, scope, report);
    check_foundation_blocks(document, annex, params, scope, report);
    check_parametric_sections(document, annex, scope, report);
    check_hollow_joints(document, annex, scope, report);
    check_joint_rows(document, annex, params, scope, report);
    check_annex_blocks(document, annex, params, scope, report);
    check_buckling_lengths(document, annex, scope, report);
    check_stress_ranges(document, annex, scope, report);
    check_fastened_joints(document, annex, params, scope, report);
    check_forming_gains(document, annex, scope, report);
    check_ductility_gate(document, annex, scope, report);
}
//#endregion 🔖️ComplianceHelpers
