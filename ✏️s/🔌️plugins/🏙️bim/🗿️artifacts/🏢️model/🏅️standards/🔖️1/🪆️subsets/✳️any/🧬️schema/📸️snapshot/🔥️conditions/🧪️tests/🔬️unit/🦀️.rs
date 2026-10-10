//! 🧪️ The thermal-data rules: set point and flow limits of the conditions of a space, the ranges of the thermal data of a window or door type.

use super::*;

fn window() -> WindowType {
    WindowType { name: "Triple".into(), width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.06, frame_depth: 0.08, panes: 3, material: "m-wood".into(), u_value: Some(0.9), g_value: Some(0.5), frame_fraction: Some(0.25) }
}

fn door() -> DoorType {
    DoorType { name: "Entrance".into(), width: 1.0, height: 2.1, frame_width: 0.06, frame_depth: 0.08, leaves: crate::DoorLeaves::Single, swing: crate::Swing::Left, material: "m-wood".into(), u_value: Some(1.3) }
}

#[test]
fn empty_conditions_state_nothing_and_are_valid() {
    let conditions = SpaceConditions::empty();
    assert_eq!(conditions_problem(&conditions), None);
    assert!(!conditions.heated());
}

#[test]
fn a_heating_set_point_makes_a_space_heated() {
    assert!(SpaceConditions { heating_setpoint: Some(20.0), ..SpaceConditions::empty() }.heated());
}

#[test]
fn set_points_stay_inside_the_limits_and_cooling_never_below_heating() {
    let with = |heating, cooling| SpaceConditions { heating_setpoint: heating, cooling_setpoint: cooling, ..SpaceConditions::empty() };
    assert_eq!(conditions_problem(&with(Some(20.0), Some(26.0))), None);
    assert_eq!(conditions_problem(&with(Some(20.0), Some(20.0))), None);
    assert_eq!(conditions_problem(&with(Some(60.0), None)).map(|problem| problem.0), Some("heating_setpoint"));
    assert_eq!(conditions_problem(&with(None, Some(f64::NAN))).map(|problem| problem.0), Some("cooling_setpoint"));
    assert_eq!(conditions_problem(&with(Some(24.0), Some(20.0))).map(|problem| problem.0), Some("cooling_setpoint"));
}

#[test]
fn flows_and_densities_are_finite_and_not_negative() {
    let base = SpaceConditions::empty();
    for (field, conditions) in [
        ("occupancy_density", SpaceConditions { occupancy_density: Some(-0.1), ..base.clone() }),
        ("ventilation_rate", SpaceConditions { ventilation_rate: Some(f64::INFINITY), ..base.clone() }),
        ("lighting_power_density", SpaceConditions { lighting_power_density: Some(-1.0), ..base.clone() }),
        ("equipment_power_density", SpaceConditions { equipment_power_density: Some(f64::NAN), ..base.clone() }),
        ("occupancy", SpaceConditions { occupancy: Some("  ".into()), ..base.clone() }),
        ("schedule", SpaceConditions { schedule: Some(String::new()), ..base.clone() }),
    ] {
        assert_eq!(conditions_problem(&conditions).map(|problem| problem.0), Some(field));
    }
    assert_eq!(conditions_problem(&SpaceConditions { occupancy_density: Some(0.0), ..base }), None);
}

#[test]
fn window_thermal_data_is_in_range() {
    assert_eq!(window_thermal_problem(&window()), None);
    assert_eq!(window_thermal_problem(&WindowType { u_value: None, g_value: None, frame_fraction: None, ..window() }), None);
    assert_eq!(window_thermal_problem(&WindowType { u_value: Some(0.0), ..window() }).map(|problem| problem.0), Some("u_value"));
    assert_eq!(window_thermal_problem(&WindowType { u_value: Some(10.5), ..window() }).map(|problem| problem.0), Some("u_value"));
    assert_eq!(window_thermal_problem(&WindowType { g_value: Some(1.2), ..window() }).map(|problem| problem.0), Some("g_value"));
    assert_eq!(window_thermal_problem(&WindowType { frame_fraction: Some(1.0), ..window() }).map(|problem| problem.0), Some("frame_fraction"));
    assert_eq!(window_thermal_problem(&WindowType { frame_fraction: Some(0.0), ..window() }), None);
}

#[test]
fn door_thermal_data_is_in_range() {
    assert_eq!(door_thermal_problem(&door()), None);
    assert_eq!(door_thermal_problem(&DoorType { u_value: Some(-1.0), ..door() }).map(|problem| problem.0), Some("u_value"));
}
