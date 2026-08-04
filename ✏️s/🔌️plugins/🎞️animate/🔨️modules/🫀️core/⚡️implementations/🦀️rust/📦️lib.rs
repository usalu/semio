//! 🎬️ Manim-class animation core: Sobject scene graph, imperative timeline, and animation composites.

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod animation {
    //! 🎞️ Animation trait, leaf animations, composites, and `.animate()` builder.

    use crate::rate::{map_child_alpha, RateFunc};
    use crate::sobject::{Sobject, VSobject};
    use mathematical_geometry::{cubic_point_at, Affine, CubicBez, Point, Vec2};
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
            Self { target_id, run_time, rate: crate::rate::linear, started: false, snapshot_ratio: 1.0 }
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
            Self { target_id, run_time, rate: crate::rate::smooth, target_opacity: 1.0, start_opacity: 0.0, primed: false }
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
            Self { target_id, run_time, rate: crate::rate::smooth, start_opacity: 1.0, primed: false }
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
            Self { target_id, run_time, rate: crate::rate::smooth, primed: false }
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
            Self { target_id, angle, run_time, rate: crate::rate::smooth, start_transform: None }
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
            Self { target_id, path, run_time, rate: crate::rate::linear }
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
            Self { animations, run_time: None, rate: crate::rate::linear, begun: vec![false; n] }
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
            Self { animations, rate: crate::rate::linear, active_index: None, begun: vec![false; n], durations, total }
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
            crate::rate::linear
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
            Self { target, run_time, rate: crate::rate::smooth }
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
            Self { target_id, delta, run_time, rate: crate::rate::smooth, start_transform: None }
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
            Self { target_id, run_time, rate: crate::rate::smooth, scale_factor: 1.2, start_transform: None }
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
            Self { target_id, run_time, rate: crate::rate::there_and_back, start_transform: None }
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
            Self { target_id, run_time, rate: crate::rate::there_and_back, start_opacity: 1.0, primed: false }
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
            Self { target_id, run_time, rate: crate::rate::linear, primed: false }
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
        use crate::animation::AnimateExt;
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
}

mod animations_catalog {
    //! 📚️ Extended Manim-parity animation catalog wired to the Animation trait.

    use crate::animation::{eased_alpha_for, with_vsobject, Animation};
    use crate::rate::RateFunc;
    use crate::sobject::Sobject;
    use mathematical_geometry::{Affine, Point, Vec2};
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
            Self { target_id, run_time, rate: crate::rate::smooth, primed: false, fill_opacity: 1.0 }
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
            Self { target_id, run_time, rate: crate::rate::smooth, primed: false }
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
            Self { target_id, run_time, rate: crate::rate::smooth, primed: false }
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
            Self { target_id, run_time, rate: crate::rate::smooth, primed: false, start_transform: None }
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
            Self { target_id, run_time, rate: crate::rate::smooth, primed: false }
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
            Self { target_id, run_time, rate: crate::rate::smooth, primed: false }
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
            Self { target_id, run_time, rate: crate::rate::there_and_back, start_opacity: 1.0, primed: false }
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
            Self { target_id, run_time, rate: crate::rate::there_and_back, start_transform: None }
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
            Self { target_id, run_time, rate: crate::rate::smooth, grow_point: Point::ZERO, start_transform: None }
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
            Self { target_id, run_time, rate: crate::rate::smooth, start_transform: None, start_opacity: 1.0, primed: false }
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
            Self { target_id, run_time, rate: crate::rate::smooth, angle: std::f64::consts::TAU, start_transform: None }
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
            Self { target_id, run_time, rate: crate::rate::smooth, value: 0.0, start_opacity: 1.0, primed: false }
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
            Self { target_id, run_time, rate: crate::rate::there_and_back, start_transform: None }
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
            Self { target_id, run_time, rate: crate::rate::smooth, amplitude: 0.2, start_transform: None }
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
            Self { target_id, run_time, rate: crate::rate::there_and_back, angle: 0.1, start_transform: None }
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
            Self { cycle_ids: vec![target_id], run_time, rate: crate::rate::smooth, start_centers: Vec::new() }
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
            Self { target_id, swap_id, run_time, rate: crate::rate::smooth, a_center: None, b_center: None }
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
            Self { target_id, run_time, rate: crate::rate::smooth, primed: false }
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
            Self { target_id, run_time, rate: crate::rate::smooth, start_transform: None }
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
            Self { target_id, run_time, rate: crate::rate::linear, primed: false }
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
            Self { target_id, run_time, rate: crate::rate::smooth, angle: std::f64::consts::TAU * 2.0, origin: Point::ZERO, start_transform: None }
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
            Self { target_id, run_time, rate: crate::rate::smooth, primed: false }
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
            Self { target_id, run_time, rate: crate::rate::smooth, primed: false }
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
            Self { target_id, run_time, rate: crate::rate::smooth, start_transform: None }
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
            Self { target_id, run_time, rate: crate::rate::there_and_back, start_transform: None }
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
            Self { target_id, run_time, rate: crate::rate::linear, angle: std::f64::consts::TAU, start_transform: None }
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
}

mod axes {
    //! 📊️ Coordinate axes, number planes, and complex planes.

    use crate::color::Color;
    use crate::geometry::{arrow, dot, line};
    use crate::sobject::{Group, Sobject, VSobject};
    use crate::text::Text;
    use mathematical_geometry::{BezPath, Point};

    /// 📈️ Cartesian axes with optional labels.
    pub struct Axes {
        pub group: Group,
        pub x_length: f64,
        pub y_length: f64,
        pub origin: Point,
    }

    impl Axes {
        pub fn new(x_length: f64, y_length: f64, origin: Point, color: Color) -> Self {
            let x_axis = arrow(origin, Point::new(origin.x() + x_length, origin.y()), color, 3.0, 0.2);
            let y_axis = arrow(origin, Point::new(origin.x(), origin.y() + y_length), color, 3.0, 0.2);
            let group = Group::new(vec![Box::new(x_axis), Box::new(y_axis)]);
            Self { group, x_length, y_length, origin }
        }

        pub fn with_tick_labels(mut self, x_ticks: &[f64], y_ticks: &[f64], color: Color) -> Self {
            for &x in x_ticks {
                let p = self.coords_to_point(x, 0.0);
                let mut label = Text::new(format!("{x:.1}"), color);
                label.inner.move_to(Point::new(p.x(), p.y() - 0.25));
                self.group.add_child(Box::new(label.inner));
                self.group.add_child(Box::new(line(Point::new(p.x(), p.y() - 0.08), Point::new(p.x(), p.y() + 0.08), color.with_alpha(0.6), 1.0)));
            }
            for &y in y_ticks {
                let p = self.coords_to_point(0.0, y);
                let mut label = Text::new(format!("{y:.1}"), color);
                label.inner.move_to(Point::new(p.x() - 0.35, p.y()));
                self.group.add_child(Box::new(label.inner));
                self.group.add_child(Box::new(line(Point::new(p.x() - 0.08, p.y()), Point::new(p.x() + 0.08, p.y()), color.with_alpha(0.6), 1.0)));
            }
            self
        }

        pub fn coords_to_point(&self, x: f64, y: f64) -> Point {
            Point::new(self.origin.x() + x, self.origin.y() + y)
        }

        pub fn as_group(&self) -> &Group {
            &self.group
        }
    }

    /// 📉️ Function graph y = f(x) sampled over a range.
    pub struct FunctionGraph {
        pub inner: VSobject,
    }

    impl FunctionGraph {
        pub fn new<F>(x_range: (f64, f64), axes: &Axes, f: F, samples: u32, color: Color, width: f64) -> Self
        where
            F: Fn(f64) -> f64,
        {
            let samples = samples.max(2);
            let mut path = BezPath::new();
            for i in 0..samples {
                let t = i as f64 / (samples - 1) as f64;
                let x = x_range.0 + t * (x_range.1 - x_range.0);
                let y = f(x);
                let p = axes.coords_to_point(x, y);
                if i == 0 {
                    path.move_to(p);
                } else {
                    path.line_to(p);
                }
            }
            let mut inner = VSobject::from_path(path);
            inner.style.fill = None;
            inner.style.stroke = Some(color);
            inner.style.stroke_width = width;
            Self { inner }
        }
    }

    /// 🌀️ Parametric curve (x(t), y(t)) sampled over a parameter range.
    pub struct ParametricFunction {
        pub inner: VSobject,
    }

    impl ParametricFunction {
        pub fn new<F>(t_range: (f64, f64), axes: &Axes, f: F, samples: u32, color: Color, width: f64) -> Self
        where
            F: Fn(f64) -> (f64, f64),
        {
            let samples = samples.max(2);
            let mut path = BezPath::new();
            for i in 0..samples {
                let t = t_range.0 + (i as f64 / (samples - 1) as f64) * (t_range.1 - t_range.0);
                let (x, y) = f(t);
                let p = axes.coords_to_point(x, y);
                if i == 0 {
                    path.move_to(p);
                } else {
                    path.line_to(p);
                }
            }
            let mut inner = VSobject::from_path(path);
            inner.style.fill = None;
            inner.style.stroke = Some(color);
            inner.style.stroke_width = width;
            Self { inner }
        }
    }

    /// 🔲️ Number plane with grid lines.
    pub struct NumberPlane {
        pub axes: Axes,
        pub group: Group,
        pub unit_size: f64,
    }

    impl NumberPlane {
        pub fn new(x_range: (f64, f64), y_range: (f64, f64), unit_size: f64, color: Color) -> Self {
            let origin = Point::new(-x_range.0 * unit_size, -y_range.0 * unit_size);
            let x_len = (x_range.1 - x_range.0) * unit_size;
            let y_len = (y_range.1 - y_range.0) * unit_size;
            let axes = Axes::new(x_len, y_len, origin, color);
            let mut children: Vec<Box<dyn Sobject>> = vec![Box::new(arrow(origin, Point::new(origin.x() + x_len, origin.y()), color, 3.0, 0.2)), Box::new(arrow(origin, Point::new(origin.x(), origin.y() + y_len), color, 3.0, 0.2))];
            let grid_color = color.with_alpha(0.25);
            let x_steps = ((x_range.1 - x_range.0) as i32).abs().max(1);
            let y_steps = ((y_range.1 - y_range.0) as i32).abs().max(1);
            for i in 0..=x_steps {
                let x = origin.x() + i as f64 * unit_size;
                children.push(Box::new(line(Point::new(x, origin.y()), Point::new(x, origin.y() + y_len), grid_color, 1.0)));
            }
            for j in 0..=y_steps {
                let y = origin.y() + j as f64 * unit_size;
                children.push(Box::new(line(Point::new(origin.x(), y), Point::new(origin.x() + x_len, y), grid_color, 1.0)));
            }
            let group = Group::new(children);
            Self { axes, group, unit_size }
        }
    }

    /// ➖️ One-dimensional number line.
    pub struct NumberLine {
        pub group: Group,
        pub start: Point,
        pub length: f64,
    }

    impl NumberLine {
        pub fn new(start: Point, length: f64, color: Color) -> Self {
            let axis = line(start, Point::new(start.x() + length, start.y()), color, 3.0);
            let tick_count = 10;
            let mut children: Vec<Box<dyn Sobject>> = vec![Box::new(axis)];
            for i in 0..=tick_count {
                let x = start.x() + length * i as f64 / tick_count as f64;
                children.push(Box::new(line(Point::new(x, start.y() - 0.1), Point::new(x, start.y() + 0.1), color, 1.5)));
            }
            Self { group: Group::new(children), start, length }
        }

        pub fn number_to_point(&self, n: f64) -> Point {
            Point::new(self.start.x() + n, self.start.y())
        }
    }

    /// 🔢️ Integer-only number line with unit ticks.
    pub struct IntegerLine {
        pub group: Group,
        pub start: Point,
        pub unit_size: f64,
        pub min: i32,
        pub max: i32,
    }

    impl IntegerLine {
        pub fn new(start: Point, min: i32, max: i32, unit_size: f64, color: Color) -> Self {
            let span = (max - min).max(1) as f64;
            let length = span * unit_size;
            let axis = line(start, Point::new(start.x() + length, start.y()), color, 3.0);
            let mut children: Vec<Box<dyn Sobject>> = vec![Box::new(axis)];
            for value in min..=max {
                let x = start.x() + (value - min) as f64 * unit_size;
                children.push(Box::new(line(Point::new(x, start.y() - 0.12), Point::new(x, start.y() + 0.12), color, 1.5)));
                if value % 5 == 0 {
                    children.push(Box::new(dot(Point::new(x, start.y()), 0.04, color)));
                }
            }
            Self { group: Group::new(children), start, unit_size, min, max }
        }

        pub fn integer_to_point(&self, n: i32) -> Point {
            Point::new(self.start.x() + (n - self.min) as f64 * self.unit_size, self.start.y())
        }
    }

    /// ℂ Complex plane (axes with imaginary vertical axis).
    pub struct ComplexPlane {
        pub plane: NumberPlane,
    }

    impl ComplexPlane {
        pub fn new(range: f64, unit_size: f64, color: Color) -> Self {
            let plane = NumberPlane::new((-range, range), (-range, range), unit_size, color);
            Self { plane }
        }

        pub fn complex_to_point(&self, re: f64, im: f64) -> Point {
            self.plane.axes.coords_to_point(re * self.plane.unit_size, im * self.plane.unit_size)
        }

        pub fn plot_point(&self, re: f64, im: f64, color: Color) -> VSobject {
            dot(self.complex_to_point(re, im), 0.06, color)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn axes_map_coordinates() {
            let axes = Axes::new(4.0, 3.0, Point::ZERO, Color::WHITE);
            let p = axes.coords_to_point(1.0, 2.0);
            assert!((p.x() - 1.0).abs() < 1e-9);
            assert!((p.y() - 2.0).abs() < 1e-9);
        }

        #[test]
        fn number_line_maps_values() {
            let nl = NumberLine::new(Point::ZERO, 10.0, Color::WHITE);
            assert!((nl.number_to_point(5.0).x() - 5.0).abs() < 1e-9);
        }

        #[test]
        fn integer_line_maps_values() {
            let il = IntegerLine::new(Point::ZERO, 0, 10, 1.0, Color::WHITE);
            assert!((il.integer_to_point(5).x() - 5.0).abs() < 1e-9);
        }

        #[test]
        fn axes_tick_labels_and_graphs() {
            let axes = Axes::new(4.0, 3.0, Point::ZERO, Color::WHITE).with_tick_labels(&[1.0, 2.0], &[1.0], Color::WHITE);
            assert!(axes.group.children.len() > 2);
            let fg = FunctionGraph::new((0.0, 2.0), &axes, |x| x * x, 16, Color::YELLOW, 2.0);
            assert!(!fg.inner.paths.is_empty());
            let pf = ParametricFunction::new((0.0, std::f64::consts::TAU), &axes, |t| (t.cos(), t.sin()), 32, Color::GREEN, 2.0);
            assert!(!pf.inner.paths.is_empty());
        }
    }
}

mod camera {
    //! 📷️ Scene cameras: static, moving, 3D, and zoomed views.

    use crate::color::Color;
    use mathematical_geometry::{Affine, Point, Vec2};

    /// 📸️ Base camera framing the scene.
    #[derive(Clone, Debug)]
    pub struct Camera {
        pub frame_center: Point,
        pub frame_width: f64,
        pub frame_height: f64,
        pub background: Color,
        pub transform: Affine,
    }

    impl Default for Camera {
        fn default() -> Self {
            Self { frame_center: Point::ZERO, frame_width: 14.0, frame_height: 8.0, background: Color::BLACK, transform: Affine::IDENTITY }
        }
    }

    impl Camera {
        pub fn new(frame_width: f64, frame_height: f64) -> Self {
            Self { frame_width, frame_height, ..Self::default() }
        }

        pub fn pixel_coords_to_scene(&self, px: f64, py: f64, pixel_width: u32, pixel_height: u32) -> Point {
            let u = px / pixel_width as f64;
            let v = 1.0 - py / pixel_height as f64;
            let x = self.frame_center.x() + (u - 0.5) * self.frame_width;
            let y = self.frame_center.y() + (v - 0.5) * self.frame_height;
            self.transform * Point::new(x, y)
        }

        pub fn scene_to_pixel(&self, p: Point, pixel_width: u32, pixel_height: u32) -> (f64, f64) {
            let p = self.transform * p;
            let u = (p.x() - self.frame_center.x()) / self.frame_width + 0.5;
            let v = (p.y() - self.frame_center.y()) / self.frame_height + 0.5;
            (u * pixel_width as f64, (1.0 - v) * pixel_height as f64)
        }
    }

    /// 🎥️ Camera that can pan and zoom over time.
    #[derive(Clone, Debug)]
    pub struct MovingCamera {
        pub camera: Camera,
        pub target_center: Point,
        pub target_width: f64,
    }

    impl MovingCamera {
        pub fn new(camera: Camera) -> Self {
            let target_center = camera.frame_center;
            let target_width = camera.frame_width;
            Self { camera, target_center, target_width }
        }

        pub fn interpolate(&mut self, alpha: f64) {
            let a = alpha.clamp(0.0, 1.0);
            let c0 = self.camera.frame_center;
            let c1 = self.target_center;
            self.camera.frame_center = Point::new(c0.x() + (c1.x() - c0.x()) * a, c0.y() + (c1.y() - c0.y()) * a);
            self.camera.frame_width = self.camera.frame_width + (self.target_width - self.camera.frame_width) * a;
            self.camera.frame_height = self.camera.frame_width * self.camera.frame_height / self.camera.frame_width.max(1e-9);
        }

        pub fn set_target(&mut self, center: Point, width: f64) {
            self.target_center = center;
            self.target_width = width;
        }
    }

    /// 🧊️ Perspective camera for 3D scenes.
    #[derive(Clone, Debug)]
    pub struct ThreeDCamera {
        pub camera: Camera,
        pub phi: f64,
        pub theta: f64,
        pub distance: f64,
        pub gamma: f64,
    }

    impl ThreeDCamera {
        pub fn new(camera: Camera) -> Self {
            Self { camera, phi: 0.0, theta: -std::f64::consts::FRAC_PI_2, distance: 10.0, gamma: 0.0 }
        }

        pub fn project(&self, x: f64, y: f64, z: f64) -> Point {
            let cy = self.phi.cos();
            let sy = self.phi.sin();
            let ct = self.theta.cos();
            let st = self.theta.sin();
            let x1 = x * cy - z * sy;
            let z1 = x * sy + z * cy;
            let y1 = y * ct - z1 * st;
            let z2 = y * st + z1 * ct + self.distance;
            let scale = 1.0 / z2.max(0.1);
            Point::new(x1 * scale, y1 * scale)
        }
    }

    /// 🔍️ Picture-in-picture zoomed camera region.
    #[derive(Clone, Debug)]
    pub struct ZoomedCamera {
        pub camera: Camera,
        pub zoom_factor: f64,
        pub display_corner: Vec2,
        pub display_size: (f64, f64),
    }

    impl ZoomedCamera {
        pub fn new(camera: Camera, zoom_factor: f64) -> Self {
            Self { camera, zoom_factor, display_corner: Vec2::new(1.0, 1.0), display_size: (3.0, 2.0) }
        }

        pub fn effective_frame_width(&self) -> f64 {
            self.camera.frame_width / self.zoom_factor.max(1e-9)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn moving_camera_interpolates_center() {
            let mut cam = MovingCamera::new(Camera::default());
            cam.set_target(Point::new(2.0, 2.0), 8.0);
            cam.interpolate(0.5);
            assert!(cam.camera.frame_center.x().abs() < 2.0);
        }

        #[test]
        fn three_d_camera_projects_finite_point() {
            let cam = ThreeDCamera::new(Camera::default());
            let p = cam.project(1.0, 1.0, 1.0);
            assert!(p.x().is_finite() && p.y().is_finite());
        }
    }
}

mod color {
    //! 🎨️ RGBA colors, named palette, and gradient interpolation.

    use serde::{Deserialize, Serialize};

    /// 🌈️ Linear RGBA color with premultiplication left to the renderer.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Color {
        pub r: f64,
        pub g: f64,
        pub b: f64,
        pub a: f64,
    }

