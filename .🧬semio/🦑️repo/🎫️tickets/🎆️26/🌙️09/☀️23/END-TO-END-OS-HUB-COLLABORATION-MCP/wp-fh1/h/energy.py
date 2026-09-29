"""🔋️ FH1 family H — energy: literal codes per refusal + parameters, declarations on the editor and the viewer."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply, regex
from texts import declare
A = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/"
R = A + "🏅️standards/🔖️1/🪆️subsets/✳️any/"
E = "✏️editor/🦀️.rs"
V = "👁️viewer/🦀️.rs"
if "--skip-regex" not in sys.argv:
  for entity, variant in [("zone", "Zone"), ("surface", "Surface"), ("construction", "Construction"), ("thermostat", "Thermostat"), ("schedule", "Schedule"), ("material", "Material"), ("glazing material", "GlazingMaterial"), ("gas material", "GasMaterial"), ("fenestration", "Fenestration")]:
      regex(R, E, rf'target_missing\("{entity}", ', f"target_missing(ModelEntity::{variant}, ", {"zone": 4, "surface": 5, "construction": 5, "thermostat": 1, "schedule": 2, "material": 1, "glazing material": 1, "gas material": 1, "fenestration": 1}[entity])
  for blocker, variant in [("space", "Space"), ("surface", "Surface"), ("thermostat", "Thermostat")]:
      regex(R, E, rf'target_in_use\("zone", \*zone, "{blocker}"\)', f"zone_in_use(*zone, ZoneBlocker::{variant})", 1)
  regex(R, E, r'unknown_property\("(?:zone|surface|material|glazing material|gas material|fenestration|construction)", property\)', "unknown_property(property)", 7)
missing = "\n".join(f'        ModelEntity::{v} => app_fault("energy.model.{c}.missing").with_parameter("id", id),' for v, c in [("Zone", "zone"), ("Surface", "surface"), ("Construction", "construction"), ("Thermostat", "thermostat"), ("Schedule", "schedule"), ("Material", "material"), ("GlazingMaterial", "glazing-material"), ("GasMaterial", "gas-material"), ("Fenestration", "fenestration")])
edits = [
    (E, '''fn kind_unavailable(kind: &'static str, missing: &str) -> Fault {
    app_fault("mutation.kind-unavailable")
}''', '''fn kind_unavailable(kind: &'static str, _missing: &str) -> Fault {
    app_fault("mutation.kind-unavailable").with_parameter("action", kind)
}'''),
    (E, '''fn target_missing(entity: &str, id: u32) -> Fault {
    app_fault("mutation.target-missing")
}

fn target_in_use(entity: &str, id: u32, blocker: &str) -> Fault {
    app_fault("mutation.target-in-use")
}''', '''/// 🧱️ The model collection an addressed id is looked up in — each names its own refusal code, so the declared
/// text says which kind of element is gone.
#[derive(Clone, Copy)]
enum ModelEntity {
    Zone,
    Surface,
    Construction,
    Thermostat,
    Schedule,
    Material,
    GlazingMaterial,
    GasMaterial,
    Fenestration,
}

/// 🚧️ What still refers to a zone a delete addresses — each names its own refusal code.
#[derive(Clone, Copy)]
enum ZoneBlocker {
    Space,
    Surface,
    Thermostat,
}

fn target_missing(entity: ModelEntity, id: u32) -> Fault {
    let id = id.to_string();
    match entity {
''' + missing + '''
    }
}

fn zone_in_use(zone: u32, blocker: ZoneBlocker) -> Fault {
    let id = zone.to_string();
    match blocker {
        ZoneBlocker::Space => app_fault("energy.model.zone.in-use-by-space").with_parameter("id", id),
        ZoneBlocker::Surface => app_fault("energy.model.zone.in-use-by-surface").with_parameter("id", id),
        ZoneBlocker::Thermostat => app_fault("energy.model.zone.in-use-by-thermostat").with_parameter("id", id),
    }
}'''),
    (E, '''            if *volume_m3 <= 0.0 {
                return Err(app_fault("mutation.invalid-payload"));''', '''            if *volume_m3 <= 0.0 {
                return Err(app_fault("energy.model.zone.volume"));'''),
    (E, '''let class = surface_class_from_id(class).ok_or_else(|| app_fault("mutation.invalid-payload"))?;''', '''let class = surface_class_from_id(class).ok_or_else(|| app_fault("energy.model.surface.class-unknown").with_parameter("class", class))?;'''),
    (E, '''            if !(-90.0..=90.0).contains(latitude_deg) || !(-180.0..=180.0).contains(longitude_deg) {
                return Err(app_fault("mutation.invalid-payload"));''', '''            if !(-90.0..=90.0).contains(latitude_deg) || !(-180.0..=180.0).contains(longitude_deg) {
                return Err(app_fault("energy.model.site.location"));'''),
    (E, '''            if !valid {
                return Err(app_fault("mutation.invalid-payload"));''', '''            if !valid {
                return Err(app_fault("energy.model.run-period.invalid"));'''),
    (E, '''let loaded = example_model(example_id).ok_or_else(|| app_fault("mutation.target-missing"))?;''', '''let loaded = example_model(example_id).ok_or_else(|| app_fault("energy.model.example.unknown").with_parameter("example", example_id))?;'''),
    (E, '''            if !settings.config().is_valid() {
                return Err(app_fault("mutation.invalid-payload"));''', '''            if !settings.config().is_valid() {
                return Err(app_fault("energy.model.simulation-settings.range"));'''),
    (E, '''            let Some(selected) = crate::editor::model::results::ResultField::from_id(field) else {
                return Err(app_fault("mutation.invalid-payload"));''', '''            let Some(selected) = crate::editor::model::results::ResultField::from_id(field) else {
                return Err(app_fault("energy.model.result-field.unknown").with_parameter("field", field));'''),
    (E, '''fn unknown_property(entity: &str, property: &str) -> Fault {
    app_fault("mutation.invalid-payload")
}''', '''fn unknown_property(property: &str) -> Fault {
    app_fault("energy.model.property.unknown").with_parameter("property", property)
}'''),
    (E, '''fn invalid_value(property: &str, value: &str) -> Fault {
    app_fault("mutation.invalid-payload")
}''', '''fn invalid_value(property: &str, value: &str) -> Fault {
    app_fault("energy.model.property.value").with_parameter("property", property).with_parameter("value", value)
}'''),
    (E, '''        if !opaque.contains(&id) {
            return Err(app_fault("mutation.invalid-payload"));''', '''        if !opaque.contains(&id) {
            return Err(app_fault("energy.model.construction.layer-material").with_parameter("material", id.0.to_string()));'''),
    (E, '''    fn unknown(action: &str) -> Fault {
        app_fault("app.command.unsupported")
    }''', '''    fn unknown(action: &str) -> Fault {
        app_fault("app.command.unsupported").with_parameter("action", action)
    }'''),
    (E, '''    use super::{EnergyModelEditorCommand as Command, Fault};''', '''    use super::{app_fault, EnergyModelEditorCommand as Command, Fault};'''),
    (E, 'app_fault("app.command.invalid-payload")', 'app_fault("energy.model.camera.invalid")', 4),
    (V, 'app_fault("app.command.invalid-payload")', 'app_fault("energy.model.camera.invalid")', 4),
    (V, 'app_fault("energy.model.3d.viewer.window-required")', 'app_fault("energy.model.3d.window-required")', 1),
    (V, 'return Err(app_fault("app.command.unsupported"));', 'return Err(app_fault("app.command.unsupported").with_parameter("action", action));'),
    (V, 'app_fault("energy.model.viewer.retained.tool-mismatch")', 'app_fault("energy.model.retained.tool-mismatch")'),
    ("👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🎚️config/🦀️.rs", 'app_fault("energy.model.3d.viewer.window-required")', 'app_fault("energy.model.3d.window-required")'),
    ("👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🎚️config/🦀️.rs", 'app_fault("energy.model.3d.viewer.window-kind")', 'app_fault("energy.model.3d.window-kind")'),
    ("../../../../🦀️.rs", '''.map_err(|error| semio_framework_plugin::Fault::from(format!("energy model child projection failed: {error}")))''', '''.map_err(|_| semio_framework_plugin::app_fault("energy.model.child-projection"))'''),
]
editor = ["app.command.unsupported", "mutation.kind-unavailable", "energy.model.zone.missing", "energy.model.surface.missing", "energy.model.construction.missing", "energy.model.thermostat.missing", "energy.model.schedule.missing", "energy.model.material.missing", "energy.model.glazing-material.missing", "energy.model.gas-material.missing", "energy.model.fenestration.missing", "energy.model.zone.in-use-by-space", "energy.model.zone.in-use-by-surface", "energy.model.zone.in-use-by-thermostat", "energy.model.zone.volume", "energy.model.surface.class-unknown", "energy.model.site.location", "energy.model.run-period.invalid", "energy.model.example.unknown", "energy.model.simulation-settings.range", "energy.model.result-field.unknown", "energy.model.property.unknown", "energy.model.property.value", "energy.model.construction.layer-material", "energy.model.3d.window-required", "energy.model.3d.window-kind", "energy.model.camera.invalid", "energy.model.retained.tool-mismatch", "energy.model.retained.extent", "energy.model.child-projection"]
viewer = ["app.command.unsupported", "energy.model.3d.window-required", "energy.model.3d.window-kind", "energy.model.camera.invalid", "energy.model.3d.viewer.retained-route-required", "energy.model.retained.tool-mismatch", "energy.model.child-projection"]
edits.append((E, '''    builder = builder.action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome);
    builder.build_definition()''', '''    builder = builder.action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome);
''' + declare(editor, "    ", statement="builder") + '''    builder.build_definition()'''))
edits.append((V, '''        .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
        .build_definition()''', '''        .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
''' + declare(viewer, "        ", qualified=True) + '''        .build_definition()'''))
apply(R, edits)
