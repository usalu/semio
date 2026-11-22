# Closure guardian: wasm32-wasip2 check of demonstrator, procedural, sourcing

Command: `repo cargo --store=shared --configuration=release --parallelism=2 -- check --locked --offline --keep-going -p semio-hub-demonstrator-component -p semio-hub-procedural-component -p semio-hub-sourcing-component --lib --target wasm32-wasip2 --manifest-path 🌎️hub/Cargo.toml --message-format=short`

Logs: `🗑️generated/guardian-<HHMM>.log` (1954, 2004, 2035, 2043, 2134, 2146, 2207, 2237).

## Errors found and fixes (times PST, 2026-11-21)

| Time | Error | Root cause | Fix |
|---|---|---|---|
| 20:21 | `pdf-page-editor-base`: E0432 unresolved import `semio_s_artifact_stdio_pdf_paint_codecs` (3 files) | The `function-colour`, `shading-preview` and `image-preview` modules moved into the base crate (🧱️model) but the `paint-codecs` dependency stayed on the `page-editor` crate | Added the dependency to `🖼️page/🧱️model/📦️packages/🦀️rust/Cargo.toml`, removed the unused one from `🖼️page/📦️packages/🦀️rust/Cargo.toml`, and updated `🌎️hub/Cargo.lock` to match (sorted entries) |
| 20:25 | `sourcing-curation-tests`: E0425 `WindowLayoutWindowNode` not found | Half-finished layout refactor: `WindowLayoutNode::Window` was already used in the body, but the return type was the removed struct | `sourcing_window` in `🗂️curation/.../🎭️modes/✏️edit/🦀️.rs` now returns `WindowLayoutNode` |
| 20:25 | `puzzle-3d-editor`: E0599 `with_original_binary_receiving_and_comparison` missing | The store API was replaced by `with_original_binary_receiving()`; the comparison source is now assigned through `operation_wire.source` (as the 5d viewer already does) | `🧊️3d/.../🧩️play-app/🦀️.rs` now sets `owners.operation_wire.source = ...prepared_operation_wire_source` and calls `.with_original_binary_receiving()` |
| 21:27 | `pdf-page-editor-base`: E0597 `inferred` does not live long enough (`🔆️image-preview`) | Borrowed value returned out of the inner block it was declared in | Hoisted the inline-image and xobject lookup and the `inferred` image out of the match, so the samples borrow a value that outlives them |
| 22:04 | `pdf-tests-2`: E0603 `WINDOW_KIND_ID` and `BODY_KEY` private (`page::...`) | `🖼️page/🦀️.rs` imported the base crate's items with a private glob, so they were not reachable as `page::X` after the split | Changed it to `pub use semio_s_artifact_stdio_pdf_page_editor_base::*;` |

## Transient or environmental, not touched

- 20:21 `puzzle-3d-precompute-geometry`: `BvhTree::step_release_bytes` missing. The method was added to `🧰️framework/.../🧿️collision/🦀️.rs` at 21:26 by another agent, so I waited. It was clean in later runs.
- 20:3x `procedural-3d-model` and `semio-mesh-snapshot`: ENOSPC (`No space left on device`) on the Rust image `.cache/b/r/m`. The coordinator remounted it; it resumed at the flag `remount-done.flag` (21:41).
- A run at 20:35 sat 35 minutes in `build:source-admission` with idle processes; I cancelled and restarted it, and the restart proceeded normally.
- The 5d and 2d puzzle editors (`✍️artifact-editor`, `🧠️artifact-editor`) still call the removed `with_original_binary_receiving_and_comparison`. They are outside this closure, so I left them. They need the same two-line change as 3d.

## Result

- 22:34: first full check passed clean (`exit 0`, no errors). A `[guardian clean]` line is in `🗑️generated/guardian-status.log`.
- Second consecutive pass: see the end of this file.
- The loop ran beyond the 90 minutes because each full check took 10 to 35 minutes under queueing, headroom waits and the pause.

## Second pass

Started at 22:37 and stopped at 22:45 (guardian-2237.log, incomplete) because the coordinator paused the Rust image for `repo cache gc`. It was still in source admission, with no errors. The guardian stopped there after exceeding the 90-minute budget. The second consecutive pass is still open and should be rerun after `remount-done.flag` reappears.

## Puzzle 2d and 5d editors (follow-up)

- Grep over `✏️s` and `🧰️framework` finds no remaining caller of `with_original_binary_receiving_and_comparison`. The 2d and 5d `✏️editor/🪟️window/🦀️.rs` `native_operation_wire` already use `ArtifactOperationWire::canonical().with_original_binary_receiving()` (migrated by another agent before this pass), so no edit was needed.
- Check: `repo cargo --store=shared --configuration=release --parallelism=1 -- check --locked --offline --keep-going -p semio-s-artifact-puzzle-2d-window -p semio-s-artifact-puzzle-5d-puzzle5d -p semio-s-artifact-puzzle-3d-editor --lib --target wasm32-wasip2 --manifest-path 🌎️hub/Cargo.toml --message-format=short` finished with exit 0 and no errors.
