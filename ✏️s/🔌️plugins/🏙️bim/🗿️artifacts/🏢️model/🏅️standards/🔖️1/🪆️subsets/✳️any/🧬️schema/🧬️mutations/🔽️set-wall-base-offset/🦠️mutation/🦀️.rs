//! 🔽️ `set-wall-base-offset` payload. Sets the distance of a wall's base above its storey's floor in metres; the resolved base, top and height follow by inference.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetWallBaseOffset {
    pub id: String,
    pub base_offset: f64,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetWallBaseOffset
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetWallBaseOffset>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "wall", kind: "set-wall-base-offset", record: "SetWallBaseOffset" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_wall_base_offset::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_wall_base_offset::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set wall base offset to {} m", self.base_offset), &format!("Wandfußversatz auf {} m setzen", self.base_offset))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
