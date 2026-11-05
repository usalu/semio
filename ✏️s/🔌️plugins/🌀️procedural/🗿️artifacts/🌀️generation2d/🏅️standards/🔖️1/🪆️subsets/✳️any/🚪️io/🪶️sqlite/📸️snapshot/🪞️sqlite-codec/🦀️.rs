//! 🌀️ Owned Generation2d retained field contract and exact declared SQLite capability.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::snapshot::Generation2dSnapshot;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_os_kernel::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding}};
use semio_s_procedural_core_generation_sqlite as typed;
impl ArtifactSqliteSnapshotHost for Generation2dSnapshot{
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{c.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;if Self::SQLITE_SCHEMA.len()>c.limits().max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Generation authored schema exceeds caller limit"))}decode_sqlite_native(payload,c,native_owner)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{c.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;if Self::SQLITE_SCHEMA.len()>c.limits().max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Generation authored schema exceeds caller limit"))}encode_sqlite_native(self,encoding,c,native_owner)}
 fn retire_sqlite_snapshot(self){self.retire_cold();}
 
 
 
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{typed::preflight(&self.host_snapshot,&self.generation,c)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1).map_err(|error|store::io_schema::IoError::from_value_error(error))?;if dialect.artifact_kind!="s.procedural.generation2d"||dialect.standard!="1"||dialect.subset!="*"{return Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"Generation2d dialect differs from its owned wildcard")))}if d.table("generation_host").map_err(|error|store::io_schema::IoError::from_value_error(error))?.single_row().map_err(|error|store::io_schema::IoError::from_value_error(error))?.text(1).map_err(|error|store::io_schema::IoError::from_value_error(error))?!=self.host_snapshot.schema{return Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"Generation2d projected host schema differs")))}c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1).map_err(|error|store::io_schema::IoError::from_value_error(error))?;Ok(store::io_schema::IoOutcome::clean(()))}
}
impl pack::snapshot::ArtifactSqliteSnapshot for Generation2dSnapshot {
const SQLITE_SCHEMA:&'static str=include_str!("../🗄️.sql");
fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{typed::project(&self.host_snapshot,&self.generation,Self::SQLITE_SCHEMA,c)}
fn from_sqlite_database(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let(host_snapshot,generation)=typed::reconstruct(d,Self::SQLITE_SCHEMA,c)?;Ok(Self{host_snapshot,generation})}
}

use crate::dsl_mirror::Generation2dSnapshotDsl;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationPlayState;
use semio_os_kernel as store;

struct Generation2dNative;

impl semio_s_procedural_core_generation_sqlite_native::GenerationNative for Generation2dNative {
    type Snapshot = Generation2dSnapshot;
    const ENVELOPE_ID: &'static str = "procedural.generation2d";
    const PACK_PREFIX: Option<&'static [u8]> = Some(b"P2D2");
    fn spec_producer() -> semio_framework_dsl_record::RecordSpecProducer {
        Generation2dSnapshotDsl::__dsl_spec_producer()
    }
    fn fields(record: &semio_framework_dsl_record::RecordValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<semio_s_procedural_core_generation_sqlite_native::GenerationFields, semio_framework_value::ValueError> {
        let parsed = Generation2dSnapshotDsl::__dsl_from_record_controlled(record, control)?;
        Ok(semio_s_procedural_core_generation_sqlite_native::GenerationFields {
            schema: parsed.schema,
            camera: parsed.camera,
            widgets: parsed.widgets,
            synapses: parsed.synapses,
            layout: parsed.layout,
            selected_generation_id: parsed.selected_generation_id,
            preview_text: parsed.preview_text,
            generations: parsed.generations,
        })
    }
    fn host(snapshot: &Generation2dSnapshot) -> &FlowHostSnapshot {
        &snapshot.host_snapshot
    }
    fn state(snapshot: &Generation2dSnapshot) -> &GenerationPlayState {
        snapshot.generation.as_state()
    }
    fn assemble(host_snapshot: FlowHostSnapshot, generation: semio_framework_artifact_playbook_playbook::GenerationPlayRoot) -> Generation2dSnapshot {
        Generation2dSnapshot { host_snapshot, generation }
    }
}

pub fn encode_sqlite_native(document: &Generation2dSnapshot, encoding: semio_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut semio_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>, native_owner: &mut semio_os_kernel::NativeSnapshotEncodeOwner<'_, '_>) -> Result<store::io_schema::IoPayload, semio_framework_value::ValueError> {
    semio_s_procedural_core_generation_sqlite_native::encode::<Generation2dNative>(document, encoding, control, native_owner)
}

pub fn decode_sqlite_native(payload: &store::io_schema::IoPayload, control: &mut semio_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>, native_owner: &mut semio_os_kernel::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Generation2dSnapshot, semio_framework_value::ValueError> {
    semio_s_procedural_core_generation_sqlite_native::decode::<Generation2dNative>(payload, control, native_owner)
}
