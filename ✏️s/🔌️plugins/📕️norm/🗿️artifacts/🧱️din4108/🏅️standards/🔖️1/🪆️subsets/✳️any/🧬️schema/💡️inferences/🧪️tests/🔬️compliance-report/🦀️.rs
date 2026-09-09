use super::*;

fn sample_wall() -> Vec<part_2::Layer> {
    vec![part_2::Layer { thickness_m: 0.24, lambda_w_mk: 0.81 }, part_2::Layer { thickness_m: 0.14, lambda_w_mk: 0.035 }]
}

fn sample_moisture_wall() -> Vec<part_3::MoistureLayer> {
    vec![part_3::MoistureLayer { thickness_m: 0.24, lambda_w_mk: 0.81, mu: 15.0 }, part_3::MoistureLayer { thickness_m: 0.14, lambda_w_mk: 0.035, mu: 1.3 }]
}

#[semio_framework_async_macros::async_test]
async fn opaque_wall_passes_din_4108_suite() {
    let report = check_opaque_wall(part_2::BuildingCategory::Residential, &sample_wall(), ClimateZoneDe::Zone2, 2.5).expect("inputs complete");
    assert!(report.all_pass(), "checks: {:?}", report.checks);
}

#[semio_framework_async_macros::async_test]
async fn full_envelope_evaluate_covers_all_eight_parts() {
    let document = Din4108Snapshot::default();
    let report = evaluate(&document);
    assert!(report.checks.len() >= 15, "expected parts 1–8 checks, got {}", report.checks.len());
    assert!(report.all_pass(), "checks: {:?}", report.checks);
    let f_dry = part_3::interior_surface_temperature_factor(&sample_moisture_wall(), R_SI_WALL_M2K_W, R_SE_WALL_M2K_W, 20.0, -14.0, 0.5);
    let f_humid = part_3::interior_surface_temperature_factor(&sample_moisture_wall(), R_SI_WALL_M2K_W, R_SE_WALL_M2K_W, 20.0, -14.0, 0.8);
    assert!(f_humid < f_dry, "humidity correction must reduce f_Rsi");
}
