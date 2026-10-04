# General script neutral execution mapping

[Fourteen current route/API/GUI bodies](general-script-neutral-wrapper-audit-inputs/full-current-route-api-gui-closure-1.json) retain hashes and complete General command/launch closure.

| Repo wrapper | Canonical execution owner | Repository behavior to retain outside General |
| --- | --- | --- |
| runRepositoryCargoTests | process/testing/cargo runCargoTestsV1 + readCargoTestPolicyV1 | Package→manifest discovery, workspace preparation, package assertion budgets |
| runRepositoryExactCargoLaws | process/testing/cargo/exact runExactCargoLaws | Explicit package manifest map, target directory, managed preparation on probe |
| runCargoLint | process/testing/cargo runCargoLintV1 | Exact manifest/package selection; retain all-targets/-D warnings |
| runCmdStatus | process/capture captureOwnedProcess (async status receipt) | Managed preparation, diagnostics and explicit build budget; no sync wrapper alias |
| runRepositoryTestCommand | process/testing/execution runBudgetedTestCommand | Preparation only for Cargo; retain explicit test-level/build budget and options |
| runVitest | process/testing/vitest readVitestPolicyV1 + runVitestV1 | Injected explicit Vitest policy/tooling environment |

The native owner-command route already prepares the supplied manifest, validates its package/workspace, injects Cargo test policy for package manifests plus Cargo artifact policy, Vitest policy and process owner context, then runs the child. The ordinary nonnative owner-command injects only Vitest policy/process context. These are distinct guarantees. A multi-package or exact-law route cannot assume the one injected manifest policy supplies every package: retain an explicit bounded manifest/policy map or transfer the cross-owner orchestration to Repo. Direct neutral APIs should consume policies, not discover Repo ownership.

General TestScript's neutral Cargo/Vitest pair, wire-retirement, core modules, deflate, action/manifest laws, package descriptor codec, lint/typegen and command-ingress consumer routes can remain General with direct process APIs and explicit owner policies. Preserve their original arguments, exact filters, build/test budgets and diagnostics. Changing synchronous status callers requires async propagation in PackageDescriptorValueCodecTestScript and typegen generate/preview/check callers; their failure exit semantics must remain exact.

FixtureOwnershipTestScript is a mixed roster: surface and artifact-flow plus OS-flow, OS-kernel and OS-MCP actual laws. Split native groups into their true owner gates; the cross-owner aggregate belongs Repo orchestration. Preserve all named groups and original stack/list/law budgets. ArtifactKindTestScript's portable neutral source law may remain IO/General; its exact os_io native law belongs OS. SnapshotSqliteTestScript's `io` branch explicitly selects OS-kernel sqlite_snapshot_native_admission and plugin sqlite_snapshot_ laws: transfer those selections to OS/plugin owner routes, preserving filters. Source/native neutral SQLite paths and interoperability oracle remain neutral.

Updating classes alone is incomplete: General project targets/package commands and both authored/generated GUI routes must point to actual owner commands. Full current files are retained to hand-edit this closure; no compatibility forwarding aliases should remain. Actual registered reruns must confirm preparation and cancellation behavior after migration. No compiler/native/generator executed in this audit.
