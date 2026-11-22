from difflib import unified_diff
from pathlib import Path
import sys

PATHS = [
    "🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️.rs",
    "🧰️framework/🔨️modules/🗺️surface/🏔️terrain/📐️grid-projection/🦀️.rs",
    "🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🎯️pick/🦀️.rs",
    "🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🗺️overlay/🦀️.rs",
    "🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🧪️tests/🔬️unit/🦀️.rs",
    "🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🗺️overlay/🧪️tests/🦀️.rs",
    "🧰️framework/🔨️modules/🗺️surface/🧪️tests/🕸️browser/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🧪️tests/🔭️unit-2/🥽️brush-environment-and-hover-laws/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🧪️tests/📜️journal-sequences/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🌪️vortex-relocate-and-terrain/🦀️.rs",
    "🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🧪️tests/🧫️fixtures/📐️rectangular-elevation-grid/🔣️.json"
]
(terrain, projection, pick, overlay, unit, overlay_test, browser,
 native_test, journal_test, native, fixture) = (Path(path) for path in PATHS)
original = {path: path.read_bytes().decode("utf-8") if path.exists() else "" for path in map(Path, PATHS)}
changed = original.copy()


def edit(path, old, new, count=1):
    source = changed[path]
    if source.count(old) != count:
        raise RuntimeError(f"{path}: expected {count} matches for {old!r}, found {source.count(old)}")
    changed[path] = source.replace(old, new)


edit(unit, "let rows: Vec<Vec<i16>>", "let rows_millimetres: Vec<Vec<i32>>")
edit(unit, "(base + if raised { grid[\"meters\"].as_i64().unwrap() } else { 0 }) as i16",
     "((base + if raised { grid[\"meters\"].as_i64().unwrap() } else { 0 }) * 1000) as i32")
edit(unit, "TerrainElevationGrid { bounds, rows }", "TerrainElevationGrid { bounds, rows_millimetres }")
edit(unit, """    let rows = (0..size).map(|row| (0..size).map(|column| {
        samples["southwest"].as_i64().unwrap()
            + row * samples["northwardStep"].as_i64().unwrap()
            + column * samples["eastwardStep"].as_i64().unwrap()
    }).collect::<Vec<_>>()).collect::<Vec<_>>();""", """    let rows_millimetres = (0..size).map(|row| (0..size).map(|column| {
        1000 * (samples["southwest"].as_i64().unwrap()
            + row * samples["northwardStep"].as_i64().unwrap()
            + column * samples["eastwardStep"].as_i64().unwrap())
    }).collect::<Vec<_>>()).collect::<Vec<_>>();""")
edit(unit, '"bounds": fixture["bounds"], "rows": rows', '"bounds": fixture["bounds"], "rowsMillimetres": rows_millimetres')
edit(overlay_test, "let rows = (0..GRID as i16).map(|row| (0..GRID as i16).map(|column|", "let rows_millimetres = (0..GRID as i32).map(|row| (0..GRID as i32).map(|column|")
edit(overlay_test, "TerrainElevationGrid { bounds, rows }", "TerrainElevationGrid { bounds, rows_millimetres }")

edit(overlay_test, 'fixture[field].as_i64().unwrap() as i16', 'fixture[field].as_i64().unwrap() as i32 * 1000')

