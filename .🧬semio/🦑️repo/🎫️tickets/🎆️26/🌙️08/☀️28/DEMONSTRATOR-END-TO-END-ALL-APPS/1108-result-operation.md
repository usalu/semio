# 1108 result operation (semio-plugin-operation; manifest cancellation/.../rust/Cargo.toml)
- FIXED-verified semio-framework-value-control-contract (nested_within): native W-operation-4, wasm W-operation-5.
- Blind port EDITED-unverified: crate dir `♻️cancellation`: `📦️lease/🦀️.rs` CancellationLeaseParts::close_step / token_step / ToolCancellationLeaseRetirement::close_step now `(…, effect) -> OriginalRetirementPoll` (RetirementPollExit ladder, settle); `🎟️issuer/🦀️.rs` 5 sites onto `close_original_effect_step` (parts close x3, cancel_close alias/close, claim_close step).
- Not ported: `📦️lease/🧪️tests/🦀️.rs` (grant-shaped close_step callers, lines ~11, 48, 65) and `🎟️issuer/🧪️tests`.
- W-operation-8 native: still blocked upstream (native-receiver-protocol, value-persistent, job escrow/payload).
- Rerun: chk.sh W-operation-<n> <manifest> native -p semio-plugin-operation
STATUS: BLOCKED upstream crates not yet compiling

## Update (W-operation-12, native)
- tree-engine now green. New upstream red: `semio-framework-value-native-owned` (208 errors, value/🛬️decode/🧳️owned/**; NativeReceiver advance_pending/close reply shape). Owners per claims log: `macros` (set, map, 🔄️map, 🌳️value/🦀️.rs; 19:52), `nested` (tuple, array, variant, unit; 19:46), `refusal2` (refusal; 19:49), `receivers2` (owned/🦀️.rs re-export). Polling.
STATUS: BLOCKED value-native-owned (macros/nested/refusal2)

## Update (W-operation-14, native)
- native-owned green. New upstream red: `semio-framework-value` umbrella (54 errs: RetainedCloneCursor::advance / BoundedOrdCursor / ErasedSnapshotRetirement close_step shape in value/🧬️retained-clone/📦️paged, 📝️shared-utf8/🧬️clone, 📣️publication/🌳️root/♻️retirement, 📦️paged/🎮️append, unresolved `admit_shared_retirement`) plus value-edit, native-receiving-original, job-receiving. Owner: no active claim found for retained-clone contract/paged; nearest: nested (value-edit), macros/receivers2 (paged map receive). Polling.
STATUS: BLOCKED semio-framework-value (retained-clone contract trait shape)
