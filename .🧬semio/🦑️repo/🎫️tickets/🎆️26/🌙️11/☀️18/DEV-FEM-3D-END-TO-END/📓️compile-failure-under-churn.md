# Native component build failing on peer edits

## Cause

`runNativeBuildDriverV1` (`🧩️component-pass/🟦️.ts`) turned every non-zero cargo exit into `Native build failed: ...`. The post-build provenance recheck (`session.close()` -> `custody.recheck()`) only ran when there were no failures, so a pass that failed because a peer edited its inputs mid-build (Cargo.toml dependency added after cargo read the manifest; a helper defined after the unit compiled) was a hard failure instead of a restartable `ConcurrentEditErrorV1` (exit 75, `concurrentEditV1`).

## Change

- New `🧩️component-pass/🔁️failure-churn/🟦️.ts`
  - `failedCompilerMessageFilesV1(message, cwd)`: the repository files a failing `compiler-message` names (its `manifest_path` and error-span sources); paths outside the checkout (registry, std) are ignored.
  - `failedPassConcurrentEditV1({startedMs, files, recheck})`: reuses the existing custody recheck (`custody.recheck()`, the same digest-memoised rehash of every file the build hashed) and maps a digest difference or vanished file to `concurrentEditV1(..., "build")`; then checks every file a failing unit read: vanished -> `vanished`, mtime after Cargo started -> `bytes`. Returns `undefined` when nothing moved.
- Driver: collects `failedFiles` from error compiler messages, runs the check once after all lanes settled (before the generation resources are disposed) when a cargo pass failed, and throws the edit (after the report is written, in place of `Native build failed`) naming the changed file. Unchanged inputs fall through to the old hard failure.
- Scope notes: a failing unit's own sources are never hashed by provenance (no artifact, no dep-info), so for those files the baseline is Cargo's start time instead of a digest. Files hashed by completed units are compared by digest through the custody. New files in a mounted module dir are not detected (no roster baseline exists in the driver).

## Tests (`🧩️component-pass/🧪️tests/🟦️.ts`)

- `failed pass inputs: ...` (pure, passes): unchanged, modified-after-start, vanished, custody digest change, non-edit recheck error, foreign-path filtering.
- `a failed cargo pass whose failing unit's inputs changed ...` (driver, stub cargo modes `error-last` and `error-touch`): unchanged inputs -> `Native build failed: comp-b`; peer rewrite of the failing source -> `ConcurrentEditErrorV1` reason `bytes` with that path.

## Native run result (Windows host, graph bypassed via native-run.ts, whole `native-artifacts` suite)

- `failed pass inputs: ...` (pure): pass.
- `error-last` / `error-touch` driver test: not runnable on this host. Every driver test of the file that goes through `runStub` (including the pre-existing ones such as the unlinked-component and stderr-exit tests) fails before cargo starts with `macOS fixed image enforcement is unavailable on this platform` (`withLawStorageV1` creates a macOS fixed image). Must be run on a macOS host.
- Pre-existing Windows failures in the same file, unrelated: unit-graph identities (`wasm32-wasip2` platform null) and component batches (`File URL path must be an absolute path` for the `path+file:///w` vector).

## Windows harness work (whole `native-artifacts` suite via native-run.ts, na6..na10)

Changes made so the component-pass driver tests can run off macOS:
- `🧪️tests/🦀️cargo/🟦️.ts`: `withLawStorageV1` opens storage through `openNativePlatformStorageV1` (the product's platform selector: mounted image on macOS, directory partition elsewhere) instead of the macOS-only image creator; takes an optional `invocation` and, off macOS, runs inside `withRepositoryCompilerRuntimeEntriesV1` (the directory partition needs a captured runtime entry provider).
- `🧩️component-pass/🟦️.ts`: `announcedUnitIdentityV1` / `announcedRunIdentityV1` split artifact paths on both separators (was `sep`; fixed the `null` platform failure).
- `🧩️component-pass/🧪️tests/🟦️.ts`: batches vector uses the drive-aware `path+file:///w` rewrite (fixed `File URL path must be an absolute path`); jobserver assertions accept the Windows `semio-jobserver-` auth value next to `fifo:`; `runStub` passes `original`.

State after the last run (na10): 13 of 32 tests in the file pass, including the first two driver tests (streams components, stderr-exit). Unit-graph identities and batches now pass. All later driver tests, including `error-last`/`error-touch`, fail with `Original process admission closed - first failure: Native build failed: cargo exit 101 (release-hub) ...`: the first deliberately failing driver test (stderr-exit) registers its error as a failure of the shared test birth (`birth.failures`), which then refuses all later admissions. On macOS the driver runs under the accepted-cleanup body control of the image, which contains that failure; the directory-partition path hands the body the outer control. Not yet resolved: the body needs a failure-containing control on the directory-partition path (or the platform image open should hand one out like the mounted image does).
An isolated single-file run through `repo tools exec bun test` is not equivalent (PATH `cargo` resolves to the refusing compiler ingress), so only the whole-suite route is meaningful.

## Final state (native-run, whole `native-artifacts` suite, na17)

component-pass file: 19 pass, 15 fail (34 tests). Passing now on Windows, among others: unit-graph identities, component batches, driver streams components, `error-last` (hard failure stays hard), `error-touch` (restartable `ConcurrentEditErrorV1` naming the touched lib.rs), stderr-exit, unlinked component, storage refused, cancellation, stall watchdog, `failed pass inputs`.

Changes beyond the churn fix:
- Product: `withNativePlatformImageStorageV1` (💾️storage/🌐️platform) is the platform-neutral scoped body (mounted image on macOS, directory partition + accepted-cleanup body control elsewhere).
- Harness: `withLawStorageV1` uses it (and the repository runtime-entry provider off macOS); jobserver assertions accept the Windows auth value; announced identities split on both separators; batches vector uses the drive-aware rewrite; `runStub` default control 120 s.
- Tests that end in a rejection are registered with `registerIsolatedBuildTestV1` (the stated rule: a rejected issued operation closes the shared Original). `error-touch` installs a no-op restartable-failure handler so the escaped concurrent edit does not restart the child (the isolation runner requires exactly one primary record). The cancellation/stall test is split in two.

Remaining 15 failures (all pre-existing, not from this change): every test that needs the real scheduler binaries (`schedulerBinaries()` -> `repo cargo --store=shared`) fails with `Internal error: Original Repo build caller required` because the native-run route has no graph/build-caller identity; the rejection of that operation then closes the shared Original for the plain tests after it (the queued/unit-graph/finite-budget/pending-stage cancellation tests were not isolated). Needs a graph-based run (or a build caller) to verify; I did not compile cargo here per the instruction to keep cargo use minimal.