changed[fixture] = """{
  "grid": {
    "bounds": [5.556, 50.586, 5.562, 50.590],
    "rowsMillimetres": [[-3007, -1999, -1, 9], [-1125, 0, 3333, 6789], [125, 1234, 2875, 4001]]
  },
  "shape": [4, 3],
  "maximumShape": [65, 65],
  "maximumBytes": 233504,
  "origin": [5.5604, 50.5874],
  "exaggeration": 1.5,
  "expected": {
    "indices": [0,4,1,1,4,5,1,5,2,2,5,6,2,6,3,3,6,7,4,8,5,5,8,9,5,9,6,6,9,10,6,10,7,7,10,11],
    "northwestHeight": 0.1875,
    "southeastHeight": 0.0135,
    "originHeight": 4.536,
    "originSample": [2, 1]
  },
  "overlays": {
    "positions": [{"id":"center","name":"Center","coordinates":[5.5604,50.5874]}],
    "routes": [{"id":"crossing","name":"Crossing","coordinates":[[5.555,50.5868],[5.563,50.5892]]}],
    "regions": [{"id":"boundary","name":"Boundary","rings":[[[5.5568,50.5864],[5.5614,50.5868],[5.5606,50.5894],[5.5572,50.5896]]]}]
  }
}
"""
changed[unit] += r'''

#[test]
fn rectangular_fractional_grid_agrees_across_mesh_pick_and_drape() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📐️rectangular-elevation-grid/🔣️.json")).unwrap();
    let shape: [u32; 2] = serde_json::from_value(fixture["shape"].clone()).unwrap();
    let [lon, lat]: [f64; 2] = serde_json::from_value(fixture["origin"].clone()).unwrap();
    let grid: TerrainElevationGrid = from_json_str(&fixture["grid"].to_string(), JsonMemberPolicy::Reject).unwrap();
    assert_eq!(grid.dimensions(), Some(shape));
    assert!(grid.is_valid());
    let mut session = TerrainSessionCore::default();
    session.set_project_origin(lon, lat);
    session.set_exaggeration(fixture["exaggeration"].as_f64().unwrap());
    assert!(session.set_elevation_grid_json(&fixture["grid"].to_string()));
    let mesh = session.terrain_tile_mesh(0, 0, 0).unwrap();
    let expected = &fixture["expected"];
    assert_eq!(serde_json::json!(mesh.indices), expected["indices"]);
    let near = |value, key: &str| {
        assert!((value - expected[key].as_f64().unwrap()).abs() < 1e-5, "{key}: {value}");
    };
    near(f64::from(mesh.positions[2]), "northwestHeight");
    near(f64::from(*mesh.positions.last().unwrap()), "southeastHeight");
    near(f64::from(session.terrain_origin_elevation(0, 0, 0).unwrap()), "originHeight");
    let hit = session.pick_elevation_grid([0.0, 0.0, 10.0], [0.0, 0.0, -1.0]).unwrap();
    assert_eq!(serde_json::json!(hit.sample), expected["originSample"]);
    near(hit.position[2], "originHeight");
    let overlays = session.overlay_tile(0, 0, 0, &from_json_str(&fixture["overlays"].to_string(), JsonMemberPolicy::Reject).unwrap()).unwrap();
    near(f64::from(overlays.positions[0].point[2]), "originHeight");
    for path in overlays.routes.iter().chain(&overlays.regions) {
        assert!(path.segments.len() > 3);
        for [a, b] in &path.segments {
            let [x, y, z] = std::array::from_fn(|axis| (f64::from(a[axis]) + f64::from(b[axis])) * 0.5);
            let hit = session.pick_elevation_grid([x, y, 20.0], [0.0, 0.0, -1.0]).unwrap();
            assert!((hit.position[2] - z).abs() < 1e-5);
        }
    }
}

#[test]
fn authored_grid_width_height_and_storage_are_bounded() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📐️rectangular-elevation-grid/🔣️.json")).unwrap();
    let [width, height]: [usize; 2] = serde_json::from_value(fixture["maximumShape"].clone()).unwrap();
    let bounds = serde_json::from_value(fixture["grid"]["bounds"].clone()).unwrap();
    let mut grid = TerrainElevationGrid { bounds, rows_millimetres: vec![vec![i32::MIN; width]; height] };
    grid.rows_millimetres[0][0] = i32::MAX;
    let mesh = grid.mesh(5.556, 50.586, 1.0).unwrap();
    assert_eq!(mesh.indices.len(), (width - 1) * (height - 1) * 6);
    let bytes = 4 * (mesh.positions.len() + mesh.normals.len() + mesh.indices.len() + mesh.uvs.len());
    assert_eq!(bytes as u64, fixture["maximumBytes"].as_u64().unwrap());
    for [width, height] in [[1, 2], [2, 1], [66, 2], [2, 66]] {
        grid.rows_millimetres = vec![vec![0; width]; height];
        assert!(!grid.is_valid());
        assert!(grid.mesh(5.556, 50.586, 1.0).is_none());
    }
    grid.rows_millimetres = vec![vec![0; 2], vec![0; 3]];
    assert!(!grid.is_valid());
    let mut session = TerrainSessionCore::default();
    for invalid in [
        r#"{"bounds":[0,0,1,1],"rowsMillimetres":[[0,0],[0,2147483648]]}"#,
        r#"{"bounds":[0,0,1,1],"rowsMillimetres":[[0,0],[0,-2147483649]]}"#,
        r#"{"bounds":[0,0,1,1],"rowsMillimetres":[[0,0],[0,0.1]]}"#,
        r#"{"bounds":[0,0,1,1],"rows":[[0,0],[0,0]]}"#,
    ] {
        assert!(!session.set_elevation_grid_json(invalid));
    }
}
'''
edit(browser, 'import elevationGridFixture from "../../🏔️terrain/🧪️tests/🧫️fixtures/📦️elevation-grid/🔣️.json";', '''import elevationGridFixture from "../../🏔️terrain/🧪️tests/🧫️fixtures/📦️elevation-grid/🔣️.json";
import rectangularElevationGrid from "../../🏔️terrain/🧪️tests/🧫️fixtures/📐️rectangular-elevation-grid/🔣️.json";''')
edit(browser, "{ bounds, rows: [[40]] }", "{ bounds, rowsMillimetres: [[40_000]] }")
edit(browser, "const rows = Array.from({ length: samples.size }", "const rowsMillimetres = Array.from({ length: samples.size }", 2)
edit(browser, "samples.southwest + row * samples.northwardStep + column * samples.eastwardStep));", "1_000 * (samples.southwest + row * samples.northwardStep + column * samples.eastwardStep)));", 2)
edit(browser, "{ bounds, rows }", "{ bounds, rowsMillimetres }")
edit(browser, "rows[raised.row]![raised.column]! += raised.meters;", "rowsMillimetres[raised.row]![raised.column]! += 1_000 * raised.meters;")
edit(browser, "{ bounds: [west, south, east, north], rows }", "{ bounds: [west, south, east, north], rowsMillimetres }")
changed[browser] += '''

test("A fractional rectangular authored grid drapes and picks against the browser surface", async () => {
 const { grid, origin, exaggeration, expected, overlays } = rectangularElevationGrid;
 const module = await import(fileUrl(file("framework_surface.js")).href);
 await module.default({ module_or_path: await WebAssembly.compile(readFileSync(file("framework_surface_bg.wasm"))) });
 const session = new module.TerrainSession();
 const geometry = new BufferGeometry();
 const material = new MeshBasicMaterial({ side: DoubleSide });
 try {
  session.set_project_origin(origin[0], origin[1]);
  session.set_exaggeration(exaggeration);
  expect(session.set_elevation_grid_json(JSON.stringify(grid))).toBe(true);
  const data = JSON.parse(session.terrain_tile_mesh_json(0, 0, 0));
  expect(data.indices).toEqual(expected.indices);
  expect(data.positions[2]).toBeCloseTo(expected.northwestHeight, 6);
  expect(data.positions.at(-1)).toBeCloseTo(expected.southeastHeight, 6);
  geometry.setAttribute("position", new Float32BufferAttribute(data.positions, 3));
  geometry.setIndex(data.indices);
  const mesh = new Mesh(geometry, material);
  const ray = { origin: [0, 0, 20], direction: [0, 0, -1] };
  const hit = JSON.parse(session.pickElevationGridJson(JSON.stringify(ray)));
  expect(hit.sample).toEqual(expected.originSample);
  expect(hit.position[2]).toBeCloseTo(expected.originHeight, 5);
  expect(new Raycaster(new Vector3(0, 0, 20), new Vector3(0, 0, -1)).intersectObject(mesh)[0]!.point.z).toBeCloseTo(hit.position[2], 5);
  const draped = JSON.parse(session.terrain_overlay_tile_json(0, 0, 0, JSON.stringify(overlays)));
  expect(draped.positions[0].point[2]).toBeCloseTo(expected.originHeight, 5);
  for (const path of [...draped.routes, ...draped.regions]) {
   expect(path.segments.length).toBeGreaterThan(3);
   for (const [a, b] of path.segments) {
    const [x, y, z] = a.map((value: number, axis: number) => (value + b[axis]) / 2);
    const oracle = new Raycaster(new Vector3(x, y, 20), new Vector3(0, 0, -1)).intersectObject(mesh)[0];
    expect(oracle!.point.z).toBeCloseTo(z, 5);
   }
  }
 } finally {
  material.dispose();
  geometry.dispose();
  session.free();
 }
});
'''
edit(journal_test, '"rows": vec![vec![height; 33]; 33]', '"rowsMillimetres": vec![vec![i32::from(height) * 1000; 33]; 33]')
edit(native_test, 'let rows: Vec<_> = (99..=131).map(|height| vec![height; 33]).collect();', '''
    let rows_millimetres: Vec<_> = [99_125, 115_375, 131_625].into_iter().map(|height| vec![height; 5]).collect();'''.lstrip())
