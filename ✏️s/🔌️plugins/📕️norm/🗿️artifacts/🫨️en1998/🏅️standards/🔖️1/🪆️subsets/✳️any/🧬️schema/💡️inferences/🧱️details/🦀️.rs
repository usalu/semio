//! 🧱️ EN 1998 inference details: the quantity constructors and annex resolution, the part details (towers, chord rotation, piers, members, N2) and the geotechnical details (liquefaction, bearing, piles, soil profile). See [`super`].

#![allow(async_fn_in_trait)]

use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use semio_s_artifact_norm_en1998_schema::{cracked_stiffness, na_de, part_1, part_2, part_3, part_5, part_6, AnnexParams};

//#region 🪢️Mounts
#[path = "../🧰️seismic-helpers/🦀️.rs"]
pub mod seismic_helpers;
#[path = "../🪨️part-details/🦀️.rs"]
pub mod part_details;
#[path = "../🌊️geotechnical-details/🦀️.rs"]
pub mod geotechnical_details;
#[path = "../🏚️assessment-checks/🦀️.rs"]
pub mod assessment_checks;
#[path = "../🫧️liquefaction-checks/🦀️.rs"]
pub mod liquefaction_checks;
//#endregion 🪢️Mounts

use seismic_helpers::*;
