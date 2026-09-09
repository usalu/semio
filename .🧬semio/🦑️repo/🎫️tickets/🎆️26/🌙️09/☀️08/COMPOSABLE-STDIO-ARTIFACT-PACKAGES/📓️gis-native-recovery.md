# GIS Native Recovery

## Failed Snapshot Classification

The first six-run recovery matrix reached terminal exit 1 for every entry, but none of those exits is a current GIS or Block assertion result.

| Gate | Recorded failure | Current-source classification |
| --- | --- | --- |
| Map component | `semio-framework-plugin` E0382, moved `events` | Stale shared-source compiler snapshot. Current plugin sources were older than a successful independent plugin rlib produced at 09:40:19 on 2026-09-09. |
| Terrain default | 39 missing serde traits on actor lifetime, cold-pair, patch and job wire types | Stale shared-source compiler snapshot. Current actor sources explicitly derive serde for each reported type. |
| Terrain component | Same 39 missing serde traits | Same stale shared-source compiler snapshot. |
| Map default | Same 39 missing serde traits | Same stale shared-source compiler snapshot. |
| GIS parent | Missing `ArtifactStoreOneItemAdmissionRejected` | Stale mid-refactor compiler snapshot. Current store source consistently exposes the batch admission/publication API used by its current callers. |
| Block parent | Missing `ArtifactStoreOneItemPublication` and admission rejection symbols | Stale mid-refactor compiler snapshot. Current store source consistently exposes the batch admission/publication API used by its current callers. |

No source edit was made for these compiler snapshots. Fresh ordinary Nx execution remains the runtime authority.

The first registered GIS exact-law batch had two separate outcomes. The durable three-store route passed its four exact kernel laws, then could not write the Map artifact build receipt because the filesystem was full. The native-codec route compiled and ran its selected test, which failed strict teardown because the artifact envelope retained a nested app-owned retirement authority at `Drop`. That is a current runtime failure until a fresh registered route establishes whether the current shared fixture completion path resolves it.

## Recovery Environment

All fresh runs use the ticket-private Nx workspace and cache, the shared ticket Cargo target, `CARGO_BUILD_JOBS=2`, `CARGO_INCREMENTAL=0`, `CARGO_NET_OFFLINE=true`, `SEMIO_BUILD_BUDGET_MS=0`, `NX_DAEMON=false`, and `NX_ISOLATE_PLUGINS=false`. Root library suites forward `--lib --no-fail-fast` and select either `--no-default-features` or exact `component-app-assembly` features as appropriate. Strict teardown and fixture assertions remain unchanged.

## Current Results

The first current-source Map component retry reached a shared OS-kernel compile and failed with seven `E0277` diagnostics. Three IO route expressions passed the synchronous schema `IoFidelity::rank` and schema `Confidence::rank` results into the future-only `resolve_ready` bridge. The repair removes those three stale async wrappers in `🧰️framework/🔨️modules/🚪️io/🦀️.rs`; exact-file `rustfmt` changed no other lines. The original gate is retained as exit 1 in `🗑️generated/native-recovery-2-map-component.txt`.

The next matrix reached terminal state in `🗑️generated/native-recovery-3.tsv`:

| Gate | Exit | Classification |
| --- | ---: | --- |
| Map component | 1 | The repaired OS kernel compiled beyond its former failure. Six private `dec_*`/`enc_*` imports then failed in the shared stdio-semio BREP codec; the BREP owner repaired their scoped visibility after this run. |
| Terrain default | 130 | Nx prerequisite failure while the shared `jco-package-adapter` taxonomy input patterns were unordered; the coordinator repaired the ordering after this run. |
| Terrain component | 130 | Same taxonomy prerequisite failure. |
| Map default | 130 | Same taxonomy prerequisite failure. |
| GIS parent | 130 | Same taxonomy prerequisite failure. |
| Block parent | 1 | Reached seven runtime tests: four passed; the Block 2D, 3D, and 5D `viewer_never_mutates` laws aborted during strict registered-fixture close because no app-owned document-store disposer was supplied. The strict cursor-disposer Drop assertion then correctly detected nonterminal ownership. |

The shared registered-viewer fixture owner is repairing the explicit bounded lifecycle. Pending coherent-source retries and registered exact-route execution.

The registered-viewer lifecycle was subsequently repaired with an explicit private bounded fixture wrapper, preserving each viewer's declared authority and supplying exact bounded fixture authority for otherwise absent lanes. The BREP source integration also completed. `🗑️generated/native-recovery-4.tsv` retried all six failed gates against those inputs with `NX_NO_CLOUD=true`, but every gate stopped during Nx graph construction before native execution. The first four encountered an invalid relocated Playbook document-contract fixture import; GIS and Block parent encountered the analogous Note import. Every graph attempt also reported `Source project does not exist: npm:@asamuzakjp/css-color`. These are shared graph prerequisites, not GIS/Block results; the failed rows and logs remain preserved.
