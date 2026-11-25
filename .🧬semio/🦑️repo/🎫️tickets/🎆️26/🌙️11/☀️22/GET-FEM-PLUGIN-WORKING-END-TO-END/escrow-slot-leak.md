# Escrow Slot Leak and Live Ordered-Map Drop (FEM Inspection Edits)

## Symptoms
1. After about 15 Inspection edits:
   `OwnershipLimit: original return escrow table has no free slot for a new receiving scope`.
2. Panic `ordered-map root must be explicitly retired before drop` (`🌱️value/🗂️ordered/🦀️.rs:105`), followed by a wasm trap.

## Bug 1: escrow rows of a component Source are never reclaimable

### How the table works
- `OriginalReturnEscrowTable` has a fixed `ORIGINAL_RETURN_ESCROW_CELLS` (32) rows. Row states are free, staged, live and tombstone.
- Each `OriginalJobDecodeOwner` claims one receiving-scope row (`admit_scope` → `claim_stock`). It settles the row on close (`release_original_scope` → `settle_stock` → `acknowledge_stock`).
- A tombstone becomes free again only when its `acknowledged` bit is set (`reclaim_one`, driven by `reclaim_original_escrow_slot` in `⏱️context/🛬️owned/🦀️.rs`).

### Root cause
- The plugin actor runs its Source with Core-wire delivery. It declares `local_acknowledgement`: `cx.acknowledge_return_rows_locally()` in `admit_plugin_step`, because a component host never hands published headers back.
- `ReturnBudget::acknowledge_stock` marked a settled row as received only when `claim.credit.local` was true, which means LocalWitness delivery only. It ignored `local_acknowledgement`.
- The quote side (`admit_original_receiving_scope` and `original_receiving_outcome`) likewise derived `local` from `delivery == LocalWitness` alone.
- As a result, every settled scope of the plugin actor stayed an unacknowledged tombstone. Each Inspection edit opens a fresh decode owner. After the 32 cells, minus the rows held live, were used up, `claim_stock` found no free and no reclaimable row.

### Fix
- `📦️stock/🦀️.rs` `acknowledge_stock`: a done witness marks the row received when `claim.credit.local || self.local_acknowledgement`.
- `🪙️budget/🦀️.rs`: new `ReturnBudget::acknowledges_locally()` (LocalWitness delivery or local acknowledgement).
- `🛬️owned/🦀️.rs`: both receiving-scope quotes use `acknowledges_locally()`. The settlement copy demand therefore includes the acknowledgement write.

### Law
- New `🧪️tests/📊️native-progress/component-owner-succession.rs`, test `successive_component_owners_reclaim_their_settled_scope_rows`.
  - It switches the fixture Source to Core-wire delivery with local acknowledgement, as the plugin actor does.
  - It then runs `4 × ORIGINAL_RETURN_ESCROW_CELLS` successive decode owners. Each opens its receiving scope and retires on the Source.
- The law needed `OriginalJobFixtureCaller::bind_source_epoch`, which binds the actor epoch at the start of every turn before any service claim, as the component actor does. Without it, the resident birth service claims under an unbound epoch.

