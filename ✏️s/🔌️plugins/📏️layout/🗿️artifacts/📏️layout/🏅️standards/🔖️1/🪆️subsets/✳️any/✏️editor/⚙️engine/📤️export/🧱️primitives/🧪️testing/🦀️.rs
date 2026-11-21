//! 🧪️ Original job fixtures for the retained export primitives.
use crate::{output::{ChunkRope,OutputWork,OUTPUT_BODY_BYTES},plan_storage::{self,PlanOffsets}};
use semio_framework_job::{Generation,Operation,OriginalJobFixtureCaller,RetainedCloneGrant};
use semio_plugin::Fault;

/// 🎟️ The one stock every fixture caller of the batch driver is born with.
pub const LAYOUT_EXPORT_FIXTURE_STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: usize::MAX / 4, maximum_copy_bytes: usize::MAX / 4, maximum_capacity_bytes: usize::MAX / 4, maximum_release_bytes: usize::MAX / 4, maximum_depth: 64 };

/// 🎟️ The grant of one fixture turn: wide enough for a complete publication frontier, narrow enough to stay bounded.
pub const LAYOUT_EXPORT_FIXTURE_TURN: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 28, maximum_capacity_bytes: 1 << 28, maximum_release_bytes: 1 << 28, maximum_depth: 32 };

/// 🚧️ How many turns this oracle gives one ladder before it calls the ladder broken. A close ladder that answers `Blocked` forever turns `while !terminal_is_empty()` into a livelock that the 30-minute test-binary watchdog turns into a lost binary; bounded here, a stuck ladder is a named failing test.
pub const LAYOUT_EXPORT_FIXTURE_TURN_BUDGET: usize = 100_000;

/// 🎟️ Admits a fixture caller whose identity is the exact operation and generation a job will run under.
pub fn layout_export_fixture_caller(operation: Operation, generation: Generation) -> OriginalJobFixtureCaller {
    OriginalJobFixtureCaller::admit(operation.operation, generation, LAYOUT_EXPORT_FIXTURE_STOCK, semio_framework_job::default_now_us, u64::MAX, u64::MAX)
}

/// 🧪️ Test bytes enter the real output owner using the same admitted original caller as its later close.
pub fn append_output_fixture(output:&mut ChunkRope,source:&[u8],caller:&mut OriginalJobFixtureCaller){for bytes in source.chunks(OUTPUT_BODY_BYTES){for turn in 0..LAYOUT_EXPORT_FIXTURE_TURN_BUDGET{if output.has_room(bytes.len()){break;}caller.turn(LAYOUT_EXPORT_FIXTURE_TURN,|cx|{let grant=cx.retained_grant();let Some((result,work))=cx.with_original_effect(grant,|_,effect|output.prepare_one(bytes.len(),effect))?else{return Ok(())};cx.consume_work(work);let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_retained(progress)?;result?;Ok(())}).expect("original output fixture backing");assert!(turn+1<LAYOUT_EXPORT_FIXTURE_TURN_BUDGET);}caller.turn(LAYOUT_EXPORT_FIXTURE_TURN,|cx|{let mut work=OutputWork::new(cx.retained_grant());let result=output.append(bytes,&mut work);cx.consume_retained(work.receipt())?;result.map_err(Fault::from)?;Ok(())}).expect("original output fixture payload");}}

/// ♻️ Tests retire every physical byte page using the exact original writer caller.
pub fn close_output_fixture(output:&mut ChunkRope,caller:&mut OriginalJobFixtureCaller){for turn in 0..LAYOUT_EXPORT_FIXTURE_TURN_BUDGET{if output.terminal_is_empty(){return;}advance_output_close_fixture(output,caller,LAYOUT_EXPORT_FIXTURE_TURN);assert!(turn+1<LAYOUT_EXPORT_FIXTURE_TURN_BUDGET);}panic!("original output fixture close did not finish");}

/// ⏱️ Executes one close turn with the original caller and posts its physical and Work receipts.
pub fn advance_output_close_fixture(output:&mut ChunkRope,caller:&mut OriginalJobFixtureCaller,grant:RetainedCloneGrant)->semio_framework_value::RetainedCloneStep{
    use semio_framework_value::RetainedCloneStep;
    caller.turn(grant,|cx|{
        let grant=cx.retained_grant();
        let Some((result,work))=cx.with_original_effect(grant,|_,effect|output.close_one(effect))?else{return Ok(RetainedCloneStep::Progress(Default::default()))};
        cx.consume_work(work);
        let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
        cx.consume_retained(progress)?;
        Ok(result?)
    }).expect("original output fixture close")
}

