from pathlib import Path

p=Path('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪆️workspace-receiving/🦀️.rs')
s=p.read_text().replace('NativeSnapshotFailure, native_snapshot_turn','NativeSnapshotFailure')
start=s.index('macro_rules! receiving_allocation')
end=s.index('/// 🗄️ Projects',start)
s=s[:start]+'''macro_rules! receiving_allocation{
 ($owner:ident,$control:ident,$output:ident,|$child:ident| $body:expr)=>{{
  use semio_framework_io_sqlite_snapshot::SqliteStageFailure as Stage;
  if $output.is_some(){RetainedWorkOutcome{result:Err(Stage::NotStarted(ValueError::literal(ValueRefusalKind::InvalidValue,"original snapshot output slot is occupied"))),work:0}}
  else if $control.allocation_remaining_bytes()==0{RetainedWorkOutcome{result:Err(Stage::NotStarted(unbacked())),work:0}}
  else if let Some(maximum)=$owner.native().owned_bytes().checked_add($control.allocation_remaining_bytes()){
   let turn=$owner.scoped_native(maximum,&mut |_|true,|$owner|$owner.receive::<(),_,NativeSnapshotFailure>($output,&mut |_frame,output,native,_body|{
    let before=$control.performed_work();
    let result:Result<(),ValueError>=(||{native.checkpoint()?;native.with_encoding_receiver(|room,_observer,allocation|$control.with_original_allocation(room,allocation,|$child|{*output=Some($body.map_err(|error:ValueError|Stage::Operation(error.into())));Ok(())}))})();
    RetainedWorkOutcome{result,work:$control.performed_work().checked_sub(before).expect("original SQL producer work is monotone")}
   }));
   RetainedWorkOutcome{result:turn.result.map_err(Stage::Operation),work:turn.work}
  }else{RetainedWorkOutcome{result:Err(Stage::NotStarted(overflow())),work:0}}
 }};
}
''' + s[end:]
start=s.index('pub fn project<')
end=s.index('macro_rules! validate',start)
s=s[:start]+'''pub fn project<P:ArtifactSqliteSnapshotHost>(snapshot:&P,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<crate::lifted_serialization::SnapshotReceivingOutput<SqliteDatabase>>)->crate::lifted_serialization::SnapshotReceivingOutcome{
 receiving_allocation!(owner,control,output,|child| snapshot.to_sqlite_database(child))
}
/// 🪆️ Reconstructs the snapshot from its authored relational model under the original encode owner.
pub fn reconstruct<P:ArtifactSqliteSnapshotHost>(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotEncodeOwner<'_,'_>,output:&mut Option<crate::lifted_serialization::SnapshotReceivingOutput<P>>)->crate::lifted_serialization::SnapshotReceivingOutcome{
 receiving_allocation!(owner,control,output,|child| P::from_sqlite_database(database,child))
}
''' +s[end:]
start=s.index('  if output.is_some()', s.index('macro_rules! validate'))
end=s.index('\n }\n };',start)
s=s[:start]+'''  receiving_allocation!(owner,control,output,|child| Ok::<IoResult<()>,ValueError>(snapshot.validate_sqlite_snapshot_subset(dialect,database,child)))''' +s[end:]
p.write_text(s)
p=Path('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/📥️lifted/🚪️io/🦀️.rs')
s=p.read_text().replace('pub trait ArtifactSqliteSnapshotHost: pack::snapshot::ArtifactSqliteSnapshot {','pub trait ArtifactSqliteSnapshotHost: pack::snapshot::ArtifactSqliteSnapshot + semio_framework_value::retirement::RetireOwned {')
p.write_text(s)
