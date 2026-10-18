//! 📏️ `authored`: the pure functions over AUTHORED data that mutations and inferences share. A diff may read the snapshot only through its own accessors and through this module; it never runs, imports or
//! reproduces an inference. Everything here is total, reads nothing but the arguments and evaluates no model graph: formula text, plan segments, storey stacking, opening sizes, attach references and profile outlines.

pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::formula;
pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::authored::option_rules;
pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::plan;
pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::profile;
#[path = "🔗️references/🦀️.rs"]
pub mod references;
#[path = "🪟️sizes/🦀️.rs"]
pub mod sizes;
#[path = "🪜️storeys/🦀️.rs"]
pub mod storeys;
