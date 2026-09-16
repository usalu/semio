//! 🩹️ Fem2d play app command — `patch-support`: re-targets a support (`nodeId`) or toggles one restrained DOF (`tx`, `ty`, `rz`) → `ReplaceSupport`.

use crate::standards::v1::subsets::any::schema::mutations::replace_support;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::FemDof;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️PatchSupport
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-support")]
pub struct PatchSupport {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ A support has no scalar "fixity" field — `fixed` is a DOF set, so the inspector shows one
/// toggle per planar DOF and each toggle is a membership edit rewritten here into the whole record.
/// The rebuilt set is always emitted in `FemDof::ALL` order, so two hosts toggling different DOFs
/// converge on the same spelling instead of on two permutations of the same set.
pub fn handle(payload: &PatchSupport, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let support = doc.snapshot.supports.iter().find(|support| support.id == payload.id).ok_or_else(|| Fault::from("fem2d.patch.support-missing"))?;
    let mut new_support = support.clone();
    match payload.field.as_str() {
        "nodeId" => new_support.node_id = payload.value.clone(),
        "tx" | "ty" | "rz" => {
            let dof = match payload.field.as_str() {
                "tx" => FemDof::Tx,
                "ty" => FemDof::Ty,
                _ => FemDof::Rz,
            };
            let restrained: bool = payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.support-value"))?;
            let mut fixed = support.fixed.clone();
            fixed.retain(|entry| *entry != dof);
            if restrained {
                fixed.push(dof);
            }
            new_support.fixed = FemDof::ALL.into_iter().filter(|candidate| fixed.contains(candidate)).collect();
        }
        _ => return Err(Fault::from("fem2d.patch.support-field")),
    }
    if &new_support == support {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem2dMutation::ReplaceSupport(replace_support::ReplaceSupport { id: payload.id.clone(), new_support })]))
}
//#endregion 🔖️PatchSupport

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
