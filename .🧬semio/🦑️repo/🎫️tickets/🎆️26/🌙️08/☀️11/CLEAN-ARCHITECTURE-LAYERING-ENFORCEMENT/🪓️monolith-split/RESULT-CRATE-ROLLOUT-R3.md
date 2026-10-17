# Result: CRATE-ROLLOUT-R3 (prefix `r3`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/r3-tools/` (`step.sh <tag> <cmd...>`, `r3-census.ts`). Logs: `F/🗑️generated/r3-*`. Started 2026-10-17 08:50.

## 0. Verdict (kept current)

| Package (subset under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets`) | Verdict | Notes |
| --- | --- | --- |
| 🖼️image | DONE | 6 producer crates + facade (section 2.1) |
| 📑️document | DONE | 7 + facade (2.2) |
| 📦️object | DONE | 5 + facade (2.5) |
| 🧰️kit | DONE | 6 + facade (2.6) |
| 🌊️flow | DONE | 7 + facade (2.3) |

All five: `--verify --oracle` exit 0 before apply, `--check-applied` exit 0 with `onlyCombined` 0 (native and `--test`), admission and shared empty, after the last source change (final run `F/🗑️generated/r3-fin-<subset>.log`). Census of all 43 crates of the five subsets (including the unchanged -export/-import codecs): 0 over the gate, lib and test (`F/🗑️generated/r3-census-final.tsv`). Nothing compiled with cargo (gate/launcher not used; no `COMPILE ADMITTED` run).

Order taken: image, document, flow, object, kit.

## 1. Baseline census (cgc7 reader, host cfg, `F/🗑️generated/r3-census-base0.tsv`)

| crate | lib files/bytes/lines | test closure files/bytes/lines |
| --- | --- | --- |
| semio-s-artifact-stdio-semio-image | 97 / 285,165 / 4,462 | 178 / 442,643 / 6,330 |
| semio-s-artifact-stdio-semio-object | 93 / 236,364 / 3,509 | 166 / 407,275 / 5,239 |
| semio-s-artifact-stdio-semio-kit | 111 / 380,090 / 4,892 | 219 / 631,062 / 7,352 |
| semio-s-artifact-stdio-semio-flow | 102 / 324,397 / 4,979 | 129 / 433,626 / 6,028 |
| semio-s-artifact-stdio-semio-document | 114 / 460,138 / 6,703 | 146 / 633,309 / 7,978 |

Sibling codecs (-export/-import) of the five are already inside the gate.

Precondition note: each subset already has staged (index) changes against HEAD by earlier slices (editor/🦀️.rs, package Cargo.toml, schema/💡️inferences/🟦️.ts; kit also sqlite snapshot, snapshot, diff, mutations); no unstaged or untracked changes.

## 2. Per package

### 2.1 image (DONE)

Sequence: plan (blocked: the mutation group of 41 files and a 68-file fixture test crate) -> generator fixes (section 3) -> `--verify --oracle` (exit 0: census, module trees, visibility, orphan, glue, unresolved crates all clean; single-crate oracle and layout differential `onlyCandidate` / `onlyCombined` 0 for lib and test) -> apply with `--backup=F/🗑️generated/r3-backup-image` -> `new crates ... --check-applied` (exit 0, `onlyCombined` 0 native and `--test`, admission/shared empty) -> owners scan (image names absent from `missing`; the listed entries are `🧰️framework/🔨️modules/🧊️3d/*` of peers) -> taxonomy (`stdio` 0 mount findings, 1 pre-existing gif tree finding) -> launch JSONs parse -> census.

| crate | lib files/bytes/lines | test closure |
| --- | --- | --- |
| semio-s-artifact-stdio-semio-image (facade) | 1 / 3,215 / 77 | 1 / 3,215 / 77 |
| -snapshot | 12 / 72,226 / 898 | 12 / 72,226 / 911 |
| -diff | 24 / 72,972 / 1,514 | 28 / 80,303 / 1,684 |
| -schema | 27 / 78,951 / 250 | 28 / 80,815 / 289 |
| -mut-family (11 generic mutation leaves) | 28 / 35,432 / 863 | 28 / 35,432 / 863 |
| -editor | 10 / 61,986 / 1,009 | 27 / 195,940 / 2,589 |
| -io (aggregate and codecs) | 4 / 16,904 / 327 | 8 / 27,283 / 464 |

Before: 97 / 285,165 / 4,462 lib, 178 / 442,643 / 6,330 test. `lib-wiring-budget`: image root is 76 lines, not pinned; the two failing assertions of that test are other slices' (jpg, energy model) and a stale mesh pin (R2).

Incident (recorded for the coordinator): my first plan run for image used `--explain` only, without `--dry-run`/`--verify`, and the plan had no problems, so `new crates` applied it unverified and without a backup. I reverted exactly my 79 paths with `F/r3-tools/revert.ts` (created files deleted, modified files restored from the index, the six members and lock packages and six owner rows removed, facade lock block restored from the index; the three pre-existing staged image files were untouched), then redid the proper sequence. Lesson: always pass `--dry-run` when exploring.

## 3. Generator and tooling changes (library `L/🏗️authoring/`)

1. `📦️crate-tree/🧬️mutation-families/🟦️.ts`: `aggregateScope` and `decoupleFromAggregate`. A converted leaf (and its `diff`/`inverse` companions) keeps `use super::*;` or `use super::super::*;` from the aggregate module, which unions it back into the aggregate slice (`subtreeCoupled`) so the leaves could never leave the aggregate crate. The glob is now replaced by explicit absolute imports of exactly the names the file uses (aggregate imports resolved to absolute paths, public child modules as `crate::..::leaf`); when a file uses an item defined in the aggregate itself, the leaf stays coupled and a note says which names. A peer (R-slice using `hoistAggregateSupport`) builds on this to host such helpers in a `support` module.
2. `📦️crate-tree/🧪️test-closure/🟦️.ts`: runtime-fixture rewrite now walks the whole `#[path]` subtree of each test mount (it used to stop at the mount file, so aggregated fixture lists like `schema/mutations/tests/fixture` kept every `include_str!` fixture compiled: 68 files for 11 cases).
3. `📦️crate-tree/🗺️plan/🟦️.ts`: crate names for packages rooted at a subset: when several crates share the first-segment label (`schema`, `schema-2`, ...) the crate that owns a nested module takes its own last segment (`snapshot`, `diff`), only the root module keeps `schema`; mutation family crates with several slices sit in the LCA directory of their leaves instead of the directory of the first leaf.
4. `🎮️command/🟦️.ts` and `📦️crate-tree/🧫️sandbox/🟦️.ts`: `new crates <subset-package> --check-applied` crashed (read `Cargo.toml` of the subset directory). The workspace manifest is now resolved from the package's `workspace` key and passed to `siblingPathCrates` as `workspaceDir`.
5. Tools: `F/r3-tools/{step.sh,r3-census.ts,trace.ts,revert.ts}`.

### 2.2 document (applied)

Hand source changes (all in `📑️document`): (1) the element codecs (`enc_/dec_` bool, u8, f64, list, run_style, run, block, list_item, cell, row, style, image) moved from `io/text/diff` into `io/text/snapshot` (`snapshot_wire_codec`), the duplicated `hex_*`/`enc_str`/`dec_str` of the diff codec now import the identical base-crate functions, and all importers (binary snapshot, binary diff, binary and text mutation codecs) point at `io::text::snapshot`: the snapshot crate no longer depends on the diff crate (it used to close a 3-cycle snapshot, diff, mutations). (2) `demo_semio_document_snapshot`, `demo_mutation_cases`, `demo_diff_cases` widened from `pub(crate)` to `pub` (they stay `cfg(test)`-gated, then converted by the testkit step below). Generator: `hoistAggregateSupport` now moves inherent `impl` blocks along with their type (`inherentSelf`), so `DocBlockPath`/`DocBlockLeaf` could be hosted in the support module and all 16 leaves decouple. `placeless` in the sandbox oracle compares unresolved derive-attribute diagnostics without file/line (an item that moves keeps its count).

Sequence: `--mutation-families=on --verify --oracle` exit 0 (oracle and layout 0 / 0, lib and test), apply with `--backup=F/🗑️generated/r3-backup-document`, `--check-applied` exit 0.

| crate | lib files/bytes/lines | test closure |
| --- | --- | --- |
| -document (facade) | 1 / 3,865 / 83 | 1 / 3,865 / 83 |
| -snapshot | 12 / 148,586 / 1,450 | 12 / 148,586 / 1,468 |
| -examples | 22 / 114,748 / 2,236 | 28 / 119,932 / 2,380 |
| -schema | 27 / 118,552 / 293 | 28 / 119,668 / 320 |
| -mut-family (16 leaves) | 28 / 38,651 / 946 | 28 / 38,651 / 946 |
| -mutations (aggregate and codecs) | 28 / 86,854 / 1,537 | 28 / 86,854 / 1,562 |
| -io | 5 / 25,958 / 514 | 28 / 191,844 / 1,542 |
| -editor | 2 / 19,651 / 358 | 4 / 21,638 / 404 |

Before: 114 / 460,138 / 6,703 lib, 146 / 633,309 / 7,978 test.

### 2.3 flow (applied)

Source changes: `demo_diff_cases`, `demo_mutation_cases`, `node` widened to `pub` (they stay `cfg(test)`-gated, then testkit). Notes: `mutation-families=on` is required (the default `auto` does not pick the converted plan); `DragNodes` stays concrete (inherent impl on the aggregate). Crates: snapshot 13 / 91,102 / 1,325, examples 24 / 70,606 / 1,339 (test 28 / 84,133 / 1,596), schema 27 / 85,315 / 271, mut-family 28 / 36,221 / 909, set-node-position 5 / 7,894 / 190, mutations 8 / 45,785 / 593, io 7 / 43,364 / 809 (test 26 / 136,488 / 1,516), facade. Before: 102 / 324,397 / 4,979 lib, 129 / 433,626 / 6,028 test.

### 2.4 cfg(test) rule (coordinator, 11:30)

`F/r4-tools/r4-xtest.ts` over image, document and flow found 14 same-subset cross-crate uses of `#[cfg(test)]` helpers (the other 12 rows of the raw scan are name collisions across subsets): `demo_image_snapshot`, `demo_diff_cases` (image, document, flow), `demo_mutation_cases` (image, document, flow), `demo_semio_document_snapshot`, and flow `fixture`, `node`, `edge`. Fixed with `F/r4-tools/r4-testkit.ts` (rows in `F/🗑️generated/r3-xtest-same.json`): the helpers and the `cfg(test)` items they need are now `#[cfg(feature = "testkit")]`; each owner crate declares `testkit = []` and a self dev-dependency with the feature, each user crate has a dev-dependency on the owner with `features = ["testkit"]`. No `cfg(any(test, ...))`. Re-ran `new crates ... --check-applied` for the three (the sandbox passes `--cfg feature="testkit"` to the `--test` run): see verdicts.

### 2.5 object (DONE)

Source change: `demo_object_snapshot` and `demo_diff_cases` widened `pub(crate)` to `pub` (visibility coupling merged the unit tests into the snapshot group and closed a cycle). `--mutation-families=on`. Crates (lib / test): -snapshot 12 / 60,747 / 740 (test 12 / 60,747 / 753), -examples 24 / 40,224 / 621 (28 / 46,664 / 773), -schema 26 / 58,750 / 380 (28 / 64,338 / 484), -mut-family (9 leaves) 28 / 34,075 / 783, -editor (aggregate, codecs, editor, viewer) 9 / 51,765 / 880 (24 / 164,277 / 2,446), facade 2 / 21,832 / 393 (8 / 32,178 / 536). Before: 93 / 236,364 / 3,509 lib, 166 / 407,275 / 5,239 test.

### 2.6 kit (DONE)

Source change: `demo_kit_snapshot` and `demo_diff_cases` widened to `pub`. `--mutation-families=on`. Crates: -snapshot 12 / 114,054 / 1,271, -schema 23 / 100,565 / 223 (test 26 / 126,436 / 649), -examples 25 / 59,450 / 933 (28 / 64,335 / 1,062), -mut-family (15 leaves) 28 / 37,803 / 805, -mutations (aggregate and codecs) 27 / 61,878 / 1,102 (29 / 63,310 / 1,162), -editor 4 / 28,878 / 559 (23 / 179,539 / 2,528), facade 2 / 21,704 / 391 (8 / 32,914 / 547). Before: 111 / 380,090 / 4,892 lib, 219 / 631,062 / 7,352 test. The ratchet pin of `kit/🦀️.rs` (287 lines) is removed (the root is 91 lines now); object's pin was already gone.

### 2.7 cfg(test) rule, final form

Canonical idiom (coordinator, 14:10) is in place for every cross-crate helper of the five subsets: owner `#[cfg(any(test, feature = "testkit"))]` plus `[features] testkit = []`; consumers list the owner in `[dev-dependencies]` with `features = ["testkit"]`; no self dev-dependency (`F/r3-tools/idiom.py` converted the first form that `r4-testkit.ts` wrote). Helpers handled: image (`demo_image_snapshot`, `demo_diff_cases`, `demo_mutation_cases`), document (`demo_semio_document_snapshot`, `demo_diff_cases`, `demo_mutation_cases`), flow (`demo_diff_cases`, `demo_mutation_cases`, `fixture`, `node`, `edge`), object (`demo_object_snapshot`, `demo_diff_cases`, `demo_mutation_cases`), kit (`demo_kit_snapshot`, `demo_diff_cases`, `demo_mutation_cases`). `r4-xtest.ts` over the five subsets now reports 0 rows. The unit tests that stayed inside the same crate as their helpers need nothing.

### 2.8 Other checks

- Oracle blind spot (kept root file modules): the five subset roots have no `mod x;` / `pub mod x;` file modules (only `#[path = "."]` wrappers with components), so no `crate::<kept-root-mod>::<item>` path can point at a moved item.
- Owners scan: no owner or package of the five subsets in `missing` (the entries listed are peers' `🧰️framework/🔨️modules/🧊️3d/*`).
- Taxonomy (`pw0-taxcheck.ts ✏️s/🔌️plugins/🗄️stdio`): 0 mount findings, 1 tree finding (gif sqlite, existing).
- `lib-wiring-budget`: none of the five roots (76 to 91 lines) is flagged; the remaining failures are other slices' (jpg, energy model, writer, sequence, bim model, puzzle support, stale mesh pin).
- Launch JSONs (`.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`) parse. The generator registers no launch routes; none were added.

## 4. Generator changes added after section 3

5. `🧬️mutation-families/🧰️support-hoist/🟦️.ts`: inherent `impl` blocks move with the type they are written for (`inherentSelf`); trait impls still pin their type.
6. `🧫️sandbox/🟦️.ts`: `placeless` (unresolved derive-attribute diagnostics compared without file/line) so an item moved to a support file is not an "extra".
7. Tests: `L/🧪️tests/🧬️mutation-leaf-generics/🟦️.ts` gained four tests (decoupling replaces the glob with absolute imports, leaves keep the glob and name aggregate items, depth mismatch ignored, `inherentSelf`); they pass (`bun test -t "leaf decoupling"`, 4 pass). Not covered by a test: the nested runtime-fixture walk, the crate-name disambiguation and `placeless`.
8. Known gaps: mutation family crates are named `-mut-family` and the crate holding the aggregate with its codecs is named after its largest slice (`-editor`, `-mutations`); acceptable, not pretty.
