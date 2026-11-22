//! 🌊️ Drive the queued release FlowHost with its own baseline publication quote.
use semio_os_flow_host::FlowHost;
use semio_framework_value::RetainedCloneGrant;
fn main() {
 let mut host=FlowHost::default();
 host.begin_baseline_publication(neural_engine::TreeSnapshot::default(),neural_engine::EvalChannels::default(),0).unwrap_or_else(|_|panic!("baseline"));
 for _ in 0..10000 {
  if host.baseline_publication_terminal_is_empty(){break}
  let demand=host.next_baseline_publication_demands(4096).unwrap();
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
  let step=semio_os_flow_host_core::cold_effect(grant,|effect|host.baseline_publication_step(effect)).unwrap().unwrap().unwrap();
  assert!(step.progress().fits(grant));
 }
 assert!(host.baseline_publication_terminal_is_empty());host.retire_cold().unwrap_or_else(|_|panic!("host must retire"));
}
