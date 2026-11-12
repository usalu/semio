# Drift-a pack: intrinsic, record-encode-cursor, record-preflight

## Changed (tests/Cargo.toml only, plus one shared rig token)
- Each crate's test module gets a thread-local `EffectBank` stock plus `turn(grant, call)`: runs one cursor turn inside a real original action, measures only the operation's heap (requests/releases), checks the work verdict. Every assertion keeps its fixture and law.
- intrinsic (`🎮️retained/🧪️tests`): `new/advance/advance_original_step/close_original_step/close_step` now take the effect; grants are passed to `turn` instead of the call.
- encode-cursor (`⏳️cursor/🧪️tests`, document cursor tests via `crate::tests::{turn,stock}`): `advance/advance_intrinsic/close` effect-threaded; `symbols.reserve_one(effect)` outcome matched on `Ready`; native paged text is now built through the paged-text crate rig (`native-rig.rs` via `#[path]`: `source`, `EncodeBank`, `encode_success`, `close_text`) instead of the removed `read_from_encoding_source`.
- preflight (`🏭️schema/🧪️tests`): `BorrowedSchemaHashCursor::advance/close` effect-threaded.
- Dev-deps added: `semio-framework-value-testing-effect-bank` (all three); clone-source, clone-source-testing, retirement, native-encoding (cursor).
- Shared rig fix: `📝️text/📦️paged/🧪️tests/encoder-rig.rs:60` `turn.witness()` -> `turn.identity()` for `new_recorded_source`.

## Status
- Combined check (drift-a-all.log) compiled the edits; the only errors were the helper's `RetainedWorkOutcome` Ok-wrapping, fixed afterwards. Not re-verified: the coordinator asked to stop cargo runs.
- Tests not run. Possible runtime risks: heap-balance assertions in preflight stage test (bank allocation is warmed via `stock()`), cursor paged-text test depends on `encode_success` funding (`Stock::for_outputs::<8194>`, maximum 65536).
- Unused-dependency (`semio-framework-trace`) possible in cursor crate; verify on next check.
