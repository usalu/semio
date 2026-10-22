//! 🌱 Generation2d mutation — `CreateWidget`: brings a new id-keyed widget into existence at a
//! FINAL-state insertion index. Wired module name (`set_widget`) is a leftover of the pre-semantic
//! generic slot this triad was repurposed from — see `sharedFileRequests` in this ticket's wave2
//! report for the glue.rs rename that would align the directory/module with the verb.

use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::widget_id;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️CreateWidget
/// 🌱 `create-widget` payload — full initial widget payload plus a FINAL-state insertion index.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateWidget {
    pub index: usize,
    pub widget: Widget,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_widget<Op: From<CreateWidget>>(index: usize, widget: Widget) -> Op {
    Op::from(CreateWidget { index, widget })
}

impl<Op> MutationKind<Generation2dSnapshot, Op> for CreateWidget
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<crate::delete_widget::DeleteWidget>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "widget", kind: "create-widget", record: "CreatedWidget" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create widget \"{}\"", widget_id(&self.widget)), &format!("Widget \"{}\" erstellen", widget_id(&self.widget)))
    }
    fn target(&self) -> Vec<String> {
        vec![widget_id(&self.widget).to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<Generation2dSnapshot, Op> for CreateWidget
where
    Op: protocol::ColdMutation<Generation2dSnapshot, ReferenceDiff = Generation2dDiff> + From<crate::delete_widget::DeleteWidget>,
{
    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::create_widget::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::create_widget::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️CreateWidget
