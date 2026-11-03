//! 🕸️ EN 1993 special frame checks: second-order frames, storeys, lateral restraints, thin-walled and built-up sections, weld groups, parametric sections and hollow joints.

#![allow(async_fn_in_trait)]
#![allow(unused_imports)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1993_model::{LoadCase, SteelJoint, SteelMaterial, SteelMember, SteelSection};
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use semio_s_artifact_norm_en1993_model::rolled_heb_catalogue;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub use semio_s_artifact_norm_en1993_checks_core::*;
pub use semio_s_artifact_norm_en1993_frame_eigen as frame_eigen;

#[path = "../🔗️special-checks/🔗️joint-frame/🦀️.rs"]
pub mod joint_frame;

#[path = "../🔗️special-checks/🕸️frame-checks/🦀️.rs"]
pub mod frame_checks;
pub use frame_checks::check_frames;

#[path = "../🔗️special-checks/🌐️storey-checks/🦀️.rs"]
pub mod storey_checks;
pub use storey_checks::check_storeys;

#[path = "../🔗️special-checks/🧷️restraint-checks/🦀️.rs"]
pub mod restraint_checks;
pub use restraint_checks::check_restraints;
pub use restraint_checks::{restraint_summary, RestraintSummary};

#[path = "../🔗️special-checks/🥫️thin-walled-section-checks/🦀️.rs"]
pub mod thin_walled_section_checks;
pub use thin_walled_section_checks::check_cold_formed_section;

#[path = "../🔗️special-checks/🪜️built-up-checks/🦀️.rs"]
pub mod built_up_checks;
pub use built_up_checks::check_built_up_members;

#[path = "../🔗️special-checks/🧵️weld-group-checks/🦀️.rs"]
pub mod weld_group_checks;
pub use weld_group_checks::check_weld_groups;

#[path = "../🔗️special-checks/🧩️plate-checks/🦀️.rs"]
pub mod plate_checks;

#[path = "../🔗️special-checks/🔩️bolt-group-ic-checks/🦀️.rs"]
pub mod bolt_group_ic_checks;

#[path = "../🔗️special-checks/🪜️effective-length-checks/🦀️.rs"]
pub mod effective_length_checks;
pub use effective_length_checks::check_buckling_lengths;

#[path = "../🔗️special-checks/⏳️stress-range-checks/🦀️.rs"]
pub mod stress_range_checks;
pub use stress_range_checks::check_stress_ranges;

#[path = "../🔗️special-checks/🪛️fastener-checks/🦀️.rs"]
pub mod fastener_checks;
pub use fastener_checks::check_fastened_joints;

#[path = "../🔗️special-checks/🥫️forming-gain-checks/🦀️.rs"]
pub mod forming_gain_checks;
pub use forming_gain_checks::check_forming_gains;

#[path = "../🔗️special-checks/🧱️plastic-gate-checks/🦀️.rs"]
pub mod plastic_gate_checks;
pub use plastic_gate_checks::check_ductility_gate;
pub use plate_checks::{check_foundation_blocks, check_joint_plates};

#[path = "../🔗️special-checks/📐️parametric-checks/🦀️.rs"]
pub mod parametric_checks;
pub use parametric_checks::check_parametric_sections;

#[path = "../🔗️special-checks/🕳️hollow-joint-checks/🦀️.rs"]
pub mod hollow_joint_checks;
pub use hollow_joint_checks::check_hollow_joints;
