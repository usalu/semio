# Result: wgpu Scenes split

Compile status: **blocked by source-boundary gate**. `cargo check` was never green before or after the split. Verification below is static plus a name-resolution differential run with `rustc`.

`E/` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/`. `S/` = `E/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/` (the real module `pub mod scenes`).

## Line counts

The original had 12,343 lines (saved as `🗑️generated/scenes-original-HEAD.rs`). It is now a 397-line parent plus 15 children, 12,449 lines in total.

| File (under `S/`) | Lines |
|---|---|
| `🦀️.rs` (parent) | 397 |
| `🗃️scene-state/` | 1131 |
| `🪣️pending-raster/` | 402 |
| `📷️scene-camera-dispatch/` | 334 |
| `⌨️scene-input/` | 1073 |
| `🎨️scene-render-entry/` | 708 |
| `🧮️table-and-paint2d/` | 822 |
| `🧱️block-list-and-diff/` | 760 |
| `📰️event-feed-and-timeline/` | 996 |
| `🖌️canvas2d/` | 1180 |
| `✒️ink-canvas-model-state/` | 2055 |
| `🖋️ink-canvas-render/` | 364 |
| `🧭️node-graph-and-tiled-map/` | 463 |
| `🖼️icon-render/` | 455 |
| `🧩️board2d-and-vfs/` | 725 |
| `✍️text-editor/` | 584 |

Each child is `<folder>/🦀️.rs`. The largest child is 2055 lines, under the 2500 ceiling. Several children are below the 800 target because the seam map set those boundaries.

## Parent

The parent keeps the original header (lines 1-28) and everything from `WorkerCell` through the state accessors (seam-map Q0, old lines 1556-1864). After the header imports it declares, for each child, `#[path = "<folder>/🦀️.rs"] mod <snake>;` plus a glob re-export.
- The re-export is `pub use` for the 8 children that contain free `pub` items, so all 109 public paths are unchanged. These are `scene_state`, `pending_raster`, `scene_camera_dispatch`, `scene_input`, `scene_render_entry`, `node_graph_and_tiled_map`, `board2d_and_vfs` and `text_editor`.
- The other 7 children get `pub(crate) use`. A first run with plain `pub use` made `rustc` warn "glob import doesn't reexport anything with visibility pub". The final run is clean.

## Moves

- Cuts are by unique content anchors, not line numbers. Each segment start asserts `count == 1` and a preceding blank line.
- One deviation from the seam map: the 4 dangling `/** 📝️ ... entry point */` doc comments before `//#endregion InkCanvasState` document `ink_effective_bounds`. They moved to the start of `🖋️ink-canvas-render` (Q11) so the `ink-canvas-model-state` child does not end in a dangling doc, which is a parse error.
- Moved text is verbatim except for the edits below. Each child starts with `//! <folder emoji> ...` and `use super::*;`.
- `git diff --stat` on the parent shows 293 insertions and 12,239 deletions, as expected for the move. The previous 12,343-line file is saved outside git.

## Path rewrites

16 literals each gained one `../`. All 31 path literals in the parent and children resolve on disk (checked with a script, 0 missing).

| Child | Literal |
|---|---|
| `🗃️scene-state` | `#[path]` for `wgpu-admitted-surface-map` |
| `📷️scene-camera-dispatch` | `include!` for `wgpu-standalone` |
| `🎨️scene-render-entry` | `#[path]` for `wgpu-surface-context-menu` and `wgpu-render-entry` |
| `🧮️table-and-paint2d` | `#[path]` for `wgpu-table` |
| `🧱️block-list-and-diff` | `#[path]` for `wgpu-block-list` and `wgpu-diff-view` |
| `📰️event-feed-and-timeline` | `#[path]` for `wgpu-event-feed` and `wgpu-graph-timeline` |
| `🖌️canvas2d` | `#[path]` for `wgpu-canvas2d` |
| `🖋️ink-canvas-render` | `#[path]` for `wgpu-raster-frame-cost` and `wgpu-ink-canvas` |
| `🖼️icon-render` | `#[path]` for `wgpu-icon-render`, and `#[path]` for `🖼️IconRenderHost/.../📤️export` |
| `🧩️board2d-and-vfs` | `#[path]` for `wgpu-virtual-file-system` |
| `✍️text-editor` | `#[path]` for `wgpu-text-editor` |

