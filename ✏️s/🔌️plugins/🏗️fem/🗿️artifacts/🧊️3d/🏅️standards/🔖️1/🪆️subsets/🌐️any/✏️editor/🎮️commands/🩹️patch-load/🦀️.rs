//! 🩹️ Fem3d play app command — `patch-load`: one-field edit of a load (`nodeId`, `dof`, `value`,
//! `elementId`, `wx`, `wy`, `wz`, `solidId`, `pressure`) → `ReplaceLoad`.

use crate::editor::fem3d::interaction::fem3d_load_owner;
use crate::standards::v1::subsets::any::schema::mutations::replace_load;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::{FemDof, FemLoad};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem3dSnapshot = crate::Fem3dSnapshot;

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
pub fn handle(payload: &PatchLoad, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let (case_id, load) = fem3d_load_owner(doc.snapshot, &payload.id).ok_or_else(|| Fault::from("fem3d.patch.load-missing"))?;
    let mut new_load = load.clone();
    let number = || -> Result<f64, Fault> { payload.value.trim().parse().map_err(|_| Fault::from("fem3d.patch.load-value")) };
    let text = payload.value.trim().to_string();
    match (&mut new_load, payload.field.as_str()) {
        (FemLoad::Nodal { node_id, .. }, "nodeId") => *node_id = text,
        (FemLoad::Nodal { dof, .. }, "dof") => {
            *dof = match text.to_ascii_lowercase().as_str() {
                "tx" => FemDof::Tx,
                "ty" => FemDof::Ty,
                "tz" => FemDof::Tz,
                "rx" => FemDof::Rx,
                "ry" => FemDof::Ry,
                "rz" => FemDof::Rz,
                _ => return Err(Fault::from("fem3d.patch.load-value")),
            }
        }
        (FemLoad::Nodal { value, .. }, "value") => *value = number()?,
        (FemLoad::MemberUdl { element_id, .. }, "elementId") => *element_id = text,
        (FemLoad::MemberUdl { wx, .. }, "wx") => *wx = number()?,
        (FemLoad::MemberUdl { wy, .. }, "wy") => *wy = number()?,
        (FemLoad::MemberUdl { wz, .. }, "wz") => *wz = number()?,
        (FemLoad::Area { solid_id, .. }, "solidId") => *solid_id = text,
        (FemLoad::Area { pressure, .. }, "pressure") => *pressure = number()?,
        _ => return Err(Fault::from("fem3d.patch.load-field")),
    }
    if &new_load == load {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![Fem3dMutation::ReplaceLoad(replace_load::ReplaceLoad { case_id: case_id.to_string(), load_id: payload.id.clone(), new_load: Box::new(new_load) })]))
}
//#endregion 🔖️PatchLoad

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
