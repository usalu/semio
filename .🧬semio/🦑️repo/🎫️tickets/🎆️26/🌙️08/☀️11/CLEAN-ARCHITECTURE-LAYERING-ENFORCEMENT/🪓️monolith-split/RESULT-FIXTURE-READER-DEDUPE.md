# Result FIXTURE-READER-DEDUPE (prefix `frd`)

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no modifying git command. `F` = this folder. Tools: `F/frd-tools/`. Logs: `F/🗑️generated/frd-*`.

## 0. Verdict

**DONE (static), UNCOMPILED, for every crate outside the running slices (FRD and FRD-2).** One framework test-support crate, `semio-framework-test-fixtures` (`🧰️framework/🔨️modules/🧪️test/🧲️fixture-reader`), owns the run-time fixture read; about 1,115 reader definitions are deleted, 5,475 call sites in 1,677 files use the macros, 515 manifests carry the dependency row, and the crate-tree generator emits the shared API (101 generator tests pass). FRD-2: R5B (pdf) migrated (section 11.1); of the 274 reader literals that named non-existent paths, 269 are repaired (re-anchored, moved under `🧪️tests`, or re-pointed at the moved TypeScript module), 4 belong to r8a (puzzle) and 1 is an intentional negative path (section 11). Still deferred because their slices are editing: ohp (82 files, 88 helper definitions), bd10 (53 files, 23), r8a (85 files, helpers gone, 351 `env!` lines), r9a (25 files, 35 lines); `frd-tools/sweep.sh --apply` finishes them (section 6). Nothing compiled: no `COMPILE ADMITTED` line in `📓️FLEET.md`, lock files not refreshed (section 9).

## 1. Inventory (before any edit)

Scan of 32,408 tracked and 315 untracked `.rs` files (ticket folders excluded): 1,952 files mention `CARGO_MANIFEST_DIR` or `runtime_fixture`.

| Shape | Definitions | Reference lines |
| --- | ---: | ---: |
| `fn runtime_fixture_text(relative) -> &'static str` (identical copy per file) | 1,177 | 4,108 calls (+49 `super::`) |
| `fn runtime_fixture_bytes(relative) -> &'static [u8]` (identical copy per file) | 78 | 199 calls (+4 `super::`) |
| `pub(crate) mod runtime_fixture` (oracle bins; `bytes`/`text` with `../../..` base) | 9 | 114 `crate::runtime_fixture::` calls |
| `fn read_static(path_from_manifest)` (panic text "test input") | 26 | |
| `fn fixture_text(relative)` single-line `.expect(relative)` | 24 | |
| `fn fixture`, `fn read_node`, `fn path`, `fn product_directory`, `fn family_any_dir`, `fn subset_root`, `fn repo_root`, `fn test_repo_root` ... | about 45 | |
| Inline `std::fs::read*(concat!(env!("CARGO_MANIFEST_DIR"), "/rel"))` | | 1,072 |
| Inline `Path::new(env!("CARGO_MANIFEST_DIR")).join(..)` and similar | | 296 |
| `include_str!/include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), ..))` (compile-time embeds, not run-time reads) | | 47 |
| `install_default_schema_leaf_reader(.., env!("CARGO_MANIFEST_DIR"))` (registry install, not a fixture read) | | 35 |
| `nx.json` ancestor walks | | 20 |

