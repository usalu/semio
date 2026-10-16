# Result W4 SURFACE: `semio-framework-surface` carved into producers behind a thin facade

Executor: Sonnet 5.5, foreground, no sub-agents. Tools: `🪓️monolith-split/surface-carve-tools/` (`census.ts`, `deps.py`, `union.py`, `defscan.py`, `verify.ts`, `register.py`, `mk*.py`, `carve_*.py`, `cut_*.py`, `tf.py`, `spec-phaseA.json`, `spec-phaseB.json`). Logs: `🗑️generated/surface-*.log`. `S` = `🧰️framework/🔨️modules/🗺️surface`.

## 0. Scope change during the task

The brief said to move `🌍️gis-policy` into the GIS plugin. Mid-task the coordinator re-assigned that to the session "Relocate wrong-direction code" and told me to carve around it. I did not touch the policy relocation. That session finished while I worked: `🌍️gis-policy` is gone from tiled-map and the host now renders by a data-driven `MapPolicy` (`🎛️map-policy`, descriptor shipped in the map JSON). I waited until the tiled-map tree had been quiet for 8 minutes, snapshotted it as the new baseline, and carved that state. Terrain got its single owner.

## 1. Census before and after (canonical `readRustSourceClosureV1`, host cfg; gate 4000 lines / 32 files / 262144 bytes)

| Crate | files / bytes / active lines (lib) | test closure | wasm32 closure |
|---|---|---|---|
| `semio-framework-surface` before (my first run) | 19 / 291,717 / 5,995 (OVER) | 46 / 859,581 / 9,109 (OVER, 18 embedded incl. 6 pbf tiles) | 20 / 303,782 / 6,496 (OVER) |
| tiled-map alone after the relocation session, before my carve | 12 / 174,381 / 4,200 (OVER) | 25 / 637,402 / 5,772 (OVER) | 13 / 186,502 / 4,500 (OVER) |
| `semio-framework-surface` facade after | 1 / 461 / 9 | 1 / 461 / 9 | 1 / 461 / 9 |
| `-surface-terrain` | 1 / 19,125 / 404 | 2 / 28,094 / 617 | 404 |
| `-surface-paint` | 3 / 74,064 / 1,306 | 10 / 159,119 / 2,262 (6 embedded json fixtures) | 1,507 |
| `-surface-node-graph` | 2 / 30,456 / 251 | 10 / 61,280 / 537 (4 embedded) | 251 |
| `-surface-tiled-map-mvt` | 1 / 19,804 / 591 | 3 / 29,917 / 817 (0 embedded) | 591 |
| `-surface-tiled-map-model` | 7 / 55,764 / 1,451 | 11 / 92,660 / 1,765 (2 embedded) | 1,451 |
| `-surface-tiled-map` (host) | 6 / 102,367 / 2,209 | 10 / 165,259 / 3,262 (2 embedded) | 7 / 114,488 / 2,509 |

Every row is inside the 3500 / 28 / 230000 target. Log: `🗑️generated/surface-census-after.log`.

## 2. Crate graph (downward only)

```
terrain, paint, node-graph (independent)
mvt  <-  model  <-  tiled-map (host)
facade `semio-framework-surface` re-exports terrain, paint, node_graph, tiled_map as modules
`semio-framework-os-infinite` -> terrain   (was: #[path] re-mount of the same source in two crates)
`semio-framework-surface-component` (cdylib) -> facade
```

| Crate | Node (lib root `<node>/🦀️.rs`, package `<node>/📦️packages/🦀️rust`) | Content | Dependencies |
|---|---|---|---|
| terrain | `🏔️terrain` | Terrarium decode, heightfield meshing, `TerrainSessionCore` | value, serde, serde_json, pixels |
| paint | `🎨️paint` (+ gesture, compositing) | raster layers, strokes, masks, `RasterSession` wasm bridge | canvas, value, serde, serde_json, pixels, ui_styling, async (wasm), js-sys/wasm-bindgen/web-sys (wasm); dev image |
| node-graph | `🕸️node-graph` (+ scene) | graph scene records, retained scene decode; owns the test allocator mount | pack, io-base64, pack-json, value, replication, ui-viewport, serde; dev serde_json, trace |
| mvt | `🗺️tiled-map/🧱️vector-tiles/📦️mvt-decoder` (the decoder file is the root) | MVT decoder, property helpers, tile-edge predicates | none; dev serde_json |
| model | `🗺️tiled-map/🧮️model` (new; mounts `../🧭️projection`, `../🧩️tiles`, `../🪢️map-state`, `../🎛️map-policy`, new child `📐️camera`) | projection, tile addressing, camera maths, descriptors and layer state, `MapPolicy` port | canvas, ui_styling, value, serde, serde_json, mvt |
| host | `🗺️tiled-map` (root file unchanged in place) | `MapHost`, camera/picking, host scene, interaction/labels, `MapSession` wasm bridge | canvas, ui_styling, value, serde_json, pixels, model, mvt, async + wasm deps (wasm); dev image |

Files stay where they are; no module file has two owners (`union.py` walks every `#[path]`). Manifests follow the recipe: `role = "framework"`, ids `surface-<x>`, exactly the three `rust-source-boundary` keys at the ceiling, `[lints] workspace`, `[lib] path = "../../🦀️.rs"`. Longest new path 150 bytes. Rust dependency aliases `tiled_map_mvt` and `tiled_map_model` keep the code short.

## 3. Code changes beyond wiring

- mvt: `decode_mvt` returns `Result<VectorTile, MvtDecodeError>` (was the host error enum, an upward reference). The host converts through the existing `From`. The decoder now mounts its own unit tests.
- model: the root helpers (camera maths, FNV tile revision, `map_viewport`, wheel and clamps, `map_view_facts`) moved verbatim into `📐️camera` and became `pub`; `MapPalette` moved to the model root; `MapTileLedger` and its fields and `touch_*` went from `pub(super)` to `pub`. Doc links to `MapHost` became plain spans. `tiles` imports `canvas::camera` directly.
- host root: explicit `pub use tiled_map_model::{...}` (54 names, no glob) plus a private `use` for the helpers the host children share; the host facade `🧱️vector-tiles/🦀️.rs` re-exports the mvt items explicitly. `pub(in crate::tiled_map)` became `pub(crate)`. `color_with_alpha`, `weighted_opaque_fill` and the cache-limit const stay in the host root.
- infinite: the `#[path]` mount of `🏔️terrain/🦀️.rs` and `framework_surface_terrain` are gone; `🌍️world/🦀️.rs` uses `semio_framework_surface_terrain::TerrainSessionCore`; the manifest depends on the terrain crate.
- facade: `🦀️.rs` is four `pub use <crate> as <module>;` lines; the manifest keeps only the four dependencies and dev `serde_json`; the dropped `pack-error` and `job` dependencies were unused by any source. `📜️script.ts` runs the four surface packages in `test` and the node-graph package in `test-scene-wire-native`.
- tests: the six pbf tiles are read at run time (`CARGO_MANIFEST_DIR`) instead of `include_bytes!` (355 KB less in the closure). Moved by function: 8 decoder tests to mvt, 18 pure tests to model, the host-level half of the policy tests to `🎛️map-policy/🧪️tests/🏛️host/🦀️.rs` (mounted by the host root, because they need `MapHost`). Nothing was rewritten except `super::` paths.

## 4. Consumers re-pointed

Rust: infinite (above), component docs. The engine, plugins and tests keep `semio_framework_surface::{paint,terrain,node_graph,tiled_map}::…` unchanged (the facade aliases); I checked every import of the engine, gismap, gisterrain and both oracle adapters against the host's `pub use` list.
TS and scripts: `🧪️tests/🧩️suite/🟦️.ts` (manifest, ports and tile-fixture checks now read the node-graph and mvt crates), root `📜️script.ts` filter line (now `semio-framework-surface-node-graph` / `scene_retained_tests::`), `🦑️repo/…/📦️packages/🟦️typescript/📜️script.ts` law package (`semio-framework-surface-paint`). The p3mn source reader (`policyReadModuleTreeSource` on `🗺️tiled-map/🦀️.rs`) still sees the host tree and every needle (`policy-reader-check.ts`: 7 files, slice non-empty).
Registrations for six crates: 6 workspace members in `🧰️framework/Cargo.toml` (byte order), 6 `📋️build-owners.json` rows, 36 launch routes in both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (orders from `900.0711`, `922.0471`, `903.5871`, all unused, inserted after the pets-staging block). `verify.ts`: ALL STATIC CHECKS PASSED (manifests parse under Bun TOML, workspace resolves, dependency names match, every command present once in both files, JSON parses).

## 5. Verification (commands actually run)

