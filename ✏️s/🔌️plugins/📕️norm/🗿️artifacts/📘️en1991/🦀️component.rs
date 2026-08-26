//! En1991 — document entities (constitutional: general).

use serde::{Deserialize, Serialize};

//#region 🔖️Types
pub mod part_1_2 {
    use super::*;

    /// 🔥️ Nominal fire exposure curve per EN 1991-1-2 §3.2/Annex B.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum FireCurve {
        Standard,
        External,
        Hydrocarbon,
    }
}

/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1991", "EN 1991")
}
//#endregion 🔖️ArtifactKind

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1991_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1991", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
pub const EN1991_DOCUMENT_SCHEMA: &str = "semio.norm.en1991/v1";

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use crate::artifacts::definition::{CapabilitySpec, ClaimSpec, LocalizationSpec};
    const SCHEMA: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1991" }];
    const INFERENCE: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1991.inference" }];
    const COMPOSER: &[ClaimSpec] = &[ClaimSpec { namespace: "dialect", value: "s.en1991@1/*" }];
    const CODEC: &[ClaimSpec] = &[ClaimSpec { namespace: "codec", value: "semio.norm.en1991/v1" }, ClaimSpec { namespace: "extension", value: "en1991" }];
    const EN: &[LocalizationSpec] = &[LocalizationSpec { locale: "en", text: "EN 1991 actions on structures" }];
    const DE: &[LocalizationSpec] = &[LocalizationSpec { locale: "de", text: "EN 1991 Einwirkungen auf Tragwerke" }];
    const CAPABILITIES: &[CapabilitySpec] = &[
        CapabilitySpec { identity: "s.en1991.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.en1991.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.en1991.schema.artifact", kind: "schema", descriptor: "s.norm.en1991", claims: SCHEMA, localizations: &[] },
        CapabilitySpec { identity: "s.en1991.inference.outline", kind: "inference", descriptor: "s.norm.en1991.inference", claims: INFERENCE, localizations: &[] },
        CapabilitySpec { identity: "s.en1991.composer.any", kind: "composer", descriptor: "s.en1991@1/*", claims: COMPOSER, localizations: &[] },
        CapabilitySpec { identity: "s.en1991.grammar.document", kind: "grammar", descriptor: "en1991.document", claims: &[ClaimSpec { namespace: "grammar", value: "en1991.document" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1991.grammar.op", kind: "grammar", descriptor: "en1991.op", claims: &[ClaimSpec { namespace: "grammar", value: "en1991.op" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1991.grammar.diff", kind: "grammar", descriptor: "en1991.diff", claims: &[ClaimSpec { namespace: "grammar", value: "en1991.diff" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1991.grammar.pack", kind: "grammar", descriptor: "en1991.pack", claims: &[ClaimSpec { namespace: "grammar", value: "en1991.pack" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1991.grammar.spr", kind: "grammar", descriptor: "en1991.spr", claims: &[ClaimSpec { namespace: "grammar", value: "en1991.spr" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1991.codec.document.v1", kind: "codec", descriptor: "semio.norm.en1991/v1:en1991", claims: CODEC, localizations: &[] },
        CapabilitySpec { identity: "s.en1991.localization.en", kind: "localization", descriptor: "EN 1991 actions on structures", claims: &[], localizations: EN },
        CapabilitySpec { identity: "s.en1991.localization.de", kind: "localization", descriptor: "EN 1991 Einwirkungen auf Tragwerke", claims: &[], localizations: DE },
    ];
    crate::artifacts::definition::assemble_definition("s.en1991", CAPABILITIES)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::en1991::schema::en1991_artifact_schema_descriptor())
        .inferences([crate::artifacts::en1991::standards::v1::subsets::any::schema::inferences::en1991_artifact_inference_descriptor()])
        .composers(crate::artifacts::en1991::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::en1991::En1991PlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention below.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "en1991.document",
                    extension: Some("en1991"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::en1990::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1990::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1990::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1990::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1991.document"),
                },
                dsl::LanguageSpec {
                    id: "en1991.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::en1990::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1990::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1990::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1990::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1991.op"),
                },
                dsl::LanguageSpec {
                    id: "en1991.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::en1990::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1990::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("en1991.diff"),
                },
                dsl::LanguageSpec {
                    id: "en1991.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1990::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1990::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1991.pack"),
                },
                dsl::LanguageSpec {
                    id: "en1991.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1990::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1990::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1991.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration
