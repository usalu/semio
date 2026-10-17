# Result CENSUS4-PKG-49 (prefix `c4p49`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `R` = `🧰️framework/🔨️modules`. Tools: `F/c4p49-tools/` (descendant of `c4p44-tools`). Logs: `F/🗑️generated/c4p49-*` (nothing deleted). Sandboxes and stage trees: `/Users/ueli/.cache/c4p49/`.

## 0. Verdict

**DONE** (static; real compile queued). All seven files are split into 55 cohesive children in 55 new folders (39 `#[path]` modules for five lib roots, 16 `include!` fragments for the two flat test files `tml` and `tmu`; largest child 322 lines, roots 24 to 125 lines), installed 15:27 and re-checked on the live tree: losslessness exact 7 of 7, test-fn multisets identical (73 = 73 and 31 = 31), region pairs balanced per file, rustfmt parse clean, taxonomy 55 of 55 resolved, RGI 55 of 55, every folder mounted exactly once, sibling emoji unique, longest path 141 UTF-8 bytes (129 absolute characters), census 0 over the gate (lib and test, 7 crates), scoped patched differential **identical** (25 of 25 comparisons: lib, `--test`, `testkit`, wasm32-unknown-unknown lib and test; 0 differences, warnings identical), full-crate typeck oracle with 0 privacy, 0 trait-scope rows before and after and equal row totals (only stub-trait emulation rows differ, section 5). Two source-text readers were re-pointed (section 3). No `COMPILE ADMITTED` line in `📓️FLEET.md` (checked 15:34), so the launcher check is queued (section 8).

## 1. Found state (fresh slice)

- No earlier c4p49 work: no `c4p49-tools`, no `🗑️generated/c4p49-*`, no RESULT file.
- Snapshots: `F/c4p49-tools/orig/<tag>.rs`, hashes in `F/c4p49-tools/orig/SHA256`. Re-measured line counts match the census except `tmu` (1,174 to 1,171: a peer removed three `[DEBUG]` `eprintln!` lines, staged in the index; part of the snapshot).

| tag | file (tail under `R/`) | lines | SHA256 (first 16) | owning crate | mount |
| --- | --- | ---: | --- | --- | --- |
| tml | `🗺️surface/🗺️tiled-map/🧪️tests/🔬️unit/🦀️.rs` | 1,001 | `01d21194b766b6a6` | `semio-framework-surface-tiled-map` | `#[cfg(test)] mod tests` of the lib root `🗺️tiled-map/🦀️.rs` |
| act | `🛂️manifest/🕹️actions/🦀️.rs` | 1,132 | `f929be02d47b18c4` | `semio-framework-manifest-actions` | the crate lib root |
| arg | `🛂️manifest/🧮️arg-schema/🦀️.rs` | 1,177 | `05103dff4cde565c` | `semio-framework-manifest-arg-schema` | the crate lib root |
| app | `🛂️manifest/🧱️app-surface/🦀️.rs` | 1,207 | `c99541f65c7f2870` | `semio-framework-manifest-app-surface` | the crate lib root |
| tmu | `🛠️tool-machine/🧪️tests/🔬️unit/🦀️.rs` | 1,171 | `3403eb2b6c5f4ec5` | `semio-framework-tool-machine-tests` | `mod tests` of `🧪️tests/🥣️companion/🦀️.rs` |
| cfg | `🧮️math/🎯️sampling/🎚️config/🦀️.rs` | 1,092 | `aeb31b0e261b5063` | `semio-framework-math-sampling-config` | the crate lib root |
| voc | `🧮️math/🎯️sampling/🔤️vocabulary/🦀️.rs` | 1,377 | `79605fc26e2b53bb` | `semio-framework-math-sampling-vocabulary` | the crate lib root |

