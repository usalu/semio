"""🧱 Prepare authored rectangular grid creation and grading."""

from difflib import unified_diff
from pathlib import Path


TICKET = Path(__file__).resolve().parent
ARTIFACT = Path("✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any")
updates = []


def replace(relative, replacements):
    """🏗️ Stage one precise source rewrite against the live owner."""
    path = ARTIFACT / relative
    before = path.read_text(encoding="utf-8")
    after = before
    for old, new in replacements:
        if after.count(old) != 1:
            raise RuntimeError(f"{path}: expected one {old!r}")
        after = after.replace(old, new)
    updates.extend(unified_diff(before.splitlines(keepends=True), after.splitlines(keepends=True), "a/" + path.as_posix(), "b/" + path.as_posix()))


units = "use semio_s_artifact_gis_gisterrain_inferences::elevation_edit::metres_to_millimetres;"
fixed_height_editor = "ArgSchema::number(Some(f64::from(i16::MIN)), Some(f64::from(i16::MAX)), Some(1.0), true)).default_value(&100_i16)"
fractional_height_editor = "ArgSchema::number(Some(f64::from(i32::MIN) / 1000.0), Some(f64::from(i32::MAX) / 1000.0), Some(0.001), false)).default_value(&100.0_f64)"

create = "✏️editor/🎮️commands/🏔️elevation-source/➕️create-grid"
replace(create + "/🦀️.rs", [
    ("use crate::schema::snapshot::ElevationSource;", "use crate::schema::snapshot::{ELEVATION_MAX_SIDE, ELEVATION_MIN_SIDE, ElevationSource};\n" + units),
    ("const GRID_SIZE: usize = 33;", "const DEFAULT_GRID_SIZE: u32 = 33;"),
    ("    pub height: i16,", "    pub columns: u32,\n    pub rows: u32,\n    pub height_millimetres: i32,"),
    ("""        let height = number("height")?;
        if height.fract() != 0.0 || !(f64::from(i16::MIN)..=f64::from(i16::MAX)).contains(&height) {
            return Err(Fault::from("gis-terrain-create-grid-invalid-height"));
        }
        let command = Self { west: number("west")?, south: number("south")?, east: number("east")?, north: number("north")?, height: height as i16 };""",
     """        let dimension = |key| {
            let value = number(key)?;
            if value.fract() != 0.0 || !(ELEVATION_MIN_SIDE as f64..=ELEVATION_MAX_SIDE as f64).contains(&value) {
                return Err(Fault::from(format!("gis-terrain-create-grid-invalid-{key}")));
            }
            Ok(value as u32)
        };
        let height_millimetres = metres_to_millimetres(number("height")?).ok_or_else(|| Fault::from("gis-terrain-create-grid-invalid-height"))?;
        let command = Self {
            west: number("west")?, south: number("south")?, east: number("east")?, north: number("north")?,
            columns: dimension("columns")?, rows: dimension("rows")?, height_millimetres,
        };"""),
    ("            && (-90.0..self.north).contains(&self.south) && self.north <= 90.0",
     """            && (-90.0..self.north).contains(&self.south) && self.north <= 90.0
            && (ELEVATION_MIN_SIDE..=ELEVATION_MAX_SIDE).contains(&(self.columns as usize))
            && (ELEVATION_MIN_SIDE..=ELEVATION_MAX_SIDE).contains(&(self.rows as usize))"""),
    ("            rows: vec![vec![self.height; GRID_SIZE]; GRID_SIZE],", "            rows_millimetres: vec![vec![self.height_millimetres; self.columns as usize]; self.rows as usize],"),
    ("/// 🧭️ Stages the four geographic bounds and the initial height.", "/// 🧭️ Stages geographic bounds, rectangular dimensions and the initial height."),
    ("    vec![\n        bound", "    let dimension = |id, en, de| ActionArgDef::with_schema(id, LocalizedLabel::native(en, de), ArgSchema::number(Some(ELEVATION_MIN_SIDE as f64), Some(ELEVATION_MAX_SIDE as f64), Some(1.0), true)).default_value(&DEFAULT_GRID_SIZE);\n    vec![\n        bound"),
    ("        ActionArgDef::with_schema(\"height\",", "        dimension(\"columns\", \"Columns\", \"Spalten\"),\n        dimension(\"rows\", \"Rows\", \"Zeilen\"),\n        ActionArgDef::with_schema(\"height\","),
    (fixed_height_editor, fractional_height_editor),
    ("grid.rows.len()", "grid.rows_millimetres.len()"),
    ("grid.rows.iter()", "grid.rows_millimetres.iter()"),
    ('expected["height"].as_i64()', 'expected["heightMillimetres"].as_i64()'),
])
replace(create + "/🧪️tests/🔣️.json", [
    ('"north": 49.9, "height": -30}', '"north": 49.9, "columns": 4, "rows": 3, "height": -30.125}'),
    ('"rows": 33, "columns": 33, "height": -30}', '"rows": 3, "columns": 4, "heightMillimetres": -30125}'),
    ('"north": 49.9, "height": 10}', '"north": 49.9, "columns": 4, "rows": 3, "height": 10}'),
    ('"north": 49.9, "height": 5.4}', '"north": 49.9, "columns": 4, "rows": 3, "height": 5.4001}'),
])

