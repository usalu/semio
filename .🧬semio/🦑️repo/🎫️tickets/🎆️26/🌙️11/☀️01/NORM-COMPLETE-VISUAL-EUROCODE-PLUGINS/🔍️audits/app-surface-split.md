# App-surface split (2026-11-02, uncompiled)

`semio-s-plugin-norm-app-surface` (557 KB / 67 files / 11.2k lines, one dependency cycle) is now a thin facade over six sibling crates. Every crate is under `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/<dir>/📦️packages/🦀️rust/Cargo.toml` and is a norm workspace member and `[workspace.dependencies]` entry. Source files were NOT moved (other agents edit them concurrently): each crate root mounts them with `#[path = "../..."]`, so the folder taxonomy is unchanged. No `📋️project.json`/`📜️script.ts` (siblings have none; the crates build through `@semio-tech/norm-artifact-contract-rs`).

## Layering (bottom to top; dependencies only point downwards)

| crate (dir) | modules | prod bytes / files / lines | incl. tests |
|---|---|---|---|
| `-foundation` (`🧱️foundation`) | ids-chrome-and-labels, value-path, field-meta, report-tree, document-editors, store-ownership-macros | 49.8 KB / 8 / 1156 | 53 KB / 9 / 1233 |
| `-viewport3d` (`🪟️visual-windows/🧊️viewport3d`, root = the old module file) | the 14 viewport3d modules without toolbar and lane-overlays | 124.4 KB / 16 / 2462 | 175 KB / 19 / 3448 |
| `-visual-chrome` (`🖼️visual-chrome`; deps foundation, viewport3d) | diagrams2d, accessibility, themes, device-profiles, manifest-and-media, interaction domain | 97.0 KB / 15 / 2008 | 133 KB / 21 / 2791 |
| `-live-core` (`🧲️live-core`; deps foundation) | live-lanes (schema, stores, runtime, clock), direct-manipulation, edit-rules, command-args-and-dispatch | 94.1 KB / 15 / 2092 | 132 KB / 21 / 2824 |
| `-panels` (`🧾️panels`; deps foundation, viewport3d, visual-chrome, live-core) | checks, properties, outliner, selection-mirror, catalogue-and-inspection, viewport3d toolbar, lane-overlays | 120.1 KB / 15 / 2324 | 172 KB / 21 / 3237 |
| `-live-editor` (`🎛️live-editor`; deps all of the above) | live-lanes editor-wiring, retained-commands, command-and-tool-job-macros, retained-disposition oracle, live-acceptance | 90.9 KB / 8 / 1527 | 98 KB / 9 / 1765 |
| facade `semio-s-plugin-norm-app-surface` | root, root unit tests | 5.6 KB / 2 / 113 | 48 KB / 3 / 838 |

Limits: 262144 B / 32 files / 4000 lines. All production closures are far below the 200 KB / 24 files / 3200 lines target. With cfg(test) the viewport3d (3448 lines) and panels (3237 lines) closures are slightly above 3200 lines but under 4000. Checked with `🧰️scripts/source-boundary-scan.py` (cfg(test) stripped) and a variant that keeps tests; no file is orphaned except cfg(test) files, each reachable from its crate's test mount.

## Cycle cuts

The 373 KB cycle (retained-commands, direct-manipulation, live-lanes, edit-rules, command-args-and-dispatch, value-path, outliner, viewport3d, checks, properties) was resolved by moving, not by trait seams (a symbol-level scan, `🧰️scripts/app-surface-deps.py`, showed the back edges were few):
- viewport3d -> live-lanes / checks / report-tree / ids: the only users were the `toolbar` and `lane-overlays` sub-modules. They moved up into `-panels` (`panels::toolbar`, `panels::lane_overlays`); the facade keeps the public paths `viewport3d::toolbar` and `viewport3d::lane_overlays`. viewport3d is now a leaf crate.
- lane-overlays -> `LaneFrame`/`PeerLane` (defined in editor-wiring): moved into `live-lanes/lane-runtime` (live-core).
- editor-wiring (depends on dispatch, edit-rules, direct-manipulation, retained-commands) is the only live-lanes member that points upwards; it is mounted by `-live-editor`, the rest of live-lanes stays in live-core.
- edit-rules <-> command-args-and-dispatch, live-lanes <-> direct-manipulation, live-lanes <-> edit-rules are cycles inside live-core only (same crate, allowed).
- value-path, field-meta, ids, report-tree, document-editors have no upward edges (the audit's `value_at` hit was a name clash).
- Viewport3d unit tests that need manifest/toolbar/lane-overlays (window definitions, layouts, interaction domain, toolbar, panel surface) moved to `🧪️tests/🔬️viewport-panels/🦀️.rs` in `-panels` with copies of the scene fixtures they need.

## Mechanics (for the integrator)

- Shared import prelude `📋️prelude/🦀️.rs` (extern crate aliases + the plugin/UI `use` list the modules reach by `use super::*` / `use crate::*`) is `include!`d by every crate root (also the facade); roots carry `#![allow(async_fn_in_trait, unused_imports)]`.
- Each root does `pub use <lower crate>::*;` so `use crate::*`/`use super::*` in the moved files keep resolving; D, P and E also `pub use ...viewport3d as viewport3d;` so `crate::viewport3d::...` paths keep working. Private `use` imports are not re-exported, hence the prelude per crate.
- `pub(super)` helpers in report-tree, value-path, ids and field-meta became `pub` (they are reached across the new crate edge).
- `NormRetainedCommandResult` moved from the old root to the live-editor root.
- `#[macro_export]` macros stay in the crate that owns them (`$crate::` paths resolve inside that crate or through its globs): store macros in foundation, lane-store macros in live-core, command/job macros, `norm_live_commands!`, `norm_live_editor!`, `norm_live_acceptance!` in live-editor. The facade re-exports them with globs.
- Facade: `pub use` of the crates (glob; per-symbol lists would be stale as soon as a module grows, a deliberate deviation from "explicit" re-export), explicit `pub mod viewport3d { pub use panels::{lane_overlays, toolbar}; pub use viewport3d::*; }`. `toolbar` and `lane_overlays` are therefore also visible at the facade root (glob leak, harmless).
- Feature `compliance-testing` lives in live-editor (optional serde, serde_json, artifact-reference); the facade feature forwards to it, so artifact dev-dependencies are unchanged. No artifact Cargo.toml needed edits: the facade keeps its path, and the new crates are reached through path dependencies (like the scene foundation crate).
- Not touched on purpose: `Cargo.lock`, `🔒️dependencies.json`, `📋️build-owners.json` (generated by synchronize/lock).

## Risks to expect at the first compile

1. Items that were `pub` only inside the old crate-root namespace but that the modules reached through private root imports (prelude gaps): add the missing line to `📋️prelude/🦀️.rs` once, it applies to all crates.
2. Glob ambiguity when two lower crates export different items with one name (the old root had them in one namespace without clashes, so unlikely).
3. Facade root tests (`🧪️tests/🔬️unit/🦀️.rs`) see only public items of the lower crates.
4. `include!` of the prelude requires the admission reader to follow `include!` (the scan and the source-boundary rule already count it as one file per crate).
5. The hub test `rust-package-commands` greps the facade `🦀️.rs` for the empty-config descriptor text; that text lived in manifest-and-media before the split too, so this is unchanged.

Scripts kept in `🧰️scripts/`: `app-surface-deps.py` (symbol-level module edges), `app-surface-layers.py` (checks a crate assignment: no file may reference a higher crate), `app-surface-gen-cargo.py` (writes all seven Cargo.toml).
