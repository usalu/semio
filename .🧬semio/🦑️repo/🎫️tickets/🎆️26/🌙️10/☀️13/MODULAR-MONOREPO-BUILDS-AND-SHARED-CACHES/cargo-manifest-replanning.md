# Cargo Manifest Replanning

The real ordinary build failed in `withPreparedCargoDependencyPairV1` when the pack manifest's bytes changed during Cargo identity acquisition. Preparation hashes actual file contents; an identical rewrite alone does not invalidate this custody. The previous behavior rejected a valid changed manifest before invoking update or fetch, preventing agents from iterating on one live checkout.

A schema-owned policy permits at most four attempts, only for typed manifest or member-roster drift before dependency commands begin. The operation retains one original exclusive preparation lease, cancellation signal, deadline, and loaded implementation-source snapshot throughout. Each retry awaits the original progress publisher and continuation, then reselects the current workspace and preparation closure. It never replaces the deadline or silently restamps a changed loaded implementation. Drift after update begins, changed preparation outputs, invalid topology, and implementation changes remain refusals.

The portable corpus covers identical rewriting, one valid version edit, continuous edits, an edit after update, and actual deadline expiry. Ajv validates the policy and cases; SQLite independently records expected identity/dependency call counts. The successful cases run real Cargo identity, offline update, locked fetch, and locked metadata. Metadata confirms the version of the final current manifest. No compiler is invoked and no target directory is created.

Actual verification:

- The original implementation failed the changed-once law with one identity call, zero dependency calls, and refusal, instead of two identity calls and successful update/fetch. Continuous changes also refused after one attempt instead of the declared four.
- The corrected prototype gate passed five tests and 41 assertions in 1.145 seconds; Nx completed in 1.2 seconds. The changed-once case observed two identities and exactly two dependency commands, and the current 0.2.0 manifest. Continuous drift observed four identities, zero dependency commands, and retained the final peer bytes.
- Final release qualification passed six tests and 47 assertions in 4.32 seconds (Nx 4.4 seconds), including strict TypeScript validation of the defining preparation owner and both launch inventories. Final prototype qualification also passed six tests and 47 assertions in 1.87 seconds (Nx 1.9 seconds).
- The existing preparation/custody regression gate passed 44 tests and 1,444 assertions across five files in 16.46 seconds (Nx 16.5 seconds). It preserves recipe source/input/output refusal, dependency ordering, generated workspace membership, original cancellation, caller transport, and current physical relay custody.

This does not qualify the entire ordinary build or post-compilation source provenance. Replanning is deliberately restricted to the pre-dependency-command boundary. A continuously edited checkout can still exhaust the declared attempt count or its original deadline.
