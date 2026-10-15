# Seam Map: wgpu Shell monolith split

Source: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
Crate: `semio-framework-os-renderer-wgpu`, wired as `pub mod shell` (`#[path]` plus `pub mod shell;` in renderer `🧊️renderer/🦀️.rs`).
Size at audit: 35,570 lines, 2,068,720 bytes (mtime Oct 14 06:46).

Mode: read-only audit. No source file was edited, no git, cargo, nx or build was run. All line numbers refer to the file as audited; re-verify the anchor lines in section 6 before cutting.

## 0. Summary

- Parent `🦀️.rs` keeps about 860 lines: imports (1-138, plus 144-149 lifted), the existing child wiring (1815-1826), the extension-ledger types, `pub struct ShellState` (3708-4302) and `impl Drop` (4305-4325).
- 20 child modules move out, 813 to 2,411 lines each (all inside the 800-3000 target).
- Only two kinds of cut are needed inside code: `impl ShellState` blocks are split at method boundaries in four places (section 3.2).
- Every moved item used from another chunk becomes `pub(super)` (section 4). The external symbols listed in section 3.3 keep working through explicit re-exports.
- Path attributes and `include_str!` inside moved code need exactly one extra `../` each (section 5). Nothing else in the file uses `self::`, `super::` or `crate::shell::` paths, so no other path rewriting is needed.
- No `macro_rules!` exists in the file. No `#![...]` inner attributes. Only one doc-only `crate::shell::` mention.

## 1. Existing child-module pattern (repo convention)

Wiring, exact text at lines 1815-1826 of the monolith:

```rust
#[path = "📤️icon-export/🦀️.rs"]
mod icon_export;

#[path = "🪟️tree-windows/🦀️.rs"]
mod tree_windows;

#[path = "⏪️time-travel/🦀️.rs"]
mod time_travel;
use time_travel::TimeTravelVerb;

#[path = "📎️local-folders/🦀️.rs"]
mod local_folders;
```

Child folder sits next to the monolith (`wgpu/<emoji><kebab>/🦀️.rs`). Child header, from `⏪️time-travel/🦀️.rs` lines 1-17 (abridged):

```rust
//! ⏪️ The wgpu shell's time-travel chrome (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, packet W2-C).
//!
//! ...more `//!` prose...
use super::*;
use semio_framework::kernel::{HistoryReprojection, ...};
use semio_framework_time_travel::TimeTravelLabel;
```

Facts from the existing children:
- Doc header is `//!`, first character is an emoji (all four children).
- Two import styles coexist: `use super::*;` (`⏪️time-travel`, `📎️local-folders`) and explicit `use super::{...};` (`📤️icon-export`, `🪟️tree-windows`). Use `use super::*;` for the new children.
- `pub(super)` already used for shared items (`📤️icon-export`: `pub(super) const CONTROL_ID`).
- Nested `#[path]` inside a child is relative to the child's own folder: `⏪️time-travel/🦀️.rs:247` `#[path = "🗣️labels/🦀️.rs"]`.
- A test path inside a child is one level deeper: `🪟️tree-windows/🦀️.rs:201` `#[path = "../../../🧪️tests/🪟️tree-windows/🦀️.rs"]`, versus `../../🧪️tests/` used at wgpu level. This is the precedent for the `+1 ../` rule in section 5.

Proposed wiring for the new children (same form, appended after line 1826 in the parent):

```rust
#[path = "🔖️identity-and-types/🦀️.rs"]
mod identity_and_types;
use identity_and_types::*;
```

## 2. Top-level sections of the monolith, in file order

Line ranges are inclusive. "Goes to" uses the child IDs of section 3. Counts come from `/usr/bin/grep` and `awk` over the file.

