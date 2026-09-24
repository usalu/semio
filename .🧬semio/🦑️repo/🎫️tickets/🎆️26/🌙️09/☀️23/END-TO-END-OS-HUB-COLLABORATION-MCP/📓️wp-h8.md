# WP-H8 — Hub-Materialized Check In (checkpoint command) + Inference Flake Root Cause

Slice: H8 (session 10). Ports: hubs 7930–7939, serves 6430–6439. Private cargo target: `.tmp-ticket/wp-h8/target`. Captures: `wp-h8/generated/`.

## Status

| Item | Status |
|------|--------|
| 1. Schema-first check-in command (request/receipt) | TODO |
| 2. Hub job: materialize checkpoint from ledger, progress/cancel, authority | TODO |
| 3. Delete `checkpoint-publications` upload path + callers | TODO |
| 4. os client wiring (TS shell + Rust host) Check In → command | TODO |
| 5. Laws (head advances, stale refused, approval after edit+check-in, cold open from new checkpoint) | TODO |
| 6. Root cause `gis_map_abandoned_pre_witness…` flake | TODO |
| 7. Gates: hub quick + long, os TS tests, live proof (W1 catalog A) | TODO |

## Log
