# 🧪️ a2: wgpu engine law crates after the job/close API removals

Crates (relative to `🧰️framework`):

- `semio-ui-wgpu-engine-laws`: `🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml`
- `semio-ui-wgpu-engine-scene-laws`: `🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🧪️tests/🎬️companion-scene/📦️packages/🦀️rust/Cargo.toml`

Runs used `a2-one.sh`. It is `one.sh` with a 5400 s bound instead of 1800 s. Every isolated build waited in the shared compiler lease queue and rebuilt about 330 crates, so it went past 1800 s more than once (`exit 124`).

## Before and after

| crate | before | after |
|---|---|---|
| engine-laws | lib test did not compile | compiles. Full run: see the final numbers below. With `--skip large_layout_and_shaping_job_admits_one_work_unit_per_slice`: 103 passed, 2 failed (`a2-laws8`). |
| engine-scene-laws | lib test did not compile (`SelfFundedCloseCaller`, `step(cx)`, `close_step(grant, ..)`, `RetainedWorkOutcome::unwrap`) | compiles. 11 passed, 3 failed (`a2-scene8`); final numbers below. |

## Changes

### Law ports to the current API

- `🖱️ui/🧪️tests/♻️physical-job-close/🦀️.rs`
  - Replaced `StepContextOwner`, `job.step(cx)` without a claim, the old `drive_step` signature and `SelfFundedCloseCaller` with the window-unit idiom:
    - one `OriginalJobFixtureCaller`, funded by `close_source::STOCK`, in a `ClipboardCaller`;
    - an `OriginalJobReturnOrigin` plus `drive_step(job, cx, receiver, ..)` inside `with_original_work_loan`;
    - `WorkerJobSessionAdmission<ClipboardIoJob, BatchJobSession<_>>` in place of `WorkerJobSession::try_admit_owned`;
    - paid closes through `caller.turn`.
  - Receipts are now measured as the delta of the turn.
  - The cancellation clock is one shared `fixture_now_us`, because worker admission compares `now_us` by function address.
- `🖱️ui/🧪️tests/♻️physical-job-close/⏱️allocator/🦀️.rs`: measures through `semio_framework_value_testing_allocator::observe_retirement_allocations`, which is the window-unit idiom. Both crates gained the dev-dependency `semio-framework-value-testing-allocator`.
- `🔬️targets-wgpu-engine-unit/📐️surface-content-height-and-worker-close-laws/🦀️.rs`
  - `original_layout_admission_params_cancel_and_full_error_close_with_system_receipts`:
    - `LayoutWorkerAdmission::close_step(effect, witness)` runs inside `OriginalJobFixtureCaller::with_effect`;
    - `CancelToken::admit_root(..).result`;
    - the root is closed with an effect;
    - each turn's copy grant is raised to the failure retirement's quoted `next_copy_byte_demand`. The `ValueError` controlled close now quotes 424 copy bytes per step, so the fixture's 1/3-byte grants could never progress.
  - `original_ui_worker_close_retains_whole_failure_and_original_cx`:
    - `close_surface_one` now needs a Source parent, so the denials (`items`/`work`/`deadline`) run as `caller.turn` + `with_original_work_loan(fuel, deadline)`;
    - funded turns use the failure's `selected_demand` quote in the same way.
- Every `Ui::new` in the mounted laws became `UiFixture::new`. Both crate roots import `semio_ui_wgpu_engine::testkit::UiFixture`.
- Layout loops on a bare `StepContext` were replaced with the coordinator's `UiFixture::step_layouts` / `settle_layout`. This covers:
  - `drive_layout` in the tooltip and accessibility laws;
  - the frame-storm laws;
  - `🪗️retained-section-collapse`;
  - `🔽️retained-select-origin`.
- `🏆️golden-widget-conversions-and-paint-goldens`
  - Added `CardStack` → Stack and `OverviewCard` → Section to the `to_widget_node` mirror.
  - `stats` skips the zero-area transparent tombstones the shared-border finalizer leaves in reserved slots.
  - The immediate list is finalized by the same `finalize_shared_borders_step`. Before, every control showed retained +4 versus immediate, and steppers showed −2 because edges are merged.
- `measure_widget_separator_uses_theme_control_height_floor`: a separator now measures `1.0`, following production commit 3cde99d4: the stack owns the gap.
- `🖱️ui/🧪️tests/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json`: `Option<UiSurfaceSlot>` budget changed from 170536 to 215760 bytes (measured).
- `large_layout_and_shaping_job_admits_one_work_unit_per_slice`: the loop is bounded at 200 000 slices and reports yields per stage. It used to be an unbounded `loop`, which hung a whole run until the timeout.
- `ui.set_theme(ui.theme())` became a two-step call to avoid a borrow conflict through `DerefMut`.

