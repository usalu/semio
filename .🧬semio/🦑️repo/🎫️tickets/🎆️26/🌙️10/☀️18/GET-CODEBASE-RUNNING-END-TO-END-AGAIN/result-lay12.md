# Result lay12: WP12 framework modules, packages and ui layering

Executor `lay12`. Source: `audit-layering.md` WP12, `result-lay0.md`. No cargo, no `package.json`/`bun.lock` edit, nothing under the frozen storage/revision paths, nothing under `✏️s/**` or `🧰️framework/🛍️products/💻️os/**`. No modifying git command (the index is being staged by another process; `MM` rows in `git status` are that process plus my later edits). Tools and corpora stay in `C/lay12-tools/`; every generated report was deleted.

## 0. Totals (WP12 scope, same cruise, before and after)

Scope: violations whose source is under `🧰️framework/🔨️modules` or `🧰️framework/📦️packages` and whose target is not a product (`bun dependency-cruise 🧰️framework/🔨️modules 🧰️framework/📦️packages --config …/🧹️lint/🕸️dependency-boundaries/🟨️.cjs`, about 3 min). The audit counted 1,178 over the full tree; the same rules cruised over just these two roots counted 1,146 at the start.

| rule | before | after (policy-adjusted, see 3.1) | after (policy-neutral, original rules) |
|---|---:|---:|---:|
| `no-circular` | 204 | **12** | **38** |
| `ui-no-framework-packages` | 291 | **174** | **273** |
| `no-cross-package-relative` | 644 | 598 | 598 |
| `no-core-path` | 7 | 0 (cleared by peer renames, not mine) | 0 |
| **total** | **1,146** | **784** | **909** |

Two caveats about the numbers:
- dependency-cruiser reports one violation per edge that lies on a cycle, so a cycle count is the number of edges inside strongly connected components. Partial work inside a component can raise the count (run7 went 48 -> 60 after the shard-client hub ports, until the ui seams cut the component). Read `no-circular` as "components removed", not as "cycles fixed".
- `no-cross-package-relative` is two violations per edge (the `@semio-tech/framework` rule and its `-rs` twin). 534 of the 598 are ui-react -> ui/elements|modules (section 4.1); I added about 18 new ones with the seam leaves (4.2).

## 1. What changed

### 1.1 Cycles broken by leaf extraction (framework modules)

New leaves (each is imported by the former child and by the former parent, imports neither):

| module | new leaf | what moved |
|---|---|---|
| `◻️2d` | `📐️drawing-contract` | `Vec2`, `PathSegment`, scene nodes, `isDrawingRef` (booleans/path/trace children no longer import the module index) |
| `◻️2d/📝️text/🔤️font/🧵️shape/📇️face` | `🧵️scalar` owns `FontMappedScalar` | face re-exports it |
| `🚪️io/🪶️sqlite-snapshot/🧩️artifact` | `🚦️checkpoint` | `ArtifactSqliteOptions`, `ArtifactSqliteProgress`, `artifactSqliteCheckpoint`, `cancellation` |
| `⏯️tool-run/📊️progress/🪟️presentation` | `📸️capture` | `ToolRunPresentation` |
| `🖱️ui/🎬️scene` | `🚚️lane-contract` | `SceneLaneRef`, `SceneLane`, `sceneFromLanes` (scene types no longer import the lane tables typed by them) |
| `📡️replication` | `🔖️sync-protocol` | the whole sync protocol type region (envelopes, frames, bootstrap pairs, presence), `TransactionRef`, `ExactWireMutationEnvelope` (5 binary codec children no longer import the module index) |
| `⏪️time-travel/🧪️testing` | `🔬️probe`, `🎗️band/📋️readout` | `evalSafe`, `sleep`, `waitUntil`, `shot`, `markToken`; the `Band` readout types. `revealHistory` moved from editor to history; `Renderer`/`Locale` moved to scenario. The public barrels keep their export lists |
| `🏃️process/🔒️leases` | `🎛️control` | `ResourceLeaseControl`, `ResourceAccess`, `LeaseWait`, `checkResourceLeaseControl`, `requireMaterializedPathV1` |
| `🏃️process/🏗️build/🎟️capacity` | `🎫️lease` | the whole lease implementation; the index is now an aggregator of lease + scope |
| `🏃️process/🧭️routing/🎛️command` | `📥️request` | `CommandRequestV1` |
| `🏃️process/🧪️testing/🦀️cargo/🎯️exact/🛠️preparation` | `📋️request` owns `ExactCargoLawPreparationV1` | |
| `🏃️process/…/💾️storage` | `🔌️port` | `NativeStoragePortV1`, cleanup port/access, observation port, journal observation, `NativeFixedPlatformOptionsV1` (the fixed module re-exports the options type for the frozen importers) |

