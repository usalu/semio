//! 🚪️ fem3d → obj — foreign `Serializer<Fem3dSnapshot>` on the framework's `io_mechanism` channel,
//! and REAL geometry: every `FemSolid` outline is triangulated, extruded by its own real
//! `height` (offset by `base_z`), split into tets and reduced to its outward boundary faces — a
//! `SemioMeshSnapshot` (`crate::fem3d_engine::meshing::build_semio_mesh_snapshot`, the same kernel the
//! `🕸️mesh/🧫️fixtures/**/🗿️expected.obj` fixtures pin), then handed to stdio's own tested
//! `SemioMeshToObj` bridge and `encode_obj` grammar — never hand-rolled bytes.
//!
//! `IoFidelity::Lossy`, and that is the honest level: an `.obj` carries vertices and faces. It
//! carries no `FemMaterial`, `FemSection`, `FemSupport`, `FemLoadCase` or analysis settings, and
//! `FemElement::Bar`/`Frame` line members carry no cross-section PROFILE (see the meshing bridge's
//! own doc) so they contribute no geometry — a bar/frame-only model exports a structurally valid,
//! EMPTY `.obj`. There is therefore no inverse: `s.stdio.obj@3.0/*` → `s.fem.fem3d@1/*` is a
//! registered refusal in this subset's `🚪️io/🦀️.rs` `geometry_import` module.

use crate::Fem3dSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{ArtifactSerializer, StandardId, SubsetId};
use semio_s_artifact_stdio_obj::standards::v3_0::engine::encode_obj;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::obj::v3_0::any::SemioMeshToObj;

/// 🎯️ The foreign dialect this leaf writes.
pub const OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId::ANY };

/// 🧊️ The tetrahedralised solid boundary mesh as real `.obj` text.
pub fn obj_text(from: &Fem3dSnapshot) -> Result<String, IoError> {
    let mesh = crate::fem3d_engine::meshing::build_semio_mesh_snapshot(from);
    let obj = semio_framework_plugin::resolve_ready(SemioMeshToObj::serialize(&mesh)).map_err(|error| IoError { message: format!("fem3d→obj: {error}"), diagnostics: Vec::new() })?;
    Ok(encode_obj(&obj))
}

/// 🧵️ `s.fem.fem3d@1/*` → `s.stdio.obj@3.0/*`.
pub struct Fem3dIntoObj;

impl Serializer<Fem3dSnapshot> for Fem3dIntoObj {
    const INTO: Dialect = OBJ_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &Fem3dSnapshot) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(obj_text(from)?)))
    }
}
