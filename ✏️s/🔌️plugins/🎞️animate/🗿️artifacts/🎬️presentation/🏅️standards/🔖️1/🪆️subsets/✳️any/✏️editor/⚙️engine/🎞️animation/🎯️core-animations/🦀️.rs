//! 🎯️ Core animations of the animate engine: the animation trait with its enum dispatch and the leaf, composite, builder and effect animations. See [`super`].
//!
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
    fn duration(&self) -> f64;
    fn rate_func(&self) -> RateFunc;
    fn begin(&mut self);
    fn finish(&mut self);
    fn interpolate_mobject(&mut self, alpha: f64);
    fn apply(&mut self, mobjects: &mut HashMap<u64, Sobjects>, parent_alpha: f64) {
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

fn eased_alpha<A: Animation + ?Sized>(animation: &A, alpha: f64) -> f64 {
    (animation.rate_func())(alpha.clamp(0.0, 1.0))
}

pub(crate) fn eased_alpha_for<A: Animation + ?Sized>(animation: &A, alpha: f64) -> f64 {
    eased_alpha(animation, alpha)
}

/// 🎯️ Resolve a VSobject by id and run a closure on it.
pub fn with_vsobject<F>(mobjects: &mut HashMap<u64, Sobjects>, id: u64, f: F)
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
pub fn interpolate_at(mobjects: &mut HashMap<u64, Sobjects>, animation: &mut Animations, parent_alpha: f64) {
    animation.apply(mobjects, parent_alpha);
}

#[path = "🍃️leaf-animations/🦀️.rs"]
mod leaf_animations;
pub use leaf_animations::*;

#[path = "🪢️composite-animations/🦀️.rs"]
mod composite_animations;
pub use composite_animations::*;

#[path = "🪄️builder-effects/🦀️.rs"]
mod builder_and_effect_animations;
pub use builder_and_effect_animations::*;

/// 🪄️ Extension trait for `.animate()` on Sobjects.
pub trait AnimateExt: Sobject + Sized {
    fn animate(&mut self, run_time: f64) -> AnimateBuilder<'_, Self> {
        AnimateBuilder::new(self, run_time)
    }
}

impl<T: Sobject + Sized> AnimateExt for T {}

/// 🧮️ Apply parent opacity recursively to an Sobject tree (Manim parity).
pub fn apply_parent_opacity_tree(root: &mut Sobjects, parent_opacity: f64) {
    root.set_parent_opacity(parent_opacity);
    let eff = root.effective_opacity();
    root.visit_children_mut(&mut |child| apply_parent_opacity_tree(child, eff));
}

//#region 🔖️Animations
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
        DrawBorderThenFill(super::animations_catalog::DrawBorderThenFill),
        FadeTransform(super::animations_catalog::FadeTransform),
        ReplacementTransform(super::animations_catalog::ReplacementTransform),
        TransformFromCopy(super::animations_catalog::TransformFromCopy),
        MoveToTarget(super::animations_catalog::MoveToTarget),
        Restore(super::animations_catalog::Restore),
        Flash(super::animations_catalog::Flash),
        Circumscribe(super::animations_catalog::Circumscribe),
        GrowFromPoint(super::animations_catalog::GrowFromPoint),
        ShrinkToCenter(super::animations_catalog::ShrinkToCenter),
        SpinInFromNothing(super::animations_catalog::SpinInFromNothing),
        ChangeDecimalToValue(super::animations_catalog::ChangeDecimalToValue),
        Broadcast(super::animations_catalog::Broadcast),
        ApplyWave(super::animations_catalog::ApplyWave),
        Wiggle(super::animations_catalog::Wiggle),
        CyclicReplace(super::animations_catalog::CyclicReplace),
        Swap(super::animations_catalog::Swap),
        TransformMatchingShapes(super::animations_catalog::TransformMatchingShapes),
        Homotopy(super::animations_catalog::Homotopy),
        ShowPassingFlash(super::animations_catalog::ShowPassingFlash),
        SpiralIn(super::animations_catalog::SpiralIn),
        Uncreate(super::animations_catalog::Uncreate),
        Write(super::animations_catalog::Write),
        GrowFromCenter(super::animations_catalog::GrowFromCenter),
        Indicate(super::animations_catalog::Indicate),
        Rotating(super::animations_catalog::Rotating),
    }
}
//#endregion 🔖️Animations

/// 🎞️ Compile animations into a flat timeline with durations.
pub fn compile_animations(animations: &[Animations]) -> Vec<Duration> {
    animations.iter().map(|a| Duration::from_secs_f64(a.duration().max(0.0))).collect()
}

#[cfg(test)]
include!("../🧪️tests/🔬️animation-unit/🦀️.rs");
