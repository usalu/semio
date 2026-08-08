//! 🧬️ Iso16757 artifact — document mutation dispatch (SetDocument only).

pub use crate::document::SetDocumentMutation;

use crate::artifacts::iso16757::Document;

/// @emoji 🧬️ Whole-document replace — the only norm-family document mutation today.
pub type Iso16757Mutation = SetDocumentMutation<Document>;
