//! 🧬️ Din16798 artifact — document mutation dispatch (SetDocument only).

pub use crate::document::SetDocumentMutation;

use crate::artifacts::din16798::Document;

/// @emoji 🧬️ Whole-document replace — the only norm-family document mutation today.
pub type Din16798Mutation = SetDocumentMutation<Document>;
