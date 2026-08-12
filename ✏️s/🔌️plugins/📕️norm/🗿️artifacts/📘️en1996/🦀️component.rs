//! 🧱️ EN 1996 app — document entities (constitutional: general).


pub use crate::artifacts::en1996::schema::snapshot::En1996Snapshot;
pub use crate::artifacts::en1996::schema::mutations::En1996Mutation;
pub use crate::artifacts::en1996::schema::diff::En1996Diff;

use crate::document::{AnnexChoice, DesignSituation};
use serde::{Deserialize, Serialize};

//#region 🔖️Types
/// 🧱️ Masonry manufacturing-control class underlying the EN-recommended γ_M table (EN 1996-1-1 Table 2.1-style).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
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
    use serde::{Deserialize, Serialize};

    /// 🌦️ Masonry durability exposure class (EN 1996-1-1 Annex B-style categorisation MX1–MX5).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum ExposureClass {
        Mx1,
        Mx2,
        Mx3,
        Mx4,
        Mx5,
    }

    /// 🧪️ General-purpose mortar compressive-strength class per EN 998-2.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
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
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::en1996::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("En1996Composer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
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
    semio_framework_plugin::ArtifactDeclaration::builder("s.en1996")
        .schema(crate::artifacts::en1996::schema::en1996_artifact_schema_descriptor())
        .inferences([crate::artifacts::en1996::standards::v1::subsets::any::schema::inferences::en1996_artifact_inference_descriptor()])
        .composers(crate::artifacts::en1996::standards::v1::subsets::any::io::io_registry::entries())
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
    ]).as_slice()
}
//#endregion 🪪️Declaration
