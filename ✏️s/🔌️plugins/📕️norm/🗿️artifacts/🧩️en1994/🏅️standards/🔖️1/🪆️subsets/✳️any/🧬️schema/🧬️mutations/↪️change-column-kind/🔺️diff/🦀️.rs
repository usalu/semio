//! Diff for `change-column-kind`.
use super::ChangeColumnKind;
use crate::diff::En1994ColumnList;
use crate::{En1994Diff, En1994Snapshot};
pub fn diff(payload: &ChangeColumnKind, base: &En1994Snapshot) -> protocol::MutationOutcome<En1994Diff> {
    let Some(col) = base.columns.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "column missing", [payload.index.to_string()]);
    };
    if col.kind == payload.new_kind {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    let mut columns = base.columns.clone();
    columns[payload.index].kind = payload.new_kind.clone();
    protocol::MutationOutcome::new(En1994Diff { columns: Some(En1994ColumnList { values: columns }), ..Default::default() })
}
