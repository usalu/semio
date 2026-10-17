# Result: CRATE-ROLLOUT-R6 (prefix `r6`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/r6-tools/` (`step.sh <tag> <cmd...>`, `r6-census.ts`, `r6-files.ts`, `r6-taxcheck.ts`). Logs: `F/🗑️generated/r6-*`. Sandboxes: `/Users/ueli/.cache/r6/`. Started 2026-10-17 09:03.

## 0. Verdict (kept current)

| Package | Verdict | Notes |
| --- | --- | --- |
| semio-s-plugin-wfc-engine | DONE (static) | 6 producer crates plus the unchanged facade; all inside the gate (lib and test); `--check-applied` exit 0, 0/0 extras |
| wfc-3d | DONE (static) | 10 crates plus facade; all inside the gate; `--check-applied` exit 0, 0/0 extras |
| wfc-2d | DONE (static) | 10 crates plus facade; `--check-applied` exit 0, 0/0; no source change needed |
| wfc-bitmap | DONE (static) | 9 crates plus facade; `--check-applied` exit 0, 0/0 |
| wfc-grid2d | DONE (static) | 7 crates plus facade; `--check-applied` exit 0, 0/0 |
| wfc-grid3d | DONE (static) | 7 crates plus facade (`--mutation-families=on`); `--check-applied` exit 0, 0/0 |
| gis gisterrain | DONE (static) | 6 producer crates plus facade; all inside the gate (lib and test); `--check-applied` exit 0, 0/0 extras (base `dir:`, see 2.5) |
| gis gismap | DONE (static) | 15 producer crates (one is the demo-asset crate) plus facade; all inside the gate; `--check-applied` exit 0, 0/0 extras (base `dir:`, see 2.4) |

## 1. Baseline census (cgc7 reader, host cfg, `F/🗑️generated/r6-census-base0.tsv`)

| crate | lib files/bytes/lines | test closure files/bytes/lines |
| --- | --- | --- |
| semio-s-plugin-wfc-engine | 43 / 398,750 / 9,356 | 91 / 641,968 / 15,039 |
| semio-s-artifact-wfc-2d | 138 / 564,027 / 8,624 | 274 / 981,031 / 12,885 |
| semio-s-artifact-wfc-grid2d | 110 / 509,000 / 8,348 | 217 / 809,663 / 12,121 |
| semio-s-artifact-wfc-bitmap | 119 / 528,898 / 8,725 | 224 / 845,835 / 13,314 |
| semio-s-artifact-wfc-3d | 147 / 539,917 / 8,867 | 288 / 913,556 / 13,921 |
| semio-s-artifact-wfc-grid3d | 110 / 499,586 / 7,841 | 254 / 903,026 / 11,885 |
| semio-s-artifact-gis-gisterrain | 100 / 294,291 / 4,431 | 149 / 468,774 / 6,397 |
| semio-s-artifact-gis-gismap | 203 / 1,058,280 / 12,314 | 388 / 2,058,629 / 20,027 |

Dependency order verified: the five wfc artifacts depend on the engine crate (workspace dependency `semio-s-plugin-wfc-engine`, unchanged name); gismap and gisterrain are independent siblings (no dependency either way).

Precondition: the artifact directories carry staged (index) changes from earlier file-split slices and no untracked files; the engine directory is clean against HEAD.

## 2. Per package

### 2.1 semio-s-plugin-wfc-engine (DONE, static)

- The generator (`new crates`) plans a flat `#[path] pub mod x;` tree only as detached test crates and then fails at the router patch (this engine has no artifact router). It cannot carve the lib closure, so the carve is a hand-built, scripted module carve: `F/r6-tools/engine-carve.ts` (plan or apply) and `F/r6-tools/engine-locks.ts` (lock patch).
- Module graph (comment-stripped `crate::x` references, lib and test): acyclic except two units ({diag, outcome} and the constraint family {constraint, constraints_card, constraints_conn, flow}); the earlier "15-module cycle" was doc links only. No impl of a trait for a type from another module exists (orphan scan), no `pub(crate)` except one test hook, no `super::` in lib files.
- Result (facade `semio-s-plugin-wfc-engine` keeps its name and its public module paths by `pub use child::{modules}`; every child crate re-imports the sibling modules it names with a private `use dep::{mods};` in its lib root so no module source line changed):

