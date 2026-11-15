# Map: content stores, GC and disk (read-only exploration)

Scope: `/Users/Shared/semio`. Read-only. No git, no builds, no GC, no services. Only the five bun unit-test files listed in section 7 were executed. Guesses are marked (guess).

Shorthands used below:
- BUILD = `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`
- CACHING = `/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching`

## 1. The CAS (`BUILD/🗄️cas`)

What it stores: blobs keyed by the sha256 of their bytes (`CasDigestV1` = hash + size). It is a blob store. It does not know build keys, tasks or worktree paths.

Layout on disk (root = `SEMIO_CAS_ROOT`, else `~/.cache/semio/cas`, or `%LOCALAPPDATA%\semio\cas` on Windows; `BUILD/🗄️cas/🗃️store/🟦️.ts:42-47`):
- `v1/o/<hex[0:2]>/<hex[2:]>`: objects, chmod 0444, placed by atomic rename (`store/🟦️.ts:50, 67-72`).
- `v1/t/`: temporary files before rename. Stale ones (older than 1 h) are swept by the collector (`collector/🟦️.ts:93-96`).
- `v1/p/<holder>.json`: pins, one file per holder, `{version,holder,pid,expiresAtMs,digests}` (`pins/🟦️.ts:9-40`).
- `v1/l/<shard>/<writer>.jsonl`: append-only access ledger, one segment per writer process (`ledger/🟦️.ts:9-16`, `:46-71`). Access = latest timestamp per digest.
- `v1/usage.json`: usage hint written by the last finished collection (`collector/🟦️.ts:162-177`).
- `v1/gc.lock`: collector single-flight lock (`collector/🟦️.ts:39-58`), stale after `lockStaleMs`.

Areas and partition:
- "Areas" are loose cache directories outside the store that count against the host cap: `<workspace>/.cache/b/g/m/build-input-closures` and `~/.cache/semio/repo/<worktree>/journal/graph-memo` (`areas/🟦️.ts:6-17`).
- "Partition" = the volume holding the store, measured with statfs of the nearest ancestor (`partition/🟦️.ts:8-13`). Used share thresholds: high 0.90, low 0.80, critical 0.97 (`schema/🔣️.json`, `Policy.partition`).
- Pure admission rule: `decideCasEnforcementV1` (`partition/🟦️.ts:35-47`): allow, collect-background, collect-now, measure, or refuse.

Pins and adoption:
- Pins protect objects from eviction and are judged live only if the holder pid is alive and the lease has not expired (`pins/🟦️.ts:28-40`). Max lease 1 h (`leaseMaxMs`).
- `adoptCasV1` (`adopt/🟦️.ts:38-108`) replaces identical files in declared places (workspace paths in `schema/🔣️.json` `Adoption.places`, e.g. `.cache/b/d/m/bun-cache`, hardlink mode, write bits cleared) with hard links to a store object. Reflink places use copy-on-write clones.

Operations (`store/🟦️.ts`): `put` (streams, hashes, stores), `putChecked` (verifies size/digest first), `get`/`read` (touch the access ledger), `has`, `link` (reflink, then copy; hardlink only via adopt), `adopt`, `pin`, `release`. Sync variant for build scripts: `store/🔁️sync/🟦️.ts` (`putCasObjectSyncV1`, `readCasObjectSyncV1`).

Writers and readers:
- Writers: `put` callers and `adopt`. Callers seen by grep (not exhaustive): `BUILD/⌨️command/🏃️task/🤝️shared/🟦️.ts`, `BUILD/🔧️tools/🧶️nx/🔌️backend/🟦️.ts`, `BUILD/🌡️host-capacity/💽️disk/🟦️.ts`. The revision store (`BUILD/🧊️revision/🟦️.ts`, `pinRevisionV1` at `:75`, `sealEntriesV1` at `:100`) keeps input manifests in the CAS and pins their digests.
- Readers: `get`, `read`, `link`, `has`.

Local or shared: local per host by default. The root can be any path via `SEMIO_CAS_ROOT`, so a shared location is possible by configuration (no shared root is configured now). Key facts that make a shared root unsafe today:
- Pin liveness and the GC lock use `process.kill(pid, 0)` (`pins/🟦️.ts:11`, `collector/🟦️.ts:30`, `:52`). A pid from another machine is meaningless on this host, so pins or locks from other machines would be judged dead or alive wrongly.
- The collector lock relies on `open(...,"wx")` (`collector/🟦️.ts:43`). Its atomicity over NFS is not guaranteed (guess).
- Access times come from the local clock and from `mtime` (`collector/🟦️.ts:81`).

