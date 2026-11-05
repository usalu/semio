//! 👁️ Canonical Forms viewer owner, isolated from editor and response event code.
extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

pub use semio_s_artifact_forms_forms_snapshot::{FormsSnapshot, FormQuestion, FormStep, FORMS_DIALECT, FORMS_DOCUMENT_SCHEMA, forms_steps, forms_snapshot_with_state, forms_genesis_child_pack};
pub use semio_s_artifact_forms_forms_snapshot::standards;
pub use semio_s_artifact_forms_forms_config::forms_config;
pub use semio_s_artifact_forms_forms_diff::question_extensions;
pub use semio_s_artifact_forms_forms_mutations::op;
pub mod schema { pub use semio_s_artifact_forms_forms_schema::schema::*; }

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod forms {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../🎭️modes/👁️view/🪟️windows/▶️try/🦀️.rs"]
                    pub mod try_wizard;
                }
            }
        }
    }
}
pub use semio_s_artifact_forms_forms_snapshot::forms_child_restore_projection;
