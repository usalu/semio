# Result: TS-TYPE-ERRORS-1 (prefix `tte`)

Executor: Sonnet, 2026-10-16 16:24 to 17:36. No sub-agents, no modifying git command, no ticket tool, no `bun install` or any download. Logs are in `🗑️generated/tte-*`, tools in `tte-tools/`, pre-edit snapshots of every touched source file in `🗑️generated/tte-snap/`.

## Verdict

DONE for the families I own. `psc` 2722 to 2454 diagnostics (-268), measured with the same `tsc` command and the same project as the baseline. Of the 2722 baseline diagnostics, 302 are removed by my edits (296 directly, 6 as a cascade); 3 are revealed by them (and are real drift); the other 40 new ones and 9 vanished ones are peers' concurrent edits (library, process, build, caching) in files I did not touch.

| Family | Before | After |
| --- | --- | --- |
| A. Puzzle 3d diff (Binary64 half-migration) | 43 | 0 |
| B. TS1205 re-export without `export type` (stdio brep) | 15 | 0 |
| C. TS2694 namespace member, stdio (mp4, avi, semio graph) | 36 | 0 |
| C'. TS2694 `TaxonomyProgress` (repo library, parent's) | 1 | 1 |
| D1. 3d mutations, stale relative import (TS2307) | 14 | 0 |
| D2. sqlite snapshot tests, bare specifier `"🧰️support/🟦️.ts"` (TS2307) | 3 | 0 |
| E. SemioDiff envelope imports 12 subsets from `🧊️brep` (TS2305) | 12 | 0 |
| F. xml `✅️valid` imports its types from itself (TS2303/TS2459) | 6 | 0 (3 real drift revealed, see below) |
| G. generated `📝️text` parsers that reject every string (TS2322) | 167 | 0 |
| Cascade of D2 (TS7006 in process3d and las sqlite tests) | 6 | 0 |
| Everything else | 2419 | 2453 |
| Total | 2722 | 2454 |

By area (diagnostics): stdio 414 to 231, puzzle 76 to 18, draw 227 to 227 (untouched, see below), library 589 to 601 (peers), other 1416 to 1377.

`sdk2` project (256 files): 52 before, 61 after. All 9 new diagnostics are in `📚️library/⚡️caching/🌐️vite`, `🏃️process/…/🎰️jobserver` and `🏃️process/🧭️routing/🎛️command`, which peers edited after the baseline. None of my files is in that list; the 52 baseline diagnostics are unchanged (same codes and counts).

Excluded as requested: the missing workspace packages (`@semio-tech/cad-js*`, `@semio-tech/stdio-*`, `@xmldom/xmldom`, `brepjs`, `graphlib`) are untouched. Baseline: 82 direct TS2307/TS7016 diagnostics for them in 40 files, plus at most 63 follow-on diagnostics (TS2769 21, TS2345 12, TS2339 9, TS18046 7, TS2352 5, TS7006 4) in those same files. With every missing package counted (also `@/lib/*`, `@semio-tech/puzzle-*`, mit-bestand, `tree-kill`, …) the baseline has 220 TS2307/TS7016 diagnostics. They need the user's install decision.

## Decisions and evidence

### 1. Half-done Binary64 migration, puzzle 3d diff: finished to the Binary64 end state

Decision: the new Binary64 API at every site. `puzzlePuzzle3dDiffGuardNumber` was not restored.

Evidence (read-only git and file reads):

