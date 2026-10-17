# RESULT: BASELINE-DEFECTS-9 (`bd9`)

Executor: Sonnet, 2026-10-17 from about 13:35. Foreground, no sub-agent, no modifying git command, no ticket tool. Continues `RESULT-BASELINE-DEFECTS-8.md` and `COMPILE-DEFECT-LEDGER.md`. Tools in `bd9-tools/`, logs in `🗑️generated/bd9-*`. No real cargo compile (`COMPILE ADMITTED` is not in `📓️FLEET.md`).

## 0. Verdict

PARTLY DONE. Items 1, 2, 3, 5 (flow brep mesh), the O-07 part of item 6, and the coordinator additions (a), (b), (c) are done (statically, no cargo compile ran; `COMPILE ADMITTED` is not in `📓️FLEET.md`). Open: item 4 (raster crates do not exist, section 7), O-03 and O-05 (need the compiler loop and the plugin owner), O-11 (8 owners throw, call sites outside the fixture), and the lib-wiring roots of running slices.

## 1. Item status (updated 15:20)

| # | item | status |
|---|---|---|
| 1 | brep-boolean `[DEBUG]` removal and repo-wide sweep | DONE for the crate (section 2); sweep DONE with documented keeps; peers keep adding new `[DEBUG]` prints, re-run `bd9-tools/strip_debug.py` at the end of the wave |
| 2 | `new crates --backup=<abs>` | DONE with 2 tests (section 3) |
| 3 | leaf-family crate names | DONE: 15 crates renamed (3 R2 + 12 found by scan, incl. R4 graph/presentation/svg), generator fixed with tests (section 4) |
| 4 | raster stub wiring | analysed, OPEN (section 7) |
| 5 | lib-wiring-budget | flow brep mesh root 1,211 -> 24 lines (DONE); stale pin removed; sequence, writer, bim, puzzle 2d support belong to running slices (section 6) |
| 6 | O-03 | OPEN, decision kept (section 8) |
| 6 | O-05 | partly: composition owner test and receiving test re-pointed (O-07 readers); the 21 close-budget tests still open |
| 6 | O-07 | DONE: window fixtures existed under another name (path drift, no authoring needed), 3 deleted-file readers re-pointed (section 8) |
| 6 | O-11 | see section 9 |
| a | all first-leaf crate names | DONE (item 3) |
| b | canonical testkit idiom | DONE for the 40 self-dev-dependency crates (section 5) |
| c | stdio semio graph off `semio_framework_graph` | DONE statically (section 5b) |

## 2. [DEBUG] removal

* `semio-framework-3d-brep-boolean`: `debug_dump_face` (+ the call in `classify_face_against_solid`), the `KB1_DUMP` block, `debug_face_tessellation` and the two `eprintln!("[DEBUG] case/matrix ...")` prints of the differential matrix and the `started` timer were removed: `🦀️.rs` 2,497 -> 2,481 lines, `🧪️tests/🔬️differential/🦀️.rs` 193 -> 160 lines. The `KB1_ONLY/KB1_THREADS/KB1_LIMIT` knobs of the matrix stay (real parameters).
* Repo-wide: tool `bd9-tools/strip_debug.py` (string/comment-aware via `lexmask.py`; statement-level removal only of `println!/eprintln!/print!/eprint!/dbg!` with a `"[DEBUG]` first argument and `console.*`/`process.std*.write` with a `[DEBUG]` first argument; control-flow positions become `{}`; Rust `#[cfg]` attributes in front of a removed statement go with it; 4 now-unused single-line `let` bindings were dropped). First pass: 6,380 statements in 2,386 files. A second tool (`restore_protocol.py`, 3-way merge against the index blob) then put back the statements that are protocol, not temporary logs: statements inside string literals (embedded child programs) and statements whose message prefix is consumed elsewhere (`startsWith`, `toContain`, ...): 70 statements in 64 files, 35 string-embedded ones were never touched in the second run. Remaining `[DEBUG]` hits outside tickets: see the table in section 2.1.
* Verification of the stripped set (coordinator request of 14:50): every stripped file was parsed. `bd9-tools/stripped-files.txt` lists all 2,428 files; TS/TSX with Bun's transpiler: 1,362 parsed, 0 failed (`bd9-tools/parse-ts.log`); Rust with `rustc -Zunpretty=normal` (`ss7-tools/parse-one.sh`): 1,066 parsed, 1 failure (`🧰️framework/…/🏪️store/🧩️composition/🌳️graph/🤝️group/🧪️tests/🔬️isolated/🦀️.rs`: a template with `@MODULE@` placeholders, byte-identical to the index version, pre-existing; `bd9-tools/parse-rs.log`). No `if`/`else` head is fused: a removed control-flow body becomes `{}` (never an empty statement).
* Data mentions are untouched: the tool only edits print calls. Files that mention `[DEBUG]` as data (the acceptance census law `🧪️test/🎯️acceptance/📋️orchestration`, fixtures such as the rust-source-direction JSON, consumers using `startsWith("[DEBUG]")`) were not edited. Where a consumer exists the producer was restored (`General command completed` in the process script, `Cargo scanner timing`, `interactive-ready`, ...).
* Two doc-commented temporary tests (`debug_print_normalized_example(s)` in the dev composition tests, `[DEBUG] Temporary` in their doc) lost their only effect (the print) to the sweep and were deleted whole.
* Re-scan at the end: 134 files outside tickets still mention `[DEBUG]`: embedded child programs and fixtures (kept), consumers of protocol lines, `expect(...)`/`panic!` messages that start with the tag (message text, not logging, left), and the acceptance census law. 11 new prints written by peers during the wave were stripped by a last pass; run `python3 bd9-tools/strip_debug.py --apply < list` again at the end of the wave.


