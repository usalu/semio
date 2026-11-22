# Graph Supersession Drain: What Really Restarts the Slice Loads, and the Fix

Written 2026-11-22. Scope: in-flight selected slice loads (`session-s`, more than an hour) should survive when the build tool's code changes. This covers the host service supersession, the client re-attach and the debounce.

## 1. Findings from the logs: host supersession is not what restarts the loads

The host service already drains. In `🏛️service/🌐️host/🟨️.mjs`, `sweep` calls `drainAndExit("superseded")` only when `connections.size===0 && governor.inFlight===0`. A reload (`🔄️reload/🟨️.mjs`) already starts the replacement first. It waits for a quiet period first (`reload.quietMilliseconds` 90 s, deferral at most 10 min). It then drains, and in-flight plans keep running until `drainStallMilliseconds` passes without progress. I checked every `shutdown superseded` from today against the host log just before it:

| Time (Z) | Host | What ended the load before the shutdown |
| --- | --- | --- |
| 09:07:49 | d2224d988b81 | `request-failed` 7 s earlier: `Build owner repo cannot be inferred ... route-table-invalid` |
| 09:28:29 | 92703e281f91 | `graph: plan answered` |
| 11:18:23 | 1dc159cbadc8 | `SIGINT received` for another agent's load (its client left), then `plan answered` |
| 11:51:14 | 7b29fb93b754 | `graph: plan answered` |
| 12:24:10 | 5954405a68fe | `request-failed` 2 s earlier: `BUILD_OWNER_FAILED` (see `graph-registry-follow.md`) |

In every row the service went idle first and was retired afterwards. The `shutdown superseded` line follows the end of the load; it does not cause it.

The resets inside one attempt are slice worker restarts in the same host. The slice seconds drop to 0 (`session-s-retry.log` line 938, `-attempt-x3.log` lines 1245 and 4400), each time with `the slice worker ended with exit code 75 and said:` followed by one of two causes:

1. `Stale source facts that a repeated inference cannot refresh; restart required: file:.../🕸️graph/📥️load/🟦️.ts` (four times today), `.../🧭️registry-follow/🟦️.ts` (once) and `📋️build-owners.json` (once). Each carries a `pinnedDrift` record (`expectedSha256 == currentSha256 != pinnedSha256`). That is `requirePinnedImplementationFactsV1` in `👁️observation/🟨️.mjs`: the pinned worker saw the current version of an implementation file that it still runs at another digest.
2. `Build graph implementation authority changed during inference; restart the owned build invocation: analysis:.../🔣️taxonomy.json`. That is `conflictingInferenceErrorV1`: a module-load authority row contradicts a later inference row.

So the restarts do happen whenever the build tool's own source changes, as you suspected. They are not done by the service, though. The worker stops on a deliberate, tested contract (`👁️observation/🧪️tests/🔒️loaded-implementation`, `🧪️tests/🕸️graph/🧊️stale-facts`): implementation files are facts of the process, and a graph never mixes the bytes it runs with other bytes of the same file. The pool then started the worker again within 0.25 to 30 s (`sliceRestartBackoffMilliseconds`). That was often while the peer was still editing, so the next drift came soon after.

## 2. Design

I left the restart contract itself in place. Relaxing it (holding a row of the current tree digest under a pin, and allowing load-authority rows to differ) is a change to the graph authority protocol, and I could not check it against a real load of more than an hour. Section 5 explains this. The fix makes every boundary around the restart drain, hand over and debounce correctly.

### 2.1 Quiet gate for worker restarts (the debounce that matters)

New module `🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🔕️quiet/🟨️.mjs`:

- `implementationDriftPathsV1(cause)` reads the changed implementation paths from the worker's stderr tail. It understands both message forms and the `pinnedDrift` JSON, and returns nothing for any other cause.
- `implementationStampsV1(paths)` gives the `ctimeMs` of each path. This machine's kernel sets it for every write, rename and replica copy. A removed path counts as quiet.
- `implementationQuietV1({stamps,nowMs,quietMilliseconds})` is the pure verdict: the newest change decides, and a stamp in the future counts as a change now.
- `awaitImplementationQuietV1` waits, announces each check and can be cancelled.

The pool (`👷️slice-pool/🟨️.mjs`) records `drift` in the exit error, collects the paths across the restarts of one load and releases the worker slot. Before the backoff pause it waits until that code has been unchanged for `sliceRestartQuietMilliseconds` (60 s), at most `sliceRestartQuietMaximumMilliseconds` (10 min). The progress line `host-slice-owners-settling` (English and German) names the file, how long it has been quiet so far, the quiet period and the bound. After a wait the replica churn window (`churnSince`) starts again, because the code was seen settled. A burst of edits now restarts a load once. The new worker never loads a module while a peer is still writing it.

### 2.2 Drain with a bound for a superseded service (a)

