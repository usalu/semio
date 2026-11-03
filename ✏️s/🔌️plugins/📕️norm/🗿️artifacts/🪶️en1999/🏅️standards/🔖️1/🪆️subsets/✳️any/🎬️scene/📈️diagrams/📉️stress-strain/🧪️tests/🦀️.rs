use super::*;

fn material(designation: &str) -> AluminiumMaterial {
    AluminiumMaterial { id: "alu".into(), designation: designation.into(), product_form: super::super::super::model::ProductForm::ExtrudedProfile }
}

#[test]
fn the_curve_is_elastic_to_the_yield_strain_then_flat_and_the_proof_point_adds_the_offset() {
    let points = curve(250.0e6);
    assert_eq!(points.len(), 3);
    assert!((points[1][0] - 250.0e6 / 70_000.0e6 * 100.0).abs() < 1e-12 && points[1][1] == 250.0);
    assert_eq!((points[2][0], points[2][1]), (2.0, 250.0));
    assert!((proof_point(250.0e6)[0] - (250.0 / 70_000.0 + 0.002) * 100.0).abs() < 1e-12);
}

#[test]
fn a_welded_alloy_adds_the_haz_reduced_curve_and_a_non_reducing_alloy_does_not() {
    let reduced = stress_strain(&material("aw6082-t6")).unwrap();
    let ids: Vec<&str> = reduced.series.iter().map(|s| match s { Series::Polyline { id, .. } | Series::Limit { id, .. } => id.as_str(), _ => "" }).collect();
    assert_eq!(ids, ["parent", "f-u", "haz", "f-u-haz"]);
    assert_eq!((reduced.markers.len(), reduced.table_twin.columns.len()), (3, 4));
    let alloy = part_1_1::resolve_alloy("aw6082-t6").unwrap();
    assert!((reduced.markers[2].at.y - alloy.f_o_pa * alloy.rho_o_haz / 1.0e6).abs() < 1e-9);
    let plain = stress_strain(&material("aw5083-o")).unwrap();
    assert_eq!((plain.series.len(), plain.markers.len(), plain.table_twin.columns.len()), (2, 2, 2));
    assert!(stress_strain(&material("nonsense")).is_none());
}