| crate | node under `⚙️engine` | modules |
| --- | --- | --- |
| `-foundation` | `🏛️foundation` | bitset, domain, error, heuristics, ids, model, propagate, sample, soft, topology, weights |
| `-oracle` | `🧿️oracle` | oracle, model_vectors (were `cfg(test)` of the engine; now a normal crate used as dev-dependency) |
| `-kernel` | `🧠️kernel` | beam, constraint, constraints_card, constraints_conn, diag, flow, nogood, outcome, prop_ac3, prop_ac4, trail |
| `-job` | `💼️job` | job (publication, headless) |
| `-solve` | `🧗️solve` | chunk, evolve, hierarchy, parallel, repair, search, serial, solver_graph |
| `-lattice` | `🧮️lattice` | extract, grid2d, grid3d, motif, solver_grid2d, solver_grid3d, sparse3d, symmetry, tiled |

- Source changes (cfg(test) rule): the `cfg(test)` hook `job::test_step_context` (used by the job tests and by the facade-level `grid_job_drive_tests`) is no longer an item of the job crate. It moved to the test-support crate `-oracle` (new module `🧿️oracle/🧰️step-context`, which now depends on `-job`) and is a dev-dependency of `-job` and of the facade (a dev-dependency cycle job to oracle to job, which Cargo allows because the function only takes and returns framework types). No `cfg(any(test, ..))` and no feature.
- Wiring: workspace members (7), 6 owner rows, router `📜️script.ts` checks/tests all 7 packages, `Cargo.lock` of the wfc workspace (members, including the oracle dev-dependency) and of `🏭️bridge`, the five artifacts and `🌎️hub` (non-members: path crates only, no dev-dependencies).
- Generator bug found and fixed with a test: `patchCargoLock` (L/🏗️authoring/📦️crate-tree/📝️render) keyed packages by name only, so any lockfile with two versions of a crate lost entries (hub: 1163 lines, bridge and 2d/grid3d locks: 400 to 500 lines) and dropped the blank line after the header. It now keeps every other block byte-identical and in place and inserts the new blocks at their sorted position (test `keeps every version of a multi-version package when it patches the lockfile`; the 50 crate-tree tests pass).
- Verification: `new crates <engine facade pkg> --check-applied --sandbox=/Users/ueli/.cache/r6/engine-applied` exit 0 (native base 54 / combined 56 / extra 0, test 140 / 140 / 0, admission, shared, sharedForeign empty); `rustfmt --check --config skip_children=true` parses all 7 roots; owners scan: no wfc row in `missing` (2588 packages / 2721 owners); `r6-taxcheck.ts` wfc: mounts 1, tree 1 (the same plugin-root finding as before); `lib-wiring-budget`: the only failures are two foreign roots (jpg 527 to 551, energy schema 165 to 214).
- Census after (lib / test, files/bytes/lines): foundation 12/79,574/2,056 / 24/110,891/2,974; oracle 3/10,613/266 / 4/13,846/355; kernel 12/70,108/1,633 / 23/120,881/2,752; job 4/111,873/2,544 / 7/144,447/3,168; solve 9/71,431/1,470 / 18/128,246/2,782; lattice 10/67,427/1,683 / 22/120,508/2,887; facade 1/1,116/15 / 2/9,002/180. Before: 43/398,750/9,356 lib and 91/641,968/15,039 test.

### 2.2 wfc-3d (`✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d`, DONE static)

