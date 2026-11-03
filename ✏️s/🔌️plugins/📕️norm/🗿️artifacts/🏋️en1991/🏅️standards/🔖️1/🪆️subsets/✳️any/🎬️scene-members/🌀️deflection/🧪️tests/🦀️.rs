use super::*;

fn peak(curve: &Curve) -> f64 {
    curve.iter().map(|(_, u)| u[2].abs()).fold(0.0, f64::max)
}

fn at(curve: &Curve, t: f64) -> f64 {
    curve.iter().min_by(|a, b| (a.0 - t).abs().total_cmp(&(b.0 - t).abs())).map(|(_, u)| u[2].abs()).expect("sample")
}

#[test]
fn the_cantilever_tip_drifts_one_five_hundredth_and_follows_the_closed_form() {
    let (w, l) = (8000.0, 12.0);
    let curve = line(l, cantilever_stiffness(w, l), Ends::Cantilever, w).expect("kernel");
    assert!((peak(&curve) - l / WIND_DRIFT_RATIO).abs() < 1e-6 * l);
    let ratio = at(&curve, 0.5) / peak(&curve);
    assert!((ratio - 0.354_166_666_7).abs() < 1e-3, "{ratio}");
}

#[test]
fn the_simply_supported_beam_sags_one_two_hundred_fiftieth_and_follows_the_closed_form() {
    let (w, l) = (1800.0, 9.0);
    let curve = line(l, beam_stiffness(w, l), Ends::Pinned, w).expect("kernel");
    assert!((peak(&curve) - l / SNOW_SAG_RATIO).abs() < 1e-6 * l);
    let ratio = at(&curve, 0.25) / peak(&curve);
    assert!((ratio - 0.7125).abs() < 1e-3, "{ratio}");
}

#[test]
fn the_default_site_loads_are_positive() {
    let d = En1991Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    assert!(wind_line_load(&d).is_some_and(|w| w > 0.0));
    assert!(snow_line_load(&d).is_some_and(|s| s > 0.0));
}
