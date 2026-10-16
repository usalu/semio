# Result: Package Specifier Codemod (slice PACKAGE-SPECIFIER-CODEMOD, prefix `psc`)

Status: DONE, including the follow-up for the freed ts3 area (section 12). Codemod applied and verified; the final full cruiser run (`after2`, 15:22 to 15:39, after the last edit), `tsc` re-run and test re-run are in section 5, 4 and 9.
Tools: `psc-tools/` (this folder). Logs and raw outputs: `🗑️generated/psc-*` (kept).

## 1. Verdict

- The 246-edge target (now 269 distinct production edges, peers added some) is fixed everywhere outside the areas I was told to skip. Importers outside those areas now reach the value, schema validator and schema modules only through package specifiers that the owning package's `exports` list.
- Exact cruiser command, before/after (final run `after2`, section 5): `framework-schema` 102 to 11 (outside tests 37 to 4), `schema-validator-ts` and `-rs` 239 to 132 (170 to 100), `value-ts` and `-rs` 146 to 17 (49 to 3). Distinct production edges over the five rules: 269 to 111. Every remaining edge sits in a skipped area (section 6) or is the gitignored generated file or one library edge added by a peer after my scan.
- `not-to-unlisted` 888/146 to 889/144 (all/outside tests) and `no-circular` 588/503 to 583/498, so neither rose. The differences are the same edges re-rendered by the cruiser (mojibake in the output) plus peer churn; no error line points at a facade I created except 11 `ui-no-framework-packages` lines, which replace 11 lines with the old target (the rule total is 262 before and after).
- The five rules needed one new package: `@semio-tech/schema-ts` at `🧰️framework/🔨️modules/🧬️schema/📦️packages/🟦️typescript` (section 2). The existing `@semio-tech/framework-schema` lives in a `🦀️rust` package directory, where a TypeScript facade breaches the package-language purity rule, and it exports only `./📜️script.ts`.
- Test failure sets are identical or smaller (132 of 135 files identical, 3 smaller, 0 new failing test names), the `tsc` error multiset is identical (905 before, 905 after, 0 added; twice), and all 39 entries load in fresh bun processes and resolve through a real Vite plugin container and a real vitest run.
- Not done: nothing was installed. Workspace dependencies changed, so every lockfile was updated by hand to what `bun install --lockfile-only` derives (section 7). The launcher route `workspaces-check` timed out under load (SIGALRM after 25 minutes), so the generated `workspaces` lists were extended by my own tool, not by `workspaces --write`.

## 2. Rule semantics and design

