# R190: g1-m02 `resource-directory-admission:v1` timeout and the terminated script process

Living report. Status at the top, updated at each step.

STATUS: LANDED 07:30:15 to 07:30:45 UTC, green (see landings.md). Section 6 has the g1-m02 watch.

## 1. What the failure is (read-only evidence, no access to g1-m02)

`service-failure.txt` 06:14:04 UTC: the stack is `awaitQueuedTurn -> reapQueueTicket -> queueClaimCatalog -> directoryAdmission -> acquireUngatedResourceLease`. What I could prove is not a stuck holder of the admission lease but a **reaper that spends the caller's whole timeout**:

- `resource-directory-admission:v1` is a SQLite lock on one file of the lease directory (`<dir>/<sha256>.sqlite`). The kernel releases it when its process dies, so a dead holder never keeps it (law 4 below). Nothing walks the catalog while holding it: every admission hold is for one ticket.
- The waiter (`awaitQueuedTurn`) reaped its predecessors **serially, one admission + one catalog + one claim database per ticket, inside its own timeout**: measured on this Mac 3.0 to 3.1 ms per abandoned ticket, linear (200: 0.6 s; 500: 1.1 s; 5000: 15.5 s; 20000: about 62 s). With 20,000 abandoned tickets every waiter whose timeout is shorter than a minute times out while it is still reaping, and the next waiter starts the same walk again from the nearest ticket.
- The timeout then surfaces **under a wrong name**: when the budget ends inside the reap (the 9 s run of 5000 tickets) the error is `Resource lease timed out: resource-directory-admission:v1`, although the admission was free; the real resource is never named. That is exactly the g1-m02 message, and the resident service reports an untyped `Error` as "unexpected failure".
- The queue directory itself had no bound on the work per acquisition and the ordering sort did a regex parse per comparison (`arrival()` in the comparator), O(N log N) parses per poll.

What I could NOT establish: how many abandoned tickets g1-m02 holds (its lease directory is not reachable from here; its host log `host-a490ddab00a8.log` is on the machine). The model is this Mac: 27 lease directories reachable from the registry, largest 5 tickets and 8 claims (oldest ticket 578 h, `framework/.cache/b/l/m/data`), so this Mac does not show the growth; g1-m02 never had its leases swept for days (history: one 11-17 read of many hours, then `SEMIO_LEASE_ORDER`). The first typed refusal after the landing will print the entry count (section 4).

## 2. Fix by construction (leases `🟦️.ts`, `⏳️wait` policy, process texts)

(a) O(1) admission, bounded reclaim, continued across acquisitions: `reapQueueTickets` takes ONE shared catalog admission per **slice** (`reapSliceTickets` 256, `reapSliceMilliseconds` 150, schema `⏳️wait/🧬️schema/📜️policy.json`, `WaitPolicy`), probes the abandoned tickets and stops; the waiter continues in its next iteration and in the next acquisition. The queue read parses each name once and sorts numerically.
(b) Liveness: a ticket whose pid is gone (`kill(pid, 0)` = ESRCH, cached per pid per pass) is **skipped in the turn decision** and reaped first; the proof that deletes a ticket stays the independent kernel claim (exclusive claim lock = requester gone), which is stronger than pid plus start time and also right for a reused pid. A live-pid ticket is probed only for the nearest one (an unbroken chain of live waiters is not probed more than before). So the waiter no longer waits for 20,000 dead tickets: it decides on the live ones.
(c) Dead admission holder: by construction, the SQLite lock dies with its process; law 4 proves a SIGKILLed raw holder is stolen at once.
(d) A timeout of the admission is a typed refusal `timeout` / `process.leases.admission-timeout` (EN and DE), built by `admissionRefusalV1` at the queue boundary: resource, directory, seconds waited, entry count of the directory, claim count, and the three longest-silent live registered processes of the directory (pid, command, silent seconds). Its `message` still starts with `Resource lease timed out: resource-directory-admission:v1`, so every existing `includes("Resource lease timed out")` check and the admission context (`resourceLeaseAdmission`) keep working.
(e) Bound: the directory is already capped (`maximumDirectoryEntries` 65,536, claims `maximumDatabaseBytes` 16 KiB) and the global claim sweep (`sweepResourceLeaseStoreV1`, 256 claims per turn, deletes the claim and its ticket together) is the collector. I did NOT add a row to R169's disk-recovery registry: that registry is about volume bytes (a lease directory with 20,000 claims is about 200 MB, relief class host) and its files are being repaired by R189 after the R186 overwrite (finding 06:30). Decision for the coordinator: add a `lease-claims` writer row there after R189's repair lands.

## 3. Laws (`🧪️tests/🪦️stale-queue/🟦️.ts`, 5 tests, bun:test, real SQLite, Python `sqlite3` as the independent oracle of the claim locks)

