# F100: per-wave retention of the whole read (row-log seal, census, wave release)
## Cause (honest status)
The dominant managed growth (about 250 MB per wave: strings and small arrays, objects flat) is NOT proven from code alone; no snapshot of a late wave exists and signalling pid 56752 is forbidden. The 07:40 profile was taken at 2.8 GB while a read ran, so its retention paths are live state, not a leak. Ruled out by reading: row-log key table is hashed (no key strings), journal rows are streamed, generation carry rows are log ids, module caches are capped or bounded-cache named. Strong lead (F96): witnesses (`typed` triples, `tuples`, `paths`) per module-graph node hold duplicate strings.
## Landed
- row-log `seal()` (drops the key table, 40-80 B per distinct row, after the part is shaped in `shapeComputeResultV1`); `rowLogCollectionsV1`.
- `releaseWaveStateV1` (release module) used by the host wave hooks: observation memory, input memos, interner, shared source files, closure round verdicts (`forgetRoundVerdictsV1`).
- Census names: row logs (key tables / offsets), generation carry (owners, edge-read strings, row lists, names, frontier), inputs memos, observation witnessStrings/fileShaMemo/loadedImplementationFacts, closure verdictSeen. `VERIFIED_CLOSURES` capped (schema `verifiedClosureLimit`).
- F96 lead: witness string table is wave-scoped (cleared in `releaseInferenceMemoryV1`, was only entry-capped at 262144) and the stat `tuples` and field `paths` of a witness are interned through it.
## Proof
`🕰️generations/🧪️tests/🌊️wave-retention` 3/0: per wave sealed 232,812 B vs unsealed 1,776,368 B (key table 1,310,720 B, equal to the closed-form model) = 7.6x; growth per wave flat within 5 %; rows readable after seal; carry census linear with constant bytes per owner. Neighbours: row-log 9/0, streamed-shape 3/0.
## Not verified
Real before/after heap of a whole read; whether per-node witness arrays are retained across waves (not found in carry/owner tables/row logs; witnesses are stored to disk via saveStamp). Next read: read `generation:carry.*`, `graph:rowLogs.*` in `largest collections`.

## Update 16:05: checks and the first waves of the restarted read (coord-graph6.log)
- `repo typecheck` (row-log, release, settle, inputs, new test, host): passed, 0 errors; dangling-imports exit 0 on all 7 touched folders. All four landing locks released.
- Managed MB after each wave settle: 2995 (wave 1), 3026, 3517, 3836 (about +320..+490 MB per wave, same slope as the old read: 2607 -> 3073 -> 3432 -> 3823). The fix did not change the slope.
- The census now names the carry and it is NOT the dominator: carry.upserts 106 -> 114 MB (717 -> 929 owners), wave.frontier 42 MB constant, carry.owners.reads 9 MB constant (78.6k strings), row logs 3 MB (3 logs, all sealed), closure:entries.rows 28 MB. Named collections total about 0.2 GB of 3.8 GB managed; strings 4M -> 12.4M, Arrays 5.1M, Objects 1.8M in 6 waves.
- Only per-wave-growing named item: `observation:scopes` maps alive 2 -> 3 -> 4, each about 44k entries (+1 map per wave), too small (est. 6 MB) to explain 400 MB/wave unless its rows are fat; holder not identified.
- Conclusion: the unnamed ~3.5 GB is outside every collection registered so far (module Maps, bounded caches, carry, row logs). Next step needs a heap snapshot with retention paths of a late wave: start the next read with `SEMIO_GRAPH_HEAP_PROFILE=<dir>` and a lower threshold than 1200 MB would not help (already past); a profile at wave 4+ (about 3.5 GB managed) is what to read.
