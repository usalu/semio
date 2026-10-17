//! 🎆️ Transform and highlight animations of the extended catalog: draw-border, fade and replacement transforms, move to target, restore, flash and circumscribe. See [`super`].

use super::*;

pub struct DrawBorderThenFill {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) primed: bool,
    fill_opacity: f64,
}

impl DrawBorderThenFill {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false, fill_opacity: 1.0 }
    }
}

impl Animation for DrawBorderThenFill {
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
        let alpha = eased_alpha_for(self, parent_alpha);
        let target_id = self.target_id;
        if !self.primed {
            let mut fill = self.fill_opacity;
            with_vsobject(mobjects, target_id, |v| {
                fill = v.style().fill_opacity;
                v.set_point_ratio(0.0);
                v.style_mut().fill_opacity = 0.0;
            });
            self.fill_opacity = fill;
            self.primed = true;
        }
        if alpha < 0.5 {
            with_vsobject(mobjects, target_id, |v| {
                v.set_point_ratio(alpha * 2.0);
                v.style_mut().fill_opacity = 0.0;
            });
        } else {
            with_vsobject(mobjects, target_id, |v| {
                v.set_point_ratio(1.0);
                v.style_mut().fill_opacity = self.fill_opacity * (alpha - 0.5) * 2.0;
            });
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
    fn is_introducer(&self) -> bool {
        true
    }
}

pub struct FadeTransform {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) primed: bool,
}

impl FadeTransform {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
    }
}

impl Animation for FadeTransform {
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
        let alpha = eased_alpha_for(self, parent_alpha);
        let target_id = self.target_id;
        let mut primed = self.primed;
        if !primed {
            with_vsobject(mobjects, target_id, |v| {
                v.save_state();
                if !v.has_target() {
                    v.generate_target();
                }
            });
            primed = true;
            self.primed = primed;
        }
        with_vsobject(mobjects, target_id, |v| {
            v.interpolate_saved_to_target(alpha);
            let fade = if alpha < 0.5 { alpha * 2.0 } else { (1.0 - alpha) * 2.0 };
            v.set_opacity(0.5 + 0.5 * (1.0 - fade * 0.5));
        });
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

pub struct ReplacementTransform {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) primed: bool,
}

impl ReplacementTransform {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
    }
}

impl Animation for ReplacementTransform {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {
        self.primed = false;
    }
    fn finish(&mut self) {
        let _ = self.target_id;
    }
    fn interpolate_mobject(&mut self, _alpha: f64) {}
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha_for(self, parent_alpha);
        let target_id = self.target_id;
        let mut primed = self.primed;
        if !primed {
            with_vsobject(mobjects, target_id, |v| {
                v.save_state();
                if !v.has_target() {
                    v.generate_target();
                }
            });
            primed = true;
            self.primed = primed;
        }
        with_vsobject(mobjects, target_id, |v| {
            v.interpolate_saved_to_target(alpha);
        });
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

pub struct TransformFromCopy {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) primed: bool,
    pub(super) start_transform: Option<Affine>,
}

impl TransformFromCopy {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false, start_transform: None }
    }
}

impl Animation for TransformFromCopy {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {
        self.primed = false;
        self.start_transform = None;
    }
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, _alpha: f64) {}
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha_for(self, parent_alpha);
        let target_id = self.target_id;
        let mut primed = self.primed;
        if !primed {
            with_vsobject(mobjects, target_id, |v| {
                v.save_state();
                if !v.has_target() {
                    v.generate_target();
                }
            });
            primed = true;
            self.primed = primed;
        }
        let mut start = self.start_transform;
        if start.is_none() {
            with_vsobject(mobjects, target_id, |v| {
                start = Some(v.transform());
            });
            self.start_transform = start;
        }
        with_vsobject(mobjects, target_id, |v| {
            v.interpolate_saved_to_target(alpha);
            if let Some(start) = self.start_transform {
                let center = v.center();
                *v.transform_mut() = scale_about_center(start, center, 0.001 + alpha * 0.999);
            }
            v.set_opacity(alpha);
        });
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
    fn is_introducer(&self) -> bool {
        true
    }
}

pub struct MoveToTarget {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) primed: bool,
}

impl MoveToTarget {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
    }
}

impl Animation for MoveToTarget {
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
        let alpha = eased_alpha_for(self, parent_alpha);
        let target_id = self.target_id;
        let mut primed = self.primed;
        if !primed {
            with_vsobject(mobjects, target_id, |v| {
                v.save_state();
                if !v.has_target() {
                    v.generate_target();
                }
            });
            primed = true;
            self.primed = primed;
        }
        with_vsobject(mobjects, target_id, |v| {
            v.interpolate_saved_to_target(alpha);
        });
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

pub struct Restore {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) primed: bool,
}

impl Restore {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
    }
}

impl Animation for Restore {
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
        let alpha = eased_alpha_for(self, parent_alpha);
        let target_id = self.target_id;
        let mut primed = self.primed;
        if !primed {
            with_vsobject(mobjects, target_id, |v| {
                v.generate_target();
            });
            primed = true;
            self.primed = primed;
        }
        if alpha >= 1.0 - 1e-9 {
            with_vsobject(mobjects, target_id, |v| v.restore());
        } else {
            with_vsobject(mobjects, target_id, |v| {
                v.interpolate_saved_to_target(1.0 - alpha);
            });
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

pub struct Flash {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) start_opacity: f64,
    pub(super) primed: bool,
}

impl Flash {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::there_and_back, start_opacity: 1.0, primed: false }
    }
}

impl Animation for Flash {
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
        let alpha = eased_alpha_for(self, parent_alpha);
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
            v.set_opacity(self.start_opacity + (1.0 - self.start_opacity) * alpha);
            let center = v.center();
            let pulse = 1.0 + 0.15 * alpha;
            *v.transform_mut() = scale_about_center(v.transform(), center, pulse);
        });
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

pub struct Circumscribe {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) start_transform: Option<Affine>,
}

impl Circumscribe {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::there_and_back, start_transform: None }
    }
}

impl Animation for Circumscribe {
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
        let alpha = eased_alpha_for(self, parent_alpha);
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
                let center = v.center();
                let pulse = 1.0 + 0.12 * alpha;
                *v.transform_mut() = scale_about_center(start, center, pulse);
                v.style_mut().stroke_width = 4.0 + 6.0 * alpha;
            });
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}
