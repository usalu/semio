# Graph inventory step slowness on Windows (2026-11-18)

## What the phase is
- Text: `🧠️memory/📋️slice-report/🔣️schema` step `inventory` (`boundMilliseconds` 600000), run by `sliceReadStepV1("inventory", inventory)` in `🕸️graph/📥️load/🟦️.ts` (`settledInventory`).
- "2110 of 600" is not units: it is the elapsed seconds of the step against its declared 600 s bound (`done`=seconds, `total`=bound). It only advances with the wall clock. 600 s is a declared slow-machine bound, nothing measured.
- The step is not the plain file walk. It is: admission memo lookup, then (on a miss) `closeBuildOwnerAdmissionV1` over 763 owners with `discoverBuildOwnerAdmissionFactsV1` per owner under `captureObservedSourceInputsV1` + `settleObservedComputationV1`, then `workspaceBuildFilesV1` + `buildGraphMembershipV1`.

## Measured
- Raw walk plus lstat of the whole tree (probe `inv-probe.mjs`, Bun, 64-wide batches as in `buildGraphMembershipV1`): 631,540 files / 255,785 dirs, walk 23 s, stat 15 s, 31k to 43k stats/s. Emoji paths, long paths, replica daemon are NOT the cost; `workspaceBuildFilesV1`/`buildGraphMembershipV1` are cheap (about 40 s even unfiltered).
- The slice worker (PID 3152) is busy in the owner-discovery part: 8 s sample = +15k read ops, +72 MB read, +278k other (stat/open) ops; lifetime 12 GB read, 42M non-read I/O ops (about 65 full-tree walks worth). `.cache/f/slice-memory.json`: observation scopes 1.8M to 2.0M entries, rowTable 132k to 234k rows and growing, so it derives and records per-file source facts for every owner.
- Only one worker round (no restart): worker started 08:20:31, whole 2500+ s inside `inventory`.

## Root cause (strong evidence, not a clean one-line fix)
- The admission memo `.cache/b/g/m/selected/<scope>.admission.json` (118 MB) is stamped 04:45; no read has finished and rewritten it since. A memo is a hit only if `module.implementation` equals the current graph implementation hash (`graphModuleAuthorityV1`, `🕸️graph/🔐️authority/🟦️.ts`), which hashes the graph entry modules, roster and `👁️observation/🟨️.mjs` (+ schema) and more.
- Those files keep changing under us: `👁️observation/🟨️.mjs` 06:32, `🕸️graph/🔐️authority/🟦️.ts` 06:02, `📁️workspace/🟦️.ts` 05:14, observation schema again later, plus 39 non-test files of the build tool changed since 04:53 (graph inference at 08:08 and 08:32). Every other agent edit to those files makes the memo and all implementation-keyed reuse miss, so each `repo run` re-derives the whole admission cold: 763 owners x per-owner observed discovery, about 40+ min on this loaded machine (about 8 min at 04:45 when the machine was idle).
- Not verified: the exact current vs memo implementation hash (cheap to confirm by logging `loadedGraphModuleAuthorityV1.implementation` against the memo `module.implementation`); which sub-step of owner discovery dominates (needs `SEMIO_BUILD_PROFILE_INPUTS=1` on a fresh run: prints `admission-memo`, `workspace-catalog-lookup`, `workspace-membership-hash` timings).

## Proposal (not implemented, architectural)
1. Key the admission memo by the observations only (each row already revalidates), not by the implementation hash of unrelated graph modules; or split the hash to the modules that change the admission format.
2. Run discovery with `SEMIO_BUILD_PROFILE_INPUTS=1` once to see which sub-step takes the bulk.
3. Stop editing the 13 hashed graph modules / observation mjs while a long activation runs; every edit restarts the cold derivation for the next run.
4. Make the "of 600" counter honest (seconds against bound) in the text: "elapsed 2110 s of expected 600 s".

No code changed. Probe script: `inv-probe.mjs` in this folder.
