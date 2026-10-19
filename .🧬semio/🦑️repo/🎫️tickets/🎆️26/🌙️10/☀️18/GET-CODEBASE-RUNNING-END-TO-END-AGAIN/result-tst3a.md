# Result tst3a: bun suites of the repo product (`🧰️framework/🛍️products/🦑️repo/**`)

Executor tst3a, foreground, no sub-agents, no cargo, no modifying git command, no `package.json`/`bun.lock` edit, no `bun repo run`. Machine load average was 50-80 for the whole session (fleet), so deadline-based laws flip between runs. Paths below are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/` (`📚️library/` = `L`).

Tools (`tst3a-tools/`): `run.sh <tag> <list>` (2 shards, `runbun.py` with the repot2 envelope: launcher-minted `SEMIO_SCRIPT_PROCESS_INVOCATION` + providers, `SEMIO_TEST_ARTIFACT_DIR`, emulated hoist, 295 s cap per suite), `t.sh <path>` single suite, `tri.py`/`first.py` triage, `birth.py` (codemod probe, reverted).

## Before / after

| | suites run | green | tests pass / fail |
|---|---|---|---|
| before (naive run of the current tree) | 326 | 162 | 2,403 / 760 |
| after (re-run of every red suite + the later edits) | 326 | 181 | 2,355 / 703 |

557 files under the product import `bun:test` (`tst3a-tools/list.txt`); 326 were run (every suite that was red or new in the repot/tst2 passes plus the first 106). **231 were not re-run** (they were green in tst2/repot final passes and are neither on a repot red list nor new; not run this session, assumed unchanged). Several "now green" suites were load flakes (deadline laws), not my edits; the real fixes are in the next table.

## Fixed (suite, before -> after, cause, fix)

| suite | before | after | cause | fix |
|---|---|---|---|---|
| `L🏗️build/👁️observation` | 50/2 | 51/1 | production regression: a reused persisted project image re-validated all its facts with the unshared `current(...)`, so provider sharing (reads 6 vs 2) never happened | `observeProjectInferenceV1` uses the shared consistency check when a parent inference exists (`🟨️.mjs`); the last failure is Bun 1.3.14 TOML `\U` (deps lane) |
| `L🏗️build/🛠️cmake` | 0/1 | 7/0 | (1) law compiled a 2020-12 definition with draft-07 Ajv; (2) production defect: `runCmakeOwnedStageV1` passed the full launcher transport ceilings (128 MiB / 1M lines) to a separated capture that refuses more than 16 MiB / 65,536 lines | law uses `Ajv2020` for `NativePresets`; stage bounds the capture output to the capture ceilings (`🎬️stage/🟦️.ts`) |
| `L🧪️tests/🏺️historical-package-owner-identity`, `☂️frozen-coordinate-wildcard-coverage` | 24/2, 4/1 | 26/0, 5/0 | the frozen census `🧪️tests/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json` had been rewritten again (a path codemod re-pointed the historical `os/…/r3f` coordinates, 239,163 vs 239,319 bytes) | restored byte-identical from the 0c3e789 revision (sha `c3e39e…`) |
| `L🧪️tests/❄️frozen-markdown-coordinates` | 34/2 | 35/1 | seal ledger still pinned four contracts that the taxonomy deliberately no longer registers (`cad-draw-projection-vectors-v1`, `readme-current-source-revision-input`, `readme-reviewed-expectation-input`, `remaining-package-purity-history-v1`) | the four rows removed from `🧫️frozen-seal-ledger/🔣️.json`; remaining failure: producer static import `@semio-tech/process/build-mode` lacks exact workspace authority (exports/deps owner) |
| `L🧪️tests/🧾️registry-catalog-gitlink-boundary`, `🛫️preflight-reference-basis` (+ gate) | 3/1, 25/5 | 4/0, 30/0 | the cache owner's mass flip made 172 of 184 library test targets `cache:true` with file-only inputs; these two gates read the git index/worktree, so the laws that require "never replayed from cache" are right | `test-registry-catalog-gitlink-boundary` and `test-preflight-reference-basis` set back to `cache:false` in `📦️packages/🟦️typescript/📋️project.json` (**the regenerator must keep them uncached**) |
| `L🧪️tests/🛫️preflight-reference-basis` (physical cases) | | | law extracted `embeddedFixture` from `🔄️transaction-v2/🟦️.ts`, which moved to `🔄️transaction-v2/🏗️fixture-builders/🟦️.ts` as an exported function | law reads the new owner and strips `export` |
| `L🧪️tests/🫙️artifact-empty-facet-authority`, `↪️rust-divergence-callback`, `✍️rust-writable-path-authority` | 4/1, 44/4, 42/5 | 5/0, 47/1, (cargo) | target registration compared with exact `toEqual`, now carries `cache`/`inputs`/`outputs` | `toMatchObject` on executor/options (+ vector `cache:true` for empty-facet); the remaining rows run `cargo` (cargo-gated) |
| `L🧪️tests/🧱️fixture-law-ownership` | 1/1 | 2/0 | same cache flip | asserts `cache===false` or declared `reads` input |
| `L🧪️tests/🧱️root-clean-scaffold-source` | 7/1 | 8/0 | protection owner gained `CLEAN_NATIVE_IMAGE_BACKING_NAMES`, `cleanIsNativeImagePath`, `cleanOpenTicketGeneratedOutputProtection` | fixture declarations updated |
| `L🧪️tests/🏭️generated-source-topology` | 1/1 | 2/0 | five `plugin-registry` rows pointed at `🔌️plugin/📇️registry/🤖️generated` while the contract's output root is `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated` | rows re-pointed (files exist there) |
| `L🧪️tests/🔬️workspace-contract/🔭️package-discovery` | 18/1 | 19/0 | the writer moved to the hub composition: marker role is `hub` | law expects `hub` |
| `L🧪️tests/🔬️micro-commit-staged-index` | 0/1 | 1/0 | law imported `L/🟦️.ts`; `runMicroCommit` lives in `🎛️commands/📝️commit/🪙️micro-commit/🟦️.ts` | path fixed |
| `L🧪️tests/🛟️transaction-recovery-authority` | 1/5 | 4/2 | registration is `repo:script` now; extracted source needs the `requireTaxonomyValue` adapter | `toMatchObject` + adapter; remaining 2: git index oracle reads 2 instead of 1 (unchanged-index) |
| `L🧪️tests/🧱️cargo-transaction-command-source` | 7/2 | 8/1 | owner types moved out of `owned-execution`; hub check-in consumer imports `@semio-tech/process/native-build` | fixture declarations/owner updated; 10 more consumer rows still name library owners that moved to the framework process package (re-census needed) |
| `L🧪️tests/🎚️tool-configuration-ownership` | 5/4 | 5/4 | stale tokens (serve owner, lint `.cjs` now referenced by `🎛️commands/🧹️code-style`, moved fixtures, removed ShellHost/playground rows) | fixture rows corrected; remaining 4: vite bounded cache (launcher), Playwright list count, `@tailwindcss/postcss` |
| `L🧪️tests/🧑‍💻os-dev-composition-ownership` | 3/8 | 4/7 | importer moved to `…/🧪️ticket-owned-browser-host-staging/🧰️test-kit/🟦️.ts` | inline law path; the owner census (47 owners) needs a re-run by its owner |
| `L🧪️tests/✂️node-native-typescript` | 1/2 | 2/1 | (1) law scanned tracked ticket TS files (a closed ticket vite config imports a moved path); (2) production: `PayloadReadView` used a constructor parameter property (not erasable) | law skips `.🧬semio/`; `⏯️tool-run/📨️payload/🌊️stream/🟦️.ts` declares the field; remaining: workspace link `@semio-tech/course` unresolvable |
| `L⚡️caching/🦀️cargo/📦️units/🧪️tests/📐️profile/♻️preparation` | 1/2 | 1/2 | child program imported the validator by a relative path that resolves against the eval cwd | absolute `path(...)`; next layer: the child imports a test-only module (`Cannot use test outside of the test runner`) |
| `🖥️server/🎛️coordinator/🏗️builder/🏃️execution` | 0/1 | 0/1 | defining-source closure 897 > ratchet 192 (was 180 on 10-19 morning) | the script now imports the four owner modules instead of the `repo-lib` barrels (`📜️script.ts`); closure still 897 (design carve needed) |

Reverted experiment: codemod `receiveScriptProcessInvocation(process.env,…)` -> `withBuildTestInvocationV1(…)` on four suites with the "Original process birth already received" class (vite storage, compiler units, cargo units, artifacts physical): the births stop but the laws then time out or cascade on the same shared Original (no suite turned green), so the edits were restored to the working-tree versions.

## Remaining red (by class; `before -> after` pass/fail)


### SOURCE-LAW-DRIFT/OTHER (51)

| suite | before | after | first error |
|---|---|---|---|
| `💻️client/🧩️vscode/🔗️graphql-types/🟦️.ts` | 5/1 | 5/1 | error: expect(received).toBe(expected) |
| `⚖️laws/📇️indexed-generated-output/🗂️indexed-generated-output/🟦️.ts` | 0/1 | 0/1 | error: expect(received).toEqual(expected) |
| `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/📥️required/🟦️.ts` | 7/2 | 7/2 | error: expect(received).toBe(expected) |
| `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🏗️native-build/🟦️.ts` | 20/3 | 0/0 | error: An unknown error occurred (Unexpected) |
| `⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/📥️inputs/📋️data/🟦️.ts` | 10/1 | 10/1 | error: Required original cache consumer unavailable |
| `⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/📚️cohorts/🟦️.ts` | 2/1 | 2/1 | error: expect(received).toBe(expected) |
| `⚡️caching/📥️inference/🟦️.ts` | 2/1 | 2/1 |  |
| `⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🟦️.ts` | 7/11 | 0/1 | error: ENFILE reading "/Users/ueli/Documents/semio/.cache/b/d/m/j/441fd20cf4b20f6dcafe/node_modules/.bun/ajv@8 |
| `⚡️caching/🕸️graph/🔏️seal/🎯️selection/🟦️.ts` | 0/1 | 0/1 | error: Actual incoming immutable Nx tooling identity required |
| `⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts` | 15/1 | 16/1 |  |
| `⚡️caching/🦀️cargo/📦️units/📐️profile/🟦️.ts` | 6/1 | 6/1 | error: expect(received).toBeLessThan(expected) |
| `⚡️caching/📚️library-search-path/🟦️.ts` | 1/1 | 1/1 | error: expect(received).toBeGreaterThan(expected) |
| `⚡️caching/🔗️import-edges/🏛️graph/🟦️.ts` | 0/2 | 0/2 |  |
| `⚡️caching/🗿️artifact-source/🟦️.ts` | 0/1 | 0/1 | error: Materialized absolute path exceeds 256 characters |
| `⚡️caching/🛑️coordinator/🔮️oracle/🟦️.ts` | 0/1 | 0/1 | SyntaxError: import.meta is only valid inside modules. |
| `⚡️caching/🧹️pruning/🏪️budget/📜️history/📏️recensus/🟦️.ts` | 0/1 | 0/1 |  |
| `🏗️build/👁️observation/🟦️.ts` | 50/2 | 51/1 | error: expect(received).toBe(expected) |
| `🏗️build/📥️inputs/💾️storage/🚪️admission/🟦️.ts` | 0/2 | 0/2 | error: expect(received).toBe(expected) |
| `🏗️build/📥️inputs/📏️windows/🟦️.ts` | 2/2 | 2/2 | SyntaxError: JSON Parse error: Unexpected EOF |
| `🏗️build/🔧️tools/💾️storage/🟦️.ts` | 2/1 | 2/1 | error: expect(received).toEqual(expected) |
| `🔍️discovery/🔬️rust-policy-source-evidence/🟦️.ts` | 5/1 | 5/1 | error: 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🧪️tests/🔬️unit/🦀️.rs |
| `🕸️dependencies/🧭️direction/🧩️compute-constructor/🟦️.ts` | 1/3 | 1/3 | error: expect(received).toBeDefined() |
| `↪️rust-divergence-callback/🟦️.ts` | 44/4 | 47/1 |  |
| `✍️rust-writable-path-authority/🟦️.ts` | 42/5 | 0/0 |  |
| `❄️frozen-markdown-coordinates/🟦️.ts` | 34/2 | 35/1 | error: Producer static import lacks exact workspace authority: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️libra |
| `🍃️artifact-support-leaf-authority/🟦️.ts` | 5/14 | 5/14 | error: expect(received).toBe(expected) |
| `🎚️vitest-configuration-ownership/🟦️.ts` | 3/4 | 6/1 | error: 🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts |
| `📏️lib-wiring-budget/🟦️.ts` | 1/2 | 1/2 | error: expect(received).toEqual(expected) |
| `📜️historical-document-evidence/🟦️.ts` | 6/1 | 6/1 | error: expect(received).toEqual(expected) |
| `📱️app-verification-source-ownership/🟦️.ts` | 2/4 | 2/4 | error: 🌎️hub/🧩️compositions/🌍️gis/📦️packages/🦀️rust/📜️script.ts |
| `🔖️readme-current-source-revision/🟦️.ts` | 31/2 | 31/2 | error: expect(received).toBe(expected) |
| `🔗️markdown-inline-references/🟦️.ts` | 0/1 | 0/1 |  |
| `🔬️workspace-contract/🎫️ticket-protection/🟦️.ts` | 2/1 | 2/1 |  |
| `🔬️workspace-contract/📖️taxonomy-loading/🟦️.ts` | 54/2 | 54/2 | error: Undeclared generator stderr refused |
| `🔬️workspace-contract/📦️package-language-handoff/🟦️.ts` | 1/5 | 1/5 | error: expect(received).toHaveLength(expected) |
| `🔬️workspace-contract/🔗️registry-and-cargo-providers/🟦️.ts` | 5/1 | 5/1 | error: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/ |
| `🔬️workspace-contract/🧬️mutation-taxonomy/🟦️.ts` | 13/3 | 13/3 | error: expect(received).toBe(expected) |
| `🔬️workspace-contract/🧳️generated-outputs/🟦️.ts` | 3/4 | 3/4 | error: expect(received).toBe(expected) |
| `🚚️readme-move-source-authority/🟦️.ts` | 13/4 | 13/4 | error: revised-exact-authority: Error: taxonomy plan moves[0].sourceAuthority has an unsupported current-sourc |
| `🛟️transaction-recovery-authority/🟦️.ts` | 1/5 | 4/2 | error: unchanged-index |
| `🥾️cross-platform-bootstrap/🟦️.ts` | 8/6 | 12/5 | error: expect(received).toEqual(expected) |
| `🧑‍💻os-dev-composition-ownership/🟦️.ts` | 3/8 | 4/7 | error: 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️browser-host/🏗️staging/🟦️.ts |
| `🧪️mutation-leaf-identity/🟦️.ts` | 9/1 | 9/1 | error: descriptorless-helper-module |
| `🧪️storybook-discovery/🟦️.ts` | 1/4 | 1/4 | error: ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📖️stor |
| `🧰️framework-root-source-topology/🟦️.ts` | 3/1 | 3/1 | error: expect(received).toBe(expected) |
| `🧱️cargo-transaction-command-source/🟦️.ts` | 7/2 | 8/1 | error: 🌎️hub/🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🟦️.ts -> ⚡️caching/📦️artifacts/🏗️native-build/🟦️ |
| `🧱️owned-script-routes/🟦️.ts` | 1/3 | 1/3 | error: expect(received).toBe(expected) |
| `🧱️rust-source-direction/🧫️fixture-ownership/🟦️.ts` | 1/1 | 1/1 | error: 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🛂️mutation-source-authority/🟦️.ts |
| `🧱️workspace-publication-source/🟦️.ts` | 9/1 | 9/1 | error: expect(received).toEqual(expected) |
| `🧹️normalization/📦️package-boundary-classification/🟦️.ts` | 107/13 | 108/12 | error: expect(received).toBe(expected) |
| `🖥️server/🎛️coordinator/🏗️builder/🏃️execution/🟦️.ts` | 0/1 | 0/1 | error: expect(received).toBeLessThanOrEqual(expected) |

### LAUNCHER/ORIGINAL-GATED (71)

| suite | before | after | first error |
|---|---|---|---|
| `🎛️dashboard/🌀️control-plane/🟦️.ts` | 9/3 | 9/3 | error: Original bounded Nx namespace required |
| `🎛️dashboard/🧊️execution/🟦️.ts` | 0/6 | 3/6 | error: Original process birth already received |
| `⚡️caching/🌐️vite/💾️storage/🟦️.ts` | 3/4 | 3/4 | error: Original process birth already received |
| `⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts` | 4/1 | 4/1 | error: Nested persistent image requires actual parent scope |
| `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/📥️phase/🟦️.ts` | 3/4 | 3/4 | error: Authored phase fixture output required |
| `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts` | 10/7 | 10/7 | error: Original process admission closed |
| `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📐️layout/🍎️physical.ts` | 0/2 | 0/2 | error: Nested persistent image requires actual parent scope |
| `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🟦️.ts` | 1/1 | 1/1 | error: Nested persistent image requires actual parent scope |
| `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts` | 2/1 | 2/1 | error: Original process birth already received |
| `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️parent/🟦️.ts` | 22/26 | 19/29 | error: Original process admission closed |
| `⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🎛️operation/🟦️.ts` | 2/3 | 2/3 | error: expect(received).toContain(expected) |
| `⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/📥️purpose/📜️script.ts` | 0/2 | 0/2 | error: Exact graph purpose context required |
| `⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts` | 20/29 | 19/29 | error: Original process birth already received |
| `⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🟦️.ts` | 12/7 | 11/8 | error: Original process admission closed |
| `⚡️caching/📥️inference/🔍️imports/🟦️.ts` | 4/15 | 4/15 | error: Original bounded graph writer custody required |
| `⚡️caching/📦️artifacts/🏗️native-build/📥️published/🟦️.ts` | 0/1 | 0/1 | error: Original process admission closed |
| `⚡️caching/📦️artifacts/💾️storage/📥️physical/🟦️.ts` | 0/15 | 0/15 | error: Original process birth already received |
| `⚡️caching/📦️artifacts/📋️native-orchestration/📬️test-body/🟦️.ts` | 0/8 | 0/8 | ReferenceError: requireIssuedResourceControlInvocationV1 is not defined |
| `⚡️caching/📦️artifacts/🔒️bounded/🍎️image.ts` | 0/1 | 0/1 | error: Original immutable SDK required |
| `⚡️caching/📦️artifacts/🔒️bounded/🟦️.ts` | 6/1 | 6/1 | error: Original immutable SDK required |
| `⚡️caching/📦️artifacts/🟦️typescript/🧭️commands/🟦️.ts` | 0/6 | 0/6 | error: Live Original process issuance required |
| `⚡️caching/🕸️graph/🏁️startup/🚦️service/🟦️.ts` | 0/1 | 0/1 | error: Original loaded SDK root required |
| `⚡️caching/🚀️bootstrap/📥️admission/💾️storage/🟦️.ts` | 1/1 | 1/1 | error: Original process birth already received |
| `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` | 24/19 | 24/19 | error: Original process admission closed |
| `⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🟦️.ts` | 1/5 | 1/5 | error: Original bounded Nx namespace required |
| `⚡️caching/🚀️bootstrap/🛠️tools/📥️use/📥️purpose/🟦️.ts` | 1/3 | 1/3 | error: Exact same-wire toolkit refusal receipt required |
| `⚡️caching/🚀️bootstrap/🛠️tools/📥️use/🟦️.ts` | 1/4 | 1/4 | error: Exact same-wire toolkit refusal receipt required |
| `⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/📤️derivative/⚠️failure/🟦️.ts` | 0/3 | 0/3 | error: expect(received).toBe(expected) |
| `⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/📤️derivative/🟦️.ts` | 3/5 | 3/5 | error: Original process admission closed |
| `⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🟦️.ts` | 2/6 | 2/6 | error: Original process birth already received |
| `⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️snapshot/🟦️.ts` | 19/23 | 19/23 | error: Original process admission closed |
| `⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📤️copy/🟦️.ts` | 0/11 | 0/11 | error: Original process admission closed |
| `⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🛂️.ts` | 1/6 | 3/4 | error: Original process admission closed |
| `⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🟦️.ts` | 14/10 | 7/17 | error: Original process admission closed |
| `⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts` | 3/4 | 4/4 |  |
| `⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📏️headroom/🟦️.ts` | 1/2 | 1/2 | error: Original resource loan deadline exhausted |
| `⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🟦️.ts` | 21/28 | 40/8 | error: Nested persistent image requires actual parent scope |
| `⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📦️generation/🟦️.ts` | 1/2 | 1/2 | error: Nested persistent image requires actual parent scope |
| `⚡️caching/🦀️cargo/📦️units/📋️plan/🔗️binding/🟦️.ts` | 6/2 | 6/2 | error: Build test law body deadline exhausted |
| `⚡️caching/🦀️cargo/📦️units/📋️plan/🟦️.ts` | 26/7 | 26/7 | error: Original process admission closed |
| `⚡️caching/🎮️playground-preparation/🟦️.ts` | 1/3 | 1/3 | error: Original Script deadline refused |
| `⚡️caching/📦️native-dependencies/🟦️.ts` | 1/5 | 1/5 | error: Original process admission closed |
| `⚡️caching/🔗️import-edges/📁️paths/🟦️.ts` | 17/8 | 17/8 | error: Original bounded graph writer custody required |
| `⚡️caching/🗃️compiler-state/🟦️.ts` | 16/1 | 16/1 | error: 🧰️framework/📦️packages/🟦️typescript/📜️script.ts |
| `⚡️caching/🧹️pruning/🏪️budget/📜️history/🟦️.ts` | 1/1 | 1/1 | error: Original historical control belongs to another caller |
| `⚡️caching/🧹️pruning/🔒️incremental/🟦️.ts` | 0/1 | 0/1 | error: Live Original resource control issuance required |
| `⚡️caching/🧹️pruning/🔒️store/🟦️.ts` | 0/1 | 0/1 | error: Live Original resource control issuance required |
| `⚡️caching/🧹️pruning/⏱️recency/🟦️.ts` | 0/1 | 0/1 | error: Exact finite scanner fixture output required |
| `🏃️process/🎛️owned-execution/🟦️.ts` | 4/2 | 4/2 | error: expect(received).toBe(expected) |
| `🏃️process/🦀️cargo/🎯️exact/🍎️image.ts` | 0/1 | 0/1 | error: Original issued native storage reservation required |
| `🏃️process/🦀️cargo/🏭️inventory/🟦️.ts` | 2/3 | 2/3 | TypeError: undefined is not an object (evaluating 'manifest.profile.prototype') |
| `🏃️process/🦀️cargo/📥️dependencies/📥️consumption/🟦️.ts` | 2/1 | 2/1 | error: Live Original resource control issuance required |
| `🏗️build/📥️caller/📥️birth/🟦️.ts` | 2/2 | 2/2 | error: Original resource loan deadline exhausted |
| `🏗️build/📥️inputs/💾️storage/🎛️phase/🟦️.ts` | 0/8 | 0/8 | error: Original process admission closed |
| `🏗️build/📥️inputs/💾️storage/📒️ledger.ts` | 4/12 | 4/12 | error: Original process admission closed |
| `🏗️build/🔧️tools/🏗️bazel/🟦️.ts` | 68/2 | 53/21 | error: Original process admission closed |
| `🏗️build/🚀️bootstrap/📦️parsers/🟦️.ts` | 2/2 | 2/2 | error: Original Script deadline refused |
| `🏗️build/🎟️phase/🟦️.ts` | 9/7 | 9/7 | error: expect(received).toBe(expected) |
| `🏗️build/🏗️native/🟦️.ts` | 3/2 | 3/2 | error: expect(received).not.toHaveProperty(path) |
| `🏗️build/🕸️graph/🔄️candidate/🟦️.ts` | 0/3 | 0/3 | error: expect(received).toBe(expected) |
| `🏗️build/🟦️.ts` | 26/28 | 26/28 | error: Original process admission closed |
| `🏗️build/🧭️invocation/🟦️.ts` | 2/1 | 2/1 | error: $ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts repo "--tool=nx" --help |
| `🔍️discovery/🕸️runtime/🏗️build-producer/🟦️.ts` | 0/1 | 0/1 | error: /Users/ueli/.bun/bin/bun repo run runtime-probe:run --tool=bazel -- exact value exited with status 1 si |
| `🔍️discovery/🕸️runtime/🟦️.ts` | 97/7 | 97/7 | error: Original process birth already received |
| `🕸️dependencies/🟦️typescript/⚙️configured/🟦️.ts` | 2/4 | 2/4 | error: Original process birth already received |
| `🗂️workspaces/🦀️cargo/🛠️preparation/🔁️replan/🟦️.ts` | 1/6 | 0/7 | error: Original process admission closed |
| `🗂️workspaces/🦀️cargo/🕰️queued-preparation/🟦️.ts` | 0/1 | 0/1 | error: [script] {"owner":"physical-cargo-fixture","command":"native:cargo-fixture-entry","stage":"running"} |
| `⚡️workspace-negation-closure/🟦️.ts` | 0/3 | 0/3 | error: Command failed: /Users/ueli/.bun/bin/bun -e const { createProjectGraphAsync } = await import("/Users/ue |
| `🔄️transaction-v2/🟦️.ts` | 0/62 | 0/62 | error: Invalid transaction fixture run identity |
| `🧱️locale-law-ownership/🟦️.ts` | 0/2 | 0/2 | error: Explicit Vitest policy required |
| `🖥️server/🎛️coordinator/🏗️builder/💾️storage/🟦️.ts` | 1/4 | 1/4 | error: expect(received).toBe(expected) |

### MISSING-DEP/EXPORT (13)

| suite | before | after | first error |
|---|---|---|---|
| `🎛️dashboard/🗺️coverage/🟦️.ts` | 0/1 | 0/1 | error: Cannot find module '@semio-tech/repo-lib/build' from '/Users/ueli/Documents/semio/🧰️framework/🛍️product |
| `⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🕸️units/🟦️.ts` | 7/4 | 0/1 | error: Cannot find module './🪆️scope/🟦️.ts' from '/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/ |
| `⚡️caching/🕸️graph/📤️publication/🟦️.ts` | 0/6 | 0/1 | error: Cannot find module './lib/utils' from '/Users/ueli/Documents/semio/.cache/b/d/m/j/441fd20cf4b20f6dcafe/ |
| `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/👓️reading/🟦️.ts` | 42/36 | 28/50 | error: Original process admission closed |
| `⚡️caching/🦀️cargo/📦️units/📐️profile/♻️preparation/🟦️.ts` | 1/2 | 1/2 | error: error: Cannot find module '../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️ty |
| `⚡️caching/🦀️cargo/📦️units/🟦️.ts` | 12/15 | 11/16 | error: Original process birth already received |
| `⚡️caching/🧱️command-source/🟦️.ts` | 10/1 | 0/1 | error: Cannot find module './🧬️schema/🔣️.json' from '/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️proce |
| `🔍️discovery/🕸️runtime/🛬️decoder-absence/🟦️.ts` | 0/1 | 0/1 | error: expect(received).toEqual(expected) |
| `🗂️workspaces/🦀️cargo/📥️original-control/🟦️.ts` | 0/1 | 0/1 | SyntaxError: Export named 'cargoPreparationProgramSourcesControlledV1' not found in module '/Users/ueli/Docume |
| `⏱️process-budgets/🟦️.ts` | 0/1 | 0/1 | SyntaxError: Export named 'npmRunPathEnv' not found in module '/Users/ueli/Documents/semio/.cache/b/d/m/j/441f |
| `✂️node-native-typescript/🟦️.ts` | 1/2 | 1/2 | error: 🧰️framework/📦️packages/🟦️typescript/🟦️.ts |
| `🎚️tool-configuration-ownership/🟦️.ts` | 5/4 | 5/4 | error: expect(received).toHaveLength(expected) |
| `🧪️test/🧱️command-composition-source/🟦️.ts` | 10/1 | 0/1 | error: Cannot find module '@semio-tech/repo-lib/inventory' from '/Users/ueli/Documents/semio/🧰️framework/🛍️pro |

### NEEDS-DASHBOARD-INSTALL (6)

| suite | before | after | first error |
|---|---|---|---|
| `🎛️dashboard/🎮️registry/🟦️.ts` | 5/6 | 5/6 | error: Dashboard is not installed. Run bun run dashboard:install once, then start bun run dashboard. |
| `🎛️dashboard/🧭️cli/🟦️.ts` | 0/2 | 0/2 | error: Dashboard is not installed. Run bun run dashboard:install once, then start bun run dashboard. |
| `🎛️dashboard/🧭️journeys/🏋️load/🟦️.ts` | 1/3 | 1/3 | error: Dashboard is not installed. Run bun run dashboard:install once, then start bun run dashboard. |
| `🎛️dashboard/🧭️journeys/💨️smoke/🟦️.ts` | 0/3 | 0/3 | error: Dashboard is not installed. Run bun run dashboard:install once, then start bun run dashboard. |
| `🎛️dashboard/🧭️journeys/🟦️.ts` | 3/11 | 3/11 | error: Dashboard is not installed. Run bun run dashboard:install once, then start bun run dashboard. |
| `🎛️dashboard-control/🟦️.ts` | 7/2 | 9/1 | error: {"args":["run","@semio-tech/repo-dashboard-rs:workflow","status"],"installed":false,"mode":"release","s |

### CARGO-GATED (2)

| suite | before | after | first error |
|---|---|---|---|
| `⚡️caching/🧾️cargo-provenance/🟦️.ts` | 5/1 | 5/1 | TypeError: undefined is not an object (evaluating 'manifest.profile[name]') |
| `🏃️process/🦀️cargo/🚪️launcher/📥️capture/🟦️.ts` | 2/2 | 2/2 | error: cargo build --locked --offline --quiet --manifest-path /Users/ueli/Documents/semio/.cache/tst3a/ta182/c |

### TIMEOUT(load) (2)

| suite | before | after | first error |
|---|---|---|---|
| `🎛️dashboard/🧭️authority/🟦️.ts` | 0/0 | 0/0 | error: expect(received).toContain(expected) |
| `⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🟦️.ts` | 0/0 | 0/0 |  |

## Launcher-gated suites: where they run
Failures of the form `Original … required`, `Original process admission closed`, `birth already received`, `resource loan deadline exhausted`, `Nested persistent image requires actual parent scope`, `Build test law body deadline exhausted`, `Explicit Vitest policy required` are consumed by the repo-run envelope (one process birth, issued native storage, mounted images). 51 of them sit under `L⚡️caching/` and run as the `repo` project targets `test-*` in `L⚡️caching/📋️project.json` (for example `test-graph-publication`, `test-cargo-launcher`, `test-sdk-snapshot`, `test-toolkit-use`, `test-pruning`, `test-repository-exact-cargo`, `test-storage-budget`, `test-import-edges`) and the library router targets in `L📦️packages/🟦️typescript/📋️project.json` (`bun repo run @semio-tech/repo-lib:test-<id>`); the others run from `L🏗️build/📥️inputs/💾️storage/📋️project.json`, `L🗂️workspaces/🦀️cargo/📋️project.json`, `🖥️server/🎛️coordinator/🏗️builder/💾️storage/📋️project.json`, `🧪️test/📋️project.json`. I did not run `bun repo run`. Most "admission closed" rows are cascades after the first rejected test of a file (shared Original), so the first failure per file in the table is the signal.

## Missing dependencies / exports for the deps owner (no manifest edited)
- `@tailwindcss/postcss` (tool-configuration-ownership), `npm-run-path` >= 5 exporting `npmRunPathEnv` (process-budgets), workspace link `@semio-tech/course` (node-native-typescript, vite config of the course target), `@semio-tech/framework-async/continuation` (decoder-absence: no resolved owner), `@semio-tech/process/build-mode` workspace authority (frozen-markdown-coordinates), `@semio-tech/repo-lib/build` and `@semio-tech/repo-lib/inventory` subpaths (dashboard coverage, coordinator execution, command-composition-source), `@semio-tech/process/artifacts/native-build/storage/publication/platform` (dependencies storage reading), `@bytecodealliance/jco-transpile` (discovery runtime), Bun TOML `\U` escapes (Bun 1.3.14). A newer node_modules generation (`441fd20cf4b20f6dcafe`) appeared mid-run next to the active `4151d226…` (cross-platform-bootstrap and ENFILE rows depend on which one is linked).
- Dashboard suites need `bun run dashboard:install` (the installed dashboard app) first.
- Cargo workspace membership is `ws-resolve`'s; no `Cargo.toml` touched. `rust-policy-source-evidence` reads `#[path]` mounts of `os/🌿️vcs/🗃️history-ledger` (peer edited the attribute during the run).