Edits without a new file:
- ui `🌳️Tree`: `treeHeaderActionsClassName` moved into actions; `FileTreeNode`, `TreeFilesProps` moved to props (index re-exports).
- ui `🕸️Diagram/📐️layout`: `DiagramLayoutPosition` and `diagramLayoutPageSize` moved to wire, `DiagramLayoutMerge` to paged-store; importers updated.
- ui `🏗️builder/🌐️vite`: `MeshCollectionAssetSpecV1` moved into mesh-collection (index re-exports).
- `⚠️diagnostic` -> `🧵️job/⚠️failure`: `faultFromOriginalTurnFailure` moved to the job failure module (it owns `OriginalTurnFailure`; diagnostic no longer imports job). Barrels and the two test users updated.
- ui `🧬️contract/🧩️component` and `🛡️limits`: import the contract types from `manifest/🏷️ui-contract-and-canvas-types` instead of the `@semio-tech/framework` facade. This single change removed 50 cycle edges (the facade re-exports manifest, which imports component).
- actor `shard-client/🔐️activation`: the `ShardClient` type import became the opaque `ShardClientIdentity` (activation only compares it by reference); `bindCapturedReturnDispatch` is generic over the client. `🩹️ui-patch` no longer imports activation.

### 1.2 ui ports: the ui no longer imports actor or kernel (schema-first, injected, bind-once)

New directory `🖱️ui/🧬️contract/🧵️retained/🔌️host/`. Every file is a leaf with no runtime imports, so the binding slot exists before either side evaluates (the first attempt put the slot in the instance module and failed with a TDZ error in vitest; caught by running the OS suite, fixed by moving the slot into a leaf).

| leaf | ui-side interface | bound by |
|---|---|---|
| `🔌️host/🟦️.ts` | `UiActivationLease`, `UiInstanceLifetime`, `UiPatchReceipt`, `UiPatchValue`, `UiPatchSource`, `UiPatchAuthority`, `UiIndexGrant` | `actor/shard-client/🩹️ui-patch` (`bindUiPatchAuthority`), class implements `UiPatchSource` |
| `🔌️host/📥️input` | `UiInputField`, `UiInputFragment`, `UiInputRelease`, `UiPayloadDetachment`, `UiInputHost` | `kernel/📤️return/📦️content/📥️input` (`bindUiInputHost`) |
| `🔌️host/🏦️composition` | `UiResidentComposition`, `UiResidentCompositionAuthority` | `actor/shard-client` index (`bindUiResidentComposition`) |
| `🔌️host/📦️return-content` | `ReturnContentAuthority`, `returnContentMatches` | `kernel` input (`bindReturnContentMatcher`); `actor/shard-client/🔐️activation` asks it, so activation no longer imports the kernel |
| `🏘️instance/🪪️resident-scope` | `OwnedUiResidentScope`, `residentScopeMatches` | `retained/💾️resident` (`bindResidentScopeMatcher`) |

Effects: `retained/🏘️instance`, `💾️resident` and `🩹️operations/📥️wire/📃️pages` no longer import actor or kernel (the `OwnedNativeUiPatchAuthority`, `ShardClient`, `OwnedKernelReturnInputField|Fragment|Release|PayloadDetachment` brand checks go through the seams); 10 retained files swapped `NumericIndexGrant` for the ui-owned `UiIndexGrant`; the instance <-> resident cycle (20-31 edges) is gone. An unbound seam refuses (fails closed) or throws "unbound" for the rich input/composition seams. The owners always bind on load, because a genuine patch source, input field or composition can only exist after its owner module loaded.