Baseline census rows (`cgc7-crate-census.csv` and the pre run of `census-own.ts`, which agree; lib / test closure; files, bytes, lines, ratio): actions 2/68,418/1,165 (0.291) both; arg-schema 1/64,795/1,176 lib, 3/78,305/1,236 test; app-surface 1/62,008/1,183 lib, 10/152,745/2,434 (0.609) test; tool-machine-tests test 4/88,686/1,716 (0.429); sampling-config 1/50,478/1,091 lib, 2/62,416/1,343 test; sampling-vocabulary 1/51,129/1,376 lib, 2/72,657/1,943 test; tiled-map 6/103,812/2,139 lib, 10/176,095/3,252 (0.813) test.

## 2. What was cut

Five lib roots are cut into `<emoji>️<kebab>/🦀️.rs` children wired in the root with `#[path = "<folder>/🦀️.rs"] mod <snake>;` plus `pub use <snake>::*;` (or a private `use` where the child holds only `pub(super)` items that siblings reach through `use super::*`), each wiring placed at the first item the child took (`wiring='each'`), so the root keeps its original `Manifest` region skeleton, its imports, its `#[path]` test mounts and (for `voc`) `#![allow(async_fn_in_trait)]`. Each child starts with an `//!` docstring whose first character is the folder emoji, then `use super::*;` (omitted for `voc` `🆔️ids`, which uses nothing of the parent). The two test files `tml` and `tmu` are flat test modules whose test paths (`tests::<test>`) must not change, so they use `include!` fragments (the mechanism of PKG-28/39/44, `F/c4p49-tools/frag/`), each fragment starting with a `///` docstring with its unique emoji; the root keeps the imports, `#![allow(unexpected_cfgs)]` (`tmu`) and the `include!` lines.

| tag | File | Before | After: root + children (max child) | Children (lines) |
| --- | --- | ---: | --- | --- |
| act | `🛂️manifest/🕹️actions` | 1,132 | 67 + 8 (199) | `🧰️utilities-commands-and-tools` 199, `🗂️action-kind-and-capability-vocabulary` 182, `🎯️audience-and-execution` 105, `🧬️action-semantics` 113, `📇️action-definition` 168, `⏪️history-and-host-event-actions` 177, `🕹️interaction-verbs` 134, `🗃️utility-tool-and-document-actions` 74 |
| arg | `🛂️manifest/🧮️arg-schema` | 1,177 | 61 + 7 (277) | `📐️arg-format-and-schema` 227, `🎨️presentation-sources-and-reference-ids` 114, `🎛️action-arg-control` 238, `🧾️action-arg-definition` 277, `📜️format-application-and-json-schema` 150, `🔢️number-facets` 113, `🪪️app-role` 73 |
| app | `🛂️manifest/🧱️app-surface` | 1,207 | 125 + 8 (201) | `🗨️dialogs` 201, `🎛️modes-and-non-empty-vec` 166, `🪟️window-kinds-and-panels` 156, `🧬️app-definition` 155, `🧭️layout-and-action-resolution` 157, `📚️breadcrumbs-examples-and-command-grammar` 144, `🧩️reserved-contributions` 91, `🕹️interaction-window-bodies` 89 |
| cfg | `🧮️math/🎯️sampling/🎚️config` | 1,092 | 35 + 7 (238) | `⏱️schedules` 191, `🎛️sampling-method-and-processor-specs` 238, `⚙️sampling-config` 216, `🧾️enum-json-and-spec-validation` 124, `🧬️processor-spec-json` 196, `🏗️sampling-config-builder` 99, `📊️step-results` 50 |
| voc | `🧮️math/🎯️sampling/🔤️vocabulary` | 1,377 | 45 + 9 (250) | `🆔️ids` 82, `⚠️errors-fallbacks-and-limits` 194, `🧾️json-value-and-parser` 250, `📤️json-writer-and-utf8` 132, `🧮️numerics` 179, `🎯️selection-and-cumulative-distribution` 92, `🧱️token-bitset` 127, `🎲️random-streams` 221, `📖️vocabulary-and-text-adapters` 122 |
| tml | `🗺️surface/🗺️tiled-map/🧪️tests/🔬️unit` | 1,001 | 24 + 9 (192) | `🧫️scene-fixtures` 66, `📷️camera-and-visible-tiles` 122, `🎯️feature-picking-and-overlays` 133, `🎨️render-scene-and-vector-cache` 192, `🧱️tile-retention-and-polygon-queries` 147, `🧭️interaction-sync-and-focus` 76, `⬆️tile-uploads-and-prefetch` 141, `🎚️theme-lod-and-label-guards` 60, `🌍️mercator-oracle-and-tile-budget` 67 |
| tmu | `🛠️tool-machine/🧪️tests/🔬️unit` | 1,171 | 28 + 7 (322) | `📄️fixture-readers` 62, `🧮️reducer-and-id-laws` 120, `🌊️gesture-tool-laws` 322, `🏃️runner-laws` 107, `🫧️scrub-laws` 239, `⌨️typing-laws` 239, `🕸️node-drag-and-once-laws` 75 |

