# Result tax2b: cross-technology edges of the tax2 router lift, value foundation, dangling renames

`C` = this ticket folder. Tools and kept inputs: `C/tax2b-tools/` (`alias.ts`, `relocate.ts`, `removed-renames.ts`, `dangling.ts`, `dangling2.ts`, `bundle-check.ts`, `summarize.mjs`, `cruise/`, `inventory.ts`, `classify-probe.ts`, `codemod-alias.test.ts`, `value-pairs.tsv`, `removed-renames.tsv`, `changed-by-alias.txt`, `queue-table.md`). Generated output `C/🗑️generated/tax2b` deleted. No modifying git command, no sub-agent, nothing under the frozen paths touched (library `📚️library/`, root `📜️script.ts`, any `package.json`, `🔣️taxonomy.json`).

## 0. Verdict

| rule (dependency-cruiser, lay13 per-area method, 13 areas, error severity) | audit-nct (19:50-20:01) | now |
| --- | ---: | ---: |
| `no-cross-technology-*` | 431 | **33** (lay15 baseline before tax2: 37) |
| `ui-foundation-closure` | 22 | **2** (both pre-existing, not tax2) |

Per area now: hub 2, smod 4, splug1 5, splug2 1, splug3 2, splug4 19, fwmodules ui-foundation 2; mit, repo, osrenderer, osrest, fwrest, rest 0. (smod re-measured after the last edit, the rest after the alias passes; fwmodules/ui-foundation after the value move.)

## 1. Deviation from the task: modules stay in `🧪️tests/🔬️<lang>-package-commands`

The task asked to move the lifted modules next to their script into `📦️packages/<lang>/`. Probed, and the taxonomy refuses it:
- `classifyPackageSource` (`tax2b-tools/classify-probe.ts`) on the wfc module: `implementation` (function body performs non-delegating work). Package source must be thin glue (`packageGlueGrammar`, `packageBoundaryRules[...].recursive`).
- Real inventory (`tax2b-tools/inventory.ts`, scope `✏️s/🔌️plugins/🀄️wfc`): placing the same file at `📦️packages/🟦️typescript/🟦️.ts` adds `package-implementation-file` ("Package implementation must move to ✏️s/🔌️plugins/🀄️wfc/🟦️.ts"). That slot is occupied in artifacts that already own an artifact-level `🟦️.ts`.
So the 158 modules keep their taxonomy-accepted slot (inventory of the value scope after my changes: 0 findings on any `package-commands`/`script-commands` path). The audit's claim that `📦️packages/<lang>` is "the correct taxonomy place" is wrong. What actually fixed the rule is the import form: both rules exempt `aliased-workspace` dependencies, so the modules import `@semio-tech/framework/<key>`, `@semio-tech/process/<key>`, `@semio-tech/repo-lib/<key>` and so on.

## 2. Changes

1. Alias rewrite (`tax2b-tools/alias.ts`, adapted from `lay15-tools/ncpr2.ts`: identity of the deep origin per imported name, existing export keys only, no glue/narrow writes, never writes into the library): 492 relative cross-technology specifiers in 138 files re-pointed to existing workspace entries (framework 345, process 114, repo-lib 28, renderer-react 4, print 3, plugin-registry 3, plugin 1, renderer-wgpu 1, os-dev 1, os 1, puzzle-js 1). File list: `tax2b-tools/changed-by-alias.txt`. Bundle check of all 141 touched files (`bun build --target=bun`): 140 ok, 1 fails on the missing `brepjs` (pre-existing, sandbox lacks the dependency). Runtime proof through the router with an issued envelope: flow `canonical-architecture bogus` and value `record verify` reach the lifted function and its own argument check.
2. Codemods fixed so reruns emit aliases: `tax2-tools/lib.ts` (`aliasSpecifier`, `aliasImportText`, export index from `node_modules/@semio-tech/*`), `lift.ts` (`rebase`), `restructure.ts` (`rebaseSpec`, also for sibling modules), `plain-lift.ts` (module imports). `ownerOf` keeps the `🧪️tests` slot, documented in its docstring. Test: `bun test ./…/tax2b-tools/codemod-alias.test.ts` 4 pass.
3. Value foundation no longer imports process: the four lifted modules (`🌱️value/🟦️.ts`, `📽️projection/🧳️record`, `🧬️retained-clone/📦️original-slot`, `🛬️decode/🧳️owned/🌳️value/👥️tuple`) moved to `<dir>/🧪️tests/🔬️script-commands/🟦️.ts` (test sources are exempt from `ui-foundation-closure`), relative specifiers rebased, their `📜️script.ts` re-pointed (`relocate.ts`). 20 ui-foundation edges gone.
4. Flow: `runBun` of the library replaced by `runCmd` of `@semio-tech/process/execution` (same `process.execPath` + `{cwd}` call), one edge less.
5. Dangling renames (coordinator addition). `removed-renames.ts` derived the 45 HEAD paths tax2 renamed (26 `🟦️.test.ts(x)` and 19 helper `<stem>.ts` to `<stem>/🟦️.ts`); `dangling.ts` (quoted literals, repo-wide `git grep`, untracked included, ticket folder excluded) and `dangling2.ts` (unquoted, last three segments) searched code, tests, fixtures, docs and `project.json`. Fixed (12 files):
   - `🧰️framework/📦️packages/🦀️rust/📜️script.ts` (`nested-io`), `🧰️framework/🔨️modules/🏃️process/📜️script.ts` (`🍎️image`), `…/🌱️value/🧪️tests/🔬️script-commands/🟦️.ts` (`🧺️heap`), `…/♾️infinite/🌍️world/🏔️terrain/🚦️ownership/📜️script.ts`, `…/🎬️MediaTransportHost/🧪️tests/♻️lifecycle/🟦️.tsx` (`🌐️browser.tsx` to `🌐️browser/🟦️.tsx`),
   - energy model package script (`🪟️optics`, `🪪️owned-target`), stdio editing contract package script (3 literals), mp4, wav (2), png, tiff, bmp package scripts (`🟦️.test.ts` to `🟦️.ts`; every successor verified on disk).
   Re-run of `dangling.ts`: 8 literals left, all in the library or non-path data (below). The audit's "suspect" energy/png/bmp references were renames, not deletions.