Language-agnostic corpora (JSON, checked by both bun tests and the module vitest gates):
- `🔌️host/🧪️tests/🧫️fixtures/🔣️.json` + `🟦️.ts`: 6 tests (unbound/bound/approving/rejecting/bind-once for patch, input, composition, return-content).
- `🏘️instance/🪪️resident-scope/🧪️tests/…`: 3 tests, including the real wiring (an instance refuses a forged resident scope once the resident module bound its check).
- Wiring tests in the owners, registered in their vitest configs: `actor/shard-client/🩹️ui-patch/🧪️tests` (forged patch source refused), `actor/shard-client/🧪️tests/🔌️ui-composition`, `kernel/📤️return/📦️content/📥️input/🧪️tests/🔌️ui-input-host`. `actor/🧪️tests/🎚️config` now lists standalone suites in `include` (comment amended: in-source files stay out of it).
- The shard-client collaborators declare narrow roles instead of importing the shard client: `actor/shard-client/🧩️hub` (`ShardClientHubPort<Shell>` + 8 role interfaces); `ShardClientHub implements ShardClientHubPort<ShardClient>`; only `resident-bootstrap` needs a typed shell, now `UiResidentComposition`.

### 1.3 Cross-package imports through public entries (38 files)
Relative deep imports became package entries that already exist in the authored exports: `@semio-tech/framework-compiler-ts/syntax/module-tree` (24 tests), `@semio-tech/schema-validator-ts`, `@semio-tech/framework/filesystem/observation/{descriptor,current}`, `@semio-tech/framework/test/source-text`, `@semio-tech/framework/job/failure`. One test (`value/♻️retirement/🎮️controlled`) needs `inspectRustPathMounts`, which the compiler entry does not export; left relative.

