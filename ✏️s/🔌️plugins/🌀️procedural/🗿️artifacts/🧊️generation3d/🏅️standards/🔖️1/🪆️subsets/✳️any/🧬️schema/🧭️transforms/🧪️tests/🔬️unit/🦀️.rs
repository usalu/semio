use super::*;

#[test]
fn per_axis_scale_matches_shared_matrix_fixtures() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["scales"].as_array().unwrap() {
        let vector = |key: &str| [0, 1, 2].map(|axis| case[key][axis].as_f64().unwrap());
        assert_eq!(compose_scale(vector("current"), vector("next")).unwrap(), vector("expected"));
    }
    for (current, next) in [([1.0; 3], [0.0, 1.0, 1.0]), ([1e200; 3], [1e200; 3]), ([1e-200; 3], [1e-200; 3])] {
        assert!(compose_scale(current, next).is_err());
    }
}

#[derive(serde::Deserialize)]
struct Fixture { cases: Vec<Case> }
#[derive(serde::Deserialize)]
struct Case { name: String, rotations: Vec<Rotation>, point: [f64; 3], expected: [f64; 3] }
#[derive(serde::Deserialize)]
struct Rotation { axis: [f64; 3], angle: f64 }

#[test]
fn composed_rotations_match_shared_world_axis_fixtures() {
    let fixtures: Fixture = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixtures.cases {
        let mut result = AxisAngle::IDENTITY;
        for rotation in case.rotations {
            result = result.then(AxisAngle { axis: rotation.axis, angle: rotation.angle }).unwrap();
        }
        let [x, y, z] = result.axis;
        let [px, py, pz] = case.point;
        let (s, c) = result.angle.sin_cos();
        let d = x * px + y * py + z * pz;
        let actual = [px * c + (y * pz - z * py) * s + x * d * (1.0 - c), py * c + (z * px - x * pz) * s + y * d * (1.0 - c), pz * c + (x * py - y * px) * s + z * d * (1.0 - c)];
        for axis in 0..3 { assert!((actual[axis] - case.expected[axis]).abs() < 1e-10, "{}: {actual:?}", case.name); }
    }
}

#[test]
fn composition_rejects_invalid_parameters() {
    for bad in [AxisAngle { axis: [0.0; 3], angle: 1.0 }, AxisAngle { axis: [f64::INFINITY, 0.0, 0.0], angle: 1.0 }, AxisAngle { axis: [1.0, 0.0, 0.0], angle: f64::NAN }] {
        assert!(AxisAngle::IDENTITY.then(bad).is_err());
    }
}
