//! 🩹️ Fem2d play app command — `patch-load`: one-field edit of a load (`nodeId`, `dof`, `value`, `elementId`, `wx`, `wy`, `regionId`, `pressure`) → `ReplaceLoad`.

use crate::standards::v1::subsets::any::schema::mutations::replace_load;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::editor::fem2d::interaction::fem2d_load_owner;
use crate::{FemDof, FemLoad};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️PatchLoad
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-load")]
pub struct PatchLoad {
    pub id: String,
    pub field: String,
    pub value: String,
}

/// 🩹️ Edits one load in place. A load id is unique across the whole document but a load LIVES in a
/// case, so the owning case is resolved by lookup and both ids reach `ReplaceLoad` — the inspector
/// never has to carry the case id in its argument map. The admissible field set is the one the
/// addressed variant actually has, so `wx` on a nodal load is an unknown field, not a silent no-op.
pub fn handle(payload: &PatchLoad, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let (case_id, load) = fem2d_load_owner(doc.snapshot, &payload.id).ok_or_else(|| Fault::from("fem2d.patch.load-missing"))?;
    let mut new_load = load.clone();
    let number = || -> Result<f64, Fault> { payload.value.trim().parse().map_err(|_| Fault::from("fem2d.patch.load-value")) };
    match (&mut new_load, payload.field.as_str()) {
        (FemLoad::Nodal { node_id, .. }, "nodeId") => *node_id = payload.value.clone(),
        (FemLoad::Nodal { dof, .. }, "dof") => {
            *dof = match payload.value.trim().to_ascii_lowercase().as_str() {
                "tx" => FemDof::Tx,
                "ty" => FemDof::Ty,
                "tz" => FemDof::Tz,
                "rx" => FemDof::Rx,
                "ry" => FemDof::Ry,
                "rz" => FemDof::Rz,
                _ => return Err(Fault::from("fem2d.patch.load-value")),
            }
        }
        (FemLoad::Nodal { value, .. }, "value") => *value = number()?,
        (FemLoad::MemberUdl { element_id, .. }, "elementId") => *element_id = payload.value.clone(),
        (FemLoad::MemberUdl { wx, .. }, "wx") => *wx = number()?,
        (FemLoad::MemberUdl { wy, .. }, "wy") => *wy = number()?,
        (FemLoad::Area { region_id, .. }, "regionId") => *region_id = payload.value.clone(),
        (FemLoad::Area { pressure, .. }, "pressure") => *pressure = number()?,
        _ => return Err(Fault::from("fem2d.patch.load-field")),
    }
    if &new_load == load {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem2dMutation::ReplaceLoad(replace_load::ReplaceLoad { case_id: case_id.to_string(), load_id: payload.id.clone(), new_load: Box::new(new_load) })]))
}
//#endregion 🔖️PatchLoad

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
