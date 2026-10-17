//! ↩️ `rename-step` / `change-step-description` — undo reconstructed from the BASE-state step;
//! missing id ⇒ `Vec::new()`.

use crate::standards::v1::subsets::any::schema::mutations::rename_step::mutation::RenameStep;
use crate::{forms_steps, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &RenameStep, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rename_step::mutation::RenameStep>,
{
    Ok((|| {
    match forms_steps(base).iter().find(|step| step.id == payload.id) {
        Some(step) => vec![Op::from(RenameStep { id: payload.id.clone(), new_title: step.title.clone() })],
        None => Vec::new(),
    }

    })())
}
