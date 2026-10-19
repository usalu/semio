# Result lw10a: WP10 part A, repo product to `framework/modules/process` deep imports

Executor `lw10a`. Scope: `🧰️framework/🛍️products/🦑️repo/**` importing into `🧰️framework/🔨️modules/🏃️process/**` (package `@semio-tech/process`, owner `…/🏃️process/📦️packages/🟦️typescript`). No `package.json`/`bun.lock` edit, nothing under `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/**` or `🏗️build/📥️inputs/🔏️revision/**`, no cargo, no git-modifying command, no compat barrel or re-export.

## 1. Totals (same dependency-cruiser config and rules as `result-lay0.md`)

Scoped run: `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework/🛍️products/🦑️repo 🧰️framework/🔨️modules/🏃️process --include-only '^🧰️framework/(🛍️products/🦑️repo|🔨️modules/🏃️process)/' --config …/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type json` (about 5 min each).

| measure | before | after |
|---|---:|---:|
| `no-cross-package-relative-@semio-tech/process`, repo product to process (edges, 1 violation per edge) | **1,203** (422 files) | **428** (233 files) |
| of which frozen storage/revision paths (untouched on purpose) | 19 | 19 |
| of which waiting for new `exports` entries (recorded) | – | 404 + 5 new edges from peer edits since the first run, 3 of those fixed in a second pass |
| whole repo product, `no-cross-package-relative` (all targets) | 1,696 | 911 (remainder targets other packages, lw10b) |
| whole repo product `no-circular` / `no-core-path` | 35 / 1 | 35 / 1 (not in my scope, unchanged) |

Second pass after the report: 3 further edges fixed (files added by peers), so the live residue is the frozen 19 plus 406 recorded edges (see section 3).

## 2. What changed

783 import edges in 368 files re-pointed (847 specifiers, all tool-generated through the TypeScript checker, `lw10a-tools/fix.ts`). Rule for the choice: an `exports` entry is used only when it already exposes every imported symbol with the identical declaration (alias-resolved through the checker; namespace imports and `const m = await import()` / `.then(({x}) => …)` are reduced to the members actually accessed). Two forms:

- **Files OUTSIDE the build CLI import closure (563 specifiers): bare workspace entry**, e.g. `@semio-tech/process/leases`, `…/execution`, `…/routing`, `…/compiler-capacity`, `…/build-mode`, `…/artifacts/native-build/storage/platform/contract`. Precedent: 5 framework module tests already import `@semio-tech/process/*` bare.
- **Files INSIDE the build CLI closure (284 specifiers): RELATIVE path to the export-target glue file** (`…/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts`), never bare. The CLI closure (entry `🏗️build/📜️script.ts`, 636 files, computed by `lw10a-tools/closure.ts`) must boot without `node_modules` (memory `feedback-bun-repo-zero-touch-and-lock-order`).

### CLI closure and the exemption question
No policy exemption is needed. The `no-cross-package-relative` selector allows `^<owner>/<export target>$` (`exportTargetPatterns` in `🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs`), i.e. a relative import that lands exactly on a manifest export target is conformant. So the closure keeps relative imports AND complies, which is strictly better than exempting. The `publicApi` taxonomy block is key-checked (`rulePrefix|sdkPackage|toolingRoles|version`), so it has no slot for an exemption anyway. Closure import edges that still point at a non-entry file (those listed in section 3) stay relative and deep until their entry exists; they are the only closure edges that violate.
Side remark: the ESLint mirror (`🧹️lint/📐️source-policy/🟨️.mjs`, warn level, name-based heuristic) flags any relative specifier naming the sibling `🏃️process`; unchanged behaviour for the closure files, it is not the authority.

## 3. Not fixable without manifest change: `exports-needed.json`

`lw10a-tools/exports-needed.json` (lay6 format) lists, per target file, the proposed subpath, glue file, union of imported symbols (`"*"` = namespace use that could not be reduced) and importer list:

- 86 new entries (`action: new-entry`), 16 entries whose glue file exists but does not re-export every needed symbol (`action: extend-existing-glue`; the glue is an authored `.ts`, only the missing re-exports have to be added), 406 importer edges. 38 of the rows are JSON files (`🔣️.json` schemas/fixtures, `kind: json`, proposed subpath `….json`): no export convention for JSON exists yet; owner decision (glue `.ts` that imports the JSON, or a direct JSON export key).
- Biggest rows (edges): `testing/invocation` 118 (`registerIsolatedBuildTestV1`, `registerBuildTestLawV1`, `withBuildTestInvocationV1`, `registerBuildTestFixtureV1`, `BuildTestFixtureV1`), `source/admission/read` 24, `leases` 23 (extend glue), `storage/platform` 16, `storage/image` 15 (extend), `storage/platform/contract` 14 (extend), `storage/schema.json` 9, `source/admission` 8, `units/profile` 8.
- Apply method for the manifest batch: add the manifest keys and glue exports, then `bun lw10a-tools/fix.ts <depcruise report> <closure list> apply <out.json>` finds the entries and rewrites the remaining 406 imports (bare outside, relative inside the closure).

