# 📈 Summary: Fleet Monitoring With Multi-Resolution Metrics (2026-11-26)

## Delivered (end to end)
- **Catalogue + tiers + format** (M2): `🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/` — 69-metric en/de catalogue (`scope origin|fleet`, per-metric `platforms`), tiers t0 1 s×300 · t1 10 s×360 · t2 1 min×1440 · t3 10 min×1008 · t4 1 h×2160, fixed-size ring files with exact folds; TS writer/reader, std-only Rust reader, golden vectors + Python oracle; merged non-uniform series with clipped `partial` buckets (no tier gap).
- **Collection + publication** (M3, M6, M7): collector inside the readiness service (no extra daemon): 1 s host gauges, 10 s counters/activity; rings published per origin to `<store>/v1/origins/<origin>/metrics/`, fleet-scope metrics (exec queue/offload, remote cache size) to `<store>/v1/fleet/metrics/<source>/`. Collector cost 0.11–0.25 % of a core, ~44 KB retained; publication ~2.8 KB/s per machine. Live on 15/16 origins (g1-m09 unprovisioned), macOS/Linux/Windows.
- **Counters** (M5): persisted cumulative remote-cache and scheduler counters (`📈️counter-ledger`), local hit counters (unit cache, Nx, CAS); store hygiene (pool temps, published-file retention, AppleDouble retirement).
- **Dashboard** (M1, M4, M8): chart widgets crate `semio-ui-tui-charts` (Sparkline, LineChart, Gauge, StatCell); `Fleet` window (`W`): per-origin overview (ready/stage/beat, CPU/mem/disk gauges, CPU and units sparklines, cache hit ratios local/remote, offload, host-name warning), fleet totals row, zoom 5 min/1 h/24 h/7 d/90 d with tier separators, per-origin drill-down by metric group, en/de labels, `⊘ n/a` for unsupported metrics; live headless render test from the real store; identity-owned collector lock.

## Evidence
Results `result-M1..M8.md`, audits `audit-MA1..MA3.md`, design `design.md`, landings in the fleet ticket `landings.md` (`## Monitoring …`).

## Not verified / open
- `repo dashboard` never run interactively by agents (per CLAUDE.md) — verified via render tests incl. the live-store render.
- Dashboard data path ~1.5 % of a core per open Fleet window at 5 min range (measured at load 45–116).
- `process.repo.cpu` unsupported on Windows; 14 pre-existing failing tests in the terminal laws crate (docking/source editor), not from this ticket.
