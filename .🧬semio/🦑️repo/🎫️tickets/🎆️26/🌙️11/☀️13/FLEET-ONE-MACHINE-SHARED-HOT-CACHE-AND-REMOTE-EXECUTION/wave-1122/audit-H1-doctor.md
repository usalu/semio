# 🩺 Audit H1 (doctor), Wave 1122

Read-only audit. No source, process, lease, cache, image or store file was changed. No fix command was run. Probe time: 2026-11-22 00:42 CET (`atMs` 1795304506448 in doctor).

Sources: `repo doctor --json` (language en, no fix), `repo capacity status --json`, `repo leases status --json`, `repo cache status --json`, `repo scheduler status --json`, `repo session list --all --json`, `repo fleet status --json`. Owner and lane mapping follows `coordination.md`.

Overall doctor verdict: `degraded`, counts error 0, warn 6, info 3. All nine doctor probes returned `ok`.

Not measured: remote cache status (6094, 113,507 entries, 147 MB, 1000 AC divergences, peers 0 hits / 704 misses / 20 errors, nx 0 puts) and remote execution status (8980/8981). These tools were not in the H1 list. Doctor and fleet status do not repeat those numbers.

## Unowned (no lane in coordination.md)

| # | Severity | Finding | Owner | Resolving command or code area | Measured |
|---|---|---|---|---|---|
| U1 | warn | Tools `micromamba` and `tectonic` not installed or provisioned (`tools.missing`) | tool-registry | `repo tools setup all` (not run) | doctor, tools probe 5681 ms |
| U2 | info | 3 undeclared cache entries: `repository:tools/.DS_Store` (8192 B), `native:test-admission` (0 B), `native:b/image-relocation.json` (4096 B) | cache | `repo cache gc --dry-run` (not run) | doctor, cache status |
| U3 | warn (H1) | 17 sessions `live` in `session list --all`. 13 have lastSeen older than 1 day, 12 older than 7 days, oldest 31.5 days. Doctor does not flag this. | sessions | not named by doctor; code area: session registry (`repo session list`) | session_list JSON, computed by H1 |
| U4 | warn (H1) | Orphan process pid 75460 `repo relay --peers g2=/Volumes/semio-peers/g2,g3=…,g4=…`, parent 1, age 549621 s (about 6.4 days), RSS 98 MB. Peers g2-g4 are not in the fleet list. | peer relay (no lane) | not named by doctor; owner must stop its own PID | doctor processes list |
| U5 | warn (H1) | Scheduler history: 12 of 16 finished jobs `exit: failed`, 4 `ok`. Failed: `run workspace:components#6f6fa2d8318a@712e68463780f74b` x6, `run @semio-tech/energy-oracle-py:deps#4bf5bb88d9d2@712e68463780f74b` x1 (later ok), and 5 test jobs (see E9 row below). Running 0, queued 0. | scheduler | `repo scheduler status` history; no fix named | scheduler status JSON, jobIds listed in result |

U5 test-job failures (`@semio-tech/math-rank-rs:test-source`, `repo:test-sdk-material` x2, `@semio-tech/process-js:test`, `repo:test-bootstrap-dependency-store-producer`) fall under E9. The `workspace:components` and energy-oracle failures have no lane.

## Owned findings

| # | Severity | Finding | Owner | Resolving command or code area | Lane |
|---|---|---|---|---|---|
| L1 | warn | 6 unfinished dependency generations of dead builds, oldest started 53939 s ago (`dependencies.unready-generation`) | dependencies | `repo dependencies ensure` (not run). Windows dependencies stage is E6. | E2 (E6 for m05) |
| L2 | warn | 24 dead process publications (`p-<pid>`) hold 489140224 bytes in partition `graph` (`caches.dead-publication`). Partition `graph` used 7031345152 B vs budget 6441402368 B, over by 589942784 B (statfs). | graph | `repo doctor --fix --only=caches` (not run) | E2 |
| L3 | warn | Load 31.8 on 10 cores (`host.load-high`). Fleet status load 31.81. | host-capacity | `repo capacity status` | E7 |
| L4 | warn | Swap 20064 MB used, swap-out 0 MB/s (`host.swap-high`). Host available 13607 MB, pressure normal. | host-capacity | `repo capacity status` | E7 |
| L5 | measured | Admission grant `YHKibCxee2WWhi4tz2Q9Xw` (`show`, pid 56836, session a3fba9d52db0.d) reserves 4096 MB, observed 446 MB, `overReservedMegabytes` 3650. Unmanaged 18715 MB. Evaluation `binding` available, `loadRoom` -11.809999999999999. | host-capacity | `repo capacity status` admission reserve | E7 |
| L6 | info | Lease registry: 12 of 71 folders belong to gone lease directories, 11 rows of dead processes (`leases.registry-stale`). Waiters 0, cycles 0, violations 0. | leases | `repo doctor --fix --only=leases` (not run) | E8 |
| L7 | info | Graph implementation files 160738 s newer than generation 260 (age 160901 s) (`graph.implementation-newer`) | graph | `repo show projects` (not run) | E2 |
| L8 | measured | Sealed entry `🏛️sealed/🚪️entry` drifted: 2296 changes since promotion 2297190 s ago, fresh false | graph / generation | `repo fleet status` sealed section | E2 |
| L9 | measured | Fleet refusals now: g1-m01 `no-headroom` (swap, load 12.98), g1-m11 `no-headroom` (swap, queued 1, offloadedOut 8), g1-m12 `no-headroom` (queue-order). Eight others `headroom: true`. Brief said six refusing; not reproduced. Counters: offloadedIn 15, offloadedOut 8, retried 7, refused 0, declined 0. Brief said offloaded total 3. | remote execution | `repo remote-execution status` (not run) | E5 |
| L10 | measured | Fleet lists 11 workers, not 13. Absent: g1-m04, g1-m06, g1-m09. Measured: g1-server, g1-m01, m02, m03, m05, m07, m08, m10, m11, m12, m13. | fleet liveness | `repo fleet status` | E8 (readiness H3) |
| L11 | measured | Zero cache traffic: g1-m05 (Windows, 16 cores) cacheHit 0 / cacheMiss 0 / cacheSkipped 0 / running 1; g1-m13 same zeros, offloadedIn 2 | cache / windows | `repo fleet status` per-machine counters | E6 (m05), E4 (m13) |
| L12 | measured | Fleet cache totals: cacheHit 131128, cacheMiss 251629; cache hits 15471, misses 7535, puts 5351. Fleet cacheSkipReasons empty on all machines. | unit cache | `repo fleet status` | E4 |

## Not verified or not measured

- Remote cache 6094 and remote execution 8980/8981: not measured (see top).
- Brief claim "coordinator swap 20.0 of 21.5 GB" uses 20064 MB from doctor, not 21.5 GB total.
- `fseventsd` RSS (9.6 GB in brief): not measured.
- Doctor `--fix`: not run. No cache, lease, process or store file changed.
- Session list: 257 entries, 237 `gone`, 3 `idle`, 17 `live`. The last-seen age is from the session list JSON only.
