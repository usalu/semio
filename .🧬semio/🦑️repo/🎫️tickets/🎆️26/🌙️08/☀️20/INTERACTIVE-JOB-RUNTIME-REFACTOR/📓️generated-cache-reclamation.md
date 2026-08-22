# Generated Cache Reclamation

## 2026-08-21

The interactivity-refactor verification fleet exhausted the workspace volume while producing isolated Cargo target directories. Before reclamation, the data volume had 5.5 GiB available and reported 100% capacity.

The following directories are generated Cargo build outputs from completed or superseded checks and are safe to rebuild from repository source and manifests:

- `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-surface-errors` (5.4 GiB)
- `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-p9r-wgpu-pollster` (4.9 GiB)
- `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-p9v-jsonschema` (3.5 GiB)
- `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-editor-graph-errors` (2.2 GiB)
- `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-plugin-errors` (2.0 GiB)
- `EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/🧪️target` (2.8 GiB)

No ticket report, command log, fixture, source file, or active agent target is included. Reclamation is recoverable by rerunning the recorded Cargo/Nx gates.

### Second wave

Concurrent release and component builds reduced free space to 1.6 GiB. Each active owner identified
the exact targets that must be preserved (`target-p4`, `target-owned-wasm-core`, and
`target-stdio-compression`). The following older generated caches are therefore reclaimed:

- `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-stdio-thiserror` (22 GiB)
- `PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/🧪️target-r19-stdio` (21 GiB)
- `RESUMABLE-WFC-PUZZLE2D-AND-ENERGY-JOBS/🧪️target-p7b` (11 GiB)
- `PHASE-5-RESUMABLE-FRAME-TRANSACTION-AND-RENDERER/🧪️target-p5a` (8.0 GiB)
- `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-owned-wasm-baseline` (7.1 GiB; superseded by the real stdio component fixture)
- `PHASE-1.5-ASYNC-SEMANTICS-CORRECTION/🧪️target-plugin-app-fleet-a` (4.4 GiB)
- `PHASE-1.5-ASYNC-SEMANTICS-CORRECTION/🧪️target-plugin-app-energy` (4.1 GiB)

The proposed `PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/🧪️target-r21-fem` cleanup was not performed because
Cargo rejected it as lacking a valid `CACHEDIR.TAG`; it remains intact. As in the first wave, the
successfully cleaned paths contained only rebuildable Cargo outputs; their reports and exact logs
remain. The second wave restored free space from 1.6 GiB to 78 GiB.

### Third wave

Concurrent Puzzle, Energy, ToolJob, and owned-WASM verification reduced free space to 14 GiB. The
active targets were resolved from the running processes and preserved: `target-p4`,
`target-p7-energy`, `target-p8-runtime`, `target-p6-root`, `target-owned-wasm-core`, and
`target-stdio-compression`. Each reclaimed directory below had a valid Cargo `CACHEDIR.TAG` and was
cleaned with `cargo clean --target-dir <exact-path>`:

- `PHASE-1-ONE-POOL-WORKER-RUNTIME/🧪️target-interactive-compute` (5.5 GiB)
- `RESUMABLE-WFC-PUZZLE2D-AND-ENERGY-JOBS/🧪️target-p7-wfc` (4.5 GiB)
- `PHASE-1-ONE-POOL-WORKER-RUNTIME/🧪️target-host-process-pool` (1.5 GiB)
- `PHASE-1.5-ASYNC-SEMANTICS-CORRECTION/🧪️target-plugin-app-imperative` (1.4 GiB)
- `PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/🧪️target-manifest-pure-root` (1.2 GiB)
- `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-p9u-notify` (1.2 GiB)
- `PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/🧪️target-r22-db` (1.2 GiB)

The cleanup removed only reproducible Cargo outputs. Ticket reports, source, fixtures, and exact
verification logs remain recoverable in place.

## 2026-08-22

Concurrent Flow, Puzzle, Energy, FEM, ToolJob, and stdio warning proof builds reduced free space to
17 GiB. Running process command lines were checked before reclamation. The active `target-p4`,
`target-p7-energy`, `target-p6-root`, `target-p8-runtime`, and `target-stdio-warning-cleanup`
directories were preserved. The completed owned-WASM packet no longer had a running build, and its
large release-component cache was explicitly abandoned after the bounded stdio result kept
Wasmtime as the default. Both exact targets had valid Cargo `CACHEDIR.TAG` markers and were cleaned
with `cargo clean --target-dir <exact-path>`:

- `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-stdio-compression` (`cargo clean`: 53,095 files,
  119.3 GiB accounting)
- `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-owned-wasm-core` (`cargo clean`: 9,690 files,
  6.9 GiB accounting)

Free space rose from 17 GiB to 92 GiB. Only rebuildable Cargo output was removed; the owned-WASM
report, test logs, fixtures, source, and all other ticket evidence remain intact.

### Stdio warning packet cache

Concurrent WASI, Energy, planar-boolean oracle, and renderer/product builds later reduced free space
to 54 GiB. All active targets were preserved. The completed stdio warning-cleanup agent had exited,
no process referenced its target, and its report and command log were already stored outside the
cache. The exact generated target
`PHASE-1.5-ASYNC-SEMANTICS-CORRECTION/🧪️target-stdio-warning-cleanup` was cleaned with
`cargo clean --target-dir`; Cargo removed 3,446 files / 3.4 GiB. Free space rose to 57 GiB. No source,
fixture, report, or log was removed.
## 2026-08-22 Owned Planar Boolean Targets

After the P9ac owned planar-boolean packet completed every native, release, WASM, feature, timing,
and oracle gate, its two isolated generated targets were reclaimed with exact `cargo clean
--target-dir` commands. The oracle target removed 384.2 MiB and the owned all-gate target removed
997.7 MiB. Source, oracle logs, command evidence, and the packet report remain in the ticket.
### Phase 8 strict-gate cache

After the framework warning-denial cohort completed, the coordinator verified that no live process
referenced `EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/🧪️target-p8-runtime`, then ran `cargo clean`
against that exact target directory. Cargo removed 12,399 generated files totaling 12.4 GiB. No
source, ticket evidence, shared target, Puzzle target, Animate target, or active WASM target was
removed. Free space rose from approximately 15 GiB to 26 GiB; the cache is recoverable by rebuilding
the recorded Phase 8 commands.

### Superseded FEM and Puzzle caches

The Puzzle/FEM/renderer owner confirmed that only
`PUZZLE-3D-RESUMABLE-VERTICAL-SLICE/🧪️target-p4` remained active. Exact `cargo clean
--target-dir` commands then reclaimed five superseded FEM targets (2.6 GiB reported by Cargo in
total) and `PUZZLE-3D-INTERACTION-CORRECTNESS/🧪️target-p4` (4.7 GiB). The active resumable-Puzzle
target, Animate target, and owned-WASM build were preserved. Cargo refused to clean
`RESUMABLE-FEM-JOB-GRAPH/🧪️target-p6` because it lacks a valid `CACHEDIR.TAG`; that directory was
left intact. No source, fixture, report, or command evidence was removed.

### Superseded de-async caches

While the Animate, Puzzle/FEM/renderer, and owned-WASM verification builds were active, free space
fell to 12 GiB. Process command lines and agent ownership were checked before reclamation. The
active master-ticket `target-animate`, resumable-Puzzle `target-p4`, shared production owned-WASM
target, and renderer snapshot targets were preserved. Exact `cargo clean --target-dir` commands
reclaimed only completed ticket-local de-async caches:

- `PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/target-infinite-wasm` (1.1 GiB)
- `PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/target-infinite-release` (1016.6 MiB)
- `PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/target-infinite-wasi` (916.5 MiB)
- `PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/target-db-wasm` (467.1 MiB)
- `PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/target-db-native` (136.0 MiB)

The cleanup removed 13,636 reproducible build files and approximately 3.5 GiB. No source, fixture,
ticket report, command log, or active cache was removed.
# Shared Incremental Cache Reclamation — 2026-08-22

Rust Analyzer restarted its unsolicited `cargo check --workspace --keep-going --all-targets` child twice while the isolated phase gates were compiling. The child was stopped without stopping Rust Analyzer or any requested fleet process. Its exact generated cache, `/Users/ueli/Documents/semio/target/debug/incremental`, had reached **106 GiB**. No active requested build used the shared target: Animate and Puzzle3D used their ticket-local targets, and the owned-WASM worker was not compiling.

Only that regenerable incremental directory's contents were removed. Source, linked artifacts, phase logs, `target/debug/deps`, `target/wasm32-wasip2`, `target-animate`, and `target-p4` were preserved. Available disk space increased from **5.2 GiB** to **106 GiB**.
