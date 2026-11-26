# F92: undefined-name failures in dependencies reading, derivative and native-dependencies tests

Lane F92, 2026-11-26. Decision per name: production bug or harness bug. All three `ReferenceError`s were harness bugs; no production file changed.

## Undefined names

| Name | Where it is used | Verdict |
|---|---|---|
| `descriptorProtocolV1` (also `descriptorRefusalV1`, `isConcurrentEditV1`, `maintenancePause`, `readRepositoryGraphMaintenanceFileOnceV1`) | `readRepositoryGraphMaintenanceFileV1` in `💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts`, imported properly at the top of that module | Harness: the reading test copied nine functions by name with a hand-written port list |
| `openNxSdkStampMemoV1` | `verifySdkSourceIndexReadV1` in `🔏️index/🟦️.ts`, imported from `./🧊️stamps/🟦️.ts` | Harness: derivative test copied one function by name with a hand-written port list |
| `javascriptDependencyCapsuleIdentityV1` (surfaced after the first fix) | helper of the dependency storage module, used by the preserving-source functions | Harness: seven functions of the storage module copied one by one |
| native-dependencies tests 2/6 | not an undefined name | Production boundary regression, see below |

The names exist and are imported in the real modules, so no grep for further dead uses was needed.

## Root fix: use the shared extractor

`🔨️modules/🏃️process/🧪️testing/🧩️extracted-function/🟦️.ts` already has `loadProductFunctionsV1` / `loadProductFunctionsSyncV1` (public as `@semio-tech/process/testing/extracted-function`). They resolve the transitive helper set (functions, const/let/var, classes, imports) from the module source and import the real module for imports, replacing only the names given in `ports`. The three test files kept hand-rolled `new Function` extractors that copy named functions only; they are migrated to the shared loader, so a new helper in the production module can no longer break them. The shared extractor itself needed no change (it is the fix); no other user of it changed.

Edits (tests only):
- `📦️dependencies/💾️storage/🧪️tests/👓️reading/🟦️.ts`: the selected-owner test (9 copied functions -> `loadProductFunctionsV1(adminPath,["planSelectedRepositoryDependencyGenerationsV1"],bindings)`), the ready-writer test and the completion test (`compile` now loads by result name through `loadProductFunctionsSyncV1`, the 7-function shared loop likewise). `new Function(`/extractor uses 34 -> 30.
- `🛠️tools/📦️sdk/📥️source/🔗️derivative/🧪️tests/🟦️.ts`: the three source-text `extract` helpers now take a module path and call `loadProductFunctionsSyncV1`; uses 4 -> 1 (the remaining one extracts statements of a function body, not a function).
- `🧪️testing/🧩️extracted-function/🧪️tests/🔣️.json`: ratchet ceilings of the two files lowered (34 -> 30, 4 -> 1).
- Reading test, cancel/drift progress rows: the abort and the drift write ran in `setImmediate`, but `yieldContinuation` now yields a microtask while less than the turn budget passed (`createEventLoopTurnV1`), so a fast read finished before the callback. They now run synchronously at the batch publish (same `events` order). This was a second harness bug hidden behind the first (the earlier failure cascade closed the Original owner).

## Counts (`repo test-file`, one file at a time)

| File | Before | After |
|---|---|---|
| dependencies reading | 49 pass / 36 fail | 78 pass / 7 fail |
| derivative | 17 / 7 | 19 / 5 |
| native-dependencies tests | 2 / 6 | 2 / 6 (not extractor related) |
| extracted-function tests | not run before | 10 / 1 (the one failure is the ceiling ratchet, see below) |

Zero `ReferenceError` left in reading and derivative.

## Remaining reds, causes found (none caused by this lane)

