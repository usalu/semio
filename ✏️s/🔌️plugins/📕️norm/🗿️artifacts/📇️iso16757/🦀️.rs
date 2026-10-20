//! 📦️ ISO 16757 building-services product catalogue: parts 1, 2, 4, 5 — document entities.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.

use std::collections::BTreeMap;
use semio_s_artifact_norm_iso16757_model::CatalogueValue;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

// #region Shared

/// 🌐️ Locale-tagged text — re-exported from `crate::document`, the single canonical definition
/// shared across every norm artifact (kills this type's former duplicate here and in `vdi3805`;
/// see `crate::document`'s `🔖️LocalizedText` region for the full rationale).

/// 🔗️ Internal or external reference.
#[derive(Clone, Debug, PartialEq, crate::value_derive::ToValue, crate::value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CatalogueReference {
    pub uri: String,
    pub label: Option<String>,
}

// #endregion Shared

// #region Part1
// #endregion Part1

// #region Part2
// #endregion Part2

// #region Part4
// #endregion Part4

// #region Part5
// #endregion Part5

// #region Session
/// 🏷️ Canonical DSL file extension for ISO 16757 documents.
pub const ISO16757_EXTENSION: &str = "iso16757";

// #endregion Session

//#region 🪪️Declaration

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(semio_s_artifact_norm_iso16757_schema::iso16757_artifact_schema_descriptor())
        .inferences([semio_s_artifact_norm_iso16757_inferences::iso16757_artifact_inference_descriptor()])
        .composers(semio_s_artifact_norm_iso16757_io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_iso16757_editor::Iso16757PlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention below.
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "iso16757.document",
                    extension: Some("iso16757"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_norm_iso16757_io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_iso16757_io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_iso16757_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_iso16757_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("iso16757.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "iso16757.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_norm_iso16757_io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_iso16757_io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_iso16757_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_iso16757_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("iso16757.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "iso16757.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_norm_iso16757_io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_iso16757_io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("iso16757.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "iso16757.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_iso16757_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_iso16757_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("iso16757.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "iso16757.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_iso16757_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_iso16757_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("iso16757.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration

// ---- Shims: keep pre-migration module paths resolving for external callers ----

//#region 🪢️Mounts
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
mod diff_unit;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs"]
mod inferences_compliance_report;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🪢️Mounts
