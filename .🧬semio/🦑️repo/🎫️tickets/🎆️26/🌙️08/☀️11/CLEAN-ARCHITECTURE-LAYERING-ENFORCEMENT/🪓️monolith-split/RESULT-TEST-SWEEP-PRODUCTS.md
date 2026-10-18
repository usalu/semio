# Result TEST-SWEEP-PRODUCTS (prefix `tsx`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `FW` = `🧰️framework`, `M` = `FW/🔨️modules`, `PR` = `FW/🛍️products`. Tools in `F/tsx-tools/`: `vitest.sh` (one vitest config, JSON report, per-test set), `modvt.sh` (all module vitest configs), `buntest.sh` and `modbun.sh` (bun test with junit over the 306 module `bun:test` files listed in `bunfiles.txt`), `vtfail.py`, `bunfail.py`, `bunfail2.py` (failure digests), `regen-entity.ts` (entity-catalog regeneration), `treeread.py`/`unionread.py` (two codemod experiments that were reverted, see section 6). Raw logs: `F/🗑️generated/tsx-vt-*`, `tsx-vtset-*`, `tsx-bun-*`, `tsx-bunset-*`, `tsx-bunfail-*` (older `tsx-*` files in `🗑️generated` belong to an earlier slice and are untouched). Machine load during the runs: 43 to 145 (see section 5: timing laws).

## 0. Verdict

**Failing tests before to after: 139 to 73 (plus one unloadable file before and after).** By lane: vitest 26 failing tests to 0 (quiz-react 8 to 0, schema 9 to 0, ui-react 6 to 0 and 1 unloadable file unchanged, styling 2 to 0, 2d 1 to 0); print catalog-coverage 2 to 0; module bun tests 98 unique to 60; print `📥️material` 12 and products fixture-placement 1 unchanged. No test newly failing, nothing deleted (one superseded assertion removed, see row 19). All 73 remaining failures are environment-gated (launcher envelope, native cargo or oracle binary, missing third-party package, artifact-dir layout), architectural debt owned by another fleet (framework code importing repo products, fixture-placement migration, Rust source-text laws pinned to code the carve moved into sibling crates) or a load-bound timing law; each is attributed in section 4.

Not done (and why): the 19 `🧵️job` source-text laws and the 5 `🖱️ui` Rust-text laws need a per-assertion relocation to the sibling crates the carve created; a blanket union reader would weaken them (tried and reverted, section 6). The fixture-placement law (7 module tests, 1 product test) needs the SDK fixture-facade migration (about 70 files in modules, os and pets). The launcher (`bun repo ...`) does not start on this machine (`System image plist required` from `nativeImageInformationV1`), so every test that needs a script envelope cannot run.

## 1. Baseline (before any edit)

| Lane | Command | Result |
|---|---|---|
| quiz core vitest | `tsx-tools/vitest.sh quiz-core <PR>/❓️quiz 🧪️tests/🎚️config/🟦️.ts` | 451/451 |
| quiz-react vitest | same, react config | 921 passed, 8 failed |
| pets react / core | same | 175/175, 1,133/1,133 |
| presentation react / core | same | 140/140, 56/56 |
| server | same | 18/18 |
| module vitest configs (12) via `modvt.sh` | | async 27/27, 2d 152/153, kernel 91/91 and label 4/4, actor 104/104, geometry 26/26, replication 21/21, machine 3/3, react-renderer 10/10, 3d 17/17, framework tests 306/306, styling 80/82, ui-react 1,161/1,167 plus 1 unloadable file, schema 6/15 (8 files failed) |
| print material bun (repo root) | `buntest.sh` | 43 tests, 12 failed |
| print pdf and artifacts bun | | 10/10, 6/6 |
| print misc bun (gallery measurement, sqlite snapshot, typed inference, products fixture-placement) | | 20 tests, 1 failed |
| print catalog-coverage scenarios | `bun` over the exported `scenario*` | 7/9 |
| print inference harness (d3 oracle) | `c4ts-tools/infrun.sh` | not re-run, no print inference file touched (c4ts baseline 631/631 quick, 659/659 long stands) |
| module bun tests (306 files, 2,061 tests) via `modbun.sh` | | 98 unique tests failed |
| `.storybook` tests | | `scope-resolution` and `owned-ui-react-lint` run inside the ui-react vitest config (green); `project-annotations` and `browser-runner` are a Storybook setup file and a Playwright config (need a built Storybook and browsers, not runnable here) |

## 2. After

