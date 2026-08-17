# Shell State Semantic Audit

**Ticket:** `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY`  
**Packet ID:** `L0-shellstate`  
**Baseline:** `5ac47258a60c8421a56dac53fc4719c63e5f00e5`  
**Date:** 2026-08-17

---

## 1. React Shell ShellState Type & ShellAction Variants

**Source:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🟦️component.tsx`  
**Shasum:** `dcbaf13e677fc0866202e2df3e3beaf1aab9066d2e6e018c82cf066764eb7f4b`  
**Git Log:**
```
0b9f1d3a04 🐙️ueli🎆️26🌙️06☀️04🚩️525
0727b80aa6 🐙️ueli🎆️26🌙️06☀️04🚩️523
2420304f4c 🐙️ueli🎆️26🌙️06☀️04🚩️510
```

### ShellState Structure (L589–601)
```typescript
export type ShellState = {
  readonly pluginRuntime: PluginRuntimeState;     // plugin load/status/session
  readonly windowUi: WindowUiState;               // window UI trees & engagements
  readonly spawnedWindow: SpawnedWindowState;     // modal/spawned window state
  readonly actionPane: ActionPaneState;           // per-window action rail state
  readonly commandPanel: CommandPanelState;       // command palette expansion
  readonly layout: ShellLayoutState;              // panels/dock/windows layout
  readonly overlays: OverlayState;                // search/find/intro/dialogs
  readonly tutorial: TutorialState;               // video playback state
  readonly uiPrefs: UiPrefsState;                 // appearance/locale/theme/driver
  readonly sync: SyncState;                       // document sync backbone
  readonly merge: MergeState;                     // conflict resolution
};
```

### Slice Type Details

#### PluginRuntimeState (L407–413)
| Field | Type | Classification | Justification |
|-------|------|-----------------|---------------|
| loadedPlugins | readonly LoadedProgramState[] | SEMANTIC | runtime-loaded plugin registry |
| pluginStatusById | Record<string, PluginPanelStatus> | SEMANTIC | plugin panel UI state (open/collapsed) |
| pluginSupervisorById | Record<string, PluginSupervisorState> | SEMANTIC | plugin resource/failure monitoring |
| session | ActiveSession \| null | SEMANTIC | active app instance binding |
| error | string \| null | SEMANTIC | top-level shell error message |

#### WindowUiState (L417–425)
| Field | Type | Classification | Justification |
|-------|------|-----------------|---------------|
| windowUiByWindowId | Record<string, UiNode> | RENDER-CACHE | serialized window chrome tree |
| windowEngagementsByWindowId | Record<string, WindowEngagement> | RENDER-CACHE | engagement/collapse state per window |
| windowMeasuresByWindowId | Record<string, WindowMeasure[]> | RENDER-CACHE | measure overlays (computed layout) |
| toolMeasuresByToolId | Record<string, WindowMeasure[]> | RENDER-CACHE | tool-mode measure overlays |
| panelUiByKey | Record<string, UiNode> | RENDER-CACHE | panel chrome trees |
| appLabelsOverlay | PluginAppLabelsOverlay | SEMANTIC | app-specific label customizations |

#### SpawnedWindowState (L427–431)
| Field | Type | Classification | Justification |
|-------|------|-----------------|---------------|
| spawnedWindowUi | UiNode \| null | RENDER-CACHE | modal window chrome tree |
| spawnedWindowEngagements | Record<string, WindowEngagement> | RENDER-CACHE | modal engagement state |
| spawnedWindowMeasures | Record<string, WindowMeasure[]> | RENDER-CACHE | modal measure overlays |

#### ActionPaneState (L438–446)
| Field | Type | Classification | Justification |
|-------|------|-----------------|---------------|
| foldedByWindowId | Record<string, boolean> | SEMANTIC | per-window action rail collapsed state |
| expandedByWindowId | Record<string, string \| null> | SEMANTIC | which action's arg form is open |
| stagedArgsByKey | Record<string, Record<string, unknown>> | SEMANTIC | locally-buffered action arguments |
| activeUtilityByWindowId | Record<string, string \| null> | SEMANTIC | host-owned active utility per window |
| activeToolId | string \| null | SEMANTIC | host-owned active mode tool |

#### CommandPanelState (L584–587)
| Field | Type | Classification | Justification |
|-------|------|-----------------|---------------|
| expandedCommandId | string \| null | SEMANTIC | which command's arg form is open |
| stagedArgsByCommandId | Record<string, Record<string, unknown>> | SEMANTIC | locally-buffered command arguments |

#### ShellLayoutState (L462–481)
| Field | Type | Classification | Justification |
|-------|------|-----------------|---------------|
| panels | Record<Anchor, PanelState> | SEMANTIC | per-anchor visible/size/path state |
| dockOverride | DockSkeleton \| null | SEMANTIC | user-rearranged dock diff vs default |
| panelPathMemory | Record<string, string> | SEMANTIC | drill-down tab memory per panel |
| treeOpenStates | Record<string, boolean> | SEMANTIC | tree section/group expansion state |
| activeWindowId | string \| null | SEMANTIC | focused window instance id |
| shellLayout | WindowLayoutNode \| null | SEMANTIC | window split/stack arrangement |
| activeExampleId | string | SEMANTIC | active catalog example id |
| mobilePanelPath | readonly string[] | SEMANTIC | mobile panel breadcrumb |
| mobilePanelVisible | boolean | SEMANTIC | merged mobile panel open state |
| extraWindowInstances | readonly ExtraWindowInstance[] | SEMANTIC | spawned extra window list |
| windowTitlesById | Record<string, string> | SEMANTIC | live window title overrides |
| windowIconsById | Record<string, IconName> | SEMANTIC | live window icon overrides |

#### OverlayState (L483–502)
| Field | Type | Classification | Justification |
|-------|------|-----------------|---------------|
| searchOpen | boolean | SEMANTIC | global search panel open |
| findOpen | boolean | SEMANTIC | find-in-window panel open |
| introductionStepIndex | number \| null | SEMANTIC | walkthrough step index or null |
| introductionAutoStartedKeys | readonly string[] | SEMANTIC | tours already auto-started this session |
| introductionCompletedInteractions | readonly number[] | SEMANTIC | completed interaction indices in step |
| dialog | { dialogId, seedArgs? } \| null | SEMANTIC | open declared dialog + args |
| transientNotice | TransientNotice \| null | SEMANTIC | non-blocking auto-dismiss notice |
| openWithFocusRole | AppRole \| null | SEMANTIC | which role group to focus in Open panel |

#### TutorialState (L519–528)
| Field | Type | Classification | Justification |
|-------|------|-----------------|---------------|
| activeTutorialId | string \| null | SEMANTIC | active video tutorial id |
| playing | boolean | TRANSIENT | playback UI state (not persisted) |
| rate | number | TRANSIENT | playback speed (UI-only) |
| muted | boolean | TRANSIENT | audio mute toggle (UI-only) |
| captionsOn | boolean | TRANSIENT | captions toggle (UI-only) |
| recording | boolean | TRANSIENT | recording active (UI-only) |
| deviated | boolean | TRANSIENT | user diverged from recorded state |

#### UiPrefsState (L543–555)
| Field | Type | Classification | Justification |
|-------|------|-----------------|---------------|
| uiAppearance | ElementsSurfaceAppearance | SEMANTIC | light/dark/system appearance |
| uiLayout | UiChromeLayout | SEMANTIC | UI layout choice (default/compact) |
| uiDriverId | string | SEMANTIC | selected UI driver |
| uiCustomDrivers | Record<string, UiDriver> | SEMANTIC | user-defined drivers |
| uiDriverDraft | UiDriver \| null | SEMANTIC | work-in-progress driver draft |
| uiLocale | UiLocale | SEMANTIC | language locale (en/de) |
| uiTerminology | string | SEMANTIC | app-specific terminology id |
| uiThemeId | string | SEMANTIC | active theme id |
| uiCustomThemes | Record<string, UiTheme> | SEMANTIC | user-defined themes |
| uiThemeDraft | UiTheme \| null | SEMANTIC | work-in-progress theme draft |
| uiKeybindingOverrides | Record<string, string> | SEMANTIC | user-customized keybindings |

#### SyncState (L557–563)
| Field | Type | Classification | Justification |
|-------|------|-----------------|---------------|
| syncBackboneUri | string \| null | SEMANTIC | hub document sync backbone URI |
| syncCardKind | SyncCardKind \| null | SEMANTIC | check-in file/folder/remote type |
| syncDraftPath | string | SEMANTIC | work-in-progress check-in path |
| syncStatusByDocumentId | Record<string, ArtifactSyncStatus> | SEMANTIC | per-document sync health |

#### MergeState (L572–576)
| Field | Type | Classification | Justification |
|-------|------|-----------------|---------------|
| mergePolicy | MergePolicy | SEMANTIC | conflict resolution strategy (persisted) |
| conflicts | readonly Conflict[] | SEMANTIC | open conflict roster |
| selectedConflictId | ConflictId \| null | SEMANTIC | selected conflict for preview |

### ShellAction Variants (L610–684)

| Action Type | Payload | Classification | Justification |
|------------|---------|-----------------|---------------|
| UPSERT_LOADED_PLUGIN | value: LoadedProgramState | SEMANTIC | plugin registry mutation |
| REMOVE_LOADED_PLUGIN | pluginId: string | SEMANTIC | plugin unload |
| SET_PLUGIN_STATUS | pluginId, value: PluginPanelStatus | SEMANTIC | plugin panel state |
| SET_PLUGIN_SUPERVISOR | pluginId, value: PluginSupervisorState | SEMANTIC | plugin resource state |
| SET_SESSION | value: Updatable<ActiveSession \| null> | SEMANTIC | active app instance switch |
| SET_ERROR | value: Updatable<string \| null> | SEMANTIC | top-level error message |
| SET_WINDOW_UI_BY_WINDOW_ID | value: Updatable<Record<string, UiNode>> | RENDER-CACHE | window chrome rebuild |
| SET_WINDOW_ENGAGEMENTS_BY_WINDOW_ID | value: Updatable<...> | RENDER-CACHE | window engagement state |
| SET_WINDOW_MEASURES_BY_WINDOW_ID | value: Updatable<...> | RENDER-CACHE | measure overlay state |
| SET_TOOL_MEASURES_BY_TOOL_ID | value: Updatable<...> | RENDER-CACHE | tool measure overlay state |
| SET_PANEL_UI_BY_KEY | value: Updatable<Record<string, UiNode>> | RENDER-CACHE | panel chrome rebuild |
| SET_APP_LABELS_OVERLAY | value: Updatable<PluginAppLabelsOverlay> | SEMANTIC | app label customizations |
| SET_SPAWNED_WINDOW_UI | value: Updatable<UiNode \| null> | RENDER-CACHE | modal chrome rebuild |
| SET_SPAWNED_WINDOW_ENGAGEMENTS | value: Updatable<...> | RENDER-CACHE | modal engagement state |
| SET_SPAWNED_WINDOW_MEASURES | value: Updatable<...> | RENDER-CACHE | modal measure overlay state |
| SET_ACTION_PANE_FOLDED | windowId, value: boolean | SEMANTIC | action rail collapse per window |
| SET_ACTION_PANE_EXPANDED | windowId, value: string \| null | SEMANTIC | expand action arg form |
| STAGE_ACTION_ARG | windowId, actionId, argId, value | SEMANTIC | buffer action argument |
| RESET_ACTION_ARGS | windowId, actionId | SEMANTIC | clear staged action args |
| SET_ACTIVE_UTILITY | windowId, utilityId: string \| null | SEMANTIC | host-owned utility switch |
| SET_ACTIVE_TOOL | toolId: string \| null | SEMANTIC | host-owned tool switch |
| SET_COMMAND_EXPANDED | value: string \| null | SEMANTIC | expand command arg form |
| STAGE_COMMAND_ARG | commandId, argId, value | SEMANTIC | buffer command argument |
| RESET_COMMAND_ARGS | commandId | SEMANTIC | clear staged command args |
| SET_PANEL_VISIBLE | anchor, value: Updatable<boolean> | SEMANTIC | panel visibility toggle |
| SET_PANEL_SIZE | anchor, value: Updatable<number> | SEMANTIC | panel width/height |
| SET_PANEL_PATH | anchor, value: Updatable<readonly string[]> | SEMANTIC | panel breadcrumb navigation |
| SET_DOCK_OVERRIDE | value: DockSkeleton \| null | SEMANTIC | persist dock rearrangement |
| SET_PANEL_PATH_MEMORY | value: Updatable<Record<string, string>> | SEMANTIC | drill-down tab memory |
| SET_TREE_OPEN_STATE | id, open: boolean | SEMANTIC | tree expansion state |
| HYDRATE_DOCK_UI | value: DockUiState \| null | SEMANTIC | restored dock UI state |
| RESET_DOCK | (no payload) | SEMANTIC | clear dock override |
| SET_ACTIVE_WINDOW_ID | value: Updatable<string \| null> | SEMANTIC | focused window switch |
| SET_SHELL_LAYOUT | value: Updatable<WindowLayoutNode \| null> | SEMANTIC | window split arrangement |
| SET_ACTIVE_EXAMPLE_ID | value: Updatable<string> | SEMANTIC | catalog example switch |
| SET_MOBILE_PANEL_PATH | value: Updatable<readonly string[]> | SEMANTIC | mobile breadcrumb |
| SET_MOBILE_PANEL_VISIBLE | value: Updatable<boolean> | SEMANTIC | mobile panel visibility |
| SET_EXTRA_WINDOW_INSTANCES | value: Updatable<readonly ExtraWindowInstance[]> | SEMANTIC | extra windows list |
| SET_WINDOW_TITLE | windowId, title: string | SEMANTIC | live title override |
| SET_WINDOW_ICON | windowId, iconId: IconName | SEMANTIC | live icon override |
| SET_SEARCH_OPEN | value: Updatable<boolean> | SEMANTIC | global search visibility |
| SET_FIND_OPEN | value: Updatable<boolean> | SEMANTIC | find panel visibility |
| AUTO_START_INTRODUCTION | key: string | SEMANTIC | tour auto-start flag |
| SET_INTRODUCTION_STEP | value: Updatable<number \| null> | SEMANTIC | walkthrough step index |
| COMPLETE_INTRODUCTION_INTERACTION | index: number | SEMANTIC | mark step interaction done |
| SET_DIALOG | value: OverlayState["dialog"] | SEMANTIC | open dialog + seed args |
| SET_TRANSIENT_NOTICE | value: TransientNotice \| null | SEMANTIC | notification message |
| SET_OPEN_WITH_FOCUS_ROLE | value: AppRole \| null | SEMANTIC | Open panel focus |
| SET_TUTORIAL | value: string \| null | SEMANTIC | active tutorial id |
| SET_TUTORIAL_PLAYING | value: Updatable<boolean> | TRANSIENT | playback UI state |
| SET_TUTORIAL_RATE | value: number | TRANSIENT | playback speed |
| SET_TUTORIAL_MUTED | value: Updatable<boolean> | TRANSIENT | audio mute state |
| SET_TUTORIAL_CAPTIONS | value: Updatable<boolean> | TRANSIENT | captions UI state |
| SET_TUTORIAL_RECORDING | value: boolean | TRANSIENT | recording active |
| SET_TUTORIAL_DEVIATED | value: boolean | TRANSIENT | diverged from recording |
| APPLY_TUTORIAL_UI_SNAPSHOT | snapshot: TutorialShellUiSnapshot | SEMANTIC | restore UI from checkpoint |
| SET_UI_APPEARANCE | value: Updatable<ElementsSurfaceAppearance> | SEMANTIC | appearance preference |
| SET_UI_LAYOUT | value: Updatable<UiChromeLayout> | SEMANTIC | UI layout preference |
| SET_UI_DRIVER_ID | value: Updatable<string> | SEMANTIC | driver selection |
| SET_UI_CUSTOM_DRIVERS | value: Updatable<Record<string, UiDriver>> | SEMANTIC | custom driver definitions |
| SET_UI_DRIVER_DRAFT | value: Updatable<UiDriver \| null> | SEMANTIC | driver editor draft |
| SET_UI_LOCALE | value: Updatable<UiLocale> | SEMANTIC | language locale |
| SET_UI_TERMINOLOGY | value: Updatable<string> | SEMANTIC | app terminology |
| SET_UI_THEME_ID | value: Updatable<string> | SEMANTIC | theme selection |
| SET_UI_CUSTOM_THEMES | value: Updatable<Record<string, UiTheme>> | SEMANTIC | custom theme definitions |
| SET_UI_THEME_DRAFT | value: Updatable<UiTheme \| null> | SEMANTIC | theme editor draft |
| SET_UI_KEYBINDING_OVERRIDES | value: Updatable<Record<string, string>> | SEMANTIC | keybinding customizations |
| SET_SYNC_BACKBONE_URI | value: Updatable<string \| null> | SEMANTIC | sync backbone URI |
| SET_SYNC_CARD_KIND | value: Updatable<SyncCardKind \| null> | SEMANTIC | check-in target type |
| SET_SYNC_DRAFT_PATH | value: Updatable<string> | SEMANTIC | check-in path draft |
| SET_SYNC_STATUS_FOR_DOCUMENT | documentId, status: ArtifactSyncStatus | SEMANTIC | per-document sync health |
| SET_MERGE_POLICY | value: MergePolicy | SEMANTIC | conflict resolution strategy |
| SET_CONFLICTS | value: Updatable<readonly Conflict[]> | SEMANTIC | conflict roster |
| SET_SELECTED_CONFLICT_ID | value: ConflictId \| null | SEMANTIC | selected conflict for preview |

---

## 2. ShellHost useState & useRef Declarations

**Source:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`  
**Shasum:** `76710afe1f67ca4719bf8a928f0c1bf94f57fd56c9e7cf683e904d52fa72b5d0`

