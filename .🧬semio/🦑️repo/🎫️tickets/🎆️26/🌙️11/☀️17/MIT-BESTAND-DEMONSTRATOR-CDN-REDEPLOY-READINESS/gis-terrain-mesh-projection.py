import re
import subprocess
from pathlib import Path


ticket = Path(__file__).resolve().parent
terrain = Path("🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️.rs").read_text()
projection = terrain[terrain.index("pub mod projection {"):terrain.index("//#endregion Projection")]
original = terrain[terrain.index("fn build_terrain_grid_mesh("):terrain.index("//#endregion TerrainTileMesh")]
const = re.search(r"const TERRAIN_GRID_RESOLUTION: u32 = [0-9]+;", terrain).group()
candidate = (ticket / "gis-terrain-mesh-projection.rs").read_text()
test = """
struct TerrainTileMeshJson {
    positions: Vec<f32>,
    normals: Vec<f32>,
    indices: Vec<u32>,
    uvs: Vec<f32>,
}

#[test]
fn same_exact_mesh_for_flat_varied_and_polar_grids() {
    let size = TERRAIN_GRID_RESOLUTION;
    for varied in [false, true] {
        for bounds in [[3.0, 50.0, 5.0, 52.0], [-179.0, 89.0, -178.0, 90.0]] {
            for exaggeration in [0.0, 1.75, 120.0] {
                let heights: Vec<f32> = (0..size * size)
                    .rev()
                    .map(|n| if varied { (n % 157) as f32 - 81.0 } else { 13.0 })
                    .collect();
                let origin = [bounds[0] + (bounds[2] - bounds[0]) / 2.0,
                              bounds[1] + (bounds[3] - bounds[1]) / 2.0];
                let old = build_terrain_grid_mesh(&heights, bounds, origin[0], origin[1], exaggeration);
                let view = TerrainGridMeshProjection::from_bounds(bounds, origin, exaggeration, size);
                let min = heights.iter().copied().fold(f32::INFINITY, f32::min);
                let max = heights.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                let height = |row, col| heights[(row * size + col) as usize];
                for row in 0..size {
                    for col in 0..size {
                        let i = (row * size + col) as usize;
                        assert_eq!(view.position(row, col, height(row, col)).map(f32::to_bits),
                                   old.positions[3*i..3*i+3].iter().map(|x| x.to_bits()).collect::<Vec<_>>().as_slice());
                        assert_eq!(view.normal(row, col, height).map(f32::to_bits),
                                   old.normals[3*i..3*i+3].iter().map(|x| x.to_bits()).collect::<Vec<_>>().as_slice());
                        assert_eq!(TerrainGridMeshProjection::uv(height(row, col), min, max).map(f32::to_bits),
                                   old.uvs[2*i..2*i+2].iter().map(|x| x.to_bits()).collect::<Vec<_>>().as_slice());
                        if row < size-1 && col < size-1 {
                            let cell = (row * (size-1) + col) as usize;
                            assert_eq!(view.triangles(row, col), old.indices[6*cell..6*cell+6]);
                        }
                    }
                }
            }
        }
    }
}
"""
out = ticket / "🗑️generated" / "gis-terrain-scalar-law"
source = "\n".join((const, projection, original, candidate, test))
subprocess.run(["repo", "tools", "exec", "rustc", "--", "--edition=2021", "--test", "-", "-o", str(out)],
               input=source, text=True, check=True)
subprocess.run([str(out)], check=True)
