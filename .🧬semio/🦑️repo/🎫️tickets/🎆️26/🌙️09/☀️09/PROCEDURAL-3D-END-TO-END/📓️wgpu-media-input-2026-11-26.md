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
