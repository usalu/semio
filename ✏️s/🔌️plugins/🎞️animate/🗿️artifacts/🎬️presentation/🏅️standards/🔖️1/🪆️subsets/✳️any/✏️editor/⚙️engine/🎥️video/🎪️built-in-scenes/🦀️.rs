//! 🎪️ Built-in scenes of the headless video engine: the scenes resolved by content hash for presentation and video export. See [`super`].
//!
//! 🎬️ Built-in scenes resolved by content hash for presentation/video export.

use crate::editor::animate::engine::camera::camera::Camera;
use crate::editor::animate::engine::config::config::AnimateConfig;
use crate::editor::animate::engine::scene::scene::{BasicStage, Scene};
use crate::editor::animate::engine::scene::section::Section;
use crate::editor::animate::engine::scene::section::SectionList;
use crate::editor::animate::engine::scene::sobject::{Sobjects, VSobject};
use std::collections::HashMap;

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
    fn config(&self) -> &AnimateConfig {
        self.base.config()
    }
    fn config_mut(&mut self) -> &mut AnimateConfig {
        self.base.config_mut()
    }
    fn camera(&self) -> &Camera {
        self.base.camera()
    }
    fn camera_mut(&mut self) -> &mut Camera {
        self.base.camera_mut()
    }
    fn mobjects(&self) -> &HashMap<u64, Sobjects> {
        self.base.mobjects()
    }
    fn mobjects_mut(&mut self) -> &mut HashMap<u64, Sobjects> {
        self.base.mobjects_mut()
    }
    fn sections(&self) -> &SectionList {
        self.base.sections()
    }
    fn sections_mut(&mut self) -> &mut SectionList {
        self.base.sections_mut()
    }
    fn scene_time(&self) -> f64 {
        self.base.scene_time()
    }
    fn set_scene_time(&mut self, time: f64) {
        self.base.set_scene_time(time);
    }
}

/// 🔍️ Builds the default scene implementation for a scene hash.
pub fn scene_for_hash(config: AnimateConfig, scene_hash: &str) -> HashDemoScene {
    HashDemoScene::new(config, scene_hash)
}

#[cfg(test)]
include!("../🧪️tests/🔬️scenes-unit/🦀️.rs");
