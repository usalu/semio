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

fn lerp_point(a: Point, b: Point, t: f64) -> Point {
    Point::new(
        a.x() + (b.x() - a.x()) * t,
        a.y() + (b.y() - a.y()) * t,
    )
}

pub struct DrawBorderThenFill {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    primed: bool,
    fill_opacity: f64,
}

impl DrawBorderThenFill {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            primed: false,
            fill_opacity: 1.0,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    primed: bool,
}

impl FadeTransform {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            primed: false,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    primed: bool,
}

impl ReplacementTransform {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            primed: false,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    primed: bool,
    start_transform: Option<Affine>,
}

impl TransformFromCopy {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            primed: false,
            start_transform: None,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    primed: bool,
}

impl MoveToTarget {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            primed: false,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    primed: bool,
}

impl Restore {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            primed: false,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    start_opacity: f64,
    primed: bool,
}

impl Flash {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::there_and_back,
            start_opacity: 1.0,
            primed: false,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    start_transform: Option<Affine>,
}

impl Circumscribe {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::there_and_back,
            start_transform: None,
        }
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

pub struct GrowFromPoint {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub grow_point: Point,
    start_transform: Option<Affine>,
}

impl GrowFromPoint {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            grow_point: Point::ZERO,
            start_transform: None,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    start_transform: Option<Affine>,
    start_opacity: f64,
    primed: bool,
}

impl ShrinkToCenter {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            start_transform: None,
            start_opacity: 1.0,
            primed: false,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    start_transform: Option<Affine>,
}

impl SpinInFromNothing {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            angle: std::f64::consts::TAU,
            start_transform: None,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    start_opacity: f64,
    primed: bool,
}

impl ChangeDecimalToValue {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            value: 0.0,
            start_opacity: 1.0,
            primed: false,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
            let flicker = if alpha < 0.5 {
                1.0 - alpha * 0.6
            } else {
                0.4 + (alpha - 0.5) * 1.2
            };
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
    start_transform: Option<Affine>,
}

impl Broadcast {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::there_and_back,
            start_transform: None,
        }
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
    start_transform: Option<Affine>,
}

impl ApplyWave {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            amplitude: 0.2,
            start_transform: None,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    start_transform: Option<Affine>,
}

impl Wiggle {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::there_and_back,
            angle: 0.1,
            start_transform: None,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
        Self {
            cycle_ids: vec![target_id],
            run_time,
            rate: crate::rate::smooth,
            start_centers: Vec::new(),
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
        Self {
            target_id,
            swap_id,
            run_time,
            rate: crate::rate::smooth,
            a_center: None,
            b_center: None,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    primed: bool,
}

impl TransformMatchingShapes {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            primed: false,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    start_transform: Option<Affine>,
}

impl Homotopy {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            start_transform: None,
        }
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
    primed: bool,
}

impl ShowPassingFlash {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::linear,
            primed: false,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
    start_transform: Option<Affine>,
}

impl SpiralIn {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            angle: std::f64::consts::TAU * 2.0,
            origin: Point::ZERO,
            start_transform: None,
        }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
        let v2 = circle(Point::new(2.0, 0.0), 0.5, Color::WHITE, None, 1.0);
        let id2 = v2.id();
        map.insert(id2, Box::new(v2));

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
            Box::new(Swap::new(id, id2, 1.0)),
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

        with_vsobject(&mut map, id, |v| {
            assert!(v.point_ratio > 0.0);
            assert!(v.opacity() > 0.0);
        });
        with_vsobject(&mut map, id2, |v| {
            assert!(v.center().x() > 0.5);
        });
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
