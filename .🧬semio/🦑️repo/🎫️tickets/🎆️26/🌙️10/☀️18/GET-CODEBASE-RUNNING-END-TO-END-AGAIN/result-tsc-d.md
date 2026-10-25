# result-tsc-d (wave 1025e, executor tsc-d)
FINAL 13:10 CET (resumed 11:40): ALL 14 strict configs type-check with 0 errors, re-run after expd-d/expc-d added their facade modules and export keys (files = proof the checker ran): framework 7216, repo-lib 7228, repo 7882, coordinator 936, repo test 1272, ui react 2984, locale 591, intro 460, ui rust 556, 3d 852, assets 594, machine 472, compute 479 (all rc 0).
Fixes since the pause (all at the source): `import type { type NativeBuildCallerCapabilitiesV1 }` (TS2206) in `🏗️build/🔧️tools/🏗️bazel/📜️script.ts` and `.../🧪️tests/📥️purpose/🌱️birth/🟦️.ts` (my edit); duplicated `directoryAdmission` property (TS1117) in `🧹️normalization/🔣️taxonomy/🏭️generators-packages-and-exclusions/🟦️.ts` line 152 (double paste, removed one). Transient peer-owned reds seen mid-run and gone on re-run: `holdServiceInvocationV1` gained a required `bootstrap` port (callers/tests updated by the peer), `nxBuildDependencyImplementationV1` moved from `build/inference` to the new `build/nx-plugin` export (3 tests re-pointed by the peer at 12:02).
Not mine (per coordinator): `lint-dependency-direction` verdict and its edge findings (expd-d/expc-d); cruiser gate retired by verify-d. `🗑️generated/tsc-d` deleted; tools stay in `tsc-d-tools/` (`chain3.sh` = all configs, `run.sh <cfg> <tag>`).

STATUS 06:30 CET: TypeScript of `🦑️repo/**` and `🔨️modules/**` type-checks CLEAN under strict mode (before: 12 errors in 2 configs; after: 0 errors except two re-checks still pending after the last 3 edits, see below). Law lane `repo-lib:lint-dependency-direction` is queued for capacity (5.6 GB, 8-14 ahead; two earlier attempts died: bazel run killed 'Original Script process terminated' after 24 min of dependency wait, nx run EMFILE host-wide).

## Method (official routes, never a global tsc)
`bun repo tools exec tsc -- -p <cfg> --noEmit --incremental --tsBuildInfoFile <C/🗑️generated/tsc-d/..> --pretty false --extendedDiagnostics` (tsc 5.9.3 registered tool; `typecheck` of `@semio-tech/repo-lib` runs exactly `repo/tsconfig.json` + coordinator, the `framework` target `🧰️framework/📦️packages/🟦️typescript/tsconfig.json` covers `🔨️modules/**`). Scripts: `C/tsc-d-tools/{run.sh,chain1.sh,chain2.sh,law.sh}`; output `C/🗑️generated/tsc-d/` (delete at the end).

## Error count per config (files = proof the checker ran)
| config | before | after |
| --- | ---: | ---: |
| repo (`🦑️repo/tsconfig.json`, 7713 files) | 11 | 0 (re-run 7717 files rc 0) |
| framework (covers `🔨️modules/**`, 7784 files) | 1 | 0 at fix time; re-run saw 1 transient: peer-created `🏃️process/🧪️testing/🗂️suites/🟦️.ts` imported `./🔣️.json` 3 min before the file existed (file exists now) |
| repo library (7072 files) | n/a | 1 = peer's new `🧮️fact-codec/🧪️tests` (fixed, below) |
| coordinator, repo test, ui react, locale, intro, ui rust, 3d, assets, machine, compute | 0 | 0 (rc 0) |

## Fixes (each at the source, no any/ts-ignore)
1. `🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.d.mts`: message-key union missed `swept`, `planned`, `summary` that `🟨️.mjs` defines (TS2345 in the child-planned graph message).
2. `🏗️build/🧭️invocation/🕸️graph/🟦️.ts`: `loadBuildInvocationGraphV1` typed its reader as `typeof (read ?? loadBuildGraphV1)` (a union whose parameters intersect); now exported `BuildInvocationGraphLoadOptionsV1` / `BuildInvocationGraphReaderV1` name the reader contract once (2 TS2345).
3. `⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🔣️.json`: graphCapacity.policy mirrors the schema constant again (`yieldWaitMilliseconds` 20000, `capacityWaitMilliseconds` 900000, `capacityPollMilliseconds` 5000; the schema `Policy.const` gained them at 03:49; the test's `toEqual` would also have failed at run time).
4. `🧪️tests/⚡️production-cache-input-boundary/🟦️.ts`: `publishNxCollectionBoundary` takes the phase control since the nested-loader fix; the test issues it from the narrowed invocation (`createIssuedResourceControlV1(invocation,"test:collection-boundary",0)`).
5. `⚡️caching/💾️storage/🧾️ledger/🧪️tests/📏️needs/🟦️.ts`: the reclaim rule lookup is `?.reclaim` with an explicit refusal (4 TS2532/TS18048).
6. `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧪️tests/🟦️.ts` and `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️whole/🧪️tests/🟦️.ts`: fixture verdict strings compared as `ReturnType<typeof planner>` (JSON strings vs literal unions).
7. `🏗️build/🧪️tests/🕸️graph/🔌️nx-engine/🟦️.ts`: `dependencies as Record<string,object[]>` before `.flat()`.
8. `🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🧪️tests/🟦️.ts` (peer's file, idle 30 min): `schema.definitions.Policy.const as typeof FACT_CODEC_POLICY_V1` (JSON `version:number` vs literal `1`).
(`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` for the paths under items 1, 2, 3 to 8 except item 6b.)

## Laws that guard imports
- `lint-dependency-direction` (dependency-cruiser over the taxonomy-owned rules): queued, result below when it lands.
- Legacy gate in `verify gate` (`dependency-cruise 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config 🧹️lint/🕸️dependency-boundaries/🟨️.cjs`, config dated 10-20): RED with 2790 violations over 24034 modules (819 + 482 + ... `no-cross-package-relative-*`, 167 `no-circular`, 108 `no-core-path`, 67 `plugins-framework-sdk-only`, 67 `no-cross-technology-✏️s-to-🧰️framework`, 18 `framework-no-products`, 12 `renderer-hosts-only-ui`): the config predates the workspace-package imports and the taxonomy-owned lane; NOT fixed here (needs a decision: retire it from `runGate` in favour of `lint-dependency-direction`, or rewrite its rules; the 2790 are not import regressions of the last day). Note for a decision owner. Plain `node` OOMs (4 GB) on this graph; use `--max-old-space-size=8192`.

## Open
- Re-run of framework + repo library configs after the last three edits (queued behind the law).
