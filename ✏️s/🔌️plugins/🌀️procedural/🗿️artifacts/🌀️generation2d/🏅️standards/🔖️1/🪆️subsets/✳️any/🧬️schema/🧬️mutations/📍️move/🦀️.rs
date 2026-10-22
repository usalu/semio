//! 📍 Generation2d mutation — `MoveWidget`: absolute reposition of a widget's canvas layout entry
//! (creates the entry if the widget had none). Wired module name (`set_layout`) is a leftover of
//! the pre-semantic generic slot this triad was repurposed from — see `sharedFileRequests` in this
//! ticket's wave2 report for the glue.rs rename that would align the directory/module with the verb.

use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_artifact_flow_flow::WidgetLayout;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️MoveWidget
/// 📍 `move-widget` payload — the widget's new absolute canvas position.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MoveWidget {
    pub id: String,
    pub layout: WidgetLayout,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_widget<Op: From<MoveWidget>>(id: String, layout: WidgetLayout) -> Op {
    Op::from(MoveWidget { id, layout })
}

impl<Op> MutationKind<Generation2dSnapshot, Op> for MoveWidget
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<crate::clear_widget_layout::ClearWidgetLayout> + From<MoveWidget>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "widget", kind: "move-widget", record: "MovedWidget" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move widget \"{}\" to ({}, {})", self.id, self.layout.x, self.layout.y), &format!("Widget \"{}\" nach ({}, {}) verschieben", self.id, self.layout.x, self.layout.y))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Generation2dSnapshot, Op> for MoveWidget
where
    Op: protocol::ColdMutation<Generation2dSnapshot, ReferenceDiff = Generation2dDiff> + From<crate::clear_widget_layout::ClearWidgetLayout> + From<MoveWidget>,
{
    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::move_widget::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::move_widget::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️MoveWidget
