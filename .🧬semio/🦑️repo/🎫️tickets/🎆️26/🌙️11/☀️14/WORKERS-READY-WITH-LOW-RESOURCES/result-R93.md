# R93: cold fleet seeding of a project record streams its closure bundles (bounded); the worker guard is NOT done

Scripts: `T/🧰️scripts/R93/` (`old-harness.ts`, `port-harness.ts`, `run.py` sampler with `footprint` of the tree every 250 ms, `overlay.ts` pre-lock overlay, `mk1.py`, `land.py`, `law/`). T = `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`.

## 1. Reproduction (cold private local closure store, fleet cache read-only, real server entries)

The entry of `mit-bestand-praesentation-projektetage` could not be addressed (its key needs the live inference head; the entry may not exist on this server). Replayed instead the largest current `project-inference` entries of the server (`.cache/f/cache-server/v1/main/ac`, 14 415 entries; every closure-heavy entry hits the 192 MB `closureBudgetBytes`): `263f99b7` (79 closure documents, 19 MB compressed, 194 MB decompressed), `316db407` (48 documents, 19 / 200 MB), through the real `createProjectFleetPortV1().gate()` inside `runTakeLookV1` (a client wrapper maps the key of a harness state to the real entry).

Allocation site (old code, `project-records` `takeClosures`, measured with marks): fetch + brotli of EVERY closure document at once (194 MB, rss 0.5 GB) -> `JSON.parse` + `unpackBundleV1` of all 79 (rss 1.5 GB) -> `localBundleV1` of all (a `JSON.stringify/parse` of the recipe and `localizeRowsV1` copies of every row list; rss 4.4 GB, heap 1.2 GB + external 1.2 GB) -> `closures.import` of the whole array (heap 2.6 GB). Row JSON strings are not the issue here, the object copies are.

| case (old code) | rss peak | tree footprint peak (250 ms) | wall |
|---|---|---|---|
| one owner, 79 bundles | 4 761 MB | 3 099 MB | 14.7 s |
| two looks at once (the take runs several) | 8 451 MB | 4 487 MB | 27 s |

Three looks at once (m07/m08 have a take concurrency above 2) plus the worker's own 1 to 2 GB reaches the 10 GB the service saw. The 10 GB itself was not reproduced here.

## 2. Fix (landed: see section 4)
- `exchange` `fetchGraphEntryV1(client,state,held,defer)` / `decideGraphEntryV1({defer})`: documents `defer` names are not downloaded; they come back in `deferred` (name -> `{hash,size,read({keep})}`); `read` downloads the blobs (client verifies size and sha256), decompresses one document; `keep` retains the compressed blobs (at most 32 MiB per entry) for the second pass.
- `project-records`: `gate` defers `closure-*` documents. `seedFrom`: pass 1 reads each document once (two in flight at most), keeps only its recipe (and whether it names the owner token), pass 2 unpacks, localises and imports ONE bundle at a time (window of 2), so at most about 2 documents (8 to 11 MB each) are held. Sharing by digest: in-flight map keyed by blob digest (two owners that need the same bundle import it once), a bounded recipe memo by digest (4096), and the existing `held` skip (a blob whose digest the local store already knows is not downloaded). No child process needed.
- No compatibility path: the all-at-once code is gone.

## 3. Laws
- `project-records/🧪️tests/🌊️seed-stream` (real child process `👶️child.ts`, real cache server, 24 entries of 14 000 rows, two owners that share them): cold machine peak resident set under 400 MB, every entry stored, the second owner downloads 1 blob (its record) and stores 0. Old code: RED (780 MB). New code: GREEN (290 MB), 7.7 s.
- Real fleet entries, old vs new code, same harness: one owner 3 099 -> 520 MB tree footprint (rss 4 761 -> 1 510 MB); two at once 4 487 -> 523 MB footprint (rss 8 451 -> 2 079 MB). Wall time unchanged (dominated by the store import).
- Regression files green under the overlay before the lock: `🚚️closures` 3, `🗜️packed` 6, `graph-state/🧪️tests` 14, `entry-lane` 5 (R92), `🙋️wanted` 9, `💰️cost` 20, `project-nodes/📥️take` 11. `project-records/🧪️tests/🟦️.ts` has 1 failure (a source-text assertion about `observation`, `if(!reused&&port){gate=await port.gate(info)`), identical on the unchanged live code; not mine, not in the gate.

## 4. Landing and after
(filled in below)

## 5. Not done / not verified
- Worker guard seeing the tree footprint (point 2, second part): NOT done. The landing freeze allowed only the seeding fix; the guard (`plan-stage`, heapUsed + external) still ignores children and unmanaged pages. Next package.
- A cut keeping durable progress: not touched.
- The named owner itself, the 10 GB figure, and the m07/m08 effect are inferred, not measured, until the reads below.
- Rss still climbs to about 1.5 to 2 GB over a 25 s multi-bundle seeding (JSC keeps freed pages); the service reads footprint (520 MB).
- Test-running trap: `bun test <path>` without a leading `./` treats the path as a filter and scans 9 million files for minutes (looked like a hang).