| Lines | Content (main items, line of item) | ~Lines | Goes to |
|---|---|---|---|
| 1-8 | `//!` file doc | 8 | P |
| 9-137 | `use` block: crate imports, `#[cfg(...)]` gated imports at 14-17 and 94-121, `use ui_wgpu::wgpu::*`, `use crate::*` | 129 | P |
| 11-12 | `#[path = "../../../🏛️ShellHost/🔀️surface-switch/📄️document/🦀️.rs"] mod document_surface_preparation;` | 2 | P (path unchanged) |
| 139-143 | FRAMEWORK_DISPLAY_WINDOWS_TAB_ID and 3 other `const`s | 5 | C01 |
| 144-149 | `use semio_framework_value::...`, `#[cfg] use protocol::...`, `use serde_json::Value` | 6 | P (lift into import block) |
| 150-229 | WindowTemplateDragPayload 153, window_template_drag_data 159, dsl_value_as_json 168, FRAMEWORK_* id consts 179-224 | 80 | C01 |
| 230-393 | SearchPaletteItem 230, ShellFindItem 247, ShellFindItems 260 (impls 268, 274, 359, 372), try_push_find_item 383 | 164 | C01 |
| 394-594 | context-menu action helpers 394-475, mint_shell_session_id 476, resolve_identity_env (cfg pair 483/491), shell_directory_request_base_url (cfg pair 503/508), shell_hub_* 517-540, SHELL_DIRECTORY_NETWORK_BYTES_PER_MINUTE 541, shell_directory_transport (cfg pair 545/550), shell_actor 566, default_persistence_bindings 576 | 201 | C01 |
| 595-760 | ShellDocumentTransports 595, SHELL_DOCUMENT_TRANSPORTS 600-603, document_bindings_admitted 608, directory_command_from_action 624, OpenArtifactRelayTarget 652, open_artifact_relay_target 663 (fn-local `use` at 664), presence helpers 711-719, S_SPACE_INDEX consts 748-749, space_index_dialect 751 | 157 | C01 |
| 755-851 | `//#region ReplayRefusal`: ReplayRefusalReasonV1 761, LabelV1 776, VectorV1 784, VocabularyV1 794, REPLAY_REFUSAL_V1 803 with `include_str!` at 804, impl 806 | 97 | C01 |
| 855-1089 | `//#region IdentityPure`: find_dialect_app 855, ShellPluginInstallPhase 864, ShellDocumentOpenPhase 876, ShellDocumentOpenAnswer 888, ShellDocumentOpenTarget 914, ShellSyncReseed 924, ShellHubDocumentSeed 933, ShellPreparedDocumentOpen 942, ShellDocumentOpening 957, ShellPluginInstall 988, RemoteBackboneUri 1017, document_execution_target_admitted 1037, wgpu_document_socket_surface 1063, bind_wgpu_document_socket_surface 1067 | 235 | C01 |
| 1091-1111 | `//#region ConflictProjection`: ShellConflictRow 1099 | 21 | C01 |
| 1113-1212 | `//#region CheckInPure`: AUTO_CHECKIN_* 1117-1118, fold_history_patch 1130, can_check_in 1172, auto_checkin_should_fire 1184, should_checkpoint_before_detach 1200, checkpoint_landed 1209 | 100 | C01 |
| 1214-1218 | `//#region IdentityDirectoryPresenceTests`, `#[cfg(all(test, not(wasm32)))] #[path = "../../🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs"]` at 1216 | 5 | C01 |
| 1220-1814 | `//#region ShellTypes` (first part): SHELL_DISPATCH_ORIGIN_* 1223, SpaceProgramEntry 1228, SpawnedAppEntry 1239, SpacePanelState 1258, ContextMenu* 1270-1466, OverlayState 1467, ShellPaletteKind 1476-1565, palette consts 1567-1572, RightClickState 1582, ActiveSession 1589, ShellSyncChannel 1601 (`#region NativeSyncChannel` 1596-1651), ui-driver axes and fns 1653-1722, `#region ChromeThreadBoundary` 1723-1813 (RetainedContentFocus 1724, ShellChromeBuildState 1732, ShellChromePresentState 1779, ShellChromeMaintenance 1790-1805) | 595 | C01 |
| 1815-1826 | child wiring: icon_export, tree_windows, time_travel (+ `use time_travel::TimeTravelVerb;` 1823), local_folders | 12 | P (keep) |
| 1827-1937 | `//#region ShellDetached`: ShellDetached 1833-1937 | 111 | C02 |
| 1939-2010 | ShellPoolFuture 1941, `impl Wake` 1999, ShellDirectoryStream 2010 (native only) | 72 | C02 |
| 2019-2166 | ShellDirectoryRunner 2021, impl 2034-2161 (native) | 148 | C02 |
| 2168-2319 | `//#region InferencePort`: ShellInferenceRunner 2169, impl 2186-2318 | 152 | C02 |
| 2321-2446 | DirectoryHomeProjection 2335, impl 2353-2445, terminal_directory_home_ack 2448 | 126 | C02 |
| 2484-2486 | `#[cfg(all(test,…))] #[path = "../../🧪️tests/🔬️wgpu-shell-pool-future/🦀️.rs"]` test mod | 3 | C02 |
| 2488-2502 | ShellIoCompletion 2489, PendingShellIo 2498 | 15 | C02 |
| 2503-2656 | `//#region ShellDocumentRetirement`: consts 2504-2508, slot/admission/registry 2510-2530, impl Default 2531, impl 2537-2651, test mod 2653-2655 | 154 | C02 |
| 2658-3020 | `//#region SpaceAdministration`: phase enum 2663-2675, turn enum 2690, operation struct 2702 and impl 2716-2873, helper fns 2877-2956, controls 2957-3020 | 363 | C03 |
| 3021-3123 | `//#region DirectoryCommandQueue`: consts 3023-3031, result enum 3036, queue struct 3046 and impl 3051-3121 | 103 | C03 |
| 3125-3196 | `//#region HubWorkspaceTask`: HUB_SIGN_IN_DEADLINE_MS 3130, ShellHubSignInAnswer 3134, ShellHubTask 3147-3196 | 72 | C03 |
| 3198-3290 | `//#region BootProgramSelection`: ShellPluginFault 3204, ShellSurfaceFault 3223, WindowTopologyPublication 3238, select_boot_program 3256, BootSelectionRequest 3266 | 93 | C03 |
| 3291-3336 | deferred/retirement/settle-round consts 3291-3325, window_paint_opportunities 3325, document_opportunity_remains 3330 | 46 | C03 |
| 3345-3487 | SHELL_SETTLE_* consts 3345-3356, ShellSettleStep 3361, ShellSettleWatch 3377, ShellSettleVerdict 3386, settle_pump_owes 3410, settle_watch_verdict 3427, ShellSettlePump 3472 | 143 | C03 |
| 3488-3517 | `#[cfg(wasm32)]` PendingExtensionInvocation 3489, PendingFileOpen 3503 | 30 | C03 |
| 3519-3587 | PointerHitOwner 3519, PointerCapture 3540 and impl 3551 | 69 | C03 |
| 3589-3626 | PresentedInputCandidateWitness 3589, PresentedInputGeometry 3591 and impl 3607 | 38 | C03 |
| 3627-3707 | SHELL_EXTENSION_LEDGER_CAPACITY 3627, ShellExtensionStoreRecord 3631, ShellExtensionLoadStatus 3644, ShellExtensionProjection 3651, WindowIconOverride 3659, WindowTitleOverride 3667, DockInputIdentity 3675, dock_mode_layout_identity 3682 | 81 | P |
| 3708-4302 | `pub struct ShellState` with its fields (no methods) | 595 | P |
| 4303-4326 | `//#endregion ShellTypes`; `#[cfg(not(wasm32))] impl Drop for ShellState` 4305 | 24 | P |
| 4327-4983 | `//#region ShellLifecycle` + `//#region PanelAnchorModel`: PanelAnchor 4335 and impl 4346, flowed_anchor_* 4429-4450, DockTabNode 4457, ShellDock 4485 and impl 4489, dock tab move fns 4554-4623, DockSkeleton 4660-4679, skeleton fns 4688-4751, PanelAnchorState 4780, PanelAnchorSnapshot 4802, DockUi* 4811-4835, os-shell-config layer fns 4843-4981 | 657 | C04 |
| 4985-5476 | `//#region WindowMeasures`: window_measures_surface_id 4990, operation_publication_refresh 4999, refresh_scope_label 5015, window_measures_owner 5026, window_measures_overlay_records 5041, MeasureRecord 5075, WindowMeasuresProjection 5085 and impl 5090-5186, measure_* helpers 5189-5265, `impl ShellState` 5266-5471 (publish_window_measures and related), test mod 5473-5475 | 492 | C04 |
| 5478-6911 | `//#region PanelRecordsAssembler`: panel_ui_records 5489, PanelRecord 5547, PanelProjection 5558 and impl 5564-6072, tree/row helpers 6076-6339, agent chat entry helpers 6359-6488, settings select/text rows 6348-6540, staged-arg rows 6541-6830, reference_selection_ids 6854, humanize_control_id 6872, panel_ui_node_tag 6887 | 1434 | C05 |
| 6913-6933 | `//#region ToolRunPanel`: tool_run_panel_reveals 6917, test mod 6930-6932 | 21 | C05 |
| 6935-6964 | `//#region AppCatalogueAttempt` 6935-6955; test mods 6957-6963 | 30 | C05 |
| 6965-8778 | `impl ShellState` part A: submit_shell_io 6968 ... boot_app_role 7741, apply_boot_mode 7859, debug_log 7890, sync_dock 8114, window instance and topology fns 8190-8460, host_effect_earned_scope 8460, settle_surface_faults 8724 | 1814 | C06 |
| 8779-10815 | `impl ShellState` part B (doc of live_window_ids starts 8779): live_window_ids 8782, world3d close/retire 8815-8888, trace/mirror engine surfaces 8914-8946, publish_app_catalogue 9028, document retention and retirement 9037-9170, build_settings_* 9263-10262, anchors 10280-10368, dock persistence 10529-10615, named layouts 10648-10669, uninstall/conflict rows 10716-10798; impl closes 10805; test mods 10807-10813; `//#endregion ShellLifecycle` 10814 | 2037 | C07 |
| 10816-10836 | `//#region ShellActions`; `pub type ShellTurn` 10820, shell_turn 10827 (native and wasm pairs) | 21 | C08 |
| 10837-12730 | `impl ShellState` part A: parse_persistence_binding 10843, active_sync_owner 10863, relabel_sync_tab 11126, footer_presence_rows 11140, sync_pill 11161, hub projection 11185-11204, observe_invocation_history 11286, handle_checkin_action 11500, request_hub_check_in 11525, handle_sync_action 11848, dispatch_action 12109-12729 | 1915 | C08 |
| 12731-15091 | `impl ShellState` part B (doc of dispatch_command starts 12731): dispatch_command 12752, dispatch_session_command 12764, handle_replay_shell_command 12804, refuse_replay 12839, open/close/ack space administration 12873-12891, directory_ctx 12945-12969, handle_hub_workspace_action 13078, begin_hub_artifact_creation 13202, hub sign-in 13351-13387, poll_hub_workspace_tasks 13499, install_document_services 13562, open_service_port 13585, pump_inference_port 13611, begin_document_resolution 13750, directory event pumps 14017-14154, bootstrap_identity 14172, apply_mutations 14388, extension record fns 14668-14757, plugin install 14819-14909, install_app_session 14962, apply_shell_uri 14999; impl closes 15089; `//#endregion ShellActions` 15090 | 2361 | C09 |
| 15092-15145 | `//#region ShellInput`; `impl ShellChromeBuildState` 15093 | 54 | C10 |
| 15146-15174 | ui_event_from_key_action 15146 | 29 | C10 |
| 15175-15245 | `impl ShellState` (small): tree_drag_handle_item_id 15191, tree_hit_item_id 15195, tree_row_has_chevron 15200; WORLD3D_CLOSE_CANCEL_SIGHTINGS 15207; HOST_WINDOW_BLUR_PENDING static 15211; note_host_window_blur 15216 (`#[wasm_bindgen]` 15228); window_host_event_action 15237; retained_router_owns_pointer 15242 | 71 | C10 |
| 15246-17173 | `impl ShellState` (pointer): handle_pointer_cancel 15248, retained hit registry 15879-16150, presented-input candidate 15954-16110, pointer ownership 16310-16496, handle_pointer_move 15672, handle_pointer_wheel 16496 | 1928 | C10 |
| 17174-19428 | `impl ShellState` part B (keyboard/palette/dock): update_tree_hover 17174, tree selection 17192-17201, settle pump 17349-17366, world3d catalogue drop 17651-17716, context menus 17850-18056, search/find items 18152-18228, command palette 18242-18359, dismiss_dismissable_layers 18483, handle_keyboard 18532, match_app_keybinding 18944, dock/keyboard ring 19011-19224, shortcut_table 19295, capture_keybinding_chord 19303; impl closes 19428 | 2255 | C11 |
| 19430-19489 | test mods (`shell_input`, `shell_shortcuts_palette`, `chord_example_role_parity`, `engine_surface_retention`, `shell_boot_isolation`, `pointer_hit_ownership`, `wheel_and_escape_routing`, `panel_window_reservation`, `dock_close_reopen_journals`, `board_presence`, `driver_editor`, `hub_projection_workspace`, `window_lifecycle_template_drag`, `settings_general_layout`, `detached_sync_task_manager`, `sync_owner_lifetime`) | 60 | C11 |
| 19490 | `//#endregion ShellInput` | 1 | C11 |
| 19491-19586 | chrome_text 19496, chrome_text_step 19508, complete/wrapped steps 19512-19537, test mods 19538-19544, empty_dock_notice_lines 19547, chrome_icon 19559, LOGO_UNTINTED 19574 | 96 | C12 |
| 19587-19898 | ChromeGroupItem 19587, RetainedChromeGroupStep 19597, width fns 19616-19636, render steps 19637-19759, floating panel sizing 19767-19780, dock tab helpers 19782-19800, anchor_panel_rect 19829, render_chrome_group (cfg test) 19853, utility_node_label 19893 | 312 | C12 |
| 19899-20473 | ShellChromeBandSpan 19899, CenteredBand 19912, free band fns 19920-19967, ShellNavbarChromeLayout 19974, ShellFooterChromeLayout 19980, SyncPill 20007-20050, ShellHubRemoteV1 20051, ShellHubDocumentV1 20066, ShellHubProjectionV1 20075, ShellHubSummaryV1 20082, hub connection text 20142, ShellHubCheckInV1 20166 (native), render_footer_status_step 20217, utility section divider 20238, utility path/render 20243-20350, render_utility_nodes (cfg test) 20352-20419, panel_tab_icon_id 20427, dock_tab_node_of 20464 | 575 | C12 |
| 20474-20605 | `//#region ActionPanelAndUtilities`: resolve_window_utilities 20479, action_host_window_id 20504 (cfg test), window_action_definition 20509, semio_cursor_from_name 20514, format_keybinding_shortcut 20542, is_reserved_shell_chord (cfg test) 20595 | 132 | C12 |
| 20606-21117 | `//#region ChromeParity`: is_reserved_shell_chord_in 20602, ShellNavbarControl 20615, width policy 20626-20640, labels 20642-20668, example rows 20675-20727, shell_mode/role controls 20731-20790, World3d status 20810-21088 (incl. world3d_compute_status 21081), safe-area types and fns 20948-21005, surface overlay fns 21006-21057 | 512 | C12 |
| 21119-21901 | `//#region WindowPaneChrome`: WindowPaneChip 21130-21216, world projection templates 21245-21351, window_pane_chip_rect 21388, `impl ShellState` 21402-21892 (window pane, projection, utility rail), test mods 21894-21900 | 783 | C12 |
| 21903-22717 | `//#region WindowActionsAndSearchPanes`: window_actions_surface_id 21908, FRAMEWORK_RESERVED_ACTION_IDS 21945, search ranking 21993-22050, engagement control rows 22061-22236, `impl ShellState` 22241-22712, test mod 22714-22716 | 815 | C13 |
| 22737-22861 | `//#region WindowScope`: command_host_platform 22733, WindowScopeStackV1 22742, mode_layout_stacks_v1 22763, resolve_keybinding_target_window_v1 22802, keybinding unowned text 22818-22830 | 125 | C13 |
| 22862-23140 | key_event_matches_chord 22866, reserved_shell_chords_v1 22840, keybinding_capture_key_token 22920, `//#region ShellShortcutTable` 22947: SHELL_SHORTCUT_ROWS 22961, ShellShortcut 22989, shell_shortcut_for_* 23014-23070, ShellKeybindingRow 23080, ShellEditVerb 23116-23131 | 279 | C13 |
| 23142-23915 | `impl ShellState` commands: active utility 23146-23217, staged args 23238-23262, execute_staged_action 23275, active_plugin_manifest 23300, build_os_commands 23314, resolved_commands 23373, command_search_items 23395, apply_os_command 23464, shell_command_for_control 23553, arm_* 23617-23687, command panel publish/build 23716-23902 | 774 | C13 |
| 23919-23998 | ResolvedCommand 23919, resolve_commands 23943, command_categories 23976 | 80 | C13 |
| 23988-24157 | `//#region FuzzyRankingParity`: FUZZY_* consts 23999-24001, normalize 24004, bounded_damerau_levenshtein 24009, fuzzy_token_score 24047, rank_fuzzy_items 24109; test mod 24154-24156; `//#endregion ActionPanelAndUtilities` 24157 | 170 | C13 |
| 24158-25888 | `//#region ShellChrome` + `//#region ChromeOverlaysAndTour`: tooltip hover 24176-24199, transient notices 24205-24300 (impl ShellChromeBuildState 24269), plugin install band 24303-24352, document-open band 24357-24403, dispatch fault and command rejection notices 24409-24530, AgentApprovalPaintOp 24530 and impl 24539-24694, ChromeDialogRequest 24713 and impl 24820-25249, chrome dialog helpers 25252-25285, ChromeTourState 25286 and impl ShellChromeBuildState 25292-25376, chrome element rects 25388-25517, introduction placement/layout 25530-25887 | 1731 | C14 |
| 25889-26701 | `//#region Tutorial`: TutorialMode 25901, TutorialCameraConverge 25913, TutorialRuntime 25923, TutorialPendingDocOp 25948, camera conversion 25972-26047, UI snapshot 26051-26126, gesture overlay 26128-26226, recording/date 26232-26259, pending doc op 26266, playhead and scrub fns 26286-26306, `impl ShellState` 26309-26668, recorder sample 26672, test mod 26698-26700 | 813 | C15 |
| 26702-28740 | `//#endregion Tutorial` (26701) then `ShellChromeFrameCursor` 26703, ShellUtilityPath 26722, ShellChromeChildCursor 26731, ShellChromeFramePhase 26752; `impl ShellState` 26902-28740: render_chrome_step 26921, prefs load/persist and introduction steps 27263-27579, chrome maintenance 27370-27450, board presence 27547-27567, nav/tab-row layout 27600-27725, dock_chrome_maps 27868, plan_dock_windows 27913, render_main_window_step 27972, anchor and tab bars 28226-28441, render_panel_step 28487, render_mobile_panel_step 28634 | 2039 | C16 |
| 28741-30289 | `impl ShellState` (render, part B): render_navbar_step 28741, surface overlay controls and status pills 29073-29135, render_navbar_cluster_step 29136, render_tutorial_bar_step 29178, render_footer_step 29330, render_overlay_step 29457, board_presence_paint_items 29885, render_context_menu_step 29898, render_chrome_tooltip_step 29980, render_agent_chat_header_step 30051, render_transient_notice_step 30102, render_plugin_install_step 30154, render_document_opening_step 30202, space_administration_plan 30264 | 1549 | C17 |
| 30290-31529 | `impl ShellState` (render, part C; doc of render_hub_workspace_step starts 30290): render_hub_workspace_step 30292, render_space_administration_step 30377, resolve_space_administration_control 30424, render_agent_approvals_step 30444, agent approvals paint/click 30490-30567, chrome_dialog_paint_ops 30568, render_chrome_dialog_step 30765, click/key/accessibility handlers 30812-31004, render_chrome_dialog 31005, render_chrome_tour_step 31066, render_chrome_tooltip 31249, tour helpers 31297-31424, render_context_menu_level 31441 (native close `}` at 31529) | 1240 | C18 |
| 31530-33459 | `// #region UiPrefsThemesI18n` 31531 and `//#region ChromeI18n`/`PrefsSync` blocks: storage keys 31540-31566, UI_* and OS_SHELL_CONFIG consts 31542-31570, PrefsStore trait and FilePrefsStore (test only) 31583-31720, native pref file fns 31762-31890, raw/stored prefs fns 31819-31906, StorageDoor 31910-32136 (encode/decode, host_storage_* 32079-32131, `#[wasm_bindgen]` 32131), AgentBridgeOfferScope 32144-32220 (`#[wasm_bindgen]` 32216), AgentBridgeConfigDoor 32222-32270 (`#[wasm_bindgen]` 32240), ShellPrefLocks (test) 32280-32310, ChromePrefsState 32315-32434 (`//#region ThemeRegistry` 32312-32434), `shell_chrome_string` 32477-33079 (`//#region ChromeI18n` 32436-33081), UiPrefsSnapshot and event log 33085-33200, resolve appearance/language 33210-33270, persist_ui_preferences 33299-33361, `impl ShellState` 33363-33403 (`//#region PrefsSync` 33083-33403), test mods 33405-33456, `//#endregion ShellChrome` 33459 | 1930 | C19 |
| 33460-33569 | blank, then `download_media_export` (cfg pairs) 33469-33506, `#[cfg(wasm32)]` export/door fns 33511-33564 | 110 | C20 |
| 33570-33925 | `//#region 📤️FileOpenImport`: OpenedFile 33574, file_open_import_actions 33588, PickedImport 33612, TransferLane 33623 (impl 33628), ImportTransfer 33642, import_transfer_notice 33656, `impl ShellState` 33668-33778, host IO trace fns 33781-33880, `host_io_call` 33879 (`pub(crate) async fn`), test mod 33922-33924 | 356 | C20 |
| 33927-34126 | `//#region RequestMediaFrames`: decode_data_url 33931, fallback_action_descriptor 33940, ffmpeg_* 33956-34078, request_media_frames 34080 (wasm), test mods 34099-34126 | 200 | C20 |
| 34128-34213 | `//#region 🎨️ThemeDocumentModel`: SHELL_THEME_DOCUMENT_JSON `include_str!` 34138, ThemeNumber 34143 (impls 34148-34186) | 86 | C20 |
| 34201-34310 | ThemePaintRef 34201, ThemeIcons 34219, ThemeDocument 34241, `#[path = "...geometry/🦀️.rs"] mod theme_geometry;` 34280-34281, impl ThemeDocument 34283, custom_theme_document_from 34308 | 110 | C20 |
| 34314-34516 | shell_theme_document_base 34314, theme_parse/blend/resolve fns 34338-34405, theme_from_document 34406, theme_document_geometry 34474, apply_theme_document_metrics 34492 | 203 | C20 |
| 34518-34820 | `//#region 🎨️ThemeEditorPanel`: SHELL_THEME_EDITOR_* consts 34521-34542, theme_editor_input_item 34544, `impl ShellState` 34577-34811, theme_paint_editor_values 34816 | 303 | C20 |
| 34822-35013 | `//#region 🎨️ThemeDocumentDoors` 34822-34911 (set_active_theme_document 34827, save_custom_theme_document 34839), `//#region ⚖️MergePolicyAndThemeCommits` 34913-35013 (SHELL_MERGE_POLICY_OPTIONS 34917, `impl ShellState` 34926-35012) | 192 | C20 |
| 35015-35158 | `//#region ♿️ChromeAccessibleNames`: SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY 35028, ChromeControlPresentation 35031, CHROME_CONTROL_NAMES 35051, note_chrome_* fns 35066-35146, chrome_accessibility_role 35146 | 144 | C20 |
| 35159-35517 | `impl ShellState` (accessibility nodes): chrome_control_label 35165, palette/dialog/chrome accessibility nodes 35172-35499, footer_status_chips 35509 | 359 | C20 |
| 35520-35558 | chrome_status_accessibility_node 35520; `//#endregion ♿️ChromeAccessibleNames` 35558 | 39 | C20 |
| 35560-35570 | test mods: theme_editor_and_accessibility, native_keyboard_ring, presented_input_authority | 11 | C20 |

