//! 🏟️ `create-curtain-wall-type` payload. Brings a new curtain wall type into the library: the grid rules of both directions, the mullion sections of the interior grid lines and of the border, the default panel and the materials.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{CurtainWallType, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateCurtainWallType {
    pub id: String,
    pub curtain_wall_type: CurtainWallType,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateCurtainWallType
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateCurtainWallType> + From<crate::standards::v1::subsets::any::schema::mutations::delete_curtain_wall_type::DeleteCurtainWallType>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "curtain-wall-type", kind: "create-curtain-wall-type", record: "CreateCurtainWallType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_curtain_wall_type::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_curtain_wall_type::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create curtain wall type \"{}\"", self.curtain_wall_type.name), &format!("Fassadentyp \"{}\" anlegen", self.curtain_wall_type.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
