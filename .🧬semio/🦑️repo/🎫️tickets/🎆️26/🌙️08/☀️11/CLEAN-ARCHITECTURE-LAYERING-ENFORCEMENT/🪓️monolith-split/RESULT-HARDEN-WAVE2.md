# Hardening wave 2: patched oracle for puzzle editors, tiled-map, hub directory and inference runtime, repo CLI and GraphQL

Executor: Sonnet 5.5, foreground, no sub-agents, 2026-10-15. The same findings are appended as a "Hardening (patched oracle)" section to `RESULT-PUZZLE-EDITORS-SPLIT.md`, `RESULT-TILED-MAP.md`, `RESULT-HUB-DIRECTORY-INFERENCE-SPLIT.md` and `RESULT-REPO-CLI-GRAPHQL.md`.

## 0. Verdict

No regression in any of the four splits. Under the patched full-resolution differential every configuration has zero differing error keys (apart from three explained external-crate lines in the hub and the CLI, section 3). The definition-level scan (fields, methods, struct literals, tuple constructors, top-level items) reports no real candidate in any tree. Both checks were seed-tested: stripping widenings in a sandbox copy makes the oracle (items) or the scan (fields and methods) light up, section 5. **No repository source file was edited.** Edited files: the four RESULT files (appended section) and this report. Created: `harden2-tools/` and the logs `🗑️generated/harden2-final.log`, `harden2-scans.log`, `harden2-comb-gql-fs.log`.

Findings that are not split regressions but that block a real compile (identical in baseline and split, section 6): 2d editor imports private `snapshot` functions (9 x E0603), 2d unit test misses an import for `PUZZLE2D_NODE_KIND_OPTIONS_MAX` (2 x E0425). Both exist at `HEAD`.

## 1. Method

Only analysis-only `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --crate-name <crate> --emit=metadata -o /dev/null --error-format=json --cap-lints allow [--test] [--target wasm32-wasip2|wasm32-unknown-unknown] [--cfg 'feature="integration-fixtures"'] <real lib root>`. No cargo, no scratch workspace, no private target dir, no `RUSTC`.

