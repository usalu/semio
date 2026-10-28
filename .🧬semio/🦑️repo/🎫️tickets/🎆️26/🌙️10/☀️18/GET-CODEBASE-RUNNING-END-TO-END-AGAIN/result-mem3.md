# result-mem3 (wave 1027b, lane mem3, mirrors m3A-m3C, dropped): where the graph parent's heap goes during a refresh, and the release that did not release

FOR coordinator: batch ready `C/mem3-tools/staged/releaseturn/` (4 files: `🧠️memory/🟨️.mjs` + `.d.mts` (new `releaseAfterTurnV1`), `🕸️graph/🟦️.ts` (host `release` goes through it, 2 anchors), new law `🧠️memory/🧪️tests/🧎️release-turn/🟦️.ts`; base = live 12:26 10-28, after the langpick landing; rebased once with mkbatch). The inference caches were released in the SAME tick that ended the inference; the runtime keeps the observation scopes / async contexts of a finished async run referenced until the event loop has turned once, so that release freed only part of what it names. The batch awaits one turn (`setTimeout 0`), releases, and collects last (also when the release throws). Apply `python3 -I C/mem3-tools/staged/releaseturn/stage.py apply`; laws `BUN=<bun> sh C/mem3-tools/staged/releaseturn/verify.sh <root>` (mirror m3C = live 12:26: new law 6/6 (three repeated runs 6/6), early-release 2/2, memory 🟦️.ts 9 pass / 2 fail with the SAME 2 failures on base (the mirror lacks the oracle `⚙️runtime/🗣️language/🧪️tests/🐍️.py` of the landed language module); RED on base: `Export named 'releaseAfterTurnV1' not found`); revert `python3 -I C/mem3-tools/staged/releaseturn/stage.py revert`; rebase after a peer edit `python3 -I C/mem3-tools/mkbatch.py C/mem3-tools/staged/releaseturn --root <tree>` (3 anchors in 2 files + 1 in the d.mts, each exactly once). Parse scan (`gate-g-tools/scan.ts`) 3 files, bad 0.
FOR coordinator: product proof = the next refreshing print after a library edit: the graph child's heap after the release (`[graph] released` is not logged; compare the child's peak footprint of the post-inference phases in `product-fp.py g<NN>-rt1 @semio-tech/print-rs:test`): expected -0.3 GB live from the release on with 12 projects (measured) and more with more projects (mem2 saw 1.0 GB pinned at 36; the 36-project harness runs of this lane were killed at 35/36 before their release step, so 36 is NOT re-measured), i.e. during shaping, index patch, pack and assembly; the inference peak itself is unchanged (see section 2).

## 1. The finding: the "1.0 GB that no release frees" (mem2 section 3) is the same-tick release
Harness `C/mem3-tools/` = mem2's captured-snapshot harness (production parent loop, scratch fact store, JSC RAM 2 GiB) rebuilt on today's live (loader anchors re-cut for the `refusal(...)` loader), plus: `Bun.generateHeapSnapshot("v8")` hook (`M3_SNAP`, `M3_SNAP_PRE`), settle hook (`M3_SETTLE`: 5 immediates + 300 ms), production-order release hook (`M3_PROD`, `M3_TURN`), step-wise release patches (`harness/steps.json`), interner/inventory stats, script-cache census (`M3_SCRIPTS`), type-count series (`M3_TYPES`).
| release applied after the 4 heavy projects (framework, framework-rs, print, plugin-registry), live MB after a full collection | live |
|---|---|
| end of inference, before any release | 1,047 |
| mem2 order (entries, scripts, observation memos, inputs, interner), SAME tick | 997, 997, 892, 882, 877 |
| the same order after one event-loop turn (settle first) | 925, 740, 418, 365, 265 |
12 projects, the production release (entries, observation memos, input memos, interner; scripts stay bound as in production): 1,213 before; **1,017 after a same-tick release; 721 after the release that follows one `setTimeout(0)` turn** (the same 721-738 is reached when the turn comes after the release, so the rest is not leaked, it is pinned); a single `setTimeout(0)` is enough (1,214 -> 721).
Mechanism, reproduced without the library (`🧎️release-turn` law, 5th test; ALS run whose store keeps an object): the store of a finished `AsyncLocalStorage.run` async function and what its pending continuations captured stay alive through `Bun.gc(true)` in the same tick and are dead after one `setImmediate`. The observation scopes (`scopes.run(nested, ...)`, the capture maps) are exactly such stores. So mem2's attribution (x1: 1,465 MB live after every release, vs 451 MB for a 1-project run) measured the pinned frames, not a leak: nothing in the library holds the ~1.0 GB; one turn frees it.

