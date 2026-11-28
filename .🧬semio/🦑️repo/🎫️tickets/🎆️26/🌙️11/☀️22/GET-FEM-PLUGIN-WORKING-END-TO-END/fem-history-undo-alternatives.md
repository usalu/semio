# FEM history: undo wedge, stale publications and Create Alternative (2D and 3D)

Scope: 2D W1/D4/D5/D6 (Ctrl+Z and History Undo do nothing, app wedged afterwards), the gumball drag `transient publication generation is stale` / `native-return-refused` (U7/T1), and 3D F7 (History > Create Alternative: `module.vcs: no checkpoint for alternative`).
Scripts and logs: `hu-law.sh`, `hu*-body.js`, `hu*.run.js`, `page-driver-hu.mjs` in this folder; logs in `🗑️generated/hu/`.

## Verdict

| Item | Status |
|---|---|
| Wedge mechanism (any per-operation publication failure poisons the whole actor) | Root cause found in source, fixed in the framework, native laws green (2D) |
| T1 gumball overlap (`transient publication generation is stale`) | Reproduced natively in 2D (`stream_ticks_in_flight_...`), now terminates as a benign `interactive-job.stale` of that one tick, app stays live; law green |
| F7 Create Alternative without a checkpoint | Already fixed in store source (11-26 20:51, AFTER the served wasm of 11-26 18:15); store laws green, 2D app laws green; needs the coordinator's wasm rebuild to show in the browser |
| W1 browser undo | Not reproducible with a trustworthy browser run (see Browser), basic undo/redo native laws green in 2D; the wedge fix above is the only mechanism found that turns a stale refusal into a dead actor |
| 3D native app laws | NOT verified: the 3D fixture boot spins in the store initializer on the current tree (other agents' in-flight snapshot-receiving work), see Open |

## Root cause

1. The browser bridge (`🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts`, `poll`) holds ANY thrown guest failure of a `poll` call as the actor's native refusal (`originalSource.holdNativeRefusal(failure)`). From then on `exclusive()` rethrows it for every call (`original-turn.native-return-refused: <tag>: <fault code>: <message>`). That is exactly the console text of W1, T1 and F7 (`tree window refresh failed ... native-return-refused: semantic: interactive-job.stale`): later refreshes are only the echo of the first failure. So one failed turn kills selection, tree, results and undo for good, which is the reported "wedge".
2. A turn fails when the instance step returns `Err`. The reserved commit path already answers a plain refusal as that command's own sealed fault (`step_framework_reserved_commit`, `OriginalTurnFailure::plain_refusal`). The typed-operation publication path did not: `advance_typed_operation_publication_unit` returned every `Err` of `publish_*_run` as the turn failure, although its own docs (`fault_typed_operation_worker`, wave B16) state "a per-operation failure becomes that operation's fault, never the turn's".
3. The stale refusals feed that Err: a typed operation captures `transient_generation` / `presence_generation` at mount (`capture_typed_command_roots`); when another operation publishes first (a fast gumball stream: several ticks mounted before the first publishes, or an undo/edit moving the roots), `begin_publish_one` rejects with `transient publication generation is stale`, and the publication unit returned it as `plugin.internal` via `plugin_sdk_fault(reason)`. The document-root variant (`cancel_stale_publication`) was already benign; the ephemeral lanes were not.
4. Why 3D looked healthy: nothing 2D-specific exists in the FEM crates (the gumball command, transient schema, history declarations and `resultAnimationTick` are shared or twins). The difference is timing (the 2D solve, results playback and tree refresh overlap an undo more often); the served 3D build also predates nothing relevant here. I found no 2D-only code path, so the fix is generic and applies to both.

## Fix (framework, one mechanism)

- `🏛️ownership/⚠️failure/🦀️.rs`: `PluginLifecycleFailure::plain_refusal()` (same contract as `OriginalTurnFailure::plain_refusal`: a semantic fault that spent and retained nothing and carries no receipt, context or work refusal).
- `🧵️instance/👷️jobs/📥️staging/🧾️typed-operations/🦀️.rs`: `fault_typed_operation_publication_with(operation, &Fault)` (the stall fault builder generalised to keep the original code and message; the old `fault_typed_operation_publication` now calls it).
- `🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🔁️stall-witness-and-unit-advance/🦀️.rs`: the `Err` arm of the publication unit answers a plain refusal as that operation's own terminal fault page (lease cancelled, ordinary cancelled-publication close retires its owners) and returns `Ok`; a non-plain failure still propagates as before.
- `🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs`: `ephemeral_rejection_fault`: a rejected presence/transient publication whose reason is `... generation is stale` becomes the benign, already localised `interactive-job.stale` (EN and DE text in `🎠️kernel/📜️history`), not `plugin.internal`.

## Laws (native, all green unless noted)

| Law | Where | Result |
|---|---|---|
| `only_a_plain_fault_is_a_plain_refusal` | framework `semio-plugin-ownership` (`⚠️failure/🧪️tests`) | ok (`law-own.log`) |
| `stream_ticks_in_flight_together_never_wedge_the_gesture` | 2D `editor_app` (gumball tests) | ok (`law-2d-g1.log`); the overlap is real: the pump surfaced `transient publication generation is stale` for a tick (earlier probe `law-2d-f.log`), the app then commits one row and undo restores the node |
| `undo_and_redo_round_trip_then_the_app_keeps_editing_2d`, `create_alternative_without_an_explicit_checkpoint_branches_from_the_head_2d`, `create_alternative_with_pending_edits_checkpoints_them_2d`, `commit_checkpoint_works_with_and_without_pending_edits_2d` | 2D `editor_app_2`, new module `🧪️tests/🕰️history-undo-and-alternatives` | ok, 4/4 before the framework edit (`law-2d-d.log`) and again after it (`law-2d-g5.log`) |
| `create_alternative_with_no_edits_and_no_checkpoints_branches_from_a_baseline_checkpoint` (existing) and `a_second_alternative_and_an_empty_commit_never_fault_without_a_checkpoint` (new) | store `semio-os-store-tests-05` | ok (`law-store2.log`) |
| 3D twins: `stream_ticks_in_flight_together_never_wedge_the_gesture` (3D `editor_app`), `undo_and_redo_..._3d`, `create_alternative_..._3d`, `commit_checkpoint_..._3d` (3D `editor_app_2`) | written, wired, compile | NOT green: the 3D demo fixture never boots (store initializer spin), see Open |

Helpers added: `dispatch_unsettled` / `settle_pending` in the 2D context (`🧪️tests/🧰️context`) and in the 3D `editor_app` context, to hold typed operations in flight like a fast pointer.

Note on the law design: the overlap law accepts only `interactive-job.stale` as the end of an overlapping tick and then requires a live app (commit as one row, undo exact). It was not A/B-run against the unfixed tree (each cycle is 30-40 min on this host); the unfixed behaviour is by construction a turn failure that the bridge holds.

## F7 detail

`ArtifactCommand::CreateAlternative` (store `🚦️cursor-admission-dispatch-and-apply-batch`) commits a baseline checkpoint when the document has none (no edits) or commits the pending edits as the checkpoint, then branches from `current_checkpoint_id`; `author_head_alternative` (event-log lane) does the same. The error text in the matrix comes from the wasm built before that change. Reserved routes answer `NothingToUndo`/empty-checkpoint refusals as benign results; Commit Checkpoint with nothing pending is `cannot create an empty checkpoint` (refusal, not a fault). Check In is a shell controller (`framework.checkin`, disabled while remote is `detached`), not a store command, so it has no native law here.

## Browser

I started my own driver (`page-driver-hu.mjs`, port 6231, 2D on 6086). The host was at ~46 cargo/rustc processes: a page reload took 200-275 s, so undo timing runs are not trustworthy. Findings that do hold: Ctrl+Z pressed while the material `<select>` of the Inspection panel still has focus emits no console line at all (n=0 messages, `hu4.out`), which matches "blur focus" in D4; the earlier `health()` probe reports `STUCK` whenever the Inspection panel is not the visible panel (History replaces it), so several earlier `STUCK` readings (`ix-undo*.out`) are false positives; the History Undo click left Redo disabled with Results at `Pending 0 / 0` (`hu5.png`), i.e. the re-solve was still queued. Nothing could be verified against the new fix because only the coordinator builds the wasm.

## Open

- The wasm must be rebuilt by the coordinator for the browser to show the fix and F7.
- 3D native laws: `fem3d_demo_app()` (and 2D `fem2d_empty_app()` via `setActiveExample`) spin in `[DEBUG] store initializer call=...` with `step_demand copy_bytes 4138 capacity_bytes 2384` on the current tree (3D `editor_app` gumball law: `call=791617` after more than 60 s, no completion; 2D `remove_selection_with_no_matching_ids`, an existing test using `fem2d_empty_app`, also ran past 60 s with `call=947873`), and a first 2D run failed at `artifact-app-laws:70:387 assertion (20,20) == (0,0)` (those multi-MB logs were deleted after quoting). All happened before and after my edits, in code another agent is changing (store initializer / snapshot receiving), so I did not touch it. The 2D laws were moved onto `fem2d_mounted_app()`, which boots. Re-run the 3D laws (`hu-law.sh 3d <tag> history_undo_and_alternatives editor_app_2`, `hu-law.sh 3d <tag> stream_ticks_in_flight editor_app`) once that lands.
- Gumball accumulation (T1 "22 of 60 px"): a tick refused as stale is dropped, so deltas of dropped ticks are lost unless the tool sends cumulative offsets. The wedge (the likely cause of the partial move) is fixed; whether the UI's tick payload is cumulative was not checked.
- Pre-existing red store tests with the old harness: `reconcile_alternative_pins_the_latest_checkpoint...`, `document_text_round_trips_with_an_active_alternative...`, `space_member_checkout_switches...` fail with `original work requires its original caller loan` (the `Apply` helper of that harness), unrelated.

## Files

Framework: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/` `🏛️ownership/⚠️failure/🦀️.rs` and `⚠️failure/🧪️tests/🦀️.rs`; `🧵️instance/👷️jobs/📥️staging/🧾️typed-operations/🦀️.rs`; `🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🔁️stall-witness-and-unit-advance/🦀️.rs`; `.../📬️typed-operation-unit/🦀️.rs`. Store law: `🏪️store/🧪️tests/🌅️bin-05/🔬️unit/📄️document-text-wire-and-command-error-laws/🦀️.rs`.
FEM (2D and 3D `…/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/`): `🧪️tests/🕰️history-undo-and-alternatives/🦀️.rs` (new, each), `🧪️tests/🔬️app-2/🦀️.rs` (module line), `🎮️commands/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs` (overlap law), 2D `🧪️tests/🧰️context/🦀️.rs` and 3D `🧪️tests/🔬️app/🦀️.rs` (helpers).

## Status of the last run

- Green natively: ownership law, store laws (2), 2D history laws (4/4, before and after the framework edit), 2D gumball overlap law.
- Not run green: all 3D app laws (boot spin, see Open); a 3D rerun of my build was cut short when I stopped my own processes.
- Cleanup: my `C:	\hu2d`, `hu3d`, `huown`, `vc3` build dirs and temp dirs removed, my driver (port 6231) stopped, large logs deleted; kept `🗑️generated/hu/law-*.log` that the table cites. `[DEBUG]` probes added by me were removed (the overlap probe in the 2D gumball law is gone).
