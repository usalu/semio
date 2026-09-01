//! 🔺️ `create-drawing` — sparse diff construction, built directly from `(payload, base)`.

use super::CreateDrawing;
use crate::artifacts::cad::diff::{CadDiff, CadDrawingChildList};
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub(crate) fn parse_target(uri: &str) -> store::os_io::ArtifactRef {
    store::os_io::ArtifactRef::parse_uri(uri).unwrap_or_else(|_| store::os_io::ArtifactRef { artifact_id: uri.to_string(), dialect: store::os_io::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } })
}

pub fn diff(payload: &CreateDrawing, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if base.drawings.iter().any(|drawing| drawing.child_id == payload.child_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A drawing with id \"{}\" already exists.", payload.child_id), [payload.child_id.clone()]);
    }
    let mut drawings = base.drawings.clone();
    drawings.push(store::ArtifactChild::new(payload.child_id.clone(), parse_target(&payload.target)));
    protocol::MutationOutcome::new(CadDiff { drawings: Some(CadDrawingChildList { values: drawings }), ..Default::default() })
}
//#endregion 🔖️Diff
