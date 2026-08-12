// GIS terrain artifact — the document entity the 3d app edits (constitutional: general).

pub use crate::artifacts::gisterrain::schema::snapshot::GisTerrainSnapshot;
pub use crate::artifacts::gisterrain::schema::mutations::GisTerrainMutation;
pub use crate::artifacts::gisterrain::schema::diff::GisTerrainDiff;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability, };

//#region 🔹Constants
/// VCS-backed, undoable document for GIS 3D — deliberately minimal for the first pass: the only
/// editable/undoable property is vertical exaggeration (a genuinely useful terrain control).


pub const GIS_3D_TERRAIN_SCHEMA: &str = "gis.terrain";
//#endregion 🔹Constants

//#region 🔹Types
/// 📸️ Persisted GIS terrain snapshot — defined in `📸️ snapshot/🧬️ schema`, re-exported here.
//#endregion 🔹Types

//#region 🔹ArtifactKind
/// 🔌️ `3d.mesh` — the interchange kind `scene:out` produces; canonically declared by `lowpoly`
/// (`mesh_from_mesh_document`'s registration). Re-declared here as an identical-shape duplicate so the
/// 3d manifest is self-describing on both sides of the edge (the registry dedupes by id).
pub fn mesh_artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.mesh".into(),
        name: "3D Mesh".into(),
        source_format: "mesh.reference".into(),
        component_kind: "mesh".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        schema: "mesh.reference".into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec!["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl"],
        import_stdio_kinds: vec!["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl"],
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
    semio_framework_plugin::ArtifactDeclaration::builder("s.gisterrain")
        .schema(crate::artifacts::gisterrain::schema::gisterrain_artifact_schema_descriptor())
        .inferences([crate::artifacts::gisterrain::standards::v1::subsets::any::schema::inferences::gisterrain_artifact_inference_descriptor()])
        .composers(crate::artifacts::gisterrain::standards::v1::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::gis3d::Gis3dPlayApp>()
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
                    id: "gis.gisterrain",
                    extension: Some("gisterrain"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::gisterrain::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::gisterrain::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::gisterrain::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gisterrain::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("gis.gisterrain"),
                },
                dsl::LanguageSpec {
                    id: "gis.gisterrain.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::gisterrain::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::gisterrain::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::gisterrain::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gisterrain::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("gis.gisterrain.op"),
                },
                dsl::LanguageSpec {
                    id: "gis.gisterrain.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::gisterrain::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::gisterrain::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("gis.gisterrain.diff"),
                },
                dsl::LanguageSpec {
                    id: "gisterrain.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::gisterrain::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gisterrain::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("gisterrain.pack"),
                },
                dsl::LanguageSpec {
                    id: "gisterrain.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::gisterrain::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::gisterrain::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("gisterrain.spr"),
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
    fn mesh_artifact_kind_matches_the_scene_out_interchange_kind() {
        let kind = mesh_artifact_kind();
        assert_eq!(kind.id, "3d.mesh");
        assert_eq!(kind.schema, "mesh.reference");
    }

    #[test]
    fn the_terrain_snapshot_defaults_to_a_flat_unimported_terrain() {
        let document = GisTerrainSnapshot::default();
        assert_eq!(document.exaggeration, 0.0);
        assert!(document.imported_features_json.is_empty());
    }
}
//#endregion 🔹Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::gisterrain::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("GisTerrainComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
