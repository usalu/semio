# Result CENSUS4-PKG-28 (prefix `c4p28`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. Tools: `F/c4p28-tools/`. Logs: `F/🗑️generated/c4p28-*`.

## 0. Verdict

DONE (static evidence, wave 7, 2026-10-17 08:25). All 7 files of package 28 are split into 35 cohesive children (largest 697 lines), installed on disk, lossless, with the source-text readers re-pointed and no compatibility re-export. The scoped patched differential equals the pre-edit tree for the pure split (puzzle, block and sourcing families; lib, test, wasm32, wasip2); the only delta is the intended removal of 22 baseline privacy rows through 8 `pub` fixes (section 1), which unmask 14 stub-layer rows. The three `--check-applied` plugin gates exit 0 with `onlyCombined` 0. No real compile was run: no `COMPILE ADMITTED` line in `📓️FLEET.md` (checked 08:24), the command for the compile queue is in section 6.

## 1. Scope and result (`MONOLITH-CENSUS-4.md` section 4, package 28)

`P` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any`, `B` = `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any`, `S` = `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any`. Line counts are file lines (trailing newline excluded).

| # | Root | Before | After: root + children (max child) | Mechanism | Children |
|---|---|---:|---|---|---|
| 1 | `P/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 2,241 | 128 + 6 (584) | `include!` fragments (test names and scope unchanged) | `🕹️roster-and-read-laws` 216, `🧵️cursorized-route-laws` 241, `🧮️extent-and-step-laws` 292, `🎛️dispatch-and-operation-laws` 410, `🧾️inspector-and-manifest-laws` 387, `💡️suggestion-and-scene-laws` 584 |
| 2 | `P/✏️editor/🧪️tests/🧰️context/🦀️.rs` | 1,190 | 12 + 4 (360) | `include!` fragments (module `context` keeps one scope) | `🏗️raw-app-fixture` 360, `🎛️settle-and-dispatch-drivers` 299, `🖼️render-and-scene-probes` 264, `📏️measure-and-tool-run-probes` 267 |
| 3 | `P/✏️editor/🧵️retained/🎬️action/🦀️.rs` (lib root of `semio-s-artifact-puzzle-3d-retained-action`) | 1,493 | 121 + 6 (292) | `#[path] mod` + `pub use self::x::*` | one child per retained work: `🪟️window-command-work` 179, `🔄️transform-work` 194, `🖌️brush-object-work` 286, `🎯️focus-selection-work` 175, `💡️accept-suggestion-work` 288, `🧠️precompute-command-work` 292 |
| 4 | `P/✏️editor/🧵️retained/🗂️document/🦀️.rs` (lib root of `...-retained-document`) | 1,536 | 51 + 5 (435) | same | `⚖️kind-weight-work` 249, `➕️add-object-kind-work` 266, `🔍️patch-inspector-work` 435, `💞️create-attraction-work` 233, `📚️set-active-example-work` 337 |
| 5 | `P/🧬️schema/🦀️.rs` (lib root of `semio-s-artifact-puzzle-3d-model`) | 1,891 | 78 + 5 (697) | same | `🗿️artifact-identity` 135, `🧱️precompute-catalogs` 365, `🎬️precompute-commands` 411, `📚️document-model` 697, `📐️pose-math` 243 |
| 6 | `B/✏️editor/🦀️.rs` (lib root of `semio-s-artifact-block-3d-editor`) | 1,121 | 243 + 4 (337) | same | `🧰️retained-commands` 222, `📬️store-preparation` 216, `🧩️play-app` 337, `📋️manifest` 131 |
| 7 | `S/✏️editor/🦀️.rs` (module `editor::sourcing::component` of `semio-s-artifact-sourcing-curation`) | 1,248 | 341 + 5 (224) | same | `🧰️retained-commands` 175, `📬️config-store-preparation` 196, `🗄️artifact-store-preparation` 159, `🧩️sourcing-app` 224, `📋️manifest` 184 |

