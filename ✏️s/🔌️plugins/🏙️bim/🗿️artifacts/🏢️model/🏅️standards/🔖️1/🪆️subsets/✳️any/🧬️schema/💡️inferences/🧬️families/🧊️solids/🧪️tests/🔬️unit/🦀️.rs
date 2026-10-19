use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{SolidAxis, SolidShape};
use std::collections::BTreeMap;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::families::issues::{FamilyIssue, FamilyIssueCode};
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::FamilySolidMesh;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::families::issues::IssueOwner;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::{SolidBounds, SolidPoint};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::profile::CHORD_TOLERANCE;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::ExprPoint;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::FamilySolid;
#[allow(unused_imports)]
use semio_framework_expression::{evaluate, parse, Kind, Value};
#[allow(unused_imports)]
use semio_framework_geometry::bulge::BulgeSeg;
#[allow(unused_imports)]
use semio_framework_geometry::loops::{self, Vertex};
#[allow(unused_imports)]
use semio_framework_geometry::mesh::{extrude_loops, revolve_profile, sweep_profile};
#[allow(unused_imports)]
use semio_framework_geometry::placement::{Affine3, ZPlane};
#[allow(unused_imports)]
use semio_framework_geometry::Point;
#[allow(unused_imports)]
use std::collections::BTreeSet;
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::solids::{Evaluated, evaluate_solid};
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit;
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::parameters::resolve;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{FamilyCategory, ParameterKind, ParametricProfile};
use semio_framework_geometry::mesh::TriMesh;
use std::f64::consts::PI;

fn evaluated(snapshot: &ModelSnapshot, id: &str) -> (Evaluated, Vec<FamilyIssue>) {
    let solid = &snapshot.family_solids[id];
    let resolution = resolve(snapshot, &solid.family, &BTreeMap::new());
    evaluate_solid(id, solid, &resolution.env, &resolution.names, snapshot)
}

fn closed(solid: &FamilySolidMesh) -> TriMesh {
    TriMesh { positions: solid.positions.chunks(3).map(|row| [row[0], row[1], row[2]]).collect(), normals: solid.normals.chunks(3).map(|row| [row[0], row[1], row[2]]).collect(), indices: solid.indices.chunks(3).map(|row| [row[0], row[1], row[2]]).collect() }
}

fn rigged() -> ModelSnapshot {
    let mut snapshot = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::model();
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::family(&mut snapshot, "fam", FamilyCategory::Generic);
    for (name, text) in [("w", "400 mm"), ("d", "300 mm"), ("h", "2 m"), ("r", "0.5 m")] {
        semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::parameter(&mut snapshot, "fam", name, ParameterKind::Length, text);
    }
    snapshot
}

#[test]
fn an_extrusion_has_the_volume_of_its_section_times_the_height_and_moves_with_its_offset() {
    let mut snapshot = rigged();
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::solid(&mut snapshot, "s", "fam", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::extrusion(semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::rectangle("w", "d"), "100 mm", "h"));
    snapshot.family_solids.get_mut("s").expect("s").offset = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::point3("1 m", "2 m", "0.5 m");
    let (result, issues) = evaluated(&snapshot, "s");
    assert!(issues.is_empty(), "{issues:?}");
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(result.mesh.volume, 0.4 * 0.3 * 2.0);
    let (lo, hi) = (result.mesh.bounds.min, result.mesh.bounds.max);
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(lo.x, 1.0 - 0.2);
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(hi.y, 2.0 + 0.15);
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(lo.z, 0.5 + 0.1);
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(hi.z, 0.5 + 0.1 + 2.0);
    assert!(closed(&result.mesh).is_watertight());
    assert_eq!(result.section.as_ref().map(Vec::len), Some(4));
    assert_eq!(result.offset, [1.0, 2.0, 0.5]);
    assert!(result.mesh.visible);
    assert_eq!(result.mesh.material, "m-steel");
}

#[test]
fn every_profile_kind_extrudes_to_its_closed_form_area() {
    let mut snapshot = rigged();
    let polygon = ParametricProfile::Polygon { points: vec![kit::point("0 m", "0 m"), kit::point("w", "0 m"), kit::point("0 m", "d")] };
    let circle = ParametricProfile::Circle { diameter: "2 * r".into() };
    let i_shape = ParametricProfile::IShape { width: "w".into(), depth: "d".into(), web: "20 mm".into(), flange: "30 mm".into() };
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::solid(&mut snapshot, "tri", "fam", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::extrusion(polygon, "0 m", "1 m"));
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::solid(&mut snapshot, "round", "fam", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::extrusion(circle, "0 m", "1 m"));
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::solid(&mut snapshot, "ibeam", "fam", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::extrusion(i_shape, "0 m", "1 m"));
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(evaluated(&snapshot, "tri").0.mesh.volume, 0.5 * 0.4 * 0.3);
    let round = evaluated(&snapshot, "round").0.mesh.volume;
    assert!((round - PI * 0.25).abs() < 1e-3 * PI * 0.25, "{round}");
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(evaluated(&snapshot, "ibeam").0.mesh.volume, 2.0 * 0.4 * 0.03 + (0.3 - 0.06) * 0.02);
    let reversed = ParametricProfile::Polygon { points: vec![kit::point("0 m", "d"), kit::point("w", "0 m"), kit::point("0 m", "0 m")] };
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::solid(&mut snapshot, "clockwise", "fam", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::extrusion(reversed, "0 m", "1 m"));
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(evaluated(&snapshot, "clockwise").0.mesh.volume, 0.5 * 0.4 * 0.3);
}

