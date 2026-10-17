//! ↩️ Inverse (undo) construction for the `rename-register-row` leaf — computed from captured pre-state (`base`), never by structurally inverting the diff.

use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse<Op>(payload: &super::RenameRegisterRow, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rename_register_row::RenameRegisterRow>,
{
    Ok(match payload.register.position(base, &payload.id).and_then(|position| payload.register.row_at(base, position)) {
        Some(existing) => vec![Op::from(super::RenameRegisterRow { register: payload.register, id: payload.id.clone(), new_name: existing.header().name.clone() })],
        None => Vec::new(),
    })
}
