//! ↩️ A rename inverse leaves every independently edited term intact.
use super::ChangeCombinationName;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
pub fn inverse<Op: From<ChangeCombinationName>>(payload:&ChangeCombinationName,base:&Fem2dSnapshotValue)->Result<Vec<Op>,semio_framework_value::ValueError>{Ok(base.combinations.iter().find(|row|row.id==payload.id).filter(|row|row.name!=payload.name).map(|row|vec![Op::from(ChangeCombinationName{id:payload.id.clone(),name:row.name.clone()})]).unwrap_or_default())}