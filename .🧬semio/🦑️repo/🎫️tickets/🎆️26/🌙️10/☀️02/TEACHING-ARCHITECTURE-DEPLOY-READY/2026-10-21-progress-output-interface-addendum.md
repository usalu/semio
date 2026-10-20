# Progress Coalescing Interface Addendum

Observed 2026-10-20T23:28:55.438Z. Read-only comparison of `2026-10-21-progress-output-implementation-decisions.md` with current writer, route and existing tests. No Source or sealed input edit, command, test or live-handle operation occurred.

The pinned five-second shared frame, eleven slots, rank-compressed ordering, due-slot replacement, bounded original raw/human pairs and at-most-once sink attempt semantics fit the existing synchronous publish/flush interface. No callback/control/process API needs to change. Current `settle` already tracks a separate failed boolean and rethrows the exact value; the failure settlement branch attempts flush inside try/catch and rethrows its exact primary. Falsy-primary tests should protect that behavior, not introduce a second lifecycle implementation.

Two contract edges must be explicit in implementation and fixture expectations:

- Existing provider/cache/tooling callbacks retain their own current classifier, cadence, resource-change/reset and flush rules. “Immediate old-family rows” can mean rows that their existing scheduler actually emits; treating every such callback as immediate contradicts the decision to preserve those rules and would bypass their pacing. Only an actually emitted old row should force older new-family pending evidence ahead of that row. Switching to an old callback that remains pending must not itself flush or reset the new frame. A shared emit boundary can order actual emissions without replacing either scheduling contract.
- A family-only reset that drains just the affected family, emits its new row and retains unrelated older pending rows necessarily permits those unrelated observations to appear later than that newer reset record. The pinned rule therefore provides last-observation order within each drain, plus immediate-unknown/boundary ordering; it cannot also promise globally monotonic callback-order output. Preserve the explicit family-only reset choice and pin this exception in a fixture. No reset should silently drain unrelated families merely to impose a stronger unstated chronology.

The historical 250 ms proposal in my earlier design is superseded by the pinned 5000 ms rule. Unknown/malformed rows drain older pending evidence without granting a new frame window. Slot-first observations after the first frame row receive no extra immediate emission. Retained latest messages must be computed at observation time. Attempted rows leave their slot before raw write; untouched later slots remain eligible for a later flush if either raw or human write throws. Sink failure does not imply pair atomicity.

## Exact Permanent File Roster

All five are existing files; no new runtime module, producer patch, route, dependency or process/lease mutation is required.

1. `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧬️schema/📈️progress.json` — closed family/slot/counter/clock/reset/cadence/order policy and language-agnostic case definitions, authored first.
2. `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🔣️.json` — neutral expected raw-event indices/attempt sequences, interleaving/reset/clock/partial-write cases, authored before runtime.
3. `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🥒️.feature` — language-agnostic behavioral contract, including ordering/reset and at-most-once attempt limits.
4. `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts` — actual writer AST execution with existing TypeScript/SQLite/Ajv/local validator interfaces and actual Original control/settlement regression; retain existing titles and old-family expectations. If a private classifier is added in the same writer file, update every existing actual-AST extraction to include its exact declaration rather than accidentally testing an unresolved helper or duplicating its logic.
5. `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` — only the narrow typed presentation classifier/scheduler and ordered sink drain. Existing signal/issuance/check/yield/settlement ownership remains in place.

No producer/facade/CJS research file needs a test-time dynamic read for this feature. Source admission for the eventual selected test should follow its actual existing imports and these direct schema/corpus/implementation reads; do not add the full research/transitive closure solely because this addendum cites it.

## Exact New Focused Titles

Both belong to the existing progress test file above:

- `dependency work presentation follows bounded interleaved frames and independent SQLite scheduling`
- `dependency work actual Original preserves checkpoints cancellation and exact failure settlement`

The first covers all eleven slots, five-second threshold and due replacement, rank compression, unit/family and old-family interleaving, family-only reset ordering, unaffected counters, elapsed-zero/equal-count semantics, unknown barriers without frame reset, owner/command/stage boundaries, explicit clock states, bounded retained state, partial raw/human sink attempts, untouched-row retention, and idempotent successful flush. Its independent SQLite oracle must derive accepted/attempted row indices from actual row facts and schema policy, not invoke the production scheduler or trust fixture labels. Existing scheduling and locale tests remain selected through their established coverage.

The second uses the actual Original envelope and issued child controls, exact publication/check/yield counts and signal identity, pending final evidence across success/failure, cancellation refusal, and exact falsy primary preservation when final presentation throws. Test execution remains deferred until the current native outcome, fresh reread, independent review and Root seal/dispatch. These are proposed exact selections, not executed or passing tests.
