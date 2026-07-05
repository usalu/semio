//! ✍️ Writer WASM package: re-exports framework editor and writer document VCS.

pub use framework_editor::*;

pub type WriterHost = EditorHost;

#[cfg(target_arch = "wasm32")]
pub type WriterSession = EditorSession;

mod document_vcs;
pub use document_vcs::*;