Generator: `fixturesToRuntime` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts` emits the per-file helper (`FIXTURE_HELPER`, `FIXTURE_BYTES_HELPER`) and `runtime_fixture_text(...)` / `super::runtime_fixture_text(...)` calls; its tests live in `…/📚️library/🧪️tests/📦️crate-tree/🟦️.ts`.

## 2. The shared API (crate `semio-framework-test-fixtures`)

Home: `🧰️framework/🔨️modules/🧪️test/🧲️fixture-reader` (package `📦️packages/🦀️rust`), beside the other `role = "test"` framework crates (`⚖️typed-input-law`, `🛬️original-snapshot-owners`). Not the renderer `test-support` crate: that one is product-specific (module-tree readers) and a plugin crate must not depend on an os renderer crate. No dependencies, no external libraries. Registered like its siblings: `🧰️framework/Cargo.toml` member (byte order), `📋️project.json`, `📋️build-owners.json` row, six launch routes in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (orders 900.33083/84, 922.33083/84, 903.33083/84).

Macros (all expand `env!("CARGO_MANIFEST_DIR")` at the call site, so they resolve against the crate under test):

| Macro | Result | Replaces |
| --- | --- | --- |
| `fixture_text!(rel)` | `&'static str`, leaked, panics with path + inputs | `runtime_fixture_text`, `read_static`, `fixture_text` ... |
| `fixture_bytes!(rel)` | `&'static [u8]`, leaked | `runtime_fixture_bytes` |
| `fixture_path!(rel)` | `PathBuf` below the manifest dir | `Path::new(env!("CARGO_MANIFEST_DIR")).join(..)`, `concat!(env!(..), "/..")` passed to `fs::*` |
| `manifest_dir!()` | `PathBuf` | `PathBuf::from(env!("CARGO_MANIFEST_DIR"))` |
| `repo_root!()` | `PathBuf` of the ancestor holding `nx.json` | the `nx.json` walks |
| `repo_text!(rel)`, `repo_bytes!(rel)` | read once per process, relative to the repo root | the cached `runtime_fixture::text` of the composition-laws bins |

The functions behind them (`path_in`, `text_in`, `bytes_in`, `repo_root_in`, `repo_text_in`, `repo_bytes_in`) take the manifest dir explicitly and are `#[track_caller]`, so a refused read points at the test line.

Tests: `🧪️tests/🦀️.rs` (10 Rust laws over the vectors `🧪️tests/🧫️fixtures/📊️.csv` plus `🔤️.txt`/`💾️.bin`), `🧪️tests/🟦️.ts` (5 bun tests replaying the same CSV with `node:fs` and `node:crypto`: bytes, sha256, UTF-8 verdicts, refusals). TS run: 5 pass. Rust: `rustfmt --check` clean; analysis-only `rustc +nightly-2026-07-20 --emit=metadata` of the lib and `--test` (scratch out dir, deleted) 0 errors 0 warnings; not executed (no `COMPILE ADMITTED`).

## 3. Generator (`fixturesToRuntime` and neighbours)

`…/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts`: the per-file `FIXTURE_HELPER`/`FIXTURE_BYTES_HELPER` texts are gone; the rewrite now emits `fixture_text!("@fixtures/…")` / `fixture_bytes!(..)` (nested modules keep the `super::` prefix) and `importFixtureMacros` puts `use semio_framework_test_fixtures::{..};` into rustfmt order at the top of the file. `📝️render/🟦️.ts`: `rebaseManifestDirPaths` also rebases the literal of `fixture_path!/fixture_text!/fixture_bytes!` when a file moves to another crate dir (placeholders untouched); `🟦️.ts`: `carveWrites` adds the dev-dependency row (`insertDevDependencyRow`, `withFixtureReaderDevDependency`, sorted) to every carved crate manifest whose rewritten files use the reader, and rebases before resolving placeholders. Tests (`🧪️tests/📦️crate-tree/🟦️.ts`): 4 new tests (imports, bytes+text emission, macro-path rebase, sorted dev-dependency) and an end-to-end carve test; the file runs 98 tests green.

## 4. Migration (applied to the free crates)

Tools (all in `F/frd-tools/`; the originals of every edited source file are kept as `orig/<sha1 of path>.rs`, of every edited manifest as `orig-manifests/<sha1>.toml`; journals `F/🗑️generated/frd-mig*.json`, `frd-wire-*.json`):

