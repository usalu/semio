// GIS terrain artifact — the document entity the 3d app edits (constitutional: general).

pub use crate::artifacts::gisterrain::schema::snapshot::GisTerrainSnapshot;

use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTopology};

//#region 🔹Constants
/// VCS-backed, undoable document for GIS 3D — deliberately minimal for the first pass: the only
/// editable/undoable property is vertical exaggeration (a genuinely useful terrain control).

pub const GIS_3D_TERRAIN_SCHEMA: &str = "gis.terrain";

/// 🪪️ The canonical surface dialect for `s.gis.gisterrain@1/*` (contract §1 grammar) — lives at the
/// ARTIFACT root, not under `✏️editor`/`👁️viewer`, so a viewer file can read it without ever
/// importing through the sibling `editor` module. `artifact_kind` is the 3-part schema id this
/// file's own `definition()` claims (`s.gis.gisterrain`), NOT the 2-part `ArtifactIdentity::parse("s.gisterrain")`
/// string above and NOT the module-private `🚪️io/🦀️component.rs` `GISTERRAIN_DIALECT` (an older,
/// unrelated io/composer const with a different 2-part `artifact_kind` — different file, no collision).
pub const GISTERRAIN_DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.gis.gisterrain", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };
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
pub async fn gis_terrain_mesh_child_handle(content_key: &str) -> store::ArtifactChild<SemioMeshSnapshot> {
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
pub async fn gis_terrain_mesh_content_key(exaggeration: f64, imported_features_json: &str) -> String {
    format!("{exaggeration}|{imported_features_json}")
}

/// 🔄️ Re-derives `mesh` from `document`'s CURRENT `(exaggeration, imported_features_json)` — the
/// single call every constructor/mutator/test fixture funnels through so the composed child never
/// drifts from what it actually describes. Mirrors `crate::artifacts::gismap`'s
/// `gis_map_snapshot_with_derived_children`.
pub async fn gis_terrain_snapshot_with_derived_mesh(mut document: GisTerrainSnapshot) -> GisTerrainSnapshot {
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
pub async fn gis_terrain_mesh_from_snapshot(document: &GisTerrainSnapshot) -> SemioMeshSnapshot {
    let bounds = crate::artifacts::gisterrain::standards::v1::subsets::any::schema::inferences::bounds::lon_lat_bounds(&crate::artifacts::gisterrain::standards::v1::subsets::any::schema::inferences::bounds::imported_lon_lat_positions(document));
    let (min_x, min_y, max_x, max_y) = bounds.map(|b| (b.lon_min, b.lat_min, b.lon_max, b.lat_max)).unwrap_or((0.0, 0.0, 1.0, 1.0));
    let (min_x, max_x) = if min_x < max_x { (min_x, max_x) } else { (min_x, min_x + 1.0) };
    let (min_y, max_y) = if min_y < max_y { (min_y, max_y) } else { (min_y, min_y + 1.0) };
    // 🏔️ Honest gap (matches `gis3d_scene_media`'s own doc comment): with no DEM heightfield, every
    // vertex is flat at z=0 regardless of `exaggeration` — `exaggeration` still round-trips as
    // real document state and still keys the mesh's content-addressed handle, it simply has no
    // per-vertex effect on THIS placeholder surface yet.
    let z = 0.0;
    let positions = vec![SemioPoint3 { x: min_x, y: min_y, z }, SemioPoint3 { x: max_x, y: min_y, z }, SemioPoint3 { x: max_x, y: max_y, z }, SemioPoint3 { x: min_x, y: max_y, z }];
    SemioMeshSnapshot {
        schema: "s.stdio.semio.mesh".into(),
        meshes: vec![SemioMesh {
            id: "gisterrain-surface".into(),
            primitives: vec![SemioPrimitive { id: "gisterrain-surface-quad".into(), topology: SemioTopology::Triangles, positions, normals: Vec::new(), uvs: Vec::new(), colors: Vec::new(), indices: vec![0, 1, 2, 0, 2, 3], material_id: None }],
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
/// 🧾️ Defines s.gisterrain's immutable runtime capability leaves.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    ArtifactDefinition::new(ArtifactIdentity::parse("s.gisterrain")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gisterrain.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.gis.gisterrain")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.gis.gisterrain")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gisterrain.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.gis.gisterrain.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.gis.gisterrain.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gisterrain.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.gisterrain@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.gisterrain@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gisterrain.composer.las")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.las@1.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.las@1.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gisterrain.composer.ply")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.ply@1.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.ply@1.0/*")?)?,
        )?
        // 🗺️ No `composer.png`/`composer.json`/`composer.dwg` here: all three collide with gismap's
        // identical literal EXPORT claims (26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME D3 —
        // `ArtifactDefinitionRegistry` rejects two artifacts in the same plugin exporting the same
        // dialect coordinate). gismap is the plugin's real top-level artifact — it alone gets
        // `.activation(...)` in `../../🦀️component.rs` and owns the plugin's only `host_media_handler`
        // (SVG export); gisterrain is a composed CHILD of gismap's own snapshot, never independently
        // activated (see that file's own doc comment) — so gismap keeps every shared EXPORT claim.
        // Import still works: `reads()` on gisterrain's own native composer is unaffected.
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gisterrain.composer.stl")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.stl@ascii/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.stl@ascii/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gisterrain.composer.gltf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.gltf@2.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.gltf@2.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gisterrain.composer.obj")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.obj@3.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.obj@3.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gisterrain.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"gis.terrain:gisterrain")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "gis.terrain")?)?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), "gisterrain")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gisterrain.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"GIS Terrain")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "GIS Terrain")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.gisterrain.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"GIS Gel\xc3\xa4nde")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "GIS Gelände")?)?,
        )
}

/// 🔖️ Assembles s.gisterrain's typed runtime declaration.
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::gisterrain::schema::gisterrain_artifact_schema_descriptor())
        .inferences([crate::artifacts::gisterrain::standards::v1::subsets::any::schema::inferences::gisterrain_artifact_inference_descriptor()])
        .composers(crate::artifacts::gisterrain::standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::gis3d::Gis3dPlayApp>>()
        .try_build()
}
//#endregion 🔖️Register

//#region 🔹Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn the_terrain_snapshot_defaults_to_a_flat_unimported_terrain() {
        let document = GisTerrainSnapshot::default();
        assert_eq!(document.exaggeration, 0.0);
        assert!(document.imported_features_json.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn the_terrain_snapshot_composes_a_content_addressed_mesh_child() {
        let document = GisTerrainSnapshot::default();
        assert!(document.mesh.is_some(), "the terrain always composes a mesh child, even when flat/unimported");
        let mesh = gis_terrain_mesh_from_snapshot(&document);
        assert_eq!(mesh.meshes.len(), 1);
        assert_eq!(mesh.meshes[0].primitives[0].positions.len(), 4, "a flat placeholder quad");
    }
}
//#endregion 🔹Tests
