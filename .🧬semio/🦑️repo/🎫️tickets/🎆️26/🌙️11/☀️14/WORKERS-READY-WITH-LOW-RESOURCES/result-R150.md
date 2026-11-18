# R150: the `tools` stage input path made fast by construction (living report)

Status: LANDED 2026-11-17 20:54:50 to 20:55:22 UTC (lock `landing.lock` owner claude-R150, hold 32 s; a first try 80 s earlier was restored on one red, see section 5). Services and reads run old code until restarted. Scripts: `T/🧰️scripts/R150/` (`edit.py` anchored edit, `land.sh`, `bench-fs.ts`, `bench-chunks.ts`, `bench-control.test.ts`, `run-bench.sh`).

## 1. Where the chunk loops and checkpoints are

The "Immutable input physical work" counter is `check` in `stageNativePlatformProtectedArtifactInputsV1` / `...GroupV1` (`…/📤️publication/🌐️platform/🟦️.ts`). One staged input is walked by three chunk loops, all stepping 65536 base64 characters (48 KiB) from the protocol constant `chunkBytes`:
1. pre-hash `verifyProtectedArtifactInputBytesV1` / `...GroupBytesV1` (decode, canonical re-encode, hash, one progress line and one yield per step; no file access);
2. the write loop (`file.write` of the filesystem layer, one `check`, one progress line, one yield, one `check` per step);
3. the read-back `verifyProtectedArtifactInputFilesV1` (`file.read`, two `check`, one progress line, one yield per step).

The real cause of the 64 KB wall is lower: the descriptor layer refused any write over `RelativeCreateProtocol.maximumWriteBytes` = 65536 and any file read over `RelativeProtocol.maximumReadBytes` = 65536 (`📁️descriptor/🧬️schema/🔣️.json`). So the stage could not use a bigger step even if it wanted to. Every write of the layer also does `stat()` (extent proof) and two guards, which on Windows is a fan-out of facts re-reads (16 checkpoint sites in the Windows layer against 5 in the POSIX layer, R146). The same ceiling sat in the dependency generation store (`relativeWriteCeilingBytes=65536`, `dependencies:workspace-edges`, one checkpoint and one yield per 64 KB).

## 2. Measurements on this Mac (load average 55 to 70, 10 cores, 32 GB; speed factor 1)

Real descriptor layer, 168 MB (21 files of 8 MiB), write then read back, digest checked (`bench-chunks.ts`, counting wrapper at the file interface):

| | old step 48 KiB | derived step 6 MiB |
|---|---|---|
| steps (progress lines, yields, checks) | 7182 | 84 |
| file interface calls per MB (write, read, sync, open) | 21.6 | 0.44 |
| checkpoints of the layer per MB | 135 | 14 (the rest is the per-file create/open/sync, 21 files) |
| ms per step | 0.08 | 5.9 |
| MB/s write plus read back | 580 | 675 |

Where the time goes on the Mac: a step costs 0.08 ms of which the stage-level parts are tiny (control `publish` 29 µs, `yieldContinuation` 4 µs, `checkResourceLeaseControl` 2 µs, `bench-control.test.ts`, at load 64). The POSIX layer does 10.4 checkpoints per write (`bench-fs.ts`, 174 to 408 MB/s). So **the Mac never showed the wall**: 580 MB/s with the old step. The 3 to 4 s per step on g1-m05 therefore sits in the Windows layer (NtCreateFile/WriteFile/SetFilePointerEx through FFI with facts re-read at each guard, about 40 checkpoints per step) or in the Windows progress presentation, and that is what the step count multiplies. The fix removes the multiplier (2584 steps to about 30 for the same 165 MB, per loop), it does not make one Windows call faster.

Full stage (`stageNative…` with token, lease and mounted image) could not be driven on this Mac: the existing worker test `protected artifact fresh input worker` fails before my change (`image.json` absent; its provider override no longer matches how the platform captures the maintenance parent), see section 6.

## 3. What changed

- Step derived from the work: `protectedArtifactInputChunkV1(totalBytes, freeMemory)` (read-plan, schema `ChunkPlan`): the largest power of two of base64 characters between 2^18 (192 KiB decoded) and 2^23 (6 MiB decoded) that is at most a quarter of the batch and at most a 128th of the free memory. 165 MB on a 32 GB host: 6 MiB; with 512 MB free: 4 MiB; a 1 MiB batch: 192 KiB. Used by the three loops (single and group variants, 5 sites in the platform file).
- Shared loops `copyProtectedArtifactInputV1` and `readProtectedArtifactInputV1` (read-plan) replace the two inline write loops and the read-back loop: one call of the file, one `check` before it, one progress line, one yield, one `check` after the yield per step. Progress per step and cancellation per step stay (the check before every call and after every yield). The read-back buffer is sized `min(step, file bytes)` (was 64 KiB per file, would have been 6 MiB per file of 32768).
- Descriptor: `maximumWriteBytes` 65536 -> 8388608; new `RelativeProtocol.maximumFileReadBytes` 8388608 used only by the relative `withFile` read (POSIX and Windows); `maximumReadBytes` stays 65536 for the publication paths that allocate it.
- Dependency generation store: `relativeWriteCeilingBytes` 8388608 (the ingress law ties it to the protocol); its test fixture `chunkCeiling`, sizes 8388607/8/9 added, the mid-write cancel case now writes 25 MB (it needed 3 steps, 300000 bytes is one step now).
- Streaming hash: unchanged in shape (pre-hash and read-back both stream); the digest of a batch (`inputSha256`) never depended on the step. No re-hash was added or removed. Work allowance (R146) deliberately still assumes the 48 KiB step: fewer steps only lowers the need.

