# Closure inventory fix (wasm build of @semio-tech/puzzle-plugin)

## Status
Official `repo run @semio-tech/puzzle-plugin:wasm --lang=en` now gets past the closure directory inventory and ten further latent defects of the same headroom closure proof; run 13 (log `🗑️generated/closinv-wasm-13.log`, EXIT 1) stops at the next one, listed under "Open blocker".

## Root cause of the reported failure
- Image walked: the graph store image `.cache/b/g/m` (527k entries incl. ignored ones; 374k counted).
- `Protocol.maximumEntries` was 262144 (`closures/🧬️schema/🔣️.json`), the real store has about 374k entries (derived-nodes 147k, inference 127k, build-input-closures 35k, part-ids 26k).
- Measured metadata cost: 278 MB heap for 262144 entries of the worst-case 105-emoji names, about 1.06 KB per entry; the declared `heapMiB` 1536 MiB budget fits about 1.5M entries. New bound 1048576 (about 1.1 GB worst case, about 2.8x the observed store); the cardinality test measures it against the same 1.6 GB budget.

## Further defects behind it (each hit in turn, each fixed)
1. `role()` is a closed vocabulary but the image hosts other stores: `inference/**/ce`, `derived-nodes`, `part-ids`, `task-keys`, `generations/{a,c}`, `selected/*.admission.json`, root memos `generations.{closures,closures-sent,neutral}.json` and the crashed `directory-stamps.json.<pid>.tmp`. They are now spills (never read, never retired, must be regular files) in `spill()`. CAVEAT: if derived-nodes or generations/a hold closure references, retirement will not see them; cache-miss only, but worth a human review.
2. `withMovedClosureReadmittedV1`: the resident service has no baked admission (`import.meta.semioCapturedSourceResolutions` undefined); it now admits fresh at once and silently.
3. `runClosureReadCohortsV1`: the maintenance control has no deadline, an owned interactive command refuses it; the channel now runs under a renewable `native-child` control.
4. `funding/replacement/backing/🟦️.ts`: `import{type X}from` is kept by Node as a side-effect import and was not declared in the captured map; now `import type{X}from`.
5. Captured Node loader hook refused the `data:` module made by `inputs/📋️data/📥️load` (registry says Node uses `data`); hook now admits `data:text/javascript;base64,` from a captured parent.
6. Authority receipts now carry `seal`; worker and headroom receipt program rejected the extra key. Both accept the optional, schema-checked `seal`.
7. `build/🧬️schema/📥️authority.json` observation kinds lacked `stamp`, `closure`, `registry` (the observation schema has 19 kinds); synced (also affects `buildGraphProducerAuthorityV1`).

## Files changed
- closures: `🧬️schema/🔣️.json`, `🟦️.ts`, `📡️session/🟦️.ts`, `👷️closure-read-worker/🟨️.mjs`
- headroom: `📏️headroom/🟦️.ts`
- `build/📥️source/🔏️closure/🛂️readmission/🟦️.ts`, `build/📥️source/🔏️closure/🟢️node/🟨️.mjs`, `build/🧬️schema/📥️authority.json`
- `native-build/💾️storage/💰️funding/🔁️replacement/📸️backing/🟦️.ts`
- tests: `closures/🧪️tests/📏️cardinality/{🟦️.ts,🧫️cases.json,🧬️schema/🔣️.json}` (also repaired missing `spill` port), `📈️owner-peaks-spill/🟦️.ts`, new `🔏️authority-seal/🟦️.ts`, `📡️session/🟦️.ts`, readmission `🧪️tests/🟦️.ts`, `🟢️node/🧪️tests/🟦️.ts`
- Scripts left here: `closinv-scan-unknown.ts` (lists names `role()` rejects in an image), `closinv-admit2.ts` (admission probe).

## Tests
cardinality 1/1 (1M entries), owner-peaks-spill 6/6, authority-seal 3/3, readmission 13/13, new session and node-loader law tests pass. The remaining failures in those files need a Script process invocation (`Original parent Script process invocation required`) and fail identically without my changes.

## Streaming authority receipts (coordinator decision: option 1)
- New `closures/🌊️authority-stream/🟨️.mjs` (`createAuthorityStreamV1(maximumUnitUnits,sink)`): a bounded incremental reader of an authority receipt. Every header value and every element of `observations` reaches the sink as soon as its text is complete; memory is bounded by one record (`P.chunkBytes*16` units), never by the receipt. Self-contained so the headroom receipt program embeds `createAuthorityStreamV1.toString()`.
- Read worker (`authorityStream` in `👷️closure-read-worker/🟨️.mjs`): reads the file in `chunkBytes` chunks with the same descriptor/stamp/budget checks, hashes incrementally, validates each observation (keys, kind enum, path) and extracts references per record; `maximumNodes`/`maximumDepth` now bound one record (`references()` runs per record), `maximumReferences` bounds the union.
- Headroom receipt program (`📏️headroom/🟦️.ts`): no `readFileSync`/`JSON.parse` of the receipt any more. Measured on the real 487 MB receipt: 3.1 s, node heap limit 512 MB.
- Test `🧪️tests/🌊️authority-stream/🟦️.ts`: equals `JSON.parse` for every record under 5 chunkings and 3 key orders (quotes, brackets, escapes inside strings), a synthetic receipt over the old node bound (1M+ nodes beyond `maximumNodes`, lazily generated) with a heap-growth bound, and refusal of malformed/truncated/trailing/duplicate-observations/oversized-record receipts.

