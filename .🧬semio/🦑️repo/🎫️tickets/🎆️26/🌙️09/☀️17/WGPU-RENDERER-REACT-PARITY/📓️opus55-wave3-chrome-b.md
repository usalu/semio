# Wave 3 chrome B (opus55)

Path shorthand: `S` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/`; `UI` = `🧰️framework/🔨️modules/🖱️ui/`.

## Verification of the re-audit claims first

| Claim | Verdict | Evidence |
|---|---|---|
| Palette 512 px is unmatched | FALSE POSITIVE | React `CommandDialog` uses the Dialog's `sm:max-w-lg` = 32 rem = 512 px (`UI/🧱️elements/💬️Dialog/🟦️.tsx:554`). The number matched, only its source was a literal. Now derived (see 1). |
| Time-travel 880 / folder 720 are React values | WRONG VALUE | React bands are `max-w-[90vw]` (`🛠️ShellHelpers/⏪️time-travel/🟦️.tsx:458,504`, `🏛️ShellHost/📎️local-folders/🟦️.tsx:97`). 880 / 720 were invented. |
| Reserved-chord check / badges use empty overrides | FALSE POSITIVE for production | The two `shell_shortcut_table(&HashMap::new())` calls are `#[cfg(any(test, feature = "testkit"))]` default-table projections (`S/🖐️pointer/🧱️chrome-paint-and-navbar/🦀️.rs:81`, `S/🖐️pointer/🧰️window-actions-and-commands/🦀️.rs:101`). Production calls `is_reserved_shell_chord_in(&self.chrome.shortcut_table(), ..)` and `shell_shortcut_for_in(&self.chrome.shortcut_table(), ..)` (`S/🖐️pointer/⌨️keyboard-and-palette/⌨️keyboard/🦀️.rs:114,408,456,462`); badges use `shell_control_hotkey_badge(&self.chrome.shortcut_table(), ..)` (`S/🪀️view-panels/🎨️theme-file-import-and-accessibility/🦀️.rs:90,242,289`). No change. |
| Dialog close fade done (wave 2) | CONFIRMED NOT DONE | Only `chrome_enter` / `chrome_enter_forget` have callers (`S/🗨️view-overlays/💬️chrome-dialog-and-tooltips/🦀️.rs:266-296`); no exit path. Left open, out of this wave's scope. |
| AgentDelegations save door missing | STALE | `hub_action::AGENT_DOWNLOAD` is handled at `S/🔀️actions/📨️command-dispatch/📇️directory-and-hub/🦀️.rs:100` via `save_text_file`; install via `post_agent_credential_install` (native build stubs it, browser posts). |
| Footer status tones | NO REACT TWIN | The footer's sync pill (`s-sync-status`) is a tab name in React (`🏛️ShellHost/✅️check-in-sync/🔸️auto-checkin-scheduler/🟦️.tsx:96`), no tone class. Nothing to match. |
| Status stacks lack byte progress, meter, elapsed line | CONFIRMED | Fixed (4). |
| Drop zones at dock edge | CONFIRMED | Fixed (5). |
| Tool-run tasks not fed | CONFIRMED | Fed (6); controls not forwarded (gap below). |
| Focus ring lacks accent inversion / selected underline | CONFIRMED | Fixed (3), underline approximate. |

## Changes per item

