# L38 3D panels, manifest and task-resume (source-only, not built)

Input: `🗑️generated/l32-3d.log` and `🗑️generated/l32-3d-failed.txt`. Nothing was compiled or run, because a build was holding the RAM.

## A. Fill cluster (panels)

### A1. "original projection must be driven or returned through its resident frame" (4 tests)
- Where it is raised: `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🎟️admission/🦀️.rs:65`. `RetainedCloneSourceAdmission::advance` refuses the phases `projectionStart|projection|projectionFailure|projectionFailed`. Every admission goes through `projectionStart` after `identity` (see `phase()` at line 49). An owned admission (`admission_owned`, so T == A) must call `project_owned` there.
- The established helpers already do this: `🔌️plugin/👷️job/🛬️receiving/🏗️steps/🦀️.rs:61-69` (`admission_demand`, `admission_step`, `admission_return_demand`, `admission_settle`). The store wire custody, the digest code and others follow the same pattern.
- Root cause: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/🎮️operations/🦀️.rs` admits every tool-run provisional op row (`tick.append_ops`) through `RetainedCloneSource::<Vec<u8>,Vec<u8>>::admission_owned`, then calls `.advance(effect)` for every phase other than `ready`. This happens at line 123 (the ingest path) and line 152 (`step_row`, the close_active path). It never runs the identity projection. Its `close_step` (line 173) also calls `return_inputs` while a projection answer is held, and that call refuses. The code has been unchanged since at least 2f4d8f8 (Nov 11). Only a tool run that emits provisional ops reaches it, which is why only the fill tests fail. Fill has been red since l12 for this reason.
- Fix (applied):
  - Demand: `"projectionStart" => owned_projection_demand()`.
  - Step: `"projectionStart" => project_owned(effect)` at both sites.
  - `close_step` first runs `discard_projection` when `projection_result()==Some(true)`, then `return_inputs`, the same as `admission_settle`.

### A2. `set_object_kind_weight_declares_fill_options_ui_scope`: Fault message "…retirement::UnorderedMap<String, f64>"
- The message is `std::any::type_name::<Self>()`, which is the default `RetirementCursor::close_into` refusal (`🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:135`).
- `Puzzle3dRuntime.object_kind_weights: HashMap<String,f64>` (`✏️editor/🎚️config/🦀️.rs:181`) becomes non-empty after `setObjectKindWeight`. Retiring it reaches the `UnorderedMap` cursor.
- `UnorderedMap`, `UnorderedSet` and `OrderedSet` each have an inherent `close_step(body,effect)->RetirementStep`, but they never used `native_retirement_reply!()`. Their siblings do: Leaf, Bytes, OrderedMap, Optional, FixedArray, Deferred and Paged. The support check still answers true for `HashMap` and `HashSet`, so the owner is admitted and then refuses at the first non-empty close.
- Fix (applied): added `native_retirement_reply!();` to all three cursors. `OrderedSet::terminal_is_empty` now also requires an empty head slot (`&&self.1.is_none()`), matching `OrderedMap`. Without that, a popped head would read as terminal and then be dropped.

### A3. `fill_tool_wins_the_world_lane_over_a_select_window_utility`: "ordered-map root must be explicitly retired before drop"
- This is a sync test. `view.active_utility_by_window_id` is an `OrderedMap` (`🛂️manifest/🖼️view-model/🦀️.rs:39`). The test inserts into it and drops the view at the end of the test.
- Fix (applied): `retire_view` in `🧪️tests/🧰️context/🏗️raw-app-fixture/🦀️.rs` is now `pub`, and the test calls `retire_view(view)` at its end.

### A4. `cancelling_a_stepping_fill_job…` "abort is enabled while the run is still starting", plus the engagement-abort law after A1 (NOT fixed, cross-editor migration)
- The 3D editor reads `doc.tool_run()` / `context.tool_run()`. These are the owned `semio_plugin::ToolRunView` fields set only by `with_tool_run`. Since 1e27248 (Oct 28) the framework no longer calls `with_tool_run` anywhere in production. It binds only `with_original_tool_run(...)` (borrowed `semio_plugin::OriginalToolRunView`). The binding sites are:
  - `🧵️instance/🖼️render/📥️staging/🪟️projections/🦀️.rs:43,91,130`
  - `🪢️members/…/🖼️render-surface:129`
  - `📨️dispatch/…/⌨️typed-command:211`
  - `🧷️command-routes/…/🚦️actions:570`
- As a result, `ArtifactView::tool_run()` is always `None` in a mounted app. That breaks every one of these:
  - `main::engagement` `on_abort` (`🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:561` via `fill_tool::abort_action`)
  - the fill count measure
  - provisional rendering (`🧩️play-app/🦀️.rs:618`)
  - `engagementAbort` (`🧵️retained/🎬️action/🪟️window-command-work/🦀️.rs:131`)
  - **`utilities::brush::run_effects(&mut owner.brush_suggestions, doc.tool_run())` (`🧩️play-app/🦀️.rs:182`)**. This is very likely the root cause of l37's "brush search never leaves stage 0". Relay this to l37.
- Required clean fix: migrate the 3D editor to `doc.original_tool_run()` / `ArtifactOwnedToolJobContext::original_tool_run()`, which return `Option<OriginalToolRunView<'_>>` (Copy, with `provisional_entities: &EntityMap` = `RetainedOrderedMap<u64,()>`). Then delete the owned `ToolRunView`, `tool_run` and `with_tool_run` from `ArtifactView` and from the owned context. The functions that need changing are:
  - `fill_tool::{count_measure,measures,live_fill_run,abort_action}`
  - `main::engagement`
  - `brush::run_effects` and the brush window code
  - `Puzzle3dActionCtx.tool_run`
  - `commands/🦀️.rs:125`
  - `geometry_jsons` (takes `&BTreeSet<u64>` today)
- The 5D editor calls the same 3D functions and is in the same state (`🖐️5d/…/✍️artifact-editor/🦀️.rs:134,409,442,463,479,502`). WFC, generation3d and the 5D play-app do too. The energy editor is the only one already on `original_tool_run()`. This touches brush (l37) and 5D, so I did not do it blind.

## B. `two_instances_converge_disjoint_object_edits_via_backbone` (not fixed)
- The close ladder is stuck on rung `OwnedStores`. It quotes `RetirementDemand{release_bytes:56,depth:3}` against a grant that admits it (items 1, release 1 MiB), with zero progress for 4096 idle turns. A step that is admitted but never advances points to a cursor that refuses or answers `BudgetExhausted` without progress inside the store owners.
- Rerun after the A2 fix first: a non-empty `HashMap`/`HashSet` in an owned store used to refuse at `close_into`, and the ladder may be swallowing that refusal as no progress. If it is still stuck, instrument the `OwnedStores` rung (`🔌️plugin/🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs:544`) and print the selected owner's type name.

## C. `window_options_are_local_to_the_window_instance…`: "Window owner must declare its original native Pack receiver"
- The default `WindowConfigOwner::native_pair_demands` refusal is at `🔌️plugin/🪟️window/🎚️config/📜️contract/📐️owner-contract-and-mutation/🦀️.rs:23`. `Puzzle3dWindowConfigOwner` (`✏️editor/🪟️window/🦀️.rs`) never declared it. Puzzle 2D and 5D did.
- Fix (applied), mirroring 2D/5D:
  - `impl OriginalArtifactPackReceiving for Puzzle3dWindowConfig` (Receiving = `OriginalSnapshotRecordReceiving<Self>`)
  - `native_pair_demands`
  - `admit_native_pair`
  - `native_operation_wire()` with `with_original_binary_receiving()`

## D. Manifest local interaction reads (not fixed)
- `Started` is published, then 65536 turns pass with `bytes=0` and no page and no `Closed`. The actor stays runnable.
- The render-pressure variant fails earlier with `WorkLimit "original fixture render tree release remained pending under its authored caller"` (`🔌️plugin/🧪️testing/🧪️artifact-app-instance-laws/🔎️declared-verb-probing/🦀️.rs:213`).
- I could not settle these from source alone. They need a `[DEBUG]` trace of `next_advanceable_typed_operation` and of the local-interaction publication stage per unit. It is possible that they share A4: local interaction reads include the tool-run view and the provisional set.

## E. Task-resume (not fixed, framework ↔ fixture contract)
- E1 and E2: "original command scope requires its actual decoder recipient" is raised by `claim_scope` (`🔌️plugin/🧵️instance/📨️dispatch/📥️staging/📣️host-event-routing/🦀️.rs:9`). It needs `cx.native_receiving_identity()`, which exists only once the context's decoder holds a receipt, meaning after at least one native receive (`🧵️job/🛬️receiving/🎟️state/🦀️.rs:36`). The test calls `advance_typed_operation_publication` with a fresh `OriginalFixtureScope::admit().cx()`, whose decoder has never received anything, so the first unit fails.
- Two possible fixes:
  - (a) make the scope claim admit or prime the decoder receipt before reading its identity;
  - (b) have the resume delivery decode first and claim afterwards.
  - Do not patch the test.
