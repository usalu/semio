use super::*;
use crate::standards::v1::subsets::any::io::export::ifc::testkit::{count, document_in, rows, string};
use crate::{ModelSnapshot, SpaceConditions};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const ENERGY: &str = include_str!("../../../../../../🧫️fixtures/🏗️ifc/🔥️energy/📸️snapshot/🔣️.json");

fn energy() -> ModelSnapshot {
    from_json_str(ENERGY, JsonMemberPolicy::Reject).expect("the committed energy model decodes")
}

fn report(schema: Schema, model: &ModelSnapshot) -> String {
    thermal_report(schema, model).expect("the model exports")
}

fn set_count(schema: Schema, model: &ModelSnapshot, name: &str) -> usize {
    rows(&document_in(schema, model), "IFCPROPERTYSET").iter().filter(|(_, args)| string(args, 2).as_deref() == Some(name)).count()
}

#[test]
fn kelvin_is_celsius_plus_273_15() {
    assert_eq!(KELVIN, 273.15);
}

#[test]
fn every_space_with_conditions_gets_its_thermal_requirements_and_the_record() {
    let model = energy();
    let stating = model.space_conditions.len();
    assert!(stating > 0);
    for schema in [Schema::Ifc2x3, Schema::Ifc4] {
        assert_eq!(set_count(schema, &model, SPACE_SET), stating, "{schema:?}: one set per space with conditions");
        let text = report(schema, &model);
        for (id, conditions) in &model.space_conditions {
            assert!(text.contains(&format!("{}:{{\"{SPACE_SET}\"", quote(id))) || text.contains(&quote(id)), "{id}");
            if let Some(heating) = conditions.heating_setpoint {
                assert!(text.contains(&format!("[\"IFCTHERMODYNAMICTEMPERATUREMEASURE\",{:?}]", heating + KELVIN)) || heating == 0.0, "{id}: {heating} C in kelvin");
            }
        }
        assert_eq!(text.matches("\"Conditions\"").count(), 0, "the record rows are listed under conditions, not as properties");
    }
}

#[test]
fn a_space_with_only_a_heating_set_point_has_no_maximum_and_no_air_conditioning() {
    let mut model = energy();
    model.space_conditions.clear();
    let id = model.spaces.keys().next().expect("a space").clone();
    model.space_conditions.insert(id.clone(), SpaceConditions { heating_setpoint: Some(20.0), ..SpaceConditions::empty() });
    let text = report(Schema::Ifc2x3, &model);
    let own = &text[text.find(&format!("{}:{{\"{SPACE_SET}\"", quote(&id))).expect("the set")..];
    let own = &own[..own.find("}}").expect("its end")];
    assert!(own.contains("\"SpaceTemperatureMin\":[\"IFCTHERMODYNAMICTEMPERATUREMEASURE\",293.15]"), "{own}");
    assert!(own.contains("\"SpaceTemperatureWinterMin\"") && !own.contains("SpaceTemperatureMax") && !own.contains("SummerMax"), "{own}");
    assert!(own.contains("\"AirConditioning\":[\"IFCBOOLEAN\",false]"), "{own}");
}

#[test]
fn the_outdoor_air_flow_is_written_as_air_changes_of_the_conditioned_or_the_natural_kind() {
    let mut model = energy();
    model.space_conditions.clear();
    let ids: Vec<String> = model.spaces.keys().take(2).cloned().collect();
    model.space_conditions.insert(ids[0].clone(), SpaceConditions { heating_setpoint: Some(20.0), ventilation_rate: Some(1.0), ..SpaceConditions::empty() });
    model.space_conditions.insert(ids[1].clone(), SpaceConditions { ventilation_rate: Some(1.0), ..SpaceConditions::empty() });
    let text = report(Schema::Ifc4, &model);
    assert!(text.contains("\"MechanicalVentilationRate\":[\"IFCCOUNTMEASURE\""), "{text}");
    assert!(text.contains("\"NaturalVentilationRate\":[\"IFCCOUNTMEASURE\"") && text.contains("\"NaturalVentilation\":[\"IFCBOOLEAN\",true]"), "{text}");
}

