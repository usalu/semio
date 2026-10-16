# Current Vite Artifact Input Source Rereview

Source-only review of Root's generic Vite implementation and input helper after the actual store/reader phase bridge landed. No Vite compiler/listener, cache image or cleanup operation was launched for this review. Working Source remains coedited; this is an observation, not a final frozen SDK qualification.

## Earlier Findings Reconciled

The former zero-argument artifact helper mismatch is closed: Storage's helper now synchronously authenticates known store custody and supplies its genuinely selected carried phase to the input helper. Passing the original options control instead would now risk dropping the narrower actual frame restriction; that old suggested workaround is obsolete.

The former unconditional-close omission is closed on the inspected ordinary failure path: serve finally gathers environment drain failures with allSettled and still reaches listener/server close and cache cleanup. Body failures remain in the failure array rather than being discarded by cleanup. Root's Original mode/workspace check is now present in Source; it is no longer merely a pending change. The new command-service owner is also present. These are Source observations, not runtime compiler/listener passes.

## P1: Accepted Transform Wrappers Are Not Drained Before Cache Disposal

`serveVite`'s `admitted` wrapper (approximately lines139–143 in the inspected version) accepts a real requested Core operation, then waits for compiler capacity before it calls the Vite transform/warmup function. Such a wrapper can therefore exist outside Vite's internal request/optimizer tracking while it waits.

Finally currently waits for `environment.waitForRequestsIdle()` and scan processing, closes listener/server, and removes the temporary cache. It owns no exact set of these accepted wrapper promises and closes no explicit wrapper admission flag before that sequence. A pending capacity waiter need not be visible to waitForRequestsIdle. On a non-cancellation failure, such as ready throwing while the phase remains live, transform/warmup wrappers can also accept new work during the initial drain.

Concrete consequence: cache disposal and server close can precede settlement of a previously accepted wrapper that has not yet entered Vite. That wrapper can later acquire capacity and invoke its captured operation while the service owner is unwinding. The inspected Source does not establish that every such late operation is refused by Vite itself before any cache effect.

This is NOT a claim that input SH already releases early. Canonical pending work can retain the enclosing reader until later settlement. The actionable gap is the earlier cache/server disposal boundary and lack of owner-level births/drain, rather than a proven stale-input publication.

Minimal truthful correction: a private service request owner synchronously retains every exact admitted wrapper promise before deferred effects; closes wrapper births and cached-transform admission at shutdown entry; joins all accepted capacity waits, operations and optimizer descendants while actual parent controls remain live; then unconditionally settles server/listener/cache cleanup and propagates all actual failures. No control copies, fresh roots or global registrar are required. A meaningful hostile law should hold real compiler capacity, accept a wrapper, trigger non-cancellation shutdown, show disposal has not begun, then release capacity and prove exact operation and cleanup ordering. Preserve Error/undefined and final actual Core failure semantics.

## P2: New Command Service State Identity Always Includes A Unique Temp Path

`serveViteCommandV1` merges its environment, pins TMPDIR/TMP/TEMP to the freshly allocated `temporary`, and hashes all resulting entries into `variables`. `withViteStorageV1` creates that path using mkdtemp. The variables digest enters compiler configuration; the actual compiler-state owner hashes configuration into its state directory identity.

Thus identical owner/config/semantic environment still selects a different persistent optimizer-state identity on each invocation. This is a concrete Source-derived warm-reuse cost, not measured runtime timing. It may remain semantically necessary if config deliberately consumes the temp pathname. Do not simply exclude these fields: any later optimization needs a truthful semantic/runtime environment boundary or a proved stable owned temporary identity, with existing bytes/guards retained.

## Positive Actual Helper Evidence

The final store/reader proof invoked the ACTUAL Vite input helper with eight distinct finite publications inside one real32MiB diagnostic image. It checked real carried-control refinement, used actual Node to read all eight values, and independently used Python SQLite EX attempts to prove all eight producer SH leases held during the body and released afterward. The generic reader's registration port was already closed before the late Node effect, demonstrating actual callback return. Both modes passed 5 laws/92 assertions, with strict both modes exit0; details and exact handles are in cache-store-consumer-lifetime-current.md.

The list is copied and closed-schema validated, canonical/absolute, path-distinct and sorted. Recursion carries the actual deeper artifact control and retains each reader through the actual body. The global store active bound is cumulative32; the eight-input proof qualifies this nesting and does not assert an additional32 reader tasks are available.

## Review Limits

No real Vite compiler, HTTP listener, optimizer shutdown or final child cancellation has been qualified by this review. No complete caller-adoption census was repeated. Exact input/output equality is refused by build; ancestor/descendant overlap is not comprehensively forbidden, and no overlapping real production caller was established here. Raw configuration/thirdparty incidental-write behavior is not newly proved by these input guards. Frozen SDK/Source boundary gates and ordinary Nx remain held.

## Observed Source Identity

Observed UTC: 2026-10-15T19:35:30.176429+00:00 . Hashes are bounded working-file observations and can change under authorized coedits.

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts`: SHA256 `154de7f9f052b61ce11e4246e8bb806181dafbc2a1e83375c9c85b3d298ecab3`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/📥️inputs/🟦️.ts`: SHA256 `ecab805d4fd54b4115ffe601756682cea35e8e870fa33e8f303135e93928c46b`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/💾️storage/🟦️.ts`: SHA256 `87cf0de2f3316e546d9400f56e1f6428fd1a156292470a003d418e2425d69339`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/👓️reading/🟦️.ts`: SHA256 `b9cd09489705cbc52ef3f1c15ba903c1b8192bb2ba02673c29b45b80359148c7`.
