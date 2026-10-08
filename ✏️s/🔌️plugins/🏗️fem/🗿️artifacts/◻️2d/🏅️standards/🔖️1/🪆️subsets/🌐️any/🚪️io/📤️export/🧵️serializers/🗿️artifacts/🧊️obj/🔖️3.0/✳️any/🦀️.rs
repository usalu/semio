//! 🚪️ fem2d → obj — foreign `Serializer<Fem2dSnapshot>` on the framework's `io_mechanism` channel,
//! and REAL geometry: every `FemRegion` footprint is triangulated and extruded by its own real
//! `thickness` into a `SemioMeshSnapshot`
//! (`crate::fem2d_engine::meshing::build_semio_mesh_snapshot`, the same kernel the
//! `🕸️mesh/🧫️fixtures/**/🗿️expected.obj` fixtures pin), then handed to stdio's own tested
//! `SemioMeshToObj` bridge and `encode_obj` grammar — never hand-rolled bytes.
//!
//! `IoFidelity::Lossy`, and that is the honest level: an `.obj` carries vertices and faces. It
//! carries no `FemMaterial`, `FemSection`, `FemSupport`, `FemLoadCase` or analysis settings, and
//! `FemElement::Bar`/`Beam` line members have no honest 3D solid at all (see the meshing bridge's
//! own doc) so they contribute no geometry — a bar/beam-only model exports a structurally valid,
//! EMPTY `.obj`. There is therefore no inverse: `s.stdio.obj@3.0/*` → `s.fem.fem2d@1/*` is a
//! registered refusal in this subset's `🚪️io/🦀️.rs` `geometry_import` module.

use crate::Fem2dSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_plugin::ArtifactSerializer,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_obj::standards::v3_0::engine::encode_obj;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::obj::v3_0::any::SemioMeshToObj;

/// 🎯️ The foreign dialect this leaf writes.
pub const OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId::ANY };

/// 🧊️ The extruded region mesh as real `.obj` text.
pub fn obj_text(from: &Fem2dSnapshot) -> Result<String, IoError> {
    let mesh = crate::fem2d_engine::meshing::build_semio_mesh_snapshot(from);
    let obj = ::semio_framework_async::poll::resolve_ready(SemioMeshToObj::serialize(&mesh)).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("fem2d→obj: {error}"))))?;
    Ok(encode_obj(&obj))
}

/// 🧵️ `s.fem.fem2d@1/*` → `s.stdio.obj@3.0/*`.
pub struct Fem2dIntoObj;

impl Serializer<Fem2dSnapshot> for Fem2dIntoObj {
    const INTO: Dialect = OBJ_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &Fem2dSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(obj_text(from)?)))
    }
}
