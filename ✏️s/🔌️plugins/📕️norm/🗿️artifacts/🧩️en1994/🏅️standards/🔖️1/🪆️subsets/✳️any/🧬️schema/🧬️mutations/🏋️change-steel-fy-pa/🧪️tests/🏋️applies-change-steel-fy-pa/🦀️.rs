use crate::artifact_schema::mutations::change_steel_fy_pa::ChangeSteelFYPa;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_steel_fy_pa() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeSteelFYPa(ChangeSteelFYPa { new_steel_f_y_pa: 460e6 });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
