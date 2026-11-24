# L37: 3D Brush Search, Hover Storm, Residue Class and Accept Menu

This is source analysis only. Nothing was compiled or run, so every fix below is UNVERIFIED.
The log is `🗑️generated/l32-3d.log`.

Abbreviations:
- E3 = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`
- E5 = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`
- P = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`

## 1. Hover storm and residue class: a deadlock in reserved lease admission (NOT fixed, design decision)

Affected tests:
- `vortex_hover_storm_admits_…`
- `vortex_hover_then_open_vortex_suggestions_publishes_menu`
- `interaction_admits_in_every_residue_class_while_a_reserved_verb_stays_pending`

These failures were already in l12 (2026-11-22), so this is not a recent regression.

### What the fault says
Every fault reads "original action fixture dispatch exceeded its authored turn bound". In each one:
- fuel is still about 1,999,959, so the turn did not run out of fuel;
- `args=false` and `metadata=true`;
- every gate is idle except `action_source=true`.

So `handle_action` answered `Ok(None)` on every one of the bounded retries.

### Why it stalls
1. Every framework-reserved verb (`interactionHover`, `interactionSelect`, `noteShellCommand`, `undo`, …) asks for its lease with `admit_original_tool_lease(verb, meta, false, cx)`. The `false` makes it a NON-keyed lease (P/🧵️instance/📨️dispatch/📥️staging/🎟️producers/🦀️.rs:116).
2. A non-keyed lease is exclusive per document: `ToolCancellationIssuer::begin_original` allows one per document slot (P/♻️cancellation/🎟️issuer/🦀️.rs, non-keyed branch of `begin_original`). If the slot is already held, `begin_original`:
   - calls `previous.token.cancel_now()`, which cancels the verb that is still pending;
   - returns `Ok(None)` until the earlier lease is gone (the final step repeats the `state.get(slot).is_some() → Ok(None)` check).
3. The earlier lease sits in `pending_reserved`. Two things could free it, and neither runs inside the dispatch retry loop:
   - Settling its Isolated job.
   - `retire_pending_reserved_latest_wins`. This only runs AFTER admission (P/🧵️instance/🧷️command-routes/📥️staging/🧊️reserved-routes/🦀️.rs:94), so it is circular. The retired lease is then released in `step_framework_reserved_commit`, which only `advance_typed_operation_publication` drives.

   The result is a livelock until the turn bound is hit.

In production this has two effects:
- Reserved gestures on one document are serialized.
- A pick cancels a pending `noteShellCommand` or `undo`. That is the bug class of #44-pre that the residue law protects against.

### Recommended clean fix (not applied, because it changes framework-wide semantics)
- (a) Reserved spawns take a keyed lease: change `false` to `true` at producers:116, as the clipboard emit producer already does at :167. Concurrent reserved verbs then coexist, each with its own operation scope.
- (b) `typed_operation_slot_is_vacant` (P/🧵️instance/📄️docs/📥️staging/🧾️typed-operations/🦀️.rs) must also refuse a residue class still held by a queued `pending.reserved_commits` permit. `keyed_operations` is a direct-mapped `ArtifactFixedRegistry`, so without this the next admission would fault with "exact keyed cancellation operation slot is unavailable".
- (c) A true latest-wins hover must not mint a new lease per superseded hover. Otherwise 70 unsettled hovers exhaust the 64 residue classes while their retired commits wait for publication turns. The superseding admission should reuse the pending permit of the same interaction verb, or retire it synchronously before it mints.

(a) on its own fixes the residue law. The storm law needs (a), (b) and (c).

## 2. Brush search stuck at stage 0 (cause not proven)

Affected tests: every `pump_brush_suggestions` law.

### Ruled out
- The l38 lead does not apply to the brush. `pending_effects` gets an `ArtifactPendingView`, whose `tool_run()` is the bound `ToolRunPollView` (P/📜️declaration/👁️view/🔭️artifact-view/🦀️.rs:36). The run is also admitted (`Running`), which confirms the start effect works.

### What changed
Before L17b, this cluster failed with "tool run receipt differs from its original caller authority" (l12). L17b moved `host_turn` publication to `policy.maintenance` (the 64-item `INTERACTIVE_COMPONENT` grant). Since then the run never publishes a tick: `total=None` means no tick was ever received.

### Candidate gates
Each one answers no-progress silently, every turn, under that grant:
- `retire_tool_runs_until` → `ToolRunLedger::retire_step` (P/⏯️tool-run/📒️driver-and-ledger/🦀️.rs:493).
  - It answers `Some` for any retirement demand.
  - It also pops LIVE `trace_windows` (:532) as if they were retiring.
  - It returns before the job steps.
- `refold_tool_run` returning `true`.
- `OriginalToolRunNativeState::initialize` and the `receive_with_domain` phases (P/⏯️tool-run/🎞️tick/🛬️native/🦀️.rs:23). Each one refuses quietly when `funds(cx, d)` is false.
- `ToolRunJobSlot::drive` → `origin.admit` (P/⏯️tool-run/⏱️limits-and-job-slot/🦀️.rs:103).

### Next step
Run a `[DEBUG]` build that prints, in `drive_tool_run_turn` and `step_tool_run_job`, which early return fired each turn, together with `cx.retained_grant()`. Test line: `-p semio-s-artifact-puzzle-3d-editor --test editor-surface the_armed_brush_runs_a_read_only_search`.

## 3. `accept_suggestion_closes_menu_even_when_placement_fails` (fixed)

### Cause
`Puzzle3dAcceptSuggestionWork` never cleared the window-transient `suggestion_menu`:
- None of its `Complete` paths did, neither success nor refusal.
- Only the `Candidate` stage released the brush link menu.

So a refused accept, or any accept at all, left the popup open.

### Fix
A new `finish` method is now the only completion of the work. On every outcome it:
- releases the brush link menu;
- returns `CompleteWithEphemeral` with the addressed window's transient minus `suggestion_menu`.

The puzzle testing step maps `CompleteWithEphemeral` to `Complete`, so the step-loop laws are unaffected.

## 4. Coordinator task: migrating to `original_tool_run()` (done for 3D and 5D)

The owned `ToolRunView` is never bound in a mounted app, so `doc.tool_run()` / `context.tool_run()` were always `None`. The effects:
- the fill abort binding fell back to `engagementAbort`;
- the fill `loading` flag never showed;
- provisional instances were never marked;
- 5D open/close-vortex-suggestions ran `run_effects` blind.

The 3D and 5D editors now read `ArtifactView::original_tool_run()` / `ArtifactOwnedToolJobContext::original_tool_run()` (`semio_plugin::OriginalToolRunView<'_>`, which is `Copy`).

