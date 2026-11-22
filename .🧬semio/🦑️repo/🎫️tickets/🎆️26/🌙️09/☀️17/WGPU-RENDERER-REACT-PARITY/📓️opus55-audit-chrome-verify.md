# Chrome parity verification (opus55 follow-up)

Auditor: read-only. No source edited, no builds run. This report verifies the ten items left unverified by `📓️opus55-audit-shell-chrome.md`.

Path shorthand:
- `R` = `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/` (React UI targets)
- `UI` = `🧰️framework/🔨️modules/🖱️ui/` (UI framework)
- `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/` (shell renderer base)
- `S` = `E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/` (live wgpu shell source; the crate facades only `#[path]`-mount these)
- `W` = `E/🎯️targets/🧊️wgpu/` (crate facades; not the real source)

Note on wgpu staging: most concerns exist in several duplicate folders under `S`. Each verdict below was checked across all copies that grep found.

## Summary

| # | Item | Verdict |
|---|---|---|
| 1 | Introduction info-box gap | GAP (16 vs 0), plus missing dock/near-miss snapping |
| 2 | Panel drop indicators / anchor zones | GAP (passive fill, active anchor accent, branch-nest highlight missing) |
| 3 | Toasts / notices | MATCH for timing, dismissal, single slot. GAP for width (520 px hard-coded) and stacked status surfaces |
| 4 | Tutorial video, captions, ghost pointer | GAP (video, captions, mute/captions/chapter controls missing; ghost cursor differs) |
| 5 | Animations / transitions | GAP (no chrome motion on wgpu; introduced pulse is test-only; loading/waiting/celebrate borders not painted) |
| 6 | Focus-visible styles | GAP (geometry and colour differ) |
| 7 | Selection marquee in chrome | MATCH (React uses it only in content hosts; earlier GAP claim was wrong) |
| 8 | Maximize / reopen | MATCH (maximize; reopen exists on neither side) |
| 9 | Density / metrics preferences | MATCH (no density control on either side; driver presets mirrored) |
| 10 | Hotkeys | GAP for mod-key platform matching and non-shell override reach. React-only bug for the fullscreen row. 23 of 29 React rows identical |

---

## 1. Introduction info-box gap: GAP

Evidence:
- React: `R/📐️introduction-geometry/🟦️.ts:206` `export const INTRODUCTION_INFO_BOX_GAP_PX = 0;`. Used at `:326` (`resolveIntroductionPlacement`). The React caller passes inset `0` at `R/🎓️introduction/🟦️.tsx:332`.
- React dock snapping: `R/🎓️introduction/🟦️.tsx:105` `INTRODUCTION_DOCK_REACH_PX = 40`, `:108` `INTRODUCTION_CLEAR_RANGE_PX = 120`, applied at `:329` (`dockIntroductionInfoBox`, defined `R/📐️introduction-geometry/🟦️.ts:232`) and `:332` (`clearIntroductionInfoBoxNearMisses`, defined `:276`).
- React max width: `R/🎓️introduction/🟦️.tsx:376` `max-w-sm` (384 px).
- wgpu: `S/🗨️view-overlays/🗨️chrome-overlays-and-tour/🦀️.rs:97` `pub const INTRODUCTION_INFO_BOX_GAP: f32 = 16.0;`, used for the clamp and placement at `:113` and `:116`-`:124`. Max width `:142` = 384.0 (matches). Min `:144` = 240.0 (React has no equivalent constant; unverified).
- wgpu dock reach and near-miss: grep for `dock_introduction`, `near_miss`, `INTRODUCTION_DOCK`, `INTRODUCTION_CLEAR` across `E` returned nothing outside tests. The snapping/clearing behaviour is missing.
- Tests pinned to the 16 px value: `E/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-chrome-overlays-tour/🦀️.rs:667`, `:668`, `:675` (they reference the constant, so changing it updates them, but the assertions encode the 16 px layout).

