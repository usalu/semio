# Lane capacity: 16 GiB lanes that fill, get collected and are born again at 16 GiB

## Evidence (runs 34 to 39)
- Ledger `.cache/b/r/shards.jsonl`: `pf943c0` born 09:04 at 16 GiB, `retired pf943c0` 11:03:13.872, `born p7c9473` 11:03:13.888 (16 GiB). A birth event is stamped *before* `hdiutil create`, so the 16 ms is the birth process waiting on the ledger lock that the retirement held. The retirement came first, from the same acquisition (or its predecessor's collector).
- Run 38 log: `[lane] image of lane pae5af0: refused (commitment)`. A lane the host-wide commitment ceiling had no room for stays in the home image `rust`, which is full. Run 39 then logs `image of lane p7c9473: birth`.
- `p7c9473/lane.size`: 16.1 GB, of which 7.3 GB rustc temporaries. `df` of its mount: 16 GiB, 100 %. Host 60 GB free: the host was never the constraint, the declared size was.
- Flat 16 GiB = declared root need 8 GiB x (100 + 100 percent headroom). Evidence existed only while a lane existed: every retirement discarded `lane.size`, so the next birth saw no measurement and fell back to 16 GiB again.

## Root causes
1. **Sizing had no memory.** `laneSizesV1`/`laneMeasuredNeedBytesV1` read `lane.size` of lanes that exist. A lane that is retired (or removed by the collector) takes its evidence with it; the successor is again born at the declared 16 GiB, fills, is collected, and so on. No run could ever get further than the one before.
2. **Self-defeating collection.** `planGcV1` answers image pressure by removing idle lanes, a retained (recent) lane last. A lane that lives in its own image and fills that image is the only content and the only candidate: pressure removed it at the end of the run (or when the next run's headroom demand arrived), its image was retired by `retireIdleLaneImagesV1` (attached, no `lane.json`) and a new lane was born empty. Deleting the lane freed nothing the next build did not need again immediately.
3. **Admission could pick the requester's own lane.** `admitNewLaneV1` retired the least recently used idle lane without regard for whether the requester (same session, agent and crate key) was about to reuse it.
4. **Commitment budget counted only declarations.** Sparse images of idle (dead-session) lanes counted their full declared capacity against the lane budget, so a live lane could be refused (`refused (commitment)`) and then lived in the full home image.
5. **A need above the cap was refused instead of capped.** `nativeImageLaneSizingV1` fed the birth gate `p95 x headroom`; for a need above about 21 GiB that is above the 32 GiB macOS cap, so the birth was refused (`cap`) and the lane again fell into the home image. Evidence-based sizing exposed it (the old lane before had held 21+ GiB).
6. **A full lane gave no typed answer.** After the single retry the launcher printed the generic `stillFull` text; nothing recorded that the lane needed more, nothing said whether the image would grow.

## Fix
1. Capacity from evidence and budget:
   - Peaks per crate key persisted beside the lanes (`lane.peaks`, schema `layout.peaks`): `recordLanePeakV1`, `readLanePeaksV1`; `recordLaneSizeV1` raises the peak of the lane's crate key; the peak never lowers and survives lane removal.
   - `laneSizesV1(root, crateKey)` / `laneMeasuredNeedBytesV1(root, crateKey)`: peak of the crate set plus recorded sizes of its lanes, else all lanes (as before). `laneImagePortsV1(..., crateKey)` threads the key (pool and product acquisition). A birth is sized `2 x peak` (rule headroom 50) up to the platform cap; growth of an existing image already targets the peak plus headroom at its next quiet moment.
   - `nativeImageLaneSizingV1(rule, root, samples, platform)`: the gate need is cut to what the cap holds with its headroom, so a lane that needs more than the cap is born at the cap, never refused.
   - Commitment budget counts what is in use: `laneIdleSlackBytesV1` is the declared capacity of idle shards less what they hold (or their recorded size); the budget port is `commitment + idle slack`, so idle sparse images count what they hold, a claimed or live lane counts in full.
2. No self-retirement:
   - `planGcV1`: a retained lane in an image of its own (`own`, set by `measureGcV1` from the image map) is never removed by pressure, only the cheaper incremental sets (and everything collected before the plan: temporaries, units). Other idle lanes still go first, a retained home-image lane still last.
   - `admitNewLaneV1(..., {requester})`: never retires the idle lane of the requester's owner and crate key; `acquirePooledLaneV1` passes it.
3. Full lane mid-build:
   - Existing: temporaries, incremental sessions and stale units are collected on out-of-space, then the command is retried once.
   - New typed refusal `reportLaneFullV1` (launch module, called from the launcher after the retry failed the same way): records the capacity as the crate set's peak, decides with `laneFullVerdictV1` (growth plan over peak, host keep, budget room) whether the image `grows` to N GiB when no build holds it or is `capped` (`at-cap`, `budget`, `host-short`, `unsupported`), and prints `laneFullGrows` or `laneFullCapped` (en, de). The lane and its compiled units are kept; the next run grows the image instead of starting empty.

## Not done
- A full lane cannot be stretched during the build on macOS (a detached resize needs the image quiet); the answer is the typed refusal and growth at the next acquisition.
- `p7c9473` (current, 16 GiB, full) was left alone: it is this session's lane and will grow at its next acquisition (peak 16 GiB, target 32 GiB if the commitment room allows +16 GiB). No idle lanes of other sessions existed (`s/` holds `cargo`, `p7c9473`, `sealed`, `w`).
- The build was not run, as instructed.

## Changed files
Library `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`:
- `⚡️caching/🦀️cargo/🗃️unit-cache/🧬️schema/🔣️.json` (`layout.peaks`, messages `laneFullGrows`, `laneFullCapped`)
- `⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🟦️.ts` (peaks)
- `⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🟦️.ts` (requester guard, crate key to ports)
- `⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🧩️image/🟦️.ts` (evidence sizes, idle slack budget, full verdict)
- `⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🎯️product/🟦️.ts` (crate key to ports)
- `⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts` (`own` lanes)
- `⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts` (`reportLaneFullV1`)
- `🏃️process/🦀️cargo/🚪️launcher/🟦️.ts` (typed refusal after the retry)

Framework `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/`:
- `🪜️growth/🟦️.ts` (`nativeImageLaneSizingV1` cap reach)

Tests:
- `…/🧵️lane/🏊️pool/🧩️image/🧪️tests/🟦️.ts` (peak outlives lane, capacity from evidence and cap, idle slack against budget, full verdict, messages)
- `…/🧵️lane/🏊️pool/🧪️tests/🚦️admission/🟦️.ts` (admission never retires the requester's lane)
- `…/🧹️gc/🧪️tests/🟦️.ts` (own retained lane never removed by pressure, Python oracle)
- `…/🪜️growth/🧪️tests/🟦️.ts` (need above the cap is born at the cap)

## Tests (`SEMIO_LANGUAGE=en repo test-file`, logs `🗑️generated/lane-capacity-*.log`)
- New or extended, all pass: gc 16 (own retained lane never removed by pressure, Python oracle), image 15 (peak outlives lane, capacity from evidence and cap, idle slack against budget, full verdict, messages), admission 9 (never retires the requester's lane), growth 10 (need above the cap born at the cap).
- Wider run over every test file below `unit-cache/{🧵️lane,🧹️gc,🧪️tests}` (61 files): 56 pass, 5 fail. The failures are environmental and not touched by this change: `rustup could not choose a version of cargo` (no default toolchain: 🏖️scratch, 🪟️windows, 🆔️metadata, ⏳️slow), `ENOENT lstat /$bunfs` and `registry gives one fleet key` (🌍️cross-root). The previous lane ticket listed the same classes.

# Follow-up (runs 40 to 44): lane could not grow, compiled units never reused

## Why `p7c9473` could not grow ("budget")
- Not the lane budget (rust store quota 66.7 GiB, lane root declares 16). The host-wide commitment ceiling was 118 GiB against 197 GiB of declared capacity, room 0: `d` 29, `r` 32, `cargo` 30, `w` 30, `sealed` 26, `g` 11, `p` 11, `t` 8 and the lane 16, nearly all of it sparse (blocks held: 18, 11, 2, 13, 1, 2, 0.3, 3, 16 GiB). The ceiling already counts what the images hold, but the sum counted every declaration in full, so the images that held nothing used up the room the one image that was full needed. (Host free was 36 GiB at this time, 60 earlier.)
- Fix, model: an image promises what it holds plus a bounded growth reserve (`growthReserveBytes`, 4 GiB), never more than it declares (`nativeImageCommitmentRowCostV1`; `committedBytes` is the sum of the promises, `declaredBytes` the plain sum). The image that asks to grow is admitted against what the others hold and may still grow by. With the measured rows this is about 105 GiB against 118, room for the +16 GiB.
- Schema, oracle and corpus: `🧮️commitment/🧬️schema/🔣️.json` (`growthReserveBytes`, corpus `sum`), `🧪️tests/🐍️.py` (independent promise sum), `🧪️tests/🔣️.json`, `🧪️tests/🟦️.ts` (9 pass).

## Which "compiler image" of 28 GiB is full
- It is no image. `measureVolumeV1` of a split root clips the volume to the capacity the caller passes, and every caller derives it from the home image of the issued storage: 32 GiB (`r/fixed.dmg`) less one compile workspace = 28 GiB. The used bytes of all images added up (lane 15.7, `w` 12 incl. an 11 GiB spill store, `cargo` 1.9, `sealed` 1.0, home 1.1) are 31.7 GiB, so the aggregate was permanently "full": 0.0 free of 28.0.
- Fix: `gcCapacityBytesV1` (gc): the capacity a split root is judged against is at least the sum of its images' declared capacities; used by `measureVolumeV1`, `measureGcV1`, `collectGarbageSingleFlightV1` and `watchUnitCachePressureV1`. Each image is still judged by its own water marks (`imageNeedBytes`), the host by the free bytes of the aggregate. Test: split-root assertions in `🧹️gc/🧪️tests/🟦️.ts` (16 pass).

## Why compiled units were not reused
Cause found in the evidence, not a guess:
- `x/*.jsonl` and `y/evictions.jsonl`: in 8 hours the collector evicted 6151 build units (`units: 977, 1100, 1012, 53, 590` per collection) and 20 lanes. 1314 units that were published and logged were missed again later; the number of units compiled per run (about 1000) equals the number evicted per collection.
- Both pressure sources above fed it: the false 28 GiB aggregate (stale-unit budget `unitShare` 0.5 of 28 = 14 GiB, while one lane legitimately holds 15 GiB, plus permanent `need > 0`) and the per-image pressure of the 98 % full 16 GiB lane image. Each collection deleted the finished units of the lane that was about to be reused, so Cargo recompiled them. Peer source churn and corrupted outputs are not the cause (fingerprints of the same keys matched at publish; the misses are of keys whose spill entries are gone: 690 of about 11000 published entries remain in `u/w/spill`).
- Separate, not changed: the host-wide CAS reports `27.81 GiB of 1 GiB cap in force` because the host has 36 GiB free against a 72 GiB hard reserve for role `cache-host`; its cap collapses to 1 GiB, writes are refused above it and its collector trims the unit cache's objects. That is the machine policy of the CAS (`cache cas status`), a host-level decision (reserve of 72 GiB on a 228 GiB volume), left to the owner of that policy.

## `p7c9473` grown
- Ticket script `lane-grow.ts` (framework `ensureLaneImageV1` growth, the same path every acquisition takes, with the real ports and the machine budget): `{"outcome":{"kind":"grown","detail":"34359738368"},"beforeGiB":16,"afterGiB":32}`. Mount now 32 GiB, 14 GiB used, 18 GiB free; ledger has `grown p7c9473 34359738368`.
- Note: the unit-cache watcher process 80889 (started 14:29 by a lane release, sealed modules of the old code) was still running and was left alone; it can evict units until it ends (stalls after 10 empty rounds, at most 2 h). A restarted build starts collectors with the fixed code.

## Changed files (follow-up)
- `🧮️commitment/🟦️.ts`, `🧮️commitment/🧬️schema/🔣️.json`, `🧮️commitment/🧪️tests/{🟦️.ts,🐍️.py,🔣️.json}` and the public re-export `📦️packages/🟦️typescript/…/🧮️commitment/🟦️.ts` (under `🧰️framework/🔨️modules/🏃️process/`)
- `…/unit-cache/🧹️gc/🟦️.ts`, `…/🧹️gc/🧪️tests/🟦️.ts`
- ticket: `lane-grow.ts`
Tests: commitment 9, gc 16, image 15, admission 9, shard 27: all pass.
