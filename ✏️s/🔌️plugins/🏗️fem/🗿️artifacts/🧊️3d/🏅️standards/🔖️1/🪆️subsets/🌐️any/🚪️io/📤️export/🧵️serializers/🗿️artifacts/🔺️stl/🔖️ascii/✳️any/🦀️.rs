//! 🚪️ fem3d → stl — foreign `Serializer<Fem3dSnapshot>` on the framework's `io_mechanism` channel,
//! and REAL geometry: every `FemSolid` outline is triangulated, extruded by its own real
//! `height` (offset by `base_z`), split into tets and reduced to its outward boundary faces — a
//! `SemioMeshSnapshot` (`crate::fem3d_engine::meshing::build_semio_mesh_snapshot`, the same kernel the
//! `🕸️mesh/🧫️fixtures/**/🧊️expected.stl` fixtures pin), then handed to stdio's own tested
//! `SemioMeshToStl` bridge and `encode_stl_ascii` grammar — never hand-rolled bytes.
//!
//! `IoFidelity::Lossy`, and that is the honest level: an STL solid is a triangle soup. It carries no
//! `FemMaterial`, `FemSection`, `FemSupport`, `FemLoadCase` or analysis settings, and
//! `FemElement::Bar`/`Frame` line members carry no cross-section PROFILE (see the meshing bridge's
//! own doc) so they contribute no geometry — a bar/frame-only model exports a structurally valid,
//! EMPTY `.stl`. There is therefore no inverse: `s.stdio.stl@ascii/*` → `s.fem.fem3d@1/*` is a
//! registered refusal in this subset's `🚪️io/🦀️.rs` `geometry_import` module.

use crate::Fem3dSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{ArtifactSerializer, StandardId, SubsetId};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl;
use semio_s_artifact_stdio_stl::standards::v_ascii::engine::encode_stl_ascii;

/// 🎯️ The foreign dialect this leaf writes.
pub const STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId::ANY };

/// 🔺️ The tetrahedralised solid boundary mesh as real ASCII `.stl` text.
pub fn stl_text(from: &Fem3dSnapshot) -> Result<String, IoError> {
    let mesh = crate::fem3d_engine::meshing::build_semio_mesh_snapshot(from);
    let stl = semio_framework_plugin::resolve_ready(SemioMeshToStl::serialize(&mesh)).map_err(|error| IoError { message: format!("fem3d→stl: {error}"), diagnostics: Vec::new() })?;
    Ok(encode_stl_ascii(&stl))
}

/// 🧵️ `s.fem.fem3d@1/*` → `s.stdio.stl@ascii/*`.
pub struct Fem3dIntoStl;

impl Serializer<Fem3dSnapshot> for Fem3dIntoStl {
    const INTO: Dialect = STL_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &Fem3dSnapshot) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(stl_text(from)?)))
    }
}
