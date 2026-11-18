# R126: a project-graph read no longer dies when the shared stores change under it

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R126/` (`run.sh` bare-bun runner with overlay, `stage.py`, `files.json`, `patch-nodes.py`, `rebase.py`, `land.mjs`, `restore.mjs`, `window.sh`, `battery.sh`, `compare.py`).
Landing status and time: see the section "Landing" at the end.

## 1. What each check compared and why it fired

All three are about one thing: a check made early in a long query is made again at the end, or a thing found earlier is read later, and a concurrent writer (pool child, peer read, collector, landing) changed the store between the two looks.

| Error | Where | What it compares | Why it fired |
|---|---|---|---|
| `closure.provider-entry-stale {"port":"layers"}` ("The provider closures layers changed...") | `🧭️script-inputs/🟨️.mjs`: `closureLayerBatchesV1`, `closureLayerViewsV1`, `adopt`; thrown from `api.resolve`/`api.plan` | NOT a directory listing, mtime or generation counter. `layerBytesSync(digest)` of `💾️entries`: the file `l/<digest>.json.br` must exist, decompress and hash to its own digest. Only a layer that is gone or damaged answers `CLOSURE_LAYER_LOST`. An added layer or entry is never looked at by a query that did not name it. | The read holds the entry in memory (digests only) and the round caches the verdict "layer holds". The retention rule (`🧹️retention`, `planClosureRetentionV1`) deletes a layer that no kept entry names one hour (`closureStoreGraceMilliseconds` 3600000) after it was written or reused; a reader that only reads never refreshes it. The dead read ran 4611 s = 77 min, a pool child/peer had superseded the entry for the same recipe slot, and the collector removed the old layers under it. |
| `closure.provider-entry-stale` with the names of the providers (same text) | `fact` of `closureApiV1` | Rows of the shared provider entries against the live tree, three refresh attempts (`closureAnswerAttempts`) | A hot tool directory changed three times during one query. After the third attempt the command died. |
| `Inference inventory absence changed` | `👁️observation/📥️reuse/🟨️.mjs` `requireInventoryLeafAbsentV1`, called at the end of `missingInventoryNamesV1` | `lstat` of `build-input-closures/<key>.json` / `build-lock-closures/<key>.json` must still be ENOENT at the end of the batch as it was at the start | The same inventory record was stored by another process (pool child, peer read) while the batch looked. Any such store inside the minutes of a batch fires it; the whole read of the owner ended with an untyped Error. The same family: `leaf changed`, `leaf changed before delivery`, `descriptor changed`, `ancestry changed`. |

## 2. What changed

1. `answeredAgainstLiveLayersV1(workspaceRoot, body)` (new, exported, `🧭️script-inputs/🟨️.mjs`) wraps `api.resolve` and `api.plan`. On a lost layer it forgets what the read holds of it: the verdict files (`l-`, `e-`) of the read and the round map, and every in-memory entry that names the layer (`forgetLostLayerV1`; the other lost layers of those entries are found with the new `lostLayersSync` of the store), then asks again against entries read anew, at most `closureLayerAttempts` (3, new limit in the inference schema with EN and DE text), one line per retry in EN and DE (`[closure] ...`). In the parent of a read with a farm (`readParentV1()`) the parent must not derive (R122), so it forgets, says so and ends the look at once; the owner is then requeued and the farm derives the entries. Refusals carry `closureLayerLost` and `storeChanged`.
2. The closure store (`💾️entries`) trusts "I wrote this layer/value within the last quarter grace" only while the file exists (`trusted(digest, path)`), so a save after a loss writes the layer again (before: the memory of the earlier write skipped it and the stored entry named a missing layer). New `lostLayersSync`.
3. `fact` refusal after the attempts carries `storeChanged: true`.
4. Inventory: the five "changed" throws of `missingInventoryNamesV1` are typed `BUILD_STORE_CHANGED` (`inventoryChangedV1`, messages unchanged); `inventoryNamesSettledV1` looks again (policy `changeAttempts` 3 in the reuse schema, EN+DE line `[inventory] ...`) against the state that is there now, for the coalescer batch and for the non-coalesced path; a caller whose names still change gets the typed error alone (the coalescer already retries caller by caller).
5. Owner level (`📦️project-nodes`): `ownerStoreChangeV1(error, name, seen)` -> `requeue` the first time, `aside` the second time, `undefined` for everything else. `changedOwner` in the owner loop requeues the owner (tail of `jobs`, not counted as finished) once and sets it aside with the reason (EN+DE, same `unavailable` record as a failed farm job via the new shared `asideOwner`) when it changes again; the read goes on and ends naming it. `setAside` now takes the running job.
6. Test pins updated: `🪂️set-aside` (signature of `setAside` and its call in the loop).

## 3. Proof (tests first)

New files (final paths): `🧭️script-inputs/🧪️tests/🧱️layer-lost/🟦️.ts` (6 tests), `👁️observation/🧪️tests/🔁️inventory-changed/🟦️.ts` (5), `📦️project-nodes/🧪️tests/🔁️store-changed/🟦️.ts` (3).
- Red before: layer-lost 0 pass 6 fail on the unchanged tree; the integration and the second-process tests reproduce production exactly: `RepoError: invalid build.closure.provider-entry-stale {"port":"layers"}` at `closureLayerBatchesV1` <- `addLayer` <- `resolve`. inventory-changed and store-changed fail on the missing exports.
- Green after (staged overlay, same files that land): 6, 5, 3 pass.
- Real second process (law 4): `Bun.spawn` of a child that saves entries and layers in a loop for 1.5 s and, once, deletes the entry files the parent holds and runs the collector with grace 0; the parent resolves in a loop inside one closure pass; every answer equals the first answer, the store is consistent at the end (every layer an entry names exists).
- Cross-check with an independent implementation: inventory law keeps the SQLite oracle style of `🧺️presence-batch`; the layer answers are compared by value with the answer of the first (pre-loss) derivation.
- Batteries (staged vs live, same failures before and after): script-inputs 27 files, observation 38 files, key pin tests; pre-existing failures unchanged (`producer-path` 1, `💾️entries/🧪️tests` 1, `🫧️take-release` 2, `🌊️layer-stream` heap measure 1 under load, `🛑️too-heavy` 1, and the observation ones listed in the landing).

## 4. What happens to the owners of a read that ends early (law 5)

Evidence from the store of the dead read (pid 5001's successor, log `server-read-1006.log`, died after 4611 s with 164 of 4775 owners done, 151 inferred here):
- Every inferred owner is a durable record `np/<owner>.json` (+ `.json.fleet` marker) under `.cache/b/g/m/inference/selections/8d8f21d7.../np`, written when the owner finishes, not at the end of the read. 386 `.json` records were written between 10:06 and 11:23 UTC (the read's own plus its pool children's); 107 existed before the read.
- The next read (`server-read-1133.log`) printed `owner records present: 531 of 4775` at its start where the dead read had started with 254, and took 51 of its first 76 owners as `present` through the normal accept path (header digest verdict), without inference. So what a dead read finished is reusable and costs the accept path only. What is lost: the in-memory closure entries and the verdict directory of the dead read (the closure entries stay in `ce/`), i.e. the first derivation checks again.
- After this change the same death costs one owner (requeued once) instead of the read.

## 5. Not verified

- No real read with this code: the live reads run pinned old code and must be restarted by the coordinator.
- The inventory race is proven at the seam (`inventoryNamesSettledV1`, coalescer, source pins on the five throw sites), not with a real second process against the managed-custody store (needs the native image fixture of `🪪️presence`).
- The requeue/aside decision of the owner loop is a pure function plus source pins; the loop itself was not run with a failing owner.
- The farm re-derivation after a loss in the parent (second attempt through the observe job) is argued from the code (children bind the store writable, the entry check is recomputed because the verdict files are removed), not run.
- A farm child that wrote a layer earlier in its own life no longer skips the write when the file is gone (fixed in `trusted`); not run with a child.

## Landing

Landed 2026-11-17 12:33:37 to 12:36:44 UTC under `landing.lock` (owner claude-R126, released, 3 min 7 s), script `window.sh` (merge over base, restore tested before on a copy: hashes equal). Earlier attempts at 12:03 (conflict with a peer edit of `📦️project-nodes/🟨️.mjs`, nothing written) and 12:04 (lock held by R127). Twelve files: 9 edited (sha256 prefixes in `landings.md`), 3 new tests.
Tests under the lock: 30 files, none worse than the baseline taken at 12:22 (too-heavy, inferred, producer-path, stopped-wave, governed-dispatch, parent-budget fail the same before and after); the three new files 6, 5, 3 pass live; both standing preflights pass.
The live read of 11:33 (and any read started before 12:33) runs pinned old code: restart it to get the change.