- Baseline plan (`new crates --dry-run --explain`, also with `--mutation-families=on`): one 31-slice, 119-file, 4,975-line cycle (snapshot, diff, mutation aggregate, 17 leaves, inferences, examples). Causes found with `F3_SLICES=1` and fixed in the source (no behaviour change):
  1. `io/text/snapshot` held an inline `mod mutations_codec` that imported all 17 leaf builders and the aggregate without using them (only `wfc3d_snapshot_json_round_trip` is used): the unused imports are removed (same in wfc-bitmap).
  2. The inverse companions of `drag-slots` and `set-slot-positions` named `Wfc3dSlotPosition` through the aggregate re-export; they now import it from `schema::mutations::set_slot_positions`.
  3. Snapshot unit tests of the binary and text facet glob-imported their host (`use ...::snapshot::*`) and used the `examples` fixtures, which kept them in the snapshot slice (an upward edge to examples); they now import `encode, decode, decode_into / parse_dsl, print_dsl` and `Wfc3dSnapshot` explicitly.
  4. The inline `examples` wrapper in the artifact root held `sources()`/`example_source_slice()` plus a `mod tests` child that reached the inferences and the text codec (upward edge, and the wrapper could not be split). The outcome test is now a root-level `#[cfg(test)] mod examples_outcome` (its 13 `super::` paths are `crate::examples::`), and `examples` is a component module `standards::v1::subsets::any::examples` (file `📚️examples/🦀️.rs` now holds the roster functions and the three example mounts) with a flat root shim `crate::examples`, like `schema`, `diff`, `mutations`, `inferences`.
  5. Three test or visibility defects the single-crate and layout differentials reported (all fixed in the rightful owner): `Wfc3dRetainedCommandJobFactory` was `pub(super)` but named by the editor test (now `pub`, reaches the test through the existing `pub use retained_commands::*`), `MAX_WFC3D_TILES` was `pub(crate)` and used by the host inference test (now `pub`, and the test names the schema inference types explicitly), and the editor test used `include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../.../fixture.json"))` which the combined harness cannot rebase (its manifest dir is the sandbox root for base and combined alike): it is a file-relative `include_str!` now.
- Commands: `--dry-run --explain` exit 0, `--verify --oracle --sandbox=/Users/ueli/.cache/r6/3d` exit 0 (oracle native 0, test 0; layout 0, 0), apply with `--backup=F/🗑️generated/r6-backup-3d` (129 files, 10 crates, 10 owner rows), `--check-applied` exit 0 (native 1348 / 1369 / 0, test 2459 / 2481 / 0, admission, shared, sharedForeign empty; base healed by the editor test).
- Census after (lib / test): snapshot 24/91,826/1,366 / 26/104,852/1,552; schema 27/99,387/453 / 28/101,114/494; config 11/21,513/464; wfc3d (editor) 14/152,366/2,776 / 19/205,234/3,641; transient 9/15,173/341; inferences 14/70,079/1,381 / 17/95,343/1,789; mutations 13/52,050/854 / 29/191,739/3,084; create-slot 22/36,932/765 / 23/39,835/824; set-slot-positions 23/39,634/819 / 28/54,053/1,136; tests 6/34,463/678 / 11/66,340/1,330; facade 2/15,998/294 / 7/58,356/776. Before: 147/539,917/8,867 lib and 288/913,556/13,921 test.

### 2.3 wfc-2d, wfc-bitmap, wfc-grid2d, wfc-grid3d (DONE static)

