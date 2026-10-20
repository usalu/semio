//! 🏛️ `update-page-columns` — atomically sets a page's column count and gutter together.


use semio_s_artifact_layout_layout_diff::{LayoutPagesDelta, PagePatch};
use semio_s_artifact_layout_layout_diff::LayoutPagesModification;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🏛️UpdatePageColumns
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UpdatePageColumns {
    pub id: String,
    pub count: u32,
    pub gutter: f64,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for UpdatePageColumns where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::update_page_columns::UpdatePageColumns> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "page-columns", kind: "update-page-columns", record: "UpdatedPageColumns" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_update_page_columns(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse_update_page_columns(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Update page \"{}\" columns", self.id), &format!("Spalten von Seite \"{}\" aktualisieren", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🏛️UpdatePageColumns

//#region 🏛️UpdatePageColumns
pub fn diff_update_page_columns(payload: &UpdatePageColumns, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if page.columns.count == payload.count && page.columns.gutter == payload.gutter {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Page \"{}\" already has those columns.", payload.id));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { modified: vec![LayoutPagesModification { id: payload.id.clone(), patch: PagePatch { columns_count: Some(payload.count), columns_gutter: Some(payload.gutter), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🏛️UpdatePageColumns

//#region 🏛️UpdatePageColumns
pub fn inverse_update_page_columns<Op: From<crate::update_page_columns::UpdatePageColumns>>(payload: &UpdatePageColumns, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.pages.iter().find(|page| page.id == payload.id) {
        Some(page) => vec![Op::from(UpdatePageColumns { id: payload.id.clone(), count: page.columns.count, gutter: page.columns.gutter })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🏛️UpdatePageColumns