`supersededDrainDueV1` (exported, pure) replaces the inline condition. A superseded service ends after the grace in two cases. If it is idle, it ends as before. If it has plans in flight, it ends only when none of them has seen original work for `reload.drainStallMilliseconds` (30 min), measured from the later of the supersession and the last progress. A progressing load always drains to its answer on the code it started with, and a stalled one is handed over. An open connection without a plan (a run that uses the service's cache endpoint) still keeps the service.

### 2.3 Hand-over and transparent re-attach

When the service ends for any reason except missing file access (reload stall cut, superseded stall, `stop`, `memory`, SIGTERM), `drainAndExit` used to abort each running plan with a bare `Error("shutdown")` and destroy the socket at the same moment. The client got `shutdown` with retry `never`, or nothing at all. Now:

- Each running plan is aborted with `{code:"handover",reason}`. `runPlan` answers it with the existing refusal code `handover`, the new key `host-service-handover` (English and German, naming the reason) and retry `later`.
- The sockets are destroyed only after these refusals are flushed, bounded by the new `handoverFlushMilliseconds` (2 s).
- The client path for `handover` already existed in `requestInvocationPlanV1` (`🧭️invocation/🛰️plan/🟦️.ts`, `askHanded`, bounded by `CUSTODY_POLICY_V1.maximumHandovers`). It asks again with `identify`, so it reaches whichever service answers now, including a successor on newer code (`connectHostV1` candidates). The new job keeps the same job key and continues from the persisted owner checkpoints (`💾️owner-checkpoint`) and the admission memo. Progress lines keep reaching the person through the same `write`. No new protocol value was needed.

### 2.4 Start locks and identity (read, unchanged)

`start-*.lock` (`runtime.lock`, `tryStartupOwnerV1`) makes one client the starter per implementation. The service identity comes from `serviceRevisionFiles`, and the new quiet module was added to that list.

## 3. Files changed

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🔕️quiet/🟨️.mjs` (new)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🔕️quiet/🟨️.d.mts` (new)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`: drift in the exit error, quiet gate before the restart, new seams `quietPolicy`, `implementationStamps`, `wallNow`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏛️service/🌐️host/🟨️.mjs`: `supersededDrainDueV1`, the `handover` refusal, flushed hand-over in `drainAndExit` (with docstring)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏛️service/🌐️host/🟨️.d.mts`: `supersededDrainDueV1`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧬️schema/🔣️.json`: `sliceRestartQuietMilliseconds`, `sliceRestartQuietMaximumMilliseconds`, `handoverFlushMilliseconds`, `serviceRevisionFiles` entry, and the messages `host-service-handover` and `host-slice-owners-settling` in English and German
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📨️protocol/🎚️policy/🟨️.d.mts`: the three policy keys
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🔕️restart-quiet/🟦️.ts` and `🔣️.json` (new)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🔀️supersession-drain/🟦️.ts` and `🔣️.json` (new)

## 4. Tests and verification

Run directly with the repo-managed bun from `🏗️build`:

- `🔕️restart-quiet`: 6 pass. The case table for the quiet verdict is checked against a SQLite oracle (`ORDER BY at DESC, path`, `MAX(0, now-at)`). Drift paths are read from real log causes. A burst of three edits gives one restart, at least the quiet period after the last edit. Endless churn waits only for the maximum. Another cause, or code that is already quiet, restarts at once with no settling line. Cancelling during the wait ends the plan at once and starts no worker. Both languages render the new message with no placeholder left over.
- `🔀️supersession-drain`: 5 pass. The case table for the drain decision is checked against a SQLite oracle. A superseded service keeps a progressing load for more than 60 s, answers it, then retires as `superseded`. A stalled superseded load reaches its client as `handover`/`later` in both languages. `stop` while a load runs gives `handover` instead of `shutdown`. End to end through `requestInvocationPlanV1`: the first service reloads mid-load, the client asks again, the second service answers, and progress lines from both services reach `write` on either side of the hand-over line.
- Neighbours still green: `🔁️restart` 8, `🗣️restart-cause` 13, `🤝️slice-share` 5, `⏳️queue-progress` 5, `🔖️revision` 3, `🤝️handover` 2, `⏳️load-outlives-idle` 3, `🤝️late-attach` 1, `💓️keepalive` 8, `🛑️cancel` 37, `🧯️failure` 20, `🚪️client` 8, `🔄️reload` 12, `🏷️worker-error` 2, `🔒️lease-wait` 3, `🩺️status-deadline` 8, `🚫️doomed-handoff` 2, `🌐️host` 39. None fail.

Not verified: a live `session-s` load. The running service (`host-0003b7003034`) runs the old code. Because `serviceRevisionFiles` changed, the reload will replace it after its quiet period, and with this change its in-flight load drains rather than being cut.

## 5. What is still open, and why

The quiet gate stops the restart storm, but one restart per settled edit of a file in the graph implementation set (`graphImplementationUrlsV1`, mainly `🕸️graph/📥️load/🟦️.ts`) remains. That file changes about every 30 to 60 minutes today. A load that needs an hour therefore finishes only if:

1. a restarted load re-verifies cheaply. The checkpoint and memo resume exists, but the restarted segments of today ran about as fast as the first, because the owner checks re-verify about 2 M memo rows on one core. `graph-incremental-load.md` names this cost and says it has not been profiled. It needs a profile of the worker (stderr is `/dev/null` today) before it can be fixed. Or
2. the pinned-drift contract changes. A pinned worker would finish on its loaded code while holding rows that name the current tree, and load-authority rows of implementation files would no longer contradict inference rows. This keeps correctness for the selected path: readers gate reuse on the module authority (`buildGraphProducerAuthorityV1`), and owner records are keyed by the admission format only. However, it contradicts the documented contract in `🔒️loaded-implementation` and `🧊️stale-facts`, and the generation path names loaded bytes in every generation. It is a graph authority protocol decision for the owners of `👁️observation` and `🕸️graph/📥️load`, and needs its own ticket and a real end-to-end load to verify.