1. Union differential `union.py` (the crates simulated as modules of one crate, `crate::` and extern names rewritten, `pub(crate)` narrowed to the crate's own module, then compared with the pre-carve monolith under the oracle patches): native, `--test`, wasm32, wasip2, `--test` wasm32. Result for the final tree: split-only 1 (the "aborting due to N errors" count line), base-only 4 (the baseline's `super::Point/Rect` imports that only fail because the canvas glob is neutralised). Phase A (terrain, paint, node-graph): 692/692, 885/885, 825/825, 692/692 errors, zero differences. Log `🗑️generated/surface-union-final.log`. The oracle ignores errors that mention canvas names, so it does not cover canvas-glob names.
2. Seeded sensitivity: privatising `tile_range_revision`, `map_viewport` and `MapTileLedger` produced E0603, E0603 and E0432 in the split.
3. Definition scan (`defscan.py`, lower crates against upper crates): the only hits are trait methods and same-named public fields; no private item crosses a crate.
4. mvt compiles for real: `rustc --emit=metadata` (native, `--test` apart from serde_json) reports no error except the `/dev/null` temp-dir write, so its type-check passes. The other crates depend on unresolved externals, so only resolution is checked there.
5. Dependency completeness (`deps.py`): unresolved extern names per crate and cfg equal the declared dependencies; host `serde` was unused and removed.
6. Losslessness: the line multiset of `S` against the pre-carve snapshot differs only by the 89 lines listed above (mounts, imports, widened visibility, moved docs).
7. rustfmt: all touched and new `.rs` parse; new files formatted.
8. `bun test` of the surface suite: 3 pass, 2 fail. Both failures exist without my change: the ports test regex misses the `#[cfg(test)] use protocol::{…}` line, and the bindings folder carries an extra `.semio-artifact.json`.
9. `bun repo cargo --store=shared … check -p semio-framework-surface-tiled-map-mvt` and `update --workspace --offline`: both refused with `Finite original maintenance authority required` (logs `surface-cargo-check-mvt.log`, `surface-cargo-update.log`). No cargo check or test ran, and `Cargo.lock` was not refreshed (next refresh adds six packages, adds the terrain edge to infinite, drops six edges from the facade).

## 6. Residual risks

- No real compile or test run of the tiled-map crates, paint, node-graph and terrain; field and method privacy is covered by the definition scan only.
- A peer is editing `⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs` (working tree has `pub(in crate::m_geometry)`, HEAD has `pub(crate)`); it breaks node-graph's test build until fixed. Not mine.
- Browser: paint's `RasterSession` and the tiled map's `MapSession` now live in producer rlibs reached through the facade into the cdylib component. The mechanism is the same as before, but `bun repo run @semio-tech/framework-surface-rs:wasm` was not run.
- `🔒️dependencies.json` was not regenerated (new manifests declare serde and serde_json as users).
- The sibling session still owns: policy relocation, plugin consumers, the oracle adapter and fixture edits; I only read them.
- `.storybook/preview.tsx` still imports stale `@semio-tech/framework-surface-*-rs/pkg/…` paths (untouched).

## 7. Files

Created: `{🏔️terrain,🎨️paint,🕸️node-graph,🗺️tiled-map,🗺️tiled-map/🧮️model,🗺️tiled-map/🧱️vector-tiles/📦️mvt-decoder}/📦️packages/🦀️rust/{Cargo.toml,📋️project.json}` (under `S`), `🧮️model/{🦀️.rs,📐️camera/🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`, `📦️mvt-decoder/🧪️tests/🔬️unit/🦀️.rs`, `🎛️map-policy/🧪️tests/🏛️host/🦀️.rs`.
Edited: facade `Cargo.toml`, `🦀️.rs`, `📜️script.ts`; `🕸️node-graph/🦀️.rs`; `📦️mvt-decoder/🦀️.rs`; host `🦀️.rs`, `🧱️vector-tiles/🦀️.rs`, `🏛️map-host/🖼️host-scene/🦀️.rs`, `🧩️tiles/🦀️.rs`, `🪢️map-state/🦀️.rs`, `🎛️map-policy/🧪️tests/🔬️unit/🦀️.rs`, `🧪️tests/🔬️unit/🦀️.rs`; `🧩️component/🦀️.rs` (docs); `🧪️tests/🧩️suite/🟦️.ts`; infinite `📦️packages/🦀️rust/{🦀️.rs,Cargo.toml}` and `🌍️world/🦀️.rs`; `🧰️framework/Cargo.toml`, `📋️build-owners.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, root `📜️script.ts`, `🦑️repo/…/📦️packages/🟦️typescript/📜️script.ts`.
