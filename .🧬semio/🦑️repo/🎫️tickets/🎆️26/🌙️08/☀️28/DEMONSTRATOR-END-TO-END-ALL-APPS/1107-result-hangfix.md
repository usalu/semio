# 🔁️ 1107 Result: hangfix (23 audited driver loops)

Static classification plus edits. Paths are under `/Users/ueli/Documents/semio/🧰️framework/` (abbreviated `F/`). "Edited" = changed on disk; nothing is "verified" unless the compile section says so. No shared cross-crate testkit helper for "no progress" exists (searched `job`, `🧪️testing`, `conformance`): the loops differ in step type and observable progress, so each guard is inline and uses the loop's own progress observable.

## Production loops

| # | Loop | Verdict | Evidence / change |
|---|---|---|---|
| 2 | `fold/🦀️.rs finish_cold` | REAL (sync, fixed grant, Pending re-run) | Pending with `progress == default` repeats identically (grant is derived from unchanged state). Edited: idle counter, `assert!(idle < 16, ...)` naming the fault; progress resets it. Cold callers keep `expect` semantics. |
| 3 | `ingress/🦀️.rs:563-564` | `fill_one_page` FALSE POSITIVE; `retire_source_step` latent | `fill_one_page` returns `false` only after admitting a page (copied advances) or if `length > maximum_bytes`; the caller passes the page constant, so the second case is unreachable. `retire_source_step`: `plugin_bytes_close` demand is `copy_bytes: 1` (non-empty) or `release_bytes: capacity` (empty), both funded by `plugin_demand_grant`; the copy override `PAGE_BYTES >= 1` cannot unfund it. Still sync and unguarded, so edited: a `Progress(default)` now returns `plugin_sdk_fault` instead of retrying. |
| 4 | presence `🔤️json/🦀️.rs` | REAL latent (two paths) | (a) admission `while phase != "ready"`: an `Ok` step with zero progress and unchanged phase repeated forever. Edited: stall counter (2 consecutive identical steps, `stalled()` refusal). (b) receiving loop `Pending{work}` mapped to `work.take(..)` and re-ran with the same remaining grant; per runtime2 rule R4 (host recipients are synchronous, Pending is recovery-only) `Pending{work:0}` now returns the existing `pending()` refusal. The parse loop above already had a no-progress guard. |
| 5 | neural `Dictionary::from_value` | FALSE POSITIVE (already guarded) | The loop has an arm `step.progress == default` that cancels, closes and returns `WorkLimit "typed input binding step made no progress"`; closure uses `cold_input_close` -> `moving()` which errors on a no-progress step. The audit missed the arm. No edit. Side note, not edited: `cold_advance` (`🧵️retirement/🦀️.rs`) loops `update.advance(.., u64::MAX)` with no progress check. |
| 6 | startup test `while !owner.ready()` | test, REAL if guest never becomes ready | Each turn gives 1 fuel (no progress observable but `ready()`). Edited: `assert!(epoch <= 1000, ...)` with message (sibling test uses 250 turns). This is a turn bound, not a progress detector, and is the only one: if it fires, find the guest cause, do not raise it. |
| 7 | `wasmtime-node-host/🦀️.rs` 3x `while !admit_root_step()` | livelock (yields), not CPU spin | `admit_root_step` returns false when `begin_policy_turn` is None (treasury exhausted) or the lock is busy; nothing refills inside the loop. Edited: new `admit_root()` helper replaced all 3 sites; refuses with `RunError::Original(WorkLimit)` when `original_remaining()` is unchanged for 65536 consecutive yields (a lock-busy `Err` resets the counter). The remaining actor-turn loops (`begin_actor_turn` None, `return_actor_turn` false) were not changed. |
| 8 | `🎫️operation-admission.rs admit` | livelock (yields) | Same shape. Edited: same treasury-unchanged detector (65536), sets `failure`, `abandon_operation()`, returns `Failed`. |
| 9 | `wal/🦀️.rs:774 close_step` | FALSE POSITIVE | `SegmentWriter::close_step` returns `true` only after taking the writer or closing a buffer step; a stuck state returns `Err("did not reach terminal ownership")`, and `false` ends the loop. Finite state machine, yields each turn. |
| 10 | `causal/🦀️.rs:524 advance_ready_one` | FALSE POSITIVE | Returns true only after `mark_applied(id)` removes `id` from `pending` (`swap_remove`), so the queue strictly shrinks. |
| 11 | `border-presentation resolve_shared_border_segments` | FALSE POSITIVE | `step()` is a cursor over a finite contributor set: each `Pending` writes one contributor or advances `last_contributor`; no grant involved. |
| 12 | poll `session drive_from` | yields each turn, no typed refusal | Terminal statuses (Complete/Refused/Closed) end it; the Pending path re-runs stage steps under per-turn grants. Not edited: a treasury detector could false-fire on legitimate multi-stage waits and the session already turns faults into `Refused`; flagged for the runtime lane (W4). |

## Test and fixture loops