#[test]
fn a_cuboid_is_placed_by_its_corner() {
    let mut snapshot = rigged();
    let shape = SolidShape::Cuboid { x: "1 m".into(), y: "-1 m".into(), z: "0.25 m".into(), width: "w".into(), depth: "d".into(), height: "h".into() };
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::solid(&mut snapshot, "box", "fam", shape);
    let (result, issues) = evaluated(&snapshot, "box");
    assert!(issues.is_empty());
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(result.mesh.volume, 0.4 * 0.3 * 2.0);
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(result.mesh.bounds.min.x, 1.0);
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(result.mesh.bounds.min.y, -1.0);
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(result.mesh.bounds.min.z, 0.25);
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(result.mesh.bounds.max.z, 2.25);
    assert!(result.section.is_none());
}

#[test]
fn a_sweep_runs_the_section_along_the_plan_path() {
    let mut snapshot = rigged();
    let path = vec![kit::point("0 m", "0 m"), kit::point("3 m", "0 m")];
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::solid(&mut snapshot, "rail", "fam", SolidShape::Sweep { profile: semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::rectangle("w", "d"), path });
    snapshot.family_solids.get_mut("rail").expect("rail").offset = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::point3("0 m", "0 m", "1 m");
    let (result, issues) = evaluated(&snapshot, "rail");
    assert!(issues.is_empty(), "{issues:?}");
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(result.mesh.volume, 0.4 * 0.3 * 3.0);
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(result.mesh.bounds.min.z, 1.0 - 0.15);
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(result.mesh.bounds.max.z, 1.0 + 0.15);
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(result.mesh.bounds.max.x, 3.0);
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::close(result.mesh.bounds.max.y, 0.2);
    let degenerate = vec![kit::point("1 m", "1 m"), kit::point("1 m", "1 m")];
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::solid(&mut snapshot, "dot", "fam", SolidShape::Sweep { profile: semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::rectangle("w", "d"), path: degenerate });
    let (empty, issues) = evaluated(&snapshot, "dot");
    assert!(empty.mesh.indices.is_empty());
    assert_eq!(issues[0].code, FamilyIssueCode::Outline);
}

#[test]
fn a_revolution_about_each_axis_is_a_cylinder_of_the_same_volume_pointing_along_the_axis() {
    let mut snapshot = rigged();
    let profile = ParametricProfile::Polygon { points: vec![kit::point("0 m", "0 m"), kit::point("r", "0 m"), kit::point("r", "h"), kit::point("0 m", "h")] };
    for (axis, extent) in [(SolidAxis::Z, [1.0, 1.0, 2.0]), (SolidAxis::X, [2.0, 1.0, 1.0]), (SolidAxis::Y, [1.0, 2.0, 1.0])] {
        let id = format!("turned-{axis:?}");
        semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::solid(&mut snapshot, &id, "fam", SolidShape::Revolution { profile: profile.clone(), axis, angle: "360 deg".into() });
        let (result, issues) = evaluated(&snapshot, &id);
        assert!(issues.is_empty(), "{issues:?}");
        let expected = PI * 0.25 * 2.0;
        assert!((result.mesh.volume - expected).abs() < 1e-3 * expected, "{axis:?}: {}", result.mesh.volume);
        let (lo, hi) = (result.mesh.bounds.min, result.mesh.bounds.max);
        let got = [hi.x - lo.x, hi.y - lo.y, hi.z - lo.z];
        for (index, want) in extent.iter().enumerate() {
            assert!((got[index] - want).abs() < 1e-3, "{axis:?} axis {index}: {got:?}");
        }
        assert!(closed(&result.mesh).is_watertight());
    }
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::solid(&mut snapshot, "quarter", "fam", SolidShape::Revolution { profile, axis: SolidAxis::Z, angle: "90 deg".into() });
    let (quarter, issues) = evaluated(&snapshot, "quarter");
    assert!(issues.is_empty());
    assert!((quarter.mesh.volume - PI * 0.25 * 2.0 / 4.0).abs() < 1e-3);
}

