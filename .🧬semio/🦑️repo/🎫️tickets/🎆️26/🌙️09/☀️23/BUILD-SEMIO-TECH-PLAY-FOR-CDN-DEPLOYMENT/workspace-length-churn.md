# Workspace Manifest Length Churn: Exit 1 Instead of Restart (75)

## Cause

1. A peer edit of a workspace `Cargo.toml` during a job made the workspace selection recheck throw the typed `concurrentEditV1("length", ..., "snapshot")`. That throw was the first failure of a tracked scope, so the scope closed.
2. Cargo had already failed, so `captureCargoArtifacts` (`native-build/🟦️.ts`, compiler-failure branch) ran `recheck()` to see whether an edit explains it. A later admission of the closed scope refused with `OriginalAdmissionClosedV1` ("Original process admission closed - first failure: ..."), whose `cause` is the typed edit.
3. That branch only accepted `error instanceof ConcurrentEditErrorV1`. The wrapper is not an instance, so it was swallowed and the plain `Error("Cargo artifact build failed: ...")` was thrown without a cause.
4. `exitIfStaleImplementationV1` (format-stamps) maps a failure to exit 75 only when `concurrentEditOfV1(error)` finds a typed edit in the cause chain. It found none, so the process exited 1. `runNativeBuildJobAttemptsV1` restarts only on 75, so it threw `exit 1`.

The same `instanceof` pattern was at four more sites that return an edit as a value so the scope is not closed.

## Catalogue law gap

`🧾️refusals/🧪️tests` only checked that every declared message exists at its declared site (and typed-constructor use). It never enumerated the `concurrentEditV1(` sites, so an unregistered message could not fail it. "Original Cargo workspace Source length changed" and "Original consumed Cargo profile Source length changed" (build layer) were both unregistered, along with about 40 other snapshot/build messages. Catalogue membership only affects classification of child output text; it was not the cause of exit 1.

## Fix

- `🔁️churn/🟦️.ts`: new `concurrentEditErrorOfV1(error)` returns the typed edit an error is, or carries as its cause.
- `native-build/🟦️.ts`: the five `instanceof ConcurrentEditErrorV1` sites that return or record an edit now use it (the plan task, the capture catch, the post-failure recheck, and the two consume-phase catches). `component-pass/🟦️.ts`: the first-stage-failure rethrow uses it too.
- `🧬️schema/🧾️refusals.json`: new classes (en/de describe) `current-physical-source`, `compiler-source-custody`, `cargo-lock-closure`, `admitted-cargo-plan`, `cargo-profile-policy`, `cargo-preparation-custody`, `unadmitted-source`, `failed-task-inputs`. `cargo-workspace-topology` gains the length, disappeared and changed messages. I removed two stale entries whose throw sites no longer exist ("Stale loaded Graph reader policy", "Stale loaded selected Rust graph assembly"). They already failed the existing law at HEAD.
- Law: the refusals test and its Python oracle now scan every `concurrentEditV1(` call from the `snapshot` or `build` layer with a literal message head and require a class declaring that message at that file (59 sites, none missing). The test also checks the scanner on synthetic text.
- Vectors: `workspace-length` (the failing text), `workspace-vanished`, `profile-length`, `plan-drift`, `compiler-copy`.

## Residual

- Calls with a dynamic or absent message head (about 15 sites) are not covered by the law.
- The `snapshot` test file has 3 failures that I did not touch and that do not involve the changed code (attempt counting and timing). I did not check whether they fail without my change.
- The native-build and component-pass suites need the repo script envelope and could not run under bare bun; both files transpile (`bun build --no-bundle`). Not run under `repo run`, to avoid competing with the running release build.
