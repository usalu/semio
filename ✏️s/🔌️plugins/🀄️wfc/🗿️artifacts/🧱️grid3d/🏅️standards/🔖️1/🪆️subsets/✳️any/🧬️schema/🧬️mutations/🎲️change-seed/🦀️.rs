//! 🎲 `s.wfc.grid3d` mutation — `ChangeSeed`: sets the deterministic WFC solve seed. PERSISTED
//! snapshot field, authored ONLY here — never ambient — so the solve inference's `DepHash` caching
//! stays sound (WFC is seeded-random internally).

use crate::diff::Grid3dDiff;
use crate::schema::snapshot::*;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeSeed
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeSeed {
    pub seed: u64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_seed<Op>(seed: u64) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_seed::ChangeSeed>,
{
    Op::from(ChangeSeed { seed })
}

impl<Op> MutationKind<Grid3dSnapshot, Op> for ChangeSeed
where
    Op: protocol::Mutation<Grid3dSnapshot, Diff = Grid3dDiff> + From<ChangeSeed>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "seed", kind: "change-seed", record: "ChangedSeed" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change seed to {}", self.seed), &format!("Startwert auf {} ändern", self.seed))
    }
}

impl<Op> protocol::ColdMutationKind<Grid3dSnapshot, Op> for ChangeSeed
where
    Op: protocol::ColdMutation<Grid3dSnapshot, ReferenceDiff = Grid3dDiff> + From<ChangeSeed>,
{
    fn diff(&self, base: &Grid3dSnapshot) -> protocol::MutationOutcome<Grid3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️ChangeSeed

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
//#region 🧊️MeshChild
/// 🧊️ Addresses one tile mesh as this document's child in the `s.stdio.semio@v1/mesh` store — the
/// one place the dialect is spelled, so every author (examples, editor affordances, importers) mints
/// the handle here rather than restating it. `child_id` IS the mesh artifact id.
pub fn mesh_child_handle(mesh_id: &str) -> store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot> {
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() };
    store::os_store::artifact_child::ArtifactChild::new(mesh_id.to_string(), semio_framework_artifact_reference::ArtifactRef { artifact_id: mesh_id.to_string(), dialect })
}
//#endregion 🧊️MeshChild
}
//#endregion 🧩️ArtifactItems3
