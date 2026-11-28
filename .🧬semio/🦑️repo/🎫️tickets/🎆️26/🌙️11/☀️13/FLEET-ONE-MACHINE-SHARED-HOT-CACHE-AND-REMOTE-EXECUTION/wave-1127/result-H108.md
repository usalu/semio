# Result H108: proof read with componentEdgeValidity on (running)
Deutsch: Ergebnis H108: Beweis-Lesevorgang mit componentEdgeValidity (läuft).

## Setup
- Supported override: env `SEMIO_GRAPH_COMPONENT_EDGES=1` (`🧩️environment-components/🟨️.mjs` `componentEdgeValidityOnV1`, read from `process.env` of the computing process in `🕰️generations/🟨️.mjs:337`); policy file untouched (`componentEdgeValidity` stays false).
- Started 01:16 CET: `SEMIO_GRAPH_COMPONENT_EDGES=1 nice -n 10 repo graph --graph-policy fresh`, pid in `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H108/pid` (41584 at start), log `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H108/graph.log`.

## Timeline
- 01:16 phase door.capacity: BLOCKED. `[capacity] waiting ... grant ATW-UsPSQk7wdTpNCPP98w was written by a newer version and cannot be read ... admitting nothing`.
  Cause (measured): the grant row `~/Library/Caches/semio/host-capacity/grants/ATW-UsPSQk7wdTpNCPP98w.json` belongs to a LIVE peer (pid 69049, `serve-forms-e2e`, 1h48m old, the Forms session) and carries inline `peakMegabytes/observations/deviationMegabytes`. Current code writes these to a `<id>.stats` sidecar (schema `GrantStats`) and the `Grant` schema has `additionalProperties:false`, so the reader treats the row as unrecognized-but-live and refuses ALL admissions (also `repo capacity status` fails with the same error). It is old code of a long-lived process, not newer code; the message is misleading. It clears when pid 69049 ends (or its grant retires). I did not touch the peer or the grant file.
- 01:23 still blocked at door.capacity (350 s+). No phase beyond door.capacity reached; no waves, no stale counts, no seal. `repo-mcp:build`/`install` NOT run.

## Needs the coordinator (no fix staged: the code path is right, the holder is old)
- Decision: let the Forms session's serve (pid 69049, grant ATW-UsPSQk7wdTpNCPP98w) end or stop it (I may only kill pids I started). The queued read (pid in `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H108/pid`, detached, nice 10) proceeds on its own the moment the grant retires; env flag already set, nothing to restart.
- Not staged on purpose: making the reader accept a row with inline stats fields would be legacy support (CLAUDE.md forbids). A real fix to consider for H101: reword the message ("written by an older/different version") and let the status reader name the holder pid/label so the block is diagnosable; and `repo capacity status` should not crash on it.
- Any new serve/test started from older code that writes inline stats will re-block every admission the same way.
- Unverified: whether componentEdgeValidity converges; whether the flag reaches the wave workers (computation reads `process.env` of the reader process; workers not checked).
- 01:22 grant retired on its own; read admitted (3072 MB), phase build:source-admission. Read pid alive; I will not poll further, check `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-fuDDx3/098f233d-9be4-4a1a-b4a8-48924f807f61/scratchpad/H108/graph.log`. Waves/stale counts/seal still unknown.
- 01:23 (log lines 94-     300)
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] build:source-admission · Admitting the sources of the build
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
  [script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
- 01:23 (log lines 301-     330)
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:4096","mode":"shared","elapsedMs":3980.4215419999964}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:8192","mode":"shared","elapsedMs":3982.8624999999956}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:12288","mode":"shared","elapsedMs":3984.227749999998}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:16384","mode":"shared","elapsedMs":3986.661791999999}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:20480","mode":"shared","elapsedMs":3987.905458000001}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:24576","mode":"shared","elapsedMs":3990.195749999999}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:28672","mode":"shared","elapsedMs":3992.0027919999993}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:32768","mode":"shared","elapsedMs":4011.196917000001}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:36864","mode":"shared","elapsedMs":4011.835291999996}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:40960","mode":"shared","elapsedMs":4012.6100420000002}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:45056","mode":"shared","elapsedMs":4013.176542000001}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:49152","mode":"shared","elapsedMs":4013.691666999999}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:53248","mode":"shared","elapsedMs":4014.1817079999964}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:57344","mode":"shared","elapsedMs":4014.6486249999944}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:61440","mode":"shared","elapsedMs":4015.108499999995}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:65536","mode":"shared","elapsedMs":4015.545791999997}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:69632","mode":"shared","elapsedMs":4015.993499999997}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:73728","mode":"shared","elapsedMs":4016.408958}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:77824","mode":"shared","elapsedMs":4016.7971249999973}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:81920","mode":"shared","elapsedMs":4017.209167000001}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:86016","mode":"shared","elapsedMs":4018.0719580000004}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:90112","mode":"shared","elapsedMs":4019.244249999996}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:94208","mode":"shared","elapsedMs":4019.7112500000003}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:generation-verify:98304","mode":"shared","elapsedMs":4022.0198329999985}}
  graph: g1-server is reading the whole workspace, waiting for its generation instead of reading again (0 s of at most 720 min)
