//! 📖 `update-parent-page` — sets a parent page's name and size.

#[cfg(test)] use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use semio_s_artifact_layout_layout_diff::{LayoutParentPagesDelta, ParentPagePatch};
use semio_s_artifact_layout_layout_diff::LayoutParentPagesModification;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UpdateParentPage {
    pub id: String,
    pub name: String,
    pub width: f64,
    pub height: f64,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for UpdateParentPage where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::update_parent_page::UpdateParentPage> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "parent-page", kind: "update-parent-page", record: "UpdatedParentPage" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Update parent page \"{}\"", self.name), &format!("Mustervorlage \"{}\" aktualisieren", self.name)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for UpdateParentPage where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::update_parent_page::UpdateParentPage> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_update_parent_page(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ inverse_update_parent_page(self, base)? 
    })
}
}

pub fn diff_update_parent_page(payload: &UpdateParentPage, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.parent_pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Parent page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.name.trim().is_empty() || !payload.width.is_finite() || payload.width <= 0.0 || !payload.height.is_finite() || payload.height <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A parent page needs a name and a positive finite size.", std::iter::empty::<String>());
    }
    if page.name == payload.name && semio_framework_value::equal_f64(page.width, payload.width) && semio_framework_value::equal_f64(page.height, payload.height) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Parent page is already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff { parent_pages: Some(LayoutParentPagesDelta { modified: vec![LayoutParentPagesModification { id: payload.id.clone(), patch: ParentPagePatch { name: Some(payload.name.clone()), width: Some(payload.width), height: Some(payload.height) } }], ..Default::default() }), ..Default::default() })
}

pub fn inverse_update_parent_page<Op: From<crate::update_parent_page::UpdateParentPage>>(payload: &UpdateParentPage, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(page) = base.parent_pages.iter().find(|page| page.id == payload.id) else { return Vec::new() };
    vec![Op::from(UpdateParentPage { id: page.id.clone(), name: page.name.clone(), width: page.width, height: page.height })]

    })())
}


impl PartialEq for UpdateParentPage {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && semio_framework_value::equal_f64(self.width, other.width)
            && semio_framework_value::equal_f64(self.height, other.height)
    }
}
