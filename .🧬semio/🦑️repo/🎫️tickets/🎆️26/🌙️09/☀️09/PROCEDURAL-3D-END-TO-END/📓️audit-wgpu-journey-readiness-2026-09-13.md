# wgpu Journey Readiness Audit — 2026-09-13

Read-only audit. No source files were edited, no builds/servers started, no git state changed. Sources: direct
reading of the wgpu shell (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, 13,635 lines), `🐚️plugin-bridge/🟦️.ts`
(1,448 lines), `🚀️browser-boot/🟦️.ts` (315 lines), the React `🏛️ShellHost/🟦️.tsx` equivalents, `git log -L`
history, plus the five requested prior audits (`📓️wgpu-dock-layout-world3d-2026-09-12.md`,
`📓️wgpu-resident-budget-settle-2026-09-12.md`, `📓️wgpu-tree-row-hit-test-2026-09-12.md`,
`📓️role-switch-keyboard-2026-09-12.md`, `📓️viewer-examples-2026-09-12.md`) and `📓️audit-hover-selection-2026-09-12.md`
§1. All wgpu file:line citations below are against the canonical (non-`🗑️generated`) path:

```
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs
```
abbreviated `wgpu-shell.rs` below.

**Note on scope:** a concurrent lane is fixing presentation + input on the live 6118 server (per
`📓️wgpu-tree-row-hit-test-2026-09-12.md` §5.4). This audit is a static/source-level readiness assessment, not a
fresh runtime probe run — it does not touch that server.

---

## 1. How does a user switch examples on the wgpu shell today?

**No `?example=` boot query.** `bootDescriptor()` in `🚀️browser-boot/🟦️.ts:45-55` only reads `plugin`, `role`,
`mode`, and (if `hub` present) `user`/`dataDir` from `location.search`. There is no `example` param anywhere in
this file, nor in `wgpu-shell.rs`'s boot path.

**No navbar picker control is painted.** `render_navbar_step` (`wgpu-shell.rs:11261-11392`) draws exactly: the
logo icon, the app-id title, a fullscreen toggle (`ui.fullscreen.toggle`), and up to 4 panel toggles
(`ui.panelToggle.display/workbench/details/settings`, phase 5, lines 11313-11380). No example-picker button, no
mode buttons, no role buttons are ever emitted here — this is a direct read of the render function, not an
inference.

**A picker *handler* exists but is dead code.** `handle_control_command` (around `wgpu-shell.rs:6488-6516`) has:
- `"playground.navbar.fixture"` (`:6488-6491`) → sets `self.overlay_state = OverlayState::Dropdown("example".to_string())`.
- `id if id.starts_with("shell.example.")` (`:6511-6518`) → sets `active_example_id` and dispatches
  `ActionDescriptor { action: "setActiveExample", args: { exampleId } }` — i.e. the *correct* target action.