- 01:24 (log lines 331-     336)
  graph: g1-server is reading the whole workspace, waiting for its generation instead of reading again (21 s of at most 720 min)
  graph: g1-server is reading the whole workspace, waiting for its generation instead of reading again (42 s of at most 720 min)
- 01:24 (log lines 337-     341)
  graph: g1-server is reading the whole workspace, waiting for its generation instead of reading again (62 s of at most 720 min)
- 01:25 (log lines 342-     347)
  graph: g1-server is reading the whole workspace, waiting for its generation instead of reading again (85 s of at most 720 min)
  graph: g1-server is reading the whole workspace, waiting for its generation instead of reading again (105 s of at most 720 min)
- 01:28 (log lines 382-     400)
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":1}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":0}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":1}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":1}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":1}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":0}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":0}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":1}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":2}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":3}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":0}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":2}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":1}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":1}}
- 01:28 (log lines 401-     409)
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":1}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":2}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":1}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":0}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":1}}
  [script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"graph-custody:/Users/ueli/Documents/semio/.cache/b/g/m/generations:7563d8b91c3d70ba4055e582","mode":"shared","elapsedMs":4}}
  graph: waiting for the graph build of a peer (20 s in the queue, generation 260 was the newest when this command queued)
  graph: a peer builds the graph; this run waits as its front door alone, without a loader (20 s, generation 260 was the newest when it queued); holder: pid 66467 (5babf44ebce5576fffd9697176c4afeccdccdb9b9522d63af408d34001631444 show projects
- 01:29 (log lines 410-     411)
  graph: a peer builds the graph; this run waits as its front door alone, without a loader (300 s, generation 260 was the newest when it queued); holder: pid 66467 (5babf44ebce5576fffd9697176c4afeccdccdb9b9522d63af408d34001631444 show project
- 01:29 (log lines 412-     413)
  graph: a peer builds the graph; this run waits as its front door alone, without a loader (330 s, generation 260 was the newest when it queued); holder: pid 66467 (5babf44ebce5576fffd9697176c4afeccdccdb9b9522d63af408d34001631444 show project
- 01:30 (log lines 414-     415)
  graph: a peer builds the graph; this run waits as its front door alone, without a loader (361 s, generation 260 was the newest when it queued); holder: pid 66467 (5babf44ebce5576fffd9697176c4afeccdccdb9b9522d63af408d34001631444 show project
- 01:30 (log lines 416-     417)
  graph: a peer builds the graph; this run waits as its front door alone, without a loader (391 s, generation 260 was the newest when it queued); holder: pid 66467 (5babf44ebce5576fffd9697176c4afeccdccdb9b9522d63af408d34001631444 show project
- 01:31 (log lines 418-     419)
  graph: a peer builds the graph; this run waits as its front door alone, without a loader (422 s, generation 260 was the newest when it queued); holder: pid 66467 (5babf44ebce5576fffd9697176c4afeccdccdb9b9522d63af408d34001631444 show project
- 01:31 (log lines 420-     421)
  graph: a peer builds the graph; this run waits as its front door alone, without a loader (452 s, generation 260 was the newest when it queued); holder: pid 66467 (5babf44ebce5576fffd9697176c4afeccdccdb9b9522d63af408d34001631444 show project
- 01:35 phase source.served: queued behind a peer graph builder (pid 66467, 'show projects', no component flag in its env) as front door without loader; generation 260 newest. The proof read only starts its own waves after that peer ends or hands over.
- 01:32 coordinator: H101 flips `componentEdgeValidity: true` in the graph policy at the 02:00 slot; I record the next whole read/build (mine or the peer's pid 66467) wave stale counts and outcome here, and on seal run repo-mcp build + install.
- 01:42 log now 2648 lines; the capacity reader now isolates older-format grants instead of refusing (a fix landed after my 01:16 block: 'isolated instead of refused, counted as N MB'). Read still queued/alive; first monitor was too noisy and was replaced by a filtered one.
- 02:12 (flip live 02:10:21, rollback stageF-20261128T011021-da26d9). My read (pid 41584, since 01:16, env flag also set) is ALIVE but not computing: its door child (pid 12643) waits in `door.capacity` for 3072 MB (35 min): "-851 MB spare = 8073 MB available - 4096 MB floor - 4828 MB reserved but not yet resident, short by 3923 MB, load 75-83, behind graph-plan wQ9nPbjr…" (peer pid 68535 holds 4828 MB). Peer graph builder pid 66467 (`show projects`, started 00:23 UTC, code 4f4230ee460d = pre-flip code, no flag) ran owners 164 of 5424 / inference 6 of 320 at ~01:50 and is still alive after 48 min; its door ended ("no live holder is ahead any more"). No wave with stale-edge counts produced by anyone yet. Memory, not the graph, is the blocker.