| Lane | Result |
|---|---|
| quiz core / react | 451/451, **929/929** (was 921/929) |
| pets react / core, presentation react / core, server | unchanged green: 175, 1,133, 140, 56, 18 |
| module vitest | async 27, 2d **153/153**, kernel 91 and 4, actor 104, geometry 26, replication 21, machine 3, react-renderer 10, 3d 17, framework 306, styling **82/82**, ui-react **1,166/1,167** (the 1 left is the unloadable `ShellScope` file, missing `color`), schema **29/29** |
| print | catalog-coverage **9/9**; material 31/43 (12 envelope-gated, unchanged); pdf, artifacts unchanged green; products fixture-placement still 1 failing |
| module bun | 2,061 tests, **60 failed** (98 before; last full run showed 61, the 2d fixture-list fix in row 7 removed the 61st) |

## 3. Fixes (owner, cause, evidence)

Every fix keeps the law and changes only the stale reference, the missing declaration or the missing warm-up. No assertion was weakened and no test deleted. Each row was re-run alone after the edit.

| # | Owner file(s) | Cause (class) | Fix |
|---|---|---|---|
| 1 | `PR/❓️quiz/🧪️tests/🎪️pets-on-stage/🟦️.tsx` | race: the first client test dynamically imports `@semio-tech/pets-react` inside a 1 s `waitFor`; the import outlives the wait and the next tests start while it is still pending (4 to 5 cascade failures; the baseline failure was only the first) | `beforeAll` awaits the same import with a 120 s hook budget (not a longer `waitFor`) |
| 2 | `M/🧬️schema/🧪️tests/✅️draft07-oracle/🟦️.ts`, `📤️schema-export-entries/🟦️.ts` | stale path: fixture joined twice (`…/X.json/X.json`, ENOTDIR) | single path segment |
| 3 | `M/🧬️schema/🧪️tests/🎚️config/🟦️.ts` | config bug: `include: ["🧪️tests/**/🟦️.ts"]` collected bun-only tests, the config itself and a helper (5 files failed to load) | explicit include of the three vitest files |
| 4 | `M/🧬️schema/🧪️tests/📤️schema-export-entries/🟦️.ts` | the repo-wide schema `$id` base is `https://json.schemas.assets.semio-tech.com/…` (557 files); the test pinned `https://semio.tech/schema/…` | expectation updated |
| 5 | `M/🧬️schema/🤖️generated/🏷️entity-kinds/🟦️.ts` and `🦀️.rs` | stale generated file: generator prints `bun repo run …`, files still said `bun nx run …` | regenerated with the generator's own plan (`tsx-tools/regen-entity.ts`, dry run listed exactly these two files, 2 bytes each) |
| 6 | `M/🖱️ui/🎨️styling/🧪️tests/🧪️theme-resolve/🟦️.ts`, `🧪️playgroundflowwasmdevstubplugin/🟦️.ts`, `🧩️suite/🟦️.ts` | stale fixture paths after fixtures moved under the owners' `🧪️tests/🧫️fixtures` (theme document, tile-proxy corpus, fonts corpus) | paths re-pointed |
| 7 | `M/◻️2d/🧮️compute/🧪️tests/📍️ownership/🧭️direction` fixture | stale list of files copied into the deletion sandbox (a deleted schema file and moved fixture); a peer is rewriting the same test concurrently | list now `[test, fixture, compute 🦀️.rs, unit 🦀️.rs, compute Cargo.toml]`; 2d bun 81/81 |
| 8 | `M/🎭️actor/🧪️tests/📣️public-facets/{🟦️.ts,🔣️.json,🧬️.json}` | product evolved: the root facet re-exports `type Lane` from `🤖️generated/🎭️actor` (schema-first) | fixture row gains `generated`/`generatedTypes`, closed schema updated, check resolves the generated source |
| 9 | `M/🎭️actor/🧬️ownership/🧪️tests/🟦️.ts` | a framework test read a repo product policy file and tripped the product-reference law | uses its own fixture limits |
| 10 | `M/⏪️time-travel/♻️retirement/🧪️tests/🟦️.ts` | Rust root file now mounts children through `#[path]` | `readModuleTreeSource` (the established reader) |
| 11 | `M/🚪️io/🧬️schema/🧪️tests/🪪️artifact-kind/🟦️.ts` | wrong schema path (`🧬️schema/🧬️schema`) and `ArtifactKindId` now lives in `M/🧬️schema/🗿️artifact-reference/🔣️.json` | re-pointed |
| 12 | `M/🚪️io/🪶️sqlite-snapshot/🧪️tests/{🏛️ownership,🫙️prefix}` and `⚠️refusal/💰️limits/🧪️tests` | model/control carved into neutral sibling crates; renames `SqliteTransferLedger` to `SqliteSnapshotLedger`, `TransferLedgerStorage::Borrowed` to `SqliteSnapshotLedgerSlot::Lent`, `retirement_active` to `is_reserved()` | dependency roster extended by the two neutral crates; source reads use the module's `🦀️.rs` union; renamed identifiers |
| 13 | `M/🕸️graph/🧪️tests/🪆️record-owner`, `🛂️manifest/🧪️tests/🏷️type`, `🧪️tests/🧩️suite` | graph root crate carved into six child crates (no root `Cargo.toml`, no root `🦀️.rs`); strict compile target ES2022 cannot see `isWellFormed` | read the owning child crates; target ESNext |
| 14 | `M/🏗️mesh-engine/🧪️tests/🧫️fixtures/🧹️metadata/🧬️schema/🔣️.json` (new) | the closed schema the test validates against was never committed (only the fixture was) | schema written from the fixture's shape (strict, draft 2020-12) |
| 15 | `M/🏗️mesh-engine/🏛️ownership/🎨️attributes/🧪️tests/🟦️.ts` | Rust reads its fixture through `test_inputs::text(…)`, not `include_str!` | both the TS scan and the embedded Node oracle follow the new mechanism |
| 16 | `M/🧊️3d/…/🕸️topology/🧪️tests/🔬️reachability/🟦️.ts`, `…/🔀️boolean/🧪️tests/🔬️unit/🟦️.ts` | stale relative fixture path; directory URL without trailing slash | corrected |
| 17 | `M/📡️replication/🎮️mutation/🧵️canonical/🪪️identity/🧪️tests`, `🔏️seal/🧪️tests`, `📨️messages/♻️retire/🧪️tests` | fixture moved to `🪪️identity/🧪️tests/🧫️fixtures`; seal no longer unwraps an `Arc` (uses `alias_copy_bytes`); os ledger wraps `protocol::mutation::EditMessageLedgerRetirement` in `📒️ledger/📨️messages` | paths and identifiers |
| 18 | `M/🖱️ui/🧪️tests/🎚️axes/🟦️.ts`, `♻️physical-job-close/🟦️.ts` | label consumer moved to `M/🖱️ui/🌐️locale/🦀️.rs` (`pub use axes::{Locale, Terminology}`); `close_input_step` is `pub fn` now | paths and markers |
| 19 | `M/🌱️value/🛬️decode/🧪️tests/🧩️ownership`, `🧪️tests/🧩️neutral-owner`, `🧬️retained-clone/🌐️wire/🪆️stage`, `🗺️ordered-map/{📏️close,📋️wire}` | decode is its own crate; owners moved into `🔗️source`, `🌐️wire`, `🗂️map-core`; the removed `next_cold_byte_demand` is covered by the `next_close_byte_demand` assertion next to it (no occurrence left in the tree) | re-pointed, one superseded assertion removed |
| 20 | `M/🔲️pixels/…` via `M/🌱️value/🗂️ordered/🔢️numeric/🧮️scratch/🧬️schema/🔣️.json` | the shared fixture carries `closeReleaseBytes` and `maximumCloseTurns`; the closed schema did not declare them | two optional integer properties |
| 21 | `M/🖼️canvas/♻️retirement/🖊️stroke/🧪️tests`, `🔤️fonts/📦️publication/🧪️tests/🔣️.json` | `dash_pattern` lives in `🎬️scene/🖌️renderer/🖊️stroke-style`; the infinite fonts producer threads `scriptCompilerAdmissionControlV1` and no longer calls `withScriptInvocationSignalV1` (no occurrence left) | path; required-call list drops that one name |
| 22 | `M/🗺️surface/🧪️tests/🧩️suite/🟦️.ts` | build output now carries `.semio-artifact.json`; the producer script names `producer:"browser-canvas"` instead of an `outputDirectory` literal (the directory is asserted through `package.json` exports two lines above) | expectations |
| 23 | `M/🗣️dsl/🧬️schema/🧪️tests/🧱️ownership`, `M/🗣️dsl/🧪️tests/🧱️ownership` | record model carved (`📐️model`) and lexer/syntax/token carved into crates; dependency rosters grew by neutral crates | rosters and mounts |
| 24 | `M/🧬️schema/✅️validator/♻️retirement/🧪️tests` | owner types moved into `🔤️pattern` etc. | reads the validator's `🦀️.rs` union |
| 25 | `M/⏯️tool-run/🧪️tests/🧪️conformance/🟦️.ts` | pending-tick schema is draft 2020-12 | `ajv/dist/2020` for that schema |
| 26 | `PR/📓️print/🧬️schema/🔣️.json`, `🖼️assets/🔣️viz-api.json` | `geo-flow` inherits `geo-route` but the schema omitted `mode`, `samples`, `stations` (LaTeX implements them); generated API reference was stale against the LaTeX sources | three options added from `geo-route`, `generateVizArtifacts` re-run (only `viz-api.json` changed; 4,296 to 4,298 keys plus two earlier LaTeX-source additions) |

