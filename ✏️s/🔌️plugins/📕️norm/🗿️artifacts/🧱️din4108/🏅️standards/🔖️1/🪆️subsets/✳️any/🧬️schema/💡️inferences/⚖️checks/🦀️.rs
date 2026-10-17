//! ⚖️ DIN 4108 compliance checks: the clause-by-clause verification behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🔖️ComplianceHelpers
/// 📐️ Pure DIN 4108 / EN ISO 6946 helpers — formula library; snapshot evaluate lives in inferences.
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, ClauseId, ClimateZoneDe, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use semio_s_artifact_norm_din4108_model::{EnvelopeElement, LayerDocument, ThermalBridge, ThermalZone};

#[path = "🧮️resistance-helpers/🦀️.rs"]
mod resistance_helpers;
pub use resistance_helpers::*;

#[path = "🧱️part-2-minimum-insulation/🦀️.rs"]
pub mod part_2;

#[path = "💧️part-3-moisture-protection/🦀️.rs"]
pub mod part_3;

#[path = "📊️part-4-design-values/🦀️.rs"]
pub mod part_4;

#[path = "🌡️part-6-u-value-checks/🦀️.rs"]
pub mod part_6;

#[path = "🌬️part-7-airtightness/🦀️.rs"]
pub mod part_7;

#[path = "🌉️bb-2-thermal-bridges/🦀️.rs"]
pub mod bb_2;

#[path = "🔧️part-10-application-types/🦀️.rs"]
pub mod part_10;

//#endregion 🔖️ComplianceHelpers
