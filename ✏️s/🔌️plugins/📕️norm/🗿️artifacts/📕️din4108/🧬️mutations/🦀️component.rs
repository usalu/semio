//! 🧬️ Din4108 artifact — document mutation dispatch (SetDocument only).

pub use crate::document::SetDocumentMutation;

use crate::artifacts::din4108::Document;

/// @emoji 🧬️ Whole-document replace — the only norm-family document mutation today.
pub type Din4108Mutation = SetDocumentMutation<Document>;
