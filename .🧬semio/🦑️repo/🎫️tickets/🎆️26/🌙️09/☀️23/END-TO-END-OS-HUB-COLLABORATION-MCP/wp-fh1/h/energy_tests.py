"""🔋️ FH1 family H — energy editor laws assert the new refusal codes."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply
R = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/"
T = "🦀️.rs"
pairs = [
    ('fenestration_property(50, "nonsense", "1.0")), "mutation.invalid-payload"', 'fenestration_property(50, "nonsense", "1.0")), "energy.model.property.unknown"'),
    ('fenestration_property(50, "uValueWM2K", "warm")), "mutation.invalid-payload"', 'fenestration_property(50, "uValueWM2K", "warm")), "energy.model.property.value"'),
    ('fenestration_property(50, "shgc", "1.5")), "mutation.invalid-payload"', 'fenestration_property(50, "shgc", "1.5")), "energy.model.property.value"'),
    ('fenestration_property(9_999, "uValueWM2K", "1.0")), "mutation.target-missing"', 'fenestration_property(9_999, "uValueWM2K", "1.0")), "energy.model.fenestration.missing"'),
    ('fenestration_property(50, "glazingConstruction", "9999")), "mutation.target-missing"', 'fenestration_property(50, "glazingConstruction", "9999")), "energy.model.construction.missing"'),
    ('surface_property(40, "boundary", "interzone")), "mutation.invalid-payload"', 'surface_property(40, "boundary", "interzone")), "energy.model.property.value"'),
    ('assert_eq!(refusal(&snapshot, &dangling), "mutation.target-missing");', 'assert_eq!(refusal(&snapshot, &dangling), "energy.model.surface.missing");'),
    ('surface_property(40, "nonsense", "x")), "mutation.invalid-payload"', 'surface_property(40, "nonsense", "x")), "energy.model.property.unknown"'),
    ('surface_property(40, "class", "wall")), "mutation.invalid-payload"', 'surface_property(40, "class", "wall")), "energy.model.property.value"'),
    ('surface_property(40, "multiplier", "0")), "mutation.invalid-payload"', 'surface_property(40, "multiplier", "0")), "energy.model.property.value"'),
    ('surface_property(9_999, "name", "Ghost")), "mutation.target-missing"', 'surface_property(9_999, "name", "Ghost")), "energy.model.surface.missing"'),
    ('surface_property(40, "construction", "9999")), "mutation.target-missing"', 'surface_property(40, "construction", "9999")), "energy.model.construction.missing"'),
    ('zone_property(1, "nonsense", "x")), "mutation.invalid-payload"', 'zone_property(1, "nonsense", "x")), "energy.model.property.unknown"'),
    ('zone_property(1, "volumeM3", "-4")), "mutation.invalid-payload"', 'zone_property(1, "volumeM3", "-4")), "energy.model.property.value"'),
    ('zone_property(9_999, "name", "Ghost")), "mutation.target-missing"', 'zone_property(9_999, "name", "Ghost")), "energy.model.zone.missing"'),
    ('glazing_property(22, "solarReflectanceFront", "0.1")), "mutation.invalid-payload"', 'glazing_property(22, "solarReflectanceFront", "0.1")), "energy.model.property.unknown"'),
    ('glazing_property(22, "thicknessM", "clear")), "mutation.invalid-payload"', 'glazing_property(22, "thicknessM", "clear")), "energy.model.property.value"'),
    ('glazing_property(9_999, "thicknessM", "0.006")), "mutation.target-missing"', 'glazing_property(9_999, "thicknessM", "0.006")), "energy.model.glazing-material.missing"'),
    ('gas_property(23, "gas", "helium")), "mutation.invalid-payload"', 'gas_property(23, "gas", "helium")), "energy.model.property.value"'),
    ('gas_property(9_999, "thicknessM", "0.016")), "mutation.target-missing"', 'gas_property(9_999, "thicknessM", "0.016")), "energy.model.gas-material.missing"'),
    ('material_property(10, "roughness", "gritty")), "mutation.invalid-payload"', 'material_property(10, "roughness", "gritty")), "energy.model.property.value"'),
    ('material_property(10, "conductivityWMK", "-1")), "mutation.invalid-payload"', 'material_property(10, "conductivityWMK", "-1")), "energy.model.property.value"'),
    ('material_property(10, "reflectance", "0.5")), "mutation.invalid-payload"', 'material_property(10, "reflectance", "0.5")), "energy.model.property.unknown"'),
    ('material_property(9_999, "name", "Ghost")), "mutation.target-missing"', 'material_property(9_999, "name", "Ghost")), "energy.model.material.missing"'),
    ('construction_property(id, "addLayer", &glazing.id.0.to_string())), "mutation.invalid-payload"', 'construction_property(id, "addLayer", &glazing.id.0.to_string())), "energy.model.construction.layer-material"'),
    ('construction_property(id, "removeLayer", "99")), "mutation.invalid-payload"', 'construction_property(id, "removeLayer", "99")), "energy.model.property.value"'),
    ('construction_property(id, "moveLayerUp", "0")), "mutation.invalid-payload"', 'construction_property(id, "moveLayerUp", "0")), "energy.model.property.value"'),
    ('construction_property(id, "thickness", "0.2")), "mutation.invalid-payload"', 'construction_property(id, "thickness", "0.2")), "energy.model.property.unknown"'),
    ('construction_property(9_999, "name", "Ghost")), "mutation.target-missing"', 'construction_property(9_999, "name", "Ghost")), "energy.model.construction.missing"'),
    ('surface_property(first.0, "interzonePartner", &first.0.to_string())), "mutation.invalid-payload"', 'surface_property(first.0, "interzonePartner", &first.0.to_string())), "energy.model.property.value"'),
    ('surface_property(first.0, "interzonePartner", "9999")), "mutation.target-missing"', 'surface_property(first.0, "interzonePartner", "9999")), "energy.model.surface.missing"'),
    ('surface_property(first.0, "interzonePartner", "0")), "mutation.invalid-payload"', 'surface_property(first.0, "interzonePartner", "0")), "energy.model.property.value"'),
    ('''    let fault = reduce(&EnergyModelEditorCommand::SetActiveExample { example_id: "nonsense".into() }, &doc).err().expect("an unknown example id is refused");
    assert_eq!(fault.code.0.as_str(), "mutation.target-missing");''', '''    let fault = reduce(&EnergyModelEditorCommand::SetActiveExample { example_id: "nonsense".into() }, &doc).err().expect("an unknown example id is refused");
    assert_eq!(fault.code.0.as_str(), "energy.model.example.unknown");'''),
    ('''    let fault = reduce(&EnergyModelEditorCommand::DeleteZone { zone: 1 }, &doc).err().expect("a referenced zone is refused");
    assert_eq!(fault.code.0.as_str(), "mutation.target-in-use");''', '''    let fault = reduce(&EnergyModelEditorCommand::DeleteZone { zone: 1 }, &doc).err().expect("a referenced zone is refused");
    assert_eq!(fault.code.0.as_str(), "energy.model.zone.in-use-by-surface");'''),
    ('''    for command in [
        EnergyModelEditorCommand::SetMaterialProperty { material: 1, property: "conductivityWMK".into(), value: "-1.0".into() },
        EnergyModelEditorCommand::SetSite { latitude_deg: 120.0, longitude_deg: 0.0, elevation_m: 0.0, time_zone_hours: 0.0, north_axis_deg: 0.0 },
        EnergyModelEditorCommand::SetRunPeriod { start_month: 13, start_day: 1, end_month: 12, end_day: 31 },
        EnergyModelEditorCommand::CreateZone { name: "Void".into(), volume_m3: 0.0, multiplier: 1, conditioned: true },
    ] {
        let fault = reduce(&command, &doc).err().expect("an out-of-range payload is refused");
        assert_eq!(fault.code.0.as_str(), "mutation.invalid-payload", "{} failed for the wrong reason", command.action_id());
    }
    let fault = reduce(&EnergyModelEditorCommand::SetMaterialProperty { material: 9, property: "conductivityWMK".into(), value: "0.04".into() }, &doc).err().expect("an unknown material is refused");
    assert_eq!(fault.code.0.as_str(), "mutation.target-missing");''', '''    for (command, code) in [
        (EnergyModelEditorCommand::SetMaterialProperty { material: 1, property: "conductivityWMK".into(), value: "-1.0".into() }, "energy.model.property.value"),
        (EnergyModelEditorCommand::SetSite { latitude_deg: 120.0, longitude_deg: 0.0, elevation_m: 0.0, time_zone_hours: 0.0, north_axis_deg: 0.0 }, "energy.model.site.location"),
        (EnergyModelEditorCommand::SetRunPeriod { start_month: 13, start_day: 1, end_month: 12, end_day: 31 }, "energy.model.run-period.invalid"),
        (EnergyModelEditorCommand::CreateZone { name: "Void".into(), volume_m3: 0.0, multiplier: 1, conditioned: true }, "energy.model.zone.volume"),
    ] {
        let fault = reduce(&command, &doc).err().expect("an out-of-range payload is refused");
        assert_eq!(fault.code.0.as_str(), code, "{} failed for the wrong reason", command.action_id());
    }
    let fault = reduce(&EnergyModelEditorCommand::SetMaterialProperty { material: 9, property: "conductivityWMK".into(), value: "0.04".into() }, &doc).err().expect("an unknown material is refused");
    assert_eq!(fault.code.0.as_str(), "energy.model.material.missing");'''),
]
apply(R, [(T, old, new) for old, new in pairs])
