# WGPU Media Phase vs Pending Input — 2026-11-26

## Root cause
The presenter's `Media` phase holds the presented-input barrier (`holds_presented_input_publication`) while the media collector keeps three presented-tree loans across many turns. Input dispatch advances the interaction epoch, so the loans cannot be released early without invalidating them; and closing them from the browser-output producer closure is refused (`original projection must be driven or returned through its resident frame` / `accepted tree alias requires its original caller`) because the loans live in the resident frame of `publish_presented_snapshot_on_original_caller`'s work loan, not in the producer closure's frame. Result: physical click accepted by the frame worker, never dispatched.

## Fix (owning layer: frame presentation)
Return the loans where they were admitted, from `AppPresenter::present_step` (same resident frame, original caller identity preserved by the earlier `original_absolute_deadline_us` alias fix):
- Hosts publish queued input with `AppPresenter::note_pending_input(bool)` (winit `build_and_publish_snapshot_on_original_caller`, browser worker output closure).
- `present_step` in `Media` asks the pure policy `media_input_step(pending, &mut released, donated)`:
  - `Release`: drive `close_presented_media_slots` on the collector each turn; when terminal, reset the collector and set `media_input_released`.
  - `Hold`: released and input still pending, nothing to do.
  - `Collect`: no input pending (or roster already donated): clear `released`, restart collection with fresh loans.
- `holds_presented_input_publication` holds the barrier in `Media` only while `media_holds_input_barrier(released, donated)`, so `pump_frame_inputs` dispatches input as soon as loans are returned.
- `OriginalPresentedRoster::output_donated()` (phase >= 15) re-added: after donation loans are already returned, no release needed.

## Files (repo-relative to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/`)
- `🧊️renderer/🖥️frame-presentation/🎞️present-state-and-engine-closers/🦀️.rs` (policy, `input_pending`, `media_input_released`)
- `🧊️renderer/🖥️frame-presentation/🧯️retirement-and-present-steps/🦀️.rs` (barrier, `note_pending_input`, Media branch of `present_step`)
- `🧊️renderer/🖥️frame-presentation/🦀️.rs` (re-exports)
- `🎬️media-slots/🎟️collect/🦀️.rs` (`output_donated`)
- `🪟️winit-app/🎞️frame-generation-and-os-host/🦀️.rs`, `🌐️browser-worker/🦀️.rs` (`note_pending_input` calls)
- test: `📺️frame-presentation/🧪️tests/🦀️.rs`

## Tests
- `semio-renderer-wgpu-frame-presentation` `tests::pending_media_input_releases_loans_in_bounded_turns`: model of 3 loans and 2 queued inputs; input dispatches within 3 turns, loans fully returned first, collection restarts afterwards, donated roster never held. PASSED (also compiles the presenter edits).
- Not a full-presenter regression with real tree loans: that needs a GPU context. The `media-slots` crate's own `--lib` test target currently does not compile at HEAD (unrelated: `media_transport_contract_valid(&str)` call at `🎬️media-slots/🦀️.rs:171`, props tests `?` conversions), and `host-plumbing` is over its 262144-byte source boundary (concurrent additions), so neither could host the test.

## Remaining
- Winit-app and browser-worker edits (one-line `note_pending_input` calls) were not compiled separately (wasm32/winit targets).
- WASM renderer rebuild/stage and `wgpu-backend-audit.mjs` not run: the native check is a policy model, and the audit asserts paint only, not click delivery; proof needs the staged build plus the Add Generation click probe (`🐍️wgpu-add-generation-probe.mjs`) after about 300 s paint.
- Risk: continuous pointer-move streams keep `input_pending` true, starving Media completion until input drains; Media then restarts from fresh loans each time.

## Follow-up: compile verification attempt (2026-11-26, evening)

Source repairs made while driving `repo cargo check -p semio-renderer-wgpu-browser-worker --target wasm32-unknown-unknown` (each was a concurrent, half-finished framework refactor blocking the dependency chain, not part of the presenter fix):
- `🎬️media-slots/🦀️.rs:171`: `media_transport_contract_valid` takes `&str`; the call passed `&props` (a `serde_json::Value`). Now passes `&slot.params_json`. This is the pre-existing media-slots compile error.
- `🧵️job/⏱️context/🎟️source/🦀️.rs`: `SourceRetirement` made `pub` (E0446, associated type of a public trait impl).
- `🧵️job/⏱️context/🎟️source/🏭️return-origin/🦀️.rs`: `impl StepContextReturnOrigin for StepContext` bodies used `cx` instead of `self` (E0425 x19).
- `🧵️job/⏱️context/🪟️facade/🦀️.rs`: private `use` of `RetirementDemand` and `RetainedCloneStep` shadowed the `pub use step::*` glob, hiding them from downstream crates (E0432).
- `🎭️actor/🎟️retained-turn/🎟️context/🦀️.rs`: imported the new `StepContextReturnOrigin` trait.
- `🏪️store/⚙️engine/🧯️close/🦀️.rs`: `StoreCloseWitness` gained `owned_roots_witness` (`[DEBUG]`), which an impl already defined (E0407).

Status: the check progressed to `semio-os-store-engine-close` and beyond with no winit-app or browser-worker diagnostics yet. It then stalled in the capacity queue: a sibling session runs `repo capacity run --memory=4500 --cpu=4 -- repo run semio-hub-procedural-component:component` (log `component-build-5.log`, 66+ minutes), whose inner graph read requests 6553 MB while its own 4500 MB grant is held, so it can never be admitted (1.7 GB headroom). My 1536 MB cargo request is queued behind it ("queue-order"). I did not kill that process. Until it is released, no cargo, test, WGPU wasm build, staging or browser probe can run, so tasks 2 (test run), 3 (build, serve, Add Generation probe) are not executed. The `media-slots` `--lib` test-only errors (props tests `?` conversions) are not yet examined because they need a compile.