Coverage check: parent 1-138 + 144-149 + 1815-1826 + 3627-4326; children C01-C20 cover the rest. Blank lines at 1827, 2657, 3626, 4326-4327 boundary, 5477, 6964, 10815, 12730, 15091, 19491, 21902, 24158, 25889, 26702, 28740, 30289, 31530, 33460 belong to the adjacent child as listed in section 3.

## 3. Partition: 20 child modules

Parent after the split keeps lines 1-149 (imports; 144-149 lifted to the top), 1815-1826 (wiring plus 20 new `#[path]`/`mod`/`use` triples), 3627-4326 (ShellState and Drop). Size about 860 lines plus about 60 lines of new wiring.

### 3.1 Children

Folder names use an emoji that already prefixes existing folders in the repo (counts checked under `🧰️framework`). All folder names are at most 36 characters. The longest child path is about 201 bytes relative to the repo root, plus the 28-byte `/Users/ueli/Documents/semio/` prefix, which stays under 256.

| # | Folder (`wgpu/<folder>/🦀️.rs`) | `mod` name | Ranges (inclusive) | Lines | Main contents | Cross-links: this child's symbols used by / this child uses |
|---|---|---|---|---|---|---|
| C01 | `🔖️identity-and-types` | identity_and_types | 139-143, 150-1814 | 1676 | identity pure, replay refusal, conflict/check-in pure, ShellTypes first part, ChromeShellBuildState | used by C08 40, C07 37, C11 33, C09 28, C10 20, C13 20, C06 20, P 18 / uses C14 14, C03 13, C04 11 |
| C02 | `🧵️shell-async-runners` | shell_async_runners | 1827-2656 | 830 | ShellDetached, pool future, directory and inference runners, DirectoryHomeProjection, ShellDocumentRetirement | used by C09 23, C06 15, P 12 / uses C03 12, C19 6, C01 5 |
| C03 | `🏛️space-admin-and-settle` | space_admin_and_settle | 2657-3626 | 970 | SpaceAdministration, DirectoryCommandQueue, HubWorkspaceTask, BootProgramSelection, settle pump, PointerCapture, PresentedInputGeometry | used by C09 32, P 28, C11 22, C06 21 / uses C02 10, C19 7, C14 5 |
| C04 | `⚓️dock-model-and-measures` | dock_model_and_measures | 4327-5476 | 1150 | PanelAnchor, ShellDock, DockTabNode, DockSkeleton, dock/named-layout stores, WindowMeasures projection and impl | used by C07 37, C16 21, C05 17 / uses C19 15, C03 8, C05 5 |
| C05 | `🧾️panel-records` | panel_records | 5477-6964 | 1488 | PanelRecordsAssembler (panel_ui_records, PanelProjection), settings row helpers, agent chat entry nodes, ToolRunPanel, AppCatalogueAttempt | used by C07 18, C13 7, C06 7 / uses C04 17, C03 7, C14 6 |
| C06 | `🧩️session-and-topology` | session_and_topology | 6965-8778 | 1814 | impl ShellState part A: IO, session, boot, dock sync, window instance ids, window-topology publication, settle faults | used by C10 21, C11 19, C09 17, C07 15, C08 15 / uses C03 21, C01 20, C02 15, C04 14 |
| C07 | `⚙️settings-and-dock-persistence` | settings_and_dock_persistence | 8779-10815 | 2037 | impl ShellState part B: live window ids, world3d close, engine surfaces, app catalogue, document retention, settings panel builders, anchors, dock/named-layout persistence | used by C13 26, C11 14, C16 14, C06 13, C08 13 / uses C01 37, C04 37, C05 18 |
| C08 | `🎬️shell-actions` | shell_actions | 10816-12730 | 1915 | ShellTurn, persistence bindings, sync pill, hub projection/check-in, observe_invocation_history, handle_sync_action, dispatch_action | used by C09 6, C16 4, C13 3 / uses C01 40, C12 20, C20 17, C13 15, C06 15 |
| C09 | `📨️command-dispatch` | command_dispatch | 12731-15091 | 2361 | impl ShellState part B: dispatch_command, session commands, replay, space admin open/close, hub workspace actions, document services, directory event pumps, identity bootstrap, apply_mutations, extension record, plugin install, apply_shell_uri | used by C08 11, C14 4 / uses C03 32, C01 28, C02 23, C06 17 |
| C10 | `🖱️pointer-input` | pointer_input | 15092-17173 | 2082 | ShellChromeBuildState impl, ui_event_from_key_action, tree helpers, HostWindowBlur, pointer handlers, retained hit registry, presented input candidate | used by C11 7, C13 4, C16 3 / uses C11 25, C06 21, C01 20, C14 18 |
| C11 | `⌨️keyboard-and-palette` | keyboard_and_palette | 17174-19490 | 2317 | impl ShellState part C: tree hover, context menus, search/find items, command palette, keyboard, dock and keyboard ring, shortcut capture; test mods 19430-19489 | used by C10 25, C13 5, C20 4 / uses C01 33, C13 31, C03 22, C06 19 |
| C12 | `🧱️chrome-paint-and-navbar` | chrome_paint_and_navbar | 19491-21901 | 2411 | chrome text/icon/group painting, bands, navbar/footer layouts, SyncPill, hub check-in, utility rendering, resolve_window_utilities, keybinding formatting, navbar controls, World3d status, safe area, WindowPaneChip, world projection templates, impl ShellState window pane | used by C16 32, C17 29, C08 20 / uses C04 15, C03 13, C01 11 |
| C13 | `🧰️window-actions-and-commands` | window_actions_and_commands | 21902-24157 | 2256 | window actions/search panes, WindowScope, chord helpers, ShellShortcutTable, command impl (staged args, build_os_commands, command panel), ResolvedCommand, fuzzy ranking | used by C11 31, C08 15, C12 10 / uses C07 26, C01 20, C04 14 |
| C14 | `🗨️chrome-overlays-and-tour` | chrome_overlays_and_tour | 24158-25888 | 1731 | tooltips, transient notices, plugin install and document-open bands, fault notices, agent approval paint ops, ChromeDialog request/paint helpers, ChromeTourState, element rects, introduction placement | used by C18 49, C17 29, C10 18 / uses C19 15, C01 10, C03 10 |
| C15 | `🎓️tutorial-runtime` | tutorial_runtime | 25889-26701 | 813 | Tutorial types and impl, camera conversion, UI snapshot, gesture overlay, recording, pure helpers | used by C17 7, C08 3 / uses C04 9, C19 6, C06 6 |
| C16 | `🖼️chrome-frame-and-main-window` | chrome_frame_and_main_window | 26702-28740 | 2039 | ShellChromeFrameCursor and cursor types; impl ShellState render part A: render_chrome_step, prefs/introduction steps, maintenance, board presence, nav/tab-row layout, dock windows, render_main_window_step, anchors, render_panel_step, mobile panel | used by C17 16, C10 7 / uses C12 32, C04 21, C19 20 |
| C17 | `🧭️navbar-footer-and-overlays` | navbar_footer_and_overlays | 28741-30289 | 1549 | impl ShellState render part B: render_navbar_step, overlay controls, navbar cluster, tutorial bar, footer, overlay, context menu, tooltip, agent chat header, transient notice, plugin install, document opening, space admin plan | used by C16 8 / uses C12 29, C14 29, C16 16 |
| C18 | `💬️chrome-dialog-and-tooltips` | chrome_dialog_and_tooltips | 30290-31529 | 1240 | impl ShellState render part C: hub workspace, space admin render, agent approvals, chrome dialog render/keys/accessibility, chrome tour render, tooltip, context menu level | used by C10 4, C16 4, C11 3 / uses C14 49, C01 10, C19 10 |
| C19 | `🔑️storage-prefs-and-i18n` | storage_prefs_and_i18n | 31530-33459 | 1930 | storage keys and door, prefs store, pref locks, ChromePrefsState, ThemeRegistry, chrome i18n string table, UiPrefsSnapshot and event log, persist_ui_preferences, PrefsSync impl; test mods 33405-33456 | used by C16 20, C14 15, C04 15, C10 13 / uses C03 10, C20 6, C04 6 |
| C20 | `🎨️theme-file-import-and-accessibility` | theme_file_import_and_accessibility | 33460-35570 | 2111 | download_media_export, file open/import (OpenedFile, TransferLane, ImportTransfer, impl ShellState), host_io_call, request_media_frames, theme document model (ThemeNumber, ThemeDocument with `mod theme_geometry`), theme editor impl, theme doors, merge policy, chrome accessibility names and nodes; test mods 33923-35569 | used by C08 17, C07 10, C16 8, C06 8 / uses C14 13, C01 11, C03 9 |

