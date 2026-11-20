//! 🏗️ Typed retained field publication and inverse owners for Layout mutations.
use semio_s_artifact_layout_layout_mutations_decisions::LayoutMutationSource;

pub trait MutationPayload<P>: LayoutMutationSource + Send + 'static {
    fn payload_mut(&mut self) -> Option<&mut P>;
    fn from_payload(payload: P) -> Self;
}

#[path = "../🪡️original-field/📋️rows.rs"]
pub mod original_field;

#[path = "../📄️document/🔺️output.rs"]
pub mod document_output;
#[path = "../📄️document/↩️inverse.rs"]
pub mod document_inverse;
#[path = "../📎️overrides/🔺️output.rs"]
pub mod override_output;
#[path = "../📎️overrides/↩️inverse.rs"]
pub mod override_inverse;
#[path = "../🧾️data/🔺️output.rs"]
pub mod data_output;
#[path = "../🧾️data/↩️inverse.rs"]
pub mod data_inverse;
#[path = "../📓️spread/🔺️output.rs"]
pub mod spread_output;
#[path = "../🌱️page-creation/🔺️output.rs"]
pub mod page_creation_output;
#[path="../🗄️parent/🧩️content/🦀️.rs"]pub mod parent_content;
#[path = "📖️story/🔺️output.rs"]
pub mod story_output;
#[path = "📖️story/↩️inverse.rs"]
pub mod story_inverse;
