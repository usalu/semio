//! ⚖️ EN 1990 basis of structural design — document entities (constitutional: general).

pub use crate::artifacts::en1990::schema::snapshot::En1990QkEntry;
pub use crate::artifacts::en1990::schema::snapshot::En1990Snapshot;

/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.

//#region 🔖️Types
//#endregion 🔖️Types

//#region 🔖️Composition
/// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2 (orchestrator-dispatched
/// correction, `norm→C:table` on `en1990.q_k`): the inline `Vec<En1990QkEntry>` variable-action
/// table is replaced by a fixed composed `s.stdio.semio.table` CHILD slot — `q_k` composes
/// stdio's `table` subset instead of hand-rolling its own two-column shape. `#[child(...)]` drives
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written. Every one of the five
/// existing `insert`/`remove`/`reorder`/`change-category`/`change-value` mutation triads keeps its
/// exact public payload/wire shape — only the internal diff/inverse implementation is rewired to
/// read/write the exact child owner below and re-mint a fresh content-addressed child handle,
/// mirroring `➗️mathematical`'s `MATH_SCRATCH`/`🕸️dag`'s/`🔀️process`'s equivalent patterns for the
/// identical per-entry mutation-rich shape.

//#region 🔖️ChildTypes
pub type En1990QkChild = store::ArtifactChild<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot>;
//#endregion 🔖️ChildTypes

//#region 🔖️Converters
/// 🌉 REAL bidirectional converter: `q_k` variable-action entries <-> `table` rows — two columns
/// (`category: Str`, `value: Float`), one row per entry in list order (positionally aligned, no
/// stable id on either side).
pub fn en1990_qk_table_from_entries(entries: &[En1990QkEntry]) -> semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot {
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![SemioTableColumn { name: "category".into(), kind: SemioTableCellKind::Str }, SemioTableColumn { name: "value".into(), kind: SemioTableCellKind::Float }],
        rows: entries.iter().map(|entry| SemioTableRow { cells: vec![SemioValue::Str { value: entry.category.clone() }, SemioValue::Float { lexeme: format!("{}", entry.value) }] }).collect(),
    }
}