- `da607013f32` (2026-10-09 01:21) rewrote the 3d diff from `*PatchEntry` / `added, removed, patched` to `*Modification` / `removed, inserted, moved, modified`. In the same commit it deleted `GuardNumber`, added `GuardBinary64`, stripped `bounds` from `GuardInteger` and added the optional `keys` parameter to `GuardObject`. It then used `GuardNumber` for the new `index`/`from`/`to` parsers, so the commit was inconsistent with itself. The call sites were left behind by the migration; the removal was not a mistake.
- The sibling files already carry the end state:
  - `3d/…/🧬️schema/📸️snapshot/🟦️.ts` (since `d4597d0410a`) types `Puzzle3dVector3 = [Binary64, Binary64, Binary64]`, `Puzzle3dVector4` and every scalar as `Binary64`, importing `{ type Binary64, parseBinary64 } from "@semio-tech/framework/value/ieee754"` (the same import architect uses).
  - `3d/…/🔺️diff/🔣️.json` refers to `Puzzle3dVector3`/`Puzzle3dVector4`/`Binary64Transport` for `position`, `direction`, `origin`, `orientation`, `radius`, `gap`…`y`, `widthWorld`, and to `integer, minimum 0` for `index`/`from`/`to`.
  - `parseBinary64` refuses plain numbers (`🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts`). The fixture `3d/…/🧪️tests/🧫️fixtures/🔺️diff/🔢️native-fields/🔣️.json` (added in the same commit) and the test `✏️s/🧑‍💻dev/🧩️puzzle/🧪️tests/🔺️diff-parsers/🟦️.ts` expect exactly that: words in, `[1,2,3]` refused.
- Freshness: the 3d diff file was last edited at 15:52 by the TSC-UNBLIND executor (the `${${` repair), 32 minutes before my edit and not by a peer; no peer touched any other file I edited within the hour (each script asserted mtime older than 30 or 60 minutes before writing).

Change in `3d/…/🧬️schema/🔺️diff/🟦️.ts` (532 to 542 lines):

- imports `parseBinary64`/`Binary64` and `Puzzle3dVector3`/`Puzzle3dVector4`;
- the five patch interfaces use `Puzzle3dVector3 | Puzzle3dVector4 | Binary64` instead of `number[]`/`number`;
- two new guards `GuardVector3`/`GuardVector4` (tuple of `GuardBinary64`, same error path `…position[0]`);
- `GuardNumber` call sites become `GuardBinary64` (10 scalar sites), `GuardInteger` (24 `index`/`from`/`to` sites) or the vector guards (7 sites); the now unused `RangeBounds` type is removed.

Unknown fields: `GuardObject` had a `keys` parameter that no caller passed, so the parsers accepted unknown fields although every `$defs` entry says `additionalProperties: false`. I passed the property list at all 38 parse functions of 3d, 32 of 2d and 36 of 5d (5d also got the `keys?` parameter 2d and 3d already had). The key lists are the keys of each function's own return literal; I cross-checked every one against `properties` of the matching `$defs` entry in `🔣️.json` (0 mismatches, 2 hand-written lists in 5d for the quoted `"2d"`/`"3d"` keys).

### 2. Related defects in the three puzzle diff files (found while testing 1)

- `🔺️diff/🔣️.json` of 2d, 3d and 5d: 3, 21 and 26 refs were written `artifact.json/$defs/X` (missing `#`); the other 6 to 9 per file were `artifact.json#/$defs/X`. Ajv could not resolve them (`can't resolve reference …/artifact.json/$defs/Puzzle3dKindCatalogs`). Repaired with `(\.json)/\$defs/` to `\1#/$defs/`, counts asserted (3, 21, 26). The Rust readers of these JSON files were not compiled or run (compiler ingress refused); the new form is the one the same files already use for their other refs.
- `dev/🧩️puzzle/🧪️tests/🔺️diff-parsers/🟦️.ts` still used the `…PatchEntry` names of all three dimensions that `da607013f32` renamed to `…Modification` in both `🟦️.ts` and `🔣️.json`. Renamed (15 occurrences, plus `PatchEntryCase`).

### 3. stdio

- **brep** `🧿️semio/…/🧊️brep/🧬️schema/🟦️.ts`: `export {` to `export type {` and `import {` to `import type {` (all 15 names are types in `📸️snapshot`). Removes the 15 TS1205.
- **TS2694 (36)**, three causes:
  - mp4 (2 files) and avi (4 files): `import("./🟦️")` in `🔺️diff` and `🧬️mutations` facets resolved to the file itself. Re-pointed to `import("../📸️snapshot/🟦️.ts")`, which re-exports the types (32 lines, 34 diagnostics).
  - `🧿️semio/…/🕸️graph/🧬️schema/🔺️diff/🟦️.ts`: `SemioValue` and `SemioPoint2` are not exported by the graph snapshot. Re-pointed to `🔢️value/🧬️schema/📸️snapshot` and `✉️base/🧬️schema/🧮️geometry`, their owners (2 diagnostics).
  - The 37th (`TaxonomyProgress`) is the repo library's (parent), not touched.
