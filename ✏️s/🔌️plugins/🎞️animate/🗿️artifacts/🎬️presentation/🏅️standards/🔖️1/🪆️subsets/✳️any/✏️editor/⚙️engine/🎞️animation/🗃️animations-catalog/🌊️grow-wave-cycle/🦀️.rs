//! 🌊️ Grow, wave and cycle animations of the extended catalog: grow from a point, shrink, spin in, decimal change, broadcast, wave, wiggle and cyclic replace. See [`super`].

use super::*;

pub struct GrowFromPoint {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub grow_point: Point,
    pub(super) start_transform: Option<Affine>,
}

impl GrowFromPoint {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, grow_point: Point::ZERO, start_transform: None }
    }
}

impl Animation for GrowFromPoint {
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
        let grow_point = self.grow_point;
        let mut start = self.start_transform;
        if start.is_none() {
            with_vsobject(mobjects, target_id, |v| {
                start = Some(v.transform());
            });
            self.start_transform = start;
        }
        if let Some(start) = self.start_transform {
            with_vsobject(mobjects, target_id, |v| {
                *v.transform_mut() = scale_about_center(start, grow_point, alpha.max(0.001));
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

pub struct ShrinkToCenter {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) start_transform: Option<Affine>,
    pub(super) start_opacity: f64,
    pub(super) primed: bool,
}

impl ShrinkToCenter {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, start_transform: None, start_opacity: 1.0, primed: false }
    }
}

impl Animation for ShrinkToCenter {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {
        self.start_transform = None;
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
                let factor = (1.0 - alpha).max(0.001);
                *v.transform_mut() = scale_about_center(start, center, factor);
                v.set_opacity(self.start_opacity * (1.0 - alpha));
            });
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
    fn is_remover(&self) -> bool {
        true
    }
}

pub struct SpinInFromNothing {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub angle: f64,
    pub(super) start_transform: Option<Affine>,
}

impl SpinInFromNothing {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, angle: std::f64::consts::TAU, start_transform: None }
    }
}

impl Animation for SpinInFromNothing {
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
                let center = v.center();
                let scaled = scale_about_center(start, center, alpha.max(0.001));
                *v.transform_mut() = rotate_about_center(scaled, center, angle * (1.0 - alpha));
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

pub struct ChangeDecimalToValue {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub value: f64,
    pub(super) start_opacity: f64,
    pub(super) primed: bool,
}

impl ChangeDecimalToValue {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, value: 0.0, start_opacity: 1.0, primed: false }
    }
}

impl Animation for ChangeDecimalToValue {
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
            let flicker = if alpha < 0.5 { 1.0 - alpha * 0.6 } else { 0.4 + (alpha - 0.5) * 1.2 };
            v.set_opacity(self.start_opacity * flicker.clamp(0.2, 1.0));
            let center = v.center();
            let pulse = 1.0 + 0.08 * (alpha * std::f64::consts::TAU).sin().abs();
            *v.transform_mut() = scale_about_center(v.transform(), center, pulse);
            let _ = self.value;
        });
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

pub struct Broadcast {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub(super) start_transform: Option<Affine>,
}

impl Broadcast {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::there_and_back, start_transform: None }
    }
}

impl Animation for Broadcast {
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
                let rings = 1.0 + 0.5 * alpha;
                *v.transform_mut() = scale_about_center(start, center, rings);
                v.set_opacity(1.0 - alpha * 0.5);
            });
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

pub struct ApplyWave {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub amplitude: f64,
    pub(super) start_transform: Option<Affine>,
}

impl ApplyWave {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, amplitude: 0.2, start_transform: None }
    }
}

impl Animation for ApplyWave {
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
        let amplitude = self.amplitude;
        let mut start = self.start_transform;
        if start.is_none() {
            with_vsobject(mobjects, target_id, |v| {
                start = Some(v.transform());
            });
            self.start_transform = start;
        }
        if let Some(start) = self.start_transform {
            with_vsobject(mobjects, target_id, |v| {
                let wave = amplitude * (alpha * std::f64::consts::TAU * 2.0).sin();
                *v.transform_mut() = start * Affine::IDENTITY.translate(Vec2::new(0.0, wave));
            });
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

pub struct Wiggle {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub angle: f64,
    pub(super) start_transform: Option<Affine>,
}

impl Wiggle {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::there_and_back, angle: 0.1, start_transform: None }
    }
}

impl Animation for Wiggle {
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
                let center = v.center();
                let wobble = angle * (alpha * std::f64::consts::TAU * 3.0).sin();
                *v.transform_mut() = rotate_about_center(start, center, wobble);
            });
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

pub struct CyclicReplace {
    pub cycle_ids: Vec<u64>,
    pub run_time: f64,
    pub rate: RateFunc,
    start_centers: Vec<Option<Point>>,
}

impl CyclicReplace {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self { cycle_ids: vec![target_id], run_time, rate: crate::editor::animate::engine::rate::rate::smooth, start_centers: Vec::new() }
    }
}

impl Animation for CyclicReplace {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {
        self.start_centers.clear();
    }
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, _alpha: f64) {}
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha_for(self, parent_alpha);
        let n = self.cycle_ids.len();
        if n == 0 {
            return;
        }
        if self.start_centers.len() != n {
            self.start_centers = (0..n)
                .map(|i| {
                    let id = self.cycle_ids[i];
                    let mut center = None;
                    with_vsobject(mobjects, id, |v| {
                        center = Some(v.center());
                    });
                    center
                })
                .collect();
        }
        if n == 1 {
            let id = self.cycle_ids[0];
            with_vsobject(mobjects, id, |v| {
                if let Some(start) = self.start_centers[0] {
                    let offset = Vec2::new(alpha * 0.5, 0.0);
                    v.move_to(start + offset);
                }
            });
            return;
        }
        for i in 0..n {
            let id = self.cycle_ids[i];
            let next = (i + 1) % n;
            if let (Some(from), Some(to)) = (self.start_centers[i], self.start_centers[next]) {
                let pos = lerp_point(from, to, alpha);
                with_vsobject(mobjects, id, |v| {
                    v.move_to(pos);
                });
            }
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        self.cycle_ids.clone()
    }
}
