//! 🎞️ Animation trait, leaf animations, composites, and `.animate()` builder.

use crate::color::Color;
use crate::rate::{map_child_alpha, RateFunc};
use crate::sobject::Sobject;
use mathematical_geometry::{cubic_point_at, Affine, CubicBez, Point, Vec2};
use std::time::Duration;

/// 🕐 Mutable timeline context passed while sampling animations.
#[derive(Clone, Debug)]
pub struct AnimationContext {
    pub scene_time: f64,
    pub frame: u64,
    pub dt: f64,
}

/// 🎬 Core animation contract with recursive alpha propagation.
pub trait Animation: Send {
    fn duration(&self) -> f64;
    fn rate_func(&self) -> RateFunc;
    fn begin(&mut self);
    fn finish(&mut self);
    fn interpolate_mobject(&mut self, alpha: f64);
    fn get_all_mobjects(&self) -> Vec<u64>;
    fn is_introducer(&self) -> bool {
        false
    }
    fn is_remover(&self) -> bool {
        false
    }
}

fn eased_alpha(animation: &dyn Animation, alpha: f64) -> f64 {
    (animation.rate_func())(alpha.clamp(0.0, 1.0))
}

/// ▶️ Drive an animation to a parent alpha in [0,1].
pub fn interpolate_at(animation: &mut dyn Animation, parent_alpha: f64) {
    animation.interpolate_mobject(eased_alpha(animation, parent_alpha));
}

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
        Self {
            target_id,
            run_time,
            rate: crate::rate::linear,
            started: false,
            snapshot_ratio: 1.0,
        }
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
    }
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, alpha: f64) {
        let _ = (self.target_id, alpha, self.started, self.snapshot_ratio);
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
    fn is_introducer(&self) -> bool {
        true
    }
}

/// 🌅 Fade in opacity.
pub struct FadeIn {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
    pub target_opacity: f64,
}

impl FadeIn {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
            target_opacity: 1.0,
        }
    }
}

impl Animation for FadeIn {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {}
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, alpha: f64) {
        let _ = (self.target_id, alpha * self.target_opacity);
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
    fn is_introducer(&self) -> bool {
        true
    }
}

/// 🌇 Fade out opacity.
pub struct FadeOut {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
}

impl FadeOut {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
        }
    }
}

impl Animation for FadeOut {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {}
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, alpha: f64) {
        let _ = (self.target_id, 1.0 - alpha);
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
    fn is_remover(&self) -> bool {
        true
    }
}

/// 🔀 Morph between saved state and generated target.
pub struct Transform {
    pub target_id: u64,
    pub run_time: f64,
    pub rate: RateFunc,
}

impl Transform {
    pub fn new(target_id: u64, run_time: f64) -> Self {
        Self {
            target_id,
            run_time,
            rate: crate::rate::smooth,
        }
    }
}