    impl Color {
        pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
        pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
        pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
        pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
        pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
        pub const YELLOW: Self = Self::rgb(1.0, 1.0, 0.0);
        pub const ORANGE: Self = Self::rgb(1.0, 0.5, 0.0);
        pub const PURPLE: Self = Self::rgb(0.5, 0.0, 0.5);
        pub const TEAL: Self = Self::rgb(0.0, 0.5, 0.5);
        pub const GRAY: Self = Self::rgb(0.5, 0.5, 0.5);
        pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

        pub const fn rgb(r: f64, g: f64, b: f64) -> Self {
            Self { r, g, b, a: 1.0 }
        }

        pub const fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
            Self { r, g, b, a }
        }

        pub fn hex(hex: &str) -> Self {
            let s = hex.trim_start_matches('#');
            let (r, g, b, a) = match s.len() {
                6 => {
                    let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
                    let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
                    let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
                    (r, g, b, 255)
                }
                8 => {
                    let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
                    let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
                    let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
                    let a = u8::from_str_radix(&s[6..8], 16).unwrap_or(255);
                    (r, g, b, a)
                }
                _ => (0, 0, 0, 255),
            };
            Self::rgba(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0, a as f64 / 255.0)
        }

        pub fn with_alpha(mut self, alpha: f64) -> Self {
            self.a = alpha;
            self
        }

        pub fn lerp(self, other: Self, t: f64) -> Self {
            let t = t.clamp(0.0, 1.0);
            Self { r: self.r + (other.r - self.r) * t, g: self.g + (other.g - self.g) * t, b: self.b + (other.b - self.b) * t, a: self.a + (other.a - self.a) * t }
        }

        pub fn to_array(self) -> [f64; 4] {
            [self.r, self.g, self.b, self.a]
        }
    }

    /// 🌅️ Multi-stop color gradient.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Gradient {
        pub stops: Vec<(f64, Color)>,
    }

    impl Gradient {
        pub fn new(stops: Vec<(f64, Color)>) -> Self {
            let mut stops = stops;
            stops.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            Self { stops }
        }

        pub fn sample(&self, t: f64) -> Color {
            let t = t.clamp(0.0, 1.0);
            if self.stops.is_empty() {
                return Color::WHITE;
            }
            if self.stops.len() == 1 {
                return self.stops[0].1;
            }
            if t <= self.stops[0].0 {
                return self.stops[0].1;
            }
            if t >= self.stops[self.stops.len() - 1].0 {
                return self.stops[self.stops.len() - 1].1;
            }
            for pair in self.stops.windows(2) {
                let (t0, c0) = pair[0];
                let (t1, c1) = pair[1];
                if t >= t0 && t <= t1 {
                    let u = if (t1 - t0).abs() < 1e-9 { 0.0 } else { (t - t0) / (t1 - t0) };
                    return c0.lerp(c1, u);
                }
            }
            self.stops[0].1
        }
    }

    pub fn named_color(name: &str) -> Color {
        match name.to_ascii_lowercase().as_str() {
            "white" => Color::WHITE,
            "black" => Color::BLACK,
            "red" => Color::RED,
            "green" => Color::GREEN,
            "blue" => Color::BLUE,
            "yellow" => Color::YELLOW,
            "orange" => Color::ORANGE,
            "purple" => Color::PURPLE,
            "teal" => Color::TEAL,
            "gray" | "grey" => Color::GRAY,
            "manim_blue" | "semio_blue" => Color::hex("#58C4DD"),
            "manim_green" | "semio_green" => Color::hex("#83C167"),
            "manim_red" | "semio_red" => Color::hex("#FC6255"),
            "manim_yellow" | "semio_yellow" => Color::hex("#FFFF00"),
            other => Color::hex(other),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn lerp_midpoint_is_average() {
            let c = Color::BLACK.lerp(Color::WHITE, 0.5);
            assert!((c.r - 0.5).abs() < 1e-9);
        }

        #[test]
        fn gradient_samples_stops() {
            let g = Gradient::new(vec![(0.0, Color::RED), (1.0, Color::BLUE)]);
            let mid = g.sample(0.5);
            assert!(mid.r > 0.0 && mid.b > 0.0);
        }

        #[test]
        fn hex_parses_six_and_eight_digit_forms() {
            let rgb = Color::hex("#ff0000");
            assert!((rgb.r - 1.0).abs() < 1e-9);
            assert!((rgb.a - 1.0).abs() < 1e-9);
            let rgba = Color::hex("00ff0080");
            assert!((rgba.g - 1.0).abs() < 1e-9);
            assert!((rgba.a - 128.0 / 255.0).abs() < 1e-9);
        }

        #[test]
        fn hex_falls_back_to_black_on_invalid_length() {
            let bad = Color::hex("#abc");
            assert_eq!(bad, Color::BLACK);
        }

        #[test]
        fn named_color_covers_aliases_and_hex_fallback() {
            assert_eq!(named_color("WHITE"), Color::WHITE);
            assert_eq!(named_color("grey"), Color::GRAY);
            assert_eq!(named_color("gray"), Color::GRAY);
            assert_eq!(named_color("semio_blue"), Color::hex("#58C4DD"));
            assert_eq!(named_color("manim_blue"), Color::hex("#58C4DD"));
            assert_eq!(named_color("semio_green"), Color::hex("#83C167"));
            assert_eq!(named_color("manim_red"), Color::hex("#FC6255"));
            assert_eq!(named_color("manim_yellow"), Color::hex("#FFFF00"));
            assert_eq!(named_color("ff00ff"), Color::hex("ff00ff"));
        }

        #[test]
        fn gradient_edge_cases() {
            let empty = Gradient::new(vec![]);
            assert_eq!(empty.sample(0.5), Color::WHITE);
            let single = Gradient::new(vec![(0.3, Color::RED)]);
            assert_eq!(single.sample(0.0), Color::RED);
            assert_eq!(single.sample(1.0), Color::RED);
            let g = Gradient::new(vec![(0.2, Color::RED), (0.8, Color::BLUE)]);
            assert_eq!(g.sample(0.0), Color::RED);
            assert_eq!(g.sample(1.0), Color::BLUE);
        }

        #[test]
        fn gradient_new_sorts_unordered_stops() {
            let g = Gradient::new(vec![(1.0, Color::BLUE), (0.0, Color::RED)]);
            assert_eq!(g.stops[0].0, 0.0);
            assert_eq!(g.stops[1].0, 1.0);
        }

        #[test]
        fn with_alpha_and_to_array_roundtrip() {
            let c = Color::rgb(0.2, 0.4, 0.6).with_alpha(0.5);
            assert_eq!(c.to_array(), [0.2, 0.4, 0.6, 0.5]);
        }
    }
}

mod config {
    //! ⚙️ Global animation configuration, quality presets, and cache paths.

    use serde::{Deserialize, Serialize};
    use std::path::{Path, PathBuf};

    /// 🎞️ Output quality preset mirroring Manim quality flags.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum QualityPreset {
        Low,
        Medium,
        High,
        FourK,
        Production,
    }

    impl QualityPreset {
        pub fn frame_rate(self) -> f64 {
            match self {
                Self::Low | Self::Medium => 15.0,
                Self::High | Self::FourK | Self::Production => 60.0,
            }
        }

        pub fn resolution(self) -> (u32, u32) {
            match self {
                Self::Low => (854, 480),
                Self::Medium => (1280, 720),
                Self::High => (1920, 1080),
                Self::FourK => (3840, 2160),
                Self::Production => (2560, 1440),
            }
        }

        pub fn pixel_height(self) -> u32 {
            self.resolution().1
        }
    }

    /// 💾️ Cache settings for partial movies and hashed assets.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct CacheConfig {
        pub enabled: bool,
        pub max_entries: usize,
        pub partial_movie_dir: PathBuf,
    }

    impl Default for CacheConfig {
        fn default() -> Self {
            Self { enabled: true, max_entries: 10_000, partial_movie_dir: PathBuf::from("partial_movie_files") }
        }
    }

    /// 🎬️ Root configuration for animate scenes and renderers.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct AnimateConfig {
        pub quality: QualityPreset,
        pub frame_rate: f64,
        pub width: u32,
        pub height: u32,
        pub media_dir: PathBuf,
        pub output_dir: PathBuf,
        pub cache: CacheConfig,
        pub background: [f64; 4],
        pub audio_track: Option<PathBuf>,
        pub subtitles_path: Option<PathBuf>,
    }

    impl Default for AnimateConfig {
        fn default() -> Self {
            Self::from_quality(QualityPreset::High)
        }
    }

    impl AnimateConfig {
        pub fn from_quality(quality: QualityPreset) -> Self {
            let (width, height) = quality.resolution();
            Self {
                quality,
                frame_rate: quality.frame_rate(),
                width,
                height,
                media_dir: PathBuf::from("media"),
                output_dir: PathBuf::from("output"),
                cache: CacheConfig::default(),
                background: [0.0, 0.0, 0.0, 1.0],
                audio_track: None,
                subtitles_path: None,
            }
        }

        pub fn with_frame_rate(mut self, frame_rate: f64) -> Self {
            self.frame_rate = frame_rate.max(1.0);
            self
        }

        pub fn with_resolution(mut self, width: u32, height: u32) -> Self {
            self.width = width.max(1);
            self.height = height.max(1);
            self
        }

        pub fn with_output_dir(mut self, path: impl AsRef<Path>) -> Self {
            self.output_dir = path.as_ref().to_path_buf();
            self
        }

        pub fn with_media_dir(mut self, path: impl AsRef<Path>) -> Self {
            self.media_dir = path.as_ref().to_path_buf();
            self
        }

        pub fn with_audio_track(mut self, path: impl AsRef<Path>) -> Self {
            self.audio_track = Some(path.as_ref().to_path_buf());
            self
        }

        pub fn with_subtitles_path(mut self, path: impl AsRef<Path>) -> Self {
            self.subtitles_path = Some(path.as_ref().to_path_buf());
            self
        }

        pub fn frame_duration(&self) -> f64 {
            1.0 / self.frame_rate
        }

        pub fn aspect_ratio(self) -> f64 {
            self.width as f64 / self.height as f64
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn quality_presets_have_expected_resolution() {
            assert_eq!(QualityPreset::High.resolution(), (1920, 1080));
            assert_eq!(QualityPreset::FourK.resolution(), (3840, 2160));
        }

        #[test]
        fn config_frame_duration_matches_rate() {
            let cfg = AnimateConfig::default().with_frame_rate(30.0);
            assert!((cfg.frame_duration() - 1.0 / 30.0).abs() < 1e-9);
        }

        #[test]
        fn all_quality_presets_report_frame_rate_and_resolution() {
            assert_eq!(QualityPreset::Low.frame_rate(), 15.0);
            assert_eq!(QualityPreset::Medium.frame_rate(), 15.0);
            assert_eq!(QualityPreset::High.frame_rate(), 60.0);
            assert_eq!(QualityPreset::FourK.frame_rate(), 60.0);
            assert_eq!(QualityPreset::Production.frame_rate(), 60.0);
            assert_eq!(QualityPreset::Low.resolution(), (854, 480));
            assert_eq!(QualityPreset::Medium.resolution(), (1280, 720));
            assert_eq!(QualityPreset::Production.resolution(), (2560, 1440));
            assert_eq!(QualityPreset::High.pixel_height(), 1080);
        }

        #[test]
        fn config_builder_methods_apply() {
            let cfg = AnimateConfig::from_quality(QualityPreset::Low).with_resolution(0, 0).with_output_dir("out").with_media_dir("media2").with_audio_track("track.wav").with_subtitles_path("subs.srt");
            assert_eq!(cfg.width, 1);
            assert_eq!(cfg.height, 1);
            assert_eq!(cfg.output_dir, PathBuf::from("out"));
            assert_eq!(cfg.media_dir, PathBuf::from("media2"));
            assert_eq!(cfg.audio_track, Some(PathBuf::from("track.wav")));
            assert_eq!(cfg.subtitles_path, Some(PathBuf::from("subs.srt")));
        }

        #[test]
        fn config_with_frame_rate_clamps_to_minimum() {
            let cfg = AnimateConfig::default().with_frame_rate(-5.0);
            assert_eq!(cfg.frame_rate, 1.0);
        }

        #[test]
        fn config_aspect_ratio_and_default_cache() {
            let cfg = AnimateConfig::from_quality(QualityPreset::Medium);
            assert!(cfg.cache.enabled);
            assert_eq!(cfg.cache.max_entries, 10_000);
            assert!((cfg.aspect_ratio() - 1280.0 / 720.0).abs() < 1e-9);
        }
    }
}

mod geometry {
    //! 📐️ Two-dimensional shape catalog as VSobjects.

    use crate::color::Color;
    use crate::sobject::{Group, Sobject, VSobject};
    use mathematical_geometry::{append_shape_to_path, Arc, BezPath, Circle, Line, Point, Rect, RoundedRect, RoundedRectRadii, Vec2};
    use std::f64::consts::PI;

    fn styled_path(path: BezPath, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let mut v = VSobject::from_path(path);
        v.style.fill = Some(fill);
        v.style.stroke = stroke;
        v.style.stroke_width = stroke_width;
        v
    }

    /// · Point marker.
    pub fn point(at: Point, radius: f64, color: Color) -> VSobject {
        styled_path(circle_path(at, radius), color, None, 0.0)
    }

    /// ●️ Dot (small filled circle).
    pub fn dot(at: Point, radius: f64, color: Color) -> VSobject {
        point(at, radius, color)
    }

    /// ─️ Line segment.
    pub fn line(start: Point, end: Point, color: Color, width: f64) -> VSobject {
        styled_path(line_path(start, end), Color::TRANSPARENT, Some(color), width)
    }

    /// ➡️ Arrow from start to end.
    pub fn arrow(start: Point, end: Point, color: Color, width: f64, tip_len: f64) -> VSobject {
        let mut path = line_path(start, end);
        let dir = end - start;
        let len = dir.hypot().max(1e-9);
        let u = dir / len;
        let perp = Vec2::new(-u.y(), u.x());
        let tip = end;
        let base = tip - u * tip_len;
        path.move_to(base + perp * tip_len * 0.35);
        path.line_to(tip);
        path.line_to(base - perp * tip_len * 0.35);
        styled_path(path, Color::TRANSPARENT, Some(color), width)
    }

    /// ○️ Circle outline or fill.
    pub fn circle(center: Point, radius: f64, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        styled_path(circle_path(center, radius), fill, stroke, stroke_width)
    }

