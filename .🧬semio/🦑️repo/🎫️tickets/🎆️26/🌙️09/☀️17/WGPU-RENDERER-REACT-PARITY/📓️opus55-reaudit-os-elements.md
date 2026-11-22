# WGPU parity re-audit of the OS element PARTIAL and MISSING rows

Read-only. No source edited, no build or test run. Verdicts are from reading the current tree (grep plus targeted reads). A verdict of FIXED means the claimed function exists and has a non-test production caller. It does not mean the code compiles or passes, because every fix note says it was never compiled.

Abbreviations (all relative to `/Users/ueli/Documents/semio`):
- `F` = `🧰️framework`
- `E` = `F/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`
- `EL` = `E/🧱️elements` (React entries live here)
- `ECW` = `EL/⚙️EngineCanvas/🎯️targets/🧊️wgpu`
- `SCW` = `EL/🎞️Scenes/🎯️targets/🧊️wgpu`
- `IRW` = `EL/🗣️Interpreter/🎯️targets/🧊️wgpu`
- `SH` = `EL/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging`
- `W3I` = `F/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world`

Verdict key: FIXED, PARTIAL, OPEN. Confidence is given where the verdict rests on name-level grep only.

## 1. Re-verification of the audit PARTIAL and MISSING rows

| Element (audit row) | Audit verdict | Re-audit | Claimed fix and evidence (React / wgpu) | What remains |
|---|---|---|---|---|
| TextEditor: formatDocument, lintDocument | PARTIAL | FIXED (lint medium confidence) | React `EL/✏️TextEditor/🟦️.tsx:189-190, 357` only dispatches the verb. wgpu `SCW/✍️text-editor/🦀️.rs:389` `text_editor_format_document`, caller `:546` (Ctrl/Cmd+Shift+F). Lint has no Rust literal: `text_editor_queue_menu_action` (`:489-500`) claims only six local menu actions, so `lintDocument` rows go to the guest. Not verified that the guest menu carries a lint row. | none for the verbs |
| TextEditor: diagnostics footer (audit did not list it) | n/a | FIXED | `text_editor_diagnostic_lines` `:402`, caller `:418` | none |
| TextEditor: touch selection (`👆️touch-selection`) | PARTIAL | OPEN | React `EL/✏️TextEditor/🟦️.tsx:21, 48, 78` (`createTouchSelectionV1`, pointer type touch). No touch-selection hit in `ECW/✍️text-editor` or `SCW/✍️text-editor`. | touch selection not ported (high confidence) |
| TextEditor: a11y `aria-busy` and `aria-invalid` (not in audit) | n/a | OPEN, low confidence | React `TextEditor/🟦️.tsx:65, 462, 658, 674`. No `invalid` or `busy` hit in wgpu text-editor folders. | name-level only; a11y may model this another way |
| World3dHost: point cloud | PARTIAL | FIXED (wgpu only) | Wiring: `W3I/🌉️bridge/🎞️scene-bridge/🦀️.rs:961` calls `world_point_cloud_draws` (defined `W3I/🗃️model/🧬️scene-records/🦀️.rs:558`); emitted at `W3I/🎛️host/🎬️render/🦀️.rs:382`; encoded by `encode_prepared_world_points` (`F/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🎬️execution/🎨️world-material-and-effect-encoding/🦀️.rs:312`, called from `…/🖍️draw-scalar-encoding-and-raster-delegates/🦀️.rs:190`); pipeline from `WORLD3D_POINTS_SHADER` at `…/🌍️world-geometry-pipelines/🦀️.rs:205`. | Metal and D3D12 ignore `point_draws` (not wgpu). Uncompiled. |
| World3dHost: vortex markers, `openVortexSuggestions` | PARTIAL | FIXED | React `EL/🌐️World3dHost` (Alt, ContextMenu, F10 keys). wgpu `world3d_open_vortex_suggestions` (`W3I/🧵️interaction/🖱️pointer-and-picking/🦀️.rs:105`), callers `SH/🖐️pointer/⌨️keyboard-and-palette/⌨️keyboard/🦀️.rs:388, 563-564`, `SH/🍱️view-chrome/🖱️pointer-input/🦀️.rs:164`, request builder `SH/🕹️input/⌨️keyboard-and-palette/🦀️.rs:159-166`. Announcement key `world3d.suggestions.noTarget` in `E/🎯️targets/🧊️wgpu/📖️shell-vocab/✅️check-in-and-palette-vocab/🦀️.rs:60`. | "no marker focused" announcement is wired through a transient notice, not a live region. Uncompiled. |
| World3dHost: `terrainFrame` | PARTIAL | FIXED (audit false negative) | `W3I/🧱️scene/🦀️.rs:266` `sync_terrain_camera_frame`, called at `W3I/🎛️host/🔄️sync-and-overlay/🦀️.rs:295`. Brush consumed at `E/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🎮️interaction-state/🦀️.rs:548`. | none |
| UtilityTree: category grouping and row option buckets | PARTIAL | FIXED | `group_utility_nodes_by_category` `EL/🎛️UtilityTree/🎯️targets/🧊️wgpu/🦀️.rs:66`; `window_utility_bar_nodes` `:252`, caller `:69` in `derive_window_utility_nodes`. Buckets `row-utility-options` and `row-selection-options` at `:232, 234`. | window bar drops history and sync utilities, as React does |
| suggestion-submenu: open or close actions, `suggestionMenuOwnsWindow` | PARTIAL, low | OPEN, low confidence | React `EL/🎣️suggestion-submenu/🟦️.ts:19` (`openVortexSuggestions`) and `EL/🌐️World3dHost/🧭️fit-and-presence/🔸️meshes-ref/🟦️.ts:8, 56` (`suggestionMenuOwnsWindow`). No ownership gate or submenu search in wgpu. The vortex open route (above) exists, so only the submenu layer is missing. | submenu ownership and search |
| VideoRenderHost (whole element) | MISSING | OPEN | React `EL/🎥️VideoRenderHost/🟦️.ts`. No scene-to-video job, UI or `video_render` identifier in wgpu. A first-party AVC encoder exists in the framework raster crate (`🖌️raster/🎥️video/🦀️.rs:243-277`, `AvcPcmEncoder`), but it has no non-test caller. | the job, UI and wiring (the audit's "no encoder crate" was wrong; the encoder is present but unwired) |
| MediaTransportHost: transport, trim and keys | PARTIAL | FIXED in browser, PARTIAL overall | Browser overlay `E/🎬️media/🌐️browser/🎛️host/🟦️.ts` holds play, pause, seek, trim (`trimValueForSelection`), keys (`mediaKeyCommand`), loading and cancel. The audit's Rust location was the wrong place. | native (non-browser) hosts have no decoder. Only `ffmpeg` appears, in a TS test and a comment. OPEN for native. |
| ChromePanels: route-not-found page | PARTIAL | FIXED | `SH/📂️doc-open/🧭️route-not-found/🦀️.rs:54` `build_route_not_found_ui`, leaf registered `SH/…/🦀️.rs:122-123`, painted through the hub overlay branch at `SH/🗨️view-overlays/💬️chrome-dialog-and-tooltips/🦀️.rs:37-46`. | none |
| ChromePanels: settings-unavailable | PARTIAL | FIXED (already present; audit stale) | `settings.unavailable` exists per `shell-panels` note | none |
| ChromePanels: plugin recovery panel keys (`ui.plugins.recovery.*`) | PARTIAL | OPEN, name-level only | No `plugins.recovery` key match in wgpu. Recovery content not read. | key mirror and content not verified |
| ShellHelpers: package sniff (`inspectPackageV1`) | PARTIAL | OPEN | 0 hits for `inspect_package` or `inspectPackage`. The generic `sniff` in `F/…/🚪️io/📦️codec/🦀️.rs:700` is not package sniffing. | not ported |
| ShellHelpers: format resolution (`resolveFormatV1`, `pickRevisionV1`) | PARTIAL | PARTIAL | Native `resolve_format` exists at `F/…/🚪️io/🧭️format-resolution/🦀️.rs:108`, but it has no production caller (only its own module). `pickRevisionV1` not found. | not wired; `pickRevisionV1` absent |
| ShellHelpers: media import and tier-2 video frames | PARTIAL | OPEN | 0 hits for `importOpenedFilesV1`, `media_import`, `runTier2VideoFrames`. | not ported |
| ShellHelpers: extension io and command registry | PARTIAL | OPEN, name-level only | Not traced beyond name grep. | not verified |
| AgentDelegations: whole element | MISSING | FIXED (uncompiled) | React `EL/🤖️AgentDelegations/🟦️.tsx`. wgpu panel `EL/🤖️AgentDelegations/🎯️targets/🧊️wgpu/🦀️.rs:826` `build_agent_delegations_ui`, composed by `EL/🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs:846, 860`. Install `begin_install` (`:644`) and `install_settled` (`:660`). MCP copy `AGENT_COPY_MCP` (`:97`, `:760`). Download via `save_text_file` at `SH/📂️doc-open/🎨️theme-file-import-and-accessibility/🦀️.rs:267`. Audience and expiry are real `select()` nodes (`HubConnection/…/🦀️.rs:591`, used at AgentDelegations `:783, 787`). | no automatic reload when sign-in completes with a space open (React gap, as noted) |
| BlockListHost: drag-to-sort and transfer, drag visual | PARTIAL | FIXED (uncompiled) | Sort and transfer were already present per the note. Row displacement: `SCW/🧱️block-list-and-diff/🦀️.rs:315` (`sortable_vertical_offsets`), `:351`, caller `:374`. Dimmed row: `begin_group_alpha` at `:501` with `BLOCK_LIST_DRAG_ALPHA = 0.5` (`:312`). Group bracketing: `F/…/🖍️draw/🎮️execution/🧷️prepared-upload-and-present-steps/🦀️.rs:150` (`group_open`). | GPU alpha path not verified on hardware |
| TiledMapHost: device-loss recovery with camera restore | PARTIAL | FIXED | Loss source: `set_device_lost_callback` in `F/…/🖍️draw/🎮️execution/🔌️device/🦀️.rs` and `F/…/🖌️render/🎯️targets/🧊️webgpu/🌐️backend/🦀️.rs`. Drain and fan-out: `E/🧊️renderer/🧠️app-runtime/🦀️.rs:219-222` calls `tiled_map_device_lost` (defined `ECW/🗺️tiled-map/🦀️.rs:57`). | none beyond the uncompiled state |
| TiledMapHost: canvas-unavailable message and Retry | PARTIAL | FIXED | `stage_map_unavailable_notice` (`ECW/🗺️tiled-map/🦀️.rs`, caller `SCW/🩹️map-unavailable`). Retry by key: `retained_map_retry_key` (`IRW/🧩️retained-ui-engine/📕️close-scene-pointer-and-dispatch/🦀️.rs:128`, called at `:498`). Retry by touch: `tiled_map_touch_pointer_down` (`SCW/🧭️node-graph-and-tiled-map/🦀️.rs:179`, called from `E/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🎮️interaction-state/🦀️.rs:443`). | none beyond the uncompiled state |
| TaskManager: runtime actor table, controls, metrics | PARTIAL | PARTIAL | Actor path: `sync_runtime_actors` and `sync_runtime_install_tasks` called at `SH/📂️doc-open/🧵️task-manager-runtime/…` shell (publish path, lines 118-119); `forward_actor_requests` (`…/task-manager-runtime/🦀️.rs:553`, caller `:250`); kernel observation `E/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🎛️runtime-observation/🦀️.rs:10`, served from the pool loop (`…/🚀️turn-dispatch-and-pool-runner…:467`); host port `E/🎯️targets/🧊️wgpu/🏛️host-base/🎛️actor-observation-port/🦀️.rs:32, 51`. | Tool-run and task-level suspend and resume are queued with no consumer. Kernel serves only when work arrives. Spawned job tasks are modelled, not fed. |
| PluginRuntime: handle contract and scope, job ledger | PARTIAL, medium | OPEN | React `EL/🔌️PluginRuntime/🪢️handle-scope/🟦️.ts`, `EL/🔌️PluginRuntime/🧺️handle-ui-intake/🟦️.ts`. 0 hits for `handle_contract`, `handle_scope`, `job_ledger` in Rust. | not ported, medium confidence the native side needs it |
| WorldTerrainLayer: brush, overlays, style parsing | PARTIAL, low | FIXED (audit false negative) | Brush: `W3I/🧱️scene/🦀️.rs:217-244`. Move-position overlay: `terrain_move_position` at the same file. Style: `apply_terrain_style_…` at `W3I/🧱️scene/🏔️environment-and-terrain/🦀️.rs:534`. | none |
| Paint2dHost: keyboard cursor, stamp, marquee, announcement, rollover, zoom | PARTIAL | FIXED (uncompiled) | Key route `SH/🖐️pointer/⌨️keyboard-and-palette/⌨️keyboard/🦀️.rs:503` (`repeat: modifiers.repeat`), `retained_paint2d_key` (`IRW/…/🦀️.rs:133`), `paint2d_key_into` (`ECW/🖌️paint2d/🦀️.rs:660`, `pixel_key_down` at `:665`). Paint crate keyboard `F/🔨️modules/🗺️surface/🎨️paint/⌨️keyboard/🦀️.rs:87` (`keyboard_camera_step`, caller `:206`). Rollover `…/✍️gesture/🦀️.rs:113`, caller `…/🖥️raster-host/🦀️.rs:176`. Dashed marquee `…/🖥️raster-host/🦀️.rs:354, 369`. Live announcement nodes `SH/🪀️view-panels/🎨️theme-file-import-and-accessibility/🦀️.rs:329` (from `ECW/🖌️paint2d/🦀️.rs:694`). | identical repeated text is not re-announced (limit noted by author). Uncompiled. |
| Board2dHost (not in audit list; swept below) | PRESENT | PARTIAL, low-medium | React `EL/🖥️Board2dHost/👆️board-pointer-and-keyboard/🟦️.ts:214-240` handles Escape and Tab (Tab focuses suggestion rows). wgpu Escape present (`ECW/🎲️board2d/🦀️.rs:715, 737`). No Tab-focus route in wgpu. | Tab focus of suggestion menu rows |

## 2. Surface kind sweep (15 kinds from `F/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🗺️surface/🦀️.rs:20-68`)

Counts are per owning wgpu scene folder and React host folder. Keyboard, context-menu and a11y evidence is count-level unless stated. "Match" means no per-feature gap was found. It does not mean every feature was traced.

| Kind | React host features (keyboard, menu, a11y) | wgpu owner and evidence | Verdict |
|---|---|---|---|
| Canvas2d | Pointer, context menu (6), wheel, catalogue drop, double-click; no React keyboard | `SCW/🖌️canvas2d`: drop 24, drag 6; context-menu files 3; a11y files 5 | Match (medium) |
| World3d | Keys Alt, AltGraph, ContextMenu, F10, Escape (`EL/🌐️World3dHost`); context menu; a11y `role=status` (7) | Alt, ContextMenu and F10 routes wired (see row above); context-menu files 5; a11y 6 | Match (medium, after fixes) |
| NodeGraph | Keys (about 9), context menu 3, aria-label 5, wheel 4, drop 6 | `SCW/🧭️node-graph-and-tiled-map`: drag 45, context menu 7 | Match (medium) |
| TextEditor | 29 key cases, `aria-busy`, `aria-invalid`, status and alert roles, completion and rename context menu, touch selection | Keys and context rows present; diagnostics and Ctrl+Shift+F added; no touch selection; no `aria-busy` or `aria-invalid` | PARTIAL (touch selection; a11y states) |
| Table | Editable cells, stepper, Home/End/PageUp, sort, drag and drop, `role=spinbutton` | `SCW/🧮️table-and-paint2d`: stepper 55, sort 66, editable 51, a11y cells 11; row double-click gate fixed (`IRW/…/⚡️events/🌳️tree-roving-and-keys/🦀️.rs:117`) | Match (medium; PageUp and Home not traced) |
| Paint2d | Pixel keyboard (arrows, stamp, marquee, zoom), context menu, wheel 9, status and alert roles, `aria-live` | All present after fixes (see Paint2d row) | Match after fixes (uncompiled) |
| VirtualFileSystem | Feed and tree keys, double-click navigate, a11y controls | `SCW/🧩️board2d-and-vfs` a11y controls 9, double-click 4 | Match (low; React host sits in `IRW/🗄️virtual-file-system`) |
| TiledMap | Pointer, context menu (2), `role=application`, `role=alert`, Enter and Space retry | Context-menu files 3; a11y 5; retry by key and touch; alert and button a11y | Match after fixes (medium) |
| Board2d | Escape and Tab (suggestion focus), context menu 5, drop 5, wheel 3 | Escape present; no Tab focus; drop 4, wheel 8 | PARTIAL (Tab focus, low-medium) |
| IconRender | No keyboard, no context menu in React host | `SCW/🖼️icon-render` minimal; no equivalent needed | Match (low) |
| InkCanvas | Keys (4 cases), pointer 9, edit routing, double-click | `SCW/🖋️ink-canvas-render`: drag 106, wheel 11, double-click 3, key files 3 | Match (medium) |
| GraphTimeline | Context menu 3, checkpoint table, selection | Context-menu files 1; a11y 5 | Match (low-medium) |
| BlockList | Key (1), context menu 3, aria-label 3, drag and drop, sort and transfer | `SCW/🧱️block-list-and-diff`: drag 61, context menu 1, keys 2, a11y 5; drag visual now present | Match (medium) |
| DiffView | Context menu 3 | Context menu 1; a11y 3 | Match (low) |
| EventFeed | Key (1), context menu 1, `aria-hidden` | Keys 2; context menu 1; a11y 5 | Match (medium) |

## 3. Corrections to the audit and notes

- The audit's point cloud, vortex, terrain, brush and tiled-map notice verdicts were stale or false negatives. All are now wired.
- Settings-unavailable already existed in wgpu. The audit was stale there.
- MediaTransport's browser behaviour lives in TS (`E/🎬️media/🌐️browser/🎛️host/🟦️.ts`), not Rust. The native gap is real.
- ShellHelpers: `resolve_format` exists natively but has no production caller. The audit's 0-hit verdict was only partly right.
- The first-run note (`opus55-hub-workspace-first-run.md`, slice 2) says AgentDelegations select fields are "check/dot buttons". The current code uses real `select()` nodes (`HubConnection/…/🦀️.rs:591`). The note is stale.
- The VideoRenderHost "no encoder crate" claim is wrong: `🖌️raster/🎥️video` contains an AVC encoder, but nothing calls it.

## 4. Open items, highest impact first

1. VideoRenderHost (MISSING): job, UI and wiring; decide whether to use the existing AVC encoder.
2. AgentDelegations is FIXED but uncompiled; the uncompiled state applies to every row.
3. TextEditor touch selection (PARTIAL).
4. MediaTransport native decoder (PARTIAL, native only).
5. ShellHelpers: package sniff, media import, tier-2 frames (OPEN); `resolve_format` unwired (PARTIAL).
6. PluginRuntime handle contract and job ledger (OPEN, medium confidence).
7. TaskManager tool-run and task controls (PARTIAL).
8. suggestion-submenu ownership and search (OPEN, low).
9. ChromePanels recovery panel (OPEN, name-level).
10. Board2d Tab focus of suggestion rows (PARTIAL, low-medium).
11. TextEditor `aria-busy` and `aria-invalid` states (OPEN, low).

## 5. Caveat

No fix in any note has been compiled or run. Every FIXED verdict above is source-level only. The integration build is the next gate.
