# Result: infinite world, board normal and dag split

Crate `semio-framework-os-infinite`. Root `INF/` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/`.
Order executed: C (dag), B (normal), A (world). Pure structural moves, no logic edits, no public item renamed.

## Status in one paragraph

All three splits are done and statically verified. **Compile is blocked, not passed.** `cargo check` was never able to run: first the workspace lock file was out of date (a peer added a workspace member without updating `Cargo.lock`, `cannot update the lock file ... --locked`), then the compiler-ingress source-boundary gate refused dependency `semio-framework-3d` (limits 262144 bytes, 32 files, 4000 active lines per crate, `⚡️caching/🔣️policy.json`). No baseline or post-split error counts exist. No unit test was run. Visibility widening is therefore derived by static cross-reference, not by the compiler, and is a superset of what rustc needs for free items, and an approximation for methods and fields (see Visibility).

## Final line counts (total / active lines = non-blank, non-comment)

Targets: parent below about 3000, children about 2500 or less. All met.

| File | Lines | Active |
|---|---|---|
| A `🌍️world/🦀️.rs` (was 17408) | 2581 | 2129 |
| A `🧹️lifecycle/` | 542 | 473 |
| A `🧵️interaction/` (wrapper, only mounts) | 20 | 16 |
| A `🧵️interaction/📇️registry/` | 767 | 645 |
| A `🧵️interaction/🔍️marquee/` | 933 | 863 |
| A `🧵️interaction/🧲️picking/` | 857 | 800 |
| A `🧵️interaction/🕹️gumball/` | 1008 | 916 |
| A `🧵️interaction/⚙️authority/` | 1732 | 1580 |
| A `🧱️scene/` | 2099 | 1789 |
| A `🖌️overlay/` | 2067 | 1812 |
| A `🌉️bridge/` | 1382 | 1115 |
| A `🎛️frame/` | 827 | 762 |
| A `🖱️pointer/` | 1553 | 1290 |
| A `📡️assets/` | 1119 | 871 |
| B `➕️normal/🦀️.rs` (was 13836) | 2669 | 2348 |
| B `🧭️host/` | 1019 | 916 |
| B `🎨️style/` | 2005 | 1870 |
| B `🪣️fill-job/` | 1976 | 1852 |
| B `🎛️selection-events/` | 2494 | 2290 |
| B `🧹️scene-delete-links/` | 2289 | 2162 |
| B `🖱️pointer-gestures/` | 1364 | 1245 |
| C `🕸️dag/🦀️.rs` (was 5430) | 1649 | 1333 |
| C `🧭️host-selection/` | 630 | 527 |
| C `🧱️frame-minimap-align/` | 492 | 417 |
| C `🛠️mutate/` | 1366 | 1158 |
| C `🖌️paint/` | 1148 | 1075 |
| C `🌐️wasm-session/` | 174 | 148 |

Every child is a folder `<emoji>️<kebab>/🦀️.rs` next to its parent, wired with `#[path = "<folder>/🦀️.rs"] mod <snake>;` plus `use <snake>::*;`, `pub use` or `pub(crate) use` according to the visibility of its top-level items (A children all `pub use`: they contain `pub` items; `impl`-only children in B and C need no re-export). Each starts with a `//!` doc whose first character is the folder emoji, then `use super::*;`.

## Deviations from the seam map

- C: `mutate` / `paint` boundary moved to `pub fn pointer_move` (the seam map boundary 3997 fell inside `pointer_down_screen`).
- B: `BoardHost` struct, `NodeIconSpan` and `WorldContentCache` stay in the B root (not in `host`), so the roughly 118 private `BoardHost` fields need no widening (descendants see parent privates). `host` therefore starts at `BoardHostRetirement`. Root is 2669 lines.
- A: `WorldOrbitViewGizmo` region went into `pointer` (it is adjacent), `mod tool_run_trace` stays in the A root, test mods stay in the A root.
- A `interaction` built in two steps (wrapper first, then five grandchildren). Grandchild items needed outside the wrapper are `pub(in crate::world)`, wrapper re-exports with `pub use`.
- B unwrap: inline `pub mod board_host { ... }` removed (body dedented 4 spaces, `// #region board_host` kept), tail `pub use board_host::*;` removed, `use super::{...}` became `use crate::infinite::board::ports::directed::{...}` (same list). Public path `directed_normal::board_host::*` is gone; no in-tree reference existed (rg over the framework tree). `directed_normal::*` exports the same item names as before. The two inner `#![allow(...)]` now apply to the whole B root and its descendants.

