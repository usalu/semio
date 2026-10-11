//! 🔀 `reorder-pages` — repositions a page within the display-ordered `pages` list (document page
//! sequence, unlike `stories`/`links` which have no display order).

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagesDelta, LayoutPageRelocation};
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔀ReorderPages
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ReorderPages {
    pub id: String,
    pub to_index: usize,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ReorderPages {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "reorder", entity: "pages", kind: "reorder-pages", record: "ReorderedPages" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_reorder_pages(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse_reorder_pages(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Reorder page \"{}\"", self.id), &format!("Reihenfolge von Seite \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔀ReorderPages

//#region 🔀ReorderPages
pub fn diff_reorder_pages(payload: &ReorderPages, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(from) = base.pages.iter().position(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let to = payload.to_index.min(base.pages.len() - 1);
    if from == to {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Page \"{}\" is already at the requested position.", payload.id));
    }
    protocol::MutationOutcome::new(LayoutDiff { pages: Some(LayoutPagesDelta { moved: vec![LayoutPageRelocation { id: payload.id.clone(), from, to }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔀ReorderPages

//#region 🔀ReorderPages
pub fn inverse_reorder_pages(payload: &ReorderPages, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.pages.iter().position(|page| page.id == payload.id) {
        Some(original_index) => vec![LayoutMutation::ReorderPages(ReorderPages { id: payload.id.clone(), to_index: original_index })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔀ReorderPages
