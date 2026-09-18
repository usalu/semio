# wgpu ⇄ React parity audit — runtime, boot, plugin bridge, action dispatch, input

Lane: RUNTIME / BOOT / PLUGIN BRIDGE / ACTION DISPATCH / INPUT. Read-only audit, 2026-09-17.
Companion Wave-0 audits (not duplicated here): shell/window system, interpreter/element coverage,
scenes/engine canvas, build/serve/theme health — see `📓️status.md`.

Methodology: direct file reads + grep across the React reference elements (`🧰️framework/🛍️products/
💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/`) and the wgpu target tree (`…/🎯️targets/🧊️wgpu/` for
both the os-renderer and the `🐚️Shell`/`🗣️Interpreter`/`🌉️ProgramBridge` element wgpu targets, plus
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/`). macOS `grep` misdetects these UTF-8/emoji-heavy files
as binary and silently returns nothing without `-a` — every negative grep result below was re-run with
`-a` before being trusted. Some findings draw on already-verified project memory (cited inline) rather
than re-deriving them from scratch; those are marked "confirmed" rather than "found here."

## Executive summary

- **Boot is architecturally split by design, not by omission.** The shared dev harness
  `🧑‍💻dev/🟦️.ts` only boots React (`if (renderer !== "wgpu")`); wgpu is served by its own Vite config
  (`…/🎯️targets/🧊️wgpu/🌐️server/🎚️config/🟦️.ts`) and its own single-mount trunk app
  (`🚀️browser-boot/🟦️.ts`). This is intentional (separate ports/pipelines), not a wiring gap. What IS a
  gap: the three wgpu boot entry points (browser trunk app, library `bootFrameworkOsWgpu`, native
  `--plugin` CLI) support **three different subsets** of the boot-axis vocabulary (plugin/role/mode/
  example/hub/appId/locks/defaults/brand) that React's single `bootFrameworkOs` supports as one set.
- **Space Administration is native-only.** Every function (`open_space_administration`,
  `pump_space_administration`, labels, controls, status) in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` is gated
  `#[cfg(not(target_arch = "wasm32"))]` with no wasm32 counterpart. The feature is entirely absent from
  the browser build of wgpu.
- **The LLM-agent bridge (AgentBridge/AgentPresence/AgentChatPanel/AgentApprovals) does not exist on
  wgpu at all** — zero references anywhere in the wgpu target tree.
- **Undo/redo has no keyboard shortcut on wgpu.** React binds `mod+z`/`mod+shift+z` directly
  (`🏛️ShellHost/🟦️.tsx:8661`). wgpu's `handle_keyboard_async` has no `"z"` chord check anywhere, and
  `build_os_commands()` declares no `os.undo`/`os.redo` command; undo/redo/commitCheckpoint are reachable
  only by opening the command palette and typing.
- **OS-level file drag-and-drop (dropping a file from the Finder/Explorer onto the canvas) is not
  wired** in the browser boot's input layer, unlike internal widget drag-reorder (tree rows, palette
  items), which IS wired end-to-end through `UiCommand::DropCommitted`/`apply_drop_committed`.
