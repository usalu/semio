# 🧪 T4: native-build driver test debt (2026-11-24 18:45-20:10, lane T4)

File: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts`. It runs as 3 suites.
- **Runs:** `repo test-file`, one at a time, 3 runs, load 45-88.
- **Other files run:** routing `🛑️owned-cancellation` 3/0 (new test) and storage `🛑️failed-birth` 1/0.
- **Locks:** `lock.tests-native-build` taken and released. No sub-agents. Nothing restarted.

P = `🧰️framework/🔨️modules/🏃️process/`

## Results

| suite | before (T3) | run 1 | run 2 | run 3 (final code) | cause |
|---|---|---|---|---|---|
| component-reclaim | 0/2 | **2/0** | 2/0 | **2/0** | **Stale test.** Commit 01324745f80 (11-22) deliberately made `CargoProvenanceSessionV1.reclaimOutput` keep witnessed (claimed) outputs. `reclaim.json` was updated then ("failed pass retains its claimed output"), but the driver test still expected `reclaimed [1-9]`. The test now asserts the hard failure plus `reclaimed 0 file(s)` for every pass. The product was right. |
| native-queued-cancellation | 2/5 | 3/4 | 4/3 | **5/1** | See below. Remaining red: `original finite caller budget` (message only). |
| native-artifacts | 49/24 | 1 real failure + 19 failures that are its cascade | no per-test output: the suite hit its Original deadline (1046 s) | no result: `Original Script deadline refused` (my budget raise, reverted, see below) | Not verified on the final code. See "Open". |

## Product fixes

### 1. Root (3): a cancelled request closed the whole process admission
**Cause.** In `P🧭️routing/📥️invocation/🏃️process/🟦️.ts`, `track()` closes every ancestor scope up to the birth for any rejection. The only exceptions were `ResourceLoanDeadlineExhaustedV1` and an `ownedCancellation` scope. `ownedCancellation` was set only by `withHandledScriptInvocationSignalV1`. So a control issued with its own `additionalSignal` (for example `scriptCompilerAdmissionControlV1(original, …, cancel.signal)`) that was cancelled put its reason into `birth.failures`. That produced "Original process admission closed - first failure: metadata cancellation" for every later request.

**Fix.** `controlFor` now gives every control with an `additionalSignal` the scope `ownedCancellation {signal: additionalSignal, parentSignal: incoming}`.
- Work that rejects with exactly that reason, while the parent still runs, closes neither the parent nor the Original.
- Other failures, AggregateErrors and parent cancellation still close the parent and the Original.
- Cancellation stays prompt: signals are unchanged, and the driver's new abort listener (see 3) kills children at once.
- What the request held is released by its own settle path.

**Regression tests.**
- New routing test: "a control's own cancellation stays with its request…". It runs the 6 shared fixture rows and passes 3/0. Before the fix, the "owned-stop" row ends with `parentLive=false`.
- The driver tests "unit graph compilation cancellation" and "queued exclusive cancellation…" (child process + successor run) pass.
- Storage `failed-birth` used to expect the leak (`failure toBe reason`). It now expects `undefined` and passes 1/0.

### 2. The body cutoff counts as a loan deadline
- `ResourceLoanDeadlineExhaustedV1` takes an optional message.
- The physical body cutoff now aborts with `new ResourceLoanDeadlineExhaustedV1("Original physical body cutoff reached")`.
- Reason: before this, reaching a finite caller budget closed the Original through the outer `withAcceptedPhysicalCleanupV1` track.

### 3. Driver (`P📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts`)
- **(a) Deadline and abort.** `ended()` classifies how a run ended:
  - a loan exhaustion, or an expired budget while the signal is not aborted, becomes `ResourceLoanDeadlineExhaustedV1("Original native build deadline exhausted")`;
  - a progress deadline and a caller cancellation keep their exact reason.
  
  An abort listener now stops the driver with that classified reason (removed in the final `finally`). The outer catch rethrows `stopReason` when the rejection is the abort reason or a loan exhaustion.
- **(b) Job wait.** When the driver was aborted, it now rethrows `stopReason` instead of wrapping it in an AggregateError ("Native build job failed (remaining jobs cancelled)"). The wrapper had turned the cancellation into a process-closing failure (run 1, queued test).
- **(c) Withheld links.** A failed Cargo exit in a withheld-link pass (warm, single package) is now named `cargo exit N`. Components that Cargo linked but whose links were withheld are no longer reported as "not linked".
- **(d) Stall sample.** When a stall sample runs past its own 5 s budget (`sample pid 3` under load), it is recorded as unavailable. It no longer replaces the stall reason (run 1: "Original resource loan deadline exhausted" instead of `/stalled/`).

## Test fixes (against the current contracts, no schema loosened)
- **Root (1):** `📋️prerequisites` (`^job:`) is the contract for job prerequisites since 10-16.
  - "jobs start…" now uses `after:["job:tool"]`.
  - The unlinked-component test now asserts the stages: only comp-a's `check` and `publish` ran.
  - "a job failing at once" now also asserts that a job after `job:broken` never starts.
- **Root (2):** controls are now issued from the Original.
  - The real wasm32 build uses `scriptCompilerAdmissionControlV1(original, …)`. It is now isolated, so a failure no longer cascades.
  - The queued-exclusive child and the unlaned job script use `receiveScriptProcessInvocation` and an issued control. The unlaned lease signal is now `control.signal`.
  - The unlaned job script had called `testPythonV1()` inside the child, where it is undefined. It is now interpolated.
  - Pending stages use `cancel` (an issued control). The structural-port fields `fractionalRemaining`, `expectedPublished` and `expectedContinuations` are dropped: the driver requires issued controls, so a fake port cannot be injected.
- **Fixture budgets:** the physical cleanup reserve is 10 000 ms, so a 6 000 ms budget was refused before any effect.
  - `finiteCallerBudget` and `originalStageAuthority`: 6 000 → 18 000 ms.
  - `unlanedStateOrder`: 15 000 → 45 000 ms.
  - The timeouts were raised to match.
- **Capacity marker:** since 10-19, environment markers cannot lend CPUs (`acquireCompilerCapacityLeaseV1` refuses `SEMIO_COMPILER_CAPACITY_HELD=true`), and the driver strips the marker. The reserved and exclusive tests now expect it to be undefined.
- **Missing source witness:** this row declares its own refusal ("no matching retained source witness"). It used to share the "inconsistent source versions" pattern.
- **"cancellation stops every Cargo tree":** it used to declare a primary rejection, which encoded the leak. It now expects no rejection and checks that a successor build is admitted.

## Open, with proven or traced causes
1. **`original finite caller budget`** fails on the message only: it receives "Original resource loan deadline exhausted" (run 3).
   - Cause: the loan exhaustion comes from a `remaining()` check outside the driver's stop path. That is the law-storage owner around the driver, so the driver's label is never applied.
   - Admission is no longer closed by it: the suite has no cascade.
   - Next: find the throwing owner with a `[DEBUG]` stack in `withCreatedNativeImageV1`, or label the expectation as a loan exhaustion.
2. **`a real build over wasm32-wasip2`** (run 1): `ConcurrentEditErrorV1 "Current physical absent input appeared before transfer: <generation>/w/<sha>/component.wasm"` from `📁️current recheckStructure`.
   - A product provenance/staging interplay: the stage file of one lane appears under a roster that another lane observed as absent. It was hidden before by the issuance error.
   - Not fixed, because it is outside this brief.
3. **native-artifacts on the final code is unverified.**
   - Run 2: the suite ran into its Original deadline (300 000 ms, stretched to 1046 s), because all ~70 tests now really run instead of cascading in milliseconds. It printed no per-test lines.
   - Run 3: I had raised `budgetMs` to 600 000. The parent then refused the stretched envelope ("Original Script deadline refused"). I reverted it (sha256 back to 2e80cefd…).
   - Next: the process suite table needs a budget or file split for native-artifacts that the test-file envelope admits.
4. **Broader effect of fix 1.** Tests elsewhere that relied on a cancelled control closing the Original will now see the Original stay open. I grep-checked them and fixed the one that asserted it (`failed-birth`). The archive and plan-control tests only gain from it.

Logs `🗑️generated/T4/` were deleted after this report.