edit(native_test, '"rows": rows', '"rowsMillimetres": rows_millimetres')
edit(native_test, 'overlays.positions[0].point[2] - 115.0', 'overlays.positions[0].point[2] - 115.375')
edit(native_test, '    assert!(state.terrain.pending_terrain_tile_urls.is_empty());\n    assert!(take_next_world3d_asset(&mut state.assets).is_none());\n\n    scene.world_3d.as_mut().unwrap().terrain_json = None;', '''    assert_eq!(state.terrain.pick_terrain_sample([0.0, 0.0, 300.0], [0.0, 0.0, -1.0]), Some([2, 1]));
    assert!(state.terrain.pending_terrain_tile_urls.is_empty());
    assert!(take_next_world3d_asset(&mut state.assets).is_none());

    scene.world_3d.as_mut().unwrap().terrain_json = None;''')

edit(terrain, "use std::collections::HashMap;", """use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::OnceLock;""")
edit(terrain, "#[derive(semio_framework_value::ToValue, semio_framework_value::FromValue)]", "#[derive(Clone, semio_framework_value::ToValue, semio_framework_value::FromValue)]")
start = changed[terrain].index("/// 🏔️ Fixed-resolution WGS84 elevation grid.")
end = changed[terrain].index("\nfn build_terrain_tile_mesh", start)
changed[terrain] = changed[terrain][:start] + '''/// 🏔️ Rectangular WGS84 sample-centre bounds in west, south, east, north order.
/// Elevation rows run south to north, with each row ordered west to east in signed millimetres.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct TerrainElevationGrid {
    pub bounds: [f64; 4],
    pub rows_millimetres: Vec<Vec<i32>>,
}

impl TerrainElevationGrid {
    /// 📐️ Returns validated sample dimensions in column, row order.
    pub fn dimensions(&self) -> Option<[u32; 2]> {
        let height = self.rows_millimetres.len();
        let width = self.rows_millimetres.first()?.len();
        if !(2..=65).contains(&width) || !(2..=65).contains(&height)
            || self.rows_millimetres.iter().any(|row| row.len() != width) {
            return None;
        }
        Some([width as u32, height as u32])
    }

    /// 🗺️ Validates geographic extent and the bounded rectangular sample shape.
    pub fn is_valid(&self) -> bool {
        let [west, south, east, north] = self.bounds;
        (-180.0..east).contains(&west) && east <= 180.0
            && (-90.0..north).contains(&south) && north <= 90.0
            && self.dimensions().is_some()
    }

    /// 🧊️ Builds the same grid triangles drawn in the viewport and published as artifact geometry.
    pub fn mesh(&self, origin_lon: f64, origin_lat: f64, exaggeration: f64) -> Option<TerrainTileMeshJson> {
        let maximum_exaggeration = f64::from(f32::MAX) / (f64::from(i32::MIN.unsigned_abs()) / 1_000.0);
        if !self.is_valid() || !(-180.0..=180.0).contains(&origin_lon) || !(-90.0..=90.0).contains(&origin_lat)
            || !(0.0..=maximum_exaggeration).contains(&exaggeration) {
            return None;
        }
        let dimensions = self.dimensions()?;
        let heights = self.rows_millimetres.iter().rev().flatten().map(|&sample| f64::from(sample) / 1_000.0).collect::<Vec<_>>();
        Some(build_terrain_grid_mesh(&heights, self.bounds, [origin_lon, origin_lat], exaggeration, dimensions))
    }
}
''' + changed[terrain][end:]
edit(terrain, "vec![0.0f32; (n * n) as usize]", "vec![0.0_f64; (n * n) as usize]")
edit(terrain, " = sample_elevation(&tile.image, px, py);", " = f64::from(sample_elevation(&tile.image, px, py));")
edit(terrain, "build_terrain_grid_mesh(&heights, [min_lon, min_lat, max_lon, max_lat], origin_lon, origin_lat, exaggeration)", "build_terrain_grid_mesh(&heights, [min_lon, min_lat, max_lon, max_lat], [origin_lon, origin_lat], exaggeration, [n, n])")
edit(terrain, """fn build_terrain_grid_mesh(heights: &[f32], bounds: [f64; 4], origin_lon: f64, origin_lat: f64, exaggeration: f64) -> TerrainTileMeshJson {
    let n = TERRAIN_GRID_RESOLUTION;
    let projection = TerrainGridMeshProjection::from_bounds(bounds, [origin_lon, origin_lat], exaggeration);
    let height = |row, column| heights[(row * n + column) as usize];
    let minimum = heights.iter().copied().fold(f32::INFINITY, f32::min);
    let maximum = heights.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let mut positions = Vec::with_capacity((n * n * 3) as usize);
    let mut normals = Vec::with_capacity((n * n * 3) as usize);
    let mut uvs = Vec::with_capacity((n * n * 2) as usize);
    let mut indices = Vec::with_capacity(((n - 1) * (n - 1) * 6) as usize);
    for row in 0..n {
        for column in 0..n {""", """fn build_terrain_grid_mesh(heights: &[f64], bounds: [f64; 4], origin: [f64; 2], exaggeration: f64, dimensions: [u32; 2]) -> TerrainTileMeshJson {
    let [width, rows] = dimensions;
    let projection = TerrainGridMeshProjection::from_bounds(bounds, origin, exaggeration, dimensions);
    let height = |row, column| heights[(row * width + column) as usize];
    let minimum = heights.iter().copied().fold(f64::INFINITY, f64::min);
    let maximum = heights.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mut positions = Vec::with_capacity(heights.len() * 3);
    let mut normals = Vec::with_capacity(heights.len() * 3);
    let mut uvs = Vec::with_capacity(heights.len() * 2);
    let mut indices = Vec::with_capacity(((width - 1) * (rows - 1) * 6) as usize);
    for row in 0..rows {
        for column in 0..width {""")
