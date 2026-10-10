use super::super::{kind_of, FieldRow};
use super::*;
use crate::editor::bim::terminology::BimLabels;
use crate::editor::bim::unit_tests::support::demo;
use crate::standards::v1::subsets::any::schema::inferences::energy_envelope::EnvelopeSurface;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::with_inference;

fn with(snapshot: ModelSnapshot, kind: &str, id: &str, parent: &str) -> ModelSnapshot {
    let create = kind_of(kind).and_then(|row| row.create).expect("a creatable kind");
    let mutation = create(&snapshot, id, parent, "Sample").expect("creates");
    crate::mutations::apply_model_mutation(&snapshot, &mutation).expect("applies")
}

fn office() -> ModelSnapshot {
    let snapshot = with(demo(), "space", "sp-office", "st-ground");
    let snapshot = with(snapshot, "window-type", "wt-triple", "");
    with(snapshot, "door-type", "dt-steel", "")
}

fn field(kind: &str, key: &str) -> &'static FieldRow {
    kind_of(kind).expect("the kind").fields.iter().find(|row| row.key == key).unwrap_or_else(|| panic!("no {key} row on {kind}"))
}

fn written(snapshot: &ModelSnapshot, kind: &str, key: &str, id: &str, value: &str) -> Option<ModelSnapshot> {
    let mutation = (field(kind, key).write.expect("editable"))(snapshot, id, value)?;
    crate::mutations::apply_model_mutation(snapshot, &mutation).ok()
}

fn surface(kind: SurfaceKind, boundary: Boundary, area: f64) -> EnvelopeSurface {
    EnvelopeSurface { kind, boundary, area, gross_area: area, ..EnvelopeSurface::default() }
}

fn envelope() -> EnvelopeSpace {
    EnvelopeSpace {
        space: "sp-office".into(),
        conditioned: true,
        open_length: 0.0,
        surfaces: vec![
            surface(SurfaceKind::Wall, Boundary::Exterior, 20.0),
            surface(SurfaceKind::Window, Boundary::Exterior, 5.0),
            surface(SurfaceKind::Wall, Boundary::Adjacent, 10.0),
            surface(SurfaceKind::Floor, Boundary::Ground, 12.0),
            surface(SurfaceKind::Ceiling, Boundary::Adiabatic, 12.0),
        ],
        ..EnvelopeSpace::default()
    }
}

fn read(kind: &str, key: &str, snapshot: &ModelSnapshot, inference: &ModelInference, id: &str) -> Option<String> {
    let row = kind_of(kind).expect("the kind").inferred.iter().find(|row| row.key == key).unwrap_or_else(|| panic!("no {key} inferred row on {kind}"));
    (row.read)(snapshot, inference, id)
}

#[semio_framework_async_macros::async_test]
async fn a_space_without_conditions_reads_empty_rows_and_the_first_edit_creates_the_record() {
    let snapshot = office();
    for key in ["occupancy", "occupancy_density", "heating_setpoint", "cooling_setpoint", "ventilation_rate", "lighting_power_density", "equipment_power_density", "schedule"] {
        assert_eq!((field("space", key).read)(&snapshot, "sp-office").as_deref(), Some(""), "{key}");
        assert_eq!((field("space", key).read)(&snapshot, "nobody"), None, "{key}");
    }
    let heated = written(&snapshot, "space", "heating_setpoint", "sp-office", "21").expect("applies");
    assert_eq!(heated.space_conditions["sp-office"], SpaceConditions { heating_setpoint: Some(21.0), ..SpaceConditions::empty() });
    assert_eq!((field("space", "heating_setpoint").read)(&heated, "sp-office").as_deref(), Some("21"));
}

#[semio_framework_async_macros::async_test]
async fn every_condition_row_writes_exactly_its_own_field_and_a_blank_clears_it() {
    let mut snapshot = office();
    let rows = [("occupancy", "Office"), ("occupancy_density", "0.1"), ("heating_setpoint", "20"), ("cooling_setpoint", "26"), ("ventilation_rate", "1.5"), ("lighting_power_density", "9"), ("equipment_power_density", "12"), ("schedule", " Office 08-18 ")];
    for (key, value) in rows {
        snapshot = written(&snapshot, "space", key, "sp-office", value).unwrap_or_else(|| panic!("{key} applies"));
    }
    let expected = SpaceConditions {
        occupancy: Some("Office".into()),
        occupancy_density: Some(0.1),
        heating_setpoint: Some(20.0),
        cooling_setpoint: Some(26.0),
        ventilation_rate: Some(1.5),
        lighting_power_density: Some(9.0),
        equipment_power_density: Some(12.0),
        schedule: Some("Office 08-18".into()),
    };
    assert_eq!(snapshot.space_conditions["sp-office"], expected);
    let cleared = written(&snapshot, "space", "cooling_setpoint", "sp-office", " ").expect("clears");
    assert_eq!(cleared.space_conditions["sp-office"], SpaceConditions { cooling_setpoint: None, ..expected.clone() });
    let unnamed = written(&snapshot, "space", "occupancy", "sp-office", "").expect("clears");
    assert_eq!(unnamed.space_conditions["sp-office"].occupancy, None);
}

