//! ⚡️ DIN V 18599 app — document entities (constitutional: general).

pub use crate::artifacts::din18599::schema::snapshot::Din18599Snapshot;

use crate::document::ClimateZoneDe;

// #region 🔖️Types
/// 🏢️ Building use class for energy reference area factors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum UseClass {
    Residential,
    Office,
    School,
}

/// 📐️ Monthly climate data for balancing. Keeps its `dsl::DslRecord` derive — unlike the snapshot's
/// own storage (now a composed `s.stdio.semio.table` child, see `🔖️Composition` below),
/// `update-climate`'s mutation PAYLOAD still carries a literal `MonthlyClimate` on the wire (the
/// payload is real data, never a handle — `📓️migration-recipe.md`'s pattern), so this type still
/// needs its own `DslField` impl for `Din18599MutationDsl`'s `#[dsl(block)]`-nested encoding.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
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

//#region 🔖️Composition
/// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2 (orchestrator-dispatched
/// correction, `norm→C:table` on `din18599.climate`): the inline `MonthlyClimate` (two twelve-month
/// arrays) is replaced by a fixed composed `s.stdio.semio.table` CHILD slot — twelve rows (one per
/// calendar month), two columns (`thetaEC`/`gHWM2`). The single `update-climate` mutation triad
/// keeps its exact public payload/wire shape (`MonthlyClimate` travels on the wire as a literal
/// value, same as before — only the SNAPSHOT's own storage becomes a composed child) — only the
/// internal diff/inverse implementation is rewired to mint a fresh content-addressed child handle,
/// mirroring `➗️mathematical`'s/en1990's equivalent pattern.
//#region 🔖️ChildTypes
pub type Din18599ClimateChild = store::ArtifactChild<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot>;
//#endregion 🔖️ChildTypes

//#region 🔖️Converters
/// 🌉 REAL bidirectional converter: `MonthlyClimate`'s two parallel twelve-month arrays <-> `table`
/// rows — one row per calendar month (index-addressed, month = row index + 1), two columns
/// (`thetaEC: Float`, `gHWM2: Float`).
pub fn din18599_climate_table_from_data(climate: &MonthlyClimate) -> semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot {
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![SemioTableColumn { name: "thetaEC".into(), kind: SemioTableCellKind::Float }, SemioTableColumn { name: "gHWM2".into(), kind: SemioTableCellKind::Float }],
        rows: climate.theta_e_c.iter().zip(climate.g_h_w_m2.iter()).map(|(theta, g)| SemioTableRow { cells: vec![SemioValue::Float { lexeme: format!("{theta}") }, SemioValue::Float { lexeme: format!("{g}") }] }).collect(),
    }
}

