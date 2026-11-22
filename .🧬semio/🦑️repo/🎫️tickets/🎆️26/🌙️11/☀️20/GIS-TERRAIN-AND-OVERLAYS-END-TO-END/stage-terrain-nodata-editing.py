from difflib import unified_diff
from pathlib import Path
import sys

ticket = Path(__file__).resolve().parent
root = Path(sys.argv[1])
result = []

def stage(relative, replacements):
    path = root / relative
    before = path.read_text()
    after = before
    for old, new in replacements:
        if after.count(old) != 1:
            raise ValueError((str(path), old))
        after = after.replace(old, new)
    result.extend(unified_diff(before.splitlines(keepends=True), after.splitlines(keepends=True),
                               fromfile=f"a/{path}", tofile=f"b/{path}"))

stage("🧬️schema/💡️inferences/📐️elevation-edit/🦀️.rs", [
    ("use semio_framework_value::{ValueError, ValueRefusalKind};",
     "use semio_framework_surface::terrain::missing_samples::is_missing;\nuse semio_framework_value::{ValueError, ValueRefusalKind};"),
    ("        let mut next = source.clone();\n        edit.write(source, &mut next)?;",
     """        if edit.mode == ElevationEditMode::SetSample
            && is_missing(source.missing_samples.as_deref(), edit.row as usize * width + edit.column as usize)
        {
            return Err(invalid("gis-terrain-elevation-sample-is-missing"));
        }
        let mut next = source.clone();
        edit.write(source, &mut next)?;"""),
    ("        let value = i64::from(edit.value);\n        for (y, row) in next.rows_millimetres.iter_mut().enumerate() {\n            for (x, height) in row.iter_mut().enumerate() {",
     """        let value = i64::from(edit.value);
        let mask = source.missing_samples.as_deref();
        for (y, row) in next.rows_millimetres.iter_mut().enumerate() {
            let row_start = y * row.len();
            for (x, height) in row.iter_mut().enumerate() {
                if is_missing(mask, row_start + x) {
                    continue;
                }"""),
    ("smooth_delta(&source.rows_millimetres, x, y, current, value * weight, limit)",
     "smooth_delta(&source.rows_millimetres, mask, x, y, current, value * weight, limit)"),
    ("""fn smooth_delta(rows: &[Vec<i32>], x: usize, y: usize, current: i64, weight: i64, limit: i64) -> i64 {
    let mut sum = 0;
    let mut count = 0;
    for row in &rows[y.saturating_sub(1)..=(y + 1).min(rows.len() - 1)] {
        for &height in &row[x.saturating_sub(1)..=(x + 1).min(row.len() - 1)] {
            sum += i64::from(height);
            count += 1;
        }
    }""",
     """fn smooth_delta(rows: &[Vec<i32>], mask: Option<&str>, x: usize, y: usize, current: i64, weight: i64, limit: i64) -> i64 {
    let mut sum = 0;
    let mut count = 0;
    for neighbor_y in y.saturating_sub(1)..=(y + 1).min(rows.len() - 1) {
        let row = &rows[neighbor_y];
        for neighbor_x in x.saturating_sub(1)..=(x + 1).min(row.len() - 1) {
            if !is_missing(mask, neighbor_y * row.len() + neighbor_x) {
                sum += i64::from(row[neighbor_x]);
                count += 1;
            }
        }
    }"""),
])

stage("✏️editor/🎮️commands/📏️grade-region/🦀️.rs", [
    ("    let x_segments = (source.rows_millimetres[0].len() - 1) as f64;",
     """    let width = source.rows_millimetres[0].len();
    let mask = source.missing_samples.as_deref();
    let x_segments = (width - 1) as f64;"""),
    ("        for (column, sample) in row.iter_mut().enumerate() {\n            let point",
     """        for (column, sample) in row.iter_mut().enumerate() {
            if semio_framework_surface::terrain::missing_samples::is_missing(mask, row_index * width + column) {
                continue;
            }
            let point"""),
])

