# audit-live1021: live build-library fixes of 10-21 (11:59 to 14:49)

Read-only audit by auditor audit-live1021. Nothing was executed (no bun, no cargo, no tests); findings come from reading `git diff HEAD` of the named files and from grep inside named files. Paths are relative to the repo root `/Users/ueli/Documents/semio/`. `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `BLD` = `LIB/🏗️build`.

Source notes:
- The executor result files do not describe these live fixes: `result-conc-leases.md` is dated 10-20 20:27 and has no fix7; `result-storage-seam.md` covers the process module, not `BLD/🕸️graph/🟦️.ts`; `result-conc-graph.md` stops at section 12 (slice-fix) and has no 14:49 retirement fix. The diff was therefore the only source.
- Coordinator edits from 02:48 to 06:14 (ledger `regular()`, project-assembly) are already committed in b7b9f10 (09:35), so they are reviewed from HEAD.
- The index holds about 127 staged library files (`git diff --cached`, peer work). Reviewed against HEAD, not the index.

## Ranked findings

### 1. HIGH: closure retirement lost its time window, so exclusive custody is held for the whole retirement (convoy regression)
- `BLD/📥️inputs/💾️storage/🟨️.mjs:93-96` `prepareBuildGraphOptionalHeadroomV1` wraps one `image.withExclusiveMaintenance` around `retireOptionalGraphHeadroomV1` (`:85-91`), which calls `retireBuildGraphOptionalGroupsHeadroomV1`.
- `⚡️caching/.../📏️headroom/🧾️closures/🟦️.ts:107` loops `for(;;)` until free space reaches the minimum with live optional bytes at or below low water, or until no intent is left. It never checks elapsed time: the file has 0 `Date.now()` and 0 `retirementWindowMilliseconds`. Its `yieldContinuation()` yields the control, but the image custody stays held.
- The old code returned `partial` every 15 s so the caller released and re-requested the image. That is gone. Conc-graph section 10 says that window fixed the reader convoy; this change reintroduces it.
- Fix: restore the window in the loop (`if(Date.now()-started>=policy.retirementWindowMilliseconds)` return `{partial:true}`) and loop in `prepareBuildGraphOptionalHeadroomV1` with release between windows.

### 2. HIGH: the cumulative closure-proof budget turns a long retirement into a hard failure
- `closures/🟦️.ts:111` `recoveryBudget(custody)` returns one `{operations,bytes}` per custody. The loop passes it as `work` into every `captureClosureProof`, so it is charged on every iteration: ops at `:27`, index bytes at `:57`, member bytes at `:64` (`proof.work.bytes=priorBytes+acceptedBytes`).
- Caps in `BLD/📥️inputs/💾️storage/🧬️schema/🔣️.json:86,110`: `maximumProofBytes` 17,179,869,184 (16 GiB), `maximumReadOperations` 16,777,216.
- Each iteration re-captures the closure proof, so charged bytes grow with (iterations x closure size). On the 1.6 GiB, 90k-leaf optional cache this plausibly exhausts 16 GiB after about ten re-proofs. The retirement then throws "Bounded closure proof read bytes required" midway, the image stays half-retired, and the caller throws "Graph cache cannot admit...". The bound moves the failure. This is an estimate, not measured.
- Fix: budget each capture separately (or reset per iteration) and measure iterations x bytes on the real cache before GO.

### 3. HIGH: admission patience (fix7) changes try-lock semantics and stalls the claims sweep
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts` `directoryAdmission` (around line 246-250): `patience=!cleanup&&options.timeoutMs===0?...admissionPatienceMilliseconds??0:0`. Every `timeoutMs:0` request now waits up to 5 s for the admission gate, not only the case fix7 targeted. The result file calls these "try-locks"; they now block.
- `retireResourceQueueClaimsV1(directory,control,0)` is called from `acquireQueuedResourceLeaseBody` (`if(options.control&&claimSweepDue(...))`, around line 436). It passes `timeoutMs:0`, so the sweep itself gets the 5 s patience. A caller with timeout 0 pays it before `remaining()` is checked. The sweep runs once per 30 s per process, so the stall is periodic but real.
- `??0` on `admissionPatienceMilliseconds` and `claimSweepIntervalMilliseconds` (`🧭️order/🔣️policy.json` gate block) silently disables both if a key is removed.
- Fix: make patience an explicit option passed only by the admission-wait callers; sweep and try-locks pass 0; make both keys required in the schema and drop `??0`.

