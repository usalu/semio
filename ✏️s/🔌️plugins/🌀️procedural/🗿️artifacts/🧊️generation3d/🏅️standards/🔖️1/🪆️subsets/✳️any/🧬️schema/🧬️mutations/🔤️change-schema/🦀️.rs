//! 🔧 `change-schema` payload — document-level scalar: the fixture's own schema version string
//! (`📓️derivation-rules.md` rule 1's `change-<field>` per remaining scalar).

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
//#region 🔖️ChangeSchema
/// 🔧 Whole-artifact scope — the fixture has exactly one schema field.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSchema {
    pub new_schema: String,
}

impl<Op> protocol::MutationKind<Generation3dSnapshot, Op> for ChangeSchema
where
    Op: protocol::Mutation<Generation3dSnapshot, Diff = Generation3dDiff> + From<ChangeSchema>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "schema", kind: "change-schema", record: "ChangedSchema" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::change_schema::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        crate::change_schema::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change schema to \"{}\"", self.new_schema), &format!("Schema auf \"{}\" ändern", self.new_schema))
    }
}
//#endregion 🔖️ChangeSchema
