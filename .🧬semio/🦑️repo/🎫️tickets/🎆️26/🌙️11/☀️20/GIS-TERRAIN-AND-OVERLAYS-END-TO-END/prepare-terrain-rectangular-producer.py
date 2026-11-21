"""🗻 Prepare the Terrain producer changes for rectangular elevation storage."""

from difflib import unified_diff
from pathlib import Path


TICKET = Path(__file__).resolve().parent
ARTIFACT = Path("✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any")
updates = []


def prepare(relative, replacements):
    """📐 Stage only the intended changes against each current owning module."""
    path = ARTIFACT / relative
    before = path.read_text(encoding="utf-8")
    after = before
    for old, new in replacements:
        if after.count(old) != 1:
            raise RuntimeError(f"{path}: expected one occurrence of {old!r}")
        after = after.replace(old, new)
    if before != after:
        updates.extend(unified_diff(before.splitlines(keepends=True), after.splitlines(keepends=True), "a/" + path.as_posix(), "b/" + path.as_posix()))


prepare("✏️editor/📤️media-export/📐️scene/🦀️.rs", [
    ("const SIZE: usize = 33;\n\n", ""),
    ("    minimum: i16,\n    maximum: i16,", "    minimum: i32,\n    maximum: i32,"),
    ("pub fn step(&mut self, rows: &[Vec<i16>]) -> Result<bool, ValueError> {\n        if rows.len() != SIZE {\n            return Err(invalid_terrain());\n        }", "pub fn step(&mut self, rows: &[Vec<i32>], width: usize) -> Result<bool, ValueError> {"),
    ("        if row.len() != SIZE {", "        if row.len() != width {"),
    ("    pub rows: &'a [Vec<i16>],", "    pub rows: &'a [Vec<i32>],\n    pub dimensions: (usize, usize),"),
    ("fn height(&self, row: u32, column: u32) -> f32 {\n        f32::from(self.rows[SIZE - 1 - row as usize][column as usize])", "fn height(&self, row: u32, column: u32) -> f64 {\n        f64::from(self.rows[self.dimensions.1 - 1 - row as usize][column as usize]) / 1000.0"),
    ("        let sample = |index, width| (index / width / SIZE, index / width % SIZE, index % width);", "        let (width, height) = self.dimensions;\n        let sample = |index, components| (index / components / width, index / components % width, index % components);"),
    ("0 | 1 if index < SIZE * SIZE * 3", "0 | 1 if index < width * height * 3"),
    ("3 if index < (SIZE - 1) * (SIZE - 1) * 6", "3 if index < (width - 1) * (height - 1) * 6"),
    ("self.projection.triangles((cell / (SIZE - 1)) as u32, (cell % (SIZE - 1)) as u32)", "self.projection.triangles((cell / (width - 1)) as u32, (cell % (width - 1)) as u32)"),
    ("4 if index < SIZE * SIZE * 2", "4 if index < width * height * 2"),
    ("f32::from(self.range.minimum), f32::from(self.range.maximum)", "f64::from(self.range.minimum) / 1000.0, f64::from(self.range.maximum) / 1000.0"),
    ("        Ok(match path {", "        let (width, height) = self.dimensions;\n        Ok(match path {"),
    ("[1, 0 | 1] => JsonWriteNode::Array(SIZE * SIZE * 3)", "[1, 0 | 1] => JsonWriteNode::Array(width * height * 3)"),
    ("[1, 3] => JsonWriteNode::Array((SIZE - 1) * (SIZE - 1) * 6)", "[1, 3] => JsonWriteNode::Array((width - 1) * (height - 1) * 6)"),
    ("[1, 4] => JsonWriteNode::Array(SIZE * SIZE * 2)", "[1, 4] => JsonWriteNode::Array(width * height * 2)"),
])

