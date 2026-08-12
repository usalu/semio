//! ⚡️ DIN V 18599 app — document entities (constitutional: general).


pub use crate::artifacts::din18599::schema::snapshot::Din18599Snapshot;
pub use crate::artifacts::din18599::schema::mutations::Din18599Mutation;
pub use crate::artifacts::din18599::schema::diff::Din18599Diff;

use crate::document::ClimateZoneDe;
use serde::{Deserialize, Serialize};

// #region 🔖️Types
/// 🏢️ Building use class for energy reference area factors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum UseClass {
    Residential,
    Office,
    School,
}

/// 📐️ Monthly climate data for balancing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct MonthlyClimate {
    pub theta_e_c: [f64; 12],
    pub g_h_w_m2: [f64; 12],
}

impl MonthlyClimate {
    pub fn german_reference(zone: ClimateZoneDe) -> Self {
        let winter = zone.design_external_temperature_c();
        let summer = zone.summer_design_temperature_c();
        let mean = (winter + summer) / 2.0;
        let amplitude = (summer - winter) / 2.0;
        let mut theta_e = [0.0; 12];
        let g_h = [30.0, 60.0, 100.0, 140.0, 180.0, 200.0, 210.0, 190.0, 140.0, 90.0, 40.0, 20.0];
        for (i, t) in theta_e.iter_mut().enumerate() {
            let month = i as f64 + 1.0;
            *t = mean + amplitude * (2.0 * std::f64::consts::PI * (month - 7.0) / 12.0).cos();
        }
        Self { theta_e_c: theta_e, g_h_w_m2: g_h }
    }
}

/// 📋️ Inputs for annual energy balancing.
// BalancingInputs remains the nested persistent payload type; snapshot is Din18599Snapshot.

// 📌️ Deviation from the original monolith: `BalancingInputs::reference_residential(..)` (the
// physically-computed reference-building constructor, needing `din4108`'s relocated
// `total_resistance`/`u_value_from_resistance` and `din16798`'s relocated
// `residential_ventilation_rate`) moved to
// `crate::artifacts::din18599::standards::v1::subsets::any::schema::reference_residential` — an
// inherent impl here would need those crates, but inherent impls must live in the crate that
// defines the type (orphan rule), and `rs` must not depend on `schema`'s compliance helpers (the
// reverse of every other constitutional dependency edge). `Default` has the same orphan-rule
// constraint, so — matching the plain-literal `Default` style `din4108`/`din16798` already use —
// this is the numeric result of `reference_residential(ClimateZoneDe::Zone2, 100.0)`, precomputed
// once and inlined; use
// `crate::artifacts::din18599::standards::v1::subsets::any::schema::reference_residential`
// directly for a live-computed reference building.

/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.

pub type BalancingInputs = Din18599Snapshot;
//#endregion 🔖️Types

// `)` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("din18599", "DIN V 18599")
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::din18599::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("Din18599Composer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
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
    semio_framework_plugin::ArtifactDeclaration::builder("s.din18599")
        .schema(crate::artifacts::din18599::schema::din18599_artifact_schema_descriptor())
        .inferences([crate::artifacts::din18599::standards::v1::subsets::any::schema::inferences::din18599_artifact_inference_descriptor()])
        .composers(crate::artifacts::din18599::standards::v1::subsets::any::io::io_registry::entries())
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
            id: "din18599.document",
            extension: Some("din18599"),
            role: dsl::LanguageRole::Document,
            grammar: Some(crate::artifacts::din18599::dsl::COMPONENT_GRAMMAR_SEMIO),
            grammar_path: Some(crate::artifacts::din18599::dsl::COMPONENT_GRAMMAR_PATH),
            protocol: Some(crate::artifacts::din18599::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::din18599::snapshot::pack::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("din18599.document"),
        },
        dsl::LanguageSpec {
            id: "din18599.op",
            extension: None,
            role: dsl::LanguageRole::Ops,
            grammar: Some(crate::artifacts::din18599::op::COMPONENT_GRAMMAR_SEMIO),
            grammar_path: Some(crate::artifacts::din18599::op::COMPONENT_GRAMMAR_PATH),
            protocol: Some(crate::artifacts::din18599::spr::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::din18599::spr::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("din18599.op"),
        },
        dsl::LanguageSpec {
            id: "din18599.diff",
            extension: None,
            role: dsl::LanguageRole::Diff,
            grammar: Some(crate::artifacts::din18599::diff::COMPONENT_GRAMMAR_SEMIO),
            grammar_path: Some(crate::artifacts::din18599::diff::COMPONENT_GRAMMAR_PATH),
            protocol: None,
            protocol_path: None,
            hooks: dsl::passthrough_hooks("din18599.diff"),
        },
        dsl::LanguageSpec {
            id: "din18599.pack",
            extension: None,
            role: dsl::LanguageRole::Pack,
            grammar: None,
            grammar_path: None,
            protocol: Some(crate::artifacts::din18599::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::din18599::snapshot::pack::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("din18599.pack"),
        },
        dsl::LanguageSpec {
            id: "din18599.spr",
            extension: None,
            role: dsl::LanguageRole::Spr,
            grammar: None,
            grammar_path: None,
            protocol: Some(crate::artifacts::din18599::spr::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::din18599::spr::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("din18599.spr"),
        },
    ]).as_slice()
}
//#endregion 🪪️Declaration
