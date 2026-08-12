//! 🌡️ DIN 4108 app — document entities (constitutional: general).


pub use crate::artifacts::din4108::schema::snapshot::Din4108Snapshot;
pub use crate::artifacts::din4108::schema::mutations::Din4108Mutation;
pub use crate::artifacts::din4108::schema::diff::Din4108Diff;

use crate::document::ClimateZoneDe;
use serde::{Deserialize, Serialize};

// #region 🔖️Types
// No `#[dsl(keyword = ...)]`: reached only through the plain, un-tagged `Vec<LayerDocument>`
// list on `Document::layers` — same reasoning as `draw`'s `GradientStop`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct LayerDocument {
    #[dsl(positional, unit = "m")]
    pub thickness_m: f64,
    #[dsl(positional)]
    pub lambda_w_mk: f64,
}


/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
//#endregion 🔖️Types

// `)` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("din4108", "DIN 4108")
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::din4108::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("Din4108Composer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.din4108")
        .schema(crate::artifacts::din4108::schema::din4108_artifact_schema_descriptor())
        .inferences([crate::artifacts::din4108::standards::v1::subsets::any::schema::inferences::din4108_artifact_inference_descriptor()])
        .composers(crate::artifacts::din4108::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention below.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES.get_or_init(|| vec![
        dsl::LanguageSpec {
            id: "din4108.document",
            extension: Some("din4108"),
            role: dsl::LanguageRole::Document,
            grammar: Some(crate::artifacts::din4108::dsl::COMPONENT_GRAMMAR_SEMIO),
            grammar_path: Some(crate::artifacts::din4108::dsl::COMPONENT_GRAMMAR_PATH),
            protocol: Some(crate::artifacts::din4108::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::din4108::snapshot::pack::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("din4108.document"),
        },
        dsl::LanguageSpec {
            id: "din4108.op",
            extension: None,
            role: dsl::LanguageRole::Ops,
            grammar: Some(crate::artifacts::din4108::op::COMPONENT_GRAMMAR_SEMIO),
            grammar_path: Some(crate::artifacts::din4108::op::COMPONENT_GRAMMAR_PATH),
            protocol: Some(crate::artifacts::din4108::spr::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::din4108::spr::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("din4108.op"),
        },
        dsl::LanguageSpec {
            id: "din4108.diff",
            extension: None,
            role: dsl::LanguageRole::Diff,
            grammar: Some(crate::artifacts::din4108::diff::COMPONENT_GRAMMAR_SEMIO),
            grammar_path: Some(crate::artifacts::din4108::diff::COMPONENT_GRAMMAR_PATH),
            protocol: None,
            protocol_path: None,
            hooks: dsl::passthrough_hooks("din4108.diff"),
        },
        dsl::LanguageSpec {
            id: "din4108.pack",
            extension: None,
            role: dsl::LanguageRole::Pack,
            grammar: None,
            grammar_path: None,
            protocol: Some(crate::artifacts::din4108::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::din4108::snapshot::pack::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("din4108.pack"),
        },
        dsl::LanguageSpec {
            id: "din4108.spr",
            extension: None,
            role: dsl::LanguageRole::Spr,
            grammar: None,
            grammar_path: None,
            protocol: Some(crate::artifacts::din4108::spr::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::din4108::spr::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("din4108.spr"),
        },
    ]).as_slice()
}
//#endregion 🪪️Declaration
