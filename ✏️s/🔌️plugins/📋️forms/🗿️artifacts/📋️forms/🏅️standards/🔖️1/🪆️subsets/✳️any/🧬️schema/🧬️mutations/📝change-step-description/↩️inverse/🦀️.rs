//! ↩️ `rename-step` / `change-step-description` — undo reconstructed from the BASE-state step;
//! missing id ⇒ `Vec::new()`.

use crate::standards::v1::subsets::any::schema::mutations::change_step_description::mutation::ChangeStepDescription;
use crate::{forms_steps, FormsSnapshot};

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &ChangeStepDescription, base: &FormsSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_step_description::mutation::ChangeStepDescription>,
{
    Ok((|| {
    match forms_steps(base).iter().find(|step| step.id == payload.id) {
        Some(step) => vec![Op::from(ChangeStepDescription { id: payload.id.clone(), new_description: step.description.clone() })],
        None => Vec::new(),
    }

    })())
}
