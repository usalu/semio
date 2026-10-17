# Result CENSUS4-PKG-6 (slice `c4p6`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/c4p6-tools/`. Logs: `F/🗑️generated/c4p6-*`.

## 0. Verdict

**DONE (static evidence; no cargo run: the compile launcher is not admitted yet).** All seven files are split and installed (25 children in 25 new folders). Wave 7 closed the open item: both `--check-applied` layout gates pass (5d exit 0, 3d complete), `onlyCombined` 0 in native and `--test` for both, and the live tree is lossless 7 of 7. No defect of the split was found, so no source edit was made in wave 7. Real-check commands are in section 6 (`COMPILE ADMITTED` is not in `📓️FLEET.md` as of 07:41; the coordinator's probe 2 holds the mutex).

| # | Target (tail under `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/`) | Lines before | Root after | Children | Largest child |
|---|---|---:|---:|---:|---:|
| 1 | `🖐️5d/…/✏️editor/👆️gestures/🦀️.rs` | 1,932 | 25 | 4 | 654 |
| 2 | `🖐️5d/…/✏️editor/🖌️session/🦀️.rs` | 1,455 | 78 | 3 | 700 |
| 3 | `🖐️5d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 2,797 | 541 | 5 | 561 |
| 4 | `🖐️5d/…/✏️editor/🧰️kit/🦀️.rs` | 1,995 | 29 | 4 | 766 |
| 5 | `🖐️5d/🦀️.rs` (crate root) | 2,505 | 1,417 | 2 | 747 |
| 6 | `🧊️3d/…/⏳️precompute/📐️geometry/🦀️.rs` | 2,241 | 105 | 4 | 875 |
| 7 | `🧊️3d/…/⏳️precompute/🖌️brush/🦀️.rs` | 1,151 | 72 | 3 | 543 |

Every child is at most 875 lines. Mode for all seven: real `#[path]` modules (`mod x; use x::*;`, re-export visibility derived from the items), no `include!`. The root of target 5 keeps 1,417 lines because 792 of them are the inline `mod standards` wiring tree and 293 the `mod editor` tree (nested `#[path = "."]` module declarations that cannot be moved without changing module identity); only the two cohesive record families were carved out. Target 3 keeps 541 lines of tests in its root (below the limit).

## 1. Found state (recovery after the session restart)

- The seven files are byte-identical to `HEAD` and to my pre-edit snapshots (`F/c4p6-tools/orig/*.rs.txt`, git blob hashes compared): nothing was half-applied, no child folder exists in the repo, no registry or reader was edited. Nothing to roll back.
- Lost with the restart: the background staged rustc differential of the 5d group (log empty), the baseline module-owner scan (log empty) and the old scratchpad (staged trees, sandboxes). The tools default now points to the new scratchpad; all seven specs were re-staged (7/7 `staged`), the staged texts are reproducible from `F/c4p6-tools/specs/*.py`.
- Survived and reusable: pos oracle baseline `a1` (`c4p6-pos-a1.log`, canary reached lib and `--test`), lit baseline (0 sites), census baseline, both `--check-applied` baselines (5d exit 0; 3d layout `onlyCombined` 0 native and test, its process was killed after the `complete` line), reader probe baselines (`c4p6-gateprobe-before.json`, `c4p6-completion-before.log`, `c4p6-audit-before.log`, `c4p6-fillprobe-before.json`).
- Peers (`c4p4`, `c4p5`, `c4p7`, `c4p8`) run in neighbouring packages; no overlap with my seven files was found.

## 1b. Second recovery (restart at about 00:18)

- Re-checked at 05:46: the seven files are still byte-identical to `HEAD` and to `c4p6-tools/orig/*` (cmp); no child folder in the repo; nothing half-applied, nothing to roll back. The `c4p6-oracle-p5d-stage1.log` was empty again (killed), `c4p6-modowners-before.log` is complete (18 multi-owner files, none of mine).
- All seven specs re-staged and `lossless` = OK for 7/7, `paths` = no path over the limit. Differential oracle re-started detached (`c4p6-oracle-<group>-pre.log`, `c4p6-oracle-pre.done` when finished).

## 1c. Found state (wave 7, 07:27)

- The seven roots are installed as recorded (line counts 25, 78, 541, 29, 1,417, 105, 72; all 25 children present in the repo index). No half-applied batch, nothing to roll back.
- `--check-applied` 5d (`c4p6-checkapplied-5d-b1.log`, 07:05): finished, `exit 0`, layout native base 3,884 / combined 3,895, `--test` base 3,405 / combined 3,058, `onlyCombined` 0 in both.
- `--check-applied` 3d (`c4p6-checkapplied-3d-b1.log`, 07:07): crashed with `ENFILE: file table overflow` while copying the sandbox (machine-wide file table exhaustion, not a gate verdict). It has to be re-run; the old scratchpad in `checkapplied.sh` is gone.
- `COMPILE ADMITTED` is not yet in `📓️FLEET.md`; `cargo-mutex.lock` is held by `coord-probe` (07:15).

## 2. Layout (new folders `<emoji>️<kebab>`, each docstring emoji equals its folder emoji and is unique among its siblings; all resolve in the taxonomy)

Under `🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`:
- `👆️gestures/`: `🎛️selection-and-weights` (654), `🩹️patches` (513), `🔗️fastener-edits` (509), `🧲️proximity-connect` (277).
- `🖌️session/`: `📋️clipboard` (380, clipboard job plus the copy/cut region), `📌️paste` (325), `📥️snapshot-import` (700).
- `🧪️tests/🔬️unit/`: `⚖️laws-and-rendering` (358, holds `editor_source()`), `🗂️manifest-and-window-actions` (556), `🔌️in-port-and-lane-verbs` (392), `☑️option-groups-and-edits` (510), `📤️import-export-and-menus` (561).
- `🧰️kit/`: `➕️add-node-and-brush-part` (476), `🧱️board-events-and-fasteners` (766), `📚️example-and-brush-mesh` (493), `💾️store-preparation` (280).

Under `🖐️5d/🔨️modules/` (existing folder): `🧱️document-records` (747), `🗂️kind-catalogs` (379).

Under `🧊️3d/…/⏳️precompute/`: `📐️geometry/{🧱️fixed-owners 675, 🧮️pose-math 415, 🧊️collision-index 875, 💥️overlap-state-machine 245}` and `🖌️brush/{🌀️vortex-compatibility 200, 🎯️candidates-and-placement 384, ⏯️suggestions-run 543}`.

Widening (`pub(in super::super)` / `pub(super)`, derived from identifier use across the split files and the declared external readers): 126 + 112 + 5 + 103 + 0 + 49 + 15 sites; it is an over-approximation by name, so visibility is never narrower than before. Longest new absolute path: 228 bytes (limit 240). Tools: `c4p6.py stage|lossless|paths|install <tag>`, specs in `c4p6-tools/specs/*.py`.

## 3. Data and tables

Scan of the seven files for static tables, long literals and `json!` spans: no data table worth moving; the large literals are interpolated builders, so no data was moved and no byte-identity proof is due.

## 4. Verification actually run (this session unless marked)

| Check | Result |
|---|---|
| Lossless line multiset (`c4p6.py lossless`, 7 of 7) | missing 0; the only added lines are wiring (`#[path]`, `mod`, `use x::*;`), child docstrings, `use super::*;`, region markers (every `//#region` has its `//#endregion`, pairs whole inside one child), and the declared `editor_source()` re-pointing. |
| Unused glob imports | the first differential showed two new `unused_imports` warnings (`use super::*;` in `🧱️document-records`, `use manifest_and_window_actions::*;` in the unit root). Fixed in the specs (`glob=False`, `reexport=False`), re-staged, re-run: warnings orig = new (4 = 4 lib, 15 = 15 test). |
| Pre-install rustc differential (`oracle2.py`, `--emit=metadata` on sandbox copies, orig vs new, lib and test) | geometry 38 = 38 lib, 55 = 55 test, CLEAN; brush 42 = 42 both, CLEAN; 5d lib 4,259 = 4,259, test 8,269 = 8,269, warnings equal, the only multiset difference is the sandbox path inside three `couldn't read` rows (same missing fixtures, `orig/` versus `new/` prefix). Logs `c4p6-oracle-{geom,brush,p5d}-pre*.log`. |
| Typeck oracle after install (`pos.py`, family `c4p6`, label `b1`; baseline `a1` taken before the first edit) | canary E0308 E0616 E0624 reached in lib and `--test`; error totals 7,203 = 7,203 (lib) and 12,676 = 12,676 (test); `by code` and privacy lines equal. Message multiset (JSON diagnostics, module qualifier removed): lib 0 differences; `--test` 0 new rows and 50 rows gone, all about `ArtifactToolPublicationContract` fields / `ArtifactToolPublicationLane` variants / `RetainedToolWireInput::declared_bytes` / `ToolJobFactoryError::new`, which live in `🧰️framework/…/🔌️plugin/📜️declaration/👷️job/🦀️.rs` (mtime 00:10, after baseline `a1` at 23:13) and the action bus: a peer edit outside my files, not an effect of the split. Logs `c4p6-pos-a1.log`, `c4p6-pos-b1.log`. |
| Seeded sensitivity (`--mutate`, sandbox copy only, label `m1`) | making `Puzzle5dClipboardJob` private in the new clipboard child: caught, +2 `E0433 cannot find type Puzzle5dClipboardJob` in lib and `--test`. Two mutations in the 3d crates (`FixedOwnerVec.len`, `BrushSuggestionsSearch.found`, then `shapes_intersect`, `compat_pair_matches`, label `m2`) were NOT visible: the cause was not isolated (most likely the stub layer silences unresolved names in those crates), so a private cross-child name raised no error there. Therefore the 3d crates (and all children) are additionally guarded by the name scan `defscan.py` over the live tree: 7 of 7 `cross-file leftovers: 0` (41, 29, 114, 51, 2, 33, 26 private declarations), and a copy with `shapes_intersect` made private is reported (`LEFTOVER pose-math shapes_intersect`), so the scan is sensitive. |
| E0451 literal scan (`rpv-tools/lit.py`, label `b1`) | 0 candidate sites (baseline `a1`: 0); 208 brace structs, 218 definitions, 417 files. |
| Census after (`pfb-tools/census.ts`, files/bytes/lines) | geometry lib 1/92,565/1,989 to 5/94,699/2,067, tests 2/136,432/2,989 to 6/138,566/3,067; brush lib 1/54,288/1,057 to 4/55,653/1,108, tests 1/54,288/1,152 to 4/55,653/1,203 (both far under 28 files, 230,000 bytes, 3,500 lines). The 5d crate stays over the gate (316/1,945,669/26,016 to 329/1,953,983/26,190; tests 819 to 837 files) as before; it is the monolith crate itself and not reduced by this slice. |
| Duplicate mounts (`dm2-tools/modowners.ts`, `c4p6-modowners-b1`) | 18 multi-owner files, identical to the baseline (the two `🔌️plugin/🧪️testing/…` groups); none involve my files. |
| rustfmt hunks (`rustfmt +nightly-2026-07-20 --check --config skip_children=true`) | snapshots 142 hunks, new roots and children 126; all 32 files parse. |
| Taxonomy (`c4p6-tools/tax.ts`) | 26 folders, 0 unresolved. Sibling docstring emoji unique, 25 of 25. |
| Lib-wiring ratchet (`📚️library/🧪️tests/📏️lib-wiring-budget`) | the geometry root (105 lines) and brush root (72) are under the 150-line budget, so their pinned entries are removed from the fixture; the 5d root entry is lowered 2,509 to 1,417 (see section 5). `bun test` of the file: the only failing rows name `📕️norm` roots of a peer; none of mine. |
| Source-text readers (section 5) | `completionRejectionOracle` ok rows= 10 (was broken by the split, fixed); fill probe failure lists identical to the baseline (`runJob` 4 = 4, `p4e` 5 = 5); tool-job gate probe: the puzzle rows (`rows5d`, `dispositions5d`, `failuresPuzzle`, proofs) equal the baseline; publication-authority audit equal; `bun test` `rust-policy-source-evidence`: 6 pass, 0 fail. |
| Wave 7 re-checks (07:27 to 07:45) | (a) `c4p6.py lossless <tag> <installed dir>` against the live tree: 7 of 7 `LOSSLESS OK`, missing 0, region and endregion counts equal in every file set (gestures 10/10, session 4/4, unit 41/41, kit 11/11, root5d 12/12, geometry 17/17, brush 15/15). (b) `--check-applied` 5d (log `c4p6-checkapplied-5d-b1.log`, 07:05): exit 0, native base 3,884 / combined 3,895, `--test` base 3,405 / combined 3,058, `onlyCombined` 0 / 0. (c) `--check-applied` 3d: the first run (`-3d-b1.log`) died with `ENFILE: file table overflow` (machine file table, not a verdict); re-run `c4p6-checkapplied-3d-w7.log` (`--base=f856dff0813`, new sandbox): 34 crates, native 4,063 / 4,063, `--test` 4,904 / 4,904, `onlyCombined` 0 / 0, `admission` empty, stage `complete`; identical to the pre-split baseline. (d) `bun test` of `📚️library/🧪️tests/📏️lib-wiring-budget` crashed twice with a Bun 1.3.14 segmentation fault (86 s, machine load ~16); not a test verdict. Evidence instead: the peer `lwr` ran the law at 07:3x (3 pass, 0 fail per `📓️FLEET.md`) after my fixture edit, and the fixture now pins `🖐️5d/🦀️.rs` at 1,418 (my root: 1,417 lines), with no pin for the geometry and brush roots (105 and 72 lines, under the 150 budget). |
| `[DEBUG]` | none added. |

## 5. Source-text readers and fixtures changed (each a minimal change)

| # | File | Change |
|---|---|---|
| 1 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/⏳️completion/🔣️.json` | the four rows `puzzle5d-copy`, `-cut`, `-paste`, `-import` read the producer region from the child that now holds it (`🖌️session/📋️clipboard`, `📌️paste`, `📥️snapshot-import`); `sourceStart` of paste and import follows the widened `pub(in super::super) struct`; `sourceEnd` is the child's `//#endregion 🧵️ReservedJobs`. Before the fix `completionRejectionOracle` failed with "puzzle5d-copy current producer region is absent"; after: ok rows= 10. |
| 2 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts` | `POLICY_SPLIT_SOURCE_ROOTS` gained the 3d geometry root, so the puzzle-fill audit (`interactivityPuzzleFillSources`, P4e spatial predicates) reads root plus mounted children as the one text the unsplit file held (geometry text 136,093 to 138,115 bytes). |
| 3 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json` | removed the `📐️geometry/🦀️.rs` (2,242) and `🖌️brush/🦀️.rs` (1,152) entries (roots now under the budget); lowered `🖐️5d/🦀️.rs` 2,509 to 1,417. |
| 4 | `5d/…/🧪️tests/🔬️unit/⚖️laws-and-rendering/🦀️.rs` (new child) | `editor_source()` now `include_str!`s the root and every new child of session, gestures and kit (spec `patch`), so the 27 source-text laws of the unit tests read the same text as before. |

Not needed (grep and probes): the publication-authority fixture lists the unchanged root paths; `rows5d`/`dispositions5d`/`failuresPuzzle` of the gate probe and the authority audit are equal to the baseline; no Rust `include_str!` or `read_to_string` of the split files exists outside the unit helper; Cargo manifests reference crate roots only.

## 6. Real-check commands for the compile queue

Run one launcher call at a time under the fleet compile mutex, prefix `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 --`:
1. `check -p semio-s-artifact-puzzle-3d-precompute-geometry -p semio-s-artifact-puzzle-3d-precompute-brush --lib --tests --keep-going --message-format=short` (manifests under `…/📦️packages/🦀️rust/Cargo.toml`).
2. `check -p semio-s-artifact-puzzle-5d --lib --tests --keep-going --message-format=short`, then the dependents of the 3d geometry/brush crates.
3. `test -p semio-s-artifact-puzzle-5d --lib` (names must equal the pre-split names: `editor::tests::…` paths are unchanged because the unit root stays the same module and the children are glob re-exported) and the geometry/brush crates `--lib`.
Review points: `unused_imports`/`private_interfaces` warnings may move; the E0451 literal scan and the name scan are clean, rustc will confirm.

## 7. Foreign findings, limits

- The lib-wiring ratchet test fails today on four `📕️norm/…/⚖️checks` roots of a peer; unrelated to this slice.
- The typeck oracle cannot see privacy of the two 3d crates (section 4, seeded sensitivity); the name scan replaces it there. No cargo/rustc build of the real crates and no Rust law was run.
- The 5d crate stays far over the source-boundary gate (329 files, 26,190 active lines in lib); this slice reduces file sizes but the crate-level split belongs to a later wave.
- Peers' foreign edits in the same plugin (`3d` editor session/fill/precompute, `2d`) were left untouched.

## 8. Files

Created (25 children, `🦀️.rs` each) in the folders of section 2. Rewritten (7): the seven targets. Edited: the three files of section 5 rows 1 to 3. Tools and snapshots: `F/c4p6-tools/` (`c4p6.py`, `oracle2.py`, `defscan.py`, `checkapplied.sh`, `post.sh`, `probes.sh`, probes, `specs/`, `orig/`, `families/c4p6.json`). Rollback: copy `c4p6-tools/orig/<tag>.rs.txt` back over the seven targets (tags gestures, session, unit, kit, root5d, geometry, brush), delete the 25 child folders, restore the three edited files of section 5.
