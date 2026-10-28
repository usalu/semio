# IFC Shared Continuation Independent Audit

No confirmed new P1/P2 source defect in the two-site integration. Reviewed `any/🚪️io/control.ts` ifcCheckpoint and `any/🧬️schema/🔺️diff/🟦️.ts` runIfcEditAsync, their neutral tests, IFC package declaration, the public async continuation entry and existing scheduler implementation. No production edits were made.

Both awaits use the real public `@semio-tech/framework-async/continuation` export, which reexports the existing singleton. The IFC package declares the first-party workspace dependency. No new scheduler, framework modification, timer workaround, batch change or timeout-policy bypass was introduced. Synchronous reference drains remain synchronous.

`ifcCheckpoint` still checks the original signal before callback, after callback and after host continuation. Callback failures propagate directly. `runIfcEditAsync` retains cancellation checks before generator work, after callback and before returning a completed value; after its await the next loop starts with cancellation before advancing again. Its finally still invokes generator.return, and runIfcIoSteps likewise closes an entered finite generator after cancellation/failure. Cancellation before first generator.next closes the unstarted generator without running its body/finally; the existing neutral test explicitly distinguishes that normal JavaScript behavior.

The existing native-host scheduler selects setImmediate, not a Promise-only microtask chain. Browser hosts select MessageChannel; hosts lacking both use the scheduler's existing timer fallback. This integration does not make a universal browser timer-fairness claim. Existing scheduler pending requests are unkeyed here, so unrelated IFC yields do not coalesce or overwrite each other's callbacks.

## Independent actual public endpoint probe

Authored `continuation-independent.ts`; output `🗑️generated/continuation-independent.json` records actual runtime versions and ten observations. It imports the mounted public IFC functions without copying source or substituting scheduler ports.

* Native timer cancellation reached both routes before exhaustion: IO two progress callbacks, edit twenty-four. Both entered generators ran finally and returned io.cancelled/cancelled respectively.
* Native immediate cancellation reached both routes: IO cancelled at its initial checkpoint before generator entry; edit cancelled after its first yielded item. No result was published.
* Each route preserved exact thrown undefined, number 23 and original Error identity, with entered generator finally executed: six of six.

Counts are observations, not fixed timing assertions. The probe process exited; no Bun mappings remain from this audit. It is actual Bun-host TypeScript execution, not Rust/native artifact execution or browser/DOM proof. Native compilation remains refused by authentic ingress; original funding and guided capability flags remain unchanged.

## Boundaries

The scheduler promise itself has no per-request AbortSignal; cancellation is observed when the genuine host turn resumes, as with the previous timer await. Disposing the process-wide scheduler while work is live would strand its awaited continuation under the existing scheduler contract; this audit found no IFC call that disposes it. Generator finally blocks that themselves throw can replace an earlier exception under ordinary JavaScript semantics; that behavior predates this two-await change and is outside the finite nonthrowing cleanup probes.