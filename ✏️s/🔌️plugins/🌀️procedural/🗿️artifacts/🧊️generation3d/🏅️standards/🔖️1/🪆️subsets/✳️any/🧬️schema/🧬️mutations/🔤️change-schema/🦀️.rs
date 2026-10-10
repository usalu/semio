//! 🔧 `change-schema` payload — document-level scalar: the fixture's own schema version string
//! (`📓️derivation-rules.md` rule 1's `change-<field>` per remaining scalar).

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
//#region 🔖️ChangeSchema
/// 🔧 Whole-artifact scope — the fixture has exactly one schema field.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSchema {
    pub new_schema: String,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for ChangeSchema {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "schema", kind: "change-schema", record: "ChangedSchema" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::standards::v1::subsets::any::schema::mutations::change_schema::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::change_schema::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change schema to \"{}\"", self.new_schema), &format!("Schema auf \"{}\" ändern", self.new_schema))
    }
}
//#endregion 🔖️ChangeSchema
