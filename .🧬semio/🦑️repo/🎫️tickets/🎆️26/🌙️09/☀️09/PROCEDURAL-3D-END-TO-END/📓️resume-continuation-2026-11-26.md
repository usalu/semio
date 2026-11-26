# Math resume continuation on the React path

## Verdict

No code path that drops the second provider request was found. The existing evidence points to latency of the second Edit tick's retained job, cut off by the 762 s journey deadline. It does not point to a missing resume emission. No code was changed and no test was run in this pass. The native Rust law and the retained-job tests were not recompiled because the host load average was about 13 and free memory was low.

## What the stable B4 trace shows

Source: `🗑️generated/journey/react-b4-mushroom-stable/console.txt`.

| Time | Event |
| --- | --- |
| 208.5 s | First Edit `flowEvalTick`. Its `HISTORY_FOLLOW_RESULT` at 210.5 s carries `effects: []`. |
| 530.2 s | `EXT_INVOKE` for Math. This is about 320 s after the first tick's result. The invocation did not come from the tick's own response. It came later, from the retained job's completion subscription. |
| 539.6 s | `EXT_ANSWER`, 87 bytes. |
| 588.167 s | `EXT_DELIVERED` with 1 effect, the rearmed Edit tick. |
| 588.169 s | The Edit tick is dispatched. |
| 597.5 s | The second Edit tick's `HISTORY_FOLLOW_RESULT` has `effects: []`. This matches the first tick's shape. |
| 597.5 to 762 s | Window closed after 174 s. The preview shows 4 of 7 nodes and 1 call in flight, and no `EXT_INVOKE` has appeared. |

The first Edit tick needed about 320 s before its invocation was emitted. The second had 174 s. In both ticks the tick command itself returns an empty effect list. The `invokeExtension` effect is only published through `subscribeOperationCompletions`, then `applyHostEffects`. The second tick therefore shows no divergence from the first. It simply had not reached that point yet.

## Code paths examined, none found to drop the request

- `dispatchInvokeExtensionEffect` in `🏛️ShellHost/🔌️extension-runtime/🟦️.ts` holds `serializePerActor("<plugin>:<instance>")` during the provider call and `completion.complete`. `publish` runs after the slot is released. The other keys (`command-ingress:`, `ui-intake:`) are distinct, so there is no queue-cycle deadlock.
- `applyDispatchActionEffect` schedules the tick. `makeEffectDispatchOne` drops its response only if `isCurrentEffectOwner` is false after `handleCommand` returns. That would silently discard an `invokeExtension`. It is a candidate cause. The trace shows `current:true` on both `HISTORY_FOLLOW_RESULT` lines, and the first request surfaced through the completion subscription path, which uses a fresh owner.
- `typedOperationCompletionRefreshV1` passes any completion with `requestedEffects.length > 0`, so an invocation-only completion is applied.
- Guest side: `FlowResolveJobWork` phases 25 to 35 settle the reply and emit the tick effect. `tick-budget` bounds each walk to a node count and about 6 ms, so slow browser operators yield one node per tick. The in-process law `the_preview_reissues_working_evaluations_as_compact_resumes` (`✅️flow-eval-resolve/🧪️tests/🔬️budget/🦀️.rs`) and `📥️evaluation/🧪️tests/🔁️math-resume.rs` already assert that the progress reply, the rearmed tick and a compact `resume:true` request share the same identity. I found no recorded run of the budget law in this ticket.

## Likely root cause and what remains

The per-tick cost in Chromium, about 5 minutes to the first emitted invocation, exceeds the journey's 174 s post-Completed window. The worker profile shows about 22 s inclusive in `advance_typed_operation_output_inner` per 28 s sampled, so the time is guest compute and not a host queue.

Next steps, in order:

1. Rerun the 6021 journey with a post-Completed window of at least 900 s (`react-journey-probe.mjs`). Add `[DEBUG]` lines at the completion-subscription callback in `⚡️host-effects/📡️operation-subscriptions/🟦️.ts` that log `requestedEffects` kinds, and in `makeEffectDispatchOne` that log the `isCurrent()` result after `handleCommand`. Both must be removed afterwards.
2. If `EXT_INVOKE` req 2 appears with `resume:true`, the resume path works and the remaining blocker is per-tick guest cost. Attack that through the inference cache and the original-document pacing notes.
3. If no invocation appears and `isCurrent()` is false, make the owner check non-dropping for invocation-only responses. That is the one real divergence candidate.
4. Run the native budget law under `repo capacity run` when the host is quiet, to confirm native parity.

## Files changed

Only this note.
