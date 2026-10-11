//! ➕️ `create-frame` — inserts a new {@link Frame} into a page's `frames` list (paint-order
//! significant), optionally registering it on one of the page's layers.

use crate::mutations::{delete_frame, LayoutMutation};
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagesDelta, LayoutPagesModification, PageFramesDelta, PagePatch};
use crate::{Frame, LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region ➕️CreateFrame
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateFrame {
    pub page_id: String,
    pub frame: Frame,
    pub index: Option<usize>,
    pub layer_id: Option<String>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for CreateFrame {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "frame", kind: "create-frame", record: "CreatedFrame" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_create_frame(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse_create_frame(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create frame \"{}\"", self.frame.id()), &format!("Rahmen \"{}\" erstellen", self.frame.id()))
    }
    fn target(&self) -> Vec<String> {
        vec![self.page_id.clone(), self.frame.id().to_string()]
    }
}
//#endregion ➕️CreateFrame

//#region ➕️CreateFrame
pub fn diff_create_frame(payload: &CreateFrame, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.page_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.page_id), [payload.page_id.clone()]);
    };
    if page.frames.iter().any(|frame| frame.id() == payload.frame.id()) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A frame with id \"{}\" already exists on page \"{}\".", payload.frame.id(), payload.page_id), [payload.frame.id().to_string()]);
    }
    if payload.index.is_some_and(|at| at > page.frames.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end of {} rows.", payload.index.unwrap_or_default(), page.frames.len()), [payload.frame.id().to_string()]);
    }
    let frame = match &payload.layer_id {
        Some(layer_id) => crate::frame_in_layer(&payload.frame, layer_id),
        None => payload.frame.clone(),
    };
    let at = payload.index.unwrap_or(page.frames.len());
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { modified: vec![LayoutPagesModification { id: payload.page_id.clone(), patch: PagePatch { frames: PageFramesDelta::insertion(at, frame), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion ➕️CreateFrame

//#region ➕️CreateFrame
pub fn inverse_create_frame(payload: &CreateFrame, _base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![LayoutMutation::DeleteFrame(delete_frame::DeleteFrame { page_id: payload.page_id.clone(), frame_id: payload.frame.id().to_string() })]

    })())
}
//#endregion ➕️CreateFrame
