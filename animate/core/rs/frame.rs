use crate::hash::{frame_hash, static_layer_hash};
use crate::sobject::MobjectStore;

/// 🖼️ Immutable scene state at one animation frame.
#[derive(Clone, Debug)]
pub struct FrameSnapshot {
    pub frame_index: u32,
    pub time: f64,
    pub mobjects: MobjectStore,
    pub background_color: [f32; 4],
}

impl FrameSnapshot {
    /// 🪪 Content hash for the full frame (animation + layout).
    pub fn animation_hash(&self) -> String {
        frame_hash(self.frame_index, self.time, &self.mobjects, self.background_color)
    }

    /// 🪨 Hash of static-layer Sobjects only.
    pub fn static_layer_hash(&self) -> String {
        static_layer_hash(&self.mobjects, self.background_color)
    }
}
