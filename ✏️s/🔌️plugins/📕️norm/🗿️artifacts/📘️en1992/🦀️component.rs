//! En1992 — document entities (constitutional: general).

use crate::document::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
pub mod part_1_2 {
    use super::*;

    /// 🏗️ Fire resistance rating.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum FireRating {
        R30,
        R60,
        R90,
        R120,
    }
}
pub mod part_3 {
    use super::*;

    /// 💧️ Tightness class per EN 1992-3 Table 7.105: required degree of protection against leakage.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum TightnessClass {
        Tc0,
        Tc1,
        Tc2,
    }
}

/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.
pub use crate::artifacts::en1992::snapshot::schema::En1992Snapshot;
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1992", "EN 1992")
}
//#endregion 🔖️ArtifactKind
