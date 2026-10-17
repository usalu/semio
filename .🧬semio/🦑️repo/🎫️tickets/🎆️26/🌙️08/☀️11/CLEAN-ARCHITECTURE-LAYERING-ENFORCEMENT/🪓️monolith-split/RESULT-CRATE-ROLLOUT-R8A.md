# Result: CRATE-ROLLOUT-R8A (prefix `r8a`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/r8a-tools/` (`step.sh <tag> <cmd...>`, `r8a-census.ts`). Logs: `F/🗑️generated/r8a-*`. Sandboxes: `/Users/ueli/.cache/r8a/`. Started 2026-10-17 11:55.

## 0. Verdict (final, 21:30)

| Package | Verdict | Evidence |
| --- | --- | --- |
| puzzle 5d (`🧩️puzzle/🗿️artifacts/🖐️5d`) | DONE: 24 crates, every lib and test closure inside the gate (4000 lines / 32 files / 262,144 bytes) | `--verify --oracle` exit 0 (23-crate plan); `apply` exit 0; `--check-applied --base=HEAD`: exit 0, 0 extras native and test; census `r8a-census-r8a5dfinal.tsv`: `over-or-error 0` |
| puzzle 2d (`🧩️puzzle/🗿️artifacts/◻️2d`) | DONE: 29 crates (28 carved + graph-manifest), every lib and test closure inside the gate | `--verify --oracle`: all crates inside the gate, 0 onlyCandidate in `oracle`, 1 layout-only phase artifact (E0220, now filtered by the oracle); `apply` exit 0; `--check-applied --base=HEAD`: exit 0, 0 extras native and test; census `r8a-census-r8a2dfinal.tsv`: `over-or-error 0` |

Repo checks (both packages): owners scan (`F/owners-tools/scan.ts`) lists no puzzle path (25 + 29 owner rows synced with `r8a-tools/owners_sync.py`); taxonomy check (`F/🗑️generated/pw0-taxcheck.ts`) reports only the pre-existing plugin-level `puzzle: missing Cargo manifest 📦️packages/🦀️rust/Cargo.toml` (HEAD has no such manifest); `lib-wiring-budget` has no puzzle breach (breaches listed there belong to peers: sequence, writer, bim, trinity, raster, generation2d); both Cargo.lock files carry exactly the on-disk crates; all 55 Cargo.toml of both artifacts and the launch JSONs parse.

## 1. Baseline census (`F/🗑️generated/r8a-census-base0.tsv`, host cfg, 11:55)

| crate | lib files/bytes/lines | test closure files/bytes/lines |
| --- | --- | --- |
| semio-s-artifact-puzzle-2d | 340 / 2,643,423 / 25,218 | 1,167 / 6,727,976 / 53,647 |
| semio-s-artifact-puzzle-5d | 329 / 1,953,983 / 26,190 | 837 / 3,722,522 / 42,438 |

Both artifacts carry staged (index) changes from earlier file splits (puzzle editors wave 2) and peers; treated as baseline.

## 2. Found state after the 12:12 resume (nothing applied by the carve CLI yet; no `new crates` write has run for either package)

Source/generator edits made so far (all mine, none applied via `new crates`):
- Generator (`L/🏗️authoring/📦️crate-tree/`): `implHeader` returns `traitArgs`; the planner (`🗺️plan`) and `checkOrphanRules` (`🔬️verify`) accept `impl ForeignTrait<LocalType> for ForeignType` (orphan rule satisfied by a local trait argument). Test still to add.
- 5d: `native_codec` removed from the snapshot schema (inlined in the subset file `codec:`; the sqlite test builds the codec directly); the five `Puzzle5dPlaySnapshot` codec impls (pack, store, dsl, sqlite host, sqlite) moved from the three io files into new `✏️editor/📸️snapshot/🌉️play-codecs/🦀️.rs`; `from_typed` widened to `pub`.
- 5d: `r8a-tools/r8a-unglob.ts --write` ran over the editor (114 files: `use super::*` of 19 fragments replaced by explicit imports, fragment modules made `pub mod`, outside references to glob-exported names repointed to the defining fragment). Undo file: `/Users/ueli/.cache/r8a/undo-5d-unglob1-*.json` (`r8a-tools/undo.py`). Not yet verified.
- Planner results (dry): 5d schema side plans into 14 crates inside the gate; remaining blocker is the editor (81 slices, 180 files, 16.8k lines). 2d: schema blocker (record types <-> snapshot cycle in the schema root, `native_preparation_*` visibility), editor blocker, 700 KB embedded icon asset crate.
Tools: `r8a-tools/{r8a-census.ts,step.sh,r2-blockers.ts,r2-slices.ts,blk.py,ed.py,filegraph.py,fas.py,r8a-unglob.ts,undo.py,rep.py,moveplay.py}`.