### Harness

- `🖱️ui/🧬️contract/🌱️values/🧪️fixture/🦀️.rs` (`UiArenaFixture`): the caller's business fuel went from 1 000 000 to 1 000 000 000. Its cumulative stock is now a separate `stock()` of 2^36 items and bytes; per-turn grants are unchanged. The old 1 M fuel and 64 MiB stock ran out during the 1 025-node layout law. Every later law then failed with `UI arena already has an actual lifetime owner`, because the fixture could not close.

## Unresolved

1. `large_layout_and_shaping_job_admits_one_work_unit_per_slice` (engine-laws), in the UiFixture / arena-harness area.
   - Under `UiFixture::step_layouts`, `Layout.CollectNodes` stops progressing at (1091 nodes, 4250 glyphs) of (2050, 8106). After that, every slice yields `(0, 0)`.
   - With the raised stock, `UiFixture::layout_turn` fails with `WorkLimit "original Source clock declined new construction"`. That comes from `recorded_clock_allows` in `🧵️job/🛬️receiving/🎟️native-turn`.
   - `UiFixture::close` then panics: `the fixture surface close is funded: … clock declined new construction`. The arena leaks, and every later UiFixture law fails at `UiArenaFixture::new` (`fixture/🦀️.rs:20`) with `UI arena already has an actual lifetime owner`.
   - The arena caller's Source clock has its own finite budget, which a long-lived fixture exhausts. A failed close should also not poison the rest of the run.
2. `same_size_moved_viewport_discards_the_old_origin_candidate_before_atomic_publication` (engine-laws), panic at `🎭️scene-host-overlay-body-and-frame-laws/🦀️.rs:391`:
   ```text
   discard and repaint stay atomic
     left:  UiFramePaintCensus { layers: 1, quads: 10, glyphs: 6, .. }
     right: UiFramePaintCensus { layers: 0, .. }
   ```
   `frame_into_step` returns `Pending` while `target` already holds items after the origin move. The retained-append census during the discard is the suspect. This is production paint-frame behaviour and was not changed here.
3. `cancelled_clipboard_outcome_preserves_original_input_until_descriptor_ack_and_paid_close` and `cancelled_clipboard_driver_binds_the_original_producer_witness` (scene-laws), panic `the original return origin admits inside its finite turns`.
   - The debug probe shows `origin_admit_false: cancelled=true`.
   - `OriginalJobReturnOrigin::admit` and `OriginalJobReturnReceiver::enter_body` (`🧵️job/📬️outcome/⏸️continuation/🎟️origin/🦀️.rs:80`) refuse a cancelled context.
   - `ClipboardIoJob` emits `Cancelled` only when `cx.is_cancelled()`, so a cancelled outcome can never be claimed through the return origin.
   - The window-unit copies of these laws (`🖥️host/🧪️tests/🔬️window-unit`) use the same idiom, and the sibling `📋️clipboard-ingress` laws in `semio-ui-wgpu-laws` also fail.
   - This needs a job-protocol decision: either cancelled outcomes are claimable, or these laws move to the close path. The job core was not changed here.
4. `original_clipboard_session_admission_is_paid_before_birth_and_preserves_denied_owners` (scene-laws), panic at `♻️physical-job-close/🦀️.rs:207`, in the `WorkerJobSessionAdmission::advance` loop:
   ```text
   left: (3747, 1789)   heap requested/released, measured with `semio_framework_trace::observe_heap_allocations_on_this_thread` around the whole `caller.turn`
   right: (0, 0)        `OriginalJobFixtureCaller::receipt()` of that turn
   ```
   One admission turn allocates 3747 bytes and frees 1789 bytes, but neither the turn's business progress nor its construction receipt records them. This is an unreceipted allocation in the `🧵️job/👷️worker` session-admission path. `OriginalJobFixtureCaller::audit_heap` would flag the same thing. The production code was not changed. The window-unit copy of this law uses the same idiom.

## Final runs

- engine-laws, full run (`a2-laws9`): `test result: FAILED. 76 passed; 30 failed`.
  - 28 of the 30 failures are the cascade from item 1: `UiFixture` close fails, the arena leaks, and later laws panic with `UI arena already has an actual lifetime owner`.
  - The other two are item 1 itself and item 2.
- engine-laws with `--skip large_layout_and_shaping_job_admits_one_work_unit_per_slice` (`a2-laws10-skiplarge`): `test result: FAILED. 104 passed; 1 failed; 1 filtered out`. The one failure is item 2.
- engine-scene-laws (`a2-scene11`): `test result: FAILED. 11 passed; 3 failed`. The failures are items 3 and 4.