GAP work items:
1. Set `INTRODUCTION_INFO_BOX_GAP` to `0.0` in `S/🗨️view-overlays/🗨️chrome-overlays-and-tour/🦀️.rs:97`. The clamp at `:113`-`:116` then matches React's flush docking.
2. Port `dockIntroductionInfoBox` (React `📐️introduction-geometry/🟦️.ts:232`) and `clearIntroductionInfoBoxNearMisses` (`:276`) with reach 40 and clear range 120, into the same file. Call them where the info box is placed, as React does at `R/🎓️introduction/🟦️.tsx:329`-`:332`.
3. Update the `🧪️tests/🔬️wgpu-chrome-overlays-tour` assertions if they assumed the 16 px gap.

## 2. Panel drop indicators and anchor zones: GAP

Evidence, React:
- Tab-row insert bar: `UI/🧱️elements/🧭️PanelTabBar/🟦️.tsx:391` `w-0.5 self-stretch rounded-full bg-accent`. The 2 px bar matches wgpu `TAB_INSERT_PREVIEW_WIDTH_PX = 2.0` (`S/🖍️view-paint/🔖️identity-and-types/🦀️.rs:7`). MATCH on the insert bar.
- Passive drop fill: `UI/🔨️modules/🎨️chrome-class-names/🟦️.ts:23` `dropZoneReadyFillClass = "bg-[var(--accent-secondary)]"`. Applied to the tab row while a tab drag is in flight: `UI/🧱️elements/🧭️PanelTabBar/🟦️.tsx` (`rowDropReady && dropZoneReadyClass`).
- Empty anchor zone: `UI/🧱️elements/🖼️Panel/🟦️.tsx:295`-`:313` `PanelEmptyDockZone`. It is registered as a drop row, filled with `dropZoneReadyFillClass`, and turns `text-accent` with `data-active` when it is the active drop anchor (`:304`).
- Branch nest (drop into a tab): `R/🧲️panel-dock/🟦️.tsx:186`-`:201` `computeTabDockDropZone` returns `kind: "child"` for a 30-70% hit on a branch button. Rendered in `UI/🧱️elements/🧭️PanelTabBar/🟦️.tsx:436`-`:437`, `:472` (`bg-accent/20`, `data-drop-nest`).

Evidence, wgpu:
- Drop completion always inserts at the root row: `S/🍱️view-chrome/🖼️chrome-frame-and-main-window/🦀️.rs:251`-`:262` builds `DockTabMoveTarget { ..., parent_path: Vec::new(), index }`. No child/nest target exists.
- Insert preview (2 px accent): `S/⚓️dock/🖼️chrome-frame-and-main-window/🦀️.rs:126`-`:145` and painted in `paint_anchor_tab_bar` (`S/🍱️view-chrome/🖼️chrome-frame-and-main-window/🦀️.rs:330`-`:365`).
- Anchor hit test: `S/🍱️view-chrome/🖼️chrome-frame-and-main-window/🦀️.rs:296`-`:325` `anchor_at_point` (open anchors first, then a 3x3 body grid). Reasonable equivalent to React's `ANCHORS` (8 anchors), not pixel-verified.
- No passive fill and no empty-anchor highlight during a drag. Empty anchors paint only the empty-shell notice (`S/🪀️view-panels/🖼️chrome-frame-and-main-window/🦀️.rs:821`-`:822`).
- The window-canvas drop indicator exists (accent alpha 0.24, `S/🪀️view-panels/🖼️chrome-frame-and-main-window/🦀️.rs:860`-`:863`). This is canvas, not panel dock, so it is not compared here.

GAP work items (wgpu):
1. In `paint_anchor_tab_bar` (`S/🍱️view-chrome/🖼️chrome-frame-and-main-window/🦀️.rs:330`), paint a passive fill (secondary accent) on each row while `dock_tab_drag` is set, mirroring `dropZoneReadyClass`.
2. Paint the closed/empty anchors as drop targets during a drag, with an active accent state for the hovered anchor. React reference: `UI/🧱️elements/🖼️Panel/🟦️.tsx:295`-`:313`.
3. Add a `child` drop kind in `finish_dock_tab_drag` (`:251`): hit-test branch buttons (30-70% centre) and set `parent_path` to the branch. React reference: `UI/🔨️modules/🧲️panel-dock/🟦️.tsx` `computeTabDockDropZone` (`R/🧲️panel-dock/🟦️.tsx:186`), and the nest highlight at `UI/🧱️elements/🧭️PanelTabBar/🟦️.tsx:436`-`:472`.

