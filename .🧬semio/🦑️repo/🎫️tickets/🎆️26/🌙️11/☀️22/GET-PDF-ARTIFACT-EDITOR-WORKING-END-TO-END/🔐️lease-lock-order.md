# Lease lock order and `--parallelism` selection

## 2. `repo cargo --parallelism=2`: "Undeclared original Repo build selection" (fixed)

Cause: `readOwnedBuildInvocationV1` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts`) strips the host capacity grant before comparing a request with the declared launches. It stripped `--parallel` and `--parallel=`, but the grant for `cargo` is spelled `--parallelism` (see `🌡️host-capacity/🟦️.ts`, `name=operation==="cargo"?"--parallelism":"--parallel"`). A granted `cargo` request therefore never matched its declared launch.

Fix: `granted()` also strips `--parallelism <n>` and `--parallelism=<n>`.

Test: `🏗️build/📥️caller/🧪️tests/🌐️globals/🟦️.ts` gained a cargo spelling test. It passes (suite `build-caller-birth`, 2 pass). I did not run it against the unfixed code to confirm it fails there. `repo cargo --parallelism=2 --help` now prints help.

## 1. Lock order violations under `repo:compiler-writer:` (diagnosed, not changed)

Policy: `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🔣️policy.json`. The writer custody is class `image`, rank 7. Artifact leases are rank 6. Cargo store, build and target-store leases are `cargo-coordination`, rank 2. Custody is `cargo-custody`, rank 2.

I tried a model correction: a separate `compiler-writer` class, a `cargo-lane` class and `allowed` edges. I reverted it completely. The policy is unchanged and the order tests pass again (78 and 24 tests). The reason is the committed fixtures `cargo-test-custody-inside-writer` and `cargo-custody-inside-cpu-units`. They record a real three-way cycle: a build holding the shared writer queues for the build-directory custody behind a peer that holds the custody and waits for a queued maintenance writer. Cargo build, store and custody leases under the writer are therefore a deadlock hazard, not a wrong declaration. The `wasm-build` test documents the same rule for artifact capture: take it after the writer is released. The ranks are right and the code that nests the other way is wrong.

Where the nesting comes from (static reading, not yet confirmed at runtime):
- `🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts`. `withCompilerWorkspaceStageWriteV1` is entered around the describe stage (`withDescribe`) and around the browser publish (`withBrowserLeaseTask`, `publishComponentBrowserV1`). `withCompilerWorkspaceSharedWriteV1` is entered around `stage.run`. Their bodies take `artifact:`, `artifact-capture:` and `artifact-scratch-private:` leases, which matches the `browser/prototype/modules/...` violations.
- `🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts` calls `acquireCargoBuildLeaseV1` at line 97. That takes the store, build, target-store and custody leases, and it runs inside the describe stage writer. This matches the `r/m/u/l/p036cce` violations.
- The launcher and the Cargo test, lint and exact paths are already correct: they take the build lease first and the writer second. `withRepositoryCargoRegistryReadyV1` holds the writer only around the fetch.

Needed fix (not done): take the stage's cargo build lease and artifact leases before `withCompilerWorkspaceStage/SharedWriteV1`, in the same order the launcher uses. The artifact lease is acquired inside `publishComponentBrowserV1`. I did not restructure those stage bodies, because I could not check the change against the running Rust and browser pipeline in this session.

Regression coverage: `🧪️tests/🧫️fixtures/🧩️component-driver/🔣️.json` already holds the cargo case. An artifact-under-writer sequence and a static scan of the driver-stage callbacks (like the existing `cpu-last` scan) should be added together with that restructuring.

## Follow-up: restructuring attempt (no code changed)

Reading the call chain again shows the stage writers are not the root, and correcting my first diagnosis:

- `withCompilerWorkspaceStageWriteV1` in `🧩️driver-stages` (describe, browser publish) is re-entrant: when the task already holds the writer for the same root it only opens a `writerOperation` and takes no new lease. The same holds for `withCompilerWorkspaceSharedWriteV1` inside `stage.run`.
- The writer that is actually held is the driver-wide one. `runRepositoryComponentDriverV1` passes `withCapacity`, which wraps the whole compile in `withCompilerWorkspaceSharedWriteV1` plus the cpu units (`🧩️driver-stages/🟦️.ts`, `withCapacity`). The component pass (`🧩️component-pass/🟦️.ts`) runs every post-link stage (describe, transpile, optimize, browser publish) inside that `withCapacity` callback, "beside the rest of the compilation" by design.
- So every lease a stage takes is requested under the shared writer: the artifact reader, scratch and destination leases of `publishComponentBrowserV1` and `publishComponentModuleV1` (`withRepositoryCacheStoreV1`, `withRepositoryArtifactDestinationV1`), and any cargo store/build/custody lease taken by a stage or lane helper. Inside the writer the browser publish does acquire its artifact leases before its own `withCompilerFiles` call, which is ascending, but that call is nested in the driver writer.
- `🏭️fresh-component` (`acquireCargoBuildLeaseV1` at line 97) is not on this path: `freshRun` has no non-test caller in the plugin build tree. It is unchanged.
- The build leases of the component pass itself (`buildLeases.push(await acquireCargoBuildLeaseV1(...))` before `withCapacity`) are already in launcher order.

Why not "take the leases first": the stage leases depend on stage outputs (the component name, the module scratch, the destination) that exist only after Cargo links the component, which is inside the driver writer. Pre-acquiring them would mean holding artifact scratch and destination leases for the whole build for every component, which changes concurrency and the artifact protocol. The alternative is to let post-link stages leave the driver writer: run them after `withCapacity` returns, or give each stage its own short shared writer opened after the driver writer is released (the stage writer is already re-entrant, so it would then take a real lease). Both change the driver scheduling contract and need the native-build driver tests plus a real build to validate. I stopped before changing it, since no Rust build may run now, and a red regression test with no fix behind it would only add noise.

Unchanged and passing: order policy tests (78) and component-driver tests (24). No `repo cargo` run was made, so `repo leases status` was not rechecked.

## Follow-up 2: option 2 not implemented, because the ordering is already in place

The ORDER lines in `repo leases status` are stale history, not current behaviour. The JSON status (`repo leases status --json`, `violations`) carries `atMs` and `pid`:

- Artifact and capture violations (`preview2-shim`, `artifact-scratch-private`, `artifact-capture`, `artifact:` of `browser/prototype/modules/🗒️note`): pid 97753, Sat 2026-11-21 11:09 CET. `📦️model/🔒️lease-task/🟦️.ts` (`captureComponentProductLeaseTaskV1`, which opens the product lease task from a snapshot taken before the compiler writer) is dated 2026-11-21 12:59, and the enforce-mode stage tests (`🎟️stage/🧪️tests`, "component model publication acquires product leases outside compiler writer custody" and "component browser publication reads vendor before its compiler writer and publishes after release") are dated 13:05. The violations predate both.
- Cargo store, build, target-store and custody violations (`.cache/b/r/m/u/l/p036cce`, profile `debug`): pid 87111, 2026-11-22 14:07 CET, one build-lease acquisition inside a writer. The launcher, `runCargoTestsV1`, the exact-law path and the native-dependencies installer all take the lease first and the offered writer second in the current source (see the doc comment of `runCargoTestsV1`). That fits the `debug` profile (the test path) being fixed after this run. I could not identify the exact pre-fix site.

Because the stages already leave the driver writer in lease-order terms (fresh lease task from a pre-writer snapshot, vendor reader and scratch taken before the stage writer, publication after its release), changing the driver to release the compile writer after link would change scheduling for no ordering gain, so I did not do it.

Validation run now (no code changed in this follow-up):
- Order policy tests 78 and component-driver tests 24: pass (earlier in this ticket, policy unchanged).
- `🎟️stage/🧪️tests` with `SEMIO_LEASE_ORDER=enforce`: 6 pass, covering model and browser publication ordering.
- `describe/emitter/tests/request` (driver-stage consumer): 6 pass.
- `repo cargo --store=shared --configuration=prototype --parallelism=1 -- metadata --format-version=1 --no-deps` (launcher path, no compile): exit 0. `repo leases status` before and after shows the same 10 ORDER lines, same two pids, newest `atMs` unchanged (14:07 on 2026-11-22). No new ORDER line.

Not done: a real plugin component build through the driver. It was not needed for the decision above and would not have been conclusive on a loaded machine. The `driver-stages` module has no test of its own that runs the driver; the strict-mode coverage lives in `🎟️stage/🧪️tests`. If an ORDER line with a pid newer than 87111 shows up, that run's code path is the real remaining site.