| Tool | Does |
| --- | --- |
| `scan.py`, `lines.py`, `refs.py`, `stats.py`, `register.py` (crate registration) | inventory of helper shapes, call forms, residual `CARGO_MANIFEST_DIR` lines, final counts |
| `migrate.ts` | tier 1: removes the canonical per-file `runtime_fixture_text` / `runtime_fixture_bytes` copies (also the indented ones inside an inline module), renames the calls to `fixture_text!` / `fixture_bytes!` (keeping `super::`), imports the macros once (rustfmt order, grows an existing import) |
| `migrate2.ts` | tier 2 (`read_static`, the one-line `fixture_text`, `read_node` helpers), tier 1b (the `mod runtime_fixture` of the oracle bins and the composition-laws bins, base resolved from the owning crate), tier 3 (`Path::new(env!(..)).join(..)` and `fs::read*(concat!(env!(..), "/..."))` to `fixture_path!`, `PathBuf::from(env!(..))` to `manifest_dir!()`, the `nx.json` ancestor walk to `repo_root!()`), with a guard that leaves production code on `env!` |
| `normalize.ts` | merges duplicate imports of the reader, drops `std::path` imports that became unused |
| `owners.ts [--only=<manifest fragments>]` | closure ownership: which crate target compiles which file, under lib / lib+all features / test cfg (canonical `readRustSourceClosureV1`) |
| `gating.ts`, `wire.ts` | decide dev-dependency, normal dependency (test-support crates) or optional dependency behind the gating feature, write the rows in sorted place |
| `verify-paths.ts`, `parsecheck.sh` | every literal path exists below the owning manifest dir; rustfmt parse gate (`+nightly --config skip_children=true`) |
| `sweep.sh [--apply]` | the whole pipeline again (rescan, migrate, normalize, owners, gating, wire); idempotent, `excluded.json` lists the slices that are skipped |

Result on the free crates (everything outside the exclusion list of section 6):

- 1,391 source files rewritten; 1,009 reader definitions removed: 940 `runtime_fixture_text`, 8 `runtime_fixture_bytes`, 8 `mod runtime_fixture`, 26 `read_static`, 24 one-line `fixture_text`, 3 `read_node`.
- 5,184 call sites now use the reader: `fixture_text!` 3,416, `fixture_path!` 845, `fixture_bytes!` 52, `manifest_dir!` 26, `repo_text!` 47, `repo_root!` 7 (counted in the rewritten files only; peers adopted the crate in about 170 more files, 1,559 files import it repo-wide).
- `CARGO_MANIFEST_DIR` lines in those files: 1,901 before, 46 after (section 7 lists what remains and why).
- 494 manifests got a dependency row (all parse): 467 dev-dependencies, 19 normal dependencies (crates that are test support themselves: `*-tests`, `*-companion`, `*-testing`, `*-mut-vectors`, `semio-hub-testing`), 8 optional dependencies wired into the gating feature (`testkit` in 6 crates, `integration-fixtures` in component-codecs-laws, `artifact-app-testing` in plugin instance) plus the dev-dependency for their own tests.
- Four production files that the first pass had converted (`run/bootstrap`, `mcp/plugin-paths-and-descriptor`, `wgpu/native-asset-stream`, `norm/en1996/bins/regenerate_example_assets.rs`) were restored from `orig/`: production code keeps `env!`; only test support depends on the test crate. The guard in `migrate2.ts` (test-like path or `cfg(test)` gate) prevents it.

## 5. Verification