| law | base (red) | stage |
|---|---|---|
| wait policy declares the reap slice, validates against its schema | red (key missing) | green |
| 20,000 abandoned tickets from a dead pid: first acquisition < 5 s, the queue shrinks in every following acquisition, no other acquisition waits more than 2 s, 40 sampled claims are free for Python `BEGIN EXCLUSIVE` | red: `Resource lease timed out: r:stale` after 81 s | green: first acquisition 455 ms (62 s before), slowest probe admission 6 ms, 15 further acquisitions reclaimed 1,720 tickets, each under 400 ms |
| a budget that ends inside the reclaim is the timeout of the real resource or the typed refusal, never the untyped admission error | red: names `resource-directory-admission:v1` | green |
| a dead holder of the admission is released by the kernel, the waiter acquires at once | green (holds by construction, kept as a guard; one noisy base run was 12 s under load) | green |
| a live slow holder: waited for, then the typed refusal in EN and DE names directory, entry count and the holder pid | red | green |

Battery before and after (staged leases against the live neighbours, identical): standing 26, standing/live 4, physical lock-attempts 5, admission 17, routing 1, status 2, terminal-release 1, threads 1, order 76, ground 5, processes 28, wait-for 6, wait 15, fairness 17 (all 0 fail, equal to base).

## 4. Task 3: who terminates the script process on g1-m02

Evidence: `ready-log-workspace-failed.txt` and `ready.json.containment`: at every start the stage runner (process 85128) SIGTERMs **two `bun` processes (65818, 65819) of the roster of the earlier try** ("it stood in the roster of an earlier try"), then logs "Starting the repo graph service for this host" again. `terminate` at `routing/invocation/process/🟦️.ts:250` is the **SIGTERM handler of a Script process** (`controller.abort(Error("Original Script process terminated"))`): the message is exactly what a Script process says when a signal arrives, and the 05:52:36 failure carries the same second as the runner's start of the try (`startedMs` 1794981156751 = 05:52:36). So the answer is the stage runner itself.

Why the shared service is in the roster: a stage's roster is every descendant sampled during the stage. The first caller of the stage starts the resident host (`bun door/script.ts repo service host run`, a Script process, plus its child `bun --eval ... <digest> service host run`, host descriptor `host-<tag>.json` with `owner.pid` = the launcher) as a descendant, so it is in the roster. The sweep spares only registry entries whose command matches `sharedCommandPattern`, and `readyKillPlanV1` spared the service and what is below it but **killed its launcher** (the owner of the service, a Script process: it gets SIGTERM, aborts "Original Script process terminated", and the service it hosts fails with it; the fixture even pinned "root and sibling still stopped" without the owner). When the registry entry is missing or does not match (a host that holds no lease at that moment, an entry pruned), the service is not spared at all. On g1-m02 both processes were reaped, so the second case is the likely one; I cannot see its registry. Both are design faults.

Fix (files under `sidecar/🚦️ready/🌲️process-tree`, none in `🚦️ready/🟦️.ts`, which R185/R186/R189 contest):
- `readyKillPlanV1`: the launchers between a spared pid and the root are protected too (the root and the siblings are still stopped). Fixture row and Python oracle extended (red on the base plan).
- `readyHostPidsV1` / `readyHostDirectoriesV1` in `🧹️containment`: a live resident host named by its own descriptor `host-<tag>.json` (`<tmp>/semio-repo/<uid>`, `<tmp>/sr<uid>`; pid in the table, started no later than the descriptor and within `hostStartWindowMilliseconds` 600 s before it, not `stopping`) is spared whether or not it has a registry entry (live predicate of the service itself); `readySharedPidsV1` unions it. Law with real `sleep` processes: live, stopping, old, late, damaged, foreign-named, missing directory.
Not proven on g1-m02: that these two processes were the host and its launcher (the host log of that machine is needed), that the service then survives a try.

## 5. NOT verified

- g1-m02's directory size and holders (not reachable), the real Script-process path on a worker, a real stage try with a spared host (only injected laws and real `sleep` processes).
- No full `repo test`; the old `testResourceLeases` super-test (needs a control/original invocation) was not run; its neighbours were (section 3).
- (e) the R169 registry row.
- Running reads and the resident services keep the old code until they restart.

## 6. Landing and the next message of g1-m02

Landed 07:30:15-07:30:45, hold 30 s; under the lock stale law 4, texts 3, wait 15, tree 16, containment 16, 0 fail. g1-m02 at 07:31 UTC: attempt 55, `state: running`, stage `workspace`, last line "Reading the project graph slice (374 s) ... 94 of 600", no new failure file after the 06:14 one. It still runs the old code until its services restart, so the verdict (next verbatim message or `ready: true`) is still open; recheck `origins/g1-m02/ready.json` and `service-failure.txt` after the next try.