edit(terrain, "    for row in 0..n - 1 {\n        for column in 0..n - 1 {", "    for row in 0..rows - 1 {\n        for column in 0..width - 1 {")
edit(terrain, "    elevation_grid: Option<TerrainElevationGrid>,", "    elevation_grid: Option<TerrainElevationGrid>,\n    elevation_grid_mesh: OnceLock<Option<TerrainTileMeshJson>>,")
edit(terrain, "elevation_grid: None }", "elevation_grid: None, elevation_grid_mesh: OnceLock::new() }")
edit(terrain, """        self.origin_lon = lon;
        self.origin_lat = lat;""", """        if [lon, lat] != [self.origin_lon, self.origin_lat] {
            self.elevation_grid_mesh.take();
            self.origin_lon = lon;
            self.origin_lat = lat;
        }""")
edit(terrain, "        self.exaggeration = exaggeration.max(0.0);", """        let exaggeration = exaggeration.max(0.0);
        if self.exaggeration != exaggeration {
            self.elevation_grid_mesh.take();
            self.exaggeration = exaggeration;
        }""")
edit(terrain, "        self.elevation_grid = grid;", """        if self.elevation_grid != grid {
            self.elevation_grid_mesh.take();
            self.elevation_grid = grid;
        }""")
edit(terrain, """    fn tile_mesh(&self, z: u32, x: u32, y: u32) -> Option<TerrainTileMeshJson> {
        if let Some(grid) = &self.elevation_grid {
            return ((z, x, y) == ELEVATION_GRID_TILE).then(|| grid.mesh(self.origin_lon, self.origin_lat, self.exaggeration)).flatten();
        }
        let tile = self.elevation.by_key.get(&tiles::tile_key(z, x, y))?;
        Some(build_terrain_tile_mesh(tile, self.origin_lon, self.origin_lat, self.exaggeration))
    }

    /// 🕸️ Tier-(e) mesh build ([\x60build_terrain_tile_mesh\x60]) over a cached decoded tile, JSON-wrapped.
    pub fn terrain_tile_mesh_json(&self, z: u32, x: u32, y: u32) -> String {
        self.tile_mesh(z, x, y).map_or_else(|| "null".to_owned(), |mesh| to_json_string(&mesh))
    }""", """    fn tile_mesh(&self, z: u32, x: u32, y: u32) -> Option<Cow<'_, TerrainTileMeshJson>> {
        if let Some(grid) = &self.elevation_grid {
            if (z, x, y) != ELEVATION_GRID_TILE {
                return None;
            }
            let mesh = self.elevation_grid_mesh.get_or_init(|| grid.mesh(self.origin_lon, self.origin_lat, self.exaggeration));
            return mesh.as_ref().map(Cow::Borrowed);
        }
        let tile = self.elevation.by_key.get(&tiles::tile_key(z, x, y))?;
        Some(Cow::Owned(build_terrain_tile_mesh(tile, self.origin_lon, self.origin_lat, self.exaggeration)))
    }

    /// 🏗️ Supplies owned renderer buffers for native mesh publication.
    pub fn terrain_tile_mesh(&self, z: u32, x: u32, y: u32) -> Option<TerrainTileMeshJson> {
        self.tile_mesh(z, x, y).map(Cow::into_owned)
    }

    /// 🕸️ Publishes terrain buffers across the browser WASM boundary.
    pub fn terrain_tile_mesh_json(&self, z: u32, x: u32, y: u32) -> String {
        self.tile_mesh(z, x, y).map_or_else(|| "null".to_owned(), |mesh| to_json_string(mesh.as_ref()))
    }""")

