# F61 result: owner records after a build-code landing (03:45)

## Verdict
The hypothesis (the owner-record key contains a digest of build-system code) is REFUTED. The "0 present" of the whole read has another cause, narrowed to the take reading a closure store that does not hold entries that exist on disk. No key change was made (nothing to change); a store-keying fix and a diagnostic landed.

## 1. Key composition (code + data)
- Record file name: `ownerCacheKeyV1(root)`. Record validity: header digest `sha256([recordStampV1(), null, projectRecordKeyV1(head, root, json, targets, inputs)])` (`👁️observation/🟨️.mjs` lines 236, 766, 1444), `head = [registryDigest (catalog providers), commandRevision (native only), assemblyAuthority (native/pipeline), "closure-references/4"]` (`📜️contract-reads`, E1 round 3). `recordStampV1` = `semio.graph-record/2`, unchanged since 2026-11-10 (git).
- Code enters only as DECLARED stamps: one `stamp` row `inferrer:<id>` per inferrer in the record (`🔖️stamps`, 18 rows: coordinates ... package-managers; native-manifests only in native records). Stamps are hand-declared protocol versions, not byte digests; the schema last changed 2026-11-15. Files under a stamped fragment or declared infrastructure are never rows. Guard test (existing, 21/22 pass): "no record, parser identity or generation fact is keyed by the bytes of the implementation", "a stamp row holds exactly while the process carries the stamp it was recorded with", "every executable file ... is claimed by an inferrer or by infrastructure".
- Measured on 23 readable records of the live store (`F61/probe.mjs`): 412 stamp rows (18 per record), 0 rows under scheduler/cache/readiness/dashboard code; the only build-library rows are data files the inferrer reads (`input-precision` json, providers `project.json`). E81 measured the same on 422 records (414 rows of the 51 scheduling prefixes, all one cargo Cargo.toml).
- Consequence: no landing in scheduler, leases, cache, readiness, dashboard, graph host or observation moves any owner record key or row. Only a stamp bump (cargo: `native-manifests`, per language) or a change of the registry/catalog providers or of the owner's own inputs does. So the requested key redesign already exists; the oracle for it exists (`🔖️stamps/🧪️tests`, Python oracle `🐍️.py`). I did not add a duplicate. Gap noticed: a stamp is not forced to change when an inferrer's behaviour changes; only the derived inferrers (`input-precision`, `package-managers`) get an automatic conformance stamp. Not changed (design decision of the stamp protocol).

## 2. What actually happens in the current read (pid 86792, scope `selections/8d8f21d7...`, verified with lsof)
- Log: wave 1 147 of 179 present (82 %), wave 2 27 of 36 (75 %), wave 3 0 of 196 present. All 300 `[graph-stale]` events (cap 400) are `kind closure, source take`, 300 x `closure-entry-absent (recipe X, never shipped or taken here)`, 1 x `closure-entry-stale`. Zero are key, stamp or file rows.
- The recipes are per-owner `target-sources` / `native-target` entries (21 599 + 17 765 + 3 430 of 43 005 entries in `.../8d8f21d7.../ce`). All 300 named recipes EXIST in that `ce`, with mtimes from Nov 17 to Nov 26, and `createClosureStoreV1(...).load(id)` loads them (checked by script, value and layers present). So "absent" is a false negative of the take's store, not a missing entry.
- Defect found by reading: `closureStoreOfV1` cached the open store per workspace root only, but `closureStoreDirectoryV1` = `workspaceDataDirectory/ce` resolves through an AsyncLocalStorage (`repositoryBuildGraphDataDirectoryV1`: in a selection `inference/selections/<id>`, outside it the bare `inference` root, which has no `ce`), and a context failure was cached as `null`. The cache is cleared by the memory shrink ("collected and shrunk") and re-opened by whichever caller comes next, in its own context; wave 3 began with such a shrink. Plausible but NOT proven.
- Not the cause (checked): wave 3 scope (`abdc752f`, born 03:01, `forgotten.json` 03:09) belongs to a peer read; the read has `ce/x/rows.dict` of 8d8f open. Generation staleness (22 239 stale facts leave 5 093 of 5 311 projects stale) is the same at the start of wave 1 and wave 3.

## 3. Landed (T/landings.md "Wave 1125 F61", lock graph)
- `script-inputs/🟨️.mjs`: `closureStoreKeyV1` (workspace, directory, mode, limits), `closureStoreOfV1` keys by it and no longer caches an unresolved directory, `bindClosureStoreV1` drops all stores of the root; the take's absent verdict appends `; store: <reason>`.
- `script-inputs/💾️entries/🟨️.mjs`: `store.why(id)` (no entry file with directory / unreadable / format / id / malformed / digest mismatch / value lost / loads).
- Tests: entries 3/0 (new `why`, new store key), moved-row 3/0, answer-validity 10/0, store-limits 3/0, layer-lost 6/0, bundle-canonical 2/0, retention 5/0, dangling-imports 0. Red, not caused here: stamps keyed-by-bytes test (asserts an observation source text `nestedProject?recordStampV1():OBSERVATION_REVISION` that a peer removed), take-release (asserts `look` text in project-nodes).

## 4. Expected effect (honest)
- Landings outside the 18 stamped fragments: 100 % of owner records survive by key (measured by composition and by the test above). A stamp bump invalidates exactly the records of its scope (project/all stamps: every record; native: native records), today none.
- The observed collapse (75-82 % present down to 0 %) is a closure-store read problem, not a key problem. If the cache-key defect is the cause, wave 3 style reads return to the 75-82 % band; if the next read still prints `closure-entry-absent`, the new `; store: ...` text names the exact reason (directory + first failing step). That read must start after the new code (a running read keeps old code; no restart needed or done).

## NOT verified
- No live read on the new code, so the cause is unproven; no measurement of survivors after a real landing.
- Reasoning about the async context of the take is from code, not traced at runtime.
- Could not read 870 of 894 records of the live store from outside a read (issued-image refusal), so the row census is 23 records here plus E81's 422.

## For other lanes
- F27 tombstones and F55 retention touch the same read; do not conflate their effects with this one.
- Scratch (outside tree): `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-1dOdZf/35823786-c219-418d-81dc-737e033e7539/scratchpad/F61/` (scan/probe scripts). Kept: `T/🧰️scripts/F61/land.py`, `land2.py`.
