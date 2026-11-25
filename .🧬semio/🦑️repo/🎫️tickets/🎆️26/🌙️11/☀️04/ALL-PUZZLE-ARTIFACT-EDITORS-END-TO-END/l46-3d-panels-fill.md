# L46 3D Panels Fill Cluster (source-only, not built)

This is source analysis only. A native batch held the RAM, so nothing was compiled or run, and every fix below is UNVERIFIED.

The L41 panic texts for these tests are gone: `🗑️generated/l41-3d.log` is being rewritten by the running batch. So I traced each test's assertions through the current source, after the l38 A1 (`project_owned`) and l37 §4 (`original_tool_run()`) changes.

Tests:
- `fill_run_finalize_publishes_one_edit_with_every_provisional_placement`
- `set_fill_count_dispatches_through_the_tool_job_path_and_updates_the_requested_count`
- `raising_and_lowering_the_fill_count_during_a_run_continues_and_retracts_the_same_sequence`
- `engagement_abort_tears_the_fill_plan_down_across_turns_and_never_inside_one`
- `cancelling_a_stepping_fill_job_never_panics_and_reports_a_cancelled_run`

## 1. Root cause shared by all five: a provisional op row publication is torn in half

### Where
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/🎮️operations/🦀️.rs`: `OriginalToolRunOperations::ingest`, `step_row` and `close_step`.

### What is wrong
`RetainedCloneSourceAdmission::take_output` (`🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🎟️admission/🦀️.rs:80`) is a **two-step publication**:
1. With an empty destination, it writes a `native_publication_destination` placeholder into `output`. The placeholder has no owner and no lease. It also marks `source.publication`. It answers `Progress`.
2. With that SAME destination in `output`, it moves the owner and lease across. It answers `Complete`.

The admission is `terminal_is_empty()` only after step 2.

The tool-run row code gave `take_output` a fresh local `let mut slot=None` on every call, and treated step 1 as the end:
- **`ingest`**:
  - It pushed the ownerless placeholder into `rows` and set `incoming+=1`.
  - It kept the admission, which was still publishing.
  - On the next call, `take_output(&mut None)` raised "source publication destination absent".
  - If that was the last op, `refold` instead decoded `rows[index].try_borrow()` on the ownerless placeholder.
- **`step_row`** (the `close_active` path): it pushed the placeholder and then set `row_admission=None`. That drops an admission that still holds custody, and its `Drop` asserts "original source admission retains unpaid custody".
- **`close_step`**: in the `ready` phase it called `return_inputs`, which refuses with "published original must return through its source".

Every fill tick carries `append_ops`, so every fill run hit this on its first provisional placement. The pump then faulted before `completed > 0` or `Complete` could hold. Until l38 the cluster failed one step earlier, on the projection refusal (A1). Fixing A1 only moved the fault here.

The established callers keep the destination resident in a field and drop the admission only once it is `terminal_is_empty()`:
- `🏪️store/🧵️operation-wire/🧳️replacement` (`self.source`)
- `🧩️composition/📨️emission/🧳️receiving`
- the 🔏️digest code
- `📤️edit/🧳️custody`

### Fix (applied)
- New field `row_output: Option<Source>`: the resident publication destination of the row in flight.
- `row_demand()` gives the phase-selected demand:
  - `ready` → `take_output_demand()`. It was `header::<Source>()` before, which under-quoted step 2.
  - `projectionStart` → `owned_projection_demand()`.
  - otherwise → `next_demand()`.
- `advance_row(effect)` runs one step:
  - `ready` → `take_output(&mut self.row_output)`;
  - `projectionStart` → `project_owned`;
  - otherwise → `advance`.

  Only when the admission is `terminal_is_empty()` does it drop the admission, move `row_output` into `rows` (capacity was reserved before the admission was born) and set `incoming += 1` and `overlay.dirty`.
- `step_row` = `paid_effect(row_demand, advance_row)`.
- `ingest` creates the admission once. It then drives an in-flight admission only through `step_row`. The duplicated inline phase match is gone.
- `close_step` finishes a `ready` admission forward with `advance_row` under `close_original_effect_step`, because a published original cannot return its inputs. After that, the row retires with the other rows. The projection-discard and `return_inputs` order is unchanged for the earlier phases.
- `close_demands` now quotes the row admission first, in the same order as `close_step`. Before, it never quoted it.
- `terminal_is_empty` also requires `row_output.is_none()`.

This module is the shared tool-run framework, so the same fix covers the 2D and 5D fill runs and every other mutating tool run.

### Same defect class elsewhere (fixed)
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🧬️native/🦀️.rs` (ephemeral one-item publication):
- The advance path dropped the admission as soon as `self.source.is_some()`, which is already true after step 1. It now drops it once the admission is `terminal_is_empty()`.
- The close path used a local `output` slot and closed `self.source` before the admission. It now takes output into `self.source`. The source closes only once no admission remains, in both `close` and `close_demand`.