edit(projection, "use super::{normalize3, projection, TERRAIN_GRID_RESOLUTION};", "use super::{normalize3, projection};")
edit(projection, "    cell_size: [f32; 2],", "    cells: [u32; 2],\n    cell_size: [f64; 2],")
edit(projection, """    pub fn from_bounds(bounds: [f64; 4], origin: [f64; 2], exaggeration: f64) -> Self {
        let mut projection = Self { bounds, origin, exaggeration, cell_size: [0.0; 2] };
        let [x, y] = projection.local_position(0, 0);
        projection.cell_size = [
            (projection.local_position(0, 1)[0] - x).abs().max(1e-3),
            (y - projection.local_position(1, 0)[1]).abs().max(1e-3),
        ];
        projection
    }""", """    pub fn from_bounds(bounds: [f64; 4], origin: [f64; 2], exaggeration: f64, [width, height]: [u32; 2]) -> Self {
        let [west, south, east, north] = bounds;
        let cells = [width - 1, height - 1];
        let (span_x, span_y) = projection::lonlat_to_local_meters(east, north, west, south);
        let span_x = span_x / south.to_radians().cos() * origin[1].to_radians().cos();
        let cell_size = [(span_x / f64::from(cells[0])).abs().max(1e-3), (span_y / f64::from(cells[1])).abs().max(1e-3)];
        Self { bounds, origin, exaggeration, cells, cell_size }
    }""")
