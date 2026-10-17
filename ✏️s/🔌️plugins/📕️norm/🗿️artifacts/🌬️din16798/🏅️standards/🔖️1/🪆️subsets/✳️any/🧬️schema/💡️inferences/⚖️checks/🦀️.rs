//! ⚖️ DIN EN 16798 compliance checks: the clause-by-clause verification behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, OccupancyType, Quantity, QuantityKind, Remedy, RemedyBound, SubjectRef};
use semio_s_artifact_norm_din16798_model::{VentSystemDocument, ZoneDocument};

//#region 🔖️ComplianceHelpers

#[path = "🎛️comfort-models-and-parsers/🦀️.rs"]
mod comfort_models_and_parsers;
pub use comfort_models_and_parsers::*;

#[path = "📜️annex-parameters-and-parts/🦀️.rs"]
mod annex_parameters_and_parts;
pub use annex_parameters_and_parts::*;

#[path = "🔎️environment-check-entry/🦀️.rs"]
mod environment_check_entry;
pub use environment_check_entry::*;

#[path = "🏠️zone-evaluation/🦀️.rs"]
mod zone_evaluation;
use zone_evaluation::*;

#[path = "🌀️vent-and-envelope-evaluation/🦀️.rs"]
mod vent_and_envelope_evaluation;
use vent_and_envelope_evaluation::*;

//#endregion 🔖️ComplianceHelpers
