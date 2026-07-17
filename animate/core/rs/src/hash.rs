//! 🪪 Content-hash animation descriptors via framework hash.

use framework_hash::{format_number_for_hash, hash_parts, merkle_node};
use serde::Serialize;

/// 🧾 Serializable animation fingerprint input.
#[derive(Clone, Debug, Serialize)]
pub struct AnimationHashInput {
    pub kind: String,
    pub run_time: f64,
    pub target_ids: Vec<u64>,
    pub rate: String,
    pub extras: Vec<String>,
}

impl AnimationHashInput {
    pub fn new(kind: impl Into<String>, run_time: f64) -> Self {
        Self {
            kind: kind.into(),
            run_time,
            target_ids: Vec::new(),
            rate: "linear".into(),
            extras: Vec::new(),
        }
    }

    pub fn with_targets(mut self, ids: Vec<u64>) -> Self {
        self.target_ids = ids;
        self
    }

    pub fn with_rate(mut self, rate: impl Into<String>) -> Self {
        self.rate = rate.into();
        self
    }

    pub fn with_extra(mut self, extra: impl Into<String>) -> Self {
        self.extras.push(extra.into());
        self
    }
}

/// 🔐 Hash a single animation descriptor.
pub fn hash_animation(input: &AnimationHashInput) -> String {
    let mut parts = vec![
        input.kind.clone(),
        format_number_for_hash(input.run_time),
        input.rate.clone(),
    ];
    for id in &input.target_ids {
        parts.push(id.to_string());
    }
    parts.extend(input.extras.clone());
    hash_parts(&parts)
}

/// 🌳 Merkle hash over an animation timeline.
pub fn hash_animation_timeline(children: Vec<String>) -> String {
    merkle_node(&["AnimateTimeline"], children)
}

/// 🎬 Hash a scene configuration snapshot.
pub fn hash_scene_config(frame_rate: f64, width: u32, height: u32, mobject_count: usize) -> String {
    hash_parts(&[
        "SceneConfig",
        format_number_for_hash(frame_rate),
        width.to_string(),
        height.to_string(),
        mobject_count.to_string(),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animation_hash_is_stable() {
        let input = AnimationHashInput::new("FadeIn", 1.0).with_targets(vec![42]);
        let a = hash_animation(&input);
        let b = hash_animation(&input);
        assert_eq!(a, b);
    }

    #[test]
    fn timeline_merkle_orders_children() {
        let h = hash_animation_timeline(vec!["a".into(), "b".into()]);
        assert!(!h.is_empty());
    }
}