## 3. Toasts and notices: MATCH on timing and dismissal; GAP on width and stacked status surfaces

Evidence, MATCH:
- Auto-dismiss 4000 ms: React `E/🧱️elements/🏛️ShellHost/🎚️shell-model/🟦️.tsx:64` (`SHELL_TRANSIENT_NOTICE_AUTO_DISMISS_MS = 4000`); wgpu `S/🎚️chrome-state/🗨️chrome-overlays-and-tour/🦀️.rs:813` (`TRANSIENT_NOTICE_AUTO_DISMISS_MS = 4000.0`), expiry at `:808`-`:809`.
- Close button and dismissal: React `E/🧱️elements/🏛️ShellHost/🍞️transient-notice/🟦️.tsx:20`; wgpu `S/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs:829`-`:867` (hit id `shell.notice.close`, dismiss at `:867`).
- Single transient slot on both sides: React `E/🧱️elements/🏛️ShellHost/🟦️.tsx:289` (`transientNotice ? <TransientNotice/>`); wgpu one `transient_notice` field.
- Tone map: React `shell-model/🟦️.tsx:56`-`:61`; wgpu `S/🖍️view-paint/🗨️chrome-overlays-and-tour/🦀️.rs:28`-`:36`.

Evidence, GAP:
- Max width: React `E/🧱️elements/🏛️ShellHost/🍞️transient-notice/🟦️.tsx:17` has no width class. The banner shrink-wraps, bounded by `left-1/2`, so it is roughly half the workbench width. wgpu hard-codes `TRANSIENT_NOTICE_MAX_WIDTH = 520.0` (`S/🖍️view-paint/🗨️chrome-overlays-and-tour/🦀️.rs:19`). The doc comment attributes 520 to React, which is not what the React source says. Top gap `:15` = 8.0 (React uses `top-workbench`; the 8 px value is unverified).
- Stacked status notices: React `E/🧱️elements/🏛️ShellHost/🟦️.tsx` renders `NoticeStack` for bootstrap (`:218`), execution-target (`:225`), artifact-creation (`:232`, `max-w-[28rem]`), route-admission (`:279`, bottom, `max-w-[90vw]`), and install (`:291`). `NoticeStack` is a shared-hairline frame with no gap between cells (`E/🧱️elements/🧱️notice-bands/🟦️.tsx:6`-`:12`). wgpu paints only the transient banner, the plugin-install band (`S/🍱️view-chrome/🗨️chrome-overlays-and-tour/🦀️.rs:37`-`:42`, `render_plugin_install_step` at `S/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs:881`) and the doc-open phase band. No wgpu symbol exists for bootstrap, execution-target, artifact-creation or route-admission status.

GAP work items:
1. Change `TRANSIENT_NOTICE_MAX_WIDTH` (`S/🖍️view-paint/🗨️chrome-overlays-and-tour/🦀️.rs:19`) to the React effective width (about 50% of the workbench, or no cap), and correct the comment.
2. Port the status stacks (bootstrap, execution-target, artifact-creation, route-admission) as hairline-shared cells, using React `NoticeStack` (`E/🧱️elements/🧱️notice-bands/🟦️.tsx`) as the reference for layout and ordering. Start from `ShellHost/🟦️.tsx:218`-`:291`.

## 4. Tutorial video, captions and ghost pointer: GAP

