# Native CI Implementation Ready For Actual RED

## Current Recovery Correction

The proposed recovery command is now `bun repo run repo-mcp:build --skip-cache` with SEMIO_BUILD_MODE=release at process birth. Do not pass --configuration: current mode resolution retains that argument and the early recovery grammar rejects it, causing ordinary selected-graph preparation. This supersedes earlier candidate command/hash text below. Current main patch SHA256 `c5d7d92fb5db5b4d90fbb35d8989d4a737777b95383da6c679c3a1b1dda663c0`; supplemental schema consumer patch SHA256 `816f0a991bf6011f9d3cc788a48263a8ba9d97501c975790e6250f7ca8306cc4`. Neither is applied.


Root now owns actualRED9854; native dry263416 remains separately live. No command was started or polled by this agent, and no workflow/production Source was edited during this preparation. Only ticket patch/report inputs were written.

## Minimal Patch

Input `native-ci.patch` is a unified patch,16582B/SHA256 `255649aded81c17670a0f4eabd380b4b87f6544debf81ab883d4edcf00355878`. It carries minimal hunks for3existing owned files, not a Source snapshot:

1. `.github/workflows/architecture-course.yml`:3conditional native setup blocks after pinned Bun,13ordinary command ingress replacements and3header examples.
2. Owning course readiness fixture: prepends exact parsed YAML setup-block scalar to3full run-step arrays and updates13ordinary commands. The earlier added nativeIngress contract remains unchanged.
3. Existing workflow test: removes the redundant positional legacy dependency-sync assertion; the new explicit13native ordinary-command roster covers this invariant for every job. All old dispatch/branch/runner/pins/permissions/login/artifact assertions remain.

Current Source fences before application: workflowd6d1e450b1d8141b20c70485909d6fd5ccb2081f01db4216151a4809629ce75c; readiness67da8556b13fbc86b4fa0064796c2aefc932617dca0b78fb00204404dbd421c0; test025ea9407d277db60de4855e978f48d0aae47b9019b88744c4e815a8a04523ed. Candidate hashes: workflowf2c35fc59f611cb992d7022d97ec01b601d3e21e6a215ff09cc15d50553367ac; readinessc21f36b2440c50ce59f7e847d30cbf81a9652c8937e483a1548097189498d8dd; testb1927ca919ee7526b5aa6286a8339c2763fda83d1f15df3d2d94a385a38151ba.

## Quoting And Recovery Audit

- Both local variables are task-specific. GITHUB_WORKSPACE, executable, PATH, HOME, build mode and GITHUB_PATH are double-quoted. The printf format contains one literal backslash in `%s\n`; it appends exactly one native directory line. There is no eval, substitution, dynamic shell program, raw Go/Cargo, detached child, retry loop, source-copy or compatibility wrapper.
- Initial exact release native help is the only tolerated failure. A failed/absent/MCP-only help triggers exactly one existing owned recovery `bun repo run repo-mcp:build --configuration=release`, inside the unchanged sudo env release envelope. It is the build-skill installer exception, not ordinary Bun ingress. Final exact native help is required before GITHUB_PATH publication. A stale receipt or ELF magic cannot bypass it.
- GITHUB_PATH takes effect in subsequent steps; same-step probes use the quoted absolute published binary. Every ordinary command still receives the existing PATH/HOME/SEMIO_BUILD_MODE/CI envelope. The build owner uses current native host compilation with no cross-target GOOS/GOARCH override.
- This architecture release workflow is authored for Debian12Linuxx64. Its bash+POSIX release output path is correct for that scope. Generic native owner publication/installation already supports native macOS and Windows (Windows repo.exe); this patch creates no parallel installer. A macOS runner would reach the existing native Linux image guard, and Windows is not an admitted architecture release runner. Do not present this Linux workflow recipe as a cross-platform standalone installer or claim a Windows run. Current workflow runner-variable validation only checks configured-label presence; actual platform/ABI checks remain the authored native image producer.
-17neutral cases remain input DATA; exact YAML recipe assertions do not execute conditional shell branches. No17case pass, actualRED, GREEN, cold native build or zero-touch CI pass is claimed.

## Apply After Actual Assertion

Wait for Root9854 to collect the new named test and fail `check has no native frontend provisioning` (or inspect its actual assertion). A precollection infrastructure refusal is not RED. Recheck3Source hashes, review/apply only the small unified hunks without Git management, then schedule the same native architecture:test gate for GREEN. If peer edits changed a fence, reread and preserve those edits while reauthoring affected hunks; never overwrite wholesale. After declaration GREEN, an actual owned native release frontend help/build and Linuxx64 cold-provision/product evidence are still required. Do not trigger the publishing workflow merely to test provisioning.

## Run Versus Exec Selection And Leases

Source confirms `buildInvocationGraphSelectionV1` in build graph admission:109–113 returns all for every non-run request. Root exec--projects=workspace therefore selects `{kind:all}` before the later one-owner exec plan. Native RED `run @teaching/architecture-course:test --configuration=release -- quick` selects `{kind:tasks,tasks:[{project:@teaching/architecture-course,target:test,configuration:release}]}`. The manifest has no dependsOn for this test, cache:true/default inputs/outputs[], but selected owner admission still closes command Source and imported prerequisite ownership. It is a smaller closure, not a zero-bootstrap direct Vitest call.

The publication locks differ:

- Global graph: exclusive `repository-graph-facts:<workspace>` and exclusive `build-graph-state:<graphStoreRoot>`.
- Selected graph: exclusive `repository-graph-facts:<workspace>:shard:<N>` where N=SHA256(selection namespace)[0] modulo configured writerShards; and exclusive `build-graph-state:<graphStoreRoot>:selected:<N>`. It writes selected/<scope>.json, not the global canonical graph. Selected scopes can collide by bounded shard; actual N was not evaluated here.
- Both can briefly share serialized optional quota/ledger custody `repository-graph-optional-ledger:<workspace>`, graph allocation/charge writers, immutable snapshot writer `build-graph-store:<graphStoreRoot>`, and shared per-content `build-graph:<snapshotPath>` readers. Ordinary image use is retained shared native-fixed-image custody, with exclusive creation/maintenance transitions guarded by the provider.
- Dependency consumers use shared `javascript-dependency-bindings:<workspace>` under dependencies store reader custody; producer/sync requires exclusive whole-image/binding custody. No sync or manifest/lock edit is part of the RED lane. A reader can still wait behind existing maintenance.
- Actual Nx execution additionally retains tooling use/storage, its graph namespace and rust/Nx fixed cache; cache operations use `nx:fixed-cache:<imageRoot>` with the requested shared/exclusive mode and exact output/cohort leases. Root dry2 may not yet have reached Nx. No current holder or wait is inferred from these declarations.

The smaller selected graph has distinct long inference writer resources from the global planner, enough to justify Root deliberate bounded lane, while common short store/ledger/cache custody means progress independence is not guaranteed. Source-only inspection cannot replace actual logs/terminal receipts or identify63416current phase.