## 4. Remaining 60 bun failures and the other open items, by cause

| Cause | Count | Tests | Why not fixed here |
|---|---:|---|---|
| Fixture-placement law (SDK fixture facades live under `📦️packages/…/🧪️testing/🧫️fixtures`, law wants `🧪️tests` before `🧫️fixtures`) | 7 module + 1 products (+38 os, 1 pets `schema-conformance`) | Actor and Job, Framework core, Pack and Replication, Value and Async, UI x2 | repo-wide migration of about 70 fixture facades and their importers; the law and the fixture-entry work (`RESULT-TEST-FIXTURE-ENTRIES.md`) disagree, needs a decision |
| Framework code imports repo products (layering debt, the goal of this fleet) | 4 | Actor script (`buildRepositoryWasmWebV1`, also in editor and surface scripts), ToolRun script, graph execution (`runRepositoryCargoTestsV1`), dsl derive scripts | the neutral replacement (`wasm policy` and `cargo admission` outside the repo product) does not exist; the law must hold for dynamic imports as well, so a lazy import does not help (tried and reverted) |
| Rust source-text laws pinned to code the carve moved or renamed | 19 `🧵️job`, 5 `🖱️ui` (clipboard dispatch x2, prepared x1, feature-ownership x2 hashes), 1 async permit, 1 actor context, 1 io final publication, 1 composition admission, 1 registry identity, 1 locale `#[default]` | | each assertion pins a code snippet; snippets now live in sibling crates or were reworded. Needs per-assertion relocation by the carve owners (hash witnesses must be regenerated from the final code); the locale one needs a generator decision (first-entry `#[default]`) |
| Launcher/script envelope (`bun repo run` fails with `System image plist required`) | 4 module + 12 print material | pixels default task, 3d adapter invocation, process Go durability, schema TS API | environment, process owner |
| Native cargo sandbox or native oracle binary | 8 sandbox + 3 interop + 1 video | `linked projection input … node_modules`, `failed to load manifest for workspace member` (cargo workspace materialisation), Rust syntax/validator/value absence laws; semantic SQLite interop needs the built relational oracle; Chromium/FFmpeg video law reads `SEMIO_TEST_ARTIFACTS_DIR` (other lanes read `SEMIO_TEST_ARTIFACT_DIR`) | cargo blocked; command: run the group with the compile queue |
| Third-party package not installed and not declared | 3 | `color` (WCAG oracle for the styling suite and the `ShellScope` vitest file), `rxjs` (async poll test) | needs `bun install` with the dependency declared; no network here, not declared in any `package.json`/`bun.lock` |
| Artifact dir must be `…/🎆/🌙/☀️/<ticket>/🗑️generated` | 1 | schema neutrality portable composition | this slice's logs are nested one level deeper (`F/🗑️generated`) |
| Timing law bound to machine load | 1 here + 12 earlier observations | graph strict compiler admission (58 s under load vs 30 s test budget); earlier at load above 100: 4 quiz-react user-event tests (30 s), 2d `alignment`, 5 ui-react `text-flow`/`component` tests, Diagram 20k-node setup (8 ms budget) | all pass at load around 45 (quiz 82/82 alone, 2d 153/153, ui-react 1,166/1,167, Diagram 55/55 in two of three runs); fixed by running quieter, not by timeouts |

