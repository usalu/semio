//! 📤️ Export Fem2dSnapshot as real ASCII .stl text — bridges through the real semio mesh subset
//! instead of hand-rolled bytes. Every `FemRegion`'s footprint is triangulated, extruded by its
//! own real `thickness`, and reduced to its outward surface
//! (`crate::fem2d_engine::meshing::build_semio_mesh_snapshot` — the honest-geometry replacement for the old
//! JsonCodec-under-.stl leaf this file used to be), then handed to stdio's real, tested
//! `SemioMeshToStl` bridge + `stl::standards::v_ascii::engine::encode_stl_ascii` grammar.
//! `FemElement::Bar`/`Beam` line members have no honest 3D solid (see the bridge fn's own doc)
//! and contribute no geometry; a bar/beam-only model exports a structurally valid, empty .stl.

use semio_framework_plugin::{ArtifactSerializer, IoError};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl;
use semio_s_plugin_stdio::artifacts::stl::standards::v_ascii::engine::encode_stl_ascii;

//#region 🔖️Export
pub fn export(snapshot: &crate::artifacts::fem2d::Fem2dSnapshot) -> Result<Vec<u8>, IoError> {
    let mesh = crate::fem2d_engine::meshing::build_semio_mesh_snapshot(snapshot);
    let stl = semio_framework_plugin::resolve_ready(SemioMeshToStl::serialize(&mesh)).map_err(|e| IoError::Payload(e.to_string()))?;
    Ok(encode_stl_ascii(&stl).into_bytes())
}

pub fn register() {}
//#endregion 🔖️Export
