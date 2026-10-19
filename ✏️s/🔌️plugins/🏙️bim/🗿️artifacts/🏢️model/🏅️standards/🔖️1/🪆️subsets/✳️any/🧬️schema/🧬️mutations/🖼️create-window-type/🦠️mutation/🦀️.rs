//! 🖼️ `create-window-type` payload. Brings a new window type into the library; it needs a free id, an existing material and valid dimensions.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, WindowType};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateWindowType {
    pub id: String,
    pub window_type: WindowType,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateWindowType
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateWindowType> + From<crate::standards::v1::subsets::any::schema::mutations::delete_window_type::DeleteWindowType>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "window-type", kind: "create-window-type", record: "CreateWindowType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_window_type::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_window_type::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create window type \"{}\"", self.window_type.name), &format!("Fenstertyp \"{}\" anlegen", self.window_type.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