edit(projection, "        let u = f64::from(column) / f64::from(TERRAIN_GRID_RESOLUTION - 1);", "        let u = f64::from(column) / f64::from(self.cells[0]);")
edit(projection, "        let v = f64::from(row) / f64::from(TERRAIN_GRID_RESOLUTION - 1);", "        let v = f64::from(row) / f64::from(self.cells[1]);")
edit(projection, "height: f32) -> [f32; 3]", "height: f64) -> [f32; 3]")
edit(projection, "(f64::from(height) * self.exaggeration)", "(height * self.exaggeration)")
edit(projection, "height: impl Fn(u32, u32) -> f32", "height: impl Fn(u32, u32) -> f64")
edit(projection, "(column + 1).min(TERRAIN_GRID_RESOLUTION - 1)", "(column + 1).min(self.cells[0])")
edit(projection, "(row + 1).min(TERRAIN_GRID_RESOLUTION - 1)", "(row + 1).min(self.cells[1])")
edit(projection, "f64::from(height(row, east) - height(row, west))", "(height(row, east) - height(row, west))")
edit(projection, "f64::from(height(north, column) - height(south, column))", "(height(north, column) - height(south, column))")
edit(projection, "f64::from(self.cell_size[0])", "self.cell_size[0]")
edit(projection, "f64::from(self.cell_size[1])", "self.cell_size[1]")
edit(projection, "pub fn uv(height: f32, minimum: f32, maximum: f32)", "pub fn uv(height: f64, minimum: f64, maximum: f64)")
edit(projection, "((height - minimum) / range).clamp(0.0, 1.0)", "((height - minimum) / range).clamp(0.0, 1.0) as f32")
edit(projection, """        let i = row * TERRAIN_GRID_RESOLUTION + column;
        [i, i + TERRAIN_GRID_RESOLUTION, i + 1, i + 1, i + TERRAIN_GRID_RESOLUTION, i + TERRAIN_GRID_RESOLUTION + 1]""", """        let stride = self.cells[0] + 1;
        let i = row * stride + column;
        [i, i + stride, i + 1, i + 1, i + stride, i + stride + 1]""")