## 2. `cancelling…` "abort is enabled while the run is still starting": no run identity before the first presentation

### What is wrong
The test calls `fill_run_identity` right after `toolRunStart`, without a host turn in between. That reads `main::engagement` → `fill_tool::abort_action(doc.original_tool_run())`.

`ToolRunLedger::original_view_for` answers `entry.native.borrowed_view(..)`, which is `None` until:
- `OriginalToolRunNativeState::initialize` runs in a driver turn;
- and the `ToolRunPresentationSlot` publishes its seed, which takes several more turns.

So after the l37 migration the abort binding fell back to `engagementAbort` for the whole `starting` window. The contract (`📋️tool-run-contract.md` §5 W1-F) says abort is offered from `starting` on. The count entry's `loading` ring had the same gap.

### Fix (applied)
The scalar run facts (tool id, identity, state) live on the ledger entry from admission on. The presentation (progress, entities, payload) arrives later. Run controls now bind to the scalar facts:
- `ArtifactView` has a new field `tool_run_poll: Option<ToolRunPollView<'a>>`, with `with_tool_run_poll` and `tool_run_poll()`. This is the same `ToolRunPollView` that `ArtifactPendingView` already carries.
- Every site that binds `with_original_tool_run` now also binds `.with_tool_run_poll(tool_runs.poll_view_for(window_id))`:
  - the three projections in `🖼️render/📥️staging/🪟️projections`;
  - `🪢️members/📥️staging/🖼️render-surface`.
- 3D `fill_tool::{count_measure, measures, live_fill_run, abort_action}` and `main::engagement` now take `Option<semio_plugin::ToolRunPollView<'_>>`. The play-app engagement and measures pass `doc.tool_run_poll()`. Provisional rendering keeps `doc.original_tool_run()`.
- 5D is in the same state, so the same change was made there:
  - `fill::{count_measure, measures, live_fill_run, abort_action}`;
  - `puzzle5d_engagement`;
  - the `✍️artifact-editor` engagement and measures.
- The 3D and 5D `engagement_abort` commands convert the captured context view with `ctx.tool_run.map(semio_plugin::ToolRunPollView::from)`.

### Residual (not changed)
The owned command context (`ArtifactOwnedToolJobContext::original_tool_run`) is still the captured presentation. An Escape typed in the first turns of `starting`, before the presentation publishes, disarms but does not abort. The engagement-abort law runs after `completed > 0`, so it is not affected.

A full fix would capture the scalar identity into the owned context as well (`ContextToolRunMetadata` already has the shape). It fits best in the separate lane that retires the dead owned `ToolRunView`.

## Expected after a rebuild
- §1 should clear the run fault in all five tests (finalize, set count, raise/lower, engagement abort, cancelling).
- §2 should clear the `cancelling…` line 98 assertion.
- If something is still red, the next suspects are:
  - the presentation provenance entities (the `provisional` instance count);
  - the retract path (`ingest==3`) for the raise/lower law.

Rerun line: `-p semio-s-artifact-puzzle-3d-editor --test editor-panels fill`.

## Files changed
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/🎮️operations/🦀️.rs`
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🧬️native/🦀️.rs`
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🔭️artifact-view/🦀️.rs`
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render/📥️staging/🪟️projections/🦀️.rs`
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🪢️members/📥️staging/🖼️render-surface/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️play-app/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️runtime/🛑️engagement-abort/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/✍️artifact-editor/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛑️engagement-abort/🦀️.rs`

2D has no fill tool-run parameter (`count_measure(envelope, labels)`). Its fill run goes through the shared operations module, so the §1 fix covers it.