prepare("✏️editor/📤️media-export/🦀️.rs", [
    ("use crate::schema::imported_map::{TerrainGridMeshProjection, terrain_grid_origin};", "use crate::schema::imported_map::{TerrainGridMeshProjection, terrain_grid_origin};\nuse crate::schema::snapshot::ELEVATION_MAX_SIDE;"),
    ("33 * size_of::<i16>()", "ELEVATION_MAX_SIDE * size_of::<i32>()"),
    ("        let complete = state.step(&source.rows).map_err", "        let (width, height) = source.dimensions().ok_or(\"The authored terrain elevation grid is invalid\")?;\n        let complete = state.step(&source.rows_millimetres, width).map_err"),
    ("f64::from(f32::MAX) / f64::from(i16::MIN.unsigned_abs())", "f64::from(f32::MAX) * 1000.0 / f64::from(i32::MIN.unsigned_abs())"),
    ("TerrainGridMeshProjection::from_bounds(source.bounds, [origin.lon, origin.lat], snapshot.exaggeration)", "TerrainGridMeshProjection::from_bounds(source.bounds, [origin.lon, origin.lat], snapshot.exaggeration, [width as u32, height as u32])"),
    ("        Ok(TerrainJsonView::Scene(TerrainSceneView { rows: &source.rows, projection, range: *range }))", "        let dimensions = source.dimensions().ok_or_else(|| ValueError::invariant(\"the prepared terrain mesh dimensions are invalid\"))?;\n        Ok(TerrainJsonView::Scene(TerrainSceneView { rows: &source.rows_millimetres, dimensions, projection, range: *range }))"),
])

prepare("🚪️io/📝️text/📸️snapshot/🦀️.rs", [
    ("    let center_elevation = f64::from(source.rows[16][16]) * descriptor.exaggeration;", """    let rows = &source.rows_millimetres;
    let width = rows[0].len();
    let height = rows.len();
    let west = (width - 1) / 2;
    let east = width / 2;
    let south = (height - 1) / 2;
    let north = height / 2;
    let centre_millimetres = [rows[south][west], rows[south][east], rows[north][west], rows[north][east]].into_iter().map(f64::from).sum::<f64>() / 4.0;
    let center_elevation = centre_millimetres / 1000.0 * descriptor.exaggeration;"""),
])

prepare("✏️editor/📌️panels/🔍️terrain-data/🦀️.rs", [
    ("grid.rows.iter().flatten().copied().fold((i16::MAX, i16::MIN, 0_i32)", "grid.rows_millimetres.iter().flatten().copied().fold((i32::MAX, i32::MIN, 0_i64)"),
    ("sum + i32::from(height)", "sum + i64::from(height)"),
    ("        let dimensions = (grid.rows[0].len(), grid.rows.len());", "        let dimensions = (grid.rows_millimetres[0].len(), grid.rows_millimetres.len());\n        let minimum = f64::from(minimum) / 1000.0;\n        let maximum = f64::from(maximum) / 1000.0;"),
    ("f64::from(sum) / samples as f64", "sum as f64 / (1000.0 * samples as f64)"),
])

prepare("✏️editor/🎮️commands/📉️contours/🦀️.rs", [
    ("source.rows.iter().rev().flatten().copied().map(f64::from)", "source.rows_millimetres.iter().rev().flatten().map(|&sample| f64::from(sample) / 1000.0)"),
    ("    let width = source.rows.len();", "    let (width, height) = source.dimensions().ok_or_else(|| ContourFault::InvalidSource.fault())?;"),
    ("let row = (vertex / width) as f64 / (width - 1) as f64;", "let row = (vertex / width) as f64 / (height - 1) as f64;"),
    ("        ElevationSource { bounds, rows }", "        ElevationSource { bounds, rows_millimetres: rows.into_iter().map(|row: Vec<i32>| row.into_iter().map(|height| height * 1000).collect()).collect() }"),
    ("        let rows = (0..33).map", "        let rows: Vec<Vec<i32>> = (0..33).map"),
    ("    fn canonical(mut path: Vec<[f64; 2]>)", """    #[test]
    fn rectangular_ramp_preserves_fractional_contour_levels_and_latitude_extent() {
        let source = ElevationSource {
            bounds: [1.0, 1.0, 4.0, 3.0],
            rows_millimetres: vec![vec![0, 10_000, 20_000, 30_000]; 3],
        };
        let lines = contours(&source, 7.5, || false).unwrap();
        assert_eq!(lines.len(), 3);
        for (line, longitude) in lines.iter().zip([1.75, 2.5, 3.25]) {
            let coordinates = points(line);
            assert!(coordinates.iter().all(|point| (point[0] - longitude).abs() < 1e-12));
            assert!(coordinates.iter().any(|point| (point[1] - 1.0).abs() < 1e-12));
            assert!(coordinates.iter().any(|point| (point[1] - 3.0).abs() < 1e-12));
        }
    }

    fn canonical(mut path: Vec<[f64; 2]>)"""),
])

output = TICKET / "terrain-rectangular-producer.patch"
output.write_text("".join(updates), encoding="utf-8")
print("[DEBUG] Prepared " + output.name)
