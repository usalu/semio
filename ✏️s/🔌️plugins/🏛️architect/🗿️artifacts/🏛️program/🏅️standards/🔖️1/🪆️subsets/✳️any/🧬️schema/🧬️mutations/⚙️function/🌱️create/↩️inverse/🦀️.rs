//! ↩️ Inverse (undo) construction for the `create-function` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚙️functions` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateFunction, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeleteFunction(super::super::delete_function::DeleteFunction { id: payload.function.header.id.clone() })]

    })())
}
