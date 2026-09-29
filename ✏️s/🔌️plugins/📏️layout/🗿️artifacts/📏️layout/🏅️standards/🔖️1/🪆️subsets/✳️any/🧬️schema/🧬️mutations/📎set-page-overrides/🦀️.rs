//! 📎 `set-page-overrides` — replaces one page's overrides of its parent-page frames.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::{LayoutBounds, LayoutDiff, LayoutSnapshot, PageOverride, PagePatch};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPageOverrides {
    pub id: String,
    pub overrides: Vec<PageOverride>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for SetPageOverrides {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-overrides", kind: "set-page-overrides", record: "SetPageOverrides" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_set_page_overrides(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Vec<LayoutMutation> { inverse_set_page_overrides(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native(&format!("Set overrides on page \"{}\"", self.id), &format!("Abweichungen von Seite \"{}\" setzen", self.id)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

fn bounds_ok(bounds: &LayoutBounds) -> bool {
    bounds.x.is_finite() && bounds.y.is_finite() && bounds.width.is_finite() && bounds.height.is_finite() && bounds.rotation.is_finite() && bounds.width >= 0.0 && bounds.height >= 0.0
}

fn overrides_ok(page_id: &str, overrides: &[PageOverride], base: &LayoutSnapshot) -> bool {
    if overrides.len() > 64 {
        return false;
    }
    let Some(page) = base.pages.iter().find(|page| page.id == page_id) else { return false };
    let Some(parent_id) = &page.parent_page_id else { return overrides.is_empty() };
    let Some(parent) = base.parent_pages.iter().find(|parent| parent.id == *parent_id) else { return false };
    overrides.iter().all(|item| parent.frames.iter().any(|frame| frame.id() == item.object_id) && item.bounds.as_ref().map(bounds_ok).unwrap_or(true))
}

pub fn diff_set_page_overrides(payload: &SetPageOverrides, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !overrides_ok(&payload.id, &payload.overrides, base) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A page holds at most 64 overrides, each names a frame on its parent page, and each box is finite with a non-negative size.", std::iter::empty::<String>());
    }
    if page.overrides == payload.overrides {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Page overrides are already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { patched: vec![LayoutPagePatchEntry { id: payload.id.clone(), patch: PagePatch { overrides: Some(payload.overrides.clone()), ..Default::default() } }], ..Default::default() }), ..Default::default() })
}

pub fn inverse_set_page_overrides(payload: &SetPageOverrides, base: &LayoutSnapshot) -> Vec<LayoutMutation> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else { return Vec::new() };
    vec![LayoutMutation::SetPageOverrides(SetPageOverrides { id: page.id.clone(), overrides: page.overrides.clone() })]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
