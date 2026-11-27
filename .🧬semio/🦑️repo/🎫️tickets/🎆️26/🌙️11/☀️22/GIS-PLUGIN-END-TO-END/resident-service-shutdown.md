# Resident Service Shutdown (`shutdown cancelled`)

## Log evidence

Log: `/var/folders/xq/z6t8hmcd0qj871bx0sj7pbn80000gn/T/semio-repo/501/host-2bf1553db5e2.log`, exit rows in `.../501/exit.log`.

- 2026-11-26T15:32:04.926Z `shutdown cancelled` (line 38322). Before it: `Process 967/66820 received SIGTERM from owned execution repository:command (killing process 66685): cancelled`, later 4464/4469 the same. 0.7 s later `request-failed a4605e03 Error: Original Script process terminated`. The service had run since 14:19:36Z (4341 s heartbeat).
- exit.log at the same second: two services with different tags die together, `66820` (tag 2bf1553db5e2, starter wrapper 66685) and `73334` (tag 7c43b02c1fa3, wrapper 72965), both `reason: stop:cancelled`. One external event ended both.
- 15:44:16.222Z `shutdown cancelled` (line 41370), pid 4665 (wrapper 4594), `stop:cancelled`, new service 81179 started 15:44:17Z.
- No `restart`, `reload`, `responsible-gone`, `idle` or `stop:SIGTERM` row in either window: no explicit restart request, no reload drain (`shutdown reload`), no idle shutdown. `service host stop` would end in `stop:SIGTERM` and is not in the log.

## Cause

The service is a `repo service host run` Script process (launchd job or detached child) that runs the host as its child. Every Script process starts `startScriptProcessOwnerWatchV1` (`🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`): its owner is read from `SEMIO_SCRIPT_PROCESS_OWNER`; when that pid is gone it calls `terminate()` (abort `Original Script process terminated`, owned-tree kill of the host and all slice workers, `stop:cancelled`).

`spawnHostServiceV1` passed the environment of the starting client on unchanged. `SEMIO_SCRIPT_PROCESS_OWNER` (the pid of the client's Script process) and `SEMIO_SCRIPT_PROCESS_INVOCATION` (its admitted deadline) therefore reached the service: through the direct spawn as they were, through the launchd property list because `allowedEnvironmentV1` allows every `SEMIO_*` name that does not look like a secret (the owner name does not). So the service took whichever client happened to start it as its owner, and an agent killing its own `repo run` (SIGTERM/SIGKILL) ended the shared service, all slice workers and every other client's request, and the restart restarted multi-minute reads. Two services started by the same agent lane ended in the same second.

The disconnect handling itself is correct: a closed connection aborts only that connection's requests, and shared loads keep running for followers (suites `cancel` and `slice-share`).

## Fix

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/🟨️.mjs`: `CLIENT_BOUND_ENVIRONMENT_V1` now also holds `SEMIO_SCRIPT_PROCESS_OWNER` and `SEMIO_SCRIPT_PROCESS_INVOCATION`; `spawnHostServiceV1` deletes them for the direct child and for the launchd job (both derive from the same `env`).
- `.../🏗️composition/🌐️host/🍎️launchd/🟨️.mjs`: `allowedEnvironmentV1` refuses names with `PROCESS_OWNER` (also keeps the consent probe job independent of its asker).

Residual, not changed: an explicit `repo service host stop` / `launchctl bootout` still ends the wrapper, whose Script termination signals the whole owned tree at once (workers included), so the 5 s drain of the host cannot protect slice loads there. That is a deliberate stop; the reload path (`shutdown reload`, supersession) already drains and hands over (suite `supersession-drain`).

## Tests

New suite `🏡️resident/🧪️tests/🛡️client-kill/🟦️.ts` (+ fixture `👁️owner-watch.mjs`, which runs the real owner watch):
1. control: a process that inherits a killed client as owner ends itself (the cause, reproduced).
2. a service started through `spawnHostServiceV1` with a client as owner carries no owner, and keeps running after that client is SIGKILLed (5 s, past the 2 s owner check).
3. `allowedEnvironmentV1` drops owner and invocation names (launchd job and probe).

Run with `repo test-file <file> --lang=en`: client-kill 3 pass; also green: `launchd` 9, `consent-service` 31, `cancel` 37, `slice-share` 5, `host` 39.
Existing coverage of the requested scenario: `slice-share` "a leader whose client is killed: the load goes on for the follower" and `cancel` "cancelling one of two clients leaves the other and the job running".

The running services (pid 81179) still carry the old environment; they pick up the fix on their next start.

## Bringing the fix into effect (no stop, no bootout)

Done without `service host stop` or `launchctl bootout`: the edit changed the implementation identity, so the old service (pid 81179, tag 4548f6e7a8a7, wrapper 81103) is superseded. At 16:08:01Z (`exit.log`) one `repo service host start` started the service of the new implementation next to it (tag debd968c7f2a, pid 15162, wrapper 15038, label d8978d3b5ce5). Nothing signalled the old one.

Verification with `ps eww`:
- old pid 81179: `SEMIO_SCRIPT_PROCESS_OWNER=81103` (its wrapper), the old wrapper 81103 still holds the starter's value.
- new wrapper 15038: no `SEMIO_SCRIPT_PROCESS_OWNER`, no `SEMIO_SCRIPT_PROCESS_INVOCATION`; new service pid 15162: owner is `15038`, its own wrapper, not the client that started it (my `repo` pid appears nowhere).

In-flight requests: the old service is not cut. By design (`drainAndExit`, supersession) it is only drained once idle or its work stalled; at the time of writing it still ran (28 min, ~4 GB, loads in flight) and no `request-failed`, `Original Script process terminated` or `shutdown` row was written after 15:44:16Z in its log. Its clients keep their answers; it exits by itself and hands remaining loads to the new service. Not forced: forcing would be the cut this ticket removes. New clients already reach the new service (`repo service host status` answers pid 15162).

## Implicit start without launchd: detach from the client's owned execution

Cause: the direct spawn (`detached: true`) gave the service its own session and group, but it stayed a child (ppid) of the starting client. The owned-tree termination of the client (`ps` ancestry on POSIX, `taskkill /T` on Windows) follows parent links, so tearing down the client's owned execution found the service and killed it with its workers ("SIGTERM from owned execution repository:command (killing process ...)"). The earlier owner-variable fix removed the owner watch only.

Fix: new `🏡️resident/🏗️composition/🌐️host/🪂️detach/🟨️.mjs` (`spawnDetachedV1`, plus `.d.mts`). A short relay (the runtime itself, `-e`) starts the service `detached` (new session and process group on POSIX, detached process on Windows), with stdin ignored and stdout/stderr on the service log (no stream shared with the client), prints its pid and exits. The relay is gone before any teardown, so the service is reparented (POSIX) or its chain to the client is broken (Windows: `taskkill /T` walks only live parent links), and it is registered as no child of the client. `spawnHostServiceV1` uses it for the direct route (macOS, Linux incl. the cgroup placement of the real pid, Windows); the launchd route is unchanged. Windows is covered by the same relay but the suite ran on macOS only.

Tests: new suite `🧪️tests/🪂️detach/🟦️.ts` (+ `🧑️client.mjs`): a client starts a service through the relay, the client's owned execution is torn down with `terminateOwnedProcessTree`; the client ends, the service (own session and group, ppid not the client) and a second client with its service keep running. Control: a plainly detached child is still the client's descendant and dies with it. Results: detach 2 pass; `host` 39, `consent-service` 31, `client-kill` 3, `client` 8 pass.

The running services (including pid 15162) were started before this change and keep the old tree link until their next start.