1. **Chrome extents from the shared schema.** The styling `🔣️.json` holds CSS metrics only; these are Tailwind classes and TS constants, so they go in the existing shared chrome-parity file `UI/🧬️schema/📐️chrome-parity-constants/` (`🧮️values.json` + `🔣️.json`), new `sizes` block: `panelDefaultPx` 300, `windowPaneDefaultPx` 300, `paletteDialogMaxWidthRem` 32, `timeTravelBandMaxWidthFraction` 0.9, `folderBandMaxWidthFraction` 0.9.
   - React: `PANEL_DEFAULT_SIZE_PX` (`UI/🧱️elements/🖼️Panel/🟦️.tsx`) and `PANE_DEFAULT_SIZE` (`UI/🧱️elements/📌️WindowPane/🟦️.tsx`) now read the JSON.
   - Rust: `ChromeParitySizes` in `S/🎚️chrome-state/🎨️theme-file-import-and-accessibility/🦀️.rs`; `WINDOW_PANE_BODY_WIDTH_PX` became `window_pane_body_width_px()` (all 6 users and imports updated); `SHELL_PALETTE_DESKTOP_MAX_WIDTH_PX` became `shell_palette_desktop_max_width_px()` (rem x `remPx`); time-travel and folder bands use `width * fraction`.
   - Left as is: `DEFAULT_PANEL_WIDTH_PX` (dock crate, a const used by about 20 sites and tests); it equals `panelDefaultPx` and the new law test pins 300.
2. **Reserved chords / badges**: false positive, no change (see table).
3. **Focus ring** (`S/🗨️view-overlays/🧭️navbar-footer-and-overlays/🦀️.rs`, step 10): a focused control whose accessibility node is selected or pressed rings in `border_emphasized` (React `data-focus-on-accent`, CSS `:6722`) instead of the accent, and gets the 2 px underline (CSS `:6691`). See the follow-up section for the final underline geometry.
4. **Status stacks**: `StatusCell.progress` plus a laid out meter track (`bar`) between copy and action.
   - Bootstrap: `ArtifactEvent::BootstrapProgress` (already stored as `sync_bootstrap_progress`) now shows React's "Restoring document: X of Y bytes; A of B chunks." with the meter and a Cancel (new `sync_bootstrap_cancel`, consumed by the frame pump which runs `detach_sync_backbone_internal`, React's `closeDocument`). en+de rows `status.bootstrap.progress` / `status.bootstrap.cancel` in `🎯️targets/🧊️wgpu/📖️shell-vocab/🧵️task-manager-and-route-vocab/🦀️.rs`.
   - Execution target: the stack also shows while the open is instantiating with native transfer progress, meter = that percent.
   - Creation elapsed line: `HubArtifactCreation.issued_at_ms` (set at issue), `ARTIFACT_CREATION_WAITING_AFTER_MS` 10 s, `hub_artifact_creation_elapsed_text`, label `Waiting` en+de, appended once a non-terminal creation passes 10 s.
5. **Drop zones**: `empty_anchor_drop_zone_rect(anchor, panel, theme)` now sits in the anchor's one-row panel box (`anchor_panel_rect` with one control of height), inset by the row gap, on the flow's first row (right anchors end-aligned). Law test rewritten.
6. **TaskManager tool-run tasks**: React source is the rendered ToolRun panel body (`toolRunPanelTasksV1`). New `tool_run_panel_tasks` (`S/🪝️window-actions/🧾️panel-records/🦀️.rs`) reads the same panel document (group label, `progress` node, Pause/Resume offered) in the render pass, `OpeningState::set_tool_runs` stores it and `sync_runtime_install_tasks` appends `tool_run_tasks` (elapsed kept per id). A change owes a refresh.
7. **Other chrome**: footer tones had no React twin; context menu, app switcher and presence chrome were not audited in depth (no gap evidence found, only existence of the item-spec, scoping and nested-level paths). Not a clean bill.

## Laws added (`S/../🧪️tests/🔬️wgpu-chrome-parity-constants/🦀️.rs`)
- `an_empty_anchors_drop_zone_sits_inside_its_anchor_row`
- `the_pane_palette_and_band_extents_are_the_shared_schemas`
- `a_status_cell_with_progress_paints_its_meter`

## Known gaps
- Tool-run Pause / Resume / Abort presses are queued as Task Manager requests but not forwarded to the run's own panel binding (needs activating the panel node through the retained accessibility path). The same queue-only state already holds for job and tool-call tasks.
- Dialog close fade (item 13 of the re-audit) still open.
- Time-travel bands now scale with the window instead of capping at a pixel width; no visual run done.
- Compiled via `repo cargo check` only (see the final message for the result); `vitest` for `chrome-parity-constants` not run (new `sizes` key is covered by its required-keys test only if the schema test passes).


