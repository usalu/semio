# mcp lane result (11/10)

## Design
- The trait is `ArtifactChannel` (`mcp/🔀️dispatch/🔌️channel-port/🦀️.rs`); `AppChannel` does not exist. `exchange` is now `async fn` on the trait and on all six implementors (Unbound, Mock, Shell, Routing, ShellRouted, Plugin); `ArtifactChannels` (dyn_enum_close) follows.
- Plugin channel inference path is fully awaited: `infer_real` and `drive_origin` are async (`yield_once` instead of `thread::yield_now`), `poll_to_completion` is deleted, router `register_plugin` / `infer` are `.await`ed. No `block_on`, no yield loop inside any channel.
- The tool registry (`ToolHandler = Fn(Value) -> CallToolResult`, `McpServer::dispatch`, thread-local request scopes) is synchronous end to end, so the sync world meets the async channel in exactly one place: `channel_port::exchange_blocking` (port crate). Callers: `ActionAdapter::{exchange_one, run_inference}`, gateway-backend (3 sites), tests. DEBT (owner: whoever asyncs `ToolRegistry::call`): delete `exchange_blocking` once the registry is async.
- `PluginArtifactChannel::read_artifact_pair` (sync guest turn) replaces the `exchange(ReadArtifact)` round trip in `persist_folder_guest`, so sync `activate` needs no executor.
- channel-port l.337 had a mangled doc comment (`#[dyn_enum]` text outside a comment, also in HEAD): fixed. This was the 7 errors of `semio-os-mcp-port`.

## Verification
- W-mcp-1 (`semio-os-mcp-port` native) rc 0 (was 7).
- W-mcp-2 (probe-document, dispatch, shell-channel, routing, plugin-runtime, workspace; native lib) rc 1: only `semio-os-mcp-probe-document` (20, unchanged) plus upstream `semio-plugin-instance-stores` (11, action-ingress/ingress) and `semio-plugin-runtime-instances` (6, opening) are red; the async edits in dispatch, shell-channel, routing, plugin-runtime (artifact-app-frames, inference route), workspace, gateway-backend and their tests are EDITED-unverified behind those (no diagnostics seen in my files, but dependants never compiled). Wasm: not run (none of these are in the hub closure of a verified native build yet).

## probe-document (20): NOT ported, needs a ruling
Causes, by file (all under `mcp/🏠️workspace`):
1. `🔬️probe-document/📥️receiving/🦀️.rs:111`: `RetirementCursor::close_step` is gone; the cursor protocol is now `close_into(body, effect, reply)` (+ `recover_close_into`, `close_original_into`). Port = reply-protocol cursor over `OwnedSlot::close_into`.
2. `🔬️probe-document/🦀️.rs:72`: `ArtifactPackReceiving::receive_pack(bytes, output: &mut Option<Self>, owner) -> RetainedWorkOutcome<(), NativeSnapshotFailure>`.
3. `🪶️sqlite/🦀️.rs`: `decode/encode_sqlite_snapshot_native` take `output: &mut Option<SnapshotReceivingOutput<_>>` and return `SnapshotReceivingOutcome` (pattern: `ReceivingSettlement` + `receive_nested`, see `🏪️store/📦️codec/🪶️snapshot-capability/🛬️native-decoding`); `to/from_sqlite_database_receiving` overrides can be deleted (trait defaults now run the authored projection under the original owner); `NativeSnapshotFailure::into_value_error` is gone.
4. `🚪️io/🔤️json/🦀️.rs`: `owner.receive` now takes `(output, |slot, out, marker, unwind, native, body| ..)`.
5. DESIGN BLOCKER: `probe_json::parse` / `print` (used by `ArtifactDsl::parse_dsl/print_dsl`, `encode_pack_with/decode_pack_with`, 9 call sites) build a plain `NativeDecodeControl` and close its retirement recipient with the old grant API (`close_retirement_recipient(CLOSE_GRANT)`). The effect-shaped close needs a turn/effect; the only turn-less driver is `semio_framework_io_control::testing::with_decoding` (test harness, not a production dependency). A production sync scope for turn-less parse/print needs a coordinator ruling (route through a driven harness / `OriginalRetirementPump`, or delete the sync `ArtifactDsl`/`ArtifactPack` text paths of the probe). I did not guess one.

## Debts / owners
- `exchange_blocking` (port) is the one sync bridge; delete when `ToolRegistry::call` / `McpServer::dispatch` go async (protocol lane, not started; thread-local request scopes block it).
- Guest turns inside the plugin channel (`exchange_one_real`, `await_response`) still `block_on(runtime.execute_turn)` inside `with_headless_identity` (same identity-loan limit as host-shard); only the inference path is fully awaited.
- Upstream reds for the rerun of the shell check: instance-stores 11 (instance lane), runtime-instances 6 (custody2/instance), probe-document 20 (mcp, blocked above).

STATUS: BLOCKED probe-document (20) needs a ruling on a production turn-less scope for probe_json parse/print; FIXED-verified W-mcp-1 (semio-os-mcp-port); async ArtifactChannel::exchange EDITED-unverified behind instance-stores/runtime-instances