Live state on this machine: `/Users/ueli/.cache/semio/cas/v1` exists (created Nov 12). 19,564 objects in `v1/o`, 256 shard directories in `v1/o` and `v1/l`, pin files in `v1/p` from one holder, `v1/t` empty, no `usage.json` yet, no `gc.lock`. Total 8.63 GB (`du -skx`).

## 2. `cache-gc` (`BUILD/🧹️cache-gc`) and the CAS collector

What is deleted, and by what entry point:
- `repo cache gc [--area=...]` (`cache-gc/⌨️command/🟦️.ts:72-105`): runs `collectCacheV1` for the workspace areas, then `collectHostStoreV1` (`:103-105`) for the CAS and its areas. The host step is skipped when `--area` is given.
- `repo cache status|verify|dedupe` are the other subcommands (`:72`). `cas` is dispatched to `BUILD/🗄️cas/⌨️command/🟦️.ts` (`:74`).

What triggers a collection (no cron or timer found in the code I read; not exhaustively verified):
- Detached background launches, single-flight by stamp files, never waiting (`cache-gc/🚀️launch/🟦️.ts`):
  - Sweep: `launchBudgetSweepIfDueV1` (`:112-126`) runs when the interval is due, or when areas are recorded over budget, per `budgetSweepDecisionV1` (`cache-gc/🧮️sweep/🟦️.ts:19-25`).
  - Free-space triggers: graph, products and tools images below their soft free-space threshold (`:63-109`), at most once per interval.
- CAS guard: a write or adoption calls `guard.admit`. At high watermark (cap 0.90 or partition 0.90) a collection starts in the background. At critical partition share (0.97) or cap exceeded, the write waits (`BUILD/🗄️cas/🚦️guard/🟦️.ts:63-77`). Process-wide state map keyed by root (`:29`), so single-flight only within one process; across processes only `gc.lock`.
- Disk admission before heavy commands: `admitBuildDiskV1` (`BUILD/🌡️host-capacity/💽️disk/🟦️.ts:15-25`) calls `guard.admit(0)`.
- Service tick `casEnforceTickV1` (`guard/🟦️.ts:94-97`): exists, caller not identified (guess: the resident host).

The rule: CAS collector (`BUILD/🗄️cas/🧹️collector/🟦️.ts:99-149`, planner in `collector/🧮️plan.ts`):
1. Scan all objects; access = max(mtime, ledger access). Protected if pinned, hardlinked (`nlink>1`), or younger than `minAgeMs` (10 min) (`:72-91`).
2. Planner, least-recently-accessed first (`plan.ts:51-93`): stage 1 drops entries older than an area's `maxAgeMs`; stage 2 cuts an area above its cap to its low watermark; stage 3 cuts the host total above high watermark 0.90 down to low 0.75, and to the partition's low share when the partition is above its high share.
3. Second pass re-checks pins and fresh access right before every unlink (`:130-141`).

Caps (`schema/🔣️.json`, `Policy.const`): `capBytes` 34359738368 = 32 GiB; area `graph-memo` cap 4 GiB and `build-input-closures` cap 8 GiB, both with `maxAgeMs` 7 days.

Workspace areas (`CACHING/💾️storage/🧾️ledger/🔣️policy.json`): roots `repository` (`.🧬semio/🦑️repo/⚡️cache`, cap 24 GiB), `native` (`.cache`, cap 768 MiB), `rust` (`.r`, cap 0). Areas have `budget` or `collect` rules (`unusedAgeMs`, `quietMs`, custody: quiet, sqlite, lease or pid). Units in use are protected by SQLite lease probes, process ids in names, or quiet time (`cache-gc/🏃️runner/🟦️.ts:89-112`).

Reachability from current state: NO. Nothing marks entries reachable from the current worktree or from current input hashes. Eviction is LRU/age plus budgets. Partial exceptions:
- Graph observation blocks no record names any more are removed (`supersede` port in `cache-gc/🚰️reclaim/🟦️.ts:8, :28`).
- Build-input closures are removed only when "vouched" (`reclaim.ts:9, :27`).
- Retired roots (`retirements` in the ledger policy).
- The cargo unit-cache GC `planGcV1` (`CACHING/🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts:47`) was not read in full.

