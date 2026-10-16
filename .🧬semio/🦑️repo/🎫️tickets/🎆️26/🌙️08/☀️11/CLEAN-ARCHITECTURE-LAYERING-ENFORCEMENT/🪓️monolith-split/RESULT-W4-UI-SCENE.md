# Result W4: `semio-framework-ui-scene` carved into 13 producer crates plus a thin facade

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git. Tools and logs: `scene-carve-tools/` (Python and Bun scripts, regenerate the whole carve from a pristine copy of the baseline) and `🗑️generated/uiscene-*.log`.
`S` = `🧰️framework/🔨️modules/🖱️ui/🎬️scene`.

## Status in one paragraph

The split is done and applied. The 14 files / 481,386 bytes / 10,181 lines monolith is now 13 producers (largest lib closure 1,465 lines, largest test closure 1,892 lines, at most 12 files and 181,892 bytes) plus the facade `semio-framework-ui-scene`, which keeps its path and package name and re-exports exactly the old public surface with explicit `pub use` lists, so none of the ~65 consumer manifests or 123 consumer Rust files changed. The UI crate-boundary gate passes with all 13 new corpus rows (59 pass, 0 fail). **No `cargo check` or `cargo test` could run**: `bun repo cargo` fails with `Finite original maintenance authority required` and the Nx route with `Active graph readers retain required Nx pool headroom` (earlier: `Repository target execution contract refused`), both before any compile, at start and at end. Verification is therefore the patched full-resolution differential (name resolution only, see limits), a definition scan, an orphan scan, a dependency completeness check and the TS tests.

## Census (canonical `readRustSourceClosureV1`, `scene-carve-tools/census.ts`, host cfg; target 3500 lines / 28 files / 230,000 bytes)

Before: `semio-framework-ui-scene` lib 14 files / 481,386 bytes / 10,181 lines (over, worst ratio 2.545, 3 oversized deps). Native test closures of the single-file modules were over as well (math 4,883 lines, scenes 4,300 lines).

| Crate | lib files / bytes / lines | test files / bytes / lines |
| --- | --- | --- |
| `-algebra` | 1 / 8,180 / 203 | 2 / 9,261 / 231 |
| `-camera` | 1 / 47,789 / 922 (testkit: 2 / 49,240 / 960) | 4 / 96,474 / 1,415 |
| `-mesh` | 4 / 79,757 / 1,310 (testkit: 5 / 86,325 / 1,430) | 9 / 136,491 / 1,890 |
| `-pass` | 1 / 12,616 / 356 | 2 / 14,739 / 408 |
| `-picking` | 1 / 23,991 / 567 | 3 / 49,478 / 1,029 |
| `-pack` | 1 / 23,643 / 643 | 5 / 66,974 / 941 |
| `-doc` | 1 / 11,279 / 223 | 2 / 13,633 / 266 |
| `-modelling` | 1 / 53,668 / 1,277 | 4 / 136,440 / 1,625 |
| `-world3d` | 2 / 57,097 / 1,215 | 8 / 154,452 / 1,685 |
| `-node-graph` | 1 / 37,231 / 788 | 4 / 52,827 / 966 |
| `-plane` | 3 / 66,955 / 1,465 | 12 / 108,567 / 1,892 |
| `-views` | 3 / 60,072 / 1,203 | 11 / 181,892 / 1,529 |
| `-surface` | 1 / 4,370 / 83 | 3 / 8,553 / 162 |
| `semio-framework-ui-scene` (facade) | 2 / 12,713 / 92 | 4 / 22,464 / 166 |

All 14 rows: no embedded assets in a lib closure, every dimension below target, so the full census table is `🗑️generated/uiscene-census-after.log`.

## Crate graph (crate names are `semio-framework-ui-scene-<key>`; lib edges, then dev edges; `scene-carve-tools/dag.py`: no cycle in lib edges nor in lib+dev edges)

