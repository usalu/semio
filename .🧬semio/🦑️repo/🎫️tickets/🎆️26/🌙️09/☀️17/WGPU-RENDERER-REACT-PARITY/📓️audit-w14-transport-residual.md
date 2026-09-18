# 🛰️ W14 — residual runtime/transport/boot gap audit (React ⇄ wgpu), post Wave 0–13

Read-only audit. Repo: `/Users/ueli/Documents/semio`. Written 2026-09-18/19, against the tree AS IT
STANDS — several agents are editing concurrently (`git status` shows live `MM` on
`🐍️parity-interact-probe.mjs` and `📓️status.md` themselves). Every negative grep below was re-run
with `-a` (macOS `grep` misdetects these emoji-heavy UTF-8 files as binary).

**Primary focus, per the dispatch**: Wave-2 plan item **W2e — "browser directory WebSocket door +
document sync + Space Administration pane + agent bridge transport + chat transcript"** was written
into `📓️status.md`'s Wave-2 plan (2026-09-18) and **never dispatched** — no `📓️w2e-*.md` exists, no
worker report references it, and grepping the whole wgpu target tree for `WebSocket` returns **zero
hits** in any `.ts` or `.rs` file. This audit confirms that gap still stands, files it precisely, and
separately surveys what changed since the Wave-0 audits (`📓️audit-runtime-boot-input.md`,
`📓️audit-build-serve-theme.md`) so the residual list isn't reporting fixed bugs as open ones.

---

## 0. Headline: what Wave 0 called a gap that Wave 1 quietly closed

Re-verified by direct grep against the current tree, not by trusting the older reports:

