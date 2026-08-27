# Exact Peer Publication Freshness

## Verified Boundary

The coordinator read the actual native output and the production publication/retirement implementations. The complete retained Presence group now passes **12 tests**, zero failures, 884 filtered, 0.01 seconds of test execution after 16.90 seconds compilation. This is a scoped native ownership result, not CAD startup, a warning-denial build, browser behavior or an 8 ms benchmark.

Exact log: `🧪️member-presence-peer-commit-green-r2-native-2026-08-27.txt`.

| Candidate | Observed result |
| --- | --- |
| Foreign installed factory | Rejected; receiver unchanged |
| Same factory, different Store/base root | Rejected; receiver unchanged |
| Stale sibling candidate | Rejected; retained old root becomes reclaimable after rejection cleanup |
| Fresh candidate | Accepted; old root handed to retirement before releasing the captured base alias |

The private commit retains its exact base Arc, preventing address-reuse ambiguity while the candidate lives. Publication checks both this exact root and the installed factory before changing the receiver. Successful publication transfers the previous root into an explicit retirement owner before dropping the candidate's base alias. Rejected commits release their base alias through their own bounded retirement. Cancelled preparation also releases its base alias explicitly.

The first green attempt proved the four new cases but failed an older late-commit fixture: it waited for Store retirement to finish while retaining the rejected commit's base read. The corrected fixture advances both independent owners under one-item/4,096-byte grants. The final run confirms that closed Store and rejected commit retire their two snapshots. The production guards and grant were not weakened.

## Independent Coordinator Checks

- Source contract self-tests: **898 passed**, 33 exact factory owners, 255 custom rows, 25 generic rows; `📓️coordinator-tool-job-selftests-r14-2026-08-27.md`.
- Persistent numeric index: **12 semantic**, 37 lifecycle, two ordinal, 3,072 Immer/Map differential and five invalid-ID cases; strict TypeScript diagnostics zero; `📓️coordinator-numeric-index-test-r2-2026-08-27.md`.
- Shared retained renderer publication source was reviewed through its final close path. Exact owner/current-root checks and acknowledgement creation after the swap are present. Actual owned wire admission, immutable payload ownership, tree/hash/notification integration and mounted acknowledgement remain unfinished.

## Continuing Work

The publication executor retains the sole fleet Rust compiler lease for the replication contract, then the coherent plugin local-interaction cohort, then actual CAD constructor/close checks. Dag owns live query transport and restore integration. The renderer executor owns private wire decoding and the real React/wgpu publication path. The full source census and static audit remain open; neither these scoped tests nor source predicates establish all-app reactivity.

Peer HEAD checked read-only: `a8d1caf41f`, authored timestamp 2026-08-27T11:04:49+02:00. No git modification or cleanup was performed.
