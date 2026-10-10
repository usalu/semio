//! 📏️ `authored`: the pure functions over AUTHORED data that mutations and inferences share. A diff may read the snapshot only through its own accessors and through this module; it never runs, imports or
//! reproduces an inference. Everything here is total, reads nothing but the arguments and evaluates no model graph: formula text, plan segments, storey stacking, opening sizes, attach references and profile outlines.

#[path = "📝️formula/🦀️.rs"]
pub mod formula;
#[path = "🧭️plan/🦀️.rs"]
pub mod plan;
#[path = "▭️profile/🦀️.rs"]
pub mod profile;
#[path = "🔗️references/🦀️.rs"]
pub mod references;
#[path = "🪟️sizes/🦀️.rs"]
pub mod sizes;
#[path = "🪜️storeys/🦀️.rs"]
pub mod storeys;
