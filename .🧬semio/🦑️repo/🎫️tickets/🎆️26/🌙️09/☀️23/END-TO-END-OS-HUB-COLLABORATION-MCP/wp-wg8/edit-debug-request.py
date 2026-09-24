import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = path.read_text()
pairs = [
    ("""                KernelRequest::ExchangeCommands { instance, driver } => KernelOutcome::Exchanged(state.exchange_commands(instance, driver).await),
""", """                KernelRequest::ExchangeCommands { instance, driver } => {
                    let debug_started = std::time::Instant::now();
                    let exchanged = state.exchange_commands(instance, driver).await;
                    eprintln!("[DEBUG] wg8 request exchange-commands instance={instance} elapsed={:?} ok={}", debug_started.elapsed(), exchanged.is_ok());
                    KernelOutcome::Exchanged(exchanged)
                }
"""),
    ("""                KernelRequest::Exchange { instance, event } => KernelOutcome::Exchanged(state.exchange(instance, vec![event.into_event()]).await),
""", """                KernelRequest::Exchange { instance, event } => {
                    let debug_started = std::time::Instant::now();
                    let exchanged = state.exchange(instance, vec![event.into_event()]).await;
                    eprintln!("[DEBUG] wg8 request exchange instance={instance} elapsed={:?} ok={}", debug_started.elapsed(), exchanged.is_ok());
                    KernelOutcome::Exchanged(exchanged)
                }
"""),
    ("""                let outcomes = self.runtime.wait_for_outcomes(expected, RUN_TURN_OUTCOME_TIMEOUT);
""", """                let debug_waited = std::time::Instant::now();
                let outcomes = self.runtime.wait_for_outcomes(expected, RUN_TURN_OUTCOME_TIMEOUT);
                if debug_waited.elapsed() > std::time::Duration::from_millis(500) {
                    eprintln!("[DEBUG] wg8 slow wait actor={} expected={expected} got={} elapsed={:?} kinds={:?}", actor.0, outcomes.len(), debug_waited.elapsed(), outcomes.iter().map(|outcome| match outcome { ShardOutcome::Turn { .. } => "turn", ShardOutcome::Job { .. } => "job", ShardOutcome::Fault { .. } => "fault", ShardOutcome::Preempted { .. } => "preempted", _ => "other" }).collect::<Vec<_>>());
                }
"""),
    ("""            while self.reserved_jobs.iter().flatten().any(|job| job.actor == actor) {
                if std::time::Instant::now() >= deadline {
""", """            while self.reserved_jobs.iter().flatten().any(|job| job.actor == actor) {
                eprintln!("[DEBUG] wg8 reserved drive actor={} jobs={:?}", actor.0, self.reserved_jobs.iter().flatten().map(|job| (job.job, job.step.map(|turn| turn.step_sequence))).collect::<Vec<_>>());
                if std::time::Instant::now() >= deadline {
"""),
]
if sys.argv[1:] == ["revert"]:
    pairs = [(new, old) for old, new in pairs]
for old, new in pairs:
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)
path.write_text(text)
print("ok")
