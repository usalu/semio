//! ⚖️ EN 1992 compliance checks: the clause-by-clause verification behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1992_model::{Anchor, En1992Snapshot, ExposureClass, FireRating, LoadCaseActions, MemberKind, RcMember, SupportCondition, TightnessClass};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};

//#region 🔖️ComplianceHelpers

// #region 🔖️NaDe

#[path = "🧰️helpers-and-annex-parts/🦀️.rs"]
mod helpers_and_annex_parts;
pub use helpers_and_annex_parts::*;

//#region 🔖️MemberEvaluate

#[path = "🧮️design-effects/🦀️.rs"]
mod design_effects;
pub use design_effects::*;

#[path = "📋️member-evaluation/🦀️.rs"]
mod member_evaluation;
pub use member_evaluation::*;

#[path = "⚓️anchor-evaluation/🦀️.rs"]
mod anchor_evaluation;
pub use anchor_evaluation::*;

//#endregion 🔖️MemberEvaluate

/// 🧭 Field metadata hook for structured inputs editor (B2).
/// 🏷️ Prefer [`crate::field_meta::en1992_field_meta`] (B2 NormFieldMetaFn).
pub fn field_metadata() -> &'static [(&'static str, &'static str, &'static str, &'static str)] {
    &[]
}
