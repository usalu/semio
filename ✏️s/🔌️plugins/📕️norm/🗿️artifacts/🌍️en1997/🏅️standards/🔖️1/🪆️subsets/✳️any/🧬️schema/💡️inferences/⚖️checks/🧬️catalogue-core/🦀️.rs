//! 📇️ EN 1997 checks catalogue: the geotechnical verification catalogue and the design-approach parameters behind it.

#![allow(async_fn_in_trait)]
#![allow(unused_imports)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::incremental_evaluation::Scope;
use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1997_model::{En1997Snapshot, FoundationLoadCase, SoilLayer, SpreadFoundation};

use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};

/// 🌐️ Bilingual copy (English, German) shared by the catalogue and the clause modules of the checks crate.
pub fn loc(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }

#[path = "../🧭️design-approach-and-parameters/🦀️.rs"]
mod design_approach_and_parameters;
pub use design_approach_and_parameters::*;

#[path = "../📇️catalogue/🦀️.rs"]
pub mod catalogue;
