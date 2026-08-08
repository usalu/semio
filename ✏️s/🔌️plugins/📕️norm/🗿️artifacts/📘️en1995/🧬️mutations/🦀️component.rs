//! 🧬️ En1995 artifact — document mutation dispatch (SetDocument only).

pub use crate::document::SetDocumentMutation;

use crate::artifacts::en1995::Document;

/// @emoji 🧬️ Whole-document replace — the only norm-family document mutation today.
pub type En1995Mutation = SetDocumentMutation<Document>;