- 2d: the plan had no problems (10 crates). `--verify --oracle` exit 0, apply 117 files, `--check-applied` exit 0 (native 1334 / 1355 / 0, test 2528 / 2547 / 0). Census after: largest lib `-schema` 28/174,582/836 (bytes), `-v1` editor 11/153,982/2,858, all test closures at most 29 files.
- bitmap source changes (all behaviour-neutral): unused leaf imports removed from the inline `mutations_codec` of `io/text/snapshot` (as in 3d); `pub(crate)` widened to `pub` in `io/text` (`bitmap_json_*`, `encode/decode_inference_value`, snapshot native helpers); the four `bitmap_json_{value,encode,bind,decode}` helpers moved from the `io/text` root (which mounts mutations and so merged snapshot, diff and the leaves into one coherent group of 72 files) into `io/text/snapshot` next to the base64 helpers they use, and all 160 call paths became `io::text::snapshot::bitmap_json_*`; `io/transient` codecs import `transient::component::*` instead of the private `component::mutations`; host inference test names the schema inference types; shared test scene: `crate::tests::fixtures::base()` (a root `cfg(test)` item that the editor brush test and the paint-input-stroke leaf test used from other crates) is now `BitmapSnapshot::reference_scene()` in the snapshot crate and the root fixture, the leaf test and the brush test all call it. 9 crates, `--check-applied` exit 0.
- grid2d: only the host inference test needed explicit schema imports and `pub(crate)` constants widened (7 crates).
- grid3d: examples component move as in 3d (inline `examples` with `mod tests` to component plus root `examples_outcome`), `sqlite/snapshot/native` child now `use super as sqlite;` (it cycled with its parent through an absolute alias), `pub(crate)` widened in sqlite and text inference helpers, the 14 inverse companions call the leaf builders by an imported name instead of `crate::mutations::name(..)` (the mutation-family conversion only rewrites imported builders), host inference test imports. Applied with `--mutation-families=on` (7 crates).
- Generator bugs fixed with tests (`L/🏗️authoring/📦️crate-tree/🧪️test-closure`, tests in `L/🧪️tests/📦️crate-tree/🟦️.ts`, 66 tests pass): `fixturesToRuntime` turned `const NAME: &str = include_str!(..)` or arrays inside inline modules into calls of a helper that was defined only at the end of the file; every fixture constant (any type, any depth) becomes a lazily read `static` now and the helper is reached with `super::` per enclosing inline module. Earlier in this task: `patchCargoLock` multi-version fix.
- Census after (lib / test): 2d `-v1` 11/153,982/2,858 / 13/172,535/3,141, `-inferences` 11/73,413/1,420 / 16/103,241/1,889, `-schema` 28/174,582/836, `-diff` 28/48,659/1,065 / 29/50,414/1,098, `-mutations` 13/54,888/933 / 29/195,593/3,022, facade 2/16,027/296; bitmap `-bitmap` 18/204,526/3,545 / 22/216,634/3,763, `-snapshot` 26/102,283/1,853, `-schema` 21/92,756/146 / 28/151,609/1,051, `-inferences` 14/95,358/1,741; grid2d `-inferences` 23/107,667/2,020 / 29/160,391/2,852, `-tests-cuts-the-pinned-corner...` 12/138,875/2,716 / 20/184,980/3,523, `-v1` 5/77,239/1,253; grid3d `-editor` 11/167,155/2,985 / 17/205,821/3,637, `-snapshot` 26/102,220/1,621, `-schema` 26/109,716/192, `-mutations` 16/50,090/858 / 29/167,233/2,611. All inside 32 / 262,144 / 4,000 for lib and test.
- Hand checks for the coordinator rules (script `F/r6-tools/r6-roots.ts`): for every produced crate of the engine and the five artifacts, every `crate::<first segment>` used in its closure appears in that crate's own root or mirror root (no `crate::host::..` style path through a kept root file module points at a moved item); no crate references another crate's `cfg(test)` item (grep for `tests::`, `crate::tests`, `mount_contract`, `store_fixture`, `examples_outcome`, `fixture_cases` finds only definitions and comments; the kept root test mounts are hoisted into the facade).

### 2.4 gis gismap (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap`, DONE static)