/// 🌉 Inverse of the converter above — real reconstruction, not a stub. A short/missing row
/// degrades honestly (`0.0` for the missing month(s)) rather than panicking, since an
/// externally-composed mismatch is possible in principle.
pub fn din18599_climate_data_from_table(table: &semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot) -> MonthlyClimate {
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableRow;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    fn cell_f64(row: Option<&SemioTableRow>, index: usize) -> f64 {
        match row.and_then(|row| row.cells.get(index)) {
            Some(SemioValue::Float { lexeme }) | Some(SemioValue::Int { lexeme }) => lexeme.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    let mut theta_e_c = [0.0; 12];
    let mut g_h_w_m2 = [0.0; 12];
    for month in 0..12 {
        let row = table.rows.get(month);
        theta_e_c[month] = cell_f64(row, 0);
        g_h_w_m2[month] = cell_f64(row, 1);
    }
    MonthlyClimate { theta_e_c, g_h_w_m2 }
}
//#endregion 🔖️Converters

//#region 🔖️WorkingScene
/// 🌱 Ephemeral representation of one exact DIN 18599 climate child. It is not serialized
/// and retires with the child owner; equal wire identities never share climate data.
#[derive(Clone, Debug)]
pub struct Din18599ClimateWorkingData {
    pub climate: MonthlyClimate,
}

fn din18599_climate_scene_id(climate: &MonthlyClimate) -> String {
    use std::hash::{Hash, Hasher};
    let content_json = pack::json::to_json_string(climate);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    format!("din18599-climate-{:016x}", hasher.finish())
}

fn din18599_climate_target() -> store::os_io::ArtifactRef {
    store::os_io::ArtifactRef { artifact_id: "din18599-climate".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "table".into() } }
}

/// 🏗️ Mints the composed-child handle and transfers the climate into that exact owner.
pub fn din18599_climate_child_from_data(climate: &MonthlyClimate) -> Din18599ClimateChild {
    let scene_id = din18599_climate_scene_id(climate);
    store::ArtifactChild::new(scene_id, din18599_climate_target()).with_local_owner(std::sync::Arc::new(Din18599ClimateWorkingData { climate: climate.clone() }))
}

/// 🔎 The live `MonthlyClimate` behind a snapshot's composed child — the single read call site
/// every energy-balance/compliance/inference/mutation-diff call path in this artifact now uses. A
/// wire-only child fails soft until its child document is materialized by the host.
pub fn din18599_climate(snapshot: &Din18599Snapshot) -> MonthlyClimate {
    snapshot.climate.local_owner::<Din18599ClimateWorkingData>().map(|data| data.climate.clone()).unwrap_or(MonthlyClimate { theta_e_c: [0.0; 12], g_h_w_m2: [0.0; 12] })
}
//#endregion 🔖️WorkingScene
//#endregion 🔖️Composition

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

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port —
/// lifted out of the pre-migration manifest's inline `.artifact_kind(ArtifactKindSpec { .. })` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("din18599", "DIN V 18599")
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const DIN18599_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.din18599", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
pub const DIN18599_DOCUMENT_SCHEMA: &str = "semio.norm.din18599/v1";
//#endregion 🔖️ArtifactKind

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use crate::artifacts::definition::{CapabilitySpec, ClaimSpec, LocalizationSpec};
    const SCHEMA: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.din18599" }];
    const INFERENCE: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.din18599.inference" }];
    const COMPOSER: &[ClaimSpec] = &[ClaimSpec { namespace: "dialect", value: "s.norm.din18599@1/*" }];
    const CODEC: &[ClaimSpec] = &[ClaimSpec { namespace: "codec", value: "semio.norm.din18599/v1" }, ClaimSpec { namespace: "codec-extension", value: "22:semio.norm.din18599/v1:din18599" }];
    const EN: &[LocalizationSpec] = &[LocalizationSpec { locale: "en", text: "DIN V 18599 energy performance of buildings" }];
    const DE: &[LocalizationSpec] = &[LocalizationSpec { locale: "de", text: "DIN V 18599 Energetische Bewertung von Gebäuden" }];
    const CAPABILITIES: &[CapabilitySpec] = &[
        CapabilitySpec { identity: "s.norm.din18599.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.schema.artifact", kind: "schema", descriptor: "s.norm.din18599", claims: SCHEMA, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.inference.outline", kind: "inference", descriptor: "s.norm.din18599.inference", claims: INFERENCE, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.composer.any", kind: "composer", descriptor: "s.norm.din18599@1/*", claims: COMPOSER, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.grammar.document", kind: "grammar", descriptor: "din18599.document", claims: &[ClaimSpec { namespace: "grammar", value: "din18599.document" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.grammar.op", kind: "grammar", descriptor: "din18599.op", claims: &[ClaimSpec { namespace: "grammar", value: "din18599.op" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.grammar.diff", kind: "grammar", descriptor: "din18599.diff", claims: &[ClaimSpec { namespace: "grammar", value: "din18599.diff" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.grammar.pack", kind: "grammar", descriptor: "din18599.pack", claims: &[ClaimSpec { namespace: "grammar", value: "din18599.pack" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.grammar.spr", kind: "grammar", descriptor: "din18599.spr", claims: &[ClaimSpec { namespace: "grammar", value: "din18599.spr" }], localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.codec.document.v1", kind: "codec", descriptor: "semio.norm.din18599/v1:din18599", claims: CODEC, localizations: &[] },
        CapabilitySpec { identity: "s.norm.din18599.localization.en", kind: "localization", descriptor: "DIN V 18599 energy performance of buildings", claims: &[], localizations: EN },
        CapabilitySpec { identity: "s.norm.din18599.localization.de", kind: "localization", descriptor: "DIN V 18599 Energetische Bewertung von Gebäuden", claims: &[], localizations: DE },
    ];
    crate::artifacts::definition::assemble_definition("s.norm.din18599", CAPABILITIES)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::din18599::schema::din18599_artifact_schema_descriptor())
        .inferences([crate::artifacts::din18599::standards::v1::subsets::any::schema::inferences::din18599_artifact_inference_descriptor()])
        .composers(crate::artifacts::din18599::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::din18599::Din18599PlayApp>>()
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
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    trait Din18599ChildOwnerOracle {
        fn expected() -> serde_json::Value;
    }

    struct SerdeJsonDin18599ChildOwnerOracle;

    impl Din18599ChildOwnerOracle for SerdeJsonDin18599ChildOwnerOracle {
        fn expected() -> serde_json::Value {
            serde_json::from_str(include_str!("🧫️fixtures/🧫️child-owner-isolation/🔣️.json")).expect("language-neutral DIN 18599 child-owner fixture")
        }
    }

    #[test]
    fn climate_working_data_is_owned_by_the_exact_child() {
        let owned = din18599_climate_child_from_data(&MonthlyClimate { theta_e_c: [1.0; 12], g_h_w_m2: [2.0; 12] });
        let wire = serde_json::to_vec(&owned).expect("DIN 18599 child wire identity");
        let reconstructed: Din18599ClimateChild = serde_json::from_slice(&wire).expect("DIN 18599 child wire roundtrip");
        let observed = serde_json::json!({
            "ownedHasPayload": owned.local_owner::<Din18599ClimateWorkingData>().is_some(),
            "wireIdentityMatches": owned == reconstructed,
            "wireHasPayload": reconstructed.local_owner::<Din18599ClimateWorkingData>().is_some(),
        });

        assert_eq!(observed, SerdeJsonDin18599ChildOwnerOracle::expected());
    }
}
//#endregion 🧪️Tests