### 4. MEDIUM: single-flight recheck serializes the 40 s check but does not shorten it
- `BLD/🕸️graph/🟦️.ts:285-292` `generationFilesV1`: `recheck` (`:287`) shares one in-flight `store.recheck()`. Joiners await a check that completes after they arrive, so the logic is sound; only a caller in the same microtask after resolution reuses a finished check (negligible).
- The 11:59 cause was a roughly 40 s recheck under load. Now all 32 borrowers wait behind that one check, and the every-64-ops trigger (`:288`) still fires it.
- Fix: make `store.recheck()` O(1) (stat the image root and identity only) or time-box it and fail closed; add a test with a slow mock recheck asserting one call per window.

### 5. MEDIUM: the slice path silently drops unknown edge targets, and the VERIFY oracle is one-directional
- `BLD/🕸️graph/🕰️generations/🔌️load/🟨️.mjs:92` in `sliceClosureV1`: `if(!known.has(at)){if(at===name)throw unknown(at);continue;}`. Only the root throws. A transitive edge to an owner missing from the manifest is dropped and the slice is returned as complete.
- `BLD/🕸️graph/🟦️.ts:521-527` `requireSelectedSubgraphV1` checks selected subset of slice only, and runs only under `SEMIO_BUILD_GRAPH_VERIFY=1`.
- Fix: throw `GENERATION_UNKNOWN_OWNER` for any unknown edge target so the caller falls back; compare both directions in the oracle.

### 6. MEDIUM: the lean path for `show projects|targets` skips validation, drops unavailable warnings and fails VERIFY
- `load/🟨️.mjs:169` returns `{graph:{version:1,projects,dependencies:[],lean:true}}` with no `host.readGraph` (schema) and no `unavailable`.
- `BLD/📜️script.ts:194` sets `lean:true` for `show projects|targets`. `script.ts:197` prints `[build] unavailable project ...` from `graph.unavailable`, which is now always empty, so users lose the warning.
- `BLD/🕸️graph/🟦️.ts:540` (`SEMIO_BUILD_GRAPH_VERIFY`) compares the lean graph with the full baseline, so `show projects|targets` throws under VERIFY.
- Output of `show targets` is names only, so the lean output itself is correct.
- Fix: carry `unavailable` in lean, validate through a lean schema variant, and skip or project the VERIFY comparison when lean.

### 7. MEDIUM: triage previewScript discards the refusal reason and still runs bun directly
- `LIB/🔍️discovery/✅️owned-file-approvals/🟦️.ts:322` `catch { throw new Error("Generator target is not the exact owner JSON preview command") }` drops the validator message from `generator-commands/🟦️.ts:33-38`.
- `owned-file-approvals/🟦️.ts:323,325` execute `"bun", ...direct`. Pre-existing, but the function was touched, and AGENTS requires the repo binary with no bun runtime dependency.
- Fix: rethrow with `{cause}` (or the validator text); route the preview through the `repo` interface.
- The triage taxonomy fix itself is covered by fixture rows (`owned-generator-preview-inventory`, +4 `ownerExecutionRoutes`), so it is not a missing test.

### 8. MEDIUM: project-assembly line 330 (coordinator edit, committed in b7b9f10)
- `BLD/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs:330` classifies native targets by substring (`path.includes("/🏃️process/📦️artifacts/🏗️native-build/")`, `CARGO_TEST_DRIVER`) and awaits `nativeTargetCommandInputs` for every non-source command target. Brittle, and cost per target is unclear; no memo is visible in the line.
- Fix: classify from declared data (execution kind or owner field) and memoize `nativeTargetCommandInputs` per project.

### 9. MEDIUM: ledger `regular()` coerces nlink 0 to ENOENT (coordinator edit, committed)
- `BLD/📥️inputs/💾️storage/📒️ledger/🟨️.mjs:39`: nlink 0 throws `code:"ENOENT"`. `optional()` (`:40`) treats that as absent for `-journal`, `-wal`, `-shm`; `recheck()` (`:44`) propagates it. Line 5 of the probe also passes the main DB through `optional()`, so an unlinked main DB is also treated as absent there. The later open uses `regular()` directly, so this is probably harmless, but unproven.
- No test folder exists under the ledger (only `🧫️fixtures` and `🧬️schema`).
- Fix: restrict the coercion to the three sidecar names and add a fixture row for nlink 0 on the main DB and the journal.