Evidence, React (`R/🎥️tutorial-ui/🟦️.tsx`):
- Bar (`:108`-`:134`): time readout, scrubber, chapter markers (`:123`-`:124`), mute `ui.tutorial.mute` (`:130`), captions toggle `ui.tutorial.captions` (`:131`), record `ui.tutorial.record` (`:134`). Rates `TUTORIAL_RATES = [0.5, 1, 1.5, 2]` (`:41`).
- Captions (`:169`-`:178`): bottom-centre glass, `bottom-double left-1/2 max-w-2xl`, `text-sm`, hidden when captions are off or no narration cue covers the playhead.
- Video overlay (`:191`-`:217`): fixed `<video>` placed by normalised rect, resynced to the playhead past 0.3 s, muted, playbackRate, `object-cover rounded-lg shadow-lg`.
- Ghost pointer (`:227`-`:275`): `size-double`, `opacity-90`, cursor glyph from `--cursor-ghost-<glyph>`, linear interpolation for drag, arc (`introductionDemoArcPoint`) for orbit.

Evidence, wgpu:
- Bar controls (`S/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs:473`): `shell.tutorial.playPause` (`:496`), `shell.tutorial.stop` (`:519`), `shell.tutorial.rate.*` (`:563`, values 0.5/1/1.5/2 match), `shell.tutorial.scrubber` (`:613`). No mute, captions, chapter or record control found.
- Runtime: `S/🪝️window-actions/🎓️tutorial-runtime/🦀️.rs:134`-`:165` renders only the gesture track (`tracks.gestures`). The framework defines narration, video and captions tracks (`F/🔨️modules/🎓️onboarding/📚️tutorial-definition/🦀️.rs:96`, `:99`, `:178`; `F` = `🧰️framework/`), which wgpu does not read for rendering.
- Ghost pointer: an 18 px two-ring dot (`:145`-`:148`), linear for every gesture, no glyph, no arc, no 2x size.
- No wgpu symbol for video, caption or narration rendering (grep across `S` and `W`).

GAP work items:
1. Render the narration track as captions: bottom-centre glass box, `max-w-2xl`, gated by a captions toggle. React reference: `R/🎥️tutorial-ui/🟦️.tsx:169`-`:178`. Add the toggle and mute controls to `navbar-footer-and-overlays/🦀️.rs` next to `shell.tutorial.rate.*`.
2. Render the video track with the `rect` placement, playback synced to the playhead (0.3 s tolerance). React reference: `:191`-`:217`. This needs a wgpu video-frame path or a documented decision to skip it.
3. Chapter markers on the scrubber (React `:123`-`:124`) and the record button (React `:134`) are missing from the bar.
4. Ghost pointer: use the cursor glyph at 2x size, opacity 0.9, arc for orbit, linear for drag. React reference: `:227`-`:275`.

## 5. Animations and transitions: GAP

React chrome motion (verified values):
- Loading border: `--loading-border-duration: 1.6s` (`UI/🎨️styling/🖌️ui/🎨️.css:243`), spin linear plus pulse ease-in-out (`:1379`-`:1380`).
- Waiting border: `3.2s` (`:244`, `:1390`-`:1391`).
- Introduced pulse: `1.6s` (`:245`), ease-in-out infinite (`:1369`).
- Celebrate border: `1.2s` linear (`:246`, `:1415`). Stamp duration `CELEBRATE_STAMP_DURATION_MS = 2400` (`UI/🔨️modules/🎉️celebration/🟦️.ts:4`).
- Scrollbar thumb colour `120ms ease` (`UI/🎨️styling/🖌️ui/🎨️.css:336`); SVG fill `120ms ease-out` (`:1604`); intro ripple `420ms ease-out` (`:1537`).
- Control colour: `interactiveControlTransitionClass = "transition-[color,background-color]"` (`UI/🔨️modules/🖱️interaction-presentation/🟦️.ts:13`). No explicit duration; Tailwind default applies (150 ms, inferred from the framework default, not confirmed in the Tailwind config).
- Dialog and select: `animate-in fade-in-0 zoom-in-95` (`UI/🧱️elements/💬️Dialog/🟦️.tsx:554`, `UI/🧱️elements/🔽️Select/🟦️.tsx:682`). Skeleton pulse `UI/🧱️elements/🦴️Skeletons/🟦️.tsx:22`.
- Tooltip delay 400 ms: `UI/🧱️elements/💡️ChromeControlHint/🟦️.tsx:21`.
- Panel open and close: no transition classes found in the panel dock, so none to match.

