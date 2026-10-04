//! 🪶️ Playground's literal document entity and controlled native boundary.
use super::PlaygroundSnapshot;
use store::{
    sqlite_snapshot::{
        artifact::{Cell, Projection, Reconstruction},
        validate_sqlite_database_schema, SnapshotEncoding, SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase,
    },
    ArtifactSqliteSnapshot,
};
use semio_framework_value::ValueError;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}


fn admit(control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<(), ValueError> {
    control.checkpoint(phase, 0, 1)?;
    if PlaygroundSnapshot::SQLITE_SCHEMA.len() > control.limits().max_schema_bytes {
        return Err(invalid("Playground SQL exceeds schema byte limit"));
    }
    control.check_rows(1)
}

impl ArtifactSqliteSnapshot for PlaygroundSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        admit(control, SqliteSnapshotPhase::DecodeNative)?;
        store::decode_sqlite_snapshot_record_native(payload, <Self as store::ArtifactDsl>::envelope_id(), Self::__dsl_spec_producer(), |record,native|Self::__dsl_from_record_controlled(record,native), control)
    }
    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<store::io_schema::IoPayload, ValueError> {
        admit(control, SqliteSnapshotPhase::EncodeNative)?;
        store::encode_sqlite_snapshot_record_native(encoding, <Self as store::ArtifactDsl>::envelope_id(), Self::__dsl_spec_producer(), |native| self.__dsl_to_record_controlled(native), control)
    }
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
        admit(control, SqliteSnapshotPhase::ProjectSnapshot)?;
        let mut projection = Projection::new(Self::SQLITE_SCHEMA, control)?;
        projection.insert("playground_document", &[Cell::Text(&self.schema)])?;
        projection.finish()
    }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        admit(control, SqliteSnapshotPhase::ReconstructSnapshot)?;
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
