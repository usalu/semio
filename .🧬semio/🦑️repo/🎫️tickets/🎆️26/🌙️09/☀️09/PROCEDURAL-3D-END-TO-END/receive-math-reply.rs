use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller};
use semio_framework_value::RetainedCloneGrant;
use semio_os_flow::{FlowEvalSession,FlowEvaluationReplyAddress,FlowEvaluationReplyReceiver,PreviewEvalPhase};
fn main() {
    std::thread::Builder::new().stack_size(64 << 20).spawn(replay).unwrap().join().unwrap();
}
fn replay() {
    let wire=std::fs::read(std::env::args_os().nth(1).unwrap()).unwrap();
    let value=pack::record::decode_value_record_body_exact(&wire,1,&pack::PackLimits::default()).unwrap();
    let hash=std::env::args().nth(2).unwrap().parse::<u64>().unwrap();
    let window="procedural-preview";
    let address=FlowEvaluationReplyAddress{node_hash:hash,window_id:window,window_kind_id:window};
    let stock=RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<27,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<29,maximum_depth:4096};
    let turn=RetainedCloneGrant{maximum_items:1,..stock};
    let mut session=FlowEvalSession::new();
    session.arm_window_tick(window,stock).unwrap();
    session.begin_window_tick(window,stock).unwrap();
    session.note_window_tick_outcome(window,true,stock).unwrap();
    session.note_window_extensions_in_flight(window,1,stock).unwrap();
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(256),Generation(1),stock,||Some(1),u64::MAX,8_000_000);
    caller.retain_dispatch().unwrap();
    let mut decoder=caller.admitted_decoder();
    let mut receiver=caller.refreshed_turn_with_decoder(turn,&mut decoder,|cx|FlowEvaluationReplyReceiver::new(value,cx).map_err(|(err,_)|err)).unwrap();
    let mut turns=0;
    while !receiver.terminal_is_empty() && turns<50_000{
        turns+=1;
        caller.refreshed_turn_with_decoder(turn,&mut decoder,|cx|receiver.step(&mut session,&address,cx)).unwrap();
    }
    assert!(receiver.terminal_is_empty());
    let progress=session.eval_progress_by_hash.get(&hash).unwrap();
    assert_eq!(progress.phase,PreviewEvalPhase::Computing);
    assert_eq!((progress.units_done,progress.units_total),(256,257));
    assert_eq!(session.window_extensions_in_flight(window),0);
    println!("[DEBUG] real Pack settled in {turns} single-item turns, units=256/257 in-flight=0");
    while !decoder.terminal_is_empty(){caller.retirement_turn(stock,|cx|decoder.close_on_source(cx).map(|_|())).unwrap();}
    caller.retire(stock);
    session.retire_cold();
}