- Baseline problem: one 203-file, 12,314-line cycle plus a 243 KB demo DSL document embedded by `include_str!` (the gate counts embedded files toward bytes). Fixes, all behaviour-neutral:
  1. Asset crate: `semio-s-artifact-gis-gismap-demo-asset` (script `F/r6-tools/gismap-asset.ts`, workspace member, owner row, router entry) owns the document; its three consumers read it through the crate (lib 2 files / 243,819 bytes, under 262,144).
  2. Root file modules of the artifact root (`map_composition`, `inference_service`, `artifact_declaration`) moved into the `any` nest, with root `pub use ...::*` re-exports and the flat shims kept as they were; `gis_map_descriptor_value` moved with `map_composition`.
  3. The editor cycle was broken by repointing `crate::` paths to module paths, explicit imports instead of `use super::*`, `pub(crate)` widened to `pub`, and `editor/constants_and_io` made a `pub mod`.
  4. New bridge module `schema/drawing-bridge` (`GIS_POINT_STYLE`, `GIS_LINE_STYLE`) so the schema no longer reaches into the editor for two constants.
  5. Tests that used `unit_tests::context` stay with their users (they are not hoistable to the facade); the ownership test has a local `Gis2dApp` alias; `schema_document_contract`, `map_window_ownership` and `demo_example_contract` are hoisted to the facade root.
  6. Facade test closure over bytes: the pbf `include_bytes!` fixtures were converted to runtime reads with the new `include_bytes!` support (see section 4) using `F/r6-tools/fix-bytes.ts`.
- Commands: `--dry-run --explain` exit 0, `--verify --oracle` clean apart from macro-defined items, apply with `--backup=F/🗑️generated/r6-backup-gismap`, `--check-applied --base=dir:/Users/ueli/.cache/r6/gm-pre` exit 0 (extras 0 for lib and test). Why `dir:`: HEAD of the artifact does not parse (earlier file-split slice in the index) and its macro-defined items (`app_commands!`, `app_labels!`, `list_delta!`) are reported differently in base and combined, so the HEAD base gives thousands of noise lines. The pre-carve snapshot `/Users/ueli/.cache/r6/gm-pre` is the same working tree before my apply, so the differential measures exactly my carve.
- Locks: gismap, gis bridge, gisterrain, dev/variants gis services, dev/fixture-sweep and hub (script `F/r6-tools/lock-sync.ts`; backups in `F/🗑️generated/r6-backup-gismap-locks`).
- Census after (lib / test): config 11/37,600/796 / 18/57,652/1,157; gis2d 15/152,812/2,401 / 20/242,290/3,849; windows 11/23,194/458; gismap 7/64,626/1,110 / 9/75,959/1,260; demo-asset 2/243,819/5; inferences 10/60,037/834 / 27/230,328/3,200; tests 16/109,068/1,899 / 29/233,385/3,663; schema 27/51,676/279 / 28/56,029/352; snapshot 26/117,763/1,477 / 28/120,735/1,541; mutations 27/70,796/1,093 / 29/87,468/1,335; mut-any 28/52,309/893; create-position 26/38,875/754; mcp 1/5,772/77 / 4/24,715/341; tile-coordinates 3/30,555/600 / 4/32,099/630; terminology-tests 1/548/17; facade 16/112,280/1,467 / 28/236,237/3,218. Census script reports `crates 16 over-or-error 0`.

### 2.5 gis gisterrain (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain`, DONE static)

