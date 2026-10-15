# Cargo Evidence Destination Audit

Read-only Source/log inspection. No production edits, tests, builds, lock refresh or SDK publication were launched for this audit. Tools/provider Source stays frozen for Root's capacity qualification.

## Finding

`nextestArtifactLocation(cwd, env)` now requires incoming `SEMIO_TEST_ARTIFACT_DIR` and returns `retain:false`. This removes routine indefinite evidence retention, but an incoming path is not a zero-touch prerequisite that ordinary package callers already satisfy.

`repositoryCargoTestPolicyV1` calls that helper while constructing the policy, before `runRepositoryCargoTestsV1` reaches its `execute()` callback. Only inside `execute()` does `withRepositoryCompilerStoreV1` hold actual tools/Rust backing and replace `targetDirectory`, `buildDirectory`, `artifactDirectory`, and coverage output with the issued store's paths. The compiler store itself sets `SEMIO_TEST_ARTIFACT_DIR=store.captureDirectory` in its outgoing environment. A standard `Script` only retains root/repository/invocation; its router and original process-envelope receiver do not create that environment binding.

Therefore the current required incoming path can reject a legitimate ordinary launch before its actual bounded default is available. A policy is a plan, not an issued writer grant; its pure default can name the canonical destination without allocating it.

## Primary callers

- Repository generic admission owner `library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts`: `runRepositoryCargoTests` constructs `repositoryCargoTestPolicyV1(manifest,cwd,env)` after workspace preparation but before `execute()` opens its actual compiler store. Its child environment remains the incoming `env`; workspace preparation does not mutate that object into an issued compiler environment. This is the shared path for YAML, most Repo packages, OS Kernel and many other ordinary script test routes.
- Eleven direct package scripts construct policy from `process.env` as an argument to `runRepositoryCargoTestsV1`: PackSource; UI budget/text/identity; borrowed artifact-reference; composition-admission; OS Blob identity and Link schema; Job preview, outcome publication and fault publication. Each likewise constructs the policy before the wrapper can project actual bounded paths.
- Root `📜️script.ts` canvas-native test branch constructs an `env` containing selected test level/process context, performs bounded preparation, then calls `repositoryCargoTestPolicyV1(manifest,cwd)` **without passing that prepared `env`**. The subsequent child receives the constructed environment. Besides evidence fallback, this caller should consistently pass its selected environment to policy construction; current default `process.env` can differ from selected level/context.

The direct callsite census is retained as generated audit output; it contains the eleven leaf scripts plus the root canvas branch. These are Source observations, not runtime qualification claims.

## Clean bounded default

When no nonempty explicit evidence destination is supplied, derive the destination through the existing owners:

1. Resolve the actual repository with `getWorkspaceRoot(env)`.
2. Resolve `repositoryCachePartitionRootV1(repository, 'rust')` from authored fixed-storage policy.
3. Read the explicit build mode and project `repositoryCompilerLayoutV1(rustRoot, mode).captureDirectory`.
4. Return that directory with `retain:false`.

The current authored Rust partition is `.🧬semio/🦑️repo/⚡️cache/rust/fixed`, and its capture layout is `cargo/{prototype|release}/captures` inside the mounted root. These spellings should not be duplicated in the evidence policy: the partition/layout helpers own them, including future compact layout changes and native separators. The existing compact build `b/{p,r}` and target `t/{p,r}` namespaces are separate; deriving evidence from Cargo's raw `target-dir` or `build-dir` config would confuse them.

This pure projection performs no mkdir, mount, allocation or receipt adoption. `execute()` must continue replacing the plan with actual issued store paths, and lower `admitCargoExecutionV1` must keep checking the real storage reservation, image ancestry and original control before `runCargoTestsV1` creates its per-operation evidence namespace. Explicit incoming evidence paths remain plan intent; they do not grant physical storage authority. Routine `retain:false` needs no new retention switch.

## Native counterexample classification

Root's retained `root-cargo-evidence-native-prototype.log` is an actual status-1 counterexample at lower `admitCargoExecutionV1` line 181, reporting `Native storage path escapes enforcing boundary`. It reached an actual compiler store and bounded dependency fetch. It is **not** evidence that the incoming evidence-path requirement failed in that run: the private test passed `store.environment` and explicitly placed its evidence child under `store.captureDirectory`.

That private test calls lower `runCargoTestsV1` directly rather than `execute()` and only overrides `artifactDirectory` in its constructed policy. Its `targetDirectory` and `buildDirectory` still come from the policy factory's Cargo-directory projection. Lower line 181 checks target, build, artifact and optional coverage paths together; the retained error does not identify which field escaped. Diagnose it with the exact policy/store path tuple under the original scope before changing a guard. The clean ordinary policy fallback above resolves the separate pre-store construction defect; it does not by itself prove this lower native failure fixed.

## Required successor evidence

Use closed neutral cases for absent/blank path, both explicit modes and native separators; compare the fallback to the canonical partition/layout owner, keep explicit-path semantics and routine cleanup. Then qualify the actual generic ordinary caller without incoming artifact env and the direct lower fixture using all actual issued target/build/capture paths. Repeat the real test operation to prove its uniquely owned evidence directories are gone after settlement. No temporary-directory fallback, unbounded retained capture or weakened image path check is needed.