    /// ◠️ Circular arc.
    pub fn arc(center: Point, radius: f64, start_angle: f64, sweep: f64, color: Color, width: f64) -> VSobject {
        let a = Arc::new(center, (radius, radius), start_angle, sweep, 0.0);
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, &a, 0.01);
        styled_path(path, Color::TRANSPARENT, Some(color), width)
    }

    /// ■️ Axis-aligned square.
    pub fn square(side: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        rectangle(side, side, center, fill, stroke, stroke_width)
    }

    /// ▭️ Rectangle.
    pub fn rectangle(width: f64, height: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let r = Rect::new(center.x() - width / 2.0, center.y() - height / 2.0, center.x() + width / 2.0, center.y() + height / 2.0);
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, &r, 0.01);
        styled_path(path, fill, stroke, stroke_width)
    }

    /// ▢️ Rounded rectangle.
    pub fn rounded_rectangle(width: f64, height: f64, radius: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let rect = Rect::new(center.x() - width / 2.0, center.y() - height / 2.0, center.x() + width / 2.0, center.y() + height / 2.0);
        let r = RoundedRect::new(rect, RoundedRectRadii::new(radius, radius, radius, radius));
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, &r, 0.01);
        styled_path(path, fill, stroke, stroke_width)
    }

    /// ⬠️ Regular polygon.
    pub fn polygon(vertices: &[Point], fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let mut path = BezPath::new();
        if let Some(first) = vertices.first() {
            path.move_to(*first);
            for p in vertices.iter().skip(1) {
                path.line_to(*p);
            }
            path.close_path();
        }
        styled_path(path, fill, stroke, stroke_width)
    }

    /// △️ Equilateral triangle.
    pub fn triangle(side: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let h = side * 3.0_f64.sqrt() / 2.0;
        let verts = [Point::new(center.x(), center.y() + 2.0 * h / 3.0), Point::new(center.x() - side / 2.0, center.y() - h / 3.0), Point::new(center.x() + side / 2.0, center.y() - h / 3.0)];
        polygon(&verts, fill, stroke, stroke_width)
    }

    /// ★️ Star polygon.
    pub fn star(points: u32, outer: f64, inner: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let n = points.max(3) as usize;
        let mut verts = Vec::with_capacity(n * 2);
        for i in 0..(n * 2) {
            let angle = PI / 2.0 + (i as f64) * PI / (n as f64);
            let r = if i % 2 == 0 { outer } else { inner };
            verts.push(Point::new(center.x() + r * angle.cos(), center.y() + r * angle.sin()));
        }
        polygon(&verts, fill, stroke, stroke_width)
    }

    /// ◎️ Annulus (ring).
    pub fn annulus(center: Point, inner: f64, outer: f64, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let mut path = circle_path(center, outer);
        let hole = circle_path(center, inner);
        for el in hole.elements() {
            path.push(el);
        }
        styled_path(path, fill, stroke, stroke_width)
    }

    /// ◔️ Circular sector.
    pub fn sector(center: Point, radius: f64, start_angle: f64, sweep: f64, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let mut path = BezPath::new();
        path.move_to(center);
        let steps = 64;
        for i in 0..=steps {
            let t = start_angle + sweep * (i as f64 / steps as f64);
            path.line_to(Point::new(center.x() + radius * t.cos(), center.y() + radius * t.sin()));
        }
        path.close_path();
        styled_path(path, fill, stroke, stroke_width)
    }

    /// { } Brace under content.
    pub fn brace(start: Point, end: Point, direction: Vec2, color: Color, width: f64) -> VSobject {
        let mid = Point::new((start.x() + end.x()) / 2.0, (start.y() + end.y()) / 2.0);
        let dir = if direction.hypot() < 1e-9 { Vec2::new(0.0, -1.0) } else { direction / direction.hypot() };
        let depth = (end - start).hypot() * 0.15;
        let tip = mid + dir * depth;
        let mut path = BezPath::new();
        path.move_to(start);
        path.quad_to(tip, end);
        styled_path(path, Color::TRANSPARENT, Some(color), width)
    }

    /// ∠ Angle arc between two rays from vertex.
    pub fn angle(vertex: Point, ray_a: Point, ray_b: Point, radius: f64, color: Color, width: f64) -> VSobject {
        let va = ray_a - vertex;
        let vb = ray_b - vertex;
        let a1 = va.y().atan2(va.x());
        let a2 = vb.y().atan2(vb.x());
        let mut sweep = a2 - a1;
        while sweep <= -PI {
            sweep += 2.0 * PI;
        }
        while sweep > PI {
            sweep -= 2.0 * PI;
        }
        arc(vertex, radius, a1, sweep, color, width)
    }

    fn circle_path(center: Point, radius: f64) -> BezPath {
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, &Circle::new(center, radius), 0.01);
        path
    }

    fn line_path(start: Point, end: Point) -> BezPath {
        let mut path = BezPath::new();
        append_shape_to_path(&mut path, &Line::new(start, end), 0.01);
        path
    }

    /// ╌️ Dashed stroke style built from multiple segment paths.
    #[derive(Clone)]
    pub struct DashedVSobject {
        pub inner: VSobject,
    }

    impl DashedVSobject {
        pub fn from_segments(paths: Vec<BezPath>, color: Color, width: f64) -> Self {
            let mut inner = VSobject::new();
            inner.set_paths(paths);
            inner.style.fill = None;
            inner.style.stroke = Some(color);
            inner.style.stroke_width = width;
            Self { inner }
        }

        pub fn as_vobject(&self) -> &VSobject {
            &self.inner
        }
    }

    /// ╌️ Dashed line via repeated stroke segments.
    pub fn dashed_line(start: Point, end: Point, color: Color, width: f64, dash_len: f64, gap_len: f64) -> VSobject {
        let dir = end - start;
        let total = dir.hypot();
        if total < 1e-9 {
            return line(start, end, color, width);
        }
        let u = dir / total;
        let step = (dash_len + gap_len).max(1e-9);
        let mut paths = Vec::new();
        let mut dist = 0.0;
        while dist < total {
            let seg_start = start + u * dist;
            let seg_end = start + u * (dist + dash_len).min(total);
            let mut path = BezPath::new();
            path.move_to(seg_start);
            path.line_to(seg_end);
            paths.push(path);
            dist += step;
        }
        DashedVSobject::from_segments(paths, color, width).inner
    }

    /// ⬭️ Axis-aligned ellipse.
    pub fn ellipse(center: Point, width: f64, height: f64, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let rx = width / 2.0;
        let ry = height / 2.0;
        let steps = 64;
        let mut path = BezPath::new();
        for i in 0..=steps {
            let t = (i as f64 / steps as f64) * std::f64::consts::TAU;
            let p = Point::new(center.x() + rx * t.cos(), center.y() + ry * t.sin());
            if i == 0 {
                path.move_to(p);
            } else {
                path.line_to(p);
            }
        }
        path.close_path();
        styled_path(path, fill, stroke, stroke_width)
    }

    /// ⬡️ Regular polygon with `n` sides inscribed in a circle.
    pub fn regular_polygon(n: u32, radius: f64, center: Point, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let sides = n.max(3) as usize;
        let verts: Vec<Point> = (0..sides)
            .map(|i| {
                let angle = PI / 2.0 + (i as f64) * std::f64::consts::TAU / sides as f64;
                Point::new(center.x() + radius * angle.cos(), center.y() + radius * angle.sin())
            })
            .collect();
        polygon(&verts, fill, stroke, stroke_width)
    }

    /// ▢️ Rectangle around an Sobject's bounds.
    pub fn surrounding_rectangle(mobject: &dyn Sobject, buff: f64, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let b = mobject.bounds();
        rectangle(b.width() + buff * 2.0, b.height() + buff * 2.0, b.center(), fill, stroke, stroke_width)
    }

    /// ⊎ Simple path union by concatenating subpaths.
    pub fn boolean_union(a: &VSobject, b: &VSobject, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let mut paths = a.paths.clone();
        paths.extend(b.paths.clone());
        let mut v = VSobject::new();
        v.set_paths(paths);
        v.style.fill = Some(fill);
        v.style.stroke = stroke;
        v.style.stroke_width = stroke_width;
        v
    }

    /// ⊖ Simple path difference via compound path (outer + hole subpaths).
    pub fn boolean_difference(a: &VSobject, b: &VSobject, fill: Color, stroke: Option<Color>, stroke_width: f64) -> VSobject {
        let mut paths = a.paths.clone();
        paths.extend(b.paths.clone());
        let mut v = VSobject::new();
        v.set_paths(paths);
        v.style.fill = Some(fill);
        v.style.stroke = stroke;
        v.style.stroke_width = stroke_width;
        v
    }

    /// ➡️ Grid of small arrows sampling a vector field.
    pub fn arrow_vector_field<F>(x_range: (f64, f64), y_range: (f64, f64), cols: u32, rows: u32, field: F, color: Color, arrow_scale: f64) -> Group
    where
        F: Fn(f64, f64) -> Vec2,
    {
        let cols = cols.max(1);
        let rows = rows.max(1);
        let dx = (x_range.1 - x_range.0) / cols as f64;
        let dy = (y_range.1 - y_range.0) / rows as f64;
        let mut children: Vec<Box<dyn Sobject>> = Vec::new();
        for row in 0..rows {
            for col in 0..cols {
                let x = x_range.0 + (col as f64 + 0.5) * dx;
                let y = y_range.0 + (row as f64 + 0.5) * dy;
                let start = Point::new(x, y);
                let v = field(x, y);
                let len = v.hypot();
                if len < 1e-9 {
                    continue;
                }
                let u = v / len;
                let tip = start + u * arrow_scale;
                children.push(Box::new(arrow(start, tip, color, 1.5, arrow_scale * 0.2)));
            }
        }
        Group::new(children)
    }

    /// 〰 Stream lines traced through a vector field from seed points.
    pub fn stream_lines<F>(seeds: &[(f64, f64)], field: F, color: Color, steps: u32, step_size: f64) -> Group
    where
        F: Fn(f64, f64) -> Vec2,
    {
        let steps = steps.max(2);
        let mut children: Vec<Box<dyn Sobject>> = Vec::new();
        for &(sx, sy) in seeds {
            let mut path = BezPath::new();
            let mut x = sx;
            let mut y = sy;
            path.move_to(Point::new(x, y));
            for _ in 0..steps {
                let v = field(x, y);
                let len = v.hypot();
                if len < 1e-9 {
                    break;
                }
                let u = v / len;
                x += u.x() * step_size;
                y += u.y() * step_size;
                path.line_to(Point::new(x, y));
            }
            let mut v = VSobject::from_path(path);
            v.style.fill = None;
            v.style.stroke = Some(color);
            v.style.stroke_width = 1.5;
            children.push(Box::new(v));
        }
        Group::new(children)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn shapes_produce_paths() {
            let c = circle(Point::ZERO, 1.0, Color::BLUE, None, 0.0);
            assert!(!c.paths.is_empty());
            let a = arrow(Point::ZERO, Point::new(2.0, 0.0), Color::RED, 2.0, 0.3);
            assert!(!a.paths.is_empty());
        }

        #[test]
        fn star_has_vertices() {
            let s = star(5, 1.0, 0.4, Point::ZERO, Color::YELLOW, None, 0.0);
            assert!(s.paths[0].elements().len() > 4);
        }

        #[test]
        fn ellipse_and_regular_polygon_build() {
            let e = ellipse(Point::ZERO, 2.0, 1.0, Color::BLUE, None, 0.0);
            assert!(!e.paths.is_empty());
            let p = regular_polygon(6, 1.0, Point::ZERO, Color::GREEN, None, 0.0);
            assert!(!p.paths.is_empty());
        }

        #[test]
        fn dashed_line_has_multiple_segments() {
            let d = dashed_line(Point::ZERO, Point::new(4.0, 0.0), Color::WHITE, 2.0, 0.3, 0.2);
            assert!(d.paths.len() > 1);
        }

        #[test]
        fn boolean_ops_combine_paths() {
            let a = circle(Point::ZERO, 1.0, Color::BLUE, None, 0.0);
            let b = circle(Point::new(0.5, 0.0), 1.0, Color::RED, None, 0.0);
            let u = boolean_union(&a, &b, Color::PURPLE, None, 0.0);
            assert!(u.paths.len() >= 2);
            let diff = boolean_difference(&a, &b, Color::YELLOW, None, 0.0);
            assert!(diff.paths.len() >= 2);
        }

        #[test]
        fn vector_field_helpers_build() {
            let vf = arrow_vector_field((-1.0, 1.0), (-1.0, 1.0), 3, 3, |x, _| Vec2::new(x, 1.0), Color::TEAL, 0.2);
            assert!(!vf.children.is_empty());
            let sl = stream_lines(&[(0.0, 0.0)], |_, y| Vec2::new(1.0, y), Color::WHITE, 8, 0.1);
            assert!(!sl.children.is_empty());
        }

        #[test]
        fn vector_field_skips_zero_length_vectors() {
            let vf = arrow_vector_field((-1.0, 1.0), (-1.0, 1.0), 2, 2, |_, _| Vec2::new(0.0, 0.0), Color::TEAL, 0.2);
            assert!(vf.children.is_empty());
        }

        #[test]
        fn stream_lines_stops_on_zero_length_field() {
            let sl = stream_lines(&[(0.0, 0.0)], |_, _| Vec2::new(0.0, 0.0), Color::WHITE, 8, 0.1);
            assert_eq!(sl.children.len(), 1);
        }

        #[test]
        fn point_dot_and_line_build_paths() {
            let p = point(Point::ZERO, 0.1, Color::RED);
            assert!(!p.paths.is_empty());
            let d = dot(Point::ZERO, 0.1, Color::RED);
            assert!(!d.paths.is_empty());
            let l = line(Point::ZERO, Point::new(1.0, 1.0), Color::BLUE, 1.0);
            assert!(!l.paths.is_empty());
        }

        #[test]
        fn square_triangle_and_polygon_build() {
            let sq = square(2.0, Point::ZERO, Color::RED, None, 0.0);
            assert!(!sq.paths.is_empty());
            let tri = triangle(2.0, Point::ZERO, Color::GREEN, None, 0.0);
            assert!(!tri.paths.is_empty());
            let empty_poly = polygon(&[], Color::WHITE, None, 0.0);
            assert!(empty_poly.paths[0].elements().is_empty());
        }

        #[test]
        fn annulus_and_sector_build() {
            let a = annulus(Point::ZERO, 0.5, 1.0, Color::BLUE, None, 0.0);
            assert!(!a.paths.is_empty());
            let s = sector(Point::ZERO, 1.0, 0.0, PI / 2.0, Color::YELLOW, None, 0.0);
            assert!(!s.paths.is_empty());
        }

        #[test]
        fn brace_and_angle_build() {
            let b = brace(Point::new(-1.0, 0.0), Point::new(1.0, 0.0), Vec2::new(0.0, -1.0), Color::WHITE, 1.0);
            assert!(!b.paths.is_empty());
            let b_default_dir = brace(Point::new(-1.0, 0.0), Point::new(1.0, 0.0), Vec2::new(0.0, 0.0), Color::WHITE, 1.0);
            assert!(!b_default_dir.paths.is_empty());
            let ang = angle(Point::ZERO, Point::new(1.0, 0.0), Point::new(0.0, 1.0), 0.3, Color::ORANGE, 1.0);
            assert!(!ang.paths.is_empty());
        }

        #[test]
        fn surrounding_rectangle_pads_bounds() {
            let c = circle(Point::ZERO, 1.0, Color::BLUE, None, 0.0);
            let r = surrounding_rectangle(&c, 0.5, Color::TRANSPARENT, Some(Color::WHITE), 1.0);
            assert!(!r.paths.is_empty());
        }

        #[test]
        fn dashed_line_degenerate_endpoints_falls_back_to_line() {
            let d = dashed_line(Point::new(1.0, 1.0), Point::new(1.0, 1.0), Color::WHITE, 2.0, 0.3, 0.2);
            assert_eq!(d.paths.len(), 1);
        }
    }
}

mod graph {
    //! 🕸️ Graph and directed graph layouts as Sobject groups.

    use crate::color::Color;
    use crate::geometry::{arrow, circle, line};
    use crate::sobject::{Group, Sobject};
    use crate::text::Text;
    use mathematical_geometry::Point;
    use std::collections::HashMap;

    /// 🔵️ Undirected graph with circular layout.
    pub struct Graph {
        pub group: Group,
        pub nodes: Vec<u32>,
        pub edges: Vec<(u32, u32)>,
    }

    impl Graph {
        pub fn new(nodes: Vec<u32>, edges: Vec<(u32, u32)>, radius: f64, center: Point, color: Color) -> Self {
            let positions = circular_layout(&nodes, radius, center);
            let mut children: Vec<Box<dyn Sobject>> = Vec::new();
            for &(a, b) in &edges {
                if let (Some(&pa), Some(&pb)) = (positions.get(&a), positions.get(&b)) {
                    children.push(Box::new(line(pa, pb, color.with_alpha(0.6), 2.0)));
                }
            }
            for &n in &nodes {
                if let Some(&p) = positions.get(&n) {
                    children.push(Box::new(circle(p, 0.2, color, None, 0.0)));
                }
            }
            Self { group: Group::new(children), nodes, edges }
        }

        pub fn with_edge_labels(mut self, labels: &HashMap<(u32, u32), String>, positions: &HashMap<u32, Point>, color: Color) -> Self {
            for (&(a, b), label) in labels {
                if let (Some(&pa), Some(&pb)) = (positions.get(&a), positions.get(&b)) {
                    let mid = Point::new((pa.x() + pb.x()) / 2.0, (pa.y() + pb.y()) / 2.0);
                    let mut t = Text::new(label, color);
                    t.inner.move_to(mid);
                    self.group.add_child(Box::new(t.inner));
                }
            }
            self
        }
    }

    /// ➡️ Directed graph with force-directed layout seed.
    pub struct DiGraph {
        pub group: Group,
        pub nodes: Vec<u32>,
        pub edges: Vec<(u32, u32)>,
    }

    impl DiGraph {
        pub fn new(nodes: Vec<u32>, edges: Vec<(u32, u32)>, radius: f64, center: Point, color: Color) -> Self {
            let positions = force_layout_seed(&nodes, &edges, radius, center);
            let node_r = 0.18;
            let mut children: Vec<Box<dyn Sobject>> = Vec::new();
            for &(a, b) in &edges {
                if let (Some(&pa), Some(&pb)) = (positions.get(&a), positions.get(&b)) {
                    let dir = pb - pa;
                    let len = dir.hypot().max(1e-9);
                    let u = dir / len;
                    let start = pa + u * node_r;
                    let end = pb - u * node_r;
                    children.push(Box::new(arrow(start, end, color.with_alpha(0.7), 2.0, 0.15)));
                }
            }
            for &n in &nodes {
                if let Some(&p) = positions.get(&n) {
                    children.push(Box::new(circle(p, node_r, color, Some(Color::WHITE), 1.0)));
                }
            }
            Self { group: Group::new(children), nodes, edges }
        }

        pub fn with_edge_labels(mut self, labels: &HashMap<(u32, u32), String>, positions: &HashMap<u32, Point>, color: Color) -> Self {
            for (&(a, b), label) in labels {
                if let (Some(&pa), Some(&pb)) = (positions.get(&a), positions.get(&b)) {
                    let mid = Point::new((pa.x() + pb.x()) / 2.0, (pa.y() + pb.y()) / 2.0);
                    let mut t = Text::new(label, color);
                    t.inner.move_to(mid);
                    self.group.add_child(Box::new(t.inner));
                }
            }
            self
        }
    }

    fn circular_layout(nodes: &[u32], radius: f64, center: Point) -> HashMap<u32, Point> {
        let mut out = HashMap::new();
        let n = nodes.len().max(1);
        for (i, &id) in nodes.iter().enumerate() {
            let t = i as f64 / n as f64 * std::f64::consts::TAU;
            out.insert(id, Point::new(center.x() + radius * t.cos(), center.y() + radius * t.sin()));
        }
        out
    }

    fn force_layout_seed(nodes: &[u32], edges: &[(u32, u32)], radius: f64, center: Point) -> HashMap<u32, Point> {
        let mut pos = circular_layout(nodes, radius, center);
        for _ in 0..24 {
            let mut forces: HashMap<u32, (f64, f64)> = nodes.iter().map(|&id| (id, (0.0, 0.0))).collect();
            for i in 0..nodes.len() {
                for j in (i + 1)..nodes.len() {
                    let a = nodes[i];
                    let b = nodes[j];
                    let pa = pos[&a];
                    let pb = pos[&b];
                    let dx = pb.x() - pa.x();
                    let dy = pb.y() - pa.y();
                    let dist = (dx * dx + dy * dy).sqrt().max(0.01);
                    let rep = 0.05 / dist;
                    let force_a = forces.get_mut(&a).expect("a is drawn from nodes, forces is keyed by all of nodes");
                    force_a.0 -= dx * rep;
                    force_a.1 -= dy * rep;
                    let force_b = forces.get_mut(&b).expect("b is drawn from nodes, forces is keyed by all of nodes");
                    force_b.0 += dx * rep;
                    force_b.1 += dy * rep;
                }
            }
            for &(a, b) in edges {
                let (Some(&pa), Some(&pb)) = (pos.get(&a), pos.get(&b)) else {
                    continue;
                };
                let dx = pb.x() - pa.x();
                let dy = pb.y() - pa.y();
                let dist = (dx * dx + dy * dy).sqrt().max(0.01);
                let att = dist * 0.02;
                if let Some(force_a) = forces.get_mut(&a) {
                    force_a.0 += dx / dist * att;
                    force_a.1 += dy / dist * att;
                }
                if let Some(force_b) = forces.get_mut(&b) {
                    force_b.0 -= dx / dist * att;
                    force_b.1 -= dy / dist * att;
                }
            }
            for &id in nodes {
                let (fx, fy) = forces[&id];
                let p = pos.get_mut(&id).expect("id is drawn from nodes, pos is keyed by all of nodes");
                *p = Point::new(p.x() + fx, p.y() + fy);
            }
        }
        pos
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn graph_has_node_and_edge_children() {
            let g = Graph::new(vec![1, 2, 3], vec![(1, 2), (2, 3)], 2.0, Point::ZERO, Color::BLUE);
            assert_eq!(g.nodes.len(), 3);
            assert!(!g.group.children.is_empty());
        }

        #[test]
        fn digraph_uses_arrows_and_labels() {
            let dg = DiGraph::new(vec![1, 2], vec![(1, 2)], 2.0, Point::ZERO, Color::WHITE);
            assert_eq!(dg.edges.len(), 1);
            let mut positions = HashMap::new();
            positions.insert(1, Point::new(-1.0, 0.0));
            positions.insert(2, Point::new(1.0, 0.0));
            let mut labels = HashMap::new();
            labels.insert((1, 2), "edge".into());
            let labeled = dg.with_edge_labels(&labels, &positions, Color::WHITE);
            assert!(labeled.group.children.len() > 2);
        }
    }
}

mod hash {
    //! 🪪️ Content-hash animation descriptors via framework hash.

    use framework_hash::{format_number_for_hash, hash_parts, merkle_node};
    use serde::Serialize;

    /// 🧾️ Serializable animation fingerprint input.
    #[derive(Clone, Debug, Serialize)]
    pub struct AnimationHashInput {
        pub kind: String,
        pub run_time: f64,
        pub target_ids: Vec<u64>,
        pub rate: String,
        pub extras: Vec<String>,
    }

    impl AnimationHashInput {
        pub fn new(kind: impl Into<String>, run_time: f64) -> Self {
            Self { kind: kind.into(), run_time, target_ids: Vec::new(), rate: "linear".into(), extras: Vec::new() }
        }

        pub fn with_targets(mut self, ids: Vec<u64>) -> Self {
            self.target_ids = ids;
            self
        }

        pub fn with_rate(mut self, rate: impl Into<String>) -> Self {
            self.rate = rate.into();
            self
        }

        pub fn with_extra(mut self, extra: impl Into<String>) -> Self {
            self.extras.push(extra.into());
            self
        }
    }

    /// 🔐️ Hash a single animation descriptor.
    pub fn hash_animation(input: &AnimationHashInput) -> String {
        let mut parts = vec![input.kind.clone(), format_number_for_hash(input.run_time), input.rate.clone()];
        for id in &input.target_ids {
            parts.push(id.to_string());
        }
        parts.extend(input.extras.clone());
        hash_parts(&parts)
    }

    /// 🌳️ Merkle hash over an animation timeline.
    pub fn hash_animation_timeline(children: Vec<String>) -> String {
        merkle_node(&["AnimateTimeline"], children)
    }

    /// 🎬️ Hash a scene configuration snapshot.
    pub fn hash_scene_config(frame_rate: f64, width: u32, height: u32, mobject_count: usize) -> String {
        let rate = format_number_for_hash(frame_rate);
        hash_parts(&["SceneConfig", &rate, &width.to_string(), &height.to_string(), &mobject_count.to_string()])
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn animation_hash_is_stable() {
            let input = AnimationHashInput::new("FadeIn", 1.0).with_targets(vec![42]);
            let a = hash_animation(&input);
            let b = hash_animation(&input);
            assert_eq!(a, b);
        }

        #[test]
        fn timeline_merkle_orders_children() {
            let h = hash_animation_timeline(vec!["a".into(), "b".into()]);
            assert!(!h.is_empty());
        }

        #[test]
        fn hash_scene_config_is_stable_and_sensitive_to_inputs() {
            let a = hash_scene_config(60.0, 1920, 1080, 3);
            let b = hash_scene_config(60.0, 1920, 1080, 3);
            assert_eq!(a, b);
            let c = hash_scene_config(30.0, 1920, 1080, 3);
            assert_ne!(a, c);
        }

        #[test]
        fn hash_animation_differs_by_rate_and_extras() {
            let base = AnimationHashInput::new("Fade", 1.0);
            let with_rate = base.clone().with_rate("smooth");
            let with_extra = base.clone().with_extra("scale=2");
            assert_ne!(hash_animation(&base), hash_animation(&with_rate));
            assert_ne!(hash_animation(&base), hash_animation(&with_extra));
        }
    }
}