Sizes: min 813 (C15), max 2411 (C12). All 20 children are between 800 and 3000 lines. Parent is about 860 lines.

### 3.2 Splits inside `impl ShellState` blocks (cut at method boundaries)

The four cuts are at blank lines between methods. Each cut needs a closing `}` appended to the upper file and an `impl ShellState {` opener at the top of the lower file. Doc comments belong to the method below the cut, and the cut line is the blank line above them.

| Original impl | Lines | Cut after | Upper child | Lower child |
|---|---|---|---|---|
| ShellLifecycle impl | 6965-10805 | 8778 (blank; `live_window_ids` doc starts 8779) | C06 gets 6965-8778 plus `}` | C07 gets `impl ShellState {` plus 8779-10805 |
| ShellActions impl | 10837-15089 | 12730 (blank; `dispatch_command` doc starts 12731) | C08 gets 10837-12730 plus `}` | C09 gets `impl ShellState {` plus 12731-15089 |
| ShellInput pointer impl | 15246-19428 | 17173 (blank; `update_tree_hover` at 17174, no doc) | C10 gets 15246-17173 plus `}` | C11 gets `impl ShellState {` plus 17174-19428 |
| ShellChrome render impl | 26902-31529 | 28740 and 30289 (blank; `render_hub_workspace_step` doc starts 30290) | C16 gets 26902-28740 plus `}`; C17 gets `impl ShellState {` plus 28741-30289 plus `}` | C18 gets `impl ShellState {` plus 30290-31529 (keeps original `}` at 31529) |

Other `impl` blocks move whole without splitting: 5266-5471 (C04), 15175-15190 (C10), 21402-21892 (C12), 22241-22712 (C13), 23142-23915 (C13), 24539-24694 (C14), 26309-26668 (C15), 33363-33403 (C19), 33668-33778 (C20), 34577-34811 (C20), 34926-35012 (C20), 35159-35517 (C20).

Note: `#[wasm_bindgen]` and `#[cfg]` attributes always precede their `fn` with no blank line, so a cut at a blank line never separates an attribute from its item (verified at 8777/8778, 12729/12730, 17172/17173, 28739/28740, 30288/30289).

### 3.3 Re-exports and the parent glue

Parent wiring to add (after the existing four children; one `#[path]`, `mod` and glob `use` per child):

