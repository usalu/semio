"""📐 Prepare precise raster-edit arithmetic and its independent fixture values."""

from difflib import unified_diff
from fractions import Fraction
import json
from pathlib import Path


TICKET = Path(__file__).resolve().parent
ARTIFACT = Path("✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any")
changes = []
proposed_sources = {}


def edit(relative, replacements):
    """🧬 Stage one edit against its current owning source."""
    edit_path(ARTIFACT / relative, replacements)


def edit_path(path, replacements):
    """📂 Stage exact tracked-source replacements for a workspace-relative path."""
    before = path.read_text(encoding="utf-8")
    after = before
    for old, new in replacements:
        if old not in after:
            raise RuntimeError(f"{path}: missing {old!r}")
        after = after.replace(old, new, 1)
    proposed_sources[path.as_posix()] = after
    changes.extend(unified_diff(before.splitlines(keepends=True), after.splitlines(keepends=True), "a/" + path.as_posix(), "b/" + path.as_posix()))


inference = "🧬️schema/💡️inferences/📐️elevation-edit/🦀️.rs"
editor = "✏️editor/🎮️commands/📐️edit-elevation/🦀️.rs"
edit(inference, [
    ("use crate::schema::snapshot::ElevationSource;", "use crate::schema::snapshot::{ELEVATION_MAX_SIDE, ElevationSource};"),
    ("pub const LAST_SAMPLE: u32 = 32;", """/// 📏️ Converts an authored metre value when its binary representation identifies an exact millimetre.
pub fn metres_to_millimetres(metres: f64) -> Option<i32> {
    let scaled = (metres * 1000.0).round();
    if !(f64::from(i32::MIN)..=f64::from(i32::MAX)).contains(&scaled) {
        return None;
    }
    let millimetres = scaled as i32;
    (f64::from(millimetres) / 1000.0 == metres).then_some(millimetres)
}

pub const LAST_SAMPLE: u32 = ELEVATION_MAX_SIDE as u32 - 1;"""),
    ("pub const MIN_ELEVATION: i32 = i16::MIN as i32;\npub const MAX_ELEVATION: i32 = i16::MAX as i32;", "pub const MIN_ELEVATION: i32 = i32::MIN;\npub const MAX_ELEVATION: i32 = i32::MAX;"),
    ("if [self.row, self.column, self.radius].into_iter().any(|index| index > LAST_SAMPLE)\n            || !(MIN_ELEVATION..=MAX_ELEVATION).contains(&self.value) {", "if [self.row, self.column, self.radius].into_iter().any(|index| index > LAST_SAMPLE) {"),
    ("    /// 🛰️ Applies one edit in raw metres", "    /// 🛰️ Applies one edit in signed millimetres"),
    ("        let edit = self.validate()?;\n        let mut next = source.clone();", """        let edit = self.validate()?;
        let (width, height) = source.dimensions().ok_or_else(|| invalid("gis-terrain-elevation-edit-out-of-range"))?;
        if edit.mode != ElevationEditMode::Offset && (edit.column as usize >= width || edit.row as usize >= height) {
            return Err(invalid("gis-terrain-elevation-edit-out-of-range"));
        }
        let mut next = source.clone();"""),
    ("        let stamps = self.stroke_stamps(samples)?;", """        self.validate_stroke_source(source, samples)?;
        let stamps = self.stroke_stamps(samples)?;"""),
    ("""    /// 🧮️ Charges each visited sample and up to nine smoothing neighbors.
    pub fn stroke_work(self, samples: &[[u32; 2]]) -> Result<u64, ValueError> {
        Ok(self.validate_stroke(samples)? as u64 * STAMP_WORK)
    }""",
     """    fn validate_stroke_source(self, source: &ElevationSource, samples: &[[u32; 2]]) -> Result<(usize, usize), ValueError> {
        let (width, height) = source.dimensions().ok_or_else(|| invalid("gis-terrain-elevation-stroke-invalid"))?;
        if samples.iter().any(|[column, row]| *column as usize >= width || *row as usize >= height) {
            return Err(invalid("gis-terrain-elevation-stroke-invalid"));
        }
        Ok((width, height))
    }

    /// 🧮️ Quotes one visit per grid sample plus the maximum smoothing neighbors within each brush.
    pub fn stroke_work(self, source: &ElevationSource, samples: &[[u32; 2]]) -> Result<u64, ValueError> {
        let count = self.validate_stroke(samples)? as u64;
        let (width, height) = self.validate_stroke_source(source, samples)?;
        let cells = (width * height) as u64;
        let visits = if self.mode == ElevationEditMode::Smooth {
            let span = u64::from(self.radius) * 2 + 1;
            cells + 9 * cells.min(span * span)
        } else {
            cells
        };
        Ok(count * visits)
    }"""),
    ("let limit = ((edit.radius + 1) * (edit.radius + 1)) as i32;", "let limit = i64::from((edit.radius + 1) * (edit.radius + 1));\n        let value = i64::from(edit.value);"),
    ("next.rows.iter_mut()", "next.rows_millimetres.iter_mut()"),
    ("let dx = x as i32 - edit.column as i32;", "let dx = x as i64 - i64::from(edit.column);"),
    ("let dy = y as i32 - edit.row as i32;", "let dy = y as i64 - i64::from(edit.row);"),
    ("let current = i32::from(source.rows[y][x]);", "let current = i64::from(source.rows_millimetres[y][x]);"),
    ("SetSample if distance == 0 => edit.value,", "SetSample if distance == 0 => value,"),
    ("Offset => current + edit.value,", "Offset => current + value,"),
    ("rounded_ratio(edit.value * weight, limit)", "rounded_ratio(value * weight, limit)"),
    ("rounded_ratio((edit.value - current) * weight, limit)", "rounded_ratio((value - current) * weight, limit)"),
    ("smooth_delta(&source.rows, x, y, current, edit.value * weight, limit)", "smooth_delta(&source.rows_millimetres, x, y, current, value * weight, limit)"),
    ("*height = i16::try_from(updated)", "*height = i32::try_from(updated)"),
    ("fn rounded_ratio(numerator: i32, denominator: i32) -> i32", "fn rounded_ratio(numerator: i64, denominator: i64) -> i64"),
    ("fn smooth_delta(rows: &[Vec<i16>], x: usize, y: usize, current: i32, weight: i32, limit: i32) -> i32", "fn smooth_delta(rows: &[Vec<i32>], x: usize, y: usize, current: i64, weight: i64, limit: i64) -> i64"),
    ("sum += i32::from(height);", "sum += i64::from(height);"),
    ("""    let numerator = i64::from(sum - count * current) * i64::from(weight);
    let denominator = i64::from(count * limit * 100);
    ((numerator.abs() + denominator / 2) / denominator * numerator.signum()) as i32""",
     """    rounded_ratio((sum - count * current) * weight, count * limit * 100)"""),
])

