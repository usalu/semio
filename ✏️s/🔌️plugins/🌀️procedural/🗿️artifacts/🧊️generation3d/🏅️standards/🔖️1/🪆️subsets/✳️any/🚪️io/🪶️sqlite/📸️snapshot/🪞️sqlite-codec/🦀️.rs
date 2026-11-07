//! 🧊️ Owned Generation3d retained field contract and exact declared SQLite capability.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::snapshot::Generation3dSnapshot;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_os_kernel::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding}};
use semio_s_procedural_core_generation_sqlite as typed;
impl ArtifactSqliteSnapshotHost for Generation3dSnapshot{
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->semio_framework_value::RetainedWorkOutcome<Self,semio_os_kernel::NativeSnapshotFailure> {semio_os_kernel::native_snapshot_turn(|work|{c.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;if Self::SQLITE_SCHEMA.len()>c.limits().max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Generation authored schema exceeds caller limit").into())};work.take(decode_sqlite_native(payload,c,native_owner))})}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->semio_framework_value::RetainedWorkOutcome<store::io_schema::IoPayload,semio_os_kernel::NativeSnapshotFailure> {semio_os_kernel::native_snapshot_turn(|work|{c.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;if Self::SQLITE_SCHEMA.len()>c.limits().max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Generation authored schema exceeds caller limit").into())};work.take(encode_sqlite_native(self,encoding,c,native_owner))})}
 
 fn retire_sqlite_snapshot(self){self.retire_cold();}
 
 
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{typed::preflight(&self.host_snapshot,&self.generation,c)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1).map_err(|error|store::io_schema::IoError::from_value_error(error))?;if dialect.artifact_kind!="s.procedural.generation3d"||dialect.standard!="1"||dialect.subset!="*"{return Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"Generation3d dialect differs from its owned wildcard")))}if d.table("generation_host").map_err(|error|store::io_schema::IoError::from_value_error(error))?.single_row().map_err(|error|store::io_schema::IoError::from_value_error(error))?.text(1).map_err(|error|store::io_schema::IoError::from_value_error(error))?!=self.host_snapshot.schema{return Err(store::io_schema::IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue,"Generation3d projected host schema differs")))}c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1).map_err(|error|store::io_schema::IoError::from_value_error(error))?;Ok(store::io_schema::IoOutcome::clean(()))}
}
impl pack::snapshot::ArtifactSqliteSnapshot for Generation3dSnapshot {
const SQLITE_SCHEMA:&'static str=include_str!("../🗄️.sql");
fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{typed::project(&self.host_snapshot,&self.generation,Self::SQLITE_SCHEMA,c)}
fn from_sqlite_database(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let(host_snapshot,generation)=typed::reconstruct(d,Self::SQLITE_SCHEMA,c)?;Ok(Self{host_snapshot,generation})}
}

use crate::dsl_mirror::Generation3dSnapshotDsl;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationPlayState;
use semio_os_kernel as store;

struct Generation3dNative;

impl semio_s_procedural_core_generation_sqlite_native::GenerationNative for Generation3dNative {
    type Snapshot = Generation3dSnapshot;
    const ENVELOPE_ID: &'static str = "procedural.generation3d";
    const PACK_PREFIX: Option<&'static [u8]> = None;
    fn spec_producer() -> semio_framework_dsl_record::RecordSpecProducer {
        Generation3dSnapshotDsl::__dsl_spec_producer()
    }
    fn fields(record: &semio_framework_dsl_record::RecordValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<semio_s_procedural_core_generation_sqlite_native::GenerationFields, semio_framework_value::ValueError> {
        let parsed = Generation3dSnapshotDsl::__dsl_from_record_controlled(record, control)?;
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
    fn host(snapshot: &Generation3dSnapshot) -> &FlowHostSnapshot {
        &snapshot.host_snapshot
    }
    fn state(snapshot: &Generation3dSnapshot) -> &GenerationPlayState {
        snapshot.generation.as_state()
    }
    fn assemble(host_snapshot: FlowHostSnapshot, generation: semio_framework_artifact_playbook_playbook::GenerationPlayRoot) -> Generation3dSnapshot {
        Generation3dSnapshot { host_snapshot, generation }
    }
}

pub fn encode_sqlite_native(document: &Generation3dSnapshot, encoding: semio_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut semio_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>, native_owner: &mut semio_os_kernel::NativeSnapshotEncodeOwner<'_, '_>) -> semio_framework_value::RetainedWorkOutcome<store::io_schema::IoPayload, semio_os_kernel::NativeSnapshotFailure> {
    semio_s_procedural_core_generation_sqlite_native::encode::<Generation3dNative>(document, encoding, control, native_owner)
}

pub fn decode_sqlite_native(payload: &store::io_schema::IoPayload, control: &mut semio_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>, native_owner: &mut semio_os_kernel::NativeSnapshotDecodeOwner<'_, '_>) -> semio_framework_value::RetainedWorkOutcome<Generation3dSnapshot, semio_os_kernel::NativeSnapshotFailure> {
    semio_s_procedural_core_generation_sqlite_native::decode::<Generation3dNative>(payload, control, native_owner)
}
