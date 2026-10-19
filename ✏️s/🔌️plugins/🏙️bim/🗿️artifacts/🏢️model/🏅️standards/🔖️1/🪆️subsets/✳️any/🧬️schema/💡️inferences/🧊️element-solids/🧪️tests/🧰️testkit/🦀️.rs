//! 🧰️ Committed element-solid fixture cases for the unit laws and the testkit consumers.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const STRAIGHT_OPENINGS: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🧊️element-solids/🚪️straight-openings/🔣️.json");
const ARC_WINDOW: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🧊️element-solids/🌀️arc-window/🔣️.json");
const ROOM_JOINS: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🧊️element-solids/🧩️room-joins/🔣️.json");
const CURTAIN_GRID: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🧊️element-solids/🏬️curtain-grid/🔣️.json");
const CEILINGS_MESHES: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🧊️element-solids/🪵️ceilings-meshes/🔣️.json");
const FRAME_TILT_JOINS: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🧊️element-solids/📐️frame-tilt-joins/🔣️.json");
const CURTAIN_OVERRIDES: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🧊️element-solids/🪟️curtain-overrides/🔣️.json");
const COMPONENTS_MEP: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/💡️inferences/🧊️element-solids/🪑️components-mep/🔣️.json");

/// 🧫️ Every case name with its committed text.
pub const CASES: [(&str, &str); 8] = [("straight-openings", STRAIGHT_OPENINGS), ("arc-window", ARC_WINDOW), ("room-joins", ROOM_JOINS), ("curtain-grid", CURTAIN_GRID), ("ceilings-meshes", CEILINGS_MESHES), ("frame-tilt-joins", FRAME_TILT_JOINS), ("curtain-overrides", CURTAIN_OVERRIDES), ("components-mep", COMPONENTS_MEP)];

/// 🧫️ The decoded snapshot and the whole committed document of a case.
pub fn case(name: &str) -> (ModelSnapshot, serde_json::Value) {
    let text = CASES.iter().find(|(case, _)| *case == name).map(|(_, text)| *text).expect("a committed case");
    let document: serde_json::Value = serde_json::from_str(text).expect("the case is JSON");
    let snapshot = from_json_str(&document["snapshot"].to_string(), JsonMemberPolicy::Reject).expect("the case snapshot decodes");
    (snapshot, document)
}
