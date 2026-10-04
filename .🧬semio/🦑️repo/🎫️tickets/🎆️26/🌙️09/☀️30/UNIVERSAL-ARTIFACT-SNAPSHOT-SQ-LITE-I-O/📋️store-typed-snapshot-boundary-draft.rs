use super::*;
/// 🪶️ A snapshot's handwritten relational model, independently interpretable through SQLite.
pub trait ArtifactSqliteSnapshot: Sized {
    const SQLITE_SCHEMA: &'static str;
    fn to_sqlite_database(&self, control: &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<crate::sqlite_snapshot::SqliteDatabase,ValueError>;
    fn from_sqlite_database(database: &crate::sqlite_snapshot::SqliteDatabase, control: &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self,ValueError>;

    /// ♻️ Retires a snapshot through its declared owner after an erased conversion.
    fn retire_sqlite_snapshot(self) { drop(self); }

    /// 🛬️ Materializes native snapshot fields under the caller's limits and cancellation control.
    fn decode_sqlite_snapshot_native(_payload: &crate::io_schema::IoPayload, control: &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self,ValueError> {
        control.checkpoint(crate::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative, 0, 1)?;
        Err(ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"snapshot owner has no controlled native decoding implementation"))
    }

    /// 🛫️ Emits native state through an owner-declared bounded and cancellable encoder.
    fn encode_sqlite_snapshot_native(&self,_encoding:crate::sqlite_snapshot::SnapshotEncoding,control:&mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<crate::io_schema::IoPayload,ValueError>{control.checkpoint(crate::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative,0,1)?;Err(ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"snapshot owner has no controlled native encoding implementation"))}

    /// 🚧️ Checks the owner's native encoding expansion before printing or packing any fields.
    fn preflight_sqlite_snapshot_encoding(&self, _encoding: crate::sqlite_snapshot::SnapshotEncoding, control: &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<(),ValueError> {
        control.checkpoint(crate::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative, 0, 1)?;
        Err(ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"snapshot owner has no bounded native encoding preflight"))
    }

    /// 🛡️ Validates the exact owned subset without lowering fields into a native wire format.
    fn validate_sqlite_snapshot_subset(&self, dialect: &crate::io_schema::ArtifactDialect, _database: &crate::sqlite_snapshot::SqliteDatabase, control: &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>) -> crate::io_schema::IoResult<()> {
        control.checkpoint(crate::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot, 0, 0).map_err(crate::io_schema::IoError::from_value_error)?;
        if dialect.subset == "*" { return Ok(crate::io_schema::IoOutcome::clean(())); }
        Err(crate::io_schema::IoError::from_value_error(ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,format!("owned snapshot subset {} has no semantic validator", dialect.to_coordinate()))))
    }

    /// 🪶️ Builds the relational codec only when the snapshot actually implements it.
    fn sqlite_codec() -> ArtifactSqliteSnapshotCodec
    where
        Self: ArtifactDsl + ArtifactPack + 'static,
    {
        fn export_snapshot_impl<P: ArtifactDsl + ArtifactPack + ArtifactSqliteSnapshot>(_schema: &str, dialect: &crate::io_schema::ArtifactDialect, payload: &crate::io_schema::IoPayload, control: &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>) -> crate::io_schema::IoResult<crate::sqlite_snapshot::SqliteDatabase> {
            control.checkpoint(crate::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative, 0, 1).map_err(crate::io_schema::IoError::from_value_error)?;
            let snapshot = sqlite_snapshot_retirement::OwnedSqliteSnapshot::new(P::decode_sqlite_snapshot_native(payload, control).map_err(crate::io_schema::IoError::from_value_error)?);
            control.checkpoint(crate::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative, 1, 1).map_err(crate::io_schema::IoError::from_value_error)?;
            let database = snapshot.to_sqlite_database(control).map_err(crate::io_schema::IoError::from_value_error)?;
            control.check_database(&database, crate::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot).map_err(crate::io_schema::IoError::from_value_error)?;
            let validation = validate_owned_sqlite_snapshot_subset(&*snapshot, dialect, &database, control)?;
            Ok(crate::io_schema::IoOutcome { value: database, diagnostics: validation.diagnostics })
        }

        fn import_snapshot_impl<P: ArtifactDsl + ArtifactPack + ArtifactSqliteSnapshot>(_schema: &str, dialect: &crate::io_schema::ArtifactDialect, database: crate::sqlite_snapshot::SqliteDatabase, encoding: crate::sqlite_snapshot::SnapshotEncoding, control: &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>) -> crate::io_schema::IoResult<crate::io_schema::IoPayload> {
            let snapshot = sqlite_snapshot_retirement::OwnedSqliteSnapshot::new(P::from_sqlite_database(&database, control).map_err(crate::io_schema::IoError::from_value_error)?);
            let validation = validate_owned_sqlite_snapshot_subset(&*snapshot, dialect, &database, control)?;
            control.checkpoint(crate::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative, 0, 1).map_err(crate::io_schema::IoError::from_value_error)?;
            let payload = snapshot.encode_sqlite_snapshot_native(encoding, control).map_err(crate::io_schema::IoError::from_value_error)?;
            control.checkpoint(crate::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative, 1, 1).map_err(crate::io_schema::IoError::from_value_error)?;
            Ok(crate::io_schema::IoOutcome { value: payload, diagnostics: validation.diagnostics })
        }

        ArtifactSqliteSnapshotCodec { schema: std::borrow::Cow::Borrowed(Self::SQLITE_SCHEMA), snapshot_type: Some(std::any::TypeId::of::<Self>()), export: export_snapshot_impl::<Self>, import: import_snapshot_impl::<Self> }
    }
}

/// 🛡️ Applies the owner's semantic policy and preserves warnings while refusing errors.
pub fn validate_owned_sqlite_snapshot_subset<P: ArtifactSqliteSnapshot>(snapshot: &P, dialect: &crate::io_schema::ArtifactDialect, database: &crate::sqlite_snapshot::SqliteDatabase, control: &mut crate::sqlite_snapshot::SqliteSnapshotControl<'_>) -> crate::io_schema::IoResult<()> {
    let outcome = snapshot.validate_sqlite_snapshot_subset(dialect, database, control)?;
    if outcome.diagnostics.iter().any(|diagnostic| matches!(diagnostic.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
        return Err(crate::io_schema::IoError { cause: ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("owned snapshot violates subset {}", dialect.to_coordinate())), diagnostics: outcome.diagnostics });
    }
    Ok(outcome)
}

