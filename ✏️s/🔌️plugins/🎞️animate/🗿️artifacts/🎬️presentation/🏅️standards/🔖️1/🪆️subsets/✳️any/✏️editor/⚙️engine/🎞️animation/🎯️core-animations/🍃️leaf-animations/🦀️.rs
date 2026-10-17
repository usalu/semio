//! 🍃️ Leaf animations of the core animation set: create, fade in and out, transform, rotate and move along a path. See [`super`].

use super::*;

/// ✏️ Draw/create path progressively.
pub struct Create {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    started: bool,
    snapshot_ratio: f64,
}

impl Create {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::linear, started: false, snapshot_ratio: 1.0 }
    }

    pub fn with_rate(mut self, rate: RateFunc) -> Self {
        self.rate = rate;
        self
    }
}

impl Animation for Create {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {
        self.started = true;
        self.snapshot_ratio = 1.0;
    }
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, alpha: f64) {
        let _ = (self.target_id, alpha, self.started, self.snapshot_ratio);
    }
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        with_vsobject(mobjects, self.target_id, |v| {
            v.set_point_ratio(alpha * self.snapshot_ratio);
        });
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
    fn is_introducer(&self) -> bool {
        true
    }
}

/// 🌅️ Fade in opacity.
pub struct FadeIn {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub target_opacity: f64,
    pub(super) start_opacity: f64,
    pub(super) primed: bool,
}

impl FadeIn {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, target_opacity: 1.0, start_opacity: 0.0, primed: false }
    }
}

impl Animation for FadeIn {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {
        self.primed = false;
    }
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, alpha: f64) {
        let _ = (self.target_id, alpha * self.target_opacity);
    }
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        with_vsobject(mobjects, self.target_id, |v| {
            if !self.primed {
                self.start_opacity = v.opacity();
                self.primed = true;
            }
            let opacity = self.start_opacity + (self.target_opacity - self.start_opacity) * alpha;
            v.set_opacity(opacity);
        });
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
    fn is_introducer(&self) -> bool {
        true
    }
}

/// 🌇️ Fade out opacity.
pub struct FadeOut {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) start_opacity: f64,
    pub(super) primed: bool,
}

impl FadeOut {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, start_opacity: 1.0, primed: false }
    }
}

impl Animation for FadeOut {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {
        self.primed = false;
    }
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, alpha: f64) {
        let _ = (self.target_id, 1.0 - alpha);
    }
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        with_vsobject(mobjects, self.target_id, |v| {
            if !self.primed {
                self.start_opacity = v.opacity();
                self.primed = true;
            }
            v.set_opacity(self.start_opacity * (1.0 - alpha));
        });
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
    fn is_remover(&self) -> bool {
        true
    }
}

/// 🔀️ Morph between saved state and generated target.
pub struct Transform {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) primed: bool,
}

impl Transform {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
    }
}

impl Animation for Transform {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {
        self.primed = false;
    }
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, alpha: f64) {
        let _ = (self.target_id, alpha);
    }
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        with_vsobject(mobjects, self.target_id, |v| {
            if !self.primed {
                v.save_state();
                if !v.has_target() {
                    v.generate_target();
                }
                self.primed = true;
            }
            v.interpolate_saved_to_target(alpha);
        });
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

/// 🔄️ Rotate an Sobject.
pub struct Rotate {
    pub target_id: u64,
    pub angle: f64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) start_transform: Option<Affine>,
}

impl Rotate {
    pub fn new(target_id: u64, angle: f64, run_time: f64) -> Self {
        Self { target_id, angle, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, start_transform: None }
    }
}

impl Animation for Rotate {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {
        self.start_transform = None;
    }
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, alpha: f64) {
        let _ = (self.target_id, self.angle * alpha);
    }
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        with_vsobject(mobjects, self.target_id, |v| {
            if self.start_transform.is_none() {
                self.start_transform = Some(v.transform());
            }
            if let Some(start) = self.start_transform {
                *v.transform_mut() = start;
                v.rotate(self.angle * alpha);
            }
        });
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

/// 🛤️ Move along a Bézier path.
pub struct MoveAlongPath {
    pub target_id: u64,
    pub path: CubicBez,
    pub run_time: f64,
    pub rate: RateFunc,
}

impl MoveAlongPath {
    pub fn new(target_id: u64, path: CubicBez, run_time: f64) -> Self {
        Self { target_id, path, run_time, rate: crate::editor::animate::engine::rate::rate::linear }
    }

    pub fn position_at(&self, alpha: f64) -> Point {
        cubic_point_at(self.path, alpha)
    }
}

impl Animation for MoveAlongPath {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {}
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, alpha: f64) {
        let _ = self.position_at(alpha);
    }
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        let point = self.position_at(alpha);
        with_vsobject(mobjects, self.target_id, |v| {
            v.move_to(point);
        });
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}
