"""🖌️ Prepare millimetre brush settings and matching World3d action limits."""

from difflib import unified_diff
import json
from pathlib import Path
import sys


ticket = Path(__file__).resolve().parent
terrain = Path(sys.argv[1]) / "✏️editor"
config = terrain / "🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config"
surface = Path("🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🖌️brush")
browser = Path("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗺️WorldTerrainLayer/🖌️brush")
world = Path("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world")
diff = []


def replace(path, changes):
    before = path.read_text(encoding="utf-8")
    after = before
    for old, new, count in changes:
        seen = after.count(old)
        if seen != count:
            raise ValueError(f"{path}: expected {count} matches for {old!r}, got {seen}")
        after = after.replace(old, new)
    diff.extend(unified_diff(before.splitlines(True), after.splitlines(True), f"a/{path}", f"b/{path}"))


replace(config / "🧬️schema/brush.json", [
    ("elevationStep", "elevationStepMillimetres", 2),
    ('{"type": "integer", "minimum": 1, "maximum": 32767}', '{"type": "integer", "minimum": 1, "maximum": 32767000, "description": "Height applied at the centre of the brush in millimetres."}', 1),
])
replace(config / "🧬️schema/🔗️.graphql", [
    ("elevationStep: Int!", "elevationStepMillimetres: Int!", 1),
])
replace(config / "🧬️schema/🛰️.proto", [
    ("elevation_step = 2;", "elevation_step_millimetres = 2;", 1),
])
replace(config / "🧬️schema/🦀️.rs", [
    ("pub elevation_step: u32,", "pub elevation_step_millimetres: u32,", 1),
])
replace(config / "🧬️schema/🟦️.ts", [
    ("elevation step in metres", "elevation step in millimetres", 1),
    ("elevationStep", "elevationStepMillimetres", 5),
    ("maximum: 32767", "maximum: 32767000", 1),
])
replace(config / "🖌️brush/🦀️.rs", [
    ("raise or lower step in meters", "raise or lower step in millimetres", 1),
    ("elevation_step", "elevation_step_millimetres", 5),
    ("elevation_step_millimetres: 25,", "elevation_step_millimetres: 25_000,", 1),
    ("impl TerrainBrushSettings {\n", """impl TerrainBrushSettings {
    /// 📏️ Largest configured height step, stored in exact millimetres.
    pub const MAX_ELEVATION_STEP_MILLIMETRES: u32 = 32_767_000;

""", 1),
    ("!(1..=32767).contains", "!(1..=Self::MAX_ELEVATION_STEP_MILLIMETRES).contains", 1),
    ('        let (operation, value) = match utility {', '        let step = f64::from(settings.elevation_step_millimetres) / 1000.0;\n        let (operation, value) = match utility {', 1),
    ('("raise-lower", settings.elevation_step_millimetres as i32)', '("raise-lower", step)', 1),
    ('("raise-lower", -(settings.elevation_step_millimetres as i32))', '("raise-lower", -step)', 1),
    ('("smooth", settings.smooth_strength as i32)', '("smooth", f64::from(settings.smooth_strength))', 1),
])
replace(terrain / "🎮️commands/👁️view/🦀️.rs", [
    ("    use semio_framework_value::DslValue;", '    use semio_framework_value::DslValue;\n    use semio_s_artifact_gis_gisterrain_inferences::elevation_edit::metres_to_millimetres;', 1),
    ("pub elevation_step: u32,", "pub elevation_step_millimetres: u32,", 1),
    ('            let payload = Self { radius: read("radius")?, elevation_step: read("elevationStep")?, smooth_strength: read("smoothStrength")? };', '''            let elevation_step_millimetres = args.get("elevationStep")
                .and_then(DslValue::as_f64)
                .and_then(metres_to_millimetres)
                .and_then(|value| u32::try_from(value).ok())
                .ok_or_else(invalid)?;
            let payload = Self { radius: read("radius")?, elevation_step_millimetres, smooth_strength: read("smoothStrength")? };''', 1),
    ("elevation_step: self.elevation_step,", "elevation_step_millimetres: self.elevation_step_millimetres,", 1),
    ('''        [
            ("radius", LocalizedLabel::native("Radius (samples)", "Radius (Rasterpunkte)"), 0, 32, defaults.radius),
            ("elevationStep", LocalizedLabel::native("Elevation Step (m)", "Höhenänderung (m)"), 1, 32767, defaults.elevation_step),
            ("smoothStrength", LocalizedLabel::native("Smoothing (%)", "Glättung (%)"), 0, 100, defaults.smooth_strength),
        ].into_iter().map(|(key, label, min, max, default)| {
            ActionArgDef::with_schema(key, label, ArgSchema::number(Some(f64::from(min)), Some(f64::from(max)), Some(1.0), true)).default_value(&default)
        }).collect()''', '''        let step = f64::from(defaults.elevation_step_millimetres) / 1000.0;
        let maximum_step = f64::from(TerrainBrushSettings::MAX_ELEVATION_STEP_MILLIMETRES) / 1000.0;
        vec![
            ActionArgDef::with_schema("radius", LocalizedLabel::native("Radius (samples)", "Radius (Rasterpunkte)"), ArgSchema::number(Some(0.0), Some(32.0), Some(1.0), true)).default_value(&defaults.radius),
            ActionArgDef::with_schema("elevationStep", LocalizedLabel::native("Elevation Step (m)", "Höhenänderung (m)"), ArgSchema::number(Some(0.001), Some(maximum_step), Some(0.001), false)).default_value(&step),
            ActionArgDef::with_schema("smoothStrength", LocalizedLabel::native("Smoothing (%)", "Glättung (%)"), ArgSchema::number(Some(0.0), Some(100.0), Some(1.0), true)).default_value(&defaults.smooth_strength),
        ]''', 1),
    ("Choose a whole-number radius, elevation step and smoothing strength within the shown ranges.", "Use a whole-number radius and smoothing percent. The elevation step supports millimetre precision.", 1),
    ("Wählen Sie ganzzahlige Werte für Radius, Höhenänderung und Glättungsstärke innerhalb der angezeigten Bereiche.", "Geben Sie Radius und Glättungsstärke als ganze Zahlen ein. Die Höhenänderung ist millimetergenau.", 1),
])
replace(config / "🧪️tests/🧫️fixtures/🔬️window/🔣️.json", [
    ('"elevationStep": 1,', '"elevationStepMillimetres": 1,', 5),
    ('"elevationStep": 0,', '"elevationStepMillimetres": 0,', 1),
    ('"elevationStep": 32767,', '"elevationStepMillimetres": 32767000,', 1),
    ('"elevationStep": 31,', '"elevationStep": 0.031,', 1),
    ('"radius": 7, "value": -31', '"radius": 7, "value": -0.031', 1),
])
replace(config / "🧪️tests/🔬️unit/🦀️.rs", [
    ("elevation_step: 31", "elevation_step_millimetres: 31", 2),
])
replace(terrain / "🧪️tests/🧩️mounted/🦀️.rs", [
    ("elevation_step: 31", "elevation_step_millimetres: 31", 1),
])
replace(surface / "🦀️.rs", [
    ("PartialEq, Eq, FromValue", "PartialEq, FromValue", 2),
    ("    pub value: i32,", "    pub value: f64,", 1),
    ('''        valid_id(&self.action) && valid_id(&self.arguments.operation)
            && self.arguments.radius < super::TERRAIN_GRID_RESOLUTION
            && i16::try_from(self.arguments.value).is_ok()''', '''        let value = self.arguments.value;
        let strength = if self.arguments.operation == "smooth" {
            (0.0..=100.0).contains(&value) && value.fract() == 0.0
        } else {
            (-32768.0..=32767.0).contains(&value) && (value * 1000.0).round() / 1000.0 == value
        };
        valid_id(&self.action) && valid_id(&self.arguments.operation)
            && self.arguments.radius < super::TERRAIN_GRID_RESOLUTION
            && strength''', 1),
])
replace(surface / "🔣️.json", [
    ('''"value": { "type": "integer", "minimum": -32768, "maximum": 32767 }
      }''', '''"value": { "type": "number", "minimum": -32768, "maximum": 32767, "multipleOf": 0.001 }
      },
      "if": { "properties": { "operation": { "const": "smooth" } } },
      "then": { "properties": { "value": { "type": "integer", "minimum": 0, "maximum": 100 } } }''', 1),
])
replace(surface / "🧪️tests/🔣️.json", [
    ('''"operation":"raise-lower","radius":3,"value":-12}},''', '''"operation":"raise-lower","radius":3,"value":-12}},
    {"action":"editElevation","arguments":{"operation":"raise-lower","radius":7,"value":-0.031}},
    {"action":"editElevation","arguments":{"operation":"raise-lower","radius":4,"value":32766.999}},''', 1),
    ('''"operation":"smooth","radius":4,"value":4.1}},''', '''"operation":"smooth","radius":4,"value":4.1}},
    {"action":"editElevation","arguments":{"operation":"raise-lower","radius":4,"value":0.0005}},''', 1),
])
replace(surface / "🧪️tests/🟦️.ts", [
    ("new Ajv().compile(schema)", "new Ajv({ multipleOfPrecision: 7 }).compile(schema)", 1),
])
replace(browser / "🟦️.ts", [
    ('''  if (!identifier(operation) || !integer(radius, 0, 32) || !integer(strength, -32768, 32767)) return undefined;''', '''  if (!identifier(operation) || !integer(radius, 0, 32) || typeof strength !== "number") return undefined;
  const valid = operation === "smooth"
    ? integer(strength, 0, 100)
    : strength >= -32768 && strength <= 32767 && Math.round(strength * 1000) / 1000 === strength;
  if (!valid) return undefined;''', 1),
])
replace(world / "🧵️interaction/📜️plan-and-publish/🦀️.rs", [
    ("f64::from(brush.arguments.value)", "brush.arguments.value", 1),
])
replace(world / "🎛️host/🧪️tests/🔭️unit-2/🥽️brush-environment-and-hover-laws/🦀️.rs", [
    ('"operation":"smooth","radius":4,"value":50', '"operation":"raise-lower","radius":4,"value":-0.031', 1),
    ('("editElevation", "smooth", 4, 50)', '("editElevation", "raise-lower", 4, -0.031)', 1),
])

(ticket / "terrain-rectangular-brush-precision.patch").write_text("".join(diff), encoding="utf-8")