## Follow-up 2: compile and test results after the deadlock was cleared

Compile:
- `repo cargo check -p semio-renderer-wgpu-browser-worker --target wasm32-unknown-unknown`: PASS (33 min, cold). It compiles the `note_pending_input` call in `🌐️browser-worker/🦀️.rs` together with `media-slots`, `host-plumbing` and `frame-presentation`.
- `repo cargo check -p semio-renderer-wgpu-host-winit`: first run failed with E0599, `RuntimeMailbox::has_pending_applies` was `#[cfg(target_arch = "wasm32")]` but the winit call site (`🪟️winit-app/🎞️frame-generation-and-os-host/🦀️.rs:361`) is native. Removed the cfg gate in `🧊️renderer/📮️mailbox-impl/⏳️pending-work-deadlines-and-spawns/🦀️.rs` (the method only reads the completion mailbox). Re-run: PASS.
- `[DEBUG]` marker removed from `StoreCloseWitness::owned_roots_witness` (trait and impl, now an emoji docstring).

media-slots `--lib` tests (`semio-renderer-wgpu-media-slots`):
- Compile errors fixed: props test `?` conversions (`resume`/`pause` tuple errors mapped to their error, `with_retained_loan` needs `R: Copy` so the grammar result is stashed outside the loan).
- Runtime failures fixed: five `media_slots::tests::*` laws built a `UiTree` without the process-wide UI arena (`UI arena has no accepting lifetime registration`); they now open `UiArenaFixture` (new dev-dependency `semio-ui-contract-values` with `testkit`). `the_shell_slot_replays_the_shared_fixture` compared props with different key order and `400` vs `400.0`; it now compares the canonical serialized values.
- Result with the two exhaustive paid-close laws skipped: 9 passed, 0 failed.
- `original_presented_params_same_source_native_cause_denials_and_full_paid_close` and `original_presented_slot_exact_identity_system_denial_donation_and_full_paid_close` compile but did not finish in the unoptimized profile (30 CPU-minutes and 10 CPU-minutes, killed); a release/optimized run is needed to complete them. Not a failure, not yet a pass.
- Real regression for pending input with live tree loans: not added. The loan-return path is exercised by the existing `host-plumbing-law` collect law (cancellation at every cut, then full paid close), and the presenter turn policy lives in `frame-presentation` (policy test passes); a presenter-level test needs a GPU context.

WGPU build, stage and Add Generation probe: BLOCKED. Every `repo cargo build -p semio-renderer-wgpu-component --target wasm32-unknown-unknown` (script and MCP, 3 attempts) fails in under 5 s with `Internal error: Exact canonical persistent Native publication required: .cache/b/r {"schema":[], ...}` (repo native-image storage, reference 2ff8734c); `check` and `test` on the same store work. Another session's `repo run @semio-tech/flow-extension-math-rust:package` was running meanwhile and the cache disk has 18 GiB free (7 %). This is repo infrastructure, not source. No wasm was built or staged and no probe ran, so whether the Add Generation click mutates the roster and whether geometry paints remains unverified. Previously staged renderer wasm (02:44) predates the `note_pending_input` change.

## Follow-up 3: WGPU wasm build resumed (2026-11-27)

- The "Exact canonical persistent Native publication required" refusal is gone: `repo cargo build -p semio-renderer-wgpu-component --target wasm32-unknown-unknown` now compiles through `wgpu-native-wasm-build.sh`.
- Attempt 1 failed on `JOB_PAYLOAD_PROCESS_OWNED_BYTES` unresolved in `🧵️job/📬️outcome/👷️worker/🦀️.rs` (a concurrent edit; the same file was rewritten while the build ran). Attempt 2 compiled about 100 minutes of units, then failed on `semio_ui_locale` unresolved in `🔌️plugin/🏗️builder` (the Cargo.toml dependency was added by another agent at 00:33, after the build began). Attempt 3 was aborted by the host guard: another session's process (4 GB) left the host critically short of memory ("Nothing is retried"). Attempt 4 is queued: `repo leases status` shows headroom -2745 MB, 13 grants held by other sessions' `graph-plan` reads and my `cargo-orchestration` request (1536 MB) waiting "queue-order" behind a 3943 MB graph-plan.
- No wasm was produced, so nothing was staged on 6118 and the Add Generation probe was not run. Roster mutation and geometry paint remain unverified. The served renderer is still the 02:44 build that predates `note_pending_input`.
- Optimized-profile run of the two exhaustive media-slots laws: not started, because it needs a second large compile that the same memory queue would block. Command when memory is calm: `repo cargo --store=shared --configuration=release -- test -p semio-renderer-wgpu-media-slots --lib -- original_presented_params_same_source original_presented_slot_exact_identity`.
- Remaining steps after the wasm exists (script already runs wasm-bindgen into `.cache/g3d-native/wasm-dev`): `wgpu-native-generate.ts`, `wgpu-native-components.sh`, `wgpu-native-activate.ts`, `screen -dmS g3dwgpu <ticket>/serve-generation3d-wgpu-native.sh`, then `SEMIO_PROBE_SETTLE=300 bun 🐍️wgpu-add-generation-probe.mjs` with one Chromium.