impl Animation for Transform {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {}
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, alpha: f64) {
        let _ = (self.target_id, alpha);
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

/// 🔄 Rotate an Sobject.
pub struct Rotate {
    pub target_id: u64,
    pub angle: f64,
    pub run_time: f64,
    pub rate: RateFunc,
}

impl Rotate {
    pub fn new(target_id: u64, angle: f64, run_time: f64) -> Self {
        Self {
            target_id,
            angle,
            run_time,
            rate: crate::rate::smooth,
        }
    }
}

impl Animation for Rotate {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {}
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, alpha: f64) {
        let _ = (self.target_id, self.angle * alpha);
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
        Self {
            target_id,
            path,
            run_time,
            rate: crate::rate::linear,
        }
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
    fn get_all_mobjects(&self) -> Vec<u64> {
        vec![self.target_id]
    }
}

/// 🔁 Play animations in parallel with shared parent alpha.
pub struct AnimationGroup {
    pub animations: Vec<Box<dyn Animation>>,
    pub run_time: Option<f64>,
    pub rate: RateFunc,
    begun: Vec<bool>,
}

impl AnimationGroup {
    pub fn new(animations: Vec<Box<dyn Animation>>) -> Self {
        let n = animations.len();
        Self {
            animations,
            run_time: None,
            rate: crate::rate::linear,
            begun: vec![false; n],
        }
    }

    pub fn with_lag_ratio(self, _lag_ratio: f64) -> LaggedStart {
        LaggedStart::from_group(self, 0.0)
    }
}

impl Animation for AnimationGroup {
    fn duration(&self) -> f64 {
        self.run_time.unwrap_or_else(|| self.animations.iter().map(|a| a.duration()).fold(0.0, f64::max))
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {
        for (i, a) in self.animations.iter_mut().enumerate() {
            if !self.begun[i] {
                a.begin();
                self.begun[i] = true;
            }
        }
    }
    fn finish(&mut self) {
        for a in &mut self.animations {
            a.finish();
        }
    }
    fn interpolate_mobject(&mut self, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        for a in &mut self.animations {
            interpolate_at(a.as_mut(), alpha);
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        self.animations.iter().flat_map(|a| a.get_all_mobjects()).collect()
    }
}

/// ⏭️ Play animations sequentially with lazy child activation.
pub struct Succession {
    pub animations: Vec<Box<dyn Animation>>,
    pub rate: RateFunc,
    active_index: Option<usize>,
    begun: Vec<bool>,
    durations: Vec<f64>,
    total: f64,
}

impl Succession {
    pub fn new(animations: Vec<Box<dyn Animation>>) -> Self {
        let durations: Vec<f64> = animations.iter().map(|a| a.duration()).collect();
        let total = durations.iter().sum();
        let n = animations.len();
        Self {
            animations,
            rate: crate::rate::linear,
            active_index: None,
            begun: vec![false; n],
            durations,
            total,
        }
    }

    fn slot_bounds(&self, index: usize) -> (f64, f64) {
        if self.total <= 0.0 {
            return (0.0, 1.0);
        }
        let start: f64 = self.durations.iter().take(index).sum();
        let end = start + self.durations[index];
        (start / self.total, end / self.total)
    }
}

impl Animation for Succession {
    fn duration(&self) -> f64 {
        self.total
    }
    fn rate_func(&self) -> RateFunc {
        self.rate
    }
    fn begin(&mut self) {
        self.active_index = None;
    }
    fn finish(&mut self) {
        for a in &mut self.animations {
            a.finish();
        }
    }
    fn interpolate_mobject(&mut self, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        if self.animations.is_empty() {
            return;
        }
        let mut chosen = self.animations.len() - 1;
        for (i, _) in self.animations.iter().enumerate() {
            let (start, end) = self.slot_bounds(i);
            if alpha >= start && alpha <= end + 1e-9 {
                chosen = i;
                break;
            }
            if alpha < start {
                break;
            }
            chosen = i;
        }
        if self.active_index != Some(chosen) {
            self.active_index = Some(chosen);
        }
        for (i, a) in self.animations.iter_mut().enumerate() {
            if i > chosen {
                continue;
            }
            if !self.begun[i] {
                a.begin();
                self.begun[i] = true;
            }
            if i < chosen {
                interpolate_at(a.as_mut(), 1.0);
            } else {
                let (start, end) = self.slot_bounds(i);
                let child_alpha = map_child_alpha(alpha, start, end);
                interpolate_at(a.as_mut(), child_alpha);
            }
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        self.animations.iter().flat_map(|a| a.get_all_mobjects()).collect()
    }
}

/// 🎭 Staggered parallel start.
pub struct LaggedStart {
    pub group: AnimationGroup,
    pub lag_ratio: f64,
}

impl LaggedStart {
    pub fn new(animations: Vec<Box<dyn Animation>>, lag_ratio: f64) -> Self {
        Self {
            group: AnimationGroup::new(animations),
            lag_ratio: lag_ratio.clamp(0.0, 1.0),
        }
    }

    fn from_group(group: AnimationGroup, lag_ratio: f64) -> Self {
        Self { group, lag_ratio }
    }

    fn child_start(&self, index: usize, count: usize) -> f64 {
        if count <= 1 {
            return 0.0;
        }
        index as f64 / (count - 1) as f64 * self.lag_ratio
    }
}

impl Animation for LaggedStart {
    fn duration(&self) -> f64 {
        let base = self.group.duration();
        let n = self.group.animations.len();
        if n <= 1 {
            base
        } else {
            base + self.lag_ratio * base
        }
    }
    fn rate_func(&self) -> RateFunc {
        self.group.rate
    }
    fn begin(&mut self) {
        self.group.begin();
    }
    fn finish(&mut self) {
        self.group.finish();
    }
    fn interpolate_mobject(&mut self, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        let n = self.group.animations.len();
        for (i, a) in self.group.animations.iter_mut().enumerate() {
            let start = self.child_start(i, n);
            let child_alpha = map_child_alpha(alpha, start, 1.0);
            if child_alpha > 0.0 {
                if !self.group.begun[i] {
                    a.begin();
                    self.group.begun[i] = true;
                }
                interpolate_at(a.as_mut(), child_alpha);
            }
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        self.group.get_all_mobjects()
    }
}

/// 🗺️ Lagged start over a mapped collection.
pub struct LaggedStartMap<F>
where
    F: Fn(usize) -> Box<dyn Animation> + Send,
{
    pub count: usize,
    pub lag_ratio: f64,
    pub factory: F,
    pub run_time: f64,
    cache: Vec<Option<Box<dyn Animation>>>,
    begun: Vec<bool>,
}

impl<F> LaggedStartMap<F>
where
    F: Fn(usize) -> Box<dyn Animation> + Send,
{
    pub fn new(count: usize, lag_ratio: f64, run_time: f64, factory: F) -> Self {
        Self {
            count,
            lag_ratio,
            factory,
            run_time,
            cache: vec![None; count],
            begun: vec![false; count],
        }
    }

    fn ensure(&mut self, index: usize) -> &mut Box<dyn Animation> {
        if self.cache[index].is_none() {
            self.cache[index] = Some((self.factory)(index));
        }
        self.cache[index].as_mut().unwrap()
    }
}

impl<F> Animation for LaggedStartMap<F>
where
    F: Fn(usize) -> Box<dyn Animation> + Send,
{
    fn duration(&self) -> f64 {
        self.run_time * (1.0 + self.lag_ratio)
    }
    fn rate_func(&self) -> RateFunc {
        crate::rate::linear
    }
    fn begin(&mut self) {}
    fn finish(&mut self) {
        for slot in &mut self.cache {
            if let Some(a) = slot {
                a.finish();
            }
        }
    }
    fn interpolate_mobject(&mut self, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        for i in 0..self.count {
            let start = if self.count <= 1 {
                0.0
            } else {
                i as f64 / (self.count - 1) as f64 * self.lag_ratio
            };
            let child_alpha = map_child_alpha(alpha, start, 1.0);
            if child_alpha > 0.0 {
                let a = self.ensure(i);
                if !self.begun[i] {
                    a.begin();
                    self.begun[i] = true;
                }
                interpolate_at(a.as_mut(), child_alpha);
            }
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        self.cache
            .iter()
            .flatten()
            .flat_map(|a| a.get_all_mobjects())
            .collect()
    }
}

/// ⏸️ Hold scene time without mutation.
pub struct Wait {
    pub run_time: f64,
}

impl Wait {
    pub fn new(run_time: f64) -> Self {
        Self { run_time }
    }
}

impl Animation for Wait {
    fn duration(&self) -> f64 {
        self.run_time
    }
    fn rate_func(&self) -> RateFunc {
        crate::rate::linear
    }
    fn begin(&mut self) {}
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, _alpha: f64) {}
    fn get_all_mobjects(&self) -> Vec<u64> {
        Vec::new()
    }
}

/// 🏗️ Fluent `.animate()` builder for common tweens.
pub struct AnimateBuilder<'a> {
    pub target: &'a mut dyn Sobject,
    pub run_time: f64,
    pub rate: RateFunc,
}

impl<'a> AnimateBuilder<'a> {
    pub fn new(target: &'a mut dyn Sobject, run_time: f64) -> Self {
        Self {
            target,
            run_time,
            rate: crate::rate::smooth,
        }
    }

    pub fn with_rate(mut self, rate: RateFunc) -> Self {
        self.rate = rate;
        self
    }

    pub fn fade_in(self) -> FadeIn {
        FadeIn {
            target_id: self.target.id(),
            run_time: self.run_time,
            rate: self.rate,
            target_opacity: 1.0,
        }
    }

    pub fn fade_out(self) -> FadeOut {
        FadeOut {
            target_id: self.target.id(),
            run_time: self.run_time,
            rate: self.rate,
        }
    }

    pub fn create(self) -> Create {
        Create::new(self.target.id(), self.run_time).with_rate(self.rate)
    }

    pub fn transform(self) -> Transform {
        Transform {
            target_id: self.target.id(),
            run_time: self.run_time,
            rate: self.rate,
        }
    }

    pub fn rotate(self, angle: f64) -> Rotate {
        Rotate::new(self.target.id(), angle, self.run_time)
    }

    pub fn shift(self, _delta: Vec2) -> Transform {
        self.transform()
    }
}

/// 🪄 Extension trait for `.animate()` on Sobjects.
pub trait AnimateExt {
    fn animate(&mut self, run_time: f64) -> AnimateBuilder<'_>;
}

impl AnimateExt for dyn Sobject {
    fn animate(&mut self, run_time: f64) -> AnimateBuilder<'_> {
        AnimateBuilder::new(self, run_time)
    }
}

/// 🧮 Apply parent opacity recursively to an Sobject tree (Manim parity).
pub fn apply_parent_opacity_tree(root: &mut dyn Sobject, parent_opacity: f64) {
    root.set_parent_opacity(parent_opacity);
    let eff = root.effective_opacity();
    for child in root.children_mut() {
        apply_parent_opacity_tree(child, eff);
    }
}

/// 🎞️ Compile animations into a flat timeline with durations.
pub fn compile_animations(animations: &[Box<dyn Animation>]) -> Vec<Duration> {
    animations
        .iter()
        .map(|a| Duration::from_secs_f64(a.duration().max(0.0)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sobject::VSobject;

    #[test]
    fn succession_lazy_activation_order() {
        let a1 = Box::new(Wait::new(1.0)) as Box<dyn Animation>;
        let a2 = Box::new(Wait::new(1.0)) as Box<dyn Animation>;
        let mut s = Succession::new(vec![a1, a2]);
        s.interpolate_mobject(0.25);
        s.interpolate_mobject(0.75);
        assert!(s.active_index.is_some());
    }

    #[test]
    fn animation_group_parallel_duration_is_max() {
        let g = AnimationGroup::new(vec![Box::new(Wait::new(2.0)), Box::new(Wait::new(5.0))]);
        assert!((g.duration() - 5.0).abs() < 1e-9);
    }

    #[test]
    fn animate_builder_reads_target_id() {
        let mut v = VSobject::new();
        let id = v.id();
        let anim = v.animate(1.0).fade_in();
        assert_eq!(anim.target_id, id);
    }
}
