# Native Validation Space Recovery

Fourteen superseded private Nx workspace-data directories were removed after their owning commands reached terminal states. A fresh process-argument and open-file inventory found no running command or open handle referencing any selected directory. Active Nx workspaces, the shared Cargo target, PDF cache and published outputs, test binaries, raw test logs, separate proof receipts and all retained source inputs remain available.

The removed directories held 1601572 KiB of allocated data. Observed free disk space changed from 3937427456 to 5525831680 bytes during removal; other builds were active, so this is not an isolated filesystem benchmark. The exact preflight and removal receipts are under `🗑️generated/retired-nx-workspaces-preflight.json` and `🗑️generated/retired-nx-workspaces-removal.json`.

- `🗑️generated/nx-workspace-data-flow13-current-3`
- `🗑️generated/nx-workspace-data-value-required-2`
- `🗑️generated/nx-workspace-data-value-required`
- `🗑️generated/nx-workspace-data-flow-draw`
- `🗑️generated/nx-workspace-data-flow-graph-current`
- `🗑️generated/nx-workspace-data-forms-prereq-2`
- `🗑️generated/nx-workspace-data-ts40`
- `🗑️generated/nx-root-gis-map-11`
- `🗑️generated/nx-root-block-parent-2`
- `🗑️generated/nx-root-gis-components-1`
- `🗑️generated/nx-root-gis-parent-1`
- `🗑️generated/nx-root-gis-map-components-2`
- `🗑️generated/nx-root-gis-independent-10`
- `🗑️generated/nx-root-collection-block-1`

A second checked removal retired two further completed root workspaces: `nx-root-block-parent-1`, `nx-root-gis-independent-9`. The generic nested workspace was preserved because the Norm and Store runs hold two child directories. Exact receipt: `🗑️generated/retired-nx-workspaces-second-removal.json`.

The registry executor confirmed fifteen predecessor subdirectories in the generic Nx workspace held only generated graph/file-map/SQLite data. Fresh process and open-file checks were repeated immediately before their removal. Active `norm-surface-3` and `store-snapshot-ownership-2` remain intact, as do raw logs and retained inputs outside this cache. Free space after this step was 6296141824 bytes. Exact receipts are `retired-nested-nx-workspaces-preflight.json` and `retired-nested-nx-workspaces-removal.json` under the generated folder.

- `🗑️generated/nx-workspace-data/norm-contract-chunk-2`
- `🗑️generated/nx-workspace-data/norm-lib-1`
- `🗑️generated/nx-workspace-data/norm-lib-2`
- `🗑️generated/nx-workspace-data/norm-lib-3`
- `🗑️generated/nx-workspace-data/norm-lib-4`
- `🗑️generated/nx-workspace-data/norm-surface-source-2`
- `🗑️generated/nx-workspace-data/norm-surface-source-fix`
- `🗑️generated/nx-workspace-data/norm-surface-1`
- `🗑️generated/nx-workspace-data/norm-surface-2`
- `🗑️generated/nx-workspace-data/norm-config`
- `🗑️generated/nx-workspace-data/norm-config-2`
- `🗑️generated/nx-workspace-data/store-snapshot-ownership-1`
- `🗑️generated/nx-workspace-data/stdio-contract-current`
- `🗑️generated/nx-workspace-data/forms-prereq-jobs2-final`
- `🗑️generated/nx-workspace-data/pdf-rust-final-inspection`

## Disk-Full Event And Scoped Retries

The later shared native build phase exhausted available disk space. Seven root sessions terminated with ENOSPC/fingerprint/SQLite write failures: GIS parent 98734, PDF population 90282, Map default 41462, Block parent 24289, Terrain default 51186, Terrain component 66717 and Map component 67105. These failures do not establish source or test failures. The durable GIS route had already passed four exact kernel laws before its following Map role-port build failed for space.

The stdio executor cancelled its owned full-catalog compiler and selected owned queues during disk pressure. After temporary compiler/link/swap cleanup and external filesystem activity, a fresh `df -h .` showed 90 GiB available. No causal storage benchmark is claimed. Root preserved the shared Cargo target, native binaries, PDF cache and raw receipts.

PDF population 6 left the published 200-file output exactly equal to population 4 by filename, size and SHA-256. The process audit found no surviving PDF-owned build; broad external clippy commands merely mention PDF in their package inventory and were left untouched. Receipts: `pdf-native-cache-6-process-audit.json` and `pdf-native-after-disk-full-6-inventory.json`. Population 7 now retries the ordinary Nx build against the same private cache.

A single root sequential matrix uses `nx-root-native-recovery-1` for Map component, Terrain default/component, Map default, GIS parent and Block parent tests. Every invocation forwards the complete runtime selector through the ordinary Nx target. All native lanes keep `CARGO_BUILD_JOBS=2`, `CARGO_INCREMENTAL=0` and the same warmed target. Existing GIS exact-law batch 74426 remains attached.
