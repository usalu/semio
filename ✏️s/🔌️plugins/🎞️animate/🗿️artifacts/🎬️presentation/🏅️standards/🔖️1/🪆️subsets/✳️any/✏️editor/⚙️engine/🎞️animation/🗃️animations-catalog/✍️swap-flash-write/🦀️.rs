//! ✍️ Swap, flash and write animations of the extended catalog: swap, matching shapes, homotopy, passing flash, spiral in, uncreate, write, grow from center, indicate and rotating. See [`super`].

use super::*;

pub struct Swap {
    pub target_id: u64,
    pub swap_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    a_center: Option<Point>,
    b_center: Option<Point>,
}

impl Swap {
    pub fn new(target_id: u64, swap_id: u64, run_time: f64) -> Self {
        Self { target_id, swap_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, a_center: None, b_center: None }
    }
}

impl Animation for Swap {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {
        self.a_center = None;
        self.b_center = None;
    }
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, _alpha: f64) {}
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha_for(self, parent_alpha);
        let target_id = self.target_id;
        let swap_id = self.swap_id;
        let mut a_center = self.a_center;
        let mut b_center = self.b_center;
        if a_center.is_none() {
            with_vsobject(mobjects, target_id, |v| {
                a_center = Some(v.center());
            });
            self.a_center = a_center;
        }
        if b_center.is_none() {
            with_vsobject(mobjects, swap_id, |v| {
                b_center = Some(v.center());
            });
            self.b_center = b_center;
        }
        if let (Some(a), Some(b)) = (self.a_center, self.b_center) {
            let a_pos = lerp_point(a, b, alpha);
            let b_pos = lerp_point(b, a, alpha);
            with_vsobject(mobjects, target_id, |v| {
                v.move_to(a_pos);
            });
            with_vsobject(mobjects, swap_id, |v| {
                v.move_to(b_pos);
            });
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id, self.swap_id]
    }
}

pub struct TransformMatchingShapes {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) primed: bool,
}

impl TransformMatchingShapes {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
    }
}

impl Animation for TransformMatchingShapes {
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

pub struct Homotopy {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) start_transform: Option<Affine>,
}

impl Homotopy {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, start_transform: None }
    }
}

impl Animation for Homotopy {
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
                let dx = 0.3 * alpha * (center.y() * 0.1).sin();
                let dy = 0.3 * alpha * (center.x() * 0.1).cos();
                *v.transform_mut() = start * Affine::IDENTITY.translate(Vec2::new(dx, dy));
            });
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

pub struct ShowPassingFlash {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) primed: bool,
}

impl ShowPassingFlash {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::linear, primed: false }
    }
}

impl Animation for ShowPassingFlash {
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
            with_vsobject(mobjects, target_id, |v| {
                v.set_point_ratio(0.0);
            });
            self.primed = true;
        }
        with_vsobject(mobjects, target_id, |v| {
            v.set_point_ratio(alpha);
            v.set_opacity(0.3 + 0.7 * (1.0 - (alpha - 0.5).abs() * 2.0).max(0.0));
        });
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

pub struct SpiralIn {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub angle: f64,
    pub origin: Point,
    pub(super) start_transform: Option<Affine>,
}

impl SpiralIn {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, angle: std::f64::consts::TAU * 2.0, origin: Point::ZERO, start_transform: None }
    }
}

impl Animation for SpiralIn {
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
        let origin = self.origin;
        let angle = self.angle;
        let mut start = self.start_transform;
        if start.is_none() {
            with_vsobject(mobjects, target_id, |v| {
                start = Some(v.transform());
            });
            self.start_transform = start;
        }
        if let Some(start) = self.start_transform {
            with_vsobject(mobjects, target_id, |v| {
                let scaled = scale_about_center(start, origin, alpha.max(0.001));
                *v.transform_mut() = rotate_about_center(scaled, origin, angle * (1.0 - alpha));
                v.set_opacity(alpha);
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

pub struct Uncreate {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) primed: bool,
}

impl Uncreate {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
    }
}

impl Animation for Uncreate {
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
        if !self.primed {
            with_vsobject(mobjects, self.target_id, |v| v.set_point_ratio(1.0));
            self.primed = true;
        }
        with_vsobject(mobjects, self.target_id, |v| v.set_point_ratio(1.0 - alpha));
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

pub struct Write {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) primed: bool,
}

impl Write {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
    }
}

impl Animation for Write {
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
        if !self.primed {
            with_vsobject(mobjects, self.target_id, |v| v.set_point_ratio(0.0));
            self.primed = true;
        }
        with_vsobject(mobjects, self.target_id, |v| v.set_point_ratio(alpha));
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
    fn is_introducer(&self) -> bool {
        true
    }
}

pub struct GrowFromCenter {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) start_transform: Option<Affine>,
}

impl GrowFromCenter {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, start_transform: None }
    }
}

impl Animation for GrowFromCenter {
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
                *v.transform_mut() = scale_about_center(start, center, alpha.max(0.001));
                v.set_opacity(alpha);
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

pub struct Indicate {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) start_transform: Option<Affine>,
}

impl Indicate {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::there_and_back, start_transform: None }
    }
}

impl Animation for Indicate {
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
                let pulse = 1.0 + 0.25 * alpha;
                let center = v.center();
                *v.transform_mut() = scale_about_center(start, center, pulse);
            });
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

pub struct Rotating {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub angle: f64,
    pub(super) start_transform: Option<Affine>,
}

impl Rotating {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::linear, angle: std::f64::consts::TAU, start_transform: None }
    }
}

impl Animation for Rotating {
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
                *v.transform_mut() = rotate_about_center(start, center, self.angle * alpha);
            });
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}
