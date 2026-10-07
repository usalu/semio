//! 📜️ Forms artifact — textual document grammar surface + laws (constitutional: dsl). Ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (design.md §1 CORRECTION): `store::ArtifactDsl
//! for FormsSnapshot` now lives HERE (moved from `🧬️schema/📸️snapshot`, which keeps only the struct)
//! — the native codec is one bidirectional thing and sits directly under `🚪️io/<facet>/<representation>`,
//! unsplit. This component owns the real `parse_dsl`/`print_dsl` impl plus the thin artifact-facing
//! wrappers and the canonical example fixtures and their round-trip laws.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::FormsSnapshot;

//#region 🔖️HandcraftedArtifactDsl
/// ✉️ `ArtifactDsl` over the derived spec-driven text of `FormsSnapshot::__dsl_spec()`, the same record
/// the pack encodes; a parsed document must also pass `FormsSnapshot::validate`.
impl store::ArtifactDsl for FormsSnapshot {
    const EXTENSION: &'static str = "forms";
    fn envelope_id() -> &'static str {
        crate::FORMS_DOCUMENT_SCHEMA
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((envelope, rest)) => {
                if !envelope.matches_identity(Self::envelope_id(), store::semio_format::Component::Dsl, 1) {
                    return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Forms text envelope mismatch", semio_framework_diagnostic::TextSpan::at(1, 1)));
                }
                rest
            },
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &crate::standards::v1::subsets::any::io::binary::snapshot::pack::record_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        let snapshot = crate::standards::v1::subsets::any::io::binary::snapshot::pack::reconstruct_record(&record).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1)))?;
        snapshot.validate().map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&crate::standards::v1::subsets::any::io::binary::snapshot::pack::record(self).expect("valid Forms native state"), &crate::standards::v1::subsets::any::io::binary::snapshot::pack::record_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

/// 📄️ The building-component fixture, handcrafted in the `.forms` DSL.
pub const BUILDING_COMPONENT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📇️ The Contact template in the native saved-document format.
pub const DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/📇️contact/🗣️.dsl.semio");

/// 🌱️ The Onboarding template in the native saved-document format.
pub const ONBOARDING_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🌱️onboarding/🗣️.dsl.semio");

