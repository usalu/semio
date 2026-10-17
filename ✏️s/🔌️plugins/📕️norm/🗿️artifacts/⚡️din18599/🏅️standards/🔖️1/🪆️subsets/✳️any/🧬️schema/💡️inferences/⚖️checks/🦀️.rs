//! ⚖️ DIN V 18599 compliance checks: the clause-by-clause verification behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_din18599_model::{Adjacency, Attachment, AutomationClass, BuildingCategory, CalculationMethod, EnvelopeElement, MonthlyClimate, ThermalZone, UsageProfile, UseClass};

//#region 🔖️ComplianceHelpers
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use semio_s_artifact_norm_din18599_model::{Din18599Snapshot, ElementKind};

#[path = "🧮️balance-model/🦀️.rs"]
mod balance_model;
pub use balance_model::*;

#[path = "📜️norm-tables/🦀️.rs"]
mod norm_tables;
pub use norm_tables::*;

#[path = "🔥️derived-balance/🦀️.rs"]
mod derived_balance;
pub use derived_balance::*;

//#endregion 🔖️ComplianceHelpers

#[path = "🩺️document-evaluation/🦀️.rs"]
mod document_evaluation;
pub use document_evaluation::*;

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "../../🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;

//#endregion 🧪️ComplianceTests
