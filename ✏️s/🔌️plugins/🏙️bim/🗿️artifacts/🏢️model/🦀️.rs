//! 🏢️ BIM model artifact: building information as a parametric, event-sourced document.
//!
//! `ModelSnapshot` stores authored parameters only; every derived value (elevations, wall heights, quantities, solids) is a field of
//! `ModelInference`. Mutations are sparse typed diffs with concrete inverses (`ModelMutation`, `ModelDiff`). The artifact is carved into
//! component crates; this crate only exports the artifact identity and the declaration tree, and every consumer imports the crate that owns what it uses.

pub use semio_s_artifact_bim_model_v1::{BimApplication, artifact, definition};

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
