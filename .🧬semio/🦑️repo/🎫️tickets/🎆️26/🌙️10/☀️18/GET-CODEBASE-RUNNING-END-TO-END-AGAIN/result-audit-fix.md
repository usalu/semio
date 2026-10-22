# result-audit-fix (executor audit-fix, 10-21)

LIB = 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library, BLD = LIB/🏗️build. All edits live, parse-checked (`node --check`) and unit-tested after each fix; no git-modifying commands, no `bun repo`, nice -n 10, one test process at a time (load waited below 30).

## #7 owned-file-approvals (LIB/🔍️discovery/✅️owned-file-approvals/🟦️.ts:322,328,331)
- Both validator rethrows now keep the refusal reason: `catch (cause) { throw new Error("Generator target is not the exact owner JSON preview command", { cause }); }`. The module has no localization table (all errors are plain English strings asserted by tests), so the message text is unchanged.
- The returned `command` is now `declared[0]` (the declared build-target literal, already proven equal to the expected one), no longer a hard-coded `"bun"`.
- NOT done: routing the preview through `repo run <previewTarget>`. The preview needs stdin projection, a byte-exact JSON stdout and no cache replay; `repo run` is an nx-cached runner with its own stdout framing, and no existing seam captures stdout of a declared `repo:script` target (`🏃️process` `runCmd*` inherit stdio). The actual spawn stays in `LIB/🧹️normalization/🔍️inventory-planning/🟦️.ts:313` (`spawnSync(execution.command, ...)`); it needs a capture-capable repo tool port, which does not exist yet. Coordinator decision needed.
- Test: `C/audit-fix-tools/preview-routes.test.ts` (ownerExecutionRoutes corpus + cause assertion): 1 pass, 32 expects.

## #8 project-assembly (BLD/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs)
- :26 `NATIVE_COMMAND_DRIVERS` (one frozen table) + exported `reachesNativeCommandDriverV1(sources)` replace the two inline substring chains.
- :330 `commandSourcesOf` memoizes `nativeTargetCommandInputs` per project (key = executor + options + configurations), used by the classification (:343) and the native input assembly (:371), so a target pays the closure once.
- :343 `nativePolicyTarget` now also requires `policy.executor !== "repo:noop"` (noop aggregates such as workspace:cpp are never native-by-closure); `execution === "native"` (declared) still wins; cargo owner tests (closure reaches the Cargo test driver) stay native.
- NOT done: classification purely from declared data. Targets without `metadata.semio.execution` (the cargo owner tests) only expose the driver through their script closure; declaring `execution:{kind:"native",scope:"test"}` on every such manifest is a repo-wide manifest sweep (forbidden here). The table is the single place to delete once manifests declare it.
- Test: new corpus `nativeDrivers` (5 rows) + schema + test in BLD/🕸️graph/📥️inference/🧪️tests/💾️cache: 9 pass, 2 fail. The same 2 failures (`workspace policy refusals...`, `artifact packages replay...`: `/var` authority ancestry and an unrelated `prepare-prototype` cache decision) fail identically on the HEAD version of project-assembly (verified by a brief swap and restore), so they are pre-existing.

## #9 ledger (BLD/📥️inputs/💾️storage/📒️ledger/🟨️.mjs:39)
- `regular()` is strict again (nlink 0 fails with the bounded `link-count` observation). `optional()` treats an nlink-0 leaf as absent only for names ending `-journal|-wal|-shm`; an unlinked main DB or gate DB now fails loudly in the probe, `recheck()` and the open path.
- Corpus: `unlinkedLeaves` (6 rows) appended to `📒️ledger/🧫️fixtures/🔎️observation.json`; test added to `🧪️tests/📒️ledger/🔎️observation/🟦️.ts`. Result: 2 pass, 99 expects (the existing `unlinked` observation row passes again).

## #12 duplicate docstring emoji (BLD/📥️inputs/💾️storage/🟨️.mjs:84,217)
- :84 `🧯️`, :217 `🫧️` (:92 keeps `🧹️`). Other pre-existing duplicates in the module (🧷️x3, 🪪️x3, 🚰️x3, 🔒️x3, 📏️, 🗂️, 🔐️) are outside the finding and untouched.

## #14 test-only seams out of storage (BLD/📥️inputs/💾️storage)
- Finding correction: `graphReadScopes` is production reader custody (used by `withRepositoryBuildGraphConsumerV1` and the observation-path check), not a test seam; it stays.
- New test-only module `🧪️fixture/🟨️.mjs` (+ `🟨️.d.mts`) owns `withRepositoryBuildGraphFixtureV1`, `withRepositoryBuildGraphPublicationFixtureV1`, the fixture policy (`fixtureSchema.definitions.Policy/PublicationPolicy`) and the attempt/declined counters (hook `observeOptionalPublication` on the scope object).
- Production keeps one generic, validated injection point, no counters and no fixture policy: `withBuildGraphDiagnosticBackingV1(backing, body)` (:234, requires a live backing with root and workspace), `currentBuildGraphDiagnosticBackingV1` (:239), `withBuildGraphWriterScopeV1` (:243, refuses outside a live backing), `repositoryBuildGraphWorkspaceV1` (wrapper; `graphDatasetWorkspaceV1` keeps its name because five tests inject it by name). `graphFixtures` is renamed `backings`; the fixture-schema parse, the `withRepositoryCacheGraphFixtureV1` import and :237-249 are gone from the production module. The pinned closure list (including `./🧪️fixture/🧬️schema/🔣️.json`) is unchanged, so the closure pin test still holds.
- Importers re-pointed (7 test files: closures `🟦️.ts` + `♻️groups.ts`, storage `♻️scoped`, observation `🧱️blocks` (+child), `🪪️presence` (+child)); `🛑️primary` test injection renamed `graphFixtures` -> `backings`.

## Tests run (bun test on the specific files)
- storage 📒️ledger/🔎️observation: 2 pass / 0 fail.
- storage 🛑️primary: 2 pass / 0 fail (after the injection rename).
- storage 🔏️closure: 1 pass / 0 fail.
- storage tests/🟦️.ts: 19 pass / 11 fail. All 11 fail for causes independent of this fix: 8x `Original parent Script process invocation required` (needs the repo runner as parent, not runnable without `bun repo`), 1x `generationScopeV1 is not defined` (graph/generations, owned by a peer), 1x Ajv strict `port` schema, 1 unnamed. Before I restored the `graphDatasetWorkspaceV1` name it was 17 pass / 13 fail (the 2 extra failures were my rename; fixed).
- closures tests/🟦️.ts, storage ♻️scoped: only fail with the same missing-parent envelope (33x / 4x); the fixture entry points could not be exercised end to end. Smoke check instead: both modules import, all 4 new production exports are functions, `withRepositoryBuildGraphFixtureV1` reaches its first gate (`Live Original process issuance required`), the backing/writer gates refuse as designed.
- cache 💾️ (inference): 9 pass / 2 pre-existing fail (see #8).
- Need a repo-runner run (`repo run repo-graph-maintenance:test`) after the E2E to confirm the fixture-based suites (closures, scoped, observation blocks/presence).