Totals: 8,157 lines in 7 files became 385 lines in 7 roots plus 55 children. Children sit next to their root as siblings, e.g. `R/🛂️manifest/🕹️actions/📇️action-definition/🦀️.rs`. No new crate, test target or command was added, so no manifest, build-owners row or launch route changed.

Specific handling (each verified in the staged and live text):
- **Regions.** A region that spans several children gets a balanced marker pair in each child that holds a part (original pairs per file `tml, act, arg, app, tmu, cfg, voc`: 1, 11, 9, 15, 8, 3, 10; all balanced in every new file, root and child alike). The fragments of `tml`/`tmu` are cut at region boundaries, so every fragment keeps whole regions.
- **Widening.** Lexical, name-based `pub(super)` insertions for private items, fields and inherent members whose name occurs in another file of the split or in a mounted test or testkit file (`widen2.py`, conservative): 0 (act), 1 (arg: `arg_schema_json_schema`), 1 (app: `NonEmptyVec.first`), 8 (cfg: five `*_json`/validation fns, two processor-spec json fns, `SamplingConfigBuilder.config`), 6 (voc: `KahanSum.sum`, `TokenBitset.words`/`len`, `CounterRng.key`, the `XoshiroSource` tuple field, `SliceTextAdapter.tokens`), 0 (fragments: `include!` keeps the module identity). No qualified path was used to fix visibility. Cross-file private declaration scan on the live files: 0 leftovers (0, 2, 6, 5, 14 private declarations scanned for act, arg, app, cfg, voc).
- **`dyn_enum` pairs (voc).** `#[dyn_enum] trait RandomSource` and its `dyn_enum_close!` stay together in `🎲️random-streams`, and `#[dyn_enum] trait TokenTextAdapter` with its `dyn_enum_close!` stays together in `📖️vocabulary-and-text-adapters`: the attribute emits a `macro_rules!` that the closing macro invokes by bare name through textual scope, so trait and closing site must share one module with the trait first (`🔀️dispatch/🦀️.rs` documents this).
- **Root mounts kept.** `act` keeps `#[path = "../🪛️utilities/🌅️initial/🦀️.rs"] mod initial_window_utility;` with its `pub use` inside the `Utilities` region; `arg` keeps its `#[cfg(test)]` number-facets test mount; `app` keeps its `testkit` mount, the `#[cfg(test)]` imports and the four test mounts; `cfg` and `voc` keep their `#[cfg(test)] mod tests;`. The test modules reach moved items through the root's glob re-exports; items they use that were private are widened as above.
- **`tml` root fixture literal.** The only relative `include_str!` of the package (`MERCATOR_ORACLE_FIXTURE`) got one more `../` (`"../🧫️fixtures/…"` to `"../../🧫️fixtures/…"`); its absolute target is unchanged (`F/c4p49-tools/frag/incl.py`).