wgpu:
- Introduced pulse: `INTRODUCED_PULSE_PERIOD_MS = 1600.0` (`S/⏳️async/🗨️chrome-overlays-and-tour/🦀️.rs:98`) and `introduced_pulse_thickness` (`:51`) are gated by `#[cfg(any(test, feature = "testkit"))]` (`:50`). The pulse is not painted in production.
- Loading, waiting and celebrate borders: no paint code. The only reference is the `celebrateBorderDurationSeconds` theme token read (`S/🎚️chrome-state/🎨️theme-file-import-and-accessibility/🦀️.rs:281`-`:282`) with no consumer in staging.
- Hover, focus and colour fades: no production interpolation found (grep for lerp, ease, tween, fade across `S` returned no production use).
- Tooltip delay matches: `S/🎚️chrome-state/🗨️chrome-overlays-and-tour/🦀️.rs:800` `CHROME_TOOLTIP_DELAY_MS = 400.0`.
- Transient notice: no fade (instant show/hide), React has none either (MATCH).

GAP work items:
1. Paint the introduced pulse in production: remove the `cfg(test)` gate on `introduced_pulse_thickness` (`S/⏳️async/🗨️chrome-overlays-and-tour/🦀️.rs:50`) and drive it from the frame clock at `INTRODUCED_PULSE_PERIOD_MS`. React reference: `UI/🎨️styling/🖌️ui/🎨️.css:1339`, `:1369`.
2. Paint loading, waiting and celebrate borders with the durations in `UI/🎨️styling/🖌️ui/🎨️.css:243`-`:246`, and consume `celebrate_duration_seconds` (`S/🎚️chrome-state/🎨️theme-file-import-and-accessibility/🦀️.rs:282`).
3. Optional, lower priority: 120-150 ms colour fades on hover and focus, and open/close for dialogs, matching the React values above.

## 6. Focus-visible styles on chrome controls: GAP (geometry and colour)

React:
- The compositor sets `data-shared-border-state="focus"` only on `:focus-visible` (`UI/🧱️elements/🗂️WindowChrome/shared-border.ts:145`-`:148`; state list `:47`).
- Stroke colour `var(--active-base)` (`shared-border.ts:89`; CSS `UI/🎨️styling/🖌️ui/🎨️.css:6721`). On an accent segment, `var(--border-emphasized-color)` (CSS `:6722`).
- Native outline suppressed on compositor-owned cells (CSS `:6685`-`:6688`).
- Selected plus focus-visible control underlines 2 px at `0.2em` offset (CSS `:6691`-`:6692`).
- Grip focus: `2px solid var(--color-primary)` with `outline-offset: 3px` (CSS `:7776`-`:7778`).
- Slider thumb: `border-color: Highlight` (CSS `:6708`).

wgpu:
- Keyboard focus ring: `S/🗨️view-overlays/🧭️navbar-footer-and-overlays/🦀️.rs:444`-`:450`. Four edges, each `stroke_hairline * 2`, outset from the hit rect, colour `theme.focus_ring`.
- `theme.focus_ring = accent.with_alpha(chromeFocusRingAlpha)` (`S/🎛️prefs/🎨️theme-file-import-and-accessibility/🦀️.rs:244`-`:245`).
- Driven by `accessibility_focused_control_id` (`S/🕹️input/⌨️keyboard-and-palette/🦀️.rs:323`-`:326`). Keyboard-only, so the focus-visible behaviour is right.

Differences: React draws the stroke along the compositor border in `--active-base` (no alpha), and inverts to the emphasized border on accent surfaces. wgpu draws an outset accent-alpha ring. wgpu has no selected-underline rule and no accent-surface inversion.

GAP work items: in `S/🗨️view-overlays/🧭️navbar-footer-and-overlays/🦀️.rs:444`-`:450`, draw the ring inset on the border with the `--active-base` colour (use `theme.active` if that is the token). Add the accent-surface inversion and the selected-underline rule. React references are above.

## 7. Selection marquee in chrome: MATCH (earlier GAP was wrong)

