# 1029 result neural

STATUS: GREEN for A, B and C. Native `--lib --tests` checks, the new law tests and the wasm32-wasip2 check pass. One foreign red test is reported at the end.

## A. Dictionary projection identity (REVIEW demobundle/fwwasm)
Cause: the blanket `impl<T:DslField> FieldProjectionSource for T` (dsl-record `🛫️encode`) did not forward `projection_identity`, so every `DslField` source used the object address and lost the stable original-root identity that the deleted explicit `OriginalDomainProjection for Dictionary` provided. `RetainedFieldProjection` compares this identity across steps while the source header moves.

Fix (no shim, one trait hook):
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🪆️binding/🦀️.rs`: `DslField::projection_identity(&self)->usize` added with the address default. `Box<T>` forwards it to the boxed inner field.
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛫️encode/🦀️.rs`: the blanket `FieldProjectionSource` impl forwards `projection_identity` to `DslField`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📝️dsl/🦀️.rs`: `dynamic_literal!` takes an identity fn. `Dictionary` returns `original_root_identity()`, falling back to its address when empty. `Value` delegates to the dictionary variant. `Atom` uses its address.
- Law test `dictionary_projection_identity_follows_the_shared_original_root_across_header_moves` in `🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧵️retirement/🦀️.rs`. It reuses the previously orphaned language-agnostic fixture `📤️projection/🧪️tests/🧫️fixtures/🔣️.json`. It checks identity equals the root and stays equal after the dictionary is moved into a Box. It also checks root keys, the text path and the object arity. Passes (`neural-tst5`).
- SHARED CHANGE: additive `DslField::projection_identity` (defaulted) in dsl-record. Every other implementor keeps the address behaviour.

## B. OrderedSet NativeReceive admission (REVIEW fwwasm)
Old: `NativeMappedReceiver<I,Vec<String>,OrderedSet>` with `from_native = collect()`, which did unadmitted inserts.
New: `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧺️set/🦀️.rs` is a dedicated `NativeSetReceiver<I>`, now `pub mod set`.
- It reads the `Vec<String>` through a `NativeReceiveSlot` (funded sequence custody, own binding and closure).
- It then consumes the members one at a time. Each member goes through the same admitted phases as the sibling `NativeMapReceiver`: `SharedOwner::admit` for key and value, `Box<UpdateCursor<()>>`, `advance_insert_controlled`, swap, funded residual close. The residual holds the old map root, the cursor and the removed duplicate.
- The emptied `Vec<String>` is released through funded `RetainedCloneClose`.
- Every allocation is quoted before it happens, and denied grants make no progress. Close is funded from any phase.
- Removed: the `NativeReceiveMap<Vec<String>> for OrderedSet` impl (unused elsewhere). `NativeDefault for OrderedSet` is unchanged.
- Law test: `🧺️set/🧪️tests/🦀️.rs`, a separate Cargo `[[test]] set_receiving` in `🧳️owned/📦️packages/🦀️rust/Cargo.toml`. Fixture `🧺️set/🧪️tests/🧫️fixtures/🔣️.json`, schema `🧺️set/🧬️schema/🔣️.json`. It follows the map receiver law:
  - every turn is tried with denied grants (items, copy, capacity, release, depth) and must give zero progress and zero heap;
  - every granted turn must satisfy `heap == (retained_capacity_bytes, released_bytes)`;
  - cancellation cuts 0..512 plus a full run, with funded close and terminal-empty at each cut;
  - the result is compared with the fixture and a `BTreeSet` oracle.
  Passes (`neural-tst3`, `neural-tst4`).
- It is a separate test target because the lib-test source closure of value-native-owned is exactly 32 files (policy ceiling). Adding test files to the lib target would have been refused by the gate. Verified closure: 32 files (`1029-tools/neural/count.ts`).

## C. Duplicate-impl audit
Greps over `🧰️framework` and `✏️s`:
- `NativeReceive`/`NativeDefault` for `OrderedSet`: exactly one definition, `🧳️owned/🧺️set/🦀️.rs`. `OrderedMap`: one, `🧳️owned/🗂️map/🦀️.rs`.
- `DslField`/`BorrowedDslField` for `OrderedMap`/`OrderedSet`: exactly one definition, `🗣️dsl/🧬️schema/🪆️binding/🗂️ordered/🦀️.rs`, mounted once by `#[path="🗂️ordered/🦀️.rs"] mod ordered_fields;` (binding `🦀️.rs` L650). No inline copy remains in the binding root.
- `DslField` for `Atom`/`Dictionary`/`Value`: three expansions of `dynamic_literal!` in `🧠️neural/⚙️engine/📝️dsl/🦀️.rs`, mounted once as `mod dsl_binding;` in `🧱️model/🦀️.rs` L61. No other `FieldProjectionSource` or `OriginalDomainProjection` impl for these types exists, so no E0119.
- The native lib/tests check and the wasm check below compile with all of these mounted together.

## Verification (logs `C/🗑️generated/1029/neural-*.txt`, `.rc` = exit code)
- nat1 (root workspace, native, `--lib --tests`): neural-engine-model, -registry, -budgeted, neural-engine, flow-model, artifact-flow-flow. rc 0.
- nat2 (framework workspace, native, `--lib --tests`): value-native-owned, dsl-record. rc 0.
- wasm1 (`--target wasm32-wasip2 --lib`): the same 8 crates. rc 0.
- tst3/tst4: `--test set_receiving` passes. tst5: neural-engine-model `--lib` has 17 passed and 1 failed (see below).
- chk1 note: a root-workspace `check --tests` of non-member crates fails for missing dev-deps. Use the owning workspace manifest (native-owned and dsl-record: `🧰️framework/Cargo.toml`).

## Findings for others (not touched)
- neural-engine-model lib test `retirement::tests::finite_schema_operator_fields_preserve_typed_work_and_physical_custody` still fails (grant blocked on turn 63). It is in the flowtests list. The other two earlier failures (`evaluation_snapshot...`, `recursive_dictionary...`) pass in tst5.
- dsl-record integration test `ordered_map_field::retained_ordered_map_field_matches_the_btree_map_oracle` fails (`neural-tst6`): `🧪️tests/🗺️ordered-map-field/🦀️.rs:41` expects an EMPTY retained map's `to_value_controlled` to succeed under an always-cancelled control. The flow `OrderedMap` DslField (`native_encoding::project_entries`) returns a cancellation error. Not diagnosed further. Owner: flow (`🪆️binding/🗂️ordered/🦀️.rs`) or flowtests, not my change.

## Files changed
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🪆️binding/🦀️.rs`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛫️encode/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📝️dsl/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧵️retirement/🦀️.rs` (added one law test only; no existing expectation touched)
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧺️set/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧺️set/🧪️tests/🦀️.rs` (new)
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧺️set/🧪️tests/🧫️fixtures/🔣️.json` (new)
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧺️set/🧬️schema/🔣️.json` (new)
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📦️packages/🦀️rust/Cargo.toml` (`[[test]] set_receiving`)
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🦀️.rs` (`pub mod set`)
- Tools: `C/1029-tools/neural/{run.sh,wait.sh,count.ts}`.
