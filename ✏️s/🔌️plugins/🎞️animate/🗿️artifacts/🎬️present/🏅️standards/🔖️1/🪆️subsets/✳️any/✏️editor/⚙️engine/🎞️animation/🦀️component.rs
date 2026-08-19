//! 🎞️ Animate app engine facet: 🎞️animation (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES:
//! relocated verbatim from the deleted artifact-tree `⚙️engine/🎞️animation`).

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod animation {
    //! 🎞️ Animation trait, leaf animations, composites, and `.animate()` builder.

    use crate::editor::animate::engine::rate::rate::{map_child_alpha, RateFunc};
    use crate::editor::animate::engine::scene::sobject::{Sobject, Sobjects, VSobject};
    use geometry::{cubic_point_at, Affine, CubicBez, Point, Vec2};
    use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
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
    #[dyn_enum]
    pub trait Animation: Send {
        async fn duration(&self) -> f64;
        async fn rate_func(&self) -> RateFunc;
        async fn begin(&mut self);
        async fn finish(&mut self);
        async fn interpolate_mobject(&mut self, alpha: f64);
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
            let rate = self.rate_func();
            let alpha = rate(parent_alpha.clamp(0.0, 1.0));
            let _ = mobjects;
            self.interpolate_mobject(alpha);
        }
        async fn get_all_mobjects(&self) -> Vec<u64>;
        async fn is_introducer(&self) -> bool {
            false
        }
        async fn is_remover(&self) -> bool {
            false
        }
    }

    async fn eased_alpha(animation: &Animations, alpha: f64) -> f64 {
        (animation.rate_func())(alpha.clamp(0.0, 1.0))
    }

    pub(crate) async fn eased_alpha_for(animation: &Animations, alpha: f64) -> f64 {
        eased_alpha(animation, alpha)
    }

    /// 🎯️ Resolve a VSobject by id and run a closure on it.
    pub async fn with_vsobject<F>(mobjects: &mut HashMap<u64, Sobjects>, id: u64, f: F)
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
    pub async fn interpolate_at(mobjects: &mut HashMap<u64, Sobjects>, animation: &mut Animations, parent_alpha: f64) {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::linear, started: false, snapshot_ratio: 1.0 }
        }

        pub async fn with_rate(mut self, rate: RateFunc) -> Self {
            self.rate = rate;
            self
        }
    }

    impl Animation for Create {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.started = true;
            self.snapshot_ratio = 1.0;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, alpha: f64) {
            let _ = (self.target_id, alpha, self.started, self.snapshot_ratio);
        }
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
            let alpha = eased_alpha(self, parent_alpha);
            with_vsobject(mobjects, self.target_id, |v| {
                v.set_point_ratio(alpha * self.snapshot_ratio);
            });
        }
        async fn get_all_mobjects(&self) -> Vec<u64> {
            vec![self.target_id]
        }
        async fn is_introducer(&self) -> bool {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, target_opacity: 1.0, start_opacity: 0.0, primed: false }
        }
    }

    impl Animation for FadeIn {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, alpha: f64) {
            let _ = (self.target_id, alpha * self.target_opacity);
        }
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
            vec![self.target_id]
        }
        async fn is_introducer(&self) -> bool {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, start_opacity: 1.0, primed: false }
        }
    }

    impl Animation for FadeOut {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, alpha: f64) {
            let _ = (self.target_id, 1.0 - alpha);
        }
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
            let alpha = eased_alpha(self, parent_alpha);
            with_vsobject(mobjects, self.target_id, |v| {
                if !self.primed {
                    self.start_opacity = v.opacity();
                    self.primed = true;
                }
                v.set_opacity(self.start_opacity * (1.0 - alpha));
            });
        }
        async fn get_all_mobjects(&self) -> Vec<u64> {
            vec![self.target_id]
        }
        async fn is_remover(&self) -> bool {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
        }
    }

    impl Animation for Transform {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, alpha: f64) {
            let _ = (self.target_id, alpha);
        }
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, angle: f64, run_time: f64) -> Self {
            Self { target_id, angle, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, start_transform: None }
        }
    }

    impl Animation for Rotate {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, alpha: f64) {
            let _ = (self.target_id, self.angle * alpha);
        }
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, path: CubicBez, run_time: f64) -> Self {
            Self { target_id, path, run_time, rate: crate::editor::animate::engine::rate::rate::linear }
        }

        pub async fn position_at(&self, alpha: f64) -> Point {
            cubic_point_at(self.path, alpha)
        }
    }

    impl Animation for MoveAlongPath {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {}
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, alpha: f64) {
            let _ = self.position_at(alpha);
        }
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
            let alpha = eased_alpha(self, parent_alpha);
            let point = self.position_at(alpha);
            with_vsobject(mobjects, self.target_id, |v| {
                v.move_to(point);
            });
        }
        async fn get_all_mobjects(&self) -> Vec<u64> {
            vec![self.target_id]
        }
    }

    /// 🔁️ Play animations in parallel with shared parent alpha.
    pub struct AnimationGroup {
        pub animations: Vec<Animations>,
        pub run_time: Option<f64>,
        pub rate: RateFunc,
        begun: Vec<bool>,
    }

    impl AnimationGroup {
        pub async fn new(animations: Vec<Animations>) -> Self {
            let n = animations.len();
            Self { animations, run_time: None, rate: crate::editor::animate::engine::rate::rate::linear, begun: vec![false; n] }
        }

        pub async fn with_lag_ratio(self, lag_ratio: f64) -> LaggedStart {
            LaggedStart::from_group(self, lag_ratio)
        }
    }

    impl Animation for AnimationGroup {
        async fn duration(&self) -> f64 {
            self.run_time.unwrap_or_else(|| self.animations.iter().map(|a| a.duration()).fold(0.0, f64::max))
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            for (i, a) in self.animations.iter_mut().enumerate() {
                if !self.begun[i] {
                    a.begin();
                    self.begun[i] = true;
                }
            }
        }
        async fn finish(&mut self) {
            for a in &mut self.animations {
                a.finish();
            }
        }
        async fn interpolate_mobject(&mut self, parent_alpha: f64) {
            let alpha = eased_alpha(self, parent_alpha);
            for a in &mut self.animations {
                interpolate_at(&mut HashMap::new(), a, alpha);
            }
        }
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
            let alpha = eased_alpha(self, parent_alpha);
            for a in &mut self.animations {
                interpolate_at(mobjects, a, alpha);
            }
        }
        async fn get_all_mobjects(&self) -> Vec<u64> {
            self.animations.iter().flat_map(|a| a.get_all_mobjects()).collect()
        }
    }

    /// ⏭️ Play animations sequentially with lazy child activation.
    pub struct Succession {
        pub animations: Vec<Animations>,
        pub rate: RateFunc,
        active_index: Option<usize>,
        begun: Vec<bool>,
        durations: Vec<f64>,
        total: f64,
    }

    impl Succession {
        pub async fn new(animations: Vec<Animations>) -> Self {
            let durations: Vec<f64> = animations.iter().map(|a| a.duration()).collect();
            let total = durations.iter().sum();
            let n = animations.len();
            Self { animations, rate: crate::editor::animate::engine::rate::rate::linear, active_index: None, begun: vec![false; n], durations, total }
        }

        async fn slot_bounds(&self, index: usize) -> (f64, f64) {
            if self.total <= 0.0 {
                return (0.0, 1.0);
            }
            let start: f64 = self.durations.iter().take(index).sum();
            let end = start + self.durations[index];
            (start / self.total, end / self.total)
        }
    }

    impl Animation for Succession {
        async fn duration(&self) -> f64 {
            self.total
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.active_index = None;
        }
        async fn finish(&mut self) {
            for a in &mut self.animations {
                a.finish();
            }
        }
        async fn interpolate_mobject(&mut self, parent_alpha: f64) {
            self.apply(&mut HashMap::new(), parent_alpha);
        }
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
                    interpolate_at(mobjects, a, 1.0);
                } else {
                    let (start, end) = bounds[i];
                    let child_alpha = map_child_alpha(alpha, start, end);
                    interpolate_at(mobjects, a, child_alpha);
                }
            }
        }
        async fn get_all_mobjects(&self) -> Vec<u64> {
            self.animations.iter().flat_map(|a| a.get_all_mobjects()).collect()
        }
    }

    /// 🎭️ Staggered parallel start.
    pub struct LaggedStart {
        pub group: AnimationGroup,
        pub lag_ratio: f64,
    }

    impl LaggedStart {
        pub async fn new(animations: Vec<Animations>, lag_ratio: f64) -> Self {
            Self { group: AnimationGroup::new(animations), lag_ratio: lag_ratio.clamp(0.0, 1.0) }
        }

        async fn from_group(group: AnimationGroup, lag_ratio: f64) -> Self {
            Self { group, lag_ratio }
        }

        async fn child_start(&self, index: usize, count: usize) -> f64 {
            if count <= 1 {
                return 0.0;
            }
            index as f64 / (count - 1) as f64 * self.lag_ratio
        }
    }

    impl Animation for LaggedStart {
        async fn duration(&self) -> f64 {
            let base = self.group.duration();
            let n = self.group.animations.len();
            if n <= 1 {
                base
            } else {
                base + self.lag_ratio * base
            }
        }
        async fn rate_func(&self) -> RateFunc {
            self.group.rate
        }
        async fn begin(&mut self) {
            self.group.begin();
        }
        async fn finish(&mut self) {
            self.group.finish();
        }
        async fn interpolate_mobject(&mut self, parent_alpha: f64) {
            self.apply(&mut HashMap::new(), parent_alpha);
        }
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
                    interpolate_at(mobjects, a, child_alpha);
                }
            }
        }
        async fn get_all_mobjects(&self) -> Vec<u64> {
            self.group.get_all_mobjects()
        }
    }

    /// 🗺️ Lagged start over a mapped collection.
    ///
    /// 🔀️ R11 "closed set" case: `LaggedStartMap` used to be generic over its factory closure type `F`
    /// (`impl<F> Animation for LaggedStartMap<F>`), which is incompatible with `dyn_enum_close!` — an
    /// enum variant needs ONE concrete type, and an unconstrained `F` is an open family of anonymous
    /// closure types, not a closed set. The factory has no live caller in this crate (nothing constructs
    /// a `LaggedStartMap` outside this file), so the generic is concretized away here rather than solved
    /// with a per-caller monomorphized variant: the factory is boxed behind `dyn Fn` (R1-legal — the
    /// erased type is `Fn`, a std trait, not a first-party one), matching the existing `Arc<dyn Fn(&mut
    /// Sobjects, f64) + Send + Sync>` pattern already used for the `Updater` callback in `⏱️rate`.
    pub struct LaggedStartMap {
        pub count: usize,
        pub lag_ratio: f64,
        pub factory: Box<dyn Fn(usize) -> Animations + Send>,
        pub run_time: f64,
        cache: Vec<Option<Animations>>,
        begun: Vec<bool>,
    }

    impl LaggedStartMap {
        pub async fn new(count: usize, lag_ratio: f64, run_time: f64, factory: impl Fn(usize) -> Animations + Send + 'static) -> Self {
            Self { count, lag_ratio, factory: Box::new(factory), run_time, cache: (0..count).map(|_| None).collect(), begun: vec![false; count] }
        }
    }

    impl Animation for LaggedStartMap {
        async fn duration(&self) -> f64 {
            self.run_time * (1.0 + self.lag_ratio)
        }
        async fn rate_func(&self) -> RateFunc {
            crate::editor::animate::engine::rate::rate::linear
        }
        async fn begin(&mut self) {}
        async fn finish(&mut self) {
            for a in self.cache.iter_mut().flatten() {
                a.finish();
            }
        }
        async fn interpolate_mobject(&mut self, parent_alpha: f64) {
            self.apply(&mut HashMap::new(), parent_alpha);
        }
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
                    interpolate_at(mobjects, a, child_alpha);
                }
            }
        }
        async fn get_all_mobjects(&self) -> Vec<u64> {
            self.cache.iter().flatten().flat_map(|a| a.get_all_mobjects()).collect()
        }
    }

    /// ⏸️ Hold scene time without mutation.
    pub struct Wait {
        pub run_time: f64,
    }

    impl Wait {
        pub async fn new(run_time: f64) -> Self {
            Self { run_time }
        }
    }

    impl Animation for Wait {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            crate::editor::animate::engine::rate::rate::linear
        }
        async fn begin(&mut self) {}
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn get_all_mobjects(&self) -> Vec<u64> {
            Vec::new()
        }
    }

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
        pub async fn new(target: &'a mut S, run_time: f64) -> Self {
            Self { target, run_time, rate: crate::editor::animate::engine::rate::rate::smooth }
        }

        pub async fn with_rate(mut self, rate: RateFunc) -> Self {
            self.rate = rate;
            self
        }

        pub async fn fade_in(self) -> FadeIn {
            FadeIn { target_id: self.target.id(), run_time: self.run_time, rate: self.rate, target_opacity: 1.0, start_opacity: 0.0, primed: false }
        }

        pub async fn fade_out(self) -> FadeOut {
            FadeOut { target_id: self.target.id(), run_time: self.run_time, rate: self.rate, start_opacity: 1.0, primed: false }
        }

        pub async fn create(self) -> Create {
            Create::new(self.target.id(), self.run_time).with_rate(self.rate)
        }

        pub async fn transform(self) -> Transform {
            Transform { target_id: self.target.id(), run_time: self.run_time, rate: self.rate, primed: false }
        }

        pub async fn rotate(self, angle: f64) -> Rotate {
            Rotate::new(self.target.id(), angle, self.run_time)
        }

        pub async fn shift(self, delta: Vec2) -> Shift {
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
        pub async fn new(target_id: u64, delta: Vec2, run_time: f64) -> Self {
            Self { target_id, delta, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, start_transform: None }
        }
    }

    impl Animation for Shift {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, scale_factor: 1.2, start_transform: None }
        }
    }

    impl Animation for ApplyMethod {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::there_and_back, start_transform: None }
        }
    }

    impl Animation for FocusOn {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::there_and_back, start_opacity: 1.0, primed: false }
        }
    }

    impl Animation for Blink {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::linear, primed: false }
        }
    }

    impl Animation for TracedPath {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
            let alpha = eased_alpha(self, parent_alpha);
            let target_id = self.target_id;
            if !self.primed {
                with_vsobject(mobjects, target_id, |v| v.set_point_ratio(0.0));
                self.primed = true;
            }
            with_vsobject(mobjects, target_id, |v| v.set_point_ratio(alpha));
        }
        async fn get_all_mobjects(&self) -> Vec<u64> {
            vec![self.target_id]
        }
        async fn is_introducer(&self) -> bool {
            true
        }
    }

    /// ⏩️ Remap playback speed of a nested animation.
    pub struct ChangeSpeed {
        pub animation: Animations,
        pub speed_factor: f64,
    }

    impl ChangeSpeed {
        pub async fn new(animation: Animations, speed_factor: f64) -> Self {
            Self { animation, speed_factor: speed_factor.max(1e-9) }
        }
    }

    impl Animation for ChangeSpeed {
        async fn duration(&self) -> f64 {
            self.animation.duration() / self.speed_factor
        }
        async fn rate_func(&self) -> RateFunc {
            self.animation.rate_func()
        }
        async fn begin(&mut self) {
            self.animation.begin();
        }
        async fn finish(&mut self) {
            self.animation.finish();
        }
        async fn interpolate_mobject(&mut self, alpha: f64) {
            self.animation.interpolate_mobject(alpha);
        }
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
            let remapped = (parent_alpha * self.speed_factor).clamp(0.0, 1.0);
            self.animation.apply(mobjects, remapped);
        }
        async fn get_all_mobjects(&self) -> Vec<u64> {
            self.animation.get_all_mobjects()
        }
        async fn is_introducer(&self) -> bool {
            self.animation.is_introducer()
        }
        async fn is_remover(&self) -> bool {
            self.animation.is_remover()
        }
    }

    /// 🪄️ Extension trait for `.animate()` on Sobjects.
    pub trait AnimateExt: Sobject + Sized {
        async fn animate(&mut self, run_time: f64) -> AnimateBuilder<'_, Self> {
            AnimateBuilder::new(self, run_time)
        }
    }

    impl<T: Sobject + Sized> AnimateExt for T {}

    /// 🧮️ Apply parent opacity recursively to an Sobject tree (Manim parity).
    pub async fn apply_parent_opacity_tree(root: &mut Sobjects, parent_opacity: f64) {
        root.set_parent_opacity(parent_opacity);
        let eff = root.effective_opacity();
        root.visit_children_mut(&mut |child| apply_parent_opacity_tree(child, eff));
    }

    /// 🎞️ Compile animations into a flat timeline with durations.
    pub async fn compile_animations(animations: &[Animations]) -> Vec<Duration> {
        animations.iter().map(|a| Duration::from_secs_f64(a.duration().max(0.0))).collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::editor::animate::engine::animation::animation::AnimateExt;
        use crate::editor::animate::engine::scene::sobject::VSobject;

        #[semio_framework_async_macros::async_test]
        async fn succession_lazy_activation_order() {
            let a1: Animations = Wait::new(1.0).into();
            let a2: Animations = Wait::new(1.0).into();
            let mut s = Succession::new(vec![a1, a2]);
            s.interpolate_mobject(0.25);
            s.interpolate_mobject(0.75);
            assert!(s.active_index.is_some());
        }

        #[semio_framework_async_macros::async_test]
        async fn animation_group_parallel_duration_is_max() {
            let g = AnimationGroup::new(vec![Wait::new(2.0).into(), Wait::new(5.0).into()]);
            assert!((g.duration() - 5.0).abs() < 1e-9);
        }

        #[semio_framework_async_macros::async_test]
        async fn animate_builder_reads_target_id() {
            let mut v = VSobject::new();
            let id = v.id();
            let anim = v.animate(1.0).fade_in();
            assert_eq!(anim.target_id, id);
        }
    }
}