## 3. Source-text readers

`git grep -F` over the repo (excluding tickets) for each file path, each crate folder and the `🎯️sampling` family, over all file types and over `*.ts *.tsx *.mjs *.py *.sh *.rs *.md`; plus the module-tree consumers (`policyReadRustSourceEvidence` and `POLICY_SPLIT_SOURCE_ROOTS`).

| reader | before | after |
| --- | --- | --- |
| `R/../🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts` line 165 (`manifest`, two uses: the `for_kind` block and the `bounded_catalog` negative regex) | `policyReadFileSafe(root, ".../🛂️manifest/🕹️actions/🦀️.rs")` | `policyReadModuleTreeSource(root, …)` (import already present) |
| `R/🛂️manifest/🔖️action-args/♻️retirement/🧪️tests/🟦️.ts` line 4 (reads `../../../🧮️arg-schema/🦀️.rs` and `🔡️mutation-inputs/🦀️.rs` and searches `pub struct/enum <owner>` plus its `#[derive(` for `RetireOwned`) | `readFileSync(new URL(path, import.meta.url), "utf8")` | `readModuleTreeSource(new URL(path, import.meta.url))`, plus the import `from"@semio-tech/framework-compiler-ts/syntax/module-tree"` (same form as the sibling `⏯️tool-run/♻️retirement` test) |

Probes and runs: `c4p49-tools/probe-tree.ts` (`F/🗑️generated/c4p49-probe-tree.log`): the actions module-tree text has the `for_kind` start and block length 1,546 characters with no `Migrated` and the `bounded_catalog` regex false, **equal** to the snapshot; the arg-schema derive probe for 12 owners is equal to the snapshot (`ACT READER EQUAL`, `ARG READER EQUAL`). `bun test` of the retirement test file: 2 pass, 0 fail (`c4p49-retirement-test.log`).

Not readers: `taxonomy.json` (folder names only; the new folders resolve, section 4), `📋️build-owners.json`/`🔒️dependencies.json`/`Cargo.toml` (crate roots only), `🔖️action-args` fixtures. No reader of `tml`, `tmu`, `app`, `cfg` or `voc` text exists (the `tiled-map` root itself is read by three consumers through `policyReadModuleTreeSource`, which excludes canonical `🧪️tests` mounts and is unchanged). No other crate mounts one of the seven files with `#[path]`. No compatibility re-export: roots hold `#[path] mod x; <vis> use x::*;` wiring and genuinely own items only.

## 4. Verification (commands, real results)

Tools in `F/c4p49-tools/`, logs in `F/🗑️generated/c4p49-*`; all long runs used `ulimit -n 65536` and `nice -n 10`.