## Follow-up (coordinator round 2)

**(a) Tool-run controls.** Another agent landed the forwarding while I was editing: `TaskManagerToolRun.controls` carries the panel's own pause / resume / abort descriptors and `take_tool_run_control_actions` feeds `settle.deferred_actions` from the dispatch arm. I had added a parallel `ToolRunControls` / `take_tool_run_actions` and removed it again, so only their path remains; my reader (`tool_run_panel_tasks`) is the one they extended. I did not verify their version beyond reading it (it sends `action.name` without the `@version` suffix, same as React for v1 verbs).

**(b) Underline.** React underlines the control's own text only (`text-decoration`, 2 px, offset `0.2em`, CSS `:6691`), so an icon-only selected control gets none. Now: the shared chrome item painter (`render_retained_chrome_group_item_step_with_trailing`) records, per control id, where its label painted (`x`, baseline, measured width capped at the label box, size, ink colour, active) in `chrome_label_span`; icon-only items remove their entry. The focus step underlines exactly that span, only when the item is active, at `baseline + 0.2 * size`, 2 px, in the label's ink. Thickness and offset live in the shared schema (`focus.underlineThicknessPx`, `focus.underlineOffsetEm`) with a law test. Controls painted by other painters than this one (panel tab chips, dock rows with their own label code) record nothing and get no underline yet.

**(c) Dialog close fade and zoom.** The wave-2 claim is true: `DialogGhost` plus `render_chrome_dialog_step` (`S/🗨️view-overlays/💬️chrome-dialog-and-tooltips/🦀️.rs`) is the production step, called as phase 5 of the overlay walk (`S/🗨️view-overlays/🧭️navbar-footer-and-overlays/🦀️.rs:310`) every frame; the ghost keeps the last request while the stack is empty, runs the enter easing backwards for alpha, glass and `dialog_zoom`, skips hits while closing, and respects reduced motion. Nothing to wire. My earlier note that it was missing was wrong (I grepped before the code landed).

**(d) Context menu, app switcher, presence (compared against React source).**
- Context menu: keys (arrows, WASD, 1-9 ordinals, Enter/Space, Escape pops a level), item paths, disabled reasons and the `closeOnSelect` shape match. Fixed: an opening menu now seeds the active row with the first enabled `checked` row (`findContextMenuCheckedPath`, new `context_menu_checked_path`, wired in `open_context_menu`); Escape and `a` out of a submenu now record `submenu_collapsed_at` like React's `applyActivePath(parent, parent)`. Not ported: React's text-selection menu (copy/cut/paste/select-all on DOM text), fusion panel placement of submenus, hover start/end item callbacks, focus return to the previous element.
- App switcher (navbar example select): width policy 12 to 28 rem, `file` icon, hide when the example is locked, and the offered-example predicate already match `NavbarExampleSelect` / `exampleOptions`. Gaps left: React's `resolveAppLabel` overlay for example labels, the `includeNoExample` row, and hiding it for the host landing app.
- Presence: fixed the accessible name, which omitted React's role qualifier (`Editing` / `Viewing`, de `Bearbeitet` / `Betrachtet`) between the agent word and the activity text (`presence_role_label`). Not fixed: React paints avatar cells with a presence-coloured ring, a robot badge for agents and an activity badge; wgpu paints the roster as one text chip (`presence_bar_chip_text`), and the `+N more` suffix differs in the visible chip. Terminology-specific agent label ("An AI agent someone gave access to" for beginners) is also ignored.

## Compile and laws
See the final status line in the closing message; the check covers the shell state, pointer, input, window-actions, view-chrome, view-chrome-overlays, view-overlays crates and the test crate holding `wgpu-chrome-parity-constants`.

## Round 3 (coordinator: close the remaining gaps) and build hand-off

Nothing was compiled by me; the queued cargo was cancelled on the coordinator's policy change. `repo test-file` on `🧪️tests/📐️chrome-parity-constants/🟦️.ts` ran and passed (5 of 5, with the new `sizes` and `focus` schema keys).

