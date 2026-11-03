//! 🗑️ Batch delete: the rows of a multi-selection leave their collection as ONE atomic edit batch. Every row is removed by the per-kind remove mutation the single `removeItem` raises (a kind whose invariant refuses a removal refuses the whole batch, nothing is half deleted); positions are removed from the end of the list, so the earlier positions stay valid while the mutations apply in order. Pure: the document tree and the ids in, [`NormEdit`] out.
//!
//! 🔗️ Command: `deleteEntities` ([`LiveCommand::DeleteEntities`](crate::LiveCommand)); commit: [`handle_edits`](crate::handle_edits) through `live_entity_edit`; the refusal notices: [`norm_fault_notices`](crate::norm_fault_notices).

use super::*;

/// 🚫️ Why a batch delete is refused before anything is emitted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeleteRefusal {
    Nothing,
    UnknownCollection(String),
    UnknownId { collection: String, id: String },
}

impl DeleteRefusal {
    /// 🆔️ The fault code of the refusal, a row of [`norm_fault_notices`](crate::norm_fault_notices) with its EN and DE sentence.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Nothing => "norm.entity-delete-empty",
            Self::UnknownCollection(_) | Self::UnknownId { .. } => "norm.entity-delete-unknown",
        }
    }

    /// 💬️ The reason in the viewer's language (no default language: the caller states it).
    pub fn message(&self, german: bool) -> String {
        match (self, german) {
            (Self::Nothing, false) => "Select at least one row to delete".to_owned(),
            (Self::Nothing, true) => "Mindestens eine Zeile zum Löschen auswählen".to_owned(),
            (Self::UnknownCollection(collection), false) => format!("'{collection}' is not a list of this document"),
            (Self::UnknownCollection(collection), true) => format!("'{collection}' ist keine Liste dieses Dokuments"),
            (Self::UnknownId { collection, id }, false) => format!("'{id}' is not a row of '{collection}'"),
            (Self::UnknownId { collection, id }, true) => format!("'{id}' ist keine Zeile von '{collection}'"),
        }
    }
}

impl std::fmt::Display for DeleteRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message(false))
    }
}

/// 🗑️ The edits that delete the rows `ids` of `collection`: one [`NormEdit::RemoveItem`] per distinct row, the last position first. An empty selection, an unknown collection and an unknown id are refused so nothing is committed.
pub fn delete_edits(tree: &DslValue, collection: &str, ids: &[String]) -> Result<Vec<NormEdit>, DeleteRefusal> {
    if ids.is_empty() {
        return Err(DeleteRefusal::Nothing);
    }
    let rows = tree.get(collection).and_then(DslValue::as_array).ok_or_else(|| DeleteRefusal::UnknownCollection(collection.to_owned()))?;
    let mut indices = ids
        .iter()
        .map(|id| rows.iter().position(|row| object_id_field(row) == Some(id.as_str())).ok_or_else(|| DeleteRefusal::UnknownId { collection: collection.to_owned(), id: id.clone() }))
        .collect::<Result<Vec<usize>, _>>()?;
    indices.sort_unstable_by(|a, b| b.cmp(a));
    indices.dedup();
    Ok(indices.into_iter().map(|index| NormEdit::RemoveItem { path: collection.to_owned(), index }).collect())
}
