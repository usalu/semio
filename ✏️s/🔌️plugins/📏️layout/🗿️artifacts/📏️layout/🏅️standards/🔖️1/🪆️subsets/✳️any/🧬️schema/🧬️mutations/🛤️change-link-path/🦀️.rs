//! 🔗 `change-link-path` — sets an {@link ImageLink}'s file `path`.

use crate::mutations::LayoutMutation;
use crate::standards::v1::subsets::any::schema::diff::{LayoutLinksDelta, LayoutLinksModification};
use crate::{ImageLinkPatch, LayoutDiff, LayoutSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔗ChangeLinkPath
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeLinkPath {
    pub id: String,
    pub new_path: String,
}

impl MutationKind<LayoutSnapshot, LayoutMutation> for ChangeLinkPath {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "link-path", kind: "change-link-path", record: "ChangedLinkPath" };
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
        diff_change_link_path(self, base)
    }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse_change_link_path(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change link \"{}\" path", self.id), &format!("Pfad von Verknüpfung \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔗ChangeLinkPath

//#region 🔗ChangeLinkPath
pub fn diff_change_link_path(payload: &ChangeLinkPath, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(link) = base.links.iter().find(|link| link.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Link \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if link.path == payload.new_path {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Link \"{}\" already has path \"{}\".", payload.id, payload.new_path));
    }
    protocol::MutationOutcome::new(LayoutDiff {
        links: Some(LayoutLinksDelta { modified: vec![LayoutLinksModification { id: payload.id.clone(), patch: ImageLinkPatch { path: Some(payload.new_path.clone()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔗ChangeLinkPath

//#region 🔗ChangeLinkPath
pub fn inverse_change_link_path(payload: &ChangeLinkPath, base: &LayoutSnapshot) -> Result<Vec<LayoutMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.links.iter().find(|link| link.id == payload.id) {
        Some(link) => vec![LayoutMutation::ChangeLinkPath(ChangeLinkPath { id: payload.id.clone(), new_path: link.path.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔗ChangeLinkPath
