# 🔬️ Debug Print Inventory (production Rust)

Read-only audit of `[DEBUG]` prints in non-test Rust under `🧰️framework`, `✏️s` and `♻️mit-bestand`. Generated 2026-11-23.

## Summary

- Production occurrences of `[DEBUG]`: **58** in **31** files (grep-verified, line-precise).
- Test occurrences (path contains `🧪`, incl. `🧪️testing`): **24** in **11** files (listed briefly at the end).
- Shared helper `debug_log` is NOT [DEBUG]-specific: it prints whatever it is given. Only its 2 callers that pass a `[DEBUG]` literal are listed; the other ~138 calls use `[TRACE]` and must be kept.
- `[DEBUG]` matches that are not prints: a doc comment (shell layouts L18) and a string match (`poll/🎛️session` L175).

## Production occurrences

| # | File | Line | Printed prefix | Emitted in (fn) | Macro | Gate | Helper / static that exists only for it | File mtime | Last git commit |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/native.rs` | 254 | `[DEBUG] top checkpoint refused phase=` | `advance` | eprintln! | - | Inline in `advance`/`close_poll` of the FEM snapshot. No helper. | 2026-11-23 01:03 | 2026-11-18 |
| 2 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/native.rs` | 255 | `[DEBUG] effect refused phase=` | `advance` | eprintln! | - | Inline in `advance`/`close_poll` of the FEM snapshot. No helper. | 2026-11-23 01:03 | 2026-11-18 |
| 3 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/native.rs` | 270 | `[DEBUG] rx.advance refused phase=` | `advance` | eprintln! | - | Inline in `advance`/`close_poll` of the FEM snapshot. No helper. | 2026-11-23 01:03 | 2026-11-18 |
| 4 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/native.rs` | 453 | `[DEBUG] close_demands refused phase=` | `close_poll` | eprintln! | - | Inline in `advance`/`close_poll` of the FEM snapshot. No helper. | 2026-11-23 01:03 | 2026-11-18 |
| 5 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/native.rs` | 477 | `[DEBUG] nodes close refused adv=` | `close_poll` | eprintln! | - | Inline in `advance`/`close_poll` of the FEM snapshot. No helper. | 2026-11-23 01:03 | 2026-11-18 |
| 6 | `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs` | 590 | `[DEBUG] page allocation occupied caller=` | `pay` | eprintln! | - | Inline; no helper. | 2026-11-23 02:28 | 2026-11-21 |
| 7 | `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🦀️.rs` | 45 | `[DEBUG] slot cu=` | `debug_state` | format! | - | `debug_state` method (L45) exists only to format this string and has no call sites in the repo (dead code). Delete the method. | 2026-11-23 01:03 | 2026-11-21 |
| 8 | `🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️owned/🦀️.rs` | 62 | `[DEBUG] admit_loan heap=(` | `admit_step` | eprintln! | - | `crate_dbg` (fn L169, own `static N` cap 400) exists only for this print; called at L138, L156, L165 of the same file. Remove the calls, the fn and the static. `admit_loan` lines are inline. | 2026-11-23 02:01 | 2026-11-22 |
| 9 | `🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️owned/🦀️.rs` | 62 | `[DEBUG] admit_loan capacity_delta=` | `admit_step` | eprintln! | - | `crate_dbg` (fn L169, own `static N` cap 400) exists only for this print; called at L138, L156, L165 of the same file. Remove the calls, the fn and the static. `admit_loan` lines are inline. | 2026-11-23 02:01 | 2026-11-22 |
| 10 | `🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️owned/🦀️.rs` | 169 | `[DEBUG] recv exit tag=` | `crate_dbg` | eprintln! | - | `crate_dbg` (fn L169, own `static N` cap 400) exists only for this print; called at L138, L156, L165 of the same file. Remove the calls, the fn and the static. `admit_loan` lines are inline. | 2026-11-23 02:01 | 2026-11-22 |
| 11 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/🔁️native-poll/🦀️.rs` | 46 | `[DEBUG] poll close:` | `close_poll` | eprintln! | RUST_BACKTRACE | Inline; no helper. | 2026-11-21 01:20 | 2026-11-21 |
| 12 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/🔁️native-poll/🦀️.rs` | 103 | `[DEBUG] poll fault:` | `poll_fault` | eprintln! | RUST_BACKTRACE | Inline; no helper. | 2026-11-21 01:20 | 2026-11-21 |
| 13 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧱️store-components/📥️lifted/🏪️store/📬️store-owners-batch-fold-and-history/🦀️.rs` | 67 | `[DEBUG] fold candidate presence=` | `fold_batch_item` | eprintln! | - | Inline; no helper. | 2026-11-23 02:16 | 2026-11-18 |
| 14 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/🦀️.rs` | 318 | `[DEBUG] prepared born` | `seal_prepared_owned` | eprintln! | first 2 calls | Inline `static N` counter (first 2 calls) and `Backtrace::force_capture()`. No separate helper. | 2026-11-23 02:36 | 2026-11-21 |
| 15 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🦀️.rs` | 503 | `[DEBUG] paged prepared records presence` | `advance_sealer` | eprintln! | - | Inline; no helper. | 2026-11-23 02:16 | 2026-11-18 |
| 16 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs` | 612 | `[DEBUG] clone prepared records presence` | `advance_local` | eprintln! | - | Inline; no helper. | 2026-11-23 02:16 | 2026-11-22 |
| 17 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🧳️replacement/⚖️canonical/🦀️.rs` | 38 | `[DEBUG] replacement wire len` | `advance` | eprintln! | - | Inline; no helper. | 2026-11-22 20:30 | 2026-11-22 |
| 18 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🧳️replacement/🦀️.rs` | 145 | `[DEBUG] rclose n=` | `close` | eprintln! | - | Inline `static N` counter (L145) used only by the `rclose` prints in `close`. No separate helper. | 2026-11-23 01:52 | 2026-11-22 |
| 19 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🧳️replacement/🦀️.rs` | 145 | `[DEBUG] rclose grant0` | `close` | eprintln! | - | Inline `static N` counter (L145) used only by the `rclose` prints in `close`. No separate helper. | 2026-11-23 01:52 | 2026-11-22 |
| 20 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🧳️replacement/🦀️.rs` | 146 | `[DEBUG] rclose effect-none` | `close` | eprintln! | - | Inline `static N` counter (L145) used only by the `rclose` prints in `close`. No separate helper. | 2026-11-23 01:52 | 2026-11-22 |
| 21 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🧳️replacement/🦀️.rs` | 146 | `[DEBUG] rclose performed=` | `close` | eprintln! | - | Inline `static N` counter (L145) used only by the `rclose` prints in `close`. No separate helper. | 2026-11-23 01:52 | 2026-11-22 |
| 22 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🎨️theme-file-import-and-accessibility/🦀️.rs` | 221 | `[DEBUG] File` | `request_file_open` | format! | - | Uses shared `debug_log` (defined in async/session-and-topology and scene-render-entry; ~140 call sites, most `[TRACE]`). Keep the helper; remove only this call. | 2026-11-22 07:34 | 2026-11-22 |
| 23 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🪟️layouts/🦀️.rs` | 18 | `[DEBUG] shell node-graph fit (doc comment only)` | `-` | doc comment | - | `shell_node_graph_fit_trace` (L19) exists only to build this string; both callers are in this file. Doc comment L18 says a port probe parses the line, so check external probes/tests before removal. | 2026-11-22 12:28 | 2026-11-22 |
| 24 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🪟️layouts/🦀️.rs` | 20 | `[DEBUG] shell node-graph fit` | `shell_node_graph_fit_trace` | format! | - | `shell_node_graph_fit_trace` (L19) exists only to build this string; both callers are in this file. Doc comment L18 says a port probe parses the line, so check external probes/tests before removal. | 2026-11-22 12:28 | 2026-11-22 |
| 25 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🖱️pointer-input/🔑️key-and-move/🦀️.rs` | 88 | `[DEBUG] world3d terrain capture cancel reached the intent qu` | `handle_pointer_cancel_for` | debug_log( | - | Uses shared `debug_log` (see above). Keep the helper; remove only this call. | 2026-11-21 17:45 | 2026-11-22 |
| 26 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs` | 95 | `[DEBUG] poll_wit authority refusal epoch=` | `poll_wit` | eprintln! | - | Inline; no helper. | 2026-11-22 23:49 | 2026-11-22 |
| 27 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/📤️receipt/🦀️.rs` | 70 | `[DEBUG] unresolved original input receipt` | `trace_input_receipt_frontier` | log( | - | `trace_input_receipt_frontier` (L67) exists only to log; callers L105, L108. Wasm-only `log`. Consumer: `poll/🎛️session` L175 parses this exact string via `rsplit_once`, so that parser must change too. | 2026-11-21 04:20 | 2026-11-21 |
| 28 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs` | 86 | `[DEBUG] FEM cleanup semantic failure instance=` | `runtime_live_cleanup_pump_one` | eprintln! | - | Inline; no helper. | 2026-11-22 03:41 | 2026-11-22 |
| 29 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/⚠️failure/🦀️.rs` | 53 | `[DEBUG] unposted-origin whole=` | `original_lifecycle_unposted` | eprintln! | - | Inline; no helper. | 2026-11-22 20:33 | 2026-11-22 |
| 30 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/⚠️failure/🦀️.rs` | 69 | `[DEBUG] unposted mismatch whole=` | `debit_unposted` | eprintln! | - | Inline; no helper. | 2026-11-22 20:33 | 2026-11-22 |
| 31 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📥️natural-file/🦀️.rs` | 315 | `[DEBUG] natural-file worker operation=` | `trace_original_file_worker` | eprintln! | - | `trace_original_file_worker` (L302) exists only for this print (gated by shared `runtime_diagnostics_enabled()`), with a thread_local `calls` counter; call site L324. | 2026-11-21 15:40 | 2026-11-22 |
| 32 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/🎫️operation-admission.rs` | 50 | `[DEBUG] relay retirement` | `trace_failure_retirement` | eprintln! | SEMIO_TRACE_RELAY_RETIREMENT | `trace_failure_retirement` (L50) exists only for this print; env-gated `SEMIO_TRACE_RELAY_RETIREMENT`; call sites in the same file. | 2026-11-21 12:26 | 2026-11-22 |
| 33 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/📦️operation.rs` | 75 | `[DEBUG] relay retirement contents` | `close_inner` | eprintln! | - | `trace_retirement_refusal` (L110) exists only for its print; env-gated; callers L23 and L67. L75 and L81 are inline in `close_inner` (L81 gated by a local `tracing` flag). | 2026-11-21 12:26 | 2026-11-22 |
| 34 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/📦️operation.rs` | 81 | `[DEBUG] relay retirement entering root close` | `close_inner` | eprintln! | - | `trace_retirement_refusal` (L110) exists only for its print; env-gated; callers L23 and L67. L75 and L81 are inline in `close_inner` (L81 gated by a local `tracing` flag). | 2026-11-21 12:26 | 2026-11-22 |
| 35 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/📦️operation.rs` | 110 | `[DEBUG] relay retirement` | `trace_retirement_refusal` | eprintln! | SEMIO_TRACE_RELAY_RETIREMENT | `trace_retirement_refusal` (L110) exists only for its print; env-gated; callers L23 and L67. L75 and L81 are inline in `close_inner` (L81 gated by a local `tracing` flag). | 2026-11-21 12:26 | 2026-11-22 |
| 36 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/🪙️escrow.rs` | 199 | `[DEBUG] relay retirement` | `trace_retirement` | eprintln! | SEMIO_TRACE_RELAY_RETIREMENT | `trace_retirement` (L199) exists only for this print; env-gated `SEMIO_TRACE_RELAY_RETIREMENT`; 5 call sites in the same file. | 2026-11-21 12:51 | 2026-11-22 |
| 37 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs` | 104 | `[DEBUG] poll refusal in` | `acknowledge` | eprintln! | RUST_BACKTRACE | `poll_trace_sample` (L22) is a power-of-two sampler used only to throttle these prints (call sites L144, L208, L251 via the `trace` field). L175 is a string match on the receipt line above, not a print. | 2026-11-21 12:28 | 2026-11-22 |
| 38 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs` | 162 | `[DEBUG] poll VM body:` | `advance_native` | eprintln! | RUST_BACKTRACE | `poll_trace_sample` (L22) is a power-of-two sampler used only to throttle these prints (call sites L144, L208, L251 via the `trace` field). L175 is a string match on the receipt line above, not a print. | 2026-11-21 12:28 | 2026-11-22 |
| 39 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs` | 172 | `[DEBUG] poll non-VM refusal in` | `advance_native` | eprintln! | RUST_BACKTRACE | `poll_trace_sample` (L22) is a power-of-two sampler used only to throttle these prints (call sites L144, L208, L251 via the `trace` field). L175 is a string match on the receipt line above, not a print. | 2026-11-21 12:28 | 2026-11-22 |
| 40 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs` | 175 | `[DEBUG] unresolved original input receipt` | `advance_native` | string match | - | `poll_trace_sample` (L22) is a power-of-two sampler used only to throttle these prints (call sites L144, L208, L251 via the `trace` field). L175 is a string match on the receipt line above, not a print. | 2026-11-21 12:28 | 2026-11-22 |
| 41 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs` | 176 | `[DEBUG] poll guest Input:` | `advance_native` | eprintln! | - | `poll_trace_sample` (L22) is a power-of-two sampler used only to throttle these prints (call sites L144, L208, L251 via the `trace` field). L175 is a string match on the receipt line above, not a print. | 2026-11-21 12:28 | 2026-11-22 |
| 42 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs` | 189 | `[DEBUG] poll native progress: step` | `advance_native` | eprintln! | - | `poll_trace_sample` (L22) is a power-of-two sampler used only to throttle these prints (call sites L144, L208, L251 via the `trace` field). L175 is a string match on the receipt line above, not a print. | 2026-11-21 12:28 | 2026-11-22 |
| 43 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs` | 209 | `[DEBUG] poll driver progress: step` | `drive_inner` | eprintln! | - | `poll_trace_sample` (L22) is a power-of-two sampler used only to throttle these prints (call sites L144, L208, L251 via the `trace` field). L175 is a string match on the receipt line above, not a print. | 2026-11-21 12:28 | 2026-11-22 |
| 44 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs` | 252 | `[DEBUG] Input SRE1 settled: receipt` | `drive_inner` | eprintln! | - | `poll_trace_sample` (L22) is a power-of-two sampler used only to throttle these prints (call sites L144, L208, L251 via the `trace` field). L175 is a string match on the receipt line above, not a print. | 2026-11-21 12:28 | 2026-11-22 |
| 45 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs` | 366 | `[DEBUG] poll close frontier:` | `close` | eprintln! | - | `poll_trace_sample` (L22) is a power-of-two sampler used only to throttle these prints (call sites L144, L208, L251 via the `trace` field). L175 is a string match on the receipt line above, not a print. | 2026-11-21 12:28 | 2026-11-22 |
| 46 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🦀️.rs` | 399 | `[DEBUG] Boot SRE1 settled: epoch` | `accept_journaled_step` | eprintln! | RUST_BACKTRACE | Inline; no helper. | 2026-11-21 04:52 | 2026-11-21 |
| 47 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🦀️.rs` | 411 | `[DEBUG] Boot SAB1 settled: epoch` | `accept_bootstrap_step` | eprintln! | RUST_BACKTRACE | Inline; no helper. | 2026-11-21 04:52 | 2026-11-21 |
| 48 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🦀️.rs` | 443 | `[DEBUG] poll unframed scalar:` | `scalar_step` | eprintln! | RUST_BACKTRACE | Inline; no helper. | 2026-11-21 04:52 | 2026-11-21 |
| 49 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔔️wake/📃️policy/📥️operation/🦀️.rs` | 125 | `[DEBUG] poll physical currency: treasury` | `physical_policy` | eprintln! | RUST_BACKTRACE | Inline; no helper. | 2026-11-21 07:34 | 2026-11-21 |
| 50 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/📊️trace/🦀️.rs` | 38 | `[DEBUG] Core Input witness length=` | `input_witness` | eprintln! | SEMIO_TRACE_CORE_PHYSICAL (enabled()) | Whole file is a `SEMIO_TRACE_CORE_PHYSICAL` trace facility: `enabled()` (L174) and its static gate it; `input_witness` (L17), `log` method (L201) and `pending_frame_return` (L227) exist only for these prints. Statics at L18 and L172 not verified. Check the rest of the file (e.g. `Breakdown`) before deleting. | 2026-11-21 11:13 | 2026-11-22 |
| 51 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/📊️trace/🦀️.rs` | 206 | `[DEBUG] Core physical event=` | `log` | see source | SEMIO_TRACE_CORE_PHYSICAL (enabled()) | Whole file is a `SEMIO_TRACE_CORE_PHYSICAL` trace facility: `enabled()` (L174) and its static gate it; `input_witness` (L17), `log` method (L201) and `pending_frame_return` (L227) exist only for these prints. Statics at L18 and L172 not verified. Check the rest of the file (e.g. `Breakdown`) before deleting. | 2026-11-21 11:13 | 2026-11-22 |
| 52 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/📊️trace/🦀️.rs` | 272 | `[DEBUG] Core pending frame return sample=` | `pending_frame_return` | see source | SEMIO_TRACE_CORE_PHYSICAL (enabled()) | Whole file is a `SEMIO_TRACE_CORE_PHYSICAL` trace facility: `enabled()` (L174) and its static gate it; `input_witness` (L17), `log` method (L201) and `pending_frame_return` (L227) exist only for these prints. Statics at L18 and L172 not verified. Check the rest of the file (e.g. `Breakdown`) before deleting. | 2026-11-21 11:13 | 2026-11-22 |
| 53 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/🦀️.rs` | 142 | `[DEBUG] Core scalar: step` | `advance_scalar` | eprintln! | - | Inline; no helper. | 2026-11-21 10:23 | 2026-11-22 |
| 54 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🚪️io/📞️imports/🦀️.rs` | 84 | `[DEBUG] poll Source clock lookup did not match: requested` | `advance_once` | eprintln! | RUST_BACKTRACE | Inline; no helper. | 2026-11-21 12:53 | 2026-11-22 |
| 55 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/👷️jobs/📥️staging/🧾️typed-operations/🦀️.rs` | 189 | `[DEBUG] stalled '` | `fault_stalled_typed_operation_publication` | eprintln! | - | Inline; no helper. | 2026-11-22 19:59 | 2026-11-22 |
| 56 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs` | 219 | `[DEBUG] one-item publication preparation refused:` | `publish_mounted_typed_operation_unit` | eprintln! | - | Inline; no helper. | 2026-11-22 23:28 | 2026-11-22 |
| 57 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs` | 220 | `[DEBUG] one-item publication blocked` | `publish_mounted_typed_operation_unit` | eprintln! | - | Inline; no helper. | 2026-11-22 23:28 | 2026-11-22 |
| 58 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs` | 365 | `[DEBUG] worker continuation cause:` | `drive_worker_step` | eprintln! | - | Inline; no helper. | 2026-11-23 02:24 | 2026-11-22 |

## Test occurrences (brief, not for removal)

- `🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🧪️tests/authority.rs` lines 50, 50, 50
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs` lines 92, 95
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️tests/📊️native-progress/retained-charge.rs` lines 144
- `🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🧪️tests/🦀️.rs` lines 116
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/🖼️display/🧪️tests/🦀️.rs` lines 27, 41
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🔧️replacement-fixtures-and-recursive-documents/🦀️.rs` lines 71
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔨️modules/🏠️host/🧪️tests/🔬️unit/🦀️.rs` lines 275
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌉️composite-bridge-girder/🧪️tests/🧩️example/🦀️.rs` lines 33, 37
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📩️archive/🦀️.rs` lines 170, 213, 264, 313, 387, 503, 513, 642, 652
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/💾️native-save/🧪️tests/🔬️original-save/🦀️.rs` lines 170
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs` lines 1069