## 3. Sizes today on this machine

Disk:
- `/System/Volumes/Data`: 926 GiB total, 797 GiB used, 106 GiB free, 89 % (`df -h /Users/Shared`). This machine is not a 256 GB mini.

Local caches (`du -skx`, so no mounted images are crossed):
- `~/.cache/semio/cas`: 8,628,076 KiB (about 8.2 GiB). Object cap 32 GiB.
- `/Users/Shared/semio/.🧬semio/🦑️repo/⚡️cache` (repository root): 3,952,504 KiB (about 3.8 GiB).
- `~/Library/Caches/semio`: 1,919,372 KiB (about 1.8 GiB). Contains agent-sessions, execution-targets, host-capacity, lease-registry, native, repo, runtimes, scheduler.
- `/Users/Shared/semio/.cache/f`: 307,460 KiB (about 293 MiB). Holds `state.json` (105 MB), `state.ckp` (124 MB), `state.wal` (13 MB). Not analysed further.
- `/Users/Shared/semio/.cache/b/c`: 4 MB (SQLite control files, claims, queues).
- `/Users/Shared/semio/.cache/b/l/fixed.dmg`: 66 MB file; mounted at `.cache/b/l/m` (`/dev/disk5s1`, 63 MiB image, 4.2 MiB used).
- `/Users/Shared/semio/.cache/b/o/fixed.dmg`: 2.1 GB file; mounted at `.cache/b/o/m` (`/dev/disk7s1`, 2 GiB image, 28 KiB used).

Budgets:
- Machine budget default: `defaultTotalBytes` 212937474048 = 198.3 GiB (`CACHING/💾️storage/🧾️ledger/🎚️machine/🔣️policy.json`). Partition stores: rust 48.4 %, dependencies 18.2 %, products 12.1 %, graph 3 %, tools 4 %, diagnostics 1 %, rest by policy.
- CAS cap 32 GiB. The comment in `BUILD/🗄️cas/🧬️schema/🟦️.ts:11` says "120 GiB of the 200 GiB disk budget". That is stale against the 32 GiB value.

How 256 GB machines cope (guess, not measured here): the machine budget is 198 GiB on a 256 GB disk, leaving about 50 GB for the OS and apps. The CAS and repo caches would then be constrained by their own watermarks and by the partition critical share (0.97).

## 4. Defining "cache only for the synchronized state"

Where keys are computed (not fully read):
- Cargo units: `unitKeyV1` (`CACHING/🦀️cargo/🗃️unit-cache/🔏️key/`), `crateKeyV1` (`🧵️lane/`).
- Graph inference and task inputs: `BUILD/🕸️graph/📥️inference/`, `BUILD/🧮️task-model/🟦️.ts` (`buildTaskInputsV1`, `planBuildTasksV1`, `buildDryRunPlanV1`, used by `BUILD/🔧️tools/🐙️semio/🔌️backend/🟦️.ts:17`).
- Revision manifests: `BUILD/🧊️revision/` (`InputManifestV1`: path to digest, pinned via `pinRevisionV1`).

Enumerating current keys without building (guess, needs verification): the task model has a dry-run planner (`buildDryRunPlanV1`), so the set of tasks a worktree state would run may be enumerable from the manifest without executing them. That would give the current task keys. The missing piece is a mapping from task key to output digest in the CAS (no such index was found in CAS). The unit-cache keeps its own lookup store (`CACHING/🦀️cargo/🗃️unit-cache/💾️store/`).

A precise definition would be: roots = digests of the current manifest plus the output digests of the current task keys (via an index); everything else in the CAS, the caches and the areas is eligible, subject to pins and holders. Nothing in the code implements this mark phase today.

## 5. Machine dependence

- CAS objects: keyed purely by content hash. Machine-independent.
- Pins and locks: holder pid and host liveness (`pins/🟦️.ts:11`, `collector/🟦️.ts:30`). Host-local.
- Access ledger and eviction: local clock and local mtime/atime. atime may be unreliable on noatime volumes (`cache-gc/🏃️runner/🟦️.ts:42-49` checks it).
- Roots: `~/.cache/semio/cas` (home-dependent); `SEMIO_CAS_ROOT` overrides.
- Native images: `.cache/b/*/fixed.dmg` plus `image.json` envelopes (host sizes). Per machine.
- Ledger roots and areas: workspace-relative. Size ledger `🧭️size-ledger.json` in the repository cache root: local.
- Adoption places use `workspace` absolute paths at run time, not in keys.
- Windows-specific roots and modes in `casRootV1` and `adopt` (write bits, chmod).