```
algebra   -> geometry
camera    -> algebra, geometry, viewport
mesh      -> algebra, geometry, mesh-engine, replication(protocol)
pass      -> algebra, mesh, geometry, serde              dev: camera(testkit)
picking   -> algebra, camera, mesh, pass, geometry, viewport   dev: camera(testkit), mesh(testkit)
pack      -> serde                                       dev: algebra, camera, viewport
doc       -> pack, replication, serde                    dev: fnv, serde_json
modelling -> replication, pack-json, value, serde        dev: algebra, camera, mesh(testkit)
world3d   -> doc, pack, modelling, pass, replication, serde
node-graph-> doc, replication, pack-json, viewport, serde dev: ui_contract
plane     -> doc, pack, replication, serde               dev: ui_contract, fnv
views     -> doc, pack, replication, pack-json, serde
surface   -> doc, pack, ui_contract                      dev: plane, views, world3d
semio-framework-ui-scene (facade) -> all thirteen        dev: serde_json
```

Nodes (lib root `<node>/🦀️.rs`, package `<node>/📦️packages/🦀️rust`): `📐️math/🧮️algebra`, `📐️math/🎥️camera`, `📐️math/🕸️mesh`, `📐️math/🎞️pass`, `📐️math/🎯️picking`, `📦️pack`, `🎬️scenes/📜️doc`, `📏️world3d-modelling`, `🎬️scenes/🌍️world3d`, `🎬️scenes/🕸️node-graph`, `🎬️scenes/🖼️plane`, `🎬️scenes/📄️views`, `🌉️surface`; the facade keeps `📦️packages/🦀️rust` and mounts `📐️math/🦀️.rs` as `pub mod math` (now an explicit `pub use` aggregator, 181 names).

Cohesion: algebra = the sync `Vec3`/`Mat4` mirror (E6); camera = orbit/parallel cameras, projection specs, gumball and LOD grid; mesh = paged owner/authority, resource custody, instance records; pass = materials, lighting, shadow, line/grid draws, culling; picking = ray and marquee selection; pack = serde codec; doc = `SceneDoc`, lane carriers, `scene_pack_wire!`, value primitives; modelling = annotations, scalar fields, section/highlight tokens; world3d, node-graph, plane (canvas, paint, board, tiled map, ink + framing + canvas snapshot), views (text editor, table, block list, diff, feed, timeline, VFS, icon + text splice) = product scenes; surface = the `ui_contract` bridge.

## What moved and what changed beyond wiring (each minimal)

1. Files stay where they are wherever one crate owns them whole (pack, modelling, surface, snapshots, framing, text-splice, block-list, retirement/resource children). Only `📐️math/🦀️.rs` and `🎬️scenes/🦀️.rs` were dismantled, by region, with every `//#region` marker kept (per-file start/end balance checked by `pathcheck.py`).
2. Visibility: the nine `*_m` helpers of the algebra, `deserialize_non_empty_interaction_domain_text`, `scene_lane_hash`, the eight `value_*` primitives, `SceneLaneRef` and the moved test helpers are `pub` in their new crate; pack's `TAG_NONE/SOME/SEQ/MAP` and `write_varint` became `pub` (the retained-scene catalog test needs them across crates). The facade does not re-export any of the new `pub` names, the public API is bit for bit the old one.
3. `scene_pack_wire!` is now `#[macro_export]` from `-doc` and imported by world3d, plane and views.
4. The `impl World3dScene { with_annotations … }` builder block was inside the modelling module; an inherent impl cannot span crates, so it moved next to the type into world3d (region `🔖️ModellingBuilders`), and modelling is now a leaf. `Canvas2dFraming`'s `ToValue/FromValue` impls moved with the type into plane.
5. `crate::pack::…`, `crate::math::…`, `crate::World3d…` qualified paths became imports; doc links in comments were left untouched.
6. Tests: `math-unit` (1,560 lines) became `🔬️{algebra,camera,mesh,pass,picking}-unit`; `scenes-unit` and `scenes-value-round-trip` became per-crate files; two mixed round-trip tests were split in two (`table_scene_round_trips`/`tiled_map_scene_round_trips`, `board2d_scene_round_trips`/`block_list_scene_round_trips`); the world3d tests of `world3d-modelling-unit` moved to `🔬️world3d-modelling-scene-unit`; the typed scene catalog test moved from pack-unit to the facade (`🔬️scene-catalog-unit`); `text_editor_buffer_lanes…` moved to surface-unit (it uses `encode/decode`); two plane tests that named `World3dSceneLane` use the literal lane body key instead. New: `🔬️doc-unit` (lane hash against published FNV-1a vectors and the `fnv` crate, lane reference codecs, value primitives).
7. Shared test kits, as in the pets carve: `🧰️mesh-testkit` (allocation observer, bounded retry, paged mesh fixtures, `pub mod testkit` behind `cfg(any(test, feature = "testkit"))`) and `🧰️camera-testkit` (the verbatim wire cameras). The `#[global_allocator]` static stays in each test module that needs it (mesh-unit, modelling-unit), so enabling the feature never installs an allocator.
8. Doc blocks: the old module docs of `math` and `scenes` are preserved verbatim in the math aggregator and in `-doc`.
9. Rule "god objects" (added during the task): not triggered. No struct's inherent impl is split across crates (orphan/inherent scan below) and no extension trait was added; `Vec3Math`/`Mat4Math` pre-date this task (E6 mirrors on foreign types) and stay whole in one crate.