## 4. Laws (new `🧮️read-plan/🧪️tests/🪣️chunks`, 10 tests; red/green)

- Step equals an independent Python model on 11 cases (`oracle.py`); step is base64 aligned, 192 KiB to 8 MiB.
- Call-count law with a counting fake, POSIX-shaped and Windows-shaped (`C:\...\f.bin`, 50 ms stat, 20 ms write, 3 ms check): file-interface calls per MB under 4, writes per MB under 1 for 64 MiB. Red: the old 48 KiB step has more than 4 calls per MB and 40 times more calls (same test, `OLD` step).
- Simulated Windows cost: 165 MB under 10 s; the old step above 60 s (same fake). This is a model of the layer, not a measurement of Windows.
- Short writes are continued, a write of nothing is refused; cancellation at the 5th check leaves exactly 4 writes and 4 steps; a run cut off at step 1, 2, n-1, n and run again from the start leaves the identical digest and the full step count.
- Real files (Mac): 8 files of 8 MiB written and read back at both the derived and the old step, every digest equals the source and an independent read; checkpoints of the layer: 911 (derived) against 17391 (old), more than 10 times; throughput at least 50 MB/s divided by the R135 speed factor (`localMachineSpeedFactorV1`), measured 784 to 1028 MB/s.
- Source law: both stage sites and the read-back use the shared loops, no inline `limits.chunkBytes` step remains in the three loops, the three progress resources are still published.
- Red on the old tree: the real-file law fails against the old descriptor with `Bounded closed relative write bytes required` (9 of 10 pass, the module functions did not exist either).
- Identical output digest: every digest above, plus the existing `inputSha256` path is untouched.

## 5. Landing (T/landings.md)

Edited (anchored script `edit.py`, each anchor present exactly once): descriptor schema and `🟦️.ts` (644379e061d8 -> new schema, f71ef79a3213 -> 51269d61c1e1), dependency generation `💾️storage/🟦️.ts`, ingress fixture `🔣️.json`, `🧬️schema/🔣️.json`, ingress test `🟦️.ts`, read-plan schema (d65ed10f14f5) and `🟦️.ts` (54f099139931 -> 56516ea2e5e1), platform `🟦️.ts` (3f8220d43f20 -> f476eb06ba5b). New: `🧮️read-plan/🧪️tests/🪣️chunks` (3 files). Under the lock (parallel): chunks 10, read-plan 8, ingress 10, descriptor 7, publication 12, lifecycle 9, errno-consensus 4, both standing preflights (2, 10), genuine-birth `fresh input reservation` 1: all green, equal to the baseline taken before. The first try (80 s earlier) was restored on the ingress law `a failed write leaves nothing behind` (its mid-write cancel needed 3 steps of 64 KB): fixed, then landed.

## 6. NOT verified / not done

- **Real Windows**: not run. Whether a 6 MiB `WriteFile` plus facts re-read is cheap on the Deck-class Windows worker, whether the 3 to 4 s per step were per call or per byte (Defender scanning), and the time of the whole `tools` stage: unknown. If the cost is per byte, this change does not help; if per call, the steps fall by 86 times.
- The whole stage through token, lease and image on this Mac: no working harness (the existing worker test is stale before and after; `fresh input complete bytes` also fails at baseline).
- **Resumability by offset and one durable commit at the end: not built.** The layer's write capability is `absent-only-exclusive` with `contiguous-only` positions and the stage demands fresh roots (`Fresh immutable input root already exists`); a killed batch cannot be continued in a file, it is restarted from the start (law above proves the digest after any cut-off). Offset resume needs a decision to add a reopen-for-append capability with prefix verification to the descriptor on both platforms. Per-file `sync` and per-directory `sync` stay (durability protocol `actual-child-flush-close-parent-flush`); for many small files those fsyncs, not the chunk steps, will dominate on Windows. A sync-once-at-end (batch fsync then verify) is the next candidate; not done.
- Progress granularity: with a 6 MiB step a slow disk (10 MB/s) shows a progress line every 0.6 s; below 1 MB/s a line every 6 s, still inside every stall window. No time-based progress added.
- Group stage and read variants are covered by the same source law and shared functions, not by a run through a group token.
- Other users of the 64 KB read/write ceilings (journals, preservation, publication) are untouched except that `maximumWriteBytes` is now 8 MiB for every new-file write.