- React `SelectionMarquee` (`R/🔲️selection-marquee/🟦️.tsx:26`) is used only by content hosts: `E/🧱️elements/🧭️TiledMapHost/`, `🕸️NodeGraph/`, `🌐️World3dHost/`, `🖌️Paint2dHost/`, `🖋️InkCanvasHost/`.
- It is not used by ShellHost, the shell chrome, or the UI canvas (`UI/🧱️elements/🎨️Canvas/`).
- wgpu marquee code lives only in content scenes (`E/🧱️elements/🎞️Scenes/`, `🗣️Interpreter/`). No marquee in the shell chrome staging.
- Both sides agree that the shell chrome has no marquee. Nothing to implement.

## 8. Maximize and reopen windows: MATCH

Maximize, React:
- Toggle: `UI/🧱️elements/🎨️Canvas/🖱️mode-drag-and-window-callbacks/🟦️.tsx:90`-`:91`.
- Keyboard: `ui.window.focus` gated on `canMaximize` (more than one window), `UI/🧱️elements/🎨️Canvas/📥️mode-template-drop-and-contexts/🟦️.tsx:182`-`:192`.
- Reset when geometry or window keys change: `UI/🧱️elements/🎨️Canvas/🧭️mode-layout-state-and-sync/🟦️.tsx:54`, `:63`.

Maximize, wgpu:
- Keyboard: `focus_active_window` (`S/⚓️dock/⌨️keyboard-and-palette/🦀️.rs:154`-`:171`), gated on more than one window, and it sets the active stack before toggling.
- Tab click: `S/🖐️pointer/🖱️pointer-input/🎯️shell-hit/🦀️.rs:236`.
- Reset on input change (owner, mode layout, roster): `S/🚶️session/🧩️session-and-topology/🦀️.rs:220`-`:222`.

Difference (minor, to verify): React resets when the layout geometry key changes. wgpu resets when the input identity changes. Treat as equivalent unless a geometry-only change is observed to keep maximize on wgpu.

Reopen: neither side has a reopen-closed-window feature. The wgpu hits for "reopen" are comments about administration, folders and overlays. The React hits are unrelated. Panel show or hide is covered by the panel-anchor chords on both sides (see item 10).

## 9. Density and metrics preferences: MATCH

- React `UiPreferences` keys (`E/🧱️elements/🎚️UiPreferences/🟦️.ts:41`-`:51`): appearance, layout, driverId, customDrivers, locale, terminology, themeId, customThemes, keybindingOverrides, namedLayouts. No density or metrics key.
- React `layout` is `"desktop" | "tablet"` (`UI/🌓️appearance/🟦️.ts:422`), not density.
- React's closest concept is `UiDriver` (`UI/🧱️elements/🚗️UiDriver/🟦️.tsx:6`-`:30`): labels, labelTier, drag, chrome, gumball, tooltips, hotkeys. Built-ins `DEFAULT_UI_DRIVER` (`:42`) and `COMPACT_UI_DRIVER` (`:44`).
- wgpu mirrors the driver model: `S/🎛️prefs/⚙️settings-and-dock-persistence/🦀️.rs:10`-`:42` (driver document, draft, axis validation), built-ins `S/🎛️prefs/🔖️identity-and-types/🦀️.rs:141`-`:147` (compact and default), custom driver ids `S/🪝️window-actions/🔖️identity-and-types/🦀️.rs:261`.
- The compact built-in config is partly visible in grep output; the full axis set was not fully compared.

Verdict: no density or metrics control on either side. The driver presets are mirrored. Nothing to implement under this item.

## 10. Hotkeys: GAP (mod matching, non-shell override reach); React-only bug (fullscreen row)

Table comparison:
- React `SHELL_KEYBINDINGS` (`UI/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:175`-`:205`): 29 rows.
- wgpu `SHELL_SHORTCUT_ROWS` (`S/🎚️chrome-state/🧰️window-actions-and-commands/🦀️.rs:22`-`:45`): 23 rows.
- All 23 shared rows are identical (diffed, values match exactly, including the eight panel anchors, `ui.window.focus = mod+shift+enter`, time-travel and playground roles).
- The six React rows absent from the wgpu table are handled elsewhere (below).

