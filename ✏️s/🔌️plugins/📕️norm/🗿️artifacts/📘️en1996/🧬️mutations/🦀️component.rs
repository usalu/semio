//! 🧬️ En1996 artifact — document mutation dispatch (SetDocument only).

pub use crate::document::SetDocumentMutation;

use crate::artifacts::en1996::Document;

/// @emoji 🧬️ Whole-document replace — the only norm-family document mutation today.
pub type En1996Mutation = SetDocumentMutation<Document>;
