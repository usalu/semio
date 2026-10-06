//! 🪶️ Playground's literal document entity and controlled native boundary.
use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use store::{sqlite_snapshot::{artifact::{Cell,RowWriter,Reconstruction},validate_sqlite_database_schema,SnapshotEncoding,SqliteDatabase,SqliteDatabaseLimits,SqliteSnapshotControl,SqliteSnapshotPhase},ArtifactSqliteSnapshot};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn extent(limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 if PlaygroundSnapshot::SQLITE_SCHEMA.len()>limits.max_schema_bytes||limits.max_tables<1||limits.max_columns<2||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Playground authored schema extent exceeds caller limits"))}Ok(())
}
fn visit(schema:&str,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{p.insert("playground_document",&[Cell::Text(schema)])?;Ok(())}
fn admit(snapshot:&PlaygroundSnapshot,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{extent(c.limits())?;let mut p=RowWriter::borrowed(c,SqliteSnapshotPhase::ProjectSnapshot)?;visit(&snapshot.schema,&mut p)?;p.finish_borrowed()}
fn admit_record(record:&semio_framework_dsl_record::RecordValue,limits:SqliteDatabaseLimits,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{
 extent(limits)?;n.step()?;if record.fields.len()!=1{return Err(invalid("Playground native record field map differs"))}let Some(semio_framework_dsl_record::FieldValue::Text(schema))=record.fields.get(&0)else{return Err(invalid("Playground native schema requires text"))};let bytes=8usize.checked_add(schema.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Playground semantic extent overflow"))?;if bytes>limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Playground semantic value limit exceeded"))}Ok(())
}

impl ArtifactSqliteSnapshot for PlaygroundSnapshot {
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{admit(self,c)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let limits=c.limits();extent(limits)?;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,n|{admit_record(record,limits,n)?;Self::__dsl_from_record_controlled(record,n)},c)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{admit(self,c)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|n|self.__dsl_to_record_controlled(n),c)}
 fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{extent(c.limits())?;let mut p=RowWriter::new(Self::SQLITE_SCHEMA,c)?;visit(&self.schema,&mut p)?;p.finish()}
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        extent(control.limits())?;
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let document = database.table("playground_document")?.single_row()?;
        if document.values.len() != 2 || document.rowid <= 0 || document.integer(0)? != document.rowid {
            return Err(invalid("Playground requires one positive document identity and its literal schema"));
        }
        let mut reconstruction = Reconstruction::new(control)?;
        let schema = reconstruction.text(document.text(1)?)?;
        reconstruction.checkpoint()?;
        Ok(Self { schema })
    }
    fn validate_sqlite_snapshot_subset(&self, dialect: &store::io_schema::ArtifactDialect, database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1).map_err(store::io_schema::IoError::from_value_error)?;
        if dialect.artifact_kind != "s.demonstrator.playground" || dialect.standard != "1" || dialect.subset != "*" {
            return Err(store::io_schema::IoError::from_value_error(invalid("Playground dialect differs from its owned coordinate")));
        }
        if database.table("playground_document").map_err(store::io_schema::IoError::from_value_error)?.single_row().map_err(store::io_schema::IoError::from_value_error)?.text(1).map_err(store::io_schema::IoError::from_value_error)? != self.schema {
            return Err(store::io_schema::IoError::from_value_error(invalid("Playground projected schema differs from its snapshot")));
        }
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 1, 1).map_err(store::io_schema::IoError::from_value_error)?;
        Ok(store::io_schema::IoOutcome::clean(()))
    }
    fn retire_sqlite_snapshot(self) {
        drop(self.schema);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