## Notes

- Line numbers come from grep -a -n; the code is minified onto long lines, so several hits share one line (multiple columns in the table).
- Mtime is the working-tree file time; last commit is `git log -1 --format=%cd -- <file>`. Both read only.
- `RUST_BACKTRACE` gates are ambient env checks, not [DEBUG]-specific; they are shown only for context.

## Refresh 2026-11-23

Scope: `rg -n --hidden -g '*.rs' -g '*.ts' -F '[DEBUG]'` from the repo root, excluding `node_modules`, `.cache`, `.🧬semio/`, and any path containing `🧪️tests` or `🧪️testing`. The one `#[cfg(test)]` module hit (`full-operation/🦀️.rs` L387, inside `mod publication_failure_tests`) is excluded. `#[cfg(test)]` single items (e.g. `poll/🎛️session` fixtures, `budget/🦀️.rs` `bind`) do not cover their neighbouring hits and were kept.

**Totals: 82 production `[DEBUG]` occurrences in 44 files.** This replaces the 58 in 31 from the earlier census, which no longer matches the tree (several old files were removed or rewritten, and `.ts` files are now included).

Kinds: eprintln 60, doc comment 5, console.error 3, census code (not a print) 3, parser string (not a print) 2, debug_log 2, print (writeln! stderr) 2, guard string (not a print) 1, console.debug 1, format! (print string) 1, log( (wasm) 1, format! (debug_state) 1. Non-print hits are doc comments, parser or guard strings, and census code.

| # | File (repo-relative, from `./`) | Lines | Kinds | Extra care |
|---|---|---|---|---|
| 1 | `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🌐️site/🟦️.ts` | 117 | guard string (not a print) | Guard string, not a print; keep |
| 2 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts` | 159, 191, 201 | console.error | - |
| 3 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts` | 338 | parser string (not a print) | PARSER L338 of `[DEBUG] Fixed UDRW image` producer lines; fixture under 🧪️tests holds them |
| 4 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/💼️job-bridge-and-live-mount/🦀️.rs` | 32 | eprintln | - |
| 5 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🗂️mounted-state-and-preparation/🦀️.rs` | 249 | eprintln | - |
| 6 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/⚡️host-effects/🧭️on-demand-request/🟦️.ts` | 6, 10 | console.debug, doc comment | Doc L6 promises a `[DEBUG]` line for dropped payloads; update doc with removal |
| 7 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/native.rs` | 254, 255, 270, 453, 477 | eprintln | - |
| 8 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🪟️layouts/🦀️.rs` | 18, 20 | doc comment, format! (print string) | Doc L18 says a port probe parses the line; helper `shell_node_graph_fit_trace` L19 becomes dead |
| 9 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🖱️pointer-input/🔑️key-and-move/🦀️.rs` | 88 | debug_log | - |
| 10 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🎨️theme-file-import-and-accessibility/🦀️.rs` | 221 | debug_log | - |
| 11 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts` | 152, 155, 157, 163, 168, 182 | census code (not a print), doc comment | Census tool (path has 🧪️test, not 🧪️tests) counts `[DEBUG]` via git grep oracle; not a print |
| 12 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/🔁️native-poll/🦀️.rs` | 46, 103 | eprintln | - |
| 13 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job/🧮️solver-view/🦀️.rs` | 350 | eprintln | - |
| 14 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🧳️replacement/🦀️.rs` | 147 | eprintln | Static counter near L145 used only by `rclose` prints |
| 15 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs` | 63 | eprintln | - |
| 16 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📥️natural-file/🦀️.rs` | 315 | eprintln | Helper `trace_original_file_worker` L302 dead (call L324) with thread_local calls counter |
| 17 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔔️wake/📃️policy/📥️operation/🦀️.rs` | 125 | eprintln | - |
| 18 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🦀️.rs` | 399, 411, 443 | eprintln | - |
| 19 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs` | 104, 162, 172, 175, 176, 189, 209, 252, 366 | eprintln, parser string (not a print) | PARSER L175 (`rsplit_once` on the L70 receipt text); helper `poll_trace_sample` L22 becomes dead |
| 20 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/📦️operation.rs` | 75, 81, 110 | eprintln | Helper `trace_retirement_refusal` L110 dead; L75/L81 inline |
| 21 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/🎫️operation-admission.rs` | 50 | eprintln | Helper `trace_failure_retirement` L50 dead (env `SEMIO_TRACE_RELAY_RETIREMENT`) |
| 22 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/🪙️escrow.rs` | 199 | eprintln | Helper `trace_retirement` L199 dead (5 callers) |
| 23 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🎞️media-transfer/🦀️.rs` | 77 | eprintln | - |
| 24 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📥️staging/🖼️render/🦀️.rs` | 125 | eprintln | - |
| 25 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs` | 86 | eprintln | - |
| 26 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs` | 107 | eprintln | - |
| 27 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/📤️receipt/🦀️.rs` | 70 | log( (wasm) | L70 text is parsed by poll/session L175; helper `trace_input_receipt_frontier` L67 (callers L105, L108) becomes dead |
| 28 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/📊️trace/🦀️.rs` | 38, 206, 272 | eprintln, print (writeln! stderr) | Whole `SEMIO_TRACE_CORE_PHYSICAL` facility: `input_witness` L17, `pending_frame_return` L227, `log`, `enabled()`; callers invocation/🦀️.rs L93, L131 |
| 29 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/🦀️.rs` | 142 | eprintln | - |
| 30 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🚪️io/📞️imports/🦀️.rs` | 84 | eprintln | - |
| 31 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/⚠️failure/🦀️.rs` | 53, 69 | eprintln | - |
| 32 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🛫️native/🦀️.rs` | 32, 34 | eprintln | - |
| 33 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/📨️wire/source_loan.rs` | 14 | eprintln | - |
| 34 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/📨️wire/source_staging.rs` | 59, 68 | eprintln | - |
| 35 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧶️guidance/type-guidance.rs` | 24, 743, 751, 756, 769, 794, 1727, 1743 | eprintln | - |
| 36 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧭️profile/retained.rs` | 59 | eprintln | - |
| 37 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/guided-type-tests.rs` | 20 | eprintln | Name suggests test-only file outside 🧪️tests; verify before removal |
| 38 | `🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget/🦀️.rs` | 270 | eprintln | - |
| 39 | `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🦀️.rs` | 45 | format! (debug_state) | `debug_state` L45 has no callers: delete method |
| 40 | `🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️decode/🦀️.rs` | 69 | eprintln | - |
| 41 | `🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/🧳️receiving/🦀️.rs` | 63 | eprintln | - |
| 42 | `🧰️framework/🔨️modules/🌱️value/🛫️encode/🦀️.rs` | 237 | eprintln | - |
| 43 | `🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🎟️work-action/🤝️loan/🦀️.rs` | 72 | eprintln | - |
| 44 | `🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs` | 171 | eprintln | - |

### Extra care (summary)

1. **Parsed text, poll receipt**: `poll/🎛️session/🦀️.rs` L175 matches the receipt string printed at `receipt/🦀️.rs` L70. Remove both together and drop the parser branch; otherwise the parsed diagnostic silently stops.
2. **Parsed text, NX graph**: `🔌️nx/📦️graph/🟦️.ts` L338 parses lines starting with `[DEBUG] Fixed UDRW image` from producer text. No production print of that string was found in `.rs`/`.ts`; the fixture `🧪️tests/🔣️.json` carries it. Check its producer before changing the prefix.
3. **Doc-referenced probe**: shell layouts L18 doc says an external port probe parses `[DEBUG] shell node-graph fit`. Check for external probes before removing L19-L20.
4. **Census**: `docstring-and-debug-census/🟦️.ts` counts `[DEBUG]` via a `git grep` oracle; its expected counts change after removal. Its path has `🧪️test` (singular), so the test-path rule does not exclude it. Its own strings are not prints.
5. **Trace facility**: `🧠️interpreter/.../📊️trace/🦀️.rs` is a whole `SEMIO_TRACE_CORE_PHYSICAL` facility. Hits there are L38 (`eprintln!`) and the `writeln!` prints L206 and L272; check the rest of the file (e.g. `Breakdown`) before deleting the module.
6. **Test-like name**: `guided-type-tests.rs` (fem/ifc `✳️any/✏️editor/`) is outside `🧪️tests` but looks test-only. Verify before removal.
7. **Doc mentions**: `on-demand-request/🟦️.ts` L6 doc promises a `[DEBUG]` line; update the doc with the console.debug removal.

### Helpers that become dead once the prints are removed

- `debug_state` (method, `🌳️value/🦀️.rs` L45): no callers in the repo. Delete.
- `poll_trace_sample` (`poll/🎛️session/🦀️.rs` L22) with `trace` field and call sites L144, L208, L251. Dead once prints go.
- `trace_input_receipt_frontier` (`receipt/🦀️.rs` L67), callers L105, L108.
- `shell_node_graph_fit_trace` (`shell-actions/.../layouts/🦀️.rs` L19), callers in same file.
- `trace_failure_retirement` (`operation-admission.rs` L50), `trace_retirement_refusal` (`operation.rs` L110, callers L23, L67), `trace_retirement` (`escrow.rs` L199, 5 callers). Env gate `SEMIO_TRACE_RELAY_RETIREMENT` goes with them.
- `trace_original_file_worker` (`natural-file/🦀️.rs` L302, call L324) with its thread_local calls counter.
- `input_witness` (L17), `pending_frame_return` (L227), `log` and `enabled()` in `📊️trace/🦀️.rs`, callers in `invocation/🦀️.rs` L93, L131.
- Static counter near L145 in `replacement/🦀️.rs` used only by the `rclose` prints.
- Kept: shared `debug_log` helper used by about 140 `[TRACE]` calls (only the `[DEBUG]` call sites go).


## Removal 2026-11-23

Re-scan before removal (`[DEBUG]` in `*.rs`/`*.ts`, excluding `.🧬semio/`, `node_modules`, `.cache`, `🧪️test*` paths): 41 files with 73 hits. The snapshot above was stale: the `pack/🔤️json/📥️decode/🧳️receiving`, `replication/.../seal` and `replacement` (rclose) prints, and `guided-type-tests.rs`, were already gone.

**Removed: 67 `[DEBUG]` lines in 36 files** (plus unprefixed companions that only fed them, e.g. `eprintln!("{}", r.debug_state())` in `snapshot/native.rs`), 37 files edited in total with the extra `invocation/📞️host/🦀️.rs` call site. Production `[DEBUG]` hits left in `*.rs`/`*.ts`: 6 lines in 5 files (below).

### Files changed

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/💼️job-bridge-and-live-mount/🦀️.rs` (1)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🗂️mounted-state-and-preparation/🦀️.rs` (1)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/native.rs` (5)
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🧵️mesh-job/🦀️.rs` (1)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job/🧮️solver-view/🦀️.rs` (1)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/work.rs` (1)
- `🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget/🦀️.rs` (1)
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️decode/🦀️.rs` (1)
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs` (2)
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🦀️.rs` (1)
- `🧰️framework/🔨️modules/🌱️value/🛫️encode/🦀️.rs` (1)
- `🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🎟️work-action/🤝️loan/🦀️.rs` (1)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/💾️native-save/🧱️program.rs` (8)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` (1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs` (1)
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📋️clipboard/🛬️decode/🦀️.rs` (1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📥️natural-file/🦀️.rs` (1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔔️wake/📃️policy/📥️operation/🦀️.rs` (1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs` (1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🦀️.rs` (3)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs` (9)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/📦️operation.rs` (3)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/🎫️operation-admission.rs` (1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/🪙️escrow.rs` (1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/⚠️failure/🦀️.rs` (2)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs` (1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/📤️receipt/🦀️.rs` (1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts` (3)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs` (1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/📊️trace/🦀️.rs` (3)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/🦀️.rs` (1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/⚡️host-effects/🧭️on-demand-request/🟦️.ts` (2)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🚪️io/📞️imports/🦀️.rs` (1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/📨️plugin-artifact-channel/🔁️native-poll/🦀️.rs` (2)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🎨️theme-file-import-and-accessibility/🦀️.rs` (1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🖱️pointer-input/🔑️key-and-move/🦀️.rs` (1)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/📞️host/🦀️.rs` (call site of the trace `Slice`)

### Dead code removed with the prints

- `debug_state` (`🌳️value/🦀️.rs`) and its only caller prints in `snapshot/native.rs` (`close_demands` now returns `self.close_quote(body)?.0`).
- Poll: `PollProgressTrace`, the `trace` field, `poll_trace_sample` and all samples/frontiers/`trace_input`/`input_receipt` locals, the `close()` attempt counter, and every `RUST_BACKTRACE` gate.
- Poll receipt: the `[DEBUG] unresolved original input receipt` guest log (`trace_input_receipt_frontier`, `frontier`/`detail` locals and their ten assignments in `receipt/🦀️.rs`) together with its only reader, the `rsplit_once("[DEBUG] unresolved original input receipt")` scrape in `poll/🎛️session/🦀️.rs`. The scrape fed only another print (`poll guest Input: ...`), so no typed replacement was needed. `GuestInstance::guest_diagnostics_text` stays: it has other callers.
- Relay retirement: `trace_retirement_refusal`, `trace_failure_retirement`, `trace_retirement`, the `SEMIO_TRACE_RELAY_RETIREMENT` gate and the `close_inner` attempt counter.
- `trace_original_file_worker` and its thread-local witness (`natural-file/🦀️.rs`).
- `#[track_caller]` on `MeshRun::fail` and the fem2d session `fail`. `MeshRun::fail` keeps its `message` parameter as `_message` because about 15 callers pass it; fix the callers in a follow-up if the value is not wanted.
- Interpreter `📊️trace/🦀️.rs`: `input_witness`, `pending_frame_return`, `ordinary_flush_enabled`, `flush_ordinary`, `Stage`/`Stages`/`STAGES`, `enabled()`/`ENABLED`, `Stage::log`, `Category::NAMES`, `zero_progress` and the `SEMIO_TRACE_CORE_PHYSICAL`/`SEMIO_TRACE_OWNED_INPUT_WITNESS` gates. What remains is the `cfg(test)` work-capture used by `invocation/🧪️tests`; `Slice::finish` lost its `module`/`function` parameters. `invocation/🦀️.rs` lost `TRACE_CORE_FRAME`/`CORE_FRAME_STEPS` and its `[DEBUG] Core scalar` block; the `trace` module wiring moved from `any(test, feature = "testkit")` to `cfg(test)`, except `testkit_scalar_site`.
- The three once-guards (`static SEEN`/`ONCE`/`N` atomics) in `decode/🦀️.rs`, `ifc/🔎️details/work.rs`, `store/.../bundle/🦀️.rs` (`blocked_here` stays, it returns `Blocked`), `loan/🦀️.rs`, and the histogram statics in `job/⏱️context/🦀️.rs`.
- `on-demand-request/🟦️.ts`: the catch no longer logs; its doc says the payload is dropped silently.
- `pdf native-save/🧱️program.rs`: eight `{eprintln!(..);Ok(PlanDecision::ReferenceRequired)}` blocks reduced to `Ok(PlanDecision::ReferenceRequired)`.
- `doc-open/.../🦀️.rs`: the `refuse` closure; `key-and-move/🦀️.rs`: the terrain-cancel enqueue is now `let _ = enqueue_world3d_events(..)` because its `Err` only fed the print.

### Left in place

- `shell-actions/.../🪟️layouts/🦀️.rs` L18-20 (`shell_node_graph_fit_trace`, calls at L141 and L146): the `[DEBUG] shell node-graph fit {"x","y","zoom"}` line is a live protocol. Consumers: `.🧬semio/🦑️repo/🎫️tickets/**/port-fit-probe.mjs` (L161, L188), `wgpu-battery.mjs` (L299, scenario "Fit graph publishes a camera"), `wgpu-node-gestures-probe.mjs` (L502), `wgpu-node-graph-retention-probe.mjs` (L248). The emitter was deliberately restored on 2026-11-22 (`engine-attach-tests-2-port.md`, `battery-readiness-audit.md` R10). It is owned by that wgpu battery work; it would trip a boot acceptance test that fails on any `[DEBUG]` output, so its owner must either move the probes to a typed channel or accept the line before the release build. Reported, not changed.
- `🔌️nx/📦️graph/🟦️.ts` L338: parses tool output (`[DEBUG] Fixed UDRW image`), not our print.
- `🌐️site/🟦️.ts` L117 (teaching deploy): the guard that fails a site build when `[DEBUG]` is present.
- `docstring-and-debug-census/🟦️.ts`: counts the prints, does not print them. Its expected counts must be re-baselined by its owner.
- `editor/tests.rs` L716 and `editor/tree-probe-tests.rs` L23 (ifc 4x3): both are `#[cfg(test)]` modules (`#[cfg(test)] #[path="tests.rs"] mod tests`, `tree_probe`), so excluded.
- Outside the requested `*.rs`/`*.ts` scope and not touched: `[DEBUG]` in `.mjs` tooling (`🎓️teaching/.../🌐️browser/🟨️render.mjs` L105-107, L221 and the build-graph `owner-discovery/🟨️.mjs` L355). These are build-time tool scripts, not runtime of the demonstrator.
- `debug_summary` (`job/📬️outcome/🪙️budget/🦀️.rs` L267, "Temporary") has no `[DEBUG]` text and no caller; left for its owner.
- Correction to the snapshot: the `full-operation/🦀️.rs` hit (`worker continuation cause`) is in a production `impl`, not in `mod publication_failure_tests` (that module ends at L135). It was removed.


### Verification

`repo cargo --store=shared --configuration=prototype -- check -j 2 -p <crates>` per workspace (a Cargo command cannot span workspaces; run from the workspace directory for the plugin workspaces):

- Passed (`Finished`, no `error`): the 6 `semio-framework-*` crates of the framework workspace (job-return-budget, job-native-decode, job-context, value-native-owned, value-native-encoding, value-control-contract); the 14 crates of the root workspace (`semio-plugin`, `-plugin-host-poll`, `-plugin-host-wake`, `-plugin-host-post-turn-relay`, `-plugin-reactor-context`, `-plugin-retained-command`, `-plugin-interpreter`, `-plugin-interpreter-imports`, `-plugin-decl-registry`, `-plugin-runtime-instances`, `semio-os-kernel`, `semio-os-mcp`, `semio-os-store-engine-bundle`, `-member-group`); the fem 2d workspace (`semio-s-artifact-fem-2d-editor-session`, `-model`).
- Not verified: `semio-s-fem-mesh`, `semio-s-artifact-fem-3d-editor-visual-job`, `semio-s-artifact-stdio-ifc-details`, `semio-s-artifact-stdio-pdf-native-save`, `semio-s-artifact-draw-drawing-editor`. Four runs stopped at an error in someone else's work (`semio-os-kernel-db-sync`, `🪞️replica-sync-protocol/🦀️.rs` L138 and L178: no method `clone` on `semio_framework_hash::Hasher`); the fem 3d run waited more than 35 minutes for host capacity and was stopped. The edits in those five crates are small and were re-read by hand; re-run them once `Hasher` compiles again.
- An earlier root-workspace run failed once on `semio-plugin-instance-app` (`last_publication_wait(id)` called with `&u64`, `🔌️plugin-app-impl/🦀️.rs` L409); a peer fixed it and the re-run passed. It was not caused by this removal.
- TS: `tsc --noEmit` over `handle-turn-drive/🟦️.ts` and `on-demand-request/🟦️.ts` (root `tsconfig.json` as base) reports no error in either file; the 245 errors it prints are unresolved `node:` types in unrelated transitive files.