grade = "✏️editor/🎮️commands/📏️grade-region"
replace(grade + "/🦀️.rs", [
    ("use crate::schema::snapshot::ElevationSource;", "use crate::schema::snapshot::ElevationSource;\n" + units),
    ("    pub height: i16,", "    pub height_millimetres: i32,"),
    ("""        let height = args.get("height").and_then(DslValue::as_f64)
            .filter(|height| height.fract() == 0.0 && (f64::from(i16::MIN)..=f64::from(i16::MAX)).contains(height))
            .ok_or_else(|| Fault::from("gis-terrain-grading-height-invalid"))?;
        Ok(Self { id: id.into(), height: height as i16 })""",
     """        let height_millimetres = args.get("height").and_then(DslValue::as_f64).and_then(metres_to_millimetres)
            .ok_or_else(|| Fault::from("gis-terrain-grading-height-invalid"))?;
        Ok(Self { id: id.into(), height_millimetres })"""),
    (fixed_height_editor, fractional_height_editor),
    ("region: &TerrainRegionData, height: i16", "region: &TerrainRegionData, height_millimetres: i32"),
    ("    let size = (source.rows.len() - 1) as f64;", "    let x_segments = (source.rows_millimetres[0].len() - 1) as f64;\n    let y_segments = (source.rows_millimetres.len() - 1) as f64;"),
    ("result.rows.iter_mut()", "result.rows_millimetres.iter_mut()"),
    ("row_index as f64 / size", "row_index as f64 / y_segments"),
    ("column as f64 / size", "column as f64 / x_segments"),
    ("                *sample = height;", "                *sample = height_millimetres;"),
    ("apply(source, region, payload.height)", "apply(source, region, payload.height_millimetres)"),
    ('fixture["initialHeight"].as_i64().unwrap() as i16', 'fixture["initialHeightMillimetres"].as_i64().unwrap() as i32'),
    ("            rows: vec![vec![initial_height; 33]; 33],", """            rows_millimetres: vec![vec![initial_height; fixture["columns"].as_u64().unwrap() as usize]; fixture["rows"].as_u64().unwrap() as usize],"""),
    ('fixture["command"]["height"].as_i64().unwrap() as i16', 'metres_to_millimetres(fixture["command"]["height"].as_f64().unwrap()).unwrap()'),
    ("result.rows.iter().flatten()", "result.rows_millimetres.iter().flatten()"),
    ("i32::from(result.rows[row as usize][column as usize])", "result.rows_millimetres[row as usize][column as usize]"),
    ("source.rows[4][4]", "source.rows_millimetres[2][4]"),
    ('id: "g_sart_tilman".into(), height: 300', 'id: "g_sart_tilman".into(), height_millimetres: 300_000'),
])
replace(grade + "/🧪️tests/🔣️.json", [
    ('  "initialHeight":10,', '  "columns":33,\n  "rows":17,\n  "initialHeightMillimetres":10000,'),
    ('"height":-45', '"height":-45.375'),
    ('  "graded":16,', '  "graded":12,'),
    ('[[3,4,10],[4,4,-45],[8,8,-45],[5,5,10],[6,6,10],[8,7,-45],[9,6,10]]',
     '[[3,2,10000],[4,2,-45375],[8,4,-45375],[5,3,10000],[6,3,10000],[8,3,-45375],[9,3,10000]]'),
])

replace("✏️editor/🧪️tests/🧩️mounted/🦀️.rs", [
    ("east: 5.61, north: 50.64, height: -30", "east: 5.61, north: 50.64, columns: 4, rows: 3, height_millimetres: -30_125"),
    ('id: "region".into(), height: 100', 'id: "region".into(), height_millimetres: 100_000'),
])

output = TICKET / "terrain-rectangular-create-and-grade.patch"
output.write_text("".join(updates), encoding="utf-8")
print("[DEBUG] Prepared " + output.name)
