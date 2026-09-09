
## Inactive Nx State Cleanup

Native verification continued under shared Cargo locking while available disk fell to approximately 23 GiB. Removed ten coordinator-owned historical Nx workspace-data directories after checking both their exact paths and names against every current process command and environment. No active process referenced them. These contain regenerable project graphs and file hashes; test logs, Markdown reports, Cargo artifacts, the active PDF cache, and every current gate state were retained.

- `🗑️generated/nx-root-gis-map-components-3`
- `🗑️generated/nx-root-gis-map-12`
- `🗑️generated/nx-root-gis-parent-2`
- `🗑️generated/nx-root-flow-recovery-5`
- `🗑️generated/nx-root-gis-exact-native-1`
- `🗑️generated/nx-root-required-option-recovery`
- `🗑️generated/nx-root-gis-terrain-default-2`
- `🗑️generated/nx-root-gis-terrain-components-2`
- `🗑️generated/nx-root-native-recovery-1`
- `🗑️generated/nx-root-block-parent-3`

Observed available-space increase during removal: 1.331 GiB. Concurrent builds can change the value.

## Finished PDF Cache Generations

Removed only the three completed private PDF cache payloads from native invocations 7, 8, and 10, reclaiming 1.61 GiB. Each retained receipt confirmed exit zero and its exact hash; each payload was an ordinary directory containing only the declared PDF publication, with a valid owner marker and no open file handles. The latest completed PDF12 generation, active PDF13 state, shared Cargo, sccache, reports, and raw proof receipts were preserved. No cache restoration acceptance is inferred from this cleanup.

## Additional Finished Coordinator Graph States

Removed nine completed coordinator-only Nx workspace graph states after checking process command/environment references and open files. Reclaimed 1.201 GiB. These were prior Flow6/7/source, failed Semio, finished contract/TypeScript and accepted framework/Run states. Their raw logs and proof receipts, the active library matrix, current PDF state/cache, shared Cargo and sccache remain intact. Exact directory receipt: 🗑️generated/finished-root-nx-state-prune-2.json.

## Inactive Private Graph Recovery

Reclaimed 3.544 GiB from 24 inactive private Nx graph-state directories. Candidates had a project graph, were not cache directories or symlinks, and had no process command/environment references or open handles. Current PDF/library/Flow8/Semio2/GIS6 states were explicitly protected. Executors confirmed old graph states were no longer needed beyond retained raw receipts. Build outputs, cache payloads, shared Cargo/sccache, all raw logs, reports, fixtures and scripts remain untouched. Exact removal/skip record: 🗑️generated/inactive-private-nx-graph-prune-current.json.

## Completed PDF4 And PDF12 Payloads

Removed only successful historical PDF cache payloads17171063120336117952 and2796437571782912483 after checking terminal exit0 receipts, exact PDF version1 owner markers and complete file lists, no symlinks and zero open handles. Reclaimed 1.0711GiB. The current PDF13 baseline, active26 state/cache, shared Cargo and all evidence were preserved. Free space had already rebounded externally before this cleanup; that external reclamation is not attributed to this task. Receipt: `🗑️generated/historical-pdf-cache-prune-4-12.json`.

## Shared Compiler Process Ownership Observation — 2026-09-09 12:24 UTC

PID 62766 is the shared sccache server (PPID 1, PGID 62765), rather than an old Semio task. The previously observed compiler PID 10020 was absent on the follow-up check. The server was preserved. A server environment reference alone does not establish that an old private Nx graph remains active; any graph cleanup still requires task ownership and open-file checks. Observation receipt: `🗑️generated/old-semio-process-ownership-observation.json`.

## Terminal Semio Private Graph Cleanup — 2026-09-09T12:32:56.289659+00:00

Removed only three completed Semio schema-default private Nx graph directories after verifying their terminal failure receipts, absence of a live task environment using each exact state directory, and zero open handles. Shared sccache server environment references were recorded and the server was preserved. Reclaimed 0.314 GiB; observed free space afterward 5.45 GiB. Cargo outputs, current graph/cache lanes and all receipts remain. Evidence: `🗑️generated/terminal-semio-private-graph-prune.json`.

## Superseded PDF13 And Completed Flow Source Graph — 2026-09-09T12:37:22.749333+00:00

PDF26 is now the successful baseline. Removed only the byte-verified PDF13 private cache payload after checking its complete200-file inventory, owner marker, successful terminal receipt and zero open handles. Also removed the finished Flow source-annotation private graph after terminal, process-environment and open-file checks. Reclaimed 0.676 GiB, with 3.4 GiB free afterward. Preserved PDF13 receipts/inventory, current PDF26 payload, active states, shared Cargo and sccache. Evidence: `🗑️generated/historical-pdf13-and-flow-source-graph-prune.json`.

## Historical PDF3 Cache Reclamation — 2026-09-09T13:02:40.446201+00:00

Removed the historical private PDF3 cache hash3618721459512557588 after independent provenance audit, successful terminal build receipt, exact owner/declared200-file closure, zero open handles and a live-task check: the active PDF27 Bun child59735 under Nx57577 had task hash15275070322891442310. Reclaimed573,381,895 bytes; preserved active/current PDF26 cache, shared Cargo and all receipts. Evidence: `🗑️generated/historical-pdf3-cache-prune.json`.

## Host Resource Observation — 2026-09-09T13:17:19.508686+00:00

The host reports32GiB physical memory and10 logical CPUs. `memory_pressure -Q` reported56% system-wide memory free; `vm.swapusage` reported48,128MiB allocated and46,985.31MiB used. This is an observation of the shared host, not evidence attributing runtime to this ticket or a controlled compilation benchmark. No host setting, compiler process or shared cache was changed. Ticket lanes retain two Cargo jobs and their reusable target directory. Evidence: `🗑️generated/native-resource-pressure-observation.json`.

The superseded PDF26 private cache payload4030302576088414454 was removed after PDF35 passed. All200 cached files matched the accepted26 inventory byte-for-byte (576,961,088 bytes), marker ownership and no-extra-file checks passed, lsof found no open handles, and the live36 task held a different key273405952024672359 before and after verification. Shared Cargo, live36 state, current35 cache/publication and every evidence receipt were preserved. Receipt: historical-pdf26-cache-prune.json.

With free space observed at1.9GiB, the redundant PDF35 private cache payload was evicted after verifying all200 files both in the cache and in the retained publication against the accepted35 inventory. This reclaimed578,054,859 bytes. No file was open, and live PDF36 held distinct key273405952024672359 before and after verification. Publication35, its golden inventory/receipts and shared Cargo remain intact. This is cache eviction under actual pressure, not acceptance of36 or a claim of restoration. Receipt: pdf35-redundant-cache-pressure-eviction.json.
