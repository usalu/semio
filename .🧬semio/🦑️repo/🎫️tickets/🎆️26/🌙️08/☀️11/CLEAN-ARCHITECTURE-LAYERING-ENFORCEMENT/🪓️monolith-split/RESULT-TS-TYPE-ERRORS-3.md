# Result: TS-TYPE-ERRORS-3 (prefix `tt3`)

Executor: Sonnet, 2026-10-17 (started about 07:30; the machine clock jumped by about four hours during the last full run, so the timestamps of the logs after that are not a duration). Foreground only, no sub-agents, no modifying git command, no ticket tool, no `bun install`. Logs and tsc outputs are in `🗑️generated/tt3-*`, tools in `tt3-tools/`, pre-edit snapshots of the files I touched with scripts in `🗑️generated/tt3-snap/`.

## 0. Verdict

**PARTIAL, large step, not zero.** The whole-workspace check (`bun ./📜️script.ts verify typescript check`) went from **13,697 roots / 1,920 diagnostics in 561 files** to **13,753 roots / 852 diagnostics in 333 files** (-1,068, -56 %). The roots grew by 56 because peers added files; the one diagnostic that is new against the baseline is a peer's new file (`⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️nodes/🧪️tests/📦️child/📜️script.ts`, 1 diagnostic). Every other file has the same or fewer diagnostics than at the baseline. A syntax scan (`bun build --no-bundle`) of the 331 `.ts/.tsx` files changed in the last two hours found no syntax error before the last run.

Of the 852 left, **104 are TS2307/TS7016 for workspace packages that are not in `node_modules`** (skipped as ordered; list in section 5) and **161 sit in `♻️mit-bestand`**, 137 of them follow-on implicit-`any` diagnostics of the missing `@semio-tech/mit-bestand-praesentation-projektetage` package (12 of the 161 are also among the 104). Without those two groups **599 real diagnostics remain, in 274 files, at most 21 per file.**

Measured runs (all with the permanent route, same command, 24 GiB heap):

| Run | When | State | Roots | Diagnostics | Files |
| --- | --- | --- | --- | --- | --- |
| `tt3-base` | before my first edit | baseline | 13,697 | 1,920 | 561 |
| `tt3-r1` | after the root `exclude`, docklayoutstore, first sqlite fixes | | 13,745 | 1,422 | 485 |
| `tt3-r2` | after matcher declarations, grant/verb fixtures, bindings, wiring | | 13,750 | 938 | 349 |
| `tt3-r3` | final | | 13,753 | **852** | 333 |

Two small edits were made after `r3` (the `bun:sqlite` statement declaration and the two remaining envelope literals of `document-catch-up`); a scoped run over their five roots (`tt3-scope4`) gave 0 diagnostics, so the true final count is 6 lower than `r3` (the four `safeIntegers` sites and the two `verb, line` sites, which were in the 852; not re-measured on the whole workspace).

## 1. Baseline buckets (before my first edit)

Command: `bun ./📜️script.ts verify typescript check --out F/🗑️generated/tt3-base.tsconfig.json` (exit 1 is the default gate `--max-diagnostics 0`, not a crash). Log `tt3-base.log`, diagnostics `tt3-base.txt`.

| Code | Count |
| --- | --- |
| TS2345 | 354 |
| TS7006 | 301 |
| TS2769 | 269 |
| TS2339 | 238 |
| TS2307 | 96 |
| TS18046 | 94 |
| TS2741 | 92 |
| TS2322 | 61 |
| TS2554 | 46 |
| TS7053 | 45 |
| TS2353 | 42 |
| TS2304 | 41 |
| TS7016 | 27 |
| TS2315 | 24 |
| TS2305 | 21 |

