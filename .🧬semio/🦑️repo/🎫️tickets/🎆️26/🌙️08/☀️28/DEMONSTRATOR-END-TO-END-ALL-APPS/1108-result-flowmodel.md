# Lane flowmodel: semio-framework-artifact-flow-flow-model

## Done
- document-model: `RetireOwned` derive on WidgetLayout, NodeChrome, Widget (hand `metadata!`/`owned_enum!` removed); `🧵️retained/🧬️fields` keeps only `artifact_retire_struct!`.
- `🧵️retained/🦀️.rs`: `FlowOwner` derives RetireOwned. `FlowRetirement` effect-shaped (mirrors flowhost2 SessionRetirement): `reserve_demand`, `admission_demand(&owner)`, `reserve_push(effect)`, `admit_owner(&mut Option<FlowOwner>,effect)`, `demands(body)`, `step(body,effect)->OriginalRetirementPoll`, `step_progress`. Cold `from_owner/push/text/push_cold/retire_cold` run through `OriginalRetirementPump` turns (new dep `semio-framework-job` in the model Cargo.toml; LOCK NEEDED logged; sync-137 later resolved it, native+wasm built).
- Removed: grant-shaped `step/next_*_demand/selected_demand/next_push_capacity_byte_demand`, `reserve_push(grant)`, `admit_owner(owner,grant)`.
- `🎚️parameter/📨️intent`: hand cursor `SetGraphParameterRetirement` + `into_retirement` removed; `SetGraphParameter` derives RetireOwned; unit test rewritten against `ControlledRetirement`+pump; flow crate `🦀️.rs` re-export trimmed.
- `🧵️retained/🧪️tests/🔬️flow-typed-retirement/🟦️.ts` source-shape oracle retargeted to the new surface.

## Verified
- native lib W-flowmodel-5 rc0; wasm32-wasip2 lib W-flowmodel-6 rc0; native --tests compile W-flowmodel-7 rc0; unit run W-flowmodel-8 (see below).
Rerun: chk.sh W-flowmodel-<n> Cargo.toml native|wasm -p semio-framework-artifact-flow-flow-model

## Debts
- `🧵️retained/🧪️tests/{🦀️.rs,🧵️retained,🧬️fields}` Rust law files are NOT included by any module (orphans before this wave too, they target flow-flow's FlowHostSnapshot) and still use the grant API; port them when wiring them into the flow-flow crate.
- Downstream callers of the removed grant-shaped FlowRetirement methods (flow-host core/session, vcs diff `handoff`, flow-copy `📑️copy`) are flowhost3/own-lane work.

## Task 2: semio-framework-artifact-flow-flow-document (coordinator hand-off)
EDITED-unverified (flow-document cannot reach rustc past the receiving cascade, see below):
- sqlite snapshot port under `🚪️io/🪶️sqlite/📸️snapshot/🚦️native`: `🛂️capability` (output-slot `decode/encode_sqlite_snapshot_native`; the two `validate_*_decoding/encoding` overrides were dropped, the trait defaults call the same `validate_sqlite_snapshot_subset`), `🛂️carrier` (`decode_sqlite_snapshot_record_native`/`encode_sqlite_snapshot_record_native` with `&mut` closures and the `output` slot), `🛬️decoding::insert` and `🛬️reconstruction::insert_ordered` (ordered `UpdateCursor`: `native_demands/native_work_demand` + `control.with_effect(grant,quote,|_,e|cursor.advance_native(4096,e))`, `close_step(grant,u64::MAX)`, duplicate key detected by `replacement.len()<=map.len()`; removed value is drained by the close ladder). Reference for the shape: neural `insert_controlled`.
- NOT done: `DslReceiving` on `Widget`/`SynapseSpec`/`FlowLayoutEntry` (flow-model document-model). I tried it (W-flowmodel-9): it cascades and broke flow-model itself (235 errs), so it is REVERTED (flow-model recheck W-flowmodel-10). The derive needs `DslOwnedReceivingField` + `DslReceivingField` for every field type, and no cursor exists for: `neural::Dictionary` (FieldValue::Map of neural-value records), `OrderedSet`, `neural::Tree` (Block of statements), `FlowUi`/`FlowGui` (FieldValue::Value), `WidgetLayout` (plain 2xf64 struct: `DslReceiving` derive suffices). Existing cursors: scalars, String/PagedUtf8, Vec, Option, Box, PersistentList, arrays, `MappedReceiving`, `WrappedReceiving`/`StatementReceiving`, transparent newtype, `NativeIntrinsicReceiving` (record whose field N is `FieldValue::Value`, `T: NativeReceive<DslValue>`; Widget/Tree/FlowUi derive NativeReceive). Needed (framework, `🗣️dsl/🧬️schema/🫴️receiving/📥️owned`): a value-field cursor generalising `NativeIntrinsicReceiving` (skip record shape step) for Value-shaped fields, and Map/Set/Block cursors, or a hand `DslOwnedReceivingField` per type through `MappedReceiving`. The store bound `OpBinaryReceiving` (`retire_flow_store_cold`) is mandatory for every artifact, so cad/draw-style artifacts will hit the same wall.
- NOT done: registry `📔️registry/📇️catalogue-{work,owner}` (26 KB step machines over `StepContext` + `RetainedCloneClose`, not read in detail).
Rerun: `chk.sh W-flowmodel-<n> Cargo.toml native -p semio-framework-artifact-flow-flow-document`

W-flowmodel-10: flow-model recheck after the revert is red only upstream in semio-framework-pack-json (controlled-and-borrowed-bridge:189, peer edit).
STATUS: BLOCKED flow-document needs owned-receiving cursors (Dictionary/OrderedSet/Tree/FlowUi) for DslReceiving on Widget/SynapseSpec/FlowLayoutEntry; sqlite port edited-unverified; catalogue not started; flow-model FIXED-verified W-flowmodel-5 (native), W-flowmodel-6 (wasm), W-flowmodel-7 (tests compile)
