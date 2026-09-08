//! 🔁 `update-widget` payload — replaces the whole body of an EXISTING id-keyed [`Widget`]
//! atomically (`Widget` is a discriminated union with no independently-settable scalar fields
//! exposed here, so whole-body replace is the cohesive facet per `📓️taxonomy.md`'s `update` row).
//!
//! Directory kept at its pre-migration `🎛set-widget` path — see `➖remove-widget/🦠️mutation`'s
//! docstring for why.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️UpdateWidget
/// 🔁 The widget's own id (via [`crate::widget_id`]) addresses the target
/// — no separate `id` field, since `Widget` already carries its identity.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UpdateWidget {
    pub widget: Widget,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for UpdateWidget {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "widget", kind: "update-widget", record: "UpdatedWidget" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::standards::v1::subsets::any::schema::mutations::update_widget::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
        crate::standards::v1::subsets::any::schema::mutations::update_widget::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Update widget \"{}\"", crate::widget_id(&self.widget))
    }

    fn target(&self) -> Vec<String> {
        vec![crate::widget_id(&self.widget).to_string()]
    }
}
//#endregion 🔖️UpdateWidget