| Area | Count |
| --- | --- |
| repo/library | 412 |
| ♻️mit-bestand | 161 |
| framework/🔨️modules/🏃️process | 127 |
| plugin/🗄️stdio | 120 |
| os/🔌️plugin | 97 |
| framework/🧪️tests/🧪️docklayoutstore | 79 |
| 🌎️hub | 60 |
| os/📺️renderer | 56 |
| product/📓️print | 56 |
| os/🧪️tests | 50 |
| repo/other-modules | 44 |
| os/🌊️flow | 41 |
| framework/🔨️modules/🎒️pack | 40 |
| plugin/🪵️sourcing | 33 |
| framework/🔨️modules/🌱️value | 30 |
| plugin/🌀️procedural | 28 |
| dev-or-tests/🧑‍💻dev/🗄️stdio | 25 |
| 🎓️teaching | 25 |
| plugin/🖍️draw | 24 |
| plugin/📐️cad | 23 |
| plugin/🖨️raster | 23 |
| dev-or-tests/🧑‍💻dev/🎭️variants | 23 |
| plugin/🗒️note | 21 |
| plugin/🏛️architect | 18 |
| .storybook | 17 |
| dev-or-tests/🧪️tests/🏘️plugin-composition | 16 |
| plugin/🧩️puzzle | 15 |
| framework/🔨️modules/⏯️tool-run | 14 |
| framework/🔨️modules/◻️2d | 14 |
| plugin/📋️forms | 13 |

total 1920

## 2. Final buckets (`tt3-r3`)

| Code | Count |
| --- | --- |
| TS7006 | 189 |
| TS2345 | 112 |
| TS2339 | 101 |
| TS2307 | 95 |
| TS2353 | 42 |
| TS2322 | 40 |
| TS7053 | 37 |
| TS2554 | 34 |
| TS7016 | 25 |
| TS2769 | 21 |
| TS2305 | 19 |
| TS2540 | 15 |
| TS2741 | 15 |
| TS18046 | 10 |
| TS2542 | 10 |

| Area | Count |
| --- | --- |
| ♻️mit-bestand | 161 |
| repo/library | 98 |
| plugin/🗄️stdio | 61 |
| os/📺️renderer | 40 |
| framework/🔨️modules/🎒️pack | 36 |
| product/📓️print | 35 |
| framework/🔨️modules/🏃️process | 34 |
| os/🔌️plugin | 30 |
| os/🌊️flow | 28 |
| dev-or-tests/🧑‍💻dev/🗄️stdio | 25 |
| plugin/🖍️draw | 24 |
| repo/other-modules | 16 |
| .storybook | 15 |
| dev-or-tests/🧪️tests/🏘️plugin-composition | 15 |
| plugin/📐️cad | 14 |
| plugin/🧩️puzzle | 13 |
| plugin/🗒️note | 12 |
| framework/🔨️modules/◻️2d | 12 |
| os/📡️spr | 10 |
| plugin/🌀️procedural | 8 |
| plugin/📏️layout | 8 |
| 🎓️teaching | 8 |
| framework/🔨️modules/⏯️tool-run | 8 |
| framework/🔨️modules/🌱️value | 8 |
| dev-or-tests/🧑‍💻dev/🎭️variants | 7 |
| framework/🔨️modules/🏗️mesh-engine | 7 |
| 🌎️hub | 6 |
| os/🏪️store | 6 |
| dev-or-tests/🧑‍💻dev/📐️cad | 5 |
| os/🧑‍💻dev | 5 |

total 852

## 3. Families fixed (baseline to `tt3-r3`)

| Family | Before | After |
| --- | --- | --- |
| Ambient `ProcessEnv.NODE_ENV` from `next-env.d.ts` (direct) | 233 | 0 |
| Spawn/exec overloads reduced to `never` (cascade of the above) | 86 | 0 |
| docklayoutstore converted test (`any` bag, `Command is not generic`) | 79 | 0 |
| TS2769 `expect(...).toBe/toEqual` expected-type overloads | 192 | 0 |
| `ScriptControl` passed for `ScriptInvocation` (`runArtifactRustTests` callers) | 26 | 0 |
| `cacheInternals` / inference `.mjs` without declarations | 26 | 0 |
| `actorIdentityGrant` / wire `verb, line` missing in test configs | 52 | 2 |
| `SQLQueryBindings[]` (bun:sqlite `run` bindings) | 31 | 5 |
| `new Database(bytes)` instead of `Database.deserialize` | 5 | 0 |
| `Cannot find name 'row'` (lost loops in converted tests) | 29 | 0 |
| `.map.map(` typo and other lost-binding residue | 16 | 0 |
| Plugin sqlite snapshot tests importing the wrong owner | 23 | 0 |
| `assert(validate(x))` narrowing `any` to `unknown` | 35 | 0 |
| `VizOptionValue` from continuous scales in `grid(...)` twins | 20 | 0 |
| Generated frame/DSL guards typed required but parsed optional | 13 | 0 |
| Window-kit boot message missing `catalog` | 12 | 0 |

