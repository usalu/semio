# ✂️ Result: Plugin File Splits B (energy and fem)

Slice PLUGIN-FILE-SPLITS-B, prefix `pfb`, executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. Date 2026-10-16. Tools: `pfb-tools/` in this folder. Logs and the pre-edit snapshots (`pfb-orig/`) are in `🗑️generated/pfb-*` (nothing deleted there).

Paths: `E/` = `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1`, `S/` = `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim`, `V/` = `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job`.

## 0. Verdict

All four files are split into cohesive children of at most 1,301 lines (Rust at most 989). The split is verified statically only; nothing was compiled by cargo, because the wave-6 brief refuses compile admission. What the checks prove:

- **Lossless.** Line multiset of old versus new: 0 lines lost in `S/` and `V/`. In `E/🦀️.rs` exactly 3 lines are gone on purpose: the `const VECTORS: &[Vector] = &[` header (each table now has its own, with a new name and a re-added `];`), the inline `mod subject {` line and its closing brace. The Python statement multiset has 0 lost statements.
- **Resolution differential identical.** For `S/`, `V/` and the Rust test: 0 new diagnostics and 0 new warnings, lib and `--test` (and `sut` for the test). For the Rust test a stubbed-API type check (section 4.3) gives 0 errors and 0 warnings in base and candidate, which is a real type check of the changed root.
- **Python test identical.** The adapter runs through the repository's own Python host (`.venv` 3.14.5, 1,189 scenarios) before and after with identical status, raw hash, projection hash and first diagnostic for every scenario: 1,159 passed, 2 failed, 28 errored in both (all baseline defects, section 6).
- **Tables proven equivalent.** The 594 committed vector rows (id, kind, five fixture paths) are identical old versus new after resolving every `include_str!` path, and the sha256 of all 2,970 referenced fixture files is the same by construction (same files).
- **Census** hard budget holds for both crates. **`--check-applied`** is red at baseline for the two artifact packages that can be checked, with numbers identical before and after (section 5); it is not an exit-0 pass, and I did not cause it.