## 3. Queue for after the freeze lifts

Library (frozen), dangling after tax2's renames:
1. `…/📚️library/🏗️build/📥️inputs/💾️storage/📜️script.ts:6` `"🧪️tests/📒️ledger.ts"` to `"🧪️tests/📒️ledger/🟦️.ts"`.
2. `…/📚️library/⚡️caching/📜️script.ts:155` `"📦️artifacts/🔒️bounded/🧪️tests/🍎️image.ts"` to `…/🍎️image/🟦️.ts` and `"💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/🏪️scope.ts"` to `…/🏪️scope/🟦️.ts` (same line).
3. `…/🏗️build/🎚️mode/🦀️profile/🧪️tests/⚙️arguments/📐️context/🧪️tests/🟦️.ts:17,30` `new URL("../🦀️.ts", …)` and `…/🦀️profile/🧪️tests/🔣️.json:552` `"🧪️tests/🦀️.ts"` point at `🦀️.ts` that is now the emoji-only directory `📐️context/🦀️/🟦️.ts`; fix together with the four emoji-only directory renames of `result-tax2.md` section 3 (rename each to `<emoji><slug>`, re-point).
Not a path reference, left: `🧪️tests/🧫️fixtures/🧫️diff-only-law-gate/🔣️.json:1607` (`"🟦️.test.tsx"` segment of a law-gate case) and `…/⚡️caching/📥️inference/🧪️tests/🧫️fixtures/🔣️.json:212` (`./production.ts` inside a source snapshot).

Dependency gaps of the new bare imports (nearest manifest lacks the dependency): `🌎️hub/package.json` needs `@semio-tech/framework-renderer-wgpu`, `@semio-tech/puzzle-js`; `♻️mit-bestand/package.json` needs `@semio-tech/print`. `✏️s/package.json` already declares every package used.

Missing export keys (33 remaining cross-technology edges; none exists in any package.json, so no alias is possible; `r3f` already has its glue file). Target, edges (importers in `queue-table.md`):