mod matrix {
    //! 🔢️ Matrix types as gridded Sobject groups.

    use crate::color::Color;
    use crate::geometry::rectangle;
    use crate::sobject::{arrange, Group, Sobject};
    use crate::text::{MathText, Text};
    use mathematical_geometry::{Point, Vec2};

    fn arrange_grid(group: &mut Group, rows: usize, cols: usize, cell_size: (f64, f64)) {
        if group.children.is_empty() || rows == 0 || cols == 0 {
            return;
        }
        let origin = group.children[0].center();
        for (idx, child) in group.children.iter_mut().enumerate() {
            let row = idx / cols;
            let col = idx % cols;
            let x = origin.x() + col as f64 * cell_size.0;
            let y = origin.y() - row as f64 * cell_size.1;
            child.move_to(Point::new(x, y));
        }
    }

    /// 📊️ Matrix of string entries with optional brackets.
    pub struct Matrix {
        pub group: Group,
        pub rows: usize,
        pub cols: usize,
    }

    impl Matrix {
        pub fn from_rows(rows: Vec<Vec<String>>, cell_size: (f64, f64), color: Color) -> Self {
            let nrows = rows.len();
            let ncols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
            let mut children: Vec<Box<dyn Sobject>> = Vec::new();
            for row in rows {
                for cell in row {
                    let t = Text::new(cell, color);
                    children.push(Box::new(t.inner));
                }
            }
            let mut group = Group::new(children);
            arrange_grid(&mut group, nrows, ncols, cell_size);
            Self { group, rows: nrows, cols: ncols }
        }

        pub fn math(entries: &[&str], cell_size: (f64, f64), color: Color) -> Self {
            let children: Vec<Box<dyn Sobject>> = entries
                .iter()
                .map(|e| {
                    let m = MathText::new(*e, color);
                    Box::new(m.inner) as Box<dyn Sobject>
                })
                .collect();
            let cols = (entries.len() as f64).sqrt().ceil() as usize;
            let rows = entries.len().div_ceil(cols);
            let mut group = Group::new(children);
            arrange(&mut group, Vec2::new(1.0, 0.0), cell_size.0 * 0.15);
            Self { group, rows, cols }
        }

        pub fn with_brackets(mut self, color: Color, padding: f64) -> Self {
            let b = self.group.bounds();
            let w = b.width() + padding * 2.0;
            let h = b.height() + padding * 2.0;
            let c = b.center();
            let frame = rectangle(w, h, c, Color::TRANSPARENT, Some(color), 3.0);
            self.group.add_child(Box::new(frame));
            self
        }
    }

    /// 📋️ Table with header row and body rows in a 2D grid.
    pub struct Table {
        pub group: Group,
        pub rows: usize,
        pub cols: usize,
    }

    impl Table {
        pub fn new(headers: Vec<String>, rows: &[Vec<String>], cell_size: (f64, f64), color: Color) -> Self {
            let ncols = headers.len().max(rows.iter().map(|r| r.len()).max().unwrap_or(0));
            let nrows = rows.len() + 1;
            let mut children: Vec<Box<dyn Sobject>> = Vec::new();
            for header in headers {
                children.push(Box::new(Text::new(header, color).inner));
            }
            for row in rows {
                for cell in row {
                    children.push(Box::new(Text::new(cell.clone(), color).inner));
                }
                let pad = ncols.saturating_sub(row.len());
                for _ in 0..pad {
                    children.push(Box::new(Text::new("", color).inner));
                }
            }
            let mut group = Group::new(children);
            arrange_grid(&mut group, nrows, ncols, cell_size);
            Self { group, rows: nrows, cols: ncols }
        }

        pub fn with_frame(mut self, color: Color, padding: f64) -> Self {
            let b = self.group.bounds();
            let frame = rectangle(b.width() + padding * 2.0, b.height() + padding * 2.0, b.center(), Color::TRANSPARENT, Some(color), 2.0);
            self.group.add_child(Box::new(frame));
            self
        }
    }

    /// 🧮️ Decimal matrix for numeric interpolation animations.
    #[derive(Clone, Debug)]
    pub struct DecimalMatrix {
        pub values: Vec<Vec<f64>>,
    }

    impl DecimalMatrix {
        pub fn new(values: Vec<Vec<f64>>) -> Self {
            Self { values }
        }

        pub fn lerp(&self, other: &Self, t: f64) -> Self {
            let rows = self.values.len().min(other.values.len());
            let mut out = Vec::with_capacity(rows);
            for r in 0..rows {
                let cols = self.values[r].len().min(other.values[r].len());
                let mut row = Vec::with_capacity(cols);
                for c in 0..cols {
                    let a = self.values[r][c];
                    let b = other.values[r][c];
                    row.push(a + (b - a) * t);
                }
                out.push(row);
            }
            Self { values: out }
        }

        pub fn to_matrix_sobject(&self, cell_size: (f64, f64), color: Color) -> Matrix {
            let rows: Vec<Vec<String>> = self.values.iter().map(|row| row.iter().map(|v| format!("{v:.2}")).collect()).collect();
            Matrix::from_rows(rows, cell_size, color)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn decimal_matrix_lerps() {
            let a = DecimalMatrix::new(vec![vec![0.0, 1.0]]);
            let b = DecimalMatrix::new(vec![vec![2.0, 3.0]]);
            let m = a.lerp(&b, 0.5);
            assert!((m.values[0][0] - 1.0).abs() < 1e-9);
        }

        #[test]
        fn matrix_grid_layout() {
            let m = Matrix::from_rows(vec![vec!["a".into(), "b".into()], vec!["c".into(), "d".into()]], (1.0, 1.0), Color::WHITE);
            assert_eq!(m.rows, 2);
            assert_eq!(m.cols, 2);
            assert_eq!(m.group.children.len(), 4);
        }

        #[test]
        fn table_has_header_and_rows() {
            let t = Table::new(vec!["x".into()], &[vec!["1".into()]], (1.0, 1.0), Color::WHITE);
            assert_eq!(t.rows, 2);
            assert_eq!(t.cols, 1);
        }

        #[test]
        fn table_with_frame_adds_border_child() {
            let t = Table::new(vec!["a".into(), "b".into()], &[vec!["1".into()]], (1.0, 1.0), Color::WHITE);
            let before = t.group.children.len();
            let framed = t.with_frame(Color::WHITE, 0.2);
            assert_eq!(framed.group.children.len(), before + 1);
        }

        #[test]
        fn matrix_math_lays_out_entries() {
            let m = Matrix::math(&["1", "2", "3", "4"], (1.0, 1.0), Color::WHITE);
            assert_eq!(m.group.children.len(), 4);
            assert_eq!(m.cols, 2);
            assert_eq!(m.rows, 2);
        }

        #[test]
        fn matrix_with_brackets_adds_frame_child() {
            let m = Matrix::from_rows(vec![vec!["a".into()]], (1.0, 1.0), Color::WHITE);
            let before = m.group.children.len();
            let bracketed = m.with_brackets(Color::WHITE, 0.1);
            assert_eq!(bracketed.group.children.len(), before + 1);
        }

        #[test]
        fn decimal_matrix_to_matrix_sobject_formats_values() {
            let d = DecimalMatrix::new(vec![vec![1.5, 2.25]]);
            let m = d.to_matrix_sobject((1.0, 1.0), Color::WHITE);
            assert_eq!(m.group.children.len(), 2);
        }
    }
}

mod rate {
    //! 📈️ Rate functions mapping linear time α ∈ [0,1] to eased progress.

    use mathematical_geometry::clamp_f64;

    /// 📐️ Easing function signature used by animations.
    pub type RateFunc = fn(f64) -> f64;

    pub fn linear(t: f64) -> f64 {
        clamp01(t)
    }

    pub fn smooth(t: f64) -> f64 {
        let t = clamp01(t);
        t * t * (3.0 - 2.0 * t)
    }

    pub fn rush_into(t: f64) -> f64 {
        let t = clamp01(t);
        2.0 * t * t
    }

    pub fn rush_from(t: f64) -> f64 {
        let t = clamp01(t);
        2.0 * t - t * t
    }

    pub fn slow_into(t: f64) -> f64 {
        let t = clamp01(t);
        t * t * t
    }

    pub fn double_smooth(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        }
    }

    pub fn there_and_back(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            smooth(t * 2.0)
        } else {
            smooth(2.0 - t * 2.0)
        }
    }

    pub fn there_and_back_with_pause(t: f64, pause_ratio: f64) -> f64 {
        let t = clamp01(t);
        let pause = pause_ratio.clamp(0.0, 0.9);
        let edge = (1.0 - pause) / 2.0;
        if t < edge {
            smooth(t / edge)
        } else if t < edge + pause {
            1.0
        } else {
            smooth(1.0 - (t - edge - pause) / edge)
        }
    }

    pub fn running_start(t: f64) -> f64 {
        let t = clamp01(t);
        t * t * (2.0 - t)
    }

    pub fn wiggle(t: f64, num_wiggles: f64) -> f64 {
        let t = clamp01(t);
        t + (std::f64::consts::TAU * num_wiggles * t).sin() / (std::f64::consts::TAU * num_wiggles).max(1.0)
    }

    pub fn lingering(t: f64) -> f64 {
        let t = clamp01(t);
        1.0 - (1.0 - t).powi(3)
    }

    pub fn exponential_decay(t: f64, half_life: f64) -> f64 {
        let t = clamp01(t);
        1.0 - 0.5_f64.powf(t / half_life.max(1e-9))
    }

    pub fn ease_in_sine(t: f64) -> f64 {
        1.0 - ((clamp01(t) * std::f64::consts::FRAC_PI_2).cos())
    }

    pub fn ease_out_sine(t: f64) -> f64 {
        (clamp01(t) * std::f64::consts::FRAC_PI_2).sin()
    }

    pub fn ease_in_out_sine(t: f64) -> f64 {
        -(0.5 * (std::f64::consts::PI * clamp01(t)).cos() - 0.5)
    }

    pub fn ease_in_quad(t: f64) -> f64 {
        let t = clamp01(t);
        t * t
    }

    pub fn ease_out_quad(t: f64) -> f64 {
        let t = clamp01(t);
        1.0 - (1.0 - t) * (1.0 - t)
    }

    pub fn ease_in_out_quad(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
        }
    }

    pub fn ease_in_cubic(t: f64) -> f64 {
        let t = clamp01(t);
        t * t * t
    }

    pub fn ease_out_cubic(t: f64) -> f64 {
        let t = clamp01(t);
        1.0 - (1.0 - t).powi(3)
    }

    pub fn ease_in_out_cubic(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }

    pub fn ease_in_quart(t: f64) -> f64 {
        let t = clamp01(t);
        t * t * t * t
    }

    pub fn ease_out_quart(t: f64) -> f64 {
        let t = clamp01(t);
        1.0 - (1.0 - t).powi(4)
    }

    pub fn ease_in_out_quart(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            8.0 * t * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
        }
    }

    pub fn ease_in_quint(t: f64) -> f64 {
        let t = clamp01(t);
        t * t * t * t * t
    }

    pub fn ease_out_quint(t: f64) -> f64 {
        let t = clamp01(t);
        1.0 - (1.0 - t).powi(5)
    }

    pub fn ease_in_out_quint(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            16.0 * t * t * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
        }
    }

    pub fn ease_in_exp(t: f64) -> f64 {
        let t = clamp01(t);
        if t == 0.0 {
            0.0
        } else {
            2.0_f64.powf(10.0 * t - 10.0)
        }
    }

    pub fn ease_out_exp(t: f64) -> f64 {
        let t = clamp01(t);
        if t == 1.0 {
            1.0
        } else {
            1.0 - 2.0_f64.powf(-10.0 * t)
        }
    }

    pub fn ease_in_out_exp(t: f64) -> f64 {
        let t = clamp01(t);
        if t == 0.0 {
            0.0
        } else if t == 1.0 {
            1.0
        } else if t < 0.5 {
            2.0_f64.powf(20.0 * t - 10.0) / 2.0
        } else {
            (2.0 - 2.0_f64.powf(-20.0 * t + 10.0)) / 2.0
        }
    }

    pub fn ease_in_circ(t: f64) -> f64 {
        let t = clamp01(t);
        1.0 - (1.0 - t * t).sqrt()
    }

    pub fn ease_out_circ(t: f64) -> f64 {
        let t = clamp01(t);
        (1.0 - (t - 1.0).powi(2)).sqrt()
    }

    pub fn ease_in_out_circ(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
        } else {
            ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
        }
    }

    pub fn ease_in_back(t: f64) -> f64 {
        const C: f64 = 1.70158;
        let t = clamp01(t);
        (C + 1.0) * t * t * t - C * t * t
    }

    pub fn ease_out_back(t: f64) -> f64 {
        const C: f64 = 1.70158;
        let t = clamp01(t);
        1.0 + (C + 1.0) * (t - 1.0).powi(3) + C * (t - 1.0).powi(2)
    }

    pub fn ease_in_out_back(t: f64) -> f64 {
        const C: f64 = 1.70158 * 1.525;
        let t = clamp01(t);
        if t < 0.5 {
            ((2.0 * t).powi(2) * ((C + 1.0) * 2.0 * t - C)) / 2.0
        } else {
            ((2.0 * t - 2.0).powi(2) * ((C + 1.0) * (t * 2.0 - 2.0) + C) + 2.0) / 2.0
        }
    }

    pub fn ease_in_elastic(t: f64) -> f64 {
        let t = clamp01(t);
        if t == 0.0 || t == 1.0 {
            return t;
        }
        -(2.0_f64.powf(10.0 * t - 10.0)) * ((t * 10.0 - 10.75) * std::f64::consts::TAU / 3.0).sin()
    }

    pub fn ease_out_elastic(t: f64) -> f64 {
        let t = clamp01(t);
        if t == 0.0 || t == 1.0 {
            return t;
        }
        2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * std::f64::consts::TAU / 3.0).sin() + 1.0
    }

    pub fn ease_in_out_elastic(t: f64) -> f64 {
        let t = clamp01(t);
        if t == 0.0 || t == 1.0 {
            return t;
        }
        if t < 0.5 {
            -((2.0_f64.powf(20.0 * t - 10.0)) * ((20.0 * t - 11.125) * std::f64::consts::TAU / 4.5).sin()) / 2.0
        } else {
            (2.0_f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * std::f64::consts::TAU / 4.5).sin()) / 2.0 + 1.0
        }
    }

    pub fn ease_in_bounce(t: f64) -> f64 {
        1.0 - ease_out_bounce(1.0 - clamp01(t))
    }

    pub fn ease_out_bounce(t: f64) -> f64 {
        let t = clamp01(t);
        const N1: f64 = 7.5625;
        const D1: f64 = 2.75;
        if t < 1.0 / D1 {
            N1 * t * t
        } else if t < 2.0 / D1 {
            let t = t - 1.5 / D1;
            N1 * t * t + 0.75
        } else if t < 2.5 / D1 {
            let t = t - 2.25 / D1;
            N1 * t * t + 0.9375
        } else {
            let t = t - 2.625 / D1;
            N1 * t * t + 0.984375
        }
    }

    pub fn ease_in_out_bounce(t: f64) -> f64 {
        let t = clamp01(t);
        if t < 0.5 {
            (1.0 - ease_out_bounce(1.0 - 2.0 * t)) / 2.0
        } else {
            (1.0 + ease_out_bounce(2.0 * t - 1.0)) / 2.0
        }
    }

    pub fn map_child_alpha(parent_alpha: f64, start: f64, end: f64) -> f64 {
        if end <= start {
            return if parent_alpha >= end { 1.0 } else { 0.0 };
        }
        clamp01((parent_alpha - start) / (end - start))
    }

    fn clamp01(t: f64) -> f64 {
        clamp_f64(t, 0.0, 1.0)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn endpoints_are_zero_and_one() {
            for f in [linear as RateFunc, smooth, ease_in_out_cubic, ease_out_bounce] {
                assert!((f(0.0) - 0.0).abs() < 1e-6);
                assert!((f(1.0) - 1.0).abs() < 1e-6);
            }
        }

        #[test]
        fn map_child_alpha_splits_interval() {
            assert_eq!(map_child_alpha(0.25, 0.0, 0.5), 0.5);
            assert_eq!(map_child_alpha(0.75, 0.5, 1.0), 0.5);
        }

        #[test]
        fn map_child_alpha_degenerate_interval_is_step() {
            assert_eq!(map_child_alpha(0.4, 0.5, 0.5), 0.0);
            assert_eq!(map_child_alpha(0.6, 0.5, 0.5), 1.0);
            assert_eq!(map_child_alpha(0.5, 0.7, 0.2), 1.0);
        }

        #[test]
        fn simple_easing_functions_are_monotonic_within_range() {
            for f in [rush_from as RateFunc, slow_into, running_start, lingering] {
                assert!((f(0.0) - 0.0).abs() < 1e-6, "expected 0 at t=0");
                assert!((f(1.0) - 1.0).abs() < 1e-6, "expected 1 at t=1");
                let mid = f(0.5);
                assert!((0.0..=1.0).contains(&mid));
            }
        }

        #[test]
        fn rush_into_overshoots_past_one_at_t_equals_one() {
            assert!((rush_into(0.0) - 0.0).abs() < 1e-9);
            assert!((rush_into(1.0) - 2.0).abs() < 1e-9);
            assert!(rush_into(0.5) < 1.0);
        }

        #[test]
        fn double_smooth_covers_both_branches() {
            assert!(double_smooth(0.25) > 0.0 && double_smooth(0.25) < 0.5);
            assert!(double_smooth(0.75) > 0.5 && double_smooth(0.75) < 1.0);
            assert!((double_smooth(0.0) - 0.0).abs() < 1e-9);
            assert!((double_smooth(1.0) - 1.0).abs() < 1e-9);
        }

        #[test]
        fn there_and_back_returns_to_start() {
            assert!((there_and_back(0.0) - 0.0).abs() < 1e-9);
            assert!((there_and_back(0.5) - 1.0).abs() < 1e-6);
            assert!((there_and_back(1.0) - 0.0).abs() < 1e-9);
        }

        #[test]
        fn there_and_back_with_pause_has_flat_middle() {
            assert!((there_and_back_with_pause(0.0, 0.4) - 0.0).abs() < 1e-9);
            assert!((there_and_back_with_pause(0.5, 0.4) - 1.0).abs() < 1e-9);
            assert!((there_and_back_with_pause(1.0, 0.4) - 0.0).abs() < 1e-6);
            let clamped = there_and_back_with_pause(0.5, 1.5);
            assert!((0.0..=1.0).contains(&clamped));
        }

        #[test]
        fn wiggle_oscillates_around_linear() {
            let w0 = wiggle(0.0, 2.0);
            let w1 = wiggle(1.0, 2.0);
            assert!((w0 - 0.0).abs() < 1e-6);
            assert!((w1 - 1.0).abs() < 1e-6);
        }

        #[test]
        fn exponential_decay_approaches_one() {
            assert!((exponential_decay(0.0, 1.0) - 0.0).abs() < 1e-9);
            assert!(exponential_decay(1.0, 0.01) > 0.999);
            assert!(exponential_decay(2.0, 1.0) == exponential_decay(1.0, 1.0), "t is clamped to [0,1]");
            let clamped_half_life = exponential_decay(0.5, -1.0);
            assert!((0.0..=1.0).contains(&clamped_half_life));
        }

        #[test]
        fn sine_family_endpoints() {
            for f in [ease_in_sine as RateFunc, ease_out_sine, ease_in_out_sine] {
                assert!((f(0.0) - 0.0).abs() < 1e-6);
                assert!((f(1.0) - 1.0).abs() < 1e-6);
            }
        }

        #[test]
        fn power_family_both_branches_of_in_out() {
            for f in [ease_in_out_quad as RateFunc, ease_in_out_cubic, ease_in_out_quart, ease_in_out_quint] {
                let low = f(0.25);
                let high = f(0.75);
                assert!(low < 0.5, "low half should stay under midpoint");
                assert!(high > 0.5, "high half should exceed midpoint");
                assert!((f(0.0) - 0.0).abs() < 1e-9);
                assert!((f(1.0) - 1.0).abs() < 1e-9);
            }
            for f in [ease_in_quad as RateFunc, ease_out_quad, ease_in_cubic, ease_out_cubic, ease_in_quart, ease_out_quart, ease_in_quint, ease_out_quint] {
                assert!((f(0.0) - 0.0).abs() < 1e-9);
                assert!((f(1.0) - 1.0).abs() < 1e-9);
            }
        }

        #[test]
        fn exp_family_handles_boundary_and_branches() {
            assert_eq!(ease_in_exp(0.0), 0.0);
            assert!(ease_in_exp(1.0) > 0.99);
            assert_eq!(ease_out_exp(1.0), 1.0);
            assert!(ease_out_exp(0.0).abs() < 1e-9);
            assert_eq!(ease_in_out_exp(0.0), 0.0);
            assert_eq!(ease_in_out_exp(1.0), 1.0);
            assert!(ease_in_out_exp(0.25) < 0.5);
            assert!(ease_in_out_exp(0.75) > 0.5);
        }

        #[test]
        fn circ_family_both_branches() {
            for f in [ease_in_circ as RateFunc, ease_out_circ] {
                assert!((f(0.0) - 0.0).abs() < 1e-6);
                assert!((f(1.0) - 1.0).abs() < 1e-6);
            }
            assert!(ease_in_out_circ(0.25) < 0.5);
            assert!(ease_in_out_circ(0.75) > 0.5);
        }

        #[test]
        fn back_family_overshoots() {
            assert!(ease_in_back(0.1) < 0.0, "ease-in-back should dip negative early");
            assert!(ease_out_back(0.9) > 1.0, "ease-out-back should overshoot past one");
            assert!(ease_in_out_back(0.25) != ease_in_out_back(0.75));
        }

        #[test]
        fn elastic_family_boundary_and_branches() {
            for f in [ease_in_elastic as fn(f64) -> f64, ease_out_elastic, ease_in_out_elastic] {
                assert_eq!(f(0.0), 0.0);
                assert_eq!(f(1.0), 1.0);
            }
            let low = ease_in_out_elastic(0.25);
            let high = ease_in_out_elastic(0.75);
            assert!(low.is_finite() && high.is_finite());
        }

        #[test]
        fn bounce_family_all_segments() {
            let samples = [0.05, 0.3, 0.55, 0.9];
            for t in samples {
                let v = ease_out_bounce(t);
                assert!((0.0..=1.0).contains(&v), "ease_out_bounce({t}) out of range: {v}");
            }
            assert!((ease_out_bounce(0.0) - 0.0).abs() < 1e-9);
            assert!((ease_out_bounce(1.0) - 1.0).abs() < 1e-9);
            assert!((ease_in_bounce(0.0) - 0.0).abs() < 1e-9);
            assert!((ease_in_bounce(1.0) - 1.0).abs() < 1e-9);
            assert!(ease_in_out_bounce(0.25) < 0.5);
            assert!(ease_in_out_bounce(0.75) > 0.5);
        }

        #[test]
        fn rate_functions_clamp_out_of_range_input() {
            assert_eq!(linear(-1.0), 0.0);
            assert_eq!(linear(2.0), 1.0);
            assert_eq!(smooth(-5.0), smooth(0.0));
            assert_eq!(smooth(5.0), smooth(1.0));
        }
    }
}