pub mod animations_catalog {
    //! 📚️ Extended Manim-parity animation catalog wired to the Animation trait.

    use crate::editor::animate::engine::animation::animation::{eased_alpha_for, with_vsobject, Animation};
    use crate::editor::animate::engine::rate::rate::RateFunc;
    use crate::editor::animate::engine::scene::sobject::Sobject;
    use geometry::{Affine, Point, Vec2};
    use std::collections::HashMap;

    async fn scale_about_center(base: Affine, center: Point, factor: f64) -> Affine {
        let t = Affine::IDENTITY.translate((center.x(), center.y())) * Affine::IDENTITY.scale(factor) * Affine::IDENTITY.translate((-center.x(), -center.y()));
        base * t
    }

    async fn rotate_about_center(base: Affine, center: Point, angle: f64) -> Affine {
        let t = Affine::IDENTITY.translate((center.x(), center.y())) * Affine::IDENTITY.rotate(angle) * Affine::IDENTITY.translate((-center.x(), -center.y()));
        base * t
    }

    async fn lerp_point(a: Point, b: Point, t: f64) -> Point {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false, fill_opacity: 1.0 }
        }
    }

    impl Animation for DrawBorderThenFill {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
            vec![self.target_id]
        }
        async fn is_introducer(&self) -> bool {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
        }
    }

    impl Animation for FadeTransform {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
        }
    }

    impl Animation for ReplacementTransform {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {
            let _ = self.target_id;
        }
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false, start_transform: None }
        }
    }

    impl Animation for TransformFromCopy {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
            vec![self.target_id]
        }
        async fn is_introducer(&self) -> bool {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
        }
    }

    impl Animation for MoveToTarget {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
        }
    }

    impl Animation for Restore {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::there_and_back, start_opacity: 1.0, primed: false }
        }
    }

    impl Animation for Flash {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::there_and_back, start_transform: None }
        }
    }

    impl Animation for Circumscribe {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, grow_point: Point::ZERO, start_transform: None }
        }
    }

    impl Animation for GrowFromPoint {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
            vec![self.target_id]
        }
        async fn is_introducer(&self) -> bool {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, start_transform: None, start_opacity: 1.0, primed: false }
        }
    }

    impl Animation for ShrinkToCenter {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
            vec![self.target_id]
        }
        async fn is_remover(&self) -> bool {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, angle: std::f64::consts::TAU, start_transform: None }
        }
    }

    impl Animation for SpinInFromNothing {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
            vec![self.target_id]
        }
        async fn is_introducer(&self) -> bool {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, value: 0.0, start_opacity: 1.0, primed: false }
        }
    }

    impl Animation for ChangeDecimalToValue {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::there_and_back, start_transform: None }
        }
    }

    impl Animation for Broadcast {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, amplitude: 0.2, start_transform: None }
        }
    }

    impl Animation for ApplyWave {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::there_and_back, angle: 0.1, start_transform: None }
        }
    }

    impl Animation for Wiggle {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { cycle_ids: vec![target_id], run_time, rate: crate::editor::animate::engine::rate::rate::smooth, start_centers: Vec::new() }
        }
    }

    impl Animation for CyclicReplace {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_centers.clear();
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, swap_id: u64, run_time: f64) -> Self {
            Self { target_id, swap_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, a_center: None, b_center: None }
        }
    }

    impl Animation for Swap {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.a_center = None;
            self.b_center = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
        }
    }

    impl Animation for TransformMatchingShapes {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, start_transform: None }
        }
    }

    impl Animation for Homotopy {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::linear, primed: false }
        }
    }

    impl Animation for ShowPassingFlash {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, angle: std::f64::consts::TAU * 2.0, origin: Point::ZERO, start_transform: None }
        }
    }

    impl Animation for SpiralIn {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
            vec![self.target_id]
        }
        async fn is_introducer(&self) -> bool {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
        }
    }

    impl Animation for Uncreate {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
            let alpha = eased_alpha_for(self, parent_alpha);
            if !self.primed {
                with_vsobject(mobjects, self.target_id, |v| v.set_point_ratio(1.0));
                self.primed = true;
            }
            with_vsobject(mobjects, self.target_id, |v| v.set_point_ratio(1.0 - alpha));
        }
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, primed: false }
        }
    }

    impl Animation for Write {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.primed = false;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
            let alpha = eased_alpha_for(self, parent_alpha);
            if !self.primed {
                with_vsobject(mobjects, self.target_id, |v| v.set_point_ratio(0.0));
                self.primed = true;
            }
            with_vsobject(mobjects, self.target_id, |v| v.set_point_ratio(alpha));
        }
        async fn get_all_mobjects(&self) -> Vec<u64> {
            vec![self.target_id]
        }
        async fn is_introducer(&self) -> bool {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::smooth, start_transform: None }
        }
    }

    impl Animation for GrowFromCenter {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
            vec![self.target_id]
        }
        async fn is_introducer(&self) -> bool {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::there_and_back, start_transform: None }
        }
    }

    impl Animation for Indicate {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
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
        pub async fn new(target_id: u64, run_time: f64) -> Self {
            Self { target_id, run_time, rate: crate::editor::animate::engine::rate::rate::linear, angle: std::f64::consts::TAU, start_transform: None }
        }
    }

    impl Animation for Rotating {
        async fn duration(&self) -> f64 {
            self.run_time
        }
        async fn rate_func(&self) -> RateFunc {
            self.rate
        }
        async fn begin(&mut self) {
            self.start_transform = None;
        }
        async fn finish(&mut self) {}
        async fn interpolate_mobject(&mut self, _alpha: f64) {}
        async fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        async fn get_all_mobjects(&self) -> Vec<u64> {
            vec![self.target_id]
        }
    }

    // 🔀️ R11 "closed set" case — the largest family in this plugin: 43 impls, all in this crate (leaf
    // tweens, composites `AnimationGroup`/`Succession`/`LaggedStart`/`LaggedStartMap`, every builder
    // returned from `.animate()` below). `dyn_enum_close!` generates the enum + match-delegating
    // `impl Animation for Animations` (O1). Bare invocation is legal — same module as `#[dyn_enum] pub
    // trait Animation` above (dyn-enum-macro finding 1).
    dyn_enum_close! {
        pub enum Animations: Animation {
            Create(Create),
            FadeIn(FadeIn),
            FadeOut(FadeOut),
            Transform(Transform),
            Rotate(Rotate),
            MoveAlongPath(MoveAlongPath),
            AnimationGroup(AnimationGroup),
            Succession(Succession),
            LaggedStart(LaggedStart),
            LaggedStartMap(LaggedStartMap),
            Wait(Wait),
            Shift(Shift),
            ApplyMethod(ApplyMethod),
            FocusOn(FocusOn),
            Blink(Blink),
            TracedPath(TracedPath),
            ChangeSpeed(ChangeSpeed),
            DrawBorderThenFill(DrawBorderThenFill),
            FadeTransform(FadeTransform),
            ReplacementTransform(ReplacementTransform),
            TransformFromCopy(TransformFromCopy),
            MoveToTarget(MoveToTarget),
            Restore(Restore),
            Flash(Flash),
            Circumscribe(Circumscribe),
            GrowFromPoint(GrowFromPoint),
            ShrinkToCenter(ShrinkToCenter),
            SpinInFromNothing(SpinInFromNothing),
            ChangeDecimalToValue(ChangeDecimalToValue),
            Broadcast(Broadcast),
            ApplyWave(ApplyWave),
            Wiggle(Wiggle),
            CyclicReplace(CyclicReplace),
            Swap(Swap),
            TransformMatchingShapes(TransformMatchingShapes),
            Homotopy(Homotopy),
            ShowPassingFlash(ShowPassingFlash),
            SpiralIn(SpiralIn),
            Uncreate(Uncreate),
            Write(Write),
            GrowFromCenter(GrowFromCenter),
            Indicate(Indicate),
            Rotating(Rotating),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::editor::animate::engine::text::color::Color;
        use crate::editor::animate::engine::geometry::geometry::circle;
        use crate::editor::animate::engine::scene::sobject::VSobject;

        #[semio_framework_async_macros::async_test]
        async fn catalog_stubs_compile_and_apply() {
            let mut map: HashMap<u64, Sobjects> = HashMap::new();
            let v = VSobject::new();
            let id = v.id();
            map.insert(id, v.into());
            let v2 = circle(Point::new(2.0, 0.0), 0.5, Color::WHITE, None, 1.0);
            let id2 = v2.id();
            map.insert(id2, v2.into());

            let stubs: Vec<Animations> = vec![
                Uncreate::new(id, 1.0).into(),
                Write::new(id, 1.0).into(),
                DrawBorderThenFill::new(id, 1.0).into(),
                FadeTransform::new(id, 1.0).into(),
                ReplacementTransform::new(id, 1.0).into(),
                TransformFromCopy::new(id, 1.0).into(),
                MoveToTarget::new(id, 1.0).into(),
                Restore::new(id, 1.0).into(),
                Indicate::new(id, 1.0).into(),
                Flash::new(id, 1.0).into(),
                Circumscribe::new(id, 1.0).into(),
                GrowFromCenter::new(id, 1.0).into(),
                GrowFromPoint::new(id, 1.0).into(),
                ShrinkToCenter::new(id, 1.0).into(),
                SpinInFromNothing::new(id, 1.0).into(),
                ChangeDecimalToValue::new(id, 1.0).into(),
                Broadcast::new(id, 1.0).into(),
                ApplyWave::new(id, 1.0).into(),
                Wiggle::new(id, 1.0).into(),
                CyclicReplace::new(id, 1.0).into(),
                Swap::new(id, id2, 1.0).into(),
                TransformMatchingShapes::new(id, 1.0).into(),
                Homotopy::new(id, 1.0).into(),
                ShowPassingFlash::new(id, 1.0).into(),
                SpiralIn::new(id, 1.0).into(),
                Rotating::new(id, 1.0).into(),
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

        #[semio_framework_async_macros::async_test]
        async fn write_reveals_point_ratio() {
            let mut map: HashMap<u64, Sobjects> = HashMap::new();
            let v = circle(Point::ZERO, 1.0, Color::WHITE, None, 1.0);
            let id = v.id();
            map.insert(id, v.into());
            let mut write = Write::new(id, 1.0);
            write.apply(&mut map, 0.5);
            with_vsobject(&mut map, id, |v| assert!((v.point_ratio - 0.5).abs() < 1e-9));
        }
    }
}
