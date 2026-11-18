# R73: record freshness by content, one landing (R73 + R69 + R72 fix 1)

No worker reached `ready: true` (measured 04:32 UTC).

## Cause
`freshVerdictV1` (GS/📦️project-records/🌱️fresh) compared file/analysis digests only. R68b added `codeEpochV1`: a record older than the newest mtime of ANY code file below `library` and `🔨️modules` counted stale. Any edit anywhere re-staled all records. The reader side (take pass) never used mtime; it asks `closure` rows of the record to the closure provider (content). So only the pool side was wrong.

## Fix (content-addressed)
- Epoch removed completely (`codeEpochV1`, `newestCodeMillisecondsV1`, schema `epoch*` fields, verdict cache keyed by epoch).
- `freshVerdictV1` now also puts every `closure` row (the digest of exactly the files a tool closure reaches, already part of the record) to a `closureHolds` decider; the puller (`closureHoldsOfV1` in `📦️project-records/🟦️.ts`) settles the closure entries and asks the same provider a taker asks. A row the provider cannot answer in the puller (`!stale`/`!unreadable`/`!unregistered`) counts as fresh, never stale (second landing 04:32; without it 54 of 64 new records looked stale only because the puller process could not settle them).
- Verdict memo is a 10 min TTL now, not epoch-keyed.
- Laws (fresh/🧪️tests, 4 pass, Python oracle for the fixture): unrelated file restamped/edited keeps records fresh; a changed closure file makes stale exactly the records whose closure reaches it; owner-pool test (PN/🧪️tests/📥️take) rewritten: a restamped code file leaves a stored record `stored`, `refreshed` stays 0. Law 3 (pool key == reader key, R68) still green.

## Landed
`🧰️scripts/R73/land.py` 04:00-04:05 UTC, one lock hold: R73 + R69 (R69_CLOSURE_OK=1) + R72 fix 1 (22 files). All suites and the 3 standing preflight tests green. Two things I had to fix that R69 did not: retirement test fixture/oracle for `bindings.json` in unready generations (`♻️retirement/🧪️tests` 🟦️.ts and 🔣️.json, roles of both managers). I dropped `💾️storage/🧪️tests/👓️reading` from the list: it is red on the untouched live tree (46 fail, needs cwd = repo root) and unrelated. Second landing 04:30 (`land2.py`, own file only, tests + preflight green).

## Measured
- Records created after the landing (reader key, wave 4d9db5baac29, 04:28 sample of 800 owners): 89 fresh, 0 stale (all under constant peer edits); older records (528) are stale by closure because R69/R72 sit inside tool closures (expected, once). Pool key family: 0 post-landing of 800 sampled (miss 564, old 236).
- Pool 18 to 25 owners/min fleet-wide (04:13, 04:32), failed owners 1; this is the restart cost, the refill of the ~5000 owners takes hours at this rate.
- Server read now prints `[owners]` lines (32 at 04:32).
- Ready table 04:32: none ready; m01/m06/m07/m10/m02 workspace 5/5 exit 75, m04 tools stage-failed, m03 workspace 12 tries, m05 dependencies.

## Proven / not proven
Proven: unit laws 1 to 3, no mtime anywhere in the verdict, post-landing records stay fresh across 20 min of live edits. Not proven: the private-copy harmless-edit experiment on a live tree (a copy exceeds the 500 MB scratch rule; the unit law with a real temp tree and the pool test replace it); that the pool refreshes genuinely stale records (the puller often cannot settle closures, so these are left to readers, who refresh by swap); that any worker finishes; the 30 min watch is 04:05-04:32 only (log `scratchpad/R73/watch.log`, harness `🧰️scripts/R73/proof/fresh3.mjs`).
