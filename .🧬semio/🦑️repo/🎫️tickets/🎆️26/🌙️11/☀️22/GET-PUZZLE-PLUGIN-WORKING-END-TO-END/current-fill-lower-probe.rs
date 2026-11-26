#![allow(unused)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧰️prelude/🦀️.rs"));
mod context {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧰️context/🦀️.rs"));
}
use context::*;
fn observe(app: &mut Puzzle3dApp, label: &str) {
 let raw=app.parts().0;
 let config=raw.stores.config_store.snapshot_owner();
 let gen=raw.stores.config_store.generation();
 for e in &raw.tool_runs.entries {
  eprintln!("[DEBUG] {label} count={} store generation={gen} watched={:?} run generation={} state={:?} operations={} job={} progress={:?}",config.fill_count,e.settings_generation,e.slot.generation,e.slot.state,e.operations.len(),e.job.is_some(),raw.tool_runs.presence());
 }
}
#[semio_framework_async_macros::async_test]
async fn lower_count() {
 let mut app=app().await;
 dispatch(&mut app,"setFillCount",Some(&json!({"value":4})),None).await.unwrap();
 start_fill_run(&mut app).await;
 pump_fill_run(&mut app,"four",|p|p.state==protocol::PresenceToolRunState::Complete).await;
 observe(&mut app,"four");
 dispatch(&mut app,"setFillCount",Some(&json!({"value":6})),None).await.unwrap();
 observe(&mut app,"raise");
 pump_fill_run(&mut app,"six",|p|p.state==protocol::PresenceToolRunState::Complete && p.completed==6).await;
 observe(&mut app,"six");
 dispatch(&mut app,"setFillCount",Some(&json!({"value":2})),None).await.unwrap();
 observe(&mut app,"lower");
 let mut collected=Puzzle3dSettled::default();
 for turn in 0..2500 {
  host_turn(&mut app,1,&mut collected).await;
  if [0,10,100,2499].contains(&turn){observe(&mut app,"after lower");}
 }
 assert_eq!(fill_run_presence(&app).completed,2);
}
