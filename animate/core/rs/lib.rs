//! 🎬 Manim-class animation core: Sobject scene graph, imperative timeline, and animation composites.

#![allow(clippy::too_many_arguments, clippy::type_complexity)]

#[path = "src/config.rs"]
mod config;
#[path = "src/rate.rs"]
mod rate;
#[path = "src/color.rs"]
mod color;
#[path = "src/sobject.rs"]
mod sobject;
#[path = "src/geometry.rs"]
mod geometry;
#[path = "src/text.rs"]
mod text;
#[path = "src/axes.rs"]
mod axes;
#[path = "src/graph.rs"]
mod graph;
#[path = "src/matrix.rs"]
mod matrix;
#[path = "src/three_d.rs"]
mod three_d;
#[path = "src/animation.rs"]
mod animation;
#[path = "src/animations_catalog.rs"]
mod animations_catalog;
#[path = "src/updater.rs"]
mod updater;
#[path = "src/camera.rs"]
mod camera;
#[path = "src/scene.rs"]
mod scene;
#[path = "src/hash.rs"]
mod hash;
#[path = "src/section.rs"]
mod section;

pub use config::*;
pub use rate::*;
pub use color::*;
pub use sobject::*;
pub use geometry::*;
pub use text::*;
pub use axes::*;
pub use graph::*;
pub use matrix::*;
pub use three_d::*;
pub use animation::*;
pub use animations_catalog::*;
pub use updater::*;
pub use camera::*;
pub use scene::*;
pub use hash::*;
pub use section::*;

pub use mathematical_geometry::{Affine, BezPath, Circle as GeoCircle, Point as GeoPoint, Rect as GeoRect, Vec2 as GeoVec2};