**Done**
- Underline for panel tab chips: `note_chrome_label_span` (view-chrome `chrome-paint-and-navbar`) is called by the desktop tab bar (key = tab id) and the mobile tab strip (`shell.panel.tab.mobile.<id>`); the footer tabs use the retained item painter that already records spans. Dock rows other than these have no painted label code that I found.
- Context-menu production painter (new finding): the production step only painted the top level at a fixed 240 px, without ordinals, shortcuts or submenus; the full renderer was `cfg(test/testkit)` only. Replaced with `context_menu_paint_ops` / `render_context_menu_levels_step` in `S/🗨️view-overlays/💬️chrome-dialog-and-tooltips/🦀️.rs`: per-level width, ordinal badges 1 to 9, icons, shortcuts, disabled ink, scroll region, and every open submenu as its own glass region at the parent row's top, flipping left past the right edge (React's fusion placement). The old `render_context_menu_step` in `🪧️overlay-steps` was removed and the tour-overlay crispness test now drives the new step. Law added: `the_open_context_menu_paints_its_submenu_beside_the_parent_row`.
- Presence avatars: `presence_initials`, `PresenceCell`, `presence_bar_cells` in the PresenceBar wgpu element (unit test added); the footer now paints one shared-border row of square cells, each an initials disc in a 2 px ring of the peer's presence colour, a bot-icon badge bottom-right for agents, the activity badge top-right, and a `+N` cell; empty roster stays muted text. Layout width = cells x footer button height + hairlines.

**Not done (reasons)**
- React's text-selection menu (Cut/Copy/Paste/Select All): wgpu has no equivalent surface; text edits and clipboard run through key routing and original-clipboard intents on the retained tree, and the guest already receives the `text` context in `context_menu` requests. Wiring chord-synthesising rows without a compile or a run was judged unsafe.
- Example-label overlay (`resolveAppLabel(appLabelsOverlay, "example", ..)`): the overlay lives in the os shell module state (`🖥️shell`) and nothing feeds it to the wgpu shell. `includeNoExample` is passed by no React host (dead option), so there is no "no example" row to match.
- Unchecked visuals: avatar sizes, the `bot` icon name in the icon atlas, and the badge glyph (`⏪`) coverage.

**For integration lane E**
- `cargo check --tests` on: `semio-renderer-wgpu-shell-chrome-state`, `-shell-view-chrome`, `-shell-view-chrome-overlays`, `-shell-view-overlays`, `-shell-view-panels`, `-shell-window-actions`, `-shell-doc-open`, `-shell-dock`, `-shell-sync`, `-shell-pointer`, `-shell-input`, `-shell-state`, `-shell-prefs`, `-shell-vocab`, `-hub-workspace` (`🏘️hub-workspace`), `semio-ui-wgpu-light` (PresenceBar), plus test crates `semio-renderer-wgpu-shell-tests-input-3` (holds `wgpu-chrome-parity-constants`), and the crate containing `🎓️tour-overlay-and-chord-glyphs`, `🔄️detached-sync-task-manager`, `🧭️wgpu-navbar-footer-parity` and `🧪️wgpu-chrome-parity` laws (find via `grep -l` on the test folder name; `shell-tests-chrome-5` imports `window_pane_body_width_px`).
- Tests to run: `wgpu-chrome-parity-constants` (all), PresenceBar `wgpu-unit`, `tour-overlay-and-chord-glyphs` (crispness and the new submenu law), `wgpu-navbar-footer-parity` (pane body width), `detached-sync-task-manager`, `wgpu-panel-anchor-model` (default panel width), `HubConnection/wgpu-unit` (`issued_at_ms` added to constructors).
- Likely compile risks: `chrome_label_span` / `note_chrome_label_span` re-export from `shell-view-chrome`; `ContextMenuPaintOp` / `context_menu_paint_ops` re-export from `shell-view-overlays`; `TaskManagerToolRun` struct literals (another agent added `controls`); `semio_framework::DslValue` path in `panel-records`.