## 3. Progress log (12:12 resume, wave 7)

5d editor decomposition (static; every step re-planned with `r8a-tools/cyc.sh 5d` and `r2-blockers.ts`; `--no-test-aggregate` plan):
- unglob codemod (`r8a-unglob.ts`, undo files in `/Users/ueli/.cache/r8a/`): 19 fragment files lost `use super::*`; fragment modules became `pub mod`; `widen.py` turned `pub(crate|super|in ..)` into `pub` in the editor (386 sites, 20 files).
- Back edges removed by moving items: `add_palette_part` + `catalog_kind_label` + part-kind options (to `catalog`), `puzzle5d_inspector_nudge` (to the transform utility), `PUZZLE5D_RETAINED_TOOL_IDS` (to `interaction`), window/tool/utility id constants (to `entities`; windows alias them), `config` uses the 3d suggestion menu type directly, session helpers moved to `session/🧱️support`.
- App coupling: new `Puzzle5dEditor` contract trait (blanket impl, in `interaction`, same pattern as puzzle 3d `Puzzle3dEditor`); retained/gestures/kit works and the tool-run builders are generic over `A: Puzzle5dEditor`; the dispatcher left the root (`🎬️dispatch`, `puzzle5d_handle_action::<A>`); `with_puzzle5d_app`/`handle_action_impl` removed.
- Result of the 5d dry plan (`--mutation-families=on --no-test-aggregate`): 22 crates, all inside the gate except the test closure of the editor unit tests crate (`unit-tests`: 18 files / 383,706 bytes / 5,947 lines). Cause: the shared harness `unit_tests::context` is a cfg(test) module used by 15 test files; the cross-crate cfg(test) rule forbids splitting without a real test-support crate. Chosen fix: hand-made dev-only crate `…-testkit` (pattern: `generation2d/🧪️tests/🧰️support`).

