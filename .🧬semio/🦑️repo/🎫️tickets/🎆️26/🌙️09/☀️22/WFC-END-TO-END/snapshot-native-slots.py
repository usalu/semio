from pathlib import Path

p=Path('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/📥️lifted/🚪️io/🦀️.rs')
s=p.read_text().replace('NativeSnapshotFailure, native_snapshot_turn','NativeSnapshotFailure')
start=s.index('    fn decode_sqlite_snapshot_native(')
end=s.index('    /// 🚧️ Checks',start)
s=s[:start]+'''    fn decode_sqlite_snapshot_native(_payload:&semio_framework_io_schema::IoPayload,control:&mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>,native_owner:&mut NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<SnapshotReceivingOutput<Self>>)->SnapshotReceivingOutcome {
        use semio_framework_io_sqlite_snapshot::SqliteStageFailure as Stage;
        let before=control.performed_work();let result=(||{if output.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original snapshot output slot is occupied"))}native_owner.native().checkpoint()?;control.checkpoint(semio_framework_io_sqlite_snapshot::SqliteSnapshotPhase::DecodeNative,0,1)?;Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"snapshot owner has no controlled native decoding implementation"))})();RetainedWorkOutcome{result:result.map_err(Stage::NotStarted),work:control.performed_work().checked_sub(before).expect("original SQL producer work is monotone")}
    }

    /// 🛫️ Emits native state through an owner-declared bounded and cancellable encoder.
    fn encode_sqlite_snapshot_native(&self,_encoding:semio_framework_io_sqlite_snapshot::SnapshotEncoding,control:&mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>,native_owner:&mut NativeSnapshotEncodeOwner<'_,'_>,output:&mut Option<SnapshotReceivingOutput<semio_framework_io_schema::IoPayload>>)->SnapshotReceivingOutcome {
        use semio_framework_io_sqlite_snapshot::SqliteStageFailure as Stage;
        let before=control.performed_work();let result=(||{if output.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"original snapshot output slot is occupied"))}native_owner.native().checkpoint()?;control.checkpoint(semio_framework_io_sqlite_snapshot::SqliteSnapshotPhase::EncodeNative,0,1)?;Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"snapshot owner has no controlled native encoding implementation"))})();RetainedWorkOutcome{result:result.map_err(Stage::NotStarted),work:control.performed_work().checked_sub(before).expect("original SQL producer work is monotone")}
    }

'''+s[end:]
p.write_text(s)
