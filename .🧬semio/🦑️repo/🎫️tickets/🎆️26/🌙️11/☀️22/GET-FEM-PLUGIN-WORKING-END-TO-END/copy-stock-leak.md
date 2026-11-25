# Copy-Stock Exhaustion Wedge (FEM 2D, `plugin.original-stock-exhausted`)

## Symptom
`plugin.original-stock-exhausted: maximumCopyBytes has 134213695 of its floor 134217728` (bridge `requireStock`). After that, selection hangs, Results stays Pending, and tree windows lose rows.

## What the numbers mean
- The actor's operation grant is 2 GiB on every axis (`🧰️framework/🔨️modules/🖼️canvas/🧵️renderer-work/🔣️stock.json`, `operation.grant`).
- Policy (`🎭️actor/🎟️retained-turn/🔁️succession/📋️policy.json`): low water = grant/4 = 512 MiB, floor = grant/16 = 128 MiB.
- So the actor had spent about 1.875 GiB of copy bytes on one tenure. The 4033-byte gap is just the last turn that crossed the floor.
- **The floor semantics are correct.** The floor is the reserve that keeps cleanup and succession funded, and the check compares what remains against it. There is no outstanding loan in that comparison.
- **There is no debit/credit leak.** Copy bytes are consumed and never returned, by design. They are renewed only by **succession**: the kernel issues a fresh operation, and `OriginalStockSuccession.advance` hands it over once the source is below low water and the actor is quiescent.

## Evidence that succession itself works
I ran an instrumented browser run (`stock-driver.mjs`, which patches the served `🌉️bridge.js` with `[DEBUG] stock` logs) on 6086. It booted, did one edit, a series of undo/redo steps, then about 9 minutes of selection and Artifact-tree churn. The stock reached low water while the actor was idle and was renewed right away:
```
[DEBUG] stock low copy=536868143 tenure=1 outstanding=false held=0 jobs= wait=0 deferral=1
[DEBUG] stock renewed tenure=2
```
The drain is real consumption (about 1.5 GiB within ~25 min of use). Renewal only fails when the **host's job mirror** claims a job is still owned (`advance()` returns while `#jobs.size>0`), or when the guest really still owns an orphaned job (`has_owned_jobs` makes it refuse with `ownership-limit`). Once that happens, the stock drains down to the floor and every non-cleanup crossing is refused for good.

## Root causes (two host paths leave a job owned forever)
1. **Component bridge registers a job before the guest admits it.**
   - Where: `🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts`, `startJob`. It called `succession.started(job)` *before* `funds.start(...)`.
   - Effect: a start the guest refuses, or one that throws, left the job in `#jobs` forever, so succession was never attempted again.
   - The existing law `a job the guest refuses to start does not hold back the renewal` (`🧪️tests/🔁️succession/🟦️.ts`) **was failing**, with the exact browser message:
     ```
     error: plugin.original-stock-exhausted: maximumCopyBytes has 1043356 of its floor 1048576
     (fail) a job the guest refuses to start does not hold back the renewal
      2 pass / 1 fail
     ```
2. **The React plugin runtime abandons a running job without cancelling it.**
   - Where: `📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts`, `driveSpawnedJob` and `commitReservedToolJobWhileSerialized`.
   - Effect: when a step threw (for example the stock-exhausted refusal itself), or the instance closed mid-drive, `finally` only did `runningJobs.delete(key)`. The guest kept its job slot, which nobody would ever step, cancel or close again. The guest therefore never quiesced and refused every succession. The parked `spawn_job` request was never answered either, which is the "Results Pending forever".
   - In the observed run, one solve job ran for about 5 minutes (`…964`, started 919894, settled 213411). If low water is crossed during such a job, consumption reaches the floor before quiescence. The failing `stepJob` then orphaned the job, and the wedge became permanent.
   - The shared wire-turn driver (`🎭️actor/🖼️wire-turn/🧵️spawned-job`) already cancels any unsettled job in `finally`. The React drive diverged from it.

## Fix
- **Bridge:** `startJob` now registers the job with succession only after `funds.start` returns admitted: `await funds.start(...); succession.started(job);`.
- **React runtime:**
  - `cancelOwnedJob` is the serialized cancel. It runs only while the host still records the job as running, and clears that record after the cancel succeeds.
  - `releaseSpawnedJob` in both drive `finally` blocks cancels any job the drive still owns. While the whole runtime is disposing, it only drops the record.
  - When a drive fails, its `catch` cancels the owned job and delivers a `job-completed` fault (`plugin.job-drive-failed`), so the guest's parked request resolves.
  - The cancel crossing is a cleanup crossing (`cleanup=true`), so it is still funded on the floor. After it the actor is quiescent, and the next crossing renews the stock.
- No Rust or guest changes. The guest's quiescence rule (no succession while it owns a job) is kept.

## Laws
- Existing bridge law, now green: `🔌️plugin/🌐️browser-bundle/🏗️materialization/🧪️tests/🔁️succession/🟦️.ts`, "a job the guest refuses to start does not hold back the renewal". It runs the generated worker, generated bridge and real kernel stock.
- New renderer law: `📺️renderer/🧑‍🎨engine/🧪️tests/📬️extension-invocation-wire/✅️completion-and-job-slices/🟦️.tsx`, "cancels the job a failed drive still owns and answers its parked request, so the guest quiesces and renews its stock". It covers a failure on step 1 and on step 3, and expects exactly one cancel plus one `job-completed` fault. With the old `finally` it fails (`Tests 1 failed`).

## Test output
```
bun test 🧪️tests/🔁️succession (bridge)       3 pass 0 fail   (before fix: 2 pass 1 fail)
bun test 🧪️tests/🏛️bootstrap                    3 pass 0 fail
bun test 🧪️tests/🚪️lifetime                     2 pass 0 fail
bun test 🎭️actor/🎟️retained-turn/🔁️succession   12 pass 0 fail
vitest plugin-runtime-suite (renderer-job-release-law.sh ".")   Tests 145 passed (145)
vitest new law with old finally                                 Tests 1 failed
```
No Rust was changed, so no cargo run was needed.

## Not done / follow-ups
- The served `.cache/b/p/m/browser/prototype/modules/🏗️fem/🌉️bridge.js` is generated. It needs rematerialization (`native-fem-materialize.ts` or `framework-plugin-web:support`) to carry the bridge fix. The renderer change is picked up by vite on reload.
- I did not re-verify in the browser after the fix. The wedge needs low water during a long job, which this run did not hit.
- Copy consumption is large (about 1.5 GiB in ~25 min, and a 5-minute solve job). That is a performance question, separate from this wedge.
- F2 (Redo via Ctrl+Shift+Z without effect) also reproduced in this run with ample stock, so it is not caused by exhaustion.
- 3D `escrow table has no free slot` and the ordered-map retire panic are handled by the other agent (`escrow-slot-leak.md`). I edited no shared lines.

## Files
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts` (updated)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts` (updated)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📬️extension-invocation-wire/✅️completion-and-job-slices/🟦️.tsx` (new law)
- Ticket inputs: `stock-driver.mjs` (instrumented page driver), `renderer-job-release-law.sh` (native vitest runner with explicit policy)