- reading 7: all trace to one law, "Store settlement isolated process laws": the child run for mode `frozen` (a thrown frozen non-Error marker) exits 0 where the corpus requires 1; the following tests fail by "Original admission closed" cascade. The working tree has uncommitted edits in `🧭️routing/📥️invocation/🏃️process/🟦️.ts` (peer); this is process-admission behavior, not extraction.
- derivative 5: isolated children fail with "A test reached the live repository store of the checkout ... give it a private repository workspace (privateRepositoryWorkspaceV1)" at the sealed-capture test; the same guard reaches the three other isolated rows. Test-isolation / store guard, not extraction.
- native-dependencies tests: first failure is the "source closure keeps first party and system boundaries" assertion. The first-party closure of `🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts` now reaches `⚡️caching/🔍️discovery/📂️source/🟦️.ts` through `📥️opening` -> `🧹️headroom` -> `🕸️graph/📏️headroom` -> `🧾️closures` -> `🕰️generations/🩹️closure-repair/🟨️.mjs` -> `library/🏗️build/🕸️graph/🩹️closure-repair/🟨️.mjs` -> `🏗️build/📥️inputs/🟨️.mjs`, whose line 1 imports `isOpaqueSourcePathV1,opaqueSourceInputExclusionsV1` from the discovery module (forbidden by the fixture). A real boundary regression of the build graph work (commit 8d17d8ba399 touched that file); fix is to move those two helpers out of `🔍️discovery/` (or have `📥️inputs` not import them). Not done here: product build code, seal window. The other five failures there are the same Original-owner cascade. The reproduction was an esbuild metafile walk with the test's first-party resolver (scratch script, deleted).
- extracted-function ceiling ratchet: red because peers added 12 files with `new Function(` (os mcp/plugin/renderer, `🔁️rescan`, `♻️reclaim`, `🧊️core-modules`); the `slack` list is empty, so the ceilings of this lane's files are exact.

## Not done

F87's `closureCodeV1` in `🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts` (142 KB test, `.mjs` sources, its own import ports) was not migrated to the shared loader: F87 added an opt-in `lets` flag that fixes the observed fault, and a migration of that file is a larger rewrite than this lane's seal-window scope. Recommended follow-up.

## Checks

`repo typecheck` on the reading and derivative test files: passed (3 errors elsewhere, not counted). `dangling-imports.py` on the reading folder, the derivative tests folder and the extracted-function tests folder: exit 0. No landing lock needed (test files and a test ceiling table only). Own scratch `F92/` removed.

# Round 2 (coordinator follow-up)

## 1. Native-dependencies boundary regression: fixed in product code
`isOpaqueSourcePathV1`, `opaqueSourceInputExclusionsV1` and `SOURCE_OPACITY_PROTOCOL_V1` (a pure function of the two schemas that define the reserved name prefixes) lived in `⚡️caching/🔍️discovery/📂️source/🟦️.ts`, so `🏗️build/📥️inputs/🟨️.mjs` (and 20 more modules) reached discovery. They now live in the neutral owner `🏗️build/📥️inputs/👻️opaque-source/🟨️.mjs` (+ hand-written `.d.mts`), which imports only the two schema JSON files. `📂️source` imports it for its own `sourceFiles`; all 24 importers were updated (grep of the three names, codemod split of the import lists, no re-export), and the command-source export fixture no longer lists them under `📂️source`. A fresh esbuild metafile walk of the first-party closure no longer contains `/🔍️discovery/`; the "source closure keeps first party and system boundaries" test passes.
Native-dependencies tests: 2/6 -> 6 pass / 1 fail. Not 8/0: after the boundary the test used to stop at the live-store guard; with the checkout-rooted Original replaced by `privateRepositoryWorkspaceV1().invoke` (storage root separate from code root: `testNativeDependencies(workspace, output, invocation, parent, storageRoot)`, `rust-toolchain.toml` written into the fixture root, python port pushes `...series`, compiler lane gets `scriptInvocationEnvironment(invocation, process.env)`) the last law reaches `withRepositoryCompilerLaneV1`, which runs a real Cargo compile and overflows the 4 MiB output cap (first bounded run: "Original physical body cutoff reached" at 10 s). That is a heavy cargo run, banned in the seal window, so the law is not verified green; the seal-window-safe part (6 of 7 tests) is.
Other importer test files re-run: patches 8/0, startup `👁️workspace` 10/0, `🎟️phase` 19/0, `📥️inputs` 4/0, owner-view inference 2/0, native-manifests build-script 2/0, bazel inventory 2/0, command-source 11/0.

