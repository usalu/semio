# Renderer Compiler Deadline

## Reproduced Cancellation

The sole renderer owners exited 1 after 3,602.874 and 3,603.588 seconds measured by the native artifact phase ledger. Both recorded `Cargo artifact build cancelled`. The second attempt spent 3,507.873 seconds inside the Cargo child. It compiled real downstream WGPU and plugin packages through 16:21 UTC. The capture showed neither a Rust diagnostic nor a source-drift refusal. The wrapper ran past 3,610 seconds and received the native owner's exit.

## Deadline Inheritance

The workspace script and its renderer child launch both request an explicit outer limit of zero. The repository compiler store issues its own bounded control with a 3,600,000 ms renewable idle window and a 21,600,000 ms maximum. Its issued process samples descendant CPU every 15 seconds. Other observable activity also renews the idle window. An explicit `SEMIO_BUILD_BUDGET_MS` further restricts the renderer's existing issued control at the entry point.

The Wasm renderer sampled `compilerAdmissionBudgetV1(control)` to fill the artifact policy's `budgetMs`. That helper returns the current smaller bound, which includes the renewable idle window. The artifact builder interpreted the resulting value as a newly issued maximum. The snapshot therefore introduced a separate cap of about an hour even if the original watchdog saw work continuously. The two phase ledgers time the cancellation to this inner artifact budget.

The existing artifact policy already supports `budgetMs: 0` with an issued caller control. In this mode it creates a child which follows the live deadline and cancellation of that control and adds zero further budget. This preserves the compiler store's idle and six-hour maximum and any explicit user budget applied by the renderer. The renderer owner is testing this change before the next managed build.

## Next Publication Evidence

Use the same owned compiler publisher and the managed lane. Turn on the existing progress-deadline trace for the retry. The expected trace has the repository preparation scope and ongoing child-CPU observations. Actual renderer publication and browser acceptance remain pending.

## Applied Deadline Fix

The Wasm renderer now passes `budgetMs: 0` alongside its existing issued control. The focused test reproduced the old value of 3,600,000 ms. All five Wasm owner tests passed after the fix, with 1,781 assertions, including the identity of the issued control passed to Cargo. Scoped whitespace checks passed. The single canonical renderer retry started as session 81602 with the existing progress-deadline trace and supported Rust offload scheduling enabled.

The new compiler declared a 60-minute idle window and a 360-minute maximum. Its trace recorded real descendant CPU activity during setup. The original workspace inputs advanced between attempts, selecting managed lane `p73dcdc`. This run has one renderer owner and the worker reuses admissible unit-cache artifacts as available.