## Further defects found while rerunning (all fixed)
8. Fleet markers are now `<digest>[:<size>:<mtimeMs>][:closures-1]` (project record port `versionOf`), the worker refused anything but a bare digest. Grammar widened, test extended.
9. Worker followed the generation manifest to `generations/x/<index>.json.br` but the writer stores `<index>.facts.br`; link corrected, law test added in `🧪️tests/🕰️generation`.

## Run history
Runs 1-13 earlier defects, 14 fleet marker, 15 and 17 the `x/*.json.br` link, run 18 (`closinv-wasm-18.log`) passes the whole closure proof and runs the graph read (slow because of fleet inference and concurrent churn); see the final status in the report.

## Receipt bound (leaf check "Owned bounded graph cache leaf required")
- Cause: `protocol.maximumReceiptBytes` of `📏️headroom/🧬️schema/🔣️.json` was 64 MiB while the writer produces authority receipts of 100 to 487 MB; `leaf()` refused them by size (nlink and device checks are fine for these files).
- Fix: `maximumReceiptBytes` is now 536870912, equal to the closure `maximumFileBytes` the streaming reader enforces and to `maximumSnapshotBytes`. `digest()` already reads in `chunkBytes` chunks; `publicationGraphV1` runs the stream program (no whole-file read or parse). Test `📏️headroom/🧪️tests/🧾️receipt-bound/🟦️.ts` (2/2) pins the equality and the absence of `readFileSync` and whole-file `JSON.parse` in the program.

## Run `closinv-sgp-1.log` (repo:test-storage-graph-processes, both fleet-off vars)
Graph read finished (about 60 min incl. waiting behind another load), then EXIT 1 with `Internal error: Directory partition capacity exceeded (reference c420ba58)` during `build:implementation-source`. Cause: the graph image `.cache/b/g` is full: `maximumPhysicalBytes` 12 GiB (12884901888), `g/m` is about 12 GB (inference 4.4 GB, selected 2.8 GB, generations 2.1 GB, derived-nodes 1.2 GB, snapshots 0.5 GB). `selected/` holds 7 stale admission files (300 to 440 MB each) from selections that have no `.authority.json`/`.json` pair (676ca, 7a0e, 8088 and others) which the headroom retirement does not recognize as retirable, plus four full receipts. The suite itself did not run. Needs a decision: retire orphan `selected/*.admission.json` in the headroom, or raise the g image cap; I did not change either.

## Orphan selected admissions (coordinator decision)
- `retireOrphanSelectedAdmissionsV1(image,directory,names,control,held,enough)` in `📏️headroom/🟦️.ts`: a `selected/<scope>.admission.json` with neither `<scope>.json` nor `<scope>.authority.json` is retired. The leaf is observed first, then inside `held(scope,…)` (per-scope writer lease in the nonwaiting recovery; exclusive image custody in the held recovery, where `held` is a pass-through) the publication must still be absent and the leaf stamp unchanged right before the unlink, else `Original orphan selected admission changed`. A scope held by a live writer is skipped. Wired into `recoverBuildGraphImageHeadroomHeldV1` (before the publication pairs) and `recoverBuildGraphSelectedHeadroomV1` (before the pair retirement); both stop as soon as the headroom is funded.
- Tests `🧪️tests/🗑️orphan-admission/🟦️.ts` 4/4: orphan retired, admission with a publication kept (and enough headroom stops), admission held by a live writer or gaining a publication under the lease kept, stamp changed refuses.

## Suite run closinv-sgp-3.log (fleet-off, repo:test-storage-graph-processes)
Graph read completed (about 50 min); then EXIT 1 `Internal error: Directory partition capacity exceeded (reference c420ba58)` again, raised in the client, no service log holds the reference. Images at or over their caps (`.cache/b/*/image.json` `maximumPhysicalBytes` against `du`): g 12 GiB cap, about 12G used (9 orphan admissions still on disk, so the retirement had not run); d 36 GiB cap, about 37G used (dependencies image: bun-cache 149k entries, `j` 726k entries), t 8 GiB cap 6.5G, r 96 GiB cap 56G. The `d` image is not covered by graph headroom at all. The official wasm build was not rerun: it would stop on the same capacity refusal.

## Cache collection and the capacity refusal (coordinator decision: no caps raised)
- `repo cache status`: graph 11.61 GiB of 6.00 GiB declared (recreate pending, the image is still 12 GiB), dependencies 36.00 of 36.00 GiB, rust 68 of 96 GiB. So the graph image is over its declared size: this, not the earlier `du` reading, is the likely source of `Directory partition capacity exceeded` (not confirmed; a temporary `[DEBUG]` trace in `native-build/💾️storage/📁️directory/🟦️.ts` was removed again before any run reached it).
- `repo cache cas gc --dry-run` then for real: 36764 entries, 6514 MiB freed (host CAS only, not the images).
- `repo cache gc --dry-run` refused: `native-fixed-image .cache/b/d` held by another agent's `tools setup all` (pid 3741612, still running after 25+ min); not killed, not run for real. The command that performs the graph "recreate pending" was not found (`repo doctor --fix --dry-run` does not list it); the policy marks it derived.
- New defect fixed in run 4: after the receipts were streamed the proof scanned their references for the first time and refused with "Complete retained closure inventory required" for references to closures that are absent. `danglingReferencesV1` (closures/🟦️.ts) treats a reference as dangling only when its leaf is provably absent (exact missing leaf recorded and rechecked); an existing but uninventoried leaf or an unknown namespace still refuses. Test in `🧪️tests/🔏️authority-seal/🟦️.ts` (4/4).
- Suite run 5 (`closinv-sgp-5.log`, fleet-off) passed that proof stage and was still in repeated graph reads (concurrent source churn from other agents) when I stopped waiting; no EXIT yet.
