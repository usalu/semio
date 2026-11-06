from pathlib import Path

p=Path('🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🛂️validation/🦀️.rs')
s=p.read_text()
s=s.replace('StageFailure::Settlement{operation:Ok(status),refusal}', 'StageFailure::Settlement{operation:Ok(status),refusal,receipt:None}')
start=s.index('macro_rules! receive{')
end=s.index('receive!(decoding',start)
s=s[:start]+'''macro_rules! receive{
 ($name:ident,$owner:ident,$progress:path,$direction:ident,$phase:ident)=>{
 pub(super) fn $name(source:&SpaceHistorySnapshot,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,owner:&mut semio_framework_io_control::$owner<'_,'_>,output:&mut Option<crate::lifted_serialization::SnapshotReceivingOutput<IoResult<()>>>)->crate::lifted_serialization::SnapshotReceivingOutcome{
  use semio_framework_io_control::NativeSnapshotFailure;
  use semio_framework_io_sqlite_snapshot::SqliteStageFailure as Stage;
  use semio_framework_value::RetainedWorkOutcome;
  if output.is_some(){return RetainedWorkOutcome{result:Err(Stage::NotStarted(ValueError::literal(ValueRefusalKind::InvalidValue,"original validation output slot is occupied"))),work:0}}
  let before=owner.native().owned_bytes();let Some(maximum)=before.checked_add(control.allocation_remaining_bytes())else{return RetainedWorkOutcome{result:Err(Stage::NotStarted(ValueError::literal(ValueRefusalKind::OwnershipLimit,"space history validation original SQL ceiling overflow"))),work:0}};
  let settlement=TypedFileSettlement::new(control,before,SqliteSnapshotPhase::$phase);let mut observer=|event:$progress|settlement.observe(event.owned_bytes,event.completed,event.total);
  let turn=owner.scoped_native(maximum,&mut observer,|owner|owner.receive::<Frame,crate::lifted_serialization::SnapshotReceivingOutput<IoResult<()>>,NativeSnapshotFailure>(output,&mut |slot,result,native,body|{
   let before_work=settlement.control.borrow().performed_work();
   let outcome=(||->std::result::Result<(),NativeSnapshotFailure>{
    let mut borrowed=settlement.control.borrow_mut();let control=&mut **borrowed;let mut port=Port{control,native:Direction::$direction(native),body};port.work(1,size_of::<Frame>())?;*slot=Some(Frame::new());let frame=slot.as_mut().unwrap();port.work(1,size_of::<IoResult<()>>())?;frame.result=Some(check(source,dialect,database,frame,&mut port));let owned=port.owned();drop(port);drop(borrowed);let status=frame.result.take().unwrap().map(|()|IoOutcome::clean(())).map_err(IoError::from_value_error);*result=Some(finish_validation(status,&settlement,owned));Ok(())
   })();
   RetainedWorkOutcome{result:outcome,work:settlement.control.borrow().performed_work().checked_sub(before_work).expect("original SQL producer work is monotone")}
  }));
  let result=match settlement.cause.borrow_mut().take(){Some(refusal)=>Err(Stage::Settlement{operation:turn.result,refusal,receipt:None}),None=>turn.result.map_err(Stage::Operation)};
  RetainedWorkOutcome{result,work:turn.work}
 }
 };
}
'''+s[end:]
s=s.replace('SqliteStageFailure::Settlement{operation:Ok(Err(authored)),refusal}', 'SqliteStageFailure::Settlement{operation:Ok(Err(authored)),refusal,..}')
p.write_text(s)