edit(pick, ", TERRAIN_GRID_RESOLUTION", "")
edit(pick, "one of the embedded grid's 2,048 rendered triangles", "the embedded grid's rendered triangles")
edit(pick, """        let cells = f64::from(TERRAIN_GRID_RESOLUTION - 1);
        let sample_index = |fraction: f64| (fraction * cells).round().clamp(0.0, cells) as u32;""", """        let [width, height] = grid.dimensions()?;
        let sample_index = |fraction: f64, extent| {
            let cells = f64::from(extent - 1);
            (fraction * cells).round().clamp(0.0, cells) as u32
        };""")
edit(pick, "sample_index((lon - west) / (east - west)), sample_index((lat - south) / (north - south))", "sample_index((lon - west) / (east - west), width), sample_index((lat - south) / (north - south), height)")

edit(projection, '''        let (span_x, span_y) = projection::lonlat_to_local_meters(east, north, west, south);
        let span_x = span_x / south.to_radians().cos() * origin[1].to_radians().cos();''', '''        let (west_x, south_y) = projection::lonlat_to_local_meters(west, south, origin[0], origin[1]);
        let (east_x, north_y) = projection::lonlat_to_local_meters(east, north, origin[0], origin[1]);
        let (span_x, span_y) = (east_x - west_x, north_y - south_y);''')

edit(overlay, "const GRID: usize = TERRAIN_GRID_RESOLUTION as usize;\nconst CELLS: f64 = (GRID - 1) as f64;", "#[cfg(test)]\nconst GRID: usize = TERRAIN_GRID_RESOLUTION as usize;")
edit(overlay, "    inline_bounds: Option<[f64; 4]>,", "    inline_bounds: Option<[f64; 4]>,\n    width: usize,\n    cells: [f64; 2],")
edit(overlay, "    fn coordinates(&self, [lon, lat]: [f64; 2]) -> [f64; 2] {", "    fn coordinates(&self, [lon, lat]: [f64; 2]) -> [f64; 2] {\n        let [columns, rows] = self.cells;")
edit(overlay, "[(lon - west) / (east - west) * CELLS, (lat - north) / (south - north) * CELLS]", "[(lon - west) / (east - west) * columns, (lat - north) / (south - north) * rows]")
edit(overlay, "(x - f64::from(first[0])) / (f64::from(last[0]) - f64::from(first[0])) * CELLS", "(x - f64::from(first[0])) / (f64::from(last[0]) - f64::from(first[0])) * columns")
edit(overlay, "(y - f64::from(first[1])) / (f64::from(last[1]) - f64::from(first[1])) * CELLS", "(y - f64::from(first[1])) / (f64::from(last[1]) - f64::from(first[1])) * rows")
edit(overlay, "coordinate.into_iter().all(|axis| (0.0..CELLS).contains(&axis))", "coordinate.into_iter().zip(self.cells).all(|(axis, extent)| (0.0..extent).contains(&axis))")
edit(overlay, """        let x = x.clamp(0.0, CELLS);
        let y = y.clamp(0.0, CELLS);
        let column = (x.floor() as usize).min(GRID - 2);
        let row = (y.floor() as usize).min(GRID - 2);
        let index = row * GRID + column;""", """        let [columns, rows] = self.cells;
        let x = x.clamp(0.0, columns);
        let y = y.clamp(0.0, rows);
        let column = (x.floor() as usize).min(columns as usize - 1);
        let row = (y.floor() as usize).min(rows as usize - 1);
        let index = row * self.width + column;""")
