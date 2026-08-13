// GIS terrain artifact — the document entity the 3d app edits (constitutional: general).

pub use crate::artifacts::gisterrain::schema::snapshot::GisTerrainSnapshot;
pub use crate::artifacts::gisterrain::schema::mutations::GisTerrainMutation;
pub use crate::artifacts::gisterrain::schema::diff::GisTerrainDiff;

use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;

//#region 🔹Constants
/// VCS-backed, undoable document for GIS 3D — deliberately minimal for the first pass: the only
/// editable/undoable property is vertical exaggeration (a genuinely useful terrain control).


pub const GIS_3D_TERRAIN_SCHEMA: &str = "gis.terrain";
//#endregion 🔹Constants

//#region 🔹Types
/// 📸️ Persisted GIS terrain snapshot — defined in `📸️ snapshot/🧬️ schema`, re-exported here.
//#endregion 🔹Types

// 🧱️ `mesh_artifact_kind()` (the shared `3d.mesh` interchange kind duplicate) REMOVED — ticket
// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`. `3d.mesh` is a duplicate kind id being deleted
// repo-wide: mesh is now canonically `s.stdio.semio@v1/mesh` (composed via `GisTerrainSnapshot.mesh:
// Option<store::ArtifactChild<SemioMeshSnapshot>>` below, never a standalone `ArtifactKindSpec`).
// lowpoly independently declared the identical id and removed its own copy first (see that
// plugin's `🦀️component.rs`, same region); this removes gis's copy, the other place this ticket's
// design doc (`📓️design-full-plan.md` §4: "gis→terrain C:mesh … kills `3d.mesh` dup") flagged it.
// Registration call site removed from `🎛️apps/🧊️3d/🦀️component.rs`'s `create_gis3d_app()`
// (`.artifact_kind(mesh_artifact_kind())`).

//#region 🔖️MeshComposition
/// 🕸️ Deterministic content-addressed CHILD handle for the terrain's composed mesh — same
/// `(child_id, target)` for identical `(exaggeration, imported_features_json)`, a different pair
/// once either actually changes. Mirrors `💠️lowpoly`'s `mesh_child_handle`/`📐️cad`'s
/// `cad_model_child_handle` (same `store::ArtifactChild::new` + `ArtifactDialect` shape).
pub fn gis_terrain_mesh_child_handle(content_key: &str) -> store::ArtifactChild<SemioMeshSnapshot> {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_key.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("gisterrain-mesh-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() };
    let target = store::os_io::ArtifactRef { artifact_id: "gisterrain-mesh".into(), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 🔑️ The single string every caller hashes into `gis_terrain_mesh_child_handle` — the exact
/// `(exaggeration, imported_features_json)` pair that determines the mesh's content below, kept in
/// one place so `to_snapshot`/`GisTerrainDiff::apply`/fixture construction can never drift apart.
pub fn gis_terrain_mesh_content_key(exaggeration: f64, imported_features_json: &str) -> String {
    format!("{exaggeration}|{imported_features_json}")
}

/// 🔄️ Re-derives `mesh` from `document`'s CURRENT `(exaggeration, imported_features_json)` — the
/// single call every constructor/mutator/test fixture funnels through so the composed child never
/// drifts from what it actually describes. Mirrors `crate::artifacts::gismap`'s
/// `gis_map_snapshot_with_derived_children`.
pub fn gis_terrain_snapshot_with_derived_mesh(mut document: GisTerrainSnapshot) -> GisTerrainSnapshot {
    document.mesh = Some(gis_terrain_mesh_child_handle(&gis_terrain_mesh_content_key(document.exaggeration, &document.imported_features_json)));
    document
}

/// 🏔️ WRITE direction, real (not a stub): builds an actual `SemioMeshSnapshot` from the terrain
/// document — a single flat quad (2 triangles) sized by the imported overlay's geographic extent
/// (falling back to a 1×1 unit quad when no overlay is present) and lifted by `exaggeration` alone
/// (there is no DEM heightfield yet — see `gis3d_scene_media`'s own long-standing doc comment: "gis3d
/// has no CPU-side heightmap tessellator yet"). Honest placeholder geometry, not a fake: every field
/// is real, computed, and round-trips — the day a tessellator lands, only this function's body needs
/// to grow real per-vertex elevation.
pub fn gis_terrain_mesh_from_snapshot(document: &GisTerrainSnapshot) -> SemioMeshSnapshot {
    let bounds = crate::artifacts::gisterrain::standards::v1::subsets::any::schema::inferences::bounds::lon_lat_bounds(
        &crate::artifacts::gisterrain::standards::v1::subsets::any::schema::inferences::bounds::imported_lon_lat_positions(document),
    );
    let (min_x, min_y, max_x, max_y) = bounds.map(|b| (b.lon_min, b.lat_min, b.lon_max, b.lat_max)).unwrap_or((0.0, 0.0, 1.0, 1.0));
    let (min_x, max_x) = if min_x < max_x { (min_x, max_x) } else { (min_x, min_x + 1.0) };
    let (min_y, max_y) = if min_y < max_y { (min_y, max_y) } else { (min_y, min_y + 1.0) };
    // 🏔️ Honest gap (matches `gis3d_scene_media`'s own doc comment): with no DEM heightfield, every
    // vertex is flat at z=0 regardless of `exaggeration` — `exaggeration` still round-trips as
    // real document state and still keys the mesh's content-addressed handle, it simply has no
    // per-vertex effect on THIS placeholder surface yet.
    let z = 0.0;
    let positions = vec![
        SemioPoint3 { x: min_x, y: min_y, z },
        SemioPoint3 { x: max_x, y: min_y, z },
        SemioPoint3 { x: max_x, y: max_y, z },
        SemioPoint3 { x: min_x, y: max_y, z },
    ];
    SemioMeshSnapshot {
        schema: "s.stdio.semio.mesh".into(),
        meshes: vec![SemioMesh {
            id: "gisterrain-surface".into(),
            primitives: vec![SemioPrimitive {
                id: "gisterrain-surface-quad".into(),
                topology: SemioTopology::Triangles,
                positions,
                normals: Vec::new(),
                uvs: Vec::new(),
                colors: Vec::new(),
                indices: vec![0, 1, 2, 0, 2, 3],
                material_id: None,
            }],
        }],
        materials: Vec::new(),
        textures: Vec::new(),
    }
}
//#endregion 🔖️MeshComposition

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
        .composers(crate::artifacts::gisterrain::standards::v1::subsets::any::io::io_registry::entries())
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
    fn the_terrain_snapshot_defaults_to_a_flat_unimported_terrain() {
        let document = GisTerrainSnapshot::default();
        assert_eq!(document.exaggeration, 0.0);
        assert!(document.imported_features_json.is_empty());
    }

    #[test]
    fn the_terrain_snapshot_composes_a_content_addressed_mesh_child() {
        let document = GisTerrainSnapshot::default();
        assert!(document.mesh.is_some(), "the terrain always composes a mesh child, even when flat/unimported");
        let mesh = gis_terrain_mesh_from_snapshot(&document);
        assert_eq!(mesh.meshes.len(), 1);
        assert_eq!(mesh.meshes[0].primitives[0].positions.len(), 4, "a flat placeholder quad");
    }
}
//#endregion 🔹Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::gisterrain::standards::v1::subsets::any::io::io_registry as v1;

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