#[semio_framework_async_macros::async_test]
async fn a_value_the_rules_refuse_or_a_text_that_is_no_number_writes_nothing() {
    let snapshot = written(&office(), "space", "heating_setpoint", "sp-office", "22").expect("applies");
    assert!(written(&snapshot, "space", "cooling_setpoint", "sp-office", "18").is_none(), "cooling below heating is refused by the leaf");
    assert!(written(&snapshot, "space", "heating_setpoint", "sp-office", "99").is_none(), "out of the set point range");
    assert!(written(&snapshot, "space", "lighting_power_density", "sp-office", "-1").is_none());
    assert!((field("space", "occupancy_density").write.expect("editable"))(&snapshot, "sp-office", "many").is_none(), "no number");
}

#[semio_framework_async_macros::async_test]
async fn window_and_door_types_carry_thermal_rows_and_the_door_has_no_g_value() {
    let snapshot = office();
    let mut edited = snapshot.clone();
    for (key, value) in [("u_value", "0.9"), ("g_value", "0.5"), ("frame_fraction", "0.2")] {
        edited = written(&edited, "window-type", key, "wt-triple", value).unwrap_or_else(|| panic!("{key} applies"));
    }
    let window = &edited.window_types["wt-triple"];
    assert_eq!((window.u_value, window.g_value, window.frame_fraction), (Some(0.9), Some(0.5), Some(0.2)));
    assert_eq!((field("window-type", "g_value").read)(&edited, "wt-triple").as_deref(), Some("0.5"));
    assert_eq!((field("window-type", "u_value").read)(&snapshot, "wt-triple").as_deref(), Some(""));
    let cleared = written(&edited, "window-type", "g_value", "wt-triple", "").expect("clears");
    assert_eq!(cleared.window_types["wt-triple"].g_value, None);
    let door = written(&snapshot, "door-type", "u_value", "dt-steel", "1.8").expect("applies");
    assert_eq!(door.door_types["dt-steel"].u_value, Some(1.8));
    assert!(kind_of("door-type").expect("the kind").fields.iter().all(|row| row.key != "g_value" && row.key != "frame_fraction"));
    assert!(written(&snapshot, "window-type", "u_value", "wt-triple", "0").is_none(), "a U-value is above zero");
    assert!(written(&snapshot, "window-type", "frame_fraction", "wt-triple", "1.5").is_none(), "a share is at most one");
}

#[semio_framework_async_macros::async_test]
async fn the_facts_add_up_the_exterior_envelope_and_the_glazing_share_of_the_facade() {
    let facts = facts(&envelope());
    assert_eq!(facts.surfaces, 5);
    assert_eq!(facts.envelope_area, 37.0, "exterior wall, window and the ground floor; the neighbour and the adiabatic ceiling do not count");
    assert_eq!(facts.glazing_area, 5.0);
    assert!((facts.glazing_ratio - 0.2).abs() < 1e-12, "5 of the 25 square metres of exterior facade");
    assert_eq!(super::facts(&EnvelopeSpace::default()), EnvelopeFacts::default());
}

#[semio_framework_async_macros::async_test]
async fn the_inferred_rows_read_the_space_its_zone_and_the_building_and_stay_silent_without_conditions() {
    let mut snapshot = with(office(), "zone", "z-office", "");
    snapshot = written(&snapshot, "space", "zone", "sp-office", "z-office").expect("joins the zone");
    let mut inference = with_inference(None, &snapshot, Clone::clone);
    for key in ["envelope_surfaces", "envelope_area", "glazing_area", "glazing_ratio", "zone_h_t_prime", "zone_a_over_v", "zone_envelope_area", "zone_glazing_ratio"] {
        assert_eq!(read("space", key, &snapshot, &inference, "sp-office"), None, "{key} without conditions");
    }
    inference.energy_envelopes.insert("sp-office".into(), envelope());
    let totals = EnergyTotals { envelope_area: 37.0, h_t_prime: 0.4231, a_over_v: 0.8, glazing_ratio: 0.2, ..EnergyTotals::default() };
    inference.energy_totals.insert(EnergyScope::Zone("z-office".into()).key(), totals.clone());
    let building = snapshot.storeys.values().next().expect("a storey").building.clone();
    inference.energy_totals.insert(EnergyScope::Building(building.clone()).key(), totals);
    let space = |key: &str| read("space", key, &snapshot, &inference, "sp-office");
    assert_eq!((space("envelope_surfaces").as_deref(), space("envelope_area").as_deref(), space("glazing_area").as_deref(), space("glazing_ratio").as_deref()), (Some("5"), Some("37.00"), Some("5.00"), Some("0.20")));
    assert_eq!((space("zone_h_t_prime").as_deref(), space("zone_a_over_v").as_deref(), space("zone_envelope_area").as_deref(), space("zone_glazing_ratio").as_deref()), (Some("0.42"), Some("0.80"), Some("37.00"), Some("0.20")));
    assert_eq!((space("open_length"), space("envelope_findings")), (None, None), "nothing open and nothing found shows no row");
    assert_eq!(read("zone", "h_t_prime", &snapshot, &inference, "z-office").as_deref(), Some("0.42"));
    assert_eq!(read("building", "a_over_v", &snapshot, &inference, &building).as_deref(), Some("0.80"));
    assert_eq!(read("zone", "h_t_prime", &snapshot, &inference, "nobody"), None);
    assert_eq!(read("wall", "envelope_area", &snapshot, &inference, "w-south"), None, "a wall is no space");
}

