# F114: A transient reader no longer aborts an image detach

## Root cause
- The failing call `hdiutil detach <repo>/.cache/b/g/m` is the retirement of the graph image (`retireNativeImageStorageV1` / `retireNativeImageStorageWithWriterV1` in `🍎️image/🟦️.ts`). A graph read starts the elastic regrow (`regrowElasticImageOutcomeV1` in the cache `♻️reset` module), which takes the exclusive image lease and then retires the image. The image was neither re-attached exclusively nor remounted read-write: it was being retired and recreated because the regrow found it due.
- The detach ran once, without retry; status 16 surfaced as `Internal error` and aborted the whole read.
- The exclusive lease was granted, so the 1 s node child held no shared lease: it read the image behind the lease. Direct readers of `<graph>/m/...` that bypass the shared lease (found by grep, not changed here): sidecar generation uploader and `progressAtMs` (`🌐️remote-execution/🛰️sidecar/🕸️graph/🟦️.ts`), head adoption (`.../📥️adoption/🟦️.ts`), `🕰️generations/🌐️remote-exchange/🟦️.ts`. Wrapping those in the shared lease is the follow-up root fix; the retry makes the detach safe meanwhile.
- The shard lifecycle (`🧩️shard`) additionally fell back to `hdiutil detach -force` on any failure, i.e. while a live reader held files.

## Change
- New `🍎️image/🔁️detach/🟦️.ts` (+ schema `🧬️schema/🔣️.json`): `detachNativeImageWithRetryV1`. Busy refusals (hdiutil 16, umount 32, Windows "used by another process"; schema-declared) are retried with the declared back-off (0.5, 1, 2, 4, 8, 15 s steps, 24 attempts). Each wait prints one bilingual progress line naming the holders (`lsof -F pc +f -- <mount>` plus shared process identity pid + start time). Cancellation (`checkResourceLeaseControl`) ends the wait within 250 ms and starts no further detach. After the last attempt a bilingual refusal names the holders. Never `-force`.
- Both retirement detach sites in `🍎️image/🟦️.ts` use the new helper `detachOwnedNativeImageV1` (owned command, lease control, cancellation).
- `🧩️shard/🟦️.ts`: the polite detach goes through the same retry; `-force` now happens only for non-busy, non-cancelled failures (a latched volume), never after a busy refusal.
- Linux/Windows: the retire path is darwin-only and shards are unsupported there; the core is platform-neutral (markers for `umount` status 32 and Windows). The idle reclaim/grow ports (device detach, `umount`, VHD) stay non-waiting by design ("a busy image refuses and nothing changes") and were not changed.

## Tests (`🔁️detach/🧪️tests/🟦️.ts`, 12 pass via `repo test-file`)
Fake port busy 3x then success (4 detaches, sleeps 500/1000/2000, 3 bilingual lines naming `node pid 4242 start ...`), never-free (24 attempts, bilingual refusal, cause kept), non-busy propagates at once, cancellation, concurrent-detach `ready`, sleep cancellation, port argv, parser vs real `lsof` oracle. Shard test file (20 pass) still green.

## Verification
- `repo typecheck` on `🔁️detach` + `🍎️image/🟦️.ts`: passed. With `🧩️shard/🟦️.ts` it exits 1 only for 5 pre-existing type errors in `🧩️shard/🧪️tests/🟦️.ts` (lines 18, 50, 101, 119, 125; not touched).
- dangling-imports: exit 0.

## Addendum: "Original Graph format identity changed" (seal blocked)
- Read-only measurement: marker `.cache/b/g/m/format.json` backing `711799973:19326345216`; live `lstat` of `fixed.dmg`: ino 711799973, size 28990021632. The inode is unchanged; only the size grew (in-place `hdiutil resize` regrow). So the hypothesis is half right: size, not inode.
- Root cause: `planRepositoryGraphFormatV1` compared the marker's `ino:size` with the live `ino:size`, so any legitimate in-place resize orphaned the store.
- Fix (`⚡️caching/.../🏪️store/📇️format/🟦️.ts`): identity is the backing inode only; size is informational. A regrow/resize keeps the inode, so no restamp is needed (nothing to race, no EX needed). A different inode (foreign or recreated file) is still refused, now bilingually with the remedy `repo-cache-reset:cache-reset-graph`. The marker schema is unchanged, so the format stamp and every stored marker stay valid. Recreated images are empty and take the existing initialize path.
- Test: new case in the format test file (size change accepted, same size accepted, other inode refused with both languages) passes. The same file's "worker role" test fails independently (closure worker AST lookup, not touched). Typecheck passed, dangling-imports 0.