New code:
- `impl From<OriginalToolRunView<'a>> for ToolRunPollView<'a>` in P/📜️declaration/⏯️tool-run/🦀️.rs, for the 5D suggestion commands.
- Provisional marking reads `run.provisional_entities.contains_key(..)` on the `EntityMap`. `geometry_jsons` and `Puzzle3dInstanceResidency::refresh` now take `Option<OriginalToolRunView<'_>>` instead of `&BTreeSet<u64>`.

The brush `pending_effects` keep `ArtifactPendingView::tool_run()`, because that is the bound poll view.

### Not done
Deleting the owned `ToolRunView`, `ArtifactView::tool_run` / `with_tool_run` and the owned context `tool_run` / `with_tool_run`. Other editors still use them: the WFC editors (2d, 3d, bitmap, grid2d, grid3d), generation2d/3d (editor, viewer, preview-eval), remodeling, the energy simulation-session, and the framework tool-run tests. Deleting the type blind would break those crates, so it needs its own lane that migrates them all.

## Files changed
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🎬️action/💡️accept-suggestion-work/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️.rs` (unused import)
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎟️session/🎬️action-context/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎟️session/🏘️instance-residency/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎟️session/🗃️session-registry/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎟️session/🧪️tests/🔬️unit/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🎬️action/🪟️window-command-work/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🎬️action/🧠️precompute-command-work/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️play-app/🦀️.rs` (lines 525, 541, 555, 618; the topology bump stays unconditional)
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🍞️geometry-cache-and-step-budget-laws/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️mutation-latency/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` (unused import)
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🧪️tests/🔬️unit/🦀️.rs` (unused import)
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/✍️artifact-editor/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🍠️play-app/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎬️dispatch/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔓️open-vortex-suggestions/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔒️close-vortex-suggestions/🦀️.rs`
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/⏯️tool-run/🦀️.rs` (`From` conversion)

## Also seen, not mine
`open_vortex_suggestions_records_explicit_window_id` fails because the fixture's `action_args` (`🎛️settle-and-dispatch-drivers`) overwrites the explicit `windowId` argument with the addressed window. That key is the shell's address, so the law's premise needs its own argument name.

## 5. Follow-up: reserved-lease fix applied (coordinator decision)

Unverified: nothing compiled or run.

