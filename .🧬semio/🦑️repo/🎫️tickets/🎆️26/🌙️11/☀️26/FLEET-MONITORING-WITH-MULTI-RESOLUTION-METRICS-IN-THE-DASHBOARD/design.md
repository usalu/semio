# 📈 Design: Fleet Monitoring (02:55 CET, 2026-11-26)

Inputs: `audit-MA1.md` (dashboard = in-house Rust TUI, daemon IPC, en/de label macro, render tests), `audit-MA2.md` (existing per-origin files, warden/host-capacity samplers per OS, unit/offload/remote-cache/remote-exec counters, no time-series store).

## 1. Metric catalogue (schema-first, one source of truth)
`metrics/🧬️schema/🔣️.json` + `metrics/📇️catalogue/🔣️.json`: each metric = `id` (dotted, e.g. `host.cpu.busy`, `host.memory.available`, `host.disk.free`, `host.swap.used`, `build.units.compiled`, `build.units.restored.local`, `build.units.restored.fleet`, `cache.remote.ac.hit`, `cache.remote.ac.miss`, `cache.local.hit`, `exec.offload.in`, `exec.offload.out`, `exec.refused`, `graph.read.owners`, `graph.read.facts`, `ready.stage`, `process.repo.count`, …), `kind` (`gauge` | `counter` (monotonic, stored as per-bucket delta) | `state` (enum)), `unit` (`percent`, `bytes`, `count`, `seconds`, `ratio`), en/de `label`, `source` (which collector). Metrics without a collector are refused by a law test.

## 2. Multi-resolution tiers (RRD-style, fixed size, O(1) writes)
Tier table in the schema (all values are schema data, not code constants):
| tier | bucket | buckets | span |
|---|---|---|---|
| t0 | 1 s | 300 | 5 min |
| t1 | 10 s | 360 | 1 h |
| t2 | 1 min | 1440 | 24 h |
| t3 | 10 min | 1008 | 7 d |
| t4 | 1 h | 2160 | 90 d |
Each bucket = `start` (aligned), `count`, `min`, `max`, `sum` (mean = sum/count), `last`. Counters: the collector feeds deltas; buckets sum them. A bucket of tier n+1 is the fold of the buckets of tier n (associative fold → exact). Older tiers are only written when a lower-tier bucket closes.

## 3. Storage format (multi-implementation)
One file per origin per tier: `metrics/<tier>.ring` — header (magic, version, tier, bucket ms, bucket count, metric ids table digest, write cursor) + fixed-size slot array `[bucket][metric] → {count u32, min f64, max f64, sum f64, last f64}` (little endian). Writes in place at the cursor slot (pwrite), header cursor last → crash-safe (a torn slot is detected by its `start` stamp per bucket row). Size e.g. t2: 1440 × ~60 metrics × 36 B ≈ 3 MB. Writer: TypeScript (sampler). Readers: TypeScript (tests, CLI) and Rust (dashboard). Golden vectors shared by both, Python oracle for fold math.

## 4. Collection (per machine, zero-touch, minimal overhead)
- Lives inside the existing per-machine agent/sidecar (no new daemon). 1 s tick for host gauges via the existing warden/host-capacity probes (macOS `host_statistics`/`vm_stat`-free native calls already used, Linux `/proc`, Windows existing helpers) — no process spawn per tick.
- Build counters: tools already maintain monotonic counters (unit cache, offload, remote exec); the collector reads their files every 10 s and feeds deltas. Remote-cache server counters must be persisted and exposed cumulatively (today in-memory, reset on restart).
- Event-driven activity (compilation tasks running, graph read progress) via the existing process-tree classification and phase lines → gauges `build.tasks.running`, `graph.read.owners.done`.
- Budget: < 0.5 % core avg, < 30 MB RSS; measured and reported in `metrics.self.*` metrics.

## 5. Fleet publication
Local rings live in the machine-local cache (`<host-capacity dir>/metrics/`). Published into the shared store `origins/<origin>/metrics/`: t1..t4 copied when a bucket closes (≤ every 10 s, small), t0 published as a 300-bucket snapshot every 5 s (live view). Readers never need the origin's IP or host name.

## 6. Dashboard
New window kind `Fleet` in the Rust TUI: overview table (one row per origin: ready state, CPU/mem/disk gauges, sparklines for CPU and units compiled/min, cache hit ratio local/remote, offload in/out), keys to change tier/zoom (5 min / 1 h / 24 h / 7 d / 90 d) and drill-down per origin (multi-row charts per metric group). Data via the daemon reading the shared store rings (Rust reader), refreshed on file change / 1 s.

## 7. Lanes
M1 TUI chart widgets (running). M2 catalogue + tier schema + ring format + TS writer/reader + Rust reader + golden vectors. M3 collectors + agent integration + publication (depends on M2 API; can start on host gauges against the spec). M4 dashboard Fleet window (depends on M1, M2 reader). M5 counters persistence for remote cache + MA2 defects (pool tmp orphans, pool-published pruning).
