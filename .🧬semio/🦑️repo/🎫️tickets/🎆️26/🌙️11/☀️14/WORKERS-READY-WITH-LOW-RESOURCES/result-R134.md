# R134: fixed caps below the workspace

Landed 2026-11-17 16:30 UTC (landings.md entry "R134 landing"). Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R134/`. Declaration of the workspace (owners 4774, closure rows 440 000, facts 330 000, tracked files 280 068, safety factor 2): new member `🏗️build/📏️workspace-scale/🧬️schema/🔣️.json`. Law: three tests appended to `⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📈️growth/🟦️.ts` (every member holds size x factor; equal-document group; scan of workspace-named ceilings below owners x factor, exemptions with EN and DE reasons).

| Gate | Old | Proven to trip (red on old tree) | New rule | Landed |
|---|---|---|---|---|
| FX-03 entry documents (GS `entryMaximumDocuments`, Note `documents`) | 4096 | law member: 4096 < 19096 | 32768, one value with generation `maximumDocuments` (law group) | yes |
| FX-09 generation `maximumDocuments` | 20000 | law: unequal and below | 32768 | yes (`maximumClosures` 20000 >= 19096, unchanged) |
| WF-02 derived nodes | 262144 | law: < 660000 (facts x 2) | 1048576 | yes |
| RS-10 output signature | 2048 entries, task re-runs | state test: 3000 outputs, `undefined` on old; python sha256 oracle | streamed hash, no ceiling, schema key removed | yes |
| WV-21 selection records / aggregate | 2048 / 64 MiB | law | 16384 / 512 MiB | yes (tooling identity of the authorizer changes) |
| CS-01 closure store | layers 262144, values 8192 | closure-store-limits test (values 8192 < 57048) | layers 524288, values 131072; entries 131072 kept | yes |
| CS-02 eviction of live records | sweep removed the oldest regardless of age | sweep test: 6 records past maximum 3, old keeps 3 | sweep never removes a record younger than the grace (store may exceed the maximum meanwhile) | yes |
| WV-14 cutoff suspects | 65536 (refusal) | law (57k entries x2) | 131072 | yes |
| OW-13 fresh rows | 400000, a 440001-row record counted fresh unchecked | fresh test, 440001 rows, python count | 1048576 | yes |
| FX-05 cursor rows | 400000 (full verify walk) | law | 1048576 | yes |
| RS-11 resident digest files / journal / memo | 262144 / 262144 / 65536 | law | 1048576 / 1048576 / 262144 | yes |
| WV-02/03 wanted projects | 8192 | law | 16384 | yes |
| FX-07 unsent spool entries | 24 | unsent test: 96 records, old keeps 24 | 16384 | yes |
| FX-12 cache server 2.5 GiB | 2684354560 | none | unchanged | no |

## Not done / unverified
- FX-12: a byte budget with LRU; the live fleet store holds 107 MB + 275 MB (mirror 865 MB, own limit). Whether a full generation plus records exceeds 2.5 GiB is not measured; no leases for eviction were added.
- `tasksPerOwner 16` and `closureRecordBytes 32768` in the declaration are estimates (marked "not measured"); entries/layers per owner come from the older closure-store law.
- Not changed: script reuse `maximumFiles 8000` (cache that clears), fact codec caches, IM-11 2000 files per crate, DP-07 524288 receipt rows (scale with installed packages), 1024 explicit project lists (exempt with reason).
- Seal selection test under the harness was not run under the lock (tooling identity changes). Generations and the whole inference tree were checked on staged copies only (generations none worse; inference battery stopped for load, only the closure-store group ran).
- Under the lock two peer tests were red without my files (`wanted/💰️cost measured-only`, `resident provision`) and `resident restart` is flaky under load; they are reported as foreign.
- Live reads and services run old code until restarted.
