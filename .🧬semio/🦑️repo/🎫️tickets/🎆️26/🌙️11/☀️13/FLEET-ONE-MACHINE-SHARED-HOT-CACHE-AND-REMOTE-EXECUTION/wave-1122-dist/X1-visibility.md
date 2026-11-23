# X1: one-machine state visible in `repo fleet status` (design decision 8)

## Added (schema-first, EN+DE; `🚢️fleet-status/🧬️schema/🔣️.json`)
Facts: `cas`, `offload`, `fleet.retention`; source ids `cas`, `offload`. Report: `cas`, `retention` (null = server does not report), `offload`. Policy readers `maximumLeaseStats` 20000, `maximumBuildOwners` 64. Python oracle (`🧪️tests/🐍️.py`) extended (sections + seeded generation), three golden corpus cases (`one-machine-visible-en/de`, `one-machine-unreported`).
- (a) Host CAS reader `readFleetCasV1`: cap in force, declared cap, high/low watermark, increment, lease count + bytes (live/expired, bytes null above 20000 stats), last collection (time, bytes left) from `v1/usage.json`. No object tree walked. "Used" = bytes left by the last collection (hint), not a live scan. "High water" = the high watermark; no true peak is recorded anywhere. `repo cache cas status` also prints/returns `lastCollection` (EN+DE `statusCollection`).
- (b) Retention: `readRemoteExecutionFleetFactsV1` now keeps `retention` of the cache server `/status` (R1 `StalenessReport`, validated against `RetentionFact`); report: mode, age of last sweep, entries, judged (=declared), stale (+by reason), unreachable, undeclared (=entries-declared), retired, publish refusals. Nothing recomputed.
- (c) Offload `readFleetOffloadV1`: held compile slots (local) vs light slots (on the fleet) grouped by parent pid (the build), plus out/refused/fellBack and per-reason counters. O8 deadline / lost-operation / silent fallbacks had no counter (all `infrastructure`): `🚀️offload/🟦️.ts` now also bumps reason `deadline`, `operation-lost`, `silent`.

## Tests (repo test-file, one at a time)
fleet-status: 17 pass / 1 fail (storage reader, 120 s timeout while host load was 60; spawns hdiutil, not touched by X1). New tests: visibility numbers/texts both languages + mutations rejected by both validators; offload + CAS readers on sandboxes. cas: 49/0. No help fixtures touched (no synopsis change).

## Live `repo fleet status` (new sections, EN)
Host store: 26.2 of 27.8 GiB used (94 %), cap 32.0 GiB, high water 25.0 GiB, low water 20.8 GiB
  input leases: 0 live (0.0 GiB), 0 expired (0.0 GiB); last collection 85 s ago, 26.2 GiB left
Fleet cache retention: no report from the cache host
Offload: 0 running locally, 0 on the fleet; 1749 offloaded, 883 refused, 2273 fell back (0 deadline, 0 lost operation, 0 silent)
DE: Host-Speicher / Eingabe-Leases / Flottencache-Aufbewahrung: keine Meldung des Cache-Hosts / Auslagerung ... as expected. Leases and retention stay empty until the cache sidecar restarts on CAS1/R1 code; deadline counters until new wrappers run.

## Open
`repo cache status` ledger view has no CAS section (cas status is the home). Per-build split needs `ps` (skipped on Windows: grouped by holder pid).
