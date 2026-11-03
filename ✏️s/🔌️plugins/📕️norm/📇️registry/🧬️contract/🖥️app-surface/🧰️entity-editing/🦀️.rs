//! 🧰️ Entity editing: the editor capabilities every norm editor shares and no artifact re-implements: copy, paste and duplicate of entities with fresh ids, bulk edit of a multi-selection, scene snap targets for handle drags and unit-aware typed entry. All pure, every result is a list of [`NormEdit`] or [`FieldWrite`] events that the existing `insertItem` and `setFields` commits turn into one undo step.
//!
//! 🔗️ Commit: [`handle_edits`]; keys and routes: [`shortcut_route`](crate::shortcut_route), bulk rows: [`bulk_rows`](crate::bulk_rows); see `visual-editor-architecture.md` §6 and §8.

use crate::*;
use semio_framework_value::DslValue;

#[path = "📋️clipboard/🦀️.rs"]
mod clipboard;
pub use clipboard::*;

#[path = "🗑️delete/🦀️.rs"]
mod delete;
pub use delete::*;

#[path = "🧲️snap-targets/🦀️.rs"]
mod snap_targets;
pub use snap_targets::*;

#[path = "🔢️typed-entry/🦀️.rs"]
mod typed_entry;
pub use typed_entry::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

/// 📦️ Resolves a list of edits into the concrete mutations of all of them, all-or-nothing, in order.
pub fn resolve_edits<D, M, F>(document: &D, edits: &[NormEdit], mut resolve: F) -> Result<Vec<M>, String>
where
    F: FnMut(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    let mut mutations = Vec::new();
    for edit in edits {
        mutations.extend(resolve(document, edit)?);
    }
    Ok(mutations)
}

/// 📤️ [`resolve_edits`] as ONE emit, so a paste, a duplicate or a bulk edit is one undo step however many rows it touches.
pub fn handle_edits<D, M, F>(document: &D, edits: &[NormEdit], resolve: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    F: FnMut(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    commit_snapshot_fields(resolve_edits(document, edits, resolve).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.edit-unresolved"), error))?)
}

/// 🌳️ Applies edits to a plain value tree (the reducer behind live previews and the test oracle).
pub fn apply_to_tree(tree: &mut DslValue, edits: &[NormEdit]) -> Result<(), String> {
    for edit in edits {
        match edit {
            NormEdit::SetField { path, value } => set_value_at_path(tree, path, value.clone())?,
            NormEdit::InsertItem { path, index, value } => insert_value_at_path(tree, path, *index, value.clone())?,
            NormEdit::RemoveItem { path, index } => remove_value_at_path(tree, path, *index)?,
            NormEdit::MoveItem { path, from, to } => {
                let row = get_value_at_path(tree, path)?.as_array().and_then(|rows| rows.get(*from)).cloned().ok_or_else(|| format!("no row {from} in {path}"))?;
                remove_value_at_path(tree, path, *from)?;
                insert_value_at_path(tree, path, *to, Some(row))?;
            }
        }
    }
    Ok(())
}
