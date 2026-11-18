# R180: nested cache custody refusal of g1-m10 (living report)

## 1. Cause (found by reading, proven by a law)

Wall of g1-m10, 2026-11-18 04:04 UTC (`/Users/Shared/semio-store/v1/origins/g1-m10/ready-log-workspace-failed.txt`):
`Original nested cache custody refused` at `norm-din4108:build`, then `Graph readers and namespace release failed`.

Where it is thrown: `⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🟦️.ts`, `withOriginalNxFixedCacheV1`, the branch taken when a custody is already open in the same async context (`cacheScope.getStore()`): one anonymous `throw Error("Original nested cache custody refused")` for six different conditions (workspace, tooling identity, environment object, graph digest, implementation identity, and `held.mode==="shared" && mode==="exclusive"`).

Which one fired (derived from the Nx patch `⚡️caching/🩹️patches/nx@23.2.0.patch`):

- `TaskOrchestrator.resolveCachedTasks` (and `applyCachedResults`) open a **shared** custody with no outputs (`withOriginalNxFixedCacheV1(..., [], 'shared', ...)`).
- Inside `resolveCachedTasks`, `cache.get` / `cache.getBatch` ask the remote (fleet) cache. On a **remote hit** they call `applyRemoteCacheResults`, which the patch wraps in an **exclusive** custody (`withOriginalNxFixedCacheV1(..., 'exclusive', publish)`).
- Exclusive nested in shared hit the rule `held.mode==="shared" && mode==="exclusive"` and was refused. So **every remote cache hit failed** inside the batch resolution. On a miss the path is never taken.

Why g1-m10 and not g1-m03/m07/m08: the three others ran `norm-din4108:build` first, all missed (`Cache: 0/7 hit`), ran the task and put the result (a put is a top-level exclusive custody, no outer custody). g1-m10 reached the same task after they had published it to the fleet cache: its first remote retrieve hit, so it was the first machine on the hit path. Parallelism 3 versus 8 only changed timing (m10 reached the task later), it is not a cause. A restore from cache is the common path from the second cycle on, so this wall would have hit every machine.

`Graph readers and namespace release failed`: `nxGraphNamespace` release (`📦️graph/🟦️.ts`) collected the failures (the abort reason of the control = the first failure, plus the child close and lease release that follow an aborted control) and threw `new AggregateError(failures, "Graph readers and namespace release failed")`: Nx prints only `message`, so the first error was masked and no step was named. The same pattern existed in the store close (`Graph namespace acquisition and close failed`) and the cleanup (`Original native graph cleanup failed`).

## 2. Reproduction on this Mac (no workspace read)

The real functions need a native caller envelope (`repo test` harness; plain `bun test` of `📦️outputs/🧪️tests` and `📦️graph/🧪️tests` is red at baseline: 10 of 15 and 40 of 48 fail with `Original parent Script process invocation required` / `Original native caller capabilities required`, same before and after the change, so they prove nothing here). So the decision was extracted into a pure function and the three paths are driven with the real task configuration of `@semio-tech/norm-din4108:build` (output `{projectRoot}/.🧾️store/build.json`, `dist`, classified by the real `classifyNxOutputV1` as partition `receipts`), plus real leases across two real processes:

- miss, run, put: top-level exclusive custody is admitted as is.
- hit, restore: exclusive nested in the shared batch resolution that holds no output (the g1-m10 path). **Red on the old predicate** (verbatim old condition as a stub: 10 of 14 laws fail, including this one), green on the new one.
- concurrent put by another process while restoring: real lease library, resource `artifact:<receipt path>`, two real processes: a put (shared) waits for a restoring writer (exclusive) of the other process and is admitted when it releases; two puts hold together and a restore waits for them.

## 3. Fix