| Check | Command | Result |
| --- | --- | --- |
| Install precondition and apply | `python3 c4p49.py install <tag>` (five tags), `cd frag && python3 install.py check; apply` (`c4p49-install.log`) | each live file equalled its snapshot (cmp, 7 of 7 at 15:2x); `installed act arg app cfg voc`; frag `children 16, roots 2, problems 0`, `installed 18 files` |
| Lossless (live, five module files) | `python3 live.py act arg app cfg voc` (`c4p49-live.log`) | 5 of 5 `bytes-equal-to-stage` and `LOSSLESS OK`: missing 0; every extra line is a docstring, `use super::*;`, `#[path]`/`mod`/`pub use` wiring or a region marker |
| Lossless (live, tml/tmu) | `cd frag && python3 lossless.py; python3 verify.py live; python3 incl.py` (`c4p49-frag-live.log`) | `gone=0 extra=0 sequence-equal=True` for both; `VERIFY OK live`: test fns 73 = 73 and 31 = 31 identical, regions balanced, docstring emoji unique, rustfmt parse clean, each fragment included once; the one declared difference is the rebased `MERCATOR_ORACLE_FIXTURE` literal; `INCLUDES OK` |
| Structure | `python3 check.py act arg app cfg voc` (`c4p49-check-live.log`) | `CHECKS OK`: 39 children mounted exactly once, doc emoji equals folder emoji, unique among siblings and different from the root's, region counts equal per file, no `[DEBUG]`, longest child 277 lines |
| Private declarations | `python3 defscan.py <tag> <live dir>` (`c4p49-defscan-live.log`) | 0 cross-file leftovers for all five; `python3 modclash.py act arg app cfg voc`: `modclash OK` (no child module name clashes with an item or import) |
| rustfmt | `rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout` over all 44 module files and 18 fragment files | 0 `error` lines |
| Taxonomy, RGI | `bun tax.ts newdirs.txt`; `bun rgi.mjs newnames.txt`; folder content scan | `dirs 55, unresolved []`; `bad=0`; each of the 55 folders holds exactly one `🦀️.rs`; no sibling emoji duplicates; every emoji appears at least 3 times in the taxonomy |
| Scoped patched differential, staged | `C4P49_NORM=1 python3 oracle.py all all` (`c4p49-oracle-stage1.log`) | 24 comparisons identical; one warning delta (`unused import: super::*` in `voc` `🆔️ids`), fixed by `glob=False` for that child |
| Scoped patched differential, live (final) | `C4P49_LIVE=1 C4P49_NORM=1 python3 oracle.py all all` (`c4p49-oracle-live.log`) | `ORACLE all CLEAN`: 25 of 25 comparisons (tml test and wasmtest; act, arg, cfg, voc lib/test/wasm/wasmtest; app lib/test/testkit/wasm/wasmtest; tmu test), 0 only-orig, 0 only-new, warnings identical; per-crate error counts equal (e.g. act 158, arg 266/281, app 183/413/186, tmu 466, cfg 1/22, voc 8/60, tml 333/406) |
| Census (canonical `readRustSourceClosureV1`) | `bun census-own.ts <repo>` pre and post (`c4p49-census-own-{pre,post}.log`); `zsh census.sh post` (`c4p49-census-post.log`, the generated `census.ts`, which lists four of the seven crates) | 0 of 7 over the gate; table below; `census.sh` rows equal the own rows for actions, app-surface, arg-schema and tiled-map (lib) |
| Wiring budget law | `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` (library module, `c4p49-wiring-budget.log`) | 2 pass, 1 fail; the breaches are plugin crates of peers (`🌊️flow` 789, `🎬️sequence`, `✒️writer`, `🏙️bim`, `🪵️sourcing`, `🖨️raster`, `🌀️generation2d`), none among these crates |

Census (files / bytes / active lines; ratio = worst of files/32, bytes/262,144, lines/4,000; before = pre run, after = live; the gate counts each crate's own source closure, so no dependent crate's row can change):

| crate | lib before | lib after | cfg(test) before | cfg(test) after |
| --- | --- | --- | --- | --- |
| tiled-map (tml) | 6 / 103,812 / 2,139 (0.535) | same (root unchanged) | 10 / 176,095 / 3,252 (0.813) | 19 / 178,426 / 3,288 (0.822) |
| actions (act) | 2 / 68,418 / 1,165 (0.291) | 10 / 72,012 / 1,260 (0.315) | same as lib | 10 / 72,012 / 1,260 (0.315) |
| arg-schema (arg) | 1 / 64,795 / 1,176 (0.294) | 8 / 67,525 / 1,259 (0.315) | 3 / 78,305 / 1,236 (0.309) | 10 / 81,035 / 1,319 (0.330) |
| app-surface (app) | 1 / 62,008 / 1,183 (0.296) | 9 / 65,146 / 1,268 (0.317) | 10 / 152,745 / 2,434 (0.609) | 18 / 155,883 / 2,519 (0.630) |
| tool-machine-tests (tmu) | 1 / 1,654 / 0 (0.031) | same | 4 / 88,686 / 1,716 (0.429) | 11 / 90,126 / 1,744 (0.436) |
| sampling-config (cfg) | 1 / 50,478 / 1,091 (0.273) | 8 / 52,713 / 1,155 (0.289) | 2 / 62,416 / 1,343 (0.336) | 9 / 64,651 / 1,407 (0.352) |
| sampling-vocabulary (voc) | 1 / 51,129 / 1,376 (0.344) | 10 / 53,842 / 1,452 (0.363) | 2 / 72,657 / 1,943 (0.486) | 11 / 75,370 / 2,019 (0.505) |