## 4. Verification (exact commands, outcomes)

All from `/Users/ueli/Documents/semio`, foreground.
- Cruise before/after: numbers in section 1 (`agg.ts` aggregates; the cruise reports lived under `🗑️generated/lw10a-run`, deleted).
- tsc: `NODE_OPTIONS=--max-old-space-size=12288 node node_modules/typescript/bin/tsc --noEmit -p 🧰️framework/🛍️products/🦑️repo/tsconfig.json --pretty false --incremental false` after the edits: 6 errors, none about module resolution or a missing export (missing packages `strip-ansi`/`smart-buffer`, an arity error in an unrelated `store/🧪️tests` call, a `string|null` argument in `nx/📦️graph/🧪️tests`, an `IssuedScriptInvocationV1` argument-shape error in `discovery/🕸️runtime/🧪️tests`). The error lines are call-argument errors, not import specifiers; I did not capture a before-run, so they are classified by content, not by diff.
- Resolution: `Bun.resolveSync(replacement, dirname(importer))` for all 844 first-pass replacements resolved, 0 failures (`resolve-check.ts`).
- `bun repo --help` booted after the edits (printed usage, stage `complete`). Closure scan after the edits: 636 files, 2 bare `@semio-tech/*` imports, both in the process module (not repo product, not mine, see section 5).
- Bundle check: `Bun.build({target:"bun", packages:"external"})` of the 367 changed files (`bundle-check.ts`) validates that every named import exists on its new target: all failures were `Unterminated string literal` / `Expected "from"` from two files corrupted in flight by a concurrent peer edit (`⚡️caching/🦀️cargo/📦️units/📥️descriptor/🕸️nodes/🟦️.ts`, `…/📋️plan/🧪️tests/🔗️binding/🟦️.ts`: a peer's rewrite of imports on lines my edit did not touch; the second was repaired by the peer while I looked) and the `.d.mts` declaration file (false positive of the bundler). Zero "No matching export" errors.
- Tests: `bun test <file>` of a changed suite fails with `Original parent Script process invocation required` because suites must run under the `repo:script` envelope; unrelated to my change. Run through `NX_FORCE_REUSE_CACHED_GRAPH=true bun repo run repo:test-build-mode`: see section 6.

## 5. Open
- 406 recorded edges (section 3) plus 19 frozen-path edges.
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🟦️.ts` and `…/♻️retirement/🧭️continuation/🟦️.ts` (modified 21:31 by a peer) import bare `@semio-tech/framework/filesystem/observation/descriptor` and `@semio-tech/schema-validator-ts`. They are inside the CLI import closure (the process module is imported by `🏗️build/📜️script.ts`), so they break the zero-touch rule; the owner of the leases split should convert them to relative paths (`relativize-bare-imports.ts` does it).
- Two files under `⚡️caching/🦀️cargo/📦️units/**` were mid-edit by a peer with broken import lines during my run (see section 4); they must be re-verified by that peer.
- Peers keep adding deep imports into process (5 new edges appeared between my two cruises); re-run `fix.ts` with a fresh report after the manifest batch.

## 6. Test outcome
- `NX_FORCE_REUSE_CACHED_GRAPH=true bun repo run repo:test-build-mode` (13.6 min) fails in the dependency inspect with `Locked workspace package lacks original admitted Source` (`…/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:152`): workspace/lock state under concurrent package splits, unrelated to import specifiers; not retried.
- Sweep of all 167 changed `🧪️tests` suites with plain `bun test --timeout 60000 <file>` (`lw10a-tools/run-tests.sh`, 4 parallel): 42 pass, 125 fail. 100 fail with `Original parent Script process invocation required` and 4 more with the same family (`Original … required`: fixture/SDK/Source oracles that need the `repo:script` envelope), so they cannot run outside `bun repo run`. 22 failed with syntax errors (`Unterminated string literal`, `Expected "from"`) from the two peer-corrupted files of section 4; after the peer repaired them I re-ran those 22: all load and fail only with the same envelope family (sample: `graph` suite 93 pass / 77 fail, errors: 52 envelope, 8 `Original bounded Nx namespace required`, 4 ajv `strictRequired`, 3 `Required original cache reader unavailable`, 3 `Original Script deadline refused`, 1 missing `@iarna/toml`; the non-envelope ones are not import-resolution errors). No failure of the 167 suites is a module-resolution or missing-export error.
- Honest limit: because the envelope blocks the bulk, the runtime proof of the rewrite is `bun repo --help` booting, the bundle check (every named import exists on its target), tsc and the 42 passing suites; a full green suite run needs `bun repo run` once the dependency state is healthy.

## Tools kept (`lw10a-tools/`)
`fix.ts` (checker-driven rewriter, plan/apply), `closure.ts` (CLI closure scan), `exports-needed.ts`/`exports-needed.json`, `agg.ts`, `edges.ts`, `skipsum.ts`, `stars.ts`, `resolve-check.ts`, `bundle-check.ts`, `syntax-check.ts`, `one.ts`, `pkgs.ts`, `run-tests.sh`, `changed-files.txt`, `changed-tests.txt`, `cli-closure.txt`.
