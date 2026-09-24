# WP-T8: Presentation Failures, Stale Hub/Flow/Stdio Laws, Print Catalog Coverage, Test-Platform Budgets, Contract Rule

Slice: T8 (session 10). Captures: `.tmp-ticket/wp-t8/generated/`. Ticket inputs: `.tmp-ticket/wp-t8/*`.
Inherits: T6 §2 (print debt), §3 (found, not fixed), §4; T4 §4.

## Status

| Item | State | Evidence |
|------|-------|----------|
| 1. Presentation react 11 failures | **Done: 147/147** (was 136/147). Two root causes, plus the projektetage suite that never ran: **28/28** (was "No test files found") | `presentation-react-5.txt`, `projektetage-test-2.txt` |
| 2a. Hub stale laws (socket-grant-command-source, foundation-source) | **Done.** socket-grant-command-source **7/7** (was 5/7), foundation-source **12/12** (was 11/12). Run as plain `bun test` (no cargo, no hub build), so no hub mutex was needed; H6 confirmed no file overlap | `hub-socket-grant-command-source-2.txt`, `hub-foundation-source-2.txt` |
| 2b. Flow ownership law (`host-runtime` exports) | **Done.** PASS directly and through `nx run semio-framework-os-flow-core:test-browser-ownership` | `flow-ownership-4.txt`, `flow-ownership-nx-1.txt` |
| 2c. Stdio ownership law (`verifyStdioCommandOwnership`, workspace links) | **Done.** Ownership **1/1**. Via nx: `package-contract`, `package-graph` and `test` (36 artifact builds + composition consumption) all succeed | `stdio-ownership-4.txt`, `stdio-package-contract-1.txt`, `stdio-package-graph-2.txt`, `stdio-composition-test-2.txt` |
| 3. Print catalog-coverage 20 findings | **Done: 20 → 0.** The 8 fundamental scenarios pass, and `nx run @semio-tech/print:generate-viz` now exists and runs | `print-coverage-1.txt` → `print-coverage-2.txt`, `print-catalog-coverage-1.txt`, `print-generate-viz-1.txt` |
| 4. Test-platform 3 timeouts | **Done, by measuring outcomes, not raising budgets.** Repository walks and scans run once at module scope; the contract is split into case and repository-wide parts | §4 |
| 5. Contract new-rule rows (~103) | **Classified (89 rows; T6's "~103" was an estimate). Systemic causes fixed:** the fem bridge region (58 manifest-only → 0), outcome drift in wfc/sequence (22 → 0), a stale os.config inventory (37 → 0). **Open:** a `no-op` projection decision (24 fem + 11 draw); owner debt in cad 5, layout 1, gis 3 (gis is frozen) | §5, `contract-inventory-classes-2.txt` |
| 6. Rerun | Test-platform suite **115 pass / 1 fail** (116), and the only failure is contract-zero, by assertion in 3.6 s. The suite now takes **54 s** (was 326 s). Contract **361 → 632**, see §6 | `test-platform-1.txt`, `contract-2.txt`, `contract-delta-2.txt` |

## 1. Presentation react

Two independent root causes, neither in the moved suite itself.

**7 geometry tests: the deck was assembled by a stale fork.**
- The tests import `deck` from `@semio-tech/mit-bestand-praesentation-projektetage-spec`, which aliases `🔖️spec.ts`. That module has no `deck`: the deck needs the slide glob and a top-level await, and `🔖️spec.ts` exists precisely so slides can import the spec without cycling into it (its header says so). `deck` lives in `📦️index.ts`.
- `📦️index.ts` carried a 469-line verbatim copy of `🔖️spec.ts` (the `//#region 🔖️spec` left by the old `merge-scattered-files.ts` consolidation). It also imported every presentation function from `@semio-tech/animate-presentation-core`, which is not a declared dependency of the package, only an alias to the animate plugin's Sep 8 fork of `@semio-tech/presentation`. That fork still parses `slide/…` paths. The slides moved to `🎞️slide/…`, so the fork found no chapter and `reorderChapters` threw `missing chapter "Einführung"`.
- **Fix:**
  - `📦️index.ts` imports the spec from `./🔖️spec.ts` and re-exports it. The duplicate region is gone (`wp-t8/dedupe-projektetage-spec.py`).
  - Everything, including `projektetagePlayAppDefinition`, now comes from the declared `@semio-tech/presentation`.
  - The package entry `🟦️.ts` is `export * from "./📦️index.ts"`. `package.json` `semio.app.definitionExport` names `projektetagePlayAppDefinition` on the package root, which never exported it before.
  - The deck tests import from `@semio-tech/mit-bestand-praesentation-projektetage`. Only slide files use the `-spec` alias.

**5 pdf dom tests: the owned `act` dropped async transactions.**
- `@semio-tech/ui-react/test`'s `act(update: () => void): void` called testing-library's `act` and discarded its thenable. So `await act(async () => …)` in `waitForPdfCanvas` left React's act scope open: React warned "You called act(async () => ...) without await". The pdf document state set inside it never committed, and the canvas stayed `loading`.
- **Fix** (`🖱️ui/🎯️targets/⚛️react/🖌️render/🟦️.ts`): overloads `act(() => Promise<void>): Promise<void>` and `act(() => void): void`, both returning the transaction.
- The `HTMLMediaElement.play` "Not implemented" noise is gone too: `🧰️vitest.setup.ts` now stubs `play`/`pause` beside the existing jsdom polyfills.

**Projektetage's own suite collected nothing.**
- Its config had `includeSource: ["🟦️.ts"]`, but its tests are registered from `📦️index.ts`. Also, the production `mount()` was called through `registerTests1` in the test module.
- **Fix:**
  - `includeSource`/`coverage` now point at `📦️index.ts` (and `🔖️spec.ts` for coverage).
  - `mount()` is called directly.
  - `registerTests2` → `registerProjektetageDeckTests`.
  - The vitest-ownership fixture hash for this owner was updated (`d12f8e15…`).

**Checked, not mine:**
- ui-react exhaustive: 806/816. All 10 failures reproduce with the old `act` (`ui-react-vitest-baseline*.txt`): 7 Shell components, 3 tree rendering.
- `vitest-configuration-ownership`: 7 other owners' projection hashes drift from peers' config edits (hub, cad, kernel, replication, ui-react, os mcp, wgpu; `wp-t8/vitest-projection-drift.ts`).

## 2a. Hub laws

**socket-grant-command-source: the wiring was never finished.** `git log -S` finds no revision of the taxonomy with its kinds.
- **13 taxonomy kinds added** (`semanticDirectoryKinds`, beside the foundation-source kinds):
  - `hub-local-relay` and `-routing`;
  - `hub-directory-authorization`;
  - `hub-directory-socket-grant` with `-decision` and `-tests`, and under `-tests`: `-fixture-verification`, `-native-law-plan`, `-execution`, `-oracles`;
  - `hub-socket-grant-command-source-{test,schema,fixture}`.
- **Hub `📋️project.json`:**
  - new named input `hubSocketGrantCommandSources`, equal to the fixture's source inputs;
  - new cached target `socket-grant-command-source-check`;
  - `socket-grant-check` now declares the same inputs.

  The router class and both launch entries already existed.
- **Wall time, by root cause.** With the wiring fixed, two tests then hit bun's 5 s timeout. `proveScopedDirectorySocketRevocationFixture` took **17.8 s cold**. The cause was `hubSchemaModuleDirectories` (`🤝️integration-harness`): it walked all of `🌎️hub`, skipping only `node_modules`/`target`. That meant 348,986 directories of `📦️packages/🦀️rust/🗑️generated/test-artifacts` (1.3 GB of hub test output). It now skips via discovery's exported `isDiscoverySkipDirectory`, the repo's single rule for opaque directories (`🗑️generated`, `dist`, `.🧬semio`, cargo targets …). Cold time is **37 ms**. The hub integration tests use the same walker, so they get the speedup too. No timeout was raised.

**foundation-source:**
- **mcp credential order.** The law forbade any `set_document_execution_target_lease(` anywhere in the mcp workspace. That was written when the MCP probe dropped its unverified pre-claim (ticket 26/09/02 `fable-execution-target-lease.md`). A newer, legitimate path, `bind_hub_session_document` → `open_hub_document_actor`, sets a lease that it fetched from the hub (`fetch_execution_target_lease`) and checked against the authenticated descriptor.
  - The predicate now states the real invariant in `mcpLeaseClaimsAreHubIssued`:
    - the probe's `persistence_binding` binds `PROBE_SURFACE_ID` and never a lease;
    - every lease claim sits inside a function handed a `DocumentExecutionTargetLeaseFieldsV1`;
    - a claim requires the workspace to fetch that lease from the hub.
  - Two new hostile mutations are asserted `false` in the law: a lease mentioned in the probe binding, and a lease-less function that claims one.
- **Fixture drift.** A peer added the export and root import `TRUSTED_CATALOG_READINESS_STALL_BOUND_MS` (`🚀️local-bootstrap/🏃️execution`). It was added to the owner's `declarations` and `rootImports`.

## 2b. Flow

Three stale points after the fixture fix:
1. **Fixture drift.** The `host-runtime` owner gained `createFlowPumpScheduler` (a peer). It is now in the fixture.
2. **Launch entries lost.** The launch rewrite of commit 628 (`6f33e313da`) dropped all four flow entries: `⚖️gate🌊️flow🌐️startup`, `…⏱️consumed-clock`, `…🏷️browser-ownership` and `📦️preview🤖️flow-browser-package`. The law pins them, and the four nx targets still exist, so they were restored in both launch files with the original names, groups and orders (407.8–407.83).
3. **A cwd-dependent assertion.** Bun's non-minified bundle writes a `/* <cwd-relative module path> */` comment per module. So "the bundle does not contain the browser owner's path" passed from the package directory and failed from the repo root. The law now strips block comments and checks the code for the owner's `🌐️browser/🏃️runtime/🟨️.js` path in any spelling. That is the real "not imported by path" property, from any cwd.

Its PASS line also lost a stray `[DEBUG] ` prefix.

## 2c. Stdio

**Root cause: an extraction left half done.**
- The owners `🗿️artifacts/{📇️inventory,🛂️contract,🕸️graph,🏃️commands}` and `🧩️composition/*` existed, and the command-ownership fixture described the target shape.
- But the root `📜️script.ts` still held the 464-line pre-extraction implementation, plus a `createStdioArtifactPackageTests(dependencies…)` factory that received its functions by injection.
- `verifyStdioCommandOwnership` had never been written.
- `🕸️graph`/`🛂️contract` imported `../../📇️inventory` (off by one level), so no owner module had ever loaded.

**Fixes:**
- **`🧪️tests/📦️artifact-package-graph/🟦️.ts`** now exports the fixture's three functions over the owner modules. `verifyStdioCommandOwnership` checks:
  - the fixture against its 2020-12 schema (Ajv2020);
  - each owner's TS-AST export surface;
  - each router: exact `.register` set, `defaultCommand`, and no own function/class declarations;
  - the admitted artifact definitions;
  - three named Nx inputs equal to the fixture's input sets, and each composition command's target inputs and `bun ./📜️script.ts <command>`;
  - package scripts;
  - both launch files.
- **Both routers** (root; `📦️packages/🟦️typescript`) are now pure routing over `🗿️artifacts/🏃️commands` and `🧩️composition/🏃️commands`.
- **`📋️project.json`:** named inputs `stdioCompositionBuild`/`stdioArtifactContract`/`stdioArtifactGraph`. `build`/`check`/`test` hash `stdioCompositionBuild` + `^production`; `package-contract`/`package-graph` hash their own set.
- **Launch:** 5 new entries for `@semio-tech/stdio-js`, the only stdio commands never registered: `📦️build🗄️stdio`, `📦️check🗄️stdio`, `⚖️gate🗄️stdio🧩️composition`, `…🛂️package-contract`, `…🕸️package-graph`.
- **Collection manifest `🗿️artifacts/🔣️.json`** named `📄️pdf` and `🧊️obj`. The directories (and their own `artifact-definition.json`) are `📖️pdf`/`🗽️obj`, so the admitted inventory threw ENOENT.
- **Fixture:** the inventory export surface now lists the 11 shared constants, types and helpers its siblings import.
- **Composition consumer probe** imported `./🟦️.d.ts`, which is TS2846. It now imports `./🟦️.js`, and tsc resolves the adjacent declaration.
- **`nx graph --print`** overflowed `runCaptured`'s 64 MiB stdout cap on this workspace. The graph owner now has nx write `--file=<mkdtemp>/graph.json` and reads that file.
- **T6's "needs `bun install`" was not the cause.** The per-artifact packages export `./dist`, which the nx target's `dependsOn: ["^build"]` produces. Only a direct, non-nx run lacks it.

## 3. Print catalog coverage

**Root cause: an orphaned, superseded package.** `🖋️latex/semio-viz-axis.sty` (commit 590, v0.1.0) is required by no package, class or template (`git grep`). It is a predecessor of what `semio-viz-guide.sty` now owns:
- `\SemioVizAxis`, `\semio_viz_plot_frame:nn` and `\semio_viz_legend_draw:n`;
- a 3-key `semio / viz / axis` path whose `legend` key set the `\l_semio_viz_legend_kind_tl` that guide also `\tl_new`s. Loading both would be a LaTeX redefinition error.

The coverage model reads every `.sty` in the directory. So its duplicate macros shadowed guide's in the macro map:
- the axis/scale/composition families "implemented" the orphan's `legend` key (3 undocumented);
- the families stopped reaching guide's real `semio / viz / legend` path (17 phantom: `columns`, `format`, `hatchLines`, `itemGap`, `kind`, `labelGap`, `length`, `swatchSize`, `symbol` on scale and composition).

**Fix:**
- The orphan is deleted; a copy is kept at `wp-t8/semio-viz-axis.sty.removed`. Its `print-latex-semio-viz-axis` basename exception in `🔣️taxonomy.json` is removed.
- T6's regenerated `🔣️viz-api.json` had gained that package. The regenerated file is now byte-identical to HEAD again.
- **The generator route T6 flagged is fixed.** The print router registers `generate` (`generate viz` → `generateVizArtifacts`), so the existing `generate-viz` target works. It has a launch entry `📦️generate🖨️print📊️viz`.
- Two `[DEBUG]` prefixes on the case's permanent PASS lines were dropped.

## 4. Test-platform budgets

**Measured.** Standalone, at load average ~32:

| Step | Time |
|---|---|
| `discoverTestCases` | 4.1–6.8 s |
| `loadOracleRegistry` | 8.1 s |
| per-case contracts (all cases) | 5.7 s |
| `oracleImportsInProduction` | **35.0 s** |
| stub serializers | 7.5 s |
| stub deserializers | 7.9 s |
| full `validateAllContracts` | **78 s** |
| `validateAllContracts` on one case | **67 s** |

The "narrowing" law paid for a whole repository contract (67 s) to check one breach id. The idempotence law walked the repository twice. The oracle-purity scan ran three times per suite: once in the contract law and twice in purity laws. And `oracleImportsInProduction` rediscovered the registry and the cases that its caller had just built.

**Fix (outcome-measuring, no budget raised):**
- **Library (`🧪️test/🟦️.ts`).** `validateAllContracts` is now `caseContractBreaches(repoRoot, cases, registry)` + `repositoryContractBreaches(repoRoot, registry, allCases, oracleHits)`.
  - The repository-wide part takes the FULL discovery and never a caller's selection. So the narrowing regression is now impossible by signature, where before it was a comment.
  - `oracleImportsInProduction` takes the registry and full case list instead of rebuilding them.
  - `validateAllContracts` keeps its API and its breach order (the contract CLI is unchanged).
- **Suite.** The file already took the contribution scan and registry once at module scope, "so a budget measures the assertion … instead of a filesystem walk". The discovered cases, the oracle-purity hits and the repository-wide breaches now follow the same pattern. So:
  - The 8 in-test discoveries read `repoCases`.
  - Idempotence compares one fresh walk with `repoCases`.
  - The contract law composes `caseContractBreaches(all) + repoWideBreaches`.
  - The narrowing law checks the one-case part plus the identity of repo-wide oracle rows with the scan.
  - The purity and debt laws read `repoOracleHits`.
  - The purity laws' 60 s budgets were removed; they now finish in milliseconds.

## 5. The "new" contract rows

**It is not a new rule.** These are the `mutationInventoryBreaches` rows `runtime-only-mutation`, `manifest-only-mutation` and `mutation-outcome-mismatch`. They were always there, but emitted only once a runtime inventory exists. T5's bridges produced inventories for 30 more subsets (`no-runtime-inventory` 172 → 142), and so the rows appeared.

T6's `contract-1` has **89** such rows, classified by owner and root cause:

| Owner | Rows | Kind | Root cause | Class |
|---|---|---|---|---|
| fem 2d + 3d | 58 | manifest-only (every kind) | **Bridge region.** `s.fem.*@1/any` is a composite manifest over the sibling subsets mesh/material/load/boundary/analysis, and none of its leaves live in `🌐️any`. The bridge filtered descriptors to `…/🌐️any/`, so it produced an **empty** inventory | systemic |
| wfc wfc2d/grid2d/grid3d | 21 | outcome mismatch | **Two hand-kept authorities disagree.** The leaf descriptor's `outcomeClasses` omit `error`/`fatal` that the leaf's own `🔺️diff` returns (16 rows). The manifest claims `rejected` for `change-seed`/`change-periodicity`, whose diffs cannot reject (5 rows) | systemic (same cause in every row) |
| sequence step | 1 | outcome mismatch | the manifest omits the `rejected` that the descriptor (`fatal`) and code produce | same cause as wfc |
| cad | 5 | runtime-only | `create/delete/move/rotate/scale-object(s)` leaves are dispatched in production, but the cad manifest has no rows (no catalog kinds, no fixtures) | owner debt |
| gis terrain | 3 | 2 runtime-only, 1 manifest-only | the editor-window manifest lacks `change-exaggeration`/`change-imported-features` and declares `set-camera`, which dispatch does not offer | owner debt; **frozen (gis)** |
| layout | 1 | runtime-only | `rotate-frame` has no manifest row | owner debt |

**Fixed:**
- **fem bridge region.**
  - `🏗️fem/🏭️bridge/🦀️.rs` measures the `🪆️subsets` root for both `any` coordinates.
  - The generator `wp-t5/bridges.py` gained the exact rule, so a rerun cannot revert it. `leaf_region`: an `any` manifest that owns none of the leaves its ids name, and whose ids are all leaves of its sibling subsets, measures the `🪆️subsets` root. Over all 175 manifests the rule selects exactly fem 2d and 3d.
  - Rebuilt (private target) and re-inventoried. fem 2d and 3d now report **29/29 runtime mutations each** (was 0/29).
- **Outcome drift** (`wp-t8/outcome-align.py`), additive and on direct leaf evidence only:
  - a descriptor gains `error`/`fatal` only where the leaf's own code calls `MutationOutcome::error`/`::fatal` (19 wfc leaves, 3 fem 3d `create-*` leaves);
  - a manifest row loses `rejected` only where neither the descriptor nor the leaf code (with no delegated guard) can reject (5 wfc rows);
  - a manifest row gains `rejected` where the descriptor declares a rejecting class (sequence `create-step`).

  `cargo check -p semio-s-artifact-wfc-2d -p semio-s-artifact-wfc-grid2d` and `-p semio-s-artifact-fem-3d` are clean (warnings only). Re-inventoried: wfc2d/grid2d/grid3d/wfc3d/bitmap and sequence step/dependency all show **0 differences**.
- **A first attempt was withdrawn.** It re-derived classes from leaf code alone, and would have dropped `fatal` from fem leaves whose rejection lives in shared `guards::` helpers. All 55 touched descriptor/manifest files were restored from HEAD (checked to differ only in outcome fields), and the additive pass was applied instead.

**Open, needs a decision, not a patch: the `no-op` projection (24 fem rows).**
- The fem manifests declare `no-op` as an outcome class. But every bridge projects the leaf severity `info` to `applied`, so `no-op` can never be reported.
- The platform's own descriptor scaffolder and vector reader (`🧪️test/🟦️.ts` ~4150/4181) treat `info` ⟺ `no-op`/`empty`.
- Survey of all 2,200 manifest rows (`wp-t8/projection-survey.py`):
  - 1,307 agree under either projection;
  - **688 agree only under `info→applied`**;
  - 12 (fem) agree only under `info→no-op`;
  - 173 agree under neither.
- Switching the bridges to `info→no-op` would therefore turn 688 rows into findings. Semantically they would be true, since each is a reachable no-op outcome with no declared fixture class.
- The contract owner has to pick the canonical projection, or give leaf descriptors the protocol outcome vocabulary. I left both sides unchanged.
- The 173 "neither" rows are the same two-authority drift as wfc, across the repository. They will surface as their owners' inventories are produced.

## 6. Contract delta (T6 `contract-1` → T8 `contract-2`)

| Class | T6 | T8 | Δ | Cause |
|---|---|---|---|---|
| no-runtime-inventory | 142 | 0 | −142 | peers produced every remaining runtime inventory (stdio, draw, os.config…) |
| inventory rows, T6 scope (fem, wfc, sequence, cad, gis, layout) | 90 | 45 | −45 | §5 fixes: fem −34 (58 manifest-only → 24 no-op outcome rows), wfc −21, sequence −1 |
| inventory rows, os.config | 0 | 0 | 0 | 37 rows came from an inventory produced at 15:29 by the pre-15:33 bridge without the subset prefix. Regenerated: 3 subsets, 0 differences |
| inventory rows, newly measured stdio | 0 | 458 | +458 | peers' new stdio inventories: gltf variant naming (119) + outcomes, subsets whose leaves live in the base vocabulary (T5 §2), pdf/dxf runtime-only. **stdio is frozen** |
| inventory rows, newly measured draw | 0 | 11 | +11 | the `no-op` projection decision |
| everything else | 129 | 129 | 0 | |
| **Total** | **361** | **632** | **+271** | |

On the set T6 could measure, the rows fell 90 → 45. The total rose only because 142 subsets that were unmeasured are now measured, and stdio is most of them.

**Found, not fixed:** `os.config`'s `UiPreferencesConfigMutation` (9 leaves: `set-appearance`, `set-layout`, `set-driver`, …) is measured by no manifest. The prefix-filtered bridge assigns it to no subset, so this dispatched surface is invisible to the gate rather than reported as runtime-only.

## Processes (pids)

All detached processes have exited; none are left running:
- fem bridge build 35842
- inventory chain 41228
- test-platform + contract chain 54295

No servers, no ports. No wasm32 and no `semio-hub` cargo, so no fleet-mutex use (the hub laws are plain `bun test`; H6 confirmed there is no overlap). Native cargo: `cargo check -p semio-s-artifact-wfc-2d -p semio-s-artifact-wfc-grid2d`, `-p semio-s-artifact-fem-3d`, and the fem/wfc/sequence/config bridges under a private `CARGO_TARGET_DIR`. `wp-t8/target` has been deleted.

## Files changed (T8)

- **Presentation:**
  - `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/{🟦️.ts,📦️index.ts}`
  - `…/33.projektetage/🧪️tests/{🎚️config/🟦️.ts,🧪️projektetage-deck/🟦️.ts}`
  - `🎤️presentation/🧪️tests/🎞️presentation-react-deck/🟦️.tsx`
  - `🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧰️vitest.setup.ts`
  - `🖱️ui/🎯️targets/⚛️react/🖌️render/🟦️.ts`
  - `📚️library/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json`
- **Hub:**
  - `🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts`
  - `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts`
  - `🌎️hub/🧫️fixtures/🧱️foundation-source/🔣️.json`
  - `🌎️hub/📦️packages/🦀️rust/📋️project.json`
  - `🌎️hub/🤝️integration-harness/🟦️.ts`
  - `📚️library/🔣️taxonomy.json` (13 hub kinds; the `print-latex-semio-viz-axis` exception removed)
- **Flow:**
  - `🌊️flow/🕸️wasm/🌐️browser/🏷️ownership/{🧫️fixtures/🔣️.json,🧪️tests/🏷️browser-ownership/🟦️.ts}`
  - `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (4 flow, 5 stdio and 1 print entries)
- **Stdio:**
  - `🗄️stdio/📜️script.ts`
  - `📦️packages/🟦️typescript/{📜️script.ts,📋️project.json}`
  - `🧪️tests/{📦️artifact-package-graph,🧩️composition-consumption}/🟦️.ts`
  - `🗿️artifacts/{🕸️graph,🛂️contract}/🟦️.ts`
  - `🗿️artifacts/🔣️.json`
  - `🧫️fixtures/🏃️command-ownership/🔣️.json`

  The coordinator reviewed these under the freeze and kept them.
- **Print:**
  - `🖋️latex/semio-viz-axis.sty` (deleted)
  - `📦️packages/🟦️typescript/📜️script.ts`
  - `🧪️tests/📚️catalog-coverage/🟦️.ts`
- **Test platform:** `🧪️test/🟦️.ts` and `🧪️test/🧪️tests/🧪️test-platform/🟦️.ts`.
- **Mutation inventory:**
  - `🏗️fem/🏭️bridge/🦀️.rs`
  - 19 wfc and 3 fem 3d leaf descriptors (`outcomeClasses`)
  - wfc wfc2d/grid2d/grid3d and sequence step manifests (`🔮️oracles/🔣️.json` outcomes)
  - `wp-t5/bridges.py` (`leaf_region`)
- **Ticket inputs:** `wp-t8/*.py`, `wp-t8/*.ts`, `wp-t8/semio-viz-axis.sty.removed`.
