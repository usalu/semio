# H73: agent lanes seed from the fleet cache before compiling

Landed 13:00 (slot, locks caching-cargo + process, `🧰️scripts/H73/land.py`, landings.md entry). `repo typecheck` 51 files 0 errors, dangling-imports 0, new tests 5/0 (`🛰️fleet/🌱️seed/🧪️tests`, Python recomputation of fingerprint/closure id, real in-process cache server).

## What exists now
- Closure record per request: fingerprint of the planned unit graph (no path/edit/key in it) -> list of unit slots, stored in the fleet cache under one key per version, listed newest first via the slot of the request, published after a successful run, only grows (cap 32768).
- On a cold lane (new or reused for other crates) the launcher reads the record and prefetches, 8 slots at a time (cancel via signal, 180 s deadline, 4 GiB cap, 128 MiB per blob, progress line every 2 s): unit manifest into the local store, newest fleet entry per slot (action result + missing blobs) into `<lane>/seed`. The wrapper's fleet client answers from there (digest-verified) or from the local object store; everything else goes to the fleet as before. Warm lanes only read/grow the record.
- Per run one en/de line: units, local hits, fleet fetched (served from seed), compiled, offloaded, build scripts served.
- Item 2 (lookup before any compile): verified by reading the wrapper: build scripts (compile + run) and proc-macros go through the same lookup; only compiler queries, native-link without key inputs and unsupported crate types bypass. No change needed.

## Measurement (`repo cargo --store=shared --configuration=prototype --fresh-lane -- check -p semio-framework-repo-dashboard-terminal`, 177 units, coordinator Mac load 25-35, local entries of all 172 crates forgotten before each cold run)
| phase | wall | local hit | fleet hit | compiled | offloaded | build scripts served |
|---|---|---|---|---|---|---|
| warm (first ever `check` run, nothing in fleet) | 350 s | 0 | 0 | 177 | 0 | 0/7 |
| cold baseline (no seed) | 321 s | 0 | 54 | 122 | 1 | 0/7 |
| cold, learn run (new code, no record yet) | 493 s | 0 | 0 | 164 | 13 | 0/7 |
| cold, seeded | 517 s | 0 | 84 | 92 | 1 | 7/7 |
Seed step itself: 183 of 184 slots prefetched in 2 s (0 MiB: blobs were already in this machine's object store; the fetch is dominated by one slot request + one result request per unit, 8 parallel).

## Honest reading
- The mechanism works end to end (record learned, prefetched, build scripts now served 7/7), but wall time did NOT improve in this setup: 92 units still compiled because the fleet did not hold them yet when asked (units published by the previous run were not all uploaded: the learn run itself compiled 164 and 13 went to other machines). Wall is dominated by those compiles under load, not by lookups.
- The cold-agent gain needs a fleet that holds the closure (uploads settled). A clean A/B needs the fleet filled first and an empty object store (`SEMIO_CAS_ROOT`); not done.
- "served from seed" (153) counts distinct action results the wrappers read from the seed, more than fleet hits (84): some were read and then not used (key/blob mismatch or local entry). Treat as upper bound.
- Why the fleet lacked the units (upload spool-full / upload-expired, see brief live state) is the larger lever for the 30-60 min cold builds than lookup latency.

## Unverified
Fresh-lane removal after the call; hit rate on an agent machine with empty object store; the closure record on the live server beyond this one machine (it was read back correctly here); `repo session open --agent` itself does not seed (no request known at open; the first cargo command per lane does).
