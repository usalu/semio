//! 📦 App instance state owned by the OS kernel.

use semio_framework_core::ViewState;

#[derive(Clone, Debug, PartialEq)]
pub struct OsInstanceState {
    pub id: u32,
    pub app_id: String,
    pub controller_id: String,
    pub document_json: String,
    pub view_state: ViewState,
    pub generation: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OsAppInstance {
    pub id: String,
    pub program_id: String,
    pub app_id: String,
    pub label: String,
    pub yields: String,
    pub source_document_json: String,
}
