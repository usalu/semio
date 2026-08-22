# P9k UI Owned Cell Width and Dependency Removal

## Outcome

`semio-framework-ui` no longer directly depends on `unicode-width` or `pollster`.

- The TUI owns a deterministic display-cell-width primitive. It returns zero for controls, format scalars, combining marks, variation selectors, and joiners; two for East Asian wide/full-width and emoji-presentation scalars; and one otherwise.
- Zero-width property lookup uses one sorted static interval table and binary partition lookup. The hot `char_cells` decision remains allocation-free and returns only `0`, `1`, or `2`.
- Existing scalar-cell renderer semantics are preserved: combining marks and U+200D consume no cell, while the two emoji scalars in a ZWJ sequence each retain their own two-cell glyph slot.
- The optional `unicode-width` row and its `tui` feature edge are removed.
- The unused optional `pollster` row and its `wgpu-engine` feature edge are removed. No WGPU source used `pollster::`.
- No generated/typegen or renderer-package source was edited.

## Golden contract

`text_cell_width_unicode_goldens` fixes the owned behavior for:

- ASCII: `Semio` is 5 cells.
- Combining: `e` plus U+0301 is 1 cell.
- Controls: embedded NUL and ESC consume 0 cells.
- CJK: `界面` is 4 cells.
- Emoji: `🙂` is 2 cells.
- ZWJ: U+200D is 0 cells and `👩‍💻` is 4 scalar-renderer cells.

The existing wide-character truncation test continues to prove that a two-cell scalar is never split.

## Gates and timing

- Focused golden: `cargo test -p semio-framework-ui --features tui text_cell_width_unicode_goldens -- --nocapture` — 1 passed, 0 failed; warm build/test completed in 0.79 seconds.
  - `📝️p9k-ui-unicode-width-native-focused-2.txt`
- Full native TUI library: `cargo test -p semio-framework-ui --features tui --lib` — 95 passed, 0 failed; warm build completed in 0.95 seconds and tests in 0.01 seconds.
  - `📝️p9k-ui-unicode-width-native-lib-1.txt`
- TUI wasm: `cargo check -p semio-framework-ui --features tui --target wasm32-wasip2` — exit 0; cold isolated build completed in 3 minutes 27 seconds.
  - `📝️p9k-ui-unicode-width-wasm-1.txt`
- Native WGPU engine after `pollster` removal: `cargo check -p semio-framework-ui --features wgpu-engine` — exit 0; cold isolated graphics build completed in 11 minutes 54 seconds.
  - `📝️p9k-ui-pollster-native-wgpu-1.txt`
- Focused TUI source rustfmt: exit 0.
  - `📝️p9k-ui-unicode-width-focused-rustfmt.txt`
- Package `cargo fmt -p semio-framework-ui -- --check` reports only the concurrently owned generated WGPU file's trailing blank-line diff. The TUI source is not reported and the generated file was left untouched.
  - `📝️p9k-ui-unicode-width-fmt-check.txt`

## Dependency ratchet

All ratchet counts are zero:

- UI manifest `unicode-width` rows/feature edges: 0.
- UI manifest `pollster` rows/feature edges: 0.
- UI Rust source `unicode_width`/`UnicodeWidth` uses: 0.
- UI Rust source `pollster::` uses: 0.
- TUI dependency-tree `unicode-width`/`pollster` rows: 0.
- WGPU-engine dependency-tree `pollster` rows: 0.

`unicode-width` remains in the workspace lock and can still appear transitively in unrelated dependency graphs; this packet removes the final direct UI dependency, not third-party transitive ownership.

Evidence: `📝️p9k-ui-dependency-ratchet.txt`.