| target | edges | importer directories |
| --- | ---: | --- |
| `FW/📦️packages/🟦️typescript/♾️infinite/🌍️world/🎨️r3f/🟦️.ts` | 6 | 🎨️pick-styling; 🎯️pick-model; 📺️view-state; 🧲️geometry-targets; 🧾️repl-model; 🪩️canvas |
| `FW/📦️packages/🟦️typescript/🔏️hash/🟦️.ts` | 1 | 🎫️requests |
| `FW/🔨️modules/⏯️tool-run/📨️payload/🌊️stream/🟦️.ts` | 1 | 🧪️tests |
| `FW/🔨️modules/🌱️value/📶️sort/🧪️tests/🟦️.ts` | 2 | 🔬️rust-package-commands; 🧪️shared-authority |
| `FW/🔨️modules/📡️replication/🧾️wire/🪪️scoped-identity/🟦️.ts` | 1 | 🕹️interaction |
| `FW/🔨️modules/🕹️interaction/🧬️schema/🟦️.ts` | 2 | 🎞️scenes; 🔁️story-reducers |
| `FW/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧪️color-input/🟦️.ts` | 1 | 🧪️shared-authority |
| `FW/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🌿️variant-field/🟦️.ts` | 1 | 🔬️unit |
| `FW/🔨️modules/🧮️math/🧮️algebra/🔲️dense/↔️inverse/🟦️.ts` | 1 | ⚡️exchange |
| `FW/🔨️modules/🧮️math/🧮️algebra/🔲️dense/🟦️.ts` | 2 | ⚡️exchange; 🌡️thermal |
| `FW/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/🧪️tests/🟦️.ts` | 1 | 🧪️tests |
| `FW/P/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🧪️testing/🎚️config/🟦️.ts` | 1 | 📺️renderer |
| `FW/P/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts` | 1 | 🚀️renderer-boot-selection |
| `FW/P/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🎨️paint/🟦️.ts` | 1 | 🔬️unit |
| `FW/P/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🔮️oracles/🖱️mounted-input/🟦️.tsx` | 1 | 🛍️canvas-catalogue |
| `FW/P/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🧪️tests/🧫️fixtures/🖱️input-contract/🔣️.json` | 1 | 🛍️canvas-catalogue |
| `FW/P/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🟦️.ts` | 1 | 🎯️pixel-selection |
| `FW/P/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🔮️oracles/🪪️session-double/🟦️.ts` | 1 | 🪪️session-factory |
| `FW/P/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧫️fixtures/🪪️entity-id/🔣️.json` | 1 | 🧪️shared-authority |
| `FW/P/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧫️fixtures/🪪️render-revision/🔣️.json` | 1 | 🧪️shared-authority |
| `FW/P/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧬️schema/🪪️entity-id/🟦️.ts` | 1 | 🧪️shared-authority |
| `FW/P/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧬️schema/🪪️render-revision/🔣️.json` | 1 | 🧪️shared-authority |
| `FW/P/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧬️schema/🪪️render-revision/🟦️.ts` | 1 | 🧪️shared-authority |
| `FW/P/🦑️repo/🔨️modules/📚️library/🏃️process/🕸️wasm/🔣️policy/🟦️.ts` | 1 | 🔬️rust-package-commands |
| `FW/P/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts` | 1 | 🔮️oracles |
| `FW/P/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts` | 1 | 🧿️semio-member-store-producer-source |
| `FW/P/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🎭️selection/🧪️tests/🧫️fixtures/🔣️.json` | 1 | 🧪️tests |

Key proposals: `@semio-tech/framework` `./infinite/world/r3f` to `./♾️infinite/🌍️world/🎨️r3f/🟦️.ts` (6 edges, glue exists); `repo-lib` entries for `runRepositoryCargoCommandV1` (hub stdio oracles), `buildRepositoryWasmWebV1` (spatial-kernel session), `policyReadMemberStoreProducerSources` (stdio producer source; this lane reads the Rust sources as text through that reader); glue entries for the framework test oracles (`verifyRetainedIndexSortOracle`, `colorInputSelfTests`, `testDeclaredVariantFieldOracle`, tool-run payload stream, job caller tests), the interaction schema, math dense, the os plugin-declaration view and the renderer element modules. Of the 33, 4 are tax2-written (fem 3d package commands, spatial-kernel session, hub stdio oracles, plus the stdio producer-source lane of fixups4); the rest are in HEAD or peer files. The 2 ui-foundation edges: `🌱️value/📋️registry/🎫️requests` imports `@semio-tech/framework/hash`, `🕹️interaction/🟦️.ts` has `export *` into `📡️replication` (both pre-existing).

## 4. Verification

- Cruise: `tax2b-tools/cruise/cruise-some.sh` (12 areas twice, smod once more after the last edit) and `summarize.mjs`; numbers in section 0.
- `bun test ./…/🏃️process/🧭️routing/🧪️tests/🟦️.ts`: 56 pass (with `--timeout 60000`; with the default 5 s one test times out under load average 42, same code).
- Command-router census (`tax2-tools/census.ts`): 157 non-ticket scripts rejected, identical to the tax2 end state (43 frozen library + root + 113 others); no package-commands script rejected.
- `bun build --no-bundle` of every edited script and of the edited codemods: ok.
- Taxonomy inventory of `🌱️value` after the move: 7 findings, all `package-implementation-file`/`package-role-unresolved` in Rust packages, none on `🔬️script-commands`.
- Not run: whole-tree `verify`, `verify dependencies`, cargo, product activation, Bazel; cruise of the library-heavy `repo` area changed nothing (0).
