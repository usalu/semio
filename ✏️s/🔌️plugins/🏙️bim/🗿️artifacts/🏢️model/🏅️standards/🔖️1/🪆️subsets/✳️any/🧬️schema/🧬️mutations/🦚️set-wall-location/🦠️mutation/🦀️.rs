//! 🦚️ `set-wall-location` payload. Chooses which face or line of the wall its axis denotes: centre, interior, exterior or core centre.

use crate::standards::v1::subsets::any::schema::{snapshot::{LocationLine, ModelSnapshot}, diff::ModelDiff};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetWallLocation {
    pub id: String,
    pub location: LocationLine,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetWallLocation
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetWallLocation>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "wall", kind: "set-wall-location", record: "SetWallLocation" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_wall_location::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_wall_location::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Locate wall \"{}\" on its {:?} line", self.id, self.location), &format!("Wand \"{}\" auf Bezugslinie {:?} legen", self.id, self.location))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