- The `SemioDiff` envelope `✉️base/🧬️schema/🔺️diff/🟦️.ts` imported all 12 subset diff types from `🧊️brep` (copy and paste). Each now comes from its own subset; I checked that every target exports the name. 12 TS2305.
- `📰️xml/…/✅️valid/🧬️schema/🟦️.ts` imported its types from itself (`'./🟦️'`; the old `✳️any` subset is now `🧱️base`). Re-pointed to `../../🧱️base/🧬️schema/📸️snapshot/🟦️.ts`. That removes 6 diagnostics and reveals 3 real ones (see "Owner decisions").
- Three sqlite snapshot tests (process3d, las, dwg) imported `"🧰️support/🟦️.ts"` without `./` (bare since `d4597d0410a`; `Bun.resolveSync` of the bare form fails, the `./` form resolves). Fixed.

### 4. Generated `📝️text` facets: 167 parsers rejected every string

`parseXText(value)` is typed `XText = string` but called `GuardObject`, so every call threw `$: value is not an object` (verified by importing a pre-edit snapshot) and produced `TS2322 Readonly<Record<string, unknown>> is not assignable to string` in 167 files across about 60 artifacts (fem, flow, puzzle, stdio, …). Changed `GuardObject` to `GuardString` in exactly those 167 functions. The script asserted for every file the one-function shape, `type X = string` and an existing `GuardString` of the same prefix (0 files rejected).

### 5. puzzle 3d mutations: 14 stale relative imports

14 of the 38 `🧬️mutations/<group>/<name>/🦠️mutation/🟦️.ts` had one `../` too few since the group folders exist (`../../🟦️.ts` to `../../../🟦️.ts`, `../../../📸️snapshot/…` to `../../../../📸️snapshot/…`). Each fix was checked with `isfile` on the resolved target.

### New test (feature test with an independent oracle)

`✏️s/🧪️tests/🏘️plugin-composition/📝️text-facet-parsers/🟦️.ts`: finds every `📝️text` facet whose parser is the single-return string shape and whose type is `string`, imports it, and asserts that it accepts `""`, `"text"`, `"a\nb"` and refuses `{}`, `[]`, `null`, `undefined`, `0`, `true`, in agreement with Ajv compiling `{ "type": "string" }`. 204 parsers plus one corpus-size test. I did not register it in `.vscode/launch.json`: the plugin-composition `test` target already runs this folder's tests through `bun ./📜️script.ts test`.

## Test routes (before and after)

All with `bun test ./<path>` from the repo root (the path needs the `./`, otherwise bun scans the whole repo for 90 s).

| Route | Before | After |
| --- | --- | --- |
| `dev/🧩️puzzle/🧪️tests/🔺️diff-parsers` (24 tests, Ajv oracle, native-fields corpus) | 0 pass, 24 fail (`tte-diffparsers-before.log`, measured with only the 3d guard edit in place; the original tree fails the same way: renamed parsers, unresolvable refs) | 24 pass, 0 fail (`tte-diffparsers-after4.log`) |
| `🧪️tests/🏘️plugin-composition/📝️text-facet-parsers` (new) | 167 of the 204 parsers throw on a string (pre-edit snapshot check) | 205 pass, 0 fail |
| process3d / las / dwg `🪶️sqlite/📸️snapshot/🧪️tests` | file does not load (bare specifier unresolvable) | las 9/9, dwg 5/5, process3d 19/20 |
| puzzle 2d `🪶️sqlite/📸️snapshot/🧪️tests` (reads the 2d diff `🔣️.json` I changed) | not measured before | 49 pass, 14 fail. 11 fail with `ENOENT` on fixture paths under `🧬️schema/🧪️tests/🧫️fixtures/🧬️mutations/…` and 3 on regexes over Rust source (`#[derive(…RetainedClone…)]`); none touches the diff `$ref`s. I could not take a before measurement without reverting the JSON, so "identical" is argued from the failure causes, not measured. |

