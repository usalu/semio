//! ⚖️ EN 1999 compliance checks: the clause-by-clause verification behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1999_model::{AluminiumMaterial, AluminiumMember, AluminiumSection, ColdFormedSheet, En1999Snapshot, FatigueCategory, FatigueDetail, PatchLoad, RingStiffener, TransverseStiffener};
use semio_s_plugin_norm_compliance::incremental_evaluation::Scope;
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Remedy, SubjectRef};

//#region 🔖️ComplianceHelpers

pub use semio_s_artifact_norm_en1999_checks_members::*;

#[path = "../../../🧭️spec-trace/🦀️.rs"]
pub mod spec_trace;

#[path = "🎯️entity-checks/🦀️.rs"]
mod entity_checks;
pub use entity_checks::*;

#[path = "🧾️structure-evaluation/🦀️.rs"]
mod structure_evaluation;
pub use structure_evaluation::*;

#[path = "🧮️units/🦀️.rs"]
mod units;
pub use units::*;

//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests
#[cfg(any(test, feature = "oracle-wiring"))]
#[path = "🧪️tests/🎯️oracle-wiring/🦀️.rs"]
pub mod oracle_wiring;
#[cfg(test)]
#[path = "🧪️tests/📊️fatigue-spectrum/🦀️.rs"]
mod fatigue_spectrum;
#[cfg(test)]
#[path = "🧪️tests/🎯️wave5-references/🦀️.rs"]
mod wave5_references;
//#endregion ️ComplianceTests
