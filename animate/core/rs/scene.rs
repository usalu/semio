use crate::config::AnimateConfig;
use crate::frame::FrameSnapshot;
use crate::hash::animation_hash;
use crate::sobject::{MobjectStore, Sobject, SobjectId};
use crate::timeline::SceneTimeline;

/// 🎭 User-authored animation scene (Manim `Scene` analogue).
pub trait Scene: Sized {
    /// 🏗️ Builds the scene timeline and mobjects.
    fn construct(&mut self, ctx: &mut SceneContext);
}

/// 🎮 Mutable scene construction context passed to `Scene::construct`.
#[derive(Clone, Debug, Default)]
pub struct SceneContext {
    timeline: SceneTimeline,
    mobjects: MobjectStore,
}

impl SceneContext {
    /// 🆕 Empty scene context.
    pub fn new() -> Self {
        Self::default()
    }

    /// ▶️ Schedules a play segment.
    pub fn play(&mut self, duration: f64) {
        self.timeline.play(duration);
    }

    /// ⏸️ Schedules a wait segment.
    pub fn wait(&mut self, duration: f64) {
        self.timeline.wait(duration);
    }

    /// ➕ Adds an Sobject to the scene.
    pub fn add(&mut self, sobject: Sobject) -> SobjectId {
        self.mobjects.add(sobject)
    }

    /// ➖ Removes an Sobject from the scene.
    pub fn remove(&mut self, id: SobjectId) -> Option<Sobject> {
        self.mobjects.remove(id)
    }

    /// 📋 Read-only mobject store.
    pub fn mobjects(&self) -> &MobjectStore {
        &self.mobjects
    }

    /// ⏱️ Scene timeline.
    pub fn timeline(&self) -> &SceneTimeline {
        &self.timeline
    }

    /// 🖼️ Snapshot at scene time `t`.
    pub fn snapshot_at(&self, frame_index: u32, time: f64, background_color: [f32; 4]) -> FrameSnapshot {
        FrameSnapshot {
            frame_index,
            time,
            mobjects: self.mobjects.snapshot(),
            background_color,
        }
    }
}

/// 🎞️ Compiled scene ready for frame iteration.
pub struct SceneRunner {
    config: AnimateConfig,
    context: SceneContext,
    scene_hash: String,
}

impl SceneRunner {
    /// 🏗️ Runs `construct` and prepares the frame loop.
    pub fn build<S: Scene>(mut scene: S, config: AnimateConfig) -> Self {
        let mut context = SceneContext::new();
        scene.construct(&mut context);
        let scene_hash = animation_hash(&config, &context);
        Self { config, context, scene_hash }
    }

    /// 🪪 Scene-level animation hash.
    pub fn animation_hash(&self) -> &str {
        &self.scene_hash
    }

    /// 🎞️ Total frame count.
    pub fn frame_count(&self) -> u32 {
        self.context.timeline().frame_count(self.config.frame_rate)
    }

    /// 🖼️ Snapshot for a frame index.
    pub fn snapshot_at(&self, frame: u32) -> FrameSnapshot {
        let time = self.context.timeline().time_at_frame(frame, self.config.frame_rate);
        self.context.snapshot_at(frame, time, self.config.background_color)
    }

    /// ⚙️ Active config.
    pub fn config(&self) -> &AnimateConfig {
        &self.config
    }

    /// 📋 Mobject store after construction.
    pub fn mobjects(&self) -> &MobjectStore {
        self.context.mobjects()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sobject::{Mobility, PaintStyle, SobjectShape, StrokeStyle};
    use mathematical_geometry::{Circle, Point};

    struct FadeCircleScene;

    impl Scene for FadeCircleScene {
        fn construct(&mut self, ctx: &mut SceneContext) {
            ctx.add(Sobject {
                id: SobjectId(0),
                shape: SobjectShape::Circle {
                    center: Point::new(0.0, 0.0),
                    radius: 1.0,
                },
                transform: mathematical_geometry::Affine::IDENTITY,
                fill: Some(PaintStyle { color: [1.0, 1.0, 1.0, 1.0] }),
                stroke: Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 0.05 }),
                z_index: 0,
                mobility: Mobility::Static,
            });
            ctx.play(1.0);
            ctx.wait(0.5);
        }
    }

    #[test]
    fn scene_runner_frame_loop() {
        let runner = SceneRunner::build(FadeCircleScene, AnimateConfig::default());
        assert_eq!(runner.frame_count(), 23);
        assert_eq!(runner.mobjects().len(), 1);
        let snap = runner.snapshot_at(0);
        assert_eq!(snap.frame_index, 0);
        assert!(!snap.animation_hash().is_empty());
    }
}