The remaining process3d failure (`Object.hasOwn(snapshot, "process3dSnapshotToSqliteDatabase")`) is a function that does not exist yet (owner decision, below).

Not run (not affected at runtime or refused by the environment): Rust (`bun repo cargo` refused), mp4/avi/brep/semio-graph/base-diff edits (type-only: `export type`, `import type`, `import("…")` in type position, erased at runtime).

## Owner decisions and hand-offs (not done)

1. **draw (227 diagnostics, unchanged).** Only 24 mention Binary64; the premise "Binary64 half-done, 226" was too coarse.
   - 24 Binary64: `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/📄️document/🟦️.ts` lines 144 to 173 (width/height/`PathGeometrySegment` as `number`, artboard `Binary64`), `🧬️schema/🧮️geometry/📷️framing/🟦️.ts:15` (`artboard.width>0` on a `Binary64`, the framing node is `number`), and the tests `📤️export-document` (11), `📄️document` (2), `📷️framing` (1). The real decision is where Binary64 turns into `number` (`binary64Value` at the viewer boundary or `Binary64` throughout the scene); without it every fixture that passes numbers breaks at runtime because `parseBinary64` refuses numbers. Needs the draw owner.
   - 45 TS2352 are not Binary64: `JSON-module fixture as PathRasterInput` (`origin: number[]` not comparable to `readonly [number, number]`, `kind: string` not a literal). A typed fixture helper is the clean fix, `as unknown as` would be lipstick.
   - `✏️editor/🕹️interaction/🧪️tests/🔬️unit/🟦️.ts` (7 diagnostics) was edited by the package-specifier codemod 58 minutes before I looked; I left the whole area alone.
2. **puzzle 5d diff and 2d.** 5d `📸️snapshot` is Binary64 (10 sites) but its diff `🔣️.json` has 0 `Binary64Transport` and the diff `🟦️.ts` is all `number`: the same half-migration as 3d, invisible to `tsc` because the patch types are independent. 2d is `number` everywhere (consistent). `GuardInteger` in 3d no longer enforces `minimum: 0` for `index`/`from`/`to` (the migration dropped the `bounds` parameter; the schema still says `minimum: 0`).
3. **xml `✅️valid` model drift (3 diagnostics revealed).** `snapshot.doc.doctype` is an `XmlDoctype` object (`name`, `externalId`, `declarations`) but `checkValidConformance` and `snapshotWith` still treat it as the raw `<!DOCTYPE …>` string. The conformance rules need rewriting against the structured type.
4. **Generated stdio diffs that call parsers that do not exist** (`TS2304`/`TS2552`, runtime `ReferenceError`): dxf (`parseDxfGroupCode`, `parseDxfValue`, `parseDxfEntity`, `parseDxf…Diff` ×8), html (`parseHtmlNodeDiff`, `parseHtmlChildAdded`, `parseNullableAttrValue`, `parseNullableString`), csv/tsv/epw/pdf/md/json (`…RecordDiff`, `…RecordAdded`, `…PageAdded`, `…BlockDiff`, `parseJsonValueDiff`), semio anim (`AnimTimeline`, `AnimChannel`, `AnimChannelDiff`), `SemioGraphPropertiesDiff`; dxf r12 header snapshot declares `Vec3` twice. There is no generator in the tree that emits them, so hand-writing is possible but would not survive a regeneration.
5. **stdio `🧿️semio/…/🏛️model/…/📝️text/📸️snapshot`**: `SemioModelSnapshotText = string` but the parser returns `{ preamble, body }`. Which of the two is the contract needs a decision (excluded from the new test for that reason). Also the dwg `📝️text/💡️inferences` files that declare both a `string` and an object type.
6. **Missing implementations the tests already call**: `curationSnapshotToSqliteDatabase`/`curationSnapshotFromSqliteDatabase` (🪵️sourcing, 24), `process3dSnapshotToSqliteDatabase`/`…FromSqliteDatabase`, `parseFilters` (sourcing config), `CadDislocateOptions` (cad config schema, 6 imports), `en1990` export, `isCanonicalArtifactKind`, `textEditorSyncPackV1`, `parseExtensionCargoManifest` and friends.
7. **Framework kernel**: `ValueRefusalKind`, `PackTransportCategory`, `PagedRefusalMetadata`, `PagedAllocationMetadata`, `ValueError` not exported by `🎒️pack/⚠️error` (10 TS2305/TS2459); `docklayoutstore` test (79: `EventId` destructured from `any` and then used as a type after the test split).
8. **Library (parent)**: `TaxonomyProgress`, `NODE_ENV` in `ProcessEnv` fixtures, `.mjs` declarations, duplicate `scriptCompilerAdmissionControlV1`. Peers added about 40 diagnostics there since the baseline (`Expected 7-8 arguments, but got 6` in many `📜️script.ts`, `ProcessEnv` signature drift, `leaseDirectory`).
9. **Stale root lists** in `psc-tsconfig-after2.json`/`tsu-tsconfig-psc-live.json`: 4 more roots disappeared since the baseline (`🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📂️opening-config-mutation`, `…/📌️set-default-app`, `…/🧹clear-default-app`, `…/♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input`). The first run stopped with TS6053 for them (0 semantics); I dropped them in `tte-tsconfig-psc-live.json` (list in `tte-psc-dropped-roots.txt`). No baseline diagnostic lived in those four files (all baseline files still exist).

