# R92: why a closure entry is stale on a worker, and fleet sharing of entries

Status 20:10 UTC: step 1 measured; steps 2 (partly) and 3 (lane, not pool units) LANDED 20:02 UTC (window 20:00-20:05, one lock hold, all 10 test files green incl. both standing preflights).
Scripts: `T/🧰️scripts/R92/` (`classify.ts`/`classify3.ts` the classifier, `mk.py` stage builder, `stagerun.py` runs staged tests against the live tree with absolute imports, `land.py`, `wait-land.py`, `poll.py`).

## 1. Why an entry is stale (measured)

Sample: 320 entries of this machine's closure store (`.cache/b/d`-free path: `.cache/b/g/m/inference/selections/8d8f…/ce`), taken evenly from the entries keyed to the live root; every distinct row of every entry and layer was probed against the live tree with the real source-analysis providers registered (`classify3.ts`, 121 s; my first run without the providers registered called 237 entries stale on a harness artifact and is discarded).

| result | entries | share |
|---|---|---|
| hold (fine) | 96 | 30 % |
| stale | 224 | 70 % |

Stale entries by the set of reasons that fail (an entry is counted once; weight = layers, a proxy for derivation time):

| failing rows | entries | share of stale | layers (weight) |
|---|---|---|---|
| analysis row, source of a script changed + dependency-generation path | 109 | 49 % | 9 286 |
| analysis (source changed) + file digest changed + generation path | 63 | 28 % | 25 641 |
| generation path only (`typescriptRealpath` row names `.cache/b/d/m/j/<old id>/…`) | 24 | 11 % | 7 767 |
| analysis, source of a script changed, alone | 18 | 8 % | 4 647 |
| file digest changed + generation path | 5 | 2 % | 11 |
| analysis + file digest changed | 5 | 2 % | 1 051 |

Reading:
- 200 of 224 stale entries (89 %, 84 % of the weight) contain a file that really changed: the closure of a `repo …` command reaches the build library's scripts and modules, which the fleet edits continuously and which landings change. This is legitimate staleness by content: no key or validity rule may keep these entries. What is wrong is that EVERY machine derives each of them again (the 30 to 60 s per Rust crate): the same derivation ran on all eight machines, and again after each landing.
- 24 entries (11 % of stale, 16 % of the weight) fail only because the rows name the previous dependency generation directory (current `d22a22b6…`, the rows say `e5027835…`): a reinstall renames the directory, the facts were about unchanged files. Fleet-taken entries do not suffer from this (the portable form replaces the single generation id by a token and the taker puts in its own). Not fixed here, see section 5.
- The `⟦analysis-owner⟧` token (R87: 291 of 303 stale entries) is down to 1 entry of 320 after the R87 fix (it remains only in entries stored before the fix); 0 of the 320 fail on a stamp, an mtime or an inode (stamps are zeroed in the neutral form and a restamped file is proven by its digest, law below).
- Machine paths: 6.59 M of the sampled rows name the absolute root, 1.05 M carry stamps. These are tolerated: the relocation layer (root rebase) and the fallback to the digest make them valid by content. What is NOT portable is the store KEY: `produceClosureEntryV1` names an entry `<workspaceRoot>\0<recipe key>` and the recipe text itself names the root (commands, env). 823 of the 1 143 ids sampled in the store (72 %) are keyed to the previous root `/Users/Shared/semio` and can never be loaded again since the move of 14:50 UTC: every machine re-derived everything after the move and keeps the dead copies.
- Worker view (slice-memory `closureCounts` at 19:54 UTC, before the landing): stale share of the entries checked: m01 5.7 % (179 of 3 136; derived 439), m02 14.7 % (555 of 3 770), m03 15.4 % (1 364 of 8 858), m08 32.8 % (424 of 1 291), m10 5.3 % (178 of 3 370; derived 309; taken 444 of 476). The small stale share hides the cost: a derived entry costs 30 to 60 s of a Rust crate (R87/R89), a loaded one 25 ms.

Share of worker time per reason (stale entries are the only ones that derive): content change in the closure 84 % of the derivation weight, dependency-generation spelling alone 16 %, token under 0.1 %, stamps/mtimes/inodes 0 %, fact revision of the machine 0 % (no row carries one), volatile layer data 0 % (layer digests are over the rows, the sampled layers re-verified).

## 2. Landed (20:02 UTC, `T/🧰️scripts/R92/land.py`)
- New member `🪪️graph-state/📦️project-records/🚚️entry-lane` (schema first, lane, 5 laws).
  - `closureEntryKeyV1(recipe, root)`: digest of the recipe with the root token. Same key on every checkout path, equal to an independent computation and to the portable layer's `neutralBundleV1().recipeKey`; a Windows root spelled with forward slashes gives the same key. The local store id still names the root (disk layout, not shared); the SHARED name is the neutral key.
  - One fleet state per entry (slot per key). `take` = decide (fetch / claim / wait for the single holder, 120 s per entry, 30 min budget per process, then derive locally), `offer` = head swap after a derivation, `prefetch` = bulk fetch of the entries a store lacks.
- `script-inputs/🟨️.mjs`:
  - `closureTravelV1.import` replaces a stored entry that no longer holds when the fleet bundle holds here (BEFORE: any present entry, however stale, blocked every bundle: this is why entries taken from the fleet stayed stale).
  - The flight of `produceClosureEntryV1` asks the fleet before deriving, derives only as last resort, and offers what it derived (waits at most 15 s so a recycled warm child still publishes).
  - `settle` prefetches an owner's missing entries in one batch before any derivation. `closureCounts.fetched`. `SEMIO_CLOSURE_FLEET=off` disables.
## 3. Laws (`entry-lane/🧪️tests`, 5 pass in the lock; staged run before)
key portable across paths; second root with other paths and mtimes takes the first root's entry with zero derivations, cold third root takes it in bulk; stale stored entry replaced by a holding fleet entry and left alone when the fleet entry does not hold here (RED on the old script-inputs, green after); restamp and unrelated edit keep entries, a closure-file edit invalidates exactly its entries; a cold reader's `settle` takes the entry from the fleet with `derived` 0 and `fetched` 1.
## 4. Read back from workers (20:09 UTC)
m02, m03, m07, m08, m10 run the new code (`fetched` field present, 0 everywhere): nothing is fetched yet because no entry has been published; workers restart every few minutes with landings and had not yet derived anything under the new code (m03: 30 derived, 4 056 checks, stale 14 %). No derivations-per-try, fetched-entry or finish-time numbers after the landing exist yet; m10 was at 444 of 476 owners taken before the restart, and no Mac is `ready: true`.
## 5. NOT done / NOT verified
- Owner pool deriving entries as claimable units (heavy ones on machines that fit): not built. Entries the pool derives while inferring owners are claimed and published by the flight, nothing derives entries nobody asked for.
- Claim/derive/offer path in a real derivation (the laws offer pre-made entries); fleet size and publish cost of large entries untested; the lane's publish/refused counts are not in slice-memory.
- Dependency-generation neutralisation (11 % of stale entries): not landed; rebasing only the rows would let an entry survive while its file lists still name the old directory.
- Root-free LOCAL store ids and reuse of the 72 % old-root orphans: not done.
- Windows/Linux not run.
