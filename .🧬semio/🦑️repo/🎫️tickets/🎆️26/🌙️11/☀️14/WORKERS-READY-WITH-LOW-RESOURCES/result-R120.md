# R120: the cold accept path of the reader (fleet entry unpack, forced collection), landed 08:30 and 09:00 UTC

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R120/` (`patches.mjs` anchored edits, lists `all` and `second`; `overlay.ts` / `overlay-base.ts` run any script against the patched or the original sources; `ot.sh` test through the overlay; `battery.sh` the 23-file test list through the harness; `land.mjs` `restore.mjs` `window.sh` `window2.sh`; `run2.sh` + `accept.ts` the R119 accept path with a private empty closure store under `BUN_JSC_forceRAMSize` 1 GiB and `/usr/bin/time -l`; `bench.ts` the deterministic cold intake benchmark; `prof.py`). Own children only.

## 1. What the 84 s of unpack and the 37 s of collection were (measured, `bun --cpu-prof`, cold `trinity-jack-rs`, 104 s)
- The intake of an entry unpacked every document of the entry set TWICE per `take` (`unpackOf` once to name the recipe keys, once to localize): `JSON.parse` 19 s, `Buffer.toString` 9 s, `unpackBundleV1` 6 s, `localBundleV1` 12 s (string moves of value and sidecar), store `save` 8 s (layer digests, brotli, writes).
- Worse, the same neutral document came back again and again: the shared entries an entry names travel inside EVERY entry that names them, each time with other sidecar bytes (the walk facts of that publisher), so the held-by-digest check never matched. Instrumented, one cold owner unpacked 17 distinct documents 68 times (97 MB of JSON, 6 376 layers, 80 MB of sidecar in the largest). Every `take` also held the decompressed bytes of all documents of the entry at once.
- The 37 s / 13.7 s of forced collection is `releaseIfGrownV1({every:16})` in `yieldInferenceV1` (footprint governor): a full collection, `Bun.shrink`, a second collection whenever the managed memory grew by 256 MiB, rate limited only to 10 percent of the wall time. Without a gate (the accept script has none) nothing else bounds it, and the intake of one document grows the heap by hundreds of megabytes. Measured the other way: with the release switched off completely the same cold owner reached a physical footprint of 26 GB (16 GB under a 1 GiB collector target), so the release cannot simply go; it has to wait for pressure.

## 2. What landed
1. Entry lane (`🚚️entry-lane`): documents are read one at a time (`defer`, never all decompressed together); the recipe and the owner mark come from the first bytes (`closureDocumentHeadV1`, a scanner over the first kilobyte, no row parse) so each document is parsed ONCE; a shared document whose neutral key the store holds (`travel/n-<key>` -> local key, entry still holds) or refused less than ten minutes ago is neither downloaded nor imported again whatever its bytes; per-document claims (`travel/c-<key>`, taken in the order of the note so two processes cannot wait on each other crosswise, stale after 2 min, waiters give up after 1 min and take the document themselves) so concurrent processes of one store unpack a document once; the same head read and neutral key in the project record lane (`📦️project-records`); a fleet bundle whose own rows do not hold here is not written to the store (every consumer would derive it again anyway).
2. Forced collection: `yieldInferenceV1` inside `runClosureTakeV1` releases only after `takeReleaseGrowthBytes` (1 GiB, schema `🦶️footprint`, four times the inference's 256 MiB) and still only when the gate does not read "run"; so accepting owners forces no collection while the growth stays below it, a larger growth collects once per that growth, never per owner. The paced collectors (`createParentCollectorV1`, the gate's `settle`) are untouched: they are the once-per-pressure-event path.
3. Not done (decision): storing entries in a neutral form that needs no rewriting on arrival. The store layers are content addressed by their local text, so a neutral store would change the layer digests, the layer index, the verdict files of R119 and the observation module's row identity (R118 and R121 work there); the redundant work (4x, 2x) was far larger than the per-document rewrite, so it came first. The next step that follows from the profile is a hold pre-check from the table/lists prefix of a document (`rows.table` precedes the 80 MB sidecar in the JSON), so a stale document costs a decompression and a scan and no parse.

## 3. Numbers
Deterministic bench (`bench.ts`: one top entry per take, each naming 8 shared entries of 20 000 rows, 200 layers, 6 MB sidecar with other bytes per publisher; cold reader, 6 takes; `BUN_JSC_forceRAMSize` 1 GiB):

| code | CPU of 3 takes | first take | later takes | imports | peak footprint |
|---|---|---|---|---|---|
| before | 16.0 s | 1.1 s (writes) | 4.5 to 9 s each (each re-fetches and re-imports the 8 shared documents) | 27 | 555 MB |
| landing 1 (08:30) | 4.7 s | 4.6 s | 8 ms | 11 | 338 MB |
| landing 2 (09:00) | 1.2 s | 1.2 s | 8 to 11 ms | 11 | 548 MB |
(6 takes: before 145 s CPU for the earlier prefix-layer fixture, 28 s per take; after 13.6 s, 8 ms per later take.)

Real fleet, original code against landed code started together at 09:02 (22 owners from the 10/11 wave with a record, private empty store, gate-less accept script, 1 GiB collector target), fleet state moving under both, so read the direction, not the digits:

| | 21 owners CPU | `stdio-dwg-rs` (cold) | next owners | peak footprint | closure entries fetched / stale |
|---|---|---|---|---|---|
| before | 130.7 s | 39.1 s | 9 owners, 79 s (1.9 to 21.5 s each) | 2.6 GB | 70 / 41 |
| after | 44.6 s | 19.0 s | 6 owners, 13.7 s (0.0 to 5.3 s each) | 1.3 GB | 3 / 4 |
Earlier samples of the same script before any change (fleet more stale): `stdio-dwg-rs` 174.6 s CPU, `trinity-jack-rs` 29.7 s warm and 92 to 104 s cold (profile above), peak footprint 2.6 GB.

Laws (red before, green after, all under `🧪️tests` of their member): `🧯️take-no-forced-collection` (4: take pass with less growth than declared forces none, a growth far beyond it collects once per growth and at most once per owner, the inference keeps its release), `🚚️entry-lane/🧪️tests/🪢️shared-intake` (4: shared document with other bytes not fetched or imported again, two lanes of one store import each document once, each document parsed once, a document that does not hold here is not stored nor taken again while the refusal is fresh), `🧳️portable/🧪️tests/🔖️document-head` (3: head equals an independent full parse for recipes with braces, quotes, escapes, emoji; fallback and refusals). Battery of 23 files green at both landings and identical to the 08:22 baseline: `🪪️graph-state/📦️project-records/🧪️tests/🟦️.ts` has one failure before and after (not mine).

## 4. NOT verified / open
- The target "under 5 s CPU for the first owner using an entry, under 0.2 s for later owners" is shown on the bench (1.2 s for 8 documents of 20 000 rows and 6 MB sidecar; later takes 8 to 11 ms), not on a real owner whose entries hold: in the fleet of this morning every sampled owner's fleet entries were stale against this tree (accepted 0 of 22 in both runs, verdict false), so the real trinity case (17 documents, 400 MB of JSON) was not re-measured after the change; a document that holds is still parsed, localized and written once (about 0.15 s per MB measured on the real run, 0.05 s per MB on the bench).
- Stale documents still cost one decompression and, until the pre-check of section 2.3 exists, the head read plus (landing 2) the hold check of their own rows; the store and its layers are not touched.
- Concurrency claims were tested with two lanes in one process and a real server; two real processes, a crashed claim holder (2 min take-over) and Windows claim directories were not.
- Memory: peak footprint did not grow in the real run (2.6 -> 1.3 GB) or the bench; a gate-less process under a larger heap target than 1 GiB was not measured (the ceiling of 1 GiB is the bound there).
- `createParentCollectorV1.afterProject` and the gate's `settle` still force collections (paced by duty and interval); whether they fire per owner on a 16 GB worker was not measured.
- Servers keep running old code until they restart.