## Files changed
Production: `L🏗️build/👁️observation/🟨️.mjs`, `L🏗️build/🛠️cmake/🎬️stage/🟦️.ts`, `🖥️server/🎛️coordinator/🏗️builder/🏃️execution/📜️script.ts`, `🧰️framework/🔨️modules/⏯️tool-run/📨️payload/🌊️stream/🟦️.ts`, `L📦️packages/🟦️typescript/📋️project.json` (two targets back to `cache:false`).
Laws/fixtures: `L🏗️build/🛠️cmake/🧪️tests/🟦️.ts`; `L🧪️tests/{🛫️preflight-reference-basis,🛟️transaction-recovery-authority,🧑‍💻os-dev-composition-ownership,✂️node-native-typescript,🫙️artifact-empty-facet-authority,↪️rust-divergence-callback,✍️rust-writable-path-authority,🧱️fixture-law-ownership,🔬️workspace-contract/🔭️package-discovery,🔬️micro-commit-staged-index}/🟦️.ts`; `L⚡️caching/🦀️cargo/📦️units/🧪️tests/📐️profile/♻️preparation/🟦️.ts`; fixtures under `L🧪️tests/🧫️fixtures/`: `🧼️remaining-package-purity-authority`, `🧫️frozen-seal-ledger`, `🫙️artifact-empty-facet-authority/🧪️registration`, `🧱️root-clean-scaffold-source`, `🏭️generated-source-topology`, `🧱️cargo-transaction-command-source`, `🎚️tool-configuration-ownership` (all `🔣️.json`).

## Hygiene
`.cache/tst3a` removed, no native image mounted by this session (`hdiutil info` clean), no `[DEBUG]` added (the two `[DEBUG]` lines in `L🏗️build/🛠️cmake/🧪️tests` and `🧭️invocation/🧪️tests` pre-existed). `🗑️generated/tst3a-out` (per-suite jsonl, logs) is kept for the coordinator until close; tools in `tst3a-tools/`.