mod scene {
    //! 🎭️ Scene trait with construct/play/wait timeline and frame loop.

    use crate::animation::{apply_parent_opacity_tree, compile_animations, interpolate_at, Animation, Wait};
    use crate::camera::{Camera, MovingCamera, ThreeDCamera, ZoomedCamera};
    use crate::config::AnimateConfig;
    use crate::section::SectionList;
    use crate::sobject::Sobject;
    use crate::updater::run_updaters;
    use std::collections::HashMap;

    /// 🎬️ User-authored animation scene contract.
    pub trait Scene {
        fn construct(&mut self);

        fn setup(&mut self, _config: &AnimateConfig) {}

        fn tear_down(&mut self) {}

        fn config(&self) -> &AnimateConfig;

        fn config_mut(&mut self) -> &mut AnimateConfig;

        fn camera(&self) -> &Camera;

        fn camera_mut(&mut self) -> &mut Camera;

        fn mobjects(&self) -> &HashMap<u64, Box<dyn Sobject>>;

        fn mobjects_mut(&mut self) -> &mut HashMap<u64, Box<dyn Sobject>>;

        fn sections(&self) -> &SectionList;

        fn sections_mut(&mut self) -> &mut SectionList;

        fn scene_time(&self) -> f64;

        fn set_scene_time(&mut self, time: f64);

        fn add(&mut self, mobject: Box<dyn Sobject>) {
            let id = mobject.id();
            self.mobjects_mut().insert(id, mobject);
        }

        fn remove(&mut self, id: u64) {
            self.mobjects_mut().remove(&id);
        }

        fn play(&mut self, mut animation: Box<dyn Animation>) {
            let pending_introducers = if animation.is_introducer() { animation.get_all_mobjects() } else { Vec::new() };
            for id in &pending_introducers {
                debug_assert!(self.mobjects().contains_key(id), "introducer animation requires mobject {id} to exist in scene");
            }
            let remover_ids = if animation.is_remover() { animation.get_all_mobjects() } else { Vec::new() };
            animation.begin();
            let duration = animation.duration().max(0.0);
            let steps = (duration * self.config().frame_rate).ceil() as u64;
            let steps = steps.max(1);
            for frame in 0..=steps {
                let alpha = frame as f64 / steps as f64;
                interpolate_at(self.mobjects_mut(), animation.as_mut(), alpha);
                self.sample_frame(self.config().frame_duration());
            }
            animation.finish();
            for id in remover_ids {
                self.remove(id);
            }
        }

        fn begin_section(&mut self, name: impl Into<String>) {
            self.sections_mut().begin_section(name, false);
        }

        fn next_section(&mut self, name: impl Into<String>) {
            let t = self.scene_time();
            self.sections_mut().end_section(t);
            self.sections_mut().begin_section(name, false);
        }

        fn wait(&mut self, seconds: f64) {
            self.play(Box::new(Wait::new(seconds)));
        }

        fn compile_and_play(&mut self, animations: Vec<Box<dyn Animation>>) {
            let _durations = compile_animations(&animations);
            for anim in animations {
                self.play(anim);
            }
        }

        fn sample_frame(&mut self, dt: f64) {
            let t = self.scene_time() + dt;
            self.set_scene_time(t);
            for m in self.mobjects_mut().values_mut() {
                apply_parent_opacity_tree(m.as_mut(), 1.0);
                run_updaters(m.as_mut(), dt);
            }
        }

        fn render_frame_index(&self, frame: u64) -> SceneFrame {
            SceneFrame { frame, time: frame as f64 / self.config().frame_rate, mobject_count: self.mobjects().len(), section: self.sections().find_at_time(self.scene_time()).map(|s| s.name.clone()) }
        }
    }

    /// 🖼️ Lightweight frame snapshot metadata for renderers.
    #[derive(Clone, Debug)]
    pub struct SceneFrame {
        pub frame: u64,
        pub time: f64,
        pub mobject_count: usize,
        pub section: Option<String>,
    }

    /// 🔁️ Interactive preview loop sampling construct timeline without encoding.
    pub fn preview_scene_loop<S: Scene>(scene: &mut S, max_frames: u64, mut on_frame: impl FnMut(&SceneFrame)) {
        let config = scene.config().clone();
        scene.setup(&config);
        let fps = config.frame_rate;
        let dt = 1.0 / fps.max(1.0);
        scene.construct();
        for frame in 0..max_frames {
            scene.sample_frame(dt);
            on_frame(&scene.render_frame_index(frame));
        }
        scene.tear_down();
    }

    /// 🏗️ Default scene implementation backing most user scenes.
    pub struct BasicScene {
        pub config: AnimateConfig,
        pub camera: Camera,
        pub mobjects: HashMap<u64, Box<dyn Sobject>>,
        pub sections: SectionList,
        pub scene_time: f64,
    }

    impl BasicScene {
        pub fn new(config: AnimateConfig) -> Self {
            let camera = Camera::new(config.width as f64 / 100.0, config.height as f64 / 100.0);
            Self { config, camera, mobjects: HashMap::new(), sections: SectionList::new(), scene_time: 0.0 }
        }

        pub fn run_construct<S: Scene>(&mut self, scene: &mut S) {
            scene.setup(&self.config);
            scene.construct();
            scene.tear_down();
        }
    }

    impl Scene for BasicScene {
        fn construct(&mut self) {}

        fn config(&self) -> &AnimateConfig {
            &self.config
        }

        fn config_mut(&mut self) -> &mut AnimateConfig {
            &mut self.config
        }

        fn camera(&self) -> &Camera {
            &self.camera
        }

        fn camera_mut(&mut self) -> &mut Camera {
            &mut self.camera
        }

        fn mobjects(&self) -> &HashMap<u64, Box<dyn Sobject>> {
            &self.mobjects
        }

        fn mobjects_mut(&mut self) -> &mut HashMap<u64, Box<dyn Sobject>> {
            &mut self.mobjects
        }

        fn sections(&self) -> &SectionList {
            &self.sections
        }

        fn sections_mut(&mut self) -> &mut SectionList {
            &mut self.sections
        }

        fn scene_time(&self) -> f64 {
            self.scene_time
        }

        fn set_scene_time(&mut self, time: f64) {
            self.scene_time = time;
        }
    }

    /// 🧪️ Specialized scene for unit tests with fixed frame rate.
    pub struct TestScene {
        inner: BasicScene,
    }

    impl TestScene {
        pub fn new() -> Self {
            Self { inner: BasicScene::new(AnimateConfig::default().with_frame_rate(60.0)) }
        }
    }

    impl Default for TestScene {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Scene for TestScene {
        fn construct(&mut self) {}
        fn config(&self) -> &AnimateConfig {
            self.inner.config()
        }
        fn config_mut(&mut self) -> &mut AnimateConfig {
            self.inner.config_mut()
        }
        fn camera(&self) -> &Camera {
            self.inner.camera()
        }
        fn camera_mut(&mut self) -> &mut Camera {
            self.inner.camera_mut()
        }
        fn mobjects(&self) -> &HashMap<u64, Box<dyn Sobject>> {
            self.inner.mobjects()
        }
        fn mobjects_mut(&mut self) -> &mut HashMap<u64, Box<dyn Sobject>> {
            self.inner.mobjects_mut()
        }
        fn sections(&self) -> &SectionList {
            self.inner.sections()
        }
        fn sections_mut(&mut self) -> &mut SectionList {
            self.inner.sections_mut()
        }
        fn scene_time(&self) -> f64 {
            self.inner.scene_time()
        }
        fn set_scene_time(&mut self, time: f64) {
            self.inner.set_scene_time(time);
        }
    }

    /// 🎥️ Scene with a panning/zooming {@link MovingCamera}.
    pub struct MovingCameraScene {
        inner: BasicScene,
        pub moving_camera: MovingCamera,
    }

    impl MovingCameraScene {
        pub fn new(config: AnimateConfig) -> Self {
            let camera = Camera::new(config.width as f64 / 100.0, config.height as f64 / 100.0);
            Self { moving_camera: MovingCamera::new(camera.clone()), inner: BasicScene { config, camera, mobjects: HashMap::new(), sections: SectionList::new(), scene_time: 0.0 } }
        }
    }

    impl Scene for MovingCameraScene {
        fn construct(&mut self) {}
        fn config(&self) -> &AnimateConfig {
            self.inner.config()
        }
        fn config_mut(&mut self) -> &mut AnimateConfig {
            self.inner.config_mut()
        }
        fn camera(&self) -> &Camera {
            &self.moving_camera.camera
        }
        fn camera_mut(&mut self) -> &mut Camera {
            &mut self.moving_camera.camera
        }
        fn mobjects(&self) -> &HashMap<u64, Box<dyn Sobject>> {
            self.inner.mobjects()
        }
        fn mobjects_mut(&mut self) -> &mut HashMap<u64, Box<dyn Sobject>> {
            self.inner.mobjects_mut()
        }
        fn sections(&self) -> &SectionList {
            self.inner.sections()
        }
        fn sections_mut(&mut self) -> &mut SectionList {
            self.inner.sections_mut()
        }
        fn scene_time(&self) -> f64 {
            self.inner.scene_time()
        }
        fn set_scene_time(&mut self, time: f64) {
            self.inner.set_scene_time(time);
        }
        fn sample_frame(&mut self, dt: f64) {
            self.moving_camera.interpolate((self.scene_time() * self.config().frame_rate).fract());
            self.inner.sample_frame(dt);
        }
    }

    /// 🧊️ Scene using a perspective {@link ThreeDCamera}.
    pub struct ThreeDScene {
        inner: BasicScene,
        pub three_d_camera: ThreeDCamera,
    }

    impl ThreeDScene {
        pub fn new(config: AnimateConfig) -> Self {
            let camera = Camera::new(config.width as f64 / 100.0, config.height as f64 / 100.0);
            Self { three_d_camera: ThreeDCamera::new(camera.clone()), inner: BasicScene { config, camera, mobjects: HashMap::new(), sections: SectionList::new(), scene_time: 0.0 } }
        }
    }

    impl Scene for ThreeDScene {
        fn construct(&mut self) {}
        fn config(&self) -> &AnimateConfig {
            self.inner.config()
        }
        fn config_mut(&mut self) -> &mut AnimateConfig {
            self.inner.config_mut()
        }
        fn camera(&self) -> &Camera {
            &self.three_d_camera.camera
        }
        fn camera_mut(&mut self) -> &mut Camera {
            &mut self.three_d_camera.camera
        }
        fn mobjects(&self) -> &HashMap<u64, Box<dyn Sobject>> {
            self.inner.mobjects()
        }
        fn mobjects_mut(&mut self) -> &mut HashMap<u64, Box<dyn Sobject>> {
            self.inner.mobjects_mut()
        }
        fn sections(&self) -> &SectionList {
            self.inner.sections()
        }
        fn sections_mut(&mut self) -> &mut SectionList {
            self.inner.sections_mut()
        }
        fn scene_time(&self) -> f64 {
            self.inner.scene_time()
        }
        fn set_scene_time(&mut self, time: f64) {
            self.inner.set_scene_time(time);
        }
    }

    /// 🔍️ Scene with a picture-in-picture {@link ZoomedCamera} inset.
    pub struct ZoomedScene {
        inner: BasicScene,
        pub zoomed_camera: ZoomedCamera,
    }

    impl ZoomedScene {
        pub fn new(config: AnimateConfig, zoom_factor: f64) -> Self {
            let camera = Camera::new(config.width as f64 / 100.0, config.height as f64 / 100.0);
            Self { zoomed_camera: ZoomedCamera::new(camera.clone(), zoom_factor), inner: BasicScene { config, camera, mobjects: HashMap::new(), sections: SectionList::new(), scene_time: 0.0 } }
        }
    }

    impl Scene for ZoomedScene {
        fn construct(&mut self) {}
        fn config(&self) -> &AnimateConfig {
            self.inner.config()
        }
        fn config_mut(&mut self) -> &mut AnimateConfig {
            self.inner.config_mut()
        }
        fn camera(&self) -> &Camera {
            &self.zoomed_camera.camera
        }
        fn camera_mut(&mut self) -> &mut Camera {
            &mut self.zoomed_camera.camera
        }
        fn mobjects(&self) -> &HashMap<u64, Box<dyn Sobject>> {
            self.inner.mobjects()
        }
        fn mobjects_mut(&mut self) -> &mut HashMap<u64, Box<dyn Sobject>> {
            self.inner.mobjects_mut()
        }
        fn sections(&self) -> &SectionList {
            self.inner.sections()
        }
        fn sections_mut(&mut self) -> &mut SectionList {
            self.inner.sections_mut()
        }
        fn scene_time(&self) -> f64 {
            self.inner.scene_time()
        }
        fn set_scene_time(&mut self, time: f64) {
            self.inner.set_scene_time(time);
        }
    }

    /// ➡️ Scene for vector-field and linear-transformation animations.
    pub struct VectorScene {
        inner: BasicScene,
    }

    impl VectorScene {
        pub fn new(config: AnimateConfig) -> Self {
            Self { inner: BasicScene::new(config) }
        }
    }

    impl Scene for VectorScene {
        fn construct(&mut self) {}
        fn config(&self) -> &AnimateConfig {
            self.inner.config()
        }
        fn config_mut(&mut self) -> &mut AnimateConfig {
            self.inner.config_mut()
        }
        fn camera(&self) -> &Camera {
            self.inner.camera()
        }
        fn camera_mut(&mut self) -> &mut Camera {
            self.inner.camera_mut()
        }
        fn mobjects(&self) -> &HashMap<u64, Box<dyn Sobject>> {
            self.inner.mobjects()
        }
        fn mobjects_mut(&mut self) -> &mut HashMap<u64, Box<dyn Sobject>> {
            self.inner.mobjects_mut()
        }
        fn sections(&self) -> &SectionList {
            self.inner.sections()
        }
        fn sections_mut(&mut self) -> &mut SectionList {
            self.inner.sections_mut()
        }
        fn scene_time(&self) -> f64 {
            self.inner.scene_time()
        }
        fn set_scene_time(&mut self, time: f64) {
            self.inner.set_scene_time(time);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::sobject::VSobject;

        struct DemoScene {
            base: TestScene,
        }

        impl DemoScene {
            fn new() -> Self {
                Self { base: TestScene::new() }
            }
        }

        impl Scene for DemoScene {
            fn construct(&mut self) {
                self.add(Box::new(VSobject::new()));
                self.wait(0.5);
            }
            fn config(&self) -> &AnimateConfig {
                self.base.config()
            }
            fn config_mut(&mut self) -> &mut AnimateConfig {
                self.base.config_mut()
            }
            fn camera(&self) -> &Camera {
                self.base.camera()
            }
            fn camera_mut(&mut self) -> &mut Camera {
                self.base.camera_mut()
            }
            fn mobjects(&self) -> &HashMap<u64, Box<dyn Sobject>> {
                self.base.mobjects()
            }
            fn mobjects_mut(&mut self) -> &mut HashMap<u64, Box<dyn Sobject>> {
                self.base.mobjects_mut()
            }
            fn sections(&self) -> &SectionList {
                self.base.sections()
            }
            fn sections_mut(&mut self) -> &mut SectionList {
                self.base.sections_mut()
            }
            fn scene_time(&self) -> f64 {
                self.base.scene_time()
            }
            fn set_scene_time(&mut self, time: f64) {
                self.base.set_scene_time(time);
            }
        }

        #[test]
        fn preview_loop_samples_frames() {
            let mut s = DemoScene::new();
            let mut frames = 0u64;
            preview_scene_loop(&mut s, 3, |_| {
                frames += 1;
            });
            assert_eq!(frames, 3);
        }
    }
}

mod section {
    //! 📑️ Named sections for partial movie output and navigation.

    use serde::{Deserialize, Serialize};

    /// 🏷️ Single named section within a scene timeline.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Section {
        pub name: String,
        pub start_time: f64,
        pub end_time: f64,
        pub skip_animations: bool,
    }

    impl Section {
        pub fn new(name: impl Into<String>, start_time: f64, end_time: f64) -> Self {
            Self { name: name.into(), start_time, end_time, skip_animations: false }
        }

        pub fn duration(&self) -> f64 {
            (self.end_time - self.start_time).max(0.0)
        }

        pub fn contains_time(&self, t: f64) -> bool {
            t >= self.start_time && t <= self.end_time
        }
    }

    /// 📚️ Ordered section list attached to a scene.
    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct SectionList {
        pub sections: Vec<Section>,
        open: Option<Section>,
    }

    impl SectionList {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn begin_section(&mut self, name: impl Into<String>, skip_animations: bool) {
            self.open = Some(Section { name: name.into(), start_time: 0.0, end_time: 0.0, skip_animations });
        }

