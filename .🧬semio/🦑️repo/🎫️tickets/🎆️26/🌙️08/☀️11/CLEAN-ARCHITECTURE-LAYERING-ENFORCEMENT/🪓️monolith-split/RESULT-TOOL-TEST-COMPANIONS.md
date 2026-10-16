# Tool Test Companions (slice TOOL-TEST-COMPANIONS, prefix `ttc`)

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no git-modifying command. `M` = `🧰️framework/🔨️modules`. Tools: `ttc-tools/` (fork of `ftc-tools/` plus the fixes in section 6). Logs: `🗑️generated/ttc-*` (nothing deleted). Pre-edit snapshots (sandbox, outside the repo): `scratchpad/ttc-snap/{toolrun,toolmachine}`.

## 0. Verdict

DONE for both crates, static evidence only. No compiler ran: the compile window is refused (Wave 6), so no `bun repo cargo` call was made.

| gate | `semio-framework-tool-run` | `semio-framework-tool-machine` |
| --- | --- | --- |
| census, native lib-test unit (files / bytes / active lines) | 47 / 462,518 / 4,351 -> **20 / 229,307 / 3,225** | 15 / 422,596 / 3,663 -> **6 / 97,637 / 2,000** |
| census, companion lib-test unit | **12 / 140,873 / 1,246** | **4 / 89,129 / 1,719** |
| census, wasm32-unknown-unknown and wasm32-wasip2 | identical to native, all `over=false` | identical to native, all `over=false` |
| compiled embedded fixtures | 16 files / 100,857 B -> **0** | 6 files / 237,878 B -> **0** |
| test fn multiset (`ttc.py tests`) | 44 = 44, IDENTICAL | 71 = 71 (40 tool-machine + 31 untouched `machine` tests that the snapshot also holds), IDENTICAL |
| fixture byte identity (`ttc.py hashes`, sha256 of every non-`.rs` input of the pre-edit closure) | 16 inputs, 0 changed or missing | 6 inputs, 0 changed or missing |
| patched union oracle, 6 configs (lib, test, wasm, p2, wasmtest, p2test) | 0 resolve, 0 privacy, 0 internal type errors NEW (section 4) | same |
| duplicate mounts (repo-wide `#[path]` scan) | 0 (38 targets, each mounted once) | 0 (8 targets) |
| rustfmt parse | 23 files parse | 7 files parse |

The baseline units in the census doc (40 / 460,530 / 4,295 and 11 / 421,588 / 3,631) predate the `tgm` root splits. The values above were measured at 14:3x, right before the first edit (`🗑️generated/ttc-baseline-units.tsv`).

## 1. Method

