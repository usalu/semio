# Pass restart after a killed unit recompiles everything (2026-11-24)

Evidence: `🗑️generated/native-component-flow-hub-12.txt`, `-13.txt`, `-14.txt`, the unit cache lookup log `.cache/b/r/m/u/y/lookups*.jsonl`, the event logs `.cache/b/r/m/u/x/*.jsonl`.

## What the logs show

- A restart is a new `repo run` with a new warm run directory (`components/runs/<id>`), but the warm generation compiles into the shared target of the lane the cache pool hands out (`generation.target`/`generation.build` are the lane's, `nativeBuildGenerationV1`). The run directory only holds report, logs, jobserver and stage work, so it is not the cause.
- Every restart of the pass compiled in a different pool lane. hub-12 passes 1..5: `p703de7`, `pe08ef7`, `p052291`, `pa65c05`, `pf32362`; hub-13: `p2d843c` then a fresh one. None of the earlier lanes exists any more (`.cache/b/r/m/u/l` holds two lanes).
- Reuse follows the lane, not the store. Phase ledger (`unit:restore-local` / `unit:compile`) per pass of the same hub component, 580 units:
  - pass in a fresh pool lane: 76/504, 77/503, 134/446, 76/504 (hub-12, hub-13 pass 2); every unit `fresh:false`, hub-13 pass 1 reported `0 reused` throughout.
  - pass in the lane the previous pass used (hub-13 pass 3 after pass 2, product lane `qda3ae2`): 492 restored, 88 compiled.
  - per lane event counts of the wrapper (`x/<lane>.jsonl`): each fresh pool lane has about 800 to 1250 `miss`/`publish` and under 100 `hit`; the long-lived product lane has 644 `hit`.
- The 76 restored units are the same every time (third party registry units that do not depend on lane local state); the 504 are the workspace crates and everything above them.
- The `[units] ... (1763 stored, 97 not shared)` and `311 not shared` counters are the fleet upload decisions (`countSkipV1`: `upload-failed`, `key-incomplete`, `path-dependent`, ...). They do not influence local restore; the units are in the local store.

## Why the lane is lost

1. The pool bounds the idle lanes to `maximumLanes` (16) and to `laneShare` (0.234375) of the compiler image: 22.5 GiB of 96 GiB. One hub or procedural lane is 16 GiB, so two idle big lanes are over the quota. `planPoolAcquireV1` takes the least recently used free lane when the pool is over its bounds and `trimPoolV1` removes the idle lanes over the bounds on every acquisition; the collector removes idle lanes first under free space pressure ("freeing 7.6 GiB of idle lanes, incremental sessions and cold units", image at 0.6 GiB free).
2. After the kill, the failed pass releases its lane. Before the restart enters it again, the restart reads the project graph slice and waits for host capacity (45 s to several minutes in the logs), and the other agents' commands acquire lanes meanwhile. The just released lane is the only unclaimed one, so it is the victim of the trim, or it is taken as the least recently used lane by another crate set (the crate set key changes, the lane is no longer warm for the hub).
3. The restart then gets a new empty lane: no Cargo fingerprints (so every unit is `fresh:false` and runs through the wrapper), no incremental state, no identity stamps. The units that were finished by the killed pass are only found again through the unit store, and that only works for units whose inputs did not change; with peers editing foundational crates at the same time (`Descriptor emitter is stale (5 changed source file(s), first: ...🌱️value/🦀️.rs)` at the start of each restarted pass) all dependents miss as well. Held in the lane they are `fresh:true` for Cargo and need neither the store nor a restore.

## Fix

A pass that the host killed a unit of holds its lane for the restart:

- `laneRetainedV1(id,lastUsedMs,nowMs,holdUntilMs?)`: a lane with a hold that has not expired is retained, so neither the pool's trim, the collector's idle age nor its lane share removes it; only free space pressure does, after every other idle lane (the existing rule for retained product lanes).
- `holdLaneV1(root,id,nowMs)` records `holdUntilMs = now + pool.restartHoldMilliseconds` (30 min, authored in `🧬️schema/🔣️.json`) in the lane metadata; entering the lane (`enterLaneV1`) writes fresh metadata and so clears the hold. `holdLaneOfBuildDirectoryV1(cacheRoot,build,nowMs)` does it for the driver from the build directory of the generation and refuses every other directory (default lane `d`, fresh generation, target directory).
- `PoolLaneV1.held`: the pool takes the held lane of the same crate set as the warm lane whatever the bounds say, never counts it against the bounds and never takes it as the least recently used lane of another crate set (`planPoolAcquireV1`, `acquirePooledLaneV1`, `readPoolV1`).
- The native build driver (`🧩️component-pass/🟦️.ts`) holds `generation.build` right before it throws `UnitKilledErrorV1`, i.e. exactly when the outermost command restarts the pass with exit code 75.

## Not fixed (observations)

- Peers' edits to foundational crates between two passes legitimately invalidate their dependents; the unit store cannot restore those. `rlib` externs are keyed by content hash, so an edit that leaves the compiled bytes unchanged does not cascade.
- The capacity guard evicted 1014 and 378 unit entries at 07:16 and 07:25 UTC (`y/evictions.jsonl`, `capacity`/`guard-background`) and `e/` holds 1890 entries in total; on this image (0.6 GiB free) the store itself is also under pressure.

## Tests

- `🧵️lane/🏊️pool/🧪️tests/⏳️restart-hold/` (TypeScript over `🔣️.json` and an independent Python oracle): corpus verdicts for retained/trim/acquire, a real pool in a temp cache root where a peer crate set arrives between the kill and the restart (without the hold the restart does not get the warm lane, with it it does), expiry and clearing of the hold, collector view.
