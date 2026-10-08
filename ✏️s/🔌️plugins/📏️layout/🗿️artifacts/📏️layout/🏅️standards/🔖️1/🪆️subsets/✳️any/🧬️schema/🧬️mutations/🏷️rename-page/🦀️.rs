//! 🏷️ `rename-page` — changes a page's identity `name` field.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutPagesDelta, LayoutPagesModification, PagePatch};
use crate::{LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🏷️RenamePage
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RenamePage {
    pub id: String,
    pub new_name: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for RenamePage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "page", kind: "rename-page", record: "RenamedPage" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_rename_page(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse_rename_page(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename page to \"{}\"", self.new_name), &format!("Seite in \"{}\" umbenennen", self.new_name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🏷️RenamePage

//#region 🏷️RenamePage
pub fn diff_rename_page(payload: &RenamePage, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(page) = base.pages.iter().find(|page| page.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Page \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if page.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Page \"{}\" already has that name.", payload.id));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        pages: Some(LayoutPagesDelta { modified: vec![LayoutPagesModification { id: payload.id.clone(), patch: PagePatch { name: Some(payload.new_name.clone()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🏷️RenamePage

//#region 🏷️RenamePage
pub fn inverse_rename_page(payload: &RenamePage, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.pages.iter().find(|page| page.id == payload.id) {
        Some(page) => vec![LayoutMutation::RenamePage(RenamePage { id: payload.id.clone(), new_name: page.name.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🏷️RenamePage