* Companion crate (precedent `ftc`: `io-base64`, `print`): `<module>/🧪️tests/🥣️companion/🦀️.rs` (`#![cfg(test)]`, `pub use <crate>::*;`, mounts the existing test files with `#[path]`, so no test file moves and every TS source reader keeps working) plus `<module>/🧪️tests/🥣️companion/📦️packages/🦀️rust/{Cargo.toml,📋️project.json}`. The parent loses every `#[cfg(test)]` mount and its dev-dependencies; the companion owns them.
* Embedded inputs: every `include_str!` of a test became `crate::fixture_text("<path relative to the module directory>")`, a run-time read through a leaked cache in the companion root (`CARGO_MANIFEST_DIR/../../../..`, same helper as the `print` companion). The three `const X: &str = include_str!` of the tool-run unit file and the six of the tool-machine files became `fn x_text() -> &'static str`. Fixtures did not move.
* Tests use the public API. What they name beyond it is exposed only behind the new `testkit` feature of the parent (`[features] testkit = []`, enabled by the companion's dependency edge):
  * struct fields keep their old visibility without the feature and are `#[doc(hidden)] pub` with it, as two cfg-exclusive field declarations next to each other (`tools: ttc-tools/dualfield.py`);
  * items that the package stub does not export are re-exported from a `#[doc(hidden)] pub mod testkit`;
  * one `pub(crate)` method gets a `#[doc(hidden)] pub fn testkit_*` wrapper.
* Four private helpers of the old parent modules that the tests used (`Receipt`, `original`, `funds`, `counters`/`steps`) are now local helpers of the test files (pure functions; the independent copy is a better oracle than calling the library's own).
* Parent private `use` lines that the tests relied on through `use super::*;` were copied into the test files, restricted to the names each test mentions (`ttc-tools/useimports.py`, `apply_imports.py`).

## 2. What changed

### tool-run (`M/⏯️tool-run`)

New: `🧪️tests/🥣️companion/🦀️.rs` (mounts 11 test files as `tests`, `original_retirement_tests`, `original_tick_successor_tests`, `original_tick_admission_tests`, `original_tick_encode_tests`, `original_receiving_limits_tests`, `original_native_tick_tests`, `original_progress_receiving_tests`, `original_step_receiving_tests`, `original_entity_column_tests`, `original_identity_receiving_tests`; owns the `#[global_allocator]` `HeapWitness` that used to sit in the lib root) and `🧪️tests/🥣️companion/📦️packages/🦀️rust/{Cargo.toml,📋️project.json}` (package `semio-framework-tool-run-tests`, project `@semio-tech/framework-tool-run-tests-rs`, id `tool-run-tests`; dependencies `semio-framework-tool-run` with `testkit`; dev-dependencies `pack`, `replication`, `trace`, `ui-locale`, `value`, `serde_json`).

Edited: `🦀️.rs` (removed the allocator and the three test mounts, added `pub mod testkit`), `📦️packages/🦀️rust/🦀️.rs` (removed the `cfg(test)` `extern crate self`, added the `testkit` re-export), `📦️packages/🦀️rust/Cargo.toml` (dev-dependencies `trace` and `serde_json` removed, `[features] testkit = []`), `📦️packages/🦀️rust/📜️script.ts` (`COMPANION` constant: `test` runs `[COMPANION, ENTITIES, PRESENTATION]`, the seven `test-original-*` commands run the companion, `check` additionally runs `-p COMPANION --tests`), the 6 receiving children + `🎞️tick/🛫️encode/🦀️.rs` + `♻️retirement/🦀️.rs` (their `#[cfg(test)] mod tests;` mounts removed; `♻️retirement/🦀️.rs` is now docstring-only and stays mounted), the 11 test files (fixture reads, imports, local helpers, `Phase` imports, `testkit_entity_column`).

Testkit surface (everything the oracle reported as a privacy error before it was added; 22 doubled fields, 2 enum exports, 1 module export, 1 method wrapper):

| item | kind |
| --- | --- |
| `ToolRunTickWriter`: `identity next_sequence next_page next_step provisional_base progress steps pages page_ops append_ops append_entities entity_marks retract_to payload` | 14 dual fields (were `pub(crate)`) |
| `ToolRunStepRing.steps` | dual field (was `pub(crate)`) |
| `ToolRunStepReceiving.{argument_at, argument}`, `ToolRunProgressReceiving.{headers, phase, slot, step}`, `ToolRunEntityColumnReceiving.phase` | 7 dual fields (were private) |
| `Phase` of progress and entity-column receiving | `enum Phase` -> `pub enum Phase` inside a private module, exported only as `testkit::{progress_receiving,entity_column_receiving}::Phase` |
| `tick_source` module | exported as `testkit::tick_source` |
| `ToolRunNativeTickPreparation::entity_column` (`pub(crate)`) | wrapper `testkit_entity_column` |

### tool-machine (`M/🛠️tool-machine`)

New: `🧪️tests/🥣️companion/🦀️.rs` (mounts `tests`, `node_graph_edit_rows_tests`, `gesture_drive_law_tests`), `📦️packages/🦀️rust/{Cargo.toml,📋️project.json}` (package `semio-framework-tool-machine-tests`, project `@semio-tech/framework-tool-machine-tests-rs`, id `tool-machine-tests`; dev-dependencies `machine` (renamed), `replication`, `value`, `blake3 = "1.8.2"`, `serde_json`).

Edited: `🦀️.rs` (the `//#region 🧪️Tests` block with its three mounts removed, both region markers with it), `📦️packages/🦀️rust/Cargo.toml` (dev-dependencies `blake3` and `serde_json` moved to the companion, `[features] testkit = []`), `📦️packages/🦀️rust/📜️script.ts` (`test` runs the companion, `check` adds `-p COMPANION --tests`), `🫧️scrub/🦀️.rs` (`ScrubLedger.closed` dual field: the only privacy crossing of the three test files), the 3 test files (fixture reads, imports).

The tool-machine tests needed exactly one hook (`ScrubLedger.closed`); it only became visible once the real `machine` crate was part of the union, because the oracle reaches body type-check only then (section 6).

### Registration (both companions)

* `🧰️framework/Cargo.toml` members (byte-sorted): `🔨️modules/⏯️tool-run/🧪️tests/🥣️companion/📦️packages/🦀️rust` (after `…/🕹️actions/…`), `🔨️modules/🛠️tool-machine/🧪️tests/🥣️companion/📦️packages/🦀️rust` (after the crate itself). Root `Cargo.toml` is not touched: neither parent is a root member.
* `📋️build-owners.json`: rows `@semio-tech/framework-tool-run-tests-rs` and `@semio-tech/framework-tool-machine-tests-rs` (name-sorted, `root` = package dir, `packages: []`).
* `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: six routes each (`🏗️build`, `🔎️check`, `🧪️test` x `🛠️prototype`, `🚀️release`), in the group and order bands the `ftc` companions use, inserted after the `os-spr-laws-tests` block (`reg.py check`: 6 named routes, 6 commands in both files, both crates).
* `🧰️framework/Cargo.lock`: hand-written entries (no cargo available): `tool-run` loses `trace` and `serde_json`, `tool-machine` loses `blake3` and `serde_json`, two new `*-tests` packages. The root `Cargo.lock` is unaffected (non-member path dependencies carry no dev-dependencies).
* Taxonomy: `🥣️companion`, `📦️packages`, `🦀️rust` under both `🧪️tests` resolve (`frs-tools/taxcheck.ts`: `checked=3 unresolved=0` twice).
* Corpus rows: none exist for these areas (the only crate-boundary corpus is the UI one).
* `📋️project.json` of the parents: unchanged (their targets call `📜️script.ts`, which now runs the companion).

## 3. Census (`ttc-tools/ttc-census.ts`, `tcc-census.ts` plus `EXTRA_MANIFESTS` and a `ttc-census-*` output prefix)

Command: `EXTRA_MANIFESTS="<both companion manifests>" ONLY=semio-framework-tool-run,semio-framework-tool-run-tests,semio-framework-tool-machine,semio-framework-tool-machine-tests bun ttc-tools/ttc-census.ts` (log `🗑️generated/ttc-census-after-both.log`): 4 crates, 396 units, 0 error units, **0 over-limit test units on any of the three labels, 0 over-limit dev-dependency rows**. Baseline: `🗑️generated/ttc-baseline-units.tsv`. Largest dev-dependency libs of the companions: `io-sqlite-snapshot` 12 / 239,072 B (0.912), `value-derive` 9 / 227,493 B (0.888), `pack-json` 9 / 226,408 B (0.864): all under the ceiling already.

The normal lib closures: tool-run grew by 2,129 B and 13 active lines (227,178 -> 229,307 B, 3,212 -> 3,225: dual fields and hooks); tool-machine shrank (97,835 -> 97,637 B, 2,005 -> 2,000: the removed test region). Figures before the edit are from `crate-census.csv`.

## 4. Differential and type-level evidence

Tool: `ttc.py oracle` = the `ftc` companion-aware union oracle (`rustc +nightly-2026-07-20 --edition 2021 --emit=metadata`, output to a sandbox `.rmeta` that is deleted, `--cap-lints allow`): baseline = pre-edit snapshot of the parent (tool-machine: parent + `machine`), current = working tree (parent built without `cfg(test)`, `--test` for the companion, features from the dependency edges, so `testkit` is on). Stub layer for every external crate; 0 stub-glob fills (no name was silently invented).

| family | lib | test | wasm32-unknown-unknown | wasm32-wasip2 | wasmtest | p2test |
| --- | --- | --- | --- | --- | --- | --- |
| tool-run (final run, `ttc-pos-toolrun-final-*`, all six in `ttc-pos-toolrun-full-*`) | 961 vs 960, NEW 1 | 1710 vs 1696, NEW 14 | NEW 1 | NEW 1 | NEW 14 | NEW 14 |
| tool-machine (`ttc-pos-toolmachine-full-*`) | 44 = 44, NEW 0 | 463 vs 515, NEW 0 | 54 = 54, NEW 0 | 44 = 44, NEW 0 | NEW 0 | NEW 0 |

Every one of the 1 + 14 NEW tool-run entries is stub noise: 11 are E0599/E0609 on `__ext::` types (`RetainedCloneGrant`, `RetirementDemand`, `RetainedRecordBodySourceDecode` have no fields or methods in the stub; they appear as NEW only because the test-local helpers `funds` and `original` repeat expressions that the lib had in other files) and 3 are E0747 (`PagedList<_, N>` const generic against the generic stub; the same message exists in the baseline for the original `entity_column` signature). `priv.py` (privacy plus every non-`__ext` NEW diagnostic): E0616 / E0624 / E0603 / E0451 = 0 in all six configurations of both families.

Seeded sensitivity (single-config `test` runs):

| seed | result |
| --- | --- |
| first run of the full tool-run companion, before the dual fields | 22 distinct E0616 and 1 E0624 (the list in section 2 is exactly this list) |
| `pub retract_to` of `ToolRunTick` -> private | E0616 `retract_to` |
| `pub fn testkit_entity_column` -> private | E0624 `testkit_entity_column` |
| `ScrubLedger.closed` `pub` -> private (tool-machine) | E0616 `closed` |
| tool-machine without `machine` in the union | **blind**: item-level errors from the stub `Machine`/`Host` traits stop rustc before body type-check (a probe `t.zzz_nothing` produced no E0609); fixed by adding the real `machine` crate to the family |

Losslessness of the test files: all 14 test files differ from the snapshot by 61 removed and 137 added lines (`🗑️generated/ttc-testfile-diff.log`); the added lines are fixture-read calls, the `fn x_text()` wrappers, parent-import lines, the `Receipt` alias in three files, the local helpers `original`, `funds`, `counters`, `steps`, and the `Phase` imports. Test fn names are untouched (44 + 40, identical multiset). Region markers: the removed tool-machine `//#region 🧪️Tests` block took both markers with it, the others are untouched.

Other static gates: duplicate-mount scan over every `#[path]` of the repo (`ttc-tools/dupmounts.py`, `git grep --untracked`): 46 targets below the two modules, 0 mounted twice, 0 missing; runtime fixture reads (`crate::fixture_text/bytes`): 22 + 6 literals, all resolve to an existing file; `include_str!` / `include_bytes!` left in the two crates' test trees: 0; `bun test` of `🏛️ownership`, `♻️retirement`, `⏭️successor`, `🎟️admission` TS tests: 4 pass, 1 fail (section 7).

## 5. Real checks for the compile queue (not run)

Under the fleet mutex, one launcher call per line (the lock entries are hand-written: run the first one; it must be a no-op diff on `🧰️framework/Cargo.lock`):

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- update --workspace --offline --manifest-path 🧰️framework/Cargo.toml
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-tool-run -p semio-framework-tool-machine --lib --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
… -- check -p semio-framework-tool-run-tests -p semio-framework-tool-machine-tests --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
… -- check -p semio-framework-tool-run -p semio-framework-tool-machine --lib --target wasm32-wasip2 --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml   (and --target wasm32-unknown-unknown)
… -- test -p semio-framework-tool-run-tests -p semio-framework-tool-machine-tests --lib --manifest-path 🧰️framework/Cargo.toml
```

Expected: 44 + 40 tests run in the companions (the old `--lib` targets of the parents now hold 0 tests). Direct dependents to add to the `check` lines once green: every crate with a `semio-framework-tool-run` or `semio-framework-tool-machine` edge (the `*-entities`, `*-presentation`, `*-actions` siblings, the s-artifact crates named in `RESULT-TOOL-GRAPH-MACHINE-SPLITS.md` §6): nothing they import changed except two removed dev-dependencies and the new feature. The TS routes stay `bun repo run @semio-tech/framework-tool-run-rs:test-source` and the `test-original-*` targets.

Highest-risk spots at the first compile (the oracle cannot see them): `RetireOwned` / `ToValue` derive expansions on the dual-field structs (the derive sees only the active field, which is the intent); the `statechart!` expansions of the tool-machine tests (they were unresolved in the sandbox, so test bodies that name generated items were only resolution-checked); `use pack::…` / `use protocol::…` heads in the companions rely on the library target names of the dependencies (`pack`, `protocol`), as the old in-crate tests did.

## 6. Tooling findings (shared by every union-oracle fork)

* **Head-regex defect.** `ttcunion.py` (and its ancestors `ftcunion.py`, `pos`/`tgm` forks) rewrites the head of a path such as `serde_json::x` only when the preceding character is not `:`. In dense code (`let law:serde_json::Value=…`, `grant:semio_framework_value::T`) the head sat after a single `:` and was left unresolved. In tool-run this produced enough resolve errors to stop rustc before body type-check, so privacy errors (E0616/E0624) were invisible. Fixed here with `(?<![\w$.])(?<!::)`; with it 22 E0616 and 1 E0624 surfaced at once. Earlier oracle runs of other slices on dense one-line files may have the same blindness (the per-crate "NEW 0" there proves resolution, not field privacy, unless the run shows body-level diagnostics such as E0599/E0308).
* **Dangling attribute.** `extern crate self as …` was removed together with nothing, leaving a `#[cfg(test)]` that attached to the next item (here `mod component;`). The rewrite now removes the preceding attribute lines too.
* **Stub traits stop type-check.** A family whose items implement traits of an external crate (`impl Machine for …`) must include that crate in the family (done for `machine`), otherwise E0437/E0407/E0220 abort before body type-check.
* New helpers: `ttc.py snapadd` (add a package to an existing snapshot), `dualfield.py`, `fixture_rewrite.py`, `apply_imports.py`, `priv.py`, `rep.py`, `dupmounts.py`, `mut.sh`, `orc.sh`.

## 7. Foreign and pre-existing

* `bun test M/⏯️tool-run/🏛️ownership/🧪️tests/🟦️.ts`: the first test passes (it now also scans the two new companion trees); the second ("General ToolRun TypeScript imports remain inside the neutral Framework owner") fails because `📦️packages/🦀️rust/📜️script.ts` imports `🛍️products/🦑️repo/…/🎟️admission`. That import is in `HEAD` and in the index (2 hits each), not from this change.
* `git` prints `error: daemon terminated` on some calls (fsmonitor); every git call here used `-c core.fsmonitor=false`.
* Machine load was 47 to 55 during the oracle runs (about 15 to 25 s per run instead of 4 s).

## 8. Hand-offs

* `🔒️dependencies.json` still lists `serde_json` and `blake3` users under the two parent manifests and none under the companions (same as the `ftc` companions). It is a generated baseline; the coordinating session regenerates it (`bun ./📜️script.ts verify dependencies write-baseline`).
* The plugin test `M/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/🌱️operation/🧪️tests/🟦️.ts` still reads the framework tool-run root as text for `admit_original_op` (hand-off already recorded by `tgm`); untouched here.
* `M/⏯️tool-run/♻️retirement/🦀️.rs` is a docstring-only module; it was test-only glue. It is kept so the taxonomy node and the TS reader `readModuleTreeSource(…/🦀️.rs)` stay unchanged. The owner of the node may drop it together with the root mount line.
* Other crates with the same pattern (private-field white-box laws mounted as `cfg(test)` children) can reuse `dualfield.py` and the `testkit` convention of section 1.

## 9. Changed files

Created: `M/⏯️tool-run/🧪️tests/🥣️companion/🦀️.rs`, `M/⏯️tool-run/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml`, `…/📋️project.json`, `M/🛠️tool-machine/🧪️tests/🥣️companion/🦀️.rs`, `M/🛠️tool-machine/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml`, `…/📋️project.json`, this file, `ttc-tools/`.

Edited: `M/⏯️tool-run/🦀️.rs`, `M/⏯️tool-run/📦️packages/🦀️rust/{🦀️.rs,Cargo.toml,📜️script.ts}`, `M/⏯️tool-run/⏱️tick-writer/🦀️.rs`, `M/⏯️tool-run/🧮️step-progress/🦀️.rs`, `M/⏯️tool-run/♻️retirement/🦀️.rs`, `M/⏯️tool-run/🎞️tick/🛫️encode/🦀️.rs`, `M/⏯️tool-run/🎞️tick/🛬️receive/{🎟️limits,🏗️native,📊️progress,📝️step,🔢️entities,🪪️identity}/🦀️.rs`, the 11 tool-run test files, `M/🛠️tool-machine/🦀️.rs`, `M/🛠️tool-machine/📦️packages/🦀️rust/{Cargo.toml,📜️script.ts}`, `M/🛠️tool-machine/🫧️scrub/🦀️.rs`, the 3 tool-machine test files, `🧰️framework/Cargo.toml`, `🧰️framework/Cargo.lock`, `📋️build-owners.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`.
