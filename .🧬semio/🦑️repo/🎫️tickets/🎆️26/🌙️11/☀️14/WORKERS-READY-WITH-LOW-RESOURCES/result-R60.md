# R60: why a stopped graph read starts over, and what now keeps finished owners

Scripts: `T/🧰️scripts/R60/` (land.py, rehearse.py, harness.ts, validity.mjs, units.mjs). Landing entry: `T/landings.md` "Package R60 landing". T = FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION (11/13).

## 1. Truth (measured)

- **"2745 of 5101 owners finished" is a misreading.** The counter `build:graph-phase:owners:<done>:<total>:<running>:<oldest seconds>` of the dead read (trace 8889.log, 18:59:04) is `owners:18:5101:12:2875`: 18 owners finished, 12 running, the oldest for 2875 s. 2745 and 2850 were seconds. So the dead read lost mainly in-flight heavy owners (up to 48 min, not durable until the owner ends), not 2745 finished ones.
- **Finished owners ARE durable at once and keyed by content.** `observeInferenceComputationV1` (👁️observation/🟨️.mjs ~l.949-995) writes the record by temp file + rename before the owner returns; identity = sha256([record stamp "semio.graph-record/2", null, key]); a taker checks the recorded observations against its own tree. Proven by real-process laws (below).
- **The real loss is retention.** Store `.cache/b/g/m/inference/selections/8d8f…/np`: 4774 owner units existed, 3611 of them are now empty `.records` dirs with no header (retired), 1183 headers remain (24 % of 4923 roots), ledger 562.9 MB over the 536.9 MB low-water mark. Cause: `retireBuildGraphOptionalEntriesLruV1` (⚡️caching/…/🗂️members/🔎️scan/🟦️.ts) retired whole units, least recently written first; a unit is a 7 KB record header plus ~370 KB fact packs, so every write over low-water evicted whole finished owners (headers) to make room for packs. Reproduced red on unmodified code: 10 owner units (header + 60 MB packs, 600 MB) -> a second writer of the same dataset retires 2 of 10 records (`records-first` test).
- Header write times (UTC): 1000 in 00-04h, 19 in 18h, none 05-17h except ~100. Record writes are also skipped for racy (<2 s old) or inconsistent inputs (`racy`, `current(rows)`), not the main cause here.
- Start phase: trace shows three `settle closures=7392 slow=7390 vouched=0 kept=0` passes in one read (747 s, 300 s, 129 s; ask 588->207->65 s): flushed chunks of the derived store are reused across passes, the manifest shortcut never fires (every closure intersects the dirty summary). NOT fixed.
- `pending.json` is only the priority steering file, not owner persistence.

## 2. Fix (landed)

1. Two-tier retirement: first the accelerator packs (`*.records/*`, `mr/*`) of the oldest units, only then whole units. Finished owner records now outlive packs. Report gains `shedUnits`, `shedBytes`; export `optionalAcceleratorV1`.
2. `📋️present` member: one directory listing gives "owner records present N of M" (real store: 1180 of 4923 in 119 ms), printed per language as `[owners] ...` before the wave; owners with a record start first inside a cost class (`orderOwnerJobsV1` `present`). Owner key function now lives there (`ownerRecordKeyV1`).

## 3. Laws
- scan/lru (6, red before: 3 failing on old code): headers survive, Python oracle equality on 120 seeded groups incl. shed/retired split.
- `records-first` (integrated, real writer/ledger): red before (8 of 10), green after.
- `stopped-wave` (real processes, 5): SIGINT after k -> next read infers exactly n-k; SIGKILL inside owner k+1 -> n-k; edit of one input -> that owner + its dependent only; restamp of identical bytes -> none; truncated/garbage/leftover tmp record -> exactly its owner; record digest equals an independent Python recomputation, no session/host text in records. These were green already on old code (durability and keying were never broken).
- `present` (4), `slow-last` ordering, owner-pool (R49, other machine's record taken), standing preflight: green. optional-cache has 3 red tests that are red on the untouched tree too.

## 4. Real server: UNVERIFIED
Landed 19:5x UTC; the running read (runner 12800, started 19:31) loaded the old code, no `[owners]` line yet, np headers still 1183. Verify on the next read after restart: `[owners] owner records present` in read.log, np header count growing, inferred = M - present - fleet takes. Not done: batched fleet existence check, intra-owner checkpointing of 40-min owners, start-phase manifest shortcut.
