# Build Bootstrap and Release Audit — 2026-10-19

Read-only source audit; no build or test was run by this auditor. No runtime success is claimed.

## Bootstrap boundary

The build source registry currently points flow-core-package to the heavy OS Flow browser publication module. Its bootstrapFlowCorePackage function is synchronous and writes only the workspace package manifest. However, importing its module loads process wasm-build, repo artifacts/storage and framework-plugin imports before the operation can run. That violates the documented fresh-clone build import closure. Changing require to dynamic import alone does not remove that dependency cycle.

Extract the static Flow package manifest projection and bootstrap into one source-only owner with system imports, JSON metadata and relative dependency-free imports only. Point the source registry to that owner. The full publication module should consume the same projection, keeping manifest parity without copied fields. Keep publication authority and compiler operations in the existing heavy owner. Preserve validation limits through a dependency-free authored policy/schema owner rather than importing the full artifacts barrel merely for maximumFiles.

Synchronous publication fits the existing contract: build bootstrap ensureBuildEnvironmentV1 calls publish(root); build/📜️script.ts invokes publishBootstrapSources at line 170; caching/bootstrap/📜️script.ts invokes it at line 623. The injected phase test publisher is a synchronous failure observer. If publication becomes asynchronous, every caller and injected publisher type must await it; otherwise dependency installation can race source publication and failures can escape control checks.

## Coverage and coordinated edits

- Flow browser ownership fixture lists bootstrapFlowCorePackage and flowBrowserPackageManifestV1 under browser-publication. Its ownership test imports that owner, checks manifest/files/exports, SQLite count parity, invalid snippet refusal and frozen values. Move expected ownership to the real new owner and import its manifest directly; do not leave compatibility exports solely to preserve a stale fixture.
- Build inputs storage phase tests extract ensureBuildEnvironmentV1 and inject publishers. Preserve refusal before effects for forged/cancelled controls.
- Add a language-neutral source-publication corpus covering missing manifest, existing manifest unchanged, authored manifest content, and ordered publisher failure. Use the existing JSON schema/SQLite or other installed independent oracle.
- The most valuable regression is fresh-process import/publication with application package resolution unavailable. A successful bun repo --help with current dependencies only does not alone prove a fresh clone works. Prefer an import-closure test or isolated fixture that rejects heavy Flow publisher and bare workspace package imports, without moving live node_modules used by other agents.
- Permanent scripts must remain 📜️script.ts. Update the declared source/output contract and relevant ownership/test input lists together.

## Release host boundary

The architecture course project explicitly authors Linux x64, Debian 12, 72 GiB minimum free bytes and seven unused writable loops. Repo preflight reads real process CAP_SYS_ADMIN, writable loop-control/device facts and tool executability before allocation. README documents a dedicated ephemeral Debian 12 x64 runner selected through ARCHITECTURE_COURSE_RUNNER; elevated managed commands preserve original release mode. The deploy owner explicitly refuses image production unless process.platform is linux and process.arch is x64 at line 456, then builds a runtime image for the actual executable architecture.

Repository infrastructure supports macOS storage and native Rust generally, but that is not evidence for an existing course release route. A macOS executable cannot be copied into the authored Debian runtime image. Do not remove the Linux refusal, falsify preflight facts, or invoke raw compilers to label an artifact deployable.

Safe remedy is an actual suitable Debian 12 x64 builder with sufficient physical backing and required mount capabilities/devices, using the existing managed targets and final image/stack/E2E gates. A VM is only valid when its actual backing, process capabilities and devices meet the same checks; Docker Desktop alone does not establish this. Without that environment, source repairs and statically validated deployment contracts can be completed, but a new verified deployable Linux bundle remains unproven.

## Live Dependency Coordination Follow-up

Read-only ps, lsof and SQLite/queue-file probes during the sole managed E2E attempt establish a changed queue state. Parent process 41418 and build child 41423 remain alive. Earlier lock writer processes 43281/43289 are absent. External dependencies-lock process 45510 is alive, but is now behind the E2E process on the dependency image queue.

The stable resource label for aa5bbea09cfe074fa78d6a26040daaef3d3c8824b1f2f3c6764dbe3f128eb95b.sqlite is native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d. Its ordered queue is:

1. 00mvf5fi9i-0000vyn--GDTeipjAe3rGCe8f0e89w — PID 41423, exclusive, sharedCohort false.
2. 00mvf5ft6o-0000z46-ym_HNvXtH2sFfS4a8klOtQ — PID 45510, shared, sharedCohort true.

A normal read-only SQLite query of d resource returned database locked; its immutable read was used only to recover the stable resource label, not to infer lock custody. lsof independently showed PID41423 holding that d resource database FD and reading the actual mounted dependency generation directory .cache/b/d/m/j/35c676d5ffcd51d1153e. PID45510 held its queue claim but no d resource database FD at that snapshot. Both processes retain shared shared-cohort tickets on the outer .cache/b/o image queue.

PID41423 CPU time advanced from 2:38.30 to 2:48.80 across approximately 23 seconds, with observed instantaneous utilization 13.7–17.9 percent. PID45510 advanced only 0:15.76 to 0:16.48 over that interval. The E2E log was still emitting generic repository:command running events and was 3,825,102 bytes at the later snapshot. These facts demonstrate ongoing dependency scanning/work by the E2E command, while the later external lock command waits behind its exclusive dependency image ticket. They do not establish completion, successful installation, or an E2E test pass. No process, queue, image, claim or log was modified by this audit.

Final snapshot: PID45510 had exited; PID41423 continued at CPU time 3:02.44, elapsed 16:19, observed utilization 57.8 percent. Its open dependency-directory FD moved into node_modules/.bun/async@3.2.6/node_modules/async, independently confirming traversal progress beyond the earlier generation root.

## Original Build Mode Follow-up

Read-only source audit confirms fixedTargetBuildMode globally classifies test-e2e as prototype despite the name encoding no compiler mode. Both resolveBuildModeV1 (generic build invocation) and resolveNxBuildMode (bootstrap/caller invocation) combine that fixed mode with explicit configuration, selection environment and inherited environment, refusing any disagreement. Therefore a release original invocation is rejected before the course owner, while an unconfigured invocation defaults to prototype and is later correctly rejected by course rehearsal/publication authority.

The shared fixed-target function also feeds targetBuildModes, including its semio.buildMode.fixed metadata and defaultConfiguration. A correct repair removes ambiguous test-e2e fixation and retains the neutral default prototype when no caller mode exists. It must not change an already issued original invocation. Regression coverage should exercise generic and Nx resolvers with inherited release, explicit release, neutral default prototype and conflicting explicit/inherited values, plus the target metadata projection. Existing Nx contract fixture lives at caching/🧪️tests/🧫️fixtures/nx-contract/🔣️.json and is consumed by caching/🧪️tests/🎚️build-mode/🟦️.ts.

Course authority call sites are package script release gate; E2E rehearsal and topology admission; deploy site publication, Docker image production/publication and readiness; dev proctor/stack intent remains dev and preserves whichever mode the original caller supplied. The course mode fixture explicitly accepts dev+prototype, dev+release and release+release, and rejects release+prototype. Do not weaken that boundary.

Course dashboard fixture declares release defaults for deploy-check, docker-image-build, docker-image-publish, publish and test-e2e. Only test-e2e matches a globally fixed prototype name among those five targets. Workflow globally supplies SEMIO_BUILD_MODE=release, including the rehearsal E2E command. README lines112–114 documents default dashboard release for both E2E topologies, and lines240–244 documents the original release requirement and explicit --configuration=release managed invocation. No documentation change is necessary to authorize an incorrectly issued prototype E2E run; the next invocation must select release before issuance.

Other ambiguity: fixed dev and the -dev suffix also force course dev and dev-site to prototype despite course dev intent preserving original release. They do not conflict with the five authored release gate targets, and course has no prepare-e2e/serve-e2e targets. Preserve genuine explicit -prototype/-release intent; consider neutral dev semantics separately if widening the shared generic rule. The in-flight prototype E2E process may finish useful dependency initialization but cannot become an authorized release rehearsal merely because source rules change beneath it.

No source edits, tests, builds, authority changes or process interruption were performed for this follow-up.

## Current Main Dependency Sync Custody Addendum

Fresh observations supersede the historical 41423/45510 snapshots above. The main sync is alive as parent9578/build9580, command dependencies sync --all. External lock writer99809/build99823 is also alive. No process was interrupted.

Actual dependency-image resource aa5bbea09cfe074fa78d6a26040daaef3d3c8824b1f2f3c6764dbe3f128eb95b (stable label native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d) queue at observation time:

1. 00mvf6f8p0-000250v-lRJhS-MvHjMUCikDoFdHfw:99823, exclusive, sharedCohort false.
2. 00mvf6gl9u-0000ldx-zYZJaay17NVvrJYuWOV3eQ:27717, shared cohort (process-runtime test).
3. 00mvf6gsqd-00002ae-wYcIQzIXLBjuR6W5by_NMw:2966, shared cohort (dashboard-control test).
4. 00mvf6wm62-00007e4-Rk9udJslTzbrIpOhzxrfSA:9580, shared cohort (main sync).

lsof independently showed99823 holding the actual d resource database descriptor and the javascript-dependency-bindings:/Users/ueli/Documents/semio resource database ed11bbd6b4730d412a686cb8df2127095c05f114ddf4865f65a67e5546677fff. Main9580 held its d queue claim descriptors and outer image resource, but no d resource database descriptor. Both retain shared cohort tickets on the outer o image queue. This supports current main sync waiting behind the external exclusive dependency-image writer, rather than main sync owning the image or the old worker remaining current.

Across approximately45 seconds (including a deliberately spaced20-second observation interval),99823 CPU time advanced2:01.01 to2:07.24, with observed7.8–8.0 percent utilization. Main9580 advanced0:04.30 to0:06.32. Main log grew4379 to6563 bytes while repeating bun:dependency-storage running. Both processes remain alive; no direct child of either was observed. The sampled writer FD lists did not show a dependency package directory traversal, so these observations establish ongoing CPU activity and the waiting relationship, not semantic installation progress, eventual completion or a self-deadlock. No environment dump, installer/build, process change, cache/queue mutation or attribution of host ENFILE was performed. Immutable SQLite reads recovered stable resource labels only; live custody was assessed from ordered queue metadata and independent descriptors.