```rust
#[path = "🔖️identity-and-types/🦀️.rs"]  mod identity_and_types;   use identity_and_types::*;
#[path = "🧵️shell-async-runners/🦀️.rs"] mod shell_async_runners;   use shell_async_runners::*;
#[path = "🏛️space-admin-and-settle/🦀️.rs"] mod space_admin_and_settle; use space_admin_and_settle::*;
#[path = "⚓️dock-model-and-measures/🦀️.rs"] mod dock_model_and_measures; use dock_model_and_measures::*;
#[path = "🧾️panel-records/🦀️.rs"]      mod panel_records;          use panel_records::*;
#[path = "🧩️session-and-topology/🦀️.rs"] mod session_and_topology;   use session_and_topology::*;
#[path = "⚙️settings-and-dock-persistence/🦀️.rs"] mod settings_and_dock_persistence; use settings_and_dock_persistence::*;
#[path = "🎬️shell-actions/🦀️.rs"]      mod shell_actions;          use shell_actions::*;
#[path = "📨️command-dispatch/🦀️.rs"]   mod command_dispatch;       use command_dispatch::*;
#[path = "🖱️pointer-input/🦀️.rs"]      mod pointer_input;          use pointer_input::*;
#[path = "⌨️keyboard-and-palette/🦀️.rs"] mod keyboard_and_palette;  use keyboard_and_palette::*;
#[path = "🧱️chrome-paint-and-navbar/🦀️.rs"] mod chrome_paint_and_navbar; use chrome_paint_and_navbar::*;
#[path = "🧰️window-actions-and-commands/🦀️.rs"] mod window_actions_and_commands; use window_actions_and_commands::*;
#[path = "🗨️chrome-overlays-and-tour/🦀️.rs"] mod chrome_overlays_and_tour; use chrome_overlays_and_tour::*;
#[path = "🎓️tutorial-runtime/🦀️.rs"]   mod tutorial_runtime;       use tutorial_runtime::*;
#[path = "🖼️chrome-frame-and-main-window/🦀️.rs"] mod chrome_frame_and_main_window; use chrome_frame_and_main_window::*;
#[path = "🧭️navbar-footer-and-overlays/🦀️.rs"] mod navbar_footer_and_overlays; use navbar_footer_and_overlays::*;
#[path = "💬️chrome-dialog-and-tooltips/🦀️.rs"] mod chrome_dialog_and_tooltips; use chrome_dialog_and_tooltips::*;
#[path = "🔑️storage-prefs-and-i18n/🦀️.rs"] mod storage_prefs_and_i18n; use storage_prefs_and_i18n::*;
#[path = "🎨️theme-file-import-and-accessibility/🦀️.rs"] mod theme_file_import_and_accessibility; use theme_file_import_and_accessibility::*;
```

Put each `#[path]`/`mod` pair on separate lines in the real file (the one-line form above is only for readability).

Glob `use child::*;` in the parent makes each child's `pub(super)` items visible to all siblings through their `use super::*;`. This is how sibling access works without listing names.

External re-exports (crate-internal consumers only; no consumer outside this crate was found, `renderer_wgpu::shell` matches nothing in the repo):

| Symbol | Current line | Child | Re-export |
|---|---|---|---|
| ShellFindItem, SpawnedAppEntry, SpacePanelState, try_push_find_item | 247, 1239, 1258, 383 | C01 | `pub use identity_and_types::{ShellFindItem, SpawnedAppEntry, SpacePanelState, try_push_find_item};` |
| PointerHitOwner, PointerCapture | 3519, 3540 | C03 | `pub use space_admin_and_settle::{PointerHitOwner, PointerCapture};` |
| PresentedInputCandidateWitness | 3589 | C03 | `pub(crate) use ...::PresentedInputCandidateWitness;` |
| ShellSettleWatch, ShellSettleVerdict, settle_pump_owes, settle_watch_verdict, SHELL_SETTLE_STALL_STEPS, SHELL_SETTLE_TERMINAL_DRIVES | 3377, 3386, 3410, 3427, 3345, 3351 | C03 | `pub(crate) use ...` (all are `pub(crate)` in source) |
| note_host_window_blur | 15216 | C10 | `pub use pointer_input::note_host_window_blur;` |
| ShellHubRemoteV1 | 20051 | C12 | `pub use chrome_paint_and_navbar::ShellHubRemoteV1;` |
| resolve_window_utilities, action_host_window_id, resolve_boot_example_id, world3d_compute_status | 20479, 20504, 20707, 21081 | C12 | `pub(crate) use chrome_paint_and_navbar::{...};` |
| key_event_matches_chord | 22866 | C13 | `pub(crate) use window_actions_and_commands::key_event_matches_chord;` |
| dispatch_refusal_discards_input_draft | 24465 | C14 | `pub(crate) use chrome_overlays_and_tour::dispatch_refusal_discards_input_draft;` |
| ShellChromeFrameCursor | 26703 | C16 | `pub(crate) use chrome_frame_and_main_window::ShellChromeFrameCursor;` |
| resolve_theme_for_ids | 32424 | C19 | `pub use storage_prefs_and_i18n::resolve_theme_for_ids;` (pub in source) |
| shell_language_axes | 33238 | C19 | `pub(crate) use storage_prefs_and_i18n::shell_language_axes;` |
| host_io_call | 33879 | C20 | `pub(crate) use theme_file_import_and_accessibility::host_io_call;` |

Use `pub use` only where the source item is `pub`; a `pub use` of a `pub(crate)` item fails (E0364). Test files outside the monolith reach these items as `shell::X` or `crate::shell::X`, so every symbol in this table must be re-exported. Test code in the same crate also reaches items through `use super::*;` (51 of 55 test files under `Shell/🧪️tests` use it), which the glob chain covers.

### 3.4 Coupling notes (what to watch)

- C14 to C18 is the heaviest seam (49 symbols). The chrome dialog state and types live in C14, but its paint, key and accessibility methods sit in C18 at 30568-31065. Optional refinement: move 30568-31065 into C14 as a second `impl ShellState { ... }` block. C18 then shrinks to about 740 lines. This is optional; the plan works without it.
- C03 to C09 (32) and C03 to P (28): settle and pointer types are used by the dispatch and the parent struct. Keep C03 as one unit.
- C12 to C16 (32) and C12 to C17 (29): C16 and C17 call navbar/footer layout and paint helpers from C12. They are the reason C12 has many `pub(super)` items.
- C11 to C13 (31) and C07 to C13 (26): keyboard/palette and window-scope/command code call each other's `ShellState` methods. No change needed beyond `pub(super)`.
- Lowest coupling (sum of cross-chunk symbol uses in both directions): C15 (63), C18 (139), C05 (140), C17 (157), C09 (179), C02 (186). Highest: C01 (389), C07 (304), C14 (303), C04 (296), C06 (292).

## 4. Visibility after moving

### 4.1 Rule

1. Every item moved into a child that is referenced from any other chunk (parent or sibling) must be `pub(super)`. Items already `pub` or `pub(crate)` keep their visibility.
2. Blanket rule (recommended, simpler than the candidate list in appendix A): prefix every private top-level item and every private `fn` in `impl ShellState` blocks of the child with `pub(super) `. Do the same for struct fields of moved structs when a sibling or the parent accesses them. This only widens visibility inside the `shell` subtree.
3. Methods: a private method moved into a child is callable only from that child. After the blanket rule, sibling calls resolve through `self.` with no import, because method lookup does not need a name import.
4. Free fns, consts, types: sibling access goes through the parent glob `use child::*;` (section 3.3), so each one must be `pub(super)`.
5. Do not widen anything to `pub` except the items in 3.3, to keep the public surface as it is today.
6. Test modules keep `use super::*;` and reach the child's own items and the glob chain.

### 4.2 Candidate names per child

Generated by tokenizing every line of the file for identifiers defined in the file (fn, struct, enum, trait, type, const, static, impl methods at 4-space indent) and checking which chunk each use sits in. Names used from a chunk other than their defining child, with private definitions, are listed below. The scan includes doc comments and string literals, so it over-approximates. Generic names (`new`, `default`, `get`, `set`, `iter`, `take`, `close`, `publish`, `name`, `node`, `label`, `drop`, `from_value`, `to_value`, `contains`, `visit`, `measure`, `parse`) are often method names on other types. Keep them only after `cargo check` shows they need widening; the blanket rule covers them anyway.

Counts: C01 122, C02 35, C03 24, C04 27, C05 26, C06 47, C07 21, C08 9, C09 10, C10 13, C11 30, C12 59, C13 21, C14 93, C15 5, C16 20, C17 8, C18 15, C19 40, C20 27 (652 names total).

Appendix A (see end of file) lists them.

## 5. Relative paths and include macros

Only the lines below contain relative paths. Rule: a moved item keeps its path text unless it is a relative path or include. For those, add one `../` (the child file is one folder deeper than `wgpu/`, so every path needs one more step up). Paths in parent-kept lines do not change.

Parent-kept lines, unchanged: 11 (`../../../🏛️ShellHost/...document/🦀️.rs`), 1815, 1818, 1821, 1825 (sibling-folder paths), plus the doc line 4 (text only).

Moved lines, each gets one extra `../` in its child file:

