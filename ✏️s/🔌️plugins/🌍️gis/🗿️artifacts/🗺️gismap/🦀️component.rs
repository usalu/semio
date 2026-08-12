//! 🗺️ GIS map artifact — the document entity the 2d app edits (constitutional: general).


pub use crate::artifacts::gismap::schema::snapshot::GisMapSnapshot;
pub use crate::artifacts::gismap::schema::mutations::GisMapMutation;
pub use crate::artifacts::gismap::schema::diff::GisMapDiff;

use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability, };
use serde::{Deserialize, Serialize};

//#region 🔹Constants


pub const GIS_MAP_SCHEMA: &str = "gis.map";
//#endregion 🔹Constants

//#region 🔹Types
/// 🗺️ One id-keyed spatial feature carried as its full opaque descriptor payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MapFeature {
    #[dsl(positional)]
    pub id: String,
    /// 🧬️ Deliberately untyped: binds through the engine's `Shape::Value` escape hatch.
    pub data: dsl::DslValue,
}

impl Identified<String> for MapFeature {
    fn id(&self) -> &String {
        &self.id
    }
}

/// Whole-payload replacement patch (features are opaque JSON); inverts to the prior payload.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct MapFeaturePatch {
    pub data: Option<dsl::DslValue>,
}

impl Patchable<MapFeaturePatch> for MapFeature {
    fn apply_patch(&mut self, patch: &MapFeaturePatch) {
        if let Some(data) = &patch.data {
            self.data = data.clone();
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<MapFeaturePatch> {
        (self.data != other.data).then(|| MapFeaturePatch { data: Some(other.data.clone()) })
    }
}

/// 📸️ Persisted GIS map snapshot — defined in `📸️ snapshot/🧬️ schema`, re-exported here.
//#endregion 🔹Types

//#region 🔹ArtifactKind
/// The `2d.map` artifact kind declaration.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.map".into(),
        name: "2D Map".into(),
        source_format: GIS_MAP_SCHEMA.into(),
        component_kind: "gismap".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        schema: GIS_MAP_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec!["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"],
        import_stdio_kinds: vec!["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"],
    }
}
//#endregion 🔹ArtifactKind

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// the plugin root's `register_gis_exports` fan-out. Relocated from `⚙️engine` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g2): `declaration()` describes the artifact
/// (kind, schema, io ports, ownership), which is not engine behaviour.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.gismap")
        .schema(crate::artifacts::gismap::schema::gismap_artifact_schema_descriptor())
        .inferences([crate::artifacts::gismap::standards::v1::subsets::any::schema::inferences::gismap_artifact_inference_descriptor()])
        .composers(crate::artifacts::gismap::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::gis2d::Gis2dPlayApp>()
        .build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention already used below. Relocated alongside
/// `declaration()` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g2) — its only caller.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "gis.gismap",
                    extension: Some("gismap"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::gismap::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::gismap::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::gismap::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gismap::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("gis.gismap"),
                },
                dsl::LanguageSpec {
                    id: "gis.gismap.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::gismap::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::gismap::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::gismap::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gismap::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("gis.gismap.op"),
                },
                dsl::LanguageSpec {
                    id: "gis.gismap.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::gismap::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::gismap::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("gis.gismap.diff"),
                },
                dsl::LanguageSpec {
                    id: "gismap.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::gismap::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gismap::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("gismap.pack"),
                },
                dsl::LanguageSpec {
                    id: "gismap.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::gismap::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gismap::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("gismap.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

//#region 🔹Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_artifact_kind_matches_the_map_out_interchange_kind() {
        let kind = artifact_kind();
        assert_eq!(kind.id, "2d.map");
        assert_eq!(kind.schema, GIS_MAP_SCHEMA);
    }

    #[test]
    fn the_map_snapshot_defaults_to_empty_feature_collections() {
        let document = GisMapSnapshot::default();
        assert!(document.positions.is_empty());
        assert!(document.routes.is_empty());
        assert!(document.regions.is_empty());
    }
}
//#endregion 🔹Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::gismap::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("GisMapComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
