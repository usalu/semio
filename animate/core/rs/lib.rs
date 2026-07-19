//! 🎬 Manim-class animation core: Sobject scene graph, imperative timeline, and animation composites.

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

#[path = "src/animation.rs"]
mod animation;
#[path = "src/animations_catalog.rs"]
mod animations_catalog;
#[path = "src/axes.rs"]
mod axes;
#[path = "src/camera.rs"]
mod camera;
#[path = "src/color.rs"]
mod color;
#[path = "src/config.rs"]
mod config;
#[path = "src/geometry.rs"]
mod geometry;
#[path = "src/graph.rs"]
mod graph;
#[path = "src/hash.rs"]
mod hash;
#[path = "src/matrix.rs"]
mod matrix;
#[path = "src/rate.rs"]
mod rate;
#[path = "src/scene.rs"]
mod scene;
#[path = "src/section.rs"]
mod section;
#[path = "src/sobject.rs"]
mod sobject;
#[path = "src/text.rs"]
mod text;
#[path = "src/three_d.rs"]
mod three_d;
#[path = "src/updater.rs"]
mod updater;

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
