//! 🕸️ `create-mesh` — sets an object's `mesh` CHILD slot to a new owned handle (overwrite-aware,

use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateMesh {
    pub id: String,
    pub child_id: String,
    pub target: semio_framework_artifact_reference::ArtifactRef,
    pub mesh_workspace: String,
    #[canonical_json(tree)]
    #[value(with="crate::managed_mesh::json")]
    pub mesh_state: Option<semio_s_artifact_lowpoly_lowpoly_model::managed_mesh::LowpolyMeshState>,
}

impl CreateMesh {
    /// 🕸️ The create-mesh row that sets `mesh` back onto `object` exactly as it holds it: the mesh entity's own replace-by-base value.
    pub fn holding(object: &semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolyObject, handle: &store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot>) -> Self {
        Self { id: object.id.clone(), child_id: handle.child_id.clone(), target: handle.target.clone(), mesh_workspace: object.mesh_content.clone(), mesh_state: object.mesh_state.clone() }
    }
}

impl<Op> protocol::MutationKind<LowpolySnapshot, Op> for CreateMesh
where
    Op: protocol::Mutation<LowpolySnapshot, Diff = semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff> + From<crate::create_mesh::CreateMesh> + From<crate::delete_mesh::DeleteMesh>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "mesh", kind: "create-mesh", record: "CreatedMesh" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create mesh on object \"{}\"", self.id), &format!("Netz auf Objekt \"{}\" erstellen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<LowpolySnapshot, Op> for CreateMesh
where
    Op: protocol::ColdMutation<LowpolySnapshot, ReferenceDiff = semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff> + From<crate::create_mesh::CreateMesh> + From<crate::delete_mesh::DeleteMesh>,
{
    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<LowpolySnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
