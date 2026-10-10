//! 🧪️ ISO 6946: the surface resistances of the three flow directions, the resistance of a homogeneous layer stack and the transmittance against hand-computed values.

use super::*;

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-12
}

#[test]
fn surface_resistances_follow_table_seven() {
    assert!(close(HeatFlow::Up.rsi(), 0.10) && close(HeatFlow::Horizontal.rsi(), 0.13) && close(HeatFlow::Down.rsi(), 0.17));
    assert!([HeatFlow::Up, HeatFlow::Horizontal, HeatFlow::Down].into_iter().all(|flow| close(flow.rse(), 0.04)));
}

#[test]
fn the_far_side_decides_the_second_surface_resistance() {
    assert!(close(FarSide::Outdoor.resistance(HeatFlow::Horizontal), 0.04));
    assert!(close(FarSide::Ground.resistance(HeatFlow::Down), 0.0));
    assert!(close(FarSide::Room.resistance(HeatFlow::Down), 0.17));
}

#[test]
fn a_wall_of_brick_and_insulation_has_the_hand_computed_transmittance() {
    let layers = [(0.175, 0.79), (0.14, 0.035), (0.015, 0.7)];
    let expected = 1.0 / (0.13 + 0.175 / 0.79 + 0.14 / 0.035 + 0.015 / 0.7 + 0.04);
    assert!(close(transmittance(&layers, HeatFlow::Horizontal, FarSide::Outdoor).unwrap(), expected));
    assert!((expected - 0.2266).abs() < 1e-4, "{expected}");
}

#[test]
fn a_floor_on_the_ground_has_no_second_surface_resistance() {
    let layers = [(0.2, 2.3), (0.12, 0.035)];
    let expected = 1.0 / (0.17 + 0.2 / 2.3 + 0.12 / 0.035);
    assert!(close(transmittance(&layers, HeatFlow::Down, FarSide::Ground).unwrap(), expected));
}

#[test]
fn a_roof_conducts_with_the_upward_resistances() {
    let layers = [(0.24, 0.035)];
    let expected = 1.0 / (0.10 + 0.24 / 0.035 + 0.04);
    assert!(close(transmittance(&layers, HeatFlow::Up, FarSide::Outdoor).unwrap(), expected));
}

#[test]
fn a_layer_without_thermal_data_has_no_transmittance() {
    assert_eq!(transmittance(&[], HeatFlow::Horizontal, FarSide::Outdoor), None);
    assert_eq!(transmittance(&[(0.2, 0.0)], HeatFlow::Horizontal, FarSide::Outdoor), None);
    assert_eq!(transmittance(&[(0.0, 1.0)], HeatFlow::Horizontal, FarSide::Outdoor), None);
    assert_eq!(transmittance(&[(0.2, f64::NAN)], HeatFlow::Horizontal, FarSide::Outdoor), None);
}

#[test]
fn doubling_the_insulation_never_raises_the_transmittance() {
    let thin = transmittance(&[(0.1, 0.04)], HeatFlow::Horizontal, FarSide::Outdoor).unwrap();
    let thick = transmittance(&[(0.2, 0.04)], HeatFlow::Horizontal, FarSide::Outdoor).unwrap();
    assert!(thick < thin);
}
