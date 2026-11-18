# Describe Tool Retention Snapshot Slowness (2026-11-17)

## Observation
- Live `describe-tool` build (bun pid 77880) at 17-27 % CPU, no cargo/rustc child: cargo finished at 19:16 (`cargo-unit-provenance-*.json`, 4.5 MB emitter captured).
- `sample`: main thread ~50 % idle in kevent; the rest sqlite `fsync` (claims), `posix_spawn` (hdiutil), `lstat`/`openat`. Not CPU bound.
- `lsof`/directory watch: `captures/cargo-artifacts-*/q/1/` grew 3 508 -> 3 928 blobs (about 120 MB) at about 8 files/s, i.e. about 92 ms per retained file; `.cache/b/l/m/data/claims/` held about 2 300 `queued-original` claims of this pid on the gate `native-storage-mapping:<root>` (4 800 claim databases in the directory).

## Hot path
`CargoCompilationSnapshotPoolV1.snapshot` (retention/snapshot) copies every file row of the completed closure (3 928 files: sources, dep-info, artifacts). Per file it runs `storage.recheck()` three times plus `verify()` (one more). Each `storage.recheck()` -> `port.measure` + `port.recheck` -> persistent image `physical` gate with `nativeRecheckWindowV1() == 0` -> a full strict reproof: queued shared `native-storage-mapping` lease (new claim sqlite, fsyncs, queue directory) + `hdiutil info -plist` through `captureOwnedCommandV1` + managed-children roster parse. About 30 ms each, about 3 per file => 92 ms/file measured. After the snapshot, `compilation.recheck()` (called twice by the describe emitter) re-verifies all files again with the same strict recheck per file, plus the staging copy.

The existing amortisation (`withNativeRecheckWindowV1` + `createNativePhysicalGateV1`: cheap identity checks every call, full reproof at most once per window) was only opened by the cargo launcher / native helpers, never around retention, so the retention loops paid a full reproof per call.

## Fix
`retention/snapshot/🟦️.ts`: the pool opens the recheck window (default 500 ms, max 5 s by schema) around
- every pooled operation (`run`: `snapshot`, `finalize`),
- `verify()` (public per-file verification),
- the `recheck` closure returned in the completed view (the re-verification of all retained files done by consumers such as the describe emitter).
Correctness unchanged: cheap per-call identity checks still run for each file, byte hashes are still compared for each file, and the strict kernel/mapping reproof still runs at least every 500 ms.

## Test
`retention/🧪️tests/🪟️recheck-window/🟦️.ts` (bun test, real `acquireNativeStorageV1` reservation, fake port whose recheck/measure use the real `createNativePhysicalGateV1`): 200 `pool.verify` calls
- every per-file recheck observes `nativeRecheckWindowV1() > 0` (fails without the fix),
- full strict reproofs <= 1 % of rechecks (here 1 per 500 ms window) versus exactly one per call without a window (200/200).
Related suites: image roster tests 16/16 pass, size-law 1/1; retention test file 15 pass, 3 fail only because the file needs the repo invocation envelope (`Original parent Script process invocation required`, independent of this change).

## Measured / expected
- Before (live): 3 928 files in about 6 min of snapshot (about 92 ms/file) plus two further full-closure rechecks and the staging pass, which kept the job running more than 20 min after cargo finished.
- Cost components measured: `hdiutil info -plist` 17 ms, raw hashing of all 3 928 retained files 2.0 s.
- After: strict reproofs bounded to about 2/s => per-file cost is I/O only (copy + 2 hashes), about 10-20 s for the whole snapshot and about 3 s per full recheck; expected speedup about 15-30x on this phase. Not re-measured end to end because no build may be started.

## Not changed (note)
All 3 928 file rows (including `current` custody rows such as registry sources) are still copied into the retained store; the "size law" test only constrains output claims. Scoping the copy to `retained` custody would be a semantic change for the receipt consumers and needs its own decision.
