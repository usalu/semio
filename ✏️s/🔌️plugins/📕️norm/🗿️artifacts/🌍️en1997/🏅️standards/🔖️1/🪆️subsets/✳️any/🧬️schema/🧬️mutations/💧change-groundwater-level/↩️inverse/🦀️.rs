use super::ChangeGroundwaterLevel;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;
pub fn inverse(_payload: &ChangeGroundwaterLevel, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeGroundwaterLevel(ChangeGroundwaterLevel { new_groundwater_level: base.groundwater_level })]
}
