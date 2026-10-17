//! 🪄️ Builder and effect animations of the core animation set: the animate builder and the shift, apply-method, focus, blink, traced path and change-speed animations. See [`super`].

use super::*;

/// 🏗️ Fluent `.animate()` builder for common tweens.
///
/// 🔀️ R11 "trivially generic" case (not the closed-set enum): this holds a single erased
/// receiver, never heterogeneous storage, so it stays generic over `S: Sobject` instead of routing
/// through `Sobjects` — that keeps `AnimateExt`'s blanket `impl<T: Sobject + Sized> AnimateExt for
/// T` working for `VSobject`/`Group`/`ThreeDVSobject` (and `Sobjects` itself) alike, exactly as the
/// pre-dyn-removal `&mut dyn Sobject` receiver did.
pub struct AnimateBuilder<'a, S: Sobject> {
    pub target: &'a mut S,
    pub run_time: f64,
    pub rate: RateFunc,
}

impl<'a, S: Sobject> AnimateBuilder<'a, S> {
    pub fn new(target: &'a mut S, run_time: f64) -> Self {
        Self { target, run_time, rate: crate::editor::animate::engine::rate::rate::smooth }
    }

    pub fn with_rate(mut self, rate: RateFunc) -> Self {
        self.rate = rate;
        self
    }

    pub fn fade_in(self) -> FadeIn {
        FadeIn { target_id: self.target.id(), run_time: self.run_time, rate: self.rate, target_opacity: 1.0, start_opacity: 0.0, primed: false }
    }

    pub fn fade_out(self) -> FadeOut {
        FadeOut { target_id: self.target.id(), run_time: self.run_time, rate: self.rate, start_opacity: 1.0, primed: false }
    }

    pub fn create(self) -> Create {
        Create::new(self.target.id(), self.run_time).with_rate(self.rate)
    }

    pub fn transform(self) -> Transform {
        Transform { target_id: self.target.id(), run_time: self.run_time, rate: self.rate, primed: false }
    }

    pub fn rotate(self, angle: f64) -> Rotate {
        Rotate::new(self.target.id(), angle, self.run_time)
    }

    pub fn shift(self, delta: Vec2) -> Shift {
        Shift::new(self.target.id(), delta, self.run_time)
    }
}

/// ↔ Translate an Sobject by a fixed delta.
pub struct Shift {
    pub target_id: u64,
    pub delta: Vec2,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) start_transform: Option<Affine>,
}

impl Shift {
    pub fn new(target_id: u64, delta: Vec2, run_time: f64) -> Self {
        Self { target_id, delta, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, start_transform: None }
    }
}

impl Animation for Shift {
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
    fn interpolate_mobject(&mut self, _alpha: f64) {}
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        let target_id = self.target_id;
        let delta = self.delta * alpha;
        let mut start = self.start_transform;
        if start.is_none() {
            with_vsobject(mobjects, target_id, |v| {
                start = Some(v.transform());
            });
            self.start_transform = start;
        }
        if let Some(start) = self.start_transform {
            with_vsobject(mobjects, target_id, |v| {
                *v.transform_mut() = start * Affine::IDENTITY.translate(delta);
            });
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

/// 🪄️ Apply a transform method over the animation duration.
pub struct ApplyMethod {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub scale_factor: f64,
    pub(super) start_transform: Option<Affine>,
}

impl ApplyMethod {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, scale_factor: 1.2, start_transform: None }
    }
}

impl Animation for ApplyMethod {
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
    fn interpolate_mobject(&mut self, _alpha: f64) {}
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        let target_id = self.target_id;
        let factor = self.scale_factor;
        let mut start = self.start_transform;
        if start.is_none() {
            with_vsobject(mobjects, target_id, |v| {
                start = Some(v.transform());
            });
            self.start_transform = start;
        }
        if let Some(start) = self.start_transform {
            with_vsobject(mobjects, target_id, |v| {
                *v.transform_mut() = start;
                v.scale(1.0 + (factor - 1.0) * alpha);
            });
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

/// 🔍️ Briefly scale and highlight a target as if focusing a camera.
pub struct FocusOn {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) start_transform: Option<Affine>,
}

impl FocusOn {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::there_and_back, start_transform: None }
    }
}

impl Animation for FocusOn {
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
    fn interpolate_mobject(&mut self, _alpha: f64) {}
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        let target_id = self.target_id;
        let mut start = self.start_transform;
        if start.is_none() {
            with_vsobject(mobjects, target_id, |v| {
                start = Some(v.transform());
            });
            self.start_transform = start;
        }
        if let Some(start) = self.start_transform {
            with_vsobject(mobjects, target_id, |v| {
                *v.transform_mut() = start;
                v.scale(1.0 + 0.3 * alpha);
                v.set_opacity(1.0 - 0.2 * alpha);
            });
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

/// 👁️ Blink opacity off and back on.
pub struct Blink {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) start_opacity: f64,
    pub(super) primed: bool,
}

impl Blink {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::there_and_back, start_opacity: 1.0, primed: false }
    }
}

impl Animation for Blink {
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
    fn interpolate_mobject(&mut self, _alpha: f64) {}
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        let target_id = self.target_id;
        if !self.primed {
            let mut opacity = self.start_opacity;
            with_vsobject(mobjects, target_id, |v| {
                opacity = v.opacity();
            });
            self.start_opacity = opacity;
            self.primed = true;
        }
        with_vsobject(mobjects, target_id, |v| {
            v.set_opacity(self.start_opacity * (1.0 - alpha * 0.9));
        });
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

/// 🛤️ Reveal a traced path progressively.
pub struct TracedPath {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) primed: bool,
}

impl TracedPath {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::linear, primed: false }
    }
}

impl Animation for TracedPath {
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
    fn interpolate_mobject(&mut self, _alpha: f64) {}
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        let target_id = self.target_id;
        if !self.primed {
            with_vsobject(mobjects, target_id, |v| v.set_point_ratio(0.0));
            self.primed = true;
        }
        with_vsobject(mobjects, target_id, |v| v.set_point_ratio(alpha));
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
    fn is_introducer(&self) -> bool {
        true
    }
}

/// ⏩️ Remap playback speed of a nested animation.
pub struct ChangeSpeed {
    pub animation: Box<Animations>,
    pub speed_factor: f64,
}

impl ChangeSpeed {
    pub fn new(animation: Animations, speed_factor: f64) -> Self {
        Self { animation: Box::new(animation), speed_factor: speed_factor.max(1e-9) }
    }
}

impl Animation for ChangeSpeed {
    fn duration(&self) -> f64 {
        self.animation.duration() / self.speed_factor
    }
    fn rate_func(&self) -> RateFunc {
        self.animation.rate_func()
    }
    fn begin(&mut self) {
        self.animation.begin();
    }
    fn finish(&mut self) {
        self.animation.finish();
    }
    fn interpolate_mobject(&mut self, alpha: f64) {
        self.animation.interpolate_mobject(alpha);
    }
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let remapped = (parent_alpha * self.speed_factor).clamp(0.0, 1.0);
        self.animation.apply(mobjects, remapped);
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        self.animation.get_all_mobjects()
    }
    fn is_introducer(&self) -> bool {
        self.animation.is_introducer()
    }
    fn is_remover(&self) -> bool {
        self.animation.is_remover()
    }
}
