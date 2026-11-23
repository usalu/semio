# L24 3D framework close and quiescence failures

Status: source analysis only. No build or test was run (RAM and disk exhausted); `🗑️generated/l23-3d.log` had no results when this was written.

## 1. History close: "terminal ticket has no explicit native header lifecycle"

Contract: every boxed `ErasedSnapshotRetirement` owner that can reach a terminal-empty state inside a retirement slot must declare `native_header_release()`. The trait default is `None` on purpose (the evidence is `unsafe`: the owner's terminal Drop must be bounded, callback-free, nonallocating and nonunwinding). `factory_ticket_demands`, `close_factory_ticket*` and `FactoryTicketClose` (framework value retirement tickets) refuse a terminal ticket without it with `unsupportedOwner`. The owner declares it, not the ticket code, and not via a blanket default.

Root cause: `TimeTravelDiscardCursor` (`🧰️framework/🔨️modules/⏪️time-travel/🎮️decision/🗑️discard/🦀️.rs`) is boxed into the plugin's time-travel `command_retirements` queue (`admit_typed_retirement(&mut time_travel.discard_cursor, ..)`, plugin time-travel `🧵️instance/🏃️turn-driving-and-replay-steps`) and closed through the ticket path, but it overrode nothing. Once `done`, its Drop is trivial (every field is `None`), so the explicit declaration is sound.

Fix: added `native_header_release` to `impl ErasedSnapshotRetirement for TimeTravelDiscardCursor`, `terminal_is_empty().then(|| unsafe { RetirementHeaderRelease::native(self, 0, 0) })` (same form as the other owners).

Expected effect: the Replaying/Editing history close no longer stops with refusalKind unsupportedOwner, so the terminal-empty witness, "unretired closing session", "original Puzzle work remains in close custody", "context root retains ... Store issuer" and "cancelled caller must release its empty mount shell" residues that cascade from the stuck close should clear. "local interaction read N never terminated" is a downstream residue of the same stuck session; unverified.

Not changed and still open: about 90 other `ErasedSnapshotRetirement` impls (store cursor/ledger/envelope/replay frames, `ArtifactDerivedOperation`, `EditReplay`, ...) have no override. They have not been seen to fail; if a probe backtrace (`[DEBUG] terminal ticket header`, left in place) names another type, add the same declaration there. `TimeTravelDiscardStoreEffects` (plugin discard file) closes through its own `discard_effects_retirement_*` path and is not ticket-closed, so it was left alone.

## 2. "puzzle3d app never quiesced" (about 42 editor-panels tests)

Evidence (/dev/shm/l17-panels.log, 41 occurrences, identical shape): no tool operation, all outboxes empty, all gates false, except one accepted latest-wins keyed command (`lw=1`, `ctx/lookup/accepted/closing/key=(0,false,true,false,None)`). It never gets past `advance_latest_wins_command_one` (`🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🖱️inline-interaction-and-latest-wins`) before `lookup_started`. The gate in that function is `window_config_store.ensure_for_view(view_state, cx)`: the native window-config partition birth (`OriginalStoreGenesis`, needs the receiving owner and the call's retained stock) must report ready with no progress, otherwise the command is re-inserted and nothing else advances. The fixture's `host_turn` runs publication under `policy.maintenance` (64 items, 64 KiB copy), unlike the earlier unbounded caller grant.

Introducing change: confirmed only as a family, not a single line. Candidates (all today, concurrent): (a) the resident birth service in `🧰️framework/🔨️modules/🧵️job/⏱️context` (`admit_original_receiving_scope`, `exclude_original_birth`, `admit_original_return_table` now returning false once the table exists, still carrying another agent's `[DEBUG] svc ...` probes), which changes how a birth turn is funded; (b) `MountedOwnerPolicyV1::INTERACTIVE_COMPONENT` copy grant 32768 -> 65536. The window-config birth was never exercised before because the fixture did not register windows with a config partition until L17a/L17b made `window_view` real; the host itself pumps `ensure_for_view` before every capture, so a keyed command normally finds a born partition.

Fix applied (harness, mirrors the host): `Puzzle3dApp::ensure_window` (`✏️s/.../🧰️context/🏗️raw-app-fixture/🦀️.rs`) now births the window's config partition under the caller's full grant right after the transient partition (new private `birth_window_config`, extracted from `window_config_generation`). A keyed command then sees `ensure_for_view` -> `Ok(true)` with no progress and proceeds, instead of having to birth inside the publication grant.

Expected effect: the latest-wins command passes the `ensure_for_view` gate and the app quiesces. Unverified. If the stall persists, the next probe is a `[DEBUG]` line in `advance_latest_wins_command_one` before the `ensure_for_view` match (ensured value, `cx.retained_progress()==before`, `cx.should_yield()`); a stuck `Ok(false)` there points at the job-context resident birth service work, owned by the agent whose probes sit in `owned/🦀️.rs`.

## Files changed
- `🧰️framework/🔨️modules/⏪️time-travel/🎮️decision/🗑️discard/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️context/🏗️raw-app-fixture/🦀️.rs`
- this report

## Verification status
Not compiled, not run. No `[DEBUG]` logs added by this lane.
