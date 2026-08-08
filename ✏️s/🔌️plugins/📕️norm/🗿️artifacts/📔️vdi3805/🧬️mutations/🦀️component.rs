//! 🧬️ Vdi3805 artifact — document mutation dispatch (SetDocument only).

pub use crate::document::SetDocumentMutation;

use crate::artifacts::vdi3805::Document;

/// @emoji 🧬️ Whole-document replace — the only norm-family document mutation today.
pub type Vdi3805Mutation = SetDocumentMutation<Document>;