- Baseline problem: a 7-slice, 46-file cycle between the schema root (21 embedded files), the snapshot plus imported-map group (18 files) and the inferences group (7 files). Fixes, all behaviour-neutral:
  1. New module `schema/terrain-descriptor` holds `TerrainProjectOrigin`, `TerrainPositionData`, `TerrainDescriptorJson` and `GIS_3D_TERRAIN_TILE_URL_TEMPLATE`; 21 files were repointed to `schema::terrain_descriptor::..` and `schema::imported_map::..`; `pub(crate)`/`pub(super)` widened to `pub` in 13 files.
  2. `parse_descriptor`, `map_positions` and `imported_positions` moved from `schema/inferences` to `io/text/snapshot` (next to `fixture_descriptor`, their only caller; they only need the snapshot and the imported map), which removes the inferences-to-snapshot-to-inferences cycle. The junk `use ...::schema::*;` glob of the text codec became `use ...::schema::imported_map;`.
  3. The pre-existing broken call `<GisTerrainSnapshot as ArtifactSqliteSnapshotHost>::original_owners::with_decoding(|owner| decode_sqlite_snapshot_native(..))` in the sqlite snapshot test (not valid Rust, present in HEAD) now reads `original_owners::with_decoding(|owner| <GisTerrainSnapshot as ArtifactSqliteSnapshotHost>::decode_sqlite_snapshot_native(..))`; the same test imported the snapshot through the private import of its former host, now through `schema::snapshot::GisTerrainSnapshot`.
  4. My own codemod had dropped the `use ` keyword of four import lines (sqlite and text snapshot codecs, viewer and editor terrain windows); restored, and grepped repo-wide for lines starting with `crate::` and ending in `;`.
- Commands: `--dry-run --explain` exit 0 (6 crates), `--verify --oracle` (one noise line, the macro-defined `Gis3dCommand`), apply with `--backup=F/🗑️generated/r6-backup-gisterrain` (64 files, 6 crates, 6 owner rows), `--check-applied --base=dir:/Users/ueli/.cache/r6/gt-pre` exit 0 (native 890 / 890 / 0, test 1443 / 1460 / 0, admission, shared, sharedForeign empty). The base is a `dir:` snapshot for the same reason as gismap (HEAD does not parse).
- Locks synced (`F/r6-tools/lock-sync.ts`): gisterrain, gis bridge, dev/fixture-sweep, hub (backup `F/🗑️generated/r6-backup-gisterrain-locks`).
- Census after (lib / test): editor 5/55,382/940 / 9/112,476/1,849; operations 18/24,905/464 / 22/56,058/895; inferences 26/96,842/1,153 / 29/102,617/1,263; schema 27/38,865/379 / 28/40,071/400; relocated-engine-tests 15/65,235/1,213 / 26/97,879/1,779; terrain-descriptor 22/26,815/518 / 23/27,336/533; facade 1/18,267/317 / 5/51,547/553. Census script reports `crates 7 over-or-error 0`.

## 3. cfg(test) rule (cross-crate `#[cfg(test)]` items)

- Engine: `job::test_step_context` moved to the test-support crate `-oracle` (normal crate, dev-dependency of the job tests and the facade).
- wfc-bitmap: the tests used `crate::tests::fixtures::base()` across crates; replaced by the normal item `BitmapSnapshot::reference_scene()`.
- gismap: tests using `unit_tests::context` stay together with the code they exercise (no crate boundary between them); the three root tests were hoisted to the facade root.
- No `cfg(any(test, ...))` hooks and no new `testkit` features were needed.
- Hand-off from CFG-TEST-CROSS-CRATE: `F/ctc-tools/ctc-scan.ts` found 18 hits for wfc-3d, wfc-grid2d, wfc-grid3d `mutations` and gismap `gis2d` `dispatch`; every one is a same-name collision (a local item with the same name as a `cfg(test)` item elsewhere), no code change. For the gisterrain crates the scan reports `hits 0` (log `F/🗑️generated/r6-gt-ctc.log`).

## 4. Generator bugs fixed in the library (all with tests; 78 tests pass in `L/🧪️tests/📦️crate-tree/🟦️.ts`)

