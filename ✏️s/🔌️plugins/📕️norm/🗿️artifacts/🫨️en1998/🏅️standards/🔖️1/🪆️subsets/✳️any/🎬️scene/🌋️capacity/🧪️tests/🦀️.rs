use super::*;
use semio_s_artifact_norm_en1998_inferences::check_full_seismic;

fn fixture() -> (En1998Snapshot, CheckReport) {
    let document = En1998Snapshot::compliant_de_multipart();
    let report = check_full_seismic(&document);
    (document, report)
}

#[test]
fn the_pushover_plateau_is_the_overstrength_times_the_design_base_shear() {
    let (document, report) = fixture();
    let site = site(&document);
    let building = &document.buildings[0];
    let response = respond(&site, building);
    let system = &building.systems[0];
    let result = &response.systems[0];
    let mode = first_mode(&response, result.along_y).expect("first mode");
    let c = capacity(&site, system, result, mode).expect("capacity");
    assert!((c.plateau - system.alpha_u_over_alpha_1 * result.f_b).abs() <= 1e-9 * c.plateau);
    assert!((c.yield_displacement * c.stiffness - c.plateau).abs() <= 1e-9 * c.plateau);
    assert!(c.target_displacement >= mode.amplitude_m - 1e-12);
    if mode.period >= site.tc {
        assert_eq!(c.target_displacement, mode.amplitude_m);
    }
    let diagram = pushover(&document, &report, building).expect("pushover");
    assert_eq!(diagram.validate(), Ok(()));
    assert!(diagram.table_twin.rows() > 0);
    assert!(diagram.markers.iter().any(|m| m.bound_check.as_deref().is_some_and(|id| id.ends_with(".baseShear"))));
}

#[test]
fn the_q_chart_reproduces_the_behaviour_factor_formula() {
    let (document, report) = fixture();
    let building = &document.buildings[0];
    let diagram = q_chart(building, &report).expect("q chart");
    assert_eq!(diagram.validate(), Ok(()));
    let system = &building.systems[0];
    let q_max = part_1::q_max_for_system(system.system_type, system.material, system.ductility_class);
    let marker = diagram.markers.iter().find(|m| m.id == format!("q-now-{}", system.id)).expect("marker");
    let expected = (system.q0 * system.alpha_u_over_alpha_1 * system.k_w.clamp(0.5, 1.0)).max(1.5).min(q_max);
    assert!((marker.at.y - expected).abs() < 1e-12);
    assert!(marker.binding.is_some());
}

#[test]
fn the_storey_forces_sum_to_the_base_shear_without_torsion() {
    let (document, report) = fixture();
    let building = &document.buildings[0];
    let response = respond(&site(&document), building);
    for system in &response.systems {
        let sum: f64 = system.forces.iter().sum();
        assert!((sum - system.f_b * system.delta).abs() <= 1e-6 * system.f_b.max(1.0));
    }
    let diagram = force_profile(&document, &report, building).expect("profile");
    assert_eq!(diagram.validate(), Ok(()));
    assert!(diagram.table_twin.rows() == building.storeys.len());
}

#[test]
fn the_adrs_diagram_puts_the_performance_points_on_the_capacity_plateau_and_the_period_line_through_the_yield_point() {
    let (document, _) = fixture();
    let building = &document.buildings[0];
    let diagram = adrs(&document, building).expect("adrs");
    assert_eq!(diagram.validate(), Ok(()));
    let system = &building.systems[0].id;
    let marker = |id: String| &diagram.markers.iter().find(|m| m.id == id).unwrap_or_else(|| panic!("{id}")).at;
    let yield_point = marker(format!("yield-{system}"));
    for state in ["dl", "sd", "nc"] {
        assert!((marker(format!("target-{system}-{state}")).y - yield_point.y).abs() <= 1e-12 * yield_point.y.max(1.0), "{state} performance point sits on the plateau");
        let elastic = marker(format!("elastic-{system}-{state}"));
        assert!(elastic.x > 0.0 && elastic.y > 0.0);
        assert!(diagram.series.iter().any(|s| matches!(s, Series::Polyline { id, .. } if *id == format!("demand-{state}"))));
    }
    let period = diagram.series.iter().find_map(|s| match s { Series::Polyline { id, points, .. } if *id == format!("period-{system}") => Some(points.clone()), _ => None }).expect("period line");
    let slope = period[1].y / period[1].x;
    assert!((slope * yield_point.x - yield_point.y).abs() <= 1e-6 * yield_point.y, "the radial line T* passes through the yield point");
    assert_eq!(diagram.table_twin.rows(), 3 * diagram.markers.iter().filter(|m| m.id.starts_with("yield-")).count());
}