| Check | Command or tool | Result |
| --- | --- | --- |
| rustfmt parse of every rewritten file | `frd-tools/parsecheck.sh` over the journal list | 1,393 files, 0 parse errors; two real defects found on the way (use line before an inner doc, use line before a `//#region` header) and fixed in `importFixtureMacros` + tests |
| New crate | `rustfmt --check`; `rustc +nightly-2026-07-20 --emit=metadata` lib and `--test` into a scratch dir (deleted) | clean, 0 warnings; not executed (no `COMPILE ADMITTED`) |
| New crate TS vectors | `bun test` in `🧪️tests/🟦️.ts` | 5 pass |
| Generator | `bun test` `…/📚️library/🧪️tests/📦️crate-tree/🟦️.ts` | 101 pass, 0 fail (new: imports, bytes+text emission, macro-path rebase, sorted dev-dependency, end-to-end carve, package exists) |
| Taxonomy gate | `bun ./📜️script.ts verify taxonomy report --scopes-from <node>` | clean=true for `🧰️framework/🔨️modules/🧪️test/🧲️fixture-reader` (a first run found a TS template literal the reference scanner cannot read, fixed) |
| Dependency ratchet | `bun ./📜️script.ts verify dependencies` | exit 0 |
| Test names | per-file `#[test]` name multiset, original snapshot against now | 4,815 test attributes in 1,391 files, 0 files differ |
| Fixture bytes | no fixture file was written; every literal path handed to the reader resolved with `verify-paths.ts` | 3,743 literals; 272 do not exist below the owning manifest, 268 of them with the identical literal in the original file (pre-existing, listed in section 8), 4 in the peer-owned puzzle files |
| Census (`frd-tools/frd-census.ts`, same canonical reader) | before `frdbase-*`, after `frdafter-*` | 2,998 common crates: 2,830 equal, 154 better, 14 worse; the oversized set is unchanged (3 lib, 1 test-only), compilable lib 2,992 to 3,008 (23 new crates, mostly peer carves); the 9 touched crates that grew grew by one import line (+0.001 of a limit, largest 3d-brep-boolean 0.987 to 0.988); 5 untouched crates drifted from peer work |
| Census again after FRD-2 (`frdafter2-*`) | same tool, 3,032 crates | 2,781 equal, 184 better, 33 worse (14 of them touched, worst +0.004; the larger ones are peer work: stdio-semio document-examples, puzzle-2d), oversized lib 3 to 2 (a peer fix), compilable lib 3,023 |
| ctc-scan | `ctc-tools/ctc-scan.ts` then `ctc-report.ts … bad` | 39 groups, 0 mention the reader or a migrated file (12 new groups are peer work: infinite-world-model registries, composition-emission, time-travel, sequence, shooting) |
| Scoped resolution oracle | `frd-tools/pos` (copy of the POS typeck oracle, family with the reader crate real, `--no-base`, lib and test cfg) + `frd-tools/pos-eval.py` (counts errors whose primary span starts at a reader macro, names the crate, or reports a missing `fixture_*`/`repo_*` macro) | 7 families, about 95 migrated crates plus the reader: `frd-a` (5), `frd-p1`, `frd-p2` (plugins, 14 each), `frd-s1` (stdio, 14), `frd-h1` (hub/dev/oracles, 13), `frd-f1`, `frd-f2` (framework, 14 each): 0 reader-related errors in lib and test cfg (the test cfg of `frd-f1` and of the 119-crate framework family abort before typeck on peer `#[path]`/`include!` quirks; `frd-s2` was cut short and is not counted). Negative control (`--mutate fixture_text! -> fixture_txt!`): caught, `cannot find macro fixture_txt` |
| Scoped oracle after FRD-2 | `frd-fix` (lib) and `frd-fix2` (test) over mp4, ifc, pdf, shooting, architect program, en1995 | 0 reader-related errors (2,448 baseline errors in test cfg, 575 in lib); the curation crate was left out of the test run because a peer `include!` aborts its sandbox before typeck |

## 6. Deferred sites (running slices) and the later pass

`frd-tools/excluded.json` lists the path fragments of the slices that were mid-edit at 17:46; `frd-tools/deferred.tsv` lists every file with a remaining helper or ad-hoc `CARGO_MANIFEST_DIR` read (owner, file, helper definitions, lines). TEST-CLOSURE-T2 (draw, layout, block) reported DONE at 18:5x and was lifted from the list: its 135 helper copies and 10 files of ad-hoc joins are migrated and wired.