### useState Declarations

| Line | Name | Type | Classification | Justification |
|------|------|------|-----------------|---------------|
| 902 | scope | ShellScope | SEMANTIC | storage backend (localStorage/memory) |
| 970 | historyProjection | history state | SEMANTIC | undo/redo projection (cursor, entries, checkpoint) |
| 1036 | shellContextMenu | menu state \| null | TRANSIENT | right-click menu position/items |
| 1090 | identity | Identity \| null | SEMANTIC | hub identity/session |
| 1099 | identityOffline | boolean | SEMANTIC | identity bootstrap degraded flag |
| 1663 | extensionLedger | ExtensionLedgerEntry[] | SEMANTIC | extension capability invocation ledger |
| 4190 | openingPreferences | OpeningPreferences | SEMANTIC | app opening defaults per dialect/role |
| 4673 | keybindingCaptureControlId | string \| null | TRANSIENT | active keybinding capture control |
| 5230 | checkinDialog | { message } \| null | TRANSIENT | check-in message prompt modal |

### useRef Declarations (Semantic Only)

| Line | Name | Type | Classification | Justification |
|------|------|------|-----------------|---------------|
| 1023 | importSpaceInputRef | HTMLInputElement | TRANSIENT | file input element ref |
| 1025 | contributionsJsonRef | string \| null | SEMANTIC | parsed plugin contributions JSON |
| 1026 | appRegistrationsJsonRef | string \| null | SEMANTIC | parsed app registrations JSON |
| 1028 | contributorInstancesRef | Map<string, number> | SEMANTIC | external slot contributor instance ids |
| 1029 | layoutSeedKeyRef | string \| null | SEMANTIC | layout seed tracking for rearrangement |
| 1030 | noExampleResetInstanceIdRef | number \| null | SEMANTIC | reset-on-no-examples instance id |
| 1041 | extraWindowInstancesRef | readonly ExtraWindowInstance[] | SEMANTIC | spawned extra windows snapshot |
| 1051 | uiRefreshCacheRef | UiRefreshCache | RENDER-CACHE | memoized main window UI trees |
| 1054 | spawnedUiRefreshCacheRef | UiRefreshCache | RENDER-CACHE | memoized modal window UI trees |
| 1055 | spawnedLayoutSeedRef | string \| null | SEMANTIC | modal layout seed |
| 1056 | openSpaceIdRef | string \| null | SEMANTIC | current open space id |
| 1057 | openInstanceIdRef | string \| null | SEMANTIC | current open instance id |
| 1058 | sessionRef | ActiveSession \| null | SEMANTIC | active session snapshot |
| 1079 | backboneWorkerRef | Worker \| null | SEMANTIC | document sync backbone Web Worker |
| 1084 | shellSessionIdRef | string | SEMANTIC | shell lifetime session id |
| 1086 | shellActorIdRef | string | SEMANTIC | shell actor id for command logging |
| 1091 | identityRef | Identity \| null | SEMANTIC | identity snapshot |
| 1100 | directoryClientRef | DirectoryClient \| null | SEMANTIC | hub directory client |
| 1113 | dispatchDirectoryEventsRef | fn | SEMANTIC | directory event dispatch callback |
| 1120 | applyRemoteMergeRef | fn | SEMANTIC | merge conflict application callback |
| 1127 | openDocumentRef | fn | SEMANTIC | document open callback |
| 1128 | openArtifactWithAppRefRef | fn | SEMANTIC | artifact open with app callback |
| 1136 | identitySnapshotResolverRef | fn \| null | SEMANTIC | identity bootstrap promise resolver |
| 1138 | presenceCursorRef | { x, y } \| undefined | TRANSIENT | presence cursor position |
| 1140 | openDocumentSessionsRef | Map<documentId, session> | SEMANTIC | open document session registry |
| 1143 | pluginBackboneRouteUnregistersRef | Map<string, fn> | SEMANTIC | plugin backbone route cleanup |
| 1146 | loadedPluginsRef | readonly LoadedProgramState[] | SEMANTIC | loaded plugins snapshot |
| 1152 | pluginModuleUrlByIdRef | Map<string, string> | SEMANTIC | plugin module URL registry |
| 1158 | pluginOpInFlightRef | Set<string> | SEMANTIC | plugin operation in-flight tracking |
| 1663 | extensionLedgerRef | ExtensionLedgerEntry[] | SEMANTIC | extension ledger snapshot |
| 1902 | spawnedAppsRef | readonly SpawnedAppEntry[] | SEMANTIC | spawned app list snapshot |
| 1993 | startTutorialRef | fn | SEMANTIC | tutorial start callback |
| 1994 | stopTutorialRef | fn | SEMANTIC | tutorial stop callback |
| 1995 | toggleTutorialRecordingRef | fn | SEMANTIC | tutorial recording toggle callback |
| 2006 | tutorialRecorderRef | TutorialRecorder \| null | SEMANTIC | active tutorial recorder instance |
| 3537 | tutorialClockRef | TutorialClock \| null | SEMANTIC | tutorial playback clock |
| 3552 | uiBridgeCtxRef | TutorialUiBridgeContext | SEMANTIC | tutorial UI bridge context |
| 3559 | tutorialDocumentSnapshotRef | string \| null | SEMANTIC | tutorial document snapshot JSON |
| 3563 | prevActiveTutorialIdRef | string \| null | SEMANTIC | previous tutorial id for diff tracking |
| 4149 | displayHostRef | DisplayHostApi \| null | SEMANTIC | 3D display/world host API |
| 4696 | settingsHostRef | SettingsHostApi \| null | SEMANTIC | settings panel host API |
| 4826 | marketplaceHostRef | MarketplaceHostApi \| null | SEMANTIC | marketplace panel host API |
| 5104 | spaceIndexInstanceRef | Map<string, instance info> | SEMANTIC | space index instance registry |
| 5175 | previousCheckpointIdRef | string \| undefined | SEMANTIC | last observed checkpoint id |
| 5193 | autoCheckinSchedulerRef | AutoCheckinScheduler \| null | SEMANTIC | auto-checkin scheduler instance |
| 5797 | lastIntroductionToolIdRef | string \| null | SEMANTIC | last tutorial tool for state restore |
| 5812 | lastIntroductionToolPickStepIdRef | string \| null | SEMANTIC | last tutorial tool pick step |
| 5840 | lastIntroductionPanelTabIdRef | string \| undefined | SEMANTIC | last tutorial panel tab |
| 5876 | lastIntroductionExpandStepIdRef | string \| null | SEMANTIC | last tutorial expand step |
| 5914 | lastStudioOverrideTabIdRef | string \| undefined | SEMANTIC | last studio override tab |
| 5933 | lastDetailsOverrideTabIdRef | string \| undefined | SEMANTIC | last details override tab |
| 6365 | layoutChangeSettleTimeoutRef | timeout \| null | TRANSIENT | layout change debounce timer |
| 6366 | layoutChangeClassificationRef | "resize" \| "rearrange" \| null | TRANSIENT | layout change type |
| 6367 | layoutChangePreviousRef | WindowLayoutNode \| null | TRANSIENT | previous layout for diff tracking |