The highest file count is 19 of 32 (tiled-map test) and the highest ratio is 0.822 (tiled-map test, lines); nothing is near a limit.

## 5. Full-crate typeck oracle (extra evidence)

`posrun.py` (the privacy and type oracle `F/pos-tools/pos.py` with a stub layer for everything outside the family, sandbox `/Users/ueli/.cache/c4p49/pos-work`), family `families/c4p49.json` = the eight crates (the seven owners plus `semio-framework-tool-machine`, no base revision): `pre` ran on the unsplit tree before the install and `post` after it. Commands: `cd F/c4p49-tools && ulimit -n 65536 && PYTHONHASHSEED=0 POS_PREFIX=c4p49 python3 posrun.py run families/c4p49.json --cfgs lib,test,wasm --label {pre,post} --no-base` (`c4p49-pos-pre.log`, `c4p49-pos-post.log`, 2 and 1.5 minutes, two iterations each), then `python3 posdiff.py c4p49-pre c4p49-post lib test wasm` (`c4p49-posdiff-1.log`).

| cfg | errors pre / post | privacy | trait-scope | resolve | type | other | canary |
| --- | --- | --- | --- | --- | --- | --- | --- |
| lib | 1,334 / 1,334 | 0 / 0 | 0 / 0 | 182 / 182 | 30 / 30 | 10 / 10 | `E0308 E0616 E0624` reached on both sides |
| test | 2,745 / 2,745 | 0 / 0 | 0 / 0 | 254 / 254 | 51 / 51 | 17 / 17 | reached |
| wasm | 1,363 / 1,363 | 0 / 0 | 0 / 0 | 186 / 186 | 30 / 30 | 10 / 10 | reached |

So no new `E0603`, `E0616`, `E0624` or unresolved name is introduced and every category total is equal. The row delta (10 rows only in `pre` and 10 only in `post` for lib and wasm, 14 and 14 for test) is stub-layer emulation of four stubbed external traits (`ToValue`, `FromValue`, `Serialize`, `Deserialize`): in `pre` the stub knows them as traits at their `impl … for` sites and reports their stub-signature consequences (`E0185`, `E0277` on `&str: ToValue`, `E0407` naming the trait, `E0599` on `to_value`); in `post` the `impl` sites sit in children that reach the root's `use` lines only through `use super::*`, which the per-file stub scan does not follow, so it types them as the placeholder `Tr` and reports `E0407`/`E0277`/`E0599` at the same sites (children `🗂️action-kind-and-capability-vocabulary`, `🪪️app-role`, `🎛️modes-and-non-empty-vec`, `🧾️action-arg-definition`, `⏪️history-and-host-event-actions`). This is the same class of artefact PKG-37 and PKG-44 recorded. Not covered: borrowck, trait coherence against the real externals, lints, `dyn_enum` expansion (the oracle strips that attribute) and the `wasm32-wasip2` target; the real check in section 8 covers them.

## 6. Foreign errors and observations