edit(editor, [
    ("MAX_STROKE_SAMPLES, MIN_ELEVATION};", "MAX_STROKE_SAMPLES, MIN_ELEVATION, metres_to_millimetres};"),
    ("    pub value: i32,", "    /// 📐️ Carries canonical millimetres for heights and an integer percent for smoothing.\n    pub value: i32,"),
    ("""        let command = Self {
            operation: args.get("operation").and_then(DslValue::as_str).ok_or_else(|| Fault::from("gis-terrain-elevation-operation-required"))?.into(),""",
     """        let operation = args.get("operation").and_then(DslValue::as_str).ok_or_else(|| Fault::from("gis-terrain-elevation-operation-required"))?;
        let value = if operation == "smooth" {
            integer("value")?
        } else {
            args.get("value").and_then(DslValue::as_f64).and_then(metres_to_millimetres)
                .ok_or_else(|| Fault::from("gis-terrain-elevation-edit-invalid-value"))?
        };
        let command = Self {
            operation: operation.into(),"""),
    ('            value: integer("value")?,', '            value,'),
    ("""        numbers("value", "Meters or Smoothing Percent", "Meter oder Glättungsprozent", f64::from(MIN_ELEVATION), f64::from(MAX_ELEVATION), 25),""",
     """        ActionArgDef::with_schema("value", LocalizedLabel::native("Meters or Smoothing Percent", "Meter oder Glättungsprozent"),
            ArgSchema::number(Some(f64::from(MIN_ELEVATION) / 1000.0), Some(f64::from(MAX_ELEVATION) / 1000.0), Some(0.001), false)).default_value(&25.0_f64),"""),
    ("edit.stroke_work(samples)", "edit.stroke_work(source, samples)"),
    ('fixture["height"].as_i64().unwrap() as i16', 'fixture["height"].as_i64().unwrap() as i32 * 1000'),
    ("            rows: vec![vec![height; size]; size],", "            rows_millimetres: vec![vec![height; size]; size],"),
    ("source.rows[row as usize][column as usize] = height as i16;", "source.rows_millimetres[row as usize][column as usize] = height * 1000;"),
    ("""let decode = |value: &Value| semio_framework_pack_json::from_json_str::<EditElevation>(
            &value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject,
        ).unwrap();""",
     """let decode = |value: &Value| {
            let args = semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            EditElevation::from_action(&semio_framework_pack_json::to_dsl_value(&args))
        };"""),
    ("&decode(&case[\"command\"])","&decode(&case[\"command\"]).unwrap()"),
    ("apply(&source, &decode(command)).is_err()", "decode(command).and_then(|command| apply(&source, &command)).is_err()"),
    ("assert_eq!(source.rows[16][16], 190);", "assert_eq!(source.rows_millimetres[16][16], 190_000);"),
    ("assert_eq!(source.rows[16][16], 190);", "assert_eq!(source.rows_millimetres[16][16], 190_000);"),
    ("edit.stroke_work(&entire_budget).unwrap(), semio_s_artifact_gis_gisterrain_inferences::elevation_edit::MAX_STROKE_WORK - 1", "edit.stroke_work(&source, &entire_budget).unwrap(), 64 * (33 * 33 + 9 * 9)"),
    ("source.rows[row as usize][column as usize] = height as i16;", "source.rows_millimetres[row as usize][column as usize] = height;"),
    ('apply(&source, &decode(&overflow["command"])).is_err()', 'apply(&source, &decode(&overflow["command"]).unwrap()).is_err()'),
    ("edited.rows.iter().flatten().zip(source.rows.iter().flatten())", "edited.rows_millimetres.iter().flatten().zip(source.rows_millimetres.iter().flatten())"),
    ("edited.rows.iter().flatten().zip(source.rows.iter().flatten())", "edited.rows_millimetres.iter().flatten().zip(source.rows_millimetres.iter().flatten())"),
    ("i32::from(edited.rows[row as usize][column as usize])", "edited.rows_millimetres[row as usize][column as usize]"),
    ("i32::from(edited.rows[row as usize][column as usize])", "edited.rows_millimetres[row as usize][column as usize]"),
    ('assert!(EditElevation::from_action(&semio_framework_pack_json::to_dsl_value(&parsed)).is_err(), "{command}");',
     'assert!(EditElevation::from_action(&semio_framework_pack_json::to_dsl_value(&parsed)).and_then(|command| apply(&source, &command)).is_err(), "{command}");'),
    ("[\"elevationGrid\"][\"rows\"][16][16]", "[\"elevationGrid\"][\"rowsMillimetres\"][16][16]"),
    ("source.rows[16][16] + 25", "source.rows_millimetres[16][16] + 25_000"),
    ("updated.rows[16][16]", "updated.rows_millimetres[16][16]"),
    ("updated.rows[16][16]", "updated.rows_millimetres[16][16]"),
    ("updated.rows[0][0], source.rows[0][0]", "updated.rows_millimetres[0][0], source.rows_millimetres[0][0]"),
    ("brushed.elevation_source.as_ref().unwrap().rows[8][column], edited.elevation_source.as_ref().unwrap().rows[8][column] + 7", "brushed.elevation_source.as_ref().unwrap().rows_millimetres[8][column], edited.elevation_source.as_ref().unwrap().rows_millimetres[8][column] + 7_000"),
])


