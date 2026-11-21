from pathlib import Path
import subprocess

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated" / "gis-next"
lane = Path(".cache/b/r/m/u/l/p0dd7ed/release/build")
dependencies = generated / "host-deps"
dependencies.mkdir(exist_ok=True)
for pattern in ["*.rlib", "*.rmeta", "*.dylib"]:
    for artifact in lane.glob(f"*/*/out/{pattern}"):
        link = dependencies / artifact.name
        if not link.is_symlink():
            link.symlink_to(artifact.resolve())
roots = ["semio_framework_job", "semio_framework_pack_json", "semio_framework_value", "serde_json"]
externs = []
for name in roots:
    path, = dependencies.glob(f"lib{name}-*.rmeta")
    externs.extend(["--extern", f"{name}={path}", "--extern", f"{name}={path.with_suffix('.rlib')}"])
source = (ticket / "gis-original-projection-law.rs").read_text()
source += "\nfn write_borrowed" + (generated / "json.rs").read_text().split("\nfn write_borrowed", 1)[1]
terrain = Path("🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️.rs").read_text()
projection = terrain[terrain.index("pub mod projection {"):terrain.index("//#endregion Projection")]
baseline = terrain[terrain.index("fn build_terrain_grid_mesh("):terrain.index("//#endregion TerrainTileMesh")]
source += "\n" + projection
source += "\nmod baseline {use super::{projection,grid::TerrainGridMeshProjection};const TERRAIN_GRID_RESOLUTION:u32=33;pub struct TerrainTileMeshJson{pub positions:Vec<f32>,pub normals:Vec<f32>,pub indices:Vec<u32>,pub uvs:Vec<f32>}\n" + baseline.replace("fn build_terrain_grid_mesh","pub(super) fn build_terrain_grid_mesh",1)+"\n}"
source += """
mod grid {
use super::projection;
const TERRAIN_GRID_RESOLUTION:u32 = 33;
fn normalize3(x:f64,y:f64,z:f64)->(f64,f64,f64) {
  let length=(x*x+y*y+z*z).sqrt().max(1e-9);
  (x/length,y/length,z/length)
}
"""
source += (generated / "grid.rs").read_text().split("\n", 1)[1] + "\n}"
source += "\nmod scene { use super::grid::TerrainGridMeshProjection;\n" + (generated / "scene.rs").read_text().split("\n", 1)[1] + "\n}"
source += "\nmod map_view {\npub struct MapFeature { pub data: DslValue }\npub struct GisMapSnapshot { pub positions: Vec<MapFeature>, pub routes: Vec<MapFeature>, pub regions: Vec<MapFeature> }\n"
source += (generated / "map-view.rs").read_text().split("\n", 1)[1] + "\n}"
source += "\nmod map_validation {\n" + (generated / "map-validation.rs").read_text() + "\n}"
source += """
mod elevation_view {
#[derive(semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ElevationSource { pub bounds: [f64; 4], pub rows: Vec<Vec<i16>> }
"""
source += (generated / "elevation.rs").read_text().split("\n", 1)[1] + "\n}"
binary = generated / "projection-law"
subprocess.run(["repo", "tools", "exec", "rustc", "--", "--edition=2021", "--crate-name", "gis_original_projection_law",
    "-L", f"dependency={dependencies}", "-o", str(binary), *externs, "-"], input=source, text=True, check=True)
subprocess.run([str(binary)], check=True)
