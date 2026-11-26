# Result M3: collectors, local rings, fleet publication

## Where it lives
- Framework member `🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🛰️collector/`: `🧬️schema` (Policy cadences, `Collectors` = 16 feeding collectors with cadence/source/ids, `Unsourced`, enums, Baseline, Lock, Texts en/de), `🟦️.ts` (`createFleetMetricsCollectorV1`, pure `runMetricsCollectorV1`), `📤️publication`, `🔌️host` (processor via os.cpus, memory, warden state file, gate `sample.json`, statfs volumes incl. `.cache` children, queue depth, self; no process spawn).
- Product sources + start: `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/📈️metrics/🟦️.ts`; started from `runReady` (the per-machine readiness service that already mirrors the warden state), no new daemon.
- Scripts: `🧰️scripts/M3/{schema.py,overhead.ts}`.

## Behaviour
- 1 s tick: cpu, memory (warden state, else gate sample, else OS), warden, process-tree (running tasks = unit, cargo, stage kinds), self. 10 s tick: volumes (least free volume), queue, unit counters, local cache (CAS localHit/localMiss + Nx nxLocalHit/nxExecuted), remote cache ledger (only on the machine that runs the server, found through `fleet/cache.json`), offload (files + `offloadedIn` from the scheduler ledger `build-scheduler.counters.json`), owner pool/slice phase, ready, journal head.
- Counters: cumulative readings to deltas with M2 `createCounterDeltasV1` (first = baseline, decrease = restart). `baseline.json` persists the readings so a restart of the service (every code change) keeps the count; older than 120 s it is only a baseline.
- One writer per ring dir (`collector.lock`, pid+start identity, dead holder taken over). Rings: `<host-capacity dir>/metrics/` (+ `live/t0.ring`, 60 buckets).
- Publication into `<SEMIO_FLEET_STORE_ROOT>/v1/origins/<origin>/metrics/t0..t4.ring`: kernel copy to `<name>.<pid>.tmp`, header check, rename; temp removed on failure, dead writers' temps swept, 16 MB bound, only the 5 names. Look every 5 s; a tier is copied when changed and after its minimum interval (t0 5 s, t1 30 s, t2 5 min, t3 30 min, t4 1 h; schema data). Deviation from the design: the published t0 is a 60-second live ring, not the 300 buckets (a full copy every 5 s is 150 KB/s per machine; now about 30 KB/s for t0, about 75 KB/s all tiers); t1 covers the rest, no hole. Origin ids only.
- Unsourced (collector declared, no cheap per-machine source, gap not zero; pinned by test): `process.repo.cpu`, `exec.offload.in` only on the scheduler host, `exec.queue.running/queued`, `graph.read.facts`, `cache.local.bytes`, `cache.remote.entries/bytes`.

## Proof
- `repo test-file` collector tests 18/0 (law: each of 69 ids in exactly one collector of its source; every collector with injected probes; delta math vs the M2 Python oracle incl. resets; baseline restart; lock; failing collector; publication atomicity, failed rename, refusal, sweep, never a torn file; host free space/size vs Python `shutil.disk_usage`, memory vs `sysconf`); product sources 3/0; M2 metrics 12/0; ready test file 58/0; dangling-imports 0.
- Overhead, 300 s on this Mac (load about 23, bun 1.4.2, real sources, publication to a scratch store): **0.50 % of one core** (1.51 s CPU incl. 0.16 s start and 69 file copies; steady about 0.45 %), step median 0.83 us x1000 = 0.8 ms, **RSS 43 MB at the end vs 31 MB bare bun (+12 MB)**, transient peak 98 MB while the rings are opened (M2 restore reads whole files; a possible follow-up). Marginal in the readiness service is smaller since modules are loaded. Right at the CPU budget; the cost is mostly the 10 s counter reads and file copies.

## Not verified
- Linux and Windows runs; gate-sample fields on Windows; rename over a reader-opened file on Windows (retried next look).
- Metrics lock was held by M2r2 throughout: only new subfolders were added in the member.
- Rust reader on the 60-bucket live t0 (TS reader verified).
