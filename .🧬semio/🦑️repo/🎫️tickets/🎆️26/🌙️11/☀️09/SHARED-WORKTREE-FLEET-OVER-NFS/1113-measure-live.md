# Live Measurements on g1-server (2026-11-13, about 10:05–10:40)

Raw outputs: `🧰️scripts/1113-measure/` (`samples.csv`, `status.out`, `verify.out`, `fleet.json`, `compaction-plan.out`, `probe-*.jsonl`). 273 000 files, five workers down, so no apply-side numbers from other machines.

| Measure | Value |
|---|---|
| Daemon CPU at rest (31 samples, 5 min) | min 0.4 %, median 12.4 %, max 95.0 % |
| Daemon RSS | min 165 MiB, median 364 MiB, max 1290 MiB, ending near 800 MiB |
| Supervisor | 0.0 % CPU, 17–20 MiB |
| `status` | 0.99 s, exit 0 |
| `verify` | 44.5 s, exit 0 (one path refused as too long) |
| `fleet --json` | 0.22 s, exit 1 (6 of 7 origins without readable attestation) |
| `compaction --plan` | 7.1 s; would remove 66 segments (94.4 MiB) and 3016 blobs (364.4 MiB) |
| Publish latency, plain write (10 probes) | p50 354 ms, p95 3698 ms, max 6272 ms |
| Publish latency, with hook-style hint (10 probes) | p50 130 ms, p95 448 ms, max 569 ms |
| Delete latency (10 probes) | 530 ms |
| Attestation (second read) | publish p50 294 / p95 2453 ms; apply p50 1669 / p95 4064 ms (3 samples) |

Abnormal: `.cache/f/beat.json` was missing; `health.json` showed `refused` failures (count 50) after 10:05; `status` reports the newest fleet snapshot (by g1-m02) as 23 754 s old. Probes under `🗑️generated/` never publish: the folder is gitignored.
