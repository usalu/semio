//! 🎞️ Animate app engine facet: 🎞️animation (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES:
//! relocated verbatim from the deleted artifact-tree `⚙️engine/🎞️animation`).

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod animation {
    //! 🎞️ Animation trait, leaf animations, composites, and `.animate()` builder.

    use crate::apps::present::engine::rate::rate::{map_child_alpha, RateFunc};
    use crate::apps::present::engine::scene::sobject::{Sobject, VSobject};
    use math::geometry::{cubic_point_at, Affine, CubicBez, Point, Vec2};
    use std::collections::HashMap;
    use std::time::Duration;

    /// 🕐️ Mutable timeline context passed while sampling animations.
    #[derive(Clone, Debug)]
    pub struct AnimationContext {
        pub scene_time: f64,
        pub frame: u64,
        pub dt: f64,
    }

    /// 🎬️ Core animation contract with recursive alpha propagation.
    pub trait Animation: Send {
        fn duration(&self) -> f64;
        fn rate_func(&self) -> RateFunc;
        fn begin(&mut self);
        fn finish(&mut self);
        fn interpolate_mobject(&mut self, alpha: f64);
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
            let rate = self.rate_func();
            let alpha = rate(parent_alpha.clamp(0.0, 1.0));
            let _ = mobjects;
            self.interpolate_mobject(alpha);
        }
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

    pub(crate) fn eased_alpha_for(animation: &dyn Animation, alpha: f64) -> f64 {
        eased_alpha(animation, alpha)
    }

    /// 🎯️ Resolve a VSobject by id and run a closure on it.
    pub fn with_vsobject<F>(mobjects: &mut HashMap<u64, Box<dyn Sobject>>, id: u64, f: F)
    where
        F: FnOnce(&mut VSobject),
    {
        if let Some(obj) = mobjects.get_mut(&id) {
            if let Some(v) = obj.as_any_mut().downcast_mut::<VSobject>() {
                f(v);
            }
        }
    }

    /// ▶️ Drive an animation to a parent alpha in [0,1], mutating scene mobjects.
    pub fn interpolate_at(mobjects: &mut HashMap<u64, Box<dyn Sobject>>, animation: &mut dyn Animation, parent_alpha: f64) {
        animation.apply(mobjects, parent_alpha);
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::linear, started: false, snapshot_ratio: 1.0 }
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
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
        start_opacity: f64,
        primed: bool,
    }

    impl FadeIn {
        pub fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, target_opacity: 1.0, start_opacity: 0.0, primed: false }
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
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
        start_opacity: f64,
        primed: bool,
    }

    impl FadeOut {
        pub fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, start_opacity: 1.0, primed: false }
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
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
        primed: bool,
    }

    impl Transform {
        pub fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, primed: false }
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
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
        start_transform: Option<Affine>,
    }

    impl Rotate {
        pub fn new(target_id: u64, angle: f64, run_time: f64) -> Self {
            Self { target_id, angle, run_time, rate: crate::apps::present::engine::rate::rate::smooth, start_transform: None }
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
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
            Self { target_id, path, run_time, rate: crate::apps::present::engine::rate::rate::linear }
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
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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

    /// 🔁️ Play animations in parallel with shared parent alpha.
    pub struct AnimationGroup {
        pub animations: Vec<Box<dyn Animation>>,
        pub run_time: Option<f64>,
        pub rate: RateFunc,
        begun: Vec<bool>,
    }

    impl AnimationGroup {
        pub fn new(animations: Vec<Box<dyn Animation>>) -> Self {
            let n = animations.len();
            Self { animations, run_time: None, rate: crate::apps::present::engine::rate::rate::linear, begun: vec![false; n] }
        }

        pub fn with_lag_ratio(self, lag_ratio: f64) -> LaggedStart {
            LaggedStart::from_group(self, lag_ratio)
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
                interpolate_at(&mut HashMap::new(), a.as_mut(), alpha);
            }
        }
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
            let alpha = eased_alpha(self, parent_alpha);
            for a in &mut self.animations {
                interpolate_at(mobjects, a.as_mut(), alpha);
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
            Self { animations, rate: crate::apps::present::engine::rate::rate::linear, active_index: None, begun: vec![false; n], durations, total }
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
            self.apply(&mut HashMap::new(), parent_alpha);
        }
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
            let alpha = eased_alpha(self, parent_alpha);
            if self.animations.is_empty() {
                return;
            }
            let bounds: Vec<(f64, f64)> = (0..self.animations.len()).map(|i| self.slot_bounds(i)).collect();
            let mut chosen = self.animations.len() - 1;
            for (i, (start, _)) in bounds.iter().enumerate() {
                if alpha >= *start {
                    chosen = i;
                }
                if alpha < *start {
                    break;
                }
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
                    interpolate_at(mobjects, a.as_mut(), 1.0);
                } else {
                    let (start, end) = bounds[i];
                    let child_alpha = map_child_alpha(alpha, start, end);
                    interpolate_at(mobjects, a.as_mut(), child_alpha);
                }
            }
        }
        fn get_all_mobjects(&self) -> Vec<u64> {
            self.animations.iter().flat_map(|a| a.get_all_mobjects()).collect()
        }
    }

    /// 🎭️ Staggered parallel start.
    pub struct LaggedStart {
        pub group: AnimationGroup,
        pub lag_ratio: f64,
    }

    impl LaggedStart {
        pub fn new(animations: Vec<Box<dyn Animation>>, lag_ratio: f64) -> Self {
            Self { group: AnimationGroup::new(animations), lag_ratio: lag_ratio.clamp(0.0, 1.0) }
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
            self.apply(&mut HashMap::new(), parent_alpha);
        }
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
            let alpha = eased_alpha(self, parent_alpha);
            let n = self.group.animations.len();
            let starts: Vec<f64> = (0..n).map(|i| self.child_start(i, n)).collect();
            for (i, a) in self.group.animations.iter_mut().enumerate() {
                let start = starts[i];
                let child_alpha = map_child_alpha(alpha, start, 1.0);
                if child_alpha > 0.0 {
                    if !self.group.begun[i] {
                        a.begin();
                        self.group.begun[i] = true;
                    }
                    interpolate_at(mobjects, a.as_mut(), child_alpha);
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
            Self { count, lag_ratio, factory, run_time, cache: (0..count).map(|_| None).collect(), begun: vec![false; count] }
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
            crate::apps::present::engine::rate::rate::linear
        }
        fn begin(&mut self) {}
        fn finish(&mut self) {
            for a in self.cache.iter_mut().flatten() {
                a.finish();
            }
        }
        fn interpolate_mobject(&mut self, parent_alpha: f64) {
            self.apply(&mut HashMap::new(), parent_alpha);
        }
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
            let alpha = eased_alpha(self, parent_alpha);
            for i in 0..self.count {
                let start = if self.count <= 1 { 0.0 } else { i as f64 / (self.count - 1) as f64 * self.lag_ratio };
                let child_alpha = map_child_alpha(alpha, start, 1.0);
                if child_alpha <= 0.0 {
                    continue;
                }
                if !self.begun[i] {
                    if self.cache[i].is_none() {
                        self.cache[i] = Some((self.factory)(i));
                    }
                    if let Some(a) = self.cache[i].as_mut() {
                        a.begin();
                    }
                    self.begun[i] = true;
                }
                if let Some(a) = self.cache[i].as_mut() {
                    interpolate_at(mobjects, a.as_mut(), child_alpha);
                }
            }
        }
        fn get_all_mobjects(&self) -> Vec<u64> {
            self.cache.iter().flatten().flat_map(|a| a.get_all_mobjects()).collect()
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
            crate::apps::present::engine::rate::rate::linear
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
            Self { target, run_time, rate: crate::apps::present::engine::rate::rate::smooth }
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
        start_transform: Option<Affine>,
    }

    impl Shift {
        pub fn new(target_id: u64, delta: Vec2, run_time: f64) -> Self {
            Self { target_id, delta, run_time, rate: crate::apps::present::engine::rate::rate::smooth, start_transform: None }
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
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
        start_transform: Option<Affine>,
    }

    impl ApplyMethod {
        pub fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, scale_factor: 1.2, start_transform: None }
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
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
        start_transform: Option<Affine>,
    }

    impl FocusOn {
        pub fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::there_and_back, start_transform: None }
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
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
        start_opacity: f64,
        primed: bool,
    }

    impl Blink {
        pub fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::there_and_back, start_opacity: 1.0, primed: false }
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
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
        primed: bool,
    }

    impl TracedPath {
        pub fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::linear, primed: false }
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
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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
        pub animation: Box<dyn Animation>,
        pub speed_factor: f64,
    }

    impl ChangeSpeed {
        pub fn new(animation: Box<dyn Animation>, speed_factor: f64) -> Self {
            Self { animation, speed_factor: speed_factor.max(1e-9) }
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
        fn apply(&mut self, mobjects: &mut HashMap<u64, Box<dyn Sobject>>, parent_alpha: f64) {
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

    /// 🪄️ Extension trait for `.animate()` on Sobjects.
    pub trait AnimateExt: Sobject + Sized {
        fn animate(&mut self, run_time: f64) -> AnimateBuilder<'_> {
            AnimateBuilder::new(self, run_time)
        }
    }

    impl<T: Sobject + Sized> AnimateExt for T {}

    /// 🧮️ Apply parent opacity recursively to an Sobject tree (Manim parity).
    pub fn apply_parent_opacity_tree(root: &mut dyn Sobject, parent_opacity: f64) {
        root.set_parent_opacity(parent_opacity);
        let eff = root.effective_opacity();
        root.visit_children_mut(&mut |child| apply_parent_opacity_tree(child, eff));
    }

    /// 🎞️ Compile animations into a flat timeline with durations.
    pub fn compile_animations(animations: &[Box<dyn Animation>]) -> Vec<Duration> {
        animations.iter().map(|a| Duration::from_secs_f64(a.duration().max(0.0))).collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::apps::present::engine::animation::animation::AnimateExt;
        use crate::apps::present::engine::scene::sobject::VSobject;

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
}

pub mod animations_catalog {
    //! 📚️ Extended Manim-parity animation catalog wired to the Animation trait.

    use crate::apps::present::engine::animation::animation::{eased_alpha_for, with_vsobject, Animation};
    use crate::apps::present::engine::rate::rate::RateFunc;
    use crate::apps::present::engine::scene::sobject::Sobject;
    use math::geometry::{Affine, Point, Vec2};
    use std::collections::HashMap;

    fn scale_about_center(base: Affine, center: Point, factor: f64) -> Affine {
        let t = Affine::IDENTITY.translate((center.x(), center.y())) * Affine::IDENTITY.scale(factor) * Affine::IDENTITY.translate((-center.x(), -center.y()));
        base * t
    }

    fn rotate_about_center(base: Affine, center: Point, angle: f64) -> Affine {
        let t = Affine::IDENTITY.translate((center.x(), center.y())) * Affine::IDENTITY.rotate(angle) * Affine::IDENTITY.translate((-center.x(), -center.y()));
        base * t
    }

    fn lerp_point(a: Point, b: Point, t: f64) -> Point {
        Point::new(a.x() + (b.x() - a.x()) * t, a.y() + (b.y() - a.y()) * t)
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, primed: false, fill_opacity: 1.0 }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, primed: false }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, primed: false }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, primed: false, start_transform: None }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, primed: false }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, primed: false }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::there_and_back, start_opacity: 1.0, primed: false }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::there_and_back, start_transform: None }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, grow_point: Point::ZERO, start_transform: None }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, start_transform: None, start_opacity: 1.0, primed: false }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, angle: std::f64::consts::TAU, start_transform: None }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, value: 0.0, start_opacity: 1.0, primed: false }
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
        start_transform: Option<Affine>,
    }

    impl Broadcast {
        pub fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::there_and_back, start_transform: None }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, amplitude: 0.2, start_transform: None }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::there_and_back, angle: 0.1, start_transform: None }
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
            Self { cycle_ids: vec![target_id], run_time, rate: crate::apps::present::engine::rate::rate::smooth, start_centers: Vec::new() }
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
            Self { target_id, swap_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, a_center: None, b_center: None }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, primed: false }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, start_transform: None }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::linear, primed: false }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, angle: std::f64::consts::TAU * 2.0, origin: Point::ZERO, start_transform: None }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, primed: false }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, primed: false }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::smooth, start_transform: None }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::there_and_back, start_transform: None }
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
            Self { target_id, run_time, rate: crate::apps::present::engine::rate::rate::linear, angle: std::f64::consts::TAU, start_transform: None }
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
        use crate::apps::present::engine::text::color::Color;
        use crate::apps::present::engine::geometry::geometry::circle;
        use crate::apps::present::engine::scene::sobject::VSobject;

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
}