- Several things are in **better shape than expected**: clipboard (copy/cut/paste, native `arboard` +
  browser `navigator.clipboard`), file open/download (fixed after being silently broken — Worker has no
  `document`), app-declared keybinding dispatch (P4, with window-ownership resolution and a loud refusal
  message), accessibility mirror (ARIA subtree over the canvas), and job-progress presentation (a
  dedicated `JobProgressPresentationBridge` with reserve/publish/lease states, arguably more explicit
  than React's coalesced-poll fix).

---

## Gap table

| # | Area | React (file:line) | wgpu (file:line / absent) | Status |
|---|---|---|---|---|
| 1 | Shared dev harness boots React only | `🧑‍💻dev/🟦️.ts:56` (`if (renderer !== "wgpu")`, no `else`) | n/a — wgpu has its own server: `…/🎯️targets/🧊️wgpu/🌐️server/🎚️config/🟦️.ts` | OK (by design, not a gap — see note) |
| 2 | Browser boot `?plugin=/&role=/&mode=/&example=/&hub=/&user=/&dataDir=` | `🧑‍💻dev/🟦️.ts:17-52` (`resolveBootQueryAppRole`, `resolveBootQueryExampleId`, `PLAYGROUND_SESSION`) | `🚀️browser-boot/🟦️.ts:46-57` (`bootDescriptor()`) — same axes, all present | OK |
| 3 | Library/multi-mount boot options | `🐚️Shell/🟦️.tsx:1218` `bootFrameworkOs(options: FrameworkOsBootOptions)` — `plugin, plugins, surfaceSessionFactories, appId, appRole, locks, defaults, brand` | `🎬️renderer-boot/🟦️.ts:9-13` `FrameworkOsWgpuBootOptions` — only `rootId, plugin, plugins, rendererModuleUrl` | **P1 divergent** — no `appId`/`appRole`/`locks`/`defaults`(example)/`brand` on the embeddable wgpu boot |
| 4 | Native CLI boot options | n/a (React has no native binary) | `⌨️native-entrypoint/🦀️.rs:89` — only `--plugin` (default `"studio"`); no `--example`, `--role`, `--mode`, `--hub`, `--user`, `--dataDir` | **P1 missing** — native wgpu cannot select an example doc, viewer/editor role, or hub connection at launch |
| 5 | Ready marker / boot liveness | React: app mount + first render (no single documented DOM flag found in this lane; owned by shell/window-system audit) | `🚀️browser-boot/🟦️.ts:429-437` `onReady` — removes status el, attaches `semioWgpuIntrospection`, wires input, focuses canvas; `🫀️boot-liveness/🟦️.ts` `describeBrowserBootPhase` | OK — wgpu's marker is arguably more instrumented (phase/silent-time/turn-overrun reporting in `fallbackLines`, `🚀️browser-boot/🟦️.ts:257-288`) |
| 6 | Boot fault surfacing | React: `console.error` (grep `[DEBUG]` — ~5 hits in `ShellHost/🟦️.tsx` boot paths) | `🚀️browser-boot/🟦️.ts:290-303` `renderFault` — `console.error("wgpu renderer fault: …")` + full-screen `role="alert"` banner with turn/step overrun ledger; `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3860,3972,4046` `[DEBUG] wgpu shell boot: …/wgpu-shell boot: …` | OK — same `[DEBUG]` convention, wgpu's fault banner is more detailed |
| 7 | Accessibility mirror for the canvas | React: DOM is the accessibility tree (Interpreter writes `aria-*` per `UiNodeRecord`) | `🚀️browser-boot/🟦️.ts:96-240` `WGPU_ACCESSIBILITY_MIRROR_ID`, 400ms-coalesced `dumpAccessibility` pull, full node attribute set (`role/label/live/shortcut/disabled/value*/focused/busy`) | OK |
| 8 | Action dispatch: app-declared keybindings (P4) | `🏛️ShellHost/🟦️.tsx` app keybinding routing (dispatch via `onAction`) | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:9358-9369` `match_app_keybinding`/`dispatch_app_keybinding`; window-target resolution `resolve_keybinding_target_window_v1` (`🦀️.rs:9384-9404`); loud refusal `keybinding_unowned_text_v1` when no window owns the verb | OK — was previously **dead code** per in-code history comment (`🦀️.rs:14018-14026`, "MAJOR FINDING" in the superseded `report-w3-shell-input-cutover.md`), fixed under "w2-input-wiring" |
| 9 | Command palette: arg-carrying Plugin/App/Mode commands | `🏛️ShellHost/🟦️.tsx` search dispatch (commands execute through `handleAction`) | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11215-11221` doc comment: "there is no `ArtifactApp::handle_command` RPC wired on the plugin bridge yet (only `handle_action` exists…)"; such commands are listed in `command_search_items` "for completeness" but **cannot actually execute** | **P1 broken** — hard-dead route, self-documented in the code |
| 10 | Undo/redo keyboard shortcut | `🏛️ShellHost/🟦️.tsx:8661-8663` — raw `keydown` listener, `mod+z`/`mod+shift+z`, bypasses the command registry | absent — `handle_keyboard_async` (`🦀️.rs:9260-9380`) has no `"z"` check; `build_os_commands()` (`🦀️.rs:11149-11197`) declares no `os.undo`/`os.redo` | **P1 missing** — undo/redo reachable only via command palette (`🦀️.rs:8907-8918`, `"studio.{undo,redo,commitCheckpoint}"`) or the History panel button, never a direct chord |
| 11 | Undo/redo dispatch mechanism (once reached) | `🏛️ShellHost/🟦️.tsx:6655,7085-7088` — generic `{controllerId, action: "undo"/"redo"}` dispatch | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12072-12074,12383-12384` — same convention, generic action dispatch against `session.app.controller_id`/host controller | OK once triggered |
| 12 | Per-window active utility / view-state base | `project-action-view-state-needs-window-instances.md`, `project-active-utility-is-per-window-in-react-host.md` (confirmed via memory) — every `handleAction` context ships `windowInstances`+`activeUtilityByWindowId`; `ViewModel.active_utility_by_window_id` keyed by window instance id, flat field mirrors only the ACTIVE window | shared kernel type `ViewModel`/`active_utility_by_window_id` consumed by both renderers (`semio_framework::kernel`) — wgpu Shell reads `self.active_utility_by_window` (`🦀️.rs:9351-9353` Escape-deactivate) keyed the same way | OK — same shared model, same trap class applies to any NEW wgpu dispatch site that forgets to project it (see element-coverage audit for per-site verification) |
| 13 | Panel actions dispatch in the ACTIVE window's context | `project-panel-actions-dispatch-in-active-window.md` (confirmed) — inspector/keybound verbs must be declared app-level or refused ("window kind X does not own action Y") | Same `try_build_definition`/ownership model is shared Rust (`semio_framework::manifest`), consumed identically by `resolved_commands()`/`dispatch_app_keybinding` on wgpu (`🦀️.rs:11205-11212`, `9384`) | OK — same shared rule, same failure mode; not renderer-specific |
| 14 | Engagement/config amend coalescing (64-edit ledger) | Plugin-side (`Emit::amend_config(…, coalesce_key)`), shared guest code, not renderer-specific (confirmed via `project-engagement-history-coalescing-and-64-edit-ledger.md`) | Same guest/plugin code runs under both renderers via `handle_action`/`ProgramBridgeEntry` — no renderer-side ledger-coalescing logic expected or found | OK / not applicable to this lane — a renderer bug here would be a plugin bug, not a wgpu-vs-React divergence |
| 15 | Spawned/background job progress → UI refresh | `🔌️PluginRuntime/🟦️.tsx` `subscribeSpawnedJobProgress` (coalesced 120ms/job) + ShellHost full refresh (confirmed via `project-react-host-never-refreshes-on-spawned-job-progress.md`, fixed 2026-09-16 only) | `🧊️renderer/🦀️.rs:3132-3340` `JobProgressPresentationBridge` — explicit `reserve/publish/cancel/take/return_lease/presented/release_presented` state machine per `JobProgressIdentity`, backed by shared `semio_framework::kernel::JobProgress*` types; wgpu's continuous per-frame draw loop repaints regardless, sidestepping the "nothing re-renders" failure class React had | OK — architecturally different (frame-driven vs on-demand), wgpu's own mechanism looks more explicit; not independently re-verified against a live repro in this pass |
| 16 | Kernel mailbox coalescing (`Envelope.coalesce`) | n/a (TS mailbox, different mechanism: `enqueueTurn(..., {coalesce})` in `🎠️kernel/🟦️.ts:2402`) | `🎭️actor/🦀️.rs:2851,3173-3175` shared kernel `Envelope.coalesce: Option<CoalesceKey>` ("200 stale mouse-moves must never queue") — the 3 wgpu call sites that construct bare `Envelope{…, coalesce: None, …}` (`🧊️renderer/🦀️.rs:7423,7432,8307`) are **inside `#[cfg(test)]` bench/harness code** (`terra-bench-instrument`), not production dispatch | OK — initial read as a P0 was a false positive; re-verified as test-only, corrected here |
| 17 | Keyboard: reserved shell chords / content-focus routing | React `SHELL_KEYBINDINGS`, `reservedShellChordsV1` | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10862-10875` `reserved_shell_chords_v1`/`chord_carries_accelerator_v1` — explicitly named as "twin of React's `reservedShellChordsV1`/`chordCarriesAcceleratorV1`"; content-focus routing at `🦀️.rs:9330-9349` (`content_has_focus`, routes real content keys to `interpreter::dispatch_ui_event` ahead of chrome shortcuts) | OK |
| 18 | Pointer input: pointermove/down/up, wheel, key, IME, paste | React: DOM listeners on the relevant host element | `🚀️browser-boot/🟦️.ts:307-380` `wireInput` — pointermove/down/up, wheel (`preventDefault`, non-passive), keydown/up, `compositionstart/update/end` (IME), `paste` (clipboard text only, 16-item cap) | OK |
| 19 | Wheel/zoom coalescing | Not throttled at the DOM layer on React either (per `project-hover-latency-anatomy-and-fixes.md`, latency was host/guest-turn-side, not input-rate) | `⏱️turn-budget` module + `🧊️renderer/🦀️.rs:10903` "how many distinct wheel POINTS one frame may owe before the stream coalesces into its newest" — wgpu explicitly buffers/coalesces wheel deltas per frame | OK — wgpu is arguably ahead here |
| 20 | Drag/drop: internal widget drag (tree rows, palette items, block-list reorder) | React: `catalogueTreeDragController`, `windowTemplatePaletteTreeDragController`, `useUiDriverDragSurface`, `useNativeDragArm`, `DragHandle`, `parsePuzzle3dCatalogueDragPayload`, world catalogue drop preview (`🎯️targets/⚛️react/🟦️.tsx:60,82,216,261,301-303,1049-1237`) | `🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:831-833,1489-1490` `UiCommand::DropCommitted{window_id,source,target,payload}`/`DropCancelled` produced by the event router's own drag-state tracking; consumed by `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:456-497` `apply_drop_committed` (dispatches an `ActionDescriptor`, matching React's `{...payload, targetId, dropPosition}` shape per the superseded ticket's `report-w3-clipboard-dnd.md`); `DropCancelled` is an intentional no-op mirroring React's own index.tsx | OK for the command-dispatch plumbing — **not independently verified per-surface** (tree/palette/world-drop-preview) whether every React drag SOURCE has a wgpu widget-side counterpart that actually starts a drag; that's element-coverage-audit territory |
| 21 | Drag/drop: OS-level file drop (drag a file from Finder onto the canvas) | React: `type DragEvent` import + drag payload plumbing at `🎯️targets/⚛️react/🟦️.tsx:29` (world catalogue/tree internal drag; a literal `<input type=file>`/native OS drop path was not separately confirmed in this pass) | absent — `grep -a "dragover\|dragenter\|'drop'\|DragEvent"` across the entire wgpu target tree returns **zero hits** outside a generated statechart's `"drop"` trigger enum string; `🚀️browser-boot/🟦️.ts`'s `wireInput` has no `dragover`/`drop` listeners | **P1 missing** — no OS-level file-drop import exists on wgpu; only the picker-based `request-file-open` (`🚪️host-io/🟦️.ts:61-101`) works |
| 22 | Clipboard copy/cut/paste | React: DOM `ClipboardEvent`/execCommand-era APIs (not traced in this lane) | Native: `🖱️ui/🎯️targets/🧊️wgpu/🏃️host/🦀️.rs:148-171` `arboard`; Browser: `🦀️.rs:202-214` `navigator.clipboard.writeText/readText`; command types `⚡️events/🦀️.rs:834-839` `ClipboardCopyRequested/CutRequested/PasteRequested`, chord routing `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7100` ("`c`/`x`/`v` clipboard chords reach `route_edit_key`") | OK — text-only both sides as far as verified; rich/file clipboard not checked |
| 23 | File open / download | React: standard `<input type=file>`/`<a download>` in DOM (owned by the app, not a special renderer concern) | `🚪️host-io/🟦️.ts` — `createWgpuPageHostIo`; **was previously silently broken**: "the shell's own browser halves... answered `web_sys::window()` with `None`... Export produced bytes and handed them to nobody" (`🚪️host-io/🟦️.ts:2-17`); fixed via one page-owned `semioWgpuHostIo` global bridged into the frame Worker | OK (post-fix) — download via mounted off-screen `<a>` + delayed `revokeObjectURL` (avoids the Chromium same-task-revoke race), open via off-screen `<input type=file>` with `dataUrl`/text read modes |
| 24 | Presence (human hub/document collaborators) | `🚦️AgentPresence/🟦️.tsx` is agent presence, NOT document presence — document collaborator presence lives in shell/hub sync plumbing, not traced here | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:513-526` `presence_peer_rows_for_surface`, `PresencePeerRow`, `PresenceRole::Author/Spectator`, `store_sync::PresencePeer`; `🦀️.rs:2776-2784` shell-local roster from `ArtifactEvent::Presence`, deliberately not folded into the shared `ViewModel` | OK |
| 25 | Agent bridge / agent presence / agent chat / agent approvals (LLM-first-OS MCP gateway) | `🔗️AgentBridge/🟦️.tsx` (476), `🚦️AgentPresence/🟦️.tsx` (48), `💬️AgentChatPanel/🟦️.tsx` (30), `🤖️AgentApprovals/🟦️.tsx` (160) — ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` | absent — `grep -a "approvalRequested\|resolveApproval\|AgentBridge\|🌉️mcp"` across the whole wgpu target tree returns **zero hits** | **P1 missing (whole feature)** — the only "approval"-shaped code on wgpu is an unrelated `GisMapInferenceApprovalRequestV1` (map-inference feature, `🦀️.rs:1500-1501`), not agent approvals |
| 26 | Task manager (background actor list, suspend/resume/cancel) | `🧵️TaskManager/🟦️.tsx:1-27` — explicitly documents it is **not yet mounted as a real window on React either** ("needs a window-kind registration and a ShellHost mount — both registrar-only"); renders via the generic `TableScene`/`SurfaceKind::Table` shape so it's pre-built to dual-render | wgpu's `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` already dispatches `SurfaceKind::Table` generically (cited by the React file's own header comment as a studied dependency); native suspend/resume/cancel land as real `Payload::Suspend/Resume/Cancel` sends per the React header, but are "unreachable... no live `Kernel` thread on native yet" | OK / shared limitation — not a wgpu-vs-React divergence, both sides are gated on the same unmounted-window + no-native-Kernel-thread gaps; `panel.spawned_apps`/`SpawnedAppEntry` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:717-738`) is a **different** concept (nested spawned plugin APP instances, an app-switcher list), not the actor task manager — do not conflate |
| 27 | Search: command palette | `🔎️ShellSearch/🟦️.tsx:16-24` `UISearch` — fuzzy-ranked (`rankFuzzyItems`, weighted fields, threshold 0.4) over an arbitrary `UISearchItem[]` | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11149-11260` `build_os_commands`/`resolved_commands`/`command_search_items` — os/plugin/app/mode commands, window kinds, keybindings, undo/redo/commitCheckpoint, `OverlayState::Search` | OK for command coverage; fuzzy-ranking ALGORITHM parity (weights/threshold) not independently verified |
| 28 | Search: find-in-document-content | `🔎️ShellSearch/🟦️.tsx` `UIFind`/`UIFindProvider`/`useUIFind` — windows register `UIFindItem[]` | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` `OverlayState::Find`, `find_query`, `activate_find_item` (referenced at `🦀️.rs:9013,9273-9276`) | OK — same two-surface split exists on both sides |
| 29 | Space administration (rename/delete/invite/permissions/members) | `🛂️SpaceAdministration/🟦️.tsx` (459 lines) | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1839-2200,6150-6220` — full parallel implementation: `ShellSpaceAdministrationOperationV1`, member/invite rows, bilingual label table (`shell_space_administration_label`, one-for-one string match per its own comment), controls builder, `open/close/pump/acknowledge_capability` — **every single one of these is `#[cfg(not(target_arch = "wasm32"))]`** | **P0 missing on browser** — Space Administration exists only in the native wgpu build; zero wasm32-gated counterpart found anywhere in the file (`grep -a 'cfg(target_arch = "wasm32")'` near these symbols returns nothing) |
| 30 | Preferences / i18n / terminology / locale | React: `active_terminologies`-style plumbing across elements (not separately traced) | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3484` `active_locale`, `3489` `active_terminology`, `5137` `active_terminologies` — 64 call sites across the file; custom theme labels resolve through the same locale/terminology pair (`🦀️.rs:11186-11192`) | OK (breadth-verified, not line-by-line) |
| 31 | Fault/error console convention | `[DEBUG]` prefix, ~5 sites in `🏛️ShellHost/🟦️.tsx` boot paths | Same `[DEBUG]` prefix convention throughout `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`debug_log`, `4087`) and a distinct `console.error("wgpu renderer fault: …")` for the browser fault banner (`🚀️browser-boot/🟦️.ts:295`) | OK |
| 32 | Native (winit) vs browser (worker) split — clipboard | n/a | Both implemented, cleanly split behind a `host`-region: native `arboard` (`🏃️host/🦀️.rs:148-171`), wasm `navigator.clipboard` (`🦀️.rs:202-214`) | OK — legitimate, necessary split |
| 33 | Native vs browser split — Space Administration | n/a | Native-only (`directory_client`, a native networking client never ported to the Worker model) — see #29 | **P0** — not a legitimate split, a missing half |
| 34 | Native vs browser split — boot options | n/a | Browser (`🚀️browser-boot`) reads role/mode/example/hub from the URL; native CLI (`⌨️native-entrypoint`) reads only `--plugin` | **P1** — the two wgpu variants disagree with each other, not just with React |

