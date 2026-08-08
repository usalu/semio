//! 🧬️ En1991 artifact — document mutation dispatch (SetDocument only).

pub use crate::document::SetDocumentMutation;

use crate::artifacts::en1991::Document;

/// @emoji 🧬️ Whole-document replace — the only norm-family document mutation today.
pub type En1991Mutation = SetDocumentMutation<Document>;
