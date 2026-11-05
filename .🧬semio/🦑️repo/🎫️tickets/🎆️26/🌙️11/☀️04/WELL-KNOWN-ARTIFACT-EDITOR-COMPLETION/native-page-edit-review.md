# Native Page and Edit Source Review

No compiler or native job was launched for this review. Production remains frozen for the shared native driver.

## Pending Page Case 4

The accepted JSON cancellation case uses one genuine caller and decoder, preserves decoder identity across cancellation, writes through the actual retirement encoding receiver, then pays traversal, page handoff and physical page closure. The lower bridge forwards the original observer/allocation port and reconciles owned-byte deltas on normal return, refusal and unwind.

The pre-cancellation successful-write helper still selects a refusal with primary.or(forwarding), which can discard another owning refusal or context diagnostic on an unexpected failure. This does not create a successful result, but a complete diagnostic check should avoid selective ownership loss. Root owns the test and was notified.

The new mounted-ui-copy cancellation test was corrected to call caller.cancel().cancel_now(); cancel() alone returns the token. This was a test-only correction before native snapshot; no execution pass is claimed.

## Preparation Error Allocation

| Owner | Actual path | Unreported allocation |
|---|---|---|
| Mesh preparation | absent or ambiguous target, line 44 | ValueError::new with formatted mesh/primitive match counts |
| Brep preparation | absent or ambiguous target, line 42 | ValueError::new with formatted vertex match count |
| ZIP preparation | post-validation failure, line 189 | formatted owning validation diagnostic |
| DOCX preparation | invalid/stale XML and measurement failures | dynamic owning diagnostics |

Mesh/Brep absent-target native cases explicitly reach the first two paths and assert work 1. Their current retained progress is zero, so the persistent fixture's exact heap-versus-original-receipt comparison should expose the diagnostic allocation. A static ValueError::literal invariant message avoids this allocation without fabricating a physical receipt. Existing tests currently pin formatted target-count suffixes and must change with the production diagnostic. Root owns those production methods and received the precise locations.

ZIP and DOCX current valid/denied cases are not expected to reach their dynamic error paths; no broader physical correctness is claimed. All semantic conclusions await actual native checks.
