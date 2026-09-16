//! 🩹️ Fem3d play app command — `patch-support`: one-field edit of a support (`nodeId`, or one of
//! the six DOF toggles `tx`…`rz`) → `ReplaceSupport`.

use crate::standards::v1::subsets::any::schema::mutations::replace_support;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::FemDof;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem3dSnapshot = crate::Fem3dSnapshot;

//#region 🔖️PatchSupport
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-support")]
pub struct PatchSupport {
    pub id: String,
    pub field: String,
    pub value: String,
}

fn flag(value: &str) -> Result<bool, Fault> {
    match value.trim() {
        "true" | "1" | "on" => Ok(true),
        "false" | "0" | "off" | "" => Ok(false),
        _ => Err(Fault::from("fem3d.patch.support-value")),
    }
}

/// 🛡️ A DOF toggle keeps the restrained set in the canonical `Tx…Rz` order whatever order the
/// user pressed them in, so two supports restraining the same DOFs compare equal.
pub fn handle(payload: &PatchSupport, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let support = doc.snapshot.supports.iter().find(|support| support.id == payload.id).ok_or_else(|| Fault::from("fem3d.patch.support-missing"))?;
    let mut new_support = support.clone();
    match payload.field.as_str() {
        "nodeId" => new_support.node_id = payload.value.trim().to_string(),
        field => {
            let dof = match field {
                "tx" => FemDof::Tx,
                "ty" => FemDof::Ty,
                "tz" => FemDof::Tz,
                "rx" => FemDof::Rx,
                "ry" => FemDof::Ry,
                "rz" => FemDof::Rz,
                _ => return Err(Fault::from("fem3d.patch.support-field")),
            };
            let on = flag(&payload.value)?;
            new_support.fixed = FemDof::ALL.iter().copied().filter(|candidate| if *candidate == dof { on } else { support.fixed.contains(candidate) }).collect();
        }
    }
    if &new_support == support {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem3dMutation::ReplaceSupport(replace_support::ReplaceSupport { id: payload.id.clone(), new_support })]))
}
//#endregion 🔖️PatchSupport

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
