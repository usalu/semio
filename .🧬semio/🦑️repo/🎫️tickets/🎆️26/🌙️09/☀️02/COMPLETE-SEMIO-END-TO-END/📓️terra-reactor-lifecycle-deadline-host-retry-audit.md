# Reactor Lifecycle Deadline Host Retry Audit

## Scope and verdict

Read-only source audit of the new `plugin.reactor-turn-deadline` retryable fault. No build or runtime command was run here.

The reducer correctly preserves its internal owner on this path, but the retryable bit is not yet consumable by any production-facing native, component, or browser dispatcher. The only retryer is the native *test helper*. The host adapters serialise the guest `Fault` into text, then treat it as a terminal/trap error. Therefore a legitimate 8 ms lifecycle deadline will strand an opening/captured/closing lifecycle until a caller reconstructs a fresh event manually; it must not be described as a production retry protocol yet.

This is separate from the already handled Wasmtime epoch interruption: the shard maps `TurnFault::DeadlineExceeded` to a no-output `MoreWork` turn. That is a host-side execution interruption, not the reducer's post-output lifecycle-clock verdict.

## Exact source trace

1. `🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:108` sets `retryable_lifecycle` only when the input has no command page and every event is `InstanceOpen`, `InstanceClose`, or `InstanceLifecycleAck`.
2. At `:770-791`, the reducer stages a pending patch, calls the output-preparation closure, then calls `finish_turn`. A lifecycle-clock failure is emitted as `Fault{code: "plugin.reactor-turn-deadline", retryable: retryable_lifecycle}` at `:774-778`. The failure arm hands the uncommitted patch owner back before returning. It does not advance the lifetime.
3. `🔌️plugin/⚛️reactor/🚪️lifetime/🧪️tests/🧵️runtime.rs:13-24` is the only current replayer. It clones the original `Vec<Event>`, retries `poll_kernel(..., None, ...)` up to 64 times, and yields between attempts. The later output-fault law proves the provisional patch receipt was not issued and that the same lifecycle ACK can later succeed. This is useful reducer coverage, but it is test scaffolding rather than a mounted host scheduler.
4. The component ABI does preserve a usable representation at the boundary. `🔌️plugin/🦀️.rs:37-39` maps a framework `Fault` to `PluginError::Fault(dsl::encode_fault_bytes(&fault))`; WIT declares `reactor.poll -> result<turn-result, plugin-error>` in `🔌️plugin/🧬️schema/📜️.wit:20-22,1154`. `dsl::decode_fault_bytes` already exists in `🔨️modules/⚠️diagnostic/🦀️.rs:625-632`.
5. `🔌️plugin/🖥️host/🦀️.rs:2065-2090` (`WasmtimeRuntime::execute_turn`) instead maps the WIT `Err(PluginError)` to `TurnFault::Trapped(format!("{error:?}"))`. `TurnFault` at `:602-608` has no structured guest-fault variant. The shard's `execute_turn` result switch only special-cases host `TurnFault::DeadlineExceeded` at `🧵️shard/🦀️.rs:1894-1914`; the new reducer fault reaches its generic `ShardOutcome::Fault` arm.
6. The owned ABI loses the same information: `OwnedRuntime::decode_owned_result` at `🖥️host/🦀️.rs:1605-1608` turns the encoded JSON fault bytes into `TurnFault::Trapped(String)`. The exported `semio_owned_poll_v1` has returned those exact bytes since `🔌️plugin/🦀️.rs:32800-32809`, so the loss is solely the host decoder.
7. The concurrent component runtime has an independent loss: `🔌️plugin/🖥️host/⏳️runtime/🦀️.rs:233-242` exposes `Poll` as `oneshot::Sender<Result<KernelTurnResult, String>>`, and `PollTask::run` at `:372-399` formats WIT `PluginError` with `format!("{fault:?}")`.
8. In the browser shard path, the generated worker invokes `reactor.poll` directly (`🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:572-580`). Its catch path's `replyError` converts a lifted fault record to `error: string` (`:254-280,446-452`). `ShardClient`'s inbound result type is also string-only (`🎭️actor/📮️shard-client/🟦️.ts:377-380`), and `handleMessage` deletes the pending request then rejects it (`:1266-1292`). Neither `captureActorActivation().turn` (`:1455-1480`) nor `sendInstanceLifecycle` (`:1724+`) retains the sent lifecycle input after that rejection.