## 3. Generator `--backup`

`resolveCarveBackupDir` in `🏗️authoring/📦️crate-tree/🟦️.ts`; `🎮️command/🟦️.ts` uses it before any write. An absolute path is used as given, a relative one resolves against the repo root, and a directory inside the repo outside `.🧬semio/🦑️repo/🎫️tickets` is refused ("inside the source tree"). Tests in `🧪️tests/📦️crate-tree/🟦️.ts`: absolute backup outside the repo (nothing created below the repo root, backup holds the old file), relative ticket-tree path accepted, source-tree paths (`f3-backup`, absolute inside, `.`, other `.🧬semio` folders) refused. Suite: 82 pass.

## 4. Leaf-family crate names (items 3 and a)

Root cause in `🗺️plan/🟦️.ts`: the `mut-<family>` label required every slice of a group to be a mutation slice (a leaf-family crate also hosts one support slice, so it fell back to the first leaf's name), and the duplicate-label rule renamed repeated `mut-*` labels after the biggest slice's last path segment, which is again a leaf. Now `mutationCrateLabel` accepts a majority of family slices, `isMutationCrateLabel` exempts `mut-*` from the rename (duplicates get `-2`), the family directory is derived from the family slices only. Tests: `🧪️tests/🧬️mutation-leaf-generics/🟦️.ts` (12 pass), crate-tree 82 pass, lift-kids pass.
Renames, one pass each with `bd9-tools/r2_rename.py` (`renames-wave2.json` lists wave 2): R2 model `-insert-element`, drawing `-create-node`/`-change-stroke-width` (to `-mut-family`, `-mut-family`, `-mut-family-2`); wave 2 (found by scanning every owner row whose lib root mounts at least two `mod component;` leaf blocks and is named after its leaf folder): animate-presentation `resize-source-frame`, gis-gismap `create-position`, mathematical-equation `create-node`, shooting `create-asset`, stdio-semio `apply-mesh`, stdio-semio-graph `create-edge`, stdio-semio-presentation `insert-layout`, stdio-svg `insert-tiny-element`, wfc-3d `create-slot`, wfc-bitmap `resize-input`/`resize-output`, wfc-grid3d `change-tile-weight`. Crate directories keep their place (the crate manifest lives in the first leaf folder, as the generator placed it before); only names changed. A repo-wide alternation grep over tracked files finds none of the 15 old names (hyphen or underscore form). Not checked by a compiler.

## 5. Canonical testkit idiom (b) and graph migration (c)

(b) `bd9-tools/tk-closure.ts` (canonical source reader, host cfg plus `feature="testkit"`) lists each crate's closure; `tk_convert.py` rewrote the 40 crates with a self dev-dependency: R1 xml/deflate/zip, R4 svg/animation/graph/cad/presentation/semio root crates, process3d host, puzzle-3d editor-session, the 22 infinite world/board crates. `ctc-scan` (2,999 crates) afterwards: 13 bad groups, none in these crates except four weak method hits in the infinite world model (`registries::push/iter/len/take_one`, class `indirect-method-weak`, hand-off). cad and presentation schema crates needed `features=["testkit"]` on their snapshot dependency (fixed). Tests of these crates now need `--features testkit`: the launcher routes that run `cargo test -p <crate>` must pass it or use `--all-features` (hand-off to compile admission).
(c) see ledger bd9-06 and the appended block in `RESULT-CRATE-ROLLOUT-R7B.md` for trinity.

## 6. lib-wiring-budget (item 5)

Failing before: sequence (+1), bim model 440, flow brep mesh 1,212, puzzle 5d/2d test support (449/392), one stale pin. Done: flow brep mesh root split into `🧰️helpers` (198 lines), `🧮️operator-core` (902) and `📇️registration` (161) with `bd9-tools/split_flow_mesh.py`; the root keeps the docstring, the `#[macro_export]` macro and the three `#[path]` mods with `pub use`. Checked: line multiset lossless apart from the 12 macro lines kept in the root, no duplicated item names across children, `-Zunpretty=normal` parses all four files, `rustc --emit=metadata` shows only the unresolved-extern-crate errors (68, same cause as the original's 18, no local name error; weak oracle because the external crates cannot be linked), taxonomy `crate-module-children` ok. Each child imports exactly the names it uses (flattened original `use` tree) plus an explicit `use super::{…}` for sibling items, never `use super::*`, so no glob ambiguity with `flow_extension_sdk::mesh::*`. Stale pins removed (`📦️object`, `🧰️kit`). Current failures are all owned by running slices: sequence (r7c), bim (r9a), flow/flow root (a peer added 5 mount lines), writer (+2, r7c). Puzzle 5d/2d support left the failure list while r8a works.

## 7. Raster (item 4): OPEN, finding

The raster artifact root `🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs` was cut from 1,835 to 203 lines by commit 793f63533f2 and mounts nothing but two example tests. The source under it was already re-pointed to a carved crate set that does not exist in Cargo: 697 references to `semio_s_artifact_raster_raster_model`, 131 `_diff`, 111 `_mutations`, 106 `_mutations_layers`, 89 `_mutations_pixels`, 47 `_io`, 20 `_examples`, 19 `_host`, 2 `_editor`, 1 `_viewer` (and the mutation bridge links `_model` and `_mutations`). The component files are the intended crate roots (`🧬️schema/🦀️.rs` already says `use crate::snapshot::RasterSnapshot;` and declares `pub mod snapshot;`), but no `📦️packages/🦀️rust/Cargo.toml` exists for any of them (only the facade manifest), nor lock blocks, build-owner rows or router entries. `new crates` cannot be re-run: the root has no mounts left to carve and the sources already carry the post-carve names. Wiring needs ten manifests with dependency lists derived from the sources, lock blocks and owner rows, which cannot be validated without the compiler. Plan for the compile admission: restore a mount root from `git show b4bf1a54b37:'✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs'` (kept at `🗑️generated/bd9-raster-old-root.rs`) only as a layout reference, author the ten manifests with `[lib] path` at the component files, add workspace members, then `cargo check --keep-going` them bottom-up (model, diff, mutations, mutations-layers/pixels, io, examples, host, editor, viewer, facade).

## 8. O-07, O-05 and O-03

* O-07 DONE (ledger bd9-08). The window fixtures existed; the tests read the wrong folder. Equation oracle OK, architect `ajv=valid json-patch=valid four-owner-isolation=valid reload=valid report-identity=valid`.
* O-05: classification of the 67 failing close-budget assertions in the ledger row (17 re-pointable literals, 43 absent, 7 without literal). The mounted private group test is green after re-anchoring on `owner.close_step(&mut stores.store, …)`, `fn close_private_child_group_step<` and the mount by `#[path = "📦️owner/🦀️.rs"]`. `advance_openings` and `advance_original_document_archive_close` have no caller; whether that is intended is for the plugin owner.
* O-03 unchanged. Mapping for the rewrite (current API read from `🌱️value/♻️retirement`): `Process3dOwnedRetirement::{snapshot,mutation,owner}` becomes `retirement::admit_owned_retirement(value, grant)` (returns `(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress)`, or `(ValueError, value)` back) or the host's `process3d_snapshot_value_factory()`/`process3d_mutation_value_factory()` (`OwnedValueRetirementFactory::<T>::retire_owned`, `pub(crate)`, so the suite must be a lib unit test); `close_step(items, bytes)` with `SnapshotRetirementStep::{Pending{..},Complete}` becomes `close_step(RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>` with quotes `next_demand(maximum_body_bytes)` (`copy_bytes`, `capacity_bytes`, `release_bytes`, `depth`) and `terminal_is_empty()`; `Process3dRetirementStack`/`process3d_retire_erased_step` become `retirement::deferred(value)` / `RetirementCursor::close_step(grant) -> RetirementStep::{Progress,Complete,Failure}`. Six tests are affected (lines 258, 435, 454, 556, 583, 619 of `🧪️tests/🔬️retained-laws/🦀️.rs`).

## 9. O-11 vitest gate

`bd9-tools/vitest-owners.ts` recomputes every owner exactly like the gate (46 owners): 16 structurally valid owners were re-pinned (`--write`). Still failing: 8 owners throw "Explicit Vitest policy required" (framework kernel, replication, machine, react targets ×2, 3d, schema; hub root), `cad` (12 selecting nothing, wrong root), framework `os` (12 patterns select nothing), `puzzle` (name drift), `flow`, `client/mcp`, `client/sqlite`, `coordinator` (1 pattern each), and the 22 `runVitest` call sites select configs the fixture does not list. Decision unchanged from bd8: the config owners fix their configs, then the 13 existing call-site configs are registered (46 to 59 owners).

## 10. Verification run

| check | result |
|---|---|
| `bun test` crate-tree, lift-kids, mutation-leaf-generics | 107 pass, 0 fail (3 files together; crate-tree alone 82) |
| parse of 2,428 stripped files | TS 1,362/1,362, Rust 1,065/1,066 (pre-existing template) |
| `bun test` lib-wiring-budget | 2 pass, 1 fail; the failures are the running slices listed in section 6 |
| equation window oracle, architect window ownership, manifest action-args retirement, completion receiving, mounted group owner | OK, OK, 2/2, 1/1, 1/1 |
| `ctc-scan` + report (2,999 crates) | 13 bad groups, none caused by the testkit conversion |
| rustc analysis of the flow mesh split | only unresolved-extern errors (weak oracle) |
| any cargo check/test | not run (no admission) |

Compile queue for admission: `cargo check -p <crate> --tests --features testkit` for the 40 crates of section 5, `-p` the 15 renamed crates, the graph subset (`semio-s-artifact-stdio-semio-graph{,-schema,-inferences,-examples}`), `semio-s-plugin-flow-brep-mesh`, `semio-framework-3d-brep-boolean` (tests too), and the touched Cargo.lock files (stdio semio, wfc ×5, gis, equation, shooting, animate, svg, hub, dev gis services, fixture-sweep).

## 11. Files

Tools (`bd9-tools/`): `strip_debug.py`, `lexmask.py`, `restore_protocol.py`, `r2_rename.py`, `renames-wave2.json`, `tk-closure.ts`, `tk_convert.py`, `graph_migrate.py`, `split_flow_mesh.py`, `vitest-owners.ts`, `parse_ts.ts`, `stripped-files.txt`, `parse-rs.log`, `parse-ts.log`, `orig/flow-mesh-root.rs`. Logs and data: `🗑️generated/bd9-*`.
Source edits: the 2,428 files of `stripped-files.txt`; brep-boolean root and differential test; crate-tree `🟦️.ts`, `🗺️plan/🟦️.ts`, `🎮️command/🟦️.ts` and two test files; 15-crate rename (133 files: manifests, locks, `📋️build-owners.json`, Rust roots and crate docs); 40 manifests and 24 Rust files of the testkit conversion; stdio semio graph subset (15 sources, 4 roots, 5 manifests, lock); flow brep mesh root plus three new children; five TS test re-points (equation, architect, manifest action-args, completion receiving, mounted group owner); vitest ownership fixture; lib-wiring ratchet fixture; two deleted temporary tests.
