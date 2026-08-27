# Query Channel Integration Regression

## Later Evidence Correction

The coordinator subsequently confirmed that native setMergePolicy returns Done(seq); only its React mock returned no frames. The initial empty-completion attribution and preservation requirement below are historical and superseded by `📓️coordinator-cad-harness-and-query-receipts-2026-08-27.md`. The timeout itself remains valid failure evidence. The real receipt contract, multiplexing and stale-handle behavior are being checked before another full gate.

## Newly Executed Scoped Gates

The coordinator read the actual outputs:

- Plugin local interaction: **17 passed**, zero failed, 429 filtered, 0.09 seconds after 49.06 seconds compilation. The five live-owner laws cover exact pages/ACK, three retained roots, partial admission, reused query generation and partial-error emitted/retired accounting. Log: `🧪️member-local-interaction-r4-native-2026-08-27.txt`.
- Replication wire: **8 passed**, zero failed, 211 filtered, test duration rounded to 0.00 seconds after 1.91 seconds compilation. Includes exact output admission. Log: `🧪️member-local-interaction-wire-r3-native-2026-08-27.txt`.
- OS TypeScript canonical main test target: **213 passed in three files**, 250 ms tests / 1.35 seconds total. Its package inference exclusion now preserves the explicit task router. Log: `🧪️local-interaction-client-full-r1-2026-08-27.txt`.

None of these gates exercised the mixed ordinary-empty/query-outcome case below. Native output still includes warnings; no strict-warning pass is claimed.

## Independent Full Renderer Failure

R8 hit the unchanged 300,000 ms overall long-profile watchdog and exited 1 without a complete suite result. R9's attempted stricter per-test diagnostic did not execute: Vitest rejected duplicate timeout options. R10 used the unmodified 30-second quick profile with a verbose reporter. It printed successful decoder, retained preparation/publication and other renderer tests, then remained pending after the action round-trip test and before the existing merge-policy/conflict test. R10 also reached its unchanged process watchdog.

Logs: `📓️coordinator-renderer-react-full-r8-2026-08-27.md`, `📓️coordinator-renderer-r9-diagnostic-2026-08-27.md`, `📓️coordinator-renderer-r10-diagnostic-2026-08-27.md`.

The executor identified the actual regression in the new query outcome demultiplexer: skipping every outcome with no ordinary frames also skips legitimate empty responses to ordinary fire-and-forget commands. Their promises then never resolve. The test fixture is not being weakened.

An initial top-level-await import-cycle hypothesis was not established. Moving the new decoder oracle import into its test bodies removes an avoidable collection-time edge, but the remaining hang and exact empty-response branch identify the channel defect.

## Assigned Correction

New query commands will return an explicit existing `Done(seq)` receipt, separate from Started/page/Closed messages. The client will retain an internal query receipt waiter, preserve ordinary empty-response semantics, and prevent unsolicited, duplicate or wrong-sequence query messages from consuming ordinary waiters. Receipt and query reply capacity must both be admitted before publication flags change. Mixed ordinary/query lifecycle tests are required before the next full renderer gate.

This is a narrow branch/consumer correction, not a new TurnOutcome ABI or compatibility layer. The publication executor permits this non-API branch edit while CAD compiles its dependencies. The CAD close tests do not exercise query receipts; a fresh actual query-native gate remains required afterward.

The native CAD four-close cohort has finally started under the sole fleet compiler lease; its real constructor follows. There is no CAD result yet. Full restore, retained topology, tutorial/native-shell consumers, complete renderer migration, fresh Wasm and all-app proof remain open.