The newer closed browser child is not a counterexample: it is not yet the reactor-poll host. Do not add a retry rule there before an actor body route is mounted.

## Minimal coherent repair

Keep the user-visible 8 ms bound. Do not run a tight retry loop in the guest, worker handler, or WIT adapter.

Add one host-owned, exact-input continuation at the existing per-actor serial dispatcher:

```text
LifecycleRetryOwner {
  exact activation + NativeCloseKey/lifetime owner,
  one lifecycle-only event list (or empty poll),
  command_page = None,
  original budget policy,
  scheduler-resume token
}

execute once
  success -> consume owner and publish the returned TurnResult
  decoded Fault(code == reactor-turn-deadline, retryable == true)
      -> retain unchanged owner; enqueue one later scheduler opportunity
  any other fault, actor replacement, close, worker loss, or scheduler terminal
      -> retire owner through the existing lifetime close/error path
```

The continuation must be constructed before dispatch, must carry the event values (not a receipt reconstructed from diagnostic text), and must be accepted only for a host-known lifecycle-only/no-page request. It may use the existing one-turn-per-actor serialization. A later scheduler tick—not `while`/`yield_now` in the request handler—must call it, so a persistently late lifecycle cannot monopolize a lane. A close/replacement must invalidate the exact activation/lifetime before replay; it must not send an old ACK/open after a successor is active.

At the adapters, preserve `Fault` first:

- Add a structured `TurnFault::Guest(semio_framework::Fault)` (or an equally typed private result) and decode the `PluginError::Fault` bytes with the existing `dsl::decode_fault_bytes` in `WasmtimeRuntime` and `OwnedRuntime`.
- Replace async-runtime `Result<KernelTurnResult, String>` with an error type that retains the decoded `Fault`; formatting belongs only at the outer diagnostic sink.
- Extend the browser worker result with a bounded, schema-validated guest-fault record/bytes. `ShardClient` should inspect it while it still owns its exact pending lifecycle request and park a retry owner. Do not decide from a substring of `error`.

The reducer's own `retryable` flag is advisory, not authority: never requeue a command-page or mixed event request merely because a hostile component returns a matching fault code. Conversely, a lifecycle retry may coexist with an unrelated pending external patch: the reducer's hand-back/reissue protocol means the provisional receipt must remain unobservable; the eventual successful turn alone may publish its newly issued receipt.

## Required acceptance laws

1. **Native component host:** a real WIT `PluginError::Fault(encode_fault_bytes(retryable deadline))` first, then success. Assert the host decodes code and retryability, replays byte-identical lifecycle event/no-page input on a later scheduler opportunity, and emits only the successful lifecycle/patch receipt.
2. **Owned ABI:** the first `semio_owned_poll_v1` result is the same encoded fault. Assert `OwnedRuntime` retains it as typed guest fault and its continuation uses the identical saved poll bytes; a nonretryable fault is terminal.
3. **Async component runtime:** the `PollTask` error preserves the structured fault through the oneshot. A second dispatch succeeds without recreating the actor/store; no formatted string is used for routing.
4. **Browser worker + `ShardClient`:** use a worker that returns the lifted retryable record first and success second. Assert exactly one later retry message with same actor, activation generation, event content, and no command page. A lifecycle replacement between attempts cancels the parked retry and produces no stale post.
5. **Hostile eligibility:** matching code with `retryable=false`, a command page, or a mixed lifecycle+ordinary event never requeues.
6. **Patch hand-back:** inject the deadline after a different pending patch is staged. Assert provisional receipt is never sent to the host/browser, the patch is retained internally, and the successful replay produces the sole publishable receipt.
7. **Fairness:** repeated retryable deadline leaves at least one other actor turn runnable between attempts and retains a single continuation, not N queued copies.

The current native helper's fixed `0..64` immediate retries is suitable only as a test driver. It neither demonstrates a production retry owner nor proves browser/component interop.