| Owner slice | Area | Files | Helper definitions left | Ad-hoc `CARGO_MANIFEST_DIR` lines left |
| --- | --- | ---: | ---: | ---: |
| r5b | stdio pdf | migrated in FRD-2 (section 11.1) | 0 | 0 |
| ohp | mathematical (equation), trinity, vcs, dag, playbook, reasoning, demonstrator | 82 | 88 | 94 |
| bd10 | raster, process3d | 53 | 23 | 124 |
| r8a | puzzle 2d/5d | 85 | 0 | 351 |
| r9a | bim | 25 | 0 | 35 |
| hdr | files it touched or created | 0 left | | |
| ohf, c4p46, c4p50 | no Rust fixture sites in their files (ohf is analysis, c4p* are TypeScript splits) | | | |

Later pass for each owner once its RESULT says DONE: delete its fragment from `excluded.json`, then `frd-tools/sweep.sh` (plan) and `frd-tools/sweep.sh --apply`. The sweep rescans, migrates, merges imports, recomputes ownership (`owners.ts`, about 25 minutes; `--only=<fragments>` limits it to the slice), classifies the dependency kind and writes the rows. r8a already adopted the reader in 89 puzzle files and bd10 in 10 raster files (their helper definitions are gone or going); its remaining 351 lines are `concat!(env!(..))` reads that the sweep converts.

Fleet tools that parse the generator output: `hdr-tools/fixt.ts` already accepts `fixture_text!(`/`fixture_bytes!(`/`fixture_path!(` (its regex was widened after the generator change); `bd10-tools/runtime-fixtures.ts` and `bd10-tools/hoist-tests.ts` call `fixturesToRuntime` and now receive macro calls plus a `use semio_framework_test_fixtures::…;` line, but nothing adds the crate to the manifest of a crate that was not wired yet: the next `sweep.sh --apply` (or `wire.ts`) does.

## 7. What stays on `env!("CARGO_MANIFEST_DIR")` in the free crates (109 lines, 72 files) and why

- `install_default_schema_leaf_reader(.., env!("CARGO_MANIFEST_DIR"))` (30): schema registry install in production host code, not a fixture read.
- `include_str!/include_bytes!(concat!(env!(..), ".."))` (47): compile-time embeds, a separate defect class (they count against the compiler source gate); the generator already turns the literal-path form into run-time reads, the `concat!` form needs a hand edit per site.
- `const NAME: &str = concat!(env!(..), "/..")` (12): compile-time path constants whose users take a `&str`; a macro cannot be `const`.
- Production path logic (16 sites, 4 restored files): hub bootstrap, run bootstrap, plugin-paths-and-descriptor, native-asset-stream, `derive`/`native-build` build code, `SEMIO_REPO_ROOT` overrides.
- Semantic variants of the repo-root walk (`bun.lock`, `.🧬semio`, `Cargo.toml` plus `🧰️framework`, `📜️artifact-definition.json`, five-parent hops): not `nx.json`, so `repo_root!()` would change meaning.
- Bare `Path::new(env!(..))` / `.ancestors().nth(N)` (21): the result is a borrowed `&Path`, a `PathBuf` macro would not borrow-check without a rewrite of the use.

## 8. Findings

- Missing fixture paths: 268 of the 3,743 literals handed to the reader did not resolve below any owning manifest, with the identical literal in the original file (pre-existing). Section 11 classifies and fixes them (FRD-2). The `u{1f9ea}` path in curation was real: a past rewrite had stripped the backslashes of `\u{..}` escapes from a Rust string, so the path named literal text; it is repaired.
- Three sandbox quirks of the POS oracle: it substitutes `env!("CARGO_MANIFEST_DIR")` textually (so the macros use plain `env!`, not `::core::env!`), it cannot follow `#[path = "\u{..}"]` escapes (workflow schema laws aborts the framework family at test cfg), and the whole-framework family does not finish in 10 minutes under load 33 (use chunks of 14 crates, `frd-tools/families/`).
- Bun `bun test` works in this checkout (the 17:4x note about a segfault does not apply to the files run here).

