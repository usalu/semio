//! 🌡️ DIN 4108 app — document entities (constitutional: general).

pub use crate::artifacts::din4108::schema::diff::Din4108Diff;
pub use crate::artifacts::din4108::schema::mutations::Din4108Mutation;
pub use crate::artifacts::din4108::schema::snapshot::Din4108Snapshot;

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

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port —
/// lifted out of the pre-migration manifest's inline `.artifact_kind(ArtifactKindSpec { .. })` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("din4108", "DIN 4108")
}
//#endregion 🔖️ArtifactKind

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const DIN4108_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect {
    artifact_kind: "s.norm.din4108",
    standard: semio_framework_plugin::app::StandardId("1"),
    subset: semio_framework_plugin::app::SubsetId::ANY,
};
pub const DIN4108_DOCUMENT_SCHEMA: &str = "semio.norm.din4108/v1";

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use crate::artifacts::definition::{CapabilitySpec, ClaimSpec, LocalizationSpec};
    const SCHEMA: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.din4108" }];
    const INFERENCE: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.din4108.inference" }];
    const COMPOSER: &[ClaimSpec] = &[ClaimSpec { namespace: "dialect", value: "s.din4108@1/*" }];
    const CODEC: &[ClaimSpec] = &[ClaimSpec { namespace: "codec", value: "semio.norm.din4108/v1" }, ClaimSpec { namespace: "extension", value: "din4108" }];
    const EN: &[LocalizationSpec] = &[LocalizationSpec { locale: "en", text: "DIN 4108 thermal insulation and energy economy in buildings" }];
    const DE: &[LocalizationSpec] = &[LocalizationSpec { locale: "de", text: "DIN 4108 Wärme- und Feuchteschutz im Hochbau" }];
    const CAPABILITIES: &[CapabilitySpec] = &[
        CapabilitySpec { identity: "s.din4108.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.din4108.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.din4108.schema.artifact", kind: "schema", descriptor: "s.norm.din4108", claims: SCHEMA, localizations: &[] },
        CapabilitySpec { identity: "s.din4108.inference.outline", kind: "inference", descriptor: "s.norm.din4108.inference", claims: INFERENCE, localizations: &[] },
        CapabilitySpec { identity: "s.din4108.composer.any", kind: "composer", descriptor: "s.din4108@1/*", claims: COMPOSER, localizations: &[] },
        CapabilitySpec { identity: "s.din4108.grammar.document", kind: "grammar", descriptor: "din4108.document", claims: &[ClaimSpec { namespace: "grammar", value: "din4108.document" }], localizations: &[] },
        CapabilitySpec { identity: "s.din4108.grammar.op", kind: "grammar", descriptor: "din4108.op", claims: &[ClaimSpec { namespace: "grammar", value: "din4108.op" }], localizations: &[] },
        CapabilitySpec { identity: "s.din4108.grammar.diff", kind: "grammar", descriptor: "din4108.diff", claims: &[ClaimSpec { namespace: "grammar", value: "din4108.diff" }], localizations: &[] },
        CapabilitySpec { identity: "s.din4108.grammar.pack", kind: "grammar", descriptor: "din4108.pack", claims: &[ClaimSpec { namespace: "grammar", value: "din4108.pack" }], localizations: &[] },
        CapabilitySpec { identity: "s.din4108.grammar.spr", kind: "grammar", descriptor: "din4108.spr", claims: &[ClaimSpec { namespace: "grammar", value: "din4108.spr" }], localizations: &[] },
        CapabilitySpec { identity: "s.din4108.codec.document.v1", kind: "codec", descriptor: "semio.norm.din4108/v1:din4108", claims: CODEC, localizations: &[] },
        CapabilitySpec { identity: "s.din4108.localization.en", kind: "localization", descriptor: "DIN 4108 thermal insulation and energy economy in buildings", claims: &[], localizations: EN },
        CapabilitySpec { identity: "s.din4108.localization.de", kind: "localization", descriptor: "DIN 4108 Wärme- und Feuchteschutz im Hochbau", claims: &[], localizations: DE },
    ];
    crate::artifacts::definition::assemble_definition("s.din4108", CAPABILITIES)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::din4108::schema::din4108_artifact_schema_descriptor())
        .inferences([crate::artifacts::din4108::standards::v1::subsets::any::schema::inferences::din4108_artifact_inference_descriptor()])
        .composers(crate::artifacts::din4108::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::din4108::Din4108PlayApp>>()
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
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration
