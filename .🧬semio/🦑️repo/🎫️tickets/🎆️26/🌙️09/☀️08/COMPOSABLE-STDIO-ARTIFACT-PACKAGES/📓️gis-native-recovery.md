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

After the graph and discovery repairs, `🗑️generated/native-recovery-5.tsv` reached terminal state for all six gates:

| Gate | Exit | Classification |
| --- | ---: | --- |
| Map component | 1 | Its root runner still imported a removed replication Map test module. The runner was subsequently redirected to the current shared Map delta source. |
| Terrain default | 1 | Same removed replication Map test-module import. |
| Terrain component | 130 | Discovery referenced the removed `artifactProjection` flag. The coordinator removed that obsolete guard after this run. |
| Map default | 1 | Compiled after the shared Cargo wait, then failed a pre-repair Store snapshot with twelve diagnostics: missing `Send + Sync + 'static` bounds in one-item and transient publication retirement paths, plus stale Presence references to `current_read` and `reads`. |
| GIS parent | 1 | Its schema oracle completed (`receipts=2 hostile=8 ajv+node+webcrypto=1`), then native compilation failed the same pre-repair Store snapshot with ten missing publication/retirement bounds. |
| Block parent | 1 | Compiled the repaired Store source, then failed in the plugin root on six stale reexports for removed bounded transient/window factory functions. The owner-bundle replacement is being completed without compatibility exports. |

The Store repair adds the required method-level `P: Send + Sync + 'static` bounds to `ArtifactEphemeralOneItemPublication::close_step` and `TransientStore::advance_publish_one`. Current Presence construction already uses `local_read`, contains no `current_read`/`reads` access, and records no transient returned-read authority, so no Presence edit was needed. The durable-group publication retirement helper already carries the propagated bound and its callers satisfy it. Exact-file `rustfmt` completed for the Store source. Fresh failed-gate execution remains required after the plugin owner-bundle source settles.

The compatibility-free window transient contract is now source-settled. `WindowTransientOwner` exposes one `build_owners()` method returning `WindowTransientOwnerBundle`; no generic fixed-page or bounded window owner helper remains. The Block3D world transient consumer composes its exact owners directly: domain preflight admits one bounded work item, domain transfer consumes the mutation into the replacement state, and `OwnedValueRetirementFactory` instances retire both state and mutation. A static contract audit confirmed those functions and trait bounds match `ArtifactEphemeralTransferPreparationFactory::new`, found no removed helper reference under Block, and passed exact-file `rustfmt --check` with child traversal disabled. The existing app-level bounded transient helpers remain authoritative for non-window Presence and Transient hooks. Runtime reacceptance remains queued behind the active shared Cargo build.

At `2026-09-09T12:53:08Z`, retry 6 had one intact owned chain: shell `28387` → Nx `28389` → Bun `34402` → nextest `35293` → Cargo `35439`. Cargo `35439` exclusively held the shared target's `debug/.cargo-lock`. Former sccache server `62766` and compiler `24617` were absent. Cargo's direct sccache client changed from XLSX `62287` to HTML `63731` during a five-second observation while stdio-semio client `59547` remained connected, establishing current dependency-scheduling progress rather than an abandoned orphan computation. No environment values were retained in this observation.

Retry 6 GIS parent compiled and listed the current library suite after its JavaScript oracle passed, with no compiler diagnostic. The fundamental runner then killed the suite after the default aggregate `15000ms` budget; the gate reached terminal exit 1 after 96 minutes 39 seconds. This is a runner-budget result rather than a test assertion or source failure. A GIS-only parent retry must set `SEMIO_TEST_BUDGET_MS=600000` while leaving every internal watchdog unchanged. The original terminal receipt and log remain in `🗑️generated/native-recovery-6.tsv` and `🗑️generated/native-recovery-6-gis-parent.txt`. Block started automatically as the second sequential gate.

Flow retry 9 captured an `E0277` from a pre-repair plugin snapshot where `validate_parent_child_restore` propagated `ChildRestoreProjectionError` directly with `?`. The current plugin source already translates that exact error through `plugin_sdk_fault("child restore projection failed: …")` before applying `?`, while preserving the separate denial that rejects a member absent from the loaded parent projection. A scoped Bun census confirmed the translated projection call is present and the raw `child_projection(&source, None)?` form is absent. The plugin path was already present in both source ledgers. Exact-file `rustfmt --check` found unrelated concurrent formatting differences elsewhere in the 1.9 MB shared file, so no formatting or source edit was applied over those changes. Fresh queued compilation remains the authority.

Retry 6 Block parent completed after 48 minutes 29 seconds with one `E0425`: its plugin compile snapshot could not resolve `store::MEMBER_OPEN_IDENTITY_BYTES` at plugin line 16939. Current source is newer than that snapshot: the Store root reexports `MEMBER_OPEN_IDENTITY_BYTES` at line 42 from its durable member-open module, where the constant remains defined as 256 bytes. Store and plugin sources both had modification time `2026-09-09T15:55:25Z`, concurrent with the failed compile, and the shared plugin owner also confirmed the current generic/state repairs. No Block-domain source or assertion ran, and no additional source edit was needed. The retry 6 sequential handle is terminal with GIS exit 1 for aggregate budget and Block exit 1 for this stale shared compiler snapshot; both rows and raw logs remain preserved.

Retry 7 uses the settled shared plugin/store source and raises the full-suite execution allowance to `SEMIO_TEST_BUDGET_MS=600000`; build/list compilation remains unbounded through `SEMIO_BUILD_BUDGET_MS=0`. GIS and Block parent both stopped before Cargo on the same Nx graph snapshot: the stdio Semio geometry unit test could not load its `../../🟦️.ts` production parser and the graph also reported the `npm:@asamuzakjp/css-color` source project absent. The three existing `../../…` references are lexically correct. The production parser target was created at `2026-09-09T16:03:36Z`, after the failed graph snapshot. No net test-source edit remains. A fresh direct Bun run then loaded that production parser, validated all 12 neutral geometry vectors against the third-party AJV oracle, and confirmed all three referenced paths exist. The subsequent Map component gate crossed a fresh graph successfully and entered `cargo-nextest list`; its terminal target-specific result is pending behind the shared Cargo queue. The two parent failures remain preserved in `🗑️generated/native-recovery-7.tsv` and their corresponding retry-7 logs.