### 10. MEDIUM: the 14:49 capacity failure may not be fixed (not proven by the diff)
- The 14:49 failure was "Inference reference text capacity exceeded". The chunk at `BLD/🕸️graph/🟦️.ts:330` caps 32 projects and 16 MB of `JSON.stringify` length (UTF-16 units, not bytes) per `portableInputs` call. The diff does not show that the capacity limit is per call rather than per process, and the 14:49 note says chunking "bounds values not text".
- No test names the chunk boundary in the named test dirs (not exhaustive).
- Fix: read the capacity constant in the observation module and assert it in a test with a project whose text exceeds it.

### 11. LOW: debug logs on the production retirement path
- `closures/🟦️.ts:67` and `:108`: `console.error("[DEBUG] ...")` in catch blocks. Remove before close (AGENTS temp-log rule) or attach as `cause`.

### 12. LOW: duplicate docstring emoji in one module
- `BLD/📥️inputs/💾️storage/🟨️.mjs:84` and `:92` both start with `🧹️`; `:217` too. AGENTS requires a unique emoji per docstring.

### 13. LOW: dead code and a retained legacy fallback
- `load/🟨️.mjs:71-82` `sliceNeighborsV1` is exported and imported (`BLD/🕸️graph/🟦️.ts:29`) but never called; `neighbors` is never passed. Delete both.
- `BLD/🕸️graph/🟦️.ts:499-507`: the `SEMIO_BUILD_GRAPH_SLICE=off` switch and the `!sliced` path back to legacy selected inference are a retained legacy/compat layer (AGENTS forbids them). Remove the switch and decide the no-generation case in one place.

### 14. LOW: test-only seams in a production storage module
- `BLD/📥️inputs/💾️storage/🟨️.mjs:24` (`graphFixtures`, `graphReadScopes`), `:106` (`optionalAttempts`/`optionalDeclined` counters), `:237-249` (`fixturePolicy`, fixture branch in `graphDatasetWorkspaceV1` and `repositoryBuildGraphDataDirectoryV1`). Move into the test harness or a test-only module.

## AGENTS checks on the changed code
- No `//` comments inside definitions in the added lines of the named fix files (checked by grep of the diff).
- Docstrings in the new code start with an emoji, except the duplicates in finding 12.
- No `legacy`, `compat`, `deprecat`, or `fallback` wording in added lines. The fallback in finding 13 is code, not wording.
- Runtime dependency: the closure reader spawns `process.versions.bun?"node":process.execPath` (`closures/🟦️.ts` around the `captureOwnedCommandResultV1` call). It is a hard-coded node/bun binary, not behind the repo interface.

## Missing tests (AGENTS: at least one language-agnostic test per feature)
- Claims sweep interval (`claimSweepDue`, 30 s) and admission patience (5 s): the resource-leases TS test and `🚪️admission/🔣️.json` have 0 hits for "sweep" or "patience". Add corpus rows: sweep skipped inside the interval, try-lock bounded at 5 s, caller timeout 0 not stalled by the sweep.
- Slice (`sliceClosureV1`, task and owner selections, `dependsOn` forms, `^target`, `*`): exercised only by TS (`load/🧪️tests/🟦️.ts:214`); `load/🧪️tests/🔣️.json` has 0 "slice" hits. Add a JSON scenario corpus shared by a non-TS oracle.
- Ledger nlink-0 coercion (finding 9): no test.
- Closure retirement: `🪟️retirement-window/🟦️.ts` names `retirementWindowMilliseconds` 3 times, but `BLD/📥️inputs/💾️storage/🧬️schema/🔣️.json` has 0 hits for it. Either the test asserts a removed field or it fails. Verify and fix.
- `closures/🧪️tests` (11 entries) was not audited.

## Not verified
- Nothing was run. Findings 1, 2 and 10 depend on sizes and limits that need one measurement each on the real optional cache.
- Single-flight correctness (finding 4) is from reading only; the mutation-style test it needs does not exist.
