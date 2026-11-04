# 1103 Flow Result Propagation Review

STATUS: CURRENT DIFF REVIEWED, ONE CONFIRMED OWNERSHIP GATE AND TWO FAILURE-PATH RISKS. No Cargo or source edits.

## Confirmed Ownership Gate

The new caller mappings cannot deliver a recoverable retirement error under the current callee contract. In framework flow host close at 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🏁️host-close/🦀️.rs:83-85, retire_cold consumes the host into local FlowHostRetirement and uses ? throughout; stalled close returns Err too. Drop for FlowHostRetirement at 73-74 asserts that a nonterminal retirement is being dropped only during panic. Thus ordinary demand/step errors and the stalled-close error panic while returning, before new evaluate/tick/UI map_err handlers can observe the ValueError. This callee predates this turn's caller-only edits, but it is a blocking prerequisite of the claimed typed cleanup behavior. Return/preserve the failed retirement custody in a retained owner/error outcome, or provide a close operation that proves terminal emptiness on failure; do not suppress the Drop assertion or discard the live source.

## Current Diff Failure-Path Risks

1. Flow editor 🎮️commands/⏱️flow-eval-tick/🦀️.rs:82-89 changes failure from uncompilable propagation to an actual returned Fault. Before the retirement check, begin_window_tick consumes the latch, tick_cold mutates session, and take_pending_extension_evals removes pending invocations from host. The new ? exits before retaining invocation origins, publishing requests and note_window_tick_outcome. Once retirement can truly return an error, this needs an explicit failure transition so the session cannot retain evaluating nodes without their corresponding requests. Test a forced close refusal and verify the window is faulted/abandoned or correctly retryable.
2. Flow editor 🎛️flow-play-app/🦀️.rs:370 now turns every evaluate_result error into Effect::Notify inside pending_effects. A persistent close failure produces the same notification for each window on every refresh, with no error latch/deduplication. Prefer the app's durable fault transition or a single notification per fault identity; test repeated polling after refusal.

Both risks depend on a close refusal being recoverably delivered; the confirmed ownership gate currently prevents that.

## Reviewed Paths That Preserve Cleanup Ordering

focus_selection_camera now returns Result<Option<CameraJson>,Fault>; its direct-store caller stores the result, retires session, then uses camera? (🧰️direct-store-command-work/🦀️.rs:154-156), so that new error does not bypass session cleanup. A snapshot composition error still precedes that retirement, but this existed before this diff.

Viewer main render stores host retirement result, retires live snapshot and session, then propagates either host/extras failure: good cleanup ordering for the newly introduced errors.

Framework node-graph-extras now preserves baseline admission errors instead of replacing them with an empty status, retires the owned host before returning, and uses transpose to keep absent-session behavior. Procedural generation2d/3d callers map the newly typed error into PluginAssemblyError rather than swallowing it.

Evaluate/tick command handlers, host-only route and extension-action route flatten Result correctly; tracked flow call-site search found no stale .effects access without unwrapping the Result. No new silent suppression was introduced in these inspected conversions. Existing panic/expect and evaluation unwrap_or_default sites were not treated as new findings.

## Verification Needed

The added scene fixture tests cover successful optional-session projection, not close failure custody or latch recovery. Require a forced close-error ownership law, tick refusal/retry law and repeated pending_effects refusal law alongside narrow wasm compile. This review ran only read-only repo git diff/grep and Node filesystem reads. Findings sent directly to flow_compile and root.