- `patchCargoLock` collapsed multi-version packages and dropped the blank line after the header; it now keeps unrelated blocks byte-identical and inserts new blocks at the sorted position.
- `fixturesToRuntime`: const and static fixtures at any depth become lazily read statics (`std::sync::LazyLock`), reference initialisers use `Box::leak`, helpers are reached with `super::` per enclosing inline module, and `include_bytes!` is supported (`runtime_fixture_bytes`).
- `dropBaseUnresolvedNames` (sandbox): unresolved-name diagnostics that only differ in wording between base and combined (macro-defined items such as `Gis2dCommand`, `GisMapFeaturesDelta`) no longer count as extras in `--check-applied`.
- `prunedUse` (module-tree): a pruned group import is flattened below its longest shared leading path, so `use super::super::{A, B}` stays `use super::super::{A, B}` instead of `use super::{super::A, super::B}`. The old form made `absolutizeSuperPaths` rewrite every inner `super` and produced `use crate::m::mutations::{crate::m::mutations::A, ..}` (E0433) when a detached test carried the host's private group import (found in gisterrain `set-camera`).
- Known remaining limitation: a hoisted or detached test that names an item through a private import of its former host (gisterrain sqlite snapshot test) is not rewritten by the generator; the layout differential reports it (E0603) and the import has to be fixed in the source.

## 5. Repo-wide checks (final)

- Owners scan (`F/owners-tools/scan.ts`): 2974 packages / 3090 owners; none of the 30 `missing` rows is under wfc or gis.
- Taxonomy (`F/r6-tools/r6-taxcheck.ts`) for wfc and gis: exit 0, only the pre-existing plugin-level sample (`missing Cargo manifest 📦️packages/🦀️rust/Cargo.toml` of the plugin directory itself, as before my changes).
- `lib-wiring-budget` test: 2 pass, 1 fail; the six breaches belong to sequence, bim model, note, flow brep mesh, puzzle 2d and 5d (other slices); no wfc or gis root is listed.
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` parse.

## 6. Final census (before to after, files/bytes/lines; gate 32 / 262,144 / 4,000)

| package | before lib | before test | after: largest lib | after: largest test | verdict |
| --- | --- | --- | --- | --- | --- |
| wfc engine | 43 / 398,750 / 9,356 | 91 / 641,968 / 15,039 | job 4/111,873/2,544 | job 7/144,447/3,168 | all 7 inside |
| wfc-2d | 138 / 564,027 / 8,624 | 274 / 981,031 / 12,885 | schema 28/174,582/836 | mutations 29/195,593/3,022 | all 12 crates inside |
| wfc-grid2d | 110 / 509,000 / 8,348 | 217 / 809,663 / 12,121 | inferences 23/107,667/2,020 | tests-cuts 20/184,980/3,523 | all inside |
| wfc-bitmap | 119 / 528,898 / 8,725 | 224 / 845,835 / 13,314 | bitmap 18/204,526/3,545 | bitmap 22/216,634/3,763 | all inside |
| wfc-3d | 147 / 539,917 / 8,867 | 288 / 913,556 / 13,921 | wfc3d 14/152,366/2,776 | wfc3d 19/205,234/3,641 | all inside |
| wfc-grid3d | 110 / 499,586 / 7,841 | 254 / 903,026 / 11,885 | editor 11/167,155/2,985 | editor 17/205,821/3,637 | all inside |
| gisterrain | 100 / 294,291 / 4,431 | 149 / 468,774 / 6,397 | inferences 26/96,842/1,153 | inferences 29/102,617/1,263 | 7 crates inside |
| gismap | 203 / 1,058,280 / 12,314 | 388 / 2,058,629 / 20,027 | demo-asset 2/243,819/5 (bytes); gis2d 15/152,812/2,401 | gis2d 20/242,290/3,849 | 16 crates inside |

Census scripts: wfc plugin `crates 62 over-or-error 0`, gismap `crates 16 over-or-error 0`, gisterrain `crates 7 over-or-error 0` (`F/🗑️generated/r6-*-final*`). Evidence is static only (no compile, as agreed): `--check-applied` layout differentials (exit 0, 0 extras for every package), census, blind-spot script, owners scan.
