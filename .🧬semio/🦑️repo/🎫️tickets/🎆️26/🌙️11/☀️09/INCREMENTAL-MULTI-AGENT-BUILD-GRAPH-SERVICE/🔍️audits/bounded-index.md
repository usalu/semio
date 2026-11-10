# Lane `bounded-index`: bounded caches and fact index diet

## Finding

The fact index is not what holds the 2.5-3.3 GB in the planner. The real index of generation 325 (`.cache/b/g/m/generations/x/b0d4…facts.br`: 12.2 MB packed, 61.7 MB expanded, 681,473 rows, 11,662 owners, 47,830 classes, 15.4 M owner ids, 468k strings) costs:

- 592 MB as a full decode (`unpackFactIndexV1`), and the generation store kept up to two of them (about 1.2 GB) in `indexCache`.
- 324 MB as the old lazy view (`openFactIndexV1`), because the class lists were plain JS arrays (15.4 M ids) and rows/texts/facts were cached without bound; 902 MB once every row was read.

## Measured RSS (light harness, `🧰️scripts/bounded-index-rss.mjs`, Bun 1.4.2, GC forced before sampling)

| Mode | Before | After |
|---|---|---|
| open view, nothing read | 324 MB | 191 MB |
| open view, every row and class walked | 902 MB, 4.3 s | 429 MB, 3.9 s |
| full decode (write/patch path, unchanged) | 592 MB | 588 MB |
| store: indexes retained | up to 2 full (about 1.2 GB) | bounded 768 MiB accounted, 2 entries max |

Resident view = packed 62 MB buffer + offset tables + four bounded LRU caches (texts, facts, rows, classes; about 30 MB of live cache at the policy bounds). The remaining RSS after a full walk is allocator high-water, not growth. Whole index stays under 0.45 GB worst case (target 1 GB, acceptance 1.2 GB).

## Changes

- New `BUILD/🧊️bounded/` (schema, `.mjs`, `.d.mts`): `createBoundedCacheV1({name,maxEntries,maxBytes,sizeOf,onEvict})` O(1) LRU (linked list; `Map.keys().next()` eviction was quadratic and made a full walk 10x slower), oversize rejection, single-flight `load` with cancellation, `shrink`, `boundedCacheStatsV1()` (`CacheStatsV1`, registry capped at 256 names), `shrinkBoundedCachesV1(fraction)` for the memory governor.
- `🧮️fact-codec` `openFactIndexV1`: class lists stay delta-varint in the buffer (offset tables only), `rows` and `classes` are read-only array views (Proxy with `get/has/ownKeys`, so every Array method works without materializing), texts/facts/rows/classes in bounded caches whose limits are the schema policy `view`; row paths no longer pass through the process interner; `stats()` and `drop()`.
- `🕰️generations` store: `indexCache` is a `BoundedCacheV1` (policy `indexCacheEntries` 2, `indexCacheBytes` 768 MiB, `indexRowBytes` 1024 for full decodes, document size for views, views are dropped on eviction); views are named `fact-index:<digest12>`.
- `🔌️load`: `pendingCratesReportedV1` (module-level unbounded Set) is a bounded cache (`pendingCrateReports` 4096).
- `⏱️phases`: clock keeps at most `maximumRows` (256) timing rows.

## Tests (all green)

- `repo --service=off tools exec --light bun -- test ./…/🧊️bounded/🧪️tests/🟦️.ts ./…/🧊️bounded/🧪️tests/🔒️lint/🟦️.ts ./…/🕰️generations/🧮️fact-codec/🧪️tests/🟦️.ts ./…/🕰️generations/🧪️tests/🍂️view-release/🟦️.ts`: 86 pass, 0 fail (also ⏱️phases 18, store `🕰️generations/🧪️tests/🟦️.ts` 30, retire/patch-growth/merge/fact-index 16).
- Bounded cache: JSON fixture (`🔣️.json`, 7 traces incl. 3 seeded) equals the Python `OrderedDict` oracle (`🐍️oracle/🐍️.py`) live on 12 random traces of 600 ops; bounds asserted after every op.
- View: equal to the full decode for every fixture (rows, classes), 70k-row/6k-class walk keeps all caches within bound with evictions; mutation refused.
- Lint test: no module-level `Map`/`Set` in `🕰️generations/**` and `⏱️phases`.

## Not done / outside ownership

- Write/patch path (`patchFactIndexV1`, merge) still decodes a full index (588 MB, transient; one may stay cached under the 768 MiB bound). Patching from the view is the next step and needs `🕸️graph/🟦️.ts` (lane 11) to stop republishing whole indexes.
- Other module-level unbounded maps (inference, resident) are listed in `📌️requests.md` under `## → daemon`.
- 2 h soak not run (no daemon yet); bounds are structural.
- Unrelated pre-existing failures seen in the wider `🕸️graph` run (source-shape tests on `🕸️graph/🟦️.ts`, claims kernel, `dirtyPathsFromJournalV1` export, `SEMIO_TEST_ARTIFACT_DIR`) are other lanes' in-flight work.
