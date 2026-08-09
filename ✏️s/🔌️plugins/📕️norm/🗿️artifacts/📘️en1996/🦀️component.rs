//! 🧱️ EN 1996 app — document entities (constitutional: general).

use crate::document::{AnnexChoice, DesignSituation};
use serde::{Deserialize, Serialize};

//#region 🔖️Types
/// 🧱️ Masonry manufacturing-control class underlying the EN-recommended γ_M table (EN 1996-1-1 Table 2.1-style).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum MasonryClass {
    Class1,
    Class2,
    #[default]
    Class3,
    Class4,
    Class5,
}

impl MasonryClass {
    pub fn gamma_m_en(self) -> f64 {
        match self {
            Self::Class1 => 1.5,
            Self::Class2 => 1.7,
            Self::Class3 => 2.0,
            Self::Class4 => 2.2,
            Self::Class5 => 2.5,
        }
    }
}

pub mod part_2 {
    use serde::{Deserialize, Serialize};

    /// 🌦️ Masonry durability exposure class (EN 1996-1-1 Annex B-style categorisation MX1–MX5).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum ExposureClass {
        Mx1,
        Mx2,
        Mx3,
        Mx4,
        Mx5,
    }

    /// 🧪️ General-purpose mortar compressive-strength class per EN 998-2.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum MortarClass {
        M1,
        /// 🔡️ `M2_5` auto-kebabs to `m2-5` (digit-underscore-digit), but the standard's own class
        /// label is `M2.5`/`M2_5` with no internal dash — kept as a genuine rename.
        #[dsl(key = "m2_5")]
        M2_5,
        M5,
        M10,
        M20,
    }

    impl MortarClass {
        pub fn compressive_strength_mpa(self) -> f64 {
            match self {
                Self::M1 => 1.0,
                Self::M2_5 => 2.5,
                Self::M5 => 5.0,
                Self::M10 => 10.0,
                Self::M20 => 20.0,
            }
        }
    }
}


pub use crate::artifacts::en1996::snapshot::schema::En1996Snapshot;


pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1996", "EN 1996")
}
