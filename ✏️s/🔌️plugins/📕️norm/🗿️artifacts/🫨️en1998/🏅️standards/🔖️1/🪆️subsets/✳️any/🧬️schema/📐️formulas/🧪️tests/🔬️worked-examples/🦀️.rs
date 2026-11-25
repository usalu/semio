//! 🔬️ EN 1998 formula worked examples: the hand-recomputed examples of every kernel module and the design-case catalogue, run against the public API of the formulas crate.

#![cfg(test)]

pub use semio_s_artifact_norm_en1998_schema_formulas::*;

#[path = "."]
mod assessment_state {
    pub use semio_s_artifact_norm_en1998_schema_formulas::part_3_state::*;
    use semio_s_artifact_norm_en1998_model::{En1998Assessment, KnowledgeLevel};
    use semio_s_artifact_norm_en1998_schema_formulas::part_3::{self, AnnexA, RcMember};
    use semio_s_plugin_norm_compliance::{ndp_number, AnnexChoice};

    #[path = "../../🏚️assessment-state/🧪️tests/🦀️.rs"]
    mod tests;
}

#[path = "."]
mod soil_structure {
    pub use semio_s_artifact_norm_en1998_schema_formulas::soil_structure::*;
    use semio_s_plugin_norm_compliance::{ndp_interpolate, AnnexChoice};

    #[path = "../../🪨️soil-structure/🧪️tests/🦀️.rs"]
    mod tests;
}

#[path = "."]
mod critical_region {
    pub use semio_s_artifact_norm_en1998_schema_formulas::critical_region::*;
    use semio_s_artifact_norm_en1998_model::{DuctilityClass, En1998CriticalRegion, En1998System, MemberKind};
    use semio_s_artifact_norm_en1998_schema_formulas::part_1;
    use semio_s_plugin_norm_compliance::{ndp_number, AnnexChoice};

    #[path = "../../🔩️critical-region/🧪️tests/🦀️.rs"]
    mod tests;
}

#[path = "."]
mod isolation {
    pub use semio_s_artifact_norm_en1998_schema_formulas::isolation::*;
    use semio_s_artifact_norm_en1998_schema_formulas::part_1;

    #[path = "../../🛞️isolation/🧪️tests/🦀️.rs"]
    mod tests;
}

#[path = "."]
mod liquefaction_hazard {
    pub use semio_s_artifact_norm_en1998_schema_formulas::liquefaction_hazard::*;
    use semio_s_artifact_norm_en1998_schema_formulas::part_5::SoilProfile;

    #[path = "../../🫧️liquefaction-hazard/🧪️tests/🦀️.rs"]
    mod tests;
}

#[path = "."]
mod case_catalogue {
    pub use semio_s_artifact_norm_en1998_schema_formulas::case_catalogue::*;
    use semio_s_plugin_norm_compliance::design_case::{self, CaseCatalogue, CaseEntry, CaseRow, DesignCase, InputSpec};
    use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, SubjectRef};

    #[path = "../../📇️catalogue/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
}