## 2. Where the parent's heap goes (measured, deterministic live numbers after a full collection)
Live end of inference is repeatable to 1 MB (1,047-1,048 MB, five runs of the 4-project set), so A/B edits are measured on it; peak RSS varies +-5 % with host load (17-30).
| 4 projects (print, framework, framework-rs, plugin-registry), step-wise release after a turn | MB freed |
|---|---|
| closure entries (`resetClosureEntriesV1`, 295 entries) | 122 |
| closure scripts / script-input cache (`cache.files/closures/declared`, 15.4k memo entries `{value,observations}`; production NEVER unbinds it: needed by the late closure verification) | 185 |
| observation row table + derived-text WeakMaps (`rowTable` step) | 164 |
| settled reads (cached file bytes, 6,122 reads) | 57 |
| project publications / used inference / registry rows | 86 |
| verified blocks, analysis images, file images, project images | 17 + 14 + 1 + 1 |
| input memos (inventory, verified closures, chunk cuts) | 53 |
| string interner (155k entries, 24M chars, 0 clears, 48k hits) | 100 |
| floor left (modules, TypeScript state; includes the harness' own 465k `git ls-files` list, ~130 MB) | 263 |
Scaling: 12 projects = the same table +165 MB (framework alone +563 MB live; it builds the command closure every other project reuses; the small projects after it add 0-20 MB each, os-mcp/plugin-registry/os-dev +70-85 MB, print +3).
- RSS is not the footprint. The 12-project run peaks at parent RSS 3.4-3.5 GB and physical footprint 1.5 GB (print step: heap 1,471 -> 2,030 MB incl. garbage, footprint 1,054 -> 1,517); mem2's 36-project b1: RSS 3,146, footprint 2,657. `ps` RSS counts freed-but-reclaimable pages (buildmem item 2: the capacity sampler and `product.py` over-state the builder by 2.3x).
- JSC RAM size is not a lever any more: 12 projects at 1 GiB 3,202 MB peak RSS, 1.5 GiB 3,412, 2 GiB 3,261-3,473 (live end 1,213-1,215 in all).
- The snapshot cannot attribute Map/Set contents (JSC records no edges for Map/Set/WeakMap entries: 364k zero in-degree objects, 173k zero in-degree strings), so retainer chains stop at the container; sizes by cache come from the clear-one-cache A/B above. Heap snapshot analyzer: `mem3-tools/heapan.ts` (streaming parser, dominators with a super root for the missing edges, Map/Set holders, duplicate-string census, module/lexical environment reach).
- String-cell "self sizes" in the snapshot count the parent buffer for every substring (139 cells of `🧪️tests/🦀️.rs` = 12 KB each), so "duplicate string" totals (128 MB of 325 MB) over-state; interning the resolved import paths and closure names was measured useless (live end 1,047 vs 1,048; control `c3`), and a 4M-character interner bound too (1,052; `c5`).

## 3. Derived-text WeakMaps (measured, not staged)
`observationIdentities` (`kind\0path`), `observationMeanings` (JSON of the fact) and `observationJsons` (JSON of the row) cache one string per row for the whole life of the row. Not caching them: live end 1,048 -> 944 MB (-104 MB, -10 %), 4 projects: identity -17, meaning -44, json -44; wall not slower (62 s vs 72 s under load 17-30). Not staged: the row JSON cache exists to make the header of thousands of projects that share rows cost concatenation, so the right form is a bounded text cache (clear at N characters, like the interner), which needs its own law and a many-project timing proof. Next item if the gate wants a further -100 MB per 4 heavy projects.

## 4. Runs that did not finish
Two 36-project harness runs (turn on/off, in parallel, RSS 3.0 GB each, footprint peak 2.98 / 2.78 GB) ended without an exit record after the 35th project at 12:23 (no H-end, no json: killed from outside, probably the runaway watch above 2 GB); partial marks: footprint peak 2,979 MB with all 36 projects in one process, matching mem2 b1 (2,657). The A/B of interning the derived texts (`c6`) failed to start (the live tree moved a file under the captured image: mirrors go stale within an hour on this tree).

## 5. Not done / next
1. The inference peak of the in-process parent is live (closure scripts 185-545 MB, entries 122-749, observation memos ~340, interner 100) + garbage; none is a leak. Cutting it needs the farm to take the first heavy projects, or compact forms of the closure entries (295 entries of 245 KB = 72 MB) and of the scripts memo entries (15.4k x 5 KB).
2. Footprint-based capacity accounting (buildmem item 2).
3. The product proof of releaseturn.

## Files
Tools `C/mem3-tools/`: `harness/{entry.ts,run.ts,mkpatches.py,steps.json,expose.json,tsgc.json,c1*.json,c3.json,c5.json}`, `prep.sh`, `run.sh`, `loadermarks.py`, `heapan.ts`, `mkbatch.py`. Staged `mem3-tools/staged/releaseturn/{edits.py,new/,files/,base/,manifest.json,stage.py,verify.sh}`. Outputs `C/🗑️generated/mem3/` (images, logs, json, heap snapshots; deletable).

## Status
12:28 DONE: releaseturn staged + mirror-verified, measurements complete; mirrors m3A-m3C dropped, snapshots/images deleted; no process of mine runs; nothing live edited.
