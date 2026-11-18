# R130: the part of a project-graph read after the last owner fits a smaller machine and survives a retry

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R130/` (`postowners.ts` is R127's driver with a row-log part, unique row-log paths and per-mark heap numbers; `stage.mjs` stages the changed modules and their tests in scratch with absolute imports; `patches.mjs`/`patches2.mjs`/`patches3.mjs` are the anchored edits per landing; `land.mjs`, `restore.mjs`, `window.sh`, `battery.sh`, `run-tests.mjs`; `new/` the new files per landing).

## 1. What held the memory (R127 harness, 4774 owners, 150 rows)

Per phase, peak resident set before (the first run of today at load 44, before any landing): compute 2182 MB, fragments 2228, patch 2974, keep 3166, assembly 3752. The holders, in the order they were removed:

1. `rows: [...captured.observations]` in `discoverBuildGraphPartV1` spread the lazy row log into one array of row objects, and `shapeComputeResultV1` then sorted a path index over all of them (`ordered`), built a `Map` of keys per owner (`wanted`) and a `Set` of all claimed key texts (`claimed`), and filtered the whole array again for the environment. About 1 GB at 150 rows and linear in the rows.
2. `patchFactIndexBytesV1` kept every new row as an object (`fresh`, `byObject`), its canonical key text (`added`), a JS string and a Buffer per distinct text (`freshText`, `sortedFresh`, `freshId`), an identity set of fact objects, and an array of `[from,to]` pairs plus a digest string per fact (`spans`, `byBytes`). About 0.7 GB at 150 rows, 3 GB at 1000 rows.
3. The row log itself kept a `Map` from canonical key text to number and an array of the texts (about 300 bytes per distinct row), and allocated a buffer per read.
4. A delta read published a full generation (patch, pack of the whole index, publish) per wave of owners.

Not held by rows, not changed: the owner nodes in the parent until the fragments are stored, and the assembled graph itself (about 1 to 1.3 GB with the harness' 1500 mock input files per target, it is returned to the caller).

## 2. What was built

Step 1 (landed 12:30 UTC): `discoverBuildGraphPartV1` returns `rows` as the row log list (`captured.observations`) instead of spreading it; `shapeComputeResultV1` claims over a list in one streamed pass (batches of 2048 rows, ancestor-directory lookup of the owner roots in a prefix map, a bitmap of claimed row numbers, only the numbers an owner holds beyond its own list), where an array part (tests, slices) still takes the sorted-array path. Test `🔌️load/🧪️tests/🌊️streamed-shape`: the 60 worlds of the owner-claims fixture through a row log equal the independent Python model, and shaping retains fewer than 24 bytes per row.

Step 2 (landed 13:00): the patch of the fact index (`🌊️stream-patch`) keeps a row that comes from a row log as its log number and re-reads it in windows of 2048 when its texts are collected and its facts are written; its dedupe table of new rows and of fact spans is a key table (new `🔑️key-table`: a 128 bit SHA-256 prefix of a text to a number in one typed array, about 40 bytes per entry), its new texts are an arena (new `🔤️text-arena`: UTF-8 bytes in one buffer, sorted by index); the streamed claim and the patch collect garbage every 32 batches so parsed rows never pile up in the resident set. Tests: key table 2, text arena 2 (Map, Python dict and `Buffer.compare` as the independent models).

Step 3 (landed 14:00): the row log names its rows by the key table (no key text per row) and reads spans through one reused buffer. Test `🧪️tests/🌊️flat-memory` (the patch adds fewer than 800 bytes of resident set for each further row between 80k and 320k rows, and takes at most 6 times the time for 4 times the rows; child process per size).

Step 4 (resumable and one generation per read, see section 5 for the landing): `🔖️journal` (header schema, rows files, the factory the store uses), `📖️documents` and `🗃️file-ports` (the pointer, manifest and fragment readers and the file and lease ports moved out of the store because `generations/🟨️.mjs` is pinned by the graph closure implementation check with a ceiling of 65536 bytes, now 61420; the store re-exports every name). The store build loop derives the waves of a delta read without publishing: it keeps the owner table, the upserts (as row log lists), the drops and the derived names in a carry, tells the host with a new `wave` hook (the host's `waveDone`: wave row, memory settle, background yield), and patches, packs and publishes once after the last wave. Every finished wave leaves a journal (`j/<key>.json` and a rows file); the sealed build (manifest written, index kept) leaves a sealed journal; each finished step leaves a marker `{step, inputs, output}` (`storing`, `indexing`, `sealing`). A rerun with the same key (generation it refines, identity, membership, owners to derive) takes the journal up: a sealed build is published and then verified against the tree by the read that follows (so the tree having moved meanwhile is found), a build with finished waves derives only the owners it did not. Tests (`🧪️tests/🔖️journal`, 6): header closed under its schema and rows files round-trip; a build killed after `indexing` is published by the next try with no derive, no store, no index (steps seen: `storing`, `indexing`, then only `sealing`) and equals the Python oracle; a sealed build whose tree changed meanwhile derives the changed owner; one generation however many waves; a build cut off after a wave derives only the rest and publishes one generation; a journal of other owners is not taken up. Existing tests changed to the new contract: `🌊️waves`, `📈️adaptive-waves`, `📈️wave-progress`, `⏱️stages`, `🔁️carried-release`, the busy-file law of `🧪️tests` (generation count 1 instead of one per wave).

## 3. Before and after (R127 harness: `postowners.ts`, 4774 owners, mock expansion of 1500 files per target, real store, shape, assemble, expand and `readBuildGraphV1`; resident set sampled every 50 ms; this Mac at load 25 to 50, so times vary by half)

| case | before (R127) | after (live code, R130) |
| --- | --- | --- |
| cold, 150 rows | 4084 MB, 16 s idle / 36 s loaded (same harness at load 44 before the first landing: 3752 MB, 48 s) | 1881 to 2209 MB, 28 to 35 s |
| cold, 300 rows | 5347 to 5766 MB, 34 to 41 s | 2359 to 3018 MB, 55 to 66 s (2402 MB without the mock expansion, 1.8 GB in a traced run) |
| cold, 1000 rows | killed past 12 GB | 5342 MB, 155 s (fits; the index is 390 MB expanded of the 512 MB limit) |
| 8 GB worker (cap 3276 MB), 150 / 300 / 1000 rows | killed at 4357 MB | passes at 150 and 300, fails at 1000 |
| delta of 473 owners on 4774 | 4723 MB, 91 s, 10 generations published | 2320 to 2727 MB, 20 to 28 s, 1 generation published |
| patch step, rows x4 | N^1.4 | time x3.9 to x4.1 in the harness (child process per size in `🌊️flat-memory`, bound 6) |

Per phase after (150 rows, a traced run): compute 792 MB, fragments 872, patch 1582, keep 1479, assembly 2060 (before: 2182 / 2228 / 2974 / 3166 / 3752). The harness' own owner nodes and row log are about 0.8 GB of that at 150 rows, 1.07 GB at 300 rows and 2.5 GB at 1000 rows.

## 4. Targets not met, and what is left

- Target "under 1.5 GB at 300 rows and under 2 GB at 1000 rows" is NOT met: 2.4 to 3.0 GB and 5.3 GB. What is flat now is the shaping (a few bytes per row) and the row log; the patch still adds about 470 to 590 bytes of resident set per row (arena of the new path texts, the key tables, the typed row and fact sinks, and the output document, which is built in memory as the old and the new bytes; the index stays under its 512 MB limit up to about 1300 rows per owner). Streaming the output of the patch to disk and an external sort of the new texts would make it flat; both are design changes of the fact codec.
- The assembled graph (about 1 to 1.3 GB with the mock expansion) is returned whole to the caller and not streamed.
- The parent-side tail stages of `discoverBuildGraphPartV1` (environment, tests, policies, dangling, files, test-edges, edges, metadata, result) are unchanged: they still hold `projects`, `neutral`, `testRows` and `tagRows` in memory. Only the rows (the stage that was measured) are streamed.
- The multi-wave memory is not measured: the harness' delta runs 10 waves with the row logs of all waves kept open until the last (4 bytes per row in memory, a file per wave); whether a closure that `dispose` captures keeps more than the log alive across waves is not proven.
- A build cut off between waves keeps the journal of the finished waves, but a kill before the first wave ends loses the wave; a cold build keeps nothing before its index is stored (the `storing` marker is a record, only the sealed journal is taken up).
- A resumed build is verified against the tree by the plan that follows (it answers `published: true`, `resumed: true`, and the caller plans again), so the tree having moved meanwhile costs a delta, not a wrong graph; the verification of 716100 facts took 15 to 24 s in the harness.
- Not run for real: a worker, the real discovery of this repository (the numbers come from the synthetic part), `restoreRemoteGenerationV1`/upload together with a resumed build, `repo run workspace:setup`.

## 5. Landings (landings.md has the details)

1. 2026-11-17 12:30:01 to 12:33:35 UTC: streamed claim over the row log list.
2. 13:00:01 to 13:03:17: patch with log references, key tables, text arena, periodic collection; `🔑️key-table`, `🔤️text-arena`.
3. 14:00:47 to 14:02:16: row log by key table; `🌊️flat-memory`.
4. 15:00:00 to 15:02:17: one generation per read, journal and resume; `🔖️journal`, `📖️documents`, `🗃️file-ports`. A try at 14:30 was restored on red (the store passed the 65536 byte ceiling of the closure implementation check, now 61420 bytes).
