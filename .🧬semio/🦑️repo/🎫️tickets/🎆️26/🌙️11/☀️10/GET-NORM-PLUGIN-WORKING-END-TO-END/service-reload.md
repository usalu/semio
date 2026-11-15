# Service host reload on newer code

## What existed

- **Resident graph host** (`build/🏡️resident`): its identity (`hostImplementationV1`) is the sealed door digest, else a hash of root, seal and `HostPolicy.serviceRevisionFiles` (a hand-listed set of files). The socket is per identity, so a client on a new identity starts a new host and the old one retires through `newerSibling()` (idle, no connections, 60 s grace). This is client-driven only: nothing noticed drift on a host nobody asked, and nothing drained.
- **Sidecars** (`remote-execution/🛰️sidecar`, including the **fleet graph server** `🕸️graph`): already self-reload (`createSidecarRestartV1`: closure digest, 90 s quiet or 10 min maximum, drain, exit 75, launcher restarts them). The `serveTotalMegabytes` fix lives there, not in the resident host, so it reaches machines through the sidecar restart (provided the machine's replica daemon is up and launches the graph sidecar), not through `repo service host stop`.
- **Replica supervisor / `boot.mjs`**: exit 0 and 75 end the boot loop and the service manager restarts it; it picks code order (worktree, sealed snapshot, store). It has no hook for the resident host.
- launchd job of the host: `KeepAlive=false`; clients start it. So nobody restarts a host that exits.

## What was added

`createHostReloadV1` (`🏡️resident/🔄️reload/🟨️.mjs`), ticked by the host every `reload.checkMilliseconds` (30 s):

1. `identify` says what the tree wants: `{implementation}` or `{implementation, blocked}`.
   - Sealed host (door digest in its environment): the sealed store pointer, read back intact (snapshot hash, loader hash, receipt). The pointer only moves after the promoter parsed, booted and self-checked the snapshot; a damaged store is `blocked`. The host never promotes (the `explicit` rule stays: landing gate or aged snapshot).
   - Live host: recomputed identity; a changed one counts only when every file named in `serviceRevisionFiles` still parses (`syntaxRowsOfPathsV1`), else `blocked`.
2. Debounce reuses the sidecar rule (`createSidecarRestartDebounceV1`): quiet 90 s, maximum deferral 10 min, a reverted edit cancels.
3. Due: `replaceHostServiceV1` starts `repo service host start` through the front door (so through the seal; `SEMIO_SEALED_*` stripped so the new door is resolved, not the old pin), waits at most 300 s, and requires exit 0 and a service answering on the wanted identity.
4. Only then the host drains: `drainAndExit("reload")` stops accepting, refuses new plans, lets every in-flight plan (and the capacity grants it holds) run to completion, cutting one only after `reload.drainStallMilliseconds` (30 min) without a progress frame, then ends with reason `reload` (exit 0).
5. Broken seal, failed identification or a replacement that does not start keep the old host serving. The state (`current|pending|draining|blocked|failed`, next digest, reason) is in the status frame (`ReloadStatus` in the schema) and printed bilingually by `repo service host status`. A failed replacement is retried after `retryMilliseconds` (300 s).

Policy: `HostPolicy.reload` in `🏡️resident/🧬️schema/🔣️.json`. Pure Node and Bun APIs, no platform branches (spawn, SIGTERM, sockets or pipes as before).

## Tests (`🏡️resident/🧪️tests/🔄️reload`)

- Shared table `🧫️cases.json` replayed by the service rule and by an independent Python model (`🐍️oracle.py`): same implementation, quiet then replace and drain, edit storm to maximum deferral, reverted edit, broken seal, failed replacement with retry delay.
- Service: drift detected, replacement started, plan in flight completes, drain, closed with `reload`, endpoint gone; blocked seal keeps answering and reports; failed replacement keeps answering, tried once.
- Replacement process: exit 0 plus answering service, wrong or missing answer, nonzero exit with output tail, timeout kills the child.
- Identity: real sealed store from the fixture promoter (new pointer followed; missing receipt, damaged snapshot, no pointer are blocked), live revision with and without syntax rows.

Run: `repo tools exec bun -- test ./…/🏡️resident/🧪️tests/🔄️reload/🟦️.ts`: 9 pass. Existing suites `🔖️revision`, `🌐️host`, `📨️protocol`, `⌨️command`, `🚫️retired`: all pass except three tests that depend on this machine (load 70+, launchd route for the "separate process" spawn test at 5 s, a 62 MiB partition message); none touches the changed code paths (spawn route untouched). `raw-process-spawn` law: no finding for the new files.

## How the other machines pick it up

- Hosts that run the old code cannot drain themselves. They are replaced by the existing identity rule: this change edits files in `serviceRevisionFiles` (host service, composition, schema) and adds three, so the identity moves on every live-mode machine. The next planned `repo` command there (any agent) starts the new host; the old one has a newer sibling and retires after 60 s without connections or in-flight plans (or at the 30 min idle shutdown). No manual restart needed unless a client holds a connection open for ever; fallback is `repo service host stop`.
- Sealed machines (`SEMIO_REPO_SEAL=sealed`): the identity moves only with a promotion (landing gate `SEMIO_REPO_SEAL=promote repo`, or the 24 h aged rule). After it the same happens.
- From the first new-code host on, later code landings reload by themselves within about 90 s to 10 min plus drain, with no client needed.
- The fleet graph server threshold needs no host restart: the sidecar restarts itself on its closure change. If a machine still does not serve, check its replica daemon (`repo fleet status` code column), not the resident host.

## Changed files

- 🏡️resident/🔄️reload/🟨️.mjs, 🟨️.d.mts (new)
- 🧭️invocation/🔄️reload/🟦️.ts (new); 🧭️invocation/🛰️plan/🟦️.ts (`hostImplementationV1` takes the revision as a seam)
- 🏡️resident/🏛️service/🌐️host/🟨️.mjs, 🟨️.d.mts (reload port, timer, drain bound, status field)
- 🏡️resident/🏗️composition/🌐️host/🟨️.mjs, 🟨️.d.mts; 🏡️resident/🟨️.mjs, 🟨️.d.mts (exports)
- 🏡️resident/⌨️command/🌐️host/🟨️.mjs (status line); ⌨️command/🌐️host/🟦️.ts (wiring)
- 🏡️resident/🧬️schema/🔣️.json (`HostPolicy.reload`, `ReloadStatus`, bilingual texts, revision files); 🏡️resident/📨️protocol/🎚️policy/🟨️.d.mts
- 🏡️resident/🧪️tests/🔄️reload/🟦️.ts, 🧫️cases.json, 🐍️oracle.py (new)
All paths are below 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/.

## Drain fix (after describe15 regression)

A fixed 300 s drain cut a 30 min slice read and the client re-issued it. Now the drain bound is progress, not time (`lastProgressAt` is stamped on every progress frame). The plan stays on its connection to the old host, so the client never re-issues it on the new identity (test: computed once, replacement asked once). Edits that cannot affect the plan are not filtered (kept simple). Staged and tested outside the tree; land with `python3 .../🗑️generated/reload-fix/apply.py <root>` (edits service, schema, command, two d.mts, replaces the reload test with `reload-test.ts`).
