//! 🧱️ The row types of the BIM entity table: one authored parameter, one inferred value and one entity kind. See [`super`].

use crate::editor::bim::terminology::BimLabels;
use crate::standards::v1::subsets::any::schema::{inferences::ModelInference, mutations::ModelMutation, snapshot::ModelSnapshot};
use semio_framework_plugin::plugin_app_close_prelude::InputKind;
use semio_framework_ui_locale::LabelText;

//#region 🔖️Types
pub type LabelOf = fn(&BimLabels) -> LabelText;

/// 🧾️ One authored parameter of an entity: how it reads off the snapshot and, when a `set-*` mutation exists for it, how an edited value becomes that mutation.
pub struct FieldRow {
    pub key: &'static str,
    pub label: LabelOf,
    pub input: InputKind,
    pub read: fn(&ModelSnapshot, &str) -> Option<String>,
    pub write: Option<fn(&ModelSnapshot, &str, &str) -> Option<ModelMutation>>,
    pub choices: Option<fn(&ModelSnapshot, &BimLabels) -> Vec<(String, String)>>,
}

/// 💡️ One inferred, read-only value of an entity.
pub struct InferredRow {
    pub key: &'static str,
    pub label: LabelOf,
    pub read: fn(&ModelSnapshot, &ModelInference, &str) -> Option<String>,
}

/// 🧱️ One entity kind of the snapshot.
pub struct EntityKind {
    pub kind: &'static str,
    pub icon: &'static str,
    pub library: bool,
    pub label: LabelOf,
    pub group: LabelOf,
    pub ids: fn(&ModelSnapshot) -> Vec<String>,
    pub name: fn(&ModelSnapshot, &str) -> Option<String>,
    pub parent: fn(&ModelSnapshot, &str) -> Option<String>,
    pub delete: Option<fn(&str) -> ModelMutation>,
    pub rename: Option<fn(&ModelSnapshot, &str, &str) -> Option<ModelMutation>>,
    pub create: Option<fn(&ModelSnapshot, &str, &str, &str) -> Result<ModelMutation, &'static str>>,
    pub fields: &'static [FieldRow],
    pub inferred: &'static [InferredRow],
}
//#endregion 🔖️Types
