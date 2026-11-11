# mcpprobe2 lane result (11/11)

## State
- `NativeSnapshotReceiving` (io-control `🛫️snapshot/🦀️.rs`) now implements the reply protocol: `close_into`, `recover_close_into`, `close_original_into`, `recover_close_original_into` settle its poll forms through `native_reply_poll` (pending keeps child state, recovery re-polls). Compiled: W-mcpprobe2-1 (probe-document native lib, which builds io-control) rc 0.
- probe_json stall bound: the literals `STALL_TURNS=8` / `CLOSE_TURNS=100_000` are gone. The turn ceiling is `close_turns(input quote)` (the `fuel(len)` quote of the owner-less read/print turn; each progressing close turn charges at least one unit) and the stall bound is `ACTION_TURNS=2` (begin turn plus its single recovery turn of a pending reply). `drain_decode/drain_encode` take the quote.
- Receiving tests (`📥️receiving/🧪️tests/🦀️.rs`) ported from the removed `NativeDecodeWorkRecipient/ConstructionRecipient` to `OriginalRetirementPump` turns (`with_native_decoder_work` + `with_allocated_effect`/`with_effect`; closes through `turn.with_effect(|e|slot.close_step(e))`). Not yet compiled/run.

## Open
- Writer runtime fault (unallocated native effect invoked an allocator): pending test run W-mcpprobe2-t1.
- W-mcp-2 set rerun after instance-render is green.

## Update 11/11 02:50
- W-mcpprobe2-t1 (lib tests): lib tests do not compile because three stale sqlite test modules (`🪶️sqlite/🫴️native|🫴️semantic/🧪️tests`, 22+49 errors on removed `*_receiving`/`encode_sqlite_snapshot_native`/`close_retirement_recipient` APIs) are included. Not ported (debt, own lane). Receiving test errors from t1 (u128 work, close poll) fixed on disk.
- To run the receiving tests the three `#[cfg(test)] pub mod probe_sqlite_*` includes in `🌉️mcp/🔬️probe-document/🦀️.rs` are TEMPORARILY commented with a `// [DEBUG] ` prefix (9 lines). MUST BE RESTORED (remove the `// [DEBUG] ` prefix) after the run; debt stays the stale sqlite tests.
- W-mcpprobe2-t2 (`--test json_round_trip`) rc1: only upstream `semio-framework-job` E0428 duplicate `admit_original_construction_settled` (stash-apply artifact, fixed by peer since). W-mcpprobe2-t3 (`--lib --test json_round_trip`) queued.
- Hypothesis for the writer fault (unverified): `retire_encode` (io-control `🛫️snapshot/🦀️.rs`) passes `bytes=None` to `effect_allocated` while the retirement step allocates frames (`admit_controlled_retirement` sets the effect actual), which trips "unallocated native effect invoked an allocator"; decode's `receive_effect` quotes the capacity. If t3 shows it, pass `(demand.capacity_bytes!=0).then_some(demand.capacity_bytes)`.

## Update 11/11 05:05 (W-mcpprobe2-t4)
- `json_round_trip`: `print` now PASSES (agrees with serde_json). Root cause of the print fault was not the writer steps: `retire_encode` (io-control `🛫️snapshot/🦀️.rs`) passed `bytes=None` to `effect_allocated` while the retirement step births frames (the effect marker records an actual capacity), so the effect refused "unallocated native effect invoked an allocator". Fix: it now passes the demand's capacity quote (`(demand.capacity_bytes!=0).then_some(demand.capacity_bytes)`), like the decoder's `receive_effect`.
- `parse` then failed with "Probe JSON step work quote was not admitted": the `fuel(len)=(len+1)*4+16` quote under-quoted (each step reserves STEP_WORK plus the invocation handoff unit and actually costs 2; strings are scanned twice, values cost token+push+pop, member names are compared pairwise). `fuel(bytes)` is now `((bytes+1)*STEPS_PER_BYTE + (bytes+1)^2)*STEP_WORK + FRAME_WORK` (checked); close turn ceiling and stall bound derive from that quote (see above). Rerun: W-mcpprobe2-t5.
- Receiving test ported compiles and runs; its first failure was the turn fuel: the pump turn must carry `work + ORIGINAL_INVOCATION_WORK` for `with_allocated_effect` (fixed in the test bank).
- Upstream reds met and waited out (not mine): `semio-framework-job` E0428 duplicate `admit_original_construction_settled` (stash-apply artifact), `value-snapshot-projection` `close_demands` bounds (closeshared).
