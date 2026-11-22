# Chrome parity re-audit (opus55, read-only)

Auditor: read-only. No source edited, no builds, no git. Re-verified every gap in `📓️opus55-audit-shell-chrome.md` and `📓️opus55-audit-chrome-verify.md` against the current tree, using the claims in `📓️opus55-fix-chrome-gaps.md`, `📓️opus55-fix-chrome-motion-tutorial.md`, `📓️opus55-fix-shell-panels.md`, `📓️opus55-hub-workspace-first-run.md`. Each verdict was checked for a production caller (not only a definition or test).

Caveat: every fix note says it was written without compiling or running. Nothing below proves compilation or behaviour. "FIXED" means the code exists and is reachable from production code in the current tree.

Path shorthand:
- `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/`
- `S` = `E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/`
- `W` = `E/🎯️targets/🧊️wgpu/` (crate facades and crate-level code)
- `UI` = `🧰️framework/🔨️modules/🖱️ui/`
- `R` = `UI/🎯️targets/⚛️react/`
- `D` = `UI/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/`

## Verdicts

| # | Gap (source audit) | Verdict | Evidence (React / wgpu, production callers) |
|---|---|---|---|
| 1 | P0 first-run flow (HubFirstRun) | FIXED | React `E/🧱️elements/🎓️HubFirstRun/🟦️.tsx`. wgpu twin `E/🧱️elements/🎓️HubFirstRun/🎯️targets/🧊️wgpu/🦀️.rs` (`hub_first_run_stage_done` :197, `hub_first_run_controls` :339). Called from `S/🚶️session/💬️chrome-dialog-and-tooltips/🦀️.rs:15-16` (`chrome_tour_frame_begin`), which runs in the frame at `S/🪀️view-panels/🖼️chrome-frame-and-main-window/🦀️.rs:354`. Reconcile at `S/📄️docs/🖼️chrome-frame-and-main-window/🦀️.rs:66`. Replay at `S/🔀️actions/📨️command-dispatch/📇️directory-and-hub/🦀️.rs:124`. Controls in `E/🧱️elements/🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs:847`. |
| 2 | P0 sign-in UI | FIXED (pre-existing) | Sign-in labels and form (Hub, Email, Password, Submit) in `E/🧱️elements/🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs:115-126`. Hub panel routed at `S/🚶️session/🧰️window-actions-and-commands/🦀️.rs:127`. |
| 3 | Intro info-box gap 16 vs 0 | FIXED | React gap now from schema: `R/📐️introduction-geometry/🟦️.ts:207`. wgpu `introduction_info_box_gap()` reads the same schema, `S/🗨️view-overlays/🗨️chrome-overlays-and-tour/🦀️.rs:98`. No `INTRODUCTION_INFO_BOX_GAP: 16.0` left. |
| 4 | Intro dock reach 40 / clear range 120 | FIXED | React `R/🎓️introduction/🟦️.tsx:20,323-326`. wgpu `dock_introduction_info_box` (:~145-190), `clear_introduction_info_box_near_misses`, `settle_introduction_box` (:~196-212), called from `chrome_tour_layout` at `S/🗨️view-overlays/🗨️chrome-overlays-and-tour/🦀️.rs:457`. Divergence: wgpu uses window content rects, React uses window silhouette lines. |
| 5 | Panel drop: passive fill, empty anchors, branch nest | FIXED (placement diverges) | Passive fill and nest highlight `S/🍱️view-chrome/🖼️chrome-frame-and-main-window/🦀️.rs:394-396`. Nest target `S/⚓️dock/🖼️chrome-frame-and-main-window/🦀️.rs:146`. Child drop `dock_tab_nest_branch` :301 and `finish_dock_tab_drag` :251-256. Empty zones `paint_empty_anchor_drop_zones` :345-356, called at `S/🪀️view-panels/🖼️chrome-frame-and-main-window/🦀️.rs:880`. Zone rect `empty_anchor_drop_zone_rect` `S/⚓️dock/🖼️chrome-frame-and-main-window/🦀️.rs:167`. React `UI/🧱️elements/🖼️Panel/🟦️.tsx:295-313`. Divergence: zones sit at the dock body edge, not inside the anchor row. |
| 6 | Toasts: width 520 hard-coded | FIXED | `TRANSIENT_NOTICE_MAX_WIDTH` gone. Width is `width * max_width_fraction` at `S/🍱️view-chrome/🗨️chrome-overlays-and-tour/🦀️.rs:53`. |
| 7 | Toasts: status stacks (bootstrap, execution-target, artifact-creation, route-admission) | PARTIAL | Wired: `render_status_stacks_step` `S/🪧️view-chrome-overlays/🧭️overlay-steps/🦀️.rs:734`, from `S/🪀️view-panels/🖼️chrome-frame-and-main-window/🦀️.rs:500`. Layout `status_stack_layout` `S/🍱️view-chrome/🗨️chrome-overlays-and-tour/🦀️.rs:214`. Route admission `shell_route_admission` :347, used at `S/🪧️view-chrome-overlays/🧭️overlay-steps/🦀️.rs:709`. React `E/🧱️elements/🏛️ShellHost/🟦️.tsx:218-291`. Missing: bootstrap byte progress, execution-target progress meter, creation elapsed line. |
| 8 | Tutorial bar: mute, captions, chapter ticks, record | FIXED | `S/🪧️view-chrome-overlays/🧭️overlay-steps/🦀️.rs:193-195` (mute, captions toggles), :247 (chapter ticks via `tutorial_chapter_tick_rect`, `S/🖍️view-paint/🎓️tutorial-runtime/🦀️.rs:52`), :22-24 (record), :108 (add chapter). |
| 9 | Tutorial captions | FIXED (left-aligned) | Phase `TutorialCaptions` `S/🪀️view-panels/🖼️chrome-frame-and-main-window/🦀️.rs:517-519`. Paint `S/🗨️view-overlays/💬️chrome-dialog-and-tooltips/🦀️.rs:588`. Multi-line text is left-aligned; React centres it (React `R/🎥️tutorial-ui/🟦️.tsx:169-178`). |
| 10 | Tutorial video | PARTIAL | Browser path FIXED: `tutorial_video_cue` `S/🪀️view-panels/🖼️chrome-frame-and-main-window/🦀️.rs:915`, pushed into the frame at `W/🧊️renderer/🖥️frame-presentation/🎬️present-step-inner/🦀️.rs:426`, parsed in `W/🚚️browser-frame-transport/🟦️.ts:692-695`, played by `W/🌐️browser-host/🟦️.ts:364` (`accept`). Native: no video decoder, so nothing paints (documented). |
| 11 | Ghost pointer (glyph, 2x, arc) | FIXED | Glyph selection `S/🖍️view-paint/👻️tutorial-ghost-glyphs/🦀️.rs:89`. Arc `tutorial_ghost_arc_point` `S/🖍️view-paint/🎓️tutorial-runtime/🦀️.rs:84`. Paint `:191-206`. |
| 12 | Chrome motion: hover fade | FIXED | `chrome_hover_amount` at `S/🍱️view-chrome/🧱️chrome-paint-and-navbar/🦀️.rs:74,785`, backed by `W`/`D` `chrome_transition` (`D/🎞️chrome-motion/🦀️.rs:116`). |
| 13 | Chrome motion: dialog enter | FIXED (close not painted) | `chrome_enter` `S/🗨️view-overlays/💬️chrome-dialog-and-tooltips/🦀️.rs:292`, with `note_motion_pending` :301. Dialog close fade and zoom 95 to 100 are not painted (documented). |
| 14 | Chrome motion: introduced pulse | FIXED | Shader pulse `0.85 ± 0.15·cos` in `UI/🖌️render/✨️shader-contract/🧵️ui-and-vector-families/🦀️.rs:121,145`. Introduced border routed at `D/🖊️draw-list-queries-and-push/🦀️.rs:258`. `introduced_pulse_opacity` (`D/🎞️chrome-motion/🦀️.rs:44`) has no production caller; the shader does the work. |
| 15 | Chrome motion: loading, waiting, celebrate borders | FIXED | Loading and waiting routed `D/🖊️draw-list-queries-and-push/🦀️.rs:259`. Celebrate `UI/🎯️targets/🧊️wgpu/🖌️paint/📶️progress-celebrate-and-widget-painters/🦀️.rs:46-49`. Reduced motion stamped at `S/🪀️view-panels/🖼️chrome-frame-and-main-window/🦀️.rs:216-218`. |
| 16 | Scrollbar thumb fade, svg fill 120 ms, intro ripple | OPEN (accepted, out of scope) | Not chrome controls on wgpu per fix note. |
| 17 | Focus ring: geometry and colour | PARTIAL | Now an on-border hairline: `S/🗨️view-overlays/🧭️navbar-footer-and-overlays/🦀️.rs:444-445`, `theme.selected` with `SharedBorderState::Focus`. React `UI/🧱️elements/🗂️WindowChrome/shared-border.ts:145-148`. Still missing: accent-surface inversion (`--border-emphasized-color`, React CSS `:6722`) and the selected-plus-focus 2 px underline (React CSS `:6691-6692`). |
| 18 | Hotkeys: `mod` platform matching | FIXED | `key_event_matches_chord` uses `host_platform_uses_meta()` at `S/📄️docs/🧰️window-actions-and-commands/🦀️.rs:80`. React `UI/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:91`. |
| 19 | Hotkeys: overrides reach non-SHELL ids | FIXED | `shell_shortcut_table` overrides `S/🎚️chrome-state/🧰️window-actions-and-commands/🦀️.rs:58-59`. `command_active_chords` :68-69. `match_app_keybinding` override `S/📄️docs/⌨️keyboard-and-palette/🦀️.rs:26`. |
| 20 | Hotkeys: reserved-chord check ignores overrides | PARTIAL | `is_reserved_shell_chord_in(&shell_shortcut_table(&HashMap::new()), …)` at `S/🖐️pointer/🧱️chrome-paint-and-navbar/🦀️.rs:82` and `shell_shortcut_for_in(&shell_shortcut_table(&HashMap::new()), …)` at `S/🖐️pointer/🧰️window-actions-and-commands/🦀️.rs:102`. A remapped SHELL chord is not treated as reserved or shown as the shortcut badge. |
| 21 | React bug: `os.toggleFullscreen: mod+shift+f` row | FIXED | No `mod+shift+f` left under `UI/🔨️modules/🕹️control-keybinding-context/`. |
| 22 | Panel reopen / maximize / selection marquee / density / tooltips | MATCH (no action) | Verify notes items 7-9 confirmed. Tooltip delay 400 ms at `S/🎚️chrome-state/🗨️chrome-overlays-and-tour/🦀️.rs:800`. |
| 23 | Panel TaskManager (runtime actors, tasks, requests) | PARTIAL | Wired: `sync_runtime_actors` and `sync_runtime_install_tasks` at `S/🚶️session/🧰️window-actions-and-commands/🦀️.rs:118-119`. Requests drained at `S/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🦀️.rs:250-251`. Kernel side `W/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🎛️runtime-observation/🦀️.rs`, called from `…/🚀️turn-dispatch-and-pool-runner/🦀️.rs:467`. Missing: tool-run tasks are not fed; the actor table is a `ComponentScene` Table, not the retained Table widget. |
| 24 | Route-not-found page | FIXED | `build_route_not_found_ui` `S/📂️doc-open/🧭️route-not-found/🦀️.rs:54`, mounted at `S/🚶️session/🧰️window-actions-and-commands/🦀️.rs:122-123`, painted at `S/🗨️view-overlays/💬️chrome-dialog-and-tooltips/🦀️.rs:42`. |
| 25 | UtilityTree window bar grouping | FIXED | `window_utility_bar_nodes` `W/🦀️.rs:252`, used by `S/⚓️dock/🧰️window-actions-and-commands/🦀️.rs:69`. |
| 26 | Hub AgentDelegations (new in this round) | PARTIAL | Built and composed: `W/🦀️.rs:826,860`. Transport and reload turns `S/📒️directory/📨️command-dispatch/🦀️.rs:368-433`. Missing: file-save door (Download falls to "could not save" path; action const `W/🦀️.rs:94`); MCP credential install is `unavailable`; select fields are buttons. React `E/🧱️elements/🤖️AgentDelegations/🟦️.tsx:175`. |
| 27 | Titlebar / DesktopTitlebar layout | OPEN | React `R/🖥️fullscreen/🟦️.tsx:151`. wgpu has the fullscreen toggle (`S/🐙️state/🧩️session-and-topology/🦀️.rs`, `S/🖐️pointer/🖱️pointer-input/🎯️shell-hit/🦀️.rs:212`) and the navbar icon, but no DesktopTitlebar layout. Not addressed by any fix note. |
| 28 | Palette width 512 px unmatched | OPEN | `SHELL_PALETTE_DESKTOP_MAX_WIDTH_PX = 512.0` `S/🚶️session/🔖️identity-and-types/🦀️.rs:95`. React value not checked. |
| 29 | Hard-coded widths (time-travel 880, folder 720, window pane 300) | OPEN (P2) | `S/🎛️prefs/⏪️time-travel/🦀️.rs:11`, `S/🪝️window-actions/🧰️window-actions-and-commands/🦀️.rs:12`. |
| 30 | Staging consolidation and crate size guard | OPEN (P2) | 312 `.rs` files still under `📥️staging/`. Not addressed. |
| 31 | Footer status tones, context menu, app switcher, presence chrome, a11y | OPEN (unverified) | Not addressed by any fix note. Still unverified. |

