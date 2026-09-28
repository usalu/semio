//! 🧪️ Laws of `import-document`. The framework hands the whole picked file
//! (`semio_framework::kernel::ImportStaging`), so the one size rule this command owns is one Artifact-lane
//! edit, refused by name before anything is decoded.

use super::{admit_payload, GENERATION3D_IMPORT_CAPACITY_CODE, GENERATION3D_IMPORT_TOTAL_BYTES};

/// ⚖️ LAW: a payload at the budget reaches the decoder, one byte more is refused with the capacity code the
/// language-agnostic surface fixture names (`🧫️fixtures/🚪️io/🗿️artifact-surface.json` `faultCodes.capacity`).
#[test]
fn an_import_above_one_artifact_lane_edit_is_refused_by_name() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🚪️io/🗿️artifact-surface.json")).expect("the artifact-surface fixture is valid json");
    assert_eq!(fixture["faultCodes"]["capacity"].as_str(), Some(GENERATION3D_IMPORT_CAPACITY_CODE), "the refusal code is the fixture's");
    assert!(admit_payload(&"x".repeat(GENERATION3D_IMPORT_TOTAL_BYTES)).is_ok(), "a payload at the budget is admitted");
    let refused = admit_payload(&"x".repeat(GENERATION3D_IMPORT_TOTAL_BYTES + 1)).expect_err("one byte above the budget is refused");
    assert_eq!(refused.code.0, GENERATION3D_IMPORT_CAPACITY_CODE);
}