def millimetres(value):
    """📏 Interpret the fixture's authored decimal metres without a floating-point conversion."""
    exact = 1000 * Fraction(str(value))
    assert exact.denominator == 1
    return exact.numerator


def evaluate(grid, command, centres):
    """🧮 Recompute each fixture through exact rational arithmetic."""
    diameter = (command["radius"] + 1) ** 2
    mode = command["operation"]
    value = command["value"] if mode == "smooth" else millimetres(command["value"])
    for column, row in centres:
        updated = []
        for y, old_row in enumerate(grid):
            next_row = []
            for x, old in enumerate(old_row):
                weight = max(0, 1 - Fraction((column - x) ** 2 + (row - y) ** 2, diameter))
                if mode == "set-sample":
                    delta = value - old if (x, y) == (column, row) else 0
                elif mode == "offset":
                    delta = value
                elif mode == "flatten":
                    delta = (value - old) * weight
                elif mode == "raise-lower":
                    delta = value * weight
                elif mode == "smooth":
                    neighborhood = [grid[j][i] for j in range(max(0, y - 1), min(len(grid), y + 2))
                                    for i in range(max(0, x - 1), min(len(old_row), x + 2))]
                    delta = (Fraction(sum(neighborhood), len(neighborhood)) - old) * Fraction(value, 100) * weight
                else:
                    raise ValueError(mode)
                change = int(delta + (Fraction(-1, 2) if delta < 0 else Fraction(1, 2)))
                next_row.append(old + change)
            updated.append(next_row)
        grid = updated
    return grid