## 6. `fleet-status` (`BUILD/🚢️fleet-status`)

- `🟦️.ts` is the pure aggregation (facts in, report out). Readers are in `📥️readers/🟦️.ts`, with `FLEET_READERS_V1` at `:129`. The command is in `⌨️command/🟦️.ts`.
- Reads (all local to this host and checkout): leases of the checkout (`.cache/b/l/m/data`, `:28`), host capacity (`readHostCapacityStatusV1`, `:56-60`; host RAM, pressure, swap, load, grants, queue), Bazel slots, storage (`readFleetStorageV1` at `:85`: host statfs, the cache ledger report without walking trees, machine budget plan), sealed store, graph generation pointer, scheduler.
- Other machines: NOT known. There is no reading of other machines' caches or load. Its scope is one host plus the checkout.

## 7. Tests run

Command form: `cd <tests dir> && /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun test ./<file> --timeout 120000`. Bun v1.4.2.

| File | Result |
|---|---|
| `BUILD/🗄️cas/🧪️tests/🟦️.ts` | 35 pass, 0 fail (1.9 s) |
| `BUILD/🧹️cache-gc/🧪️tests/🧶️units/🟦️.ts` | 6 pass, 0 fail (1.4 s) |
| `BUILD/🚢️fleet-status/🧪️tests/🟦️.ts` | 11 pass, 0 fail (1.3 s) |
| `CACHING/💾️storage/🧾️ledger/🎚️machine/🧪️tests/🟦️.ts` | 7 pass, 1 FAIL |
| `CACHING/💾️storage/🧾️ledger/🧪️tests/📐️model/🟦️.ts` | 3 pass, 0 fail |

The one failure: "laws: quotas add up, nothing under its fair share is cut, planning converges in one step, a larger budget never evicts more". Assertion at test line 110: `case 280 blocked never grows`, expected `<= 15641791724`, received `15641791725`. A 1-byte rounding discrepancy in the split or the blocked-bytes law.

Not run (they spawn processes, touch real caches, or are not unit tests): `cache-gc/🏃️runner`, `cache-gc/🚀️launch`, `cache-gc/👯️dedupe`, the CAS python oracle and enforcement tests, `fleet-status/🛡️guard` (spawns workers), `sealed` process tests.

## 8. Gaps between what exists and the goal

1. No mark phase. Nothing computes the set of keys the current synchronized worktree state would produce. GC is LRU/age/budget only, so entries for old states are only removed by age (7 days for areas, 10 min minimum for store objects) or by budget pressure.
2. No index from task or unit key to CAS digest. Outputs in the CAS are unreachable by build key.
3. Pins and the GC lock are judged with local `kill(pid,0)`. A shared hot store across 50+ machines would misjudge holders from other hosts. Needs host-qualified holders plus leases.
4. Single-flight is per process for the guard (`guard/🟦️.ts:29`) and a local-filesystem lock file for the collector. Not safe for a shared NFS store as-is.
5. Access times come from each host's clock; no logical or fleet-wide recency.
6. Two stores with overlapping purpose: CAS (`~/.cache/semio/cas`), the repository and native caches (per workspace), and the cargo unit-cache lookup store. No single hot cache.
7. Caps are fixed (CAS 32 GiB) while the machine budget is 198 GiB by default. Caps are not derived from the synced state or from per-host disks.
8. Stale doc comment: `BUILD/🗄️cas/🧬️schema/🟦️.ts:11` says 120 GiB of 200 GiB, but the value is 32 GiB.
9. A failing law test in the machine budget (1-byte rounding, see section 7).
10. `fleet-status` shows no other machine state; no fleet-wide load or cache view exists.
11. The trigger for `casEnforceTickV1` is not identified (guess: resident host). Unverified whether any timer runs it.
12. The cargo unit-cache `planGcV1` (`CACHING/🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts:47`) was not read in full; it may hold a different eviction notion.