| Line | Child | Kind | Current text (abridged) | After move |
|---|---|---|---|---|
| 804 | C01 | `include_str!` | `"../../../🏛️ShellHost/📣️replay-refusal/🔣️.json"` | `"../../../../🏛️ShellHost/📣️replay-refusal/🔣️.json"` |
| 1216 | C01 | `#[path]` test | `"../../🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs"` | `"../../../🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs"` |
| 2485, 2654 | C02 | `#[path]` test | `"../../🧪️tests/🔬️wgpu-shell-pool-future/🦀️.rs"`, `"../../🧪️tests/🔬️wgpu-shell-document-retirement/🦀️.rs"` | `"../../../🧪️tests/..."` (same suffix) |
| 5474 | C04 | `#[path]` test | `"../../🧪️tests/📏️wgpu-window-measures/🦀️.rs"` | `"../../../🧪️tests/..."` |
| 6931, 6958, 6962 | C05 | `#[path]` tests | `tool-run-panel`, `app-catalogue-attempt`, `action-window-scope` | `"../../../🧪️tests/..."` |
| 10808, 10812 | C07 | `#[path]` tests | `wgpu-panel-anchor-model`, `wgpu-display-conflicts-marketplace` | `"../../../🧪️tests/..."` |
| 19431 to 19488 (16 attrs) | C11 | `#[path]` tests | all `"../../🧪️tests/…"` | `"../../../🧪️tests/…"` |
| 19539, 19543, 21895, 21899 | C12 | `#[path]` tests | `"../../🧪️tests/…"` | `"../../../🧪️tests/…"` |
| 22715, 24155 | C13 | `#[path]` tests | `"../../🧪️tests/…"` | `"../../../🧪️tests/…"` |
| 26699 | C15 | `#[path]` test | `"../../🧪️tests/🔬️wgpu-tutorial/🦀️.rs"` | `"../../../🧪️tests/…"` |
| 33407, 33416, 33422, 33432, 33438, 33444, 33450, 33455 (8 attrs) | C19 | `#[path]` tests | `"../../🧪️tests/…"` | `"../../../🧪️tests/…"` |
| 33923, 34100, 34105, 34109, 34113, 34117, 34121, 34125 (8 attrs) | C20 | `#[path]` tests | `"../../🧪️tests/…"` | `"../../../🧪️tests/…"` |
| 34138 | C20 | `include_str!` | `"../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🔣️.json"` (9 `..`) | 10 `..` |
| 34280 | C20 | `#[path]` mod | `"../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🌓️theme/📐️geometry/🦀️.rs"` (9 `..`) | 10 `..` |
| 35561, 35565, 35569 | C20 | `#[path]` tests | `"../../🧪️tests/…"` | `"../../../🧪️tests/…"` |

Total: 54 moved attributes/includes, 5 parent-kept. This matches the precedent in `🪟️tree-windows/🦀️.rs:201`.

Two alternatives for the test attributes: (a) keep all `#[path]` test mods declared in the parent file and leave the paths untouched (they will then reach items in other children only via the glob chain); (b) move each test mod with the child that owns the code it tests, and rewrite as shown. Option (b) is recommended and is the plan above.

Other path-like text: `use` paths are crate-absolute (`crate::...`, `semio_framework...`) and do not change. Three `use ...DialectCoordinateText as _` lines (664, 9597, 13306) are fn-local and move with their fn bodies. No `self::` or `super::` appears anywhere in the file. The only `crate::shell::` occurrence is in a doc comment at line 6. Intra-doc links like `[`Self::pump_hub_artifact_creation`]` remain valid.

## 6. Extraction sequence and procedure

Anchor lines are the first and last lines of each child range, verified at audit time. Re-check them before each cut.

| Child | First line (text) | Last line (text) |
|---|---|---|
| C01 | 139 `const FRAMEWORK_DISPLAY_WINDOWS_TAB_ID` | 1814 blank (1813 `//#endregion 🔖️ChromeThreadBoundary`) |
| C02 | 1827 blank (1828 `//#region 🧵️ShellDetached`) | 2656 `//#endregion 📄️ShellDocumentRetirement` |
| C03 | 2657 blank (2658 `//#region 🏛️SpaceAdministration`) | 3626 blank (3625 `}`) |
| C04 | 4327 `//#region ShellLifecycle` | 5476 `//#endregion 📏️WindowMeasures` |
| C05 | 5477 blank (5478 `//#region 🧾️PanelRecordsAssembler`) | 6964 blank (6963 `mod action_window_scope_tests;`) |
| C06 | 6965 `impl ShellState {` | 8778 blank (8777 `    }`); append `}` |
| C07 | 8779 doc `/// 🪟️ Every declared window instance` (prepend `impl ShellState {`) | 10815 blank (10814 `//#endregion ShellLifecycle`) |
| C08 | 10816 `//#region ShellActions` | 12730 blank (12729 `    }`); append `}` |
| C09 | 12731 doc `/// 🎛️ Sends one owner-qualified` (prepend `impl ShellState {`) | 15091 blank (15090 `//#endregion ShellActions`) |
| C10 | 15092 `//#region ShellInput` | 17173 blank (17172 `    }`); append `}` |
| C11 | 17174 `    fn update_tree_hover` (prepend `impl ShellState {`) | 19490 `//#endregion ShellInput` |
| C12 | 19491 blank (19492 `/// ✍️ Immediate-mode Shell chrome text`) | 21901 `//#endregion 🪟️WindowPaneChrome` |
| C13 | 21902 blank (21903 `//#region 🎬️WindowActionsAndSearchPanes`) | 24157 `//#endregion ActionPanelAndUtilities` |
| C14 | 24158 blank (24159 `//#region ShellChrome`) | 25888 `//#endregion 🔖️ChromeOverlaysAndTour` |
| C15 | 25889 blank (25890 `//#region 🎬️Tutorial`) | 26701 `//#endregion 🎬️Tutorial` |
| C16 | 26702 blank (26703 `pub(crate) struct ShellChromeFrameCursor`) | 28740 blank (28739 `    }`); append `}` |
| C17 | 28741 `    fn render_navbar_step` (prepend `impl ShellState {`) | 30289 blank (30288 `    }`); append `}` |
| C18 | 30290 doc `/// 🔐️ Paints the /hub workspace` (prepend `impl ShellState {`) | 31529 `}` (end of impl) |
| C19 | 31530 blank (31531 `// #region 💾️🎨️🌐️ UiPrefsThemesI18n`) | 33459 `//#endregion ShellChrome` |
| C20 | 33460 blank (33461 `/// ⬇️ Browser half of the wgpu shell's export door`) | 35570 `mod presented_input_authority_tests;` (last line of file) |

Preparation (before the first cut):
1. Lift lines 144-149 (`use` items) into the parent import block; they are part of C01's text range otherwise.
2. Keep a byte copy of the original file outside the repo (scratch) for diffing after each step.
3. Make sure the parent region markers stay balanced: `//#region ShellTypes` opens at 1220 (now in C01) and closes at 4303 (in P). Open and close each marker in the same file. Likewise `ShellLifecycle` (C04 opens, C07 closes), `ShellActions` (C08/C09), `ShellInput` (C10/C11), `ActionPanelAndUtilities` (C12/C13 end), `ShellChrome` (C14 opens at 24159, C19 closes at 33459).

Order: extract from the bottom of the file upward. Each cut then leaves the line numbers above it unchanged, so the table in section 2 and section 6 stay valid for later steps. Sequence:

1. C20 (33460-35570)
2. C19 (31530-33459)
3. C18 (30290-31529)
4. C17 (28741-30289)
5. C16 (26702-28740)
6. C15 (25889-26701)
7. C14 (24158-25888)
8. C13 (21902-24157)
9. C12 (19491-21901)
10. C11 (17174-19490)
11. C10 (15092-17173)
12. C09 (12731-15091)
13. C08 (10816-12730)
14. C07 (8779-10815)
15. C06 (6965-8778)
16. C05 (5477-6964)
17. C04 (4327-5476)
18. C03 (2657-3626)
19. C02 (1827-2656)
20. C01 (139-143, 150-1814)

Each step, in order:
a. Create `wgpu/<folder>/🦀️.rs` with the `//!` doc (first character an emoji, same folder emoji), then `use super::*;`.
b. Copy the range; apply the cut edits from 3.2 (openers/closers) and the path rewrites from section 5.
c. Apply the blanket `pub(super)` rule (section 4.1) to the moved text.
d. In the parent: add `#[path]`, `mod`, and `use <child>::*;` (section 3.3), and the `pub use`/`pub(crate) use` lines for the children that own re-exported symbols.
e. Delete the moved range from the parent, leaving the blank line.
f. Verify the step compiles before the next step (section 7).

Each intermediate state is compile-safe: un-moved code stays in the parent (or a sibling) and reaches moved items through the glob chain once they are `pub(super)`.

## 7. Verification (to be run by the executor; not run here)

- Both targets must compile: native and `wasm32` (many `#[cfg(target_arch = "wasm32")]` branches; keep each pair in the same child). Use the repo's cargo gate, not raw cargo (per the project memory, raw cargo is refused).
- Test build: `--tests` for the renderer crate, since 55 test files are wired by `#[path]`.
- Line count check: parent under 1000 lines (target about 860 plus wiring); each child 800-3000 lines.
- Diff check: concatenation of all child ranges plus parent ranges equals the original line set, modulo the inserted openers/closers, `pub(super)` prefixes, `..` path edits, child headers and wiring.

## 8. Hazards

