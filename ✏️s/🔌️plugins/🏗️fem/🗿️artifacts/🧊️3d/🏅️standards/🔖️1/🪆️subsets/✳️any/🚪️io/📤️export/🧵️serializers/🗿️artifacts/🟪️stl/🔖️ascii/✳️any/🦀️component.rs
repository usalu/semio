//! 📤️ Export Fem3dSnapshot as real ASCII .stl text — bridges through the real semio mesh subset
//! instead of hand-rolled bytes. Every `FemSolid`'s footprint is triangulated, extruded by its
//! own real `height` (offset by `base_z`), and reduced to its outward surface
//! (`crate::fem3d_engine::meshing::build_semio_mesh_snapshot` — the honest-geometry replacement for the old
//! JsonCodec-under-.stl leaf this file used to be), then handed to stdio's real, tested
//! `SemioMeshToStl` bridge + `stl::standards::v_ascii::engine::encode_stl_ascii` grammar.
//! `FemElement::Bar`/`Frame` line members have no honest 3D solid (see the bridge fn's own doc)
//! and contribute no geometry; a bar/frame-only model exports a structurally valid, empty .stl.

use semio_framework_plugin::{ArtifactSerializer, IoError};
use semio_s_plugin_stdio::artifacts::stl::standards::v_ascii::engine::encode_stl_ascii;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl;

//#region 🔖️Export
pub async fn export(snapshot: &crate::artifacts::fem3d::Fem3dSnapshot) -> Result<Vec<u8>, IoError> {
    let mesh = crate::fem3d_engine::meshing::build_semio_mesh_snapshot(snapshot);
    let stl = semio_framework_plugin::resolve_ready(SemioMeshToStl::serialize(&mesh)).map_err(|e| IoError::Payload(e.to_string()))?;
    Ok(encode_stl_ascii(&stl).into_bytes())
}

pub async fn register() {}
//#endregion 🔖️Export
