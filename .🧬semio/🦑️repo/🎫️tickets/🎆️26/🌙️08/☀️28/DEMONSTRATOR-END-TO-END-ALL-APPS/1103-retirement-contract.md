# Flow Retirement Refusal Contract

Status: focused native regressions authored, not executed. Runtime lane currently owns the Cargo window. No production lifecycle changes made.

## Concrete Authority Contract

- `OriginalStoreGenesisFactory<P,M>` constructs a concrete initialized ArtifactStore and its original envelope; Flow supplies the canonical factory `FLOW_HISTORY_STORE_FACTORY`.
- The Store's installed `ArtifactStoreOwnedDisposer::close_step` returns `Result<RetainedCloneStep, ValueError>` independently of grant validation. `ArtifactStore::close_owned_store_step` takes the exact original disposer, calls its fallible close, and reinstalls that disposer whenever the Store is nonterminal, including `Err`. This preserves custody for a later turn.
- `FlowHostRetirement::close_step` similarly keeps its owned host on propagated child refusal because its receiver is borrowed. The cold `FlowHost::retire_cold(self)` instead owns a local retirement and applies `?`; its nonterminal Drop asserts when not already unwinding. Thus an advertised normal Err path appears to panic before reaching the caller. This remains source evidence until the regression executes.
- Deliberately insufficient grants, missing installed authorities, incorrect terminal witnesses, integer depth overflow, and closing a host while external read custody is still live are separate programmer/precondition cases. They are not the injected recoverable case below.

## Meaningful Fault Seam

The test factory delegates the actual canonical Flow constructor, then decorates its exact original installed disposer. The decorator retains the genuine original Box and all Store/factory/snapshot owners. It injects one deterministic `AllocationFailed` close refusal only after the actual complete quoted grant is admitted. The Store and host receive unchanged fully quoted grants; no depth/currency is lowered, no invalid receiver, arbitrary global recipient, fake caller ID, cancellation recipient, or capacity increase is introduced.

The decorator delegates demand and original closure, adds its own logical owner depth, and separately quotes/releases the original disposer allocation before its wrapper becomes terminal. This avoids hiding a nested physical release in the final wrapper drop. Constructor setup is inside the existing cold history factory seam. This is deterministic fault injection into a supported fallible authority, not a claim that a naturally occurring allocation failure was reproduced.

## Regressions and Neutral Fixture

Mounted from the existing Flow host unit-test root:

- `tests::retained_host_close_preserves_custody_after_a_real_history_authority_refusal`: drive exact quotes, observe one actual AllocationFailed refusal while the host remains nonterminal, then retry the same retained owner to terminal. Assert every returned success/error receipt fits its original grant.
- `tests::cold_host_close_returns_a_real_history_authority_refusal_without_panicking`: use the same initialized detached host and authority fault, then require the cold fallible API to return Err. Current source predicts a red nonterminal-retirement-Drop panic; the test adds no panic catcher or suppression.

The sibling `contract.json` is language-neutral and parsed through existing third-party serde_json. It defines refusal kind/count, turn bound, and terminal/refusal outcomes. Serde validates fixture decoding; it is not an external implementation of the repository-specific ownership protocol. The independent runtime comparison is the Store/retained ownership contract versus the cold wrapper under the same original authority and grants.

## Per-Window Ownership Scope

Current Flow tick begins its latch, mutates evaluation state and removes pending extension requests before cold retirement; returning an error there could prevent request publication and outcome recording. The pending-effects trait's Vec return surfaces the current conversion through Notify; a persistent actual recoverable error could repeat on polls. These are conditional source-control-flow risks, not yet demonstrated runtime failures. Shared-session cancellation would also affect sibling windows and is explicitly excluded. No per-window lifecycle production change is proposed before the focused host refusal result.

## Planned Exact Native Handoff

After root/runtime hand back the one global Cargo window, execute the two exact tests separately through the verified repository-root owner using durable helper labels. Retained and cold results must remain separate: a retained green plus cold red would establish the minimal cold-custody fix requirement. No Cargo/test command has run in this follow-up yet.

## Owned Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` (one include)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🏁️cold-retirement/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🏁️cold-retirement/contract.json`
- This report.

## Actual Native Execution Window

2026-11-03T23:29:05.929Z: retirement-red entered proper repository-root workspace, package semio-framework-os-flow-host --lib with filter history_authority_refusal. Global Cargo window explicitly acquired from root and communicated to runtime. Two-job compiler admitted; terminal result pending. Both tests remain tests-only; production retirement unchanged. Durable result/logs under ticket 🗑️generated/1103-retirement-red.*.

retirement-red terminal 2026-11-03T23:32:56.839Z: compilation failed before test execution on seven authored protocol::os_store/os_vcs namespace references. These APIs belong to the already declared kernel facade. Test-only references corrected to semio_framework_os_kernel; no production/manifests changed. retirement-red2 warm rerun started23:33:31.958Z; actual behavior remains pending.

## Actual Fault-Path Result

retirement-red2 finished2026-11-03T23:34:08.496Z, compiler20.64s GREEN. Executed exactly2tests: retained_host_close_preserves_custody_after_a_real_history_authority_refusal PASS; cold_host_close_returns_a_real_history_authority_refusal_without_panicking FAIL. Overall1passed1failed130filtered, runtime0.06s, outer exit1 (Cargo101). Cold failure is the production host-close Drop assertion at line74: Flow host retirement abandoned its original source.

The exact original canonical initialized Store and original installed disposer were retained by the test decorator; the one injected AllocationFailed was issued only after its real quoted currencies/depth were admitted. The retained test received that original error, checked its receipt against the supplied grant, retained the same owner, then drove it to actual terminal emptiness with one refusal. No fake grants, aliases, caller identities, global recipients, cancellation, panic catcher or production seam were introduced. This is concrete supported recoverable authority failure, rather than a fabricated programmer invariant.

The cold test calls the unchanged production wrapper directly. The recoverable original error reaches a question-mark return while its local retirement remains nonterminal; Drop panics and prevents the declared Result from returning. The current fail-closed ManuallyDrop assertion does not prove physical terminal release on failure; this test does not claim heap-traffic instrumentation. The same-authority retained test proves resumable custody and terminal completion.

## Minimal Owner-Preserving Proposal (Not Implemented)

The repair belongs to the cold driver return boundary, not Drop, Store authority, or an error-string conversion. A consuming fallible driver cannot promise Result<(),ValueError> while its refused nonterminal original must survive. Return a typed refusal containing both the original ValueError and exact FlowHostRetirement, or expose the uninterrupted driver on an explicitly borrowed retirement owner. The consuming convenience API must then require an actual retained failure recipient or return that custody; preserving only ValueError is insufficient. This is a source-contract change, with no backward compatibility requirement.

For a typed refusal, caller propagation must retain the exact returned retirement under the caller/window owner before surfacing a framework Fault; simply mapping the error and dropping the refusal reproduces the defect. Existing retained-session authority is the correct recipient where available. Per-window publication/outcome ownership must remain intact if retirement refusal occurs after evaluation work has been taken. Do not introduce global queues, replacement caller IDs, retry loops that erase the primary error, shared-session cancellation, extra budgets, leak/panic suppression or empty-success fallbacks. Root coordinates the specific production API and necessary caller scope before implementation.

Global Cargo window explicitly handed back at terminal. No production retirement changes made. Canonical seal normal contract remains separately verified3laws0.