## New React chrome features wgpu lacks

| Feature | React | wgpu evidence | Verdict |
|---|---|---|---|
| Document close dialog, unsaved marker, beforeunload guard | `E/🧱️elements/🏛️ShellHost/🚪️close-guard/🟦️.tsx:23` (`DocumentCloseDialog`), `:59` (`UnsavedMarker`), `:73` (`useBeforeUnloadGuardV1`), used from `E/🧱️elements/🏛️ShellHost/🟦️.tsx` and `…/📋️panel-hosting/🟠️mode-windows/🟦️.tsx` | No `CLOSE_GUARD`, `DocumentClose`, `UnsavedMarker`, `before_unload` or saved-state symbol in `S` or `W` (only unrelated browser-worker wire events) | MISSING |
| Session authority notice (pending or unavailable, with cancel) | `E/🧱️elements/🏛️ShellHost/🟦️.tsx:216`, `🔖️session-notice/🟦️.tsx:8` | No `authority` notice symbol in `S` | MISSING (partly covered by route-admission stack) |
| OS file drop onto the shell (natural file drop) | `E/🧱️elements/🏛️ShellHost/🛬️natural-file-drop/🟦️.ts:15` (`installNaturalFileDropV1`) | No `DroppedFile` / `HoveredFile` / `FileDrop` handler in `W` or `S` | MISSING (browser path not checked) |
| Titlebar layout | `R/🖥️fullscreen/🟦️.tsx:151` (`DesktopTitlebar`) | No titlebar layout symbol in `S` | MISSING |

## Items still needing a compile and visual run

- Every fix note was written without compiling. The first gate is `cargo check` on the shell crates and the hub-workspace crate, plus `vitest` for the new contract tests.
- Expected test fallout from the `mod` rule on macOS is listed in `📓️opus55-fix-chrome-gaps.md` (Run list).