## Addendum 2: "Original cache charge disagrees with global policy" after the regrow
- Sweep of size-dependent records: (1) image receipt `image.json` (already rewritten by the grow), (2) graph format marker (now inode-only identity), (3) charge receipt `<label>.json` (stale: 19326304256), (4) native journal envelope of the same allocation (stale, shrink-only `rebudgeted` event could not grow), (5) elastic declaration (source of truth, already current).
- Journal: new closed event `regrown` (schema, journal apply, `regrowNativeStorageAllocationV1` beside the rebudget). Admits only a committed artifact envelope of exact id/owner/kind/bytes/phase and sequence, growing within the capacity left; `rebudgeted` stays shrink-only (its oracle test is unchanged and green).
- Reservation (`🏪️store/🎟️reservation`): `repositoryCacheChargeStaleV1` (same owner, other bytes), `requoteRepositoryCacheChargeV1` (under `withExclusiveMaintenance` kernel-EX: journal event, then atomic receipt replace, one bilingual progress line; foreign owner/allocation refused bilingually with the remedy, nothing changed) and `openRepositoryElasticChargeV1` (open, detect, close, re-quote, reopen, verify). Wired into `openRepositoryCachePartitionAttemptV1` for graph and rust, so already-regrown stores (this machine, others) recover on the next open and later regrows are healed the same way. Products keeps its own layout reconciliation.
- Tests: `💾️storage/🧪️tests/📈️regrow` (journal event vs SQLite oracle, capacity, rebudget still shrink-only), `🎟️reservation/🧪️tests/📈️regrow` (regrow with size change, re-quote, foreign refusals, open/reopen read path), existing reservation charge-order law and rebudget oracle green. Typecheck of touched files: only 8 pre-existing errors in untouched importer tests (replacement, receiver/existing, cache retirement). dangling-imports 0.
- Not run: real image re-quote on this machine (done by the next open through the fixed path).

## Addendum 3: measured test counts (`repo test-file`, one file at a time)
| File | pass | fail |
|---|---|---|
| `💾️storage/🧪️tests/📈️regrow/🟦️.ts` (journal `regrown` vs SQLite oracle) | 2 | 0 |
| `🏪️store/🎟️reservation/🧪️tests/📈️regrow/🟦️.ts` (re-quote, refusals, read path) | 4 | 0 |
| `🏪️store/🎟️reservation/🧪️tests/🟦️.ts` (charge-order law, untouched) | 2 | 0 |
| `💾️storage/🧪️tests/📉️rebudget/🟦️.ts` (shrink-only oracle, untouched) | 4 | 0 |
| `🏪️store/📇️format/🧪️tests/🟦️.ts` (format identity incl. new inode case) | 7 | 0 |
| `🍎️image/🔁️detach/🧪️tests/🟦️.ts` (F114 detach retry) | 12 | 0 |
| `🍎️image/🧩️shard/🧪️tests/🟦️.ts` (shard lifecycle) | 20 | 0 |

Format test case 7 ("closed scanned worker role"), two root causes, both fixed:
1. `role` moved from `🧾️closures/🟦️.ts` to `🧾️closures/🏷️names/🟦️.ts` (a peer refactor); the test's AST lookup still read the old file. The lookup now reads the names module.
2. `🧾️closures/🟦️.ts` captured each closure implementation file with a 65536-byte bound; `🏗️build/🕸️graph/🕰️generations/🟨️.mjs` (peer lane adding `mapBatchesV1`/footprint gate) grew to 66227 bytes, so the whole file failed at load ("Bounded physical Source revision required"), and the production check `requireCurrentGraphClosureImplementationV1` would have thrown on the same file. Both captures now use the declared 8 MiB source bound (`maximumSourceBytes`).
