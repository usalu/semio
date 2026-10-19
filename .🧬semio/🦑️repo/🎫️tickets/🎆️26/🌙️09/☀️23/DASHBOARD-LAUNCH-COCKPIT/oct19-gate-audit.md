# October 19 Control Authority Gate Audit

Read-only source audit. No source changes, Git mutations, worktrees, or test runs were performed. Source observations are current checkout evidence; concurrent changes may alter them.

## Actionable findings

1. Dashboard authority gate is self-contradictory. `dashboard/🧪️tests/🧭️authority/🟦️.ts:22` permits `.vscode`, `.vscode/launch.json`, and the launch seed; the authority schema ActiveRepositoryPath has the same exception at line 234. The fixture requires both launch files at lines 161-163 and accepts their paths at lines 904-908. The test named “registered editor launch projections preserve neutral build dispatch” (line 284) actively requires executable editor configurations. Remove these exceptions and replace the projection requirement with absence and dashboard owner/parameter verification.

2. Active inventory gate currently fails before traversal. `activeRepositoryPaths` line 38 calls `fixture.excludedDirectories.map`, but the fixture has no `excludedDirectories` property. It assigns the shared cache to `[]` before this failure, so subsequent tests can see an empty inventory. Build the inventory in a local variable, publish only after success, and explicitly model narrowly scoped cache/generated exclusions. Add traversal cases proving nested authored source under names such as `build` or `dist` remains inspected. A shared directory basename exclusion is overbroad for this repository architecture; exclude concrete generated roots and dependency/cache boundaries. Preserve ticket history as history.

3. IDE resurrection remains reachable through several independent routes. Root `📋️project.json:1840` and `:1848` register `generate-launch` and its check. Library `🎛️commands/🏭️generate/🟦️.ts:31-51` dispatches generation and imports the catalog writer. Library authoring command `🏗️authoring/🎮️command/🟦️.ts:118,175` registers crate launch routes after scaffolding. `🏗️authoring/🚀️launch-catalog/🟦️.ts:177-219` writes the seed and catalog. Its package authoring barrel exports these writers. Dashboard Rust package script imports `projectDashboardLaunch` at line 12, implements projection/writes around lines 40-50, and its manifest retains a `launch` target at line 284. Remove registrations, implementation, exports, and generator coupling together; leave ordinary crate/build registration intact.

4. Documentation gate masks `.vscode/launch.json` and seed references as `canonical-launch` (authority line 98). This lets instructions advertise editor execution. Remove masking and test actual live developer documents while leaving AGENTS.md unchanged per explicit user instruction. Do not weaken all instruction validation simply to accommodate root generic launch instructions; scope the explicit protected instructions exception precisely if necessary.

5. Tests outside dashboard encode IDE authority and must migrate with removal. Current concrete examples: library `🧪️tests/🚀️launch-catalog`, `🧪️tests/🏗️regeneration-commands`, `🧪️tests/📦️crate-tree/🚀️launch-routes`, Cargo replan tests, build-mode context tests, native runtime reader contracts, and build graph tests all read both editor files. Frozen-seal-ledger fixture simultaneously says these files were removed, so historical claims cannot substitute for active checks.

6. Bazel is now real build infrastructure (`.bazelrc` enables Bzlmod). Its `🏗️build/🔧️tools/🏗️bazel/🧪️tests/🟦️.ts:48` requires bilingual editor pickString launch inputs. Replace this with validation of the actual dashboard declaration parameters and all allowed integration phase values, then verify resolution into build-owned argument vectors. Do not remove Bazel or revert the custom repo build entry to old Nx conventions. The actual package bootstrap fixture/schema and root package already agree on `bun repo run` and the build script entry.

## Focused verification

- Run independent Ajv authority path/instruction/owner cases first with negative vectors for root and nested editor files and positive vectors for compiler-owned config and historical plans.
- Run the active inventory gate independently and twice in one process; exercise a traversal failure to prove no empty-success cache is published. Use a ticket-local fixture tree, with forbidden files below authored `build` and `dist` directories and allowed dependency/generated boundaries.
- Run root dashboard declaration and bootstrap checks against the actual current custom build schema. Keep repository command implementation canonical; ownership is more meaningful than superficial string bans on every internal Nx/Bazel reference.
- Run dashboard native dry-run resolution of representative retained live controls, typed parameters, forwarded args/environment, compounds, readiness, and cancellation declarations. Preserve runtime debug evidence already emitted by these tests.
- Run crate scaffold tests proving build-owner registration survives without editor writes. Use ticket-local scratch roots and assert an entire generated inventory contains no IDE configurations.
- Run config regeneration, dashboard install/build, and relevant registry regeneration in isolated ticket-local fixture roots where supported; assert they cannot recreate editor artifacts. Static symbol absence alone misses independent writer routes.
- Run Bazel integration dispatch validation against dashboard parameter declarations, including every integration phase and both locale metadata values. Run native execution only as appropriate to available toolchains; report unexecuted cases honestly.
- Run IDE passive navigation runtime tests and independent Babel/TypeScript evidence to keep repository operation handlers absent while retaining source navigation.

The derived-config implementation inspected has no launch/Nx matches; avoid indiscriminate removal of unrelated compiler and build configuration. This audit does not claim tests pass or runtime behavior is confirmed.
