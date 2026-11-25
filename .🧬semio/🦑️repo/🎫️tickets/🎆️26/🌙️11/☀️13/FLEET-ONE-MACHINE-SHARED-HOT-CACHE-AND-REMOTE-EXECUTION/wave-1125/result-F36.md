# F36 result: parent heap retention of the whole read (19:45)

## Verdict: retainer NOT yet named; no flatness fix landed. What is proven, what is ruled out, what landed.
### Measured (from the dead read's log, `.cache/f/graph-server/read.log`, old code)
- `[graph-wave]` parentMegabytes: 2871, 3540, 3579, 4150, 4451 (first pass, +300-570 MB per wave); new pass 3073, 3182, 3526, 3816, 3953 (+110-350 MB per wave of 57 present owners, no inference); later 4665, 5915, 6074 (done 5915, 6305, 6696). ~2.6 MB per project.
- Census after forced GC ("collected and shrunk"): 4.86 GB (22 M objects, 12 M strings) -> 5.5 GB (32 M, 17 M) -> 5.9 GB (36 M objects, 18-20 M strings, 6 M arrays, 4.4 M objects) so the growth is live data (about 2 strings per observation row), not garbage; external 1.7 -> 2.1 GB.
- Journal rows of the running read (decoded, `generations/j`): median 1,579 rows per owner and mode, mean 7.6 k, max 26 k (test owners). As 4-byte row-log ids that is 6-30 KB per owner, so `carry.upserts/append` ids are NOT the 2.6 MB; only row OBJECTS (about 0.5 KB each, 2 strings) would be.
### Ruled out by code reading
- Taken memo (`createTakenMemoV1`): only created by `settleOwnerRegistryV1`, which no production caller uses; `discoverBuildGraphV1` is called without it. Not in the read.
- `inferenceRowCountsV1` closures: forgotten in the owner `finally`. Release between waves (`releaseInferenceMemoryV1` + interner clear + collect + shrink) covers rowTable, verifiedBlocks, settledReads, images, publications.
- `carry.disposers` pin the closure scope of each wave's `discoverBuildGraphPartV1` (stubs of ALL owners, `stubs` entries, `previousOwners`, `nodeRows`, `derivedFacts`): real but about 5-15 MB per wave by size, not 110-350 MB; candidate for a later fix (dispose should drop the references it does not need).
- Rows of tagged (test/policy) owners are plain arrays (`tagRows`, `testNodeRows` = own + testRows, 26 k rows each) kept in `carry.upserts` until the seal: only object-holding retention found on the seal path; candidate if the census names `upserts`.
### Why I could not pin it
- No whole read ran after 19:10 (`serve-state` "reading none", the 19:10 read is gone), a scratch whole read is not possible without claiming the shared graph image, and the diagnostics of the whole read were blind: the collection census (`observation:*`, interner, closure, owner-facts) was registered ONLY in the slice-pool worker; the `retained` line was only printed on `release({more:true})`, which never ran (zero such lines in read.log). Slice workers (same inference code) report with F19 in place: heapUsed 2.0-2.5 GB at 643 owners, 13-17 M objects, `owner-facts` memo estimate 1.5 GB (the estimator JSON-sizes entries and may count paged facts, so unreliable).
### Landed (T/landings.md "Wave 1125 F36")
- The whole read now registers the observation, interner and closure collections and, after every published/wave settling, prints `graph: after the wave managed X MB, resident Y MB; largest collections: name entries MB, ...` (8 largest, EN+DE) next to the existing census line. Next read names the retainer per wave.
### Next live read: how to read it
- Compare the `largest collections` line across waves: the entry whose MB grows ~100-300 per wave is the retainer (candidates: `observation:scopes`, `part:nodeRows`, `owners`, `graph:ownerFactsMemo`). Expected parent curve with F19 + farm minimum 1: baseline 2.7-3.1 GB at wave 1, still +100-350 MB/wave until the named collection is fixed; do not expect flat yet.
- Then fix by the evidence: stream the wave rows to the journal and replace `carry.upserts/append` entries by journal-backed lists (the resume path already does this with `log.append`), drop the disposer scopes, page the named collection. Byte-identical sealed output must be checked with the existing `flat-memory`/`journal` tests plus a before/after seal digest.
### Not verified
- No before/after RSS numbers (no read available); the landing is diagnostics only, tested by one new test.
