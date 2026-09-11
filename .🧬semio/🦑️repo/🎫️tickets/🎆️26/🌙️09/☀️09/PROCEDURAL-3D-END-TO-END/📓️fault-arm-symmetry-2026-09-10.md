# Fault Arm Symmetry (2026-09-10)

Canonical `completion-result.fault` / host-async `Err` bytes are a **pack** of the fault value (`store::pack_rt::encode_wire_value(Fault.to_value())` = `encodePackValue(fault)`). There is no JSON-string dual-decode layer on this ABI.

## Defect

The OK arm was already pack (`extension_response_args` + `decode_wire_value`). The FAULT arm was not:

| Side | Was | Schema / live browser |
|---|---|---|
| Shell TS (`ShellHost`) | `outcome = { fault: encodePackValue(fault) }` | pack |
| Guest `outcome_to_result` | `dsl::decode_fault_bytes` | JSON |
| Native host-async / `RequestOutcome::Err` producers | `encode_fault_bytes` | JSON (same-side with old guest, wrong vs schema and vs the live browser path) |

A packed fault decoded as JSON is mojibake. generation3d / procedural 3D showed an unreadable error on this arm.

## What changed

### Guest (`semio-framework-plugin` / `host/`)

- Added `encode_fault_pack` / `decode_fault_pack` via `pack_rt` + `ToValue` / `FromValue` on `Fault`.
- `outcome_to_result` Err arm uses `decode_fault_pack`.
- Every Direct `host-async` `.map_err(|bytes| decode_fault_bytes)` site now uses `decode_fault_pack` (guest `decode_fault_bytes` count is **0**).

### Native host (`semio-framework-plugin-host`)

Producers that write the guest-visible Err pack now encode pack, not JSON:

- `host/effects` `fault_bytes`
- `host/imports` `fault_bytes`
- `host.rs` `poll_backed_direct_await_fault`
- `host/shard` `start_job_fault_bytes`

Shape: `store::pack_rt::encode_wire_value(&dsl::ToValue::to_value(&Fault::new(...)))`.

The host crate does not depend on the guest SDK, so it does **not** call `crate::host::encode_fault_pack`.

### AppFrame::Error summaries

Producers were already pack. Summaries now decode pack:

- `ProgramBridge` wgpu `app_frame_fault_summary`
- `run.rs` `app_frame_fault_summary`

### Laws

- Fixture `plugin/reactor/fixtures/extension-result-fault-pack.json` — origin `os`, code `extension.missing`, message `no such extension`, scope.pluginId `flow-extension-brep`.
- Rust: `a_packed_host_fault_round_trips_through_outcome_to_result` — encode pack, assert not JSON, `outcome_to_result`, `take_extension_response` surfaces `ok: false` + typed code/message.
- TS (in-source via `PluginRuntime.tsx` `registerTests1`): `packs completion-result.fault the same way the guest decodes it` — `encodePackValue` / `decodePackValue` on the same fixture.

## Audit — still JSON on purpose

These are **not** `completion-result.fault` / host-async Err:

| Site | Direction | Verdict |
|---|---|---|
| `encode_fault_bytes` / `decode_fault_bytes` (diagnostic) | JSON convention | Keep |
| `plugin_error` in `plugin.rs` | guest→host `plugin-error.fault` | Still JSON; host `decode_guest_fault_bytes` still JSON. Same-side consistent; not the live extension path |
| `host.rs` transaction prepare (~6526) | `AppFrame::Error` / `TransactionPrepared` rejection | Still `decode_fault_bytes`. Different door than extension-result |
| Job unit tests | encode/decode their own JSON | Leave |
| `describe` host-async impure fault | `#[cfg(test)]` `encode_fault_bytes` | Describe door, not live extension |
| Command ingress / other AppFrame | mixed | `encode_fault_bytes` stays where producer+consumer are JSON |

WIT: `invoke-extension` is `result<pack, pack>`; other host-async Err arms are `pack`.

## Mismatch count

- Live extension-result fault arm: **1** found (shell pack vs guest JSON). **0** remaining after this change.
- Native host-async Err producers: were JSON-vs-schema (same-side with the old guest). Now pack. **0** remaining on that door.
- AppFrame Error summaries: **2** were pack-produce / JSON-decode. Now pack-decode. **0** remaining on those two summaries.
- Left JSON on other doors (plugin-error, describe test helper, transaction prepare decode, job tests): not this ABI.

## Checks and tests (real numbers)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-plugin` | exit 0, 36.53s, 2 warnings (unused `encode_fault_pack` in lib; preexisting job `new`) |
| `cargo check -p semio-framework-plugin-host` | exit 0, 1m 08s |
| `cargo check -p semio-s-plugin-procedural` | exit 0, 2m 11s |
| `cargo check -p semio-s-plugin-procedural --target wasm32-wasip2` | exit 0, 2m 33s |
| `cargo test -p semio-framework-plugin packed_host_fault` | **1 passed**, 0 failed, 645 filtered |
| `cargo test -p semio-framework-plugin a_faulted_invocation` | **1 passed**, 0 failed, 645 filtered |
| vitest PluginRuntime `--testNamePattern=packs completion-result` (`SEMIO_TEST_LEVEL=long`) | **1 passed**, 92 skipped |

Logs: `generated/fault-*.txt`.

Repo MCP (`repo://goals`, `ticket_open` / `ticket_close`) was unavailable. Bookkeeping is this file plus the `fault-*` logs.

## Restage

**Yes.** Guest decode and native host encode both changed. Browser WASM for `semio-s-plugin-procedural` must be rebuilt before a generation3d fault can be trusted in the shell.

This lane did **not** restage and did **not** run `activate-generation3d-react-dev`. Do not claim the UI error is readable until a restaged guest is probed with `[DEBUG]` console evidence.

## Not claimed

Runtime browser readability is unverified. The rust/TS laws prove pack round-trip of `code` + `message` on the ABI, not a live generation3d fault in Chrome.
