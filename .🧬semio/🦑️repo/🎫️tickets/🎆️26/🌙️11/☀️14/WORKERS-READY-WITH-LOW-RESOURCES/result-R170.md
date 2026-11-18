# R170: SEMIO_LEASE_ORDER wall of g1-m02 (2026-11-18 01:00 UTC)

Scripts: `T/🧰️scripts/R170/` (T = `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`): `patch.py` (anchored edits), `stage.py`, `preload.ts` (bun preload serving the staged files), `tests.sh`, `repro.ts` (two real processes), `land.py` (apply/restore by sha), `window.sh`.

## 1. Every `SEMIO_LEASE_ORDER` since 2026-11-17 12:00 UTC (all origins of `/Users/Shared/semio-store/v1/origins`, all files, plus the local service logs)

Exactly one occurrence, on g1-m02, reference 17306fe5, 2026-11-18T00:59:59.856Z (also in `ready-log-workspace.txt`, `ready-log-workspace-failed.txt`, `service-failure*.txt`, `ready.json`). The ten other `service-failure-*.txt` of g1-m02 are other codes (unserved, invalid, graph.farm-overrun, graph.farm-job-failed, none). No other origin has the code; the local host logs on this Mac have none.

| wanted | held | seconds |
|---|---|---|
| `native-storage-mapping:5874ac7a…` (shared) | `repository-graph-facts:/Users/ueli/Documents/semio:shard:53` (exclusive) | 17 |

The message carries no blocker (`held by process P`): the registry lookup `holderOf` found no other process holding the mapping exclusively. So the holder was either another task of the SAME process (the lookup skipped its own pid), or a process that publishes no registry entry (older code), or only an earlier claim in the arrival queue. The worker's host log (`host-92a24b545e19.log`, on m02 only) is not readable from here: **who held the mapping on m02 is not established**. The read itself had run 28,073 s ("taken 0 of 4455").

## 2. How the order module works (read from the code)

- Classes with a rank (`🔣️policy.json`), declared edges (`allowed`), strict pairs. `checkLeaseOrderV1` (rank inversion) is `observe` only. The enforced rules: hold-and-wait, self-deadlock, cycle, strict wait.
- Hold-and-wait: a blocking request that has waited `holdAndWait.graceMilliseconds` (15 s; 17 s observed = poll granularity) while its task holds ANY exclusive lease is refused, unless the wanted class is `inert` or a declared edge `heldClass → wantedClass` exists. It protects against the deadlock where a waiter keeps a lease others need, while the one it waits for needs that one.
- `native-storage-mapping:<sha256 of image root>` (`💾️storage/🔒️mapping`) had NO class: it classified as `unclassified` (rank 0), and no edge can name an unclassified class, so every exclusive hold (here the graph writer shard, held for the whole read) turned the first 15 s wait for the mapping into a refusal. The mapping is taken shared by every journalled operation of an image (`withNativeStorageMappingReadV1`) and exclusively only by retirement/maintenance/publication writers (`acquireNativeStorageMappingWriterV1`, `…MaintenanceMappingWriterV1`), which take their other leases without waiting (try-locks), so waiting for it cannot close a cycle (the cycle detector still enforces that).
- The refusal was rendered by the resident service as "unexpected failure … try again" (no typed reason, nothing about the holder).

## 3. Reproduction (two real processes, scratch state root) and red

`repro.ts 30`: process A holds the mapping exclusively for 30 s; process B holds the graph facts shard exclusively and takes the mapping shared (queued, shared cohort), exactly the production call. Live (old) code: B is refused after 15.2 s with `SEMIO_LEASE_ORDER` hold-and-wait (red, run 01:22 UTC). New tests are red on the old files by construction (new exports and class missing).

## 4. Fix (LANDED 2026-11-18 02:00:27 to 02:01:43 UTC under landing.lock, GREEN, no restore; entry in landings.md)

