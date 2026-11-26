# F56 result: pid-only liveness baseline 63 files / 68 sites -> 1 file / 1 site

Landings (7 areas, each under its lock, atomic writes via `T/🧰️scripts/F56/lib.py`, tests, dangling-imports exit 0, shrunk baseline) are in `T/landings.md` ("Wave 1125 F56 ...").

## Twin
`🔖️identity/🟨️.mjs` is a plain-JS twin of the TS module (same functions, incl. the new `processRecordAliveV1(pid, startMs?)`: record liveness, no startMs = pid-only like F45, invalid pid = dead). `🧪️tests/👯️twin/🟦️.ts` (5/0) runs both on the same vectors (records, verdicts, dead/alive with injected reads, ps text parsing, own identity, a real child before/after death), so there is one definition.

## Conversion
- Persisted records that gained `startMs` (optional in the schema, written from the own identity, read with `processRecordAliveV1`, old records stay pid-only): host-capacity Grant, Waiter and Ask (schemas, types, `holderStartV1`), lease registry entry, owner-pool guard, generation file-lease and claim lock, owner-registry lock, change-journal lock, semio store lock token, worker `.owner`. Door-board holders are judged by pid + their existing `startedMs`.
- `processPidExistsV1` (the single named raw check, TS or twin) for pids that are a spawned child, a parent/env-handed owner, a pid in a file name, or a record that already has its own start check (warden lock string, ready lease/stage via `startOf`, sandbox owner `start`).
- Tests (one at a time, repo test-file) all green except two failures that are not caused here: leases `💓️heartbeat` (test calls a missing `leaseRegistryV1.heartbeat`) and sandbox 2/21 ("Short test sandbox root required": agent scratch tmpdir too long). One owner-pool run hit the 180 s timeout under load, green on rerun.

## Baseline
Before: 63 files / 68 sites (plus one raw check a peer added in registry-consistency, converted). After: 1 file / 1 site, `🔖️identity/🧪️tests/🔒️law/🔣️.json` -> `🥾️entry/🟨️.mjs` with a `reason`: it is installed standalone as `boot.mjs` (nothing can be imported there) and only watches its own parent, no holder record. An inlined twin would be needed there; not done.

## Not done / not verified
- Sites on `processPidExistsV1` that are still pid-only by record shape: SQLite sealed-partition pins (no startMs column), replica health file pid (cursor, replicaHealthy), bazel waiter/server pids, take claims (their `startMs` field is the claim time), env-handed owners, teaching/hub/os-dev leases, funding registry port, transaction lease record (closed schema), run-lease. Follow-up: add `startMs` to those records.
- No direct test files for: semio store lock, cache-gc runner, doctor ports, sealed retention, os-dev lease/local-hub, hub, teaching cast (syntax checked with `bun build --no-bundle` only). No new startMs test for registry, worker `.owner`, guard, claims (existing suites pass); asks has a new one.
- Not run on Windows; `ownProcessIdentityV1` costs one powershell spawn per process there.
- Live daemons: readers fall back to pid-only for records without startMs; writers rewrite on next start.