## Commands run (all from `/Users/ueli/Documents/semio`)

1. Baseline: `🗑️generated/tsu-psc-live.txt` (2722) and `tsu-sdk2-live.txt` (52) from `RESULT-TSC-UNBLIND.md`.
2. `python3 tte-tools/tte-3d-diff.py`, `tte-diff-json-ref.py`, `tte-text-facets.py apply`, and inline scripts for the keys, the mp4/avi/graph/base-diff imports, the 3d mutation imports, the xml and sqlite specifiers. All with asserted counts and a snapshot in `tte-snap/` first.
3. `perl -e 'alarm 600; exec @ARGV' -- bun test ./<path>` for the routes above (logs `tte-diffparsers-*.log`, `tte-textfacets-test.log`, `tte-support-spec-*.log`, `tte-sqlite2d-after.log`, `tte-draw-raster-before.log`: 32 pass, 0 fail, draw untouched).
4. `tsc` re-measurement: `🗑️generated/tte-run-tsc2.sh` (detached with `setopt no_bg_nice`): `node --max-old-space-size=12288 node_modules/typescript/bin/tsc -p 🗑️generated/tte-tsconfig-psc-live.json --noEmit --pretty false` exit 2, 858 s, 2454 diagnostics (`tte-psc-after.txt`); then the same for `tte-tsconfig-sdk2-live.json`: exit 2, 135 s, 61 diagnostics (`tte-sdk2-after.txt`). The first attempt (`tsu-tsconfig-psc-live.json`) ended in 2 s with TS6053, the second (nice +5) was cancelled after 18 min at 17 percent CPU.
5. Comparison: `python3 tte-tools/tte-compare.py tsu-psc-live.txt tte-psc-after.txt` (family table) and `tte-newdiag.py` (multiset difference by file, code and message): 48 new, 316 gone.
6. Read-only git: `git log -S puzzlePuzzle3dDiffGuardNumber`, `git log -S parseBinary64`, `git show da607013f32 -- <3d diff>`, `git show <rev>:<path>`, `git diff --stat`, `git ls-files`, `git grep`. `GIT_FSMONITOR` crashed intermittently (`daemon terminated`); I used `-c core.fsmonitor=false`.

No `[DEBUG]` logs were added. Nothing was deleted in any `🗑️generated` folder.
