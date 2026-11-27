# H2: unit-cache-denied (m11, m12) and unit-cache-read-only (g1-server)

## Findings
- Credentials are fine. All 16 origins have a registration in `S/origins/<o>/fleet-auth.json`, each pinned in the server's `pins.json` with a matching key. The server `/status` shows fleet.origins ok=16 and only 3 refusals this session (before-start 1, clock 2).
- m11/m12 write to the cache now: `provenance.jsonl` shows their puts 2 min old (3176 and 4561 recent). g1-server also writes (19823).
- The counts were history. `upload-expired-denied` (448/409) and `read-only` (5053) did not grow across two readings 100 s apart. On g1-server the `read-only` file last grew 2026-11-24 14:55, before the facts-derived access fix. Facts say `access: read-write`.
- Root cause of the standing alarm: skip counters are append-only since the first count (about 12 days), and `repo fleet status` fired on any count > 0, so it never cleared.
- g1-server decision: it must write (it is the seed origin and holds the `admin` role). The read-only default only applies when neither env nor facts name an access; it is correct as a safe fallback.

## Changes
- `🚢️fleet-status`: findings `unit-cache-denied`/`unit-cache-read-only` fire only if the reason grew within `findings.skipGrowthSeconds` (900, in the schema policy). The text no longer claims a provisioning error and names the real causes (clock, registration, server restart). EN and DE.
- `🌐️remote-execution/🚢️status`: `readSkipGrowthV1` compares with the previous reading in `.cache/f/fleet-skip-baseline.json` (atomic write) and yields `cacheSkipGrewAgoSeconds`. No wire or scheduler change. The first sight of a reason has no known growth, so it shows after the second reading.
- `👷️agent`: the 30 s loop now re-runs `syncFleetIdentityV1`, so a missing or stale registration in the store is re-published within 30 s (before: only at agent start). Zero-touch self-heal for a worker whose registration is missing.
- Oracle (`🐍️.py`), fixture and test updated for growth.

## Proof
- `repo fleet status --json` twice: no `unit-cache-*` findings, all sources ok.
- Raw counters unchanged across readings (m11 448, m12 409, server 5053).
- `build-fleet-status` suite: 31 pass. `repo typecheck` on the 3 touched files plus 11 importers: 0 errors.

## Unverified
- Why m11/m12 were denied historically: only counters remain; the cause is probably a server restart (before-start) or a clock skew.
- The agent re-sync is not exercised live (the running agents need a restart to load it).
- A real new denial would need a second reading to show up.
