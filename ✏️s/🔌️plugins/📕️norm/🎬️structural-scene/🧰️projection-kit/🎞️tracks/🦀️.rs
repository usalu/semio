//! 🎞️ Tracks: constructors for the parameter tracks every scene offers its viewport (fire time, concrete age, mode index, combination index, oscillation phase) and the field names they bind.

use crate::*;

/// 🔖️ One keyframe at parameter value `at`, showing `field` and revealing `layer`.
pub fn key(at: f64, caption: (&str, &str), field: Option<&str>, layer: Option<&str>) -> Keyframe {
    Keyframe { at, caption: LocalizedCopy::new(caption.0, caption.1), field: field.map(str::to_string), layer: layer.map(str::to_string) }
}

/// ⏳️ A track over `min..=max` in `step`s; keyframes must ascend inside the range.
pub fn track(id: &str, kind: TrackKind, label: (&str, &str), unit: QuantityKind, range: (f64, f64, f64), keyframes: Vec<Keyframe>) -> ParameterTrack {
    ParameterTrack { id: id.into(), kind, label: LocalizedCopy::new(label.0, label.1), unit, min: range.0, max: range.1, step: range.2, looped: false, keyframes }
}

/// 🔢️ A snapping index track (modes, combinations): keyframes sit at `1..=n` with `(English, German, field, layer)` stops; `None` below two stops, since nothing can be scrubbed.
pub fn indexed(id: &str, kind: TrackKind, label: (&str, &str), stops: Vec<(String, String, Option<String>, Option<String>)>) -> Option<ParameterTrack> {
    (stops.len() >= 2).then(|| {
        let keyframes = stops.iter().enumerate().map(|(i, (en, de, field, layer))| key((i + 1) as f64, (en, de), field.as_deref(), layer.as_deref())).collect();
        track(id, kind, label, QuantityKind::Dimensionless, (1.0, stops.len() as f64, 1.0), keyframes)
    })
}

/// ⏱️ A time or age track whose keyframes blend linearly; `stops` are ascending `(seconds, field, English, German)` rows, the range spans first to last in the finest spacing.
pub fn timed(id: &str, kind: TrackKind, label: (&str, &str), stops: &[(f64, &str, &str, &str)]) -> Option<ParameterTrack> {
    (stops.len() >= 2 && stops.windows(2).all(|w| w[0].0 < w[1].0)).then(|| {
        let step = stops.windows(2).map(|w| w[1].0 - w[0].0).fold(f64::MAX, f64::min);
        let keyframes = stops.iter().map(|&(at, field, en, de)| key(at, (en, de), Some(field), None)).collect();
        track(id, kind, label, QuantityKind::Time, (stops[0].0, stops[stops.len() - 1].0, step), keyframes)
    })
}

/// 🌀️ The oscillation phase track: `0..1` per cycle, looped, no keyframes.
pub fn phase() -> ParameterTrack {
    ParameterTrack { looped: true, ..track("phase", TrackKind::Phase, ("Oscillation phase", "Schwingungsphase"), QuantityKind::Dimensionless, (0.0, 1.0, 0.05), Vec::new()) }
}

/// 🌀️ The mode-index track over the mode layers in use: one stop per `(layer, English, German)` row of `modes` that holds an entity, each revealing its own layer; `None` below two modes.
pub fn mode_track(entities: &[SceneEntity], modes: &[(&str, &str, &str)]) -> Option<ParameterTrack> {
    let stops = modes.iter().filter(|(layer, ..)| entities.iter().any(|e| e.layer == *layer)).map(|&(layer, en, de)| (en.to_string(), de.to_string(), None, Some(layer.to_string()))).collect();
    indexed("mode", TrackKind::Mode, ("Buckling mode", "Beulform"), stops)
}

/// 🔖️ The id of the combination index track every scene with combinations offers; the view-step layer finds it by [`TrackKind::Combination`].
pub const COMBINATION_TRACK: &str = "combination";

/// 🧮️ The combination index track over ordered `(English, German)` captions with duplicate English captions removed (first wins); `None` below two distinct combinations, since there is nothing to step through.
pub fn combination_track(captions: impl IntoIterator<Item = (String, String)>) -> Option<ParameterTrack> {
    let mut seen = std::collections::BTreeSet::new();
    let stops = captions.into_iter().filter(|(en, _)| seen.insert(en.clone())).map(|(en, de)| (en, de, None, None)).collect();
    indexed(COMBINATION_TRACK, TrackKind::Combination, ("Combination", "Kombination"), stops)
}

/// 🎞️ Adds the [`combination_track`] of `captions` to `scene` unless the scene already offers a combination track; the shared hook of every artifact whose checks run per combination.
pub fn push_combination_track(scene: &mut StructuralScene, captions: impl IntoIterator<Item = (String, String)>) {
    if scene.tracks.iter().any(|track| matches!(track.kind, TrackKind::Combination)) {
        return;
    }
    scene.tracks.extend(combination_track(captions));
}