By construction, not by a longer window:
1. `image-mapping` is a declared **ground class** (rank 7, `ground.reason` EN/DE): waiting for it is never a hold-and-wait, whatever is held; the cycle detector stays active. (`🔣️policy.json`, schema `Class.ground`, contract.)
2. The wait is still bounded: `groundWait.boundMilliseconds` 1,800,000 x the R135 machine speed factor (`🐢️speed` registers `localMachineSpeedFactorV1` with the order module through `registerLeaseWaitScaleV1`; the process layer imports nothing from the build library). Past the bound `checkGroundWaitV1` refuses with the violation `hold-and-wait` carrying `boundMilliseconds`, `speedFactor`.
3. The blocker is named properly: `holderOf` now also finds an exclusive holder in the SAME process (other task) and returns the command; the text names pid, command and age, or says "held by another task of this very process", or "held by no process of the lease registry (older code, or a claim queue ahead)". EN and DE (`Messages.waitBound`, `blockerCommand`, `blockerSelf`, `blockerUnknown`).
4. The resident service renders a `LeaseOrderErrorV1` as the typed refusal `host-lease-refused` / `host-lease-refused-unregistered` (unserved, retry later, EN/DE, fields requested, mode, held, seconds, holder pid, command, age, bound), not as an unexpected failure (`leaseRefusalOfV1` in `🏛️service/🧯️failure`).
5. Law `🧭️order/🧪️tests/🧭️sites`: a static census of every `acquire…ResourceLease` site of `🏃️process` and the build library (170 sites; names resolved through literals, same-file definitions and schema constants: 87 resolved, 83 dynamic in 32 files, counted per file in `🧫️fixtures/🧭️sites.json`). It fails on a resolved name that no class ranks (acknowledged list of 5 with a reason EN/DE, the mapping removed from it by the fix), on a new or removed dynamic site, and on any lexically nested pair of sites that the policy would refuse (one declared pair). Red on the old policy (mapping unclassified).

## 5. Proof (staged, all with the transpiler cache disabled)

- Order tests 76, processes 28, wait-for 6, registry-root 2, component-driver 21 (one assertion widened to exclude ground classes), sites 5, ground 5 (two real processes: 18 s wait then success with the wait published in the registry; bound 4 s, typed refusal naming holder pid and command in both languages; EN/DE texts; speed factor 2.83 equals the independent formula for 4 cores / 8 GB); standing 26; resident failure 18, failure-frames 2, worker-error 2; speed 3.
- Restore of `land.py` proven byte-identical on a mirror.

## 6. Hazard found: bun's transpiler cache inlines JSON of staged runs

`~/Library/Caches/bun/@t@` caches the transpile of an UNCHANGED importer together with the JSON it inlines. A staged run (preload serving other JSON) poisoned live runs of the same test file (component-driver saw `image-mapping` live; resident failure test passed live on a function that did not exist). Every staged run of R170 sets `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`; other stagers (R160 to R168 preloads) should too. The entries are rewritten by the next cached run of the same file; after the landing the affected test files are run once live with the cache on.

## 7. Not verified / open

- The real holder on m02 (see 1). The hold-and-wait refusal for the five acknowledged unranked names remains.
- Static pairs across call boundaries (callbacks) cannot be seen by a text census: the graph writer to mapping pair itself is not a lexical pair; it is closed by the class, not found by the law.
- No live service restart observed on a worker yet; step 4 of the task (watch the workers, record the next wall of g1-m02) follows the landing.

## 8. After the landing (02:04 UTC)

- Under the lock all suites were green (list in landings.md). Live re-run with the transpiler cache on: order 76, processes 28, component-driver 21, sites 5, ground 5, resident failure 18, 0 fail.
- The cache hazard of section 6 happened for real: the first cached live run of the order test resolved `staged/order.ts` in scratch (`Cannot find module`). I found the 8 cache files containing my scratch path (`rg -a -l -F scratchpad/R170` in `~/Library/Caches/bun/@t@`), removed exactly those 8 `.pile` files (a cache, regenerated on demand), and re-ran: no entry names the scratch path any more. Any other stager should run that search for its own scratch path.
- Step 4 (watch the workers for `SEMIO_LEASE_ORDER`, record the next wall of g1-m02) is NOT done: the measurement freeze 02:05 to 03:30 started; the landing restarts the workers, so the first reading is after their next ready cycle. Command to watch: `grep -rl SEMIO_LEASE_ORDER /Users/Shared/semio-store/v1/origins` (use `rg -a`, the shell grep alias misses emoji files).
