//! 🔺️ `create-building-model` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::CreateBuildingModel;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
/// 🔗️ Parses a wire URI into a real `ArtifactRef`, degrading to an empty (invalid, harmlessly
/// unresolvable) ref on malformed input rather than panicking — `MutationKind::diff` is infallible.
pub(crate) fn parse_target(uri: &str) -> store::os_io::ArtifactRef {
    store::os_io::ArtifactRef::parse_uri(uri).unwrap_or_else(|_| store::os_io::ArtifactRef { artifact_id: uri.to_string(), dialect: store::os_io::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } })
}

pub fn diff(payload: &CreateBuildingModel, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let candidate = store::ArtifactChild::new(payload.child_id.clone(), parse_target(&payload.target));
    if base.building_model.as_ref() == Some(&candidate) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Building-model child is already {}.", payload.child_id));
    }
    protocol::MutationOutcome::new(CadDiff { building_model: Some(Some(candidate)), ..Default::default() })
}
//#endregion 🔖️Diff
