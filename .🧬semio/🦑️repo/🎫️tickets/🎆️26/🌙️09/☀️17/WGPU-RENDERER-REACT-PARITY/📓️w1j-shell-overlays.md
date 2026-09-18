# 🧊️ W1j — shell overlays the wgpu renderer lacked (approvals, transient notice, agent presence/chat, tooltip delay)

Packet W1j of `26/09/17/WGPU-RENDERER-REACT-PARITY`. Scope: the four shell overlays the
`📓️audit-shell-window-system.md` (§6 rows, recommendations 5–7) and `📓️audit-runtime-boot-input.md`
(rows 24–25, recommendation 6) list as absent on wgpu. All paths are absolute under
`/Users/ueli/Documents/semio`.

## 1. Overlay table — React → wgpu

| # | Overlay | React source | wgpu implementation (new) | Geometry / behaviour |
|---|---|---|---|---|
| 1 | Agent approvals modal | `🧱️elements/🤖️AgentApprovals/🟦️.tsx` (160 lines, Radix `Dialog`, mounted at `🏛️ShellHost/🟦️.tsx:10997`) | `🧱️elements/🤖️AgentApprovals/🎯️targets/🧊️wgpu/🦀️.rs` (parse/labels/layout/open-model) + `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` `render_agent_approvals_step` / `agent_approvals_paint_ops` / `resolve_agent_approval_control` | Centred, `max-w-lg` → `APPROVALS_MODAL_WIDTH = 512.0`, list capped at `APPROVALS_LIST_MAX_HEIGHT = 384.0` (`max-h-96`), clamped to the viewport; scrim = `theme.overlay_shadow`, body = `push_glass(Level::Dialog)`; one row per request (capability / change summary / requested-by / risk lines) + three `theme.control_height` × 150 px decision buttons; opens purely from `pending_approvals.len() > 0`, dismissible, re-opens on a newly arrived request |
| 2 | Transient notice banner | `showTransientNotice` (`🏛️ShellHost/🟦️.tsx:8065-8117`), `TRANSIENT_NOTICE_TONE_CLASS` (`:802`), JSX (`:10932-10942`) | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` region `🧯️TransientNoticeAndAgentOverlays` + `render_transient_notice_step` | Top-centre at `theme.navbar_height + 8.0` (`TRANSIENT_NOTICE_TOP_GAP`), width ≤ `TRANSIENT_NOTICE_MAX_WIDTH = 520.0` and ≤ viewport; severity tint from `Theme` tokens; exactly one at a time (a new notice replaces and restarts the clock); auto-dismiss at `TRANSIENT_NOTICE_AUTO_DISMISS_MS = 4000.0`; explicit close control `shell.notice.close` |
| 3 | Agent presence indicator | `🧱️elements/🚦️AgentPresence/🟦️.tsx` (48 lines), placed by React **only** inside the chat panel header (`🏛️ShellHost/🟦️.tsx:8558`) | `🧱️elements/🚦️AgentPresence/🎯️targets/🧊️wgpu/🦀️.rs` + `render_agent_chat_header_step` | 8 px dot (`h-2 w-2`) + 6 px gap (`gap-1.5`) + `font_size_small` status text, right-aligned in the chat panel header band (`control_height + padding_standard` tall, hairline underline); control id `shell.agent.presence` with the accessible name as its tooltip title |
| 4 | Agent chat panel | `🧱️elements/💬️AgentChatPanel/🟦️.tsx` (30 lines) = header + `BasicChatPanel` | `🧱️elements/💬️AgentChatPanel/🎯️targets/🧊️wgpu/🦀️.rs` (`plan_agent_chat_header`) + the same shell step | Header **done**; transcript **not** — see §5 gap 1 |
| 5 | Agent bridge (wire) | `🧱️elements/🔗️AgentBridge/🟦️.tsx` (476 lines) | `🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs` | Frame codec + consumer state; `ShellState::apply_agent_bridge_frame` / `take_agent_bridge_outbox` are the transport seam |
| 6 | Tooltip delay | `CHROME_CONTROL_TOOLTIP_DELAY_MS = 400` (`💡️ChromeControlHint/🟦️.tsx:20`) | `CHROME_TOOLTIP_DELAY_MS` in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | Was `500.0`, now `400.0`; arm/dismiss behaviour was already identical |

### Frame-phase placement

`ShellChromeFramePhase` gained three phases, all inside the existing retained chrome step chain
(`render_chrome_step`), so every one of them yields between opportunities like the rest of the chrome:

| Phase | Position | Step |
|---|---|---|
| `AgentChatHeader` | after `Panels`, before `Navbar` | `render_agent_chat_header_step` |
| `AgentApprovals` | after `Overlay`, before `TransientNotice` | `render_agent_approvals_step` |
| `TransientNotice` | after `AgentApprovals`, before `TreeDrag` | `render_transient_notice_step` |

The approvals modal paints above every other overlay (React mounts `<AgentApprovals>`
unconditionally, last in the shell tree) and the notice banner paints above that, matching React's
`z-50` banner over the dialog's own layer.

## 2. Frame table — `GatewayToShell` → wgpu effect

The Rust SSOT is `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️.rs` (`GatewayToShell`
`:1293-1416`, `ShellToGateway::finish` `:1255-1281`). It cannot be depended on from the renderer:
`semio-framework-os-mcp` pulls `axum` + `tokio` with `features = ["full"]`, which does not build for
`wasm32-unknown-unknown`, a target this crate must produce. The wgpu codec is therefore a third
implementation (Rust SSOT, `🟦️.ts` twin, this one) held to the SSOT's own anti-drift mechanism — see
§4.

| Tag | Frame | React (`useAgentBridge`) | wgpu (`AgentBridgeState::apply_frame`) |
|---|---|---|---|
| 0 | `Welcome` | status → `open`, clear error, publish `ShellState` | status → `Open`, clear `last_error`, reset `reconnect_attempt` (no `ShellState` publish — see §5 gap 3) |
| 1 | `ShellCommand` | reduce against the mirror, reply `shellCommandResult` | reply `shellCommandResult{ok:false}` — the wgpu shell IS the authority and has no reducer twin, so claiming `ok` would lie |
| 2 | `AppCommand` | ignored | ignored |
| 3 | `ApprovalRequested` | park in `pendingApprovals` (replace same id) | same, `PendingAgentApproval{approval_id, summary, requested_at_ms}` |
| 4 | `ApprovalResolved` | drop from `pendingApprovals` | same, emits nothing |
| 5 | `AgentPresence` | `setPresence({active,label,invocationId})` | same → `AgentBridgePresence` |
| 6 | `Pong` | ignored | ignored |
| 7 | `Bye` | `setLastError(reason)` | `last_error = reason`, status → `Closed` |

Outbound (`ShellToGateway`): `Hello` (tag 0, announcing `ShellKind::WgpuWeb`/`WgpuNative`),
`ShellCommandResult` (5), `Approval` (6), `Ping` (7), `Bye` (8). Tags 1–4 (`ShellState`,
`ShellStatePatch`, `Instances`, `AppFrames`) are deliberately unmodelled — they only exist for a
shell that mirrors `ShellState`.

Decision → frame: `Deny`/`Once`/`Session` emit `Approval{approval_id, decision, note: None}`, with
the request dropped from the modal in the same call (`resolveApproval`'s own pair).

## 3. Trigger table — what raises a transient notice

Every `showTransientNotice(` call site in `🏛️ShellHost/🟦️.tsx` was enumerated (lines 5350, 5506,
6344, 6747, 6827, 6947, 8093, 8115, 8626, 9332, 9347, 9364) and mapped:

| React trigger | React severity / code | wgpu trigger | Status |
|---|---|---|---|
| keybinding-unowned chord (`:8626`) | `info`, `KEYBINDING_UNOWNED_CODE` | `dispatch_app_keybinding`'s no-owner branch — was `self.error = …`, now `show_transient_notice(…, Info, KEYBINDING_UNOWNED_CODE)` | **wired** |
| viewer read-only fault (`:6747`, `:6827`, `:9332`, `:9364`) | `info`, `viewer.read-only` | `classify_dispatch_fault_notice` on any dispatch fault carrying the frozen code, via `ShellState::note_dispatch_fault` | **wired** |
| mutation rejected (`:8093`) | `fault.severity`, `mutation.rejected` | same funnel; severity `Error` (the wgpu dispatch seam returns a `String`, not a typed `Fault` with a `severity` field) | **wired, severity approximated** |
| render error (`:6947`, `:9347`) | `error` | same funnel, generic branch | **wired** |
| guest `Effect::Notify` (`:5506`) | `warning` | not wired — the wgpu shell's effect application has no `Notify` arm at all (zero hits for `Notify` in the shell file) | **gap** (§5 gap 4) |
| surface-switch busy (`:5350`), input refusal (`:6344`), degraded remote merge (`:8115`) | `info` / worst | not wired — none of the three producers exists on the wgpu side yet | **gap** (§5 gap 4) |

The funnel is `renderer/🦀️.rs`'s `FrameDeferredWork::Action` arm, which previously parked the fault
string in `shell.error` (the persistent bottom-left line that never clears) and now calls
`note_dispatch_fault`.

## 4. Anti-drift: the codec is fixture-checked, not hand-trusted

`🧵️bridge/🧫️fixtures/📨️frames.json` carries a hex encoding for every frame variant in both
directions and is what the SSOT's own `mod quick` and the `🟦️.ts` twin assert against. The wgpu
codec replays the same file (`include_str!`, so a moved fixture is a build error):

- `every_gateway_to_shell_fixture_round_trips_through_this_codec` — all 8 inbound tags decode and
  re-encode to the identical hex.
- `every_modelled_shell_to_gateway_fixture_round_trips_through_this_codec` — the 6 outbound rows this
  shell models, with the 4 snapshot frames explicitly listed as unmodelled rather than skipped
  silently.

## 5. Remaining gaps

1. **No chat transcript.** React's body is `BasicChatPanel`, an arbitrary React subtree hosted through
   `Tree`'s `emptyState` escape hatch (`📌️ChromePanels/🟦️.tsx:1376-1391`). The wgpu panel pipeline has
   no equivalent — panel content is exclusively `UiNode`/`UiTree` under the fixed-credit
   `MountedLayout` engine. This is the shell audit's own recommendation 7, which names
   `RightPanelKind::Chat` as its forcing case. The header renders the empty-transcript line so the
   panel is never blank.
2. **No socket.** `AgentBridgeState` is transport-free by design, and nothing dials the gateway yet.
   The browser half needs the `WebSocket` `web-sys` feature (not in this crate's feature list) plus a
   page-owned bridge in `🚪️host-io/🟦️.ts`; the native half needs a websocket client, which lives
   behind `semio-framework-os-services`. `ShellState::apply_agent_bridge_frame` /
   `take_agent_bridge_outbox` are the two functions such a transport has to call, and
   `note_connecting` / `note_socket_opened` / `note_socket_closed` / `reconnect_delay_ms` /
   `queue_ping` / `queue_bye` are the lifecycle it drives. Until then the overlays are reachable only
   from a simulated frame — which is exactly what the tests inject.
3. **No `ShellState` mirror.** React's hook reduces inbound `shellCommand` frames against a
   `@semio-tech/framework-os-shell` twin and republishes snapshots. The wgpu shell holds the real
   `ShellState`, so the port would have to drive the live shell instead of a mirror — a different
   packet. Today those frames are refused with an explicit fault rather than dropped.
4. **Four notice triggers have no wgpu producer** (guest `Effect::Notify`, surface-switch busy, input
   refusal, degraded remote merge). The banner is in place; those producers are separate lanes.
5. **Mutation-rejected severity is approximated.** React reads `fault.severity` off a typed `Fault`;
   the wgpu dispatch seam returns `Result<(), String>`, so the funnel classifies by frozen code and
   uses `Error`. A typed dispatch result would close this.
6. **No modal scrolling.** Requests past the `max-h-96` list cap are clipped, not scrollable (React
   has `overflow-y-auto`). The layout already computes the cap and stops emitting rows past it.

## 6. Files

New:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🔬️wgpu-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🚦️AgentPresence/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🚦️AgentPresence/🧪️tests/🔬️wgpu-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🤖️AgentApprovals/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🤖️AgentApprovals/🧪️tests/🔬️wgpu-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/💬️AgentChatPanel/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-agent-overlays/🦀️.rs`

Edited:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — mounts the
  four element modules (`🤖️AgentBridgeElements` region); routes the frame-deferred dispatch fault to
  `note_dispatch_fault`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` —
  three `ShellChromeBuildState` fields, the `🧯️TransientNoticeAndAgentOverlays` region, three frame
  phases and their steps, the tooltip-delay constant, the keybinding-unowned trigger, the test mount.

## 7. Verification

All three ran in the foreground on 2026-09-18 (logs under `🗑️generated/w1j-*.txt`).

| Command | Result | Log |
|---|---|---|
| `cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going` | **exit 0**, 0 errors, 6m17s | `w1j-native-check.txt` |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- agent_bridge:: agent_presence:: agent_approvals:: agent_overlays_tests::` | **48 passed, 0 failed** | `w1j-tests.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --keep-going` | **exit 0**, 0 errors, 3m06s | `w1j-wasm-check.txt` |

Neither check emitted a single warning naming any of the eight new/edited files — grepping both logs
for `AgentBridge|AgentApprovals|AgentPresence|AgentChatPanel|transient_notice|agent_*` returns
nothing.

### The 48 tests

| Module | Count | What they pin |
|---|---|---|
| `agent_bridge::tests` | 13 | Fixture replay both directions (all 8 inbound tags, all 5 modelled outbound variants, byte-identical); truncated/trailing/unknown-tag faults are `Err`, never a panic; `ApprovalRequested` parks and re-arrival replaces by id; each of the three decisions emits exactly one `Approval` frame and clears the request; a gateway-side `ApprovalResolved` withdraws silently; presence frames drive the presence record; welcome resets and close raises the reconnect attempt; backoff doubles and saturates at 30 s; `Hello` announces `ShellKind::WgpuNative`; an inbound `ShellCommand` is refused rather than dropped; `Bye` closes and keeps its reason |
| `agent_presence::tests` | 6 | The full 7-row (status × presence) → tone truth table; a frame moves the indicator connected→working→connected→disconnected; a dropped socket forgets the working label; four distinct theme tokens; status text precedence in `en` **and** `de`; the `data-semio-agent-presence-tone` vocabulary |
| `agent_approvals::tests` | 11 | Plain-text / rich-JSON / rich-JSON-without-rich-fields / malformed / array / empty summary parsing; unknown risk word dropped; open purely from a non-empty queue; dismissed modal re-opens on a new request; an emptied queue closes it; control-id round-trip for all three decisions and rejection of foreign ids; the end-to-end simulated-frame → modal → decision → cleared path; the modal clamps to a 360×240 viewport; three risk tokens and both locales |
| `shell::agent_overlays_tests` | 18 | Notice replaces rather than queues; clears exactly at 4000 ms and not 1 ms earlier; dismissible; a replacement restarts the clock; `error`/`fatal` share the destructive tone while `info`/`warning` differ; banner at `navbar_height + 8` and horizontally centred, close control inside it, never overflowing a 320 px viewport; the three React fault classifications incl. the German copy; a failed dispatch leaves `shell.error` untouched; a simulated frame opens the modal and paints capability/diff/requested-by/risk with three hit-testable buttons; every decision emits the right `ShellToGateway::Approval`; dismissing leaves the request parked and answers nothing; a plain-text summary still lists a change-summary line; presence frames move the shell's own tone; the chat header plan stays inside the panel; the tooltip delay is 400 ms with the threshold checked at 399/400 |

### Fleet note

The crate was under heavy contention while this packet ran — up to 28 concurrent
`cargo check -p semio-framework-os-renderer-wgpu` processes from peer packets, load average ~45, and
two runs of this packet's own check died with `exit=137` (SIGKILL/OOM) before any of them reached the
crate. The passing runs above are single, uninterrupted foreground invocations with `-j 2` and
`CARGO_INCREMENTAL=0`.