The `include_str!` inside the included `🧊️wgpu-standalone` test file is relative to that file, so it needed no change. `super::super::` in `📤️export/🎬️scene` now resolves to `icon_render`, which defines the names it imports. `pub(crate) mod icon_export` was already `pub(crate)` and stays reachable as `crate::scenes::icon_export`.

## Visibility widenings (not compiler-driven)

The compiler was unavailable, so widening is a conservative static closure rather than error-driven.

1. **Free items.** 141 col-0 `fn`, `struct`, `enum`, `const`, `static`, `type`, `trait` or `mod` items became `pub(crate)`. Each is one whose name occurs as a token in another child, in the parent, or in a test file attached to another child. Per child: Q1 14, Q2 7, Q3 18, Q4 7, Q5 1, Q6 10, Q7 5, Q8 9, Q9 12, Q10 33, Q11 8, Q12 5, Q13 1, Q14 7, Q15 4.
2. **Fields and methods.** A private named field became `pub(crate)` when its name occurs as `.f`, `f:` or a shorthand in another segment. A private inherent-impl method or associated const became `pub(crate)` when `.m` or `::m` occurs elsewhere. This is name-level and over-approximate, with 383 widenings in total. Trait-impl methods were never touched. Per child (fields and methods together): Q1 130, Q2 31, Q3 6, Q4 15, Q6 21, Q7 9, Q8 15, Q9 53, Q10 66, Q13 13, Q14 14, Q15 10; Q5, Q11 and Q12 got none. Some of these may be unnecessary; a compile pass could tighten them later.
3. **Included test file.** `S/../🧪️tests/🧊️wgpu-standalone/🦀️.rs` is `include!`d into `📷️scene-camera-dispatch` and defines 5 helpers used by other children' tests. These are now `pub(crate)`: `paint2d_navigator_fit_viewport`, `paint2d_navigator_overlay_rect`, `ink_items_at_point`, `cursor_from_click` and `line_col_at`.
4. Tuple struct `VfsFileNodeKinds` and `WorkerCell` are only touched inside their own module or by descendants, so no change.

## Edits outside the Scenes tree (text-law readers)

- `E/🧪️tests/♻️frame-close/🟦️.ts` (~line 220): the `camera_owns_surface` slice now reads `📷️scene-camera-dispatch/🦀️.rs`. Line 137 reads the parent for `camera_storage::DirectoryOwner`, which is still there.
- `E/🧪️tests/🛑️scene-pointer-cancellation/🟦️.ts`: the `?raw` import is now the `🧭️node-graph-and-tiled-map` child (variable renamed `tiledMapScenesSource`).
- `E/🧪️tests/🎨️wgpu-paint-policy/🦀️.rs`: 4 allowlist paths now point to `🖌️canvas2d/🦀️.rs`.
- `E/🧪️tests/🔬️interactivity-mounted-frame-transaction/🟦️.ts`: reads `🎨️scene-render-entry/🦀️.rs`.
- Root `📜️script.ts`:
  - `INTERACTIVITY_AUDIT_CANVAS_RASTER_FILE` became `INTERACTIVITY_AUDIT_CANVAS_RASTER_FILES` (parent, `🪣️pending-raster`, `🖌️canvas2d`), joined at its single use.
  - `mountedFrameScenes` reads `🎨️scene-render-entry/🦀️.rs`.
- `✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts`: `canvasRenderer` reads `🖌️canvas2d/🦀️.rs`.
- Unchanged: `🧱️elements/🗣️Interpreter/🧵️worker-cell/🧪️tests/🟦️.ts`. It checks `WorkerCell` strings, which all remain in the parent.
- **Not done:** the generated, already stale `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json` still lists the old Scenes file bytes and sha. The root-script policy reader `policyReadRustSourceEvidence` follows only the parent's own conditional test modules, so test modules moved into children are no longer visible to it. I did not change that helper.
- These readers were re-pointed by reasoning about which markers moved. None of them were run.

## Verification commands and real results

