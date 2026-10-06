//! 📜️ Playground artifact — textual document grammar surface + laws.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;

/// 📄️ The `demo` example checkpoint.
pub const PLAYGROUND_DEMO_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses playground DSL text into a `PlaygroundSnapshot`.
pub fn parse_dsl(text: &str) -> Result<PlaygroundSnapshot, semio_framework_diagnostic::TextError> {
    <PlaygroundSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `PlaygroundSnapshot` back to DSL text.
pub fn print_dsl(snapshot: &PlaygroundSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type PlaygroundSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::PLAYGROUND_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;

impl store::ArtifactDsl for PlaygroundSnapshot {
    const EXTENSION: &'static str = "playground";
    fn envelope_id() -> &'static str {
        "playground.playground"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::change_schema::*;
use crate::standards::v1::subsets::any::schema::{diff::PlaygroundDiff, mutations::PlaygroundMutation, snapshot::PlaygroundSnapshot};
use semio_framework_pack_json::{array, from_dsl_value, from_json_str, object, to_string, Value};

/// 🔁️ Parses, prints, and reparses one language-neutral playground document.
pub fn round_trip_playground_dsl(text: &str) -> Result<String, String> {
    use store::ArtifactDsl;
    let parsed = <PlaygroundSnapshot as ArtifactDsl>::parse_dsl(text).map_err(|error| format!("the committed playground example does not parse: {error:?}"))?;
    let printed = <PlaygroundSnapshot as ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <PlaygroundSnapshot as ArtifactDsl>::parse_dsl(&printed).map_err(|error| format!("the reprinted playground document does not parse: {error:?}"))?;
    let value = object([("printed".to_string(), Value::String(printed)), ("snapshot".to_string(), from_dsl_value(&semio_framework_value::ToValue::to_value(&parsed))), ("reparsed".to_string(), from_dsl_value(&semio_framework_value::ToValue::to_value(&reparsed)))]);
    Ok(to_string(&value))
}
}
pub use mutations_codec::*;
