//! 🕸️ `create-mesh` — sets an object's `mesh` CHILD slot to a new owned handle (overwrite-aware,

use crate::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateMesh {
    pub id: String,
    pub child_id: String,
    pub target: semio_framework_artifact_reference::ArtifactRef,
    pub mesh_workspace: String,
    #[value(with="crate::managed_mesh::json")]
    pub mesh_state: Option<crate::LowpolyMeshState>,
}

impl CreateMesh {
    /// 🕸️ The create-mesh row that sets `mesh` back onto `object` exactly as it holds it: the mesh entity's own replace-by-base value.
    pub fn holding(object: &crate::LowpolyObject, handle: &store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot>) -> Self {
        Self { id: object.id.clone(), child_id: handle.child_id.clone(), target: handle.target.clone(), mesh_workspace: object.mesh_content.clone(), mesh_state: object.mesh_state.clone() }
    }
}

impl protocol::MutationKind<LowpolySnapshot, LowpolyMutation> for CreateMesh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "mesh", kind: "create-mesh", record: "CreatedMesh" };

    fn diff(&self, base: &LowpolySnapshot) -> protocol::MutationOutcome<<LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &LowpolySnapshot) -> Result<Vec<LowpolyMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create mesh on object \"{}\"", self.id), &format!("Netz auf Objekt \"{}\" erstellen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
