# SDK Headroom Recovery Independent Audit

Read-only Source review, 2026-10-15. No probes, tests, images, publication or implementation edits. Fixtures remain in progress elsewhere.

## Inspected Recovery Data Path

No concrete authority escalation or token-as-result defect established in the new DATA path. `tools/🧹️retention/📏️headroom/🟦️.ts:14-20` recognizes exact private WeakMap-issued objects, observes actual reservation/statfs under the original invocation, and privately binds root, original object, minimum bytes, actual free bytes and one-shot consumption. Public kind copies do not authenticate. A refusal is not a quote, reservation, maintenance store or writer registrar.

Publisher `retention/📥️physical/🟦️.ts:53` derives the minimum from the authenticated quote or existing unquoted bound, observes it before calling job/admitted clone, and returns refusal as successful data. Registered publisher tasks drain and registry EX releases before returning. No caught capacity Error is cleared from an accepted-operation error ledger to enable retry.

Bun ready selection returns without publication (`tools/🟦️bun/📜️script.ts:68-71`); Bun/pristine/Nx branches propagate the union explicitly. Pristine lines 44-46 return refusal before adopting a completed destination. Bootstrap line 404 returns refusal before assigning/activating tooling. Rustup callers without recovery convert refusal to fatal error via requireToolingPublicationValue, rather than casting the token to their nominal result.

Bootstrap `:374` wraps the entire tools-store attempt in withNxToolingHeadroomRecovery. That first attempt resolves only after canonical publisher, quote/projection and store/image callbacks have drained/closed. Lower recovery consumes the exact first refusal, performs one maintenance attempt, then reruns the same attempt exactly once with the same still-live Original. A second refusal becomes fatal; there is no recursive retry or deadline replacement. Replayed/foreign first refusal is rejected before recovery.

`prepareManagedToolingStorageV1` closes its observational image SH before optional maintenance. Actual maintenance uses image.withExclusiveMaintenance(wait:false), privately issued maintenance store/task scopes, and drains accepted work before EX release. An active holder prevents nonblocking EX; the same one retry still encounters actual physical admission and cannot adopt an unfunded writer. The lower API alone does not prove arbitrary supplied attempt closures have no ancestors, but the inspected canonical bootstrap wrapper closes its tools attempt before invoking recovery.

## Pending Ready-Leaf Integration

At review, bootstrap `:369` still eagerly called prepareNxToolingStorageV1 before selecting a ready generation. Under low free space this can request maintenance EX even when Bun/pristine/Nx are already ready. Nx explicitly identified and is reconciling this concurrent callsite to the authoritative-refusal path. Therefore ready leaves avoiding all unnecessary EX is not yet qualified in this checkpoint; it is not silently accepted as complete. Re-review the removed/reworked eager call before canonical runtime qualification.

No actual recovery/active-holder/replay fixture was run by this audit. Exact Source classification and lifetime inspection do not establish runtime success or available physical capacity.

## Final Recovery Source and Fixture Re-Review

The canonical eager preflight call is now absent from bootstrap.run; runInStorage still wraps the entire attempt. The Nx recovery callback now calls actual nonblocking maintenance directly and does not suppress its busy refusal. The lower whole recovery operation is itself registered withIssuedResourceControlV1 before awaiting attempts. These changes close the previously open ready-leaf/eager-EX integration point by Source inspection. A private outcome check also rejects a forged refusal-shaped object rather than returning it as T.

The seven scenario fixture at `headroom/🧪️tests/📜️script.ts` uses an actual 32 MiB image and 24 MiB private reservation, writes a real 4 MiB file, and derives required bytes from actual baseline statfs minus 1 MiB. It does not enlarge the image, change production floors or manufacture a success from an Error. Each attempt retains the same original object/signal through a narrowed issued scope; accepted failure remains in that scope even though Promise.allSettled observes it. Child-root failure is caught only outside receiveScriptProcessInvocation, so it cannot become a successful Original. Cleanup occurs through actual storage/image retirement and close.

The positive pressure scenario checks accepted child completion before SH close before maintenance. Actual EX is independently observed by Python SQLite failing a reader. Busy uses a real peer image SH and requires maintenance acquisition refusal. Ready has one attempt, zero recoveries and one effect; insufficient retries once then fails; foreign root and consumed replay fail; accepted throw-undefined drain failure prevents recovery/publication. These invoke production observer/recovery rather than a duplicated token state machine.

Specific remaining validation scope: the Python SQLite oracle directly proves EX blocks a reader, but the opposite SH-blocks-EX direction is asserted through actual provider acquisition rather than an independent SQL query. The scenario named foreign changes root while retaining the same Original; it does not independently exercise a token issued by a different live Original. There is no explicit expired-token or ordinary callback-throw scenario, nor a fixture specifically demonstrating ignored publisher refusal propagation. Those are test coverage gaps, not newly established production defects. The owner reports BOTH/strict outcomes; this audit did not run or independently verify them.

Historical-format dispatch is not yet authored at this checkpoint. Planned cleanup limited to explicitly typed immutable-binary-objects plus shared-pristine is appropriate to inspect next; material-absent artifacts remain protected. No retirement approval follows from this recovery fixture review.