## Wiring (recipe steps)

- 13 manifests (role `framework`, id `ui-scene-<key>`, exact three `rust-source-boundary` keys, `[lints] workspace`, `[lib] path = "../../🦀️.rs"`, `testkit` feature on mesh and camera) and 13 `📋️project.json` (`@semio-tech/ui-scene-<key>-rs`, tags `scope:framework`, `language:rust`).
- Facade manifest rewritten (13 path deps, dev `serde_json`); `📜️script.ts` `test` now runs all 14 packages through `runRepositoryCargoTests` (same call as pets), `check-wasm` unchanged (the facade closure covers every crate).
- `🧰️framework/Cargo.toml` members: 13 rows inserted contiguously with the existing scene row, byte-sorted among themselves (the file's list is not globally sorted by peers' edits).
- `📋️build-owners.json`: 13 owner rows, the 15 `ui-scene*` rows are lexically ordered.
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: 78 routes each (6 per crate), orders `900.01nnn / 922.04nnn / 903.58nnn` with nnn in 951..978, unused beforehand; both files parse, every command exists once, no duplicate order in a group.
- Corpus `🖱️ui/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json`: 13 rows; the schema `🖱️ui/🧬️schema/📦️crate-boundaries/🔣️.json` package pattern widened from `[a-z-]` to `[a-z0-9-]` because `world3d` has a digit.
- Comment-only path updates in peer files: the four `$comment`s of the `*-scene-lanes` fixtures and the scene `🟦️.ts` header, `World3dHost/🔭️scene-schema/🟦️.ts` (2), `Interpreter/🟦️.tsx` (1), the cad picking engine docs (1) and one doc line in os-infinite's world unit tests (new path and line numbers).

## Consumers re-pointed

None needed: the facade keeps path, package name, lib name and API. `scene-carve-tools/consumers.py` extracted every name taken from `semio_framework_ui_scene::` / `ui_scene::` in the 123 consumer Rust files (548 paths) and resolved all against the facade, `math` aggregator and `pack` module: 0 unresolved (seeded check: removing `TableScene` from the facade makes it report 11).

## Verification (commands run, actual results)