1. Cut order: extracting top-down invalidates every later range in this document. Use bottom-up (section 6) or re-locate by anchor text.
2. `impl ShellState` must be re-opened in every lower half of a cut (section 3.2). Forgetting the opener or closer is the most likely syntax error.
3. Relative paths: 54 moved attributes/includes need one extra `../` (section 5). Missing one fails only at compile time with E0583 or include errors, so check each file.
4. `pub(super)` visibility on methods: calls to private methods across children fail with E0624 until widened. The blanket rule (4.1) avoids the iteration.
5. Struct fields: fields of moved structs accessed from another child also need `pub(super)`. The field-level list is not in appendix A; apply the blanket rule to fields of every moved struct.
6. Glob cycles: children use `use super::*;` and the parent uses `use child::*;`. This cycle is normal Rust, but any name defined in two children (only method names duplicate; free items do not) would be ambiguous. None found in the scan; check during step c.
7. `pub use` of a `pub(crate)` item fails (E0364). Follow the table in 3.3 exactly.
8. Parent imports: after the split some `use` lines in the parent are used only through children. Keep them; `unused_imports` warnings may appear and should be resolved by moving the import into the one child that needs it, not by deleting it.
9. `#[wasm_bindgen(js_name = ...)]` exports (lines 557, 15228, 32131, 32216, 32240) keep their names; `pub` stays `pub`.
10. `#[cfg]`-paired duplicates (native/wasm): `resolve_identity_env` 484/491, `shell_directory_request_base_url` 504/509, `shell_directory_transport` 546/550, `chrome_now_ms` 25407/25412, `download_media_export` 33469/33489, `host_io_request_trace_label` 33856/33874, `HostIoRequestTraceGuard` impls 33817/33849. The plan keeps each pair in one child.
11. Cfg-gated `use` lines 14-17, 94-121 stay in the parent; they feed children through the glob chain.
12. Private structs with private fields used from the parent `ShellState` struct literal or from siblings (ShellChromeBuildState, ShellChromePresentState, WindowTopologyPublication, etc.): visibility on fields, not only on the struct.
13. Doc-only references (`shell::ShellChrome`, `ShellInput`, `ActionPanelAndUtilities`, `shell::resolve_boot_example_id` at 19758) name regions, not paths. No change needed; some doc links use `crate::host_platform_uses_meta`, unchanged.
14. The `#[allow(clippy::too_many_arguments, reason = ...)]` attribute at 30291 must move with its fn (it is in the C18 range).
15. Stray column-zero `use` lines inside fn bodies (664, 9597, 13306) must stay inside their fns. Do not hoist them.

## 9. Target outcome

- Parent: about 860 lines plus about 60 wiring lines (under 1000; target below 3000 is met with wide margin).
- 20 children, all between 813 and 2411 lines.
- The four existing children (`📤️icon-export`, `🪟️tree-windows`, `⏪️time-travel`, `📎️local-folders`) are unchanged except that their own test paths and imports stay the same.

## Appendix A: candidate `pub(super)` names per child

Format: child, count, names (private at definition, used from another chunk; generic names included). Use the blanket rule in section 4.1 if this list is incomplete.

C01 (122): AGENT_CHAT_VISIBLE_ENTRIES AUTO_CHECKIN_EDIT_THRESHOLD AUTO_CHECKIN_IDLE_MS CHROME_ICON_TINY COMPOSE_WINDOW_TEMPLATE_MIME FRAMEWORK_CATEGORY_COMMAND_ID FRAMEWORK_CATEGORY_DISPLAY_ID FRAMEWORK_CATEGORY_TOOL_ID FRAMEWORK_CHAT_FEED_ID FRAMEWORK_CHAT_PANEL_ID FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX FRAMEWORK_DISPLAY_LAYOUT_TAB_ID FRAMEWORK_DISPLAY_WINDOWS_TAB_ID FRAMEWORK_MARKETPLACE_TAB_ID FRAMEWORK_MOBILE_APP_TAB_ID FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID FRAMEWORK_SETTINGS_GENERAL_TAB_ID FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID FRAMEWORK_SETTINGS_PANEL_ID FRAMEWORK_SETTINGS_THEME_TAB_ID FRAMEWORK_SYNC_PANEL_TAB_ID FRAMEWORK_TASK_MANAGER_PANEL_ID FRAMEWORK_TOOL_PANEL_TAB_PREFIX MARKETPLACE_LOCAL_SECTION_ID MARKETPLACE_WINDOW_NODE_BUDGET OpenArtifactRelayTarget PANEL_CHROME_GRIP_SIZE PANEL_CHROME_ICON_TINY RetainedContentFocus SHELL_CHROME_IO_FIELD_BYTES SHELL_DIRECTORY_NETWORK_BYTES_PER_MINUTE SHELL_DISPATCH_ORIGIN_GESTURE SHELL_DISPATCH_ORIGIN_USER SHELL_DRIVER_AXES SHELL_PALETTE_DESKTOP_MAX_WIDTH_PX SHELL_PALETTE_LIST_MAX_UI_SPACING SHELL_PALETTE_MEDIUM_UI_SPACING SHELL_PALETTE_PLAN_ENTRY_CAPACITY SHELL_PALETTE_TINY_UI_SPACING SYNC_RESEED_RETRY_MS S_SPACE_INDEX_DOCUMENT_ID S_SPACE_INDEX_DOCUMENT_SCHEMA ShellChromeBuildState ShellChromePresentState ShellDocumentOpenAnswer ShellDocumentOpenTarget ShellHubDocumentSeed ShellPaletteKind ShellPalettePlan ShellPalettePlanEntry ShellPreparedDocumentOpen ShellSyncOwner ShellSyncReseed TAB_INSERT_PREVIEW_WIDTH_PX action_window_instance_id auto_checkin_should_fire begin_next_generation bind bind_wgpu_document_socket_surface builtin_ui_driver_document can_check_in checkpoint_landed close_id close_step context_menu_action_kind_str context_menu_item_at_path context_menu_move_active context_menu_open_submenu_path context_menu_path_for_item_id context_menu_path_for_ordinal context_menu_paths_equal context_menu_submenu_open custom_ui_driver_id decode_window_template_drag default_persistence_bindings dialog_id directory_command_from_action document_open_resolve_step_code document_socket_surface_from_descriptor dsl_value_as_json find_dialect_app fold_history_patch from_close_id from_input_id input_id iter list_id mint_shell_session_id open_artifact_relay_target optional_dsl_value_as_json owns_control_id painted_label plugin_modules_root_of pop_front presence_peer_rows_for_surface register_hit resolve_identity_env resolve_ui_driver_chrome resolve_ui_driver_document route_document_backbone_effects row_index scope_action_to_window scope_context_menu_items shell_actor shell_context_menu_item_from_spec shell_directory_request_base_url shell_directory_transport shell_hub_bootstrap_origin shell_hub_device_instance_id shell_presence_label shell_sync_channel_owner shell_sync_owner_matches should_checkpoint_before_detach space_index_dialect try_push ui_driver_axis ui_driver_document_chrome uncommitted_edit_count valid_ui_driver_axis_value wgpu_document_socket_surface window_template_drag_data

C02 (35): DirectoryHomeProjection PendingShellIo ShellDetached ShellDirectoryHomePublicationOutcome ShellDirectoryHomePublicationResult ShellDirectoryPageResult ShellDirectoryRunner ShellDocumentRetirementRegistry ShellInferenceRunner ShellIoCompletion ShellPoolFuture ShellRefreshSections ShellRenderedRefresh _ active_session admit_reserved begin_epoch close close_one drain empty intend is_instance publish retry start submit_action take take_destroy_authority take_terminal terminal_directory_home_ack terminal_is_empty try_admit try_reserve_admission wake

C03 (24): BROWSER_IDENTITY_RETRY_MS DIRECTORY_COMMAND_DEADLINE_MS HUB_SIGN_IN_DEADLINE_MS PendingExtensionInvocation PendingFileOpen PresentedInputGeometry SHELL_DEFERRED_CHAIN_ROUNDS SHELL_DOCUMENT_RETIREMENT_BYTES SHELL_DOCUMENT_RETIREMENT_ITEMS SHELL_DOCUMENT_RETIREMENT_STEPS SHELL_SETTLE_HEARTBEAT_STEPS SHELL_SETTLE_ROUNDS SHELL_WINDOW_PAINT_OPPORTUNITIES ShellHubSignInAnswer ShellHubSpacesAnswer ShellHubTask WINDOW_TOPOLOGY_PUBLICATION_ATTEMPTS WindowTopologyPublication answer cancel document_opportunity_remains pending retain spawn

C04 (27): DEFAULT_PANEL_WIDTH_PX DOCK_SKELETON_VERSION WindowMeasuresProjection contains control flowed_anchor_content_rect flowed_anchor_tab_divider_y flowed_anchor_tab_row_y load_dock_skeleton_from_store load_dock_ui_from_store load_named_layouts_from_store measure measure_binding measure_bindings measure_container measure_label measure_row_target measure_slider paint_window_measures_step publish_surface_records publish_window_measures refresh_scope_label save_dock_skeleton_to_store save_dock_ui_to_store save_named_layouts_to_store visit window_pane_owner

C05 (26): DEFAULT_APP_NONE_VALUE PanelProjection agent_chat_entry_node claim_app_catalogue_fetch display_panel_body display_unavailable_section drag_data humanize_control_id named_layout_rows node panel_ui_scroll_records place reference_selection_ids reserve settings_section settings_select_row_with settings_text_row settings_tree_select_item settings_tree_select_item_with staged_arg_row staged_arg_value staged_command_arg_row tool_run_panel_reveals ui_node_is_windowed window_measure_tree_rows world_projection_template_rows

C06 (47): action_session allocate_window_instance_id app_owns_command apply_host_panel_selection apply_window_title_host_command begin_pending_dock_drag begin_window_template_drag_from_hit chrome_dialog_request claim_chrome_pointer command_session commit_window_topology_publication context_window_instance_id deactivate_active_window debug_log default_host_panel_state document_controller_id flatten_panel_tab_leaves host_config host_controller_id host_effect_earned_scope instant_now_ms live_view_state live_window_kind_id note_retained_body_painted observe_interaction_output owe_refresh owe_window_topology_refresh persist_dock_layout poll_shell_io record_document_paint_fault reserve_window_topology_action same_session session_action_invocation session_example_rows set_interaction_selection_projection set_search_open settle_surface_faults shell_command_controller_id submit_shell_io_future sync_dock sync_session_chrome take_window_topology_action toggle_immediate_tree_item window_document_session window_icon_override window_title_override window_topology_publication_has_credit

C07 (21): active_terminologies clear_driver_draft close_document_one column_anchors command_category_tabs dock_store_app_id drain_retained_document_arenas framework_settings_tabs live_window_ids publish_app_catalogue queue_host_effects resident_census restore_window_document retain_document_for_close retire_documents_outside retire_one_surface_document seed_new_window_projection set_driver_field take_window_document tool_panel_tabs world3d_close_cancel_settled

