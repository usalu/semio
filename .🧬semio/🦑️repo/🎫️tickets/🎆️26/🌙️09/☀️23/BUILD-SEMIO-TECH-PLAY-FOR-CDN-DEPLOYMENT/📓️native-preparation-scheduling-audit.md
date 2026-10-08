# Native Preparation Scheduling Audit

The observed duplication is real: NativeScript owner-command calls prepareCargoWorkspaceInvocation before spawning its selected package body. runRepositoryCargoTests independently calls the same mandatory preparation immediately before constructing the Cargo policy and running Cargo/Nextest. Both spawn the same preparation router, acquire the global repository cargo-preparation exclusive queue and rescan/discover the closure. There is no inherited receipt or supported already-prepared proof: SEMIO_CARGO_PREPARATION_ACTIVE only rejects recursion in owner recipes; it cannot safely serve as a bypass.

Current examples are Pub String body78697→inner78722 and Process79453→inner79854 after their respective outer preparations completed. Freshness is still necessary at the body consumption point because other developers may edit source between outer and inner stages. Therefore skipping the inner preparation based only on package name or an environment flag would be unsound.

The narrow sound scheduling solution is to move preparation responsibility to the canonical consuming Cargo operation for an explicitly typed repository test-body route: that route must guarantee runRepositoryCargoTests (or another canonical Cargo consumer) prepares before consumption, while the outer native wrapper performs only command admission/progress. Direct/unknown owner commands retain their existing preparation. No source change has been made yet; a schema-first route-selection fixture and independent argv/recording oracle must prove one preparation for the canonical test route, current-source invalidation, and unchanged direct/unknown behavior before implementation. This avoids introducing stale inherited receipts or weakening lease ownership.

Native output retention is SEMIO_TEST_ARTIFACT_DIR, read by library nextestArtifactLocation at line1404, producing policy artifactDirectory and retainArtifacts=true. SEMIO_NATIVE_GENERATED_ROOT has no reader and must not be used as the native retention contract.

## Explicit Consumer Body Repair

Actual11057 source semantic RED: external owner phases passed, new repository-body route was refused instead of executing owned→fresh preparation→Cargo. Node independent argv output and schema validated before failure. Native wrapper now has explicit repository-test-body admission, exact Bun/canonical owner script/test command validation, and the same owner policies/progress. Consuming body performs mandatory fresh preparation at runRepositoryCargoTests immediately before neutral Cargo driver. Arbitrary owner-command unchanged. No active-recursion environment, inherited receipt, fingerprint weakening, timeout or lease bypass is used.

Five reviewed standard test owners declare cargo-consumer in existing target metadata: Plugin, Cad, Process3D, Flow, Curation. Nx preserves this declaration for generated level siblings and emits explicit route. Plugin inventory/exact-law alternatives also prepare at actual Cargo probe consumption. Kernel and Value standard routes directly use neutral runCargoTestsV1 and therefore keep mandatory outer owner preparation. Custom/source/build commands remain owner-command.

Current green78071 source retry verifies actual wrapper and actual consuming orchestration through Bun/TypeScript compilers against independent Node argv recording. Whole zero-touch native acceptance is pending.

Actual78071 source GREEN Nxexit0:2tests/25assertions,5 closed neutral wrapper argv rows executed through both Bun and independent TypeScript compilers; independent Node verifies expected preparation phase records. Canonical Nx selector checks5explicit body declarations/level siblings and2direct-driver owners plus unknown/custom declarations. Actual emitted graph target receipt retained as generated/native-test-body-actual-nx-targets.json; zero-touch consuming native run is the next verification.

Actual emitted shared graph snapshot proves Plugin/Cad/Process3D base and long targets use repository-test-body, Kernel/Value remain owner-command. That snapshot still has an older Flow owner-command node and Curation was initially looked up under an incorrect name (actual sourcing-curation-rs). These are not five emitted-graph confirmations. Native zero-touch consumer validation is now running through a fresh task-private Nx graph directory release-6Z7Lxu/nx-data-test-body-validation, same current-task Cargo outputs, both caches bypassed, existing exact one-unit Plugin gate. Shared graph/caches/leases remain intact.

Catalog4 retry25742 advanced past reviewed prerequisite generators: latest log shows ui axes completed then process-extension-wood-rust:component-dev admitted. Four descriptors are not yet emitted/refreshed. Old and new native validation owners remain live.

## Actual Fresh Nx Projection

The task-private validation graph has now emitted all seven selected projects. Actual generated test/test-quick/test-long/test-exhaustive commands were inspected and asserted: all five declared Cargo-consumer owners route to repository-test-body; direct neutral-driver Kernel and Value owners retain owner-command preparation. The complete compact projection receipt is native-test-body-fresh-nx-targets.json (28 targets). This is emitted Nx graph evidence, separate from the still-pending zero-touch native assertion. Existing shared state was not removed or replaced.

The actual zero-touch target has now entered emitted native repository-test-body owner99294, immediately spawned canonical package test body99305, and then the sole current Cargo-consumption preparation99341. The retained live tree native-test-body-zero-touch-current-subtree.json contains no outer preparation between the native owner and test body. Mandatory current preparation is live before Cargo consumption. Runtime assertion results remain pending.

## Current Zero-Touch Runtime Receipt

Resumed canonical Plugin60148 completed actual Nx exit0: the exact one-unit retained-command law passed,1087 outside selection,0.032s assertions; target1m50. Its actual emitted execution route was repository-test-body -> canonical package test -> sole consuming preparation -> Cargo. Earlier interrupted87833 is not counted as runtime proof. Current log: native-test-body-zero-touch-resumed-current.log.
