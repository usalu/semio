//! ↕️ `change-page-height` — sets a page's `height` scalar.


use semio_s_artifact_layout_layout_diff::{LayoutPagesDelta, PagePatch};
use semio_s_artifact_layout_layout_diff::LayoutPagesModification;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region ↕️ChangePageHeight
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangePageHeight {
    pub id: String,
    pub new_height: f64,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for ChangePageHeight where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::change_page_height::ChangePageHeight> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "page-height", kind: "change-page-height", record: "ChangedPageHeight" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_page_height(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_change_page_height(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change page \"{}\" height", self.id), &format!("Höhe von Seite \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion ↕️ChangePageHeight

//#region ↕️ChangePageHeight
pub fn diff_change_page_height(payload: &ChangePageHeight, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if page.height == payload.new_height {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Page \"{}\" already has height {}.", payload.id, payload.new_height));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { modified: vec![LayoutPagesModification { id: payload.id.clone(), patch: PagePatch { height: Some(payload.new_height), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion ↕️ChangePageHeight

//#region ↕️ChangePageHeight
pub fn inverse_change_page_height<Op: From<crate::change_page_height::ChangePageHeight>>(payload: &ChangePageHeight, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.pages.iter().find(|page| page.id == payload.id) {
        Some(page) => vec![Op::from(ChangePageHeight { id: payload.id.clone(), new_height: page.height })],
        None => Vec::new(),
    }

    })())
}
//#endregion ↕️ChangePageHeight

