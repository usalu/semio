# lease-request: 🖥️host/🦀️component.rs `//#region 🔀️PostTurnRelay` (`PluginInstanceHandle::run_job_to_completion`)

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`, lines ~1428-1439
(current, after this packet's edits landed — `PluginInstanceHandle::run_job_to_completion`,
`//#region 🔀️PostTurnRelay`).

Reason: my packet (terra-trait-asyncify, W6) changed `GuestRuntime::start_job`/`step_job` (and
`execute_turn`/`cancel_job`/`checkpoint`/`restore`) to return `semio_framework_async::HostFuture<T>`
instead of a bare `Result<T, _>`, so a genuinely-suspending backend can share the trait with
today's synchronous `WasmtimeRuntime`/`MockGuestRuntime` — see the trait's own doc comment
(`GuestRuntime`, ~L490) and the `poll_ready` helper doc comment right after it (~L525) for the
full rationale. `run_job_to_completion` is a real, live caller of `start_job`/`step_job` and sits
in `//#region 🔀️PostTurnRelay`, which is NOT in my packet's owned paths (owned: the `GuestRuntime`
trait region ~490-510, `WasmtimeRuntime` impl, `MockGuestRuntime` impl only) — so I could not
apply this myself without violating rule 3. **The crate does not compile until this lands.**

## The fix (mechanical, two lines)

`poll_ready` is already defined in THIS SAME FILE (`pub fn poll_ready<T>(future: HostFuture<T>) ->
T`, right after the `GuestRuntime` trait) — no new import needed, just wrap both calls:

CURRENT:
```rust
    fn run_job_to_completion(&self, kind: &str, input: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        let job = self.next_job_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut instance = self.instance.lock().map_err(|_| PluginHostError::LockPoisoned("plugin instance handle"))?;
        self.runtime.start_job(&mut instance, job, kind, input).map_err(|fault| PluginHostError::Plugin(format!("{kind} start-job: {fault}")))?;
        loop {
            match self.runtime.step_job(&mut instance, job, RELAY_JOB_BUDGET).map_err(|fault| PluginHostError::Plugin(format!("{kind} step-job: {fault}")))? {
                JobStep::Done { output } => return Ok(output),
                JobStep::Failed { error } => return Err(PluginHostError::Plugin(format!("{kind} job failed: {}", String::from_utf8_lossy(&error)))),
                JobStep::Running { .. } => continue,
            }
        }
    }
```

REPLACEMENT (only the two `self.runtime.*` lines change, wrapped in `poll_ready(...)`):
```rust
    fn run_job_to_completion(&self, kind: &str, input: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
        let job = self.next_job_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut instance = self.instance.lock().map_err(|_| PluginHostError::LockPoisoned("plugin instance handle"))?;
        poll_ready(self.runtime.start_job(&mut instance, job, kind, input)).map_err(|fault| PluginHostError::Plugin(format!("{kind} start-job: {fault}")))?;
        loop {
            match poll_ready(self.runtime.step_job(&mut instance, job, RELAY_JOB_BUDGET)).map_err(|fault| PluginHostError::Plugin(format!("{kind} step-job: {fault}")))? {
                JobStep::Done { output } => return Ok(output),
                JobStep::Failed { error } => return Err(PluginHostError::Plugin(format!("{kind} job failed: {}", String::from_utf8_lossy(&error)))),
                JobStep::Running { .. } => continue,
            }
        }
    }
```

Behavior is unchanged — `poll_ready` panics only if the future is not ready on first poll, and
every `GuestRuntime` impl in the crate today (`WasmtimeRuntime`, `MockGuestRuntime`) is eagerly
ready by construction, same as at every other call site this packet converted
(`🧵️shard/🦀️component.rs`'s `ShardLoop::pump`/`pump_primed`, this file's own
`mock_guest_runtime_tests`).

No other line in `run_job_to_completion`, `io_run`, `io_sniff`, or anywhere else in
`//#region 🔀️PostTurnRelay` needs to change — `run_job_to_completion`'s own signature
(`Result<Vec<u8>, PluginHostError>`, fully synchronous) is untouched; only its two internal
`GuestRuntime` calls change shape.