C08 (9): current_shell_actor default_bindings_for_current_session footer_presence_rows hub_connection_state hub_footer_label install_document_seed observe_invocation_history shell_turn sync_pill

C09 (10): apply_shell_uri bootstrap_identity cancel_inference_port directory_command_ctx directory_ctx directory_now_ms dispatch_session_command handle_hub_workspace_action handle_replay_shell_command open_service_port

C10 (13): HOST_WINDOW_BLUR_PENDING WORLD3D_CLOSE_CANCEL_SIGHTINGS clear_content_focus content_has_focus host_window_blur_pending note_content_focus_commands publish_retained_hit_registry register_retained_body_hits register_retained_panel_scroll_region retained_hit_window retained_keyboard_focus tree_hit_item_id ui_event_from_key_action

C11 (30): claim_text_editor_alt_completions claim_text_editor_menu_action close_command_palette close_dock_window_and_arm close_open_selects command_palette_plan dismiss_overlays dispatch_tree_selection dock_stack_path_for dock_window_order filtered_find_items filtered_search_items finish_canvas2d_catalogue_drop focus_active_window keyboard_focus_ring_rect keyboard_ring match_app_keybinding open_active_window_in_new_window palette_query push_uri queue_tree_selection render_tree_drag_overlay_node resolve_context_menu_surface retained_window_body_rect set_palette_query sync_canvas2d_catalogue_drag sync_context_menu_hover sync_world3d_catalogue_drop_preview sync_world3d_catalogue_drop_preview_from_retained_ui update_tree_hover

C12 (59): ANCHOR_COLUMN_ORDER ChromeGroupItem HUB_CHECK_IN_DEADLINE_MS HUB_CHECK_IN_POLL_MS HUB_CHECK_IN_QUIESCENCE_MS LOGO_UNTINTED RetainedChromeGroupStep SHELL_NAVBAR_LOGO_UI_SPACING ShellBoardPresencePaint ShellFooterChromeLayout ShellKeyboardStop ShellNavbarChromeLayout WINDOW_PANE_CHIP_PARENT WINDOW_UTILITY_RAIL_PARENT WORLD_PROJECTION_PANE_PARENT anchor_panel_rect apply_initial_world_projection_template board_presence_chip_width chrome_icon chrome_text chrome_text_complete_step chrome_text_wrapped_complete_step dock_tab_node_carries dock_tab_node_of dock_tab_select_window_id empty_dock_notice_lines engine_host_id_for_window floating_panel_max_width floating_panel_width format_keybinding_shortcut hub_check_in_local_status hub_check_in_status_text label_key paint_window_pane_chips_step paint_window_projection_step paint_window_utilities_step panel_resize_anchor render_footer_status_step render_retained_chrome_group_item_step render_retained_panel_chrome_item_step render_utility_nodes retained_chrome_group_item_width retained_panel_chrome_item_width select_world_projection_template semio_cursor_from_name shell_chrome_clip_horizontal shell_hub_connection_text shell_hub_document_key shell_hub_remote shell_offered_examples shell_sync_link_after_status shell_sync_link_terminal spec surface_fits_overlay toggle_window_pane_chip width world3d_state_for_window world3d_state_for_window_mut world_projection_template_selection_id

C13 (21): FUZZY_SEARCH_LIMIT FUZZY_SEARCH_THRESHOLD active_utility_bar_window_kind arm_control_command arm_window_activation_note chrome_accessibility_checked chrome_accessibility_pressed chrome_accessibility_selected command_address_stable_key command_host_platform derive_utility_nodes derive_window_utility_nodes execute_staged_action note_shell_command_action paint_window_pane_body_step parse_dock_tab_action_id publish_shell_panel_document refresh_window_action_panes republish_shell_panel_document republish_window_action_panes shell_owned_panel_leaves

C14 (93): AgentApprovalPaintOp ChromeDialogFieldKind ChromeDialogPaintOp ChromeDialogPress ChromeDialogRequest ChromeDialogSemantics ChromeDialogStop ChromeElementRectEntry ChromeTooltipHover ChromeTourState DOCUMENT_OPEN_CANCEL_CONTROL_ID PLUGIN_INSTALL_CANCEL_CONTROL_ID RETAINED_PANEL_SCROLL_SUFFIX RefusedGuestFault SYNC_COMMAND_TRANSFORMED_NOTICE_CODE TRANSIENT_NOTICE_CLOSE_CONTROL_ID TRANSIENT_NOTICE_STATUS_ID accessibility_key accessibility_target action advance_introduction back_introduction choose chrome_now_ms chrome_tooltip_ready chrome_tour_chip_hotkey_box chrome_tour_chip_icon_x chrome_tour_chip_label_box chrome_tour_layout close_chrome_overlay_glass_content close_topmost_dialog command_rejection_notice compute_click_edge confirm control_id cycle dialog_open dismiss_introduction dismiss_transient_notice document_opening_banner_text document_opening_rect document_opening_tone effective expire_transient_notice field_text field_value flip focus_stop focused from_definition introduction_veil_bands introduction_was_seen mark_introduction_seen move_focus note_chrome_dialog_control nudge nudge_by open_chrome_overlay_glass_content open_dialog options panel_tab_introduction_element_id plugin_install_action_rect plugin_install_banner_text plugin_install_rect plugin_install_tone point_slider program_fault_text push_chrome_tour_chip_border reference_ids refused_guest_call register_element_rect remove_chip resolve_element_rect resolved set_draft set_slider should_auto_start_introduction show_transient_notice skip_introduction slide slider_value stage start_introduction stops tooltip_shows tour_is_open transient_notice_close_rect transient_notice_expired transient_notice_rect transient_notice_tone use_selection utility_collection_path_to_id utility_subtree_has_active_path

C15 (5): render_tutorial_gesture_overlay_node tutorial_bar_height tutorial_note_real_dispatch tutorial_scrub_progress tutorial_scrub_target_ms

C16 (20): ShellChromeChildCursor anchor_content_rect anchor_panel_tab_rows anchor_rect body_rect finish_dock_tab_drag footer_chrome_layout footer_tab_row_rect fullscreen_item_width name navbar_center_tab_row_item navbar_chrome_layout navbar_control_width navbar_leading_tab_row_rect navbar_trailing_tab_row_item plan_dock_windows render_main_window_step render_panel_step reveal_panel_tab_row shell_uri

C17 (8): render_agent_chat_header_step render_document_opening_step render_footer_step render_navbar_step render_overlay_step render_plugin_install_step render_transient_notice_step render_tutorial_bar_step

C18 (15): chrome_tour_active_step chrome_tour_advance_current_step chrome_tour_frame_begin chrome_tour_note_action_performed chrome_tour_note_utility_performed chrome_tour_reveal_panel_tab handle_chrome_dialog_accessibility_event handle_chrome_dialog_key render_agent_approvals_step render_chrome_dialog_step render_chrome_tooltip render_chrome_tour_step render_context_menu_level render_hub_workspace_step render_space_administration_step

C19 (40): ChromePrefsState OS_SHELL_CONFIG_MAX_BYTES OS_SHELL_CONFIG_STORAGE_KEY UI_COMPUTE_WORKER_COUNT_STORAGE_KEY UI_INTRODUCTION_BACK_CONTROL_ID UI_INTRODUCTION_DIALOG_ELEMENT_ID UI_INTRODUCTION_NEXT_CONTROL_ID UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX UI_INTRODUCTION_SKIP_CONTROL_ID UI_INTRODUCTION_VEIL_CONTROL_ID UiPrefsSnapshot appearance_id capture commit_ui_preferences_write custom_themes_from default default_compute_worker_count empty_os_shell_config env_lock flush get new persist_ui_preferences persist_ui_prefs_if_changed prefs_get prefs_set publish_agent_bridge_offer_scope raw_prefs_get raw_prefs_set read_ui_preferences replay_ui_preferences resolve_appearance_id resolve_locale_id scene_chrome_labels set shell_chrome_string stored_field_get stored_field_set take_pending_agent_bridge_config with_chrome_prefs

C20 (27): IMPORT_TRANSFER_CANCEL_ACTION SHELL_DEFAULT_MERGE_POLICY SHELL_MERGE_POLICY_OPTIONS SHELL_THEME_IMPORT_ACCEPT apply_theme_editor_commit begin build_settings_theme_editor_tree chrome_accessibility_nodes clear_theme_draft custom_theme_document_from custom_theme_id_for_label document_theme_for_ids download_media_export drop from_value merge_committed_args note_chrome_control_description note_chrome_control_disabled note_chrome_control_name note_chrome_control_semantics note_chrome_group_item note_dock_control_name parse request_media_frames scalar to_value with_chrome_control_names

Notes on appendix A: the `P` (parent) use list is the ShellState struct field types, all covered by names above (ShellChromeBuildState, ShellChromePresentState, ShellSyncReseed, ShellDocumentRetirementRegistry, DirectoryHomeProjection, PendingShellIo, ShellInferenceRunner, ShellPoolFuture, PendingExtensionInvocation, PendingFileOpen, WindowTopologyPublication, PresentedInputGeometry, ShellSettlePump, ShellDirectoryCommandQueueV1, ShellHubTask, ShellHubSpacesAnswer, ShellHubSignInAnswer, ShellSpaceAdministrationOperationV1, ShellPluginFault, ShellSurfaceFault, ShellDocumentOpening, ShellPluginInstall, ShellSyncChannel, ShellFindItems, ContextMenuState, RightClickState, OverlayState, ActiveSession, ShellConflictRow, ShellHubRemoteV1, ShellHubCheckInV1, TutorialRuntime, TutorialPendingDocOp, ImportTransfer, ThemeDocument, AgentBridgeOfferScopeOwnerV1, ChromePrefsState, UiPrefsSnapshot). Those are `pub` or `pub(crate)` mostly, so the struct already sees them after the glob; the private ones in this list need `pub(super)`.

Report produced by read-only audit. No file in the repository was modified.
