//! ↩️ `change-has-mechanical-ventilation` inverse.

use super::ChangeHasMechanicalVentilation;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(_payload: &ChangeHasMechanicalVentilation, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeHasMechanicalVentilation(ChangeHasMechanicalVentilation { new_has_mechanical_ventilation: base.has_mechanical_ventilation })]
}