Where the root cause lives and what the real fix was:

1. **Ambient `ProcessEnv.NODE_ENV` (233 direct, 86 follow-on `never` diagnostics, about 320 in all).** `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/next-env.d.ts` (`/// <reference types="next" />`) is a tracked, Next-generated file of one app. In a whole-workspace program it declares `ProcessEnv.NODE_ENV` as required for every other package, so every `spawn({ env })`, `execFile` and `Command`-style call with a plain record failed, and the overload failure reduced `ChildProcess` to `never` (`once`, `stdout`, `pid`, ...). Fix at the owner of the workspace project: `"**/next-env.d.ts"` in the root `tsconfig.json` `exclude` (the one place `verify typescript` reads for "not source"). The app is still checked by its own `tsconfig.json`. I proved before the full run that a program containing only `next/server` imports does not load `next/types/global.d.ts`, so nothing else leaks it.
2. **`docklayoutstore` converted test (79).** The in-source converter had replaced every import by `type X = any` and `dependencies: any`. Now `dependencies: typeof import("@semio-tech/framework")`, the 21 types come from `import type { ... } from "@semio-tech/framework"` (`EventId`, a value and a type, is imported as `EventIdBrand`). Vitest: 86 of 86 pass.
3. **`bun:test` matcher typing (192 TS2769 plus 21 TS2558 that a first version caused).** `bun-types` declares `toBe/toEqual/toStrictEqual/toContain/toContainEqual` with `expected: T` (the actual's own type), so every comparison of parsed, projected or independently derived data is a compile error although the runtime compares by value; Jest and Vitest type `expected` loosely. New ambient module augmentation `🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🧪️bun-test-matchers/🟦️.d.ts` replaces those five matchers by `(expected: unknown)` plus a generic `<X>(expected: X)` overload (so `toEqual<readonly string[]>(...)` still compiles). This is a repo-wide loosening of five matcher parameters; it is **a ruling for the coordinator to confirm or revert** (revert = delete that file; 192 diagnostics return). The alternative was `expect<unknown>(...)` at about 190 call sites, as TS-TYPE-ERRORS-2 did for 63. Sites where the expectation is the wrong shape still fail at run time as before.
4. **`runArtifactRustTests(…, this.invocation.control)` (26 sites, all in permanent `📜️script.ts`).** The function takes the whole `ScriptInvocation` now; about 20 plugin and product scripts still passed `.control`. Replaced by `this.invocation` at the 26 sites reported by `tsc` (`tt3-tools/tt3_control_to_invocation.py`, asserts the exact text at the reported position).
5. **Inference `.mjs` modules without declarations (26).** `🏗️build/🕸️graph/📥️inference/🟨️.d.mts` is the typed surface of `cacheInternals`; the tests used 12 members it did not declare (`bunLockGraph`, `cargoTargets`, `collectPlaygroundCatalog`, `createMetadataImplementation`, `createRustSourceCache`, `inferenceCandidates`, `moduleSourceRecordCacheRootV1`, `playgroundSessionTargets`, `rustSourceFiles`, `sourceAnalysisOwnerIdentityV1`, `targetExecutionKindV1`, `targetScriptSources`) and rows of dependency inference had to carry `targets` although only `root` is read. Declared with the real signatures read from the implementation (new types `BunLockGraphV1`, `RustSourceCacheV1`, `PlaygroundCatalogRowV1`, `BuildMetadataGraphV1`, `BuildModuleGraphV1`, `BuildMetadataEntryV1`, `BuildProjectRootV1`). New `⚡️caching/📥️inference/🟨️.d.mts` for the five exports of the parser module the scheduling and observation tests import. `🧪️test/🟨️.d.mts`: `internals.*` typed (`TestVocabularyV1`, `rustSutCratePath`, `taxonomy`, `oracleContributionPaths`, `inputsFor`, `isExcluded`, `projectNameFor`). `cargoDirectoryEntriesV1` now returns the four literal entry kinds it produces.
6. **Test actor configs without `actorIdentityGrant`, wire envelopes without `verb, line` (52).** New `WORKER_LAW_ACTOR_IDENTITY_GRANT` in `🧪️tests/🧰️worker-law-fakes` (the module every one of those tests already imports); 44 literals in 13 files carry it, 9 envelope literals carry `verb: null, line: null`. After the edits every inserted property was checked to sit in a literal that names `documentId` or `mutation_id`.
7. **Wrong sqlite owner in plugin snapshot tests (23).** The curation and architect tests imported the schema snapshot module and called `…SnapshotToSqliteDatabase` on it; the functions live in the sibling sqlite module. Imported from there. Test result: curation `3 pass / 46 fail` at HEAD, **`49 / 0`** now.
8. **bun:sqlite typing (31 + 5).** `db.run(sql, a, b)` to `db.run(sql, [a, b])` at the sites `tsc` reported (about 27 in 20 files) (Bun accepts both; verified; the 5 matches left in `r3` are the 4 `safeIntegers` sites below and one spread argument in `plugin-runtime-runtime-close-budget`), `new Database(bytes)` to `Database.deserialize(bytes)` at 5 sites (raster sqlite tests: 9 of 9 pass), and one ambient declaration `🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🗄️bun-sqlite-statement/🟦️.d.ts` for the per-statement `safeIntegers(...)` that Bun implements but its typings omit (4 sites, verified by `tt3-scope4`).
9. **`assert(validate(x))` narrows `any` to `unknown` (35).** Ajv's `ValidateFunction<unknown>` is a type guard to `unknown`, so the `any` fixture became `unknown`. Fixed with typed admissions: `compile<LoanInput[]>`, `validate<DeadlineFixture>`, `compile<HostileCandidate>`, and for the GIS peer-rebootstrap test a full Ajv admission of the fixture against the owning schema (`$ref`s into `GisMapPeerRebootstrap*`; I verified the fixture passes before using it).
10. **Converted tests that lost a loop or a binding (45 diagnostics, real run-time bugs).** `for (const row of …)` missing in `taxonomy-pattern-compiler-reuse`, `markdown-inline-references`, `path-emoji-statutes`, `workspace-contract` and `rust-divergence-callback`; `.map.map(` in six tests (a `TypeError` at run time); `assets`, `control`, `policyReadFileSafe` and `toolJobMountedDispatchOneTurnExact` undefined in `mesh-engine` attributes, dashboard execution and the note `📜️script.ts`. Test effect (bun, before at HEAD to after): taxonomy-pattern-compiler-reuse 15/6 to 19/2, path-emoji-statutes 26/11 to 28/9.
11. **Generated frame guards (13).** The six `stdio/semio` frame/DSL schemas (`object` and `kit` binary and text facets) did not list `required`, while the grammar mirrors (`ksy`/`abnf`) and the TS interfaces say every field is mandatory; the generated parsers therefore returned `T | undefined` into required fields. Schema first: `required` added to the six `🔣️.json`, the six parsers no longer wrap their guards in `=== undefined ? undefined :`.
12. **Others, each a handful:** `VizOptionValue` from continuous scales (20, five `grid` helpers now refuse a non-number loudly), Binary64 weights in the WFC 2d inference (converted once with `binary64Value`, per the ruling), `NonNullable<T>` in three WFC diff `applyRows`, `0 as Binary64` to `binary64(0)`, CAD `CadDislocateOptions` (TS twin of the Rust struct, 5 import sites) and `reject(): never` as a function declaration (assertion narrowing), `window-kit` boot messages missing `catalog` (12), `.storybook/main.ts` spreads of `string | RegExp | …` and `allowedHosts: true`, teaching/discovery runtime observation literals (`compilerResourceRoot`, `compilerResources`), `generation3d` brep-interchange test import path, `test.each` over a `readonly` corpus (`test.each([...rows])`), `fast-json-patch` result wrapper in the retirement tests (`patched(...)` returns a record).
13. **Obsolete test removed.** `workspace-contract` described `parseExtensionCargoManifest`, which `1bae6115bc5` (2026-10-16) deleted together with `runExtensionComponentPackage`. I removed the `describe("extension Cargo installation owner")` block (snapshot `tt3-snap/workspace-contract-extension-block.txt`) and its only fixture `🧫️fixtures/📦️extension-installation-owner/🔣️.json` (snapshot `tt3-snap/extension-installation-owner.json`).

## 4. Tests run (real results)

| Test | Before | After |
| --- | --- | --- |
| vitest `🧰️framework/🧪️tests/🎚️config`, `docklayoutstore` file | not run | **86 pass, 0 fail** |
| vitest `🧰️framework/🛍️products/💻️os/🧪️tests/🎚️config` (22 files) | 609 tests, 499 pass, 110 fail (`RESULT-TS-LEFTOVERS.md`) | 609 tests, 499 pass, 110 fail; **the set of failing names is identical, 0 new, 0 fixed** |
| bun curation sqlite snapshot | 3 pass, 46 fail | **49 pass, 0 fail** |
| bun architect sqlite snapshot | 9 pass, 49 fail | 10 pass, 48 fail (the other 48 fail for unrelated reasons) |
| bun forms sqlite snapshot | 14 pass, 0 fail | 14 pass, 0 fail |
| bun raster sqlite snapshot | 9 pass, 0 fail | 9 pass, 0 fail |
| bun taxonomy-pattern-compiler-reuse | 15 pass, 6 fail | 19 pass, 2 fail |
| bun path-emoji-statutes | 26 pass, 11 fail | 28 pass, 9 fail |
| bun microsecond-clock | not run | 2 pass, 0 fail |
| bun retained-clone controlled retirement | not run | 15 pass, 0 fail |
| bun job loan tests | 2 pass, 1 fail | 2 pass, 1 fail |
| bun dsl architecture | 4 pass, 3 fail | 4 pass, 3 fail |
| bun workspace-contract router tests (`-t "metadata router"`) | not comparable | 0 pass, 2 fail (runtime contract of the generated scripts moved; not a type problem) |
| bun markdown-inline-references | fails at load (`markdown-it` missing in the cache, before and after) | same |
| bun history-inputs | fails (committed fixtures missing, before and after) | same |

Before numbers come from the HEAD copy of the file run in place (`tt3-before-tmp.ts`, removed after) or from the snapshot. The twin adapters of `📓️print` (`defineTestAdapter`) are not runnable on their own; they are type-only changes plus a loud refusal in `grid`.

## 5. Not done, with owners

| What | Count (`r3`) | Why not |
| --- | --- | --- |
| Workspace packages missing in `node_modules` (TS2307/TS7016, non-relative) | 104 | user decision (`graphlib` 10, `@semio-tech/mit-bestand-praesentation-projektetage` 12, `@xmldom/xmldom` 7, `brepjs` 4, `jsdom` 4, `yargs-parser` 4, `@semio-tech/cad-js-module-*`, `stdio-*`, ...) |
| Their follow-on diagnostics in `♻️mit-bestand` and the SVG export tests | about 150 | same cause |
| `🧑‍💻dev/🗄️stdio/🟦️.ts` | 23 | all TS2307 of `@semio-tech/stdio-*` |
| Layout-drawing sqlite test and the `layout-layout-ts` fixture shims | 15 + 7 | peer in flight: the sqlite fixtures are staged as deleted and a hub composition fixture directory appears; the shims point to the old paths |
| `PackError` taxonomy (TS lags the Rust: `refusalKind`, `fromPagedRefusal`, `PackTransportCategory`, ...) | 36 | feature gap, `⚠️error/package.json` was being edited by a peer |
| `discovery/🕸️runtime/🧪️tests` | 40 | API drift (`IssuedScriptInvocationV1`, `measure`, `RuntimeCompilerResourceEntryV1`), needs the runtime owner |
| `fast-json-patch` shape-changing patches read through `.newDocument` | 57 | each site needs the post-patch shape; a two-type-parameter helper was not worth building for them |
| Forged `SqliteDatabase` rows (`rows.reverse()`, `values[i] = …`, `rowid = …`) | about 25 | the types are `readonly` on purpose; a mutable test view needs a new testing export in `sqlite-snapshot` |
| `quiz/🚀️deploy` | 8 | `runRepositoryCommand` now needs an invocation and a policy; the deploy module has to thread them |
| `.storybook` | 15 | five unresolved wasm bindings (missing packages), `OwnedBuildPlugin` is not a Vite `PluginOption`, and a peer is rewriting `main.ts` |
| Generated `…/🔺️mesh/…/diff` guards that return `Readonly<Record<string, unknown>>` for `NamedTripleDiff` | 3 | needs a generic named-triple parser in the generator |
| `SemioGraphPropertiesDiff` not declared, `Wfc` and `Canvas` typings of third-party packages (`pdfjs`, `opentype`, `svg2pdf`) | 6 | dependency versions, schema-owner decision |
| Test fixture sources with intentional duplicate declarations (`🧫️fixtures/📁️some/📁️folder/*`) | 6 | data, not code |
| `world3d-inline-surface` | 10 | modified by a peer minutes before the run |

## 6. Rulings and risks to confirm

- **Matcher loosening** (family 3): one ambient file, revertible by deleting it; the generic overloads keep explicit `toEqual<T>` usage compiling.
- **`next-env.d.ts` out of the workspace program** (family 1): the Next app is no longer checked by `verify typescript` for its `next-env` references; it still has its own `tsconfig.json`.
- **Schema `required` on six stdio frames** (family 11): the Rust readers of those JSON files (`🦀️.rs`) were not compiled or run (compiler ingress); I did not search for fixtures or tests that validate instances against these six schemas; the TS interfaces and `ksy`/`abnf` already said "required".
- **Obsolete extension-packaging test and fixture deleted** (family 13).
- **Binary64**: applied as ruled; the only conversion I added is in `slotEntropy` of the WFC 2d inference, once at the computational boundary through `binary64Value`.

## 7. Commands run (all from `/Users/ueli/Documents/semio`)

1. `bun ./📜️script.ts verify typescript check --out F/🗑️generated/tt3-{base,r1,r2,r3}.tsconfig.json --max-diagnostics 100000` (whole workspace, four times), and scoped `--files-from F/🗑️generated/tt3-{env,dock,scope1,scope2,scope3,scope4}-files.txt` runs (39 and 80, 1, 60, 486, 486 and 5 roots).
2. `node --max-old-space-size=8192 node_modules/typescript/bin/tsc -p F/🗑️generated/tt3-next.tsconfig.json --noEmit --explainFiles` (proof that `next/server` alone does not load Next's globals); two probe programs under `tt3-tools/probe/` for the two ambient augmentations.
3. `zsh tt3-tools/tt3-vt.sh a dock|os <cwd> 🧪️tests/🎚️config/🟦️.ts [file]` (vitest, the `tsl-tools/vt.sh` policy shim) and `bun test ./<file>` for the files in section 4.
4. Scripted edits (anchors asserted, one snapshot per file): `tt3-tools/tt3_control_to_invocation.py`, `tt3_database_deserialize.py`, `tt3_sqlite_bindings.py`, `tt3_actor_grant.py`, `tt3_insert_props.py`. **Defect found and fixed on the way:** `tsc` columns count UTF-16 code units and Python offsets count code points, so on lines with emoji before the column my first offsets drifted; the bindings batch was restored from snapshots and rerun with `tt3_blocks.offset`, the grant and property batches were verified afterwards by a back-walk that checks the enclosing literal. One batch edit also ran on stale line numbers after a sibling edit shifted a file (`worker-law-fakes`), which produced a syntax error that `tsc` reported as the only diagnostics of `tt3-scope2`; repaired by hand, and the final scan of all 331 recently changed files is clean.
5. Read-only git (`diff`, `show HEAD:`, `log`, `grep`, `ls-files`, `status`). No `git stash`, `checkout`, `add` or commit. No `[DEBUG]` line was added.

## 8. Files changed

By family (about 150 files): root `tsconfig.json`; `🧰️framework/📦️packages/🟦️typescript/🌿️ambient/` (two new declaration files); `🏗️build/🕸️graph/📥️inference/🟨️.d.mts` and `🧪️test/🟨️.d.mts` (edited), `⚡️caching/📥️inference/🟨️.d.mts` (new); `🏃️process/📦️artifacts/🏗️native-build/🟦️.ts` (return type); six `stdio/semio` schemas and their six parsers; WFC `2d` inference and three `diff` parsers; CAD config schema; `os/🧪️tests/🧰️worker-law-fakes` plus 12 test files; 20 plugin and product `📜️script.ts` (invocation); 21 test files (sqlite bindings); the library, framework and plugin test files named above; `.storybook/main.ts`. Exact paths of every file I scripted are in the snapshot names under `🗑️generated/tt3-snap/` and in `tt3-tools/tt3-edited-by-snapshot.txt`.