/// 📖️ Parses `.forms` DSL text into a `FormsSnapshot` — `FormsSnapshot`'s OWN persisted wire
/// format, the derived text of its own `dsl::DslRecord` spec.
pub fn parse_dsl(text: &str) -> Result<FormsSnapshot, semio_framework_diagnostic::TextError> {
    <FormsSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `FormsSnapshot` back to `.forms` DSL text.
pub fn print_dsl(document: &FormsSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ExternalBridges
/// 📖️ Parses `.forms` DSL text with a plain-`String` error, reachable from OUTSIDE this crate —
/// `store` is a private `extern crate` alias (`🦀️.rs`), so `store::TextError` cannot be named
/// by the exhaustive mutation case's test adapter that has to read the committed
/// `🗣️.dsl.semio` artifact.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn parse_forms_dsl(text: &str) -> Result<FormsSnapshot, String> {
    <FormsSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 🖨️ Prints a [`FormsSnapshot`] back to `.forms` DSL text under a name an external caller can reach, paired
/// with [`parse_forms_dsl`].
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn print_forms_dsl(snapshot: &FormsSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🔖️ExternalBridges

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::{FormsDiff, FormsSnapshot};
use protocol::Mutation;

/// 📥️ Decodes a committed `📸️snapshot/{⬅️before,➡️after}/🔣️.json` vector.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_form_snapshot_json(text: &str) -> Result<FormsSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.into_message())
}

/// 📤️ The snapshot as the same canonical JSON the committed vectors are written in — the
/// projection an external test host compares through.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn encode_form_snapshot_json(snapshot: &FormsSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}
}
pub use mutations_codec::*;

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::*;
use crate::op::FormMutation;
use crate::standards::v1::subsets::any::io::text::snapshot as forms_dsl;
use crate::{forms_snapshot_with_state, forms_steps, FormsResultsChild, FormsSnapshot, FormsStructureChild, FORMS_DOCUMENT_SCHEMA};
use semio_framework_pack_json::{Object, Value};
use framework_schema::ArtifactSchema;
use validation::{can_advance, step_errors};
/// 🌿️ Pure compute over the shared `playbook` kernel crate's step/block domain, re-exported here under
/// forms' historical names (relocated from the deleted `⚙️engine`, ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
use crate::playbook::{
    default_value_for_block as default_value_for_question, eval_playbook_expr as eval_form_expr, find_block_location as find_question_location, flatten_playbook_blocks as flatten_form_questions, is_block_visible as is_question_visible,
    is_extension_block_kind as is_extension_question_kind, visible_blocks as visible_questions,
};
use crate::FormQuestion;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::FormStep;

/// 📇️ Loads the canonical Contact template.
pub fn default_example_spec() -> FormsSnapshot {
    forms_dsl::parse_dsl(forms_dsl::DEFAULT_EXAMPLE_TEXT).expect("bundled Forms template must be valid")
}

/// 📄️ JSON re-serialization of [`default_example_spec`], for the framework-generic call sites that
/// contractually require JSON text (`App::example`'s manifest `document_json`).
pub fn default_example_json() -> String {
    semio_framework_pack_json::to_json_string(&default_example_spec())
}

/// 📄️ The `onboarding` example, parsed once from `forms_dsl::ONBOARDING_EXAMPLE_TEXT`.
pub fn onboarding_example_spec() -> FormsSnapshot {
    forms_dsl::parse_dsl(forms_dsl::ONBOARDING_EXAMPLE_TEXT).expect("bundled Forms template must be valid")
}

/// 📄️ JSON re-serialization of [`onboarding_example_spec`], for the framework-generic call sites that
/// contractually require JSON text (`App::example`'s manifest `document_json`).
pub fn onboarding_example_json() -> String {
    semio_framework_pack_json::to_json_string(&onboarding_example_spec())
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use crate::standards::v1::subsets::any::schema::*;
use crate::op::FormMutation;
use crate::standards::v1::subsets::any::io::text::snapshot as forms_dsl;
use crate::{forms_snapshot_with_state, forms_steps, FormsResultsChild, FormsSnapshot, FormsStructureChild, FORMS_DOCUMENT_SCHEMA};
use semio_framework_pack_json::{Object, Value};
use framework_schema::ArtifactSchema;
use validation::{can_advance, step_errors};
/// 🌿️ Pure compute over the shared `playbook` kernel crate's step/block domain, re-exported here under
/// forms' historical names (relocated from the deleted `⚙️engine`, ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
use crate::playbook::{
    default_value_for_block as default_value_for_question, eval_playbook_expr as eval_form_expr, find_block_location as find_question_location, flatten_playbook_blocks as flatten_form_questions, is_block_visible as is_question_visible,
    is_extension_block_kind as is_extension_question_kind, visible_blocks as visible_questions,
};
use crate::FormQuestion;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::FormStep;

/// 🌱️ The forms app's default document — the building-component fixture, seeded from its derive-
/// generated `.forms` DSL text.
pub fn building_component_spec() -> FormsSnapshot {
    forms_dsl::parse_dsl(forms_dsl::BUILDING_COMPONENT_EXAMPLE_TEXT).expect("bundled Forms template must be valid")
}

}
pub use snapshot_wire_codec::*;



#[path="📨️response/🦀️.rs"]
pub mod response;

mod field_native_codec {
use crate::FormsSnapshot;
use semio_framework_pack_json::{Object,Value};
pub fn initial_try_values(spec: &FormsSnapshot, overrides: &Object) -> Object {
    let overrides_map: crate::playbook::PlaybookValues = overrides.iter().map(|(key, value)| (key.to_string(), semio_framework_pack_json::to_dsl_value(value))).collect();
    let result = crate::playbook::initial_values(&crate::mutations::as_playbook_spec(spec), &overrides_map);
    result.iter().map(|(key, value)| (key.clone(), semio_framework_pack_json::from_dsl_value(value))).collect()
}

pub fn value_to_dsl(value: &Value) -> semio_framework_value::DslValue {
    semio_framework_pack_json::to_dsl_value(value)
}

pub fn dsl_to_value(value: &semio_framework_value::DslValue) -> Value {
    semio_framework_pack_json::from_dsl_value(value)
}

pub fn dsl_string_value(value: &semio_framework_value::DslValue) -> String {
    json_string_value(&dsl_to_value(value))
}

pub fn dsl_f64_value(value: &semio_framework_value::DslValue) -> f64 {
    json_f64_value(&dsl_to_value(value))
}

pub fn json_string_value(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(semio_framework_pack_json::Number::UInt(v)) => v.to_string(),
        Value::Number(semio_framework_pack_json::Number::Int(v)) => v.to_string(),
        Value::Number(semio_framework_pack_json::Number::Float(v)) => v.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

pub fn json_f64_value(value: &Value) -> f64 {
    value.as_f64().unwrap_or(0.0)
}
}
pub use field_native_codec::{initial_try_values,value_to_dsl,dsl_to_value,dsl_string_value,dsl_f64_value,json_string_value,json_f64_value};
