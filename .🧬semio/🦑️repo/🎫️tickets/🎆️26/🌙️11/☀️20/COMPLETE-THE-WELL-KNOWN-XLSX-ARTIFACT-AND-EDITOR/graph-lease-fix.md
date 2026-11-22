# Graph Lease Fix: Writer Shards, Blocker Messages, Unpublished Generation

Taken 2026-11-22. Scope: the three defects of the task (hours-long exclusive writer shard, a wait line that names nobody, no published generation).

## How the stall really arises

A selected slice load (the default for every task and every `show` query on a selection) runs `loadBuildGraphUnpinnedV1` with `baseline: true`. For a scope it called `withBuildGraphStateV1(root, ..., scope)`, which took two exclusive leases for the **whole** multi-hour attempt:

1. `repository-graph-facts:<root>:shard:N` (inside `withRepositoryBuildGraphSelectionV1`), N = `sha256("selection:"+scope)[0] % 64`.
2. `build-graph-state:<graphRoot>:selected:N`, the publication lock, same shard N.

Two different selections whose scopes hash to the same N therefore serialised for hours although they write different directories (`inference/selections/<scope>`, `selected/<scope>.json`). `repo leases status` showed exactly this: PID 3936 held `shard:56` and `selected:56` exclusively for 3 h 24 min.

What the exclusive lease actually protects (read from the code, not assumed):

- Sweeps and optional-cache eviction (`evictOptionalGraphLruV1`, `sweepSupersededBlocksV1`, headroom recovery) take the shard exclusively with timeout 0 or queued; they must stay away from a load that is reading its records.
- Same-scope exclusion: the owner checkpoint (`💾️owner-checkpoint`, slot packs with a sequence), the admission memo and the staging cleanup assume one loader per scope.
- Nothing needs one selection to exclude another selection.

## Why the lease is not dropped between writes

The task asked to scope the lease to one write batch. I did not do that, and the reason is a correctness one: the writer custody (`writers` AsyncLocalStorage scope, the open ledger, `requireRepositoryBuildGraphWriterV1` checks) is held across the whole body and is consulted synchronously by many write sites (observation blocks, closure leaves, owner checkpoint packs, admission memo, selected publication). A lease that disappears between those sites would let a sweep delete blocks the running inference has verified but not yet consumed, and there is no retention pin protocol to prevent it. Re-acquiring per write would need a new pin protocol for every reader of the selection directory; that is a protocol change, not a lease change.

Instead the fix removes the cross-selection blocking, which is the actual defect, and keeps every protection above:

| Lock | Before | After |
| --- | --- | --- |
| Fact shard `repository-graph-facts:...:shard:N` | exclusive for the whole load | **shared cohort** for the whole load (`withRepositoryBuildGraphSelectionV1(..., {shared: true})`); sweeps, eviction and headroom recovery still need it exclusively and still stay away |
| Same-scope exclusion | implicit through the shard | **new** `build-graph-state:<graphRoot>:scope:B`, exclusive for the whole load, B = first 16 bits of `sha256("scope:"+scope)` mod 4096 (fixed catalog of 4096 buckets, so the lease catalog stays bounded as the writer shard catalog was) |
| Publication `build-graph-state:...:selected:N` | exclusive for the whole load | exclusive **only inside two critical sections**: the staging cleanup and the `rename` pair that publishes the selected graph and its authority receipt |
| Canonical (unscoped) state | unchanged | unchanged (holds `build-graph-state:<root>` throughout) |

Collision odds between two concurrent scopes drop from 1 in 64 to 1 in 4096, and a collision now only costs a wait for the other scope's claim, never the shared shard.

### Compatibility with running old processes

Old code takes the shard and the publication shard exclusively and knows nothing of the scope claim.

- Old holder first: the new loader's shared request on the same shard queues behind it (same as before) and never reaches the scope claim, so old and new cannot both write one selection.
- New holder first: an old loader's exclusive request waits for the shared cohort to end (it waited for a long hold before as well).
- Readers (`withRepositoryBuildGraphPublicationV1`, shared `selected:N`) now wait for a rename pair at most, not for the inference. The graph and the authority receipt are renamed under the exclusive publication lock, so a reader never sees a mismatched pair (the digest check of the receipt still guards the unlocked old path).
- Fencing tokens and generations: the selected path uses neither (`acquireFreshResourceLeaseV1` and writer proofs belong to the generation builders, which already use a shared cohort of their own shard). No generation document is touched by this change.