Totals: 7 roots rewritten, 35 children in 35 new folders, largest child 697 lines (limit about 1,000). Every new file starts with a unique docstring emoji (`//!` for module children, `///`/`//` for include fragments, as in `RESULT-CENSUS4-PKG-5.md`), no comment inside a definition, no `[DEBUG]` marker, longest repo-relative path of the 42 files 212 bytes (limit 240). Every `//#region` / `//#endregion` marker of the originals is kept; the two regions that now span two files are re-opened and re-closed in each child (`PrecomputeModel` in root 5; the unclosed `// #region 🔖️Document` of root 5 is now closed at the end of `📚️document-model`).

Decisions per file:
- Roots 1 and 2 are flat test targets, so the cut is by `include!` fragments (the module identity of every item and the test paths are unchanged, nothing is widened). The `context` module keeps its single scope, which the four test targets (`unit`, `panels`, `budgets`, `gestures`) mount with `#[path]`.
- Roots 3 to 7 are cut into modules. Items another child, the root or a descendant names are widened by `widen.ts` (name-based, `pub(super)`), exactly as in wave 5. Roots 3, 4, 5 are now below the 150-line wiring budget (121, 51, 78 lines).
- Root 5 keeps `SCHEMA_LEAVES` (`include_str!("🦀️.rs")`, which must stay beside the schema leaves), the `precompute_model_tests` mount and the inline `io` / `snapshot` module tree.
- Root 7 keeps the surface constants, the io, the command enum, `SourcingCurationApp`, `reset_document_effect` and the test and example mounts.

### Intended visibility fixes (8 lines, pre-existing defects of the earlier crate carve, found by the oracle baseline)

The baseline oracle (before any edit) already reported 22 `E0624` rows (`new` 12, `bound` 10) in the puzzle editor's `🧩️play-app`, and the E0451 scan reported 1 candidate site (`Puzzle3dInstanceBinding { ... }` literal): the editor crate calls members of the retained crates that were private. They are inside my files, so they are fixed as part of "visibility correct" (`replace` entries in `c4p28-tools/specs/{3,4}.json`, listed by modsplit as the only `LOST` lines):
- retained-action `🪟️window-command-work`: `Puzzle3dInstanceBinding` fields `app_instance_id`, `parent_document_id`, `owner` -> `pub`; `Puzzle3dWindowCommandWork::{new, with_tool_run, bound}` -> `pub`.
- retained-action `🔄️transform-work`: `Puzzle3dTransformWork::new` -> `pub`; `💡️accept-suggestion-work`: `Puzzle3dAcceptSuggestionWork::bound` -> `pub`; `🧠️precompute-command-work`: `Puzzle3dPrecomputeCommandWork::{new, bound}` -> `pub`.
- retained-document `⚖️kind-weight-work`: `Puzzle3dKindWeightWork::new` -> `pub`.

## 2. Found state

No `c4p28-*` file, tool folder or log existed at the first start (fresh slice). Files 1, 2, 6, 7 carried staged (index) modifications by peers (a re-point in the unit and context files from `c4p5`, 3-line edits in 6 and 7), all treated as the baseline.

### Found state (wave 7, 07:28)

The process restart cut the executor before any edit. On disk: `c4p28-tools/` (generic tools copied from `c4p5-tools`, `specs/` empty, no `c4p28.json`), `🗑️generated/c4p28-orig/{1..7}.rs` + `.path` (pre-edit snapshots, taken 07:01). Compared with `cmp`: all 7 live files were byte-identical to their snapshots (2241, 1190, 1493, 1536, 1891, 1121, 1248 lines). No stage dir, no installer journal, no `*.c4p28-tmp`, no children. Nothing half-applied, nothing to roll back; the slice started from the snapshots and the rest of this file is the wave 7 work (07:28 to 08:15).

## 3. Evidence (all commands run in this session)

