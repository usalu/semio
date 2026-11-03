//! 🎞️ Tracks of the EN 1998 scene: the eigenmode index (each mode is a ghost slab set on its own layer, one shown at a time) and the oscillation phase.

use super::horizontal_elements::{DISTRIBUTION_FIELD, DISTRIBUTION_STEPS};
use super::kit::MODE_LAYERS;
use super::scene::projection_kit::tracks as build;
use super::scene::*;

/// 🌊️ Normalised shear-cantilever mode shape `sin((2n - 1) pi z / 2H)` at height ratio `z / H` for mode `n` (1-based).
pub fn cantilever_shape(n: u32, ratio: f64) -> f64 {
    (f64::from(2 * n - 1) * std::f64::consts::PI * ratio / 2.0).sin()
}

/// 🎞️ Adds the mode track over the mode layers in use, in table order, and the phase track; nothing when fewer than two modes were projected.
pub fn finish(scene: &mut StructuralScene) {
    let stops: Vec<(String, String, Option<String>, Option<String>)> = MODE_LAYERS.iter().filter(|(id, ..)| scene.entities.iter().any(|e| e.layer == *id)).map(|(id, en, de)| (en.to_string(), de.to_string(), None, Some(id.to_string()))).collect();
    if let Some(track) = build::indexed("mode", TrackKind::Mode, ("Eigenmode", "Eigenform"), stops) {
        scene.tracks.push(track);
        scene.tracks.push(build::phase());
    }
    distribution_track(scene);
}

/// 🎞️ The base shear build-up track: one stop per step of `DISTRIBUTION_STEPS`, each activating the scalar field `distribution-k` of the storey shear blocks, so playing it grows the shear diagram from the roof down to the base.
fn distribution_track(scene: &mut StructuralScene) {
    let first = format!("{DISTRIBUTION_FIELD}-0");
    if !scene.entities.iter().any(|e| matches!(&e.appearance.channel, ColourChannel::Fields { fields, .. } if fields.iter().any(|f| f.id == first))) {
        return;
    }
    let rows: Vec<(f64, String, String, String)> = (0..=DISTRIBUTION_STEPS)
        .map(|k| (k as f64, format!("{DISTRIBUTION_FIELD}-{k}"), if k == 0 { "Roof".to_string() } else { format!("{} % of the storeys", k * 100 / DISTRIBUTION_STEPS) }, if k == 0 { "Dach".to_string() } else { format!("{} % der Geschosse", k * 100 / DISTRIBUTION_STEPS) }))
        .collect();
    let stops: Vec<(f64, &str, &str, &str)> = rows.iter().map(|(t, field, en, de)| (*t, field.as_str(), en.as_str(), de.as_str())).collect();
    if let Some(track) = build::timed("base-shear", TrackKind::Time, ("Base shear build-up", "Aufbau der Gesamtquerkraft"), &stops) {
        scene.tracks.push(track);
    }
}