### (a) Keyed leases for reserved spawns
`admit_framework_reserved_spawn` now calls `admit_original_tool_lease(verb,meta,true,cx)` (P/🧵️instance/📨️dispatch/📥️staging/🎟️producers/🦀️.rs:116).
- Reserved verbs on one document coexist, each with its own `keyed_operations` scope.
- A new verb no longer cancels a pending `noteShellCommand` or undo.
- `admit_original_tool_lease` has only two callers, :116 and the clipboard emit producer at :167, and both are now keyed. Its `pending.keyed != keyed` invariant therefore cannot trip.

### (b) Queued reserved commits occupy their residue class
`MediaTransfers::typed_operation_slot_is_vacant` (P/🧵️instance/📄️docs/📥️staging/🧾️typed-operations/🦀️.rs) now also treats a class as occupied while a queued `pending.reserved_commits` permit holds it. `keyed_operations` is direct-mapped, so without this the next mint would fault with "exact keyed cancellation operation slot is unavailable".

Callers checked:
- `admit_typed_operation_slot`, which serves the typed command, the reserved lease, the action preview and the emit producers;
- `live_typed_operation_slots`;
- the WP3 retained-command test host (P/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs:196, :220, :500), which keeps no reserved commits queued.

No 2D, 3D, 5D or hub plugin calls either function.

### (c) Latest-wins reuses the superseded permit
The new `PendingReserved::superseded_reserved_interaction` (P/🧵️instance/👷️jobs/📥️staging/🧷️command-routes/🦀️.rs) finds a pending interaction to reuse. It must match on all of:
- the same verb;
- the same actor and instance;
- the same window id;
- the same expected document revision;
- the same store generation as the permit.

When one is found, `dispatch_framework_reserved_action` (P/🧵️instance/🧷️command-routes/📥️staging/🧊️reserved-routes/🦀️.rs):
- replaces only that pending's `args`, keeping its retained `ActionMeta` and permit, so no unpaid metadata drop happens;
- re-emits `SpawnJob` under the same job id with the new input.

What happens to the job afterwards:
- If the earlier spawn is still live, the host ledger drops the duplicate spawn (`JobLedger::begin` answers `false`). The earlier job then commits the newest args, because commits read `pending.args`.
- If the earlier job already handed its output to the app, the pending entry is gone and the new gesture mints normally.
- Otherwise (no match) the old mint path runs, including `retire_pending_reserved_latest_wins`.

A storm of N unsettled hovers now holds 1 lease and 1 residue class.

### Fixture `action_args`
`action_args` (E3/🧪️tests/🧰️context/🎛️settle-and-dispatch-drivers/🦀️.rs) now keeps an explicit `windowId` argument. Routing still follows the addressed view (`puzzle3d_addressed_window_id` prefers `view.window_id`), so `open_vortex_suggestions_records_explicit_window_id` reads the menu's own `windowId`.

### [DEBUG] probe (temporary, remove once the gate is known)
- `debug_tool_run_gate` lives in P/⏯️tool-run/🧵️instance/🦀️.rs.
- It is called at every early return of `drive_tool_run_turn` (P/⏯️tool-run/🧵️instance/👀️generation-watch-and-turn-driver/🦀️.rs): should-yield, held-receipt, retiring, refold, unselected.
- It is also called at every early return of `step_tool_run_job` (P/⏯️tool-run/🧵️instance/🎟️job-admission-and-finalize-turn/🦀️.rs): should-yield, no-entry, native-initialize, no-job, failed-or-continued, acknowledge-outcome, preview-receiving or preview-received, outcome-unfunded, drive.
- It prints `[DEBUG] tool-run gate <previous> x<turns> -> <next>` only when the gate changes, at most 256 lines per thread.
- Find it with `grep -n "\[DEBUG\] tool-run gate" 🗑️generated/<log>`.

### Files changed in this follow-up
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📨️dispatch/📥️staging/🎟️producers/🦀️.rs`
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📥️staging/🧾️typed-operations/🦀️.rs`
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/👷️jobs/📥️staging/🧷️command-routes/🦀️.rs`
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🧊️reserved-routes/🦀️.rs`
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🧵️instance/🦀️.rs` ([DEBUG])
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🧵️instance/👀️generation-watch-and-turn-driver/🦀️.rs` ([DEBUG])
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🧵️instance/🎟️job-admission-and-finalize-turn/🦀️.rs` ([DEBUG])
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️context/🎛️settle-and-dispatch-drivers/🦀️.rs`
