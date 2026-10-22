//! 🦠️ `🔁️replace-widget` payload and its `MutationKind` impl; diff/inverse delegate to the sibling leaves.
use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::widget_id;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ReplaceWidget {
    pub widget: Widget,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_widget<Op: From<ReplaceWidget>>(widget: Widget) -> Op {
    Op::from(ReplaceWidget { widget })
}

impl<Op> MutationKind<Generation2dSnapshot, Op> for ReplaceWidget
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<ReplaceWidget>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "widget", kind: "replace-widget", record: "ReplacedWidget" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace widget \"{}\"", widget_id(&self.widget)), &format!("Widget \"{}\" ersetzen", widget_id(&self.widget)))
    }
    fn target(&self) -> Vec<String> {
        vec![widget_id(&self.widget).to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<Generation2dSnapshot, Op> for ReplaceWidget
where
    Op: protocol::ColdMutation<Generation2dSnapshot, ReferenceDiff = Generation2dDiff> + From<ReplaceWidget>,
{
    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::replace_widget::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::replace_widget::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
