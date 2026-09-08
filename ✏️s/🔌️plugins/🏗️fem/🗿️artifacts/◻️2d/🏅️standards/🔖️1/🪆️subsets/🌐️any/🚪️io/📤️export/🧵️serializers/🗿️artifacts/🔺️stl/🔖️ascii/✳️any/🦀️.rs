//! 🚪️ fem2d → stl — foreign `Serializer<Fem2dSnapshot>` on the framework's `io_mechanism` channel,
//! and REAL geometry: every `FemRegion` footprint is triangulated and extruded by its own real
//! `thickness` into a `SemioMeshSnapshot`
//! (`crate::fem2d_engine::meshing::build_semio_mesh_snapshot`, the same kernel the
//! `🕸️mesh/🧫️fixtures/**/🧊️expected.stl` fixtures pin), then handed to stdio's own tested
//! `SemioMeshToStl` bridge and `encode_stl_ascii` grammar — never hand-rolled bytes.
//!
//! `IoFidelity::Lossy`, and that is the honest level: an STL solid is a triangle soup. It carries no
//! `FemMaterial`, `FemSection`, `FemSupport`, `FemLoadCase` or analysis settings, and
//! `FemElement::Bar`/`Beam` line members have no honest 3D solid at all (see the meshing bridge's
//! own doc) so they contribute no geometry — a bar/beam-only model exports a structurally valid,
//! EMPTY `.stl`. There is therefore no inverse: `s.stdio.stl@ascii/*` → `s.fem.fem2d@1/*` is a
//! registered refusal in this subset's `🚪️io/🦀️.rs` `geometry_import` module.

use crate::artifacts::fem2d::Fem2dSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{ArtifactSerializer, StandardId, SubsetId};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl;
use semio_s_artifact_stdio_stl::standards::v_ascii::engine::encode_stl_ascii;

/// 🎯️ The foreign dialect this leaf writes.
pub const STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId::ANY };

/// 🔺️ The extruded region mesh as real ASCII `.stl` text.
pub fn stl_text(from: &Fem2dSnapshot) -> Result<String, IoError> {
    let mesh = crate::fem2d_engine::meshing::build_semio_mesh_snapshot(from);
    let stl = semio_framework_plugin::resolve_ready(SemioMeshToStl::serialize(&mesh)).map_err(|error| IoError { message: format!("fem2d→stl: {error}"), diagnostics: Vec::new() })?;
    Ok(encode_stl_ascii(&stl))
}

/// 🧵️ `s.fem.fem2d@1/*` → `s.stdio.stl@ascii/*`.
pub struct Fem2dIntoStl;

impl Serializer<Fem2dSnapshot> for Fem2dIntoStl {
    const INTO: Dialect = STL_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &Fem2dSnapshot) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(stl_text(from)?)))
    }
}
