# H17 result: coordinator volume regrowth (measured 11-27 00:40-01:10 CET)

## Measured (du -x -d1/2 on candidate roots only)
Free: 92 GiB at start, fluctuating 75.7-88 GiB (builds), 82.1 GiB at the end; reserve 74.1 GiB. F80 left 114-123 GiB. Loss since F80 about 30-40 GiB.
| Root | Size | Note |
|---|---|---|
| ~/.cache/semio/cas (CAS) | 95-98 GiB (F80: 53) | +45 GiB: the cache server's remote-cache blobs refilled to their bound (server: referenced 34.9 GiB, bound 33.8 GiB) after the 507 episode, on top of 2 unit-cache holders (28.6 + 27.9 GiB, hard-linked, not evictable) + ~3.4 GiB graph reserve + young transients. 62k objects (49.7 GiB) younger than 1 h with link count 1 = remote blobs; 0 older than 6 h unlinked. |
| ~/.sinc/play | 135.7 GiB | one fixed 139.6 GB sparse image (.cache/b/r/fixed.dmg, 117 GiB used inside, 13 GiB free inside, live Play serve and bun processes running); inside: wasm32-wasip2 unit-cache target 82 GiB, components/runs 36 GiB. Fixed envelope, own in-image gc (fill.jsonl); cannot grow further, owner is the live Play lane: NOT touched. |
| checkout .cache | 49.7 GiB | 47.6 GiB are the 8 fixed .cache/b images (envelopes), .cache/f 1.7 GiB |
| /Users/Shared/semio-store/v1 | blobs 60 GiB (du; ~37 GiB logical, 725k small files), bootstrap 13.8, snapshots 11.6, fleet 11.5, tmp 0.0 | live replica store, no staging garbage |
| ~/Library/Caches/semio | 1.9 GiB | |
| ~/.semio/cache/wasmtime | 0.00 GiB | stays empty since F80/F82 |
| ticket 🗑️generated (older than 1 day) | < 0.5 GiB total (largest 0.25 GiB DEV-PROCESS-REACT-E2E) | not worth deleting, tickets not mine |
| /private/tmp/claude-501 | 4.4 GiB | 4.2 GiB another live session's scratch, left |

## Reclaimed
Nothing deleted: no build-system garbage exists. `repo cache gc --idle --dry-run` found 0 blobs/roots; `repo cache cas gc --unreferenced --dry-run`: 3.6 GiB unreferenced is the only candidate and the server sweep takes it itself; wasmtime, store tmp, leaked-staging all 0 (the F82 relief ran at 23:34-23:39Z and freed 76 MiB). No orphan cargo incremental sessions (target dirs sit in the fixed images). Leftover big non-build items are user data (Docker.raw 70 GiB, Zotero 22, Claude vm_bundles 11): untouched, to the user.

## Why it regrows (real cause)
Not garbage: the 507 episode starved the cache, F80/F82 made room, and the fleet backlog (spool-full 27.9k, upload-expired) refilled the server's blob bound within an hour (about 35-50 GiB/h at the peak). Steady state of the CAS is held (53) + blob bound (34) + graph (3.4) + transients, about 95-100 GiB, so a 926 GiB disk with 139 GiB Play image, 48 GiB images, 97 GiB shared store and ~100 GiB user data sits at about 75-85 GiB free: 1-10 GiB above the reserve. F82's relief rows (wasmtime, staging, leaked-staging) are correct but have nothing left to free; the only remaining lever is the content classes, i.e. the blob bound.
Observed defect: usage.json/census.json were 54 min stale while 35 GiB landed (cap-in-force and "writes refused above cap" were judged on a 61 GiB hint; `cas status` showed 60.9 GiB vs 98 GiB on disk). The tick's remeasure after relief waits behind the collection scan of 100k objects (event-loop stalls 0.5-2 s in log); the server log shows no tick line between 23:39Z and 00:56Z.