**Hard-dead routes found (explicit, self-documented in code comments, not inferred):**
- `command_search_items` for arg-carrying Plugin/App/Mode-scope commands (#9) — the comment says outright there is no RPC to execute them; they render in the palette "for completeness" only.
- Prior to the "w2-input-wiring" fix (already landed, cited for context, not a live gap): `handle_keyboard_async` was entirely dead code, meaning P4 app-keybinding dispatch, P5 idle-Escape-deactivate-utility, and committing a focused `Input`'s typed text via Enter/Escape never fired at all. Confirmed fixed as of the code read in this pass (`🦀️.rs:14018-14026`).

---

## Boot sequence detail

React and wgpu do **not** share a boot entry point, and are not meant to: `🧑‍💻dev/🟦️.ts` (the shared
os-dev harness) resolves `renderer` from `VITE_SEMIO_RENDERER`/`SEMIO_RENDERER` and only calls
`bootFrameworkOs` when it is not `"wgpu"` — there is no `else` branch (`🧑‍💻dev/🟦️.ts:56-65`). wgpu is
served by a dedicated Vite config (`…/🎯️targets/🧊️wgpu/🌐️server/🎚️config/🟦️.ts`) keyed off
`SEMIO_PLUGIN`/`PLAYGROUND_BUILD_TARGETS`, mounting the single-mount trunk app
`🚀️browser-boot/🟦️.ts`. That app:
1. Waits for `DOMContentLoaded`.
2. Reads `bootDescriptor()` from `window.location.search` (`?plugin=&role=&mode=&example=&hub=&user=&dataDir=`), bounded at 8192 chars with a named overflow error.
3. Creates the canvas (`#semio-wgpu-canvas`, fixed id — deliberately single-mount, unlike the library path), transfers it to a dedicated frame Worker via `OffscreenCanvas`.
4. Boots `BrowserFrameTransport` with the renderer wasm module + wasm binary URLs, hands it `hostIo`, `onProgress`, `onUiTurn`, `onReady`, `onDirectives`, `onFault` callbacks.
5. On `onReady`: removes the status line, attaches `window.semioWgpuIntrospection` (the structural oracle the parity harness's `triageParityBoot` reads), builds the accessibility mirror, wires all input, and focuses the canvas — this callback IS the ready marker.
6. On `onFault`: tears everything down and renders a full-screen `role="alert"` banner with a human-readable fallback-state dump (surface, boot stage, silent-for duration, UI/worker turn-overrun ledgers) plus a matching `console.error`.

A second, independent boot path exists for embedding: `🎬️renderer-boot/🟦️.ts`'s `bootFrameworkOsWgpu`
is explicitly id-less (comment at `🚀️browser-boot/🟦️.ts:59-63`: "several independently-rooted mounts
coexist on one page there") but its options type is narrower than both the trunk app's own descriptor
and React's `bootFrameworkOs` (#3 above) — no role/example/locks/appId/brand.

A third, native path: `⌨️native-entrypoint/🦀️.rs` parses raw `env::args()` for `--plugin` (default
`"studio"`), plus unrelated ops flags (`--assert-no-local-credential-state`, `--credential-probe`,
`--socket-grant-probe`, `--scale`/`--scale-wasm`/`--report`/`--shards` for a scale-test harness,
`--smoke` for a headless widget-tree dump). No `--example`/`--role`/`--mode`/`--hub` (#4).

**Net effect**: opening "the same document" on React (`?plugin=cad&example=concrete-forest`), wgpu
browser (`?plugin=cad&example=concrete-forest`), and wgpu native (`--plugin cad`, nothing else) is only
possible for the first two today.

---

## Action dispatch pipeline detail

The wgpu shell's keyboard-driven dispatch (`handle_keyboard_async`, `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:
9260-9380`) is a well-ordered priority chain, each stage documented with why it must precede the next:
context menu → search/find Enter-activate → content-field Enter/Escape commit → idle-only shell
role/mode chords → fullscreen chord → in-palette zero-arg command chord match → content-focus routing
into `interpreter::dispatch_ui_event` → idle-Escape deactivates the focused window's active utility →
app-declared keybinding dispatch (with window-ownership resolution and a loud "unowned" error) → sync
`handle_keyboard` fallback. This is materially more sophisticated than a naive "one keymap" design and
mirrors React's layered precedence (reserved shell chords win over app chords; content focus wins over
chrome shortcuts) closely enough that the two named helpers (`chord_carries_accelerator_v1`,
`reserved_shell_chords_v1`) are explicitly documented as ports of React's own
`chordCarriesAcceleratorV1`/`reservedShellChordsV1`.

The one concrete hole in this otherwise-mature pipeline is `mod+z`/`mod+shift+z`: it is not a reserved
shell chord, not an app keybinding (apps don't own it), and not an OS command
(`build_os_commands()` has no undo/redo entry) — so it simply falls through to nothing. A user must
know to open the search palette and type "undo".

The command palette's own dispatch has a second, self-documented hole: only zero-arg commands and
os-scope single-select-arg commands actually execute (`command_search_items`,
`🦀️.rs:11215-11260`); any Plugin/App/Mode-scope command that takes a non-select argument is listed but
inert, because `ProgramBridgeEntry` only exposes `handle_action`, not a `handle_command` RPC.

---

## Input detail

`wireInput` (`🚀️browser-boot/🟦️.ts:307-380`) covers pointer (move/down/up with capture-on-down),
wheel (non-passive, `preventDefault`ed), key (down/up), IME composition (start/update/end), and paste
(clipboard text items, capped at 16, first string item wins). It does **not** listen for `dragover`,
`dragenter`, or `drop` — confirmed by a repo-wide `-a` grep across the whole wgpu target tree, which
returns zero hits for any of those outside one generated statechart's trigger-name string enum. Internal
drag/drop (tree rows, palette entries, world-drop targets) does not need DOM drag events — it is
implemented as a pointer-tracked custom drag inside `events::EventRouter` that emits
`UiCommand::DropCommitted`/`DropCancelled` on pointer-up over a target, consumed by
`Interpreter::apply_drop_committed`. What's missing is specifically the **OS-level** file drop (dragging
a file in from the Finder/Explorer) — only the picker-based `request-file-open` exists as an import
door.

Clipboard, wheel-coalescing, IME, and accessibility are all further along than the ticket's framing
("wgpu must catch up") suggested going in — these should not be work packets, just regression-tested.

---

## Recommended work packets

Ordered roughly by risk × isolation (P0s first, independent file seams, minimal overlap with the other
Wave-0 audits' territory — element/widget-level drag sources and scenes/world hover latency are
deliberately left to those lanes).

1. **Wire `mod+z`/`mod+shift+z` as a direct shell chord.**
   Seam: `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `handle_keyboard_async` (~9260-9380) or `build_os_commands`
   (~11149). React ref: `🏛️ShellHost/🟦️.tsx:8661-8663`. Acceptance: a browser probe presses
   `Meta+z`/`Meta+Shift+z` with focus on the canvas (no palette open) and observes `"undo"`/`"redo"`
   dispatched against the session controller, matching the existing palette-driven path's outcome.

2. **Port Space Administration to wasm32.**
   Seam: `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1839-2200,6150-6220` — replace/bridge `directory_client`
   (native-only networking) with a wasm32-compatible transport (likely a `fetch`/Worker-message bridge,
   mirroring the `host-io` pattern already used for file open/download). React ref:
   `🛂️SpaceAdministration/🟦️.tsx`. Acceptance: `open_space_administration`/`pump_space_administration`
   compile and round-trip a member list under `--target wasm32-unknown-unknown` (or the project's actual
   wasm target), not just native.

3. **Wire OS-level file drag-and-drop on the browser boot canvas.**
   Seam: `🚀️browser-boot/🟦️.ts`, `wireInput` (~307-380) — add `dragover`/`dragenter` (`preventDefault`)
   and `drop` listeners that read `event.dataTransfer.files` and feed them through the existing
   `🚪️host-io` open-file path (or a new `WgpuHostIoRequest` variant) rather than through
   `UiCommand::DropCommitted` (that's for internal widget drag, a different payload shape). React ref:
   the `DragEvent` import at `🎯️targets/⚛️react/🟦️.tsx:29` — first confirm React's actual OS-drop
   handler location (not fully traced in this pass) before mirroring its payload shape. Acceptance: a
   probe drags a file object onto the canvas and observes the same import path `request-file-open`
   would produce.

4. **Add `--example`/`--role`/`--mode`/`--hub`/`--user`/`--dataDir` to the native entrypoint.**
   Seam: `⌨️native-entrypoint/🦀️.rs` (~89, alongside the existing `arg_value("--plugin")`). React/wgpu-
   browser ref: `🚀️browser-boot/🟦️.ts:46-57` `bootDescriptor()` (same field set, already validated
   against a length cap — reuse the same bounds). Acceptance: `--plugin cad --example concrete-forest`
   opens the same document natively that `?plugin=cad&example=concrete-forest` opens in the browser.

5. **Align `FrameworkOsWgpuBootOptions` with `FrameworkOsBootOptions`.**
   Seam: `🎬️renderer-boot/🟦️.ts:9-13`. React ref: `🐚️Shell/🟦️.tsx:1218` `FrameworkOsBootOptions`.
   Acceptance: the embeddable wgpu boot accepts `appId`/`appRole`/`locks`/`defaults`/`brand` and threads
   them into the same `bootDescriptor`-shaped payload the trunk app already builds, so a single call site
   can swap renderers without dropping options.

6. **Wire a minimal agent-bridge presence indicator on wgpu (or explicitly scope it out).**
   Seam: new — there is no existing partial implementation to extend (`🦀️.rs` has zero AgentBridge
   references). React ref: `🔗️AgentBridge/🟦️.tsx` (476 lines, the full wire protocol),
   `🚦️AgentPresence/🟦️.tsx` (48 lines, the minimal consumer — start here, not with the 476-line bridge).
   This is large enough that it likely deserves its own ticket/scoping decision rather than a single
   packet; flagging it here so it isn't silently dropped from the parity backlog. Acceptance (minimal):
   a connected/working/idle/disconnected status renders somewhere in wgpu chrome, driven by the same
   `agentPresence` frames React consumes.

7. **Make arg-carrying Plugin/App/Mode command-palette entries either work or disappear.**
   Seam: `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11215-11260` `command_search_items` — either wire a real
   `handle_command` RPC on `ProgramBridgeEntry` (larger, crosses into `🌉️ProgramBridge/🎯️targets/🧊️wgpu/
   🦀️.rs`, 1107 lines) or filter these entries out of the palette so they're not presented as live
   options that silently do nothing when selected. Acceptance: selecting such an item either executes it
   or the item does not appear.

8. **Re-verify job-progress staleness under a real spawned-job repro on wgpu.**
   Seam: `🧊️renderer/🦀️.rs:3224-3340` `JobProgressPresentationBridge` — this audit found the mechanism
   exists and looks more explicit than React's fix, but did not drive an actual spawned job end-to-end
   in a live wgpu session to confirm a window visibly updates mid-job (continuous frame redraw could
   still be painting stale retained content if the presentation lease is never `take()`n by the right
   window). React ref / regression baseline:
   `project-react-host-never-refreshes-on-spawned-job-progress.md`. Acceptance: a probe starts a
   long-running spawned job and observes the adopting window's content change before the job completes,
   not just at completion.

9. **Confirm per-surface drag SOURCES (not just the command sink) for tree/palette/world-drop.**
   Seam: cuts across `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🌳️tree` (or wherever tree rows live)
   and `engine_canvas`/`scenes` world-drop handling — deliberately not scoped in this lane's audit since
   it's element/scenes-coverage territory; flagged here so it isn't lost. `UiCommand::DropCommitted`
   plumbing (#20 in the gap table) is confirmed end-to-end at the command layer; whether every React
   drag surface (catalogue→world drop, template palette→tree, block-list reorder) has a wgpu widget that
   actually **starts** a drag was not verified row-by-row.

10. **Audit the three-way boot-axis mismatch with an integration law, not just a doc.**
    Seam: a new test under `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/` that
    parses `FrameworkOsBootOptions`, `bootDescriptor()`'s field set, `FrameworkOsWgpuBootOptions`, and
    the native entrypoint's `--flag` set, and asserts the same field-name set across all four (or an
    explicit, commented allowlist of intentional exceptions). This turns packets 2/4/5 above into a
    standing law instead of three one-off fixes that can drift apart again.

---

## Honest gaps in this audit

- Hover-latency local-paint parity (React's `worldHoverPaintIdV1` fix, `project-hover-latency-anatomy-
  and-fixes.md`) was NOT traced on the wgpu side — the world-pointer pipeline lives in
  `engine_canvas`/`scenes` modules that are the scenes/engine-canvas Wave-0 audit's territory. A `grep -a
  "raycast\|hovered_id\|local_hover"` across the os-renderer and Shell wgpu targets returned nothing,
  which is inconclusive (wrong search terms are as likely as absence) rather than a confirmed gap —
  flagged as a risk, not asserted as a finding.
- Item #12/#13 (view-state/panel-action ownership traps) are confirmed as *shared* framework rules from
  prior sessions' work on OTHER apps (draw, energy, puzzle3d), not independently re-derived against
  wgpu's actual per-window dispatch call sites in this pass; a new wgpu dispatch site could still forget
  to project `active_utility_by_window_id` the same way React app code once did.
- Search fuzzy-ranking algorithm parity (weights, threshold, tie-breaking) was not compared line-by-line.
- Two parallel sub-agent investigations (engagement-ledger/spawned-job-progress and presence/task-
  manager/search/space-admin) were attempted via the `Agent` tool but hit a concurrent-subagent-limit
  error from other in-flight sessions in this multi-agent ticket; all findings above were produced by
  this session directly instead, at correspondingly less parallel coverage than planned.
