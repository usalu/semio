//! ⚙️ GIS 3D app — headless compute (constitutional: engine).

use gis3d::Gis3dTerrainDocument;
use gis3d_dsl::REUSE_TERRAIN_EXAMPLE_TEXT;

//#region 🔖DocumentHelpers
pub fn empty_gis3d_terrain_projection() -> Gis3dTerrainDocument {
    Gis3dTerrainDocument { exaggeration: 1.0 }
}

/// 🗺️ The default terrain document, seeded from the bundled reuse example's `gisterrain
/// exaggeration=...` header (see `gis3d::Gis3dTerrainDocument`'s derive-generated `.gisterrain` DSL).
pub fn default_terrain_document() -> Gis3dTerrainDocument {
    <Gis3dTerrainDocument as store::DocumentDsl>::parse_dsl(REUSE_TERRAIN_EXAMPLE_TEXT).unwrap_or_else(|_| empty_gis3d_terrain_projection())
}
//#endregion 🔖DocumentHelpers
