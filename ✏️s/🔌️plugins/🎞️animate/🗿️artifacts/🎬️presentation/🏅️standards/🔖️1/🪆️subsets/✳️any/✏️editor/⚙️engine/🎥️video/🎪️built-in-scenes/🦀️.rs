//! 🎪️ Built-in scenes of the headless video engine: the scenes resolved by content hash for presentation and video export. See [`super`].
//!
//! 🎬️ Built-in scenes resolved by content hash for presentation/video export.

use crate::editor::animate::engine::config::config::AnimateConfig;
use crate::editor::animate::engine::scene::scene::{BasicStage, Scene};
use crate::editor::animate::engine::scene::sobject::VSobject;
use crate::editor::animate::engine::scene::section::Section;

/// 🧩️ Demo scene used when no bespoke scene is registered for a hash.
pub struct HashDemoScene {
    base: BasicStage,
    hash: String,
}

impl HashDemoScene {
    pub fn new(config: AnimateConfig, hash: impl Into<String>) -> Self {
        Self { base: BasicStage::new(config), hash: hash.into() }
    }
}

impl Scene for HashDemoScene {
    fn construct(&mut self) {
        self.add(VSobject::new().into());
        let label = format!("scene-{}", &self.hash[..self.hash.len().min(8)]);
        self.sections_mut().push(Section::new(label, 0.0, 0.2));
        self.wait(0.2);
    }
    fn stage(&self) -> &BasicStage {
        &self.base
    }

    fn stage_mut(&mut self) -> &mut BasicStage {
        &mut self.base
    }
}

/// 🔍️ Builds the default scene implementation for a scene hash.
pub fn scene_for_hash(config: AnimateConfig, scene_hash: &str) -> HashDemoScene {
    HashDemoScene::new(config, scene_hash)
}

#[cfg(test)]
include!("../🧪️tests/🔬️scenes-unit/🦀️.rs");
