//! 🎞️ Scene-based presentation documents and static site compiler.

pub mod compiler;
pub mod slide;

pub use compiler::{compile_present_site, PresentCompileError};
pub use slide::{PresentScene, PresentSection, PresentSlide, PRESENT_SCENE_SCHEMA};