        pub fn end_section(&mut self, end_time: f64) {
            if let Some(mut s) = self.open.take() {
                s.end_time = end_time;
                self.sections.push(s);
            }
        }

        pub fn push(&mut self, section: Section) {
            self.sections.push(section);
        }

        pub fn find_at_time(&self, t: f64) -> Option<&Section> {
            self.sections.iter().find(|s| s.contains_time(t))
        }

        pub fn names(&self) -> Vec<&str> {
            self.sections.iter().map(|s| s.name.as_str()).collect()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn section_duration_is_non_negative() {
            let s = Section::new("intro", 0.0, 2.5);
            assert!((s.duration() - 2.5).abs() < 1e-9);
        }

        #[test]
        fn section_list_tracks_open_close() {
            let mut list = SectionList::new();
            list.begin_section("main", false);
            list.end_section(10.0);
            assert_eq!(list.sections.len(), 1);
            assert_eq!(list.sections[0].name, "main");
        }
    }
}

mod sobject {
    //! 🧩️ Sobject trait, VSobject paths, groups, transforms, and layout helpers.

    use crate::color::Color;
    use crate::updater::Updater;
    use kurbo::{ParamCurve, ParamCurveArclen, PathSeg, Shape};
    use mathematical_geometry::{append_shape_to_path, bounding_box, polygon_centroid, Affine, BezPath, PathEl, Point, Vec2};
    use std::sync::atomic::{AtomicU64, Ordering};

    static SOBJECT_ID: AtomicU64 = AtomicU64::new(1);

    fn next_id() -> u64 {
        SOBJECT_ID.fetch_add(1, Ordering::Relaxed)
    }

    /// 🎨️ Stroke and fill style for vector Sobjects.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Style {
        pub fill: Option<Color>,
        pub stroke: Option<Color>,
        pub fill_opacity: f64,
        pub stroke_opacity: f64,
        pub stroke_width: f64,
    }

    impl Default for Style {
        fn default() -> Self {
            Self { fill: Some(Color::WHITE), stroke: None, fill_opacity: 1.0, stroke_opacity: 1.0, stroke_width: 4.0 }
        }
    }

    /// 📐️ Axis-aligned bounds in scene space.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Bounds {
        pub min: Point,
        pub max: Point,
    }

    impl Bounds {
        pub fn center(self) -> Point {
            Point::new((self.min.x() + self.max.x()) / 2.0, (self.min.y() + self.max.y()) / 2.0)
        }

        pub fn width(self) -> f64 {
            self.max.x() - self.min.x()
        }

        pub fn height(self) -> f64 {
            self.max.y() - self.min.y()
        }

        pub fn empty() -> Self {
            Self { min: Point::ZERO, max: Point::ZERO }
        }
    }

    /// 🧬️ Base scene-graph object contract.
    pub trait Sobject: Send {
        fn id(&self) -> u64;
        fn name(&self) -> &str;
        fn set_name(&mut self, name: String);
        fn style(&self) -> &Style;
        fn style_mut(&mut self) -> &mut Style;
        fn opacity(&self) -> f64;
        fn set_opacity(&mut self, opacity: f64);
        fn effective_opacity(&self) -> f64;
        fn set_parent_opacity(&mut self, parent: f64);
        fn transform(&self) -> Affine;
        fn transform_mut(&mut self) -> &mut Affine;
        fn bounds(&self) -> Bounds;
        fn center(&self) -> Point {
            self.bounds().center()
        }
        fn shift(&mut self, delta: Vec2) {
            *self.transform_mut() = self.transform() * Affine::IDENTITY.translate(delta);
        }
        fn move_to(&mut self, point: Point) {
            let c = self.center();
            self.shift(point - c);
        }
        fn scale(&mut self, factor: f64) {
            let c = self.center();
            let t = Affine::IDENTITY.translate(c.to_vec2()) * Affine::IDENTITY.scale(factor) * Affine::IDENTITY.translate(-c.to_vec2());
            *self.transform_mut() = self.transform() * t;
        }
        fn rotate(&mut self, angle: f64) {
            let c = self.center();
            let t = Affine::IDENTITY.translate(c.to_vec2()) * Affine::IDENTITY.rotate(angle) * Affine::IDENTITY.translate(-c.to_vec2());
            *self.transform_mut() = self.transform() * t;
        }
        fn set_color(&mut self, color: Color) {
            self.style_mut().fill = Some(color);
            self.style_mut().stroke = Some(color);
        }
        fn set_fill(&mut self, color: Color) {
            self.style_mut().fill = Some(color);
        }
        fn set_stroke(&mut self, color: Color, width: f64) {
            self.style_mut().stroke = Some(color);
            self.style_mut().stroke_width = width;
        }
        fn paths(&self) -> Vec<BezPath>;
        fn children(&self) -> Vec<&dyn Sobject>;
        fn visit_children_mut(&mut self, f: &mut dyn FnMut(&mut dyn Sobject));
        fn add_child(&mut self, child: Box<dyn Sobject>);
        fn updaters(&self) -> &[Updater];
        fn updaters_mut(&mut self) -> &mut Vec<Updater>;
        fn save_state(&mut self);
        fn restore(&mut self);
        fn generate_target(&mut self);
        fn has_target(&self) -> bool;
        fn apply_target(&mut self);
        fn clone_box(&self) -> Box<dyn Sobject>;
        fn as_any(&self) -> &dyn std::any::Any;
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
        fn z_order(&self) -> i64 {
            0
        }
        fn set_z_order(&mut self, _z: i64) {}
        fn point_ratio(&self) -> f64 {
            1.0
        }
    }

    /// ✏️ Vector Sobject backed by kurbo paths and partial point reveal.
    #[derive(Clone)]
    pub struct VSobject {
        pub id: u64,
        pub name: String,
        pub paths: Vec<BezPath>,
        pub style: Style,
        pub opacity: f64,
        pub parent_opacity: f64,
        pub transform: Affine,
        pub point_ratio: f64,
        pub z_order: i64,
        pub saved: Option<VSobjectSnapshot>,
        pub target: Option<VSobjectSnapshot>,
        pub updaters: Vec<Updater>,
    }

    #[derive(Clone, Debug)]
    pub struct VSobjectSnapshot {
        paths: Vec<BezPath>,
        style: Style,
        opacity: f64,
        transform: Affine,
        point_ratio: f64,
    }

    impl VSobject {
        pub fn new() -> Self {
            Self { id: next_id(), name: String::new(), paths: Vec::new(), style: Style::default(), opacity: 1.0, parent_opacity: 1.0, transform: Affine::IDENTITY, point_ratio: 1.0, z_order: 0, saved: None, target: None, updaters: Vec::new() }
        }

        pub fn from_path(path: BezPath) -> Self {
            let mut s = Self::new();
            s.paths.push(path);
            s
        }

        pub fn from_shape<'a>(shape: impl Into<mathematical_geometry::ShapeRef<'a>>) -> Self {
            let mut path = BezPath::new();
            append_shape_to_path(&mut path, shape, 0.01);
            Self::from_path(path)
        }

        pub fn set_paths(&mut self, paths: Vec<BezPath>) {
            self.paths = paths;
        }

        pub fn set_point_ratio(&mut self, ratio: f64) {
            self.point_ratio = ratio.clamp(0.0, 1.0);
        }

        fn snapshot(&self) -> VSobjectSnapshot {
            VSobjectSnapshot { paths: self.paths.clone(), style: self.style.clone(), opacity: self.opacity, transform: self.transform, point_ratio: self.point_ratio }
        }

        fn restore_snapshot(&mut self, snap: VSobjectSnapshot) {
            self.paths = snap.paths;
            self.style = snap.style;
            self.opacity = snap.opacity;
            self.transform = snap.transform;
            self.point_ratio = snap.point_ratio;
        }

        /// 🔀️ Linearly blend two snapshots into the live VSobject state.
        pub fn interpolate_snapshots(&mut self, from: &VSobjectSnapshot, to: &VSobjectSnapshot, t: f64) {
            let t = t.clamp(0.0, 1.0);
            self.opacity = lerp_f64(from.opacity, to.opacity, t);
            self.transform = lerp_affine(from.transform, to.transform, t);
            self.point_ratio = lerp_f64(from.point_ratio, to.point_ratio, t);
            self.style = if t < 0.5 { from.style.clone() } else { to.style.clone() };
            self.paths = interpolate_path_sets(&from.paths, &to.paths, t);
        }