fixture_path = ARTIFACT / "✏️editor/🎮️commands/📐️edit-elevation/🧪️tests/🔣️.json"
original = fixture_path.read_text(encoding="utf-8")
fixture = json.loads(original)
grid = [[millimetres(fixture["height"])] * fixture["size"] for _ in range(fixture["size"])]
for column, row, height in fixture["seed"]:
    grid[row][column] = millimetres(height)
fixture["cases"][0]["command"]["value"] = 201.125
for case in fixture["cases"] + fixture["strokeEdits"]:
    command = case["command"]
    centres = command.get("stroke", [[command["column"], command["row"]]])
    stamps = [centres[0]]
    spacing = max(1, (command["radius"] + 1) // 2)
    for start, end in zip(centres, centres[1:]):
        distance = max(abs(start[0] - end[0]), abs(start[1] - end[1]))
        steps = (distance + spacing - 1) // spacing
        for step in range(1, steps + 1):
            stamps.append([(a * (steps - step) + b * step + steps // 2) // steps for a, b in zip(start, end)])
    expected = evaluate(grid, command, stamps)
    case["changed"] = sum(before != after for previous, next_row in zip(grid, expected) for before, after in zip(previous, next_row))
    case["samples"] = [[x, y, expected[y][x]] for x, y, _ in case["samples"]]
fixture["refused"][0]["value"] = 2147483.647
outside_grid = json.loads(json.dumps(fixture["refusedStrokes"][2]))
fixture["refusedStrokes"][2]["stroke"][1][0] = 65
fixture["refusedStrokes"].append(outside_grid)
fixture["overflowStroke"][0]["seed"][2] = 2147483647
changes.extend(unified_diff(original.splitlines(keepends=True),
                            (json.dumps(fixture, ensure_ascii=False, indent=2) + "\n").splitlines(keepends=True),
                            "a/" + fixture_path.as_posix(), "b/" + fixture_path.as_posix()))

edit_path(Path("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗺️WorldTerrainLayer/🖌️brush/🧪️tests/🟦️.test.ts"), [
    ("minimum: 0, maximum: 32", "minimum: 0, maximum: 64"),
    ("coordinates.map(() => fixture.height)", "coordinates.map(() => fixture.height * 1000)"),
    ("original[y]![x] = height;", "original[y]![x] = height * 1000;"),
    ("let delta = value * normalized;", "let delta = value * 1000 * normalized;"),
    ("overflow.seed[2]! + overflow.command.value).toBeGreaterThan(32767)", "overflow.seed[2]! + overflow.command.value * 1000).toBeGreaterThan(2147483647)"),
])

output = TICKET / "terrain-rectangular-edit-reducer.patch"
output.write_text("".join(changes), encoding="utf-8")
print("[DEBUG] Prepared " + output.name)
