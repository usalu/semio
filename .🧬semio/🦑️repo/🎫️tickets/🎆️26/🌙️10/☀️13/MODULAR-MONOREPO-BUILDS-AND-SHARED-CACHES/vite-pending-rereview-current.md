# Generic Vite Pending-Work Source Rereview

This is an independent read-only review of current generic `library/⚡️caching/🌐️vite/🟦️.ts` and its input helper after the actual-phase bridge wiring. No physical execution or generic service implementation edit was made.

## Concrete Remaining Concern

The `admitted` transform/warmup callback waits for all current optimizer `metadata.discovered` dependency `processing` promises in `finally`, but discards rejected `Promise.allSettled` rows (current line139). If the transform itself succeeds and a discovered optimization fails, this callback can release its compiler capacity lease and resolve successfully. The later shutdown joins `waitForRequestsIdle()` and `scanProcessing`, which do not visibly preserve the exact settled discovered rejection discarded at this boundary. The Source therefore does not establish failure propagation for that case. Root was notified; this report makes no runtime failure reproduction claim.

The outer pending set otherwise visibly registers accepted task promises, closes births on task failure/shutdown, joins the set before server/input/store release, and retains errors including an actual `undefined` through its failure array. The input helper obtains the actual carried phase from the authenticated store helper, nests exact readers and their accepted tasks, and retains all input scopes through the compiler/listener callback. No old-phase mismatch remains visible in that helper.

Other explicitly retained frontiers remain: build/in-process serve still reread mutable option DATA after awaits; the CLI owner captured its root/config/listener/env/input data already. Initial store opening precedes the phase bridge as documented in `native-failed-open-boundary-current.md`. This Source rereview does not claim actual Vite listener/compiler qualification, full store opening phase cancellation, or retirement/cleanup-reserve coverage.