#[test]
fn faults_become_issues_and_leave_the_solid_empty() {
    let mut snapshot = rigged();
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::parameter(&mut snapshot, "fam", "broken", ParameterKind::Length, "(");
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::parameter(&mut snapshot, "fam", "negative", ParameterKind::Length, "-1 m");
    let case = |snapshot: &mut ModelSnapshot, id: &str, shape: SolidShape| {
        semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::solid(snapshot, id, "fam", shape);
        evaluated(snapshot, id)
    };
    let (result, issues) = case(&mut snapshot, "flat", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::extrusion(semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::rectangle("w", "d"), "0 m", "negative"));
    assert!(result.mesh.indices.is_empty());
    assert_eq!((issues[0].code, issues[0].field.as_str()), (FamilyIssueCode::Negative, "height"));
    let (_, issues) = case(&mut snapshot, "ghost", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::extrusion(semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::rectangle("ghost", "d"), "0 m", "h"));
    assert_eq!((issues[0].code, issues[0].field.as_str(), issues[0].names.clone()), (FamilyIssueCode::Unknown, "profile.width", vec!["ghost".to_string()]));
    let (_, issues) = case(&mut snapshot, "late", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::extrusion(semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::rectangle("broken", "d"), "0 m", "h"));
    assert_eq!(issues[0].code, FamilyIssueCode::Dependency);
    let (_, issues) = case(&mut snapshot, "angle", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::extrusion(semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::rectangle("w", "d"), "0 m", "30 deg"));
    assert_eq!((issues[0].code, issues[0].field.as_str()), (FamilyIssueCode::Kind, "height"));
    let (_, issues) = case(&mut snapshot, "plain-zero", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::extrusion(semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::rectangle("w", "d"), "0 m", "1 m / 0"));
    assert_eq!(issues[0].code, FamilyIssueCode::DivisionByZero);
    let (_, issues) = case(&mut snapshot, "zero", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::extrusion(semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::rectangle("w", "d"), "0 m", "h / (1 m - 1 m)"));
    assert_eq!(issues[0].code, FamilyIssueCode::DivisionByZero);
    let thin = ParametricProfile::IShape { width: "w".into(), depth: "d".into(), web: "w".into(), flange: "30 mm".into() };
    let (_, issues) = case(&mut snapshot, "web", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::extrusion(thin, "0 m", "1 m"));
    assert_eq!(issues[0].code, FamilyIssueCode::Outline);
    let bowtie = ParametricProfile::Polygon { points: vec![kit::point("0 m", "0 m"), kit::point("1 m", "1 m"), kit::point("1 m", "0 m"), kit::point("0 m", "1 m")] };
    let (_, issues) = case(&mut snapshot, "bowtie", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::extrusion(bowtie, "0 m", "1 m"));
    assert_eq!(issues[0].code, FamilyIssueCode::Outline);
    let two = ParametricProfile::Polygon { points: vec![kit::point("0 m", "0 m"), kit::point("1 m", "1 m")] };
    let (_, issues) = case(&mut snapshot, "line", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::extrusion(two, "0 m", "1 m"));
    assert_eq!(issues[0].code, FamilyIssueCode::Outline);
    let centred = SolidShape::Revolution { profile: semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::rectangle("w", "d"), axis: SolidAxis::Z, angle: "90 deg".into() };
    let (_, issues) = case(&mut snapshot, "crossing", centred);
    assert_eq!(issues[0].code, FamilyIssueCode::Outline);
    let full = SolidShape::Revolution { profile: ParametricProfile::Polygon { points: vec![kit::point("0 m", "0 m"), kit::point("r", "0 m"), kit::point("r", "h")] }, axis: SolidAxis::Z, angle: "400 deg".into() };
    let (_, issues) = case(&mut snapshot, "overturned", full);
    assert_eq!((issues[0].code, issues[0].field.as_str()), (FamilyIssueCode::Negative, "angle"));
}

#[test]
fn visibility_and_material_formulas_are_evaluated() {
    let mut snapshot = rigged();
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::parameter(&mut snapshot, "fam", "shown", ParameterKind::Boolean, "w > 300 mm");
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::parameter(&mut snapshot, "fam", "finish", ParameterKind::Material, "\"m-steel\"");
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::solid(&mut snapshot, "s", "fam", semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::extrusion(semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::tests::kit::rectangle("w", "d"), "0 m", "h"));
    snapshot.family_solids.get_mut("s").expect("s").visible = "shown".into();
    snapshot.family_solids.get_mut("s").expect("s").material = "finish".into();
    let (result, issues) = evaluated(&snapshot, "s");
    assert!(issues.is_empty(), "{issues:?}");
    assert!(result.mesh.visible);
    assert_eq!(result.mesh.material, "m-steel");
    snapshot.family_parameters.get_mut("fam.w").expect("w").value = "200 mm".into();
    assert!(!evaluated(&snapshot, "s").0.mesh.visible);
    snapshot.family_solids.get_mut("s").expect("s").material = "\"m-ghost\"".into();
    let (result, issues) = evaluated(&snapshot, "s");
    assert_eq!(issues[0].code, FamilyIssueCode::Unknown);
    assert_eq!(result.mesh.material, "m-ghost", "the solid keeps the id, the issue says it is unknown");
    snapshot.family_solids.get_mut("s").expect("s").visible = "1 m".into();
    assert_eq!(evaluated(&snapshot, "s").1.iter().find(|issue| issue.field == "visible").map(|issue| issue.code), Some(FamilyIssueCode::Kind));
}
