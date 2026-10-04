//! 📐 `set-page-guides` — replaces the guides on one page.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagePatchEntry, LayoutPagesDelta};
use crate::{LayoutDiff, LayoutRect, LayoutSnapshot, PagePatch};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPageGuides {
    pub id: String,
    pub guides: Vec<LayoutRect>,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for SetPageGuides {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-guides", kind: "set-page-guides", record: "SetPageGuides" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_set_page_guides(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({ inverse_set_page_guides(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Set guides on page \"{}\"", self.id), &format!("Hilfslinien von Seite \"{}\" setzen", self.id)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

fn guide_ok(guide: &LayoutRect) -> bool {
    guide.x.is_finite() && guide.y.is_finite() && guide.width.is_finite() && guide.height.is_finite() && guide.width >= 0.0 && guide.height >= 0.0
}

pub fn diff_set_page_guides(payload: &SetPageGuides, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.guides.len() > 64 || payload.guides.iter().any(|guide| !guide_ok(guide)) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A page has at most 64 guides, and each guide origin and size must be finite with a non-negative size.", std::iter::empty::<String>());
    }
    if page.guides == payload.guides {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Page guides are already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { patched: vec![LayoutPagePatchEntry { id: payload.id.clone(), patch: PagePatch { guides: Some(payload.guides.clone()), ..Default::default() } }], ..Default::default() }), ..Default::default() })
}

pub fn inverse_set_page_guides(payload: &SetPageGuides, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else { return Vec::new() };
    vec![LayoutMutation::SetPageGuides(SetPageGuides { id: page.id.clone(), guides: page.guides.clone() })]

    })())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