### Test output
Before the fix (same message as in the browser):
```
component_owner_succession::successive_component_owners_reclaim_their_settled_scope_rows ... FAILED
primary: Value(ValueError { kind: OwnershipLimit, message: "original return escrow table has no free slot for a new receiving scope" ...
test result: FAILED. 0 passed; 1 failed
```
After the fix (`semio-framework-job-context`, test target `native-progress`):
```
test component_owner_succession::successive_component_owners_reclaim_their_settled_scope_rows ... ok
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Pre-existing failures (unrelated)
In the wider `job-context`/`return-budget`/`return` run, these tests fail independently of this change. None of them use Core-wire delivery or local acknowledgement, so the changed branches are inert for them. They come from concurrent peer edits, for example the `with_original_effect` fuel change in `⏱️context/🦀️.rs`.
- `native_parent_survives_foreign_context_cancellation_and_return` (WorkLimit "original Source clock declined new construction")
- three receiving-context tests (`original_context_charges_child_construction_and_enclosing_semantic_once`, `original_context_retirement_after_cancel…`, `original_receiving_ready_scope_settles…`)
- `law_2::return_origin_setup_and_close_preserve_the_actual_source_at_quantum_one`

## Bug 2: a window/panel projection of the host view dropped live

### Root cause
- `ViewModel` owns two ordered roots: `active_utility_by_window_id` and `tool_run_trace_cursor_by_window_id`. They are non-empty as soon as a window utility is armed or tool-run traces exist, for example the FEM Transform utility or results playback.
- `ViewModel::for_window_instance` and `for_panel` return owned projections that alias both roots. Such a projection must either return its aliases (`ScopedWindowView` does this on drop) or be retired. A plain drop panics.
- Two host paths built an owned projection and dropped it:
  1. **Typed command capture.** `capture_typed_command_roots` (`🧵️instance/📨️dispatch/📥️staging/⌨️typed-command/🦀️.rs`) projected `meta.view_state.for_window_instance(window_id)` for the targeted window and dropped it after capture. This runs on every window-targeted typed command, for example FEM 3D `ResultAnimationTick` and `SetResultAnimation`, and on Inspection commands while a utility is armed.
  2. **Context menu.** `ContextMenuWireRequest::into_parts` (`⚛️reactor/🏃️runtime/🧰️ops/🖥️surface-render/🦀️.rs`) replaced the decoded host view by an owned `for_window_instance`/`for_panel` projection. That projection was dropped after `context_menu` in both `arm_context_menu` (binary exchange) and `plugin_context_menu` (JSON entry). The decoded original was dropped live inside `into_parts`.

### Fix (owner retires on every path)
- `🛂️manifest/🖼️view-model/🦀️.rs`: new `ViewModel::scope_panel()`, the panel counterpart of `scope_window_instance()`. Both lend a `ScopedWindowView`, whose drop returns the projection's aliases to the original.
- Typed command capture now uses `view.scope_window_instance(window_id)`. The capture reads it through `as_deref()`, so no owned projection exists.
- Context menu:
  - `ContextMenuWireRequest::into_parts` is infallible and returns the decoded host view itself.
  - New `with_context_menu_view` lends the scoped window or panel projection. An unknown window is refused there.
  - New `answer_context_menu` answers on that projection, then hands the host view to `SurfaceContexts::displace_view` on success and on refusal alike.
  - `arm_context_menu` and `plugin_context_menu` both use `answer_context_menu`.
- `⚛️reactor/🪟️surfaces/🦀️.rs`: new `SurfaceContexts::displace_view`. It queues the view as `Displaced::View`, which the funded displaced retirement (`plugin_drive_displaced_surfaces`) already retires.
- The assertion in `OrderedMap::drop` is untouched.

### Law
- `⚛️reactor/🪟️surfaces/🧪️tests/🪟️surface-context-lifecycle/🦀️.rs`, test `context_menu_projections_return_their_aliases_and_the_host_view_retires_through_displacement`.
  - It decodes the lifecycle fixture view, whose utility map is non-empty.
  - Over 64 rounds it lends a scoped window or panel projection for every fixture surface, checks the projected `window_id`/`active_utility_id` and drops it. It also checks that an unknown window is refused and displaces a host-view alias into `SurfaceContexts`.
  - It then retires the contexts and the view through genuine controlled retirement (`drain`).
- `🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-7/🧭️ribbon-menus-and-surface-context/🦀️.rs` (`context_menu_wire_requires_and_forwards_canonical_view_state`) now uses the new API: infallible `into_parts`, `with_context_menu_view` for the panel, and refusal of an unknown window. Its old fixture loop dropped live projections of a non-empty view and moved into the surfaces law above.

### Test output
- The plugin crates are excluded from the `🧰️framework` workspace.
  - Libraries were checked through the `🌎️hub` workspace (`escrow-check.sh`). Result: `semio-framework-manifest-view-model`, `semio-plugin-plugin-program`, `semio-plugin-instance-dispatch`, `semio-plugin-runtime-ops-surface` and `semio-plugin-runtime-exchange` check with `rc=0`.
  - Tests ran standalone through `escrow-standalone.sh`. It runs `cargo test --manifest-path`, then deletes the nested `Cargo.lock`.
- `semio-plugin-plugin-program`, filter `surface_context`:
```
test surface_context::tests::context_menu_projections_return_their_aliases_and_the_host_view_retires_through_displacement ... ok
test surface_context::tests::auxiliary_context_leases_use_actual_windows_and_never_survive_source_replacement ... ok
test surface_context::tests::mount_retains_the_owned_wire_view_and_projects_the_exact_window ... ok
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```
- The companion crate that hosts the part-7 test (`semio-plugin-tests-12-2`) does not compile at the moment, with 71 errors unrelated to this change. Examples: `crate::retained_command`, `COMPOSITION_MAINTENANCE_GRANT`, `FrameworkSetSelectionModeJob`, `charge_turn_execution_us`. None of them is reported on the edited test lines, so the updated part-7 test is compiled only as far as that crate gets and has not been run.

### Remaining gap
- If the instance id is unknown (`find_instance` fails), the decoded context-menu view still has no owner to displace into. This is unreachable for a mounted instance.
- `plugin_refresh_ui` and `plugin_render_with_document` (legacy JSON paths in `surface-render`) still pass the original to `update_view(&view)` and drop it. Neither runs in the component actor.

## Files
- `🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget/📦️stock/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️owned/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️tests/📊️native-progress/component-owner-succession.rs` (new)
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️tests/📊️native-progress/🦀️.rs`
- `🧰️framework/🔨️modules/🛂️manifest/🖼️view-model/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📨️dispatch/📥️staging/⌨️typed-command/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🖥️surface-render/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🎯️document-and-child-commands/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/🧪️tests/🪟️surface-context-lifecycle/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-7/🧭️ribbon-menus-and-surface-context/🦀️.rs`
- Ticket scripts: `escrow-test.sh`, `escrow-suite.sh`, `escrow-check.sh`, `escrow-standalone.sh`, `escrow-context-menu-edit.py`
