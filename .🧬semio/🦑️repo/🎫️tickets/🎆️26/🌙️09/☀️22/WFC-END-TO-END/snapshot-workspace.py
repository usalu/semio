from pathlib import Path

p = Path('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪆️workspace-receiving/🦀️.rs')
s = p.read_text().replace('use semio_framework_io_control::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};','use semio_framework_io_control::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner, NativeSnapshotFailure, native_snapshot_turn};').replace('use semio_framework_value::{ValueError, ValueRefusalKind};','use semio_framework_value::{ValueError, ValueRefusalKind, RetainedWorkOutcome};')
start = s.index('macro_rules! receiving_allocation')
s = s[:start] + '''macro_rules! receiving_allocation{
 ($owner:ident,$control:ident,|$child:ident| $body:expr)=>{{
  native_snapshot_turn(|work|{
   let remaining=$control.allocation_remaining_bytes();if remaining==0{return Err(unbacked().into())}
   let maximum=$owner.native().owned_bytes().checked_add(remaining).ok_or_else(overflow)?;
   let turn=$owner.scoped_native(maximum,&mut |_|true,|$owner|{
    let before=$control.performed_work();
    let result=(||{$owner.native().checkpoint()?;$owner.native().with_encoding_receiver(|room,_observer,allocation|$control.with_original_allocation(room,allocation,|$child|$body))})();
    RetainedWorkOutcome{result,work:$control.performed_work().checked_sub(before).expect("original SQL producer work is monotone")}
   });
   work.take(turn)
  })
 }};
}

/// 🗄️ Projects the snapshot through its authored relational model under the original decode owner.
pub fn project<P:ArtifactSqliteSnapshotHost>(snapshot:&P,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<SqliteDatabase,NativeSnapshotFailure>{
 receiving_allocation!(owner,control,|child| snapshot.to_sqlite_database(child))
}

/// 🪆️ Reconstructs the snapshot from its authored relational model under the original encode owner.
pub fn reconstruct<P:ArtifactSqliteSnapshotHost>(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut NativeSnapshotEncodeOwner<'_,'_>)->RetainedWorkOutcome<P,NativeSnapshotFailure>{
 receiving_allocation!(owner,control,|child| P::from_sqlite_database(database,child))
}

macro_rules! validate{
 ($name:ident,$owner:ident)=>{
 pub fn $name<P:ArtifactSqliteSnapshotHost>(snapshot:&P,dialect:&ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut $owner<'_,'_>)->RetainedWorkOutcome<IoResult<()>,NativeSnapshotFailure>{
  if control.allocation_remaining_bytes()==0{
   let before=control.performed_work();let result=owner.native().checkpoint().map(|()|snapshot.validate_sqlite_snapshot_subset(dialect,database,control)).map_err(Into::into);
   return RetainedWorkOutcome{result,work:control.performed_work().checked_sub(before).expect("original SQL producer work is monotone")}
  }
  receiving_allocation!(owner,control,|child| Ok::<IoResult<()>,ValueError>(snapshot.validate_sqlite_snapshot_subset(dialect,database,child)))
 }
 };
}
validate!(validate_decoding,NativeSnapshotDecodeOwner);
validate!(validate_encoding,NativeSnapshotEncodeOwner);
'''
s = s.replace('use semio_framework_io_schema::{IoError, IoResult};','use semio_framework_io_schema::IoResult;')
p.write_text(s)
p = Path('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/📥️lifted/🚪️io/🦀️.rs')
s = p.read_text()
s = s.replace(')->semio_framework_io_schema::IoResult<()>{crate::sqlite_snapshot_workspace::validate_', ')->RetainedWorkOutcome<semio_framework_io_schema::IoResult<()>,NativeSnapshotFailure>{crate::sqlite_snapshot_workspace::validate_')
p.write_text(s)
