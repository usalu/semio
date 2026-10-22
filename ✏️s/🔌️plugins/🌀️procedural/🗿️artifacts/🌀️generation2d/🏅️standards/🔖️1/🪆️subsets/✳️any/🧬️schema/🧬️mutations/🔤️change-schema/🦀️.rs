//! 🏷️ Generation2d mutation — `ChangeSchema`: sets the fixture's document-level schema field to a
//! new value.

use semio_s_artifact_procedural_2d_model::diff::Generation2dDiff;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️ChangeSchema
/// 🏷️ `change-schema` payload — the fixture's new schema id.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeSchema {
    pub schema: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_schema<Op: From<ChangeSchema>>(schema: String) -> Op {
    Op::from(ChangeSchema { schema })
}

impl<Op> MutationKind<Generation2dSnapshot, Op> for ChangeSchema
where
    Op: protocol::Mutation<Generation2dSnapshot, Diff = Generation2dDiff> + From<ChangeSchema>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "hostSnapshot", kind: "change-schema", record: "ChangedSchema" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change schema to \"{}\"", self.schema), &format!("Schema auf \"{}\" ändern", self.schema))
    }
}

impl<Op> protocol::ColdMutationKind<Generation2dSnapshot, Op> for ChangeSchema
where
    Op: protocol::ColdMutation<Generation2dSnapshot, ReferenceDiff = Generation2dDiff> + From<ChangeSchema>,
{
    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        crate::change_schema::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_schema::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️ChangeSchema
