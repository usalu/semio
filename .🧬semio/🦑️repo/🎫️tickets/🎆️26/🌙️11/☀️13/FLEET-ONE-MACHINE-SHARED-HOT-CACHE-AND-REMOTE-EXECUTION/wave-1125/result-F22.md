# F22 result

## Wave 1125 F22: partition operation back-pressure and four distinct refusals (m12 `Exact live repository partition cache owner required`)
- 17:27-17:50, area `caching-storage` (lock released). Scripts `🧰️scripts/F22/{patch,tests-patch}.py`. Root cause: one catch-all message hid four causes (copied/unissued handle, closed owner, failed owner, 32 pending); the log of m12 holds no earlier fault, so which one fired is not provable, but a failed owner (first failure masked) or a closed or copied handle fit; 32 pending would have hit the store's own `Bounded cache consumer admission exceeded` first.
- New `📥️opening/🚦️admission/{🟦️.ts,🧬️schema/🔣️.json,🧪️tests}`: `awaitPartitionSlotV1` (bounded 600 s, cancellable by the control, bilingual progress notice after 15 s), typed bilingual refusals `notIssued|closed|failed|closing|waitTimeout` (`texts{en,de}`, failed carries the first failure as `cause` and in the text).
- `📥️opening/🟦️.ts` `withRepositoryPartitionOperationV1`: back-pressure instead of a fatal at 32 top-level operations (nested operations bypass the cap, no parent/child deadlock), owner gets `roots`/`waiters`. Briefly unparsable at 17:27-17:3x (stray brace), fixed.
- Tests: admission 6/0 (m12 sequence of 40 submissions, nested, cancel, four refusals, Python text oracle). Updated bindings in headroom `🧪️tests` (22 pass, 5 fail unrelated), selected-state-recovery and scoped-retirement tests: their remaining failures are peers' unbound `refreshStaleImageBeforeOpenV1` / `requireRepositoryPartitionFitsHostV1`.

Not done: the same 32 bound exists in `nativeImageOperationAdmissionV1` (native-image borrower) and the `library/📦️packages` opening copy; unchanged.
