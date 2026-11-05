//! 🧬️ EN 1992 checks core. Foundation: the catalogue case and outcome types, NDP tables, material properties and design-effect combinations.

#![allow(async_fn_in_trait)]
#![allow(unused_imports)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1992_model::{Anchor, En1992Snapshot, ExposureClass, FireRating, LoadCaseActions, MemberKind, RcMember, SupportCondition, TightnessClass};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, Station, SubjectRef};

/// 🔢 Value of one shared-registry NDP (`ndp.en1992.*`) for the annex; the registry is the only source of national parameters.
pub fn ndp(annex: AnnexChoice, id: &str) -> f64 {
    semio_s_plugin_norm_compliance::ndp_number(id, annex)
}

#[path = "../📇️catalogue/🦀️.rs"]
pub mod catalogue;
pub use catalogue::*;

#[path = "../🧱️materials-and-time/🦀️.rs"]
pub mod materials_and_time;
pub use materials_and_time::*;

#[path = "../🧮️design-effects/🦀️.rs"]
pub mod design_effects;
pub use design_effects::*;
