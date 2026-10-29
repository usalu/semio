# 1029 result flowtests
Status: GREEN. flow-document lib (4/4), neural-engine-model lib (18/18) and dsl-record `ordered_map_field` pass natively; `check --target wasm32-wasip2` rc 0 for flow-document, neural-engine-model (root workspace) and dsl-record (framework workspace).

## flow-document (3 failures)
- Root causes: (1) tests dropped decoded `FlowHostSnapshot` without `retire_cold()`; (2) fixture `expanded` was unsorted but `OrderedSet` is key-sorted by design; (3) fixture omitted `"tree"` on cluster neurons while the neural wire (`Neuron::to_value` / `encode_neuron`, also cluster-tree fixture) always emits `"tree": null`.
- Fix: `FlowHostSnapshot::retire_cold` in all three tests (incl. the re-parsed temporary); fixture `expanded` sorted to `["n","text"]`; fixture neurons get `"tree": null`; `schema.json` neuron.tree is `oneOf [tree, null]`. The TS schema oracle (`🧪️tests/🧬️dsl-native-snapshot/🟦️.test.ts`) still passes (5/5).

## neural engine-model (3 failures) - one real bug, two stale expectations
- `evaluation_snapshot_and_channel_owners...` (464 vs 72): REAL accounting bug. `RetireOwned for TreeSnapshot::retirement_birth_bytes` quoted the children's final cursor frames (`shared_retirement_birth_bytes`, 464) while `retirement()` allocates only the sequence + two `Deferred` frames (72). Fixed to quote `deferred_birth_bytes::<SnapshotTreeLease/SnapshotSeedLease>()` (`🧵️retirement/🦀️.rs`).
- `recursive_dictionary...` (11 vs 15): STALE. Value-sweep law (`♻️retirement/📦️bytes` test, "native metadata retirement invented payload copy") makes `String` payload retirement copy-0 (truncate -> `Advanced`); only ordered key bytes are copy work. Expectation is now keys only (`"nestedvalue".len()`); fixture label `logical` = `utf8OrderedKeys`.
- `finite_schema_operator_fields...` (blocked turn 63): STALE. `Box<ValueType>` (FieldSpec.value `list`) is an atomic whole-value move: `BoxedOwner::next_demand` quotes `copy_bytes = 2*size_of::<T>()` (48) and cannot be split, so a fixed 1/3-byte copy grant is below the quote. The test now grants `max(fixture grant, selected quote)` (divisible work keeps the narrow grants) and the expected copy total is derived independently of Rust: `OrderedKeyBytes + BoxedValueTypes*2*size_of::<ValueType>()` (fixture `expected.{schema,operator}{OrderedKeyBytes,BoxedValueTypes}`, cross-checked by the TS source-contract test with its own key/box counting). Old `*StringBytes` totals stay (TS oracle).
- Also: `dictionary_projection_identity...` (new peer test) dropped a `Value::Dictionary` clone unretired -> `wrapped.retire_cold()`.

## dsl-record `ordered_map_field`
`native_encoding::project_entries` (`🗣️dsl/🧬️schema/🛫️encode/🦀️.rs`) called `begin_stage` (a cancellation checkpoint) even for an empty map. An empty map owns nothing: now returns `FieldValue::Map(vec![])` before staging. Non-empty maps still cancel (test asserts both).

## Files changed
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧪️tests/🧬️dsl-native-snapshot/🦀️.rs`
- `.../🌊️flow/🗿️artifacts/🌊️flow/🧫️fixtures/🧬️dsl-native-snapshot/{🔣️.json,schema.json}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs`
- `.../🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧵️retirement/🦀️.rs`
- `.../🧵️retirement/🧪️tests/🧫️fixtures/🧬️finite-fields/🔣️.json`, `.../🎮️typed-owners/🔣️.json`, `.../🧪️source-contract/🟦️.ts`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛫️encode/🦀️.rs`

## SHARED CHANGE
`dsl-record` `project_entries` empty-map early return (all map owners); neural `TreeSnapshot` birth quote.
