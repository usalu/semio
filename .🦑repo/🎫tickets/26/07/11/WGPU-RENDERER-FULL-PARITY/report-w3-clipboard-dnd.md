# W3 — Clipboard + Drag-Drop Command Wiring (WS2b/2c)

The implementing agent's final chat message was a non-answer ("waiting for a monitor"), but its
actual work (660 lines in `framework/renderer/wgpu/rs/lib.rs`, 49 lines in `ui/wgpu/rs/lib.rs`) was
real and largely correct, left uncommitted. This report is written from direct verification.

## What's in place
- `ClipboardCopy`/`ClipboardCut` wired to write to the OS clipboard; `ClipboardPasteRequested` reads
  from it and round-trips as a `UiEvent::Paste` back into the retained engine — both native (winit
  target) and wasm (browser) paths implemented behind a `host`-region abstraction.
- `DropCommitted`/`DropCancelled` wired to dispatch `ActionDescriptor`s into the same pipeline
  `UiCommand::App` already uses, matching React's `{...payload, targetId, dropPosition}` shape.
- Touched `render_block_list` (a `scenes`-region function, outside its nominal `RetainedEngineCutover`
  scope) — justified: block-list rows are drag-reorderable, so wiring `DropCommitted` for them
  required a small, real touch to that function's own drag-state handling. Included a border-drawing
  fix (`draw_ink_rect_outline` for step cards — 4-sided border instead of top/bottom-only) as part of
  that pass.

## Bug found in the agent's own new test (fixed directly, not by the agent)
`step_card_draws_a_full_four_sided_border_not_just_top_and_bottom` asserted 24 vertices, got 30. Root
cause: it counted `vector_vertices` by color only, but `theme.separator` and `theme.border_normal`
are byte-identical by design (`Theme::from_chrome`), so the test's color filter also caught an
unrelated main/palette divider line 6 vertices away. A position-based filter was tried first but the
card's right edge sits only ~8px from the divider — too tight a margin to be robust. Rewrote the test
to unit-test `draw_ink_rect_outline` directly instead of filtering `render_block_list`'s full output,
which is both correct and more precisely scoped to what the test claims to verify.

## Verification (run directly)
- `cargo check -p semio-framework-renderer-wgpu --lib`: clean.
- `cargo test -p semio-framework-renderer-wgpu --lib`: **265/265 PASS** (after the test fix above).

## Files touched
- `framework/renderer/wgpu/rs/lib.rs` — `RetainedEngineCutover` clipboard/dnd command handling, `host`
  region additions, `scenes::render_block_list`'s drag-reorder wiring and border fix, plus the test
  fix.
- `ui/wgpu/rs/lib.rs` — additive clipboard-related helpers/event plumbing.