/// 🌉 Inverse of the converter above — real reconstruction, not a stub. A short/missing cell
/// degrades honestly (empty category, `0.0` value) rather than panicking, since an
/// externally-composed mismatch is possible in principle.
pub fn en1990_qk_entries_from_table(table: &semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot) -> Vec<En1990QkEntry> {
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableRow;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    fn cell_str(row: &SemioTableRow, index: usize) -> String {
        match row.cells.get(index) {
            Some(SemioValue::Str { value }) => value.clone(),
            _ => String::new(),
        }
    }
    fn cell_f64(row: &SemioTableRow, index: usize) -> f64 {
        match row.cells.get(index) {
            Some(SemioValue::Float { lexeme }) | Some(SemioValue::Int { lexeme }) => lexeme.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    table.rows.iter().map(|row| En1990QkEntry { category: cell_str(row, 0), value: cell_f64(row, 1) }).collect()
}
//#endregion 🔖️Converters

//#region 🔖️WorkingScene
/// 🌱 Ephemeral representation of one exact EN 1990 table child. It is not serialized and
/// retires with the child owner; equal wire identities never share entries.
#[derive(Clone, Debug, Default)]
pub struct En1990QkWorkingTable {
    pub entries: Vec<En1990QkEntry>,
}

fn en1990_qk_scene_id(entries: &[En1990QkEntry]) -> String {
    use std::hash::{Hash, Hasher};
    let content_json = serde_json::to_string(entries).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    format!("en1990-qk-{:016x}", hasher.finish())
}

fn en1990_qk_target() -> store::os_io::ArtifactRef {
    store::os_io::ArtifactRef { artifact_id: "en1990-qk".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "table".into() } }
}

/// 🏗️ Mints the composed-child handle and transfers the entry list into that exact owner.
pub fn en1990_qk_child_from_entries(entries: &[En1990QkEntry]) -> En1990QkChild {
    let scene_id = en1990_qk_scene_id(entries);
    store::ArtifactChild::new(scene_id, en1990_qk_target()).with_local_owner(std::sync::Arc::new(En1990QkWorkingTable { entries: entries.to_vec() }))
}

/// 🔎 The live `q_k` entries behind a snapshot's composed child — the single read call site every
/// combination/compliance/inference/mutation-diff call path in this artifact now uses. A wire-only
/// child fails soft until its child document is materialized by the host.
pub fn en1990_qk(snapshot: &En1990Snapshot) -> Vec<En1990QkEntry> {
    snapshot.q_k.local_owner::<En1990QkWorkingTable>().map(|table| table.entries.clone()).unwrap_or_default()
}
//#endregion 🔖️WorkingScene
//#endregion 🔖️Composition

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1990", "EN 1990")
}
//#endregion 🔖️ArtifactKind

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1990_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1990", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
pub const EN1990_DOCUMENT_SCHEMA: &str = "semio.norm.en1990/v1";

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use crate::artifacts::definition::{CapabilitySpec, ClaimSpec, LocalizationSpec};
    const SCHEMA: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1990" }];
    const INFERENCE: &[ClaimSpec] = &[ClaimSpec { namespace: "schema", value: "s.norm.en1990.inference" }];
    const COMPOSER: &[ClaimSpec] = &[ClaimSpec { namespace: "dialect", value: "s.en1990@1/*" }];
    const CODEC: &[ClaimSpec] = &[ClaimSpec { namespace: "codec", value: "semio.norm.en1990/v1" }, ClaimSpec { namespace: "extension", value: "en1990" }];
    const EN: &[LocalizationSpec] = &[LocalizationSpec { locale: "en", text: "EN 1990 basis of structural design" }];
    const DE: &[LocalizationSpec] = &[LocalizationSpec { locale: "de", text: "EN 1990 Grundlagen der Tragwerksplanung" }];
    const CAPABILITIES: &[CapabilitySpec] = &[
        CapabilitySpec { identity: "s.en1990.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.en1990.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        CapabilitySpec { identity: "s.en1990.schema.artifact", kind: "schema", descriptor: "s.norm.en1990", claims: SCHEMA, localizations: &[] },
        CapabilitySpec { identity: "s.en1990.inference.outline", kind: "inference", descriptor: "s.norm.en1990.inference", claims: INFERENCE, localizations: &[] },
        CapabilitySpec { identity: "s.en1990.composer.any", kind: "composer", descriptor: "s.en1990@1/*", claims: COMPOSER, localizations: &[] },
        CapabilitySpec { identity: "s.en1990.grammar.document", kind: "grammar", descriptor: "en1990.document", claims: &[ClaimSpec { namespace: "grammar", value: "en1990.document" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1990.grammar.op", kind: "grammar", descriptor: "en1990.op", claims: &[ClaimSpec { namespace: "grammar", value: "en1990.op" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1990.grammar.diff", kind: "grammar", descriptor: "en1990.diff", claims: &[ClaimSpec { namespace: "grammar", value: "en1990.diff" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1990.grammar.pack", kind: "grammar", descriptor: "en1990.pack", claims: &[ClaimSpec { namespace: "grammar", value: "en1990.pack" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1990.grammar.spr", kind: "grammar", descriptor: "en1990.spr", claims: &[ClaimSpec { namespace: "grammar", value: "en1990.spr" }], localizations: &[] },
        CapabilitySpec { identity: "s.en1990.codec.document.v1", kind: "codec", descriptor: "semio.norm.en1990/v1:en1990", claims: CODEC, localizations: &[] },
        CapabilitySpec { identity: "s.en1990.localization.en", kind: "localization", descriptor: "EN 1990 basis of structural design", claims: &[], localizations: EN },
        CapabilitySpec { identity: "s.en1990.localization.de", kind: "localization", descriptor: "EN 1990 Grundlagen der Tragwerksplanung", claims: &[], localizations: DE },
    ];
    crate::artifacts::definition::assemble_definition("s.en1990", CAPABILITIES)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::en1990::schema::en1990_artifact_schema_descriptor())
        .inferences([crate::artifacts::en1990::standards::v1::subsets::any::schema::inferences::en1990_artifact_inference_descriptor()])
        .composers(crate::artifacts::en1990::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::en1990::En1990PlayApp>>()
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
                    id: "en1990.document",
                    extension: Some("en1990"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::en1990::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1990::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1990::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1990::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1990.document"),
                },
                dsl::LanguageSpec {
                    id: "en1990.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::en1990::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1990::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1990::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1990::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1990.op"),
                },
                dsl::LanguageSpec {
                    id: "en1990.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::en1990::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1990::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("en1990.diff"),
                },
                dsl::LanguageSpec {
                    id: "en1990.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1990::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1990::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1990.pack"),
                },
                dsl::LanguageSpec {
                    id: "en1990.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1990::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1990::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("en1990.spr"),
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

    trait En1990ChildOwnerOracle {
        fn expected() -> serde_json::Value;
    }

    struct SerdeJsonEn1990ChildOwnerOracle;

    impl En1990ChildOwnerOracle for SerdeJsonEn1990ChildOwnerOracle {
        fn expected() -> serde_json::Value {
            serde_json::from_str(include_str!("🧪️fixtures/🎯️child-owner-isolation.json")).expect("language-neutral EN 1990 child-owner fixture")
        }
    }

    #[test]
    fn qk_working_table_is_owned_by_the_exact_child() {
        let owned = en1990_qk_child_from_entries(&[En1990QkEntry { category: "snow".into(), value: 42.0 }]);
        let wire = serde_json::to_vec(&owned).expect("EN 1990 child wire identity");
        let reconstructed: En1990QkChild = serde_json::from_slice(&wire).expect("EN 1990 child wire roundtrip");
        let observed = serde_json::json!({
            "ownedHasPayload": owned.local_owner::<En1990QkWorkingTable>().is_some(),
            "wireIdentityMatches": owned == reconstructed,
            "wireHasPayload": reconstructed.local_owner::<En1990QkWorkingTable>().is_some(),
        });

        assert_eq!(observed, SerdeJsonEn1990ChildOwnerOracle::expected());
    }
}
//#endregion 🧪️Tests