- New member `🔌️nx/🤝️nested-custody` (schema `🧬️schema/🔣️.json`, `🟦️.ts`): `admitNestedNxCustodyV1(held, request)` returns the held scope, or a copy in exclusive mode when a reader that holds **no output, no cohort and no product** asks for a writer (both modes hold the image shared, outputs are leased per output, so it is the same custody a top-level exclusive call takes); every other mismatch is the typed bilingual `NxNestedCustodyRefusedErrorV1` (code `nx-nested-cache-custody-refused`, `rule`, `task` (from `NX_TASK_TARGET_*` when the environment names it, else "unnamed task"/"unbenannte Aufgabe"), `paths`, held and requested mode, held and requested value for the identity rules; EN and DE; `toJSON` validated against the schema). A reader that holds outputs still cannot become a writer (rule `reader-holds-outputs`), the text still contains `nested cache custody` so the existing hook law holds.
- `🔌️nx/🟦️.ts`: the nested branch calls it and runs the nested body in the upgraded scope (`cacheScope.run(held, ...)`) so `requireOriginalNxRustCachePromotionV1` (`scope.mode==="exclusive"`) sees an exclusive scope.
- New member `📦️graph/🧯️release-failure` (schema, `🟦️.ts`): `nxGraphReleaseFailureV1(operation, where, failures)`: one failure is thrown as it is, several become `NxGraphReleaseFailedErrorV1` (extends `AggregateError`, `errors` and `cause` keep the originals in order, message `<title> at <namespace dir>: first failure: <msg>; failed steps: readers (...); funded child (...); namespace lease (...) | <DE>`). Wired at the three `AggregateError` sites of `📦️graph/🟦️.ts` (namespace release, store close, cleanup).
- Not done: the task is only named when Nx puts `NX_TASK_TARGET_*` in the environment (the orchestrator process does not); the output paths always name it. Passing the task id through the Nx patch was not done.

## 4. Proof

Staged in scratch (`R180/stage`, `staged-live`, bun preload substituting staged files, `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`, no cache entry names the scratch path):

- `🤝️nested-custody/🧪️tests/🟦️.ts` 14 pass (10 red on old predicate), `🔒️cross-process/🟦️.ts` 3 pass (real two-process leases), `📦️graph/🧯️release-failure/🧪️tests/🟦️.ts` 5 pass (5 of 5 red on the old aggregate, 200 seeded failure lists against an independent re-rendering).
- Staged graph, outputs and put-slots test files: same pass/fail counts as live (7/40, 5/10, 14/0). `tsc --strict` clean on both new modules and on the call site types. Staged `nx` and `graph` load (7 and 41 exports).

## 5. Landing

Landed 2026-11-18 04:33:58 to 04:33:59 UTC under `landing.lock` (hold 1 s, after the 04:30 window opened on a stale lock of 04:06 that another agent removed; R179 took the lock next). Scripts `🧰️scripts/R180/` `land.sh` (restore by base copy), `patch.py`, `preload.ts`. Under the lock: nested-custody 14/0, cross-process 3/0, release-failure 5/0, live import smoke `7 41 true`. Two inert new directories were created live for about 5 minutes at 04:23 by mistake (before staging), moved to scratch at once.

## 6. Not verified

- The real `withOriginalNxFixedCacheV1` with a real repository (hook law `📦️outputs/🧪️tests` "actual Nx hook ..." and the 40 graph laws need the `repo test` harness).
- A real remote hit end to end: only the next cycles of the workers show it. Record below.

## 7. Outcomes after landing

- 04:35 UTC: g1-m03, g1-m07, g1-m08 `ready: true` (digest 70ffaf34...). g1-m10 `running`, stage workspace, attempts 51, reason `previous-failed` (retry after the 04:04 failure), peak 5120 MB.
- 04:43 UTC: g1-m10 still in the read of the graph slice (`Reading the project graph slice (1106 s): checking that the observed source facts still hold (round 1) 27 of 600`); it has not reached the build phase, so the remote-hit path is NOT yet observed after the fix. Next check: the line `nx run @semio-tech/norm-din4108:build:prototype` and `Cache: n/m hit` in `/Users/Shared/semio-store/v1/origins/g1-m10/ready-log-workspace.txt`; a refusal would now read `Original nested cache custody refused for <task> on <path>: <rule> | Verschachtelter Cache-Gewahrsam verweigert ...`, a release failure `Graph readers and namespace release failed at <dir>: first failure: ...`.
