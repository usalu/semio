# F19 result: paged owner facts memo, streamed admission memo read

## Landed (area graph, B = build library)
- `📥️load/📚️owner-facts-pages` (new): every memo entry is written once to an append-only page file of the process (tmpdir, unlinked on POSIX, removed on close/exit); resident facts are bounded by a character budget, least recently used leave the heap; a paged entry is read back on use. Budget = recycle heap threshold (`SEMIO_SLICE_RECYCLE_HEAP_MB`, else policy 2048) / `ownerFactsResidentHeapDivisor` 16 / `ownerFactsHeapBytesPerCharacter` 8 = 16.7 M characters (about 128 MB heap). A write failure or full page file keeps facts resident (the cache stays correct).
- `📥️load/🟦️.ts`: memo entries admitted through the pages; the hit path reads facts through them (unreadable page = miss); `selectedAdmissionFactsV1.entries` yields lazy `facts` getters, new `suffixes`; packing and writing the admission memo therefore never holds all facts.
- `🧠️admission-memo`: `readAdmissionMemoV1` reads as a stream (new `🌊️json-stream`): header validated and a foreign format refused before observations are read; observations held, owners streamed into the pages, file closed via `close()`. `ownersOfAdmissionMemoV1` is an async generator.
- Schema: 3 new Limits with EN/DE texts. Tests: pages 6, json-stream 3 (cross-checked against JSON.parse at chunk sizes 1..1 MiB), admission-memo 14, plus updated bindings in repair-rounds, stale-facts, seal-under-churn, inventory (all green; see landings.md).

## Measured (this Mac, load 31-44; real memo 113 MB, 67 owners, 90,850 observations; script `T/🧰️scripts/F19/measure.ts` runs read, seed, pack, write through the real load module; "before" = copy of the pre-change files)
| | before | after |
|---|---|---|
| live heap retained after seed (post-GC) | 795 MB | 195 MB |
| retained after write | 442 MB | 201 MB |
| peak during read | 482 MB | 104 MB |
| peak overall (uncollected garbage included) | 949-1269 MB | 671-809 MB |
| time | 2.8 s | 4.3 s |
Output memo sha256 f412ae4295489d24... identical before, after and to the input file (byte-identical).

## Not verified / limits
- Not a full slice read (a cold 40-owner read costs ~20 min under this load); the harness exercises exactly the memo retention and admission memo read/write.
- The pass result (closed admission facts) still holds facts per pass; only the memo copy is paged. If handovers persist, that is the next retention. The remaining 195 MB is mostly observations/rows (needed for hold checks).
- Expected on workers: facts retention about 130 MB instead of 0.6-1.4 GB; cost about +1.5 s per 67 owners (stringify and page write), up to ~15 s for 643.
- g1-m10: grep of `ready-log-workspace.txt` for "recycl" found no lines at 17:33; the new code is not yet observed on a worker (supervisors restart on replication). Re-check the handover lines after rollout.
- Lock `landing.lock.graph` was taken after the edits (small, compile-atomic; tests run before release). `dangling-imports.py` exits 0 on the new/changed folders (it flags two unrelated test files of peers: wiring/source-demand strings).