## Created files

A: `🌍️world/{🧹️lifecycle,🧵️interaction,🧱️scene,🖌️overlay,🌉️bridge,🎛️frame,🖱️pointer,📡️assets}/🦀️.rs` and `🌍️world/🧵️interaction/{📇️registry,🔍️marquee,🧲️picking,🕹️gumball,⚙️authority}/🦀️.rs`.
B: `➕️normal/{🧭️host,🎨️style,🪣️fill-job,🎛️selection-events,🧹️scene-delete-links,🖱️pointer-gestures}/🦀️.rs`.
C: `🕸️dag/{🧭️host-selection,🧱️frame-minimap-align,🛠️mutate,🖌️paint,🌐️wasm-session}/🦀️.rs`.

Edited (besides the three parents):
- `🌍️world/🧪️tests/🔬️unit/🦀️.rs`: four `include_str!("../../🦀️.rs")` source-scanning reads now read `WORLD_SOURCE`, a `concat!` of the root and all 13 child files (new const at top of file). The fifth read (`../../../🦀️.rs`, the glue file) is unchanged.
- `➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`: four `include_str!("../../🦀️.rs")` reads now read `BOARD_HOST_SOURCE` (root plus the 6 children). This file is pulled in with `#[cfg(test)] include!(...)` (it is compiled, not an orphan).
- `➕️normal/🧭️host/🦀️.rs`: that `include!` path got one `../` (found by the rustc oracle, see below).
- `📺️renderer/🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts`: two `readFileSync` of the dag file re-pointed to `🧭️host-selection/🦀️.rs` (`plan_graph_edits`) and `🛠️mutate/🦀️.rs` (`screen_pointer_gesture_begins_at`). Outside the crate, but it reads moved source.
- C `🌐️wasm-session/🦀️.rs`: inline `mod wasm_session { ... }` body dedented into the file; the parent keeps the `#[cfg(...)]` line and the `pub use wasm_session::DagSession;` re-export.

## Visibility widenings

Rule applied: a private item that is defined in a child and referenced outside it gets `pub(super)` (visible to the parent and siblings); inside the A `interaction` wrapper, items used outside the wrapper get `pub(in crate::world)`. Nothing is wider than that, and nothing `pub` was touched. This is narrower than `pub(crate)`; a mechanical `pub(super)` to `pub(crate)` rewrite is possible if each child later becomes a crate.

How decided (no compiler): (1) top-level items by bare identifier match in other modules; (2) inherent-impl methods and associated consts by `.name` or `::name` match; (3) struct fields by `.name` match, or all fields of a struct whose literal or `Self {` appears in another module. Comments stripped, test files and the existing child modules (`input`, `retirement`, `tool-run-trace`, tests, A `♻️resources`, `🏔️terrain`, `🎨️r3f`) counted as consumers. Result is a superset for names (false positives such as `.len` or `.slot` widen a few extra members; none missed by the independent check below).

Counts of widened members (from `🗑️generated/infinite-split-*.widened.txt`): A first pass 755, A interaction second pass 27, B 246, C 45 (of which 3 false positives in `wasm-session` were removed by hand). Full per-child lists are in those files.

Not widened by name analysis and left for the compiler: tuple struct fields, macro-generated accesses (none found), `E0446`-style private-in-public lint changes (warnings only).

## Path rewrites

- `include!` in B host child: `"🧪️tests/..."` to `"../🧪️tests/..."`.
- `#[path]` of existing children (`🎛️input`, `🧹️retirement`, `🧪️tests/*`, `⏯️tool-run-trace`) unchanged, still declared in the parents.
- No `super::` or `self::` path occurs in moved code (checked with rg), so module re-parenting does not change meaning. No `module_path!`, `file!`, `line!`.

## Static verification (all run, results)

