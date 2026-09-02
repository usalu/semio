//! 🧱️ EN 1996 app — document entities (constitutional: general).


//#region 🔖️Types
/// 🧱️ Masonry manufacturing-control class underlying the EN-recommended γ_M table (EN 1996-1-1 Table 2.1-style).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum MasonryClass {
    Class1,
    Class2,
    #[default]
    Class3,
    Class4,
    Class5,
}

impl MasonryClass {
    pub fn gamma_m_en(self) -> f64 {
        match self {
            Self::Class1 => 1.5,
            Self::Class2 => 1.7,
            Self::Class3 => 2.0,
            Self::Class4 => 2.2,
            Self::Class5 => 2.5,
        }
    }
}

pub mod part_2 {

    /// 🌦️ Masonry durability exposure class (EN 1996-1-1 Annex B-style categorisation MX1–MX5).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub enum ExposureClass {
        Mx1,
        Mx2,
        Mx3,
        Mx4,
        Mx5,
    }

    /// 🧪️ General-purpose mortar compressive-strength class per EN 998-2.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub enum MortarClass {
        M1,
        /// 🔡️ `M2_5` auto-kebabs to `m2-5` (digit-underscore-digit), but the standard's own class
        /// label is `M2.5`/`M2_5` with no internal dash — kept as a genuine rename.
        #[dsl(key = "m2_5")]
        M2_5,
        M5,
        M10,
        M20,
    }

    impl MortarClass {
        pub fn compressive_strength_mpa(self) -> f64 {
            match self {
                Self::M1 => 1.0,
                Self::M2_5 => 2.5,
                Self::M5 => 5.0,
                Self::M10 => 10.0,
                Self::M20 => 20.0,
            }
        }
    }
}

pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1996", "EN 1996")
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1996_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1996", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
pub const EN1996_DOCUMENT_SCHEMA: &str = "semio.norm.en1996/v1";

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use crate::artifacts::definition::{CapabilitySpec, ClaimSpec, LocalizationSpec};
    const SCHEMA: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1996" }];
    const INFERENCE: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1996.inference" }];
    const COMPOSER: &[ClaimSpec] = &[ClaimSpec { namespace: "dialect", value: "s.en1996@1/*" }];
    const CODEC: &[ClaimSpec] = &[ClaimSpec { namespace: "codec", value: "semio.norm.en1996/v1" }, ClaimSpec { namespace: "extension", value: "en1996" }];
    const EN: &[LocalizationSpec] = &[LocalizationSpec { locale: "en", text: "EN 1996 design of masonry structures" }];
    const DE: &[LocalizationSpec] = &[LocalizationSpec { locale: "de", text: "EN 1996 Bemessung und Konstruktion von Mauerwerksbauten" }];
    const CAPABILITIES: &[CapabilitySpec] = &[
        CapabilitySpec { identity: "s.en1996.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.en1996.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.en1996.schema.artifact", kind: "schema", descriptor: "s.norm.en1996", claims: SCHEMA, localizations: &[] },
        CapabilitySpec { identity: "s.en1996.inference.outline", kind: "inference", descriptor: "s.norm.en1996.inference", claims: INFERENCE, localizations: &[] },
        CapabilitySpec { identity: "s.en1996.composer.any", kind: "composer", descriptor: "s.en1996@1/*", claims: COMPOSER, localizations: &[] },
        CapabilitySpec { identity: "s.en1996.grammar.document", kind: "grammar", descriptor: "en1996.document", claims: &[ClaimSpec { namespace: "grammar", value: "en1996.document" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1996.grammar.op", kind: "grammar", descriptor: "en1996.op", claims: &[ClaimSpec { namespace: "grammar", value: "en1996.op" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1996.grammar.diff", kind: "grammar", descriptor: "en1996.diff", claims: &[ClaimSpec { namespace: "grammar", value: "en1996.diff" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1996.grammar.pack", kind: "grammar", descriptor: "en1996.pack", claims: &[ClaimSpec { namespace: "grammar", value: "en1996.pack" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1996.grammar.spr", kind: "grammar", descriptor: "en1996.spr", claims: &[ClaimSpec { namespace: "grammar", value: "en1996.spr" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1996.codec.document.v1", kind: "codec", descriptor: "semio.norm.en1996/v1:en1996", claims: CODEC, localizations: &[] },
        CapabilitySpec { identity: "s.en1996.localization.en", kind: "localization", descriptor: "EN 1996 design of masonry structures", claims: &[], localizations: EN },
        CapabilitySpec { identity: "s.en1996.localization.de", kind: "localization", descriptor: "EN 1996 Bemessung und Konstruktion von Mauerwerksbauten", claims: &[], localizations: DE },
    ];
    crate::artifacts::definition::assemble_definition("s.en1996", CAPABILITIES)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::en1996::schema::en1996_artifact_schema_descriptor())
        .inferences([crate::artifacts::en1996::standards::v1::subsets::any::schema::inferences::en1996_artifact_inference_descriptor()])
        .composers(crate::artifacts::en1996::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::en1996::En1996PlayApp>>()
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
                    id: "en1996.document",
                    extension: Some("en1996"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::en1995::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1995::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1995::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1995::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1996.document"),
                },
                dsl::LanguageSpec {
                    id: "en1996.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::en1995::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1995::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1995::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1995::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1996.op"),
                },
                dsl::LanguageSpec {
                    id: "en1996.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::en1995::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1995::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("en1996.diff"),
                },
                dsl::LanguageSpec {
                    id: "en1996.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1995::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1995::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1996.pack"),
                },
                dsl::LanguageSpec {
                    id: "en1996.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1995::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1995::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1996.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration
