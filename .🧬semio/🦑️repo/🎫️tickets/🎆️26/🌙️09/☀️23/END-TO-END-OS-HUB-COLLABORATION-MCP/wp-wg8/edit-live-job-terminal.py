import pathlib

shard = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs")
host = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs")
law = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧪️tests/🔬️unit/🦀️.rs")


def replace(text, old, new):
    assert text.count(old) == 1, (text.count(old), old[:100])
    return text.replace(old, new)


s = shard.read_text()
s = replace(s, """            // 🔀️ Same E0502 reason as the turn-execution loop above — computed before `get_mut`.
            let job_budget = job_budget_from_grant(self.granted_budget(actor_id));
            let actor_lane = self.actor_lane(actor_id);
            let watchdog_stage = interactive_stage_for(actor_lane);
            let Some(instance) = self.instances.get_mut(&actor_id) else {
                self.close_replay_job(actor_id, job, ReplaySeedCloseReason::ActorLost);""", """            // 🔀️ Same E0502 reason as the turn-execution loop above — computed before `get_mut`.
            let job_budget = job_budget_from_grant(self.granted_budget(actor_id));
            let actor_lane = self.actor_lane(actor_id);
            let watchdog_stage = interactive_stage_for(actor_lane);
            let replayable = self.replay_seeds.iter().flatten().any(|seed| seed.actor == actor_id && seed.job == job && seed.replayable);
            let Some(instance) = self.instances.get_mut(&actor_id) else {
                self.close_replay_job(actor_id, job, ReplaySeedCloseReason::ActorLost);""")
s = replace(s, """                        JobStep::Done { output } => match self.runtime.checkpoint(instance).await {
                            Ok(state) => {""", """                        JobStep::Done { output } => match if replayable { self.runtime.checkpoint(instance).await } else { Ok(Vec::new()) } {
                            Ok(state) => {""")
shard.write_text(s)

h = host.read_text()
h = replace(h, """        let guest_checkpoint = if state.pending.is_none() {
            begin_owned_operation(state, OwnedOperation::Checkpoint, None).map_err(turn_fault_host)?;
            let invocation = resume_owned_operation(state, OwnedOperation::Checkpoint, 100_000_000, 1_000).map_err(turn_fault_host)?;
            Some(decode_owned_result::<Vec<u8>>(&invocation.output).map_err(turn_fault_host)?)
        } else {""", """        let guest_checkpoint = if state.pending.is_none() {
            begin_owned_operation(state, OwnedOperation::Checkpoint, None).map_err(turn_fault_host)?;
            let invocation = match resume_owned_operation(state, OwnedOperation::Checkpoint, 100_000_000, 1_000) {
                Ok(invocation) => invocation,
                Err(fault) => {
                    cancel_owned_operation(state).map_err(turn_fault_host)?;
                    return Err(turn_fault_host(fault));
                }
            };
            Some(decode_owned_result::<Vec<u8>>(&invocation.output).map_err(turn_fault_host)?)
        } else {""")
host.write_text(h)

l = law.read_text()
l = replace(l, """    assert!(matches!(steps.as_slice(), [JobStepOutcome::Yield, JobStepOutcome::Complete { candidate }] if candidate.output == b"undone"), "{steps:?}");
    let completed = mock.observed_events(actor).await.into_iter().find(|event| matches!(event, Event::JobCompleted { job, .. } if *job == job_id));""", """    assert!(matches!(steps.as_slice(), [JobStepOutcome::Yield, JobStepOutcome::Complete { candidate }] if candidate.output == b"undone" && candidate.state.is_empty()), "{steps:?}");
    let GuestInstanceState::Mock(state) = &shard.instances.get(&actor.0).expect("registered instance").state else { panic!("mock instance") };
    assert_eq!(state.checkpoint, None, "a live-only reserved job commits no restore state, so its end never checkpoints the guest either");
    let completed = mock.observed_events(actor).await.into_iter().find(|event| matches!(event, Event::JobCompleted { job, .. } if *job == job_id));""")
law.write_text(l)
print("ok")
