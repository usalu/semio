//! 🎞️ Tracks of the EN 1997 scene: Terzaghi consolidation time of the settlement trough, scrubbed from the undeformed ground to the final settlement; every stop colours the trough by its settlement field and reveals the trough of that stage on its own layer, so the trough itself deepens while the slider moves.

use super::kit::{LAYER_SETTLEMENT, STAGE_LAYERS};
use super::model::{LabTest, LabTestKind, SpreadFoundation};
use super::scene::projection_kit::tracks as build;
use super::scene::*;

pub const PERMEABILITY: f64 = 1.0e-9;
pub const WATER_UNIT_WEIGHT: f64 = 9810.0;
pub const DAY: f64 = 86_400.0;
/// 📐️ The Terzaghi time factor at 90 % consolidation, the root of the exact series shared with the catalogue (SLS-02, GI-10).
pub fn t90() -> f64 {
    super::checks::catalogue::settlement::t90_factor()
}
pub const TIME_FACTORS: [f64; 7] = [0.0, 0.05, 0.1, 0.197, 0.4, 0.8, 1.5];

/// 📈️ Average degree of consolidation `U(Tv) = 1 - sum 8 / ((2m+1)^2 pi^2) exp(-(2m+1)^2 pi^2 Tv / 4)` of a layer drained on one side.
pub fn degree(tv: f64) -> f64 {
    if tv <= 0.0 {
        return 0.0;
    }
    let pi2 = std::f64::consts::PI.powi(2);
    1.0 - (0..50).map(|m| { let k = f64::from(2 * m + 1); 8.0 / (k * k * pi2) * (-k * k * pi2 * tv / 4.0).exp() }).sum::<f64>()
}

/// 🧪️ The governing laboratory consolidation coefficient `cv = T90 h² / t90` in m²/s with the id of its test: the smallest over the oedometer series that state a drainage height and a t90, since the slowest layer governs the time; `None` without such a series.
pub fn laboratory_cv(tests: &[LabTest]) -> Option<(f64, String)> {
    tests.iter().filter(|t| t.kind == LabTestKind::Oedometer && t.drainage_height > 0.0 && t.consolidation_time > 0.0).map(|t| (t90() * t.drainage_height * t.drainage_height / t.consolidation_time, t.id.clone())).min_by(|a, b| a.0.total_cmp(&b.0))
}

/// ⏱️ Consolidation coefficient in m2/s and the drainage path of the compressed zone of `footing` (twice its larger side, drained upward): the laboratory `lab_cv` when an oedometer series states one, else `cv = k M / gamma_w` for the mean oedometric modulus of the layers and the nominal clay permeability; `None` without a compressible layer.
pub fn consolidation(footing: &SpreadFoundation, moduli: &[f64], lab_cv: Option<f64>) -> Option<(f64, f64)> {
    let m: Vec<f64> = moduli.iter().copied().filter(|m| *m > 0.0).collect();
    (!m.is_empty()).then(|| (lab_cv.unwrap_or_else(|| PERMEABILITY * m.iter().sum::<f64>() / m.len() as f64 / WATER_UNIT_WEIGHT), 2.0 * footing.width.max(footing.length)))
}

/// 🕰️ Whole days at time factor `tv`.
pub fn days(tv: f64, cv: f64, path: f64) -> f64 {
    (tv * path * path / cv / DAY).round()
}

/// 🕰️ The consolidation stops `(days, degree of consolidation)` over the time factors, one per distinct whole day; `None` without a compressible layer.
pub fn stops(footing: &SpreadFoundation, moduli: &[f64], lab_cv: Option<f64>) -> Option<Vec<(f64, f64)>> {
    let (cv, path) = consolidation(footing, moduli, lab_cv)?;
    let mut stops: Vec<(f64, f64)> = Vec::new();
    for tv in TIME_FACTORS {
        let d = days(tv, cv, path);
        if stops.last().is_none_or(|(last, _)| d > *last) {
            stops.push((d, degree(tv)));
        }
    }
    Some(stops)
}

/// 🆔️ The scalar field id of every stop; the last stop is the final `settlement` field.
pub fn field_ids(stops: &[(f64, f64)]) -> Vec<String> {
    let last = stops.len().saturating_sub(1);
    stops.iter().enumerate().map(|(i, (d, _))| if i == last { "settlement".to_string() } else { format!("settlement@{d:.0}d") }).collect()
}

/// 🎞️ The per-vertex settlement fields at each time factor and the consolidation track over them; the last stop is the final `settlement` field on the settlement layer and every earlier stop reveals its stage layer ([`STAGE_LAYERS`]).
pub fn consolidation_track(footing: &SpreadFoundation, moduli: &[f64], depths: &[f64], lab_cv: Option<f64>) -> Option<(Vec<ScalarField>, ParameterTrack)> {
    let stops = stops(footing, moduli, lab_cv)?;
    let last = stops.len() - 1;
    let ids = field_ids(&stops);
    let fields: Vec<ScalarField> = stops.iter().zip(&ids).take(last).map(|((_, u), id)| ScalarField { id: id.clone(), unit: QuantityKind::Length, samples: FieldSamples::Vertex { values: depths.iter().map(|s| s * u).collect() } }).collect();
    let rows: Vec<(f64, &str, String, String)> = stops.iter().zip(&ids).map(|((d, _), id)| (d * DAY, id.as_str(), format!("{d:.0} d"), format!("{d:.0} d"))).collect();
    let stops: Vec<(f64, &str, &str, &str)> = rows.iter().map(|(t, id, en, de)| (*t, *id, en.as_str(), de.as_str())).collect();
    build::timed("consolidation", TrackKind::Time, ("Consolidation time", "Konsolidationszeit"), &stops).map(|mut track| {
        for (i, keyframe) in track.keyframes.iter_mut().enumerate() {
            keyframe.layer = Some(if i == last { LAYER_SETTLEMENT } else { STAGE_LAYERS[i].0 }.to_string());
        }
        (fields, track)
    })
}