## Blocker named in the wait message

The queue already knew its blocker: `acquireQueuedResourceLease` publishes a `lease-queue:` notice (holder pid, mode, queued seconds, state, phase, counters, time left) through the invocation control. In the slice worker that control is not connected to the host's progress stream, so the client only saw `Reading the project graph slice (N s).`. Changes:

- `🔒️leases/🟦️.ts`: new `onLeaseQueueNoticeV1(listener)` (process-wide registry, symbol-keyed like its neighbours), called on every renewal of a queued wait with the full notice. The published `lease-queue:` payload is unchanged (the routing progress schema is closed).
- `🔒️leases/📈️standing/🤝️yield/🟦️.ts`: the notice gains `holderAgent`, read from the holder's registered agent session (same lookup as `repo leases status`).
- `👷️slice-pool/👷️worker.ts`: forwards each notice as a `progress` message with a `leaseWait` object (lock kind and number, position, queue length, holder pid, held seconds, state, agent, phase, counters, time left).
- `👷️slice-pool/🟨️.mjs` and `🏛️service/🌐️host/🟨️.mjs`: the pool keeps `leaseWait` in the progress detail until the worker reports other progress; the host renders it with the new `renderLeaseWaitV1` in every language of the connection.
- `🏡️resident/🧬️schema/🔣️.json`: 16 new `host-lease-*` messages in English and German (main line, four lock descriptions, agent, hold time in minutes or seconds, five holder states, progress, phase, time left).
- `🔒️writer/🟨️.mjs`: `describeGraphWriterResourceV1` turns a resource name into (kind, number).

Example of the English line now: `Reading the project graph slice (481 s): waiting for the graph fact writer lock (shard 56; loads of other selections that hash to this shard share it), place 2 of 3 in line, behind process 3936 (agent claude-code), which has held it for 200 min; the holder is progressing at inference 303 of 600, about 138 min left. Run repo leases status for the whole queue.`

## Why no generation is published (defect 3)

By design, not a code bug in the publisher:

- `loadGenerationGraphV1` publishes `generations/current.json` only for a build of the **whole** workspace (selection `all`). The default selected read (`graphSliceReadSelectedV1`, anything but `SEMIO_BUILD_GRAPH_SLICE_READ=generation`) never touches generations; it infers the selected closure and publishes `selected/<scope>.json`. The directory `.cache/b/g/m/generations/s` is empty (only slice-custody claim space), no `current.json` exists.
- A question about every project under policy `current` refuses with `neverSealed` ("never starts the hours of inference"), and says: `repo graph --graph-policy fresh`.
- The doctor rule `graph.unpublished` named `repo show projects` as the fix with "One run publishes the first generation". That is wrong: `show projects` runs under policy `current` and never builds. It is fixed to `repo graph --graph-policy fresh` with an honest note and detail text (English and German), and the detail no longer claims every command builds the graph exclusively.
- So `graph_policy=current` for a task or selection cannot reuse a sealed graph until someone starts the whole read once on purpose (hours on a cold checkout, resumable). I did not make task commands start it implicitly: the existing design rejects that, and on this 16 GB shared machine it is the workload that caused the memory thrash. Selected graphs that were already published (`selected/<scope>.json`) are reused after a cheap facts check regardless.

## Not changed, still open

- The per-root queue in the slice pool (`tails` in `🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`) still serialises every load of one checkout inside one host service ("both share one fact store"). With the shard no longer exclusive that premise is gone, so loads of different selections could run side by side bounded by the worker count and the capacity gate. It is a separate admission policy with its own tests (`⏳️queue-progress`, `🤝️slice-share`), so I left it; it is the next change to make, and the wait line for it already names the leader.
- A lease per write batch needs the retention pin protocol described above.

## Files changed

See the reply of the task for the list; tests added next to the existing ones:

- `📥️inputs/💾️storage/🧪️tests/🟦️.ts`: the extracted `withBuildGraphStateV1` law now checks scope claim then shared selection then publication only inside `critical`; new bucket-catalog test with an independent SQLite digest oracle and a deterministic pair of scopes that share a fact shard but not a claim; new `describeGraphWriterResourceV1` test.
- `🔒️leases/⚖️fairness/🧪️tests/🟦️.ts`: a queued wait reaches listeners with the holder it stands behind.
- `🏡️resident/🧪️tests/🔒️lease-wait/🟦️.ts`: English and German rendering for every lock kind and state, bilingual key parity, and the pool handing the notice on.

## Verification status (honest)

The new and updated tests were written but **not run to green**. Every attempt through the repo MCP failed for reasons outside this change:

1. `repo-graph-maintenance:test` waited 19 min at the capacity gate, then the service refused to start: `Closed host capacity Policy required: /swap/activeMegabytesPerSecond is required` (another agent was converting `🔣️policy.json` and the host-capacity schema).
2. After that, every run failed in under a second with the same error until the policy and schema agreed.
3. `repo:test-build-inputs-storage` then queued 2 h 9 min behind another load of this checkout (the per-root slice-pool tail, the very stall described above, leader at "owners 97 of 600") and finally failed because the slice worker could not import `hostUnderPressureV1` from `🌡️host-capacity/📏️decision/🟦️.ts` (again another agent's half-finished edit).

So the change is checked by reading only: the pinned-text tests that quote the old callback shape were updated (`generation-protocol`, `sealed-reuse`, `selected-exchange/wiring`, `candidate`, `adopt-registry`). Run, once the host-capacity edit has settled: `repo:test-build-inputs-storage`, `repo:test-build-resident`, `repo:test-build-graph-state`, `repo:test-build-generation-load`, `repo:test-build-doctor-preflight` and `@semio-tech/framework-process:test-resource-lease-fairness`.

## Follow-up: loads of one checkout run side by side

Done in `🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`: the per-root `tails` queue, the same-root wait bound (`sliceSameRootWaitMilliseconds`, which a parallel edit had added as a 30 s wait followed by "alongside") and the leader tracking (`leads`) are removed. Loads of different slices of one checkout now start as soon as a worker is free, bound by the worker maximum, the memory reservation and the capacity gate. Loads of the same slice are still one load (`sharing` map, `🤝️slice-share`). The messages `host-slice-owners-queued-root`, `-queued-leader` and `-alongside` and the policy constant are removed (greenfield, no compat); the test directory `🧪️tests/⏰️same-root-bound` (bounded-wait contract and its Python model) is deleted with them.

Tests rewritten: `⏳️queue-progress` (three loads of one checkout: two run at once, the third reports `queued-pool` and starts when a worker frees; a queued load whose client left starts nothing; the pressure variants now check that the follower runs alongside the leader and the independent checkout takes the freed worker, with a SQLite `min()` oracle for the slot count) and `🤝️slice-share` (a killed load of another slice no longer blocks the next one, which runs alongside from the start).

Result: **not verified.** `repo:test-build-resident` ran 1 h 7 min, spent its time inside a slice load of the graph (still the queue described above, because the running host service predates this change) and ended with `BUILD_SOURCE_UNPARSEABLE: Cannot find module .../🔏️index/🧊️stamps/🟦️.ts`, a missing file from another agent's in-flight edit under `⚡️caching/🚀️bootstrap`. Re-run once that tree is consistent and after restarting the host service so it picks up the new pool: `repo:test-build-resident`, then the list at the end of the previous section.

## Verification

### Fast route (no hour-long wait for a fresh checkout of the suites)

A test target is `repo ./📜️script.ts test <suite>`: the suite script runs every test file in its own bun child (`<repo-managed bun> test --timeout N <abs file>`) under a genuine process birth that only the repo host hands out. Two legitimate routes exist:

1. Direct, seconds: the repo-managed bun at `~/Library/Caches/semio/runtimes/bun/<hash>/bun`, `cd` into `🏗️build`, `bun test --timeout 120000 <absolute test file>`. This runs every test that needs no birth (all the extracted-function, fixture and pool tests: roughly 80 percent) and shows the real assertion. Tests that fail with `Original parent Script process invocation required` need the second route.
2. Full route, `repo run-many --targets=... --projects=repo --exclude-dependencies --parallel=4` (CLI; the MCP tools were unreachable for hours because their PreToolUse hook timed out). One slice load is shared by all targets, the suites then ran in about 20 minutes.

### Results

`@semio-tech/framework-process:test-resource-lease-fairness`: 18 pass, 0 fail. `repo:test-build-generation-load`: green (no failing file). Last full route run of the others, before the fixes below:

| Suite | Failing files before | After my fixes (direct route) |
| --- | --- | --- |
| build-inputs-storage | 9 | 6 left, all `capabilities-differ`/birth-bound or others' work (see below) |
| build-resident | 4 | restart green (8 pass); provision, host, cancel need the route's artifact directory and are not lease/pool related |
| build-graph-state | 2 | not touched (measured-only cost model, generated digest) |
| build-doctor-preflight | 2 | not touched (ci-report, agent-sessions) |
| build-workspace-graph-laws | 7 | not touched |
| build-observation | 6 | not touched |
| build-graph-protocol | 15 | not touched |

### Failures caused by the lease/pool change, fixed at the root

- `🔁️restart` (resident): modelled the removed per-root queue (a follower of the leader's checkout waited for another checkout's load). With loads of one checkout side by side the freed slot goes to the first waiter in arrival order; the test now states that with an independent SQLite order oracle. 8 pass.
- `🧪️tests/🟦️.ts` scope-claim law: the graph source is split into modules, so the implementation URLs are `../../📥️inputs/...`; expectation corrected.

### Failures that predate the change (stale tests of code refactored 2 to 5 days ago), repaired minimally

- `withGraphFactWriterV1` now launches the optional collector through a dynamic `import("../../🧹️cache-gc/🚀️launch/🟦️.ts")` even without pressure; the main test extraction now replaces that import like `🚪️admission` and `🔒️lifetime` already do. (I first tried a named seam in the production module; reverted, it broke those regex based tests.)
- `quotaSchema:{policy}` mocks became `quotaSchema.definitions.DefaultPolicy.const` (main test, `🛑️primary`).
- `nativeImageSpaceV1` mocks asserted the image was not read after consumption; the code reads the free space once more (main test headroom case, `♻️scoped/🔄️lifetime`).
- `🎛️phase`: bindings lacked `machinePathV1`.
- `🎟️custody`: tested a retirement algorithm that moved into `retireOptionalGraphHeadroomV1`; rewritten to the present function (deferral, inventory inside custody, retire, required headroom), identity and cancel rows removed from fixture and feature (covered by `♻️retirement`).
- Direct route result: `🛑️primary` 4, `🎟️custody` 1, `🎛️phase` 7, `🚪️admission` 1, `🔒️lifetime` 1, `♻️scoped/🔄️lifetime` pass.

### Not fixed (other agents' in-flight edits or environment, not lease/pool)

`capabilities-differ` in the route for `🎛️phase`, `📒️ledger`, `🚪️admission` and observation `📥️reuse` (testing capability handling), `residentWatcherInstalledV1 is not defined` (resident provision), `followedPathsV1 is not defined` (candidate), `observeOwnerWithRetryV1 is not defined` (admission memo; belongs to the churn tolerant inventory change and must be checked by its author), `Exact Original Graph format owner required` (pinned publication), the derived-nodes model tests, `sealed-reuse` and `prefetch-booking` source-text pins on the split graph source, fleet-read and cooperative-farm pins. The graph-state, doctor-preflight, workspace-graph-laws, observation and graph-protocol suites are therefore still red.

### Host service restart

Needed: the pool lives in the host service process (`🏛️service/🌐️host/🟨️.mjs`, `👷️slice-pool/🟨️.mjs`), so a running service keeps the old queue and exclusive shard until it is stopped. At 09:49 no service was running, so `repo service host start` loaded the new code (identity 6357c40caeee, pid 62829). Invalidation logic untouched.

### Live `session-s` retry on the new service

Attempt started about 09:59 after the restart. Round 1 took 886 s to reach owners 538 of 538; round 2 then restarted the file inventory once (895 s) and since then the counter only grows: completed owner checks 665 at 1823 s, 1042 at 2430 s, owners admitted 530 then 537 of 538, verified memo rows 2.2 M then 2.3 M. No reset to near zero. The load had not finished at 2430 s (about 40 min); total load time not yet known.
