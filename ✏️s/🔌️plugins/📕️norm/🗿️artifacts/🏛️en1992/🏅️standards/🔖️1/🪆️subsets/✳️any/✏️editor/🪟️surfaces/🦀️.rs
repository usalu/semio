//! 🪟️ En1992 editor surfaces: the edit mode with its windows and the panels, mounted from their taxonomy nodes.

extern crate semio_os_kernel as store;

pub use semio_s_artifact_norm_en1992_editor_family::{LABEL, VARIANT, CONTROLLER_ID, DOCUMENT_SCHEMA, ENTITY_KIND_SCENE_KINDS, En1992Family, Host};
pub extern crate semio_s_artifact_norm_en1992_editor_field_meta as field_meta;

//#region 🪢️Mounts
#[path = ".."]
pub mod modes {
    #[path = "🎭️modes/✏️edit/🦀️.rs"]
    pub mod edit;
}
#[path = ".."]
pub mod panels {
    #[path = "📌️panels/📚️catalogue/🦀️.rs"]
    pub mod catalogue;
    #[path = "📌️panels/🗿️artifact/🦀️.rs"]
    pub mod document;
    #[path = "📌️panels/🔍️inspection/🦀️.rs"]
    pub mod inspection;
}
//#endregion 🪢️Mounts
