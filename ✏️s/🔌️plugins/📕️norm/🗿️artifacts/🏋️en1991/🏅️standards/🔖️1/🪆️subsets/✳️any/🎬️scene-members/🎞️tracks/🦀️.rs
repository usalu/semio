//! 🎞️ EN 1991 tracks: the standard-fire time track owned by the fire entity (gas temperature field of the compartment and its gas glyph, EN 1991-1-2) and the traffic position track, added to the merged scene by the kit `finish` from the tandem layers its entities use (the LM1 tandem stepped along the bridge deck, EN 1991-2).

use super::kit::{projection_kit::tracks, Out, TANDEM_STOPS};
use super::model::En1991Snapshot;
use super::scene::*;
use super::thermal_fire::gas_temperature_k;

pub const TRACK_FIRE: &str = "fire-time";
pub const TRACK_TRAFFIC: &str = "traffic-position";
const FIRE_STOPS_MIN: [f64; 9] = [0.0, 5.0, 10.0, 15.0, 30.0, 45.0, 60.0, 90.0, 120.0];

/// 🔖️ The field id of the gas temperature at `minutes` of fire.
pub fn fire_field(minutes: f64) -> String {
    format!("temperature@{minutes}")
}

/// ⏱️ The stops of the fire track in minutes: the standard grid up to the design duration, closed by the design duration itself.
pub fn fire_stops(duration_s: f64) -> Vec<f64> {
    let design = duration_s / 60.0;
    let mut stops: Vec<f64> = FIRE_STOPS_MIN.iter().copied().filter(|m| *m < design - 1e-9).collect();
    stops.push(design);
    stops
}

pub fn fire(d: &En1991Snapshot, out: &mut Out) {
    let stops: Vec<(f64, f64)> = fire_stops(d.fire_duration).into_iter().filter_map(|m| gas_temperature_k(&En1991Snapshot { fire_duration: m * 60.0, ..d.clone() }).filter(|k| k.is_finite()).map(|k| (m, k - 273.15))).collect();
    let rows: Vec<(f64, String, String, String)> = stops.iter().map(|(m, _)| (m * 60.0, fire_field(*m), format!("{m} min"), format!("{} min", format!("{m}").replace('.', ",")))).collect();
    let borrowed: Vec<(f64, &str, &str, &str)> = rows.iter().map(|(t, f, en, de)| (*t, f.as_str(), en.as_str(), de.as_str())).collect();
    let Some(track) = tracks::timed(TRACK_FIRE, TrackKind::Time, ("Fire duration", "Branddauer"), &borrowed) else { return };
    let mut touched = false;
    out.entities = std::mem::take(&mut out.entities).into_iter().map(|mut e| {
        if e.entity.id == "fire" || e.entity.id == "fire:gas" {
            let mut fields: Vec<ScalarField> = stops.iter().map(|(m, c)| ScalarField { id: fire_field(*m), unit: QuantityKind::Temperature, samples: FieldSamples::Instance { values: vec![*c] } }).collect();
            let mut active = fire_field(stops[stops.len() - 1].0);
            if let ColourChannel::Fields { active: kept, fields: own } = &e.appearance.channel {
                active = kept.clone();
                let mut all = own.clone();
                all.append(&mut fields);
                fields = all;
            }
            e.appearance.channel = ColourChannel::Fields { active, fields };
            e = e.sealed();
            touched = true;
        }
        e
    }).collect();
    if touched {
        out.track(track);
    }
}

pub fn traffic(entities: &[SceneEntity]) -> Option<ParameterTrack> {
    let used = |layer: &str| entities.iter().any(|e| e.layer == layer);
    let stops: Vec<_> = TANDEM_STOPS.iter().filter(|s| used(s.1)).map(|s| (s.2.to_string(), s.3.to_string(), None, Some(s.1.to_string()))).collect();
    tracks::indexed(TRACK_TRAFFIC, TrackKind::Combination, ("Tandem position", "Tandemposition"), stops)
}