/// 🚧️ Tests observe the real append refusal without replacing the original caller or output owner.
pub fn output_append_refused_fixture(output:&mut ChunkRope,source:&[u8],caller:&mut OriginalJobFixtureCaller)->bool{caller.turn(LAYOUT_EXPORT_FIXTURE_TURN,|cx|{let mut work=OutputWork::new(cx.retained_grant());let result=output.append(source,&mut work);cx.consume_retained(work.receipt())?;Ok(result.is_err())}).expect("original output fixture refusal")}

/// 🗃️ Fixture rows move only after the original caller funds their backing and metadata copy.
pub fn append_plan_fixture<T:semio_framework_value::retirement::RetireOwned>(owner:&mut plan_storage::PlanList<T>,value:T){let operation=Operation::new(semio_framework_job::OperationId(918),semio_framework_job::RevisionId(1),semio_framework_job::Generation(1),0);let mut caller=layout_export_fixture_caller(operation,operation.generation);let mut source=Some(value);for _ in 0..LAYOUT_EXPORT_FIXTURE_TURN_BUDGET{if source.is_none(){break;}caller.turn(LAYOUT_EXPORT_FIXTURE_TURN,|cx|{let mut work=semio_framework_2d_text_font::work::RetainedWorkLedger::new(cx.retained_grant());let grant=cx.retained_grant();let Some((result,_))=cx.with_original_effect(grant,|_,effect|->Result<(),semio_framework_value::ValueError>{if owner.reserve_one_in(&mut work,effect)?{owner.push_in(source.take().unwrap(),&mut work)?;}Ok(())})?else{return Ok(())};cx.consume_retained(work.receipt())?;result?;Ok(())}).unwrap();}assert!(source.is_none());caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);}
/// ♻️ Fixture teardown pays the same original plan owner and validates terminal emptiness.
pub fn close_plan_fixture<T:semio_framework_value::retirement::RetireOwned>(owner:&mut plan_storage::PlanList<T>){let operation=Operation::new(semio_framework_job::OperationId(919),semio_framework_job::RevisionId(1),semio_framework_job::Generation(1),0);let mut caller=layout_export_fixture_caller(operation,operation.generation);for _ in 0..LAYOUT_EXPORT_FIXTURE_TURN_BUDGET{if owner.terminal_is_empty(){break;}caller.turn(LAYOUT_EXPORT_FIXTURE_TURN,|cx|{let demand=owner.next_close_demand()?;semio_framework_job::close_original_effect_step(cx,demand,|effect|owner.close_one(effect))?;Ok(())}).unwrap();}assert!(owner.terminal_is_empty());caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);}

/// 📍️ Companion laws append a real funded PDF offset without exporting private paint-row types.
pub fn append_offset_fixture(owner:&mut PlanOffsets,value:u32){append_plan_fixture(owner,value);}

/// ♻️ Native contour fixtures return successful child originals through their paired paid owner.
pub fn close_original_fixture<T:semio_framework_value::retirement::RetireOwned>(value:T){
    let operation=Operation::new(semio_framework_job::OperationId(920),semio_framework_job::RevisionId(1),semio_framework_job::Generation(1),0);
    let mut caller=layout_export_fixture_caller(operation,operation.generation);
    let mut source=Some(value);
    let mut retirement=None;
    for _ in 0..LAYOUT_EXPORT_FIXTURE_TURN_BUDGET{
        if source.is_none()&&retirement.is_none(){break}
        caller.turn(LAYOUT_EXPORT_FIXTURE_TURN,|cx|{
            let demand=plan_storage::owner_close_demand(&source,&retirement)?;
            semio_framework_job::close_original_effect_step(cx,demand,|effect|plan_storage::close_owner_slot(&mut source,&mut retirement,effect))?;
            Ok(())
        }).unwrap();
    }
    assert!(source.is_none()&&retirement.is_none());
    caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK)
}


pub fn fixture_effect<R>(grant:semio_framework_value::RetainedCloneGrant,step:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->(R,semio_framework_value::RetainedCloneProgress))->R{
    let operation=Operation::new(semio_framework_job::OperationId(921),semio_framework_job::RevisionId(1),semio_framework_job::Generation(1),0);
    let mut caller=layout_export_fixture_caller(operation,operation.generation);
    let output=caller.turn(grant,|cx|{
        let ((output,receipt),work)=cx.with_original_effect(grant,|_,effect|step(effect))?.expect("original export fixture effect");
        cx.consume_work(work);
        cx.consume_retained(receipt)?;
        Ok(output)
    }).unwrap();
    caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);
    output
}
