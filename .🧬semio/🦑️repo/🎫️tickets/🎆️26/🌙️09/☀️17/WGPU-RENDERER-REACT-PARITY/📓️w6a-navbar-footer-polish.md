# 🧭️ W6a — navbar, logo, footer, the Projection chip's missing hit target, and the readiness beacon

Packet W6a of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`. Closes `📓️audit-visual-parity-puzzle3d.md`
§7 items 1, 3, 4, 5, 7 and 8, and answers item 6's question about `session.app.introduction`.
Line numbers are post-edit. Every gate under **VERIFY** was RUN, in the foreground; logs are under
`🗑️generated/w6a-*.txt`.

Reference captures read for this packet: `🗑️generated/wgpu-boot-7/shot-35s.png` (wgpu, 1440×900 dpr 1 —
navbar `⬤ s.puzzle.puzzle3d@1/*#editor · Concrete Forest · Editor · Viewer`, footer
`Remote: detached · No one else is here · Check In · Display · … · Tool · Command · … · Settings ·
Marketplace · History`, a bordered `Projection` pill at each pane's bottom-right) and
`🗑️generated/react-6313/final.png` (React, tour-blurred — usable for band positions, not for glyphs;
every React fact below is read off SOURCE, not that image).

---

## 1 — P0: the `Projection` chip painted but registered no hit target

### Root cause

`ShellState::window_pane_chips` mounted it as `(chip, true, true)` — permanently folded, permanently
**disabled** — and `paint_window_pane_chips_step` ends with `if !disabled { input.register_hit(…) }`.
So the chip went through the whole `render_retained_chrome_group_item_step` walk (its own glass region,
its border, its icon, its label — which is why it is visibly indistinguishable from `Utilities` in the
screenshot) and then registered nothing. That is exactly what the audit measured: 43 hit rows from a
live `dumpChrome`, every other painted chip present, this one absent.

The disable was a deliberate W4b decision resting on a misreading of React: *"React's own contract for a
pane with no body is the disabled toggle"*. React's projection pane never reaches that contract.

### React reference

- `WorldOrbitProjectionSwitchPane` (`🧱️elements/🌐️World3dHost/🟦️.tsx:5189`) passes **no**
  `toggleDisabled` to its `Pane` at all, and is mounted (`:7315`) with
  `spec={worldProjectionSpec}` where
  `worldProjectionSpec = cameraState.projectionSpec ?? (cameraState.projection === "orthographic" ?
  worldProjectionDefaults("orthographic") : worldProjectionDefaults("threePoint"))` (`:6598`) — the spec
  is **synthesised** whenever the camera carries none, so the chip is always live.
- Its press does nothing but fold: `onFoldToggle={() => setFolded((value) => !value)}` over a pane-local
  `useState(true)` (`:5190`).
- Its id is ONE constant on both sides of the fold: `chromeToggleId = toggleId ?? childElementId(id,
  "pane", "fold")` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10006`), i.e.
  `framework.worldOrbit.projection.<segment>.pane.fold`.
- Its icon is **spec-derived**: `icon={worldProjectionSpecIconId(spec)}` (`🌐️World3dHost/🟦️.tsx:5202`
  → `♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3218`), so the folded chip shows which projection is live.
- Its body is `WorldProjectionKindSwitch` (`🎨️r3f/🟦️.tsx:1991`), a `Tree` over
  `createWorldProjectionTemplates` (`:3097`): `Parallel > Orthographic | Axonometric
  (Isometric/Dimetric/Trimetric) | Oblique (Cabinet/Cavalier/Military)` and `Perspective >
  1-Point/2-Point/3-Point/Curvilinear`.
- Selecting a row **dispatches nothing**: `handleProjectionKindChange` (`🌐️World3dHost/🟦️.tsx:6580`)
  only sets `externalPendingProjectionSpec`, and its own comment records why — the single `setProjection`
  consumer takes granular `{field, value}` pairs and has no lossless mapping from a whole spec.

**So the honest answer to the packet's question is: React shows the chip ENABLED, always, and there is
no "disabled look" to mirror.**

### Fix

| site | change |
| --- | --- |
| `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13230` | `Projection => … .then_some((chip, self.projection_pane_folded(window_id), false))` — mounted enabled, so the hit registers through the existing `if !disabled` arm |
| `…:2769`, `:4540` | new `ShellState::projection_pane_folded` and `world_projection_template` maps (React's pane-local `folded` and `externalPendingProjectionSpec`) |
| `…:13160-13220` | new `WorldProjectionTemplate` + `WORLD_PROJECTION_TEMPLATES` (React's taxonomy, depth-first, branch before its leaves), `WORLD_PROJECTION_DEFAULT_TEMPLATE_ID = "three-point"`, `world_projection_template(id)`, `world_projection_column_width` |
| `…:13139` | `WindowPaneChip::icon_id()`'s `Projection` arm now answers the DEFAULT template's icon (`projection-three-point`, not the hard-coded `projection-orthographic`) |
| `…:13251` | new `ShellState::window_pane_chip_icon_id(chip, window_id)` — the live per-pane icon, React's `worldProjectionSpecIconId(spec)`; `paint_window_pane_chips_step` reads it instead of the static one |
| `…:13385` | new `paint_window_projection_step` — the unfolded body: one row per template, growing UP from the chip on the pane's bottom-right, indented per tree level by `theme.tree_indent_per_level`, the selected row `active` |
| `…:16995` | new window-walk phase 11 (after the utilities rail) driving it, so its rows outrank every chip they grow over |
| `…:9679`, `:9685` | new hit arms: `shell.projection.template.<windowId>::<templateId>` selects (view state only, never dispatched — React's own rule) and `shell.projection.fold.<windowId>` flips only that pane's fold |
| `🧫️fixtures/🪟️window-pane-chrome/🔣️.json` | projection row: `icon` → `projection-three-point`, `disabledWithoutBody` → `false`, explicit `controlIdOpen` equal to `controlId` (React's single id) |

**Deliberately NOT done, and why**: nothing moves the camera. `setCamera`'s `camera` object has no
`projection` member and a stray `projection` string makes the whole object fail deserialization and
silently drops the dispatch (`♾️infinite/🌍️world/🦀️.rs:6478`'s own comment), and this renderer's orbit is
perspective-only — it carries `fov_y`, never an orthographic frustum
(`♾️infinite/🌍️world/🦀️.rs:9344`, `WORLD3D_PERSPECTIVE_CAMERA_ZOOM`). React is in the same position for a
different reason (it holds the spec in R3F view state and dispatches nothing), so a view-state-only
selection is the faithful port. The chip's icon is where the selection becomes observable on both
renderers. **Making the orbit follow the selection is the World3d lane's open P1** — W4b §7 already
carries it.

---

## 2 — Navbar centre: raw dialect id, no role badge, no chord hints

### Root cause

`render_navbar_step` phase 3 was one line: `let title = self.session.as_ref().map_or("semio · os",
|session| session.app.id.as_str());`. The Rust twins of React's two title functions **already existed**
(`semio_framework::manifest::app_breadcrumb` / `resolve_app_breadcrumb`,
`🔨️modules/🛂️manifest/🦀️.rs:3746` / `:3751`) and had no navbar caller. React's role BADGE had no twin at
all, and the role chips carried no chord.

### React reference

`🏛️ShellHost/🟦️.tsx:10054-10066` — the centre cluster's `logoAndTitle` is, in order:

1. `<SemioLogo className="size-workbench shrink-0" />` (or the brand's `logoSvg`);
2. `<span data-slot="app-name">` = `appBreadcrumb(resolveAppBreadcrumb(session.app, uiTerminology))`,
   joined by `APP_BREADCRUMB_SEPARATOR = " · "` (`🛠️ShellHelpers/🟦️.tsx:240`, `:1348`, `:1353`);
3. `<span data-slot="surface-role-chip">` = `surfaceRoleChipText(session.app.role, uiLocale)`
   (`🛠️ShellHelpers/🟦️.tsx:2422`, frozen `{en:"Editor"/"Viewer", de:"Editor"/"Betrachter"}`) — a badge
   with **no id and no onClick**.

then `exampleSelectElement`, `modeSwitcherElement`, `roleSwitcherElement` (`:10083-10086`).

Two things the audit's DOM `innerText` read made look like visible chrome and are not:

- **`Example` is screen-reader-only.** `NavbarExampleSelect` (`🖱️ui/🧱️elements/🧪️NavbarExampleSelect/🟦️.tsx:75`)
  renders `<Label id={`${id}.label`} … className="sr-only" />` OUTSIDE the trigger, precisely so the
  trigger's own text is the active example's label and nothing else. The visible control is
  `[file icon] Concrete Forest` — which is what wgpu's `shell_example_control` already paints
  (`icon_id: Some("file")`, label = the selected row's own label). **No visual delta; no change made.**
  The remaining gap is accessibility, not pixels: this target publishes no accessible name for chrome
  controls at all (`🐍️wgpu-accessibility-dump-probe.mjs` found the ARIA mirror covers scene trees only).
- **Modes correctly absent.** `modeSwitcherElement` renders only for `session.app.modes.length > 1`
  (`🏛️ShellHost/🟦️.tsx:9178`); puzzle3d's editor declares `modes: ["edit"]`, and
  `shell_mode_controls` already gates on `app.modes.len() < 2`.

`Editor ⌘️⌥️E` / `Viewer ⌘️⌥️V` IS visible chrome: `ButtonGroupItem` paints a `ControlHotkeyBadge`
(`🔳️ButtonGroup/🟦️.tsx:117` → `🔨️modules/⌨️control-hotkey-presentation/🟦️.tsx:32`) whenever the active
driver's `hotkeys` axis is `"inline"`, which `DEFAULT_UI_DRIVER` sets (`🚗️UiDriver/🟦️.tsx:41`). The chord
comes from `useControlHotkey(id)` → `formatKeybindingShortcut`
(`🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts:40`) over `SHELL_KEYBINDINGS`, and
`playground.navbar.roles.*` are the only two per-button rows in that table by construction
(`🕹️control-keybinding-context/🟦️.tsx:162`: `AppRole` is a closed two-value union).

### Fix

| site | change |
| --- | --- |
| `🐚️Shell/…:12717` | new `SHELL_NAVBAR_SESSIONLESS_TITLE` and `shell_navbar_title(app, terminology_id)` — React's pair, through the SAME `semio_framework::manifest` functions, so `" · "` is one constant and a terminology override (`reuse` → `Entwerfen mit Bestand · Aggregator`) lands identically |
| `🐚️Shell/…:12731` | new `shell_surface_role_chip_text(role, is_de)` — `surfaceRoleChipText`'s frozen pair |
| `🐚️Shell/…:12744` | new `shell_control_hotkey_badge(table, control_id)` — `ControlHotkeyBadge`'s text, off this session's own REMAPPABLE `shell_shortcut_table`, through the existing `format_keybinding_shortcut` (`:12526`, already the Rust twin of `formatKeybindingShortcut`) |
| `🐚️Shell/…:17480` | navbar phase 3 paints the breadcrumb title, carried in `cursor.window` as a `UiText` across paint opportunities |
| `🐚️Shell/…:17509` | **new navbar phase 11** — the role badge, painted as a `ChromeGroupItem` with an EMPTY control id, so it registers no hit (React's badge is not a control); phase 3 → 11 → 4 |
| `🐚️Shell/…:12805` | `shell_role_controls` takes the shortcut table and appends `" {badge}"` to each label |
| `🐚️Shell/…:17607` | its call site passes `&self.shortcut_table()` |

---

## 3 — Logo: a black disc where React paints the brand mark

### Root cause

Not a missing asset. `semio-logo` **is** in the icon atlas on both doors — the native/wasm builder
appends it last (`🖼️IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs:113`) and the page door rasterises the same SVG
(`🎬️renderer-boot/🟦️.ts:95`), both deliberately skipping the white tint-mask pass every Lucide cell gets
(`rasterize_svg(svg, tint_mask = id != "semio-logo", …)`, `:111`). It is the one cell in the atlas kept
in its real colours — navy disc `#001117`, orange `#fa9500`, red `#ff344f`, teal `#34d1bf`.

The navbar then painted it through `chrome_icon(draw, icons, "semio-logo", …, theme.text)`, and
`UI_SHADER`'s textured branch is `icon.rgb * in.color.rgb`
(`🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs:144`). Multiplying four hues by the near-black chrome text colour
collapses all of them — which is why the pixel sample at (219,14) read `#000102`, i.e. the logo's own
silhouette in text colour. It was also **19.2 px** (`btn_h - gap_standard`) where React's is 24.

### React reference

`className="size-workbench"` → `--size-workbench: calc(1.5 * var(--size-small))` and
`--size-small: calc(5 * var(--ui-spacing))` (`🖱️ui/🎨️styling/🖌️ui/🎨️.css:736-737`) = **7.5 ui-spacings**
= 24 px at this target's `UI_SPACING_COMPACT_PX` of 3.2, in a 32 px navbar, followed by `gap-single`
(1 ui-spacing) before the title.

### Fix

`🐚️Shell/…:11790` — new `LOGO_UNTINTED: Rgba = Rgba::new(1.0, 1.0, 1.0, 1.0)` (the identity tint for a
full-colour cell) and `SHELL_NAVBAR_LOGO_UI_SPACING: f32 = 7.5`, both with the token derivation written
down; `🐚️Shell/…:17470` (navbar phase 2) paints `theme.gap_standard * SHELL_NAVBAR_LOGO_UI_SPACING`
untinted. `theme.gap_standard` IS `1.0 × --ui-spacing` (`GAP_STANDARD_UI_SPACING`), so the size is
token-derived, not a magic number.

---

## 4 — Footer composition and order

### Root cause

Two separate things, both in `render_footer_step`:

1. **All the pills were painted ahead of the bands off one running cursor.** Phase 3 walked
   `render_sync_status_and_checkin` from `cursor.x = padding` and handed its final `x` to phase 5 as
   `lead_x`, so the whole footer read `pills … then bands`, i.e.
   `Remote: detached · No one else is here · Check In · Display · … · Tool · Command · … · Settings ·
   Marketplace · History`.
2. **`Check In` is not footer chrome on either renderer.** React's `#s-checkin` is a row inside the
   HISTORY panel's own tree (`framework.history.checkin`, `🏛️ShellHost/🟦️.tsx:9016`), gated by
   `!canCheckIn(session.app.role)` and reachable only with that panel open. There is no footer
   check-in chip in React at all.

### React reference

`footerItems` (`🏛️ShellHost/🟦️.tsx:10610-10635`) is
`[bottom-left tabs] [bottom-middle tabs, centered] [fill] [<PresenceBar id="s-presence-peers">]
[bottom-right tabs]`, and `#s-sync-status` is the bottom-left band's own **sync tab**
(`frameworkSyncTab`, `:9070` — the status pill lives on that tab's folded chrome button). W3b's measured
DOM at vw 1024 confirms the resulting run: `Display` x3 · `Remote: detached` x78 | `Tool` x439 ·
`Command` x497 (centred on 512) | `No one else is here` x660 | `Settings` x764 · `Marketplace` x845 ·
`History` x946–1021.

### Fix

| site | change |
| --- | --- |
| `🐚️Shell/…:12185` | `render_sync_status_and_checkin` → **`render_footer_pills_step`**: two pills only, each positioned from its own band's anchor rather than a running cursor. The `#s-checkin` branch, its `framework.checkin` `open` event and the `uncommitted_count`/`session` parameters are gone |
| `🐚️Shell/…:12240` | new `FooterPillAnchors { sync_left, presence_right }` |
| `🐚️Shell/…:16820` | new `ShellState::footer_pill_anchors(atlas, theme, width)` and `footer_band_width(atlas, theme, anchor)` — measured on the painting atlas, so the anchors are known before the first chip is drawn |
| `🐚️Shell/…:16840` | `footer_tab_row_rect` drops its `lead_x` parameter: bottom-left now always opens at `theme.padding_standard` (the wgpu-only utility rail it used to make room for went in W4b) |
| `🐚️Shell/…:17915` | footer phase 3 calls the new stepper with those anchors and no longer advances `cursor.x` |
| `🧫️fixtures/🪟️window-pane-chrome/🔣️.json` | `footerComposition` gains React's measured `order`, the three band descriptors, and `s-checkin` in `forbiddenControlIds` |

**Consequence, reported not hidden**: with the chip gone, nothing in the wgpu shell now dispatches
`framework.checkin`'s `open`, so `handle_checkin_action` (`🐚️Shell/…:7296`) and the
`checkin_dialog_draft` keyboard card (`:11104`) are reachable only from a future caller. React's own home
for the trigger is the History panel's `framework.history.checkin` row, and this renderer's History panel
body comes from the shared framework tree (`🔌️plugin/🦀️.rs:10520`), which carries
`framework.history.commitCheckpoint` but no check-in row — **hand-off below**. Auto check-in
(`poll_auto_checkin`, `dispatch_checkpoint`) is untouched and still fires.

---

## 5 — `data-semio-os-ready` / `data-semio-os-error` on the wgpu page

### Root cause

Genuinely unwired. React writes both on `document.documentElement` from its `🔖️ReadinessBeacon`
`useEffect` (`🏛️ShellHost/🟦️.tsx:10658-10700`): `root.dataset.semioOsReady = pluginFilter ?? "unknown"`
on a resolved session, `semioOsError` on a fault, `semioOsNotFound` on a host-mode 404, each **deleting**
the other two, all cleared on unmount. The wgpu page set none, so a 45 s clean boot still read
`data-semio-os-ready = null` and every reader — `🐍️*-console-dump-probe.mjs`, `🐍️parity-interact-probe.mjs`,
and `🌎️hub/📦️packages/🦀️rust/📜️script.ts:11230`'s `page.locator("[data-semio-os-ready]")` e2e gate — had to
fall back to a fixed sleep on this target alone.

### Fix

The beacon lives in `🧭️boot-descriptor/🟦️.ts` (new `🔖️ReadinessBeacon` region:
`WGPU_READINESS_BEACON_UNKNOWN_PLUGIN`, `wgpuReadinessBeacon(root, beaconId)`), beside the page realm's
other reads and for the same reason — the frame Worker owns no `document`. It is a **pure** module, which
is what makes the law able to import it: `🚀️browser-boot/🟦️.ts` mounts the shell at top level on import
and cannot be imported by a suite.

`🚀️browser-boot/🟦️.ts` stamps it at the milestones the page owns:
`const beacon = wgpuReadinessBeacon(document.documentElement, descriptor.pluginVariant)` (`:427`),
`beacon.ready()` in the transport's `onReady` (`:440`, the Worker has reported its surface live),
`beacon.error()` in `onFault` (`:464`), `beacon.clear()` on `pagehide` (`:511`), and
`wgpuReadinessBeacon(document.documentElement, WGPU_READINESS_BEACON_UNKNOWN_PLUGIN).error()` in the
mount `catch` (`:529`) — React's own `"unknown"` fallback, reached here only when no descriptor could be
built. `descriptor.pluginVariant` is the twin of React's `pluginFilter`.

`semioOsNotFound` is deliberately absent: it is React's host-mode `shellRoute.kind === "notFound"` arm,
and this door has no host route.

**Not covered**: `🎬️renderer-boot/🟦️.ts` (the embeddable multi-mount door) has no readiness milestone of
any kind, so it stamps nothing. React mirrors per-shell `data-shell-ready` onto each
`[data-shell-id]` scope root for exactly that case — hand-off below.

---

## 6 — The tour's `session.app.introduction` IS populated

Verified, no change needed — and the audit already contains the live proof: a `dumpChrome` of the running
wgpu boot returned `shell.tour.skip` and `shell.tour.next` hit rows at generation 25
(`📓️audit-visual-parity-puzzle3d.md` §1). `auto_start_introduction` (`🐚️Shell/…:16480`) is the only
production arming path and reads `session.app.introduction.is_some()` directly, so those rows could not
exist unless `has_introduction` was already true on this target.

Chain confirmed statically too: the puzzle descriptor authors
`manifest.apps["s.puzzle.puzzle3d@1/*#editor"].introduction` with a `welcome` step
(`✏️s/🔌️plugins/🧩️puzzle/🔣️.json`), `AppDefinition::introduction` is a plain
`#[serde(skip_serializing_if = "Option::is_none")] Option<IntroductionDefinition>`
(`🔨️modules/🛂️manifest/🦀️.rs:3578`) with no boot-descriptor stripping between the descriptor and the
shell, and a new law now parses the authored record through that very type.

**The open question from the audit's §1 is therefore NOT "is the introduction missing" — it is why the
tour's rows exist while the same-moment screenshot shows no tour overlay.** That is an overlay-lane
question (the tour paints into the OVERLAY draw list, which is also where W5c's world pass work sits),
not a session-data one. Left for the coordinator's live look, unchanged by this packet.

---

## Tests

`🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs` — **new, 8 laws, all run**:

| law | what it pins |
| --- | --- |
| `every_painted_pane_chip_registers_a_hit_target` | **the hit-target law** — every chip a world pane paints is in the ledger unless the fixture allows it to lose its body; `shell.projection.fold.<id>` present by name |
| `the_projection_chip_folds_its_own_pane_and_switches_its_template` | one id on both sides of the fold, fold is per pane, the unfolded body paints React's whole taxonomy in React's order, the default reads `three-point`, a selection moves the chip's icon and only that pane's |
| `the_navbar_cluster_walks_to_completion_and_keeps_its_bands` | the three new navbar phases still terminate, leave no retained fault, and keep top-left's band at the navbar's own padding ahead of the cluster |
| `the_navbar_title_is_the_apps_breadcrumb_not_its_dialect_id` | **the title-formatter law** — `semio · puzzle · 3d`, the `reuse` terminology override, the `" · "` separator, no dialect id, and the empty-breadcrumb contract |
| `the_navbar_role_badge_reads_reacts_frozen_pair` | `Editor`/`Viewer` en, `Editor`/`Betrachter` de |
| `the_navbar_role_chips_carry_their_inline_hotkey_badge` | both role chips carry a chord, through the same formatter the palette uses, following a REMAP rather than the frozen default; a chordless control carries none |
| `the_footer_pills_band_with_reacts_own_order` | **the footer order fixture** — leading tabs → sync pill → … → presence pill → trailing tabs, no overlaps, no `s-checkin` |
| `the_puzzle3d_app_carries_the_introduction_the_tour_arms_on` | the authored introduction deserialises through `IntroductionDefinition`, and `should_auto_start_introduction` arms on it |

`🧪️tests/🔖️wgpu-readiness-beacon/🟦️.ts` — **new, 2 cases** (the **ready-attribute law** in the TS unit
suites): both attributes, each clearing the other, the plugin id as the value, and React's `"unknown"`
fallback. Registered in `🧪️tests/🎚️config/🟦️.ts`'s `include` and in `BrowserWorkerTestScript`
(`📦️packages/🟦️typescript/📜️script.ts:285`), so `test-browser-worker` runs it.

Updated:
- `🧫️fixtures/🪟️window-pane-chrome/🔣️.json` — projection row (icon, enabled, open id) and the whole
  `footerComposition` order block.
- `🌓️appearance-tour-and-footer-pills/🦀️.rs` — the source-reading pill law re-pointed at
  `render_footer_pills_step`, and it now asserts `s-checkin`'s **absence** from the footer.
- `🔬️wgpu-shell-chrome-parity/🦀️.rs:234`, `🔬️wgpu-panel-anchor-model/🦀️.rs:930` — the two changed
  signatures.
- `🪟️wgpu-window-pane-chrome/🦀️.rs` — `split_pane_shell` is `pub(super)` (shared with the new suite) and
  the stale "Projection is mounted DISABLED" docstring corrected.

## VERIFY (all foreground, all RUN)

| gate | result | log |
| --- | --- | --- |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | **0 errors**, 61 warnings (was 62 before this packet) | `w6a-native-check.txt` |
| `cargo check -p … --lib --target wasm32-unknown-unknown -j 4` | **0 errors** | `w6a-wasm-check.txt` |
| `cargo check -p … --lib --tests -j 4` | **0 errors** | (folded into the test runs below) |
| `cargo test -p … --lib -- --test-threads=1 navbar_footer_parity_tests` | **8 passed / 0 failed** | `w6a-test-navbar-footer.txt` |
| `cargo test -p … --lib -- --test-threads=1 navbar_footer_parity window_pane_chrome chrome_parity panel_anchor` | **86 passed / 1 failed** | `w6a-test-neighbours.txt` |
| `cargo test -p … --lib -- --test-threads=1 shell::` | **330 passed / 10 failed** | `w6a-test-shell-serial.txt` |
| `nx run-many -t lint,check-browser-worker,check-frame-worker,generate-browser-boot,generate-frame-worker,test-browser-worker -p @semio-tech/framework-renderer-wgpu` (`NX_DAEMON=false`) | lint/check/generate **all ✔**; `test-browser-worker` **89 passed / 2 failed** | `w6a-ts-gates.txt`, `w6a-generate.txt` |

### Every failure above is pre-existing, and here is the attribution

- **`shell::` 10 failures** — byte-for-byte the set W3b and W4b both attributed (W4b's baseline on this
  filter was 310 passed / 10 failed; this run is 330 / 10, i.e. +20 passes and no new failure):
  `window_silhouette_border_emits_notched_outline_segments`,
  `directory_home_bootstrap_retries_…`, `the_overlay_row_steps_clear_of_an_open_floating_panel`,
  two `shell_document_retirement_*`, two `tool_run_panel_*`, two `ui_prefs_themes_i18n_*` (process-global
  env locks under a shared process), `window_measures_overlay_paints_…`.
- **The 1 failure in the narrower run** is `the_overlay_row_steps_clear_of_an_open_floating_panel`, which
  asserts the World3d cancel control is flush at 7.2 while `surface_overlay_controls_for` leads it by the
  status pill at 151.2 — `world3d_status_pill_width`'s lead, W3b's own recorded hand-off, in a file this
  packet never opened.
- **The 2 TS failures** are `⏱️wgpu-ui-turn-budget` and `⏱️wgpu-worker-step-budget`, both SOURCE-reading
  laws over `🎞️frame-worker/🟦️.ts` and `⏱️turn-budget/🟦️.ts` — two files a peer is editing right now
  (mtimes 09:34 and 06:51, both `M` in the tree, neither touched here). The `phaseUs=${step.elapsedUs}`
  string the second one demands is absent from BOTH the working tree and `HEAD`, i.e. it is a law written
  ahead of its production code, not a regression.

## What the coordinator must confirm LIVE

Nothing below was seen in a browser: this packet ran no `activate-*`, no `framework-renderer-wgpu:wasm`,
no trunk. After the next real boot of `?plugin=puzzle3d`, in order:

1. **`data-semio-os-ready` is set.** This is the cheapest check and it gates the others: the probes can
   stop sleeping. Expect `document.documentElement.dataset.semioOsReady === "puzzle3d"` shortly after the
   canvas paints, `semioOsError` absent. On a quarantined renderer expect the inverse.
2. **The navbar centre reads `[colour logo] semio · puzzle · 3d  Editor  [file] Concrete Forest  Editor ⌘️⌥️E  Viewer ⌘️⌥️V`** —
   a four-colour mark ~24 px tall (NOT a black disc), the humanised title, the small `Editor` badge, and
   the two role chips each ending in a chord. Watch the total cluster width: the badge and two chords add
   roughly 110 px to a cluster that was ~250 px, and this renderer still LEFT-packs the centre cluster
   where React centres it (W3b §4's declared divergence), so on a narrow viewport it will reach the
   trailing band sooner than React's does.
3. **The footer reads `Display · Remote: detached | Tool · Command | No one else is here · Settings ·
   Marketplace · History`** — `Display` first, no `Check In` anywhere, the presence pill immediately left
   of `Settings`.
4. **Press `Projection` on each pane.** It must now respond at all (this is the P0): the chip lights, a
   15-row indented tree grows upward from it, `3-Point` reads selected, and picking `Orthographic` changes
   the folded chip's icon on THAT pane only. The 3D view will **not** change — that is expected and is the
   World3d lane's open P1, not a regression. Re-run `🐍️wgpu-chrome-hits-probe.mjs` and confirm
   `shell.projection.fold.<windowId>` is now one of the rows (the audit's 43 becomes 45 — one per pane).
5. **The tour.** With `data-semio-os-ready` available the audit's §1 ambiguity can finally be settled
   cheaply: take a headed capture at t≈1–5 s on a profile with
   `ui.introduction.seen.s.puzzle.puzzle3d@1/*#editor` cleared. `session.app.introduction` is confirmed
   present (§6), so if no overlay paints, the fault is in the overlay draw list, not the session.
6. **Boot cost.** The window walk gained one phase per pane and the navbar one phase; the per-pane phase
   returns immediately while the pane is folded, but `world_projection_column_width` measures 15 labels on
   the atlas on each opportunity of an UNFOLDED pane. Watch `declare_boot_subphase` and the present
   watchdog if anyone leaves the pane open.

## Hand-offs (ordered by what they cost)

1. **P1 — `#s-checkin` now has no trigger in this shell.** React's is the History panel's
   `framework.history.checkin` row; the shared framework history body (`🔌️plugin/🦀️.rs:10520`) carries
   only `framework.history.commitCheckpoint`. Adding the check-in row there would converge BOTH renderers
   on one source — React currently builds its own history tree in TSX. Whoever owns the history lane
   should take it; the wgpu controller (`framework.checkin`) and its message card are already written and
   waiting for a caller.
2. **P1 — the projection selection does not reach the camera.** `World3dState`'s `OrbitController` has no
   orthographic frustum and `setCamera`'s `camera` value has no `projection` member. This is W4b §7's own
   P1, now with a live UI in front of it.
3. **P2 — `🎬️renderer-boot/🟦️.ts` publishes no readiness signal.** The embeddable multi-mount door has no
   `onReady`/`onFault` shape to hang one on. React solves the multi-mount case with per-shell
   `data-shell-ready` on each `[data-shell-id]` root; this door would need the same, plus a milestone to
   fire it.
4. **P2 — chrome controls have no accessible names on this target.** React's `Example` label is `sr-only`
   and reaches a screen reader; the wgpu ARIA mirror (`🚀️browser-boot/🟦️.ts`'s `accessibilityMirror`)
   projects scene trees only, so no navbar or footer chip is named at all. Not a visual delta, which is
   why it is not fixed here, but it is a real parity gap.
5. **P2 — the centre cluster is still left-packed**, not centred as React's `centered: true` navbar item
   is (W3b §4's declared divergence). Item 2's added width makes it more visible than it was.
