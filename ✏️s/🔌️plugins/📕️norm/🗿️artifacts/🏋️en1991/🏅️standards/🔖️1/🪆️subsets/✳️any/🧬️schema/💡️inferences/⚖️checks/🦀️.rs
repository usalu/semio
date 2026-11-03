//! ⚖️ EN 1991 compliance checks: the clause-by-clause verification behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;

//#region 🔖️ComplianceHelpers
use semio_s_plugin_norm_compliance::{CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};

pub use semio_s_artifact_norm_en1991_checks_parts::*;

pub use semio_s_artifact_norm_en1991_spec_trace as spec_trace;

#[path = "📇️catalogue/🦀️.rs"]
pub mod catalogue;
//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "../../🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
#[cfg(test)]
#[path = "../../🧪️tests/📇️catalogue/🦀️.rs"]
mod catalogue_tests;
#[cfg(test)]
#[path = "../../🧪️tests/🔮️catalogue-oracle/🦀️.rs"]
mod catalogue_oracle_tests;
//#endregion 🧪️ComplianceTests
