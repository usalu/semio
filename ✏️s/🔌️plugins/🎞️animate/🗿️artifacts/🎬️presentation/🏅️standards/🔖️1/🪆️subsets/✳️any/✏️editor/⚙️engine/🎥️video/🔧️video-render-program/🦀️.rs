//! 🔧️ Video render program of the headless video engine: the builder that turns captured frames into the host render program. See [`super`].
//!
//! 🎞️ Captured frames → `semio_framework::kernel::VideoRenderProgram`: the target-neutral half of video export. A
//! guest has no GPU and no encoder, so it describes every frame (shared path table, deduplicated scenes, run-length
//! timeline) and the host renders + encodes it (`Effect::VideoRenderExport`). Paint order and colours are exactly
//! the native `VelloRenderer`'s (`renderer::paint_mobject`), so both tiers draw the same picture.

use crate::editor::animate::engine::camera::camera::Camera;
use crate::editor::animate::engine::config::config::AnimateConfig;
use crate::editor::animate::engine::scene::sobject::Sobject;
use crate::editor::animate::engine::video::renderer::{color_from_style, scene_affine, CapturedFrame};
use geometry::PathEl;
use semio_framework::kernel::{VideoRenderOp, VideoRenderPath, VideoRenderProgram, VideoRenderRun, VideoRenderScene, VIDEO_RENDER_PROGRAM_SCHEMA};
use std::collections::HashMap;

type OpKey = (u8, u32, [u64; 6], [u64; 4], u64);

/// 🧱️ Accumulates captured frames into one program: identical paths share one table row, identical frames one scene,
/// consecutive identical frames one timeline run.
pub struct VideoProgramBuilder {
    program: VideoRenderProgram,
    paths: HashMap<(String, Vec<u64>), u32>,
    scenes: HashMap<Vec<OpKey>, u32>,
}

fn bits<const N: usize>(values: [f64; N]) -> [u64; N] {
    values.map(f64::to_bits)
}

fn op_key(op: &VideoRenderOp) -> OpKey {
    match op {
        VideoRenderOp::Fill { path, transform, color } => (0, *path, bits(*transform), bits(*color), 0),
        VideoRenderOp::Stroke { path, transform, color, width } => (1, *path, bits(*transform), bits(*color), width.to_bits()),
        VideoRenderOp::Image { image, crop, transform, opacity } => (2, *image, bits(*transform), bits(*crop), opacity.to_bits()),
    }
}

fn unit(value: f64) -> f64 {
    if value.is_nan() {
        0.0
    } else {
        value.clamp(0.0, 1.0)
    }
}

impl VideoProgramBuilder {
    /// 🏗️ An empty program at `config`'s picture size, frame rate and background.
    pub fn new(config: &AnimateConfig) -> Self {
        let program = VideoRenderProgram {
            schema: VIDEO_RENDER_PROGRAM_SCHEMA.into(),
            width: config.width,
            height: config.height,
            fps: config.frame_rate.round().max(1.0) as u32,
            background: config.background.map(unit),
            paths: Vec::new(),
            images: Vec::new(),
            scenes: Vec::new(),
            timeline: Vec::new(),
        };
        Self { program, paths: HashMap::new(), scenes: HashMap::new() }
    }

    fn path_index(&mut self, path: &geometry::BezPath) -> u32 {
        let mut verbs = String::new();
        let mut points = Vec::new();
        for element in path.elements() {
            match element {
                PathEl::MoveTo(p) => {
                    verbs.push('M');
                    points.extend([p.x(), p.y()]);
                }
                PathEl::LineTo(p) => {
                    verbs.push('L');
                    points.extend([p.x(), p.y()]);
                }
                PathEl::QuadTo(a, b) => {
                    verbs.push('Q');
                    points.extend([a.x(), a.y(), b.x(), b.y()]);
                }
                PathEl::CurveTo(a, b, c) => {
                    verbs.push('C');
                    points.extend([a.x(), a.y(), b.x(), b.y(), c.x(), c.y()]);
                }
                PathEl::ClosePath => verbs.push('Z'),
            }
        }
        let key = (verbs.clone(), points.iter().map(|value| value.to_bits()).collect::<Vec<_>>());
        if let Some(index) = self.paths.get(&key) {
            return *index;
        }
        let index = self.program.paths.len() as u32;
        self.program.paths.push(VideoRenderPath { verbs, points });
        self.paths.insert(key, index);
        index
    }

    /// 🎞️ Appends one captured frame, painted like `VelloRenderer::render_capture` (z-order, then id).
    pub fn push_capture(&mut self, capture: &CapturedFrame, camera: &Camera, config: &AnimateConfig) {
        let view = scene_affine(camera, config.width, config.height);
        let mut order: Vec<usize> = (0..capture.mobjects.len()).collect();
        order.sort_by_key(|&index| (capture.mobjects[index].z_order(), capture.mobjects[index].id()));
        let mut ops = Vec::new();
        for index in order {
            let mobject = &capture.mobjects[index];
            let transform = (view * mobject.transform()).as_coeffs();
            let style = mobject.style();
            let opacity = mobject.effective_opacity();
            for path in mobject.paths() {
                let path = self.path_index(&path);
                if let Some(fill) = style.fill {
                    let color = color_from_style(fill.with_alpha(fill.a * style.fill_opacity * opacity)).map(unit);
                    ops.push(VideoRenderOp::Fill { path, transform, color });
                }
                if let Some(stroke) = style.stroke {
                    let color = color_from_style(stroke.with_alpha(stroke.a * style.stroke_opacity * opacity)).map(unit);
                    ops.push(VideoRenderOp::Stroke { path, transform, color, width: style.stroke_width.max(0.0) });
                }
            }
        }
        let key: Vec<OpKey> = ops.iter().map(op_key).collect();
        let scene = match self.scenes.get(&key) {
            Some(scene) => *scene,
            None => {
                let scene = self.program.scenes.len() as u32;
                self.program.scenes.push(VideoRenderScene { ops });
                self.scenes.insert(key, scene);
                scene
            }
        };
        match self.program.timeline.last_mut() {
            Some(run) if run.scene == scene => run.frames += 1,
            _ => self.program.timeline.push(VideoRenderRun { scene, frames: 1 }),
        }
    }

    /// 📦️ The finished program.
    pub fn finish(self) -> VideoRenderProgram {
        self.program
    }
}

#[cfg(test)]
include!("../🧪️tests/🔬️program-unit/🦀️.rs");
