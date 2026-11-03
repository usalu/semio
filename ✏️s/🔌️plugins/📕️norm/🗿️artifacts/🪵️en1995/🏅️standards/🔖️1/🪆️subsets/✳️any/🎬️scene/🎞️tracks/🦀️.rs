//! 🎞️ Tracks: the fire-time track of the EN 1995 scene; every keyframe (every 15 minutes up to two hours) reveals the char layer, its glowing front and the zero-strength layer of EN 1995-1-2 at that duration (`d_char = β_n t`), so playing the track grows the char front inward.

use super::facts;
use super::kit::*;
use super::layout::Layout;
use super::members::fire_layers;
use super::model::{TimberMember, TimberProduct};
use super::scene::projection_kit::tracks as kit_tracks;
use super::scene::*;
use super::SceneInput;

pub use entities::{FIRE_LAYERS, FIRE_MINUTES};

/// 🔥️ Char and zero-strength layer entities of one member for every keyframe duration, each on the layer of its keyframe.
pub(crate) fn char_growth(input: &SceneInput, layout: &Layout, member: &TimberMember, product: TimberProduct, frame: &Frame, length: f64, out: &mut Out) {
    for (minutes, layer) in FIRE_MINUTES.iter().zip(FIRE_LAYERS) {
        let fire = facts::fire_after(input.document.annex, product, member.b_m, member.h_m, f64::from(*minutes) * 60.0);
        let slot = format!("@{minutes}");
        fire_layers(input, layout, member, frame, length, &fire, (layer, &slot), out);
    }
}

/// 🎬️ Adds the `fire` track to the merged scene once any entity grows a char layer; it scrubs layers of members and catalogue cases alike, so it belongs to no single entity.
pub(crate) fn finish(scene: &mut StructuralScene) {
    if !scene.entities.iter().any(|e| e.layer == FIRE_LAYERS[0]) {
        return;
    }
    let keyframes = FIRE_MINUTES.iter().zip(FIRE_LAYERS).map(|(m, layer)| kit_tracks::key(f64::from(*m) * 60.0, (&format!("R{m}"), &format!("R{m}")), None, Some(layer))).collect();
    scene.tracks.push(kit_tracks::track("fire", TrackKind::Time, ("Fire exposure time", "Branddauer"), QuantityKind::Time, (900.0, 7200.0, 900.0), keyframes));
}
