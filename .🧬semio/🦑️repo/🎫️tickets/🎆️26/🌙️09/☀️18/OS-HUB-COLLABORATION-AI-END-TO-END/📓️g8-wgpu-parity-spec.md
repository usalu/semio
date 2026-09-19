# G8 — wgpu-renderer parity spec for tonight's React-only UI landings

Read-only audit, slice G8. Sources: `📓️au2-os-sign-in-and-spaces-ui.md`, `📓️u1-progress-cancel-and-connection-status.md`,
`📓️u2-touch-tablet-contrast-diagram-a11y.md`, `📓️m4-mcp-bridge-approval-binding.md`,
`📓️m3-mcp-tests-and-wgpu-agent-panel.md`, `📓️o1-multi-plugin-hub.md`, `📓️o2-activation-follow-ups.md`, plus direct
reads of the wgpu renderer source named below. No files were edited outside this report.

## 0. What the parity mechanism actually is (measured, not assumed)

Confirmed by `find`: `🧵️TaskManager`, `🔄️ShellSync`, `🏘️SpaceBrowser`, `🔐️HubSignIn`, `🔗️HubConnection`,
`📌️ChromePanels`, `🕸️NodeGraph`, `🖥️Board2dHost`, `🌐️World3dHost` — **none** of these os-renderer elements
have a `🎯️targets/🧊️wgpu/` subfolder today. `💬️AgentChatPanel`, `🔗️AgentBridge`, `🤖️AgentApprovals`,
`🛰️Dock`, `🐚️Shell`, `🛂️SpaceAdministration` **do**. The declarative half of parity (`UiNode`/`SurfaceKind`
schema, `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🗺️surface/🦀️.rs:81` `SurfaceKind` incl. `Table`/`NodeGraph`)
is renderer-neutral and already shared; the imperative half (dock-tab registration, footer pills, retained
geometry/paint-ops, input routing) is hand-written per element in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
(26 020 lines) and in each element's own `🎯️targets/🧊️wgpu/🦀️.rs`.

The wgpu shell's own dock-tab builder (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7745-7777`) is the one canonical
place every framework-owned window/tab gets registered — `FRAMEWORK_MARKETPLACE_TAB_ID` leaf at `:7771`,
`FRAMEWORK_CHAT_PANEL_ID` leaf at `:7759` — and `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8317-8354` /
`:14319-14345` (`footer_presence_rows`, `sync_pill`, `render_footer_pills_step`) is the one canonical
footer-pill row (`s-sync-status` item 0, `s-presence-peers` item 1).

## 1. Work items, one per React-only feature

### WG-1 — TaskManager window (U1 item 1)

- **React landing**: `🧵️TaskManager/🟦️.tsx` (`🔖️LiveFeed`), mounted via
  `📌️ChromePanels/🟦️.tsx:1394` (`createFrameworkTaskManagerPanelTab`) and `🏛️ShellHost/🟦️.tsx:8684,9591`.