| # | Loop | Verdict | Change |
|---|---|---|---|
| 1 | neural evaluation `🧪️tests/🦀️.rs:22,26,27` | REAL per audit | Owned by lane port-j; not touched. |
| 13 | `node-drag-history while let ReplayStep::Pending` | REAL if replay stalls | Edited: 16 consecutive identical `ReplayProgress` -> panic naming guest, edit index and progress. |
| 14 | `retire_cancelled_world3d_asset_step` | bounded by contract | Delegates to the claim owner's `close_step`; returns false when no cancelled claim remains. Not edited (no progress observable). |
| 15 | wgpu prepared `close_packet_step` | REAL via `Pending`/`Blocked` mapping to `false` | Edited in the shared helper module `🎟️prepared/🧪️testing/🦀️.rs` (7 helpers: packet, abandoned packet/mailbox/job/input, job): `Pending` with default progress under the exact demanded grant now asserts; `Blocked` now panics (a close that cannot progress is a refusal, not a retry). One edit covers all callers. |
| 16 | reactor `mounted-output-admission-laws` | REAL if a step stalls | Edited: reservation and output close loops assert `UiValueRetirementStep.progressed` on every non-complete step. |
| 17 | catalog-bootstrap tests | cross-thread waits | Edited: file-local `wait_while(what, busy)` (120 s, message names the waited condition) replaces 8 `while cond { yield_now }` loops incl. the three audited. The multi-assert close loop in the lost-handle test (uses `Blocked` legitimately while the IO lane retires) is unchanged. |
| 18 | `lost-owner-ring` `db_io_lost_owner_close_step` | bounded | Returns true only after a `fault.close_step()` on a fixed-size slot arena; not edited. |
| 19 | `retained-turn-fixtures control.close_step()` | FALSE POSITIVE | `close_step` returns false when the terminal flag is unset, so the loop exits (assert then fails), true only after closing one owner. |
| 20 | `retire_cancelled_renderer_asset_step` | bounded by contract | Same owner pattern as 14. Not edited. |
| 21 | `drive_text` loops | bounded | `drive_text_step` returns true after a projection or published edit and `Idle -> false`; errors propagate via `expect`. Not edited. |
| 22 | `close_scene_interaction_step` | bounded | `SCENE_INTENTS.close_step()` on a finite intent store. Not edited. |
| 23 | `finalize_shared_borders_step` | bounded | `BorderFinalize` cursor over finite layers, sets `border_finalized`. Not edited. |

Unproven (static only, no progress observable): 14, 18, 20-23. If a hang is seen there, add a stall check at the owner `close_step`, not a loop cap.

## Compile status (cargo check --tests via chk.sh/slot.sh, native; logs `🗑️generated/1105b/hangfix-{3..10}.txt`)

Invocation copied from `1105b-tools/chk.sh`. Package names are workspace-specific: os-side crates only resolve on the root `Cargo.toml`, `replication-history` and `ui-prepared` only on `🧰️framework/Cargo.toml` (hangfix-3/4/5 failed on that alone).

| Crate | Verdict | Evidence |
|---|---|---|
| semio-framework-replication-history (fold `finish_cold`) | lib: compiled, no diagnostic on my lines (hangfix-6/7). lib test: RED, 14 errors in `drafts/🦀️.rs` and `fold/🦀️.rs:92`, not my lines | hangfix-7 |
| semio-os-presence-native-snapshot (json guards + claimed test port) | lib + lib test COMPILE GREEN. Tests (`--lib`, hangfix-18): 0 passed, 3 failed. Local work+construction harness (no manifest change; the `testing-admission` dev-dep was withdrawn: its `#[global_allocator]` conflicts with the crate's own) now gets decode through; the close of the decode retirement recipient fails in `value/♻️retirement/🎮️controlled/🦀️.rs:124`: a retirement cursor in the presence JSON decode chain returns `terminal_release_bytes() == None` ("retirement owner has no controlled terminal release authority"), and the cancel row returns `UnsupportedOwner` instead of `Canceled`. Both are owner-side gaps (value/pack-json cursors), not my guards; the production path under test is the port's retirement ladder | hangfix-12..18 |
| semio-plugin-interpreter (claimed, checkpoint-codec port) | lib GREEN (absent from errors in hangfix-8) | hangfix-8 |
| semio-framework-tool-run-presentation (claimed) | lib GREEN after `as_str()`; lib test RED, 61 stale-call errors in `🧪️tests/🔬️unit/🦀️.rs` (unwrap on poll, 2-arg methods, missing trace import): a test port, not done | hangfix-8 |
| semio-os-services (claimed) | lib + test: no error in hangfix-9 after the `admit(.., &mut actual)`, `u128` fuel and `RetainedWorkOutcome::<_, ValueError>` fixes (two E0283 sites) | hangfix-9 |
| semio-ui-prepared (wgpu prepared `🧪️testing`) | lib compiled; lib test RED: `OriginalJobFixtureCaller` removed from `semio_framework_job` (job `🧪️testing/♻️close` and my `close_job_step` signature) plus shared-border tests. My guard edits show no diagnostic. Needs the job lane's fixture caller replacement | hangfix-7 |
| semio-plugin-instance-docs (ingress), semio-os-run (admit_root), semio-plugin-host-post-turn-relay (operation-admission), semio-plugin-interpreter-runtime (startup), semio-plugin (node-drag-history), semio-plugin-reactor-patches (output admission), semio-os-kernel-db-engine-catalog-bootstrap (wait_while) | NOT CHECKED: blocked by red `semio-os-store-snapshot-clone` (58 errors in hangfix-9; earlier `store-space-history`, `store-operation-wire`, `store-one-item-publication`, owned by store3/store4). Rerun the hangfix-12 command when the store crates are green (hangfix-12: `semio-os-store-replay` 54 errors and `semio-os-store-paged-one-item` 66 errors, both store lanes) | hangfix-6, 8, 9 |

Not mine, noted: `semio-framework-pixels-codecs` (1 error, hangfix-8).

Guards that fired in runs: none (nothing past compile ran except the presence tests, whose failures are the missing recipient, not a guard).

A guard that fires in a run is a finding about the owner, not about the guard.
