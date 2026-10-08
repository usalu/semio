# Kernel Test Wrapper Repair

## Observed Failure And Neutral Red

The complete Store selection failed before executing tests: the package wrapper emitted raw `cargo test --lib long --lib os_store::component::tests -- --nocapture`. The wrapper bypassed the existing bounded Nextest driver, duplicated caller target selection, and treated a test level as a Cargo filter.

The new language-neutral three-case corpus covers explicit library selection with the long policy, explicit integration selection with the quick policy, and default selection of all targets. The independent TypeScript AST oracle extracts and executes the actual package class; each accepted route then compiles and runs a tiny native Cargo package. Before production repair the actual registered Nx gate exited 1 with `Kernel runner bypasses exact bounded Cargo policy`. Existing execution-driver cases still reached their real native assertions. The expanded suite also reached its previous 15-second outer deadline; it now uses the existing quick-suite budget because it contains seven native assertions and multiple cold Cargo builds. Individual test deadlines, selections, native policy budgets and refusal checks remain enforced.

## Production Repair

The kernel package resolves the declared CLI test level, preserves all remaining explicit target/filter/capture arguments, and delegates its exact package and manifest to `runCargoTestsV1` and `readCargoTestPolicyV1`. It does not insert an additional library target. The existing registered process driver target covers the new corpus; no additional executable or launch route is introduced.

## Exact Authored Files

- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🏃️kernel-test-runner/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🧪️tests/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📜️script.ts`

## Verification

Actual neutral red: session 17048, exit 1, retained log `🗑️generated/kernel-driver-neutral-red.log`. Post-repair registered uncached driver run: session 88124, log `🗑️generated/kernel-driver-neutral-green.log`, result pending. The complete 371-case Store rerun is assigned to the editor agent. Neither that whole Store result nor the final release/browser gate is claimed green here.

## Completed Expanded Driver Proof

Actual uncached Nx exit0: five source tests passed, zero failed,206 assertions. All three actual package routes ran native Cargo binaries: long library, quick integration, and fundamental all-targets. Those routes executed four native assertions; existing capture control cases independently executed three native assertions. Empty-selection refusal remains asserted.

```text
[DEBUG] kernel driver actual owned library selection
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Summary [   0.017s] 1 test run: 1 passed, 0 skipped
[DEBUG] kernel package neutral selection=store-long-explicit-library policy=long actual-native=true
     Summary [   0.017s] 1 test run: 1 passed, 0 skipped
[DEBUG] kernel package neutral selection=integration-quick-preserves-target policy=quick actual-native=true
     Summary [   0.016s] 2 tests run: 2 passed, 0 skipped
[DEBUG] kernel package neutral selection=default-retains-complete-target-selection policy=fundamental actual-native=true
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Summary [   0.018s] 1 test run: 1 passed, 0 skipped
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Summary [   0.016s] 1 test run: 1 passed, 0 skipped
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Summary [   0.015s] 1 test run: 1 passed, 0 skipped
     Summary [   0.001s] 0 tests run: 0 passed, 1 skipped
 5 pass
 0 fail
 206 expect() calls
 NX   Successfully ran target test-cargo-driver for project @semio-tech/framework-process
```