| Check | Command (tools in `F/c4p28-tools/`) | Result |
|---|---|---|
| Losslessness (non-blank line multiset, test-fn multiset, region balance, emoji, path length, brace depth, rustfmt parse) | `python3 verify.py live` (`c4p28-verify-live.log`) | `VERIFY OK live`: 0 lines removed in 5 of 7 files; 7 lines (root 3 children) and 1 line (root 4) changed only by the `pub` fixes above; test fns 93 -> 93 (unit), 0 elsewhere; the added lines are mounts, docstrings, region re-openings and the 8 `pub` lines |
| Installed bytes = staged bytes | `cmp` loop over the 42 files | 42 compared, 0 differ |
| Mounts: every `#[path]`/`include!` target exists, each new file reached once, no new duplicate mount | `python3 mounts.py live` | `MOUNTS OK (0 new duplicates, all targets exist)`, 7/7 |
| Public API | `python3 apicheck.py live` | `APICHECK OK`: pub top-level items 10 -> 10, 6 -> 6, 83 -> 83, 16 -> 16, 21 -> 21 (roots 3 to 7), every child with a `pub` item is re-exported with `pub use` |
| Definition scan (private declaration named in another file of the tree must carry a visibility) | `python3 defscan.py live` | `DEFSCAN OK`, 0 leftovers |
| rustfmt | `python3 fmtdebt.py` | `--check` hunks original 156 -> 155 (no new formatting debt) |
| Typeck/privacy oracle, puzzle family (model, retained-action, retained-document, editor), lib and test | `runpos.sh pre` before the edit, `runpos.sh post <stage>` after (stage = installed bytes), `python3 posdiff.py c4p28p-prep c4p28p-postp lib test` | canary reached (`E0308, E0616, E0624`) in all runs. **Pure split: IDENTICAL.** The run `novis` (same split with the 8 `pub` fixes left out, `c4p28-pos-novisp.log`) gives `POSDIFF IDENTICAL` against the pre-edit tree for lib (2710 = 2710) and test (2937 = 2937). **With the fixes:** -22 rows (the baseline E0624 `new` 12 and `bound` 10) and +14 rows (E0277 `Puzzle3dPlayApp: Puzzle3dEditor` not satisfied x12 and `(): ArtifactCommandWork` x2, all in the editor's `🧩️play-app` lines 154 to 200): they were hidden behind the private-call errors and are the stub layer's view of the `Puzzle3dEditor` blanket impl and the external `semio_s_plugin_puzzle_retained` types, not caused by the split |
| Same oracle, wasm32-unknown-unknown and wasm32-wasip2 | `CFGS=wasm,p2 runpos.sh prew` / `postw`, `posdiff.py` | puzzle family: the same -22/+14 and nothing else (wasm 2774 -> 2766, p2 2710 -> 2702); block and sourcing families IDENTICAL |
| Typeck oracle, block 3d editor | same, family `b` | lib 767 = 767, test 1162 = 1162, wasm and p2 767 = 767: `POSDIFF IDENTICAL`, canary reached in every run |
| Typeck oracle, sourcing curation | same, family `s` | lib 2349 = 2349, wasm and p2 2349 = 2349: `POSDIFF IDENTICAL`, canary reached. The `test` configuration cannot reach typeck at baseline (1 row = 1 row): the crate mounts a non-existing file `🪶️sqlite/📸️snapshot/🧪️tests/💰️backing/🦀️.rs` (rustc stops at "couldn't find file"); a sandbox shim for that file was tried (`c4p28-pos-preshim.log`) and is not honoured by the stub builder, so the sourcing tests are covered by the lib run, the definition scan and the widening of descendants only |
| E0451 literal scan (blind spot of the oracle) | `POS_PREFIX=rpv python3 rpv-tools/lit.py c4p28-tools/c4p28.json c4p28fin` | baseline (`c4p28-lit-pre.log`): 1 candidate site (`Puzzle3dInstanceBinding`); after: **0 candidate sites** (136 brace structs, 140 definitions, 164 files) |
| Census (source boundary, `measureRustCrate`) | `bun census2.ts <7 roots>` (`c4p28-census-after.log`) | all inside the hard gate: unit 15 files / 248,230 bytes / 3,550 lines, context 5 / 71,575 / 1,207, retained-action 7 / 98,186 / 1,542, retained-document 6 / 93,042 / 1,577, schema 21 / 221,620 / 2,706, block editor 7 / 94,512 / 1,373, sourcing editor 8 / 94,474 / 1,296 |
| Taxonomy resolution of the 42 directories | `bun tax.ts c4p28-newdirs.txt` | 0 unresolved: children of the test roots -> `members-of-tests`, all others -> `crate-module-children` |
| RGI check of the 35 new folder names | `node rgi.mjs` | `bad=0` |
| Source-text readers (`probe.ts`: tool-job inventory, interactivity audit of the fill tool, completion oracle) | `bun probe.ts` before (`c4p28-probe-before.json`) and after (`c4p28-probe-after.json`) the install, `diff` | identical: `failuresTotal` 1312 = 1312, `failuresHash` 2980433518510787621 = 2980433518510787621, proofs 640 = 640, all `interactivity` arrays equal, 185 puzzle 3d rows equal. The only differences: `sources` 16,997 -> 17,020 (peer slices installed meanwhile) and a tail offset of the `computeOwnership` environment error string |
| Block publication-authority oracle over the new block editor tree | `bun blockauth.ts orig` / `tree` (the script itself needs a parent Script process invocation: `Original parent Script process invocation required`) | identical for the original text and the module tree: 22 ids = 22 contracts = 22 classifications = 22 routes, the same hashes, the same single missing anchor `base.return_to_registry()` (the audit rejects production at baseline, not caused by the split) |
| Lib-wiring ratchet law | `bun test .../📏️lib-wiring-budget/🟦️.ts` (`c4p28-ratchet-final.log`) | 2 pass (including "ratchet lists no root that complies"), 1 fail: only `🔋️energy/.../🧬️schema/🦀️.rs: grew from 165 to 214 lines` (peer slice). None of my roots |
| Composition-admission caller corpus | `bun test .../📥️admission/🧪️tests/🟦️.ts -t "all declared canonical projection callers"` and a python re-check of all 87 rows | my row (the sourcing editor, now the `🧩️sourcing-app` child) passes; 7 other rows fail (foreign, section 5) |
| `--check-applied` layout gate | `checkapplied.sh after puzzle:... block:... sourcing:...` (logs `c4p28-checkapplied-<tag>-after.log`) | see section 3b |

### 3b. Layout gates (`bun ./📜️script.ts new crates <artifact>/📦️packages/🦀️rust --check-applied --base=f856dff0813`)

Run in parallel with `LC_ALL=en_US.UTF-8`, `ulimit -n 65536`, sandboxes in the session scratchpad, logs `c4p28-checkapplied-{puzzle,block,sourcing}-after.log`; all three `exit 0`, `admission` `[]`, `shared` `[]`, `sharedForeign` `[]`:

| Package | crates / base | native base -> combined | `--test` base -> combined | `onlyCombined` |
|---|---|---|---|---|
| puzzle 3d (`🧩️puzzle/🗿️artifacts/🧊️3d`) | 34 / 34 | 4063 -> 4063 | 4904 -> 4904 | 0 / 0 (same numbers as the `c4p5` run) |
| block 3d (`🧱️block/🗿️artifacts/🧊️3d`) | 22 / 22 | 2245 -> 2250 | 3101 -> 3102 | 0 / 0 |
| sourcing curation (`🪵️sourcing/🗿️artifacts/🗂️curation`) | 0 / 0 | 1440 -> 1445 | 2286 -> 2286 | 0 / 0 |

The small count differences for block and sourcing (+5, +1, +5) are multiplicities of diagnostics that exist on both sides (`onlyCombined` 0): the base revision `f856dff0813` predates the whole fleet's edits to these trees. `siblings` lists `semio-framework-test-original-snapshot-owners` for block and sourcing and `healedBase` the sourcing unit test file, as reported by the tool.

## 4. Non-Rust edits (log `c4p28-edits.log`)

- `🧰️framework/.../📚️library/🔍️discovery/📖️source-access/🟦️.ts`: `POLICY_SPLIT_SOURCE_ROOTS` gets six roots (puzzle 3d unit test root, retained-action, retained-document, schema, block 3d editor, sourcing editor), so the tool-job inventory and the interactivity audit keep reading each root as the one text the unsplit file held (without it the children would be scanned as separate production sources).
- `✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📜️script.ts`: the publication-authority audit reads the block 3d editor through `readModuleTreeSource`.
- `🧰️framework/🔨️modules/🧬️schema/🧩️composition/📥️admission/🧪️tests/📋️callers.json`: the caller row of the sourcing editor now names `🧩️sourcing-app/🦀️.rs`, the child that holds `semio_framework_composition_admission::ChildRestoreProjection`.
- `🧰️framework/.../📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json`: pins of roots 3, 4, 5 removed (they comply), pin of the block 3d editor root 1122 -> 244.
- `c4p28-tools/`: copied from `c4p5-tools` and adapted (`mkspecs.py`, `modsplit.ts` with a `replace` option, `defscan.py`, `edits.py`, `runpos.sh`, `checkapplied.sh`, `blockauth.ts`, `posdiff.py` with family labels).

## 5. Remaining reds, all attributed (none caused by the split)

- Foreign (probably peer splits, same cause as the one I fixed for the sourcing editor; not verified against their baselines): the composition-admission caller corpus lists 7 further files whose text does not contain `semio_framework_composition_admission::`: `🌀️procedural/.../🌀️generation2d/.../✏️editor/🦀️.rs`, `🎬️sequence/.../✏️editor/🦀️.rs`, `🏭️process/.../🧊️process3d/.../✏️editor/🦀️.rs`, `📖️playbook/.../🦀️.rs`, `📜️imperative/.../📜️procedure/🦀️.rs`, `🔌️plugin/🧵️instance/📨️dispatch/🦀️.rs` and a missing `🔌️plugin/🧵️instance/🚪️ingress/🦀️.rs`. Their owners need to re-point the rows to the child that holds the call.
- Foreign: `lib-wiring-budget` breach of the `energy` schema root (165 -> 214).
- Baseline defects not touched: the block 3d publication-authority audit misses the anchor `base.return_to_registry()` (also in the original file); the sourcing curation crate mounts a missing test file; `computeOwnership` of the puzzle 3d probe exits 1 before and after.
- Oracle caveat: the `+14` E0277 stub rows described in section 3 are the unmasked stub-layer view of `Puzzle3dEditor` (a blanket impl over `ArtifactEditor` with associated-type equalities) and of the unlinked `semio_s_plugin_puzzle_retained`; the real compile decides.

## 6. Queued real check (no `COMPILE ADMITTED` line in `📓️FLEET.md` at 08:13; the mutex `🗑️generated/cargo-mutex.lock` is held by `coord-probe2`)

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-artifact-puzzle-3d-model -p semio-s-artifact-puzzle-3d-retained-action -p semio-s-artifact-puzzle-3d-retained-document -p semio-s-artifact-puzzle-3d-editor -p semio-s-artifact-block-3d-editor -p semio-s-artifact-sourcing-curation --lib --tests --keep-going --message-format=short
```

Also run it once with `--target wasm32-wasip2` for the wasm-built plugin crates. Expected attribution if it reports errors in the puzzle editor `🧩️play-app`: lines 154 to 200 are the unmasked callers of the retained works (section 3).

## 7. Files

Created: the 35 child files `🦀️.rs` in the 35 folders of section 1 (all under `P`, `B` or `S`). Rewritten: the 7 roots of section 1 and the 4 non-Rust files of section 4. Ticket-side: `F/c4p28-tools/` (tools and specs, kept), `F/🗑️generated/c4p28-*` (logs, snapshots `c4p28-orig/`, stage dirs `c4p28-stage*`, kept), this file.
