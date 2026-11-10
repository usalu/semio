# November 7 Dependency Child Sample

A normal 768 MiB, one-CPU `repo capacity run` admitted the retained read-only `nov07-dependency-child-probe.ts`, which invokes system PowerShell through the repository `runCmd` interface. Its command completed exit 0. The process snapshot confirmed writer 12572, parent 6064, Bun image, exact birth 2026-11-07T02:43:01.3718540Z and captured Source hash 35fff2ced85c1fb351c3e6451b210b175ad2e025bd7a2ed0db113b9a8e15aeb0. Command lines were used only for validation and sanitized role classification, not emitted.

At 2026-11-07T03:45:08Z after the bounded sample, the earlier depth-four tree contained:

| PID | Parent | Birth UTC | Image | Depth | Sanitized role | Initial CPU ms | Initial read / write bytes |
| --- | --- | --- | --- | --- | --- | ---: | --- |
| 12572 | 6064 | 02:43:01.3718540 | bun.exe | 0 | install | 1,225,781.25 | 101,096,746 / 6,322,146 |
| 29576 | 12572 | 03:39:36.0646050 | bun.exe | 1 | install | 8,593.75 | 186,420,211 / 185,276,035 |
| 5116 | 29576 | 03:39:36.0783850 | conhost.exe | 2 | other | 15.625 | 0 / 0 |

The one ten-second follow-up revalidated child 29576 birth and parent. Its CPU rose 125 ms, read and write bytes each rose 665,036, and read operations rose 216; write operation count stayed 3. Role classification reflects command text, not independent proof of the precise installer operation. Counters indicate activity during this particular sample, not useful semantic progress, successful install, publication or Ready. The snapshot does not prove continued liveness after its observation. No process, settings, or other job was modified.