1. Losslessness (`🗑️generated/infinite-verify-lossless.py`, originals saved in `🗑️generated/infinite-orig/{A-world,B-normal,C-dag}.rs` from `git show HEAD:`): multiset of normalised non-blank lines (vis tokens stripped). A: 0 removed, 65 added (all mod decls, `#[path]`, `//!` docs, `use super::*;`). B: 3 removed (`pub mod board_host {`, `pub use board_host::*;`, `use super::{`), 32 added (decls, docs, wrapper `impl BoardHost {` and `}`, the re-pointed `use crate::...::directed::{`). C: 1 removed (`mod wasm_session {`), 24 added (same kinds). Nothing else differs.
2. `rustfmt --edition 2021 --emit stdout < file` on all 28 touched or new files in the three trees: all exit 0.
3. Path literals (`🗑️generated/infinite-verify-paths.py`): 298 `#[path]` and `include_str!/include_bytes!` literals in the whole infinite tree, 0 unresolved. Longest path in the tree 193 characters (a pre-existing fixture); all new paths are shorter.
4. Visibility closure (`🗑️generated/infinite-verify-visibility.py`, independent re-scan of the final tree): a name defined only privately in file G and referenced in a file that is not G and not a descendant of G. A: 1 hit, `write_str` in `🧪️tests/🔬️unit`, a false positive (`fmt::Write::write_str` trait method). B: 0. C: 0.
5. Emoji admission: every new folder emoji (🧹 🧵 🧱 🖌 🌉 🎛 🖱 📡 📇 🔍 🧲 🕹 ⚙ 🧭 🎨 🪣 🛠 🌐) was found as an existing folder prefix elsewhere in the repo (find over `🔨️modules` and `🧰️framework`); no sibling folder shares an emoji (checked against the existing siblings listed in the seam map).
6. rustc name-resolution differential (`rustc +nightly-2026-07-20 -Zunpretty=expanded --crate-type lib`, lib root as crate root, original tree in a scratch copy with the three originals restored vs the split tree; script `scratchpad/inf-oracle.sh`): header multisets identical for `lib` (168 errors both), `--test` (201 both), `--target wasm32-wasip2` (76 both), `--test --target wasm32-wasip2` (194 both). All errors are unresolved external crates in both. It caught one real mistake (the `include!` path) and cannot see body-level or method and field privacy (negative control confirmed it does not detect a removed `pub(super)` on a free fn).

## Compile and tests

- `cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-infinite --message-format=short`: never succeeded. About 28 retries over 60 minutes (log `🗑️generated/infinite-baseline-check.log`): `cannot update the lock file ... because --locked` (peer added workspace member `🔌️plugin/🏛️ownership/🧱️owner` to `Cargo.toml`), transient `scriptInvocationEnvironment` import error from a peer's tooling edit, then `Rust crate Source refused before compiler: semio-framework-3d (Rust crate Source boundary exceeded ...)`. Last single attempt after the coordinator message: lock file error again (`🗑️generated/infinite-final-check.log`). Status: **compile blocked by source-boundary gate / workspace lock state; no error counts, no baseline.**
- `--tests` and `--target wasm32-wasip2` checks and `cargo test -p semio-framework-os-infinite --lib`: not run (blocked the same way).
- Browser-only code (`cfg(all(wasm32, not(p2)))`, the dag `wasm-session` child) is covered only by the text checks above.

## Known follow-ups and risks

- Source-scanning consumers outside the crate still read the old single files and will fail until re-pointed (not edited, outside ownership): root `📜️script.ts` line 8114 (dag file, interactivity audit p3mn), line 9305 via `INTERACTIVITY_AUDIT_WORLD3D_FILE` (`mountedFrameWorld3d`), and `✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts` line 29 (`frameworkWorld`). `policyReadRustPolicySource` reads only the root file plus declared test modules, not non-test children. Also `🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:23` reads the B root: its needle (`opaque_scene_retirement: Cell<...>`, a `BoardHost` field) still lives in the B root, so it should keep passing.
- Doc and comment references to the old files remain (`🗺️surface/🏔️terrain/🦀️.rs`, `🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs`, EngineCanvas, Shell wgpu, intrinsic-size): comments only.
- Unused-import warnings: parent `use` lines now used only by children may warn (workspace lints are warn only). Not pruned, to keep the move mechanical.
- A stray orphan `//#endregion 🧵️FillJob` sits at the top of the `selection-events` impl (the region start is in `fill-job`); harmless comment mismatch.
- puzzle2d wrong-direction naming in B was NOT touched and is NOT collected in one child: it follows the original sections. Counts of case-insensitive `puzzle2d`/`puzzle_2d` matches: B root 9 (`PUZZLE_2D_LODS`, `PUZZLE_2D_LOD_SCALE`, `puzzle_2d_lod_scale_json`), `🎨️style` 3 (brush id prefix `puzzle2d.brush.{serial}`), `🪣️fill-job` 34 (the `puzzle2d-fill-*` action ids and more), `🧹️scene-delete-links` 2.
- Seam-map side findings unchanged: second "normal" tree under `🎲️board/➕️normal/` and `infinite_canvas` doc naming drift.
- Helper scripts are left in `🗑️generated/`: `infinite-split-tool.py`, `infinite-fix-privacy.py` (compiler-driven E0624/E0603/E0616 fixer, ready for when a compile works), `infinite-split-{A1,A2,B,C}.json`, the verify scripts, `infinite-orig/`. Backups of the pre-edit files are in the session scratchpad only.
- When the gate lets cargo run: run the check (`--tests`, `--target wasm32-wasip2` too) and feed the JSON output to `infinite-fix-privacy.py <log> <scope dir> 'pub(in crate::world)' --apply` for any residual E0624/E0603/E0616.