### 1.4 Policy (shared file `🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs`, tests in `🧪️tests/🚫️resolved-boundary-classification`)
- `no-circular`: `from.pathNot` and `to.pathNot` `(^|/)🧪️tests/`. A module's own in-source tests are registered behind `import.meta.vitest` (dead code in every production bundle) and import the module for types, so the module <-> test pair is not a production cycle. Longer cycles that pass through a test still surface at their non-test edges. Impact: 26 edges.
- `ui-no-framework-packages`: `from.pathNot` = the existing `BOOTSTRAP_TOOLING_ENTRY_PATH` (`📜️script.ts`, the vite builder entry, vitest configs), the same carve-out `no-cross-technology` already has. Scripts are composition roots; injection happens exactly there. Impact: 99 edges (95 scripts importing `process`).
- 8 new corpus cases (cycle and ui rule, checked against our regex evaluation and dependency-cruiser's own `validateDependency`). `bun test` on the classification and bootstrap suites: 23 pass (before the new cases); classification suite alone: 4 pass, 133 expects (after).
- Neither rule is in `dependencyDirectionStrictRules`, so the `verify` gate is not relaxed. If you disagree with either exemption, the "policy-neutral" column in section 0 is the honest number.

## 2. Verification (exact commands, outcomes)

All from `/Users/ueli/Documents/semio`.
- Type check: scratch `tsc` config extending the root, include `🧰️framework/🔨️modules/**/*.ts(x)` and `🧰️framework/📦️packages/**/*.ts` (6,129 files, Check 33 s): `node_modules/.bin/tsc --noEmit -p <C/🗑️generated/lay12-run/tsconfig.json>`. Baseline 49 errors (missing packages), after 42; the diff of sorted messages vs baseline shows no new error that is mine (peers' concurrent cargo-execution test and process package edits appeared and disappeared during the run).
- Latent type error exposed (and fixed) by breaking a cycle: `💾️storage/🧪️tests` built a port whose `withOperation` returned `Promise<unknown>` against the now-resolved generic `NativeStoragePortV1`; the test uses `<T>`.
- vitest (via `lay12-tools/vt1.py`, same policy/env wiring as `tst2-tools/runvt.py`):
  - `🎭️actor` config: 12 files / 107 tests pass (was 10 / 104; 3 new tests). Includes shard-client (10 tests), lifetime, page, return, output.
  - `🎠️kernel`: 10 / 93 pass (was 9 / 91). `📡️replication`: 2 / 21. `⏳️async`: 2 / 27. `🔄️machine`: 1 / 3. `🧊️3d`: 1 / 17. `🖱️ui/🎨️styling`: 3 / 82.
  - `◻️2d`: 7 files pass, 1 file fails 1 test (`scene text placement matches native canvas pixels`: playwright chromium missing, environment).
  - `ui-react`, `canvas/react-renderer`: red before and after, unresolved `@dnd-kit/sortable`, `color` (deps frozen). The Tree/Diagram/scene edits are type-checked only.
  - OS renderer `UiDocumentStore` in-source suite (`SEMIO_TEST_LEVEL=exhaustive`): 114 pass, 103 fail; 100 are `Missing exact fixture request` (native fixture not available here), the rest are a 300 s timeout, a missing `🎬️action.rs` and one hash assertion. These suites cannot exercise the resident seam in this environment; the new seam tests do.
- bun suites of the touched process/job/diagnostic/tool-run/io areas (55 files via `tst2-tools/runbun.py`): 43 green, 12 red; every red file is red in `result-tst2-suites.md` with the same cause (`Live Original … issuance required`, `tree-sitter-wasms` not installed, receiver admission closed).
- New seam suites: `bun test` host (6), resident-scope (3), then under vitest in actor/kernel (3 files, all green).
- Cruise: `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework/🔨️modules 🧰️framework/📦️packages --config …/🕸️dependency-boundaries/🟨️.cjs --output-type json`. Policy-neutral numbers from a config copy that restores the two original rules.

## 3. Decisions
1. Seams are bind-once slots in no-import leaves, matching the repo's existing idiom (`bindCapturedReturnDispatch`: "ES imports are read-only, so the client binds through the declaring module"). Type-position classes became interfaces; static brand checks became authority objects. Forged objects are still rejected by the owner's private-field brand check.
2. Duplicating small wire shapes (`UiInstanceLifetime`, `UiPatchReceipt`, `UiIndexGrant`) in the ui is deliberate interface segregation; the owning schemas stay in actor/value.
3. Hub ports and the policy exemptions can raise a raw count temporarily; I kept the cleaner structure.

## 4. Still open (with the data to decide)

### 4.1 `no-cross-package-relative` 534 (ui-internal): needs exports or a relocation (skipped by the freeze)
All 267 distinct edges are `ui/🎯️targets/⚛️react/*` (package `@semio-tech/ui-react`, nested owner) importing `ui/🧱️elements/*` (70 edges to about 60 elements) and `ui/🔨️modules/*` (about 30 helpers), which belong to the `@semio-tech/framework` region. No package exports them. Options: (a) a manifest batch adding export targets for `ui/🧱️elements/**` and `ui/🔨️modules/**` under `@semio-tech/framework` or a new `@semio-tech/ui-elements`; (b) relocate elements/modules under the ui-react owner (taxonomy `elementsDirName` constrains where `🧱️elements` may live). Not doable under the freeze. Other ncpr: actor/kernel -> ui seam leaves and retained classes 52, ui tests 4, plus 6 single edges (surface/schema/value/packages). The seam leaves would become export targets in the same manifest batch (+18 removable).

### 4.2 ui -> framework residue (174 adjusted, 273 neutral)
SRC 117: facade `@semio-tech/framework` from ui-react 31 (Introduction/Tutorial/CanvasPick/ShellLocale types, defined in `manifest`/`onboarding`/`platform`; the generated TS lives at `manifest/🤖️generated/📜️ui-contract` and its path is baked into `taxonomy.json` generator contracts, `ui/🧬️contract/…/📜️script.ts:105` and a Rust `include_str!`, so moving it needs a regeneration), `assets` 22 (`IconName`, icon catalog, delivery/mesh in the vite builder), `value` 15 (`NumericIndex` is the core data structure of the retained document model, 11 files; `OwnedResident*`), `process` 14 (ui build tooling libraries: python/dotnet providers, axes execution), `manifest` 11 (generated contract types), `transient` 11 (`ephemeralBox/Map/Set`; a ui-local store would not be reset by the shell's `defaultTransientStore.reset()`), `schema/validator` 5, `interaction` 3. TEST 57: compiler `module-tree` 11, assets 11, process 9, facade 8, value/manifest/schema/test fixtures, stories.
Recommended decision (owner): either (i) name a short allow-list of dependency-free foundation modules the ui may use (`transient`, `schema/validator`, `value` data structures, `assets` vocabulary), or (ii) move the retained document model (`ui/🧬️contract/🧵️retained/**`, built on `value`'s `NumericIndex` and bound to actor/kernel through the new seams) and the builder tooling out of `ui` into the modules that own them. I did not do (ii): it is a relocation of about 130 files with the manifest freeze and exports.

### 4.3 Remaining cycles (12 adjusted, 38 neutral)
- 26 module <-> own in-source test pairs (policy-exempt; structurally the test imports the module for types and the module registers the test). A full fix means tests that register themselves instead of the module knowing them.
- `kernel/📤️return/📦️content/📥️input` <-> `actor/shard-client` index (1): the index imports `OwnedKernelReturnContent` only to put it in `shardClientTestDependenciesV1`, which three OS tests (lay6 area, not touched) destructure. Remove the class from the bag and import it in those tests.
- `actor/shard-client/💾️resident-bootstrap` <-> index: gone; `activation` <-> `ui-patch`: gone.
- `process/🔒️leases` <-> `📦️metadata`: intentional lazy `import()` of a function that needs the private lease registry (`requireResourceLeaseCustodyV1`); pass the check as a parameter (changes two test callers).
- `ui/…/💾️resident` <-> `🩹️operations/📥️wire/📃️pages` (1): mutual brand pair inside ui.
- `process/…/💾️storage` (10): the index dispatches to the persistent and fixed strategies, which call core primitives in the index (`readNativeStorageMountedChildV1`, `requireNativeStorageChildCleanupV1`, `readNativeStorageImageReservationV1`, `withNativeStorageFundedChildReservationV1`). Needs the index split into core primitives and orchestration (WeakMap-backed brands, dense code); not attempted.

### 4.4 Notes for the coordinator
- The shared scratchpad is shared with other executors: my `q.ts` was overwritten by someone else's file; my scripts are now in `C/lay12-tools/` (`agg.ts`, `fas.ts`, `swap.ts`, `edgeimp.ts`, `cruise.sh`, `tsc.sh`, `vt1.py`).
- The index is staged continuously by another process (`git status` shows `M `/`MM`); file lists below are from my own edit log, not `git status`.
- `fas.ts` finds the minimal feedback edge set of a cruise; `swap.ts` rewrites deep imports to package entries; both are reusable for the WP8/WP10 executors.

## 5. Files

Created (all under `🧰️framework/🔨️modules/`): `◻️2d/📐️drawing-contract/🟦️.ts`; `🚪️io/🪶️sqlite-snapshot/🧩️artifact/🚦️checkpoint/🟦️.ts`; `⏯️tool-run/📊️progress/🪟️presentation/📸️capture/🟦️.ts`; `🖱️ui/🎬️scene/🚚️lane-contract/🟦️.ts`; `📡️replication/🔖️sync-protocol/🟦️.ts`; `⏪️time-travel/🧪️testing/🔬️probe/🟦️.ts`; `…/🎗️band/📋️readout/🟦️.ts`; `🏃️process/🔒️leases/🎛️control/🟦️.ts`; `🏃️process/🏗️build/🎟️capacity/🎫️lease/🟦️.ts`; `🏃️process/🧭️routing/🎛️command/📥️request/🟦️.ts`; `🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️port/🟦️.ts`; `🖱️ui/🧬️contract/🧵️retained/🔌️host/{🟦️.ts,📥️input/🟦️.ts,🏦️composition/🟦️.ts,📦️return-content/🟦️.ts,🧪️tests/🟦️.ts,🧪️tests/🧫️fixtures/🔣️.json}`; `…/🏘️instance/🪪️resident-scope/{🟦️.ts,🧪️tests/🟦️.ts,🧪️tests/🧫️fixtures/🔣️.json}`; `🎭️actor/📮️shard-client/🧩️hub/🟦️.ts`; `…/🩹️ui-patch/🧪️tests/🟦️.ts`; `…/🧪️tests/🔌️ui-composition/🟦️.ts`; `🎠️kernel/📤️return/📦️content/📥️input/🧪️tests/🔌️ui-input-host/🟦️.ts`. Tools: `C/lay12-tools/*`.

Edited:
- `2d`: `🟦️.ts`, `🔀️booleans/🟦️.ts`, `🔀️booleans/🛤️paths/🟦️.ts`, `🛤️path/📏️flatten`, `🛤️path/🖊️stroke`, `🔍️trace`, `📝️text/…/📇️face/🟦️.ts`, `…/🧵️scalar/🟦️.ts`.
- `io/sqlite-snapshot/🧩️artifact`: `🟦️.ts`, `🗂️row`, `🗂️table`, `🔤️keys`.
- `tool-run/…/🪟️presentation`: `🟦️.ts`, `🧬️update/🟦️.ts`.
- `ui`: `🎬️scene/{🟦️.ts,🛤️lanes,🌐️world3d,📄️documents}`; `🧱️elements/🌳️Tree/{🟦️.tsx,⚡️actions,🎨️chrome,🧩️props,🪢️views}`; `🧱️elements/🕸️Diagram/{🟦️.tsx,📐️layout/{🟦️.ts,⚙️job,📊️job-types,📜️wire,🗃️paged-store,📢️publication}}`; `🎨️styling/🏗️builder/🌐️vite/{🟦️.ts,🗺️mesh-collection}`; `🧬️contract/{🧩️component,🛡️limits}`; `🧬️contract/🧵️retained/{🏘️instance,💾️resident,🩹️operations/📥️wire/📃️pages,🩹️operations/📥️wire,📦️wire/🔢️bytes,📦️wire/🧾️typed,📖️read-lease,🎬️scene/🔗️binding,🔢️hash}`; ui test files switched to package entries (wgpu presented-source/close/draw, runtime-tree-retirement, prepared-close, mounted-layout).
- `replication`: `🟦️.ts`, `🚪️io/💾️binary/{🔗️causal,👥️presence,🔗️causal/🧮️backbone,📡️wire}`, `🚪️io/📥️bootstrap`.
- `time-travel`: testing `{📄️page,🎗️band,🧱️chrome,🖊️editor,📜️history,🧵️history-session,🧊️wgpu-mirror,🧠️state,🧩️scenario,🔁️replay-arm,🔄️reprojection,♿️aria-oracle}` and `📦️packages/🟦️typescript/🧪️testing/🖊️editor`.
- `process`: `🔒️leases/{🟦️.ts,♻️retirement,📦️metadata}`, `🧭️routing/📥️invocation/🏃️process/{🟦️.ts,🧹️cleanup}`, `🏗️build/🎟️capacity/{🟦️.ts,🪆️scope}`, `🧭️routing/🎛️command/{🟦️.ts,🎟️custody}`, `🧪️testing/🦀️cargo/🎯️exact/🛠️preparation/{🟦️.ts,📋️request}`, `💾️storage/{🟦️.ts,🌐️platform/📋️contract,🧱️fixed,🌱️birth,🧹️cleanup/♻️retirement/📋️contract,🧪️tests}`, `🔒️leases/♻️retirement/🧭️continuation`, `🧪️testing/📥️invocation/🧪️tests`.
- `diagnostic`: `🧾️retained`, `📦️packages/…/🧾️retained`, `📤️wire/👣️cursor/🧩️domain/🧪️tests`. `job/⚠️failure/{🟦️.ts,🧪️tests}` and `🧰️framework/📦️packages/🟦️typescript/🧵️job/⚠️failure`.
- `actor`: `📮️shard-client/{🟦️.ts,🔐️activation,🩹️ui-patch,↩️captured-return,⏱️watchdog,🌉️host-effect-bridge,💾️resident-bootstrap,📥️ui-acknowledgement,📬️request-transport,🧭️routing,🪪️exact-activation,🚪️instance-lifecycle}`, `🧪️tests/🎚️config`.
- `kernel`: `📤️return/📦️content/📥️input/🟦️.ts`, `🧪️tests/🎚️config`.
- Alias swaps (38 test/script files): `⏳️async`, `🎒️pack`, `🌱️value`, `🎭️actor`, `📡️replication`, `🖼️canvas`, `🧵️job`, `⏱️trace` test dirs; `value/♻️retirement/🧪️tests/🧫️fixtures`; `process/🔒️leases/♻️retirement`.
- Policy: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/{🟨️.cjs,🧪️tests/🚫️resolved-boundary-classification/{🟦️.ts,🧫️fixtures/🔣️.json}}`.