But nothing ever paints a `"playground.navbar.fixture"` hit target (it is not among the navbar items enumerated
above), and nothing ever generates a `"shell.example.<id>"` hit target either: the "example" dropdown's overlay
renderer (`render_overlay_step`, `wgpu-shell.rs:11623-11663`) only draws a static "Examples" title
(`:11640`) — it never iterates a list of examples into rows. The actual per-item row/hit-rect renderer,
`render_context_menu_step` (`wgpu-shell.rs:11705-1177x`), only ever draws from `self.context_menu`, which is
populated exclusively by the *right-click* context-menu builder (`wgpu-shell.rs:7116-7148`, window actions +
"Go Home") — never by the example list. So `shell.example.*` is registered as a click handler for a control
that can never appear on screen: `grep -rn "shell.example" --include=*.rs .` finds only the handler itself in
the live file, plus an identical-but-fuller version (including the row-emission call
`control_id: Some(format!("shell.example.{}", example.id))`) in
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️18/DEGENERALIZE-HARDCODED-S-APP-IDENTITY-FROM-OS-SHELL/before/wgpu-lib.rs:21912` —
i.e. the row-emission half of this feature existed before a refactor and was dropped; `git log -L 6488,6520` on
the current file shows the surviving handler untouched since rename-only commit `a5cc4dd9ab` (a namespace
migration), confirming it predates the 2026-09-12 dialect-keyed `examples_for_app` work
(`📓️viewer-examples-2026-09-12.md`) and was never reconnected to it.

**Compounding the gap:** the wgpu example-resolution code that *does* run (`wgpu-shell.rs:3516-3521`) is also
stale — it reads `plugin.manifest.examples` filtered only by `plugin_id`, not by dialect:
```rust
let examples = self.plugins.iter().find(|p| p.plugin_id == session.plugin_id).map(|p| p.manifest.examples.as_slice()).unwrap_or(&[]);
```
This predates the `ExampleDefinition.app_id → dialect` schema change (`🛂️manifest/🦀️.rs`,
`📓️viewer-examples-2026-09-12.md` line 1-18) and never calls the new `examples_for_dialect`/`examples_for_app`
(`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3642` and `:3650`). It would need re-pointing even once a real UI is
built.

**Cleanest design (reuse, don't invent):**
1. In `wgpu-shell.rs`, replace the stale filter at `:3516-3521` with a call to
   `semio_framework::manifest::examples_for_app(&plugin.manifest.examples, &session.app)` (the same
   dialect-keyed predicate the React `ShellHost` already calls at `🏛️ShellHost/🟦️.tsx:8838-8849`
   — `examplesForApp` mirrored 1:1 into Rust).
2. Add a navbar item to `render_navbar_step` (phase 5's group, or a new phase before it) gated the same way
   React gates it — `exampleOptions.length > 0 && !locks.exampleId` (`🏛️ShellHost/🟦️.tsx:8863,9797`) — reusing
   the existing `"playground.navbar.fixture"` control id so the dead handler becomes live.
3. Make the "example" dropdown (`OverlayState::Dropdown("example")`) actually populate `self.context_menu`
   (or a dedicated list) with one row per `examples_for_app` result, each row's `control_id` =
   `format!("shell.example.{}", example.id)` — restoring exactly the row-emission logic already proven correct
   in the pre-refactor `wgpu-lib.rs:21912` (same id scheme, same target action), but sourced from the new
   dialect-keyed list.
4. No new action is needed on the guest side — `setActiveExample` already exists and is view-safe
   (`✏️s/…/generation3d/…/👁️viewer/🎮️commands/🎨️set-active-example/🦀️.rs`, `📓️viewer-examples-2026-09-12.md`
   line 4th bullet), and the wgpu dispatch call at `wgpu-shell.rs:6516` already targets it correctly.
5. Optionally add `example` to `bootDescriptor()` in `🚀️browser-boot/🟦️.ts:45-55` (one more `params.get("example")`
   field threaded into `active_example_id` at session creation) so a probe/deep-link can boot straight into an
   example without needing the (still-to-be-built) picker — this is the cheapest of the five steps and unblocks
   a probe immediately even before the navbar UI lands.

---

## 2. Control-by-control parity table (React shell vs wgpu shell)

| Control | React (owning file) | wgpu status | wgpu owning file:line |
|---|---|---|---|
| Example picker (`NavbarExampleSelect`) | `🏛️ShellHost/🟦️.tsx:8838-8877` (`exampleOptions` via `examplesForApp`) | **Missing** (handler present, nothing renders it — see §1) | `wgpu-shell.rs:6488-6491, 6511-6518, 11640` (dead) |
| Mode buttons (`playground.navbar.modes.<id>`) | rendered navbar buttons + `mod+alt+ArrowRight/Left` chords, `🏛️ShellHost/🟦️.tsx:7035, 7061-7088` | **Painted only via URL, not clickable** — `handle_control_command` accepts `playground.navbar.modes.*` (`:6492-6501`) and `shell.layout.*` (`:6684-6685, 9077`), but `render_navbar_step` never emits either as a hit target; only reachable via `?mode=` boot param (`🚀️browser-boot/🟦️.ts:52`) | `wgpu-shell.rs:6492-6501` (dispatch only) |
| Role buttons (`playground.navbar.roles.editor/viewer`) | rendered navbar buttons + `mod+alt+e/v` chords, `🏛️ShellHost/🟦️.tsx:8586-8626, 7061-7088` | **Missing entirely** — no `playground.navbar.roles.*` string anywhere in `wgpu-shell.rs`; role is boot-time only via `?role=` (`🚀️browser-boot/🟦️.ts:51`), no `switchToPluginApp`/`switchToSessionRole` counterpart exists | n/a (absent) |
| Mode/role keyboard chords (`mod+alt+e/v/←/→`) | `SHELL_KEYBINDINGS`, bound via `useActionHotkey`, `🏛️ShellHost/🟦️.tsx:7061-7088` | **Missing** — `is_reserved_shell_chord` (`wgpu-shell.rs:8585-8596`) hardcodes only palette (`mod+p`), find (`mod+f`), panel toggles (`mod+b`,`mod+[`,`mod+]`), nav (`mod+↑`), fullscreen (F11 / ctrl+meta+f); no alt-chord entries for mode/role at all | `wgpu-shell.rs:8585-8596` |
| `Fit graph` | node-graph camera-fit control (owned by `📓️node-graph-camera-fit-labels-2026-09-12.md`, not read this pass) | **Missing** — preview camera is hardcoded `[4,-4,3]→[0,0,0]`, no fit pass (`📓️wgpu-resident-budget-settle-2026-09-12.md` L385-387) | not found in `wgpu-shell.rs` |
| Preview status overlay | `World3dHost` status pill (React DOM) | **Present, no cancel wiring confirmed** — guest status payload already carries `"cancellable":true,"cancelAction":"cancelPreviewEval"` (`📓️wgpu-dock-layout-world3d-2026-09-12.md` L356: data exists), but no UI control/hit target for firing that cancel action was found in `wgpu-shell.rs` (no `cancelPreviewEval`/`"cancel"`-for-preview string outside the unrelated check-in dialog's `"cancel"` arm at `:4617`) | data path only; no chrome control |
| Inspection / Catalogue / Artifact panels | guest-published window bodies inside dock | **Missing in generate mode** — guest doesn't publish `framework.panel.artifact`/`.catalogue`/`.inspection` bodies there (`wgpu-ui.surface-not-published`, `📓️wgpu-resident-budget-settle-2026-09-12.md` L376-379); wgpu chrome itself has no dedicated enum for these (its own panel model is generic `LeftPanelKind{Workbench,Display}` / `RightPanelKind{Details,Settings}`, `wgpu-shell.rs:111-122`, unrelated to the plugin's own panel window-kinds) | `wgpu-shell.rs:111-122` (chrome-level panels only) |
| History / Settings footer (sync + check-in) | React sync/history footer | **Present and working** — `render_sync_status_and_checkin` (`wgpu-shell.rs:8167`, called `:11608`), `history_entries`/`history_cursor`/`uncommitted_edit_count` (`:4399-4508`), check-in dialog `open/cancel/submit` (`:4610-4626`) | `wgpu-shell.rs:8167, 11602-11618, 4610-4626` |
| Panel toggles (display/workbench/details/settings) | equivalent React panel toggles | **Present and working** — rendered navbar group (`:11313-11380`) | `wgpu-shell.rs:11313-11380` |
| Fullscreen toggle | React fullscreen control | **Present and working** | `wgpu-shell.rs:11291-11312` |
| Dock/multi-window layout | `resolveLayoutForMode` | **Present and working** (fixed 2026-09-12, both Rust+TS fixture-proven 4/4 + 6/6) | `wgpu-shell.rs` (`render_main_window_step` + `Dock` paint/hit paths); `📓️wgpu-dock-layout-world3d-2026-09-12.md` L258-286 |
| Resident document budget / settle loop | N/A (React re-renders freely) | **Present and working** (fixed 2026-09-12: 0 Capacity faults over 120s, event-driven settle converges in 1 round) | `📓️wgpu-resident-budget-settle-2026-09-12.md` L25-28, L163-171, L256-269 |
| Tree-row hit-test geometry (e.g. "Add Generation") | React DOM hit-testing (native) | **Geometry fixed and proven** (single `Theme::tree_row_height` source, 4/4 + 6/6 fixture law), **but click still doesn't dispatch** because the currently-served Vite host (a peer's in-flight migration) doesn't route pointer/wheel/keyboard events into `Ui::dispatch_event` at all — 0 new console lines on any input, `dumpStructure().state.hovered` never flips | `📓️wgpu-tree-row-hit-test-2026-09-12.md` L19, L296-339 |
| Hover / select / orbit dispatch (engine layer) | `World3dHost/🟦️.tsx` dispatch sites (see §3) | **Native-test proven, zero browser proof** (blocked by the same input-routing gap above, plus a masked stack-overflow risk in the wgpu World3d test harness) | `⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs` (see §3) |

---

## 3. Hover / selection / orbit — verbs dispatched vs emitted vs missing

Per `📓️audit-hover-selection-2026-09-12.md` §1 (lines 46-52) the React `World3dHost/🟦️.tsx` dispatches three
framework-reserved interaction verbs, gated on `scene.domainId` (generation3d sets
`domain_id: Some(GENERATION_3D_INTERACTION_DOMAIN.into())`, `preview/🦀️.rs:89-97`):

| Gesture | React dispatch site | Verb |
|---|---|---|
| Instance pointer-down | `World3dHost/🟦️.tsx:5331` (`handleInstancePointerDown`) | `interactionSelect` (`world3dSelectionActionArgs`, `:4541`) |
| Hover move | `World3dHost/🟦️.tsx:5352` | `interactionHover` (`world3dHoverActionArgs`, `:4460`) |
| Marquee release | `World3dHost/🟦️.tsx:5916` | `interactionSelect` (mode `"replace"`) |
| Empty click | `World3dHost/🟦️.tsx:6021` (`handleEmptyClick`) | `interactionSelect` (mode `"merge"`) |
| Completed orbit/pan/zoom | `World3dHost/🟦️.tsx:4886` (`handleCameraChange`, debounced) | `setCamera` (`worldCameraSetCameraDispatchArgs`) |

`nodeGraphViewport` does **not** appear anywhere in the hover-selection audit — it is not a verb this codebase
uses for generation3d's World3d surface; do not design the wgpu probe around it without independently confirming
it exists elsewhere.

**wgpu engine-surface side** (`⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs`, 387 lines, cited at audit
`:272-291,311`):
- `world3d_pointer_down_emits_the_graph_domain_selection_react_dispatches` (`:282`) — asserts a native pointer-down
  emits `select.action == "interactionSelect"`, `controller_id == "generation3d"` — the **same verb name** as
  React, proven at the engine-emission layer via a Rust unit test, not a browser.
- `world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload` (`:311`) — asserts wheel-zoom and
  alt/meta+button-2 drag emit `setCamera` with a 3-axis camera position — again the same verb as React.
- Orbit/pan were dead code until 2026-09-10 (`WorldInteractionAuthority::step` retired right-drag intents as
  `Complete` before reaching `plan_world3d_drag`; fixed in `infinite_world/🌍️world/🦀️.rs:5271-5291`).

**Gap is proof, not verb parity.** Both shells target the identical verb set
(`interactionSelect`/`interactionHover`/`setCamera`); the wgpu side has never been exercised end-to-end in a
real browser because (a) `📓️audit-hover-selection-2026-09-12.md` §5.3 records `meshes=0` from a `refreshUi`
supersession race blocking any browser proof as of 2026-09-12, and (b)
`📓️wgpu-tree-row-hit-test-2026-09-12.md` §5.3-5.4 (L312-356) records that the currently-served Vite host does
not route pointer/wheel/keyboard input into `Ui::dispatch_event` at all — confirmed independently by direct
reading of `🚀️browser-boot/🟦️.ts:152-229` (`wireInput`), which *does* correctly forward
`pointermove/down/up`, `wheel`, `keydown/up`, IME and paste into `BrowserFrameTransport.enqueue{Lossless,Replaceable}`
— i.e. the canonical browser-boot module wires input correctly, so the break the tree-row audit found is either
downstream in the transport→frame-worker path or in a *different*, in-flight replacement host that bypasses this
file; this needs the concurrent presentation/input lane to resolve, not this audit.

**Secondary caveat:** all three wgpu World3d generation3d engine tests overflow the platform-default 2 MiB
thread stack unless run under the repo's `RUST_MIN_STACK` floor (32 MiB clears it,
`runCargoTestBudgeted`/`🧰️framework/…/📚️library/📦️packages/🟦️typescript/🟦️.ts:1655-1663`) — they read green in
every gate log today, but it is unestablished whether the production wgpu frame/interaction thread is thin
enough to hit the same class of defect (audit §6, `:322-375`).

---

## 4. Concrete design for `🐍️wgpu-journey-probe.mjs`

**Introspection surface (no DOM oracles exist on wgpu — replace every React `[data-*-json]` read):**
- `window.semioWgpuIntrospection.dumpStructure(windowId?)` / `.dumpFrameStats(windowId?)` — attached only once
  the frame Worker reports `booted` (`🚀️browser-boot/🟦️.ts:73-91`), so awaiting the function's existence is
  itself a truthful boot gate; each call resolves `""` instead of throwing when unavailable. Both now take an
  optional `windowId`, fixed 2026-09-12 specifically so multi-pane layouts are measurable
  (`📓️wgpu-dock-layout-world3d-2026-09-12.md` §3.3 L118-124).
- `[role="status"]` (boot/eval progress text) and `[role="alert"]` (fault banner) — the only two DOM elements
  the wgpu host writes outside the canvas (`🚀️browser-boot/🟦️.ts:93-100, 140-148`).
- Console-line vocabulary to classify (already used by existing probes, all directly reusable):
  `"wgpu-shell render begin"` (carries `surface=` token), `"wgpu-shell dock plan"` (format
  `<id>@<w>x<h>+<x>,<y>` per window, parsed by `🐍️wgpu-hit-probe.mjs:62-72`), `"world3d surface"`,
  `resident-roots=`/`resident-bytes=` (per-render-leave census), `Capacity`/`retained document permit failed`,
  `"ui chain settled after N round(s)"` / `"ui chain exhausted"`, `extrude@solid`/`facesDone`/`ratio` (mesh
  convergence), `"invokeExtension faulted"`, `"unknown tag 115"`.
- Per-example mesh-count/convergence proof: poll `dumpFrameStats(previewWindowId)` for
  `scenePasses/sceneDraws/sceneInstances/quadCount`, cross-checked against `dumpStructure(previewWindowId)`'s
  `state.meshes_json` occurrence count of the expected geometry tag (e.g. `eval-extrude@solid#0`) — this is
  exactly the pair `📓️wgpu-resident-budget-settle-2026-09-12.md` §6.1 (L253-274) used to prove the hexagonal
  column converged (`facesDone:8/8, ratio:1.0`, `quadCount:17`).

