# Phase 9 Z0 Root Dependency-Gate Import Repair Fresh Audit — 2026-08-25

## Verdict

**RED for the requested whole-repository stale-route closure; GREEN for the repaired root dependency gate itself.**

The repaired root repo policy router now imports the three required APIs from the existing, repo-owned Library boundary, the root dependency verifier starts by enforcing that exact specifier and an in-repository existing target, and the exact required Go literal command returns `[]` with exit 0. No shim or external package is involved.

The broader requested assertion that the deleted Math/Graph route is fully replaced and that no unrelated policy router regressed is false in the current worktree. `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/📜️script.ts` still imports four symbols from the deleted Math/Graph path and fails before dispatch with Bun's missing-module error. The same Library boundary already exports its needed `BundleLinter`, `getWorkspaceRoot`, `runPolicyOnlyMain`, and `defineLint` APIs. This is a real executable stale route, not a historical ticket reference.

No source, manifest, baseline, lock, launch file, Cargo, Nx, Wasm, browser, or product build was changed by this audit. This report is the sole audit output.

## Root Repair Evidence

`🧰️framework/🛍️products/🦑️repo/📜️script.ts:3` has one direct local import:

```ts
import { defineLint, runPolicyOnlyMain, type TechnologyLinter } from "./🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
```

The target is inside the repository and is not changed by the repair. Its direct exports are `TechnologyLinter` (line 233), `defineLint` (478), and `runPolicyOnlyMain` (1046). Executing the router reaches its usage diagnostic (`usage: bun ./📜️script.ts policy`), proving all three imports resolve; it does not emit a missing-module failure.

At the top of `VerifyScript.runDependencyFreeze`, before any subcommand dispatch, `dependencyAssertRepoPolicyImportBoundary(this.root)` checks the exact owner specifier, verifies its resolved path remains inside the repository, and verifies that file exists. Therefore every `verify dependencies` mode takes the same gate. Its diagnostics identify both the importer and owned module. The self-test injects both a missing target and a stale specifier and completes with `hostile-mutations=15 clean`; those two hostile cases are explicitly required to produce the owned-boundary diagnostics.

The root router diff replaces only the three deleted Math/Graph imports with the single Library import. No compatibility layer or external dependency was added.

## Reproduced Gates

| Gate                                                                                    | Result                                                                                |
| --------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| `bun ./📜️script.ts verify dependencies list go --literal-external --format json`        | **GREEN**, exit 0, exact `[]`                                                         |
| `bun ./📜️script.ts verify dependencies self-test`                                       | **GREEN**, exit 0, `hostile-mutations=15 clean`                                       |
| `verify dependencies summary` (text)                                                    | **GREEN**, exit 0                                                                     |
| `verify dependencies summary --format json`                                             | **GREEN**, exit 0                                                                     |
| `verify dependencies list all --raw`                                                    | **GREEN**, exit 0, 172 entries                                                        |
| `verify dependencies list all --literal-external`                                       | **GREEN**, exit 0, 154 entries                                                        |
| `verify dependencies literal-external` (text)                                           | expected **RED fail-closed**, exit 1 at literal 154                                   |
| `verify dependencies literal-external --format json`                                    | expected **RED fail-closed**, exit 1 at literal 154                                   |
| Bun TypeScript transpilation of root script, repaired repo router, and Library boundary | **GREEN**, 3 files, 0 errors                                                          |
| `git diff --check -- 📜️script.ts …repo/📜️script.ts`                                     | **GREEN**, exit 0                                                                     |
| Prettier on repaired repo router                                                        | **GREEN**, exit 0                                                                     |
| Prettier on root `📜️script.ts`                                                          | **RED pre-existing whole-file formatting gate**; `HEAD:📜️script.ts` fails identically |
| `bun …repo/🔨️modules/💻️client/📜️script.ts`                                              | **RED**, stale deleted Math/Graph import fails resolution                             |

The literal-zero failures are intentional: `target=0, current=154, oracle-conflicts=3, toolchain-owner-conflicts=2, toolchain-failures=0`. The exact JSON gate reports the same values.

## Direct Manifest Reconciliation

The live verifier and direct-manifest inspection agree on the current target counts:

| Ecosystem  |     Raw | Third-party | First-party | Composition-scoped | Fully mandated | Literal external | Production-reachable |
| ---------- | ------: | ----------: | ----------: | -----------------: | -------------: | ---------------: | -------------------: |
| Rust       |      85 |          85 |           0 |                  0 |              0 |               85 |                   65 |
| JavaScript |      70 |          70 |           0 |                  0 |              1 |               69 |                   31 |
| Go         |       2 |           0 |           2 |                  0 |              0 |                0 |                    0 |
| Python     |      15 |          15 |           0 |                 15 |              0 |                0 |                    0 |
| .NET       |       0 |           0 |           0 |                  0 |              0 |                0 |                    0 |
| **Total**  | **172** |     **170** |       **2** |             **15** |          **1** |          **154** |               **96** |

Independent manifest facts: 123 non-Composition Cargo manifests yield 85 distinct external Rust identities; the dependency walk's 85 non-Composition, non-policy-cache package manifests yield 70 external JavaScript identities; the four non-Composition `go.work` modules declare only the two existing local identities `github.com/usalu/semio/repo/client` and `github.com/usalu/semio/repo/go`; root `pyproject.toml` has 15 dependency-group entries and an exclusively `compose/` UV workspace; and the sole non-Composition `.csproj` has no `PackageReference`.

Owner-scoped JS toolchain evidence is exact and honest: root `package.json` has three valid, lock-owned Nx rows; only root-only `@nx/js` is fully mandated; and the Library package's `@nx/devkit@21.4.1` plus `nx@^21.4.1` remain literal external, recorded as two lock-owned owner conflicts. Thus all five audited rows have lock evidence (`5/5`), with no toolchain audit failures.

## Boundary And Change Audit

The repair-specific diff is restricted to root `📜️script.ts` and the root repo policy router; the Library boundary has no repair diff. The scoped diff contains no `🔒️dependencies.json`, package manifest, `bun.lock`, Cargo manifest/lock, Go workspace/sum, Python manifest, `.csproj`, `project.json`, or launch-file change. Other worktree changes are unrelated concurrent work and were not attributed to this repair.

The stale path scan outside `.🧬semio/` proves the root router is repaired but also finds the executable client-policy router named in the verdict. Its stale imports are not covered by `dependencyAssertRepoPolicyImportBoundary`, which deliberately validates only `DEPENDENCY_REPO_POLICY_SCRIPT` (the root repo router).

## Required Closure

1. Replace the client bundle policy router's deleted Math/Graph imports with the sibling repo-owned Library boundary and rerun its policy usage/import gate.
2. Extend the dependency import-boundary guard or add a sibling guard so every repo product policy router with this contract is covered; otherwise a stale client route can return without any `verify dependencies` diagnostic.
3. Resolve the pre-existing root-script Prettier drift separately if whole-file formatting is a release gate.