Data to fixtures: the per-vector data is already committed as 2,970 JSON fixture files that both tests read. What the giant files held was only the index (594 rows in Rust, 594 in Python `VECTOR_ROOTS`, 594 in the feature's `Examples`). Replacing the Rust `include_str!` index by a runtime read from one shared manifest would change test semantics and cannot be validated without a compile, so I left it as a hand-off (section 7).

## 1. Found state

Fresh slice, nothing resumed. Peer state at the start: the four files were identical to the snapshot taken before the first edit (`diff -rq` against `🗑️generated/pfb-orig/`); other agents edit unrelated parts of the same plugin trees.

## 2. What changed

### 2.1 Source splits (children are `<emoji>️<kebab>/🦀️.rs` or `🐍️.py`, wired with `#[path]` plus explicit `use`)

| File | Before | After |
| --- | ---: | --- |
| `E/🦀️.rs` | 5,872 | root 100; `🏷️kinds` 316; seven vector tables `🏠️building` 819, `🪟️openings` 459, `🪨️fabric-and-gains` 729, `💨️airflow-and-controls` 801, `🔧️zone-systems` 873, `🔌️power-and-plant` 909, `📆️schedules-and-layers` 819; `🔮️oracle-role` 22; `🎯️subject-role` 130 |
| `E/🐍️.py` | 7,883 | root 398; `🧱️base` 69; the same seven domain folders: `🏠️building` 934, `🪟️openings` 687, `🪨️fabric-and-gains` 1,073, `💨️airflow-and-controls` 1,189, `🔧️zone-systems` 1,269, `🔌️power-and-plant` 1,301, `📆️schedules-and-layers` 1,216 |
| `S/🦀️.rs` | 3,026 | root 62; `🧱️state` 626; `📮️packets` 402; `🔍️pipeline` 841; `🏃️run` 544; `🔚️close` 515; `🚀️engine` 86 |
| `V/🦀️.rs` | 2,836 | root 40; `🧾️contract` 133; `🧮️solver-view` 357; `🧱️model-step` 644; `🪜️solve-step` 989; `📄️page-job` 723 |

Details that matter:

- **Rust test (`E/`).** Test titles are unchanged (`mutate-<id>`, `inverse-<id>`, `identity-round-trip`; registration loop and scenario id format untouched). The single `VECTORS` table became seven `pub(super) const <DOMAIN>_VECTORS` tables, one per domain folder, chained in catalog order by `fn vectors()` in the root; the three users (`vector()`, the `debug_assert!`, the registration loop) call it. `include_str!` paths gained one `../` because each table sits one folder deeper. `mod subject` became a file module gated by `#[cfg(feature = "sut")]` on its mount; its body is dedented by 4 spaces and keeps its own explicit imports. All `//#region` markers are kept; a region that now spans files is opened and closed in each file that holds a piece of it (`Fixtures` in the root and in each table).
- **Python test (`E/`).** The same seven domain folders hold the Python kinds, so the two tests stay parallel: each folder has a `🦀️.rs` vector table and a `🐍️.py` with its slice of `VECTOR_ROOTS`, its kind functions and inverse companions, its helper functions, and its slice of `VOCABULARY` and `EXTRA_INVERT`. The root merges the slices in catalog order (verified: key order and function names equal the original). Shared wire/outcome/link vocabulary is `🧱️base`. Helpers were placed by dependency analysis (`FIELD_LEAVES`, `_zone`, `_site_leaf`... in `building`; `_finite`, `_on_plane`, `_change_glazing_scalar` in `schedules-and-layers`); the import graph is `base <- every group`, no cycles. The root loads the children by path with a small `_load(alias, folder)` helper (importlib, registered in `sys.modules`), because the host loads the adapter file alone and emoji folders are not importable names. Each file has `from __future__ import annotations`, only the stdlib imports it uses, and a docstring starting with its folder emoji.
- **Engine (`S/`).** One 2,000-line `impl EnergyJobAuthority` is spread over four children (`packets`, `pipeline`, `run`, `close`) as separate inherent impl blocks; the type itself lives in `state`. `pub use self::state::*` and `pub use self::engine::*` keep every public path; `use self::close::*` (private) shares the moved helper functions among siblings. No region markers existed in this file.
- **Visual job (`V/`).** `Fem3dNumericalChild` (struct and impl) is spread over `model-step` and `solve-step`. Regions: `🔖️Contract` was never closed in the original; its child now ends with `//#endregion 🔖️Contract` (the only marker added). `📦️PageVisualJob` spans four children and is opened and closed in each.
- **Visibility.** Private items, methods and fields that another file of the module tree still names were widened to `pub(super)`: 162 in `S/`, 68 in `V/`, 6 in `E/🦀️.rs` (`KINDS`, `UNOBSERVABLE`, `DSL_ASSET`, the two oracle fns, one helper). Two splitter defects were found and fixed on the way: a struct field list containing `Option<X<'static>>` (a `>>` token) stopped the field scan, and a private struct whose fields another file reads was left private. The oracle caught both in `V/` (E0616).

### 2.2 Source-text readers and registry (edits outside the four files, each one line or one small block)

| File | Change |
| --- | --- |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📃️case-contract/🟦️.ts` | New `adapterModuleTree`: a test-case child directory that holds only taxonomy-declared adapter files (recursively) is no longer an `unknown-case-child`. Before this, any split adapter inside a case folder breached the contract. The existing `🧩️support` child of `plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences` is accepted by the same rule. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧫️case-fixtures/🟦️.ts` and `…/🧫️fixtures/🧫️case-fixtures/🔣️.json` | Rows `adapter-module-child-is-accepted` and `adapter-module-child-with-other-files-is-rejected` (children may now carry `files`). |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs` | Nx cache inputs track `{workspaceRoot}/<caseDir>/**/<adapter file name>` instead of the single adapter file, so editing a split child invalidates the case. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/⚡️exhaustive-cache-inputs/🟦️.ts` | New test `inputsFor tracks every adapter file of the case folder...`. Not run: this whole suite fails in its `beforeAll` under bare `bun test` (`Original parent Script process invocation required`) and also calls into `🟨️.mjs` internals that need the owned envelope, the same before my change. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json` | Removed the pin of `S/🦀️.rs` (3,027); the root now has 62 lines and complies. |
| `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🏛️bestest/🦀️.rs` | One doc comment re-pointed from `🧪️sim/🦀️.rs` to `🧪️sim/🔍️pipeline/🦀️.rs` (`step_aggregate_zone` moved). |

No Rust or Python reader of the four files' text exists (searched with `git grep -F` over the tree for the paths, the crate names and the moved item names): the only hits were Cargo manifests, `📋️build-owners.json`, the ratchet and the comment above. The one adapter-text reader in the test platform (`case-contract`, entry-point regex) reads the root, which still holds `pub fn adapter()` and `def adapter()`.

Taxonomy: no `🔣️taxonomy.json` edit was needed. All 22 new folders resolve through the live engine function `semanticDirectoryKindId` (`pfb-tools/tax.ts`, walking each path from the repo root): the energy case folders through `members-of-tests`, the `S/` and `V/` folders through `crate-module-children` (the slug-pattern rows added since the taxonomy sweep). 66 directories checked (the new folders plus their ancestors), 0 unresolved. Folder names were chosen so none collides with a structural kind (`🔮️oracle` would have resolved to the `oracle` kind, so the folder is `🔮️oracle-role`, and `🎯️subject-role` likewise). Sibling emojis are unique in every folder; every new file starts with a docstring whose first character is its folder emoji; the longest path is far below 240 bytes (`pfb-tools/verify.py`).

## 3. Census (`pfb-tools/census.ts`, the planner's `measureRustCrate`, files / bytes / active lines)

| crate | lib before | lib after | test before | test after | hard 32/262,144/4,000 | target 28/230,000/3,500 |
| --- | --- | --- | --- | --- | --- | --- |
| `semio-s-plugin-energy-engine-sim` | 1 / 146,500 / 2,970 | 7 / 149,581 / 3,019 | 2 / 192,904 / 3,822 | 8 / 195,985 / 3,871 | pass | fails before and after (test lines above 3,500; the 800-line unit-test mount is the cause, untouched) |
| `semio-s-artifact-fem-3d-editor-visual-job` | 2 / 140,507 / 2,858 | 7 / 142,602 / 2,907 | same as lib | same as lib | pass | pass |

The energy test case is not a crate (the host mounts it with `#[path]`), so it has no census row. `lib-wiring-budget`: the pin removal is consistent (see 4.2).

## 4. Verification actually run

### 4.1 Tools

All in `pfb-tools/`: `split.ts` (line-range splitter, specs `sim.spec.json`, `vj.spec.json`, `rs.spec.json`), `widen.ts` (visibility widening, copied from `pas-split-flat.ts` and fixed), `rspatch.py` (root patch of the Rust test), `pysplit.py` (Python splitter), `restore.py` (reverts one split to the snapshot), `rdiff.py` (differential oracle), `rscmp.py` (table equivalence), `pyrun.py` / `pycmp.py` (Python host run and comparison), `pychk.py` (Python static and load checks), `privacy.ts` (visibility fixpoint check), `tax.ts`, `census.ts`, `verify.py`.

### 4.2 Commands and results

| Check | Command (from `/Users/ueli/Documents/semio`, `F` = this folder) | Result |
| --- | --- | --- |
| lossless, Rust | `bun F/pfb-tools/split.ts F/pfb-tools/{sim,vj,rs}.spec.json` | `S/` 0 lost, `V/` 0 lost; `E/` 3 lost (the 3 intended lines above); widened 162 / 68 / 6 |
| lossless, Python | `.venv/bin/python3 -B F/pfb-tools/pychk.py` | 0 lost statements; the added statements are the loader, the seven merges, docstrings and region wrappers |
| rustfmt parse | `rustfmt --edition 2021 --emit stdout <each new file>` (inside `verify.py`) | exit 0 for all 24 Rust files |
| scoped differential, `S/` | `python3 F/pfb-tools/rdiff.py sim` and `sim --test` | 149 / 68 errors in base and candidate (all unresolved external crates), identical multisets, 0 new, 0 new warnings |
| scoped differential, `V/` | `rdiff.py visual-job` and `visual-job --test` | 31 / 31 errors, identical, 0 new warnings. This oracle reached type checking (it reported E0616 for the two splitter defects before they were fixed) |
| scoped differential, Rust test | `rdiff.py energy-test`, `--cfg feature=sut`, `--test --cfg feature=sut` | 1 / 6 / 6 errors (unresolved host crate, and the baseline E0599 of section 6), identical, 0 new warnings; every `include_str!` path resolves (no read error) |
| type check with stubbed API | `rdiff.py energy-test --stub`, `--stub --test`, `--stub --without-expect --cfg feature=sut`, `--stub --without-expect --test --cfg feature=sut` | 0 errors and 0 warnings in base and candidate in all four (the stub is an inline module in the sandbox copies only; `--without-expect` removes the baseline defect from both copies) |
| oracle blind spot | one-off experiment in the scratchpad: strip the `pub(super)` of a `S/` child and run rustc | no E0616/E0624 appears, so for `S/` the differential does not see privacy errors (typeck stops on the unresolved external types). Compensated by `privacy.ts`: 0 violations on the final files of `S/`, `V/` and `E/`; the negative control (all `pub(super)` stripped from `🧱️state`) reports 135 |
| table equivalence | `python3 F/pfb-tools/rscmp.py` | 594 vectors old = new (id, kind, five resolved fixture files each, sha256 of all 2,970 files), 297 kinds with exactly two vectors each, chain order = catalog order |
| structure | `python3 F/pfb-tools/verify.py` | no problems: mounts equal folders, no duplicate `mod`, region balance, docstring emoji, path length, at most 1,500 lines |
| Python before and after | `python3 F/pfb-tools/pyrun.py <🐍️.py> F/🗑️generated/pfb-py-{before,after}.jsonl`; `python3 F/pfb-tools/pycmp.py before after` | 1,189 scenarios: 1,159 passed, 2 failed, 28 errored in both runs; 0 differences in status, raw hash, projection hash, first diagnostic. Runs are deterministic (two baseline runs also identical) |
| Python static | `pychk.py` | all 9 files compile with 0 undefined globals (symtable); original and new modules loaded side by side: `VECTOR_ROOTS` 594, `VOCABULARY` 297, `EXTRA_INVERT` 270, `FIELD_LEAVES` 13 equal in content and order, 1,189 handler registrations equal |
| taxonomy | `bun F/pfb-tools/tax.ts F/🗑️generated/pfb-tax-dirs.txt` | 66 directories, 0 unresolved |
| case-contract change | `SEMIO_TEST_ARTIFACT_DIR=F/🗑️generated/pfb-case-fixtures bun test ./🟦️.ts` in `🧪️test/🧪️tests/🧫️case-fixtures` | 10 pass 0 fail (8 existing + 2 new rows) |
| wiring budget | `bun test ./🟦️.ts` in `📚️library/🧪️tests/📏️lib-wiring-budget` | 2 pass 1 fail. The pin removal is consistent ("ratchet lists no root that complies" passes). The one failure is a peer root, section 6 |
| 🟨️.mjs loads | `bun -e "import('…/🟨️.mjs')"` | loads, exports present (the change is a one-line template) |
| contract lint of the case | `perl -e 'alarm 900; exec @ARGV' -- bun ./📜️script.ts contract --owner-root <E owner>` in `🧪️test` | not completed: it reached stage `running` and produced no output until the 900 s cap killed it (machine load about 40); log `pfb-contract-energy.log`. The `unknown-case-child` rule is therefore covered only by the 10 case-fixture tests |

### 4.3 What the stubbed type check covers

`rdiff.py --stub` replaces `semio_repo_test_host::` and the two `semio_s_artifact_energy_model::…::*_json` imports by an inline stub module (types `Json`, `Context`, `Outcome`, `Adapter`, `law::*`, `parse_json`; two report functions) in the sandbox copies of both the original and the split tree. It checks the root, the vector chain, the oracle and subject children and the registration loop with real type inference and borrow checking.

## 5. `--check-applied`

`bun ./📜️script.ts new crates <pkg> --check-applied --sandbox=<dir>` (the tool needs an artifact package; `S/` is a module crate, so it exits with `ENOENT … 🧪️sim/Cargo.toml` and is not checkable).

| package | before my change | after my change |
| --- | --- | --- |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust` | restored original `V/`: native base 41, onlyCombined 39; test base 191, onlyCombined 41; exit 1 (`pfb-check-applied-fem3d-baseline.log`) | native onlyCombined 39, test onlyCombined 41; exit 1 (`pfb-check-applied-fem3d.log`); no sample diagnostic names a `visual-job` file |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust` | not run on the original (my files are not in its crate layout) | native 2, test 2, exit 1 (`pfb-check-applied-energy-model.log`): inner attribute in `🧵️simulation-session/🦀️.rs:9` and a missing `✏️editor/…/⚡️simulation/🦀️.rs`, both in files I did not touch |

So the requested exit 0 is not reached for either package, for reasons that predate and do not involve this slice. `--check-applied` for the fem package takes about 2 minutes under load 40.

## 6. Foreign findings (not caused by this slice, not fixed here)

1. `E/🦀️.rs` root `adapter()` (sut branch): `subject::inverse(vector.id).expect("valid retained mutation inverse fixture")` calls `.expect` on an `impl Fn`. This is E0599 in the original and in the split tree, so the `sut` feature of this case does not compile at the current tree. Either `subject::inverse` should return `Result<impl Fn…, _>` or the `.expect` is stale. I kept the line verbatim.
2. Python adapter baseline: 28 errored scenarios are 7 kinds x 4 scenarios whose `VECTOR_ROOTS` rows (and the feature `Examples` rows) name fixture folders that do not exist (`🧬️mutations/🅱️change-infiltration-temperature-term-coefficient/…`, same for `🆑️change-infiltration-…`, `💠️change-mechanical-…`, `🎈️change-mechanical-…`, `☔️change-humidistat-…`, `🧍️change-ideal-loads-…`, `🔳️change-ideal-loads-…`); the Rust table uses the real shorter folder names (`🅱️change-infiltration`, …) and the case-contract rule `missing-fixture` should flag them. Two more baseline failures: `mutate-bind-weather-file-refuses-a-bad-uri` (the oracle reports the error path `['']`, the committed outcome says `['not a uri']`) and `identity-round-trip` (`'artifact-id' is not the child target`).
3. `lib-wiring-budget` test: `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧰️testing/🦀️.rs` has 152 lines, over the 150 budget, with no pin (a peer's root, appeared after my last look at the ratchet).
4. `--check-applied` red packages: section 5.
5. `exhaustive-cache-inputs` suite cannot run under bare `bun test` (needs the owned script envelope).

## 7. Hand-offs

### 7.1 Real checks for the compile queue

Use the fleet mutex of the wave-5 brief; one launcher call each. Framework-workspace members would need `--manifest-path 🧰️framework/Cargo.toml`; these crates belong to the plugin workspaces:

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-plugin-energy-engine-sim -p semio-s-plugin-energy-engine-sim-restore -p semio-s-plugin-energy-engine-bestest --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/🔋️energy/Cargo.toml
```

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-artifact-fem-3d-editor-visual-job -p semio-s-artifact-fem-3d-editor-session -p semio-s-artifact-fem-3d-editor-model-window -p semio-s-artifact-fem-3d-viewer --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml
```

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-artifact-energy-model-session --lib --tests --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml
```

The energy mutation case (oracle role, then subject role once finding 1 is resolved) runs through the test platform: `cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test && bun ./📜️script.ts cases ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any exhaustive --case mutate-energy-model-1` (or the owner's Nx `test` target). Expect, for the Python oracle, the 30 baseline non-passes of section 6 and nothing else.

Also run once the compile window opens: `cargo test -p semio-s-plugin-energy-engine-sim` (the unit-test mount uses private items of the root through `use super::*`; `privacy.ts` covers its identifiers, rustc does not yet).

### 7.2 Follow-ups

- One shared manifest (`id`, `kind`, fixture folder) for the Rust index, the Python `VECTOR_ROOTS` and the feature `Examples` would remove the triple 594-row duplication. It needs the Rust side to read fixtures through `ctx.input_bytes(<uri>)` like the Python side instead of `include_str!`, which is a behavior change to validate by a real run; not done.
- The energy `S/` test closure is above the census target (3,871 active lines) because of the 800-line unit-test mount; splitting `🧪️tests/🔬️unit/🦀️.rs` would bring it under 3,500.
- If the compile queue finds an E0616/E0624/E0603 in `S/`, widen the named item to `pub(super)`; do not qualify paths.

### 7.3 Contract lint of the case

Re-run `cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test && bun ./📜️script.ts contract --owner-root ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any` when the machine is quiet; the expected result is no `unknown-case-child` for the ten new folders (the `missing-fixture` breaches of finding 2 of section 6 are baseline).