## 9. Compile queue (when `COMPILE ADMITTED` appears)

Not run: the launcher does not admit (no `COMPILE ADMITTED` line in `📓️FLEET.md`). Lock files are not refreshed: the new path dependency edges are missing in the root `Cargo.lock`, `🧰️framework/Cargo.lock` and the per-artifact `Cargo.lock` files; refresh with the launcher (`update --workspace --offline`) before the first check. Commands:

1. `cd /Users/ueli/Documents/semio && bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-test-fixtures --tests --message-format=short`
2. `… -- test -p semio-framework-test-fixtures --lib` (10 tests)
3. Per area, one batched call, e.g. `… -- check -p semio-s-artifact-stdio-svg --tests` and `… -p semio-framework-quiz-schema --features testkit --tests`, then the 494 manifests of `F/🗑️generated/frd-wire-apply*.json`.

## 10. Incident noted

My own tier-3 pass rewrote `PathBuf::from(env!("CARGO_MANIFEST_DIR"))` inside the reader crate's test to `manifest_dir!()` (a tautology plus a self import). Found by the final rustc check of the crate, restored by hand, and `excluded.json` now lists the reader crate itself (`owner: self`). No other file outside the migration lists was touched; every edited file has its original in `frd-tools/orig/`.

## 11. FRD-2: R5B migration, missing fixture paths

### 11.1 R5B (stdio pdf) migrated

R5B reported DONE (`📓️FLEET.md` 19:0x), so the `r5b` fragment left `excluded.json`. Run as in section 4 (tiers 1, 2, 1b, 3, normalize, `owners.ts --only=📖️pdf`, gating, `wire.ts --apply`): 119 pdf source files rewritten (105 reader definitions removed, 263 calls renamed, the pdf oracle `mod runtime_fixture` and its 20 users re-pointed, 10 ad-hoc joins), 20 manifests got the dev-dependency row. All rewritten files parse (0 errors); the first pass of tier 2/3 reported its own fresh edits as hot because the guard only read `frd-mig*` journals, now it reads every `frd-(mig|pdf|sweep|fix)*` journal. Repo totals after the pass: 1,677 files import the reader, 5,475 call sites, 515 manifests carry the row, helper copies left only in ohp (88) and bd10 (23).

### 11.2 Classification of the 274 paths that did not exist

`frd-tools/classify-missing.py` (nearest-ancestor search of the literal tail, then of the tail with `🧪️tests/` inserted before `🧫️fixtures`), usage read from the call context (`F/🗑️generated/frd-missing-class.json`):

| Use of the path | Count | Verdict |
| --- | ---: | --- |
| `fs::read_to_string` / `fs::read` argument, mostly `static BEFORE/AFTER/MUTATION` of the lowpoly mutation tests (188 rows, 41 files) | 205 | real run-time reads: each would panic with its `.expect` message |
| `fixture_text!` / `fixture_bytes!` argument | 27 | real run-time reads |
| `let root = fixture_path!(..)` used by `read_dir`, `canonicalize`, `join` | 25 | real: directory walks and `assert!(exists)` guards (en1990 oracle, cad interaction assets, en1996 assets, dashboard journeys, trusted-catalog linked codec registries, kinds-catalog) |
| regenerate / bless / refresh writers behind an env switch (`regenerate_committed_*`, `debug_emit_*`, `refresh_lowpoly_*`, `EN1995_REGEN_PACKS`, `SEMIO_ENERGY_EPJSON_REGENERATE`) | 9 | real but inert unless the switch is set; they pointed at a directory that never existed, so a bless would have written a stray tree |
| intentional negative path | 1 | dead by design: print `chart-mutations` reads `fixture_path!("🕳️nowhere")` to prove the "unreadable document" error; left as is |
| peer-owned (r8a puzzle fill-run.json, capsule-dream x3) | 4 | recorded, not touched |
| other helper functions returning a directory (`family_any_dir`, `catalogue_folder`, `fixtures_root`) | 3 | real (feed 'exists' asserts and `join`) |