- `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts`: breaches only in plugin crates of peers (section 4).
- `git` printed `daemon terminated` on several read-only calls (fsmonitor noise). No `ENFILE`/`EMFILE` or Bun segfault occurred. One long `git grep` loop was moved to the background by the tool timeout and I started a second read-only search loop in the background myself; both finished normally (`c4p49-readers-search.log` keeps the second, whose `head -20` truncated the generic patterns; the narrower repeats are listed in section 3).
- `tmu` differed from `HEAD b84bf526c7b` by a peer edit that is part of the snapshot (three `[DEBUG]` `eprintln!` lines removed); the snapshot is the working-tree file.
- The generated `census.ts` (`ONLY=`) printed rows for four of the seven crates only; the other three (sampling config, sampling vocabulary, tool-machine-tests) are in `census-own.ts`, which uses the same canonical reader and reproduces `cgc7-crate-census.csv` exactly before the split.

## 7. Files

- Edited (7 roots): `R/🗺️surface/🗺️tiled-map/🧪️tests/🔬️unit/🦀️.rs`, `R/🛂️manifest/🕹️actions/🦀️.rs`, `R/🛂️manifest/🧮️arg-schema/🦀️.rs`, `R/🛂️manifest/🧱️app-surface/🦀️.rs`, `R/🛠️tool-machine/🧪️tests/🔬️unit/🦀️.rs`, `R/🧮️math/🎯️sampling/🎚️config/🦀️.rs`, `R/🧮️math/🎯️sampling/🔤️vocabulary/🦀️.rs`.
- Created (55 children, one `🦀️.rs` per new folder): the folders of section 2, next to their roots (listed in `F/c4p49-tools/newdirs.txt`).
- Edited outside the package (readers, section 3): `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts` (1 expression) and `R/🛂️manifest/🔖️action-args/♻️retirement/🧪️tests/🟦️.ts` (1 import, 1 expression).
- Tools, kept: `F/c4p49-tools/` (`c4p49.py` splitter, `specs/{act,arg,app,cfg,voc}.py`, `frag/` fragment pipeline with `specs/{1,2}.json`, `oracle.py`, `live.py`, `check.py`, `defscan.py`, `modclash.py`, `globuse.py`, `census.sh`, `census-own.ts`, `tax.ts`, `rgi.mjs`, `probe-tree.ts`, `posrun.py`, `posdiff.py`, `families/c4p49.json`, `newdirs.txt`, `newnames.txt`, `installed.json`, `orig/` snapshots with `SHA256`). Do not re-stage after install: the specs read the original snapshots, but `install` refuses when the live file differs from them.
- Logs, kept in `F/🗑️generated/`: `c4p49-oracle-{stage1,live}.log`, `c4p49-install.log`, `c4p49-live.log`, `c4p49-check-live.log`, `c4p49-frag-live.log`, `c4p49-defscan-live.log`, `c4p49-probe-tree.log`, `c4p49-retirement-test.log`, `c4p49-wiring-budget.log`, `c4p49-census-own-{pre,post}.log`, `c4p49-census-post.log`, `c4p49-pos-{pre,post}.log`, `c4p49-posdiff-1.log`, `c4p49-readers-search.log` with the `c4p49-c4p49-*` diagnostic dumps.

## 8. Compile queue (not admitted)

No `COMPILE ADMITTED` line in `📓️FLEET.md` at 15:34 and no `bun repo cargo` call was made by this slice. One batched call under the compile mutex of `📓️WAVE5-BRIEF.md` (framework workspace members need the manifest path):

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-surface-tiled-map -p semio-framework-manifest-actions -p semio-framework-manifest-arg-schema -p semio-framework-manifest-app-surface -p semio-framework-tool-machine-tests -p semio-framework-math-sampling-config -p semio-framework-math-sampling-vocabulary --lib --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
```

then `--target wasm32-unknown-unknown` for the same crates except `semio-framework-tool-machine-tests` with `--lib`, and one `--features testkit` lib check of `semio-framework-manifest-app-surface`. If a build shows a missing `pub(super)`, widen that single definition line in the child (never with a qualified path); if the `voc` `dyn_enum_close!` sites report an unresolved `__semio_dispatch_*` macro, the trait and its closing macro are no longer in one module.