**Driving the picker once §1's fix lands:** there is no `<select>`/`[role=combobox]` on wgpu, so a probe cannot
reuse `🐍️journey-probe.mjs`'s `listOptions()`/`pick()` pattern verbatim. Instead:
1. Click `"playground.navbar.fixture"` by deriving its painted rect the same way `🐍️wgpu-hit-probe.mjs:62-100`
   derives the "Add Generation" row rect: parse the last `"wgpu-shell dock plan"`/navbar-equivalent trace line
   (or, once built, dump the navbar subtree via `dumpStructure()` and locate the node whose `path` matches the
   fixture control), compute page coordinates as `bodyRect.x + nodeRect.x + nodeRect.w/2, ...`.
2. Click the resulting "Examples" overlay's per-example row the same way, once §1's row-emission fix lands
   (rows will carry `control_id = "shell.example.<id>"`, discoverable via `dumpStructure()`'s node `path`/`kind`).
3. Wrap every click in the **pointer-delivery witness** pattern from `🐍️wgpu-hit-probe.mjs:103-113` — inject a
   capturing-phase `pointerdown` listener on the canvas before clicking, so "no dispatch" (console silent) can be
   distinguished from "no event ever arrived" (witness also silent) — this is essential right now given §3's
   open input-routing question.
4. Until §1 ships, boot straight into a named example via the boot-time `active_example_id` extension proposed
   in §1 step 5 (`?example=`), so the mesh-convergence/journey-probe machinery can be built and proven
   independent of the picker UI.

**Mode/role coverage:** since neither has a clickable control (§2), drive them via `?mode=`/`?role=` boot-time
navigation (reload with a new URL) rather than in-page interaction, until the navbar/keybinding gaps in §2 are
closed. Record this explicitly as a probe limitation, not a false pass.

**Reusable helpers already written, verbatim-portable into a new script:**
- `🐍️wgpu-settle-probe.mjs:73-86` — Trunk/dev-server overlay removal before every screenshot (a peer's build
  failure can cover the whole page even on a healthy serve).
- `🐍️wgpu-settle-probe.mjs:93-118` and `🐍️wgpu-dock-probe.mjs:93-104` — console-line-to-verdict classification.
- `🐍️wgpu-hit-probe.mjs:62-72, 89-100, 103-113, 134-156` — dock-plan parsing, rect-to-page-coordinate math,
  pointer-delivery witness, `renderBeginDelta` (count of `"render begin"` lines before/after a gesture) as the
  wgpu equivalent of React's `converged()` polling.
- `🐍️wgpu-probe.mjs:32-57` — the base `snap()` shape (status/alert/canvas size/beacon-present/nodeCount/kinds).

**Skeleton step sequence for `wgpu-journey-probe.mjs`:**
```
boot (await beacon) → for each example (once picker exists, else reload with ?mode=/&example=):
  click picker → waitConverged(previewWindowId, expectedMeshTag) → screenshot
→ reload with ?mode=generate → waitConverged(3-window layout) → click "Add Generation" (dock-plan-derived rect,
  pointer-witness) → waitConverged → reload with ?mode=edit
→ reload with ?role=viewer → for each example: same loop
→ write results.json / console.txt under <ticket>/🗑️generated/wgpu-journey-audit/
```

---

## 5. Ranked lanes to reach "all 8 examples + hover/selection + every window on wgpu"

1. **Input-routing on the served host** (blocks everything else) — `📓️wgpu-tree-row-hit-test-2026-09-12.md`
   §5.4 (L341-356): confirm whether the canonical `🚀️browser-boot/🟦️.ts:152-229` (`wireInput`, verified correct
   by this audit) is actually what's mounted on the live serve, or whether the peer's new
   `🎯️targets/🧊️wgpu/🌐️server` Vite host bypasses it; get one real pointer/keyboard event flowing end-to-end
   into `Ui::dispatch_event`. Nothing below is provable in a browser until this lands.
2. **Example picker wiring** — reconnect `wgpu-shell.rs:3516-3521` to `examples_for_app`
   (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3650`), add the navbar control and row-emission for
   `"playground.navbar.fixture"`/`"shell.example.<id>"` (§1). Files: `wgpu-shell.rs` (navbar render +
   overlay/context-menu population), optionally `🚀️browser-boot/🟦️.ts:45-55` for a `?example=` boot shortcut.
3. **Mode/role navbar UI + keyboard chords** — render `playground.navbar.modes.*`/a new
   `playground.navbar.roles.*` group in `render_navbar_step`, and extend `is_reserved_shell_chord`
   (`wgpu-shell.rs:8585-8596`) with the `mod+alt+e/v/←/→` chords React already declares in `SHELL_KEYBINDINGS`
   (`🏛️ShellHost/🟦️.tsx:7061-7088`). Needs a wgpu-side `switchToPluginApp`/`switchToSessionRole` equivalent
   (currently role is boot-time-only).
4. **Generate-mode panel publication** — guest needs to publish `framework.panel.artifact`/`.catalogue`/
   `.inspection` bodies in generate mode (`wgpu-ui.surface-not-published`,
   `📓️wgpu-resident-budget-settle-2026-09-12.md` L376-379); likely a generation3d plugin-side fix, not
   shell-side.
5. **Camera fit ("Fit graph")** — hardcoded `[4,-4,3]→[0,0,0]` camera needs a fit pass; explicitly owned by
   `📓️node-graph-camera-fit-labels-2026-09-12.md` (not re-read this pass — follow up there).
6. **Preview cancel button wiring** — guest already emits `cancellable:true, cancelAction:"cancelPreviewEval"`;
   needs a chrome control that fires it (no such control found in `wgpu-shell.rs`).
7. **Hover/selection/orbit browser proof** — once (1) lands, extend
   `⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs`'s already-passing native assertions with a live-browser
   probe (§4's skeleton) and resolve the `RUST_MIN_STACK` masking risk (§3) so CI can't silently hide a real
   regression class.
8. **`wgpu-journey-probe.mjs` itself** — write it per §4 once (1)-(3) give it something real to click; land it
   in `<ticket>/🐍️wgpu-journey-probe.mjs` mirroring `🐍️journey-probe.mjs`'s step/output shape.