No path was a dead reference except the intentional one. Cause in every case: the literal had been written for the crate depth before the carve (R2/R3/R5/R7) or before `🧫️fixtures` moved under `🧪️tests/`, and the verbatim conversion kept it.

### 11.3 Repairs

- 249 literals in 78 files re-anchored mechanically (`fixpaths.py`: the same tail, found at the nearest ancestor of the owning manifest directory; `.expect("..")` messages carrying the literal updated too). Covers lowpoly (188 rows), semio `✉️base` apply-* inversion tests (the 20 failing `fixture_text!` reads from section 8), png owned-native-samples, cad, norm en1990/en1996/en1995 oracles and assets, trusted-catalog registries, dashboard journeys, dsl derive authority.
- 14 rows in 12 files where the fixtures directory had moved under tests (`…/🧫️fixtures/…` to `…/🧪️tests/🧫️fixtures/…`): lowpoly regenerate writers, wfc grid3d emitters, ifc2x3 wellness-center fixture bytes (2 files), epJSON bestest root, teaching proctor catalog, ui limits conformance, store sync replication wire, document facets conformance.
- sqlite exchange tests whose TypeScript module moved: ifc 4.4 and ifc 2x3 now load `…/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts` (exports `ifcSnapshotFromSqliteDatabase` / `ifc2x3SnapshotFromSqliteDatabase`, checked); mp4 loads the artifact facade `🎥️mp4/🟦️.ts` (re-exports the sqlite and schema functions the script calls) and reads its neutral sample from the existing `…/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json` instead of the vanished `🧫️fixtures/🪶️sqlite/🔣️.json` (the `snapshot_root` variable is gone). The sample parses as an `Mp4Snapshot` (keys schema, ftyp, movie, tracks); nothing was authored.
- shooting `🔬️independent` oracle fixtures directory and architect program mutation fixtures now point below `🧪️tests/🧫️fixtures`.
- hub norm `compliance-gate`: `scan_evaluate_gaming` scanned `📕️norm/🗿️artifacts` inside the hub composition, a directory that does not exist, so the source scan ran over zero files and passed vacuously (same literal at HEAD). It now scans `✏️s/🔌️plugins/📕️norm/🗿️artifacts`; the four gaming tokens have no hit in the norm artifacts' evaluate files.
- curation `retained_table_lanes_restore_complete_neutral_owner`: the `u{1f9ea}/u{fe0f}…` literal is restored to the real characters (`🧪️tests/🧫️fixtures/🚚️retained-table/🔣️.json`); this was the only file in the repo with that corruption (searched tracked and untracked `.rs`).
- pdf `owned_fixture_publication_reports_canonical_logical_carriers`: the converted test used an undefined `protocol` variable and a file-relative literal. Both reads now use the manifest-relative path of the PDF 1.7 compiled protocol (`fixture_bytes!` and `fixture_path!`). Both reads name the same file now, so the original compile-time embed comparison is a tautology until the protocol is exposed as a crate constant; recorded for R5B.
- No fixture was authored: every missing fixture file exists, only the paths were stale.

### 11.4 Result

`verify-paths.ts` over 4,614 reader literals: 5 unresolved, 4 in r8a's puzzle files (`🧠️precompute/🪣️fill` fill-run.json, `capsule-dream` assets) and the intentional `🕳️nowhere`. 1,678 files that import the reader parse (0 errors), every unqualified macro use has its import. The ohp, bd10 and r9a sites that still use `env!` were not path-verified because they are not migrated yet; `sweep.sh` plus `verify-paths.ts` covers them. Scoped oracle on the touched crates (`frd-fix`): see the table in section 5.
