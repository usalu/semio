# F55 result: parent heap growth of the whole read (02:40)
## Findings
- The running read (pid 8462, started 01:24) logs NO `largest collections` line, although F36's code is on disk since 19:31: `reportRetainedGraphMemoryV1` returned early through `quiet(env, limitBytes)`; the whole read has no plan limit (limitBytes Infinity) and no SEMIO_GRAPH_PHASES, so F36's diagnostics were silent by design. Fixed (always prints, one line per wave).
- Heap census series of the live log (after forced GC): managed 3.27 GB / external 1.74 GB / 36.4 M objects / 15.9 M strings -> 3.90 / 2.03 / 46.2 M / 19.3 M over the last ~7 waves; process 5.4 -> 6.6 GB (grant 3072). Strings and arrays dominate: live row data, not garbage.
- Retainers found by code (cannot be measured without a heap snapshot of the live process, signalling it is forbidden, and a scratch whole read does not fit: 10 GB available, load 54):
  1. `discoverBuildGraphPartV1` returned `dispose: () => rowLog.close()` created inside the read function; `carry.disposers` keeps it to the seal, so every wave pinned that function's whole scope (stubs of all owners, `nodeRows`, `derivedFacts` incl. the re-derived test facts with row support arrays, `part`, `files`). Fixed: module-level `disposerOfRowLogV1`.
  2. Every wave's row log (kept to the seal, ~110 waves for 5300 owners) kept its unflushed text (up to 4 MB of JSON line strings) and offset/length typed arrays with doubling slack. Fixed: `rowLog.settle()` (flush + trim) at the end of each part read. The key table (20 B x slack per distinct row) stays; it is the next candidate if `external` still grows (name it from the new line: `memory:...`/`part:nodeRows`).
- Not proven: before/after parent heap per wave and identical sealed result (no read available to run both; code lands for the next read). Tests: row-log 9/0 (new settle test: rows readable after settle, later rows join), flat-memory 1/0, settle 5/1 (known flaky REAL heap).
## Owners per minute: farm not used
Every wave logs `graph: inferring 48 projects in this process (memory)` (8 times in the log; also `251` at start): the project farm is declined because the parent already holds more than its grant, so 48 owners are time-sliced on one thread; 0 farm workers. So growth causes slowness: flat parent -> farm engages again. Also load 54, 10 GB free, kernel time dominated (F30).
## Next read: check
`largest collections` per wave; if a name grows 100+ MB per wave, fix by it. Expect farm lines `workers` > 0 once the parent stays under the grant.

# Round 2 (03:30)
## What I could not do
- No real snapshot of a scratch whole read: there is no standalone way to run a real read apart from the shared graph image (every test harness is mocked), and a 3-5 GB snapshot by `Bun.generateHeapSnapshot` is itself multi-GB. So the dominator is NOT proven, and per-wave flatness is NOT proven.
## Evidence from the live log (coord-graph2.log)
- The first wave (251 owners, in process) already leaves 5.6 M strings / 1.8 M arrays / 0.9 M objects at 1.5 GB managed (cold boot: 0.2 M strings); later +2-4 per cent per wave; the counts grow together (strings > arrays > objects, 300-650 k JSLexicalEnvironments), the ratio of a pool of parsed syntax trees or per-owner records, not of row data (row maps sum to 60 MB).
- Strongest unregistered candidate in code: `sharedSourceFilesV1` (up to 8000 parsed TypeScript source files, trees with identifier strings, node arrays and node objects). It is cleared only by `releaseOwnerStateV1` every 32 finished owners (or on shrink), never by the wave release, and it is absent from the collections line, so it cannot show up there.
## Landed (next read)
1. Census row `typescript:sharedSourceFiles` (entries, text bytes x 32 estimate) in the largest-collections line: names or clears the candidate in one wave.
2. The shared parsed trees are cleared at every wave release (3 sites in the host). Costs a re-parse of the lib files per wave if it was not the retainer.
3. `SEMIO_GRAPH_HEAP_PROFILE=<dir>`: first wave past 1200 MB managed writes `heap-profile-<pid>.json` (classes by bytes; deepest heavy retention chains with root paths and string counts). Tested on a real snapshot (names `hold:Map`). Limit: JSC snapshots do not enumerate Map/Set entries, so a Map's mass appears as one `Cell Butterfly` under its property name, which is still the retaining name.
## Next
Start the next read with `SEMIO_GRAPH_HEAP_PROFILE=<dir>` (parent only, one snapshot, expect a stall of tens of seconds and extra GBs at that moment: do it with 10+ GB free) and read `largest collections` for `typescript:sharedSourceFiles`.

# Round 3 (05:25)
Source: `heap-profile-14256.json` (managed 2997 MB; path names are local variable names of closures).
- `reuse > holds > pending > entry > carry > exact` (85 MB, 347 k strings) and `pending.levels` (27 MB) and `deriver > scripts > pass > dirty` (21 MB) are in `🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs` `journalGraphSeamsV1`: `pending` is the cursor pass state (`entry.carry` = the owed exact paths of the stored cursor, `levels` = memo path -> doubt level), `memory` Map keeps `{cursor, carry}` per generation for ever (grows by one carry of up to 400 k path strings per verified generation: the per-wave growth), `levels` grows with every distinct row path judged.
- `state > cache > generatedOutput > options > onOwnerObservations` (27 MB) is `scripts.generatedOutput` in `🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs` (2 sites): the arrow closed over the scope of the read function, so the shared script cache pinned the options and the wave's observations.
## Landed
1. `memory` bounded to the newest `maximumCarriedCursors` (2); `published()` only needs the pass base, `lookup` falls back to the store.
2. `levels`/`fileLevels` cleared at 100000 entries (re-derivable).
3. `generatedOutputReaderV1` module-level factory for both sites.
Not changed: `entry.carry.exact` of the live `pending` (85 MB, constant per pass; `published` re-stores it, so it is needed); 514 MB unattributed root and `FunctionCodeBlock > JSModuleEnvironment` (63 MB) are module-level state, untouched.
## Proof
Not run: no scratch read possible. Next read with `SEMIO_GRAPH_HEAP_PROFILE`: expect the `memory`-map growth and the `generatedOutput` path (27 MB) to disappear and the `levels` path to stay under about 10 MB; compare `managed` per wave. Tests: graph-hooks 49/0, pass-retention 1/0. `repo typecheck`: 2 errors, both in peer test files I did not touch.
