//! ➖️ `delete-frame` — removes a {@link Frame} from a page by id (and every layer's `object_ids`
//! referencing it); inverse recreates it via `create-frame`.

use crate::mutations::{create_frame, LayoutMutation};
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagesDelta, LayoutPagesModification, PageFramesDelta, PagePatch};
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region ➖️DeleteFrame
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteFrame {
    pub page_id: String,
    pub frame_id: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for DeleteFrame {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "frame", kind: "delete-frame", record: "DeletedFrame" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_delete_frame(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse_delete_frame(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete frame \"{}\"", self.frame_id), &format!("Rahmen \"{}\" löschen", self.frame_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame_id.clone()]
    }
}
//#endregion ➖️DeleteFrame

//#region ➖️DeleteFrame
pub fn diff_delete_frame(payload: &DeleteFrame, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    let Some(at) = page.frames.iter().position(|frame| frame.id() == payload.frame_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame \"{}\" does not exist on page \"{}\".", payload.frame_id, payload.page_id), [payload.frame_id.clone()]);
    };
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { modified: vec![LayoutPagesModification { id: payload.page_id.clone(), patch: PagePatch { frames: PageFramesDelta::removal_by_id(payload.frame_id.clone(), at), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion ➖️DeleteFrame

//#region ➖️DeleteFrame
pub fn inverse_delete_frame(payload: &DeleteFrame, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return Vec::new();
    };
    let Some(index) = page.frames.iter().position(|frame| frame.id() == payload.frame_id) else {
        return Vec::new();
    };
    let frame = page.frames[index].clone();
    let layer_id = page.layers.iter().find(|layer| layer.object_ids.iter().any(|id| id == &payload.frame_id)).map(|layer| layer.id.clone());
    vec![LayoutMutation::CreateFrame(create_frame::CreateFrame { page_id: payload.page_id.clone(), frame, index: Some(index), layer_id })]

    })())
}
//#endregion ➖️DeleteFrame
