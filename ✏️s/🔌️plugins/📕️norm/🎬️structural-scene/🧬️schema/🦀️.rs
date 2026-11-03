//! 🧬️ Structural scene schema: every module mounted here is re-exported flat for clients.

#[macro_use]
#[path = "🪄️macros/🦀️.rs"]
mod macros;

#[path = "✅️validation/🦀️.rs"]
mod validation;
pub use validation::{Validate, Violation};
pub use validation::*;

#[path = "🧾️canonical-json/🦀️.rs"]
mod canonical_json_writer;
pub use canonical_json_writer::canonical_json;

#[path = "📐️geometry/🦀️.rs"]
mod geometry;
pub use geometry::*;

#[path = "🎨️appearance/🦀️.rs"]
mod appearance;
pub use appearance::*;

#[path = "🧱️section-shapes/🦀️.rs"]
mod section_shapes;
pub use section_shapes::*;

#[path = "🏹️load-glyphs/🦀️.rs"]
mod load_glyphs;
pub use load_glyphs::*;

#[path = "🧩️entities/🦀️.rs"]
mod entities;
pub use entities::*;

#[path = "🏷️annotations/🦀️.rs"]
mod annotations;
pub use annotations::*;

#[path = "🔺️scene-diff/🦀️.rs"]
mod scene_diff;
pub use scene_diff::*;

#[path = "📈️diagram2d/🦀️.rs"]
mod diagram2d;
pub use diagram2d::*;

#[path = "🕹️handles/🦀️.rs"]
mod handles;
pub use handles::*;
