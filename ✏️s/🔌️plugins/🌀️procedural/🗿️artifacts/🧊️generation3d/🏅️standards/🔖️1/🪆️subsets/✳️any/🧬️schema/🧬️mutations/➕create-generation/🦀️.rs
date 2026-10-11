//! ➕ `create-generation` payload — brings a new id-keyed [`FormGeneration`] into existence.
//! Delegates to `semio_framework_artifact_playbook_playbook`'s existing `GenerationMutation::Add` engine (framework territory,
//! out of this facet's writable boundary) via the sibling `🔺️diff`/`↩️inverse` leaves.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_playbook_playbook::FormGeneration;
//#region 🔖️CreateGeneration
/// ➕ Full initial payload for a new generation.
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateGeneration {
    pub generation: FormGeneration,
    /// 📍 Zero-based insertion position among the generations; `None` or past the end appends.
    pub index: Option<usize>,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for CreateGeneration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "generation", kind: "create-generation", record: "CreatedGeneration" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_generation::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
        crate::standards::v1::subsets::any::schema::mutations::create_generation::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create generation \"{}\"", self.generation.name), &format!("Erzeugung \"{}\" erstellen", self.generation.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.generation.id.clone()]
    }
}
//#endregion 🔖️CreateGeneration
