# App Command Admission Gaps

## Actual Native Failure

The new encoded transaction test does not reach transaction dispatch. It receives Error with the exact original sequence and code `plugin.command-route-state-machine-required`. The coordinator read the diagnostic output (`🧪️member-transaction-receipts-r2-native-2026-08-27.txt`): zero passed, one failed, 447 filtered, 0.07 seconds after 10.97 seconds compilation. Earlier 18 live-query tests remain separate passing evidence. Adding companion Done frames to success branches cannot repair a decoder that never admits those commands.

## Complete Source-Mapped Gap

The coordinator read the real PagedAppCommandDecodeCursor header and checked every numeric tag against the TypeScript codec table. The ordinary decoder admits tags 0–4, 6–9, 15, 16, 27 and 29. Presence tag 28 deliberately uses a separate reserve-before-decode authority and is not a missing generic route.

| Missing tags | Commands |
| --- | --- |
| 5 | ApplyEnvelopes |
| 10, 11, 12 | MediaIn, MediaOut, MediaFingerprint |
| 13 | PureCommand |
| 14 | LoadChildren |
| 17, 18, 19 | TransactionPrepare, TransactionCommit, TransactionRollback |
| 20, 21 | TransactionUndo, TransactionRedo |
| 22, 23, 24 | OpenArtifact, SetDefaultApp, ClearDefaultApp |
| 25, 26 | SetMergePolicy, ResolveConflict |

These sixteen tags are a wire-admission ledger, not sixteen runtime results and not interchangeable with the plugin command census. Only the transaction Prepare rejection above was executed in this diagnostic; the other mappings are source findings.

## Implementation and Verification Decision

The query executor owns the remaining route ledger and bounded decoder cohorts after the coherent client receipt checkpoint. Each route needs exact field admission, cancellation/retirement for partial fields, malformed/truncated wire tests, and bounded downstream dispatch. A blanket whitelist or whole-command decoder fallback is forbidden. Short scalar routes can be a separate cohort from nested prepared operations, media or child envelopes.

The three transaction success-branch receipt laws may be tested directly using an explicitly typed cold owner, but must be named and reported as cold branch coverage. Actual encoded denial remains a distinct test/open gate until the owned route exists. No production bypass and no registered interactive transaction completion claim follows from cold tests. Native ABI frame tags are unchanged by the three companion Done replies.

The publication executor has resumed the real CAD four-close build while this packet proceeds. All-app functionality remains open.
