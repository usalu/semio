//! 🏗️ set-parent-page-content produces a structural diff and retains its semantic inverse.

use semio_s_artifact_layout_layout_diff::*;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetParentPageContent {
    pub id: String,
    pub layers: Vec<semio_s_artifact_layout_layout_model::Layer>,
    pub frames: Vec<semio_s_artifact_layout_layout_model::Frame>,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for SetParentPageContent
where Op: protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::set_parent_page_content::SetParentPageContent> + From<crate::set_page_overrides::SetPageOverrides> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "parent-page-content", kind: "set-parent-page-content", record: "SetParentPageContent" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Set parent page content", "Mustervorlageninhalt festlegen") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for SetParentPageContent
where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::set_parent_page_content::SetParentPageContent> + From<crate::set_page_overrides::SetPageOverrides> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_set_parent_page_content(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { inverse_set_parent_page_content(self, base) }
}

/// 🔺️ Resolves a semantic payload into a sparse document diff.
pub fn diff_set_parent_page_content(payload: &SetParentPageContent, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(at) = base.parent_pages.iter().position(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The parent page does not exist.", [payload.id.clone()]);
    };
    let mut page = base.parent_pages[at].clone();
    page.layers = payload.layers.clone();
    page.frames = payload.frames.clone();
    if let Err(message) = semio_s_artifact_layout_layout_model::validation::normalize_parent_content(base, &mut page) {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, [payload.id.clone()]);
    }
    if page == base.parent_pages[at] { return protocol::MutationOutcome::empty().warning("mutation.no-op", "Parent content is already set.") }
    let pages = base.pages.iter().filter(|held| held.parent_page_id.as_deref() == Some(payload.id.as_str())).filter_map(|held| {
        let overrides: Vec<_> = held.overrides.iter().filter(|entry| page.frames.iter().any(|frame| frame.id() == entry.object_id)).cloned().collect();
        (overrides != held.overrides).then(|| LayoutPagesModification { id: held.id.clone(), patch: PagePatch { overrides: page_override_rows(&held.overrides, &overrides), ..Default::default() } })
    }).collect::<Vec<_>>();
    protocol::MutationOutcome::new(LayoutDiff { pages: (!pages.is_empty()).then_some(LayoutPagesDelta { modified: pages, ..Default::default() }), parent_pages: Some(LayoutParentPagesDelta { removed: vec![LayoutParentPageRemoval { id: payload.id.clone(), index: at }], inserted: vec![LayoutParentPageInsertion { index: at, row: page }], ..Default::default() }), ..Default::default() })
}

/// ↩️ Restores every affected field and its original position.
pub fn inverse_set_parent_page_content<Op: From<crate::set_parent_page_content::SetParentPageContent> + From<crate::set_page_overrides::SetPageOverrides>>(payload: &SetParentPageContent, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if diff_set_parent_page_content(payload, base).diff() == &LayoutDiff::default() { return Ok(Vec::new()) }
    let page = base.parent_pages.iter().find(|page| page.id == payload.id).expect("validated target");
    let mut steps: Vec<Op> = base.pages.iter().filter(|held| held.parent_page_id.as_deref() == Some(payload.id.as_str())).map(|held| Op::from(crate::set_page_overrides::SetPageOverrides { id: held.id.clone(), overrides: held.overrides.clone() })).collect();
    steps.push(Op::from(SetParentPageContent { id: page.id.clone(), layers: page.layers.clone(), frames: page.frames.clone() }));
    Ok(steps)
}