Rows absent from wgpu table, and their handling:
- `ui.introduction.skip` (escape), `.next` (enter, arrowright), `.back` (arrowleft): handled in the tour branch of `handle_keyboard`, `S/🖐️pointer/⌨️keyboard-and-palette/⌨️keyboard/🦀️.rs:84`-`:99` (Escape dismiss or skip; Enter and ArrowRight next; ArrowLeft back). MATCH.
- `ui.dialog.cancel` (escape), `ui.dialog.submit` (enter): handled by `handle_chrome_dialog_key` (`:54`, Escape and Enter branches at `:126`-`:205`). MATCH on keys; the override path is not applied (see below).
- `os.toggleFullscreen`: React SHELL row is `mod+shift+f` (`:183`), but React's command registry sets `f11` on Windows and Linux and `control+meta+f` on macOS (`E/🛠️ShellHelpers/🎛️command-registry/🟦️.tsx:223`-`:233`). wgpu follows the registry (`S/📄️docs/🧰️window-actions-and-commands/🦀️.rs:134`-`:137`) and fires via `ui.fullscreen.toggle` → `apply_os_command` (`S/🖐️pointer/🖱️pointer-input/🎯️shell-hit/🦀️.rs:213`). In React, `composeControlKeybindings` (`:206`-`:214`) gives SHELL defaults precedence over the registry, and no React code binds `os.toggleFullscreen` by control id. So the React table row contradicts React's own registry. Classification: React-only bug. wgpu matches the registry.

Behaviour gaps on wgpu:
- `mod` matching: React maps `mod` to meta on Apple platforms and ctrl elsewhere (`UI/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:91`-`:92`; text form `UI/🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts:116`). wgpu `key_event_matches_chord` (`S/📄️docs/🧰️window-actions-and-commands/🦀️.rs:57`-`:86`) accepts ctrl XOR meta on every platform. On macOS, ctrl+P and on Windows, the Win/meta key plus P, would fire `mod+p` on wgpu but not in React. Real GAP. wgpu already has the host platform (`command_host_platform`, `:20`-`:22` of that file), so the fix is local.
- Override reach: React `composeControlKeybindings` applies `keybindingOverrides` to any control id unconditionally (`:206`-`:214`). wgpu `shell_shortcut_table` (`S/🎚️chrome-state/🧰️window-actions-and-commands/🦀️.rs:58`-`:66`) applies overrides only to the 23 SHELL rows. App action ids and OS command ids (for example `os.toggleFullscreen`) cannot be remapped on wgpu, and app bindings are not overridable (`match_app_keybinding`, `S/📄️docs/⌨️keyboard-and-palette/🦀️.rs:19`-`:28`).
- Persistence: wgpu persists overrides as preference events (`S/🎛️prefs/🔑️storage-prefs-and-i18n/🦀️.rs:309`-`:317`) and resets a row by removal (`S/🔀️actions/🎬️shell-actions/🎯️dispatch-action/⌨️staging/🦀️.rs:22`-`:29`). Unparsable overrides are dropped, matching React's parse gate (`:66`). MATCH for SHELL rows.

GAP work items:
1. In `key_event_matches_chord` (`S/📄️docs/🧰️window-actions-and-commands/🦀️.rs:57`-`:86`), resolve `mod` to meta on macOS and ctrl elsewhere, using the same host platform source as `command_host_platform`. React reference: `UI/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:91`.
2. Apply overrides to non-SHELL control ids in `shell_shortcut_table` (or a parallel table for app and OS commands), so remaps of `os.toggleFullscreen` and app actions take effect. React reference: `UI/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:206`-`:214`.
3. React-only bug (not a wgpu item): correct or remove the `os.toggleFullscreen: "mod+shift+f"` row at `UI/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:183`, or align it with the command registry. Decide which chord is intended first.

Not verified: browser-level dispatch of `mod` chords and the dialog Enter/Escape override path.
