from pathlib import Path

p = Path('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🛂️validation/🦀️.rs')
s = p.read_text()
start = s.index('macro_rules! receive{')
end = s.index('receive!(decoding', start)
s = s[:start] + '''macro_rules! receive{
 ($name:ident,$owner:ident,$progress:path,$direction:ident,$phase:ident)=>{
 pub(super) fn $name(source:&SpaceHistorySnapshot,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut semio_framework_io_control::$owner<'_,'_>)->semio_framework_value::RetainedWorkOutcome<IoResult<()>,semio_framework_io_control::NativeSnapshotFailure>{
  use semio_framework_io_control::{NativeSnapshotFailure,native_snapshot_turn};
  use semio_framework_value::RetainedWorkOutcome;
  native_snapshot_turn(|work|{
   let before=owner.native().owned_bytes();let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"space history validation original SQL ceiling overflow"))?;let settlement=TypedFileSettlement::new(control,before,SqliteSnapshotPhase::$phase);let mut observer=|event:$progress|settlement.observe(event.owned_bytes,event.completed,event.total);
   let turn=owner.scoped_native(maximum,&mut observer,|owner|owner.receive::<Frame,IoResult<()>,NativeSnapshotFailure>(|slot,native,body|{
    let before_work=settlement.control.borrow().performed_work();
    let result=(||->std::result::Result<IoResult<()>,NativeSnapshotFailure>{
     let mut borrowed=settlement.control.borrow_mut();let control=&mut **borrowed;let mut port=Port{control,native:Direction::$direction(native),body};port.work(1,size_of::<Frame>())?;*slot=Some(Frame::new());let frame=slot.as_mut().unwrap();port.work(1,size_of::<IoResult<()>>())?;frame.result=Some(check(source,dialect,database,frame,&mut port).map(|()|IoOutcome::clean(())).map_err(IoError::from_value_error));let owned=port.owned();drop(port);drop(borrowed);settlement.finish_body(owned)?;Ok(frame.result.take().unwrap())
    })();
    RetainedWorkOutcome{result,work:settlement.control.borrow().performed_work().checked_sub(before_work).expect("original SQL producer work is monotone")}
   }));
   let turn=RetainedWorkOutcome{result:turn.result.map_err(|error|settlement.refused_failure(owner.native().owned_bytes(),error)),work:turn.work};
   work.take(turn)
  })
 }
 };
}
''' + s[end:]
p.write_text(s)
