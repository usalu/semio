//! 🧹 Generation2d mutation — `ClearWidgetLayout`: empties a widget's canvas layout entry wholesale
//! (e.g. resetting to auto-layout). Wired module name (`remove_layout`) is a leftover of the
//! pre-semantic generic slot this triad was repurposed from — see `sharedFileRequests` in this
//! ticket's wave2 report for the glue.rs rename that would align the directory/module with the verb.

use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ClearWidgetLayout
/// 🧹 `clear-widget-layout` payload — removes the layout entry for widget `id`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ClearWidgetLayout {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn clear_widget_layout<Op: From<ClearWidgetLayout>>(id: String) -> Op {
    Op::from(ClearWidgetLayout { id })
}

impl<Op> MutationKind<Generation2dSnapshot, Op> for ClearWidgetLayout
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<crate::move_widget::MoveWidget>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "clear", entity: "widget-layout", kind: "clear-widget-layout", record: "ClearedWidgetLayout" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Clear layout for widget \"{}\"", self.id), &format!("Layout für Widget \"{}\" leeren", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Generation2dSnapshot, Op> for ClearWidgetLayout
where
    Op: protocol::ColdMutation<Generation2dSnapshot, ReferenceDiff = Generation2dDiff> + From<crate::move_widget::MoveWidget>,
{
    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::clear_widget_layout::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::clear_widget_layout::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️ClearWidgetLayout
