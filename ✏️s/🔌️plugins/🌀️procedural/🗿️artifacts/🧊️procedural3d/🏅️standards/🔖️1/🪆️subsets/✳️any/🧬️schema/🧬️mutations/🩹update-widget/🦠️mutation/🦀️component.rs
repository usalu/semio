//! 🔁 `update-widget` payload — replaces the whole body of an EXISTING id-keyed [`Widget`]
//! atomically (`Widget` is a discriminated union with no independently-settable scalar fields
//! exposed here, so whole-body replace is the cohesive facet per `📓️taxonomy.md`'s `update` row).
//!
//! Directory kept at its pre-migration `🎛set-widget` path — see `➖remove-widget/🦠️mutation`'s
//! docstring for why.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::Widget;
use serde::{Deserialize, Serialize};

//#region 🔖️UpdateWidget
/// 🔁 The widget's own id (via [`crate::artifacts::procedural3d::widget_id`]) addresses the target
/// — no separate `id` field, since `Widget` already carries its identity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWidget {
    pub widget: Widget}

impl protocol::MutationKind<Procedural3dSnapshot, Procedural3dMutation> for UpdateWidget {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "widget", kind: "update-widget", record: "UpdatedWidget" };

    fn diff(&self, base: &Procedural3dSnapshot) -> Procedural3dDiff {
        crate::artifacts::procedural3d::mutations::update_widget::diff::diff(self, base)
    }

    fn inverse(&self, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
        crate::artifacts::procedural3d::mutations::update_widget::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Update widget \"{}\"", crate::artifacts::procedural3d::widget_id(&self.widget))
    }

    fn target(&self) -> Vec<String> {
        vec![crate::artifacts::procedural3d::widget_id(&self.widget).to_string()]
    }
}
//#endregion 🔖️UpdateWidget
