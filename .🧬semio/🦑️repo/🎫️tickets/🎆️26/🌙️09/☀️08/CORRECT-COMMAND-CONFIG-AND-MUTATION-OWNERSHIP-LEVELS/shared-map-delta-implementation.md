# Shared Map Delta Implementation

Replication's `🎮️mutation/🗂️map` now owns generic MapDelta, MapEntryDelta, MapEntryOperation and MapPresence contracts. Rust and TypeScript implement strict original-base preconditions, tagged set/remove/reject values, atomic map application, duplicate-key rejection, canonical key ordering, and base-free sequential composition. One compact entry remains per changed key. An empty entries list is deliberately the identity delta, enabling total composition without a separate identity sentinel.

JSON Schema is the shared wire contract. GraphQL and Protobuf share its operation and presence vocabulary with dynamic value payloads; the concrete consumer validates the payload's domain type. Rewriting references this owner for parameter bindings and layout changes, and its layout JSON Schema refines set payloads to LayoutPoint. Rewriting's local generic apply/merge helpers and duplicate diff value/camera declarations were removed. Four committed mutation fixtures and their native assertions now use explicit tagged operations. OS Store's retirement owner handles map deltas, entries and values incrementally through the existing RetireOwned interface.

## Validation

- The original Rewriting native regression failed both null transport and insert/remove composition before this implementation (`rewriting-map-ownership-red-2.log`).
- `shared-map-delta-oracle-1.log`: passed in 3.3 seconds. Ajv and fast-json-patch independently checked 12 neutral cases; exhaustive checks covered 648 single-key triple/base combinations with associative compact output. Duplicate keys and malformed operations are rejected.
- `rewriting-map-contract-green-2.log`: passed in 2.1 seconds. The Rewriting schema/parser oracle validated all seven committed native diff inputs and rejected a null LayoutPoint set payload, alongside document/snapshot checks.
- `shared-map-delta-native-1.log`: compilation failed on an incorrect pair-slice lookup in the new decoder. The lookup is corrected. `shared-map-delta-native-2.log` passed both native tests (12 neutral vectors plus 648 composition bases), with the exact runtime receipts; Nx duration was 5m 31s including the shared build lock.
- `rewriting-map-ownership-green-1.log`: the consumer failed to compile because its local `protocol` alias names OS Kernel, not the Replication crate. Rewriting now directly depends on Replication and imports its map contract through an explicit `replication` alias, without extending the OS facade.
- `rewriting-map-ownership-green-2.log`: both native map regressions passed with production diagnostics; nineteen of twenty-one committed-diff assertions passed. Two canonical JSON assertions rejected the newly handcrafted layout fixture's integer `24` against the native f64 `24.0`. The fixture now uses the native numeric representation.
- `rewriting-map-ownership-green-3.log`: passed both map regressions and all twenty-one committed-diff generation/canonical/application laws, with zero failures. Nx completed in 3m 27s.
- GraphQL/Protobuf compiler validation is not claimed.

The new shared targets are registered via the root script, Nx and both launch catalogs as `shared-map-delta` and `shared-map-delta-oracle`.