## 5. Verification actually run

All counts above come from the commands in section 1 run before and after (`modbun.sh z-*` groups for the final bun numbers, `modvt.sh z` and `vitest.sh *-z` for vitest). Per-fix re-runs: `buntest.sh t-*` logs `F/🗑️generated/tsx-bun-t-*`. Set difference of the 98 baseline and 61 final failing names: 0 newly failing. Not run: Playwright/Storybook browser lanes, the print inference harness, tsc.

## 6. Incidents and decisions

- Two codemods (`treeread.py`: module-tree reader for Rust reads; `unionread.py`: union of a module directory) were tried on 31 failing tests. They fixed only tests whose children are `#[path]`-mounted (kept for `⏪️time-travel`) and would weaken the others; both were reverted file by file from `tsx-tools/orig/`. The restore loop also overwrote 12 files that had already been fixed by hand; they were re-applied from the same edit scripts and re-verified in one batch (49 tests, 1 cargo-gated failure). `tsx-tools/orig/` is kept as the pre-edit snapshot of every file those codemods touched.
- `M/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts` was edited (lazy cargo import) and restored byte-identical to HEAD; no change remains there.
- Peers: the 2d compute ownership test and fixtures were rewritten concurrently by another slice (new test name, crate-root assertions); my fixture-list correction completes it. Files other slices modified in the same trees were left alone.

## 7. Files created or edited

Created: `M/🏗️mesh-engine/🧪️tests/🧫️fixtures/🧹️metadata/🧬️schema/🔣️.json`; `F/tsx-tools/*`; this file. Edited: the test, fixture, schema and generated files named in section 3 (about 55 files). Logs: `F/🗑️generated/tsx-*` (kept).
