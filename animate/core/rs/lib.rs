//! 🎬 Manim-class animation core: Scene timeline, Sobject graph, and frame snapshots.

pub mod config;
pub mod frame;
pub mod hash;
pub mod scene;
pub mod sobject;
pub mod timeline;

pub use config::{AnimateConfig, OutputFormat, QualityPreset};
pub use frame::FrameSnapshot;
pub use hash::{animation_hash, frame_hash};
pub use scene::{Scene, SceneContext, SceneRunner};
pub use sobject::{Mobility, MobjectStore, PaintStyle, Sobject, SobjectId, SobjectShape, StrokeStyle};
pub use timeline::SceneTimeline;