#[test]
fn an_authored_thermal_transmittance_wins_and_is_not_listed_as_derived() {
    let model = energy();
    let text = report(Schema::Ifc2x3, &model);
    let wall = &text[text.find("\"w-g-south\":{\"Pset_WallCommon\"").expect("the authored wall")..];
    let wall = &wall[..wall.find("}}").expect("its end")];
    assert!(wall.contains("\"ThermalTransmittance\":[\"IFCREAL\",0.18]"), "the authored value and type are kept: {wall}");
    let derived = &text[text.find("\"derived\":").expect("the derived rows")..];
    assert!(!derived.contains("\"w-g-south\":\"Pset_WallCommon.ThermalTransmittance"), "{derived}");
    assert!(derived.contains("Pset_WallCommon.ThermalTransmittance"), "other walls derive theirs");
}

#[test]
fn windows_and_doors_carry_the_values_of_their_type_and_the_type_its_exact_rows() {
    let model = energy();
    let text = report(Schema::Ifc4, &model);
    assert!(text.contains("\"Pset_DoorCommon\":{\"ThermalTransmittance\":[\"IFCTHERMALTRANSMITTANCEMEASURE\",1.3]"), "{text}");
    assert!(text.contains("\"GlazingAreaFraction\":[\"IFCPOSITIVERATIOMEASURE\",0.7]") || text.contains("\"GlazingAreaFraction\":[\"IFCPOSITIVERATIOMEASURE\",0.75]"), "{text}");
    assert!(text.contains("\"SolarHeatGainTransmittance\":[\"IFCNORMALISEDRATIOMEASURE\""), "IFC4 types the glazing value as a normalised ratio");
    assert!(report(Schema::Ifc2x3, &model).contains("\"SolarHeatGainTransmittance\":[\"IFCPOSITIVERATIOMEASURE\""), "IFC 2x3 as a positive ratio");
    assert!(text.contains("\"dr-entry\":{\"UValue\":1.3}"), "{text}");
    assert!(text.contains("\"UValue\":1.4,\"GValue\":0.55,\"FrameFraction\":0.3") || text.contains("\"FrameFraction\":0.3"), "{text}");
}

#[test]
fn only_ifc4_has_a_thermal_transmittance_in_the_roof_set() {
    let model = energy();
    assert!(!report(Schema::Ifc2x3, &model).contains("Pset_RoofCommon"));
    assert!(report(Schema::Ifc4, &model).contains("\"Pset_RoofCommon\":{\"ThermalTransmittance\""));
}

#[test]
fn walls_and_slabs_take_the_area_weighted_value_of_their_surfaces() {
    let model = energy();
    let text = report(Schema::Ifc2x3, &model);
    for set in ["Pset_WallCommon", "Pset_SlabCommon"] {
        assert!(text.contains(&format!("\"{set}\":{{")), "{set}");
    }
    assert!(count(&document_in(Schema::Ifc2x3, &model), "IFCPROPERTYSET") > 0);
}

#[test]
fn a_model_without_conditions_derives_nothing_for_walls_and_spaces_but_keeps_the_types() {
    let mut model = energy();
    model.space_conditions.clear();
    let text = report(Schema::Ifc2x3, &model);
    assert!(!text.contains(SPACE_SET) && !text.contains("\"Conditions\""));
    assert!(!text.contains("Pset_WallCommon.ThermalTransmittance"), "no envelope, no wall U-value");
    assert!(text.contains("Pset_WindowCommon.ThermalTransmittance"), "the window values come from the type");
}

#[test]
fn the_report_is_deterministic_and_parses_as_json() {
    let model = energy();
    let first = report(Schema::Ifc4, &model);
    assert_eq!(first, report(Schema::Ifc4, &model));
    assert!(first.starts_with('{') && first.ends_with('}') && first.matches('{').count() == first.matches('}').count() && first.matches('"').count() % 2 == 0, "{first}");
}

const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🏗️ifc/🔥️energy");

#[test]
fn the_committed_files_are_the_current_export() {
    let model = energy();
    for (name, export) in [("energy-2x3.ifc", super::super::export_ifc2x3(&model)), ("energy-4.ifc", super::super::export_ifc4(&model))] {
        let (bytes, _) = export.expect("the export");
        let path = format!("{FIXTURES}/{name}");
        if std::env::var("BIM_BLESS").is_ok() {
            std::fs::create_dir_all(FIXTURES).expect("the fixture directory");
            std::fs::write(&path, &bytes).expect("the file is written");
        }
        let committed = std::fs::read(&path).unwrap_or_else(|error| panic!("{name}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the export case."));
        assert_eq!(committed, bytes, "{name}: the committed export drifted: rewrite it with BIM_BLESS=1");
    }
}
