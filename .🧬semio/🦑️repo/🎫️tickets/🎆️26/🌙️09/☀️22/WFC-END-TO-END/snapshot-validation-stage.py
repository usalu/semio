from pathlib import Path

p = Path('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🛂️validation/🦀️.rs')
s = p.read_text()
start = s.index('macro_rules! receive{')
end = s.index('receive!(decoding', start)
s = s[:start] + '''macro_rules! receive{
 ($name:ident,$owner:ident,$progress:path,$direction:ident,$phase:ident)=>{
 pub(super) fn $name(source:&SpaceHistorySnapshot,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut semio_framework_io_control::$owner<'_,'_>)->crate::lifted_serialization::SnapshotValidationOutcome{
  use semio_framework_io_control::NativeSnapshotFailure;
  use semio_framework_io_sqlite_snapshot::SqliteStageFailure as Stage;
  use semio_framework_value::RetainedWorkOutcome;
  type Inner=Stage<IoResult<()>,NativeSnapshotFailure>;
  let before=owner.native().owned_bytes();let Some(maximum)=before.checked_add(control.allocation_remaining_bytes())else{return RetainedWorkOutcome{result:Err(Stage::NotStarted(ValueError::literal(ValueRefusalKind::OwnershipLimit,"space history validation original SQL ceiling overflow"))),work:0}};
  let settlement=TypedFileSettlement::new(control,before,SqliteSnapshotPhase::$phase);let mut observer=|event:$progress|settlement.observe(event.owned_bytes,event.completed,event.total);
  let turn=owner.scoped_native(maximum,&mut observer,|owner|owner.receive::<Frame,std::result::Result<IoResult<()>,Inner>,NativeSnapshotFailure>(|slot,native,body|{
   let before_work=settlement.control.borrow().performed_work();
   let result=(||->std::result::Result<std::result::Result<IoResult<()>,Inner>,NativeSnapshotFailure>{
    let mut borrowed=settlement.control.borrow_mut();let control=&mut **borrowed;let mut port=Port{control,native:Direction::$direction(native),body};port.work(1,size_of::<Frame>())?;*slot=Some(Frame::new());let frame=slot.as_mut().unwrap();port.work(1,size_of::<IoResult<()>>())?;frame.result=Some(check(source,dialect,database,frame,&mut port));let owned=port.owned();drop(port);drop(borrowed);let status=frame.result.take().unwrap().map(|()|IoOutcome::clean(())).map_err(IoError::from_value_error);Ok(match settlement.finish_body(owned){Ok(())=>Ok(status),Err(refusal)=>Err(Stage::Settlement{operation:Ok(status),refusal})})
   })();
   RetainedWorkOutcome{result,work:settlement.control.borrow().performed_work().checked_sub(before_work).expect("original SQL producer work is monotone")}
  }));
  let operation=match turn.result{Ok(value)=>value,Err(error)=>Err(Stage::Operation(error))};
  let result=match settlement.cause.borrow_mut().take(){Some(refusal)=>Err(Stage::Settlement{operation,refusal}),None=>operation.map_err(Stage::Operation)};
  RetainedWorkOutcome{result,work:turn.work}
 }
 };
}
''' + s[end:]
p.write_text(s)
