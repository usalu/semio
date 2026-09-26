use super::ChangeSlopeAngle;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;
pub fn inverse(payload: &ChangeSlopeAngle, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    let a = base.slopes.iter().find(|f| f.id == payload.id).map(|f| f.angle_deg).unwrap_or(payload.new_angle_deg);
    vec![En1997Mutation::ChangeSlopeAngle(ChangeSlopeAngle { id: payload.id.clone(), new_angle_deg: a })]
}
