# R103: why `@semio-tech/wfc-bitmap` needed 4.4 to 4.6 GB alone, and a single heavy owner no longer fails a machine's read

Scripts: `T/🧰️scripts/R103/` (T = `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`): `cold-child.mjs` (the cold `prepare` job of one owner in a fresh process, the way `👷️service` runs it, session of the 14:26 wave), `sample.py` (footprint of the process tree from outside with macOS `footprint`, every second), `hooks.mjs` (logs every JSON text above 2 MB with its call stack; wraps `JSON.parse`/`JSON.stringify`/`readFileSync` only, no profiler, no heap snapshot), `overlay.ts`, `land.py`/`restore.py`/`runall.sh`/`window.sh`/`compare.py`, the two new tests.

## 1. What holds the memory (measured)

The farm job is the cold derivation (`prepare`), not the warm one R97 measured. The owner is a 633-byte TypeScript script owner whose script imports the repository library; its closure entries come from the fleet (`lane.take`) and R97's serial intake does cover that path. The memory is not in the intake but in what the intake calls for every entry:

- With `hooks.mjs`: `localBundleV1` (`🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts`) wrote the members of the bundle besides rows and layers (`value`, `sidecar` = walk facts and members of every layer) out as ONE JSON text, replaced the root in the text and parsed it again: `JSON.stringify` 406 MB, `JSON.parse` 457 MB, for one entry whose stored document is 15 to 18 MB (rows are packed once, the sidecar of a library closure is hundreds of layers of thousands of paths). The same line existed on the publishing side (`neutralBundleV1`, `packBundleV1`). Parsed, 457 MB of JSON are 4 to 6 GB of JS objects and strings: the heap alternated 60 MB / 4 to 8 GB with `extraMemorySize` as large, 4.7 million strings, at every entry taken.
- Ruled out by the same run: closure entry check/load (counts `loaded 9`, `checked 36`), row tables (35 000 rows, 13 MB), analysis images (230 KB).

## 2. Numbers (cold job of `@semio-tech/wfc-bitmap`, g1-server, store clone, fleet cache live, load 11 to 14)

| run | peak footprint (tree) | peak rss | wall | note |
|---|---|---|---|---|
| before (live tree) | 11264 MB (macOS rounds above 10 GB), 8506 rss | 8506 MB | 99 s | reply `peakMegabytes` 7282; the farm of a 16 GB worker saw 4358 / 4632 MB |
| after, run 1 | **772 MB** | 1462 MB | 22 s | staged `portable.ts` through the overlay |
| after, run 2 | **916 MB** (tree 941) | 1426 MB | 22 s | reply `peakMegabytes` 1316 (rss based) |

## 3. Fix 1 (bounded memory)

`movedRestV1(rest, direction, host, policy)` in `portable/🟦️.ts`: the members are copied by a walk in which every distinct string is moved once (`indexTextRewriterV1`, the existing per-text equivalent of the JSON-domain rewrite), the structure is copied, nothing is written out as one text. A Windows host, an empty root, members that are no plain JSON (`Set`, `undefined`, `toJSON`, non-finite numbers) take the old text path, so the result is the same. The three sites (`neutralBundleV1`, `localBundleV1`, `packBundleV1`) use it.

Law `portable/🧪️tests/🪶️rest-walk`: (a) the walk equals the text round trip in both directions (the reference is written in the test from `neutralTextV1`/`localTextV1`), (b) odd members, a Windows host and a residue token behave as the text path (the residue still throws `UnportableV1`), (c) moving, taking and packing a bundle with a 5.3 MB sidecar makes no JSON text of 1 MB or more (red on the old code: 5 322 225 characters).

## 4. Fix 2 (a single owner over the cap never fails the read)

Before: `prepare` of the farm threw `GraphFarmOverrunErrorV1` (`graph.farm-overrun`); the throw crossed the owner loop, ended the whole project-graph read and surfaced as "unexpected failure (reference 36766c07)". The machinery for owners this machine can never infer already existed (`never`, `waitHeavy`, the fleet wait with its bilingual progress line, the owner pool offer with the recorded megabytes), but only for owners whose cost was recorded before.

Now `setAsideOverrunV1` (project-nodes): an overrun joins `never` and `alone`, the cost it showed is recorded under its name (scaled by the footprint factor so `known(name)` reads the measured megabytes, which is what the offer to the fleet carries and what a machine with a larger limit compares with), and one line per language says why (`OVERRUN_ASIDE_V1`, EN and DE, with owner, measured and allowed megabytes). The owner loop chains `.catch(error=>{if(!setAside(error))throw error;})` onto `runInOwnerScopeV1` (a wrapper around `inferProject` is refused by the law `governed-dispatch`: every `inferProject` call stands under `governor.run`; the first landing attempt at 00:30 was red for that and restored); the read goes on with every other owner; at the end `waitHeavy` shows "heavy owners ... are inferred by a machine of the fleet that can hold them: N of M arrived" and picks the owners up when they are published; only if nothing in the fleet answers does it end with the existing typed `graph.owner-too-heavy` refusal. The set-aside needs a limited machine (the soloLimit exists); a serving machine without a limit still throws as before.

Law `project-nodes/🧪️tests/🪂️set-aside`: the set-aside flags, record and bilingual line, other errors untouched; the recorded cost is known afterwards and `neverHere` lists the owner on a 16 GB machine; the loop chains `setAside` and the wait comes after the last loop.

## 5. Landing and standing results

LANDED 2026-11-17 01:00:00 to 01:01:41 UTC (lock `landing.lock`, released), 21 test files GREEN against the same battery on the unchanged tree, see `landings.md`, entry R103. Baseline red on the UNCHANGED tree (not mine): `project-nodes/🧪️tests/🛣️lanes` 1, `🚦️governed-dispatch` 2, `🛑️too-heavy` 1 (source order check of `enforcement.recorder?.flush()`), `🧹️parent-budget` 1; `compare.py` reports red only for an increase.

## 6. Not verified / observed on the way

- The two 16 GB workers were not run again; the numbers are from g1-server with a store clone and the 14:26 session.
- The set-aside is proven by unit laws and the source shape, not by a real farm overrun end to end.
- A warm job that overruns still falls back to the parent, which runs the owner under its own limit and then reaches `prepare`.
- Finding, not changed: `warmOwners` `describe` sets `knownMegabytes` to `Math.round(known/1048576)`, but `enforcement.known` already answers megabytes, so R97's admission sees 0 for every owner.