## Reserve policy (decision)
Keep the hard reserve at max(8 %, 72 GiB) = 74.1 GiB: 72 GiB = server transient 4 + 2 releases x 16 + server 24 + safety, matches declared burst need (largest observed real burst: F80's 51.6 GiB test staging + 17 GiB release). Lowering it only trades 507s for failed builds. The inconsistency to fix is the remote blob bound (33.8 GiB, fixed share) not shrinking when free approaches the reserve: it should be min(share, free - reserve - burst margin). Not changed in this lane (cache role code closure would hand over the server and the remeasure fix needs the guard test suite; both are one landing for the next lane: see findings).

## R11 (wasmtime cwasm cap + prune)
Already implemented by F82 (policy JSON cap 8 GiB, age 14 d, LRU by mtime, run interval 6 h; floor row in the volume relief; TS oracle 4 pass). The open part was the Rust compile and tests. Started `repo cargo --store=shared --configuration=prototype -- test -p semio-plugin-host-engine --lib cache_retirement` (nice 10): at report time the cargo ingress was still in source admission (unit 429/504, load 55); result below if it finished.

Update: after ~25 min the cargo run was at crate 67 of the closure (compiling, load 55), no test result yet; the run was left in its scratch log (`scratchpad/h17-cargo.txt`). Rust cache_retirement tests remain UNVERIFIED.

## Not done / unverified
- No code change, no landing, no landings.md entry (nothing to reclaim; policy fix needs a cache-role landing).
- Next lane: (1) blob bound = min(share, free - reserve - margin); (2) guard tick must remeasure (usage hint) before the collection scan of 100k objects, not after; (3) 507 rate from the new refusal log only after the next hand-over (session counter: 116 rejected of 23,690 puts = 0.5 %, down from 1.13 M).

# Round 2 (01:15-01:50)

## (1) Blob bound tracks free space
Already structurally true: `remoteCacheBlobBoundV1` = share of the cap in force, and the cap in force = min(proportional, store + free - reserve), re-read on every call. What was wrong is the store figure: it came from `usage.json`, 54 min stale, so the bound (and `cas status`) ignored 35 GiB. Fixed through (2). New test (`🌐️remote-cache/📐️blob-bound/🧪️tests`): the bound with the free space near the reserve is below the bound with 20 GiB more room, never above (store + free) / 2, and a fresh measurement replaces a stale hint (5 pass). Added an optional partition port to `remoteCacheBlobBoundV1` as the test seam.

## (2) Fresh usage, measured first
Schema-first: `UsageHint` gained optional `measuredBytes`/`measuredAtMs` (bilingual comment). `recordCasMeasurementV1` writes a plain walk next to the collection stamp; `readCasUsageHintV1` returns the newer one (without the collection credit, so the cooldown rule is unchanged), so guard, blob bound, `cas status` and fleet status all read it. `guard.remeasure` records every walk; `casEnforceTickV1` now remeasures a stale hint (older than 2 sweep periods) BEFORE the volume relief and the sweep, and not again afterwards. The earlier flaw: the walk's result lived only in process memory, so the file stayed stale and every reader judged on it.
Tests: volume-relief 5 pass (new: order measure -> relieve, hint file holds the walk), cas 72 pass, remote-cache server 10 pass, blob-bound 5 pass; `repo typecheck` of the 5 touched files: passed, 0 errors (473 s); dangling imports: exit 0. Landing: `landings.md` entry written; locks cas + remote-cache released. Landed at about 01:20-01:35, outside the :00-:10 slot (the addendum reached me at about 01:37); the sidecar picks it up at its next hand-over (3 h policy).
Not done: incremental put/delete deltas (the guard already adds put bytes via `account`; deletes by the server's collector are not subtracted, so the in-process base only drifts up between walks, never down).

## (3) Duplicate blobs between the two holders and the CAS
None. The unit caches keep only indexes in their images (main `.cache/b/r/m/u`: 1.1 GiB total, `e`/`r` entries reference `outputs.*.sha256`); the blobs are CAS objects that the holder keeps by hard link in the CAS (`o/xx/...` link count 2, `k/<holder>`). Census: holder A 32,906 objects, holder B 15,485 objects, sum 48,391 = the held class exactly, so one physical copy per object and the 28 + 28 GiB are not two copies of one blob. Hard links cannot cross to the unit-cache image volumes, and nothing needs to: no blob lives there. The big `u/l` (90 GiB) in the Play image is its cargo target dir, not unit blobs. Clone/hard-link dedupe would save nothing.

## R11 Rust test
Run 1 (`repo cargo --store=shared --configuration=prototype -- test -p semio-plugin-host-engine --lib cache_retirement`) was cancelled with SIGTERM from an owned execution at about 62 crates compiled (not a test failure). Run 2 restarted and, at report time, was still in shared source admission behind the bun tool (500 s, load 55); no compile result. R11 Rust tests: STILL UNVERIFIED. Log: `scratchpad/h17-cargo2.txt`.