edit(overlay, "index + GRID", "index + self.width", 3)
edit(overlay, "clip_segment(start, delta)", "clip_segment(start, delta, self.cells)")
edit(overlay, "fn clip_segment(start: [f64; 2], delta: [f64; 2])", "fn clip_segment(start: [f64; 2], delta: [f64; 2], cells: [f64; 2])")
edit(overlay, "for (coordinate, movement) in start.into_iter().zip(delta) {", "for ((coordinate, movement), extent) in start.into_iter().zip(delta).zip(cells) {")
edit(overlay, "(0.0..=CELLS).contains(&coordinate)", "(0.0..=extent).contains(&coordinate)")
edit(overlay, "let b = (CELLS - coordinate) / movement;", "let b = (extent - coordinate) / movement;")
edit(overlay, "impl TerrainSessionCore {\n", """impl TerrainSessionCore {
    fn overlay_grid<'a>(&self, positions: &'a [f32]) -> Option<OverlayGrid<'a>> {
        let (inline_bounds, dimensions) = match &self.elevation_grid {
            Some(grid) => (Some(grid.bounds), grid.dimensions()?),
            None => (None, [TERRAIN_GRID_RESOLUTION; 2]),
        };
        Some(OverlayGrid {
            positions,
            origin_lon: self.origin_lon,
            origin_lat: self.origin_lat,
            inline_bounds,
            width: dimensions[0] as usize,
            cells: dimensions.map(|samples| f64::from(samples - 1)),
        })
    }

""")
edit(overlay, "let grid = OverlayGrid { positions: &mesh.positions, origin_lon: self.origin_lon, origin_lat: self.origin_lat, inline_bounds: self.elevation_grid.as_ref().map(|grid| grid.bounds) };", "let grid = self.overlay_grid(&mesh.positions)?;", 2)

edit(native, """                let mesh_json = terrain_state.terrain_session.terrain_tile_mesh_json(z, x, y);
                if let Ok(mesh_payload) = serde_json::from_str::<TerrainTileMeshPayload>(&mesh_json) {""", """                if let Some(mesh) = terrain_state.terrain_session.terrain_tile_mesh(z, x, y) {
                    let mesh_payload = TerrainTileMeshPayload { positions: mesh.positions, normals: mesh.normals, indices: mesh.indices, uvs: mesh.uvs };""")


edit(overlay, "    let (mut enter, mut leave) = (0.0_f64, 1.0_f64);", """    if !start.into_iter().chain(delta).all(f64::is_finite) {
        return None;
    }
    let (mut enter, mut leave) = (0.0_f64, 1.0_f64);""")
edit(overlay, """            for (offset, slope) in [(start[0], delta[0]), (start[1], delta[1]), (start[0] + start[1], delta[0] + delta[1])] {
                if slope == 0.0 {
                    continue;
                }
                let first = (offset + slope * enter).min(offset + slope * leave).ceil() as i32;
                let last = (offset + slope * enter).max(offset + slope * leave).floor() as i32;""", """            let [columns, rows] = self.cells;
            for (offset, slope, extent) in [(start[0], delta[0], columns), (start[1], delta[1], rows), (start[0] + start[1], delta[0] + delta[1], columns + rows)] {
                if !offset.is_finite() || !slope.is_finite() || slope == 0.0 {
                    continue;
                }
                let a = slope.mul_add(enter, offset);
                let b = slope.mul_add(leave, offset);
                let first = a.min(b).ceil().max(0.0) as i32;
                let last = a.max(b).floor().min(extent) as i32;""")
edit(unit, "let near = |value, key: &str|", "let near = |value: f64, key: &str|")
patch = []
for path, before in original.items():
    after = changed[path]
    if after == before:
        continue
    if any(len(str(part).encode()) > 256 for part in [path, path.absolute()]):
        raise RuntimeError(f"{path}: path exceeds byte limit")
    patch.extend(unified_diff(
        before.splitlines(keepends=True), after.splitlines(keepends=True),
        "/dev/null" if not before else f"a/{path}", f"b/{path}", n=3
    ))
destination = Path(__file__).with_name("terrain-renderer-resolution.patch")
contents = "".join(patch)
if "--apply" in sys.argv:
    for path, before in original.items():
        after = changed[path]
        if after == before:
            continue
        if (path.read_bytes().decode("utf-8") if path.exists() else "") != before:
            raise RuntimeError(f"{path}: changed during patch assembly")
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(after.encode("utf-8"))
else:
    destination.write_bytes(contents.encode("utf-8"))
    print(destination)
