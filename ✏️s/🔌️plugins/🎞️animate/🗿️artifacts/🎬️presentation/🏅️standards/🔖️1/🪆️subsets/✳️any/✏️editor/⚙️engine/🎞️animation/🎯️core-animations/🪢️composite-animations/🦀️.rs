//! 🪢️ Composite animations of the core animation set: groups, successions, lagged starts and the wait. See [`super`].

use super::*;

/// 🔁️ Play animations in parallel with shared parent alpha.
pub struct AnimationGroup {
    pub animations: Vec<Animations>,
    pub run_time: Option<f64>,
    pub rate: RateFunc,
    begun: Vec<bool>,
}

impl AnimationGroup {
    pub fn new(animations: Vec<Animations>) -> Self {
        let n = animations.len();
        Self { animations, run_time: None, rate: crate::editor::animate::engine::rate::rate::linear, begun: vec![false; n] }
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
            interpolate_at(&mut HashMap::new(), a, alpha);
        }
    }
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
        let alpha = eased_alpha(self, parent_alpha);
        for a in &mut self.animations {
            interpolate_at(mobjects, a, alpha);
        }
    }
    fn get_all_mobjects(&self) -> Vec<u64> {
        self.animations.iter().flat_map(|a| a.get_all_mobjects()).collect()
    }
}

/// ⏭️ Play animations sequentially with lazy child activation.
pub struct Succession {
    pub animations: Vec<Animations>,
    pub rate: RateFunc,
    pub(super) active_index: Option<usize>,
    begun: Vec<bool>,
    durations: Vec<f64>,
    total: f64,
}

impl Succession {
    pub fn new(animations: Vec<Animations>) -> Self {
        let durations: Vec<f64> = animations.iter().map(|a| a.duration()).collect();
        let total = durations.iter().sum();
        let n = animations.len();
        Self { animations, rate: crate::editor::animate::engine::rate::rate::linear, active_index: None, begun: vec![false; n], durations, total }
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
    pub fn new(animations: Vec<Animations>, lag_ratio: f64) -> Self {
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
    fn get_all_mobjects(&self) -> Vec<u64> {
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
    pub fn new(count: usize, lag_ratio: f64, run_time: f64, factory: impl Fn(usize) -> Animations + Send + 'static) -> Self {
        Self { count, lag_ratio, factory: Box::new(factory), run_time, cache: (0..count).map(|_| None).collect(), begun: vec![false; count] }
    }
}

impl Animation for LaggedStartMap {
    fn duration(&self) -> f64 {
        self.run_time * (1.0 + self.lag_ratio)
    }
    fn rate_func(&self) -> RateFunc {
        crate::editor::animate::engine::rate::rate::linear
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
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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
        crate::editor::animate::engine::rate::rate::linear
    }
    fn begin(&mut self) {}
    fn finish(&mut self) {}
    fn interpolate_mobject(&mut self, _alpha: f64) {}
    fn get_all_mobjects(&self) -> Vec<u64> {
        Vec::new()
    }
}
