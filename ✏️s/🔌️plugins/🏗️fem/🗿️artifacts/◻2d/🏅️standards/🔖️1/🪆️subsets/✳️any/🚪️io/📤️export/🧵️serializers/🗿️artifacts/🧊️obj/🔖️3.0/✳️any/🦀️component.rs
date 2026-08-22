//! 📤️ Export Fem2dSnapshot as real .obj text — bridges through the real semio mesh subset
//! instead of hand-rolled bytes. Every `FemRegion`'s footprint is triangulated, extruded by its
//! own real `thickness`, and reduced to its outward surface
//! (`crate::fem2d_engine::meshing::build_semio_mesh_snapshot` — the honest-geometry replacement for the old
//! JsonCodec-under-.obj leaf this file used to be), then handed to stdio's real, tested
//! `SemioMeshToObj` bridge + `obj::standards::v3_0::engine::encode_obj` grammar. `FemElement::
//! Bar`/`Beam` line members have no honest 3D solid (see the bridge fn's own doc) and contribute
//! no geometry; a bar/beam-only model exports a structurally valid, empty .obj.

use semio_framework_plugin::{ArtifactSerializer, IoError};
use semio_s_plugin_stdio::artifacts::obj::standards::v3_0::engine::encode_obj;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::obj::v3_0::any::SemioMeshToObj;

//#region 🔖️Export
pub fn export(snapshot: &crate::artifacts::fem2d::Fem2dSnapshot) -> Result<Vec<u8>, IoError> {
    let mesh = crate::fem2d_engine::meshing::build_semio_mesh_snapshot(snapshot);
    let obj = semio_framework_plugin::resolve_ready(SemioMeshToObj::serialize(&mesh)).map_err(|e| IoError::Payload(e.to_string()))?;
    Ok(encode_obj(&obj).into_bytes())
}

pub fn register() {}
//#endregion 🔖️Export
