# Current Dirty Quarantine Snapshot

## Repository State

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Total porcelain records: 2,981.
- Top-level distribution: 2,796 under `✏️s`, 110 ticket records, 71 framework paths, and one each for the root script, package manifest, Bun lock, and Cargo lock.

## Active Quarantines

- stdio currently has 369 dirty records. It increased from the prior stable two-sample value of 361, so its graph, glTF mounts, and registrars remain an active external wave. The known absent `no-mutation` glTF mount is not safe for this coordinator to repair independently.
- SPR/store currently has 12 dirty records. Current fingerprints are:
  - channel: `ea5e141c781c2bdbd8fe3e82085dc9313a0dafc9538d59de3c03c32d83d12132`;
  - testkit: `fbe88ae85718c554c6df417bccb9f0377bdd32280d6d6ab45ccbd8ab7abc4403`;
  - store: `1d1faaf1829ea6f82f22262dedb9735e8654ed8dcea1fa897d3f22f87d7f559b`.
- Testkit and store advanced since the preceding sample. Their `HistoryLog`, `HistoryOpMeta`, receipt, reconciliation, conflict, and command-shape compile failures remain external migration drift, not isolated refactor failures.

## Scheduling Decision

Keep stdio, SPR, store, channel, testkit, and all downstream validation repairs quarantined. Continue only hash-stable package-local discovery and graph-coloured leases. Recheck queued compiler, Run, graph, Pack, 2D, 3D, and neural validations after these owners stop advancing.

## Subsequent Sample

- stdio advanced again from 369 to 371 dirty records.
- SPR/store remains at 12 dirty records, but testkit advanced to `3a1f56b31fc69e462daa2e40a08170909975284d3797544aeb3273974029a7f5` and store advanced to `fdfc84efc17d8d746e9fb62196146fa6843ed2097e1c6fa0a20fc33878883b83`.
- Channel remained `ea5e141c781c2bdbd8fe3e82085dc9313a0dafc9538d59de3c03c32d83d12132`, but the SCC is not stable while its direct testkit/store consumers advance.

## Cargo Build-Lock Evidence

The Graph acceptance retry is deferred rather than repeated. A read-only process sample found simultaneous external Cargo jobs for the workspace, OS kernel tests, framework plugin, and layout, norm, imperative, raster, lowpoly, DAG, forms, and playbook plugins, with an active framework UI rustc child. This directly explains both Graph Nx attempts exhausting the quick-test budget while waiting for the shared build directory. No process was interrupted or modified.
