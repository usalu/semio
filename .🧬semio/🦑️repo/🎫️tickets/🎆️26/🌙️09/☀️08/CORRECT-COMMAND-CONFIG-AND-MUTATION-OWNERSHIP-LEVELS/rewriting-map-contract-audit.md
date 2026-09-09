# Rewriting Map Contract Ownership

## Traced Ownership And Invariants

Rewriting's document diff contains plugin-local generic `apply_map_delta` and `merge_map_delta` helpers. Their underlying concepts are product-neutral mutation application and sequential composition; the appropriate shared owner is framework Replication's `🎮️mutation` contract. Rewriting should own only its parameter-binding and rule-layout fields and semantic mutation leaves.

The current `Option<BTreeMap<String, Option<PropertyValue>>>` representation has two independently testable problems. A present map entry with `Some(PropertyValue::Null)` encodes as the same JSON null as a removal `None`; decoding loses the distinction. The last-entry-wins map merger also replaces an insert followed by a remove with a removal against the original base. If that key was initially absent, both sequential edits succeed but the combined diff rejects the missing removal target.

These are source-derived findings pending the focused native regressions. New language-neutral cases cover setting null, inserting then removing an initially absent key, and removing an existing null value. The independent fast-json-patch oracle passed and emitted a diagnostic validating the expected maps. Two native tests exercise actual diff transport and the `MutationDiff::absorb` sequential law. Their first Nx run is queued.

## Required Shared Contract

Use an explicit set/remove entry vocabulary so a null value can never mean removal. Sequential composition must preserve successful application and missing-target rejection without reading a base snapshot. A plain per-key overwrite of the latest set/remove operation is insufficient. Retaining an ordered sequence per key is one viable representation; exact safe compaction must prove its preconditions before discarding intermediate edits. Audit this contract independently before selecting its final shape.

The shared contract needs native and TypeScript behavior, a language-neutral schema, transport round trips, exact retirement for owned generic values, and independent JSON Patch traces. Migrate Rewriting's parameter and layout maps together where their application/composition semantics are identical. Handcraft all affected fixture and schema representations; do not introduce a compatibility reader.

## Related Schema Corrections

Rewriting's non-native diff still invents an `artifact` field, accepts the wrong nullability for actual slots, and uses invalid Protobuf optional maps. Its generic PropertyValue declarations belong to the framework graph/value owner. The native graph PropertyValue currently exposes a JSON-equivalent projection backed by `DslValue`; verify the projection before referencing the shared dynamic-value schema. Bare camera helper declarations remaining in artifact schemas must be removed or referenced only by their actual window owners.

Permanent regression routes: `workspace:rewriting-map-ownership` and `workspace:rewriting-map-ownership-oracle`, with matching launch entries.

## Native Regression Receipt

`rewriting-map-ownership-red-2.log` completed compilation and ran both regression tests. Both failed as predicted: a null-valued binding decoded into a removal and raised `mutation.apply.missing-target`; insert-then-remove composition also raised `missing-target` on an absent base. The independent JSON Patch oracle passed before the native tests. Result: 0 passed, 2 failed, 94 filtered out; Nx duration 6m 8s. This establishes a native behavioral defect, and the shared contract design is under independent Terra audit.