- **wgpu element/file**: new `🧵️TaskManager/🎯️targets/🧊️wgpu/🦀️.rs` (none exists) + a dock-tab leaf in
  `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, closest analogue the Marketplace leaf at `:7771`
  (`DockTabNode::leaf(FRAMEWORK_MARKETPLACE_TAB_ID, …, "circle-dot", 1)`) — a
  `FRAMEWORK_TASK_MANAGER_TAB_ID` leaf goes in the same `bottom_right` vec, "beside Marketplace" per U1 §1.
- **Declarative vs new Rust code**: **split**. The row body is declarative-cheap — U1 §1 confirms
  `buildTaskManagerTableScene` already emits the generic `ui_wgpu::wgpu::SurfaceKind::Table`
  (`🧬️contract/🗺️surface/🦀️.rs:97-99`) both renderers parse, so once a window exists the table paints
  for free. What is NOT declarative: (a) the dock-tab registration itself (Rust, ~10 lines following the
  `:7771` pattern), (b) a `Kernel::runtime_metrics_snapshot` publisher — the wgpu shell owns a real
  `Kernel`/`ActivationRegistry` (unlike the React DOM host, which U1 §1 says has none at all), so on wgpu
  the live-feed rows are a real host-side subscription, not a "no runtime attached" empty state — this is
  new Rust code in the kernel/renderer boundary, not UI.
- **Tests**: native `cargo test -p semio-framework-os-renderer-wgpu --lib -- task_manager` (dock tab
  present, table scene shape matches `taskManagerMetricCell`'s em-dash-for-null convention U1 §1 pins) +
  `--target wasm32-unknown-unknown` check (the metrics publisher must compile under the browser arm too,
  since the wgpu browser build has no native `Kernel` either — same asymmetry `sync_pill`'s doc comment at
  `:8330-8334` already states for sync).
- **Dependencies**: none on other G8 items; independent of the kernel-metrics publisher only if that
  publisher is deferred (window can land first, showing "no runtime attached" honestly, exactly as U1's
  React version does today).

### WG-2 — Agent chat cancel control (U1 item 2)

- **React landing**: `💬️AgentChatPanel/🟦️.tsx` cancel button + `ShellToGateway::AgentCancel` (tag 10),
  frame + gateway effect already in the Rust SSOT (`🌉️mcp/🧵️bridge/🦀️.rs`).
- **wgpu element/file**: `💬️AgentChatPanel/🎯️targets/🧊️wgpu/🦀️.rs` (exists — M3 landed the panel body).
  Closest analogue: `agent_chat_entry_node` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4879-4904`), which already
  renders a `state` label per row (`agent_chat_state_label`, `:4823`) and a `data-agent-chat-state`
  attribute (`:4856`) — a cancel `HitTarget` follows the exact pattern `render_plugin_install_step`
  registers its cancel hit at `:22286` (`HitTarget{…, control_id: Some(PLUGIN_INSTALL_CANCEL_CONTROL_ID)…}`).
- **Declarative vs new Rust code**: **cheap, mostly for free**. U1 §wgpu-parity table row 2 states this
  outright: "the frame and the gateway effect are in the Rust SSOT, so a wgpu shell gets them for free the
  moment it renders a chat surface" — M3's panel already renders on the live bridge (§4.3, 75+3+21 tests
  green). The only new code is one retained button bound to the existing `AgentCancel` frame send path
  M3's `send_agent_chat_draft` (`:7512`) already exercises for messages.
- **Tests**: native `cargo test -p semio-framework-os-renderer-wgpu --lib -- agent_cancel` (new) mirroring
  `os-mcp`'s own `ui::quick::an_agent_cancel_frame_flips_the_tool_calls_own_job_and_settles_it_cancelled`
  (already green per U1 §2.1); `wasm32-unknown-unknown` check only (no new native-only surface).
- **Dependencies**: none — independently landable; benefits from WG-1's `Kernel` wiring not at all.

### WG-3 — ShellSync status text (U1 item 4)

- **React landing**: `🔄️ShellSync/🟦️.tsx` localized `syncStatusLabelV1`.
- **wgpu status**: **already at parity, nothing to do.** U1's own table: "wgpu's own status strings live in
  `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` and use `LocalizedLabel` (exhaustive `Locale` match, compile-enforced
  en/de)" — confirmed present as `shell_chrome_string`/`shell_sync_pill_text` feeding `sync_pill()`
  (`:8335-8354`) and `render_footer_pills_step` (`:14319`). No work item.

### WG-4 — Plugin install cancel chrome (U1 item 5 / O1 §6.4 / O2 §3)

