//! ⚖️ EN 1993 compliance checks: the clause-by-clause verification behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1993_model::{LoadCase, SteelJoint, SteelMaterial, SteelMember, SteelSection, TowerLeg};
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use semio_s_artifact_norm_en1993_model::rolled_heb_catalogue;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub use semio_s_artifact_norm_en1993_checks_special::*;

#[path = "🧱️member-checks/🦀️.rs"]
mod member_checks;
pub use member_checks::*;

#[path = "🧮️units/🦀️.rs"]
pub mod units;
pub use units::En1993Units;

pub use semio_s_artifact_norm_en1993_spec_trace as spec_trace;


#[cfg(test)]
#[path = "../../🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;

#[cfg(test)]
#[path = "🧪️tests/🔭️special-wiring/🦀️.rs"]
mod special_wiring_tests;

#[cfg(test)]
#[path = "🧪️tests/🧩️plate-wiring/🦀️.rs"]
mod plate_wiring_tests;

#[cfg(test)]
#[path = "🧪️tests/🧮️norm-wiring/🦀️.rs"]
mod norm_wiring_tests;

#[cfg(test)]
#[path = "🧪️tests/🏛️annex-blocks/🦀️.rs"]
mod annex_block_tests;

#[cfg(test)]
#[path = "🧮️clauses/🧪️tests/🔭️special-parts/🦀️.rs"]
mod special_parts_tests;

#[cfg(any(test, feature = "oracle-wiring"))]
#[path = "🧪️tests/🎯️oracle-wiring/🦀️.rs"]
pub mod oracle_wiring;

#[cfg(test)]
#[path = "🧪️tests/🗂️catalogue-trace/🦀️.rs"]
mod catalogue_trace_tests;

#[path = "📇️catalogue/🦀️.rs"]
pub mod case_catalogue;