| Wave-0 claim | Current truth | Evidence |
|---|---|---|
| "hub axis still has no wasm reader" (`📓️audit-runtime-boot-input.md` row 2/#33, repeated in `📓️w1d`'s own "remaining gaps" §2) | **Fixed by W1e.** `resolve_identity_env()` in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:399-401` has a `#[cfg(target_arch = "wasm32")]` arm reading `BOOT_HUB_ENV` (set by `semioWgpuSetHubEnv`, `:433-437`), and `🎞️frame-worker/🟦️.ts:543` calls it from `message.descriptor.hub`. Identity/Space-Administration now activate on the browser build when a hub axis is present. | grep `BOOT_HUB_ENV`/`semioWgpuSetHubEnv` in Shell `.rs` and `🎞️frame-worker/🟦️.ts` |
| "Space Administration is native-only... P0 missing on browser" (`📓️audit-runtime-boot-input.md` #29/#33) | **Fixed (transport) by W1e**, still **no window/pane** (see §2 below) | `📓️w1e-space-administration-wasm.md` |
| "browser wgpu shell has NO persistent preference storage at all" (found by `📓️w4a`, closed by `📓️w5a`) | **Fixed.** `HOST_STORAGE` door + boot snapshot, key census pinned against React's own constants by a standing law. | `📓️w5a-browser-prefs-persistence.md` |
| "the LLM-agent bridge does not exist on wgpu at all" (`📓️audit-runtime-boot-input.md` #25) | **Half-fixed by W1j.** Codec + overlays (approvals modal, presence dot, chat header) exist; **no socket, no transcript** (see §3). | `📓️w1j-shell-overlays.md` |
| "`prefers_dark_scheme()` always `true` in the Worker" | Fixed by W4a (`HostAppearance` published from the page) | `📓️w4a-boot-appearance-and-tour.md` |
| "chord glyphs render Ctrl+Alt instead of ⌘⌥ in the browser" | **Half-fixed by W7a** — see §4.1, a second, un-ported call site still breaks this | new finding below |

---

## 1. W2e, the dispatch target: verified absent, itemised

| Sub-item | React (file:line) | wgpu (file:line / absent) | wasm32? | Severity | Fix direction |
|---|---|---|---|---|---|
| Directory WebSocket (live event stream) | `🧰️framework/🛍️products/💻️os/🟦️.ts:4738` `DirectoryClient.streamFor` — `new WebSocket(wsUrl(), …)` at `:4772`, feeds `DirectoryHomeProjection`/presence | `📇️directory-door/🦀️.rs` — `open_ws`/`issue_socket_grant` **refuse** (the request/response mailbox this door rides cannot carry a duplex channel); `directory_door` has zero `WebSocket` references | **absent** | P0 | New door: either a second page-owned duplex bridge (`WebSocket` in `🚪️host-io/🟦️.ts`, forwarded frame-by-frame over the existing `postMessage` pipe to the Worker) or a native-style poll fallback. `📓️w1e` gap 1 names the seam exactly. |
| Document sync / collaboration backbone | React's `ArtifactHost`-equivalent document backbone (opens via `os.open-artifact`) | `🐚️Shell/…/🦀️.rs:8241` `os.open-artifact`/`os.open-artifact-with` handling is `#[cfg(not(target_arch = "wasm32"))]` (`:8240`, explicit comment: "ends in `open_document`, which needs the native `ArtifactHost`... the browser wgpu build does not link") | **absent, gated out** | P0 | Needs the browser transport for `ArtifactHost`'s own wire, not just directory REST — a materially larger packet than W1e's (W1e was pure request/response; this is a live document channel with conflict resolution). |
| Space Administration **pane** | `🛂️SpaceAdministration/🟦️.tsx` (459 lines) | `open_space_administration`/`pump_space_administration`/`shell_space_administration_controls` (`🐚️Shell/…/🦀️.rs:8248` onward) run and are transport-complete since W1e, but **nothing paints a window** — no `WindowKind::SpaceAdministration` mount, confirmed by grep (`open_space_administration` has exactly one call site, from a directory-command relay, never from a window-open action) | n/a (chrome, not wasm-specific) | P1 | Chrome-only packet now — the hard part (transport) is done. Build the window from `shell_space_administration_controls`, the same builder native already renders from (`📓️w1e` gap 3). |
| Agent bridge **transport** (the socket itself) | `🔗️AgentBridge/🟦️.tsx:509` `new WebSocket(admittedConfig.url, [...bridgeProtocols(admittedConfig)])` | `AgentBridgeState::apply_agent_bridge_frame`/`take_agent_bridge_outbox` (`🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs`) is transport-FREE by design — nothing dials the gateway; needs the wasm `web-sys` `WebSocket` feature (not in the crate's feature list today) plus a page-owned bridge, and a **separate** native transport (`semio-framework-os-services`) since native has no browser socket either | **absent on both wgpu targets** (browser AND native) | P0 | Two transports, one seam: `note_connecting`/`note_socket_opened`/`note_socket_closed`/`reconnect_delay_ms`/`queue_ping`/`queue_bye` are already the lifecycle a transport must drive (`📓️w1j` gap 2) — only the dialing code is missing. |
| Agent **chat transcript** | `💬️AgentChatPanel/🟦️.tsx` (30 lines) hosts React's `BasicChatPanel` — an arbitrary React subtree via `Tree`'s `emptyState` escape hatch | `💬️AgentChatPanel/🎯️targets/🧊️wgpu/🦀️.rs` is **72 lines total**, header + empty-state string only (verified: `grep -c transcript` → 1 hit, a comment) | n/a | P1 | The wgpu panel pipeline has no free-content escape hatch at all (every panel body is `UiNode`/`UiTree` under the fixed-credit `MountedLayout` engine) — this is a layout-engine capability gap, not a chat-specific one; scope as "a scrollable message-list `UiNode` shape" rather than "port `BasicChatPanel`". |

**Why it wasn't dispatched**: cross-referencing `📓️status.md`'s own timeline, W2e was *planned* in the
same entry (2026-09-18, "Wave 2 plan") as W2a–W2k, but the very next entry says only "W2a scene
remainder... W2f CAD spatial editor" were *dispatched*, with W2e never named again in any later entry
through W13e. It is not blocked on anything technical (W1e/W1j already did the load-bearing discovery
work above) — it appears to have been dropped from the dispatch queue while the coordinator's
attention moved to the live-boot verification track (W2i onward), which produced far more visible
"chrome doesn't paint" defects that crowded it out.

---

## 2. Full capability gap table (residual, current tree)

Severity: **P0** functional/architectural hole, **P1** real but scoped/isolated, **P2** cosmetic/hygiene, **OK** parity confirmed.

| # | Capability | React (file:line) | wgpu (file:line / absent) | wasm32 OK? | Severity | Fix direction |
|---|---|---|---|---|---|---|
| 1 | Hub directory WebSocket / live presence stream | `💻️os/🟦️.ts:4738,4772` | absent (`📇️directory-door/🦀️.rs`) | n/a | **P0** | §1 row 1 |
| 2 | Document sync backbone | native `ArtifactHost` (not this lane) | `🐚️Shell/…/🦀️.rs:8240-8243` `#[cfg(not(wasm32))]` | **no** | **P0** | §1 row 2 |
| 3 | Agent bridge socket (browser AND native) | `🔗️AgentBridge/🟦️.tsx:509` | absent both targets | **no** | **P0** | §1 row 4 |
| 4 | Agent chat transcript | `💬️AgentChatPanel/🟦️.tsx` (30 lines, `BasicChatPanel`) | `…/🎯️targets/🧊️wgpu/🦀️.rs` (72 lines, header only) | n/a | **P1** | §1 row 5 |
| 5 | Space Administration window/pane | `🛂️SpaceAdministration/🟦️.tsx` (459 lines) | operation+controls live, no window mount (`🐚️Shell/…/🦀️.rs:8248`) | **yes** (transport) | **P1** | §1 row 3 |
| 6 | **Platform detection for keybinding filtering — NEW finding, not in any prior audit** | n/a (React reads OS natively via server env, not this pattern) | `command_host_platform()` (`🐚️Shell/…/🦀️.rs:14945-14968`) still calls raw `web_sys::window()` on `wasm32` (the OLD pattern `format_keybinding_shortcut` was fixed away from at the SAME file, `:13373-13379`, via `crate::host_platform_uses_meta()`) — in the frame Worker `window()` is `None`, so `unwrap_or_default()` on `navigator.platform` always yields `""`, which falls through to `Platform::Linux`. Two live call sites still use the broken function: `handle_keyboard_async`'s command-chord matching (`:12022`) and `command_search_items` (`:15622`) | **no — silently wrong, not absent** | **P0** | `command_host_platform()` should call `host_platform_uses_meta()` (or whatever underlying door W7a built) instead of re-deriving from `web_sys::window()`. Concretely broken today: `os.toggleFullscreen` declares `Platform::MacOs → "control+meta+f"` vs `Platform::Windows/Linux → "f11"` (`:15537-15540`) — a real macOS user on the browser build sees/matches the F11 binding, not ⌃⌘F, in the palette and in ANY app-declared platform-scoped keybinding (not just fullscreen, since the same broken function gates `resolved_commands()` filtering generally). Note the *fullscreen key itself* still works by luck — `handle_keyboard_async` has a separate hardcoded `ctrl && meta` check at `:12011` that bypasses platform filtering entirely — but every OTHER platform-scoped command/keybinding does not have that luck. |
| 7 | Tab-hidden / visibility throttling — **NEW finding** | none needed (DOM naturally throttles via rAF/timers; React has no explicit `visibilitychange` in `ShellHost/🟦️.tsx` either) | wgpu's trunk boot (`🚀️browser-boot/🟦️.ts`) has **no `visibilitychange` listener anywhere** (confirmed: zero hits across the whole `.ts` target tree, only prose mentions of "hidden pane" in `⏱️turn-budget/🟦️.ts` and `🐚️plugin-bridge/🟦️.ts` about *why* wall-clock budgets get unreliable when hidden, no code that reacts to it) | n/a | **P2** | Not a functional break (the continuous per-frame draw loop just keeps running, throttled only by the browser's own rAF/timer clamp) but it means wgpu burns strictly more idle CPU/GPU in a backgrounded tab than React's on-demand repaint model, and the turn-budget code's own comments already document the resulting measurement noise. Packet: pause/resume the render loop (or drop to a slow poll) on `document.visibilitychange`, mirroring the throttling the comments already reason about. |
| 8 | `?mode=` has no React reader | absent (`🧑‍💻dev/🔗️boot-query/🟦️.ts` has `resolveBootQueryAppRole`/`resolveBootQueryExampleId`, no mode fn; `FrameworkOsBootOptions` has no `appMode`) | `?mode=` fully wired (`🧭️boot-descriptor/🟦️.ts`) | **yes (wgpu-only axis)** | **P2** | Unchanged since W1d, allowlisted intentionally. React-side packet, not wgpu's. |
| 9 | Brand registry | `resolveShellBrandById` — windowTitle/ephemeral/replayIntroductionOnLoad/brand-level locks | `boot_brand_id()` (`🧊️renderer/🦀️.rs:15769`) carries only an id, no table | **yes (id only)** | **P1** | Unchanged since W1d gap 1. Concrete visible cost: the tour's persisted-seen key omits the brand prefix on branded playgrounds (`aggregator`, confirmed by `📓️w5a` §2) — a real, demonstrated divergence, not theoretical. |
| 10 | OS-level file drag-and-drop onto canvas | `🎯️targets/⚛️react/🟦️.tsx:29` (`DragEvent` import; exact React handler not fully traced) | absent — zero `dragover`/`dragenter`/`drop`/`DragEvent`/`dataTransfer` hits in `🚀️browser-boot/🟦️.ts` (re-verified this pass) | n/a | **P1** | Unchanged since Wave 0 audit packet 3. Only the picker-based `request-file-open` works. |
| 11 | Downloads (export → `DownloadMediaExport`) | DOM `<a download>` | `🚪️host-io/🟦️.ts:25,174` `download-media-export` op, mounted off-screen `<a>` + delayed `revokeObjectURL` | **yes** | OK | Confirmed still wired, unaffected by later waves. |
| 12 | File open (picker) | `<input type=file>` | `🚪️host-io/🟦️.ts` off-screen `<input type=file>`, `dataUrl`/text modes | **yes** | OK | — |
| 13 | Clipboard copy/cut/paste (text) | DOM `ClipboardEvent` | native `arboard`, browser `navigator.clipboard.writeText/readText` (`🖱️ui/🎯️targets/🧊️wgpu/🏃️host/🦀️.rs:148-171,202-214`) | **yes** | OK | Text-only both sides; rich/file clipboard unverified either side. |
| 14 | IME composition | DOM `compositionstart/update/end` | `🚀️browser-boot/🟦️.ts` `wireInput` — `compositionstart/update/end` wired | **yes** | OK | — |
| 15 | Pointer input incl. touch/pen | DOM Pointer Events unify mouse/touch/pen | `wireInput` reads `event.pointerType` (`:338`), canvas has `touch-action:none` (`:87`) so the browser never steals the gesture | **yes** | OK | No dedicated pinch/rotate gesture handling on EITHER renderer (neither has `gesturestart`/multi-touch fan-out); both rely on browser-translated wheel+ctrlKey for trackpad pinch. Not a wgpu-specific gap. |
| 16 | Resize / DPI | `ResizeObserver`, `devicePixelRatio` | `🚀️browser-boot/🟦️.ts:330,409,453,481-486,495` `ResizeObserver` + a SEPARATE `matchMedia(resolution: …dppx)` density watcher (a `ResizeObserver` never fires on a pure DPI change) | **yes** | OK | Ahead of naive parity — the density-only-change edge case is explicitly handled. |
| 17 | Readiness / fault DOM beacons | React sets `data-semio-os-ready`/`data-semio-os-error` on `documentElement` | `🧭️boot-descriptor/🟦️.ts:391-403` `wgpuReadinessBeacon`, called from `🚀️browser-boot/🟦️.ts:403,538` | **yes** | OK | Fixed since the Wave-0 audit found it missing (`📓️audit-visual-parity-puzzle3d.md` §7 item 8 → closed by W6a). |
| 18 | Diagnostics/introspection parity (`__semioInputLedger` vs `dumpChrome`) | `🏛️ShellHost/🟦️.tsx:4248` `Object.defineProperty(globalThis, "__semioInputLedger", …)` — synchronous getter, always live | `🚀️browser-boot/🟦️.ts:100-108` `window.semioWgpuIntrospection` — `dumpStructure/dumpFrameStats/dumpAccessibility/dumpMeshStats/dumpChrome`, each an ASYNC probe round-trip to the Worker | **yes** | OK (shape differs, function equivalent) | Documented and probed already (`📓️w5b-interaction-parity-probe.md`). Only real asymmetry: React's ledger is a synchronous property read, wgpu's is an async round-trip — fine for probes, would matter for a synchronous DevTools-style inspector if one is ever built. |
| 19 | Undo/redo direct shortcut | `🏛️ShellHost/🟦️.tsx:8661-8663` raw `mod+z`/`mod+shift+z` listener | absent in `handle_keyboard_async` / `build_os_commands()` | n/a | **P1** | Unchanged since Wave 0 packet 1. Not re-verified as fixed by any Wave 1–13 report. |
| 20 | Arg-carrying Plugin/App/Mode command-palette entries | executes via `handleAction` | `command_search_items` (`🐚️Shell/…/🦀️.rs:~15622` region) still self-documents "no `handle_command` RPC… listed for completeness" | n/a | **P1** | Unchanged since Wave 0 packet 7. |
| 21 | `windowPanes`/`namedLayouts` persistence readers | `WindowPaneStateStore`/`NamedLayoutStore` | carried in the storage door's `semio.os.config` document, **no wgpu reader** (`📓️w5a` gap 2) | **yes (carried, unconsumed)** | **P1** | New since Wave 0 (found by W5a) — a saved named layout/window-pane state now survives a wgpu round-trip on disk but is never applied. |
| 22 | Space Administration presence stream inside the admin doc | `🌎️hub` event-page bootstrap | `DirectoryHomeProjection`/`ShellDirectoryRunner` stay native-only (same root cause as row 1) | **no** | **P0** (subset of row 1) | — |
| 23 | `host_storage_remove` production caller | `OsShellConfig.reset()` (ephemeral brand only) | door supports `remove`, no live caller (brand-less on wgpu, row 9) | n/a | **P2** | Not reachable until row 9 lands. |
| 24 | Service worker / offline | none found in `ShellHost/🟦️.tsx` either | none | n/a | OK (parity: neither side has one) | — |
| 25 | `window.print()` / print-to-PDF | not in `ShellHost` (plugin-owned, e.g. Draw's own `exportDocument`) | not in Shell wgpu either | n/a | OK (parity: both push this to the owning plugin, per `project-draw-pdf-export-path.md`) | — |
| 26 | Deep link / URL space-document routing | `useUIHistory` push on space/document nav (`🏛️ShellHost/🟦️.tsx:3487`) | wgpu never writes the URL at all (confirmed by W1d: "no per-switch URL writeback... adding one to wgpu alone would have been a divergence") | n/a | **P1** (structural, tied to row 2) | Meaningless until document sync (row 2) exists — a wgpu tab cannot open "the same document" via URL today regardless of routing, since there is no native-independent document-open path on wasm32. |

---

## 3. Detail: the duplicate platform-detection bug (new this pass)

Two functions answer "what platform am I on" in the same file
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`), and only one was fixed:

- `crate::host_platform_uses_meta()` (used by `format_keybinding_shortcut`, `:13373-13379`) — **fixed
  by W7a**, reads a value the page/`winit` publishes through a real door, because the code comment at
  `:13370-13373` explicitly records the `web_sys::window()`-in-a-Worker defect and its fix.
- `command_host_platform()` (`:14945-14968`) — **the same defect, unfixed**, still does
  `web_sys::window().and_then(|w| … navigator.platform …)` inside the `#[cfg(target_arch = "wasm32")]`
  arm. Two live callers: `handle_keyboard_async` (`:12022`, gates which declared keybinding's
  `platform` field must match before a chord dispatches) and `command_search_items`
  (`:15622`, gates which chords are shown/described in the search palette).

Demonstrated concrete impact: `os.toggleFullscreen`'s own definition
(`🐚️Shell/…/🦀️.rs:15537-15540`) declares `Platform::Windows→"f11"`, `Platform::Linux→"f11"`,
`Platform::MacOs→"control+meta+f"`. Since `command_host_platform()` always answers `Linux` on the
browser build (`window()` is `None` in the frame Worker, `unwrap_or_default()` → empty string → falls
through to the `else` arm), a real macOS user running the browser wgpu build:
- sees "F11" in the command palette's description for Toggle Full Screen, not "⌃⌘F";
- would fail to dispatch ANY OTHER app-declared, Mac-scoped keybinding through the general chord-match
  path in `handle_keyboard_async` (only `os.toggleFullscreen` happens to survive, because it has an
  UNRELATED hardcoded `ctrl && meta` short-circuit at `:12011` that bypasses platform filtering
  entirely — that is luck, not a fix).

This was never flagged in `📓️audit-runtime-boot-input.md`, `📓️w1d`, `📓️w4a`, or `📓️w7a` — W7a's own
scope was explicitly the tour/glyph lane and it fixed the ONE call site it was looking at
(`format_keybinding_shortcut`) without a repo-wide grep for the same defective pattern.

---

## 4. Honest gaps in THIS audit

- Rich/file clipboard (as opposed to plain text) was not independently re-verified on either
  renderer — repeating the Wave-0 audit's own caveat, not newly checked.
- The exact React "OS-level file drop" handler location was not traced to a file:line (Wave 0 already
  flagged this as unconfirmed; this pass did not attempt it either — the absence on wgpu's side is
  independently confirmed regardless of where React's handler lives).
- `SpaceAdministration`'s browser transport (W1e) compiled and unit-tested but was never exercised in
  a live boot by any later wave's live-verification packets (W2i onward tracked puzzle3d/generation3d/
  gis2d chrome, not Space Administration specifically) — "transport is live-correct" is asserted by
  W1e's own fixture tests, not by a browser probe in this pass.
- Search fuzzy-ranking algorithm parity (weights/threshold) — still not compared, unchanged caveat
  from Wave 0.
- This audit did not run any build, serve, or probe (per instructions) — every finding above is a
  static-source claim, cross-checked by direct grep against the CURRENT tree, not a live repro.

---

## Top 10 packets to dispatch

1. **Dispatch W2e's agent-bridge socket half first** (row 3): wire the WebSocket dial on both browser
   (`web-sys` WebSocket feature + `🚪️host-io/🟦️.ts` bridge) and native
   (`semio-framework-os-services` client) — `AgentBridgeState`'s lifecycle hooks already exist and are
   tested; only the dial is missing. Smallest-diff, highest-leverage half of W2e.
2. **Fix `command_host_platform()`'s wasm32 arm** (row 6) — one-line redirect to
   `host_platform_uses_meta()`/its underlying door instead of re-deriving from `web_sys::window()`.
   Trivial diff, silent functional bug on every macOS browser user today.
3. **Build the Space Administration window/pane** (row 5) — the transport (W1e) and controls builder
   are done; this is a chrome-only packet reusing the native rendering path.
4. **Dispatch W2e's directory WebSocket door** (row 1) — second half of W2e, needed for live presence/
   event-page; larger than #1 because the request/response mailbox pattern the other doors use cannot
   carry a duplex stream, so this needs new plumbing shape.
5. **Wire the undo/redo direct chord** (row 19) — Wave-0 packet 1, still open, one-line-class fix
   named with exact acceptance criteria in `📓️audit-runtime-boot-input.md`.
6. **Wire OS-level file drag-and-drop** (row 10) — Wave-0 packet 3, still open, exact seam named
   (`wireInput` in `🚀️browser-boot/🟦️.ts`).
7. **Give `windowPanes`/`namedLayouts` a wgpu reader** (row 21) — data already round-trips through the
   storage door (W5a), only the consumer is missing; low-risk, additive.
8. **Port the brand registry** (row 9) — closes the tour-key mismatch W5a already demonstrated
   concretely on the `aggregator` playground, plus unblocks `ephemeral`/`replayIntroductionOnLoad`.
9. **Either wire or hide arg-carrying Plugin/App/Mode palette commands** (row 20) — Wave-0 packet 7,
   self-documented dead route in the code, currently presents live-looking options that silently
   no-op.
10. **Scope and start the agent chat transcript** (row 4) — largest single item, correctly deferred
    last: needs a "free-content" `UiNode` escape hatch in the layout engine before a transcript can be
    painted at all; should be scoped as that layout capability, not as a chat-specific port.
