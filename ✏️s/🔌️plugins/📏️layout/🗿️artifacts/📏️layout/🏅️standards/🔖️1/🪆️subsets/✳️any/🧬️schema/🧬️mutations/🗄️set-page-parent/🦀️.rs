//! 🗄️ `set-page-parent` — assigns or clears the parent page of a page.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagesDelta, LayoutPagesModification, PagePatch};
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPageParent {
    pub id: String,
    pub parent_page_id: Option<String>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for SetPageParent {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-parent", kind: "set-page-parent", record: "SetPageParent" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_set_page_parent(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({ inverse_set_page_parent(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Set parent of page \"{}\"", self.id), &format!("Mustervorlage von Seite \"{}\" setzen", self.id)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

pub fn diff_set_page_parent(payload: &SetPageParent, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(parent) = &payload.parent_page_id {
        if !base.parent_pages.iter().any(|item| item.id == *parent) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Parent page \"{}\" does not exist.", parent), [parent.clone()]);
        }
    }
    if page.parent_page_id == payload.parent_page_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Page parent is already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { modified: vec![LayoutPagesModification { id: payload.id.clone(), patch: PagePatch { parent_page_id: Some(payload.parent_page_id.clone()), ..Default::default() } }], ..Default::default() }), ..Default::default() })
}

pub fn inverse_set_page_parent(payload: &SetPageParent, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else { return Vec::new() };
    vec![LayoutMutation::SetPageParent(SetPageParent { id: page.id.clone(), parent_page_id: page.parent_page_id.clone() })]

    })())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
