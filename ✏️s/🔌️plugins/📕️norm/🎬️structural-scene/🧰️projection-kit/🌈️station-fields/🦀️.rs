//! 🌈️ Station fields: turn the sampled [`norm::Station`]s of a check into scene scalars (per-vertex fields, per-segment colours, per-station binding values).

use crate::*;
use norm::CheckResult;

/// 📍️ The member stations of a check as ascending, unique scene [`StationValue`]s (largest utilisation wins on equal positions); empty for 2D/3D fields.
pub fn station_values(c: &CheckResult) -> Vec<StationValue> {
    let mut out: Vec<StationValue> = c.stations.iter().filter(|s| s.at.len() == 1 && s.is_well_formed()).map(|s| StationValue { at: s.at[0], value: s.utilization }).collect();
    out.sort_by(|a, b| a.at.total_cmp(&b.at));
    out.dedup_by(|later, kept| {
        let same = later.at == kept.at;
        if same {
            kept.value = kept.value.max(later.value);
        }
        same
    });
    out
}

fn lerp(stations: &[StationValue], t: f64) -> f64 {
    match stations {
        [] => 0.0,
        [only] => only.value,
        _ => {
            let i = stations.partition_point(|s| s.at <= t);
            if i == 0 {
                stations[0].value
            } else if i == stations.len() {
                stations[i - 1].value
            } else {
                let (a, b) = (&stations[i - 1], &stations[i]);
                a.value + (b.value - a.value) * (t - a.at) / (b.at - a.at)
            }
        }
    }
}

/// 🌈️ Utilisation interpolated linearly at each member coordinate `params` (per-vertex scalar field along a member; clamped to the end stations).
pub fn field_along(c: &CheckResult, params: &[f64]) -> Vec<f64> {
    let stations = station_values(c);
    params.iter().map(|&t| lerp(&stations, t)).collect()
}

/// 🎨️ Utilisation of each of `n` equal member segments: the maximum of the interpolated end values and every station inside (conservative colour per segment).
pub fn segment_values(c: &CheckResult, n: usize) -> Vec<f64> {
    let stations = station_values(c);
    let n = n.max(1);
    (0..n)
        .map(|i| {
            let (a, b) = (i as f64 / n as f64, (i + 1) as f64 / n as f64);
            stations.iter().filter(|s| s.at > a && s.at < b).map(|s| s.value).fold(lerp(&stations, a).max(lerp(&stations, b)), f64::max)
        })
        .collect()
}

/// 🗺️ Utilisation at plate or solid points by inverse-distance weighting (power 2) of the stations over their own dimension; an exact hit returns the station value, a member check interpolates along the point's first coordinate.
pub fn field_at_points(c: &CheckResult, points: &[[f64; 3]]) -> Vec<f64> {
    let dim = c.stations.first().map_or(0, |s| s.at.len());
    if dim == 1 {
        return points.iter().map(|p| p[0]).map(|t| lerp(&station_values(c), t)).collect();
    }
    points
        .iter()
        .map(|p| {
            let mut num = 0.0;
            let mut den = 0.0;
            for s in c.stations.iter().filter(|s| s.at.len() == dim && s.is_well_formed()) {
                let d2: f64 = s.at.iter().zip(p).map(|(a, b)| (a - b) * (a - b)).sum();
                if d2 < 1e-24 {
                    return s.utilization;
                }
                num += s.utilization / d2;
                den += 1.0 / d2;
            }
            if den > 0.0 { num / den } else { 0.0 }
        })
        .collect()
}

/// 🏔️ Upper envelope of the member stations of several checks: the union of their positions with the largest interpolated utilisation at each (empty when no check has member stations).
pub fn envelope<'a>(checks: impl IntoIterator<Item = &'a CheckResult>) -> Vec<StationValue> {
    let sets: Vec<Vec<StationValue>> = checks.into_iter().map(station_values).filter(|v| !v.is_empty()).collect();
    let mut at: Vec<f64> = sets.iter().flatten().map(|s| s.at).collect();
    at.sort_by(f64::total_cmp);
    at.dedup();
    at.into_iter().map(|t| StationValue { at: t, value: sets.iter().map(|s| lerp(s, t)).fold(0.0, f64::max) }).collect()
}
