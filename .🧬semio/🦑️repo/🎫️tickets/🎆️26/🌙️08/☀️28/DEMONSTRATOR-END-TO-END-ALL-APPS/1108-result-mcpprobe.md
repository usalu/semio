# mcpprobe lane result (11/10)

## State
- `semio-os-mcp-probe-document` native lib: rc 0 (W-mcpprobe-6, was 20 errors).
- Ported: receiving cursor `close_into` reply protocol; `receive_pack(bytes, output, owner)`; sqlite native decode/encode (output slot, `ReceivingSettlement`, `receive_nested`); `to/from_sqlite_database_receiving` overrides deleted; json `receive`/`write` (output slot) and the JSON parser step through `with_allocated_effect`; `probe_json::parse/print` run as owner-less `OriginalRetirementPump` turns with fuel sized from the text length (`(len+1)*STEP_WORK+FRAME_WORK`; print: the 32 MiB file ceiling), close turns funded by the exact `retirement_work_demand` quote, stall bound 8 turns, 100k turn ceiling.
- Added (shared): `OriginalRetirementTurn::with_native_decoder_work/with_native_encoder_work` (job-context): the turn lends its fuel as the original work loan and its grant as the original construction loan to a native control. `NativeEncodeControl::retirement_demands/retirement_work_demand` (value/encode) mirror the decode quote accessors.
- New test: `mcp/🔬️probe-document/🧪️tests/json_round_trip.rs` ([[test]] json_round_trip): print agrees with serde_json, parse restores the tree.
- LOCK NEEDED: probe-document Cargo.toml (+semio-framework-job).

## Debts
- `workspace/🔬️probe-document/📥️receiving/🧪️tests/🦀️.rs` is stale (imports removed `NativeDecodeWorkRecipient` / `NativeDecodeConstructionRecipient`): `--lib` tests of this crate do not compile. Port with the driven effect harness (owner: next probe lane).
- `semantic::Decode/Projection` (sqlite semantic receiving) lost its only caller with the deleted `*_receiving` overrides; dead-code warnings remain.

## Update (11/11)
- Coordinator ruling acknowledged: no `NativeSnapshotEncodeEffectQuote` reference remains (store/🦀️.rs untouched by me; probe writer does not use it). I re-added the export twice because a peer overwrite dropped it; sorry for the churn.
- Writer is back on `native.with_effect(grant, STEP_WORK, ..)` steps. Last lib-green label: W-mcpprobe-6 (before the writer/capacity edits); W-mcpprobe-8 (current tree) is queued, rc not yet read.
- Open: runtime `print` fails with "unallocated native effect invoked an allocator" in the final writer steps; allocating steps need the current io-control `encode_effect` / `NativeSnapshotEncodeAllocation` API (see `copy_text_encoded` in `🚪️io/⏱️control/🦀️.rs`). Encoder frame close also returns UnsupportedOwner (NativeSnapshotReceiving has no `close_into`): snapretire lane.
- W-mcp-2 set rerun (W-mcpprobe-c1) is blocked upstream by `semio-plugin-instance-render` (21 errors, instance lane); instance-stores/runtime-instances status not re-observed.

W-mcpprobe-8 (current tree, probe-document native lib): rc 0, no errors.

STATUS: BLOCKED writer runtime (encode allocation API port) and W-mcp-2 set (instance-render upstream); FIXED-verified W-mcpprobe-8 (probe-document native lib compiles)