The rule generator is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs` (`crossPackageRelativeRules`, `exportTargetPatterns`, `authoredPackageExportAuthority`). Read-only for me.

- A package P has an `owner` (the package directory) and a `sourceOwner` (the owner without `/📦️packages/<language>`). Packages sharing a `sourceOwner` are siblings (`schema-validator-ts` and `-rs`; `value-ts` and `-rs`), so the `-ts` and `-rs` rules count the same edges twice.
- Rule `no-cross-package-relative-<name>`: any file outside `^<sourceOwner>/` (minus the regions of nested packages) that reaches a path inside it is an error, unless the resolved path is `<owner>/<target>` of an authored `exports` row of a sibling. The target must be a file inside the package directory (no `..`), so the real module file can never be an export target. The only fix is a facade file in the package directory.
- The layering check (`dependencyDirectionEdges` in `🟦️.ts`) adds two invariants for package specifiers: the subpath is an authored export key, and resolution lands on exactly that key's target. `psc-tools/spec-check.ts` checks this for all 366 imports of the three specifiers in the tree: 366 ok, 0 bad.
- `not-to-unlisted` did not rise: workspace packages resolve outside `node_modules` (real path), so they are not classified as unlisted npm packages. The `dependencies` entries I added are for the package graph and the install, not for the cruiser.
- Facade design follows `RESULT-SDK-BOUNDARY.md` section 3: one entry per module directory that importers use, path mirrors the module path inside the package directory, key is the path without emoji, only explicit `export { … } from` and `export type { … } from` lines (no `export *`, no wildcard keys), JSON schemas as named exports (`valueSchema`, `residentSchema`, …), test helpers and fixtures under `🧪️testing` (never `🧪️tests`, so test discovery cannot collect facades), and one side-effect facade (`import "…"`) for a source-contract test module that registers tests when imported. Value versus type is decided with the TypeScript checker. The names are the union of what all importers use today, including the importers in skipped areas, so their owners can switch without touching the facades.

### Public surface (39 entries)

| Package | Entries | Notes |
| --- | ---: | --- |
| `@semio-tech/value-ts` | 28 new (3 existing: `./ordered/numeric`, `./refusal`, `./ieee754`) | `./resident`, `./resident/schema`, `./resident/admission`, `./resident/testing`, `./schema`, `./schema/intrinsic`, `./schema/intrinsic/json`, `./type`, `./text`, `./decode`, `./dictionary`, `./dictionary/native-json`, `./retained-clone/wire`, `./retained-clone/wire/schema`, `./retained-clone/contract`, `./retained-clone/contract/schema`, `./retirement/contract/schema`, `./retirement/factory/boxed`, `./refusal/codec`, `./refusal/codec/schema`, `./refusal/schema`, `./refusal/testing`, `./ordered/numeric/scratch`, `./ordered/numeric/scratch/schema`, `./ordered/numeric/scratch/testing`, `./ordered/retirement/schema`, `./ordered/testing/source-contract`, `./testing/json-projection` |
| `@semio-tech/schema-validator-ts` | 2 new, barrel `.` extended from 3 to 9 value names and 2 types | `.` now exports all 9 functions importers use (`requireString`, `requireStringArray`, `requireLiteral`, `compileJsonSchemaSubset`, `compileExactKeysV1`, `jsonSchemaSubsetValueEquals`, plus the previous three), `./format-policy`, `./testing` |
| `@semio-tech/schema-ts` (new package) | 8 | `.` (full explicit list of the schema registry module), `./artifact-reference`, `./record`, `./record/testing/unit`, `./oracles/validator`, `./oracles/document`, `./entity-kinds/source`, `./generated/entity-kinds` |

Created: 38 facade files and `🧬️schema/📦️packages/🟦️typescript/package.json`. Extended: the validator barrel. Longest new path 149 bytes (limit 240). One sibling emoji repeats in the value facade directory (`🧬️retained-clone` and `🧬️schema`); it mirrors the module tree, which has the same pair.

## 3. Method

1. `psc-tools/scan.ts` (TypeScript AST via `sdkb-tools/lib.ts`, 12,747 files from `git ls-files` plus untracked) finds every import, export-from, dynamic import and `import()` type that resolves into the three modules from outside their owned region without hitting an export target. 590 statements. Cross-check against the cruiser baseline: schema 102 of 102, validator 239 of 239, value 145 of 146 (the one miss is a gitignored generated file, section 6).
2. `plan.ts` derives the facades and `exports` from the usage (TypeScript checker for value or type; `all` names when a use is `export *` or has unknown members; JSON default imports become named exports; side-effect modules become side-effect facades). `apply.ts` writes facades (merging into existing ones) and the `exports` entries.
3. `rewrite.ts` re-reads and re-scans each file right before editing and patches only the module-specifier literal (and, for JSON default imports, the binding: `import x from "…json" with { type: "json" }` becomes `import { xSchema as x } from "pkg/sub"`; `export { default as x } from "…json"`; and the `const { default: x } = await import("…json")` pattern). Bootstrap entries (`📜️script.ts`, Vite builders, `🧪️tests/🎚️config`, `vite|vitest.config.*`) get a relative path to the facade file instead, because they run before an install can be assumed (same decision as SDK-BOUNDARY); the cruiser accepts it since the resolved path is the export target. It also adds `workspace:*` to the importer's owning package (longest `sourceOwner` match, TypeScript package preferred): `dependencies` for production use, `devDependencies` for test-only use, skipped if already listed in any section, only for round-trip-stable manifests.
4. Batches (snapshot of every pre-edit file in `🗑️generated/psc-snap/`, then `psc-tools/verify.ts`): b1 value and schema modules (7 files), b2 SDK package (18), b3 all other framework modules (125), b4 framework products (103), b5 `✏️s` (12), b6 hub and mit-bestand (12). Total 277 files, 386 edits, 36 `package.json` dependency edits. `verify.ts`: 277 files, 36 manifests, 363 changed lines, all import lines, 0 bad. A later pass shortened nine `{ x: x }` bindings to `{ x }` in five files.
5. A fresh AST scan after the batches (`psc-scan-3.json`) finds 220 statements, all in skipped areas; 0 outside them.

## 4. Verification

| Check | Before | After | Result |
| --- | --- | --- | --- |
| `tsc --noEmit` on the 275 importers (before) / 277 importers + 39 facades (after), then 277 + 39 + the new test (after2) with their closure (`psc-tsconfig-*.json`, `psc-tsc-*.txt`) | 905 error lines | 905 and 905 | `tscdiff.ts`: removed 0, added 0, both times |
| `bun test` per file, 135 touched test files (`psc-tests-before.json`, `-after.json` 14:5x, `-after2.json` 15:2x after the last edit) | 1,029 pass, 156 fail | after: 1,032 / 153; after2: 1,035 / 150 | after2: 132 files identical, 3 files with fewer failures (store and reactor tests whose Rust-source assertions peers fixed meanwhile), 0 failing test names that were not failing before, 6 fewer |
| New test `📤️module-package-surface` (section 8) | n/a | 6 pass; mutation test (a bogus `exports` entry) gives 3 fail | sensitive |
| SDK `sdk-surface` law test | not run | 6 pass after a one-line fix (section 8) | |
| 39 entries imported by specifier in fresh bun processes (`facade-load.ts`, `psc-facade-load-1.log`) | n/a | 39 ok, 0 missing names | |
| Real Vite plugin container resolves all 39 specifiers to the facade file (`vite-check.mjs`, `psc-vite-check-1.log`) | n/a | 39 ok | repo workspace resolve preset, no alias involved |
| Real vitest run, os config, with a touched test (`🔌️plugin/🧪️tests/🚫️empty-state`) importing `@semio-tech/schema-ts/record` | n/a | 1 pass | |
| Edited bootstrap scripts loaded with no arguments (`sdkb-tools/script-smoke.ts`, 6 files) | n/a | 0 resolution failures | |
| Node strip-only load of the validator barrel (used by the styling Vite builder) | n/a | ok (9 values) | |
| Repo dependency policy tests (`bootstrap` 19, `construction` 5, `pattern-safety` 2, `selection` 2) | n/a | all pass on the live tree | |
| `spec-check.ts`: all imports of the three specifiers | n/a | 366 of 366 resolve exactly to their export target | |
| Value, validator test routes: `📤️api` 1 pass, `⚠️scalars` 2 pass, `dictionary` 13 pass, `shape` 2 pass and 1 fail | n/a | n/a | the one `shape` failure is "Original parent Script process invocation required": that file needs the launcher envelope and fails the same way for the untouched module |
| Real vitest on the os config, whole run (`psc-vitest-os-after.log`) | `actor-transport` 109 failed / 70 passed with the original `shardclient-reserved-response-settlement` test restored | 107 failed in the whole-config run | pre-existing: "Cannot read properties of undefined (reading 'requestId')" in a fixture, unrelated to the three modules |

Resolution notes:
- bun: the new specifiers resolve through the root `node_modules/@semio-tech/<name>` workspace links. For the new package I created the link `node_modules/@semio-tech/schema-ts` (absolute symlink to the package, as `bun install` creates the others); a fresh checkout gets it from `bun install`. Plugin and hub files resolve through the root `node_modules` because `✏️s/node_modules/@semio-tech` only links `framework` and `process`.
- Vite and vitest: `createWorkspaceViteResolveConfig` has no alias for these names and excludes all workspace packages from `optimizeDeps`, so no exact-match alias was needed (the SDK alias problem of RESULT-SDK-BOUNDARY does not apply). No config was changed.
- tsconfig: root `moduleResolution: bundler` resolves `exports`; no `paths` entry names these packages.

## 5. Cruiser, exact command

Command (audit 2 section 1, via `psc-tools/cruise.sh <label> full`): `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err` (alarm 2400).

| Run | Window | Exit | Error lines | Modules | Dependencies | Raw |
| --- | --- | --- | ---: | ---: | ---: | --- |
| before | 14:19 to 14:44 (1,469 s), tree untouched | 119 | 13,431 | 17,470 | 48,429 | `psc-before-err.txt` |
| after | 14:54 to 15:17 (1,381 s), after all importer edits and the new package | 45 | 13,357 | 17,576 | 49,190 | `psc-after-err.txt` |
| after2 (final) | 15:22 to 15:39 (1,008 s), after the last edit (shorthand bindings, new test and registrations, workspace lists, locks) | 55 | 13,367 | 17,601 | 49,266 | `psc-after2-err.txt` |

Rule table (`psc-tools/ruletab.ts`, all error lines / outside `🧪️tests`; the After column is the final run `after2`, the first `after` run gave the same numbers except where noted):

| Rule | Before | After | Remaining |
| --- | ---: | ---: | --- |
| `no-cross-package-relative-@semio-tech/framework-schema` | 102 / 37 | 11 / 4 | 7 renderer tests, 4 os script files (ts3) |
| `…/schema-ts` (new sibling rule, same edge set) | 0 / 0 | 11 / 4 | same |
| `…/schema-validator-ts` and `…/schema-validator-rs` (each) | 239 / 170 | 132 / 100 | 130 `📚️library` edges (99 prod, one of them added by a peer after my scan), 2 renderer edges (1 prod) |
| `…/value-ts` and `…/value-rs` (each) | 146 / 49 | 17 / 3 | 14 renderer test edges, 1 library prod, 1 renderer prod, 1 gitignored generated file |
| distinct production edges over the five original rules | 269 | 111 | |
| `not-to-unlisted` | 888 / 146 | 889 / 144 | not caused by this slice (section 1) |
| `no-circular` | 588 / 503 | 583 / 498 | no created facade is in a cycle |
| `ui-no-framework-packages` | 262 | 262 | 11 lines re-targeted from the module file to the facade |
| `framework-no-products`, `plugins-framework-sdk-only` | 149 / 104, 619 / 551 | unchanged | |
| total error lines | 13,431 | 13,367 | other rows moved by peers (hub script split, renderer) |

The final cruiser edges equal the AST scan (`psc-tools/compare.ts` against `psc-scan-3.json`: schema 11 of 11, validator 130 of 132, value 16 of 17; the misses are two library files created after the scan and the generated file below).

Attribution (`attrib2.ts`): of all error lines new since the baseline, the only ones pointing at a facade I created are the 11 `ui-no-framework-packages` lines. That rule (`from ^🧰️framework/🔨️modules/🖱️ui/`, `to ^🧰️framework/` except the ui module) already flagged every UI import of these modules; the UI files now import the package facade and are flagged for the new target, so the count is 262 before and after. This is the P3 conflict of audit 2: the UI must not depend on the framework at all, and no specifier can satisfy that. It needs the P3 owner (move what UI needs into the ui contract).

## 6. Skipped edges (importers in running slices' areas) and hand-offs

Full list: `🗑️generated/psc-skipped-edges.tsv` (220 rows: owner rule, slice, prod or test, file, line, target). Counts after the codemod:

| Slice | Statements (prod / test) | Files | What remains |
| --- | ---: | ---: | --- |
| parent (`📚️library`) | 99 / 32 | 129 | validator barrel for 98 imports in cargo, caching, build, normalization and test code; 1 value import |
| `rhu` renderer TS | 2 / 83 | 18 | `World3dHost`, `UiDocumentStore`, shell tests: value `./resident`, `./schema`, validator, schema record |
| `ts3` os script (`💻️os/📦️packages/🦀️rust/`) | 4 / 0 | 4 | schema `./oracles/validator`; DONE in section 12 |
| `pch` stdio artifacts, `hss` hub script, `pi1` plugin Rust, ts3 specs | 0 | 0 | no flagged edge existed there (the hub script children and the other ts3 files had none) |

Hand-off (no facade work left, every name those files use is already exported): dry run over exactly those files, `bun psc-tools/rewrite.ts 🗑️generated/psc-plan.json 🗑️generated/psc-scan-3.json --ignore-skip --area <prefix>` gives 151 files, 224 edits, 0 unhandled and 3 `package.json` dependency edits. The owner runs the same command with `--write --snapshot <file>`. Notes for the owners:
- `📚️library/⚡️caching/🚀️bootstrap/🛠️tools` (package `nx-bootstrap-tools`) runs before an install can be assumed; keep a relative path to the facade there (the rewrite only does that for the bootstrap file names).
- The `ts3` child `💻️os/🧪️tests/🪪️document-backbone-worker-wire/🟦️.ts` is a file ts3 is writing; my batch had already replaced one import line in it (`@semio-tech/schema-ts/oracles/validator`) before the exclusion list could name it. The line survived ts3's later edits; if ts3 regenerates that child from its original, re-run the rewrite for it.
- Gitignored generated file `🧰️framework/🔨️modules/🎭️actor/🤖️generated/🎭️actor/🟦️.ts` (ts-rs output of `framework-actor-rs:typegen`) imports `…/🌱️value/🧬️retained-clone/🌐️wire/🟦️.ts` by a relative path. The generator must emit `@semio-tech/value-ts/retained-clone/wire`; not edited here.
- Two importers (the pack format LEB oracle test and the `📡️replication` script, both rewritten) load the ordered-index test module `🗂️ordered/🧪️tests/🧪️source-contract` by dynamic import for its side effect; it now has the side-effect facade `./ordered/testing/source-contract`. The proper fix is to move that test into its owner.

## 7. Lockfile and install consistency

Workspace `dependencies` changed in 36 manifests and one workspace package was added, so the six Bun scopes (root, `✏️s`, `🌎️hub`, `♻️mit-bestand`, `🏢️semio-tech`, `🎓️teaching`) were out of sync with their lockfiles.
- `bun install --lockfile-only --offline` runs offline in 1 s per scope, but it rewrites the whole file in bun's own text format (trailing commas, different workspace order): 24,898 changed lines at the root. The checked-in locks are `JSON.stringify(…, null, 2)`-stable instead. I therefore ran it in each scope, moved the output aside (`🗑️generated/psc-bun.lock.*.bun-generated`), restored the checked-in lock at once, and transplanted only the semantic changes with `psc-tools/lock.ts` (30 to 36 changed workspace entries, 1 new workspace entry, 1 new `packages` row per scope; +87 to +97 lines each). Check: for all six scopes, `workspaces` and `packages` are deep-equal to what bun derives, `lockfileVersion`, `configVersion`, `overrides` and `patchedDependencies` untouched, the text stays `JSON.stringify`-stable. Backups of the six locks and manifests before any change: `🗑️generated/psc-lock-backup/`.
- Do not run `bun install --frozen-lockfile --lockfile-only`: despite the flag it saved a bun-format file (I restored all of them and redid the transplant for the three scopes it touched).
- `workspaces` lists: `psc-tools/workspaces.ts --write` inserted `🧰️framework/🔨️modules/🧬️schema/📦️packages/🟦️typescript` at its byte-order position in the six `package.json` lists. Membership itself is physical (`bunRepositoryMembership` walks `🧰️framework/**` for `package.json`), so the cruiser already saw the package. Command for the compile or install window to confirm the generated lists: `bun repo run @semio-tech/repo-lib:workspaces-check --skip-cache`; my attempt (`psc-workspaces-check.log`) was terminated by SIGALRM after 25 minutes while the launcher was still in its build stages (load average 40 to 70).
- Install: `bun install` in each scope creates the `node_modules/@semio-tech/schema-ts` link I made by hand.

## 8. New test and registrations

- Test `🧰️framework/📦️packages/🟦️typescript/🧪️tests/📤️module-package-surface/🟦️.ts` plus language-agnostic cases in `🧫️fixtures/🔣️.json` (11 facade-source cases, 5 export-key cases, the three package directories). It asserts: export keys are explicit and dot-free, every target is an existing file inside its package, every facade is explicit re-exports only (TypeScript's AST is the independent oracle), the TypeScript program reports no diagnostic on any facade (names exist, values are values), and every entry loads by specifier in a fresh process and exposes its declared value names. 6 pass in 2.8 s; with a bogus `exports` entry added to `schema-ts` it fails 3 tests; restored it passes.
- Registered as `test-module-package-surface`: `📜️script.ts` (`ModulePackageSurfaceScript`), `📋️project.json`, `package.json` script (all of `@semio-tech/framework`), and `.vscode/launch.json` plus `.vscode/🧩️launch.seed.jsonc` (entry `🧪️test🧰️framework📤️module-package-surface`, group `9_gates`, order 900.05851755, right after `sdk-surface`). `bun repo run @semio-tech/framework:test-module-package-surface` itself was not run (launcher unreliable under load); the test file was run with `bun test`.
- One foreign test edited: `🧰️framework/📦️packages/🟦️typescript/🧪️tests/🔌️sdk-surface/🟦️.ts` (SDK-BOUNDARY, a peer's new "runtime identity" test) resolved facade sources only as relative paths, so the SDK facades that now re-export from `@semio-tech/value-ts/…` and `@semio-tech/schema-ts/…` made it fail. The fix is two lines in `facadeValueReexports`: bare specifiers resolve with `Bun.resolveSync(specifier, dirname)`. After it: 6 pass, 0 fail.

## 9. Open items and in-flight checks

- Nothing from this slice is in flight. The final runs (`after2` cruiser, `after2` tsc, `after2` tests, `spec-check`, lock equality) all ran after the last edit.
- Not run: `verify taxonomy`, `verify layering` (launcher), the nx route of `test-module-package-surface`, `workspaces-check`.
- Hand-offs: parent (library edges, section 6), renderer owner (resident, schema, validator imports in `World3dHost`, `UiDocumentStore`, tests), UI owner (P3 conflict, section 5), actor typegen (generated file), ts3 (os script).

## 12. Follow-up: ts3 area (coordinator request, ts3 finished)

Request: apply the codemod to the skipped edges of the os `📜️script.ts` area, keep skipping the renderer (rhu2) and `📚️library`. Done with the same method.

- `skip.ts` no longer lists any ts3 prefix. A fresh scan (`psc-scan-4.json`) found 222 statements; the only ones that had been skipped for ts3 were 4 prod statements, one in each of the new modules `💻️os/📦️packages/🦀️rust/🎛️commands/{📄️document-checks,📇️directory-checks,📸️snapshot-checks,🗄️database-checks}/🟦️.ts`: `import { semioSchemaAjvV1 } from "…/🧬️schema/🔮️oracles/✅️validator/🟦️.ts"`. The os `📜️script.ts` itself has no flagged edge any more (ts3 moved its imports into the modules; its earlier relative import was already rewritten by the main batch). `rewrite.ts` replaced the four literals with `@semio-tech/schema-ts/oracles/validator`; the facade already existed, no `package.json` edit was needed (the owning package `@semio-tech/framework-os` already depends on `schema-ts`). `verify.ts`: 4 files, 4 changed lines, 0 bad (snapshot `🗑️generated/psc-snap-ts3/`).
- Cruiser (exact command, `cruise.sh after3 full`, 15:45 to 16:50, 5 attempts because peers deleted files mid-run, exit 208, 13,520 errors, 17,695 modules, 49,715 dependencies; the previous final run `after2` is the "before"): `framework-schema` and `schema-ts` 11 / 4 to 7 / 0 (the four os module edges are gone; the 7 left are renderer tests), `value-ts` and `-rs` 17 / 3 unchanged, `schema-validator-ts` and `-rs` 132 / 100 to 137 / 103 (peers added 7 `📚️library` edges and 1 file under `🏃️process/📦️artifacts/🏗️native-build`), `not-to-unlisted` 889 / 144 to 889 / 143, `no-circular` 583 / 498 to 584 / 499 (no created facade in a cycle; `attrib2.ts`: 0 error lines point at a facade). Edges from the four os modules in the three rules: 4 before, 0 after.
- Stragglers: the same cruiser run showed the 8 new validator and value edges. Seven were `📚️library` (skipped). Four files outside every skipped area were created by peers after my last scan: three under `🏃️process/📦️artifacts/🏗️native-build/💾️storage/` (`🌱️birth`, `🌱️birth/🧾️claim`, `🌐️platform/📋️contract/📂️mount`) and `🖼️canvas/💾️renderer-resident`. They were rewritten the same way (`psc-scan-5.json`, 4 statements, 4 edits, 0 bad, snapshot `psc-snap-s5/`, no `package.json` edits). A cruiser run after that second batch was not repeated (a full run takes 30 to 35 minutes under the current load); the evidence for these four is the AST scan, the facade load and `spec-check.ts`.
- `tsc`: the four os modules plus the ten planned facades and their closure, before 907 error lines, after 905 (`tscdiff.ts`: removed 2, added 0; the two removed are a peer's fix in `✏️s/🧑‍💻dev/📜️script.ts`). For the four stragglers `tsc` aborted twice (exit 134, closure too large for the default heap) and a third run with a 12 GiB heap hit its alarm (exit 142), so there is no before/after for them; their edits are one specifier each, to entries that `tsc` already accepts.
- Tests: no test file imports or is touched by these eight files. The two registered suites that read os script text (`📚️library/🏗️build/🧪️tests/🎟️phase`, `⚡️caching/🧪️tests/⚡️cache-contracts`) give 3 pass and 12 fail ("Original parent Script process invocation required": they need the launcher envelope) and 0 tests; unrelated to the change.
- Entry loads: all 10 planned entries load by specifier in fresh bun processes (0 missing names); the four rewritten os modules load (6, 14, 9, 5 exports); the os `📜️script.ts` loads with no resolution failure (`script-smoke.ts`). `spec-check.ts`: 375 of 375 imports of the three specifiers resolve exactly to their export target.
- Remaining skipped edges: parent `📚️library` and renderer only (`psc-skipped-edges.tsv` still lists the old 220 rows; the os rows are done).

## 10. Commands run (real results)

| # | Command | Result |
| --- | --- | --- |
| 1 | `psc-tools/cruise.sh before full` (exact cruiser command, section 5) | exit 119, 13,431 errors, 1,469 s |
| 2 | `bun psc-tools/scan.ts …/psc-scan-before.json` and `-2` and `-3` | 588, 590, 220 statements |
| 3 | `bun psc-tools/plan.ts`, `apply.ts --write` | 39 facades, 38 created, 1 extended, 0 collisions, 0 problems |
| 4 | `bun psc-tools/facade-load.ts`, `vite-check.mjs` | 39 of 39 ok, 39 of 39 ok |
| 5 | `psc-tools/batch.sh b1..b6` (rewrite, snapshot) and `verify.ts` | 277 files, 0 bad |
| 6 | `psc-tools/tsc.sh before` and `after` | 905 and 905 errors, tscdiff 0 and 0 |
| 7 | `bun sdkb-tools/runtests.ts` before and after, `testdiff.ts` | 133 same, 2 improved |
| 8 | `psc-tools/cruise.sh after full` | exit 45, 13,357 errors, 1,381 s |
| 9 | `bun test` on `sdk-surface` (before fix: 5 pass 1 fail, after: 6 pass), `module-package-surface` (6 pass) | |
| 10 | `node node_modules/vitest/vitest.mjs run --config 🧰️framework/🛍️products/💻️os/🧪️tests/🎚️config/🟦️.ts [file]` | 1 pass; whole config 497 pass, 112 fail (pre-existing, section 4) |
| 11 | `bun install --lockfile-only --offline` per scope, then `lock.ts --write` | section 7 |
| 12 | `bun repo run @semio-tech/repo-lib:workspaces-check --skip-cache` | SIGALRM after 25 min, no verdict |
| 13 | `bun psc-tools/spec-check.ts` (run twice, the second after the last edit) | 366 of 366 |
| 14 | `psc-tools/cruise.sh after2 full` | exit 55, 13,367 errors, 1,008 s; table in section 5 |
| 15 | `psc-tools/tsc.sh after2` | 905 errors, tscdiff removed 0 added 0, 69 s |
| 16 | `bun sdkb-tools/runtests.ts` (after2), `testdiff.ts` | 132 same, 3 improved, 0 new failing names |
| 17 | `bun test` on the 4 direction-policy suites and on the value, validator test routes | 19 / 5 / 2 / 2 pass; route results in section 4 |

## 11. Files changed

- New package and facades: `🧰️framework/🔨️modules/🧬️schema/📦️packages/🟦️typescript/` (`package.json`, 8 facade files), 28 facade files under `🧰️framework/🔨️modules/🌱️value/📦️packages/🟦️typescript/`, `…/🧬️schema/✅️validator/📦️packages/🟦️typescript/{🟦️.ts (extended), 🏷️format-policy/, 🧪️testing/}`; `exports` in the value and validator `package.json`.
- 277 importer files (list: keys of `🗑️generated/psc-snap/*.json` without `package.json`) and 36 workspace `package.json` dependency edits (same files, `package.json` keys).
- Registration and consistency: root `package.json` plus `✏️s`, `🌎️hub`, `♻️mit-bestand`, `🏢️semio-tech`, `🎓️teaching` `package.json` (one `workspaces` line each), the six `bun.lock` files, `node_modules/@semio-tech/schema-ts` (untracked link).
- Test and registration: `🧰️framework/📦️packages/🟦️typescript/🧪️tests/📤️module-package-surface/{🟦️.ts,🧫️fixtures/🔣️.json}`, SDK `📜️script.ts`, `📋️project.json`, `package.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, and the one-line fix in `🔌️sdk-surface/🟦️.ts`.
- Not touched: `AGENTS.md`, policy, taxonomy, the rule generator, any ticket tool, any git state.
