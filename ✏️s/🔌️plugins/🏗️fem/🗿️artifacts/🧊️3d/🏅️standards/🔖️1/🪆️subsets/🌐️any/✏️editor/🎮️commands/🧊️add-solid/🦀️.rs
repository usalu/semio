//! 🧱️ 🧱️ FEM 3D app commands command — `add-solid`.

use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::Fem3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-solid")]
pub struct AddSolid {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub depth: f64,
    pub height: f64,
    pub material_id: String,
    pub base_z: Option<f64>,
    pub layers: Option<u32>,
    pub mesh_size: Option<f64>,
}

/// 🧱️ Builds a rectangular footprint `[x,y]..[x+width,y+depth]` with `base_z`/`layers`/`mesh_size`
/// defaulted to `0.0`/`1`/`0.5` when unspecified — mirrors the pre-migration `handle_action` defaults.
pub fn handle(payload: &AddSolid, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let id = crate::app_surface::next_id(snapshot.solids.iter().map(|s| s.id.clone()), "sol");
    let outline = vec![[payload.x, payload.y], [payload.x + payload.width, payload.y], [payload.x + payload.width, payload.y + payload.depth], [payload.x, payload.y + payload.depth]];
    let solid = crate::FemSolid {
        id,
        name: "Solid".into(),
        outline,
        holes: Vec::new(),
        base_z: payload.base_z.unwrap_or(0.0),
        height: payload.height,
        layers: payload.layers.map_or(1, |v| v as usize),
        mesh_size: payload.mesh_size.unwrap_or(0.5),
        material_id: payload.material_id.clone(),
    };
    Ok(Emit::mutations(vec![Fem3dMutation::CreateSolid(crate::standards::v1::subsets::any::schema::mutations::create_solid::CreateSolid { solid })]))
}