- **Sandbox**: a directory tree in the session scratchpad in which every directory on the path of every module file is real and everything else is a symlink into the repo, so `#[path]` and `include!` resolve physically inside the sandbox. The lib root itself is always copied (a crate whose root sits below a symlinked `📦️packages` directory otherwise resolves `../../` into the real repo; this made my first tiled-map baseline silently the split tree, caught by the module count 29 / 29 and fixed: 16 / 29 now).
- **Patches** (identical on baseline and split, never on the repo; `neutral.py`): remove `extern crate` lines of missing crates (plus the attribute lines above them), replace every glob import of an external crate by `const _: () = ();` (nested `{self, *}` forms drop the `*`). Counts per tree: puzzle 5d and 2d 15 externs and 1 glob each (the `🌉️wasm` bridge, in the parent in both), tiled-map 0 externs and 2 globs (`pub use canvas::{self, *}` and the wasm bridge), hub 0 / 0, CLI 0 / 0, GraphQL 0 / 0. So in these trees the patch changes less than in the plugin and hub bin trees of wave 1; the large unpatched/patched gap is in the puzzle crates only (section 4).
- **Baselines** (only the split differs): `git show HEAD:` text saved by the splitters, placed at the monolith path, all other files current. Details per crate in section 2. Losslessness of each baseline against the current split family (`lossless.py`, normalised line multiset, `pub(...)` and `super::super::` undone): every missing line is an intended edit already documented in the RESULT files: inline `pub mod` openers and closers (hub 1, tiled-map 3 plus 2 braces, CLI 3 plus 3 braces), deleted empty region shells (2d 16 markers, 5d 2), the tiled-map `include!` path, the CLI cycle-break call `crate::repo_cli::format_markdown_result` and two relocated `use` lines, and GraphQL `fn parse_milestone_number` (now `pub fn`). No content line was lost, and no line is unexplained, so no peer changed the split files after the splitters ran.
- **Comparison**: multiset of (code, message with sandbox paths masked, primary-span text, file). All files of the split family fold into one key (`FAMILY0`, `FAMILY1` for the hub's second family). I dropped the primary span label from the key after the first puzzle-2d `--test` run: rustc adds a "similarly named constant" label only when the neighbouring constant is in the same file, which differs when the neighbour moved to a child (same error, different label).
- **Typeck limit** unchanged: with external crates unresolved rustc stops after resolution, so E0616, E0624, E0451 and trait-in-scope errors are never produced (verified: the error-code histograms contain only E0425, E0432, E0433, E0422, E0405, E0603, E0252, E0255, E0449). Fields, methods, struct literals and tuple constructors are covered by the definition scan.
- **Definition scan** (`famscan.py`, `famitems.py`, `tuplecheck.py`, token based on `scan.py`): for every private or narrowed (`pub(super)`, `pub(in ..)`) field, inherent method, associated const or trait, and every top-level item defined in a child, report a use by name (`.field`, `.call(`, `Type::name`, `Self::name`, struct-literal field, trait-method call, bare identifier) in a file that lies inside the **original** module scope but outside the new visibility scope. The original scope matters: a use from a file that was never inside the monolith module cannot have compiled before and is a name collision. For the tiled-map the original scope of the three former inline modules is their own module (`tiles`, `projection`) or the `vector_tiles` group; for every other child it is the whole parent subtree. For the CLI the scope includes `📡️event-stream` (it was inside `repo_cli`).
- Tools and logs: `harden2-tools/` (`ho.py`, `spec.json`, `neutral.py`, `mksbx.py`, `seed.py`, `comb.py`, `lossless.py`, `famscan.py`, `famitems.py`, `tuplecheck.py`, `runscans.sh`, baseline texts). Use `HO_WORK=<dir>` for the sandbox location; `python3 ho.py prep <t>; python3 ho.py run <t> <cfg,..> [patched|raw]` with targets `p5 p2 tm hub cli gql gqlfs gqlfull`. Logs: `🗑️generated/harden2-final.log` (all differentials), `harden2-scans.log`, `harden2-comb-gql-fs.log`.

## 2. Baselines and families

| Target | Crate, lib root | Baseline text | Extra baseline edits (so only the split differs) | Family regex folds |
|---|---|---|---|---|
| p5 | `semio_s_artifact_puzzle_5d`, `🖐️5d/🦀️.rs` | `🗑️generated/puzzle-editors/orig5d.rs.txt` (equals `HEAD`) at `✏️editor/🦀️.rs` | none (the current unit and retained-wiring test files read the children by `include_str!`, present in both trees) | parent + `🧩️entities 🧭️view 🗂️catalog 🕹️interaction 🖌️session 🧵️retained 👆️gestures 🧰️kit` |
| p2 | `semio_s_artifact_puzzle_2d`, `◻️2d/🦀️.rs` | `orig2d.rs.txt` (equals `HEAD`) | none | parent + `🕹️interaction 🖌️session 🧵️retained 🧰️kit` |
| tm | `semio_framework_surface`, `🗺️surface/📦️packages/🦀️rust/🦀️.rs` | `HEAD` tiled-map root (4,187 lines) | `image` to `semio_framework_pixels` (the `From<image::ImageError>` impl becomes `From<semio_framework_pixels::RasterError>`, `upload_tile` uses `decode_png`; 4,185 lines) as in the split; the decode-geometry test file is the `HEAD` version (the two GIS tests still inside), so the two tests the split moved to `🌍️gis-policy/🎨️vector-style/🧪️tests` are compiled on both sides | parent, geometry test, 8 child folders (all `🦀️.rs` below them) |
| hub | `semio_hub`, `🌎️hub/📦️packages/🦀️rust/🦀️.rs` | `dirinf-tools/orig-directory.rs.txt` and `orig-runtime.rs.txt` (equal to `HEAD`) | the 8 occurrences of `<X as pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot>::decode_pack` rewritten to `<X as pack::snapshot::ArtifactPack>::decode_pack` exactly as in the children; `🗿️artifact-authority/🌱️creation/🧬️schema/🦀️.rs:175` reverted to `crate::directory::NewDirectoryEvent` (the split pointed it at `directory::events::`) | `FAMILY0` directory parent + 8 children, `FAMILY1` runtime parent + 11 children |
| cli | `semio_framework_repo_cli`, `⌨️cli/🦀️.rs` | `rcg-tools/cli-baseline-import-retargeted.rs.txt` (original with the `FsRepoContext` import retargeted to the new crate) | none | root, `🧭️repo-cli` tree, `📡️event-stream`, `🔌️mcp-verb` |
| gql core | `semio_framework_repo_graphql`, `🔗️graphql/📦️packages/🦀️rust/🦀️.rs` | `graphql-original.rs.txt` lines 1 to 3,956 plus 5,031 to 5,038 (the `FsRepoContext` region 3,957 to 5,030 left for the adapter) | none | root + 6 children |
| gql + fs adapter (combined) | one crate containing both as sibling modules | full original 5,038 lines | adapter `use semio_framework_repo_graphql::` rewritten to `use crate::semio_framework_repo_graphql::` (sandbox copy only) so the 3 core names the adapter imports are resolved and privacy-checked against the split core | both |
| gqlfs (standalone) | `semio_framework_repo_graphql_fs`, `🔗️graphql/🗄️fs-context/🦀️.rs` | none | none | every name must resolve without a baseline |

## 3. Differential results (patched)

Totals are errors (rustc `error` level incl. the final "aborting" line), "non-ext" is the same after removing unresolved-crate lines (`cannot find module or crate`, `unresolved import`, `use of unresolved module`, `aborting`, `cannot find attribute`). "Differing keys" = split-only plus baseline-only after the key normalisation. Final run: `🗑️generated/harden2-final.log`.

| Crate | Config | Baseline / split | Non-ext | Differing keys |
|---|---|---|---|---|
| p5 | native | 4,276 / 4,276 | 50 / 50 | 0 |
| | `--test` | 8,340 / 8,340 | 189 / 189 | 0 |
| | wasm32-wasip2 | 4,276 / 4,276 | 50 / 50 | 0 |
| | `--test` wasm32-wasip2 | 8,340 / 8,340 | 189 / 189 | 0 |
| p2 | native | 3,710 / 3,710 | 143 / 143 | 0 |
| | `--test` | 9,826 / 9,826 | 508 / 508 | 0 |
| | wasm32-wasip2 | 3,710 / 3,710 | 143 / 143 | 0 |
| | `--test` wasm32-wasip2 | 9,807 / 9,807 | 508 / 508 | 0 |
| tm | native | 689 / 689 | 265 / 265 | 0 |
| | `--test` | 878 / 878 | 308 / 308 | 0 |
| | wasm32-unknown-unknown | 820 / 820 | 303 / 303 | 0 |
| | wasm32-wasip2 | 689 / 689 | 265 / 265 | 0 |
| hub | native | 1,538 / 1,539 | 34 / 34 | 1 (below) |
| | `--test` | 3,059 / 3,060 | 83 / 83 | 1 |
| | `integration-fixtures` | 1,610 / 1,611 | 36 / 36 | 1 |
| | `--test` + `integration-fixtures` | 3,122 / 3,123 | 85 / 85 | 1 |
| cli | native | 301 / 304 | 0 / 0 | 3 (below) |
| | `--test` | 301 / 304 | 0 / 0 | 3 |
| gql core | native | 10 / 10 | 0 / 0 | 0 |
| | `--test` | 10 / 10 | 0 / 0 | 0 |
| gql + fs combined | native | 128 / 130 | 0 / 0 | 2 (below) |
| | `--test` | 128 / 130 | 0 / 0 | 2 |
| gqlfs standalone | native, `--test` | 122 / 122 | 0 | all errors external, no unresolved in-crate name |

The remaining differences are all external-crate resolution that is correct in a real build:
- hub (4 configs): one extra `E0433 cannot find module or crate directory` in `📇️directory/🕰️events/🦀️.rs:3`, the explicit `use directory::os_directory::{DirectoryActor, DirectoryEventBody, Hlc};`. `directory` is a Cargo dependency (`semio-framework-os-kernel`, `Cargo.toml:67`) and the three names are imported by the parent at `🦀️.rs:50`. Same cause as in `RESULT-HUB-DIRECTORY-INFERENCE-SPLIT.md` section 6 (before it was counted with the other 1,517 errors).
- cli (2 configs): three extra `E0432` for the `use semio_framework_repo_graphql`, `semio_framework_repo_model`, `serde_json` lines of the new `📡️event-stream` file; all three are declared in `⌨️cli/📦️packages/🦀️rust/Cargo.toml`.
- combined gql + fs (2 configs): two extra `E0432` for the adapter's own `semio_framework_repo_identity` and `serde_json` imports (both declared in the adapter manifest per `RESULT-REPO-CLI-GRAPHQL.md`).

No new E0412, E0425, E0422, E0423, E0432 (in-crate), E0433 (in-crate), E0603 in any configuration. Zero line edits were therefore made: no widening, no `super::super::` fix, no `#[cfg(test)]` import, no `include!` depth change was needed.

Wave-1 cross-check of earlier numbers: the earlier 5d `3157 = 3157` and `--test` `6884 = 6884` reproduce exactly in the unpatched run (below); tiled-map earlier saw `691 vs 689` because its baseline still used `image`; with the same substitution on the baseline it is `689 = 689` (and `878`, `820`, `689`).

## 4. Sensitivity of the patch (unpatched versus patched totals)

Unpatched = the same sandboxes without `extern crate` stripping and with glob imports intact; baseline / split.

| Crate | Config | Unpatched | Patched | Patch exposes |
|---|---|---|---|---|
| p5 | native | 3,157 / 3,157 (non-ext 65) | 4,276 / 4,276 (50) | +1,119 errors |
| | `--test` | 6,884 / 6,884 (204) | 8,340 / 8,340 (189) | +1,456 |
| p2 | native | 1,359 / 1,359 (38) | 3,710 / 3,710 (143) | +2,351 (+105 non-ext) |
| | `--test` | 1,813 / 1,813 (50) | 9,826 / 9,826 (508) | +8,013 (+458 non-ext) |
| tm | native, `--test` | 689 / 689, 878 / 878 | same | 0 (2 globs replaced, totals unchanged) |
| hub | native | 1,538 / 1,539 | same | 0 (no extern crate and no external glob in the lib tree) |
| cli, gql | native, `--test` | 301 / 304, 10 / 10 | same | 0 |

So the blindness of the unpatched oracle is real for the puzzle crates: the 15 failing `extern crate` lines (E0463, e.g. `extern crate semio_framework_os_kernel as dsl;`) make every path that goes through the alias fail silently, while the patched run checks those paths (E0433 2,023 to 3,146 and E0425 11 to 11 in 5d native; E0433 450 to 2,723 and E0425 0 to 76 in 2d native). The extra visible errors are identical in baseline and split. In the other trees the patch is a no-op, which is itself a result: there is nothing hidden behind an external glob. The one glob that matters for name flow, `pub use canvas::{self, *}` at the tiled-map root, now makes every canvas name (`Point`, `Color`, `Scene`, ...) unresolved on both sides, so a child that lost its `use super::*` would hide behind it. Static counter-check: every tiled-map child that uses canvas names starts with `use super::*;` or `use super::{..}` (listed by grep), `🧱️vector-tiles/📦️mvt-decoder` uses no canvas name (it is the neutral decoder and its only former import, `use super::Color`, moved to the style child), and the 138 child names do not collide with any `pub` name of the canvas crate sources (re-computed: 190 canvas names, 111 tiled-map names, intersection 0).

## 5. Definition scan and seeded sensitivity checks

Candidates after the original-scope filter, per tree. "Defs" = fields / inherent members / top-level items defined in the family files, private versus narrowed (`pub(super)` or `pub(in ..)`).

| Tree | Family files | Private / narrow fields | Private / narrow members | Private / narrow top-level | Candidates | Real |
|---|---|---|---|---|---|---|
| p5 | 9 | 131 / 244 | 40 / 59 | 92 / 62 | 0 member, 0 item, 0 tuple | 0 |
| p2 | 5 | 63 / 58 | 28 / 14 | 75 / 23 | 0 / 0 / 0 | 0 |
| tm | 15 | 75 / 20 | 55 / 8 | 60 / 18 | 81 member rows (6 owner groups), 0 item, 0 tuple | 0 |
| hub (directory 9 files, runtime 12 files, scanned as two families) | 21 | 14 / 122 | 24 / 62 | 54 / 96 | directory 0 / 0 / 0, runtime 0 / 0 / 0 | 0 |
| cli | 9 | 3 / 0 | 11 / 2 | 65 / 9 | 1 member row (`SharedBuffer.0`), 0 item, 0 tuple | 0 |
| gql core | 7 | 7 / 26 | 24 / 7 | 65 / 33 | 0 / 0 / 0 | 0 |

The tiled-map rows are all name collisions, each read against the source: `VisibleTileCursor` x/y/z (private fields of a `pub struct` whose only outside references are by type: `tile_range_revision`, `visible_raster_tile_cursor`; scope equal to the original `tiles` module), `RawTile`/`RawLayer`/`RawFeature`/`ProtoCursor`/`DecodedGeometry` (types named only inside `📦️mvt-decoder`; every hit is outside the original `vector_tiles` group), `MapHostRetirement` fields (type named only in its own file, constructed by `MapHostRetirement::new`; hits are `MapFeatureTables`/`MapTileLedger`/`MapHost` fields), `VectorTileCoordinates` (named only inside `🖼️host-scene`), `LabelDeclutter.width/height` (struct pub(super), fields never read outside the file), and `tile_rect_intersects_viewport`/`tile_raster_affine` (`pub(in crate::tiled_map)`, users are the unit tests, which are inside that scope). The one CLI row is `.0` on unrelated tuples; `SharedBuffer` is named only in `🚀️run-and-projection`.

The GraphQL core/adapter edge is additionally checked by name (the adapter imports exactly `parse_milestone_number`, `ContextError`, `RepoContext`; all three are `pub` in `🖼️sources` / `🎥️context-port` and reach the crate root through `pub use <child>::*`) and by the combined differential above.

Seeded checks (sandbox copies of the split files only, restored after each run; `seed.py`). "Clean" is the unseeded split.

| Tree | Seed | Oracle (items) clean to seeded | Scan (members) clean to seeded |
|---|---|---|---|
| p5 | strip `pub(super)` from 5 top-level items of `🖌️session`; for the scan from 18 fns and 70 fields | 4,276 to 4,283 (+1 E0425, +6 E0433) | 0 to 110 rows (items scan 0 to many) |
| p2 | same in `🖌️session` (4 items; 13 items and 32 fields) | 3,710 to 3,714 (+2 E0425, +2 E0433) | 0 to 37 member rows + 4 item rows |
| tm | 8 items in `🌍️gis-policy`; for the scan 3 items and 5 fields in `🪢️map-state` | 689 to 696 (+1 E0432, +6 E0425) | 81 to 99 member rows, `MapTileLedger` flagged by the item scan |
| hub | 30 items in `🔐️committer`; for the scan also 39 fields | +80 (72 E0433, 4 E0425, 4 E0422) | 0 to 146 member rows + 11 item rows |
| cli | `pub(crate)` of 2 items in `📡️event-stream`; of 7 in `🗂️tree-source-and-search`; scan: 2 methods in `🎮️command-framework` | +2 E0425; +11 E0425 | 1 to 5 rows |
| gql | `pub(crate)` of `matches_filter` in `⚡️execution`; scan: 4 fns and 8 fields | 10 to 11 (+1 E0425) | 0 to 4 rows |
| gql + fs | `parse_milestone_number` to `pub(super)` | +1 E0603 in the adapter | |

So a missing top-level widening is caught by the oracle and a missing field or method widening by the scan; both fire on every tree. (The first hub seed, stripping the single `pub(super)` top-level item of `🛠️service`, produced no new error: that item is not used outside its child, i.e. the widening there is not load-bearing.)

## 6. Findings that are not caused by the splits

Identical in baseline and split, hence outside this task; they will stop a real compile of the 2d crate and should go to the owner of the 2d editor restructure:
- 9 x `E0603` in 2d: `new_node_id`, `patched_field_value`, `puzzle2d_placed_handles`, `unique_edge_id`, `unique_node_id` (imported by `📼️recorder/🦀️.rs:7`) and `puzzle2d_interaction_definition`, `puzzle2d_node_kind_arg`, `sync_host_fixture_content`, `sync_host_runtime_state` (imported by the editor parent, line 17) are private `fn` in `📸️snapshot/🦀️.rs` (for example `fn new_node_id` at line 1006). `HEAD` has the same.
- 2 x `E0425` in the 2d `--test` configs: `PUZZLE2D_NODE_KIND_OPTIONS_MAX` (`pub const` at `📸️snapshot/🦀️.rs:322`) is used at `🧪️tests/🔬️unit/🦀️.rs:1535` and `:1747` without an import. `HEAD` has the same two uses.
- `include_str!` of the missing fixture `🎮️commands/📥️import-snapshot/🧪️tests/🧫️fixtures/📄️payload-input/🔣️.json` (2d `--test`), also in both.

## 7. Residual risks

- Trait-in-scope for method calls, type and trait coherence, `private_interfaces` and orphan rules are not decided by rustc here (no typeck). The scan checks traits defined in a child only by method name; no trait method of a child-defined trait is called outside its child in any tree (scan `trait` kind: 0 rows). Traits imported by the monolith's top-level `use` and needed by a child without `use super::*` are not systematically checked (a missing import of an external trait would not appear as an unresolved name). Mitigation: the lossless check shows every moved `use` line is present, and `use super::*` is in every child that is not an old inline module.
- Names supplied by an external glob (`canvas::*`, the only one that matters) are invisible on both sides; checked by hand and by the name-collision computation above, not by the compiler.
- The scan matches by name. Receiver-unprovable hits were read for the 81 + 1 candidates; rows in the clean trees are zero, so there is nothing to read for p5, p2, hub, gql.
- The oracle runs one configuration at a time; combinations beyond the listed (for example `wasm32-unknown-unknown` for the puzzle crates and the hub) were not run.
- Results hold for the tree state at about 05:50; peers keep editing neighbouring files. The 2d crate is mid-restructure by another party (section 6).
- Nothing was compiled by cargo and no test was executed, as before; the earlier scratch-workspace compile evidence of `RESULT-REPO-CLI-GRAPHQL.md` section 7.2 remains the only type-checked evidence for the CLI and GraphQL crates.