#[semio_framework_async_macros::async_test]
async fn an_open_boundary_and_findings_show_rows_only_when_there_are_some() {
    let snapshot = office();
    let mut inference = with_inference(None, &snapshot, Clone::clone);
    let mut space = envelope();
    space.open_length = 1.25;
    space.issues.push(crate::standards::v1::subsets::any::schema::inferences::energy_envelope::EnergyIssue { code: crate::standards::v1::subsets::any::schema::inferences::energy_envelope::EnergyCode::OpenBoundary, element: "sp-office".into(), detail: "1.25".into() });
    inference.energy_envelopes.insert("sp-office".into(), space);
    assert_eq!(read("space", "open_length", &snapshot, &inference, "sp-office").as_deref(), Some("1.25"));
    assert_eq!(read("space", "envelope_findings", &snapshot, &inference, "sp-office").as_deref(), Some("1"));
}

#[semio_framework_async_macros::async_test]
async fn every_energy_row_has_an_english_and_a_german_label() {
    for kind in ["space", "window-type", "door-type", "zone", "building"] {
        let row = kind_of(kind).expect("the kind");
        for labels in [&BimLabels::NATIVE_EN, &BimLabels::NATIVE_DE] {
            for field in row.fields {
                assert!(!(field.label)(labels).as_str().trim().is_empty(), "{kind}.{}", field.key);
            }
            for inferred in row.inferred {
                assert!(!(inferred.label)(labels).as_str().trim().is_empty(), "{kind}.{}", inferred.key);
            }
        }
    }
    for (kind, key) in [("space", "heating_setpoint"), ("window-type", "g_value")] {
        let label = field(kind, key).label;
        assert_ne!(label(&BimLabels::NATIVE_EN).as_str(), label(&BimLabels::NATIVE_DE).as_str(), "{kind}.{key} is translated, not copied");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_summary_names_the_set_points_the_occupancy_and_the_schedule_in_one_line() {
    let both = SpaceConditions { heating_setpoint: Some(21.0), cooling_setpoint: Some(26.5), occupancy: Some("Office".into()), schedule: Some("Office 08-18".into()), ..SpaceConditions::empty() };
    assert_eq!(summary(&both), "21–26.5 °C · Office · Office 08-18");
    assert_eq!(summary(&SpaceConditions { heating_setpoint: Some(20.0), ..SpaceConditions::empty() }), "≥ 20 °C");
    assert_eq!(summary(&SpaceConditions { cooling_setpoint: Some(24.0), ..SpaceConditions::empty() }), "≤ 24 °C");
    assert_eq!(summary(&SpaceConditions::empty()), "");
}

#[semio_framework_async_macros::async_test]
async fn a_curtain_wall_type_carries_the_three_thermal_rows_of_a_window() {
    let mut snapshot = with(demo(), "curtain-wall-type", "cwt-glass", "");
    for (key, value) in [("u_value", "1.3"), ("g_value", "0.45"), ("frame_fraction", "0.15")] {
        assert_eq!((field("curtain-wall-type", key).read)(&snapshot, "cwt-glass").as_deref(), Some(""), "{key} unstated");
        snapshot = written(&snapshot, "curtain-wall-type", key, "cwt-glass", value).unwrap_or_else(|| panic!("{key} applies"));
    }
    let row = &snapshot.curtain_wall_types["cwt-glass"];
    assert_eq!((row.u_value, row.g_value, row.frame_fraction), (Some(1.3), Some(0.45), Some(0.15)));
    assert!(written(&snapshot, "curtain-wall-type", "g_value", "cwt-glass", "2").is_none(), "a g-value is at most one");
    assert!(written(&snapshot, "curtain-wall-type", "u_value", "cwt-glass", "").is_some_and(|cleared| cleared.curtain_wall_types["cwt-glass"].u_value.is_none()));
}