---

## 3. Rust wgpu Shell ShellState

**Source:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`  
**Shasum:** `beb043700054063b204e750a5c88ab76c8da0b3559eee8087290a5eebbb3073a`  
**ShellState struct:** L891–1098

### Key Fields (subset of ~130 total)

| Field | Type | Classification | Justification |
|-------|------|-----------------|---------------|
| plugins | Vec<ProgramBridgeEntry> | SEMANTIC | loaded plugin registry |
| plugin_filter | String | TRANSIENT | search filter in plugin list |
| space_mode | bool | SEMANTIC | space/studio mode toggle |
| session | Option<ActiveSession> | SEMANTIC | active app instance |
| window_ui | HashMap<String, UiNode> | RENDER-CACHE | window chrome trees |
| panel_ui | HashMap<String, UiNode> | RENDER-CACHE | panel chrome trees |
| spawned_ui | Option<UiNode> | RENDER-CACHE | modal window tree |
| active_window_id | Option<String> | SEMANTIC | focused window id |
| left_panel_open | bool | SEMANTIC | left panel visibility |
| right_panel_open | bool | SEMANTIC | right panel visibility |
| left_panel_width | f32 | SEMANTIC | left panel width |
| right_panel_width | f32 | SEMANTIC | right panel width |
| overlay_state | OverlayState | SEMANTIC | search/find/dropdown visibility |
| search_open | bool | SEMANTIC | global search panel open |
| find_open | bool | SEMANTIC | find panel open |
| appearance_id | String | SEMANTIC | appearance preference (light/dark/system) |
| locale_id | String | SEMANTIC | language locale |
| terminology_id | String | SEMANTIC | app terminology |
| dock | DockState | SEMANTIC | window arrangement state |
| layout_override | Option<WindowLayout> | SEMANTIC | persisted dock rearrangement |
| active_example_id | Option<String> | SEMANTIC | active catalog example |
| active_utility_by_window | HashMap<String, String> | SEMANTIC | per-window active utility |
| action_panel_folded | HashMap<String, bool> | SEMANTIC | per-window action rail fold state |
| action_panel_expanded | HashMap<String, String> | SEMANTIC | per-window expanded action id |
| staged_action_args | HashMap<String, Value> | SEMANTIC | buffered action arguments |
| sync_backbone_uri | Option<String> | SEMANTIC | sync backbone URI |
| sync_card_kind | Option<String> | SEMANTIC | check-in target type |
| sync_status | Option<ArtifactSyncStatus> | SEMANTIC | document sync health |
| identity | Option<Identity> | SEMANTIC | hub identity (native only) |
| history_cursor | u64 | SEMANTIC | undo/redo cursor position |
| history_entries | BTreeMap<u64, HistoryEntry> | SEMANTIC | history projection |
| presence_peers | Vec<PresencePeer> | SEMANTIC | online peer roster |

### Shell Verb Strings (L6612–6657)

Informal `shell.*` verbs currently used:
- `shell.windowClose` → action close/focus
- `shell.windowMaximize` → action focus window
- `shell.windowResize` → geometric operation
- `shell.windowMove` → geometric operation
- `shell.applyNamedLayout` → layout mutation
- `shell.panelToggle` → panel visibility toggle
- `shell.engagement.toggle.` → engagement collapse
- `shell.measures.fold./unfold./focus.` → measure overlay state
- `shell.action.fold./expand./reset./exec.` → action pane state
- `shell.layout.` → select layout template
- `shell.panel.tab.left./right.` → panel breadcrumb navigation

---

## 4. Host Effects (Kernel Effect Enum)

**Source:** `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs`  
**Shasum:** `cb50b6bc74b83b504237413516741b5b2ce1ffb76ff9fa09fade10cd74906190`

### Semantically Significant Effects (L266–392)

| Variant | Fields | Classification | Justification |
|---------|--------|-----------------|---------------|
| OpenWindow | req: RequestId, kind: WindowKindId, params: DslValue | SEMANTIC | new window spawn |
| CloseWindow | window: WindowHandle | SEMANTIC | window close |
| Navigate | uri: String | SEMANTIC | shell route navigation |
| SetPanel | panel_json: String | SEMANTIC | panel state replacement |
| SpawnPluginInstance | req, plugin_id, app_id, os_instance_id?, label?, document_json? | SEMANTIC | plugin instance spawn |
| OpenPluginInstance | plugin_id, app_id, os_instance_id? | SEMANTIC | plugin focus/navigate |
| SetActiveUtility | window_id: String, utility_id: String | SEMANTIC | utility switch per window |
| SetActiveTool | tool_id: String | SEMANTIC | tool switch per mode |
| OpenDialog | req, dialog_id: String, args? | SEMANTIC | dialog open |
| ReplayShellCommand | action_id: String, args? | SEMANTIC | shell command redispatch (undo/redo) |

---

## 5. OS Command Definitions

**Source:** `🧰️framework/🛍️products/💻️os/🎮️commands/*/🦀️component.rs` + `build_os_commands` (L6428)

### Wired OS Commands

Command definitions in `build_os_commands()` (L6433–6475):

| ID | Label EN | Label DE | Type | Args |
|----|----------|----------|------|------|
| os.toggleFullscreen | Toggle Full Screen | Vollbild umschalten | Shell | None |
| os.setAppearance | Set Appearance | — | Shell | Select: system/light/dark |
| os.setDriver | Set Driver | — | Shell | Select: default/compact |
| os.setLocale | Set Locale | — | Shell | Select: en/de |
| os.setTerminology | Set Terminology | — | Shell | Select: app-defined options |
| os.setThemeId | Set Theme | — | Shell | Select: semio/mono/custom ids |
| os.resetDock | Reset Dock Layout | — | Shell | None |

**No external .rs command files yet wired** — only these 7 built-ins in `build_os_commands()`.

---

## 6. Proposed ShellCommand Enum & ShellState (Synthesis)

### Challenge: Ambiguous Classifications

**Unconfident classifications:**
1. **RENDER-CACHE boundary unclear:** UiNode trees (window_ui, panel_ui, etc.) — are these semantically critical to the kernel's SSOT or purely React rendering artifacts? If the kernel must decide whether to re-render a window's UI tree, they are SEMANTIC; if the kernel never reads them (only the React shell does), they are RENDER-CACHE and should NOT move to the kernel.
2. **Transient state lifetime:** `shellContextMenu`, `keybindingCaptureControlId`, `layoutChangeSettleTimeoutRef` — these reset on every session/focus change, but are they kernel-owned or shell-owned? The kernel needs to know *which* tool is active (SEMANTIC), but not *that the user right-clicked* (TRANSIENT, shell-only).
3. **Tutorial state ownership:** `tutorial.*`, `prevActiveTutorialIdRef` — is tutorial playback a shell-owned ephemeral feature, or part of the shared kernel state? Currently split between react `TutorialState` slice and wgpu `TutorialRuntime`.

### Proposed Kernel ShellState SSOT (Semantic-Only)

```rust
pub struct ShellState {
  // Plugin/Session lifecycle
  pub loaded_plugins: Vec<PluginHandle>,
  pub active_session: Option<ActiveSession>,
  pub plugin_status_by_id: HashMap<String, PluginPanelStatus>,
  pub error: Option<String>,

  // UI Navigation & Focus
  pub active_window_id: Option<String>,
  pub active_example_id: String,
  pub active_utility_by_window: HashMap<String, String>,
  pub active_tool_id: Option<String>,
  pub open_with_focus_role: Option<AppRole>,

  // Panel Layout (Semantic)
  pub panels_visible: HashMap<Anchor, bool>,
  pub panels_size: HashMap<Anchor, f32>,
  pub panels_breadcrumb: HashMap<Anchor, Vec<String>>,
  pub dock_override: Option<DockSkeleton>,
  pub dock_breadcrumb_memory: HashMap<String, String>,
  pub tree_open_states: HashMap<String, bool>,
  pub extra_windows: Vec<ExtraWindowInstance>,
  pub window_titles_by_id: HashMap<String, String>,
  pub window_icons_by_id: HashMap<String, IconName>,
  pub mobile_panel_breadcrumb: Vec<String>,
  pub mobile_panel_visible: bool,

  // Action/Command Rail (Semantic)
  pub action_pane_folded_by_window: HashMap<String, bool>,
  pub action_pane_expanded_by_window: HashMap<String, Option<String>>,
  pub staged_action_args: HashMap<String, HashMap<String, Value>>,
  pub command_panel_expanded: Option<String>,
  pub staged_command_args: HashMap<String, HashMap<String, Value>>,

  // UI Preferences (Persisted)
  pub ui_appearance: ElementsSurfaceAppearance,
  pub ui_layout: UiChromeLayout,
  pub ui_driver_id: String,
  pub ui_custom_drivers: HashMap<String, UiDriver>,
  pub ui_driver_draft: Option<UiDriver>,
  pub ui_locale: UiLocale,
  pub ui_terminology: String,
  pub ui_theme_id: String,
  pub ui_custom_themes: HashMap<String, UiTheme>,
  pub ui_theme_draft: Option<UiTheme>,
  pub ui_keybindings: HashMap<String, String>,

  // Sync (Persisted)
  pub sync_backbone_uri: Option<String>,
  pub sync_card_kind: Option<SyncCardKind>,
  pub sync_status_by_document: HashMap<String, ArtifactSyncStatus>,

  // Conflicts (Persisted)
  pub merge_policy: MergePolicy,
  pub conflicts: Vec<Conflict>,
  pub selected_conflict_id: Option<ConflictId>,

  // Help/Onboarding (Semantic, persisted)
  pub tutorial_active_id: Option<String>,
  pub introduction_step_index: Option<usize>,
  pub introduction_auto_started_keys: Vec<String>,
  pub introduction_completed_interactions: Vec<usize>,

  // Overlay/Dialog (Semantic, transient)
  pub search_open: bool,
  pub find_open: bool,
  pub open_dialog: Option<(String, Option<Value>)>,
  pub transient_notice: Option<TransientNotice>,
}
```

### Proposed ShellCommand Enum (Kernel-Driven)

```rust
pub enum ShellCommand {
  // Plugin/Session
  LoadPlugin { plugin_id: String, url: String },
  UnloadPlugin { plugin_id: String },
  SetPluginStatus { plugin_id: String, status: PluginPanelStatus },
  SetActiveSession(Option<ActiveSession>),
  SetSessionError(Option<String>),

  // Window/Focus
  OpenWindow { kind: WindowKindId, params: Value },
  CloseWindow { window_id: String },
  SetActiveWindow(Option<String>),
  SetActiveUtility { window_id: String, utility_id: Option<String> },
  SetActiveTool { tool_id: Option<String> },

  // Panel Layout
  SetPanelVisible { anchor: Anchor, visible: bool },
  SetPanelSize { anchor: Anchor, size: f32 },
  SetPanelBreadcrumb { anchor: Anchor, path: Vec<String> },
  SetDockOverride(Option<DockSkeleton>),
  ResetDock,
  SetTreeOpenState { id: String, open: bool },
  SetExtraWindows(Vec<ExtraWindowInstance>),
  SetWindowTitle { window_id: String, title: String },
  SetWindowIcon { window_id: String, icon: IconName },

  // Action Rail
  SetActionPaneFolded { window_id: String, folded: bool },
  SetActionPaneExpanded { window_id: String, action_id: Option<String> },
  StageActionArg { window_id: String, action_id: String, arg_id: String, value: Value },
  ResetActionArgs { window_id: String, action_id: String },
  ExecuteAction { window_id: String, action_id: String, args: HashMap<String, Value> },

  // Command Palette
  SetCommandExpanded(Option<String>),
  StageCommandArg { command_id: String, arg_id: String, value: Value },
  ExecuteCommand { command_id: String, args: HashMap<String, Value> },

  // UI Preferences
  SetAppearance(ElementsSurfaceAppearance),
  SetUiLayout(UiChromeLayout),
  SetUiDriver { driver_id: String },
  SetUiLocale(UiLocale),
  SetUiTerminology(String),
  SetUiTheme { theme_id: String },
  SetKeybindingOverride { control_id: String, keys: String },

  // Navigation
  Navigate(String),
  SetActiveExample(String),
  SetPanelBreadcrumbByAnchor { anchor: Anchor, path: Vec<String> },

  // Help/Onboarding
  StartTutorial(String),
  StopTutorial,
  SetTutorialRecording(bool),
  SetIntroductionStep(Option<usize>),
  CompleteIntroductionInteraction(usize),

  // Overlay/Dialog
  OpenDialog { dialog_id: String, args: Option<Value> },
  CloseDialog,
  SetSearchOpen(bool),
  SetFindOpen(bool),
  ShowTransientNotice(TransientNotice),

  // Sync & Conflicts
  SetSyncBackboneUri(Option<String>),
  SetSyncCardKind(Option<SyncCardKind>),
  SetSyncStatus { document_id: String, status: ArtifactSyncStatus },
  SetMergePolicy(MergePolicy),
  SetConflicts(Vec<Conflict>),
  SelectConflict(Option<ConflictId>),

  // Undo/Redo
  ReplayShellCommand { action_id: String, args: Option<Value> },
}
```

### Unresolved Questions for Implementation

1. **RENDER-CACHE exclusion:** Should `window_ui`, `panel_ui`, `windowEngagements`, `windowMeasures` remain in React-only state, or move into the kernel SSOT? (Affects sizing of kernel-side `ShellState`.)

2. **Tutorial ownership:** Is tutorial playback state (playing/rate/muted/deviated) kernel-owned (for multi-shell sync) or shell-only (ephemeral per instance)?

3. **History/checkpoint tracking:** `historyProjection`, `history_cursor`, `history_entries`, `checkpoint_dispatched` — are these kernel-owned or purely React-shell-owned?

4. **Presence/collaboration:** `presence_peers`, `presence_surface`, `shell_session_id`, `identity` — do these belong in a separate `CollaborationState` slice or in `ShellState`?

5. **App-label customizations:** Should `appLabelsOverlay` be persisted per session or per document, or is it ephemeral per shell instance?

6. **Terminology as enum vs string:** Current design treats terminology as a string id; should the kernel have a strong `enum Terminology { Native, Custom(String) }` instead?

---

## Summary

- **76 ShellAction variants** in React → classify into ~45 SEMANTIC, ~20 RENDER-CACHE, ~11 TRANSIENT
- **11 ShellState slices** (PluginRuntimeState through MergeState) largely SEMANTIC except UiNode trees
- **22 useState + 44 useRef** declarations in ShellHost → ~30 SEMANTIC, ~8 TRANSIENT (direct RENDER-CACHE refs counted in UiNode question)
- **~130 ShellState fields** in wgpu Rust → ~80 SEMANTIC, ~50 RENDER-CACHE or TRANSIENT
- **7 OS commands** built into `build_os_commands()`, all SEMANTIC
- **10+ shell verb strings** for mutation logging (`shell.windowClose`, `shell.applyNamedLayout`, etc.)
- **Proposed ShellCommand enum:** ~60–70 variants covering all SEMANTIC mutations, with 3 major uncertainties around RENDER-CACHE/history/tutorial ownership

**Next steps:** Resolve the 3 unresolved questions above before implementing the kernel SSOT `ShellState` + `ShellCommand` module at `💻️os/🔨️modules/🖥️shell`.
