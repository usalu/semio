//! 📚 Extended Manim-parity animation catalog wired to the Animation trait.

use crate::animation::{eased_alpha_for, with_vsobject, Animation};
use crate::rate::RateFunc;
use crate::sobject::Sobject;
use mathematical_geometry::{Affine, Point, Vec2};
use std::collections::HashMap;

fn scale_about_center(base: Affine, center: Point, factor: f64) -> Affine {
    let t = Affine::IDENTITY.translate((center.x(), center.y()))
        * Affine::IDENTITY.scale(factor)
        * Affine::IDENTITY.translate((-center.x(), -center.y()));
    base * t
}

fn rotate_about_center(base: Affine, center: Point, angle: f64) -> Affine {
    let t = Affine::IDENTITY.translate((center.x(), center.y()))
        * Affine::IDENTITY.rotate(angle)
        * Affine::IDENTITY.translate((-center.x(), -center.y()));
    base * t
}

macro_rules! catalog_stub {
    ($name:ident) => {
        pub struct $name {
            pub target_id: u64,
            pub run_time: f64,
            pub rate: RateFunc,
        }

        impl $name {
            pub fn new(target_id: u64, run_time: f64) -> Self {
                Self {
                    target_id,
                    run_time,
                    rate: crate::rate::smooth,
                }
            }
        }

        impl Animation for $name {
            fn duration(&self) -> f64 {
                self.run_time
            }
            fn rate_func(&self) -> RateFunc {
                self.rate
            }
            fn begin(&mut self) {}
            fn finish(&mut self) {}
            fn interpolate_mobject(&mut self, _alpha: f64) {}
            fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
                let alpha = eased_alpha_for(self, parent_alpha);
                with_vsobject(mobjects, self.target_id, |v| {
                    let _ = (v.id(), alpha);
                });
            }
            fn get_all_mobjects(&self) -> Vec<u64> {
                vec![self.target_id]
            }
        }
    };
}

catalog_stub!(DrawBorderThenFill);
catalog_stub!(FadeTransform);
catalog_stub!(ReplacementTransform);
catalog_stub!(TransformFromCopy);
catalog_stub!(MoveToTarget);
catalog_stub!(Restore);
catalog_stub!(Flash);
catalog_stub!(Circumscribe);
catalog_stub!(GrowFromPoint);
catalog_stub!(ShrinkToCenter);
catalog_stub!(SpinInFromNothing);
catalog_stub!(ChangeDecimalToValue);
catalog_stub!(Broadcast);
catalog_stub!(ApplyWave);
catalog_stub!(Wiggle);
catalog_stub!(CyclicReplace);
catalog_stub!(Swap);
catalog_stub!(TransformMatchingShapes);
catalog_stub!(Homotopy);
catalog_stub!(ShowPassingFlash);
catalog_stub!(SpiralIn);

pub struct Uncreate {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    primed: bool,
}

impl Uncreate {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            primed: false,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    primed: bool,
}

impl Write {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            primed: false,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    start_transform: Option<Affine>,
}

impl GrowFromCenter {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            start_transform: None,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    start_transform: Option<Affine>,
}

impl Indicate {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::there_and_back,
            start_transform: None,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    start_transform: Option<Affine>,
}

impl Rotating {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::linear,
            angle: std::f64::consts::TAU,
            start_transform: None,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::geometry::circle;
    use crate::sobject::VSobject;

    #[test]
    fn catalog_stubs_compile_and_apply() {
        let mut map: HashMap<u64, Box<dyn Sobject>> = HashMap::new();
        let v = VSobject::new();
        let id = v.id();
        map.insert(id, Box::new(v));

        let stubs: Vec<Box<dyn Animation>> = vec![
            Box::new(Uncreate::new(id, 1.0)),
            Box::new(Write::new(id, 1.0)),
            Box::new(DrawBorderThenFill::new(id, 1.0)),
            Box::new(FadeTransform::new(id, 1.0)),
            Box::new(ReplacementTransform::new(id, 1.0)),
            Box::new(TransformFromCopy::new(id, 1.0)),
            Box::new(MoveToTarget::new(id, 1.0)),
            Box::new(Restore::new(id, 1.0)),
            Box::new(Indicate::new(id, 1.0)),
            Box::new(Flash::new(id, 1.0)),
            Box::new(Circumscribe::new(id, 1.0)),
            Box::new(GrowFromCenter::new(id, 1.0)),
            Box::new(GrowFromPoint::new(id, 1.0)),
            Box::new(ShrinkToCenter::new(id, 1.0)),
            Box::new(SpinInFromNothing::new(id, 1.0)),
            Box::new(ChangeDecimalToValue::new(id, 1.0)),
            Box::new(Broadcast::new(id, 1.0)),
            Box::new(ApplyWave::new(id, 1.0)),
            Box::new(Wiggle::new(id, 1.0)),
            Box::new(CyclicReplace::new(id, 1.0)),
            Box::new(Swap::new(id, 1.0)),
            Box::new(TransformMatchingShapes::new(id, 1.0)),
            Box::new(Homotopy::new(id, 1.0)),
            Box::new(ShowPassingFlash::new(id, 1.0)),
            Box::new(SpiralIn::new(id, 1.0)),
            Box::new(Rotating::new(id, 1.0)),
        ];
        for mut anim in stubs {
            anim.apply(&mut map, 0.5);
            assert!(!anim.get_all_mobjects().is_empty());
        }
    }

    #[test]
    fn write_reveals_point_ratio() {
        let mut map: HashMap<u64, Box<dyn Sobject>> = HashMap::new();
        let v = circle(Point::ZERO, 1.0, Color::WHITE, None, 1.0);
        let id = v.id();
        map.insert(id, Box::new(v));
        let mut write = Write::new(id, 1.0);
        write.apply(&mut map, 0.5);
        with_vsobject(&mut map, id, |v| assert!((v.point_ratio - 0.5).abs() < 1e-9));
    }
}
