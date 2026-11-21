//! ⚖️ DIN V 18599 compliance checks: the clause-by-clause verification behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_din18599_model::{Adjacency, AutomationClass, BuildingCategory, CalculationMethod, EnvelopeElement, MonthlyClimate, ThermalMassClass, ThermalZone, UsageProfile, UseClass};

//#region 🔖️ComplianceHelpers
use semio_s_plugin_norm_compliance::incremental_evaluation::Scope;
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use semio_s_artifact_norm_din18599_model::{Din18599Snapshot, ElementKind};

#[path = "🧮️balance-model/🦀️.rs"]
mod balance_model;
pub use balance_model::*;

#[path = "🏘️residential-gains/🦀️.rs"]
mod residential_gains;
pub use residential_gains::*;

#[path = "📜️norm-tables/🦀️.rs"]
mod norm_tables;
pub use norm_tables::*;

#[path = "☀️potsdam-irradiance/🦀️.rs"]
mod potsdam_irradiance;
pub use potsdam_irradiance::potsdam_irradiance_w_m2;

#[path = "🏢annex-2-reference/🦀️.rs"]
mod annex_2_reference;
pub use annex_2_reference::*;

#[path = "🏢annex-3-envelope/🦀️.rs"]
mod annex_3_envelope;
pub use annex_3_envelope::*;

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