- **wgpu status**: **already closed**, and closed BEFORE this ticket's React work landed — O1's own
  progress state (`ShellPluginInstallPhase`, `CancelToken`, `:9117-9200` in O1's numbering) was chrome-less
  when U1 wrote its report, but O2 §3 ("Chrome for `plugin_install`") added
  `ShellChromeFramePhase::PluginInstall` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:18062-18073` banner text/tone,
  `:20043-20045` cursor dispatch, `:22249-22286` `render_plugin_install_step` incl. the cancel hit target)
  with en/de through `shell_chrome_string`. **No G8 work item** — flag only: U1 §wgpu-parity row 5 and O1
  §6.4 both predate O2's fix and still read as open; the coordinator should mark both closed.

### WG-5 — Hub-connection indicator (U1 item 8)

- **React landing**: `hubConnectionSummaryV1` fold rule (`🔄️ShellSync/🟦️.tsx`, `🔖️hub-connection-indicator`)
  + footer `HubConnectionIndicator`.
- **wgpu element/file**: `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`. Closest analogue: the existing two-pill footer
  row — `sync_pill()` (`:8335-8354`, folds `ArtifactSyncStatus`/`RemoteState` into `ShellSyncPill`) and
  `render_footer_pills_step` (`:14319-14345`, `cursor.item == 0` → `"s-sync-status"`,
  `cursor.item == 1` → `"s-presence-peers"`). A third pill (`cursor.item == 2` → `"s-hub-connection"`)
  follows the identical shape.
- **Declarative vs new Rust code**: new Rust code — U1 §wgpu-parity row 8 is explicit: "needs a Rust twin
  of `hubConnectionSummaryV1` plus a footer band in the wgpu shell. The aggregation rule is 10 lines of
  pure logic … specified above precisely so a Rust twin can be byte-identical." **Hard-blocked on WG-6**
  (there is no wgpu `HubConnection`/session state to fold yet — this pill has nothing to read until the
  hub session exists on this renderer).
- **Tests**: a pure Rust unit test mirroring `hubConnectionSummaryV1`'s fold-order laws (live-document
  outranks backoff/connecting, `peerCount` is a max not a sum, `signedOut` outranks all transport states)
  + one `render_footer_pills_step` extension test for the third pill's `control_id`/text. Native only —
  no wasm32-specific surface (footer paint code is already cfg-symmetric per `sync_pill`'s doc note).
- **Dependencies**: depends on WG-6 (needs `HubSessionPresenceV1`-equivalent state).

### WG-6 — HubSignIn + SpaceBrowser + HubWorkspace at `/hub` (AU2, whole slice)

- **React landing**: `🔐️HubSignIn/🟦️.tsx`, `🏘️SpaceBrowser/🟦️.tsx`, `🔗️HubConnection/🏛️workspace/🟦️.tsx`
  (`HubWorkspace`), mounted by `🏛️ShellHost/🟦️.tsx` on route `/hub`.
- **wgpu element/file**: no `🎯️targets/🧊️wgpu/` exists for `🔐️HubSignIn`, `🏘️SpaceBrowser`, or
  `🔗️HubConnection`. Closest existing analogue in the SAME domain (directory commands, per-space
  administration) is `🛂️SpaceAdministration/🎯️targets/🧊️wgpu/🦀️.rs` (201 lines) — a retained-geometry +
  paint-ops module: `space_administration_sheet_rect`/`_list_rect`/`_row_rect` (`:44-76`),
  `space_administration_title`/`_close_label`/`_public_notice`/`_spectator_notice` (`:87-111`, the
  `LocalizedLabel` idiom WG-3 confirmed), `SpaceAdministrationRow`/`SpaceAdministrationPlan`
  (`:140-149`) and `space_administration_paint_ops` (`:160`). A `HubSignIn`/`SpaceBrowser` wgpu module
  would follow this exact shape: pure `Rect` geometry functions + a `*Plan`/`*Row` pair + one
  `*_paint_ops` function, wired into `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` the same way `build_agent_chat_ui`
  (`:7535-7606`) builds and the dock/route machinery mounts it (a `/hub` URI branch analogous to
  `applyShellUri`'s `path === "/hub"` at `🏛️ShellHost/🟦️.tsx:6081`, ported into whatever the wgpu shell
  uses for URI/route dispatch inside this same file).
- **Declarative vs new Rust code**: **new Rust code, largest item in this spec.** The AU2 slice itself
  built four TS layers (pure contract → headless hook → presentational panes → composed surface) — porting
  the pure contract (`🔐️sign-in/🟦️.ts`, `🏘️spaces/🟦️.ts`: token parsing, reducer, command builders) is a
  second implementation of real logic, not a schema read, because `CQRS`/`DirectoryCommand` construction
  and the `session.v1.<hex>.<hex>` token parser are hand-written TS with no schema-driven counterpart on
  the Rust side today (AU2 §2 "Decisions worth defending" — the reducer and the closure-captured token are
  bespoke). The retained-paint half (forms, rows, buttons) is the same craft as
  `🛂️SpaceAdministration/🎯️targets/🧊️wgpu` — hand-written but small per screen (~200 lines/element).
- **Tests**: native — a Rust port of AU2's oracle idea (`hubAuthContractOracle`) reading the hub's
  `🌎️hub/🔐️auth/🧬️schema/🔣️.json` + `.rs` directly, since Rust needs no cross-language byte-compare, just
  the same status/error-class table AU2's 31+24 vitest laws pin; `--target wasm32-unknown-unknown` check
  for the browser wgpu arm (the fetch/session-storage port, wgpu's twin of
  `createHubConnectionFetchPortV1`).
- **Dependencies**: **blocks WG-5**. Also inherits AU2's own honest gap #1 (§5 of AU2's report) — no real
  hub `POST /auth/sessions` route is mounted yet (AU1 landed the auth module, not the route), so a wgpu
  port would be unverifiable against a live hub for the same reason AU2's React port is: nothing to sign in
  to. This item should NOT be scheduled ahead of that route landing, or it repeats AU2's own honest gap on
  a second renderer for no new information.

### WG-7 — Approval affordance in AgentChatPanel (M4, § "The surface")

- **React landing**: inline Deny / Approve Once / Approve for Session row in `💬️AgentChatPanel/🟦️.tsx`
  (M4 §2 items 25-27), reading the same `ApprovalRequest`/`shell_summary` the Rust `ApprovalCoordinator`
  already emits (`🛡️policy/🦀️.rs:273-430`).
- **wgpu element/file**: `💬️AgentChatPanel/🎯️targets/🧊️wgpu/🦀️.rs` (exists, M3 landed the panel body but
  not this control) + `🤖️AgentApprovals/🎯️targets/🧊️wgpu/🦀️.rs` (exists — the standalone approvals
  surface already has a wgpu target, unlike everything in WG-6). Closest analogue for the three-button
  row: the same `render_plugin_install_step` hit-registration idiom (`:22286`) or, more precisely, whatever
  `🤖️AgentApprovals/🎯️targets/🧊️wgpu/🦀️.rs` already does for a decision row (read that file before
  starting — it is the one wgpu surface in this whole spec that already renders an approve/deny decision,
  so it is the right template, not `AgentChatPanel`'s own file).
- **Declarative vs new Rust code**: **cheap** — M4's own design (§1.3) makes the approval channel transport
  a `GatewayToShell::ApprovalRequested`/`ShellToGateway::Approval` frame pair that any shell answers
  identically; `🤖️AgentApprovals` already proves the wgpu side can decide one. This item is "read the
  pending approval off the SAME conversation entry `agent_chat_entry_node` already renders (`:4879`,
  `agent_chat_state_label` already has an `approvalPending` case per `:4821` doc) and add the three hit
  targets," not new protocol work.
- **Tests**: native `cargo test -p semio-framework-os-renderer-wgpu --lib -- agent_chat` extension +
  `wasm32-unknown-unknown` check.
- **Dependencies**: depends on WG-2 only in the sense both touch `agent_chat_entry_node`/
  `agent_chat_state_label` in the same file — sequence them through one PR or expect a merge conflict, not
  a design dependency.

### WG-8 — Multi-touch pinch/pan (U2 item 6, `🖥️Board2dHost`/`🌐️World3dHost`)

- **React landing**: `🧰️framework/🔨️modules/🕹️interaction/👆️gesture/🟦️.ts` (pure gesture math) wired into
  both hosts' pointer handlers.
- **wgpu element/file**: no per-element wgpu targets exist for `Board2dHost`/`World3dHost` (both are
  React-only compositions over generic `SurfaceKind::Board2d`/`World3d` scenes). The blocking gap is one
  level lower: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:259-280` `InputState<E>` models
  exactly **one** pointer (`pointer_x`, `pointer_y`, `pointer_down`, no id-keyed collection at all).
  U2 §6 gap 2 confirms: "the wgpu target has no multi-touch of its own and does not share this [gesture]
  code." Before any element-level pinch code can exist, `InputState` itself needs a `pointer_id`-keyed
  contact map (the same shape U2's `gesturePointerDown/Move/Up` uses in TS,
  `🕹️interaction/👆️gesture/🟦️.ts`).
- **Declarative vs new Rust code**: **new Rust code, foundational** — this is not element work, it is
  input-layer work (`ui_wgpu`'s `InputState<E>` is shared by every wgpu scene). Two sub-items: (a) extend
  `InputState` with multi-contact tracking + a `pinch_frame`/`pinch_step` port of the pure TS math (small,
  ~200 lines mirroring `👆️gesture/🟦️.ts`'s 214), (b) wire it into whichever scene handler owns
  `Board2d`/`World3d` camera dispatch on the wgpu side (not located in this pass — needs its own file
  search before estimating).
- **Tests**: a pure-math native unit-test port of the 22 `👆️gesture` tests (pinch centroid/scale/rotation,
  `clampZoom` against the SAME bounds `Board2dHost` reads from `STYLING_METRICS.camera.zoomMin/zoomMax`)
  is cheap and zero-risk to write first, independent of wiring; the wiring itself needs a native
  integration test (synthetic multi-pointer `InputState`) plus `wasm32-unknown-unknown` check (touch
  events reach the wgpu browser build through the same door `👆️gesture` was deliberately kept out of per
  U2's own scoping note).
- **Dependencies**: none on other G8 items. Recommend landing (a) alone first — it is reusable by any
  future multi-touch surface, not just these two hosts.

### WG-9 — Tablet breakpoint (U2 item 7)

- **wgpu status**: **already at parity.** U2 §2.2 landed the Rust twin directly:
  `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1100-1122` `MODE_DOCK_TABLET_MAX_WIDTH_PX` + `mode_dock_device_for_width`,
  gated by a test that reads the Rust source off disk and asserts literal equality with the TS constants
  (`🖱️ui/📱️device/🧪️tests/🔬️unit/🟦️.ts`, 7/7 green). U2 §6 gap 3 flags only that
  `mode_dock_device_for_width` was never run through `cargo check -p` (no borrow/generic/trait risk, per
  U2). **G8 work item: run `cargo check -p semio-framework-os-renderer-wgpu --lib` once** (or whichever
  crate owns `🛰️Dock`'s wgpu target) to close that one residual verification gap — trivial, no new code.

### WG-10 — Theme contrast badge (U2 item 9)

- **wgpu status**: **framework-level, not renderer-specific.** `contrastRatio`/`wcagContrastGrade`
  (`🎨️styling/🌓️theme/🟦️.ts:686-740`) are pure TS with no Rust caller anywhere in either renderer — the
  React editor is the only *consumer* (`📌️ChromePanels/🟦️.tsx:657-717`), and `📌️ChromePanels` itself has
  no wgpu target (§0). If/when a wgpu theme editor exists, this is a straight port of the WCAG formula
  (relativeLuminance + contrast ratio, ~20 lines) plus a badge string in the settings-theme dock tab
  (`FRAMEWORK_SETTINGS_THEME_TAB_ID`, `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7807`). Not scheduled as a G8 slice
  on its own — it rides whichever slice eventually builds a wgpu theme editor; no such editor exists today
  to attach it to, so listing it as a standalone item would invent a dependency that isn't there yet.

### WG-11 — Diagram keyboard/ARIA (U2 item 10)

- **React landing**: `🕸️Diagram/🟦️.tsx` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram`) — pure
  `diagramArrowNavigationTarget`/`diagramActivateSelection` + `role="application"` DOM surface.
- **wgpu element/file**: `🕸️Diagram` has no `🎯️targets` subfolder at all (it is a generic ui-kit
  presentational component, not tied to a `SurfaceKind`). The consuming surface on wgpu would be whatever
  renders `SurfaceKind::NodeGraph` (`🧬️contract/🗺️surface/🦀️.rs:91-93`) — `🕸️NodeGraph` itself also has no
  wgpu target (§0). Closest existing infrastructure: keyboard routing already exists generically in
  `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs` — string-keyed arrow handling at
  `:1552-1558` (`"ArrowLeft"`/`"ArrowRight"`) and `:1870-1871`, and `InputState::focused_id`
  (`📥️input/🦀️.rs:268`) already tracks a focused control id. This is NOT starting from zero.
- **Declarative vs new Rust code**: small new Rust code — a port of
  `diagramArrowNavigationTarget(nodes, focusedId, direction)`'s pure nearest-node-in-half-plane math
  (~60 lines, `🕸️Diagram/🟦️.tsx:1072-1130`) plus wiring it into whatever paints `NodeGraph` scenes through
  the existing `⚡️events` arrow-key dispatch and `InputState::focused_id`. wgpu has no DOM, so there is no
  `aria-*` attribute to port — the accessibility surface there is whatever the wgpu renderer's own
  accessibility bridge already exposes (not located in this pass; check for a native `AccessKit`/
  `accesskit`-style tree before assuming one must be built).
- **Tests**: a pure-math native unit test port of U2's 5 pure-law tests (arrow-direction/half-plane
  weighting/tie-break/no-focus-enters-opposite-extreme) is independent and cheap; a component-level
  integration test needs the NodeGraph wgpu surface to exist first.
- **Dependencies**: soft-depends on whichever slice gives `NodeGraph`/`🕸️Diagram` a wgpu target at all —
  today there is none to attach keyboard nav to.

## 2. wgpu renderer crate: 30 failing tests + 1 SIGABRT (from M3's report, classified by text only)

M3 (§4.4) names the **module groups**, not all 30 individual test names, and is explicit that none of them
belong to M3's own agent-panel work ("zero [failures] in agent bridge / agent presence / agent overlays /
agent chat"). Classified here by the module names M3 gave, as owner/cause:

| module | cause (from M3's text) | owner |
|---|---|---|
| `kernel_runtime::semantic_document_tests::product_ingress_kind_and_input_max_plus_one_return_exact_spawn_and_remainder` | **the SIGABRT** — panics inside `MountedProductReplayRequest::drop` on a poisoned lock; aborts the whole process and is `--skip`ped to let the other 1012 run at all | kernel-runtime/replay owner (not named in M3; not G8/O1/O2/U1/U2/M4) |
| `engine_canvas` | not diagnosed by M3 beyond the module name | EngineCanvas wgpu owner |
| `interpreter::render_plan_validator` | not diagnosed | interpreter/render-plan owner |
| `kernel_runtime::semantic_document` (the other, non-abort failures in this module) | not diagnosed | same as the SIGABRT owner — likely the same in-flight change |
| `scenes` | not diagnosed | Scenes wgpu owner |
| `shell::theme_editor` | not diagnosed | Shell/theme-editor owner |
| `shell::tool_run_panel` | not diagnosed | Shell/tool-run owner |
| `shell::window_measures` | not diagnosed | Shell/window-measures owner |
| `shell::window_actions_search_pane` | not diagnosed | Shell/window-actions owner |
| `shell::display_conflicts_marketplace` | not diagnosed | Shell/marketplace owner |
| `shell::appearance_tour` | not diagnosed | Shell/appearance-tour owner |
| `shell::shell_document_retirement` | not diagnosed | Shell/document-retirement owner |
| `shell::panel_anchor_model_tests::build_settings_theme_ui_lists_builtins_and_gates_delete_on_custom_theme` | one named test, not diagnosed beyond its name (theme-list/delete-gate assertion) | Shell/settings-theme owner |

M3 is explicit these 30 are "other slices' in-flight work" it made compile but did not investigate (§4.4,
§6.3); **none overlap any WG item above** — no G8 work item depends on these failures clearing, since G8's
items are additive (new dock tabs / new elements) rather than edits to `engine_canvas`, `scenes`, or
`shell::*` theme/window-measure code. Whoever owns each module is not stated anywhere in the read reports;
this table is the full extent of what can be classified "by text only" per the task's own instruction.

## 3. Grouping into worker-sized slices (disjoint files)

**G8a — Chat-surface parity (small, fast, mostly free)**
WG-2 (agent cancel) + WG-7 (approval affordance). Both touch only
`💬️AgentChatPanel/🎯️targets/🧊️wgpu/🦀️.rs`, `🤖️AgentApprovals/🎯️targets/🧊️wgpu/🦀️.rs`, and
`agent_chat_entry_node`/`agent_chat_state_label` in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4879-4904`. No
foundational work, no other slice's files. Sequence WG-2 before WG-7 (same functions) rather than parallel.

**G8b — Shell chrome additions (dock tab + footer pill)**
WG-1 (TaskManager window, dock-tab half only — defer the `Kernel::runtime_metrics_snapshot` publisher as a
follow-up sub-task since it is host/kernel work, not chrome) + WG-9 (tablet `cargo check` closure, trivial).
Touches `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7745-7815` (dock builder) and a new
`🧵️TaskManager/🎯️targets/🧊️wgpu/🦀️.rs`; WG-9 touches only `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs` (already landed,
verification-only). Disjoint from G8a's files (different functions in the same `🐚️Shell` file — the dock
builder region vs. the agent-chat region — so a shared-file note but no line overlap).

**G8c — Foundational input/math ports (independent, no dependency on a live hub)**
WG-8 sub-item (a) only: extend `InputState<E>` (`🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:259-280`) with
multi-contact tracking + pure `pinch_frame`/`pinch_step` math, and WG-11's pure-math port
(`diagramArrowNavigationTarget` → a standalone Rust function, not yet wired to any scene). Both are
self-contained math ports with their own unit tests and touch no other slice's files
(`📥️input/🦀️.rs` and a new module, not `⚡️events/🦀️.rs` or any `Shell` file, until wiring — which is
explicitly deferred here as a separate follow-up once a NodeGraph/Board2d/World3d wgpu surface exists to
attach to).

**Deliberately NOT grouped into a G8 slice (blocked or too large for a worker slot):**
- WG-6 (HubSignIn/SpaceBrowser/HubWorkspace) — blocked on the hub's `POST /auth/sessions` route landing
  (AU1/AU2 gap, unrelated to wgpu); also the largest single item (new pure-contract port + new retained-UI
  module), sized as its own future slice once unblocked.
- WG-5 (hub-connection indicator) — hard-depends on WG-6.
- WG-8 sub-item (b) (wiring pinch into `Board2d`/`World3d` scene dispatch) — needs the scene-handler
  location identified first (not found in this read-only pass); do not bundle into G8c's math-only work.
- WG-10 (contrast badge) — no wgpu theme editor exists to attach it to; not actionable as a slice today.
