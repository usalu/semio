//! 🎢️ `set-element-storey` payload. Stands a storey-placed element (wall, curtain wall, column, beam, slab, ceiling, roof, stair, railing, ramp, space) on another storey of its building. Hosted openings follow their host by reference; refused while the top constraint would no longer lie above the base or a hosted opening would rise above the new height.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetElementStorey {
    pub id: String,
    pub storey: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetElementStorey
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetElementStorey>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "element", kind: "set-element-storey", record: "SetElementStorey" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_element_storey::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_element_storey::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move element \"{}\" to storey \"{}\"", self.id, self.storey), &format!("Element \"{}\" in Geschoss \"{}\" verschieben", self.id, self.storey))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