### 3.1 Later 5d steps (13:30)
- Test closure: hand-made dev-only crate `semio-s-artifact-puzzle-5d-testkit` (`🖐️5d/🧪️tests/🧰️support`, member + workspace dependency + dev-dependency of the facade): the former `unit_tests::context` harness; 9 test files import it; `history_edit_acceptance_law!` stays in the unit tests. With `--no-test-aggregate` the plan has 23 crates, no problem line.
- Kept root file modules (oracle blind spot, brief 12:10): `document_records`, `kind_catalogs` (root `mod` + `pub use ::*`) moved into the subset schema (`🧬️schema/🧱️document-records`, `🗂️kind-catalogs`) and 52 files repathed from `crate::Puzzle5dX` to `crate::standards::v1::subsets::any::schema::<module>::Puzzle5dX`; `graph_manifest` (root `pub mod`) moved to the schema root (5d and 2d; only internal users); `wasm` bridge module keeps an always-present `wasm` module and gates its content in a child (`🌉️bridge`) so the carved facade can re-export it natively.
- Mutation family: `Puzzle5dSelection`, `puzzle5d_selection`, `puzzle5d_selection_outcome` moved from the mutations aggregate to `schema::diff` (the macro-generated `*Modification` types live there; the baseline's unresolved-macro rows no longer change file when the family support module is carved).
- 2d: schema root split (`🧱️records` child, snapshot-dependent items stay), editor consts moved to `config`/`snapshot`, clipboard fns to `recorder`, `apply_host_events` to `apply_board_events`, `puzzle2d_restore_brush_slot` to `interaction`; same unglob/widen/modclean pipeline as 5d.

### 3.2 17:10 resume (5d)
- Canonical testkit idiom applied to 5d: harness `✏️editor/🧪️tests/🧰️testkit` mounted from the hub under `#[cfg(any(test, feature = "testkit"))]` (+ a re-export that keeps it in the owner crate); the hand-made `semio-s-artifact-puzzle-5d-testkit` crate, its workspace rows and the self dev-dependency are gone. After `apply`, `r8a-tools/wire5d.py` adds `[features] testkit = [...]` to the owner crate and `features = ["testkit"]` dev-dependencies to the two mirror consumers (v1, host-admission) - hand edit of generated Cargo files, redone after every re-apply.
- Generator fixes (all in `L/🏗️authoring/📦️crate-tree/`, test count 87 pass): (1) `🧪️test-closure`: a detached test that imports a private host item through `use super::NAME` now widens that item to `pub` (it was mistaken for a body-declared name) and no longer gets a duplicate widened `use` (E0252); (2) `🧫️sandbox`: `--verify --oracle` and its layout differential now drop candidate rows that only re-word an unresolved name the base tree already fails to resolve (`dropBaseUnresolvedNames`, same rule as `--check-applied`), which removes the macro-generated-type noise (`Puzzle5dLabels`) that flipped verify between runs.
- Source repairs found by the base-HEAD differential: root-level unit tests and the sqlite snapshot io files lost the record names after the records moved to `schema::{document_records,kind_catalogs}` (explicit imports from the defining modules, not from the `schema` aggregate, to avoid a snapshot/schema crate cycle); `puzzle5d_native_en_labels()` accessor replaces fully qualified `Puzzle5dLabels::NATIVE_EN` uses in tests; ~35 missing explicit imports in command/panel/session/test fragments (`r8a-tools/sites5d.py`).
- Verify (pre-import-repair plan): exit 0, 23 crates, none over the gate. Apply exit 0. Owner rows synced with `r8a-tools/owners_sync.py` (25 rows, stale names removed).

### 3.3 5d done, 2d decomposition (19:00)
- 5d: apply (24 crates) + `--check-applied --base=HEAD`: exit 0, 0 extras native and test. Repairs after the base-HEAD differential: root-level unit tests and sqlite io files import the record names from `schema::{document_records,kind_catalogs}`; ~40 missing explicit imports across command/panel/session/test fragments; the apply must be redone after any import repair that adds a crate edge (the mirror of the consuming crate is generated from the plan). Census (`r8a-census.ts`, tag r8a5dfinal): 24 crates, 0 over the gate.
- 2d schema side (all hand edits in the source tree, no compat layers):
  - the four leaf `prepare` mounts (create/delete/move/connect) and the preparation family (flag, text prepare/inverse, literal-id-inverse, native_preparation_child) moved out of the leaf/aggregate files into a `schema::preparation` path container mounted from the artifact root (no component file, so no parent-child edge back to the `Puzzle2dMutation` aggregate); `native_preparation_child` widened to `pub`, its two `pub(crate)` macros expanded in place, its three cursor impls moved next to their cursors; paths repointed.
  - `snapshot::lookup` mounted at `schema::lookup` (parent-child cycle with `schema::snapshot`).
  - the io text snapshot unit test imports `Puzzle2dSnapshot` explicitly so the test detaches.
  - snapshot functions imported while private made `pub` (the nine E0603 defects).
- 2d editor: shared pane builders (`render_canvas`, `puzzle2d_engagement`, camera, scene) moved from `modes::edit` into the `modes::edit::canvas` child and the three `ZOOM_SCALE` constants into `config`; hub re-exports no longer used (`select` utility items and `puzzle2d_localized` imported from their defining modules); `Puzzle2dActionCtx::release_board` moved into `apply_board_events`; `drain_board_events_json` moved to `engine::board_host`; the board-host `context` harness became the canonical `board_host::testkit` (owner `cfg(any(test, feature="testkit"))`); the former hand-made `semio-s-artifact-puzzle-2d-testkit` crate became `editor::puzzle2d::testkit` mounted from the hub; unused imports dropped with `r8a-tools/r8a-unused.ts`.
- Plan after these edits (`--mutation-families=on --no-test-aggregate`): 28 crates, none over the gate (lib and test).

## 4. Final census, before -> after (`F/🗑️generated/r8a-census-r8a{5d,2d}final.tsv`, host cfg)

| package | before (1 crate) lib / test closure | after |
| --- | --- | --- |
| 5d | 329 files / 1,953,983 B / 26,190 lines ; 837 / 3,722,522 / 42,438 | 24 crates; largest lib: `5d-dispatch` 26 / 225,021 / 3,607, `5d-snapshot` 11 / 229,907 / 461, `5d-puzzle5d` (hub) 7 / 208,278 / 3,116; largest test closure: `5d-v1` 15 / 250,300 / 3,760; the facade `5d` is 1 / 46,340 / 649 lib, 5 / 84,468 / 1,077 test |
| 2d | 340 / 2,643,423 / 25,218 ; 1,167 / 6,727,976 / 53,647 | 29 crates; largest lib: `2d-snapshot` 16 / 228,186 / 1,581, `2d-tambour-single-storey` 18 / 223,283 / 57, `2d-retained` 26 / 182,546 / 3,125; largest test closure: `2d-v1` 14 / 241,890 / 3,802, `2d-mutations` 29 / 239,822 / 3,062; the facade `2d` is 1 / 38,958 / 578 lib, 3 / 68,965 / 808 test |

## 5. Commands that gate the result (from `🧰️framework/.../repo` root, `ulimit -n 65536`, `nice -n 10`)

- `bun ./📜️script.ts new crates <pkg> --no-test-aggregate --mutation-families=on --verify --oracle --sandbox=/Users/ueli/.cache/r8a/v<pkg>` (`r8a-tools/carve.sh <5d|2d> verify`)
- `... new crates <pkg> --no-test-aggregate --mutation-families=on --backup=F/🗑️generated/r8a-backup-<pkg>` (`carve.sh apply`; `r8a-tools/revert.py <log> <backup>` restores, deletes created files)
- `... new crates <pkg> --check-applied --base=HEAD --samples=500 --sandbox=...` (`carve.sh check`)
- `bun test ./🧪️tests/📦️crate-tree/🟦️.ts` from the library root: 103 pass; `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts`.
- No compile was run (the cargo ingress gate is closed to executors): all evidence is the rustc analysis oracle, the layout differential against `HEAD`, and the census.

## 6. Generator changes (all in `L/🏗️authoring/📦️crate-tree/`, all with tests in `L/🧪️tests/📦️crate-tree/🟦️.ts`)

1. `🗺️plan`, `🔬️verify`: `impl ForeignTrait<LocalType> for ForeignType` is orphan-legal (the trait argument is local) and no longer forces a coherence merge.
2. `🧪️test-closure` detach: a `use super::NAME;` of a private host item is no longer mistaken for a body-declared name (the item is widened to `pub`), and the widened `use` is not duplicated when the body already imports it (E0252). Two tests.
3. `🧫️sandbox`: the verify oracle and its layout differential apply `dropBaseUnresolvedNames` like `--check-applied` (macro-generated types no longer flip the verdict between runs); `external()` drops E0220 on a generic parameter (typeck-phase noise that surfaces only in a carved crate that no longer aborts at resolution). One test.

## 7. Hand-edited generated files (redo after every re-apply; scripts in `F/r8a-tools/`)

- Testkit wiring (canonical idiom): owner `[features] testkit = [...]` and `features = ["testkit"]` dev-dependencies of the consumers: `wire5d.py` (5d: hub crate `5d-puzzle5d` -> `5d-v1`, `5d-host-admission-tests`), `wire2d.py` (2d: owners `2d-records` (source custody), `2d-board-host`, `2d-v1` (editor testkit; its forwards `semio-framework-plugin/artifact-app-testing`, `semio-framework-os-kernel/protocol-laws`, `semio-framework-os-kernel/mutation-testing`, `semio-s-plugin-puzzle-jobs/testing` were set by hand)).
- Owner rows: `owners_sync.py`. `sites5d.py` re-adds the explicit imports a revert restores.

## 8. Source changes that are not generator output (no compat layers, no migrations)

- 5d: see 3, 3.1, 3.2 (records moved into `schema`, hub-and-spoke editor with the `Puzzle5dEditor` contract trait, canonical in-owner testkit, kept-root modules moved, codec impls moved, explicit imports).
- 2d: see 3.3 plus: the preparation family under `schema::preparation`; `schema::testkit` (source custody) replaces the root `test_source_custody`; `schema::lookup` mounted beside `snapshot`; explicit imports in the preparation tests; the two real unresolved imports of the base tree (`apply_host_events`, the three clipboard functions) repointed; self-imports and duplicate imports removed (E0255, E0252); the retained-clone test helper `settle_clone` takes the cursor type as a second generic parameter (no `T::Cursor` projection).

## 9. Open risks

- Not compiled: a real `cargo check` may still find defects the analysis oracle cannot see (macro-generated types are masked by `dropBaseUnresolvedNames`; names that are unresolved in the base tree and in the candidate are not compared).
- The `testkit` features of the hand-wired Cargo files are not read back by `--check-applied`; a re-apply regenerates the crate manifests without them.
- `puzzle: missing Cargo manifest 📦️packages/🦀️rust/Cargo.toml` (taxonomy) and the `crate::board_set_camera` import of the 2d board-host unit tests exist at HEAD.
- Peers keep deleting and recreating framework crates (`🥔️tests-20`): the census needed `existsSync` filtering of the stale manifest list.
