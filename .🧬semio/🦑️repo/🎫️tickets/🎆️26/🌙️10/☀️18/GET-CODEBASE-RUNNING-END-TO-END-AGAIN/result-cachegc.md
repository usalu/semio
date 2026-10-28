# result-cachegc (wave 1027b, lane cachegc, mirror prefix cg2): the optional graph cache enforces its own limit, `repo cache gc` no longer crashes

`C` = this ticket. Tools `C/cachegc-tools/` (`make.py` batch builder, `src/` snippets, `staged/quota/`, `relief.py`, `worker-e2e.ts`, `worker-closure.ts`, `inventory.py`, `difftests.py`, `closure-size.ts`). Nothing was edited live, no official `bun repo run` was started. Mirror `cg2` (at `SEMIO_MIRROR_HOME=/private/tmp/m`) is dropped.

## FOR coordinator

- **Immediate relief (run now, before landing):** `python3 -I C/cachegc-tools/relief.py "$PWD"` (dry run) then `python3 -I C/cachegc-tools/relief.py "$PWD" --apply`. It removes only the 2 sparse test pressure leaves (0 allocated blocks, 878,636,250 fake bytes) that optional-cache test runs left in throw-away dataset namespaces of the live store. Real records are never sparse and are never touched. The next graph writer measures the ledger again when it opens it (`clean(measure())`), so the used bytes drop from 1,073,741,824 to 195,105,574 (below the 512 MiB low-water mark) and records are stored again from the next run. No existing lock-aware tool can do this today: `repo cache gc` crashes in its closure stage before it gets that far, and even without the crash it only reclaims when free disk space is low.
  - Why the relief comes first: least-recently-used order is by write time. The real selection group `selections/8d8f…` (41,401 files, 193 MB, last write 10-26 18:55) is older than the 18:56:49 pressure leaf, so a `repo cache gc` run right after landing would retire the real records before the fake leaf. That costs one full re-inference. A build's own namespace is always excluded, so a build would not do this.
- **Batch ready** `C/cachegc-tools/staged/quota/` (11 files, 3 of them new). The base is the live bytes of 20:35; `stage.py check` on live shows 11/11 base.
  - Mode law: `python3 -I C/xplat5-tools/tests/test_stage_mode.py C/cachegc-tools/staged/quota/stage.py` OK.
  - Parse check: `C/gate-g-tools/scan.ts` over the 9 TS/MJS files: 0 bad.
  - tsc `--strict` on the touched TS files: only the existing `🚧️operands` error.
  - Apply: `python3 -I C/cachegc-tools/staged/quota/stage.py apply`.
  - Revert: `python3 -I C/cachegc-tools/staged/quota/stage.py revert`.
  - Rebase: `python3 -I C/cachegc-tools/make.py` rebuilds the batch from the current live files by exact anchors and exits 2 if an anchor is gone. Hot files: `💾️storage/🟨️.mjs`, the optional-cache suite, the ledger texts JSON.
  - Laws: `sh C/cachegc-tools/staged/quota/verify.sh <root>` should print `VERIFY OK quota`. `... verify.sh <root> before` restores the code to base and keeps the new laws; they are RED (0/3, 0/1, 0/1).
- **Product proof after landing:**
  1. `bun repo cache gc --dry-run --area=graph-inference` completes (today: `First failure: Bounded captured Source channel required`) and prints `The optional graph cache holds … against its limit of 1.00 GiB …`.
  2. The next course or print run prints no `the graph cache declined the leaves of project` lines.
  3. A second course or print run refreshes a small number of projects (today 168-198).
- **Worker probe** (read-only on live, writes only to the image directory you give it):
  - `bun C/cachegc-tools/worker-e2e.ts "$PWD" closure` reproduces the gc failure with the REAL `buildSourceClosureNodeArgumentsV1`.
  - After landing, `bun C/cachegc-tools/worker-e2e.ts "$PWD" static "$PWD" "$PWD/C/🗑️generated/cachegc/probe"` runs the real Node worker on the static snapshot.

## Root causes (all proven)

1. **Why the cache is full.**
   - `.cache/b/g/m/optional.sqlite` shows `used_bytes = maximum_bytes = 1073741824`.
   - The optional leaves add up to exactly 1,073,741,824 logical bytes but only 289,910,784 allocated (`C/cachegc-tools/inventory.py`).
   - Two of those leaves are sparse test pressure files:
     - `contexts/d44957…/np/000000000000000000000000.json`: 490,384,864 B, 0 blocks, mtime 10-26 18:56:49. That is exactly the moment publishing stopped. It is the "denial" law of the optional-cache suite (`(…"1":"0").repeat(24)`, size `maximum - used`).
     - `contexts/329c…/np/329c021ccf69b0deee3c18fd.json`: 388,251,386 B, mtime 1970. That is the "nested maintenance" law (`futimes(0)`).
   - Both were written while that suite used `process.cwd()` as its store (optcache store batch, landed in G46).
   - Real records: 193 MB.
2. **Why nothing evicted them.**
   - At writer open, `used > lowWater` sends the writer to exclusive image maintenance with `wait:false`. Every concurrent graph reader holds the image shared, so that maintenance is practically never acquired. The writer then declines every publication for the rest of its run.
   - When it is acquired, the closure proof dies in one of two places:
     - Its worker spawn fails (cause 3).
     - On a private store, `Ordinary single-link bounded closure proof leaf required` for a pressure leaf over 512 MiB. This is the base red of the insertion law.
   - `repo cache gc` reclaims only to a free-space target. It never looks at the optional limit, so it reported "Within the budget".