1. **Gate blocked cargo.** `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-renderer-wgpu --message-format=short` was run as a baseline before any edit.
   - It first failed on a stale `Cargo.lock` (a peer had added a workspace member).
   - Then it failed 8 times from 02:04 to 02:41, each time with `Rust crate Source refused before compiler: semio-framework-3d (Rust crate Source boundary exceeded ...)`.
   - The cause is `storage.sourceBoundary` in `⚡️caching/🔣️policy.json`. No baseline and no after-check exists. `--tests` and wasm checks were not run. I stopped the retry loop on the coordinator's instruction.
2. **Lossless multiset comparison.** `verify_equiv.py` normalises `pub(crate) ` prefixes and the one extra `../` on path literals, then compares the line multisets of original and split. Result: the only differences are the 16 intended path lines, 15 headers, `use super::*;`, and the mod/re-export declarations.
3. **Name-resolution differential with `rustc`.**
   - Command: `rustc +nightly-2026-07-20 -Zunpretty=expanded --crate-name scenes_probe --edition 2021 --crate-type lib <file>`, run on a scratch copy of the original and on the split tree (and later on the real tree).
   - It resolves modules, globs and imports but cannot resolve external crates, so about 1,000 errors are expected in both runs.
   - Four configurations were compared as sorted multisets of `error[...]`/`warning` headers: default, `--test`, `--target wasm32-unknown-unknown`, and `--test --target wasm32-unknown-unknown`.
   - Baseline versus real split: **identical** in all four (1017 / 1602 / 1023 / 1592 errors, same warnings).
   - A sensitivity check, making `render_table` private again, produced a new `E0425`, so the oracle does detect missing visibility.
   - It does not reach type-checking, so field and method privacy (E0616/E0624) are covered only by the static widening above.
4. **`rustfmt --edition 2021 --emit stdout`** on the parent, all 15 children and the included test file: exit 0 for all.
5. **Emoji admission.** All 15 folder emoji are RGI (`\p{RGI_Emoji}`) and unique among siblings. The longest absolute path is 219 bytes, under the 240 limit.
6. **External references.** I extracted the 188 distinct `scenes::NAME` references from the rest of the engine tree. Every one is defined as `pub` or `pub(crate)` in the parent or a child. Two names (`RenderEntry`, `TextEditor`) appear only in comments.
7. **Full-crate probe.** Running the same probe on the crate root aborts on unresolved external crates before it reaches the in-crate module paths, so it adds nothing.

## Not done

- No compile (native, `--tests`, or wasm).
- Items I could not check without a compiler: private fields or methods whose use is invisible at name level, and trait-method scoping beyond the header imports. Both are expected to be covered by the superset widening.
- The widenings for fields and methods are broader than necessary until a compile can tighten them.
- The root-script evidence reader and the generated distribution manifest are not updated, as described above.
- Scratch tooling (`split_scenes.py`, `analyze_scenes.py`, `oracle*.sh`, `verify_equiv.py`, sandboxes) lives in the session scratchpad, not in the repo. The logs are `🗑️generated/scenes-baseline-check.log`, `scenes-baseline-attempts.log`, `scenes-split-run.log` and `scenes-original-HEAD.rs`.

## Hardening (full-resolution differential)

Full report: `RESULT-HARDEN-WGPU.md`. Differential from the real lib root with the original `scenes-original-HEAD.rs` restored in the baseline sandbox (15 child folders removed; readers in `🧪️tests/🧊️wgpu-standalone`, `🔬️wgpu-admitted-surface-map`, `🎨️wgpu-paint-policy` reverted line by line; the 31 `element_contracts` edits reversed): native lib 450/450, native `--cfg test` 511/511, wasm32 590/590, wasm32 `--test` 654/654 distinct non-external errors, identical multisets (baseline/current) after the fixes. Scenes needed no widening: the definition scan (family = the Scenes module subtree) found 0 fields, methods or items used outside their child; the only candidate was `AdmittedSurfaceMap.retired`, a field name reused by an unrelated Shell type (cross-family, illegal before the split too). Seeded checks as in the main report. Residual: no typeck (E0616/E0624), native `--test` blind; Scenes and Interpreter still call each other by function.
