//! ↔️ `change-page-width` — sets a page's `width` scalar.


use semio_s_artifact_layout_layout_diff::{LayoutPagesDelta, PagePatch};
use semio_s_artifact_layout_layout_diff::LayoutPagesModification;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region ↔️ChangePageWidth
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangePageWidth {
    pub id: String,
    pub new_width: f64,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for ChangePageWidth where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::change_page_width::ChangePageWidth> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "page-width", kind: "change-page-width", record: "ChangedPageWidth" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_page_width(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_change_page_width(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change page \"{}\" width", self.id), &format!("Breite von Seite \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion ↔️ChangePageWidth

//#region ↔️ChangePageWidth
pub fn diff_change_page_width(payload: &ChangePageWidth, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if page.width == payload.new_width {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Page \"{}\" already has width {}.", payload.id, payload.new_width));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { modified: vec![LayoutPagesModification { id: payload.id.clone(), patch: PagePatch { width: Some(payload.new_width), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion ↔️ChangePageWidth

//#region ↔️ChangePageWidth
pub fn inverse_change_page_width<Op: From<crate::change_page_width::ChangePageWidth>>(payload: &ChangePageWidth, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.pages.iter().find(|page| page.id == payload.id) {
        Some(page) => vec![Op::from(ChangePageWidth { id: payload.id.clone(), new_width: page.width })],
        None => Vec::new(),
    }

    })())
}
//#endregion ↔️ChangePageWidth