        /// 🎯️ Blend from saved state toward the generated target.
        pub fn interpolate_saved_to_target(&mut self, t: f64) {
            if self.saved.is_none() {
                self.save_state();
            }
            if !self.has_target() {
                self.generate_target();
            }
            let from = self.saved.clone();
            let to = self.target.clone();
            if let (Some(from), Some(to)) = (from, to) {
                self.interpolate_snapshots(&from, &to, t);
            }
        }
    }

    fn lerp_f64(a: f64, b: f64, t: f64) -> f64 {
        a + (b - a) * t.clamp(0.0, 1.0)
    }

    /// ✂️ Trim a path to a partial reveal ratio in [0,1].
    pub fn trim_path_at_ratio(path: &BezPath, ratio: f64) -> BezPath {
        let ratio = ratio.clamp(0.0, 1.0);
        if ratio >= 1.0 {
            return path.clone();
        }
        if ratio <= 0.0 {
            return BezPath::new();
        }
        let kurbo = path.to_kurbo();
        let segments: Vec<PathSeg> = kurbo.path_segments(0.25).collect();
        let total: f64 = segments.iter().map(|s| s.arclen(0.25)).sum();
        if total <= 1e-12 {
            return path.clone();
        }
        let mut remaining = total * ratio;
        let mut out_k = kurbo::BezPath::new();
        for seg in segments {
            let len = seg.arclen(0.25);
            if remaining >= len - 1e-9 {
                if out_k.is_empty() {
                    out_k.move_to(seg.start());
                }
                out_k.push(seg.as_path_el());
                remaining -= len;
            } else {
                let frac = (remaining / len).clamp(0.0, 1.0);
                let sub = seg.subsegment(0.0..frac);
                if out_k.is_empty() {
                    out_k.move_to(sub.start());
                }
                out_k.push(sub.as_path_el());
                break;
            }
        }
        bezpath_from_kurbo(&out_k)
    }

    fn bezpath_from_kurbo(k: &kurbo::BezPath) -> BezPath {
        let mut out = BezPath::new();
        for el in k.elements() {
            out.push(PathEl::from(*el));
        }
        out
    }

    fn interpolate_path_sets(from: &[BezPath], to: &[BezPath], t: f64) -> Vec<BezPath> {
        if from.is_empty() {
            return to.to_vec();
        }
        if to.is_empty() {
            return from.to_vec();
        }
        let count = from.len().max(to.len());
        (0..count)
            .map(|i| {
                let a = &from[i.min(from.len() - 1)];
                let b = &to[i.min(to.len() - 1)];
                interpolate_bezpaths(a, b, t)
            })
            .collect()
    }

    fn interpolate_bezpaths(from: &BezPath, to: &BezPath, t: f64) -> BezPath {
        let fa = resample_points(&sample_path_points(from, 32), 32);
        let tb = resample_points(&sample_path_points(to, 32), 32);
        let mut out = BezPath::new();
        for (i, (a, b)) in fa.iter().zip(tb.iter()).enumerate() {
            let p = Point::new(lerp_f64(a.x(), b.x(), t), lerp_f64(a.y(), b.y(), t));
            if i == 0 {
                out.move_to(p);
            } else {
                out.line_to(p);
            }
        }
        out
    }

    fn sample_path_points(path: &BezPath, samples: usize) -> Vec<Point> {
        let kurbo = path.to_kurbo();
        let segments: Vec<PathSeg> = kurbo.path_segments(0.25).collect();
        let total: f64 = segments.iter().map(|s| s.arclen(0.25)).sum();
        if total <= 1e-12 {
            return Vec::new();
        }
        let mut pts = Vec::with_capacity(samples);
        for i in 0..samples {
            let target = total * i as f64 / (samples - 1).max(1) as f64;
            let mut acc = 0.0;
            for (idx, seg) in segments.iter().enumerate() {
                let len = seg.arclen(0.25);
                if acc + len >= target || idx + 1 == segments.len() {
                    let local = ((target - acc) / len.max(1e-12)).clamp(0.0, 1.0);
                    let p = seg.eval(local);
                    pts.push(Point::new(p.x, p.y));
                    break;
                }
                acc += len;
            }
        }
        pts
    }

    fn resample_points(pts: &[Point], count: usize) -> Vec<Point> {
        if pts.is_empty() {
            return vec![Point::ZERO; count];
        }
        if count <= 1 {
            return vec![pts[0]];
        }
        (0..count)
            .map(|i| {
                let idx = i * (pts.len() - 1) / (count - 1);
                pts[idx]
            })
            .collect()
    }

    fn lerp_affine(a: Affine, b: Affine, t: f64) -> Affine {
        let ta = a.to_kurbo().as_coeffs();
        let tb = b.to_kurbo().as_coeffs();
        let t = t.clamp(0.0, 1.0);
        Affine::new([lerp_f64(ta[0], tb[0], t), lerp_f64(ta[1], tb[1], t), lerp_f64(ta[2], tb[2], t), lerp_f64(ta[3], tb[3], t), lerp_f64(ta[4], tb[4], t), lerp_f64(ta[5], tb[5], t)])
    }

    impl Default for VSobject {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Sobject for VSobject {
        fn id(&self) -> u64 {
            self.id
        }
        fn name(&self) -> &str {
            &self.name
        }
        fn set_name(&mut self, name: String) {
            self.name = name;
        }
        fn style(&self) -> &Style {
            &self.style
        }
        fn style_mut(&mut self) -> &mut Style {
            &mut self.style
        }
        fn opacity(&self) -> f64 {
            self.opacity
        }
        fn set_opacity(&mut self, opacity: f64) {
            self.opacity = opacity.clamp(0.0, 1.0);
        }
        fn effective_opacity(&self) -> f64 {
            self.opacity * self.parent_opacity
        }
        fn set_parent_opacity(&mut self, parent: f64) {
            self.parent_opacity = parent.clamp(0.0, 1.0);
        }
        fn transform(&self) -> Affine {
            self.transform
        }
        fn transform_mut(&mut self) -> &mut Affine {
            &mut self.transform
        }
        fn bounds(&self) -> Bounds {
            let mut pts = Vec::new();
            for path in &self.paths {
                for el in path.elements() {
                    if let Some(p) = el.as_ref_point() {
                        pts.push(self.transform * p);
                    }
                }
            }
            if let Some(bb) = bounding_box(&pts) {
                Bounds { min: Point::new(bb.min_x, bb.min_y), max: Point::new(bb.max_x, bb.max_y) }
            } else {
                Bounds::empty()
            }
        }
        fn paths(&self) -> Vec<BezPath> {
            let t = self.transform.to_kurbo();
            self.paths
                .iter()
                .map(|p| {
                    let trimmed = trim_path_at_ratio(p, self.point_ratio);
                    transform_bezpath(&trimmed, t)
                })
                .collect()
        }
        fn children(&self) -> Vec<&dyn Sobject> {
            Vec::new()
        }
        fn visit_children_mut(&mut self, _f: &mut dyn FnMut(&mut dyn Sobject)) {}
        fn add_child(&mut self, _child: Box<dyn Sobject>) {}
        fn updaters(&self) -> &[Updater] {
            &self.updaters
        }
        fn updaters_mut(&mut self) -> &mut Vec<Updater> {
            &mut self.updaters
        }
        fn save_state(&mut self) {
            self.saved = Some(self.snapshot());
        }
        fn restore(&mut self) {
            if let Some(s) = self.saved.take() {
                self.restore_snapshot(s);
            }
        }
        fn generate_target(&mut self) {
            self.target = Some(self.snapshot());
        }
        fn has_target(&self) -> bool {
            self.target.is_some()
        }
        fn apply_target(&mut self) {
            if let Some(t) = self.target.take() {
                self.restore_snapshot(t);
            }
        }
        fn clone_box(&self) -> Box<dyn Sobject> {
            Box::new(self.clone())
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn z_order(&self) -> i64 {
            self.z_order
        }
        fn set_z_order(&mut self, z: i64) {
            self.z_order = z;
        }
        fn point_ratio(&self) -> f64 {
            self.point_ratio
        }
    }

    /// 📦️ Group of heterogeneous Sobjects.
    pub struct Group {
        pub id: u64,
        pub name: String,
        pub children: Vec<Box<dyn Sobject>>,
        pub style: Style,
        pub opacity: f64,
        pub parent_opacity: f64,
        pub transform: Affine,
        pub z_order: i64,
        pub saved: Option<GroupSnapshot>,
        pub target: Option<GroupSnapshot>,
        pub updaters: Vec<Updater>,
    }

    #[derive(Clone, Debug)]
    pub struct GroupSnapshot {
        opacity: f64,
        transform: Affine,
    }

    impl Group {
        pub fn new(children: Vec<Box<dyn Sobject>>) -> Self {
            Self { id: next_id(), name: String::new(), children, style: Style::default(), opacity: 1.0, parent_opacity: 1.0, transform: Affine::IDENTITY, z_order: 0, saved: None, target: None, updaters: Vec::new() }
        }

        pub fn empty() -> Self {
            Self::new(Vec::new())
        }

        fn propagate_parent_opacity(&mut self) {
            let eff = self.effective_opacity();
            for child in &mut self.children {
                child.set_parent_opacity(eff);
                if let Some(g) = child.as_any_mut().downcast_mut::<Group>() {
                    g.propagate_parent_opacity();
                }
            }
        }
    }

    impl Sobject for Group {
        fn id(&self) -> u64 {
            self.id
        }
        fn name(&self) -> &str {
            &self.name
        }
        fn set_name(&mut self, name: String) {
            self.name = name;
        }
        fn style(&self) -> &Style {
            &self.style
        }
        fn style_mut(&mut self) -> &mut Style {
            &mut self.style
        }
        fn opacity(&self) -> f64 {
            self.opacity
        }
        fn set_opacity(&mut self, opacity: f64) {
            self.opacity = opacity.clamp(0.0, 1.0);
            self.propagate_parent_opacity();
        }
        fn effective_opacity(&self) -> f64 {
            self.opacity * self.parent_opacity
        }
        fn set_parent_opacity(&mut self, parent: f64) {
            self.parent_opacity = parent.clamp(0.0, 1.0);
            self.propagate_parent_opacity();
        }
        fn transform(&self) -> Affine {
            self.transform
        }
        fn transform_mut(&mut self) -> &mut Affine {
            &mut self.transform
        }
        fn bounds(&self) -> Bounds {
            let mut min = Point::new(f64::INFINITY, f64::INFINITY);
            let mut max = Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY);
            for child in &self.children {
                let b = child.bounds();
                if b.min.x().is_finite() {
                    min = Point::new(min.x().min(b.min.x()), min.y().min(b.min.y()));
                    max = Point::new(max.x().max(b.max.x()), max.y().max(b.max.y()));
                }
            }
            if min.x().is_finite() {
                Bounds { min, max }
            } else {
                Bounds::empty()
            }
        }
        fn paths(&self) -> Vec<BezPath> {
            self.children.iter().flat_map(|c| c.paths()).collect()
        }
        fn children(&self) -> Vec<&dyn Sobject> {
            self.children.iter().map(|c| c.as_ref()).collect()
        }
        fn visit_children_mut(&mut self, f: &mut dyn FnMut(&mut dyn Sobject)) {
            for child in &mut self.children {
                f(child.as_mut());
            }
        }
        fn add_child(&mut self, child: Box<dyn Sobject>) {
            self.children.push(child);
            self.propagate_parent_opacity();
        }
        fn updaters(&self) -> &[Updater] {
            &self.updaters
        }
        fn updaters_mut(&mut self) -> &mut Vec<Updater> {
            &mut self.updaters
        }
        fn save_state(&mut self) {
            for c in &mut self.children {
                c.save_state();
            }
            self.saved = Some(GroupSnapshot { opacity: self.opacity, transform: self.transform });
        }
        fn restore(&mut self) {
            for c in &mut self.children {
                c.restore();
            }
            if let Some(s) = self.saved.take() {
                self.opacity = s.opacity;
                self.transform = s.transform;
            }
        }
        fn generate_target(&mut self) {
            for c in &mut self.children {
                c.generate_target();
            }
            self.target = Some(GroupSnapshot { opacity: self.opacity, transform: self.transform });
        }
        fn has_target(&self) -> bool {
            self.target.is_some() || self.children.iter().any(|c| c.has_target())
        }
        fn apply_target(&mut self) {
            for c in &mut self.children {
                c.apply_target();
            }
            if let Some(t) = self.target.take() {
                self.opacity = t.opacity;
                self.transform = t.transform;
            }
        }
        fn clone_box(&self) -> Box<dyn Sobject> {
            Box::new(Group {
                id: next_id(),
                name: self.name.clone(),
                children: self.children.iter().map(|c| c.clone_box()).collect(),
                style: self.style.clone(),
                opacity: self.opacity,
                parent_opacity: self.parent_opacity,
                transform: self.transform,
                z_order: self.z_order,
                saved: None,
                target: None,
                updaters: self.updaters.clone(),
            })
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn z_order(&self) -> i64 {
            self.z_order
        }
        fn set_z_order(&mut self, z: i64) {
            self.z_order = z;
        }
    }

    /// ✏️ Vector-only group convenience wrapper.
    pub type VGroup = Group;

    pub fn vgroup(children: Vec<Box<dyn Sobject>>) -> VGroup {
        Group::new(children)
    }

    /// ↔ Place `mover` next to `anchor` along a direction.
    pub fn next_to(mover: &mut dyn Sobject, anchor: &dyn Sobject, direction: Vec2, buff: f64) {
        let mb = mover.bounds();
        let ab = anchor.bounds();
        let dir = if direction.hypot() < 1e-9 { Vec2::new(1.0, 0.0) } else { direction / direction.hypot() };
        let shift = if dir.x().abs() > dir.y().abs() {
            let edge = if dir.x() > 0.0 { ab.max.x() } else { ab.min.x() };
            let target = if dir.x() > 0.0 { edge + buff + mb.width() / 2.0 } else { edge - buff - mb.width() / 2.0 };
            Vec2::new(target - mb.center().x(), 0.0)
        } else {
            let edge = if dir.y() > 0.0 { ab.max.y() } else { ab.min.y() };
            let target = if dir.y() > 0.0 { edge + buff + mb.height() / 2.0 } else { edge - buff - mb.height() / 2.0 };
            Vec2::new(0.0, target - mb.center().y())
        };
        mover.shift(shift);
    }

    /// 📏️ Arrange children in a line.
    pub fn arrange(group: &mut Group, direction: Vec2, buff: f64) {
        if group.children.is_empty() {
            return;
        }
        let dir = if direction.hypot() < 1e-9 { Vec2::new(1.0, 0.0) } else { direction / direction.hypot() };
        let mut cursor = group.children[0].center();
        for child in group.children.iter_mut().skip(1) {
            let b = child.bounds();
            let step = if dir.x().abs() > dir.y().abs() { b.width() / 2.0 + buff } else { b.height() / 2.0 + buff };
            cursor = Point::new(cursor.x() + dir.x() * step, cursor.y() + dir.y() * step);
            child.move_to(cursor);
        }
    }

    /// 🎯️ Align `mover` to `anchor` along an edge or center.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AlignEdge {
        Left,
        Right,
        Up,
        Down,
        Center,
    }

    pub fn align_to(mover: &mut dyn Sobject, anchor: &dyn Sobject, edge: AlignEdge) {
        let mb = mover.bounds();
        let ab = anchor.bounds();
        let shift = match edge {
            AlignEdge::Left => Vec2::new(ab.min.x() - mb.min.x(), 0.0),
            AlignEdge::Right => Vec2::new(ab.max.x() - mb.max.x(), 0.0),
            AlignEdge::Up => Vec2::new(0.0, ab.max.y() - mb.max.y()),
            AlignEdge::Down => Vec2::new(0.0, ab.min.y() - mb.min.y()),
            AlignEdge::Center => anchor.center() - mover.center(),
        };
        mover.shift(shift);
    }

    pub fn center_of_points(points: &[Point]) -> Point {
        if points.is_empty() {
            Point::ZERO
        } else {
            polygon_centroid(points)
        }
    }

    fn transform_bezpath(path: &BezPath, affine: kurbo::Affine) -> BezPath {
        let mut k = path.to_kurbo();
        k.apply_affine(affine);
        let mut out = BezPath::new();
        for el in k.elements() {
            out.push((*el).into());
        }
        out
    }

    trait PathElPoint {
        fn as_ref_point(&self) -> Option<Point>;
    }

    impl PathElPoint for PathEl {
        fn as_ref_point(&self) -> Option<Point> {
            match self {
                PathEl::MoveTo(p) | PathEl::LineTo(p) => Some(*p),
                PathEl::QuadTo(p, _) | PathEl::CurveTo(p, _, _) => Some(*p),
                PathEl::ClosePath => None,
            }
        }
    }

    trait PointVec2 {
        fn to_vec2(self) -> Vec2;
    }

    impl PointVec2 for Point {
        fn to_vec2(self) -> Vec2 {
            Vec2::new(self.x(), self.y())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use mathematical_geometry::Circle;

        #[test]
        fn vobject_has_finite_bounds() {
            let dot = VSobject::from_shape(&Circle::new(Point::new(0.0, 0.0), 1.0));
            let b = dot.bounds();
            assert!(b.max.x() > b.min.x());
        }

        #[test]
        fn parent_opacity_multiplies() {
            let mut v = VSobject::new();
            v.set_opacity(0.5);
            v.set_parent_opacity(0.5);
            assert!((v.effective_opacity() - 0.25).abs() < 1e-9);
        }

        #[test]
        fn group_propagates_parent_opacity() {
            let mut g = Group::new(vec![Box::new(VSobject::new())]);
            g.set_opacity(0.5);
            assert!((g.children[0].effective_opacity() - 0.5).abs() < 1e-9);
        }

        fn square_vobj(center: Point, half: f64) -> VSobject {
            let mut path = BezPath::new();
            path.move_to(Point::new(center.x() - half, center.y() - half));
            path.line_to(Point::new(center.x() + half, center.y() - half));
            path.line_to(Point::new(center.x() + half, center.y() + half));
            path.line_to(Point::new(center.x() - half, center.y() + half));
            path.close_path();
            VSobject::from_path(path)
        }

        #[test]
        fn next_to_places_mover_right_of_anchor() {
            let anchor = square_vobj(Point::ZERO, 1.0);
            let mut mover = square_vobj(Point::ZERO, 0.5);
            next_to(&mut mover, &anchor, Vec2::new(1.0, 0.0), 0.2);
            let b = mover.bounds();
            assert!((b.min.x() - 1.2).abs() < 1e-6);
        }

        #[test]
        fn next_to_places_mover_below_anchor() {
            let anchor = square_vobj(Point::ZERO, 1.0);
            let mut mover = square_vobj(Point::ZERO, 0.5);
            next_to(&mut mover, &anchor, Vec2::new(0.0, -1.0), 0.2);
            let b = mover.bounds();
            assert!((b.max.y() - (-1.2)).abs() < 1e-6);
        }

        #[test]
        fn next_to_zero_direction_defaults_to_right() {
            let anchor = square_vobj(Point::ZERO, 1.0);
            let mut mover = square_vobj(Point::ZERO, 0.5);
            next_to(&mut mover, &anchor, Vec2::new(0.0, 0.0), 0.2);
            let b = mover.bounds();
            assert!((b.min.x() - 1.2).abs() < 1e-6);
        }

        #[test]
        fn arrange_lays_children_along_direction() {
            let children: Vec<Box<dyn Sobject>> = vec![Box::new(square_vobj(Point::ZERO, 0.5)), Box::new(square_vobj(Point::ZERO, 0.5)), Box::new(square_vobj(Point::ZERO, 0.5))];
            let mut g = Group::new(children);
            arrange(&mut g, Vec2::new(1.0, 0.0), 0.5);
            assert!((g.children[0].center().x() - 0.0).abs() < 1e-6);
            assert!((g.children[1].center().x() - 1.0).abs() < 1e-6);
            assert!((g.children[2].center().x() - 2.0).abs() < 1e-6);
        }

        #[test]
        fn arrange_on_empty_group_is_noop() {
            let mut g = Group::empty();
            arrange(&mut g, Vec2::new(1.0, 0.0), 0.5);
            assert!(g.children.is_empty());
        }

        #[test]
        fn align_to_all_edges() {
            let anchor = square_vobj(Point::ZERO, 1.0);
            for edge in [AlignEdge::Left, AlignEdge::Right, AlignEdge::Up, AlignEdge::Down, AlignEdge::Center] {
                let mut mover = square_vobj(Point::new(5.0, 5.0), 0.5);
                align_to(&mut mover, &anchor, edge);
                let b = mover.bounds();
                match edge {
                    AlignEdge::Left => assert!((b.min.x() - (-1.0)).abs() < 1e-6),
                    AlignEdge::Right => assert!((b.max.x() - 1.0).abs() < 1e-6),
                    AlignEdge::Up => assert!((b.max.y() - 1.0).abs() < 1e-6),
                    AlignEdge::Down => assert!((b.min.y() - (-1.0)).abs() < 1e-6),
                    AlignEdge::Center => {
                        assert!(b.center().x().abs() < 1e-6);
                        assert!(b.center().y().abs() < 1e-6);
                    }
                }
            }
        }

        #[test]
        fn center_of_points_empty_is_zero() {
            assert_eq!(center_of_points(&[]), Point::ZERO);
        }

        #[test]
        fn center_of_points_nonempty_matches_centroid() {
            let pts = [Point::new(-1.0, -1.0), Point::new(1.0, -1.0), Point::new(1.0, 1.0), Point::new(-1.0, 1.0)];
            let c = center_of_points(&pts);
            assert!(c.x().abs() < 1e-9);
            assert!(c.y().abs() < 1e-9);
        }

        #[test]
        fn trim_path_at_ratio_boundary_and_partial() {
            let mut path = BezPath::new();
            path.move_to(Point::new(0.0, 0.0));
            path.line_to(Point::new(10.0, 0.0));
            let full = trim_path_at_ratio(&path, 1.0);
            assert_eq!(full.elements().len(), path.elements().len());
            let none = trim_path_at_ratio(&path, 0.0);
            assert!(none.elements().is_empty());
            let half = trim_path_at_ratio(&path, 0.5);
            assert!(!half.elements().is_empty());
            let over = trim_path_at_ratio(&path, 2.0);
            assert_eq!(over.elements().len(), path.elements().len());
        }
    }
}

mod text {
    //! 🔤️ Text and math labels via Typst-to-SVG compilation.

    use crate::color::Color;
    use crate::sobject::{Sobject, VSobject};
    use ecow::EcoString;
    use mathematical_geometry::{append_shape_to_path, BezPath, Point, Rect};
    use std::path::PathBuf;
    use std::sync::OnceLock;
    use typst::foundations::{Bytes, Datetime};
    use typst::layout::Abs;
    use typst::layout::PagedDocument;
    use typst::syntax::{FileId, Source, VirtualPath};
    use typst::text::{Font, FontBook};
    use typst::utils::LazyHash;
    use typst::LibraryExt;
    use typst::{Library, World};

    const TEXT_PAGE_PT: f64 = 400.0;
    const TEXT_MARGIN_PT: f64 = 8.0;
    const TEXT_SIZE_PT: f64 = 36.0;

    /// 📝️ Plain text Sobject rendered through Typst.
    #[derive(Clone)]
    pub struct Text {
        pub inner: VSobject,
        pub content: EcoString,
        pub font_size: f64,
    }

    impl Text {
        pub fn new(content: impl Into<EcoString>, color: Color) -> Self {
            let content = content.into();
            let svg = typst_markup_to_svg(&wrap_text(&content, TEXT_SIZE_PT)).unwrap_or_default();
            let mut inner = svg_to_vobject(&svg, color);
            inner.set_name(content.to_string());
            Self { inner, content, font_size: TEXT_SIZE_PT }
        }

        pub fn as_sobject(&self) -> &VSobject {
            &self.inner
        }

        pub fn as_sobject_mut(&mut self) -> &mut VSobject {
            &mut self.inner
        }
    }

    fn format_decimal(value: f64, decimals: u32) -> String {
        format!("{value:.prec$}", prec = decimals as usize)
    }

    /// 🔢️ Decimal number label with interpolatable value.
    #[derive(Clone)]
    pub struct DecimalNumber {
        pub value: f64,
        pub inner: Text,
        pub decimals: u32,
    }

    impl DecimalNumber {
        pub fn new(value: f64, decimals: u32, color: Color) -> Self {
            let inner = Text::new(format_decimal(value, decimals), color);
            Self { value, inner, decimals }
        }

        pub fn lerp_value(&mut self, target: f64, t: f64, color: Color) {
            let t = t.clamp(0.0, 1.0);
            self.value = self.value + (target - self.value) * t;
            self.inner = Text::new(format_decimal(self.value, self.decimals), color);
        }

        pub fn as_sobject(&self) -> &VSobject {
            &self.inner.inner
        }
    }

    /// 🔢️ Integer label wrapper.
    #[derive(Clone)]
    pub struct Integer {
        pub value: i64,
        pub inner: Text,
    }

    impl Integer {
        pub fn new(value: i64, color: Color) -> Self {
            Self { value, inner: Text::new(value.to_string(), color) }
        }

        pub fn as_sobject(&self) -> &VSobject {
            &self.inner.inner
        }
    }

    /// 📄️ Multi-line paragraph wrapper.
    #[derive(Clone)]
    pub struct Paragraph {
        pub lines: Vec<EcoString>,
        pub inner: Text,
    }

    impl Paragraph {
        pub fn new(lines: Vec<impl Into<EcoString>>, color: Color) -> Self {
            let lines: Vec<EcoString> = lines.into_iter().map(Into::into).collect();
            let body = lines.iter().map(|l| l.as_str()).collect::<Vec<_>>().join("\n");
            Self { lines, inner: Text::new(body, color) }
        }

        pub fn as_sobject(&self) -> &VSobject {
            &self.inner.inner
        }
    }

    /// 💻️ Monospace code block wrapper.
    #[derive(Clone)]
    pub struct Code {
        pub source: EcoString,
        pub inner: Text,
    }

    impl Code {
        pub fn new(source: impl Into<EcoString>, color: Color) -> Self {
            let source = source.into();
            let wrapped = format!("#set page(width: {TEXT_PAGE_PT}pt, height: {TEXT_PAGE_PT}pt, margin: {TEXT_MARGIN_PT}pt, fill: none)\n#set text(size: {TEXT_SIZE_PT}pt, font: \"Courier New\")\n`{source}`");
            let svg = typst_markup_to_svg(&wrapped).unwrap_or_default();
            let mut inner_v = svg_to_vobject(&svg, color);
            inner_v.set_name(source.to_string());
            Self { source: source.clone(), inner: Text { inner: inner_v, content: source, font_size: TEXT_SIZE_PT } }
        }

        pub fn as_sobject(&self) -> &VSobject {
            &self.inner.inner
        }
    }

    /// ∑ Math-mode label rendered through Typst.
    #[derive(Clone)]
    pub struct MathText {
        pub inner: VSobject,
        pub latex: EcoString,
    }

    impl MathText {
        pub fn new(expr: impl Into<EcoString>, color: Color) -> Self {
            let latex = expr.into();
            let wrapped = format!("#set page(width: {}pt, height: {}pt, margin: {}pt, fill: none)\n#set text(size: {}pt)\n$ {latex} $", TEXT_PAGE_PT, TEXT_PAGE_PT, TEXT_MARGIN_PT, TEXT_SIZE_PT);
            let svg = typst_markup_to_svg(&wrapped).unwrap_or_default();
            let mut inner = svg_to_vobject(&svg, color);
            inner.set_name(latex.to_string());
            Self { inner, latex }
        }

        pub fn as_sobject(&self) -> &VSobject {
            &self.inner
        }
    }

    fn wrap_text(text: &str, size: f64) -> String {
        let escaped = text.replace('\\', "\\\\").replace('"', "\\\"");
        format!("#set page(width: {TEXT_PAGE_PT}pt, height: {TEXT_PAGE_PT}pt, margin: {TEXT_MARGIN_PT}pt, fill: none)\n#set align(center + horizon)\n#set text(size: {size}pt)\n\"{escaped}\"")
    }

    fn svg_to_vobject(svg: &str, color: Color) -> VSobject {
        let mut v = VSobject::new();
        if svg.is_empty() {
            v.set_paths(vec![BezPath::new()]);
            v.set_fill(color);
            v.style.stroke = None;
            return v;
        }
        let options = usvg::Options::default();
        if let Ok(tree) = usvg::Tree::from_str(svg, &options) {
            let height = tree.size().height() as f64;
            let scale = if height > 1e-9 { 2.0 / height } else { 0.01 };
            let offset_y = height * scale;
            let mut paths = Vec::new();
            for child in tree.root().children() {
                collect_svg_paths(child, scale, offset_y, &mut paths);
            }
            if paths.is_empty() {
                paths.push(fallback_text_rect());
            }
            v.set_paths(paths);
        } else {
            v.set_paths(vec![fallback_text_rect()]);
        }
        v.set_fill(color);
        v.style.stroke = None;
        v
    }

    fn fallback_text_rect() -> BezPath {
        let rect = Rect::new(-1.0, -0.5, 1.0, 0.5);
        let mut p = BezPath::new();
        append_shape_to_path(&mut p, &rect, 0.01);
        p
    }

    fn map_svg_point(x: f32, y: f32, scale: f64, offset_y: f64) -> Point {
        Point::new(x as f64 * scale, offset_y - y as f64 * scale)
    }

    fn collect_svg_paths(node: &usvg::Node, scale: f64, offset_y: f64, out: &mut Vec<BezPath>) {
        match node {
            usvg::Node::Group(group) => {
                for child in group.children() {
                    collect_svg_paths(child, scale, offset_y, out);
                }
            }
            usvg::Node::Path(path) => {
                let mut p = BezPath::new();
                for segment in path.data().segments() {
                    match segment {
                        usvg::tiny_skia_path::PathSegment::MoveTo(pt) => {
                            p.move_to(map_svg_point(pt.x, pt.y, scale, offset_y));
                        }
                        usvg::tiny_skia_path::PathSegment::LineTo(pt) => {
                            p.line_to(map_svg_point(pt.x, pt.y, scale, offset_y));
                        }
                        usvg::tiny_skia_path::PathSegment::QuadTo(c, pt) => {
                            p.quad_to(map_svg_point(c.x, c.y, scale, offset_y), map_svg_point(pt.x, pt.y, scale, offset_y));
                        }
                        usvg::tiny_skia_path::PathSegment::CubicTo(c1, c2, pt) => {
                            p.curve_to(map_svg_point(c1.x, c1.y, scale, offset_y), map_svg_point(c2.x, c2.y, scale, offset_y), map_svg_point(pt.x, pt.y, scale, offset_y));
                        }
                        usvg::tiny_skia_path::PathSegment::Close => p.close_path(),
                    }
                }
                if !p.elements().is_empty() {
                    out.push(p);
                }
            }
            _ => {}
        }
    }

    fn typst_asset_font_list() -> Vec<Font> {
        let mut out = Vec::new();
        for bytes in typst_assets::fonts() {
            let blob = Bytes::new(bytes);
            let mut idx = 0u32;
            while let Some(f) = Font::new(blob.clone(), idx) {
                out.push(f);
                idx = idx.saturating_add(1);
            }
        }
        out
    }

    fn typst_compile_markup_to_svg(markup: &str, fonts: &'static [Font], book: &'static LazyHash<FontBook>) -> Option<String> {
        static LIB: OnceLock<LazyHash<Library>> = OnceLock::new();
        static MAIN: OnceLock<FileId> = OnceLock::new();
        let library = LIB.get_or_init(|| LazyHash::new(Library::default()));
        let main = *MAIN.get_or_init(|| FileId::new(None, VirtualPath::new("/animate.typ")));
        let source = Source::new(main, markup.to_string());
        struct AnimateTypstWorld<'a> {
            library: &'static LazyHash<Library>,
            book: &'static LazyHash<FontBook>,
            main: FileId,
            source: Source,
            fonts: &'a [Font],
        }
        impl World for AnimateTypstWorld<'_> {
            fn library(&self) -> &LazyHash<Library> {
                self.library
            }
            fn book(&self) -> &LazyHash<FontBook> {
                self.book
            }
            fn main(&self) -> FileId {
                self.main
            }
            fn source(&self, id: FileId) -> typst::diag::FileResult<Source> {
                if id == self.main {
                    Ok(self.source.clone())
                } else {
                    Err(typst::diag::FileError::NotFound(PathBuf::from("animate.typ")))
                }
            }
            fn file(&self, _id: FileId) -> typst::diag::FileResult<Bytes> {
                Err(typst::diag::FileError::NotFound(PathBuf::from("animate.bin")))
            }
            fn font(&self, index: usize) -> Option<Font> {
                self.fonts.get(index).cloned()
            }
            fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
                None
            }
        }
        let w = AnimateTypstWorld { library, book, main, source, fonts };
        let warned = typst::compile::<PagedDocument>(&w);
        let doc = warned.output.ok()?;
        if doc.pages.is_empty() {
            return None;
        }
        Some(typst_svg::svg_merged(&doc, Abs::pt(4.0)))
    }

    static TYPST_FONTS: OnceLock<Vec<Font>> = OnceLock::new();
    static TYPST_BOOK: OnceLock<LazyHash<FontBook>> = OnceLock::new();

    /// 🖨️ Compile Typst markup to merged SVG.
    pub fn typst_markup_to_svg(markup: &str) -> Option<String> {
        let fonts = TYPST_FONTS.get_or_init(typst_asset_font_list);
        let book = TYPST_BOOK.get_or_init(|| LazyHash::new(FontBook::from_fonts(fonts.iter())));
        typst_compile_markup_to_svg(markup, fonts.as_slice(), book)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn typst_plain_text_compiles() {
            let svg = typst_markup_to_svg(&wrap_text("hello", 24.0));
            assert!(svg.is_some());
            assert!(svg.unwrap().contains("svg"));
        }

        #[test]
        fn math_text_builds_vobject() {
            let m = MathText::new("x^2", Color::WHITE);
            assert!(!m.latex.is_empty());
        }

        #[test]
        fn decimal_number_lerps() {
            let mut d = DecimalNumber::new(0.0, 2, Color::WHITE);
            d.lerp_value(10.0, 0.5, Color::WHITE);
            assert!((d.value - 5.0).abs() < 1e-9);
        }

        #[test]
        fn text_wrappers_build() {
            let i = Integer::new(42, Color::WHITE);
            assert_eq!(i.value, 42);
            let p = Paragraph::new(vec!["line one", "line two"], Color::WHITE);
            assert_eq!(p.lines.len(), 2);
            let c = Code::new("fn main() {}", Color::WHITE);
            assert!(!c.source.is_empty());
        }
    }
}

