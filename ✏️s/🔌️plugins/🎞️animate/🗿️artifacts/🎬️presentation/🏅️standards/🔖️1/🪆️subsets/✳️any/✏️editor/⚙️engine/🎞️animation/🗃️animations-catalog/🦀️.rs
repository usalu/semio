//! 🗃️ Extended animation catalog of the animate engine: the transform, grow, wave, swap and write animations beyond the core set. See [`super`].
//!
//! 📚️ Extended Manim-parity animation catalog wired to the Animation trait.

use crate::editor::animate::engine::animation::animation::{eased_alpha_for, with_vsobject, Animation};
use crate::editor::animate::engine::rate::rate::RateFunc;
use crate::editor::animate::engine::scene::sobject::{Sobject, Sobjects};
use geometry::{Affine, Point, Vec2};
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

#[path = "🎆️transform-highlight/🦀️.rs"]
mod transform_and_highlight_animations;
pub use transform_and_highlight_animations::*;

#[path = "🌊️grow-wave-cycle/🦀️.rs"]
mod grow_wave_and_cycle_animations;
pub use grow_wave_and_cycle_animations::*;

#[path = "✍️swap-flash-write/🦀️.rs"]
mod swap_flash_and_write_animations;
pub use swap_flash_and_write_animations::*;

#[cfg(test)]
include!("../🧪️tests/🔬️animations-catalog-unit/🦀️.rs");
