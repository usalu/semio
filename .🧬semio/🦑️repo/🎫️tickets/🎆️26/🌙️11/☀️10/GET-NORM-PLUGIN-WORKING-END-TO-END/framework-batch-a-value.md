# Framework batch A, value crates (lib targets)

All four lib targets pass `repo cargo ... check -p <crate> --lib` (logs: 🗑️generated/batch-a-schema-3.log, batch-a-dictionary-4.log, batch-a-presence-3.log, batch-a-failure-3.log).

## semio-framework-value-native-schema
The earlier split was un-wirable as written: the schema matcher and the key frame in the engine's source module reference each other. Resolved by making the schema crate sit on top of the engine (semio-framework-value-native-schema-engine).
- Engine `🧩️record/🌳️source/🦀️.rs`: now owns `NativeSchemaKeySource` (a source query selector) and exposes `key_frame`, `NativeSourceKey` as pub.
- Engine `🧩️record/🌳️source/🗝️key-frame.rs`: items pub; `Settle` and `settlement_of` moved here from the matcher driver (key frame concern, also used by its test).
- Engine `🧩️record/🦀️.rs`: no longer includes the schema module; re-exports `NativeSchemaKeySource` from source.
- `🧩️record/🔎️schema/**`: `crate::` and `super::` paths now point to the engine; Cargo.toml rewritten (engine, control-contract, decoding, receiver-protocol, clone-backing, retained-clone, persistent, retirement; dev deps allocator, effect-bank, serde_json).
- `🧳️owned/🦀️.rs` and its Cargo.toml: schema items re-exported from the new crate, key source and record items from the engine (public owned API unchanged).

## semio-framework-value-dictionary
`🟰️equal/🦀️.rs`: cursor `advance`, `rebind_step`, `close_step` take `&mut RetainedAllocationEffect` (grant from the effect) and use effect-threaded `bind`, `close_one`, `reserve_one_funded`, `release_empty_page`. `🦀️.rs`: bound the control to a local (callback lifetime). Its own test file still uses the old grant signatures (lib-test not in scope).

## semio-framework-value-testing-presence-retirement
`🦀️.rs`: factory trait now uses `SharedOriginalStorage<P>`; retirement root holds it and releases via `retire()`/`take_value()`; `close_step(body, effect)`, `FactoryAuthority::step(body, effect)`.

## semio-framework-value-failure-tests
`🧬️contract/🧪️tests/parts.rs`: helpers run through `EffectBank` (effect-bank dev dependency added in Cargo.toml); `owner.step(body, effect)`; `OriginalFailureStorage::admit(effect)`. `#[test]` bodies in the same file still use the old API.

## Left
Lib-test targets of all four crates (and engine tests under 🧩️record/🧪️tests and 🌳️source/🧪️tests) not migrated or checked.