## 2. Store settlement isolated-process law: root cause = stale expectation, fixed
The process admission file was last written at 11:02 (about 2 h before this run), no `landing.lock.process` exists, so the peer edit is landed. Its contract (`ownedStops` in `track`, `createRequestedIssuedResourceControlV1 ... additionalSignal` doc): work that rejects with exactly the reason of the caller-owned stop signal, while the parent still runs, stays with the request and never poisons the Original. The Store frame aborts its `stop` with the failure and the test passes `stop.signal`, so `frozen`, `accepted-failure`, `release-failure` and `reader-frozen` correctly exit 0; only `undefined` (an `abort(undefined)` reason is a DOMException, so the thrown `undefined` is foreign) poisons and exits 1. Measured by printing the six child statuses: success 0, undefined 1, frozen 0, accepted-drain 0, accepted-failure 0, release-failure 0, reader-frozen 0. The corpus conflated "the settle failed" with "the process exits 1". Schema-first fix: `SourceOwnerStoreLifetimeCase` gained the required `poisonsOriginal` boolean (`🧬️schema/📥️install.json`), the corpus rows state it (true only for `undefined`), the law asserts `status == poisonsOriginal`. Reading test: 78/7 -> 84 pass / 0 fail.

## 3. Derivative isolated children: fixed
Cause: `withBuildTestInvocationV1` Original is born at the checkout, so `readNativeCallerAuthority(...).repositoryRoot` pointed at the live store while the fixture store was private. The isolated laws now start from `repository.invoke` (F83 helper `privateRepositoryWorkspaceV1`), the unsealed-publication phase control is a real descendant (`createRequestedIssuedResourceControlV1(store.control, 20000)`), and the pinned-Bun law prepares with the private root as workspace (checkout `package.json` copied in; checkout root from `testCheckoutRootV1`, since `repository.invoke` also moves the working directory). Derivative: 19/5 -> 24 pass / 0 fail.

## 4. Extracted-function ratchet and F87's closureCodeV1
- The 12 listed files and the 5 files whose table rows reached 0 are off `new Function(`: product functions by name go through `loadProductFunctionsV1/SyncV1` (rescan, reclaim x3, wgpu drivers x3 via a shared `factory`, plugin discovery, describe emitter x3); text that is itself the artifact under test (worker body, host shim, embedded class, staged guest, a drive region, a launcher body whose dynamic `import` line is stripped, the whole mcp publication projection with method-level AST surgery, one nested arrow of `driver-stages`) goes through the new shared `evaluateGeneratedSourceV1(text, result, ports)` (public barrel too, with a test). Honest limit: these generated-text uses are not counted by the ratchet; they are not loader migrations (a loader needs a top-level declaration).
- New shared `productClosureCodeV1(module, names, portNames)` returns the transpiled declaration closure from the same planner as the loader (functions, classes, const/let/var, destructuring, ported names left out). F87's `closureCodeV1` in source-demand is now a 1-line adapter over it: the `lets` flag, the two manual `let` prefixes and the hand-rolled AST closure are gone; the planner copies module-level `let` state by itself. source-demand 34/0, extracted-function tests 13/0 (ratchet green with exact ceilings; 5 emptied rows removed).
Counts, all `repo test-file`: rescan 1/0, reclaim 14/0, wgpu drivers 6/0, plugin discovery 13/0, describe emitter 8/0 (two stale source-shape assertions of the production driver updated: preflight line and `planEmitterJobV1`), mcp revalidate 8/0, mcp publication 24/0, wit-lowering 7/0, core-modules 10/0, staged-guest 2/0, cores 2/0, more-work-drive 33/0.

## Checks
dangling-imports exit 0 on every touched folder (caching, build, process testing, actor, os plugin/mcp/renderer). `repo typecheck`: no error introduced in a touched file after fixes (the remaining errors are in untouched lines of touched test files and importers: inputs `🧪️tests` 41, bazel storage tests, lookup-fault, wgpu fixture `Turn` shape, plugin request 158-167, cores 56, strip-debug, two-checkout). Landing locks: none taken (tests, test fixtures and one neutral module; no lock exists for these areas in the ticket). Scratch dir removed.
