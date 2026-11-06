from pathlib import Path

p=Path('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/📥️lifted/🚪️io/🦀️.rs')
s=p.read_text()
start=s.index('/// 🧳️ Carries an original result')
end=s.index('/// 🪶️ A snapshot',start)
s=s[:start]+'''/// 🧳️ Retains the original result alongside an independent body settlement refusal.
pub type SnapshotReceivingOutput<T>=std::result::Result<T,semio_framework_io_sqlite_snapshot::SqliteStageFailure<T,NativeSnapshotFailure>>;
/// 🪙️ Carries the native turn and an independent observer refusal after any output handoff.
pub type SnapshotReceivingOutcome=RetainedWorkOutcome<(),semio_framework_io_sqlite_snapshot::SqliteStageFailure<(),NativeSnapshotFailure>>;
'''+s[end:]
s=s.replace("owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->SnapshotReceivingOutcome<semio_framework_io_sqlite_snapshot::SqliteDatabase>{crate::sqlite_snapshot_workspace::project(self,control,owner)}", "owner:&mut NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<SnapshotReceivingOutput<semio_framework_io_sqlite_snapshot::SqliteDatabase>>)->SnapshotReceivingOutcome{crate::sqlite_snapshot_workspace::project(self,control,owner,output)}")
s=s.replace("owner:&mut NativeSnapshotEncodeOwner<'_,'_>)->SnapshotReceivingOutcome<Self> {crate::sqlite_snapshot_workspace::reconstruct(database,control,owner)}", "owner:&mut NativeSnapshotEncodeOwner<'_,'_>,output:&mut Option<SnapshotReceivingOutput<Self>>)->SnapshotReceivingOutcome {crate::sqlite_snapshot_workspace::reconstruct(database,control,owner,output)}")
s=s.replace("native:&mut NativeSnapshotDecodeOwner<'_,'_>)->SnapshotValidationOutcome{crate::sqlite_snapshot_workspace::validate_decoding(self,dialect,database,control,native)}", "native:&mut NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<SnapshotReceivingOutput<semio_framework_io_schema::IoResult<()>>>)->SnapshotReceivingOutcome{crate::sqlite_snapshot_workspace::validate_decoding(self,dialect,database,control,native,output)}")
s=s.replace("native:&mut NativeSnapshotEncodeOwner<'_,'_>)->SnapshotValidationOutcome{crate::sqlite_snapshot_workspace::validate_encoding(self,dialect,database,control,native)}", "native:&mut NativeSnapshotEncodeOwner<'_,'_>,output:&mut Option<SnapshotReceivingOutput<semio_framework_io_schema::IoResult<()>>>)->SnapshotReceivingOutcome{crate::sqlite_snapshot_workspace::validate_encoding(self,dialect,database,control,native,output)}")
assert 'SnapshotValidationOutcome' not in s
p.write_text(s)

p=Path('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪆️workspace-receiving/🦀️.rs')
s=p.read_text()
start=s.index('/// 🗄️ Projects the snapshot')
s=s[:start]+'''fn publish<T>(output:&mut Option<crate::lifted_serialization::SnapshotReceivingOutput<T>>,turn:RetainedWorkOutcome<T,NativeSnapshotFailure>)->crate::lifted_serialization::SnapshotReceivingOutcome{
 use semio_framework_io_sqlite_snapshot::SqliteStageFailure as Stage;
 if output.is_some(){return RetainedWorkOutcome{result:Err(Stage::NotStarted(ValueError::literal(ValueRefusalKind::InvalidValue,"original snapshot output slot is occupied"))),work:0}}
 match turn.result{Ok(value)=>{*output=Some(Ok(value));RetainedWorkOutcome{result:Ok(()),work:turn.work}},Err(error)=>RetainedWorkOutcome{result:Err(Stage::Operation(error)),work:turn.work}}
}
/// 🗄️ Projects the snapshot through its authored relational model under the original decode owner.
pub fn project<P:ArtifactSqliteSnapshotHost>(snapshot:&P,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<crate::lifted_serialization::SnapshotReceivingOutput<SqliteDatabase>>)->crate::lifted_serialization::SnapshotReceivingOutcome{
 if output.is_some(){return publish(output,RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original snapshot output slot is occupied").into()),work:0})}
 let turn=receiving_allocation!(owner,control,|child| snapshot.to_sqlite_database(child));publish(output,turn)
}
/// 🪆️ Reconstructs the snapshot from its authored relational model under the original encode owner.
pub fn reconstruct<P:ArtifactSqliteSnapshotHost>(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotEncodeOwner<'_,'_>,output:&mut Option<crate::lifted_serialization::SnapshotReceivingOutput<P>>)->crate::lifted_serialization::SnapshotReceivingOutcome{
 if output.is_some(){return publish(output,RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original snapshot output slot is occupied").into()),work:0})}
 let turn=receiving_allocation!(owner,control,|child| P::from_sqlite_database(database,child));publish(output,turn)
}
macro_rules! validate{
 ($name:ident,$owner:ident)=>{
 pub fn $name<P:ArtifactSqliteSnapshotHost>(snapshot:&P,dialect:&ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut $owner<'_,'_>,output:&mut Option<crate::lifted_serialization::SnapshotReceivingOutput<IoResult<()>>>)->crate::lifted_serialization::SnapshotReceivingOutcome{
  if output.is_some(){return publish(output,RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original snapshot output slot is occupied").into()),work:0})}
  if control.allocation_remaining_bytes()==0{
   let before=control.performed_work();let result=owner.native().checkpoint().map(|()|snapshot.validate_sqlite_snapshot_subset(dialect,database,control)).map_err(Into::into);
   return publish(output,RetainedWorkOutcome{result,work:control.performed_work().checked_sub(before).expect("original SQL producer work is monotone")})
  }
  let turn=receiving_allocation!(owner,control,|child| Ok::<IoResult<()>,ValueError>(snapshot.validate_sqlite_snapshot_subset(dialect,database,child)));publish(output,turn)
 }
 };
}
validate!(validate_decoding,NativeSnapshotDecodeOwner);
validate!(validate_encoding,NativeSnapshotEncodeOwner);
'''
p.write_text(s)