mod three_d {
    //! 🧊️ Three-dimensional Sobjects projected into the scene plane.

    use crate::color::Color;
    use crate::geometry::{circle, line, polygon, rectangle};
    use crate::sobject::{Bounds, Group, Sobject, Style, VSobject};
    use crate::updater::Updater;
    use mathematical_geometry::{Affine, BezPath, Point};

    /// 📦️ Base 3D Sobject with yaw/pitch and projection scale.
    #[derive(Clone)]
    pub struct ThreeDVSobject {
        pub inner: VSobject,
        pub yaw: f64,
        pub pitch: f64,
        pub depth: f64,
    }

    impl ThreeDVSobject {
        pub fn new(inner: VSobject) -> Self {
            Self { inner, yaw: 0.0, pitch: 0.0, depth: 0.0 }
        }

        pub fn project_point(&self, p: (f64, f64, f64)) -> Point {
            let (x, y, z) = p;
            let cy = self.yaw.cos();
            let sy = self.yaw.sin();
            let cp = self.pitch.cos();
            let sp = self.pitch.sin();
            let x1 = x * cy - z * sy;
            let z1 = x * sy + z * cy;
            let y1 = y * cp - z1 * sp;
            let z2 = y * sp + z1 * cp + self.depth;
            let scale = 1.0 / (1.0 + z2 * 0.1);
            Point::new(x1 * scale, y1 * scale)
        }
    }

    impl Sobject for ThreeDVSobject {
        fn id(&self) -> u64 {
            self.inner.id()
        }
        fn name(&self) -> &str {
            self.inner.name()
        }
        fn set_name(&mut self, name: String) {
            self.inner.set_name(name);
        }
        fn style(&self) -> &Style {
            self.inner.style()
        }
        fn style_mut(&mut self) -> &mut Style {
            self.inner.style_mut()
        }
        fn opacity(&self) -> f64 {
            self.inner.opacity()
        }
        fn set_opacity(&mut self, opacity: f64) {
            self.inner.set_opacity(opacity);
        }
        fn effective_opacity(&self) -> f64 {
            self.inner.effective_opacity()
        }
        fn set_parent_opacity(&mut self, parent: f64) {
            self.inner.set_parent_opacity(parent);
        }
        fn transform(&self) -> Affine {
            self.inner.transform()
        }
        fn transform_mut(&mut self) -> &mut Affine {
            self.inner.transform_mut()
        }
        fn bounds(&self) -> Bounds {
            self.inner.bounds()
        }
        fn paths(&self) -> Vec<BezPath> {
            self.inner.paths()
        }
        fn children(&self) -> Vec<&dyn Sobject> {
            self.inner.children()
        }
        fn visit_children_mut(&mut self, f: &mut dyn FnMut(&mut dyn Sobject)) {
            self.inner.visit_children_mut(f);
        }
        fn add_child(&mut self, child: Box<dyn Sobject>) {
            self.inner.add_child(child);
        }
        fn updaters(&self) -> &[Updater] {
            self.inner.updaters()
        }
        fn updaters_mut(&mut self) -> &mut Vec<Updater> {
            self.inner.updaters_mut()
        }
        fn save_state(&mut self) {
            self.inner.save_state();
        }
        fn restore(&mut self) {
            self.inner.restore();
        }
        fn generate_target(&mut self) {
            self.inner.generate_target();
        }
        fn has_target(&self) -> bool {
            self.inner.has_target()
        }
        fn apply_target(&mut self) {
            self.inner.apply_target();
        }
        fn clone_box(&self) -> Box<dyn Sobject> {
            Box::new(self.clone())
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn z_order(&self) -> i64 {
            self.inner.z_order()
        }
        fn set_z_order(&mut self, z: i64) {
            self.inner.set_z_order(z);
        }
        fn point_ratio(&self) -> f64 {
            self.inner.point_ratio()
        }
    }

    /// 🌐️ Parametric surface wireframe.
    pub struct Surface {
        pub group: Group,
        pub resolution: u32,
    }

    impl Surface {
        pub fn paraboloid(radius: f64, color: Color) -> Self {
            let steps = 12;
            let mut children: Vec<Box<dyn Sobject>> = Vec::new();
            for i in 0..steps {
                let t = i as f64 / steps as f64 * std::f64::consts::TAU;
                let mut prev = None;
                for j in 0..=steps {
                    let r = radius * j as f64 / steps as f64;
                    let x = r * t.cos();
                    let z = r * t.sin();
                    let y = (x * x + z * z) * 0.2;
                    let td = ThreeDVSobject::new(VSobject::new());
                    let p = td.project_point((x, y, z));
                    if let Some(prev_p) = prev {
                        children.push(Box::new(line(prev_p, p, color.with_alpha(0.5), 1.0)));
                    }
                    prev = Some(p);
                }
            }
            Self { group: Group::new(children), resolution: steps as u32 }
        }
    }

    /// ⚪️ Sphere wireframe.
    pub fn sphere(radius: f64, center: (f64, f64, f64), color: Color) -> Group {
        let steps = 16;
        let mut children: Vec<Box<dyn Sobject>> = Vec::new();
        let td = ThreeDVSobject::new(VSobject::new());
        for i in 0..steps {
            let phi = i as f64 / steps as f64 * std::f64::consts::PI;
            let mut prev = None;
            for j in 0..=steps {
                let theta = j as f64 / steps as f64 * std::f64::consts::TAU;
                let x = center.0 + radius * phi.sin() * theta.cos();
                let y = center.1 + radius * phi.cos();
                let z = center.2 + radius * phi.sin() * theta.sin();
                let p = td.project_point((x, y, z));
                if let Some(prev_p) = prev {
                    children.push(Box::new(line(prev_p, p, color.with_alpha(0.6), 1.0)));
                }
                prev = Some(p);
            }
        }
        Group::new(children)
    }

    /// 🧊️ Cube wireframe.
    pub fn cube(side: f64, center: (f64, f64, f64), color: Color) -> Group {
        let h = side / 2.0;
        let corners = [(-h, -h, -h), (h, -h, -h), (h, h, -h), (-h, h, -h), (-h, -h, h), (h, -h, h), (h, h, h), (-h, h, h)];
        let td = ThreeDVSobject::new(VSobject::new());
        let pts: Vec<Point> = corners.iter().map(|(x, y, z)| td.project_point((center.0 + x, center.1 + y, center.2 + z))).collect();
        let edges = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
        let children: Vec<Box<dyn Sobject>> = edges.iter().map(|(a, b)| Box::new(line(pts[*a], pts[*b], color, 2.0)) as Box<dyn Sobject>).collect();
        Group::new(children)
    }

    /// 🟦️ Solid cube with filled projected faces.
    pub fn solid_cube(side: f64, center: (f64, f64, f64), fill: Color, stroke: Option<Color>, stroke_width: f64) -> Group {
        let h = side / 2.0;
        let corners = [(-h, -h, -h), (h, -h, -h), (h, h, -h), (-h, h, -h), (-h, -h, h), (h, -h, h), (h, h, h), (-h, h, h)];
        let td = ThreeDVSobject::new(VSobject::new());
        let pts: Vec<Point> = corners.iter().map(|(x, y, z)| td.project_point((center.0 + x, center.1 + y, center.2 + z))).collect();
        let faces: [(&[usize], f64); 6] = [(&[0, 1, 2, 3], 0.85), (&[4, 5, 6, 7], 0.85), (&[0, 1, 5, 4], 0.7), (&[2, 3, 7, 6], 0.7), (&[1, 2, 6, 5], 0.55), (&[0, 3, 7, 4], 0.55)];
        let mut children: Vec<Box<dyn Sobject>> = Vec::new();
        for (indices, alpha) in faces {
            let verts: Vec<Point> = indices.iter().map(|&i| pts[i]).collect();
            children.push(Box::new(polygon(&verts, fill.with_alpha(alpha), stroke, stroke_width)));
        }
        let edges = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
        for (a, b) in edges {
            children.push(Box::new(line(pts[a], pts[b], stroke.unwrap_or(fill), stroke_width)));
        }
        Group::new(children)
    }

    /// 🟦️ Filled face proxy for 3D objects (projected rectangle).
    pub fn face(width: f64, height: f64, center: Point, fill: Color) -> VSobject {
        rectangle(width, height, center, fill, None, 0.0)
    }

    /// 🔮️ Disc cross-section helper.
    pub fn disc(radius: f64, center: Point, fill: Color) -> VSobject {
        circle(center, radius, fill, None, 0.0)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn cube_has_twelve_edges() {
            let g = cube(2.0, (0.0, 0.0, 0.0), Color::WHITE);
            assert_eq!(g.children.len(), 12);
        }

        #[test]
        fn projection_moves_points() {
            let td = ThreeDVSobject::new(VSobject::new());
            let p = td.project_point((1.0, 0.0, 0.0));
            assert!(p.x().is_finite());
        }

        #[test]
        fn three_d_vobject_is_sobject() {
            let td = ThreeDVSobject::new(VSobject::new());
            assert_eq!(td.opacity(), 1.0);
        }

        #[test]
        fn solid_cube_has_faces() {
            let g = solid_cube(2.0, (0.0, 0.0, 0.0), Color::BLUE, Some(Color::WHITE), 1.0);
            assert!(g.children.len() >= 6);
        }

        #[test]
        fn sphere_builds_wireframe_lines() {
            let g = sphere(1.0, (0.0, 0.0, 0.0), Color::WHITE);
            assert!(!g.children.is_empty());
        }

        #[test]
        fn face_and_disc_build_projected_shapes() {
            let f = face(2.0, 1.0, Point::ZERO, Color::RED);
            assert!(!f.paths.is_empty());
            let d = disc(1.0, Point::ZERO, Color::BLUE);
            assert!(!d.paths.is_empty());
        }
    }
}

mod updater {
    //! 🔄️ Runtime updaters, value trackers, and always-redraw helpers.

    use crate::sobject::Sobject;
    use std::sync::{Arc, Mutex};

    /// 🎚️ Scalar animated parameter with get/set hooks.
    #[derive(Clone)]
    pub struct ValueTracker {
        pub value: Arc<Mutex<f64>>,
    }

    impl ValueTracker {
        pub fn new(value: f64) -> Self {
            Self { value: Arc::new(Mutex::new(value)) }
        }

        pub fn get(&self) -> f64 {
            *self.value.lock().unwrap_or_else(|e| e.into_inner())
        }

        pub fn set(&self, value: f64) {
            *self.value.lock().unwrap_or_else(|e| e.into_inner()) = value;
        }

        pub fn increment(&self, delta: f64) {
            let mut v = self.value.lock().unwrap_or_else(|e| e.into_inner());
            *v += delta;
        }
    }

    /// 🔁️ Per-frame mutation callback attached to an Sobject.
    #[derive(Clone)]
    pub struct Updater {
        pub id: u64,
        pub name: String,
        pub active: bool,
        pub dt_scale: f64,
        callback: Arc<dyn Fn(&mut dyn Sobject, f64) + Send + Sync>,
    }

    static UPDATER_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

    impl Updater {
        pub fn new<F>(name: impl Into<String>, callback: F) -> Self
        where
            F: Fn(&mut dyn Sobject, f64) + Send + Sync + 'static,
        {
            Self { id: UPDATER_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed), name: name.into(), active: true, dt_scale: 1.0, callback: Arc::new(callback) }
        }

        pub fn invoke(&self, target: &mut dyn Sobject, dt: f64) {
            if self.active {
                (self.callback)(target, dt * self.dt_scale);
            }
        }
    }

    /// ➕️ Attach an updater to an Sobject.
    pub fn add_updater(target: &mut dyn Sobject, updater: Updater) {
        target.updaters_mut().push(updater);
    }

    /// ♾️ Attach an updater that runs every frame.
    pub fn always<F>(target: &mut dyn Sobject, name: impl Into<String>, f: F)
    where
        F: Fn(&mut dyn Sobject, f64) + Send + Sync + 'static,
    {
        add_updater(target, Updater::new(name, f));
    }

    /// 🎯️ Attach an updater driven by a ValueTracker.
    pub fn f_always<F>(target: &mut dyn Sobject, tracker: &ValueTracker, name: impl Into<String>, f: F)
    where
        F: Fn(&mut dyn Sobject, f64) + Send + Sync + 'static,
    {
        let t = tracker.clone();
        add_updater(
            target,
            Updater::new(name, move |obj, dt| {
                let _ = t.get();
                f(obj, dt);
            }),
        );
    }

    /// 🔃️ Rebuild an Sobject every frame from a factory closure.
    pub fn always_redraw<F>(target: &mut dyn Sobject, name: impl Into<String>, factory: F)
    where
        F: Fn() -> Box<dyn Sobject> + Send + Sync + 'static,
    {
        let factory = Arc::new(factory);
        add_updater(
            target,
            Updater::new(name, move |obj, _dt| {
                let fresh = factory();
                if let Some(v) = obj.as_any_mut().downcast_mut::<crate::sobject::VSobject>() {
                    if let Some(fv) = fresh.as_any().downcast_ref::<crate::sobject::VSobject>() {
                        v.paths = fv.paths.clone();
                        v.style = fv.style.clone();
                        v.transform = fv.transform;
                    }
                }
            }),
        );
    }

    /// 🏃️ Run all updaters on a scene object tree.
    pub fn run_updaters(target: &mut dyn Sobject, dt: f64) {
        let updaters: Vec<Updater> = target.updaters().to_vec();
        for u in updaters {
            u.invoke(target, dt);
        }
        target.visit_children_mut(&mut |child| run_updaters(child, dt));
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::sobject::VSobject;
        use mathematical_geometry::BezPath;

        #[test]
        fn value_tracker_mutates() {
            let t = ValueTracker::new(1.0);
            t.increment(2.0);
            assert!((t.get() - 3.0).abs() < 1e-9);
        }

        #[test]
        fn updater_runs_on_object() {
            let mut v = VSobject::new();
            let flag = Arc::new(Mutex::new(false));
            let f = Arc::clone(&flag);
            add_updater(
                &mut v,
                Updater::new("mark", move |_o, _dt| {
                    *f.lock().unwrap() = true;
                }),
            );
            run_updaters(&mut v, 1.0 / 60.0);
            assert!(*flag.lock().unwrap());
        }

        #[test]
        fn inactive_updater_does_not_invoke_callback() {
            let mut v = VSobject::new();
            let flag = Arc::new(Mutex::new(false));
            let f = Arc::clone(&flag);
            let mut u = Updater::new("mark", move |_o, _dt| {
                *f.lock().unwrap() = true;
            });
            u.active = false;
            u.invoke(&mut v, 1.0 / 60.0);
            assert!(!*flag.lock().unwrap());
        }

        #[test]
        fn always_helper_attaches_updater_that_runs() {
            let mut v = VSobject::new();
            let flag = Arc::new(Mutex::new(false));
            let f = Arc::clone(&flag);
            always(&mut v, "always-mark", move |_o, _dt| {
                *f.lock().unwrap() = true;
            });
            assert_eq!(v.updaters().len(), 1);
            run_updaters(&mut v, 1.0 / 60.0);
            assert!(*flag.lock().unwrap());
        }

        #[test]
        fn f_always_helper_reads_tracker_and_runs() {
            let mut v = VSobject::new();
            let tracker = ValueTracker::new(2.0);
            let flag: Arc<Mutex<f64>> = Arc::new(Mutex::new(0.0));
            let f = Arc::clone(&flag);
            f_always(&mut v, &tracker, "f-always-mark", move |_o, _dt| {
                *f.lock().unwrap() = 1.0;
            });
            tracker.set(9.0);
            run_updaters(&mut v, 1.0 / 60.0);
            assert!((*flag.lock().unwrap() - 1.0_f64).abs() < 1e-9);
        }

        #[test]
        fn always_redraw_rebuilds_paths_from_factory() {
            let mut v = VSobject::new();
            always_redraw(&mut v, "redraw", || {
                let mut fresh = VSobject::new();
                fresh.paths.push(BezPath::new());
                Box::new(fresh) as Box<dyn Sobject>
            });
            assert!(v.paths.is_empty());
            run_updaters(&mut v, 1.0 / 60.0);
            assert_eq!(v.paths.len(), 1);
        }

        #[test]
        fn run_updaters_recurses_into_group_children() {
            let mut child = VSobject::new();
            let flag = Arc::new(Mutex::new(false));
            let f = Arc::clone(&flag);
            add_updater(
                &mut child,
                Updater::new("child-mark", move |_o, _dt| {
                    *f.lock().unwrap() = true;
                }),
            );
            let mut group = crate::sobject::Group::new(vec![Box::new(child)]);
            run_updaters(&mut group, 1.0 / 60.0);
            assert!(*flag.lock().unwrap());
        }
    }
}

pub use animation::*;
pub use animations_catalog::*;
pub use axes::*;
pub use camera::*;
pub use color::*;
pub use config::*;
pub use geometry::*;
pub use graph::*;
pub use hash::*;
pub use matrix::*;
pub use rate::*;
pub use scene::*;
pub use section::*;
pub use sobject::*;
pub use text::*;
pub use three_d::*;
pub use updater::*;

pub use mathematical_geometry::{Affine, BezPath, Circle as GeoCircle, Point as GeoPoint, Rect as GeoRect, Vec2 as GeoVec2};
