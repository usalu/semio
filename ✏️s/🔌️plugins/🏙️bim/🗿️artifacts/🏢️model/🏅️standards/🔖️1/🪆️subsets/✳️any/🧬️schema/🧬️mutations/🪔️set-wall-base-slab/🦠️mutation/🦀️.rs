//! 🪔️ `set-wall-base-slab` payload. Attaches the base of a wall to the top surface of a slab of its building, or frees it again: an attached base follows the slab (a sloped slab included) plus the base offset; the resolved base height is inferred.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetWallBaseSlab {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub slab: Option<String>,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetWallBaseSlab
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetWallBaseSlab>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "wall", kind: "set-wall-base-slab", record: "SetWallBaseSlab" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Attach the base of wall \"{}\" to {}", self.id, self.slab.as_deref().map_or("the storey".to_string(), |slab| format!("slab \"{slab}\""))), &format!("Basis der Wand \"{}\" an {} anbinden", self.id, self.slab.as_deref().map_or("das Geschoss".to_string(), |slab| format!("Decke \"{slab}\""))))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetWallBaseSlab
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetWallBaseSlab>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_wall_base_slab::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_wall_base_slab::inverse::inverse(self, base))
    }
}
