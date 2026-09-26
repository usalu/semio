use super::ChangeStoreys;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeStoreys, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeStoreys(ChangeStoreys { new_storeys: base.storeys })]
}