1. Baseline: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-ui-scene --message-format=short` -> `Finite original maintenance authority required` (`uiscene-cargo-baseline.log`); `bun repo run @semio-tech/ui-scene-rs:check …` -> `Repository target execution contract refused`. End of task: the same cargo error (`uiscene-cargo-after.log`), `bun repo run @semio-tech/ui-scene-algebra-rs:check …` -> `Active graph readers retain required Nx pool headroom` (`uiscene-nx-after.log`), `update --workspace --offline` -> same cargo error (`uiscene-cargo-update.log`). No cargo check, no cargo test and no unit test run exist.
2. Patched full-resolution differential (`union.py`, `run_all.py`): `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null`, extern crates removed and external globs neutralised identically; baseline = the pristine tree compiled from the original lib root; new = all 13 crates and the facade as modules of one union crate (`semio_framework_ui_scene_<k>::` mapped to `crate::…`, `crate::` inside a crate mapped to its module, `pub(crate)` to `pub(in crate::<module>)`, `#[macro_export]` macro imported by path, `#[global_allocator]` kept once). Result for native, `--test`, `wasm32-unknown-unknown`, `wasm32-unknown-unknown --test`, `wasm32-wasip2`: 311/311 errors native, 316/316 test (seven `couldn't read` of fixtures outside the sandbox are identical on both sides), **0 new and 0 vanished error classes** in all five. Sensitivity (seeded copy): removing an import gives 3 new E0425 (`Instance3d`, `SceneViewportMask3d`), making `transform_aabb` private gives 2 new E0603, so resolution and `pub(crate)` leaks are covered. Honest limit: with external crates (serde derive, geometry, protocol) missing, rustc stops after resolution, so field/method privacy (E0616/E0624/E0451), trait-method scope and type errors are not checked by it.
3. Definition scan (`defscan2.py`): private fields and methods of the types a crate actually names from another crate and uses there: no hit (4 false positives on `Type::base`, whose docs are block comments); seeded `lease.slot` use in picking is reported. Trait methods (`*_m`, `to_value`, `from_value`, `encode_pack`) are imported by a syntactic detector (`synth.py`), unverifiable by rustc here.
4. Orphan/inherent scan (`orphan.py`): every `impl` in every crate is inherent on a local type, or a trait impl of a local trait or type: clean.
5. Dependency completeness (`depcheck.py`): each crate compiled alone, lib and `--test`: unresolved external crate set equals the declared `[dependencies]` (lib) and is within and covers `[dependencies] + [dev-dependencies]` (test), 13/13 ok.
6. Losslessness (`lossless.py`): non-blank lines of all `.rs` before/after: 12,712 / 12,986; the 170 lost lines are exactly the intended edits (qualified-path rewrites, import blocks, mount paths, the dismantled module docs now carried over, visibility widenings, the split/moved test functions), listing in `lossless.py -v`.
7. rustfmt (`--check`, repo `rustfmt.toml`, nightly 2026-07-20): hunks 231 before, 227 after over the scene tree, i.e. only moved pre-existing debt; all authored header blocks are rustfmt output.
8. Path resolution (`pathcheck.py`): 548 `#[path]`/`include_*!` targets, 0 missing, region markers balanced in every file, longest path 141 bytes.
9. TOML/JSON: all 14 scene manifests parse with `Bun.TOML` (also checked by the gate under `@iarna/toml`), `📋️build-owners.json`, `launch.json`, `Bun.JSONC` of the seed and the corpus parse; 14 workspace rows exist, unique, byte-ordered.
10. Gate: `bun test ./🧰️framework/🔨️modules/🖱️ui/🧪️tests/📦️crate-boundaries/🟦️.ts` -> first run after my rows: `54 pass 0 fail` (41 existing tests + 13 new rows; the first attempt failed one schema test, see the schema pattern note above); final run `59 pass 0 fail 2482 expect() calls` (`uiscene-boundaries-final.log`), including per-row budgets, launch routes, token scan, acyclic check with the independent SCC oracle.
11. TS: `bun test` of the 7 scene TS suites (block-list, text-editor-lanes, table-lanes, text-splice, world3d-scene-lanes, world3d-modelling, node-graph-scene-lanes): `95 pass 0 fail` (`uiscene-ts-tests.log`).

## Residual risks and open items

- No compiler ever saw the new crates (see 1 and the limit in 2). Highest-risk spots to compile first: trait-method imports in the moved tests, the `scene_pack_wire!` macro invoked from three crates (it relies on `Serialize`/`Deserialize` being imported at the call site, which they are), the `#[serde(deserialize_with = "…")]` strings in node-graph and plane (resolved through the `-doc` import), and the `testkit` feature dev-dependencies of pass, picking and modelling.
- Lock files are not refreshed: root `Cargo.lock`, `🧰️framework/Cargo.lock` and the separate plugin/artifact workspace locks list `semio-framework-ui-scene` with its old dependency set. Run `update --workspace --offline` per lock once the launcher works; until then `--locked` resolves fail.
- `🔒️dependencies.json` (generated 2026-10-03) is untouched; `verify dependencies literal-external` was not run (launcher).
- Taxonomy: no change. The new folders sit at depth two below existing registry members (`📐️math`, `🎬️scenes`), like the existing `♻️retirement` and `📦️resource`; `members-of-ui-scene` is not extended because no new direct child of `🎬️scene` exists (`🎬️scenes`, `📏️world3d-modelling`, `📷️framing` were already outside that list). No taxonomy validator was run.
- The ticket MCP (`repo`) was unavailable and the brief did not require a ticket; no ticket was opened or closed.
- `@semio-tech/ui-scene-rs:test` now selects 14 packages with `-p` through `runRepositoryCargoTests`; the nextest filters in the existing launch routes (for example `test(ray_segment_distance)`) still match because test names are unchanged, but this was not executed.
- Facade explicit lists are generated from the old public surface; any future addition to a producer must be re-exported by hand.