## Hardening (full-resolution differential, 2026-10-15)

Cause of the audit findings: a bug in my splitter. Structs with a `{` body were treated as a container for their fields and never as an item themselves, so `struct X { .. }` definitions referenced outside their child stayed private (fields and methods were widened, free items of other kinds were fine). The earlier `-Zunpretty=expanded` probe and my independent scan shared the blind spot (same struct handling), so both reported clean.

Oracle used (`🗑️generated/infinite-full-oracle.py`): `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null` from the infinite lib root, original tree in a sandbox (three originals restored from `🗑️generated/infinite-orig/`) against a fresh copy of the split tree. Two sandbox-only patches were needed to get past early resolution, applied identically to both trees and never to the repo: (1) `extern crate semio_framework_os_kernel as dsl;` and the `semio_framework_async` extern line removed from the lib root (E0463 aborts rustc before late resolution), (2) every `use <external>::...::*;` glob replaced by `const _: () = ();` (a failed glob makes rustc suppress every unresolved-name error in that module, which hid the regressions). Error headers compared as multisets, four configurations: lib, `--test`, `--target wasm32-wasip2`, `--test --target wasm32-wasip2`.

Before (split tree, lib configuration): 2718 errors against 2576 in the original, 147 distinct new error lines, all E0422/E0425/E0433 "cannot find type/struct ..." for 44 struct definitions in A and 8 in B, e.g. `WorldInteractionId`, `WorldInteractionAuthority`, `WorldFlatAction` (23 uses), `WorldCameraSync`, `BoardPointerSpan`, `BoardDeletePlan`, `BoardPointerCommitOperation`.

Fix (`🗑️generated/infinite-widen-tree.py`, module-aware definition scan over the final tree; log of the edits `🗑️generated/infinite-widen-pass1.txt`): widened to the narrowest visibility that reaches every referencing module.
- A: 44 struct definitions. 20 to `pub(super)`, 24 inside `🧵️interaction/*` to `pub(in crate::world)` (they are used by the world root or by non-interaction children).
- B: 7 struct definitions in `🧭️host` to `pub(super)`.
- Reverted by hand as false positives of the same scan: `pub(super)` on `const _: fn()` in `🪣️fill-job` (warning, no effect) and on three private `DagSessionInner` fields in the browser-only `🌐️wasm-session` (name collision on `.width`, `.height`, `.dpr`).
- Net new widenings: A 44, B 7, C 0. Together with the earlier lists: A 826, B 253, C 45 widened members in total.
- Re-export hygiene found by the oracle warnings: `pub use marquee::*;` in `🧵️interaction` became `pub(super) use marquee::*;` (no public item, glob exported nothing), and the unused `use fill_job::*;` in the B root was removed (the child contains only `impl` blocks and one `const _`).

After, all four configurations: error multisets identical. Counts baseline / split: lib 2576 / 2576, `--test` 4205 / 4205, wasip2 1510 / 1510, `--test` wasip2 4207 / 4207. The only remaining header difference in every configuration is a 1:1 substitution: the original has one multi-name `E0432 unresolved imports super::board_locked_option, ...` (the B `use super::{...}`), the split has one `E0433 cannot find infinite in crate` (the same list now imported via `crate::infinite::board::ports::directed::{...}`; the identical `crate::infinite` prefix is used by the unchanged tail glob at HEAD). No warnings remain that the split introduced.

Definition scan for fields and methods (step 4): the same module-aware scan covers private inherent methods, associated consts and struct fields of every moved child. Result after the fixes: 0 further edits needed in A, 0 in B, 3 false positives in C (the wasm fields above, reverted). Independent checks re-run on the final tree: visibility scan 0 hits (1 known false positive, `fmt::Write::write_str`), lossless diff unchanged except the removed `use fill_job::*;`, 333 path literals all resolve, `rustfmt` parses all touched files.

Residual risk, unchanged: the oracle still cannot see typeck-phase privacy (E0616 fields, E0624 methods); those rest on the name-based scan plus the compile that remains blocked by the source-boundary gate and workspace lock state. Run the real check first thing once it works.