stage("✏️editor/🎮️commands/📉️contours/🦀️.rs", [
    ("use crate::schema::imported_map::{ImportedMap, terrain_grid_origin};",
     "use crate::schema::imported_map::ImportedMap;"),
    ("use semio_framework_value::{DslValue, ToValue};",
     "use semio_framework_surface::terrain::TerrainGridTopology;\nuse semio_framework_value::{DslValue, ToValue};"),
    ("""    let grid = source.grid();
    let origin = terrain_grid_origin(grid.bounds);
    let mesh = grid.mesh(origin.lon, origin.lat, 1.0).ok_or_else(|| ContourFault::InvalidSource.fault())?;
    let heights = source.rows_millimetres.iter().rev().flatten().map(|&sample| f64::from(sample) / 1000.0).collect::<Vec<_>>();
    let (minimum, maximum) = heights.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), &height| (low.min(height), high.max(height)));""",
     """    let (width, height) = source.dimensions().ok_or_else(|| ContourFault::InvalidSource.fault())?;
    let topology = TerrainGridTopology::new([width as u32, height as u32], source.missing_samples.as_deref());
    let heights = source.rows_millimetres.iter().rev().flatten().map(|&sample| f64::from(sample) / 1000.0).collect::<Vec<_>>();
    let mut surface_heights = topology.faces().flat_map(|face| face.vertices.map(|vertex| heights[vertex as usize]));
    let Some(first_height) = surface_heights.next() else { return Ok(Vec::new()); };
    let (minimum, maximum) = surface_heights.fold((first_height, first_height), |(low, high), height| (low.min(height), high.max(height)));"""),
    ("    let (width, height) = source.dimensions().ok_or_else(|| ContourFault::InvalidSource.fault())?;\n    let [west",
     "    let [west"),
    ("""        for triangle in mesh.indices.chunks_exact(3) {
            let vertices = [triangle[0] as usize, triangle[1] as usize, triangle[2] as usize];
            if let Some(segment) = contour_segment(vertices, level, &heights) {""",
     """        for face in topology.faces() {
            if let Some(segment) = contour_segment(face.vertices.map(|vertex| vertex as usize), level, &heights) {"""),
])

stage("✏️editor/📌️panels/🔍️terrain-data/🦀️.rs", [
    ("use crate::GisTerrainSnapshot;",
     "use crate::GisTerrainSnapshot;\nuse semio_framework_surface::terrain::missing_samples::is_missing;"),
    ("""        let (minimum, maximum, sum) = grid.rows_millimetres.iter().flatten().copied().fold((i32::MAX, i32::MIN, 0_i64), |(low, high, sum), height| {
            (low.min(height), high.max(height), sum + i64::from(height))
        });
        let dimensions = (grid.rows_millimetres[0].len(), grid.rows_millimetres.len());
        let minimum = f64::from(minimum) / 1000.0;
        let maximum = f64::from(maximum) / 1000.0;
        let samples = dimensions.0 * dimensions.1;
        let items = ui_node_list([
            item("grid-size", "Grid Size", "Rastergröße", format!("{} × {}", dimensions.0, dimensions.1)),
            item("grid-range", "Elevation Range", "Höhenbereich", format!("{minimum}–{maximum} m")),
            item("grid-mean", "Mean Elevation", "Mittlere Höhe", format!("{:.1} m", sum as f64 / (1000.0 * samples as f64))),
            item("grid-longitude", "Longitude Bounds", "Längengrade", format!("{west:.6}° → {east:.6}°")),
            item("grid-latitude", "Latitude Bounds", "Breitengrade", format!("{south:.6}° → {north:.6}°")),
        ])?;""",
     """        let mask = grid.missing_samples.as_deref();
        let (minimum, maximum, sum, present) = grid.rows_millimetres.iter().flatten().copied().enumerate()
            .filter(|(ordinal, _)| !is_missing(mask, *ordinal))
            .fold((i32::MAX, i32::MIN, 0_i64, 0_usize), |(low, high, sum, present), (_, height)| {
                (low.min(height), high.max(height), sum + i64::from(height), present + 1)
            });
        let dimensions = (grid.rows_millimetres[0].len(), grid.rows_millimetres.len());
        let absent = dimensions.0 * dimensions.1 - present;
        let mut items = Vec::with_capacity(6);
        items.push(item("grid-size", "Grid Size", "Rastergröße", format!("{} × {}", dimensions.0, dimensions.1)));
        if present > 0 {
            let minimum = f64::from(minimum) / 1000.0;
            let maximum = f64::from(maximum) / 1000.0;
            let mean = sum as f64 / (1000.0 * present as f64);
            items.push(item("grid-range", "Elevation Range", "Höhenbereich", format!("{minimum}–{maximum} m")));
            items.push(item("grid-mean", "Mean Elevation", "Mittlere Höhe", format!("{mean:.3} m")));
        }
        if absent > 0 {
            items.push(item("grid-missing-count", "Missing Samples", "Fehlende Rasterpunkte", absent.to_string()));
        }
        items.extend([
            item("grid-longitude", "Longitude Bounds", "Längengrade", format!("{west:.6}° → {east:.6}°")),
            item("grid-latitude", "Latitude Bounds", "Breitengrade", format!("{south:.6}° → {north:.6}°")),
        ]);
        let items = ui_node_list(items)?;"""),
])

(ticket / "terrain-nodata-editing.patch").write_text("".join(result))