3. **Why `repo cache gc` crashes.**
   - The closure-read worker is captured with the "closure" scope. That scope follows the parent kernel's resolution map, including the backend registry's runtime-resolved `import(new URL(row.module))` modules, through `🦀️resolution → ⚙️operations/📖️help → 🔌️backends`.
   - So the worker snapshot is 1,438 of the kernel's 1,449 files: 17,563,527 bytes, against a 16,777,216-byte bootstrap channel (`node/🟨️.mjs`, the Node loader and `ReadSessionProtocol.maximumInputBytes`).
   - `worker-e2e.ts closure` reproduces the exact error with the real function.

## What the batch does

- **`📡️session/🟦️.ts`:** `closureReadWorkerScopeV1(root)` captures the worker with the existing `static` scope. That is its static imports, plus the Node runtime adapter, message catalog and native provider (`createRequire` sites of `⚙️runtime/🧭️selection` and `🟢️node/🧲️native`) declared as lazy roots.
  - Result: 67 files, 935,174 bytes, compared with 17.56 MB.
  - Anything else the worker loaded would be refused at that import.
  - E2E: the real Node loader and the real worker load the static snapshot, report ready in 0.6-0.9 s, answer one authenticated inference-record read through the session protocol, and exit 0.
- **`🗂️members/🔎️scan/🟦️.ts`:** new `retireBuildGraphOptionalGroupsLruV1(root, {lease}, control, {targetBytes = OptionalQuota.lowWaterBytes, dryRun, exclude, protectedMilliseconds, now})`.
  - It measures optional bytes the same way the ledger does: the logical size of every optional-path leaf.
  - It retires groups (`canonical`, `selections/<id>`, `contexts/<id>`) least recently written first until the bytes are at or below the target.
  - Each group is retired only under its own non-blocking writer lease. Readers and writers of a namespace hold that lease, so a busy group is skipped and never waited for.
  - It never retires the caller's own namespace, and never a group with an optional leaf younger than `groupProtectedIdleMilliseconds` (10 min); this is re-checked under the lease.
  - It deletes optional leaves only. Mandatory publications, receipts and the index never match the optional path. Records go first, then packs, then observation blocks, so an interrupted pass never leaves a record that names a missing block. The member index is then compacted to the leaves that remain.
  - It reports progress per group and stops at the next group when cancelled.
  - Optional leaves are recomputable, and no generation, pin or mandatory publication names them (generations live in `generations/`).
- **`💾️storage/🟨️.mjs` (insertion):** when a writer opens and the ledger shows `used > lowWaterBytes`, it runs that LRU pass excluding its own namespace and lease shard, before deciding anything else. If anything was retired, it reopens the ledger, which measures again. Only then does the existing pressure path run. So an insertion into a full cache stores the record and retires old groups, instead of declining everything.
- **`🧹️cache-gc` (`🚰️reclaim` + `⌨️command`):** a new `lru` port runs first. It enforces the optional limit whatever the free space is: a dry run reports the over-limit area, a real run retires down to the low-water mark. The collector then sweeps closures, which can collect more inventories after the retirement.
  - New texts in EN and DE: `gc.graph.quota.within`, `.plan`, `.done`.
- **Units:** I kept logical bytes. Publication plans and the SQLite `used<=maximum` check are logical, and the ledger's open measure must equal the in-session sums. Allocated bytes would let a session overshoot and make the next open throw `Physical optional graph bytes exceed fixed quota`.

## Laws (mirror `cg2`; base = code files restored, law files kept)

| law | file | base | batch |
|---|---|---|---|
| over-limit cache: LRU groups retired down to the target; busy, in-use and excluded groups stay; mandatory files and a compacted index stay; repeat passes; dry run, cancellation and the default low-water target | `🗂️members/🧪️tests/♻️lru/🟦️.ts` (new) | 0/3 | 3/3 |
| retired groups and remaining bytes equal the independent Python model (`🐍️.py`) on 120 seeded caches | same | red | green |
| gc enforces the optional limit with ample free space; dry run reports the over-limit area; a run retires first and completes; within-limit and cancelled lines | `🧹️cache-gc/🧪️tests/♻️quota/🟦️.ts` (new) | 0/1 | 1/1 |
| an insertion into a full optional cache (private real store) stores the new record, retires the stale group and leaves used bytes at or below the low-water mark | optional-cache suite (+1 test) | 0/1 (`Ordinary single-link bounded closure proof leaf required`) | 1/1 |
| writer admission model: eviction happens under the store and ledger before the pressure path, once when over the low-water mark | `💾️storage/🧪️tests/🚪️admission` (binding + assertions) | (base model has no eviction) | 1/1 |
| worker bootstrap: real `buildSourceClosureNodeArgumentsV1` | `worker-e2e.ts` (ticket) | refused at 17,563,527 B | 935,174 B, ready, read OK, exit 0 |

Neighbour suites, run one file at a time, base against batch, 46 files: cache-gc ×7, ledger ×7, members ×5, closures session/admission ×9, storage ×18. The failure sets are identical. The one difference, `🚪️admission`, I updated (row above). The optional-cache suite is 11 pass / 1 skip / 3 fail, and the 3 fails are exactly the 3 documented maintenance-worker reds from `result-optcache.md`.

## Not done / notes

- No live product proof (rule). The FOR lines above describe it.
- The exclusive closure-proof retirement (`retireBuildGraphOptionalGroupsHeadroomV1`) still orders by largest allocation and still needs exclusive image custody. It remains the free-space path. The LRU pass is now the quota path.
- Other test suites that use `process.cwd()` stores (optcache audit: 107 files) can still write pressure into a live store. The LRU eviction now heals that on the next writer open, but those suites should still move to `privateGraphWorkspaceV1`.

## Status

20:40 DONE: batch staged and mirror-verified (after and before), relief script dry-run on live (2 leaves, 878.6 MB fake bytes), mirror `cg2` dropped, no process of mine running.
