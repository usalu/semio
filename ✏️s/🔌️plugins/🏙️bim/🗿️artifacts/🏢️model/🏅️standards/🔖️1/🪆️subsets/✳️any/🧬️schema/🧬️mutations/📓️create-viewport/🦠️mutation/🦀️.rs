//! 📓️ `create-viewport` payload. Places a view on a sheet: a plan, ceiling plan, section or elevation at a scale from 1:1 to 1:1000, with the position of its window on the paper, an optional crop of the drawing and an optional label. The drawing is the inferred linework of the view.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, Viewport};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateViewport {
    pub id: String,
    pub viewport: Viewport,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateViewport
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateViewport> + From<semio_s_artifact_bim_model_mut_any_105::standards::v1::subsets::any::schema::mutations::delete_viewport::DeleteViewport>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "viewport", kind: "create-viewport", record: "CreateViewport" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_viewport::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_viewport::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Place view \"{}\" on sheet \"{}\"", self.viewport.view, self.viewport.sheet), &format!("Ansicht \"{}\" auf Blatt \"{}\" platzieren", self.viewport.view, self.viewport.sheet))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