- E3 ("cancel after 0 units left the queue or the typed source owned"): while the context is cancelled, `claim_scope` never claims (`!cx.is_cancelled()` guard), so queued resumes are never retired. The cancel path must return queued resumes without a scope claim.

## Files changed
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/🎮️operations/🦀️.rs` (A1)
- `🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs` (A2)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️context/🏗️raw-app-fixture/🦀️.rs` (A3, `pub fn retire_view`)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️panels/🪣️fill-and-distribution-laws/🦀️.rs` (A3)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs` (C)

## Expected after a rebuild
- A1 fixes the publication failure in the 4 fill tests. Their later assertions on provisional rendering and Escape still need A4.
- A2 should fix `set_object_kind_weight…`.
- A3 should fix `fill_tool_wins…`.
- C should fix the window-pack reload.
- `cancelling_a_stepping_fill_job…` and `engagement_abort…` stay red until the A4 migration is done.

## E follow-up: task-resume claim and cancel fix (applied, source-only, not built)

### Where the refusal comes from
- The E1/E2 refusal is raised by `CommandLease::claim`, not by `claim_scope`. The resume lane calls `claim_original_command_scope` (`🧵️instance/🔌️plugin-app-impl/🦀️.rs:312`).
- When the context has no `native_receiving_identity`, `claim` called `pay(cx)`. Once the turn was funded, `pay` raised the invariant "original command scope requires its actual decoder recipient".
- The identity is `(decoder ledger identity, recipient)`, taken from the decoder receipt (`🌱️value/🛬️decode/🫴️recipient/🔁️continuation/🦀️.rs:44`). That receipt exists only after the caller's first native receive.
- A lane whose very first work is a task resume therefore could never claim. This happens with a fresh `OriginalFixtureScope` or `OriginalFixtureCaller` context, and on a host turn that has not decoded anything yet. The framework's own `🔬️plugin-runtime-plugin-builder-contract-part-6/🧵️spawned-task-resume` laws use the same fresh-caller shape.
- E3: once the context is cancelled, `pay` answers false, so the lane never claims. `task_resume_step` (which would run `TaskResumes::plan(cancelled=true, …)`) was never reached, and the queue stayed owned.

### Fix
- `📣️host-event-routing/🦀️.rs`:
  - New `bind_decoder(cx)`. It births the caller's actual decoder receipt through one funded empty `cx.with_native_receiving(…, |_,_| Ok(()))`, follows the `native_received` pattern (`👷️job/🛬️receiving/🏗️steps/🦀️.rs:149-158`), and reconciles the receive with `receive_original_receiving_work`. While the receive is still being admitted, the claim answers `Ok(false)`.
  - `CommandLease::claim` and `claim_scope` call `bind_decoder` when there is no identity, instead of paying and refusing.
  - The identity law is unchanged. A lease or scope is still keyed and checked only by `(binding, actual decoder identity)`, and the invariant in `pay` and `claim_scope` stays.
- `🔌️plugin-app-impl/🦀️.rs` (after the `release_idle` block, about line 311):
  - A new branch handles a cancelled context with a runnable resume queue and no delivery in flight. It retires the queue through `TaskResumes::plan(true, source_pending, delivering)` without any scope claim. Nothing in that path touches the shared typed source: `plan` only clears the queue when the queue holds no source.
  - A delivery that is in flight was started by a lane that holds the lease, so it still closes through the normal claim and `TaskResumeStep::Close` path.

### Callers checked
- `claim_scope` has exactly one caller: `HostEventRoutingCtx::claim_original_host_event_scope` (`📣️host-event-routing/🦀️.rs`), used at `🔌️plugin-app-impl/🦀️.rs:301`.
- `CommandLease::claim` is reached only through `claim_original_command_scope`, at `🔌️plugin-app-impl/🦀️.rs:312` and inside `capture_original_host_event_route`.
- No 2D, 3D, 5D or hub plugin source calls either one. The only plugin-side references are test assertions on `host_events.resumes` (3D task-resume and the framework spawned-task-resume law).
- Host-event cancel was left as it was. While cancelled, its unclaimed routes have no source in flight, and the close ladder retires them (`close_original_host_event_source`).

### Files changed (E follow-up)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📨️dispatch/📥️staging/📣️host-event-routing/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs`

### Risk to verify on the next build
- Whether an empty `with_native_receiving` actually leaves a decoder receipt. It goes through `OriginalDecodeState::receive`, which creates the continuation through `NativeDecodeControl::new_retained(..).pause()`.
- Whether the empty receive is accepted by the return-scope accounting, which `native_received` handles the same way.
- If either one refuses, the refusal now carries the message "original command scope decoder binding refused" instead of the invariant.
