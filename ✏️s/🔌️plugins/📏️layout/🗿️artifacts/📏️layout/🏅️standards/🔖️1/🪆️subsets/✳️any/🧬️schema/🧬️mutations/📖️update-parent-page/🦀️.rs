//! 📖 `update-parent-page` — sets a parent page's name and size.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutParentPagePatchEntry, LayoutParentPagesDelta, ParentPagePatch};
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UpdateParentPage {
    pub id: String,
    pub name: String,
    pub width: f64,
    pub height: f64,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for UpdateParentPage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "parent-page", kind: "update-parent-page", record: "UpdatedParentPage" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_update_parent_page(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> { inverse_update_parent_page(self, base) }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Update parent page \"{}\"", self.name), &format!("Mustervorlage \"{}\" aktualisieren", self.name)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

pub fn diff_update_parent_page(payload: &UpdateParentPage, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.parent_pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Parent page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.name.trim().is_empty() || !payload.width.is_finite() || payload.width <= 0.0 || !payload.height.is_finite() || payload.height <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A parent page needs a name and a positive finite size.", std::iter::empty::<String>());
    }
    if page.name == payload.name && page.width == payload.width && page.height == payload.height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Parent page is already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff { parent_pages: Some(LayoutParentPagesDelta { patched: vec![LayoutParentPagePatchEntry { id: payload.id.clone(), patch: ParentPagePatch { name: Some(payload.name.clone()), width: Some(payload.width), height: Some(payload.height) } }], ..Default::default() }), ..Default::default() })
}

pub fn inverse_update_parent_page(payload: &UpdateParentPage, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.parent_pages.iter().find(|page| page.id == payload.id) else { return Vec::new() };
    vec![LayoutMutation::UpdateParentPage(UpdateParentPage { id: page.id.clone(), name: page.name.clone(), width: page.width, height: page.height })]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
