import { Component, createContext, useCallback, useContext, useEffect, useMemo, useReducer, useRef, useState, useSyncExternalStore, type CSSProperties, type ReactElement, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import type { GraphWasmSession } from "@semio-tech/infinite-cavas-react-renderer";
import {
  App,
  applyDockSkeleton,
  Button,
  ButtonGroup,
  ButtonGroupItem,
  ChromeAwareWindowScrollSurface,
  COMPOSE_WINDOW_TEMPLATE_MIME,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  dockSkeletonOf,
  dockSkeletonsEqual,
  Field,
  findPanelTabInDock,
  Footer,
  Fuse,
  Icon,
  IconSelector,
  Input,
  Layout,
  LevelProvider,
  Mode,
  moveTabInDock,
  moveTreeUnitInDock,
  Navbar,
  NavbarExampleSelect,
  PANEL_ANCHORS,
  PanelDockProvider,
  PanelToggleGroup,
  Popover,
  PopoverAnchor,
  PopoverContent,
  Ribbon,
  SemioLogo,
  Slider,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  singleTreeLeaf,
  Toggle,
  ToggleGroup,
  ToolbarDivider,
  ToolbarGroup,
  ToolbarItem,
  ToolbarZone,
  findPanelTabNode,
  findPanelTabPath,
  panelTabChildren,
  reconcileActivePath,
  WindowMeasureTreeGroup,
  WindowMeasureTreeLeaf,
  WindowMeasuresTree,
  bootstrapElementsSurfaceChromeDocument,
  borderElementClass,
  borderNormalBottomClass,
  cn,
  createEvenWindowLayout,
  iconRenderPort,
  getLevelBgClass,
  insertWindowAtDropZone,
  interactiveActiveFillClass,
  shellChromeTitleClassName,
  staticTreePanelDefinition,
  UiChromeLabelPolicyProvider,
  UI_MOBILE_MEDIA_QUERY,
  usePanelChromeHotkeys,
  useElementsSurfaceChrome,
  useMediaQuery,
  useActionHotkey,
  readStoredUiChromeCompact,
  readStoredUiChromeExpertise,
  readStoredUiChromeLayout,
  readStoredUiChromeLocale,
  readStoredUiChromeAppearance,
  readStoredUiChromeTerminology,
  readStoredUiChromeThemeId,
  readStoredUiChromeThemeSnapshot,
  readStoredUiCustomThemes,
  writeStoredUiChromeCompact,
  writeStoredUiChromeExpertise,
  writeStoredUiChromeLayout,
  writeStoredUiChromeLocale,
  writeStoredUiChromeAppearance,
  writeStoredUiChromeTerminology,
  writeStoredUiChromeThemeId,
  writeStoredUiChromeThemeSnapshot,
  writeStoredUiCustomThemes,
  activeUiTheme,
  builtinUiThemes,
  parseUiTheme,
  resolveThemeAppearancePalettes,
  semioTheme,
  serializeUiTheme,
  setActiveUiTheme,
  type ThemeAppearanceName,
  type ThemePaletteGroup,
  type UiTheme,
  windowTemplatePaletteTreeDragController,
  Expertise,
  resolveTranslationLabel,
  setUiLocale,
  uiI18n,
  UI_TERMINOLOGY_NATIVE,
  UIIntroduction,
  UIDialog,
  introductionWindowActionPaneUnfoldSelector,
  introductionWindowToolbarUnfoldSelector,
  readStoredIntroductionSeen,
  writeStoredIntroductionSeen,
  fundedByZukunftBauFooterItem,
  navbarFillItem,
  type ElementsSurfaceAppearance,
  type ElementsSurfaceDevice,
  type EngagementControl,
  type EngagementSpec,
  type FuseResult,
  type ModeWindowDescriptor,
  type NavbarItem,
  type PanelAnchor,
  type PanelDock,
  type PanelTabDockMove,
  type PanelTabNode,
  type PanelToggleItem,
  type PanelTreeUnitDockMove,
  type RibbonDirection,
  type RibbonRow,
  type TreeDataItem,
  type TreeDataSection,
  type TreePanelConfig,
  type UiChromeLayout,
  type UiChromeTerminologyId,
  type UiLocale,
  type UiTranslationKey,
  type WindowLayoutNode,
  type ModeCanvasDropTarget,
  type WindowTemplateDropPayload,
} from "@semio-tech/ui-react";
import { ICONS, type IconName } from "@semio-tech/ui-asset";
import { declarativeTreeDragController, interpretUiNode, InterpretedUiNode, uiTreeNodeToTreePanelConfig } from "./ui-interpreter.tsx";
import {
  DEFAULT_PLUGIN_REGISTRY,
  type InvocationResponse,
  type HostEffect,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
  FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
  FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID,
  FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
  FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
  FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
  FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
  FRAMEWORK_PANEL_TAB_PARAMETERS_ID,
  FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
  DockLayoutStore,
  DockUiStateStore,
  NamedLayoutStore,
  createBrowserStoragePort,
  createNamedLayout,
  type DockSkeleton,
  type DockUiPanelState,
  type DockUiState,
  loadPluginModule as loadCorePluginModule,
  loadPluginWasm as loadCorePluginWasm,
  buildContributionsJson,
  expandPluginRegistry,
  nodeGraphActions,
  panelTabKindId,
  resolveExternalSlots,
  resolveLayoutForMode,
  resolvePlaygroundDefaultAppId,
  resolvePluginRegistryId,
  resolveUiDirtyScope,
  textEditorActions,
  normalizeAppLabelsOverlay,
  deriveUtilityNodes,
  resolveWindowActions,
  partitionWindowMeasures,
  effectiveActionArgs,
  missingRequiredArgs,
  SET_ACTIVE_UTILITY_ACTION_ID,
  START_INTRODUCTION_ACTION_ID,
  type ActionArgControl,
  type ActionArgDef,
  type ActionDefinition,
  type ActionDescriptor,
  type AppDefinition,
  type AppModeDefinition,
  type AppWindowKindDefinition,
  type CommandDefinition,
  type DerivedUtilitySpec,
  type UtilityDefinition,
  type AppPanelTabDefinition,
  type Canvas2dScene,
  type DialogDefinition,
  type IntroductionAnchor,
  type IntroductionDefinition,
  type IntroductionStepDefinition,
  type ComponentKind,
  type ComponentSceneHostProps,
  type GisMapScene,
  type IconRenderScene,
  type NamedLayout,
  type NodeGraphScene,
  type NoteCanvasScene,
  type PluginAppLabelsOverlay,
  type PluginHotSwapEvent,
  type PanelTabKind,
  type PluginRegistryEntry,
  type PluginUiRefreshRequest,
  type PluginUiRefreshResponse,
  type PluginUiRefreshSectionResponse,
  type PluginViewState,
  type PluginWasmHandle as CorePluginWasmHandle,
  type PresencePeer,
  type UiDirtyScope,
  type Puzzle2dBoardScene,
  type RasterScene,
  type StyleSpec,
  type TableScene,
  type TextEditorScene,
  type UtilityCategory,
  type UtilityLeaf,
  type UtilityNode,
  type UiButtonNode,
  type UiComponentSceneNode,
  type UiControlNode,
  type UiExternalSlotNode,
  type UiFieldNode,
  type UiIconSelectNode,
  type UiImageNode,
  type UiInputNode,
  type UiInspectorFieldGroup,
  type UiKeyValueEntry,
  type UiKeyValueNode,
  type UiNode,
  type UiNumberStepperNode,
  type UiRingNode,
  type UiSectionNode,
  type UiSelectItem,
  type UiSelectNode,
  type UiSeparatorNode,
  type UiSliderNode,
  type UiStackNode,
  type UiTextNode,
  type UiToggleNode,
  type UiTreeItemAction,
  type UiTreeItemNode,
  type UiTreeNode,
  type UiTreeSectionNode,
  type UiVec3Node,
  type VcsHistoryScene,
  type VirtualFileSystemScene,
  type WindowEngagement,
  type WindowEngagementControl,
  type WindowEngagementInput,
  type WindowEngagementOption,
  type WindowEngagementPossible,
  type WindowEngagementRingOption,
  type WindowEngagementSelectItem,
  type WindowEngagementStatus,
  type WindowEngagementToggleGroupOption,
  type WindowLayout,
  type WindowLayoutAxisNode,
  type WindowLayoutStackNode,
  type WindowLayoutWindowNode,
  type WindowMeasure,
  type World3dScene,
} from "@semio-tech/framework-core";
import {
  FRAMEWORK_SYNC_CONTROLLER_ID,
  buildFileBackboneUri,
  buildFolderBackboneUri,
  buildFrameworkSyncUtilities,
  buildRemoteBackboneUri,
  type BackboneWorkerRequest,
  type BackboneWorkerResponse,
  type DocumentActorMsg,
  type DocumentSyncStatus,
  type FrameworkSyncUtilityLeaf,
  type PersistenceBinding,
} from "@semio-tech/framework-os-core";

/** 🔁 Re-exported so `node-graph-host.tsx`/`text-editor-host.tsx` can import action-name maps from this shell module rather than reaching into `@semio-tech/framework-core` directly. */
export { nodeGraphActions, textEditorActions };

//#region 🔖types
/** 🌐 Locale-resolved mixed-value placeholder for this renderer layer; framework/core/js/index.ts keeps its own non-reactive low-level default. */
export const UI_INSPECTOR_MIXED_PLACEHOLDER = shellLabel("ui.common.mixedValues");

/** 🎭 Renderer-side view state passed to plugin wasm calls — structurally mirrors `@semio-tech/framework-core`'s {@link PluginViewState}, kept as a distinct local alias since `ViewState` is the established name used throughout this file. */
export type ViewState = PluginViewState;

/** ⚠️ Not folded into `@semio-tech/framework-core`'s `PluginManifest`: this shell-local shape types `apps`/`programs` richly (`AppDefinition[]`, `document` on programs) where core intentionally keeps the wasm-boundary shape loose (`Record<string, unknown>[]`) for other consumers (e.g. compose, coda). Left for a human to decide whether to widen core's `PluginManifest` itself. */
export type PluginManifest = {
  readonly pluginId: string;
  readonly label: string;
  readonly version: string;
  readonly apps: readonly AppDefinition[];
  readonly programs: readonly { readonly programId: string; readonly appId: string; readonly label: string; readonly document: readonly string[]; readonly yields: string }[];
  readonly examples: readonly { readonly id: string; readonly label: string; readonly documentJson: string; readonly appId: string }[];
  readonly contributions?: readonly {
    readonly kind: "protocolBlockKind";
    readonly appId: string;
    readonly blockKind: string;
    readonly label: string;
    readonly iconId: string;
    readonly defaultValueJson?: string;
    readonly paramsBodyKey: string;
    readonly previewBodyKey: string;
  }[];
  /** 🎛️ Plugin-scope commands this plugin exposes — apply whenever any of its apps is focused. */
  readonly commands?: readonly CommandDefinition[];
};

type LoadedPluginState = {
  readonly handle: PluginWasmHandle;
  readonly manifest: PluginManifest;
};

type ActiveSession = {
  readonly pluginId: string;
  readonly instanceId: number;
  readonly app: AppDefinition;
  readonly viewState: ViewState;
};

type StudioProgramEntry = {
  readonly pluginId: string;
  readonly programId: string;
  readonly appId: string;
  readonly label: string;
  readonly document: readonly string[];
  readonly yields: string;
};

type SpawnedAppEntry = {
  readonly id: string;
  readonly pluginId: string;
  readonly instanceId: number;
  readonly appId: string;
  readonly label: string;
  readonly document: readonly string[];
};

type StudioPanelState = {
  readonly activePanelTab: string;
  readonly programs: readonly StudioProgramEntry[];
  readonly spawnedApps: readonly SpawnedAppEntry[];
  readonly activeSpawnedId?: string;
};

export type FrameworkOsBootOptions = {
  readonly rootId?: string;
  readonly plugin?: string;
  readonly plugins?: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
  readonly appId?: string;
  readonly locks?: FrameworkOsLocks;
};

//#region 🔒FrameworkOsLocks
/** 🔒 Raw boot-time lock values from env, before validation (any of the five may be unset). */
export type FrameworkOsLocks = {
  readonly exampleId?: string;
  readonly locale?: string;
  readonly terminology?: string;
  readonly themeId?: string;
  readonly appearance?: string;
};

/** 🔒 Validated locks: unknown values warn and fall back to a safe default while staying locked. */
export type ResolvedShellLocks = {
  readonly exampleId?: string;
  readonly locale?: UiLocale;
  readonly terminology?: string;
  readonly themeId?: string;
  readonly appearance?: ElementsSurfaceAppearance;
};

/**
 * 🔒 Validates raw `FrameworkOsLocks` against what the shell can actually apply at boot. A locked
 * session stays locked even on an invalid value (falls back to a default) rather than silently
 * degrading to switchable — the CLI asked for no in-app switching, so a typo must not remove that.
 */
export function resolveShellLocks(locks: FrameworkOsLocks | undefined): ResolvedShellLocks {
  if (!locks) return {};
  const resolved: { -readonly [K in keyof ResolvedShellLocks]?: ResolvedShellLocks[K] } = {};
  if (locks.exampleId) resolved.exampleId = locks.exampleId;
  if (locks.locale !== undefined) {
    if (locks.locale === "en" || locks.locale === "de") {
      resolved.locale = locks.locale;
    } else {
      console.warn(`[os] invalid SEMIO_LOCKED_LOCALE ${JSON.stringify(locks.locale)}, falling back to "en"`);
      resolved.locale = "en";
    }
  }
  if (locks.terminology) resolved.terminology = locks.terminology;
  if (locks.themeId !== undefined) {
    const known = new Set([...builtinUiThemes().map((t) => t.id), ...Object.keys(readStoredUiCustomThemes())]);
    if (known.has(locks.themeId)) {
      resolved.themeId = locks.themeId;
    } else {
      console.warn(`[os] invalid SEMIO_LOCKED_THEME ${JSON.stringify(locks.themeId)}, falling back to "semio"`);
      resolved.themeId = "semio";
    }
  }
  if (locks.appearance !== undefined) {
    if (locks.appearance === "light" || locks.appearance === "dark") {
      resolved.appearance = locks.appearance;
    } else {
      console.warn(`[os] invalid SEMIO_LOCKED_APPEARANCE ${JSON.stringify(locks.appearance)}, falling back to "system"`);
      resolved.appearance = "system";
    }
  }
  return resolved;
}

/** 🔒 Stable empty-locks reference so an omitted `locks` prop never busts memo dependency arrays. */
const EMPTY_SHELL_LOCKS: ResolvedShellLocks = {};
//#endregion 🔒FrameworkOsLocks

type SyncCardKind = "file" | "folder" | "remote";

type UIHistoryEntry = { readonly uri: string };
type UIHistory = { readonly entries: readonly UIHistoryEntry[]; readonly index: number };
//#endregion 🔖types

//#region 🧮ShellStore
/** 🧮 Single consolidated `useReducer` state tree for `FrameworkOsShell`, replacing what used to be ~38 independent `useState` calls with one dispatch-driven store, grouped by concern. */

//#region slice shapes
type PluginRuntimeState = {
  readonly loadedPlugins: readonly LoadedPluginState[];
  readonly session: ActiveSession | null;
  readonly error: string | null;
};

type WindowUiState = {
  readonly windowUiByKind: Readonly<Record<string, UiNode>>;
  readonly windowEngagementsByKind: Readonly<Record<string, WindowEngagement>>;
  readonly windowMeasuresByKind: Readonly<Record<string, readonly WindowMeasure[]>>;
  readonly panelUiByKey: Readonly<Record<string, UiNode>>;
  readonly appLabelsOverlay: PluginAppLabelsOverlay;
};

type SpawnedWindowState = {
  readonly spawnedWindowUi: UiNode | null;
  readonly spawnedWindowEngagements: Readonly<Record<string, WindowEngagement>>;
  readonly spawnedWindowMeasures: Readonly<Record<string, readonly WindowMeasure[]>>;
};

/**
 * 🧰 Per-window Action rail (P1–P5) state: fold/expand chrome, locally-buffered staged arg values
 * (keyed `${windowId}:${actionId}`, never dispatched until Execute), and the host-owned active utility per
 * window (never a document field, never a VCS op). See {@link WindowActionPane}.
 */
type ActionPaneState = {
  readonly foldedByWindowId: Readonly<Record<string, boolean>>;
  readonly expandedByWindowId: Readonly<Record<string, string | null>>;
  readonly stagedArgsByKey: Readonly<Record<string, Readonly<Record<string, unknown>>>>;
  readonly activeUtilityByWindowId: Readonly<Record<string, string | null>>;
};

/** 🧰 Composite key into {@link ActionPaneState.stagedArgsByKey}. */
export function actionStageKey(windowId: string, actionId: string): string {
  return `${windowId}:${actionId}`;
}

type ExtraWindowInstance = { readonly id: string; readonly windowKindId: string; readonly title: string };

/** 🧭 Per-anchor fold/size/active-tab-path state for one of the six {@link Panel}s. */
type PanelState = {
  readonly visible: boolean;
  readonly size: number;
  readonly path: readonly string[];
};

type ShellLayoutState = {
  readonly panels: Record<PanelAnchor, PanelState>;
  /** 🗄️ User-rearranged dock diff against `defaultDock`, persisted via `DockLayoutStore`; `null` means "use the computed default arrangement". */
  readonly dockOverride: DockSkeleton | null;
  /** 🌱 Per-branch drill-down memory across every anchor + mobile (see `progressPanelTabSelection`), persisted via `DockUiStateStore`. */
  readonly panelPathMemory: Readonly<Record<string, string>>;
  /** 🌱 Persisted tree section/group expansion, namespaced per {@link PanelTreeUnit}, persisted via `DockUiStateStore`. */
  readonly treeOpenStates: Readonly<Record<string, boolean>>;
  readonly activeWindowId: string | null;
  readonly shellLayout: WindowLayoutNode | null;
  readonly activeExampleId: string;
  readonly mobilePanelPath: readonly string[];
  readonly extraWindowInstances: readonly ExtraWindowInstance[];
};

type OverlayState = {
  readonly searchOpen: boolean;
  readonly findOpen: boolean;
  /** 🎓 Current step of the active app's introduction walkthrough, or `null` when none is playing. */
  readonly introductionStepIndex: number | null;
  /** 🗨️ The open declared dialog (id + `HostEffect`-seeded args), or `null` when none is open. */
  readonly dialog: { readonly dialogId: string; readonly seedArgs?: Readonly<Record<string, unknown>> } | null;
};

type UiPrefsState = {
  readonly uiAppearance: ElementsSurfaceAppearance;
  readonly uiLayout: UiChromeLayout;
  readonly uiCompact: boolean;
  readonly uiExpertise: Expertise;
  readonly uiLocale: UiLocale;
  readonly uiTerminology: string;
  readonly uiThemeId: string;
  readonly uiCustomThemes: Record<string, UiTheme>;
  readonly uiThemeDraft: UiTheme | null;
};

type SyncState = {
  readonly syncBackboneUri: string | null;
  readonly syncCardKind: SyncCardKind | null;
  readonly syncDraftPath: string;
  /** 🚦 Per-document sync health fed by `backbone-worker.ts`'s `DocumentEvent::Status` events, keyed by `documentId`. */
  readonly syncStatusByDocumentId: Readonly<Record<string, DocumentSyncStatus>>;
};

/**
 * 🎛️ Command palette state not already covered by the generic per-anchor `Panel` state: the command
 * whose arg form is expanded one level above the command list (exclusive — only one at a time), and
 * locally-buffered staged arg values (never dispatched until Execute). Which category is active/folded
 * lives in `layout.panels["bottom-middle"]` like any other anchor — see `buildCommandCategoryTabs`.
 */
export type CommandPanelState = {
  readonly expandedCommandId: string | null;
  readonly stagedArgsByCommandId: Readonly<Record<string, Readonly<Record<string, unknown>>>>;
};

export type ShellState = {
  readonly pluginRuntime: PluginRuntimeState;
  readonly windowUi: WindowUiState;
  readonly spawnedWindow: SpawnedWindowState;
  readonly actionPane: ActionPaneState;
  readonly commandPanel: CommandPanelState;
  readonly layout: ShellLayoutState;
  readonly overlays: OverlayState;
  readonly uiPrefs: UiPrefsState;
  readonly sync: SyncState;
};
//#endregion slice shapes

//#region actions
/** 🌀 A `useState`-style value-or-updater payload, kept so every migrated `setXxx` call-site can dispatch its existing `value` or `(prev) => next` argument unchanged. */
type Updatable<T> = T | ((prev: T) => T);

const resolveUpdatable = <T,>(next: Updatable<T>, prev: T): T => (typeof next === "function" ? (next as (prev: T) => T)(prev) : next);

export type ShellAction =
  | { readonly type: "SET_LOADED_PLUGINS"; readonly value: Updatable<readonly LoadedPluginState[]> }
  | { readonly type: "SET_SESSION"; readonly value: Updatable<ActiveSession | null> }
  | { readonly type: "SET_ERROR"; readonly value: Updatable<string | null> }
  | { readonly type: "SET_WINDOW_UI_BY_KIND"; readonly value: Updatable<Readonly<Record<string, UiNode>>> }
  | { readonly type: "SET_WINDOW_ENGAGEMENTS_BY_KIND"; readonly value: Updatable<Readonly<Record<string, WindowEngagement>>> }
  | { readonly type: "SET_WINDOW_MEASURES_BY_KIND"; readonly value: Updatable<Readonly<Record<string, readonly WindowMeasure[]>>> }
  | { readonly type: "SET_PANEL_UI_BY_KEY"; readonly value: Updatable<Readonly<Record<string, UiNode>>> }
  | { readonly type: "SET_APP_LABELS_OVERLAY"; readonly value: Updatable<PluginAppLabelsOverlay> }
  | { readonly type: "SET_SPAWNED_WINDOW_UI"; readonly value: Updatable<UiNode | null> }
  | { readonly type: "SET_SPAWNED_WINDOW_ENGAGEMENTS"; readonly value: Updatable<Readonly<Record<string, WindowEngagement>>> }
  | { readonly type: "SET_SPAWNED_WINDOW_MEASURES"; readonly value: Updatable<Readonly<Record<string, readonly WindowMeasure[]>>> }
  | { readonly type: "SET_ACTION_PANE_FOLDED"; readonly windowId: string; readonly value: boolean }
  | { readonly type: "SET_ACTION_PANE_EXPANDED"; readonly windowId: string; readonly value: string | null }
  | { readonly type: "STAGE_ACTION_ARG"; readonly windowId: string; readonly actionId: string; readonly argId: string; readonly value: unknown }
  | { readonly type: "RESET_ACTION_ARGS"; readonly windowId: string; readonly actionId: string }
  | { readonly type: "SET_ACTIVE_UTILITY"; readonly windowId: string; readonly utilityId: string | null }
  | { readonly type: "SET_COMMAND_EXPANDED"; readonly value: string | null }
  | { readonly type: "STAGE_COMMAND_ARG"; readonly commandId: string; readonly argId: string; readonly value: unknown }
  | { readonly type: "RESET_COMMAND_ARGS"; readonly commandId: string }
  | { readonly type: "SET_PANEL_VISIBLE"; readonly anchor: PanelAnchor; readonly value: Updatable<boolean> }
  | { readonly type: "SET_PANEL_SIZE"; readonly anchor: PanelAnchor; readonly value: Updatable<number> }
  | { readonly type: "SET_PANEL_PATH"; readonly anchor: PanelAnchor; readonly value: Updatable<readonly string[]> }
  | { readonly type: "SET_DOCK_OVERRIDE"; readonly value: DockSkeleton | null }
  | { readonly type: "SET_PANEL_PATH_MEMORY"; readonly value: Updatable<Readonly<Record<string, string>>> }
  | { readonly type: "SET_TREE_OPEN_STATE"; readonly id: string; readonly open: boolean }
  | { readonly type: "HYDRATE_DOCK_UI"; readonly value: DockUiState | null }
  | { readonly type: "RESET_DOCK" }
  | { readonly type: "SET_ACTIVE_WINDOW_ID"; readonly value: Updatable<string | null> }
  | { readonly type: "SET_SHELL_LAYOUT"; readonly value: Updatable<WindowLayoutNode | null> }
  | { readonly type: "SET_ACTIVE_EXAMPLE_ID"; readonly value: Updatable<string> }
  | { readonly type: "SET_MOBILE_PANEL_PATH"; readonly value: Updatable<readonly string[]> }
  | { readonly type: "SET_EXTRA_WINDOW_INSTANCES"; readonly value: Updatable<readonly ExtraWindowInstance[]> }
  | { readonly type: "SET_SEARCH_OPEN"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_FIND_OPEN"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_INTRODUCTION_STEP"; readonly value: Updatable<number | null> }
  | { readonly type: "SET_DIALOG"; readonly value: OverlayState["dialog"] }
  | { readonly type: "SET_UI_APPEARANCE"; readonly value: Updatable<ElementsSurfaceAppearance> }
  | { readonly type: "SET_UI_LAYOUT"; readonly value: Updatable<UiChromeLayout> }
  | { readonly type: "SET_UI_COMPACT"; readonly value: Updatable<boolean> }
  | { readonly type: "SET_UI_EXPERTISE"; readonly value: Updatable<Expertise> }
  | { readonly type: "SET_UI_LOCALE"; readonly value: Updatable<UiLocale> }
  | { readonly type: "SET_UI_TERMINOLOGY"; readonly value: Updatable<string> }
  | { readonly type: "SET_UI_THEME_ID"; readonly value: Updatable<string> }
  | { readonly type: "SET_UI_CUSTOM_THEMES"; readonly value: Updatable<Record<string, UiTheme>> }
  | { readonly type: "SET_UI_THEME_DRAFT"; readonly value: Updatable<UiTheme | null> }
  | { readonly type: "SET_SYNC_BACKBONE_URI"; readonly value: Updatable<string | null> }
  | { readonly type: "SET_SYNC_CARD_KIND"; readonly value: Updatable<SyncCardKind | null> }
  | { readonly type: "SET_SYNC_DRAFT_PATH"; readonly value: Updatable<string> }
  | { readonly type: "SET_SYNC_STATUS_FOR_DOCUMENT"; readonly documentId: string; readonly status: DocumentSyncStatus };
//#endregion actions

//#region slice reducers
function pluginRuntimeReducer(state: PluginRuntimeState, action: ShellAction): PluginRuntimeState {
  switch (action.type) {
    case "SET_LOADED_PLUGINS":
      return { ...state, loadedPlugins: resolveUpdatable(action.value, state.loadedPlugins) };
    case "SET_SESSION":
      return { ...state, session: resolveUpdatable(action.value, state.session) };
    case "SET_ERROR":
      return { ...state, error: resolveUpdatable(action.value, state.error) };
    default:
      return state;
  }
}

function windowUiReducer(state: WindowUiState, action: ShellAction): WindowUiState {
  switch (action.type) {
    case "SET_WINDOW_UI_BY_KIND":
      return { ...state, windowUiByKind: resolveUpdatable(action.value, state.windowUiByKind) };
    case "SET_WINDOW_ENGAGEMENTS_BY_KIND":
      return { ...state, windowEngagementsByKind: resolveUpdatable(action.value, state.windowEngagementsByKind) };
    case "SET_WINDOW_MEASURES_BY_KIND":
      return { ...state, windowMeasuresByKind: resolveUpdatable(action.value, state.windowMeasuresByKind) };
    case "SET_PANEL_UI_BY_KEY":
      return { ...state, panelUiByKey: resolveUpdatable(action.value, state.panelUiByKey) };
    case "SET_APP_LABELS_OVERLAY":
      return { ...state, appLabelsOverlay: resolveUpdatable(action.value, state.appLabelsOverlay) };
    default:
      return state;
  }
}

function spawnedWindowReducer(state: SpawnedWindowState, action: ShellAction): SpawnedWindowState {
  switch (action.type) {
    case "SET_SPAWNED_WINDOW_UI":
      return { ...state, spawnedWindowUi: resolveUpdatable(action.value, state.spawnedWindowUi) };
    case "SET_SPAWNED_WINDOW_ENGAGEMENTS":
      return { ...state, spawnedWindowEngagements: resolveUpdatable(action.value, state.spawnedWindowEngagements) };
    case "SET_SPAWNED_WINDOW_MEASURES":
      return { ...state, spawnedWindowMeasures: resolveUpdatable(action.value, state.spawnedWindowMeasures) };
    default:
      return state;
  }
}

/** 🧰 Reducer for the per-window Action rail slice (P1–P5). Every case preserves referential identity when nothing actually changes so downstream memos can bail. */
function actionPaneReducer(state: ActionPaneState, action: ShellAction): ActionPaneState {
  switch (action.type) {
    case "SET_ACTION_PANE_FOLDED": {
      if (state.foldedByWindowId[action.windowId] === action.value) return state;
      return { ...state, foldedByWindowId: { ...state.foldedByWindowId, [action.windowId]: action.value } };
    }
    case "SET_ACTION_PANE_EXPANDED": {
      if ((state.expandedByWindowId[action.windowId] ?? null) === action.value) return state;
      return { ...state, expandedByWindowId: { ...state.expandedByWindowId, [action.windowId]: action.value } };
    }
    case "STAGE_ACTION_ARG": {
      const key = actionStageKey(action.windowId, action.actionId);
      const current = state.stagedArgsByKey[key] ?? {};
      if (Object.prototype.hasOwnProperty.call(current, action.argId) && current[action.argId] === action.value) return state;
      return { ...state, stagedArgsByKey: { ...state.stagedArgsByKey, [key]: { ...current, [action.argId]: action.value } } };
    }
    case "RESET_ACTION_ARGS": {
      const key = actionStageKey(action.windowId, action.actionId);
      if (!Object.prototype.hasOwnProperty.call(state.stagedArgsByKey, key)) return state;
      const next = { ...state.stagedArgsByKey };
      delete next[key];
      return { ...state, stagedArgsByKey: next };
    }
    case "SET_ACTIVE_UTILITY": {
      if ((state.activeUtilityByWindowId[action.windowId] ?? null) === action.utilityId) return state;
      return { ...state, activeUtilityByWindowId: { ...state.activeUtilityByWindowId, [action.windowId]: action.utilityId } };
    }
    default:
      return state;
  }
}

/** 🎛️ Reducer for the command palette's arg-expansion/staging slice — category active/fold state is the `bottom-middle` anchor's own generic `SET_PANEL_VISIBLE`/`SET_PANEL_PATH` (see `shellLayoutReducer`), not handled here. */
function commandPanelReducer(state: CommandPanelState, action: ShellAction): CommandPanelState {
  switch (action.type) {
    case "SET_COMMAND_EXPANDED": {
      if (state.expandedCommandId === action.value) return state;
      return { ...state, expandedCommandId: action.value };
    }
    case "STAGE_COMMAND_ARG": {
      const current = state.stagedArgsByCommandId[action.commandId] ?? {};
      if (Object.prototype.hasOwnProperty.call(current, action.argId) && current[action.argId] === action.value) return state;
      return { ...state, stagedArgsByCommandId: { ...state.stagedArgsByCommandId, [action.commandId]: { ...current, [action.argId]: action.value } } };
    }
    case "RESET_COMMAND_ARGS": {
      if (!Object.prototype.hasOwnProperty.call(state.stagedArgsByCommandId, action.commandId)) return state;
      const next = { ...state.stagedArgsByCommandId };
      delete next[action.commandId];
      return { ...state, stagedArgsByCommandId: next };
    }
    default:
      return state;
  }
}

function shellLayoutReducer(state: ShellLayoutState, action: ShellAction): ShellLayoutState {
  switch (action.type) {
    case "SET_PANEL_VISIBLE":
      return { ...state, panels: { ...state.panels, [action.anchor]: { ...state.panels[action.anchor], visible: resolveUpdatable(action.value, state.panels[action.anchor].visible) } } };
    case "SET_PANEL_SIZE":
      return { ...state, panels: { ...state.panels, [action.anchor]: { ...state.panels[action.anchor], size: resolveUpdatable(action.value, state.panels[action.anchor].size) } } };
    case "SET_PANEL_PATH":
      return { ...state, panels: { ...state.panels, [action.anchor]: { ...state.panels[action.anchor], path: resolveUpdatable(action.value, state.panels[action.anchor].path) } } };
    case "SET_DOCK_OVERRIDE":
      return { ...state, dockOverride: action.value };
    case "SET_PANEL_PATH_MEMORY":
      return { ...state, panelPathMemory: resolveUpdatable(action.value, state.panelPathMemory) };
    case "SET_TREE_OPEN_STATE":
      return { ...state, treeOpenStates: { ...state.treeOpenStates, [action.id]: action.open } };
    case "HYDRATE_DOCK_UI": {
      if (!action.value) return state;
      const panels = { ...state.panels };
      for (const anchor of PANEL_ANCHORS) {
        const saved = action.value.anchors[anchor];
        if (!saved) continue;
        panels[anchor] = {
          visible: saved.visible ?? panels[anchor].visible,
          size: saved.size ?? panels[anchor].size,
          path: saved.path ?? panels[anchor].path,
        };
      }
      return { ...state, panels, panelPathMemory: action.value.pathMemory ?? state.panelPathMemory, treeOpenStates: action.value.treeOpen ?? state.treeOpenStates };
    }
    case "RESET_DOCK": {
      const panels = {} as Record<PanelAnchor, PanelState>;
      for (const anchor of PANEL_ANCHORS) panels[anchor] = { visible: false, size: DEFAULT_PANEL_SIZES[anchor], path: [] };
      return { ...state, dockOverride: null, panels, panelPathMemory: {}, treeOpenStates: {} };
    }
    case "SET_ACTIVE_WINDOW_ID":
      return { ...state, activeWindowId: resolveUpdatable(action.value, state.activeWindowId) };
    case "SET_SHELL_LAYOUT":
      return { ...state, shellLayout: resolveUpdatable(action.value, state.shellLayout) };
    case "SET_ACTIVE_EXAMPLE_ID":
      return { ...state, activeExampleId: resolveUpdatable(action.value, state.activeExampleId) };
    case "SET_MOBILE_PANEL_PATH":
      return { ...state, mobilePanelPath: resolveUpdatable(action.value, state.mobilePanelPath) };
    case "SET_EXTRA_WINDOW_INSTANCES":
      return { ...state, extraWindowInstances: resolveUpdatable(action.value, state.extraWindowInstances) };
    default:
      return state;
  }
}

function overlayReducer(state: OverlayState, action: ShellAction): OverlayState {
  switch (action.type) {
    case "SET_SEARCH_OPEN":
      return { ...state, searchOpen: resolveUpdatable(action.value, state.searchOpen) };
    case "SET_FIND_OPEN":
      return { ...state, findOpen: resolveUpdatable(action.value, state.findOpen) };
    case "SET_INTRODUCTION_STEP":
      return { ...state, introductionStepIndex: resolveUpdatable(action.value, state.introductionStepIndex) };
    case "SET_DIALOG":
      return { ...state, dialog: action.value };
    default:
      return state;
  }
}

function uiPrefsReducer(state: UiPrefsState, action: ShellAction): UiPrefsState {
  switch (action.type) {
    case "SET_UI_APPEARANCE":
      return { ...state, uiAppearance: resolveUpdatable(action.value, state.uiAppearance) };
    case "SET_UI_LAYOUT":
      return { ...state, uiLayout: resolveUpdatable(action.value, state.uiLayout) };
    case "SET_UI_COMPACT":
      return { ...state, uiCompact: resolveUpdatable(action.value, state.uiCompact) };
    case "SET_UI_EXPERTISE":
      return { ...state, uiExpertise: resolveUpdatable(action.value, state.uiExpertise) };
    case "SET_UI_LOCALE":
      return { ...state, uiLocale: resolveUpdatable(action.value, state.uiLocale) };
    case "SET_UI_TERMINOLOGY":
      return { ...state, uiTerminology: resolveUpdatable(action.value, state.uiTerminology) };
    case "SET_UI_THEME_ID":
      return { ...state, uiThemeId: resolveUpdatable(action.value, state.uiThemeId) };
    case "SET_UI_CUSTOM_THEMES":
      return { ...state, uiCustomThemes: resolveUpdatable(action.value, state.uiCustomThemes) };
    case "SET_UI_THEME_DRAFT":
      return { ...state, uiThemeDraft: resolveUpdatable(action.value, state.uiThemeDraft) };
    default:
      return state;
  }
}

function syncReducer(state: SyncState, action: ShellAction): SyncState {
  switch (action.type) {
    case "SET_SYNC_BACKBONE_URI":
      return { ...state, syncBackboneUri: resolveUpdatable(action.value, state.syncBackboneUri) };
    case "SET_SYNC_CARD_KIND":
      return { ...state, syncCardKind: resolveUpdatable(action.value, state.syncCardKind) };
    case "SET_SYNC_DRAFT_PATH":
      return { ...state, syncDraftPath: resolveUpdatable(action.value, state.syncDraftPath) };
    case "SET_SYNC_STATUS_FOR_DOCUMENT":
      return { ...state, syncStatusByDocumentId: { ...state.syncStatusByDocumentId, [action.documentId]: action.status } };
    default:
      return state;
  }
}
//#endregion slice reducers

/** 🧵 Root reducer for `FrameworkOsShell` — fans every action out to its owning slice reducer; slices that ignore an action's type return their input unchanged, so unrelated slices keep referential identity. */
export function shellReducer(state: ShellState, action: ShellAction): ShellState {
  return {
    pluginRuntime: pluginRuntimeReducer(state.pluginRuntime, action),
    windowUi: windowUiReducer(state.windowUi, action),
    spawnedWindow: spawnedWindowReducer(state.spawnedWindow, action),
    actionPane: actionPaneReducer(state.actionPane, action),
    commandPanel: commandPanelReducer(state.commandPanel, action),
    layout: shellLayoutReducer(state.layout, action),
    overlays: overlayReducer(state.overlays, action),
    uiPrefs: uiPrefsReducer(state.uiPrefs, action),
    sync: syncReducer(state.sync, action),
  };
}

//#region selectors
export const selectUiDevice = (state: ShellState, mobile: boolean): ElementsSurfaceDevice => (mobile ? "mobile" : state.uiPrefs.uiLayout);
//#endregion selectors

/** 🌱 Builds the starting `ShellState` for `FrameworkOsShell`, mirroring exactly what each migrated `useState` used to initialize to (including reads from local storage for UI prefs). */
export function initialShellState(_props: {
  readonly pluginFilter?: string;
  readonly plugins: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
  readonly locks?: ResolvedShellLocks;
}): ShellState {
  const locks = _props.locks ?? {};
  return {
    pluginRuntime: { loadedPlugins: [], session: null, error: null },
    windowUi: { windowUiByKind: {}, windowEngagementsByKind: {}, windowMeasuresByKind: {}, panelUiByKey: {}, appLabelsOverlay: EMPTY_APP_LABELS_OVERLAY },
    spawnedWindow: { spawnedWindowUi: null, spawnedWindowEngagements: {}, spawnedWindowMeasures: {} },
    actionPane: { foldedByWindowId: {}, expandedByWindowId: {}, stagedArgsByKey: {}, activeUtilityByWindowId: {} },
    commandPanel: { expandedCommandId: null, stagedArgsByCommandId: {} },
    layout: {
      panels: Object.fromEntries(PANEL_ANCHORS.map((anchor) => [anchor, { visible: false, size: DEFAULT_PANEL_SIZES[anchor], path: [] }])) as Record<PanelAnchor, PanelState>,
      dockOverride: null,
      panelPathMemory: {},
      treeOpenStates: {},
      activeWindowId: null,
      shellLayout: null,
      activeExampleId: locks.exampleId ?? "",
      mobilePanelPath: [],
      extraWindowInstances: [],
    },
    overlays: { searchOpen: false, findOpen: false, introductionStepIndex: null, dialog: null },
    uiPrefs: {
      uiAppearance: locks.appearance ?? readStoredUiChromeAppearance(),
      uiLayout: readStoredUiChromeLayout(),
      uiCompact: readStoredUiChromeCompact(),
      uiExpertise: readStoredUiChromeExpertise(),
      uiLocale: locks.locale ?? readStoredUiChromeLocale() ?? (uiI18n.resolvedLanguage?.toLowerCase().startsWith("de") ? "de" : "en"),
      uiTerminology: locks.terminology ?? readStoredUiChromeTerminology(),
      uiThemeId: locks.themeId ?? readStoredUiChromeThemeId() ?? "semio",
      uiCustomThemes: readStoredUiCustomThemes(),
      uiThemeDraft: null,
    },
    sync: { syncBackboneUri: null, syncCardKind: null, syncDraftPath: "", syncStatusByDocumentId: {} },
  };
}
//#endregion 🧮ShellStore

//#region ShellHelpers
function syncDocumentId(session: ActiveSession, panel: StudioPanelState | null, studioMode: boolean): string {
  if (studioMode && panel?.activeSpawnedId) {
    const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
    if (spawned) return `${spawned.pluginId}-${spawned.instanceId}`;
  }
  return `${session.pluginId}-${session.instanceId}`;
}

const S_HOME_APP_ID = "home";
const S_HOME_CONTROLLER_ID = "s-home";
const S_PLAY_APP_ID = "studio";
const S_PLAY_CONTROLLER_ID = "s-play";
const S_PLAY_CATALOGUE_TAB_ID = "s-play-catalogue";
/** @emoji 🧭 Starting width for each panel anchor — `top-left`/`top-right` mirror the old left/right side-panel defaults; `bottom-left`/`bottom-right` host the sync card and a compact utility tree, so a narrower default suits them; the middle anchors start empty but default wider since they grow both ways and tend to host transient centered content (e.g. search). */
const DEFAULT_PANEL_SIZES: Record<PanelAnchor, number> = {
  "top-left": 280,
  "top-middle": 360,
  "top-right": 320,
  "bottom-left": 240,
  "bottom-middle": 360,
  "bottom-right": 240,
};

/** @emoji 🌳 Root category id for the nested dock tab tree — the top row of {@link defaultDock}'s bottom-left (Display) anchor tabs; top-left (Workbench), top-right (Details) and bottom-right (Settings) render their tabs flat instead of under a category branch. */
const FRAMEWORK_CATEGORY_DISPLAY_ID = "framework.category.display";
/** @emoji 🎛 Root category id bundling every command-category leaf under one expandable Command toggle on bottom-middle (mirrors Display on bottom-left). */
const FRAMEWORK_CATEGORY_COMMAND_ID = "framework.category.command";
const APP_DOCUMENT_SEPARATOR = " · ";

const PRESENCE_CLIENT_STORAGE_KEY = "semio.presence.client";
const PRESENCE_HEARTBEAT_INTERVAL_MS = 5000;

function presenceClientIdentity(): { readonly clientId: string; readonly name: string } {
  if (typeof window === "undefined") return { clientId: "server", name: "Server" };
  const stored = window.sessionStorage.getItem(PRESENCE_CLIENT_STORAGE_KEY);
  if (stored) {
    try {
      const parsed = JSON.parse(stored) as { readonly clientId?: string; readonly name?: string };
      if (parsed.clientId && parsed.name) return { clientId: parsed.clientId, name: parsed.name };
    } catch {
      /* reseed identity */
    }
  }
  const clientId = `client-${Math.random().toString(36).slice(2, 10)}`;
  const identity = { clientId, name: `Guest ${clientId.slice(-4).toUpperCase()}` };
  window.sessionStorage.setItem(PRESENCE_CLIENT_STORAGE_KEY, JSON.stringify(identity));
  return identity;
}

function readBrowserUri(): string {
  if (typeof window === "undefined") return "/";
  return `${window.location.pathname}${window.location.search}` || "/";
}

function useUIHistory(initialUri = "/", syncBrowser = false) {
  const [history, setHistory] = useState<UIHistory>(() => ({
    entries: [{ uri: syncBrowser ? readBrowserUri() : initialUri }],
    index: 0,
  }));
  const uri = history.entries[history.index]?.uri ?? initialUri;
  const canGoBack = history.index > 0;
  const canGoForward = history.index < history.entries.length - 1;
  const segments = uri.split("/").filter(Boolean);
  const canGoUp = segments.length > 0;
  const parentUri = canGoUp ? `/${segments.slice(0, -1).join("/")}` : null;

  const goBack = useCallback(() => {
    setHistory((prev) => (prev.index > 0 ? { ...prev, index: prev.index - 1 } : prev));
  }, []);
  const goForward = useCallback(() => {
    setHistory((prev) => (prev.index < prev.entries.length - 1 ? { ...prev, index: prev.index + 1 } : prev));
  }, []);
  const goUp = useCallback(() => {
    if (!canGoUp || parentUri === null) return;
    setHistory((prev) => {
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: parentUri }], index: newEntries.length };
    });
  }, [canGoUp, parentUri]);
  const navigate = useCallback((targetUri: string) => {
    setHistory((prev) => {
      const existingIndex = prev.entries.findIndex((entry) => entry.uri === targetUri);
      if (existingIndex >= 0) return { ...prev, index: existingIndex };
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
    });
  }, []);
  const syncUri = useCallback((targetUri: string) => {
    setHistory((prev) => {
      const existingIndex = prev.entries.findIndex((entry) => entry.uri === targetUri);
      if (existingIndex >= 0) return { ...prev, index: existingIndex };
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
    });
  }, []);

  useEffect(() => {
    if (!syncBrowser || typeof window === "undefined") return;
    const current = `${window.location.pathname}${window.location.search}`;
    if (current !== uri) window.history.pushState(null, "", uri);
  }, [syncBrowser, uri]);

  useEffect(() => {
    if (!syncBrowser || typeof window === "undefined") return;
    const onPopState = () => syncUri(readBrowserUri());
    window.addEventListener("popstate", onPopState);
    return () => window.removeEventListener("popstate", onPopState);
  }, [syncBrowser, syncUri]);

  return { uri, canGoBack, canGoForward, canGoUp, parentUri, goBack, goForward, goUp, navigate, syncUri };
}

function downloadMediaExport(filename: string, mimeType: string, data: string, encoding?: string): void {
  if (typeof document === "undefined") return;
  const payload = encoding === "base64" ? Uint8Array.from(atob(data), (char) => char.charCodeAt(0)) : data;
  const blob = new Blob([payload], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.click();
  URL.revokeObjectURL(url);
}

function downloadDataUrl(filename: string, dataUrl: string): void {
  if (typeof document === "undefined") return;
  const anchor = document.createElement("a");
  anchor.href = dataUrl;
  anchor.download = filename;
  anchor.click();
}

function requestFileOpen(accept: string, readAs?: string): Promise<{ contents: string; name: string } | null> {
  if (typeof document === "undefined") return Promise.resolve(null);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = accept;
    input.onchange = async () => {
      const file = input.files?.[0];
      if (!file) {
        resolve(null);
        return;
      }
      if (readAs === "dataUrl") {
        const reader = new FileReader();
        reader.onload = () => resolve(typeof reader.result === "string" ? { contents: reader.result, name: file.name } : null);
        reader.onerror = () => resolve(null);
        reader.readAsDataURL(file);
        return;
      }
      resolve({ contents: await file.text(), name: file.name });
    };
    input.click();
  });
}

function isStudioMode(pluginFilter?: string): boolean {
  return pluginFilter === "s";
}

export interface StudioShellPath {
  readonly studioId: string;
  readonly instanceId?: string;
}

/** @emoji 🧭 Parses `/studios/:id` and `/studios/:id/instances/:iid` shell paths; null for any other route (e.g. home). */
export function parseStudioShellPath(path: string): StudioShellPath | null {
  const match = /^\/studios\/([^/]+)(?:\/instances\/([^/]+))?$/.exec(path);
  if (!match) return null;
  return { studioId: match[1]!, instanceId: match[2] };
}

function buildStudioPrograms(loaded: readonly LoadedPluginState[]): readonly StudioProgramEntry[] {
  return loaded.flatMap((entry) =>
    entry.manifest.programs.map((program) => ({
      pluginId: entry.handle.pluginId,
      programId: program.programId,
      appId: program.appId,
      label: program.label,
      document: program.document,
      yields: program.yields,
    })),
  );
}

export function appDocumentLabel(document: readonly string[]): string {
  return document.join(APP_DOCUMENT_SEPARATOR);
}

export function appWindowDocumentLabel(app: AppDefinition, windowLabel: string): string {
  return windowLabel.trim() || app.label.trim();
}

function buildStudioPanelState(programs: readonly StudioProgramEntry[], spawnedApps: readonly SpawnedAppEntry[], activePanelTab = "s-play-catalogue", activeSpawnedId?: string): StudioPanelState {
  return { activePanelTab, programs, spawnedApps, activeSpawnedId };
}

function panelJsonFromState(state: StudioPanelState): string {
  return JSON.stringify(state);
}

function parsePanelState(viewState: ViewState): StudioPanelState | null {
  if (!viewState.panelJson) return null;
  try {
    return JSON.parse(viewState.panelJson) as StudioPanelState;
  } catch {
    return null;
  }
}

/** @emoji 🧭 Default anchor a plugin-declared panel-tab `group` docks into — groups only ever map to the four corners; the two middle anchors start empty and are user-populated via drag-and-drop or a dock skeleton override. */
function panelAnchorForGroup(group: string): PanelAnchor {
  if (group === "workbench" || group === "document") return "top-left";
  if (group === "details") return "top-right";
  if (group === "display") return "bottom-left";
  if (group === "settings") return "bottom-right";
  return "top-right";
}

function convertFrameworkLayoutNodeToModeLayout(node: WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode, appLabelsOverlay: PluginAppLabelsOverlay): WindowLayoutNode {
  if (node.kind === "window") {
    return { kind: "window", id: node.windowKindId, title: resolveAppLabel(appLabelsOverlay, "windowKind", node.windowKindId, node.title ?? node.windowKindId) };
  }
  if (node.kind === "stack") {
    return {
      kind: "stack",
      size: node.size,
      children: node.children.map((child) => ({
        kind: "window" as const,
        id: child.windowKindId,
        title: resolveAppLabel(appLabelsOverlay, "windowKind", child.windowKindId, child.title ?? child.windowKindId),
      })),
    };
  }
  return {
    kind: node.kind,
    size: node.size,
    children: node.children.map((child) => convertFrameworkLayoutNodeToModeLayout(child, appLabelsOverlay)),
  };
}

/** @emoji 🗣️ Re-resolves every window's title from the current app-labels overlay in place, preserving the tree's structure/sizes/arrangement — used to react to a locale/terminology switch without discarding the user's live layout. */
function retitleWindowLayoutNode(node: WindowLayoutNode, appLabelsOverlay: PluginAppLabelsOverlay): WindowLayoutNode {
  if (node.kind === "window") {
    return { ...node, title: resolveAppLabel(appLabelsOverlay, "windowKind", node.id, node.title ?? node.id) };
  }
  return { ...node, children: node.children.map((child) => retitleWindowLayoutNode(child, appLabelsOverlay)) } as WindowLayoutNode;
}

function convertFrameworkLayoutToModeLayout(layout: WindowLayout | undefined, windowIds: readonly string[], appLabelsOverlay: PluginAppLabelsOverlay): WindowLayoutNode {
  if (!layout?.root) return createEvenWindowLayout(windowIds.length ? windowIds : ["main"]);
  return convertFrameworkLayoutNodeToModeLayout(layout.root, appLabelsOverlay);
}

function modeLayoutNodeToFramework(node: WindowLayoutNode): WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode {
  if (node.kind === "window") {
    return { kind: "window", windowKindId: node.id, ...(node.title ? { title: node.title } : {}) };
  }
  if (node.kind === "stack") {
    return {
      kind: "stack",
      ...(node.size !== undefined ? { size: node.size } : {}),
      children: node.children.map((child) => ({
        kind: "window" as const,
        windowKindId: child.id,
        ...(child.title ? { title: child.title } : {}),
      })),
    };
  }
  return {
    kind: node.kind,
    ...(node.size !== undefined ? { size: node.size } : {}),
    children: node.children.map((child) => modeLayoutNodeToFramework(child) as WindowLayoutStackNode | WindowLayoutAxisNode),
  };
}

function captureCurrentFrameworkLayout(shellLayout: WindowLayoutNode | null, fallback?: WindowLayout): WindowLayout | undefined {
  if (!shellLayout) return fallback;
  const root = modeLayoutNodeToFramework(shellLayout);
  if (root.kind === "window") return { root: { kind: "stack", children: [root] } };
  return { root };
}

function findDefaultActiveWindowKindId(layout: WindowLayout | undefined, windowKinds: readonly { readonly id: string }[]): string | null {
  const collectWindowIds = (node: WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode): string[] => {
    if (node.kind === "window") return [node.windowKindId];
    if (node.kind === "stack") return node.children.map((child) => child.windowKindId);
    return node.children.flatMap((child) => collectWindowIds(child));
  };
  const ordered = layout?.root ? collectWindowIds(layout.root) : windowKinds.map((kind) => kind.id);
  for (const id of ordered) {
    if (windowKinds.some((kind) => kind.id === id)) return id;
  }
  return windowKinds[0]?.id ?? null;
}

function windowEngagementControlToSpec(control: WindowEngagementControl | undefined, onAction: (action: ActionDescriptor) => void): EngagementControl | undefined {
  if (!control) return undefined;
  if (control.kind === "ring" || control.kind === "toggleGroup") {
    return {
      kind: control.kind,
      id: control.id,
      label: control.label,
      value: control.value,
      disabled: control.disabled,
      options: control.options.map((row) => ({ id: row.id, label: row.label, disabled: row.disabled })),
      onSelect: control.onSelect ? (id: string) => onAction({ ...control.onSelect!, args: { ...(control.onSelect!.args as object | undefined), id } }) : undefined,
    };
  }
  if (control.kind === "select") {
    return {
      kind: "select",
      id: control.id,
      label: control.label,
      value: control.value,
      placeholder: control.placeholder,
      disabled: control.disabled,
      items: control.items.map((row) => ({ id: row.id, value: row.value, label: row.label })),
      onChange: control.onChange ? (value: string) => onAction({ ...control.onChange!, args: { ...(control.onChange!.args as object | undefined), value } }) : undefined,
    };
  }
  const dispatchNumeric = (action: ActionDescriptor | undefined, value: number) => {
    if (!action) return;
    onAction({ ...action, args: { ...(action.args as object | undefined), value } });
  };
  return {
    kind: control.kind,
    id: control.id,
    label: control.label,
    value: control.value,
    min: control.min,
    max: control.max,
    step: control.step,
    unit: control.unit,
    disabled: control.disabled,
    onChange: control.onChange ? (value: number) => dispatchNumeric(control.onChange, value) : undefined,
    onCommit: control.onCommit ? (value: number) => dispatchNumeric(control.onCommit, value) : undefined,
  };
}

const PLUGIN_LOAD_TIMEOUT_MS = 30_000;

async function loadPluginModuleResilient(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle | null> {
  try {
    return await Promise.race([
      loadPluginModule(pluginId, moduleUrl),
      new Promise<never>((_, reject) => {
        window.setTimeout(() => reject(new Error(`timeout loading ${pluginId}`)), PLUGIN_LOAD_TIMEOUT_MS);
      }),
    ]);
  } catch (error) {
    console.error("[DEBUG] plugin load failed", pluginId, error);
    return null;
  }
}

function isViewportSurface(surfaceKind: string | undefined): boolean {
  return surfaceKind === "world-3d" || surfaceKind === "node-graph" || surfaceKind === "canvas-2d";
}

function defaultViewportEngagement(): WindowEngagement {
  return {
    sessionActive: true,
    status: [{ id: "framework.viewport.status", text: shellLabel("ui.engagement.viewport") }],
  };
}

function resolveWindowEngagement(kind: AppDefinition["windowKinds"][number], byKind: Readonly<Record<string, WindowEngagement>>): WindowEngagement | undefined {
  const surfaceKind = (kind as { surfaceKind?: string }).surfaceKind;
  const declaredEngagement = kind.options.engagement.kind === "some" ? kind.options.engagement.value : undefined;
  return byKind[kind.id] ?? declaredEngagement ?? (isViewportSurface(surfaceKind) ? defaultViewportEngagement() : undefined);
}

function windowEngagementToSpec(engagement: WindowEngagement | undefined, onAction: (action: ActionDescriptor) => void): EngagementSpec | undefined {
  if (!engagement) return undefined;
  const options = engagement.options?.map((option) => ({
    id: option.id,
    label: option.label,
    icon: option.iconId ? <Icon icon={option.iconId in ICONS ? (option.iconId as IconName) : "circle-dot"} size="small" /> : undefined,
    pressed: option.pressed,
    disabled: option.disabled,
    onPress: option.action ? () => onAction(option.action!) : undefined,
  }));
  const input = engagement.input
    ? {
        id: engagement.input.id,
        value: engagement.input.value,
        placeholder: engagement.input.placeholder,
        disabled: engagement.input.disabled,
        onChange: engagement.input.onChange ? (value: string) => onAction({ ...engagement.input!.onChange!, args: { ...(engagement.input!.onChange!.args as object | undefined), value } }) : undefined,
        onSubmit: engagement.input.onSubmit ? (value: string) => onAction({ ...engagement.input!.onSubmit!, args: { ...(engagement.input!.onSubmit!.args as object | undefined), value } }) : undefined,
        onRepeatLast: engagement.input.onRepeatLast ? () => onAction(engagement.input!.onRepeatLast!) : undefined,
        onAbort: engagement.input.onAbort ? () => onAction(engagement.input!.onAbort!) : undefined,
      }
    : undefined;
  const status = engagement.status?.map((row) => ({ id: row.id, content: row.text }));
  const possibleEngagements = engagement.possibleEngagements?.map((row) => ({
    id: row.id,
    label: row.label,
    detail: row.detail,
    onSelect: row.action ? () => onAction(row.action!) : undefined,
  }));
  const control = windowEngagementControlToSpec(engagement.control, onAction);
  const controls = engagement.controls?.map((row) => windowEngagementControlToSpec(row, onAction)).filter((row): row is EngagementControl => row !== undefined);
  const hasContent = (options?.length ?? 0) > 0 || Boolean(input) || Boolean(control) || (controls?.length ?? 0) > 0 || (status?.length ?? 0) > 0 || (possibleEngagements?.length ?? 0) > 0;
  if (!hasContent) return undefined;
  return { sessionActive: engagement.sessionActive, options, input, control, controls, status, possibleEngagements };
}

function panelTabIcon(tabId: string, group: string): React.FC<{ size?: number }> {
  if (tabId === S_PLAY_CATALOGUE_TAB_ID || group === "workbench") return shellTabIcon(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID);
  if (tabId.includes("parameters")) return shellTabIcon(FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID);
  if (tabId.includes("inspector")) return shellTabIcon(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID);
  return shellTabIcon(tabId);
}

/** @emoji 🌳 Category-row icon: the first child's icon, or `fallback` when the category has no tabs yet. */
function categoryTabIcon(tabs: readonly PanelTabNode[], fallback: IconName): React.FC<{ size?: number }> {
  const FirstIcon = tabs[0]?.icon;
  return function CategoryTabIcon({ size = 16 }: { size?: number }) {
    return FirstIcon ? <FirstIcon size={size} /> : <Icon icon={fallback} size="small" />;
  };
}

/** @emoji 🌳 Depth-first leaves of a recursive panel-tab tree — the nodes that actually carry a `bodyKey` to render. */
export function flattenPanelTabLeaves<T extends { readonly children?: readonly T[] }>(tabs: readonly T[]): T[] {
  return tabs.flatMap((tab) => (tab.children && tab.children.length > 0 ? flattenPanelTabLeaves(tab.children) : [tab]));
}

/** @emoji 🌳 Converts one plugin-declared {@link AppPanelTabDefinition} (recursively) into a {@link PanelTabNode}. */
function panelTabDefinitionToNode(tab: AppPanelTabDefinition, group: string, panelUiByKey: Readonly<Record<string, UiNode>>, onAction: (action: ActionDescriptor) => void, order: number, appLabelsOverlay: PluginAppLabelsOverlay): PanelTabNode {
  const tabId = panelTabKindId(tab.kind);
  const label = resolveAppLabel(appLabelsOverlay, "panelTab", tabId, tab.label);
  if (tab.children && tab.children.length > 0) {
    return {
      kind: "branch",
      id: tabId,
      icon: panelTabIcon(tabId, group),
      name: label,
      order,
      children: tab.children.map((child, childOrder) => panelTabDefinitionToNode(child, group, panelUiByKey, onAction, childOrder, appLabelsOverlay)),
    };
  }
  return singleTreeLeaf({
    id: tabId,
    icon: panelTabIcon(tabId, group),
    name: label,
    order,
    tree: staticTreePanelDefinition(uiNodeToTreePanelConfig(panelUiByKey[tabId] ?? { type: "text", value: shellLabel("ui.common.loading") }, onAction)),
  });
}

function resolveCanvasBodyKey(app: AppDefinition): string {
  const windowKind = app.windowKinds[0];
  if (!windowKind) return "main";
  if (windowKind.bodyKey.includes("composite")) {
    const mediaGraph = app.windowKinds.find((kind) => kind.bodyKey.includes("media-graph"));
    return mediaGraph?.bodyKey ?? windowKind.bodyKey;
  }
  return windowKind.bodyKey;
}

//#region 🧰UtilityRegistry
/**
 * 🧰 Resolves the `UtilityDefinition`s in scope for one window kind against the app's utility registry:
 * the window kind's own `utilities` refs when non-empty, otherwise every utility the app declares (the
 * scoping fallback, mirroring `resolveWindowActions`' intent for utilities). Unresolvable refs are dropped.
 */
export function resolveUtilities(app: Pick<AppDefinition, "utilities">, windowKind: Pick<AppWindowKindDefinition, "utilities">): UtilityDefinition[] {
  const registry = app.utilities ?? [];
  const refs = windowKind.utilities ?? [];
  if (refs.length === 0) return [...registry];
  const resolved: UtilityDefinition[] = [];
  for (const ref of refs) {
    const utility = registry.find((entry) => entry.id === ref);
    if (utility) resolved.push(utility);
  }
  return resolved;
}

/** 🧰 One `UtilityDefinition` → the lean `DerivedUtilitySpec` consumed by {@link deriveUtilityNodes}, resolving the label through the app's locale/terminology overlay. */
function utilityDefinitionToSpec(utility: UtilityDefinition, appLabelsOverlay: PluginAppLabelsOverlay): DerivedUtilitySpec {
  return { id: utility.id, label: resolveAppLabel(appLabelsOverlay, "utility", utility.id, utility.label), iconId: utility.iconId, group: utility.group ?? undefined, category: utility.category ?? "tools" };
}

/** 🧰 Stamps the owning `windowId` onto every `setActiveUtility` descriptor in a derived toolbar tree so the shell's `onAction` interceptor targets the right window regardless of which window is globally active. */
function tagSetActiveUtilityWindow(nodes: readonly UtilityNode[], windowId: string): UtilityNode[] {
  return nodes.map((node) => {
    if (node.kind === "collection") return { ...node, children: tagSetActiveUtilityWindow(node.children, windowId) };
    if (node.kind === "toggle" && "onChange" in node && node.onChange.action === SET_ACTIVE_UTILITY_ACTION_ID) {
      return { ...node, onChange: { ...node.onChange, args: { ...(node.onChange.args as object | undefined), windowId } } };
    }
    return node;
  });
}

/**
 * 🧰 Builds the window toolbar `UtilityNode[]` for one window purely from the static utility registry plus
 * the host-owned active utility id — the replacement for the deleted plugin `list-tools` sourcing. Each
 * `setActiveUtility` descriptor is tagged with `windowId` so activation is scoped to this exact window.
 */
export function resolveUtilityNodes(app: Pick<AppDefinition, "utilities" | "controllerId">, windowKind: Pick<AppWindowKindDefinition, "utilities">, activeUtilityId: string | null | undefined, windowId: string, appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY): UtilityNode[] {
  const utilities = resolveUtilities(app, windowKind);
  if (utilities.length === 0) return [];
  return tagSetActiveUtilityWindow(deriveUtilityNodes(app.controllerId, utilities.map((utility) => utilityDefinitionToSpec(utility, appLabelsOverlay)), activeUtilityId ?? undefined), windowId);
}
//#endregion 🧰UtilityRegistry

/** @emoji 💬 Builds spawned-window engagement, measures, and utility-options chrome for one window kind. */
export function spawnedWindowChromeForKind(
  kind: AppDefinition["windowKinds"][number],
  engagementsByKind: Readonly<Record<string, WindowEngagement>>,
  measuresByKind: Readonly<Record<string, readonly WindowMeasure[]>>,
  activeUtilityId: string | undefined,
  onAction: (action: ActionDescriptor) => void,
): { readonly engagement?: EngagementSpec; readonly measures: ReactNode; readonly utilityOptions: ReactNode } {
  const { measures, utilityOptions } = windowMeasuresChrome(measuresByKind[kind.id] ?? kind.options.measures, activeUtilityId, kind.id, onAction);
  return {
    engagement: windowEngagementToSpec(resolveWindowEngagement(kind, engagementsByKind), onAction),
    measures,
    utilityOptions,
  };
}

function isTreeNode(node: UiNode): node is UiTreeNode {
  return node.type === "tree";
}

export function uiNodeToTreePanelConfig(node: UiNode, onAction: (action: ActionDescriptor) => void): TreePanelConfig {
  if (isTreeNode(node)) return { ...uiTreeNodeToTreePanelConfig(node, onAction), dragAndDropController: declarativeTreeDragController(node, onAction) };
  return {
    sections: [
      {
        id: "panel.body",
        label: "",
        items: [
          {
            id: "panel.body.content",
            label: "",
            control: <ChromeAwareWindowScrollSurface className="min-h-0 flex-1">{interpretUiNode(node, { onAction })}</ChromeAwareWindowScrollSurface>,
          },
        ],
      },
    ],
  };
}

function shellTabIcon(iconId: string): React.FC<{ size?: number }> {
  return function ShellTabIcon({ size = 16 }: { size?: number }) {
    let iconName: IconName = "circle-dot";
    if (iconId === FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID) {
      iconName = "file-text";
    } else if (iconId in ICONS) {
      iconName = iconId as IconName;
    }
    return <Icon icon={iconName} size={size} />;
  };
}

/** @emoji 🌐 Resolves a chrome translation key outside hook context (tree builders run there). */
function shellLabel(key: UiTranslationKey): string {
  return resolveTranslationLabel(uiI18n.t(key)) ?? key;
}

/** @emoji 🗣️ Stable empty overlay reference so components depending on it don't re-render before the first `appLabels` fetch resolves. */
const EMPTY_APP_LABELS_OVERLAY: PluginAppLabelsOverlay = {
  windowKindLabels: {},
  panelTabLabels: {},
  modeLabels: {},
  actionLabels: {},
  utilityLabels: {},
  exampleLabels: {},
  actionArgLabels: {},
  dialogLabels: {},
  introductionLabels: {},
};

/** @emoji 🗣️ Resolves a window-kind/panel-tab/mode/action/utility/example/actionArg/dialog/introduction id's locale-aware label from the active app's overlay, falling back to the static manifest label. */
function resolveAppLabel(overlay: PluginAppLabelsOverlay, kind: "windowKind" | "panelTab" | "mode" | "action" | "utility" | "example" | "actionArg" | "dialog" | "introduction", id: string, fallback: string): string {
  const map =
    kind === "windowKind"
      ? overlay.windowKindLabels
      : kind === "panelTab"
        ? overlay.panelTabLabels
        : kind === "mode"
          ? overlay.modeLabels
          : kind === "action"
            ? overlay.actionLabels
            : kind === "utility"
              ? overlay.utilityLabels
              : kind === "example"
                ? overlay.exampleLabels
                : kind === "actionArg"
                  ? overlay.actionArgLabels
                  : kind === "dialog"
                    ? overlay.dialogLabels
                    : overlay.introductionLabels;
  return map[id] ?? fallback;
}

/** @emoji 🗣️ Resolves one action-arg's label + (for `select` controls) its options' labels from the overlay's `actionArgLabels` map, keyed `"{scopeId}.{argId}"` / `"{scopeId}.{argId}.option.{value}"`. `scopeId` is an action id for staged/palette forms or a dialog id for dialog args. */
function resolveActionArgDef(def: ActionArgDef, scopeId: string, overlay: PluginAppLabelsOverlay): ActionArgDef {
  const label = resolveAppLabel(overlay, "actionArg", `${scopeId}.${def.id}`, def.label);
  if (def.control.kind !== "select") return label === def.label ? def : { ...def, label };
  const options = def.control.options.map((option) => ({ ...option, label: resolveAppLabel(overlay, "actionArg", `${scopeId}.${def.id}.option.${option.value}`, option.label) }));
  return { ...def, label, control: { ...def.control, options } };
}

/** @emoji 🗣️ Resolves a `DialogDefinition`'s title/body/submitLabel/args from the overlay's `dialogLabels`/`actionArgLabels` maps, keyed by the dialog's own id. */
function resolveDialogDefinition(dialog: DialogDefinition, overlay: PluginAppLabelsOverlay): DialogDefinition {
  return {
    ...dialog,
    title: resolveAppLabel(overlay, "dialog", `${dialog.id}.title`, dialog.title),
    body: dialog.body ? resolveAppLabel(overlay, "dialog", `${dialog.id}.body`, dialog.body) : dialog.body,
    submitLabel: resolveAppLabel(overlay, "dialog", `${dialog.id}.submit`, dialog.submitLabel),
    args: dialog.args.map((def) => resolveActionArgDef(def, dialog.id, overlay)),
  };
}

/** @emoji 🗣️ Resolves an `IntroductionDefinition`'s title and every step's title/body from the overlay's `introductionLabels` map. */
function resolveIntroductionDefinition(introduction: IntroductionDefinition, overlay: PluginAppLabelsOverlay): IntroductionDefinition {
  return {
    title: resolveAppLabel(overlay, "introduction", "intro.title", introduction.title),
    steps: introduction.steps.map(
      (step): IntroductionStepDefinition => ({
        ...step,
        title: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.title`, step.title),
        body: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.body`, step.body),
      }),
    ),
  };
}

/** @emoji 🗣️ Resolves a terminology id's display name; chrome-known ids get a translated label, app-declared ids fall back to their raw id. */
function shellTerminologyLabel(id: string): string {
  const isChromeKnown = id === "native" || id === "reuse";
  return isChromeKnown ? shellLabel(`ui.settings.terminology.${id as UiChromeTerminologyId}`) : id;
}

/** @emoji 🎚️ Serializes async updates while retaining only the newest value requested during an in-flight update. */
export function createLatestAsyncDispatcher<T>(dispatchValue: (value: T) => unknown): (value: T) => void {
  let running = false;
  let queued: T | undefined;
  let hasQueued = false;
  const dispatchLatest = (value: T) => {
    if (running) {
      queued = value;
      hasQueued = true;
      return;
    }
    running = true;
    void Promise.resolve(dispatchValue(value)).finally(() => {
      running = false;
      if (!hasQueued) return;
      const next = queued as T;
      queued = undefined;
      hasQueued = false;
      dispatchLatest(next);
    });
  };
  return dispatchLatest;
}

/** @emoji 🎚️ Keeps a measure slider live without accumulating stale document actions behind the pointer. */
function WindowMeasureSlider({ measure, onAction }: { readonly measure: Extract<WindowMeasure, { kind: "slider" }>; readonly onAction: (action: ActionDescriptor) => unknown }) {
  const dispatchLatest = useMemo(() => createLatestAsyncDispatcher(onAction), [onAction]);

  return (
    <Slider
      id={measure.id}
      value={[measure.value]}
      min={measure.min}
      max={measure.max}
      step={measure.step}
      onValueChange={(values) => dispatchLatest({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value: values[0] ?? measure.value } })}
    />
  );
}

function renderWindowMeasure(measure: WindowMeasure, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  if (measure.kind === "group") {
    return (
      <WindowMeasureTreeGroup key={measure.id} id={measure.id} label={measure.label} defaultOpen={measure.defaultOpen}>
        {measure.children.map((child) => renderWindowMeasure(child, onAction))}
      </WindowMeasureTreeGroup>
    );
  }
  if (measure.kind === "select") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
        <Select value={measure.value} onValueChange={(value) => onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value } })}>
          <SelectTrigger id={measure.id} className="h-small w-full min-w-0" size="sm">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {measure.items.map((item) => (
              <SelectItem key={item.id} value={item.value}>
                {item.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "slider") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
        <WindowMeasureSlider measure={measure} onAction={onAction} />
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "toggle") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label}>
        <Toggle
          id={measure.id}
          pressed={measure.pressed}
          text={measure.text}
          icon={<Icon icon={measure.iconId in ICONS ? (measure.iconId as IconName) : "circle-dot"} size="small" />}
          onPressedChange={(pressed) => onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), pressed } })}
        />
      </WindowMeasureTreeLeaf>
    );
  }
  return null;
}

function windowMeasuresOverlay(measures: readonly WindowMeasure[] | undefined, onAction: (action: ActionDescriptor) => unknown): ReactNode | undefined {
  if (!measures || measures.length === 0) return undefined;
  return <WindowMeasuresTree>{measures.map((measure) => renderWindowMeasure(measure, onAction))}</WindowMeasuresTree>;
}

function SelectionUtilityOptions({ activeUtilityId, windowId, onAction }: { readonly activeUtilityId: string | undefined; readonly windowId: string; readonly onAction: (action: ActionDescriptor) => void }) {
  const selectionMethod = activeUtilityId === "selectLasso" ? "lasso" : "rectangle";

  const [selectionMode, setSelectionMode] = useState<"default" | "additive" | "subtractive" | "invertive">(() => {
    return (globalThis as any).__selectionMode || "default";
  });

  const handleModeChange = (mode: "default" | "additive" | "subtractive" | "invertive") => {
    (globalThis as any).__selectionMode = mode;
    setSelectionMode(mode);
    window.dispatchEvent(new CustomEvent("semio:selectionOptionsChanged"));
  };

  const handleMethodChange = (method: "rectangle" | "lasso") => {
    onAction({
      controllerId: "window",
      action: SET_ACTIVE_UTILITY_ACTION_ID,
      args: { windowId, utilityId: method === "lasso" ? "selectLasso" : "selectMarquee" },
    });
  };

  return (
    <div className="flex items-center gap-double">
      <div className="flex items-center gap-single">
        <span className="text-tiny text-muted-foreground uppercase tracking-wider font-semibold">Method</span>
        <ToggleGroup
          kind="single"
          value={selectionMethod}
          onValueChange={(val) => {
            if (val === "rectangle" || val === "lasso") {
              handleMethodChange(val);
            }
          }}
          items={[
            { value: "rectangle", icon: <Icon icon="square-dashed" size="small" />, text: "Rectangle" },
            { value: "lasso", icon: <Icon icon="lasso" size="small" />, text: "Lasso" },
          ]}
        />
      </div>
      <ToolbarDivider />
      <div className="flex items-center gap-single">
        <span className="text-tiny text-muted-foreground uppercase tracking-wider font-semibold">Mode</span>
        <ToggleGroup
          kind="single"
          value={selectionMode}
          onValueChange={(val) => {
            if (val === "default" || val === "additive" || val === "subtractive" || val === "invertive") {
              handleModeChange(val);
            }
          }}
          items={[
            { value: "default", text: "Selective" },
            { value: "additive", text: "Additive" },
            { value: "subtractive", text: "Subtractive" },
            { value: "invertive", text: "Invertive" },
          ]}
        />
      </div>
    </div>
  );
}

function windowMeasuresChrome(
  measures: readonly WindowMeasure[] | undefined,
  activeUtilityId: string | undefined,
  windowId: string,
  onAction: (action: ActionDescriptor) => unknown,
): { readonly measures: ReactNode | undefined; readonly utilityOptions: ReactNode | undefined } {
  const { general, utilityOptions } = partitionWindowMeasures(measures ?? [], activeUtilityId);
  return {
    measures: windowMeasuresOverlay(general, onAction),
    utilityOptions: windowMeasuresOverlay(utilityOptions, onAction),
  };
}

/** @emoji 🎓 Whether a utility node tree has a node (leaf or group) with the given id anywhere in it — used
 * to decide if this window's toolbar is the one an introduction step's `Utility` anchor targets. */
function utilityNodeTreeContainsId(nodes: readonly UtilityNode[], targetId: string): boolean {
  return nodes.some((node) => node.id === targetId || (node.kind === "collection" && utilityNodeTreeContainsId(node.children, targetId)));
}

function utilityBarNode(utilities: readonly UtilityNode[] | undefined, windowId: string, onAction: (action: ActionDescriptor) => void, revealUtilityId?: string | null): ReactNode {
  if (!utilities?.length) return undefined;
  const categories = groupUtilityNodesByCategory(utilities, UTILITY_CATEGORIES);
  if (!categories.length) return undefined;
  const grouped: UtilityNode[] = [];
  for (const node of categories) {
    if (node.kind === "collection" && (node.category === "tools" || node.category === "selection")) {
      if (node.id === "group:Select" || node.id === "group:selection" || node.label === "Select" || node.text === "Select") {
        grouped.push(...node.children);
      } else {
        for (const child of node.children) {
          if (child.kind === "collection" && (child.id === "group:Select" || child.id === "group:selection" || child.label === "Select" || child.text === "Select")) {
            grouped.push(...child.children);
          } else {
            grouped.push(child);
          }
        }
      }
    } else {
      grouped.push(node);
    }
  }
  return <UtilityTree id={`ui.toolbar.${windowId}`} utilities={grouped} onAction={onAction} direction="up" revealUtilityId={revealUtilityId} />;
}

//#region 🧰WindowActionPane
/**
 * 🎛 Renders one {@link ActionArgControl} into a STAGED form field — the crucial difference from
 * `renderUiControl` in `ui-interpreter.tsx` is that this dispatches NOTHING globally; `onChange` only
 * writes to the caller's local staged buffer. `value` is the already-resolved effective value
 * (staged ?? default ?? unset).
 */
export function renderStagedArgControl(def: ActionArgDef, value: unknown, onChange: (value: unknown) => void, disabled?: boolean): ReactElement {
  const control: ActionArgControl = def.control;
  switch (control.kind) {
    case "text":
      return <Input id={def.id} type="text" className="h-medium w-full min-w-0" value={typeof value === "string" ? value : ""} placeholder={control.placeholder} disabled={disabled} onChange={(event) => onChange(event.target.value)} />;
    case "number":
      return (
        <Input
          id={def.id}
          type="number"
          className="h-medium w-full min-w-0"
          value={value === undefined || value === null || value === "" ? "" : String(value)}
          min={control.min}
          max={control.max}
          step={control.step}
          disabled={disabled}
          onChange={(event) => onChange(event.target.value === "" ? undefined : Number(event.target.value))}
        />
      );
    case "slider": {
      const numeric = typeof value === "number" && Number.isFinite(value) ? value : control.min;
      const slider = <Slider id={def.id} className="w-full min-w-0" min={control.min} max={control.max} step={control.step ?? 1} value={[numeric]} disabled={disabled} onValueChange={(values) => onChange(values[0] ?? numeric)} />;
      if (!control.unit) return slider;
      return (
        <div className="flex w-full min-w-0 items-center gap-single">
          {slider}
          <span className="shrink-0 text-xs tabular-nums text-muted-foreground">
            {numeric} {control.unit}
          </span>
        </div>
      );
    }
    case "toggle":
      return <Toggle id={def.id} pressed={value === true} text={def.label} disabled={disabled} onPressedChange={(pressed) => onChange(pressed)} />;
    case "select":
      return (
        <Select value={typeof value === "string" && value ? value : undefined} disabled={disabled} onValueChange={(next) => onChange(next)}>
          <SelectTrigger id={def.id} className="h-medium w-full min-w-0" size="sm">
            <SelectValue placeholder={def.label} />
          </SelectTrigger>
          <SelectContent>
            {control.options.map((option, index) => (
              <SelectItem key={`${def.id}:${index}:${option.value}`} value={option.value}>
                {option.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      );
    case "vec3": {
      const tuple = Array.isArray(value) && value.length >= 3 ? (value as readonly number[]) : null;
      const axes = ["x", "y", "z"] as const;
      return (
        <div className="grid grid-cols-3 gap-single">
          {axes.map((axis, index) => (
            <Input
              key={`${def.id}.${axis}`}
              id={`${def.id}.${axis}`}
              type="number"
              className="h-medium w-full min-w-0"
              value={tuple ? String(tuple[index] ?? 0) : ""}
              placeholder={axis}
              disabled={disabled}
              onChange={(event) => {
                const parsed = Number(event.target.value);
                if (!Number.isFinite(parsed)) return;
                const next: [number, number, number] = tuple ? [tuple[0] ?? 0, tuple[1] ?? 0, tuple[2] ?? 0] : [0, 0, 0];
                next[index] = parsed;
                onChange(next);
              }}
            />
          ))}
        </div>
      );
    }
    case "iconSelect":
      return <IconSelector id={def.id} classifyIconSelectorMode={undefined} value={typeof value === "string" ? value : ""} uniform onChange={(next) => onChange(next)} />;
  }
}

/** 🧰 True when an action carries arguments and therefore stages a form instead of firing immediately (P1–P4). */
export function actionRequiresStagedForm(action: Pick<ActionDefinition, "args">): boolean {
  return (action.args?.length ?? 0) > 0;
}

/** 🧰 The decision a bound hotkey makes for one action (P4). */
/** ⌨️ Splits a declared `keys` binding (comma-separated chord alternatives, e.g. `"mod+z,ctrl+z"`) into individual chords. Shared by the action and command keydown listeners. */
export function parseKeybindingChords(keys: string): string[] {
  return keys
    .split(",")
    .map((key) => key.trim().toLowerCase())
    .filter(Boolean);
}

/** ⌨️ True when a keydown's target is a text-editing surface (input/textarea/select/contenteditable) — hotkeys never fire while the user is typing. */
export function isEditableEventTarget(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false;
  const tag = target.tagName;
  if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
  if (target.isContentEditable) return true;
  return target.closest("[contenteditable='true'], [role='textbox']") != null;
}

/** ⌨️ True when a keydown event matches one `+`-joined chord (e.g. `"mod+shift+z"`), where `mod` accepts either ctrl or meta. */
export function keyboardEventMatchesChord(event: KeyboardEvent, chord: string): boolean {
  const parts = chord.split("+").map((part) => part.trim());
  const key = parts[parts.length - 1] ?? "";
  const needsCtrl = parts.includes("ctrl") || parts.includes("meta") || parts.includes("mod");
  const needsShift = parts.includes("shift");
  const needsAlt = parts.includes("alt");
  const hasCtrl = event.ctrlKey || event.metaKey;
  if (needsCtrl !== hasCtrl) return false;
  if (needsShift !== event.shiftKey) return false;
  if (needsAlt !== event.altKey) return false;
  return event.key.toLowerCase() === key;
}

export type KeybindingIntent = { readonly kind: "fire" } | { readonly kind: "open"; readonly actionId: string } | { readonly kind: "execute"; readonly actionId: string; readonly args: Record<string, unknown> };

/**
 * ✍️ Pure P4 decision: an arg-less action fires directly; an arg-carrying action opens its staged form,
 * unless that form is already the expanded one in the active window AND validation passes, in which case
 * the hotkey executes with the merged effective args. An already-open-but-invalid form stays open.
 */
export function resolveKeybindingIntent(definition: ActionDefinition | undefined, expandedActionId: string | null, stagedArgs: Readonly<Record<string, unknown>>): KeybindingIntent {
  if (!definition || !actionRequiresStagedForm(definition)) return { kind: "fire" };
  if (expandedActionId === definition.id) {
    const effective = effectiveActionArgs(definition.args, stagedArgs);
    if (missingRequiredArgs(definition.args, effective).length === 0) return { kind: "execute", actionId: definition.id, args: effective };
  }
  return { kind: "open", actionId: definition.id };
}

/** 🧰 Pure P5 activation decision: an empty request, or re-requesting the already-active utility, deactivates (null); otherwise the requested utility becomes active. */
export function resolveUtilityActivation(current: string | null | undefined, requested: string): string | null {
  return requested === "" || (current ?? null) === requested ? null : requested;
}

/** 🎛 Props for the per-window Action rail body (P1/P2). */
export type WindowActionPaneProps = {
  readonly windowId: string;
  readonly controllerId: string;
  readonly actions: readonly ActionDefinition[];
  readonly expandedActionId: string | null;
  readonly stagedArgsByKey: Readonly<Record<string, Readonly<Record<string, unknown>>>>;
  readonly disabled: boolean;
  readonly onExpandedChange: (actionId: string | null) => void;
  readonly onStageArg: (actionId: string, argId: string, value: unknown) => void;
  readonly onResetArgs: (actionId: string) => void;
  readonly onExecute: (descriptor: ActionDescriptor) => void;
};

/**
 * 🎛 The per-window Actions rail body (P1/P2). Zero-arg actions ARE the execute button; arg-carrying
 * actions expand a locally-buffered staged form — nothing dispatches on edit, effective value is
 * `staged ?? default ?? unset`, Execute is enabled only when every required arg has an effective value,
 * fires exactly ONE `ActionDescriptor` with the merged args, and keeps the staged values afterward.
 * When `disabled` (an active utility with `allowsActionsWhileActive === false`), every row renders disabled.
 */
export function WindowActionPane(props: WindowActionPaneProps): ReactElement {
  const { windowId, controllerId, actions, expandedActionId, stagedArgsByKey, disabled, onExpandedChange, onStageArg, onResetArgs, onExecute } = props;
  return (
    <div data-slot="window-action-pane" className="flex min-w-0 flex-col gap-single p-single">
      {actions.map((action) => {
        if (!actionRequiresStagedForm(action)) {
          return (
            <Button
              key={action.id}
              id={`${windowId}-action-${action.id}`}
              text={action.label}
              icon={action.iconId && action.iconId in ICONS ? (action.iconId as IconName) : "play"}
              disabled={disabled}
              onClick={() => onExecute({ controllerId, action: action.id })}
            />
          );
        }
        const expanded = expandedActionId === action.id;
        const staged = stagedArgsByKey[actionStageKey(windowId, action.id)] ?? {};
        const effective = effectiveActionArgs(action.args, staged);
        const missing = missingRequiredArgs(action.args, effective);
        return (
          <div key={action.id} data-slot="window-action-row" className={cn("flex min-w-0 flex-col rounded-md border", borderElementClass)}>
            <Button id={`${windowId}-action-${action.id}-disclosure`} text={`${action.label}…`} icon={expanded ? "chevron-down" : "chevron-right"} onClick={() => onExpandedChange(expanded ? null : action.id)} />
            {expanded ? (
              <div data-slot="window-action-form" className="flex min-w-0 flex-col gap-single p-single">
                {action.args.map((def) => (
                  <Field key={def.id} id={`${windowId}-action-${action.id}-arg-${def.id}`} label={def.label} description={def.description} required={def.required}>
                    {renderStagedArgControl(def, effective[def.id], (value) => onStageArg(action.id, def.id, value), disabled)}
                  </Field>
                ))}
                <div className="flex items-center gap-single">
                  <Button
                    id={`${windowId}-action-${action.id}-execute`}
                    text={shellLabel("ui.common.execute")}
                    icon="check"
                    disabled={disabled || missing.length > 0}
                    onClick={() => onExecute({ controllerId, action: action.id, args: effectiveActionArgs(action.args, staged) })}
                  />
                  <Button id={`${windowId}-action-${action.id}-reset`} text={shellLabel("ui.common.reset")} icon="undo" onClick={() => onResetArgs(action.id)} />
                </div>
              </div>
            ) : null}
          </div>
        );
      })}
    </div>
  );
}

/** 🧰 Slice of the {@link ActionPaneState} the {@link windowActionPaneNode} builder reads. */
type ActionPaneSlice = Pick<ActionPaneState, "expandedByWindowId" | "stagedArgsByKey" | "activeUtilityByWindowId">;

/**
 * 🧰 Sibling of {@link utilityBarNode}: resolves a window kind's panel-eligible actions and returns a
 * bound {@link WindowActionPane}, or `undefined` when the window has no resolved actions (so the rail
 * chip never renders). Rows render disabled while an active utility gates actions
 * (`allowsActionsWhileActive === false`).
 */
function windowActionPaneNode(
  app: AppDefinition,
  windowKind: AppWindowKindDefinition,
  windowId: string,
  actionPane: ActionPaneSlice,
  onAction: (action: ActionDescriptor) => void,
  dispatch: (action: ShellAction) => void,
  appLabelsOverlay: PluginAppLabelsOverlay = EMPTY_APP_LABELS_OVERLAY,
): ReactNode {
  const resolvedActions = resolveWindowActions(app, windowKind);
  if (resolvedActions.length === 0) return undefined;
  const actions = resolvedActions.map((action) => ({
    ...action,
    label: resolveAppLabel(appLabelsOverlay, "action", action.id, action.label),
    args: action.args.map((def) => resolveActionArgDef(def, action.id, appLabelsOverlay)),
  }));
  const activeUtilityId = actionPane.activeUtilityByWindowId[windowId] ?? null;
  const activeUtility = activeUtilityId ? (app.utilities ?? []).find((utility) => utility.id === activeUtilityId) : undefined;
  const disabled = Boolean(activeUtility && activeUtility.allowsActionsWhileActive === false);
  return (
    <WindowActionPane
      windowId={windowId}
      controllerId={app.controllerId}
      actions={actions}
      expandedActionId={actionPane.expandedByWindowId[windowId] ?? null}
      stagedArgsByKey={actionPane.stagedArgsByKey}
      disabled={disabled}
      onExpandedChange={(actionId) => dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId, value: actionId })}
      onStageArg={(actionId, argId, value) => dispatch({ type: "STAGE_ACTION_ARG", windowId, actionId, argId, value })}
      onResetArgs={(actionId) => dispatch({ type: "RESET_ACTION_ARGS", windowId, actionId })}
      onExecute={onAction}
    />
  );
}
//#endregion 🧰WindowActionPane

//#region 🎛CommandRegistry
/** 🎛 Where a resolved command came from — drives palette/footer category grouping and dispatch routing. */
export type ResolvedCommand = {
  readonly definition: CommandDefinition;
  readonly source: { readonly kind: "os" } | { readonly kind: "plugin" } | { readonly kind: "app" } | { readonly kind: "mode"; readonly modeId: string };
};

/**
 * 🎛 Aggregates every command visible for the current session: os built-ins, the active session's
 * plugin-scope commands, the app's App-scope commands, and Mode-scope commands referenced by the
 * active mode's `commands` refs. There are no window-level commands (see `CommandScope`) — unlike
 * `resolveWindowActions`/`resolveUtilities`, this never takes a window kind.
 */
export function resolveCommands(
  osCommands: readonly CommandDefinition[],
  activePluginManifest: Pick<PluginManifest, "commands"> | null | undefined,
  app: Pick<AppDefinition, "commands" | "modes"> | null | undefined,
  activeModeId: string,
): ResolvedCommand[] {
  const resolved: ResolvedCommand[] = osCommands.map((definition) => ({ definition, source: { kind: "os" as const } }));
  for (const definition of activePluginManifest?.commands ?? []) {
    resolved.push({ definition, source: { kind: "plugin" as const } });
  }
  if (!app) return resolved;
  const activeMode = (app.modes as readonly AppModeDefinition[] | undefined)?.find((mode) => mode.id === activeModeId);
  const modeCommandIds = new Set(activeMode?.commands ?? []);
  for (const definition of app.commands ?? []) {
    if (definition.scope === "app") resolved.push({ definition, source: { kind: "app" as const } });
    else if (definition.scope === "mode" && modeCommandIds.has(definition.id)) resolved.push({ definition, source: { kind: "mode" as const, modeId: activeModeId } });
  }
  return resolved;
}

/** 🎛 Chrome-known command category ids that already have a `ui.settings.tab.*` translation key. */
const CHROME_KNOWN_COMMAND_CATEGORIES = new Set(["general", "mode", "expertise", "app", "appearance", "layout", "language", "terminology", "theme"]);

/** 🎛 Loose title-case for an open-set command category id (e.g. "appearance" -> "Appearance"). Falls back to this for app/plugin-invented categories that have no fixed framework vocabulary entry. */
function titleizeCommandCategory(category: string): string {
  return category.replace(/[-_]+/g, " ").replace(/\b\w/g, (char) => char.toUpperCase());
}

/** 🎛 Resolves a command category's display label, reusing the existing `ui.settings.tab.*` keys for chrome-known ids and falling back to a loose title-case for open-set app/plugin categories. */
function commandCategoryLabel(category: string): string {
  return CHROME_KNOWN_COMMAND_CATEGORIES.has(category) ? shellLabel(`ui.settings.tab.${category as "general" | "mode" | "expertise" | "app" | "appearance" | "layout" | "language" | "terminology" | "theme"}`) : titleizeCommandCategory(category);
}

/** 🎛 Ordered, deduped category tabs for the footer command panel, derived from whatever commands actually resolved. */
export function commandCategories(commands: readonly ResolvedCommand[]): { readonly id: string; readonly label: string }[] {
  const seen = new Set<string>();
  const categories: { readonly id: string; readonly label: string }[] = [];
  for (const { definition } of commands) {
    if (seen.has(definition.category)) continue;
    seen.add(definition.category);
    categories.push({ id: definition.category, label: commandCategoryLabel(definition.category) });
  }
  return categories;
}

function selectCommandArg(id: string, label: string, options: readonly { readonly value: string; readonly label: string }[]): ActionArgDef {
  return { id, label, control: { kind: "select", options: options.map((option) => ({ ...option })) }, required: true };
}

/**
 * 🎛 Os-level built-in commands — app introduction/theme/layout/locale/appearance/expertise,
 * `scope: "os"`, handled
 * locally by the shell (never routed to a plugin). Rebuilt via `useMemo` since the theme and
 * terminology option lists are live state.
 */
export function buildOsCommands(themeList: readonly UiTheme[], terminologies: readonly string[], hasIntroduction: boolean, locks: ResolvedShellLocks = EMPTY_SHELL_LOCKS): CommandDefinition[] {
  const lockedCommandIds = new Set<string>([
    ...(locks.appearance ? ["os.setAppearance"] : []),
    ...(locks.themeId ? ["os.setThemeId"] : []),
    ...(locks.locale ? ["os.setLocale"] : []),
    ...(locks.terminology ? ["os.setTerminology"] : []),
  ]);
  return [
    ...(hasIntroduction
      ? [{ id: "os.introduceApp", label: shellLabel("ui.command.introduceApp"), scope: "os" as const, category: "app", inPalette: true, args: [] }]
      : []),
    {
      id: "os.setAppearance",
      label: shellLabel("ui.command.setAppearance"),
      scope: "os",
      category: "appearance",
      inPalette: true,
      args: [
        selectCommandArg("appearance", shellLabel("ui.settings.tab.appearance"), [
          { value: "system", label: shellLabel("ui.settings.appearance.system") },
          { value: "light", label: shellLabel("ui.settings.appearance.light") },
          { value: "dark", label: shellLabel("ui.settings.appearance.dark") },
        ]),
      ],
    },
    {
      id: "os.setThemeId",
      label: shellLabel("ui.command.setTheme"),
      scope: "os",
      category: "appearance",
      inPalette: true,
      args: [
        selectCommandArg(
          "themeId",
          shellLabel("ui.settings.tab.theme"),
          themeList.map((theme) => ({ value: theme.id, label: theme.label || theme.id })),
        ),
      ],
    },
    {
      id: "os.setLayout",
      label: shellLabel("ui.command.setLayout"),
      scope: "os",
      category: "layout",
      inPalette: true,
      args: [
        selectCommandArg("layout", shellLabel("ui.settings.tab.layout"), [
          { value: "desktop", label: shellLabel("settings.layout.desktop") },
          { value: "tablet", label: shellLabel("settings.layout.tablet") },
        ]),
      ],
    },
    { id: "os.toggleCompact", label: shellLabel("ui.command.toggleCompact"), scope: "os", category: "layout", inPalette: true, args: [] },
    { id: "os.resetDock", label: shellLabel("ui.settings.resetDock"), scope: "os", category: "layout", inPalette: true, args: [] },
    {
      id: "os.setLocale",
      label: shellLabel("ui.command.setLocale"),
      scope: "os",
      category: "language",
      inPalette: true,
      args: [
        selectCommandArg("locale", shellLabel("ui.settings.tab.language"), [
          { value: "en", label: shellLabel("ui.settings.language.en") },
          { value: "de", label: shellLabel("ui.settings.language.de") },
        ]),
      ],
    },
    {
      id: "os.setTerminology",
      label: shellLabel("ui.command.setTerminology"),
      scope: "os",
      category: "language",
      inPalette: true,
      args: [
        selectCommandArg(
          "terminology",
          shellLabel("ui.settings.tab.terminology"),
          terminologies.map((id) => ({ value: id, label: shellTerminologyLabel(id) })),
        ),
      ],
    },
    {
      id: "os.setExpertise",
      label: shellLabel("ui.command.setExpertise"),
      scope: "os",
      category: "general",
      inPalette: true,
      args: [
        selectCommandArg("expertise", shellLabel("ui.settings.tab.expertise"), [
          { value: "beginner", label: shellLabel("settings.expertise.beginner") },
          { value: "normal", label: shellLabel("settings.expertise.normal") },
          { value: "expert", label: shellLabel("settings.expertise.expert") },
        ]),
      ],
    },
  ].filter((command) => !lockedCommandIds.has(command.id));
}

/** 🎛 Os-scope command ids that are handled locally by the shell — mirrors {@link buildOsCommands}. */
export function dispatchOsCommand(
  commandId: string,
  args: Record<string, unknown> | undefined,
  dispatch: (action: ShellAction) => void,
  dockLayoutStore: DockLayoutStore,
  dockUiStateStore: DockUiStateStore,
  locks: ResolvedShellLocks = EMPTY_SHELL_LOCKS,
): void {
  switch (commandId) {
    case "os.introduceApp":
      dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
      return;
    case "os.setAppearance":
      if (locks.appearance) return;
      dispatch({ type: "SET_UI_APPEARANCE", value: (args?.appearance as ElementsSurfaceAppearance) ?? "system" });
      return;
    case "os.setThemeId":
      if (locks.themeId) return;
      if (typeof args?.themeId === "string") dispatch({ type: "SET_UI_THEME_ID", value: args.themeId });
      return;
    case "os.setLayout":
      dispatch({ type: "SET_UI_LAYOUT", value: (args?.layout as UiChromeLayout) ?? "desktop" });
      return;
    case "os.toggleCompact":
      dispatch({ type: "SET_UI_COMPACT", value: (current) => !current });
      return;
    case "os.resetDock":
      dispatch({ type: "RESET_DOCK" });
      dockLayoutStore.reset();
      dockUiStateStore.reset();
      return;
    case "os.setLocale":
      if (locks.locale) return;
      if (typeof args?.locale === "string") {
        setUiLocale(args.locale as UiLocale);
        dispatch({ type: "SET_UI_LOCALE", value: args.locale as UiLocale });
      }
      return;
    case "os.setTerminology":
      if (locks.terminology) return;
      if (typeof args?.terminology === "string") dispatch({ type: "SET_UI_TERMINOLOGY", value: args.terminology });
      return;
    case "os.setExpertise":
      if (typeof args?.expertise === "string") dispatch({ type: "SET_UI_EXPERTISE", value: args.expertise as Expertise });
      return;
    default:
      return;
  }
}

/** @emoji 🎛 Fallback icon for every command-category leaf — categories are open-set strings any plugin/app/mode author can invent, so there's no per-category icon metadata to key off (unlike the framework's own Workbench/Details/Display/Settings categories). */
const COMMAND_CATEGORY_ICON = shellTabIcon("wrench");

/**
 * 🎛 One category's command list (and, if a command is expanded, its staged arg form) as a `TreePanelConfig`
 * — the content a category `PanelTabLeaf` resolves to. A zero-arg command's row fires immediately on click
 * (a plain fire-and-forget tree row, same pattern as {@link groupNamedLayoutsToTreeItems}'s layout rows —
 * no `selectedIds`/`onSelectionChange` override, so it takes `Tree`'s default single-select highlight after
 * firing, same as clicking a Display→Layout row does). An arg-carrying command's row toggles `expandedCommandId`
 * itself (kept as its own exclusive, bespoke state — not `Tree`'s per-row `openStates`, which isn't naturally
 * exclusive across sibling rows) and, when expanded, a synthetic form section (one row per arg, `control`
 * holding the staged input, replacing the old `Field` wrapper since `TreeDataItem` already renders label +
 * description + control in the same two-column layout) is prepended so it renders above the command list —
 * `Tree` reverses top-level `sections` for `direction="up"` (bottom anchors), threaded here via `flowFromAnchor`/
 * `FlowProvider`/`useFlow` down from the hosting `Panel`, not any manual reversal in this function.
 */
export function buildCommandCategoryTree(
  commands: readonly ResolvedCommand[],
  expandedCommandId: string | null,
  stagedArgsByCommandId: Readonly<Record<string, Readonly<Record<string, unknown>>>>,
  onExecute: (entry: ResolvedCommand, executeArgs?: Record<string, unknown>) => void,
  onToggleExpanded: (commandId: string | null) => void,
  onStageArg: (commandId: string, argId: string, value: unknown) => void,
  onResetArgs: (commandId: string) => void,
): TreePanelConfig {
  const argCarryingCommands = commands.filter((entry) => entry.definition.args.length > 0);
  const autoExpandedSingleton = argCarryingCommands.length === 1 ? argCarryingCommands[0] : undefined;
  const expanded =
    (expandedCommandId ? commands.find((entry) => entry.definition.id === expandedCommandId) : undefined) ?? autoExpandedSingleton;
  const effectiveExpandedId = expanded?.definition.id ?? null;
  const sections: TreeDataSection[] = [];
  if (expanded && expanded.definition.args.length > 0) {
    const staged = stagedArgsByCommandId[expanded.definition.id] ?? {};
    const effective = effectiveActionArgs(expanded.definition.args, staged);
    const missing = missingRequiredArgs(expanded.definition.args, effective);
    sections.push({
      id: `command.category.${expanded.definition.category}.form`,
      items: expanded.definition.args.map(
        (def): TreeDataItem => ({
          id: `command.${expanded.definition.id}.arg.${def.id}`,
          label: def.label,
          description: def.description,
          control: renderStagedArgControl(def, effective[def.id], (value) => onStageArg(expanded.definition.id, def.id, value)),
        }),
      ),
      actions: [
        {
          id: `command-${expanded.definition.id}-execute`,
          icon: <Icon icon="check" size="small" />,
          text: shellLabel("ui.common.execute"),
          disabled: missing.length > 0,
          onClick: () => onExecute(expanded, effective),
        },
        {
          id: `command-${expanded.definition.id}-reset`,
          icon: <Icon icon="undo" size="small" />,
          text: shellLabel("ui.common.reset"),
          onClick: () => onResetArgs(expanded.definition.id),
        },
      ],
    });
  }
  const listCommands = commands.filter((entry) => entry.definition.id !== effectiveExpandedId);
  if (listCommands.length > 0) {
    sections.push({
      id: "command.category.list",
      items: listCommands.map((entry): TreeDataItem => {
        const argCarrying = entry.definition.args.length > 0;
        const icon = entry.definition.iconId && entry.definition.iconId in ICONS ? <Icon icon={entry.definition.iconId as IconName} size="small" /> : undefined;
        if (!argCarrying) return { id: `command.${entry.definition.id}`, label: entry.definition.label, icon, onClick: () => onExecute(entry) };
        return {
          id: `command.${entry.definition.id}`,
          label: `${entry.definition.label}…`,
          icon: <Icon icon={expandedCommandId === entry.definition.id ? "chevron-down" : "chevron-up"} size="small" />,
          onClick: () => onToggleExpanded(expandedCommandId === entry.definition.id ? null : entry.definition.id),
        };
      }),
    });
  }
  return { sections };
}

/**
 * 🎛 One `PanelTabLeaf` per resolved command category — consumers wrap these under the Command branch
 * (`FRAMEWORK_CATEGORY_COMMAND_ID`) on `defaultDock.anchors["bottom-middle"]` so the folded chrome shows
 * a single expandable Command toggle. The command palette's fold/active-category/size/persistence is the
 * generic per-anchor `Panel` state (see `buildPanelProps`); this only builds the category tab leaves.
 * Content is a *lazy* `resolveTree` (mirrors {@link createFrameworkDisplayPanelTabs}'s windows tab) so
 * this array — and therefore `defaultDock`'s own memo — never depends on `expandedCommandId`/
 * `stagedArgsByCommandId`, which change on every keystroke while staging a command argument; `resolveTree`
 * reads those fresh off refs at render time instead.
 */
export function buildCommandCategoryTabs(
  resolvedCommands: readonly ResolvedCommand[],
  categories: readonly { readonly id: string; readonly label: string }[],
  expandedCommandIdRef: React.RefObject<string | null>,
  stagedArgsByCommandIdRef: React.RefObject<Readonly<Record<string, Readonly<Record<string, unknown>>>>>,
  onCommand: (source: ResolvedCommand["source"], commandId: string, args?: Record<string, unknown>) => void,
  dispatch: (action: ShellAction) => void,
): PanelTabNode[] {
  return categories.map((category) => {
    const categoryCommands = resolvedCommands.filter((entry) => entry.definition.category === category.id);
    return singleTreeLeaf({
      id: `command.category.${category.id}`,
      icon: COMMAND_CATEGORY_ICON,
      name: category.label,
      tree: {
        resolveTree: () =>
          buildCommandCategoryTree(
            categoryCommands,
            expandedCommandIdRef.current,
            stagedArgsByCommandIdRef.current,
            (entry, executeArgs) => onCommand(entry.source, entry.definition.id, executeArgs),
            (commandId) => dispatch({ type: "SET_COMMAND_EXPANDED", value: commandId }),
            (commandId, argId, value) => dispatch({ type: "STAGE_COMMAND_ARG", commandId, argId, value }),
            (commandId) => dispatch({ type: "RESET_COMMAND_ARGS", commandId }),
          ),
      },
    });
  });
}
//#endregion 🎛CommandRegistry

/** @emoji 🐢 Structural equality over plain JSON-shaped values (the shape every `UiNode`/`WindowEngagement`/`WindowMeasure` plugin payload takes) — no cycles, no non-JSON types. */
function uiJsonDeepEqual(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (typeof a !== "object" || typeof b !== "object" || a === null || b === null) return false;
  if (Array.isArray(a) !== Array.isArray(b)) return false;
  if (Array.isArray(a) && Array.isArray(b)) {
    if (a.length !== b.length) return false;
    for (let index = 0; index < a.length; index += 1) {
      if (!uiJsonDeepEqual(a[index], b[index])) return false;
    }
    return true;
  }
  const aRecord = a as Record<string, unknown>;
  const bRecord = b as Record<string, unknown>;
  const aKeys = Object.keys(aRecord);
  const bKeys = Object.keys(bRecord);
  if (aKeys.length !== bKeys.length) return false;
  for (const key of aKeys) {
    if (!Object.prototype.hasOwnProperty.call(bRecord, key)) return false;
    if (!uiJsonDeepEqual(aRecord[key], bRecord[key])) return false;
  }
  return true;
}

/**
 * @emoji 🐢 Reuses `previous`'s object identity when it's structurally equal to `next` — every plugin
 * `render()`/`utilities()`/`windowEngagements()`/`windowMeasures()` call re-parses a fresh JSON payload
 * every time, even when nothing about that body actually changed (e.g. a camera-only or selection-only
 * action still returns byte-identical panel/utility JSON). Without this, every downstream `React.memo`
 * (see `InterpretedUiNode`) sees a new prop reference every render and can never bail.
 */
export function preserveJsonIdentity<T>(previous: T | undefined, next: T): T {
  return previous !== undefined && uiJsonDeepEqual(previous, next) ? previous : next;
}

/**
 * @emoji 🐢 Builds a `Record<string, V>` from `entries`, reusing `prev`'s per-key value reference where
 * `preserveJsonIdentity` finds no structural change, and reusing `prev` itself (the whole record) when
 * no key actually changed — so a no-op action's `dispatch` doesn't hand `windowUiByKind`/etc. a new
 * object reference and cascade an unmemoizable re-render through every downstream consumer.
 */
export function mergeRecordPreservingIdentity<V>(prev: Readonly<Record<string, V>>, entries: readonly (readonly [string, V])[]): Readonly<Record<string, V>> {
  const next: Record<string, V> = {};
  let changed = Object.keys(prev).length !== entries.length;
  for (const [key, value] of entries) {
    const preserved = preserveJsonIdentity(prev[key], value);
    next[key] = preserved;
    if (preserved !== prev[key]) changed = true;
  }
  return changed ? next : prev;
}

//#region UiRefresh
/** @emoji 🐢 One cached section value keyed by `${section}:${key}` (e.g. `window:2d-overview`, `engagements`) — the hash is what gets sent back to the plugin next time so it can skip re-serializing unchanged content. */
export type UiRefreshCache = Map<string, { readonly hash: string; readonly value: unknown }>;

function uiRefreshWantsWindow(scope: UiDirtyScope, bodyKey: string): boolean {
  return scope.kind === "full" || (scope.kind === "partial" && (scope.windowBodies ?? []).includes(bodyKey));
}
function uiRefreshWantsPanel(scope: UiDirtyScope, bodyKey: string): boolean {
  return scope.kind === "full" || (scope.kind === "partial" && (scope.panelBodies ?? []).includes(bodyKey));
}
function uiRefreshWantsFlag(scope: UiDirtyScope, flag: "engagements" | "measures" | "labels"): boolean {
  return scope.kind === "full" || (scope.kind === "partial" && scope[flag] === true);
}

/**
 * @emoji 🐢 Builds one batched `refresh-ui` request restricted to `scope` — `null` when the scope
 * resolves to nothing worth fetching (`none`, or a `partial` whose fields all miss this app's actual
 * bodies/kinds). Every requested entry carries the host's cached hash so the plugin can omit payloads
 * for sections that didn't change.
 */
export function buildUiRefreshRequest(
  scope: UiDirtyScope,
  windowKinds: readonly { readonly id: string; readonly bodyKey: string }[],
  panelTabLeaves: readonly { readonly kind: PanelTabKind; readonly bodyKey?: string }[],
  viewState: PluginViewState,
  cache: UiRefreshCache,
): PluginUiRefreshRequest | null {
  if (scope.kind === "none") return null;
  const windows = windowKinds.filter((kind) => uiRefreshWantsWindow(scope, kind.bodyKey)).map((kind) => ({ key: kind.id, bodyKey: kind.bodyKey, hash: cache.get(`window:${kind.id}`)?.hash }));
  const panels = panelTabLeaves
    .filter((tab): tab is { readonly kind: string; readonly bodyKey: string } => Boolean(tab.bodyKey) && uiRefreshWantsPanel(scope, tab.bodyKey!))
    .map((tab) => ({ key: panelTabKindId(tab.kind), bodyKey: tab.bodyKey, hash: cache.get(`panel:${panelTabKindId(tab.kind)}`)?.hash }));
  const engagements = uiRefreshWantsFlag(scope, "engagements") ? { hash: cache.get("engagements")?.hash } : undefined;
  const measures = uiRefreshWantsFlag(scope, "measures") ? { hash: cache.get("measures")?.hash } : undefined;
  const labels = uiRefreshWantsFlag(scope, "labels") ? { hash: cache.get("labels")?.hash } : undefined;
  if (windows.length === 0 && panels.length === 0 && !engagements && !measures && !labels) return null;
  return { viewState, windows, panels, engagements, measures, labels };
}

/** @emoji 🐢 Writes every changed section (`value !== undefined`) from a `refresh-ui` response into `cache`; unchanged sections are left as-is since the cached value is still current. */
function applyUiRefreshSectionsToCache(cache: UiRefreshCache, prefix: string, entries: readonly PluginUiRefreshSectionResponse[] | undefined): void {
  for (const entry of entries ?? []) {
    if (entry.value !== undefined) cache.set(`${prefix}:${entry.key}`, { hash: entry.hash, value: entry.value });
  }
}

export function applyUiRefreshResponseToCache(cache: UiRefreshCache, response: PluginUiRefreshResponse): void {
  applyUiRefreshSectionsToCache(cache, "window", response.windows);
  applyUiRefreshSectionsToCache(cache, "panel", response.panels);
  if (response.engagements?.value !== undefined) cache.set("engagements", { hash: response.engagements.hash, value: response.engagements.value });
  if (response.measures?.value !== undefined) cache.set("measures", { hash: response.measures.hash, value: response.measures.value });
  if (response.labels?.value !== undefined) cache.set("labels", { hash: response.labels.hash, value: response.labels.value });
}
//#endregion UiRefresh
//#endregion ShellHelpers

//#region Boot
export async function bootFrameworkOs(options: FrameworkOsBootOptions = {}): Promise<void> {
  const root = document.getElementById(options.rootId ?? "root");
  if (!root) throw new Error("missing #root");
  const locks = resolveShellLocks(options.locks);
  bootstrapElementsSurfaceChromeDocument(locks.appearance ?? readStoredUiChromeAppearance());
  createRoot(root).render(<FrameworkOsShell pluginFilter={options.plugin} plugins={options.plugins ?? DEFAULT_PLUGIN_REGISTRY} appId={options.appId} locks={locks} />);
}
//#endregion Boot

//#region ErrorBoundary
class ShellRenderErrorBoundary extends Component<{ readonly children: ReactNode }, { readonly hasError: boolean; readonly message: string }> {
  constructor(props: { readonly children: ReactNode }) {
    super(props);
    this.state = { hasError: false, message: "" };
  }

  static getDerivedStateFromError(error: Error) {
    return { hasError: true, message: error.message };
  }

  render() {
    if (this.state.hasError) {
      return (
        <p className="p-double text-sm text-destructive" role="alert">
          {shellLabel("ui.common.renderError")}: {this.state.message}
        </p>
      );
    }
    return this.props.children;
  }
}
//#endregion ErrorBoundary

//#region FrameworkOsShell
export function FrameworkOsShell({
  pluginFilter,
  plugins,
  appId,
  locks: locksProp,
}: {
  readonly pluginFilter?: string;
  readonly plugins: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
  readonly appId?: string;
  readonly locks?: ResolvedShellLocks;
}) {
  const studioMode = isStudioMode(pluginFilter);
  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const locks = locksProp ?? EMPTY_SHELL_LOCKS;
  const [shellState, dispatch] = useReducer(shellReducer, undefined, () => initialShellState({ pluginFilter, plugins, locks }));
  const { loadedPlugins, session, error } = shellState.pluginRuntime;
  const { windowUiByKind, windowEngagementsByKind, windowMeasuresByKind, panelUiByKey, appLabelsOverlay } = shellState.windowUi;
  const { spawnedWindowUi, spawnedWindowEngagements, spawnedWindowMeasures } = shellState.spawnedWindow;
  const { foldedByWindowId: actionPaneFoldedByWindowId, expandedByWindowId: actionPaneExpandedByWindowId, stagedArgsByKey: actionPaneStagedArgsByKey, activeUtilityByWindowId } = shellState.actionPane;
  const { expandedCommandId, stagedArgsByCommandId: commandStagedArgsByCommandId } = shellState.commandPanel;
  const { panels, dockOverride, panelPathMemory, treeOpenStates, activeWindowId, shellLayout, activeExampleId, mobilePanelPath, extraWindowInstances } = shellState.layout;
  const { searchOpen, findOpen, introductionStepIndex, dialog: overlayDialog } = shellState.overlays;
  const { uiAppearance, uiLayout, uiCompact, uiExpertise, uiLocale, uiTerminology, uiThemeId, uiCustomThemes, uiThemeDraft } = shellState.uiPrefs;
  const { syncBackboneUri, syncCardKind, syncDraftPath, syncStatusByDocumentId } = shellState.sync;
  const importStudioInputRef = useRef<HTMLInputElement>(null);
  const refreshGenerationRef = useRef(0);
  const spawnedRefreshGenerationRef = useRef(0);
  const contributorInstancesRef = useRef<Map<string, number>>(new Map());
  const layoutSeedKeyRef = useRef<string | null>(null);
  const noExampleResetInstanceIdRef = useRef<number | null>(null);
  const extraWindowCounterRef = useRef(0);
  // 🐢 Per-instance content-hash cache for the batched `refresh-ui` call, keyed by the same
  // `pluginId:appId:instanceId` triple as `layoutSeedKeyRef` — cleared on session switch below.
  const uiRefreshCacheRef = useRef<UiRefreshCache>(new Map());
  // 🐢 Same idea for the studio-mode spawned-instance view, keyed by spawned instanceId — cleared when
  // the spawned instance itself changes (tracked via `spawnedLayoutSeedRef`).
  const spawnedUiRefreshCacheRef = useRef<UiRefreshCache>(new Map());
  const spawnedLayoutSeedRef = useRef<string | null>(null);
  const openStudioIdRef = useRef<string | null>(null);
  const openInstanceIdRef = useRef<string | null>(null);
  const sessionRef = useRef<ActiveSession | null>(null);
  const uiDevice: ElementsSurfaceDevice = mobile ? "mobile" : uiLayout;
  const uiTheme: UiTheme = useMemo(() => {
    if (uiThemeDraft) return uiThemeDraft;
    const found = builtinUiThemes().find((t) => t.id === uiThemeId) ?? uiCustomThemes[uiThemeId];
    return found ?? readStoredUiChromeThemeSnapshot() ?? semioTheme();
  }, [uiThemeId, uiCustomThemes, uiThemeDraft]);
  /** 🧵 Lazily-created worker running `backbone-worker.ts` — one per shell instance, reused across `openDocument` calls. */
  const backboneWorkerRef = useRef<Worker | null>(null);
  /** 🖋️ Stable per-tab actor id for hub `Hello`/presence frames and op-origin filtering. */
  const shellActorIdRef = useRef<string>(`client-${Math.random().toString(36).slice(2)}`);
  /** 🗂️ Which session/plugin owns each open document id, so incoming worker events route correctly. */
  const openDocumentSessionsRef = useRef<Map<string, { session: ActiveSession; plugin: PluginWasmHandle }>>(new Map());

  const ensureBackboneWorker = useCallback((): Worker => {
    if (backboneWorkerRef.current) return backboneWorkerRef.current;
    const worker = new Worker(new URL("../../product/os/core/js/backbone-worker.ts", import.meta.url), { type: "module" });
    worker.onmessage = (messageEvent: MessageEvent<BackboneWorkerResponse>) => {
      const message = messageEvent.data;
      if (message.kind !== "event") return;
      const entry = openDocumentSessionsRef.current.get(message.documentId);
      if (!entry) return;
      const { event } = message;
      if (event.kind === "status") {
        dispatch({ type: "SET_SYNC_STATUS_FOR_DOCUMENT", documentId: message.documentId, status: { persisted: event.persisted, pendingOps: event.pendingOps, remote: event.remote } });
      } else if (event.kind === "presence") {
        // 👥 Presence now flows only through here (the deleted `presence:` URI hack used to be the
        // source) — plugins keep reading it via `ViewState.presencePeersJson`.
        const peersJson = JSON.stringify(event.peers.map((peer) => ({ clientId: peer.actor, name: peer.label ?? peer.actor, selectionCount: 0 })));
        dispatch({
          type: "SET_SESSION",
          value: (current) => (current && current.instanceId === entry.session.instanceId ? { ...current, viewState: { ...current.viewState, presencePeersJson: peersJson } } : current),
        });
      } else if (event.kind === "remoteOps" && entry.plugin.applyOperations) {
        void entry.plugin.applyOperations(entry.session.instanceId, JSON.stringify(event.envelopes));
      } else if (event.kind === "snapshotReplaced" && entry.plugin.loadAppDocument) {
        void entry.plugin.loadAppDocument(entry.session.instanceId, event.envelopeJson);
      } else if (event.kind === "conflict") {
        console.warn("[os-shell] sync conflict", message.documentId, event.message);
      }
    };
    backboneWorkerRef.current = worker;
    return worker;
  }, []);

  const { uri: shellUri, canGoBack, canGoForward, canGoUp, goBack, goForward, goUp, navigate: navigateHistory } = useUIHistory("/", studioMode);

  const namedLayoutStore = useMemo(() => new NamedLayoutStore(session?.app.id ?? "framework-os", createBrowserStoragePort()), [session?.app.id]);
  const dockLayoutStore = useMemo(() => new DockLayoutStore(createBrowserStoragePort(), session?.app.id), [session?.app.id]);
  const dockUiStateStore = useMemo(() => new DockUiStateStore(createBrowserStoragePort(), session?.app.id), [session?.app.id]);

  const registry = useMemo(() => {
    const expanded = expandPluginRegistry(plugins, pluginFilter ? resolvePluginRegistryId(pluginFilter) : undefined, studioMode);
    if (studioMode) return expanded;
    return pluginFilter ? expanded : plugins;
  }, [pluginFilter, plugins, studioMode]);

  // 🐢 Memoized on the raw `panelJson` string (not `session` object identity, which churns every
  // action) so a `session` refresh that leaves `panelJson` untouched reuses the same parsed `panel`
  // object — a prerequisite for any downstream `useMemo`/`React.memo` keyed on `panel` to bail.
  const panel = useMemo(() => (session ? parsePanelState(session.viewState) : null), [session?.viewState.panelJson]);
  const activeAppTitle = appDocumentLabel(panel?.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId)?.document ?? session?.app.document ?? []);

  useEffect(() => {
    sessionRef.current = session;
  }, [session]);

  // 🎓 Auto-starts an app's introduction the first time it launches on this device; replaying stays
  // available afterward via the shell-owned Introduce App command.
  useEffect(() => {
    if (!session?.app.introduction) return;
    if (readStoredIntroductionSeen(session.app.id)) return;
    dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
  }, [session?.app.id, session?.app.introduction]);

  // 🧰 Refs so `refreshUi`/`onAction`/`applyHostEffects` can read the current host-owned active utility and
  // active window without re-creating those callbacks on every utility switch.
  const activeUtilityByWindowIdRef = useRef(activeUtilityByWindowId);
  activeUtilityByWindowIdRef.current = activeUtilityByWindowId;
  const activeWindowIdRef = useRef(activeWindowId);
  activeWindowIdRef.current = activeWindowId;
  const actionPaneExpandedByWindowIdRef = useRef(actionPaneExpandedByWindowId);
  actionPaneExpandedByWindowIdRef.current = actionPaneExpandedByWindowId;
  const actionPaneStagedArgsByKeyRef = useRef(actionPaneStagedArgsByKey);
  actionPaneStagedArgsByKeyRef.current = actionPaneStagedArgsByKey;
  const introductionStepIndexRef = useRef(introductionStepIndex);
  introductionStepIndexRef.current = introductionStepIndex;
  // 🎛️ So the command-category leaves' lazily-resolved tree content (built once per resolved-commands
  // change, not per keystroke — see `buildCommandCategoryTabs`) can read the latest expand/staged-arg
  // state without becoming a `defaultDock` memo dependency, which would otherwise persist-write the dock
  // skeleton on every keystroke while staging a command argument.
  const expandedCommandIdRef = useRef(expandedCommandId);
  expandedCommandIdRef.current = expandedCommandId;
  const commandStagedArgsByCommandIdRef = useRef(commandStagedArgsByCommandId);
  commandStagedArgsByCommandIdRef.current = commandStagedArgsByCommandId;

  /** 🧰 Overlays the active window's host-owned `activeUtilityId` onto a view state at plugin-call time. */
  const injectActiveUtility = useCallback((viewState: ViewState, windowId?: string | null): ViewState => {
    const key = windowId ?? activeWindowIdRef.current;
    const utilityId = key ? (activeUtilityByWindowIdRef.current[key] ?? undefined) : undefined;
    return viewState.activeUtilityId === utilityId ? viewState : { ...viewState, activeUtilityId: utilityId };
  }, []);

  useEffect(() => {
    dispatch({ type: "SET_SYNC_BACKBONE_URI", value: null });
    dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
  }, [panel?.activeSpawnedId, session, studioMode]);

  useEffect(() => {
    const worker = backboneWorkerRef.current;
    return () => worker?.terminate();
  }, []);

  useEffect(() => {
    if (activeAppTitle) document.title = activeAppTitle;
  }, [activeAppTitle]);

  useEffect(() => {
    let cancelled = false;
    void (async () => {
      try {
        const settled = await Promise.allSettled(registry.map((entry) => loadPluginModuleResilient(entry.pluginId, entry.moduleUrl)));
        const loaded = settled.flatMap((result, index) => {
          if (result.status === "fulfilled" && result.value) return [result.value];
          if (result.status === "rejected") {
            console.error(`[DEBUG] plugin rejected: ${registry[index]?.pluginId}`, result.reason);
          }
          return [];
        });
        if (loaded.length === 0) throw new Error(shellLabel("ui.common.noPluginsLoaded"));
        if (cancelled) return;
        const loadedState = loaded.map((handle) => ({ handle, manifest: handle.manifest }));
        dispatch({ type: "SET_LOADED_PLUGINS", value: loadedState });

        if (studioMode) {
          const sPlugin = loadedState.find((entry) => entry.handle.pluginId === "s");
          const sApp = sPlugin?.manifest.apps.find((app) => app.id === S_HOME_APP_ID) ?? sPlugin?.manifest.apps[0];
          if (!sPlugin || !sApp) throw new Error("s studio plugin missing home app");
          const programs = buildStudioPrograms(loadedState);
          const panelState = buildStudioPanelState(programs, []);
          const instanceId = await sPlugin.handle.createApp(sApp.id);
          const viewState: ViewState = {
            activeModeId: sApp.defaultModeId ?? sApp.modes[0]?.id,
            activeWindowKindId: sApp.windowKinds[0]?.id,
            panelJson: panelJsonFromState(panelState),
          };
          dispatch({ type: "SET_SESSION", value: { pluginId: sPlugin.handle.pluginId, instanceId, app: sApp, viewState } });
          dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: sApp.windowKinds[0]?.id ?? null });
          return;
        }

        const registryPluginId = pluginFilter ? resolvePluginRegistryId(pluginFilter) : undefined;
        const primary = (registryPluginId ? loaded.find((entry) => entry.pluginId === registryPluginId) : undefined) ?? loaded[0];
        const primaryApp = appId
          ? (() => {
              const found = primary?.manifest.apps.find((app) => app.id === appId);
              if (!found) throw new Error(`appId "${appId}" does not resolve to any app in the loaded plugin manifest`);
              return found;
            })()
          : (() => {
              const defaultAppId = pluginFilter ? resolvePlaygroundDefaultAppId(pluginFilter) : undefined;
              return (defaultAppId ? primary?.manifest.apps.find((app) => app.id === defaultAppId) : undefined) ?? primary?.manifest.apps[0];
            })();
        if (primary && primaryApp) {
          const instanceId = await primary.createApp(primaryApp.id);
          dispatch({
            type: "SET_SESSION",
            value: {
              pluginId: primary.pluginId,
              instanceId,
              app: primaryApp,
              viewState: {
                activeModeId: primaryApp.defaultModeId ?? primaryApp.modes[0]?.id,
                activeWindowKindId: primaryApp.windowKinds[0]?.id,
              },
            },
          });
          dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: primaryApp.windowKinds[0]?.id ?? null });
        }
      } catch (bootError) {
        if (!cancelled) {
          console.error("[DEBUG] framework os boot failed", bootError);
          dispatch({ type: "SET_ERROR", value: bootError instanceof Error ? bootError.message : String(bootError) });
        }
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [registry, studioMode, appId]);

  const findPluginForAction = useCallback(
    (action: ActionDescriptor) => {
      const byController = loadedPlugins.find((entry) => entry.manifest.apps.some((app) => app.controllerId === action.controllerId));
      if (byController) return byController;
      return loadedPlugins.find((entry) => entry.handle.pluginId === session?.pluginId);
    },
    [loadedPlugins, session?.pluginId],
  );

  const refreshUi = useCallback(
    async (nextSession: ActiveSession, scopeArg: UiDirtyScope = { kind: "full" }) => {
      if (scopeArg.kind === "none") return;
      const generation = ++refreshGenerationRef.current;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === nextSession.pluginId)?.handle;
      if (!plugin) return;
      const layoutSeedKey = `${nextSession.pluginId}:${nextSession.app.id}:${nextSession.instanceId}`;
      // 🐢 A session switch invalidates every cached hash from the previous instance — force a full
      // fetch regardless of what scope this particular call was given.
      let scope = scopeArg;
      if (layoutSeedKeyRef.current !== layoutSeedKey) {
        uiRefreshCacheRef.current = new Map();
        scope = { kind: "full" };
      }
      const cache = uiRefreshCacheRef.current;
      const contributionsJson = buildContributionsJson(loadedPlugins.map((entry) => ({ pluginId: entry.handle.pluginId, manifest: entry.manifest })));
      const viewState: ViewState = injectActiveUtility({ ...nextSession.viewState, contributionsJson, locale: uiLocale, terminology: uiTerminology });
      const panelTabLeaves = flattenPanelTabLeaves(nextSession.app.panelTabs);
      // 🐢 One batched, hash-conditional round trip replaces the old ~12 sequential
      // render/utilities/windowEngagements/windowMeasures/appLabels calls — the plugin omits payloads for
      // any section whose hash still matches what `cache` already holds.
      const request = buildUiRefreshRequest(scope, nextSession.app.windowKinds, panelTabLeaves, viewState, cache);
      if (request) {
        const response = await plugin.refreshUi(nextSession.instanceId, request);
        if (generation !== refreshGenerationRef.current) return;
        const slotContext = {
          plugins: new Map(loadedPlugins.map((entry) => [entry.handle.pluginId, entry.handle])),
          contributorInstances: contributorInstancesRef.current,
          viewState,
        };
        // Resolve external slots on freshly-changed window/panel bodies only, before caching them, so a
        // later no-op refresh reuses the already-resolved cached value instead of re-resolving.
        const resolveIfChanged = async (entry: PluginUiRefreshSectionResponse): Promise<PluginUiRefreshSectionResponse> => (entry.value !== undefined ? { ...entry, value: await resolveExternalSlots(entry.value as UiNode, slotContext) } : entry);
        const [resolvedWindows, resolvedPanels] = await Promise.all([Promise.all((response.windows ?? []).map(resolveIfChanged)), Promise.all((response.panels ?? []).map(resolveIfChanged))]);
        if (generation !== refreshGenerationRef.current) return;
        applyUiRefreshResponseToCache(cache, { ...response, windows: resolvedWindows, panels: resolvedPanels });
      }
      // 🐢 Merge-with-identity-preservation: unrequested/unchanged sections keep exactly the object
      // reference already in `cache` (dispatched from a prior refresh), so `mergeRecordPreservingIdentity`
      // bails on them via reference equality — this is what lets `InterpretedUiNode`'s `React.memo` (and
      // `modeWindows`'s `useMemo`) skip reconciling the whole shell on every interaction.
      dispatch({
        type: "SET_WINDOW_UI_BY_KIND",
        value: (current) =>
          mergeRecordPreservingIdentity(
            current,
            nextSession.app.windowKinds.map((kind) => [kind.id, (cache.get(`window:${kind.id}`)?.value as UiNode | undefined) ?? current[kind.id] ?? { type: "text", value: `${shellLabel("ui.common.loading")}: ${kind.id}` }] as const),
          ),
      });
      const dynamicEngagements = (cache.get("engagements")?.value as Readonly<Record<string, WindowEngagement>> | undefined) ?? {};
      dispatch({
        type: "SET_WINDOW_ENGAGEMENTS_BY_KIND",
        value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicEngagements)),
      });
      const dynamicMeasures = (cache.get("measures")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {};
      dispatch({
        type: "SET_WINDOW_MEASURES_BY_KIND",
        value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicMeasures)),
      });
      const appLabelsOverlay = normalizeAppLabelsOverlay(cache.get("labels")?.value as Partial<PluginAppLabelsOverlay> | undefined);
      dispatch({ type: "SET_APP_LABELS_OVERLAY", value: (current) => preserveJsonIdentity(current, appLabelsOverlay) });
      dispatch({
        type: "SET_PANEL_UI_BY_KEY",
        value: (current) =>
          mergeRecordPreservingIdentity(
            current,
            panelTabLeaves
              .filter((tab) => tab.bodyKey)
              .map((tab) => [panelTabKindId(tab.kind), (cache.get(`panel:${panelTabKindId(tab.kind)}`)?.value as UiNode | undefined) ?? current[panelTabKindId(tab.kind)] ?? { type: "text", value: shellLabel("ui.common.loading") }] as const),
          ),
      });
      const windowIds = nextSession.app.windowKinds.map((kind) => kind.id);
      if (layoutSeedKeyRef.current !== layoutSeedKey) {
        layoutSeedKeyRef.current = layoutSeedKey;
        dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: [] });
        extraWindowCounterRef.current = 0;
        dispatch({ type: "SET_SHELL_LAYOUT", value: convertFrameworkLayoutToModeLayout(nextSession.app.defaultLayout, windowIds, appLabelsOverlay) });
        const defaultWindowId = findDefaultActiveWindowKindId(nextSession.app.defaultLayout, nextSession.app.windowKinds);
        if (defaultWindowId) dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: defaultWindowId });
        else if (windowIds[0]) dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: windowIds[0] });
      }
    },
    [injectActiveUtility, loadedPlugins, uiLocale, uiTerminology],
  );

  /** @emoji 🗣️ Keeps already-built window titles (workbench layout, extra spawned windows) in sync with the app-labels overlay on every locale/terminology switch — `refreshUi` only rebuilds `shellLayout` from scratch on a session change, so an existing session's baked-in titles would otherwise go stale. */
  useEffect(() => {
    dispatch({ type: "SET_SHELL_LAYOUT", value: (current) => (current ? retitleWindowLayoutNode(current, appLabelsOverlay) : current) });
    dispatch({
      type: "SET_EXTRA_WINDOW_INSTANCES",
      value: (current) => current.map((entry) => ({ ...entry, title: resolveAppLabel(appLabelsOverlay, "windowKind", entry.windowKindId, entry.title) })),
    });
  }, [appLabelsOverlay]);

  const refreshSpawnedUi = useCallback(
    async (spawned: SpawnedAppEntry, viewState: ViewState, scopeArg: UiDirtyScope = { kind: "full" }) => {
      if (scopeArg.kind === "none") return;
      const generation = ++spawnedRefreshGenerationRef.current;
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId);
      const plugin = pluginEntry?.handle;
      const app = pluginEntry?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
      if (!plugin || !app) {
        console.warn("[os-shell] refreshSpawnedUi: plugin/app unavailable", { pluginId: spawned.pluginId, appId: spawned.appId });
        dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: { type: "text", value: `Plugin unavailable: ${spawned.pluginId}/${spawned.appId}` } as UiNode });
        dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
        dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
        return;
      }
      const spawnedSeed = `${spawned.pluginId}:${spawned.appId}:${spawned.instanceId}`;
      if (spawnedLayoutSeedRef.current !== spawnedSeed) {
        spawnedLayoutSeedRef.current = spawnedSeed;
        spawnedUiRefreshCacheRef.current = new Map();
      }
      const cache = spawnedUiRefreshCacheRef.current;
      const contributionsJson = buildContributionsJson(loadedPlugins.map((entry) => ({ pluginId: entry.handle.pluginId, manifest: entry.manifest })));
      const fullViewState: ViewState = injectActiveUtility({ ...viewState, contributionsJson, locale: uiLocale, terminology: uiTerminology }, spawned.id);
      const bodyKey = resolveCanvasBodyKey(app);
      // 🐢 A spawned instance's view is a single body + utilities + engagements + measures (no panels, no
      // labels) — that's already the minimal grouping, so there is no narrower-than-full "partial" scope
      // worth expressing here; only `none` (handled above) short-circuits the request.
      const singleWindowKind = [{ id: bodyKey, bodyKey }];
      const request = buildUiRefreshRequest({ kind: "full" }, singleWindowKind, [], fullViewState, cache);
      if (request) {
        const response = await plugin.refreshUi(spawned.instanceId, request);
        if (generation !== spawnedRefreshGenerationRef.current) return;
        applyUiRefreshResponseToCache(cache, response);
      }
      const ui = (cache.get(`window:${bodyKey}`)?.value as UiNode | undefined) ?? { type: "text", value: shellLabel("ui.common.loading") };
      const dynamicEngagements = (cache.get("engagements")?.value as Readonly<Record<string, WindowEngagement>> | undefined) ?? {};
      const dynamicMeasures = (cache.get("measures")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {};
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: (current: UiNode | null) => preserveJsonIdentity(current ?? undefined, ui) });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: dynamicEngagements });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: dynamicMeasures });
    },
    [injectActiveUtility, loadedPlugins, uiLocale, uiTerminology],
  );

  // 🐢 Keyed on the pluginId/app/instance triple (not `session` object identity) so this only fires on
  // a genuine session switch (app open/spawn/instance change) — every other action already calls
  // `refreshUi` explicitly via `applyHostEffects`, and re-running it here too on every `session` object
  // churn was a second, redundant full-shell refresh cascade per interaction.
  const sessionIdentityKey = session ? `${session.pluginId}:${session.app.id}:${session.instanceId}` : null;
  useEffect(() => {
    const current = sessionRef.current;
    if (!current) return;
    void refreshUi(current).catch((renderError) => {
      console.error("[DEBUG] render failed", renderError);
      dispatch({ type: "SET_ERROR", value: renderError instanceof Error ? renderError.message : String(renderError) });
    });
  }, [loadedPlugins, refreshUi, sessionIdentityKey]);

  useEffect(() => {
    if (!studioMode || !session) {
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: null });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
      return;
    }
    const activeSpawned = panel?.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
    if (!activeSpawned) {
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: null });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
      return;
    }
    void refreshSpawnedUi(activeSpawned, session.viewState).catch((renderError) => {
      console.error("[DEBUG] spawned render failed", renderError);
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: null });
    });
  }, [loadedPlugins, panel, refreshSpawnedUi, session, studioMode]);

  const updateStudioPanel = useCallback((panelState: StudioPanelState) => {
    dispatch({
      type: "SET_SESSION",
      value: (current) => {
        if (!current) return current;
        return { ...current, viewState: { ...current.viewState, panelJson: panelJsonFromState(panelState) } };
      },
    });
  }, []);

  const switchToSApp = useCallback(
    async (appId: string, viewState?: ViewState): Promise<ActiveSession | null> => {
      const sPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === "s");
      const app = sPlugin?.manifest.apps.find((candidate) => candidate.id === appId);
      if (!sPlugin || !app) return null;
      if (session?.pluginId === sPlugin.handle.pluginId && session.app.id === appId) {
        if (!viewState) return session;
        const nextSession: ActiveSession = { ...session, viewState };
        dispatch({ type: "SET_SESSION", value: nextSession });
        await refreshUi(nextSession);
        return nextSession;
      }
      const instanceId = await sPlugin.handle.createApp(app.id);
      const programs = buildStudioPrograms(loadedPlugins);
      const nextViewState: ViewState = viewState ?? {
        activeModeId: app.defaultModeId ?? app.modes[0]?.id,
        activeWindowKindId: app.windowKinds[0]?.id,
        panelJson: panelJsonFromState(buildStudioPanelState(programs, [])),
      };
      const nextSession: ActiveSession = { pluginId: sPlugin.handle.pluginId, instanceId, app, viewState: nextViewState };
      dispatch({ type: "SET_SESSION", value: nextSession });
      dispatch({
        type: "SET_SHELL_LAYOUT",
        value: convertFrameworkLayoutToModeLayout(
          app.defaultLayout,
          app.windowKinds.map((kind) => kind.id),
          appLabelsOverlay,
        ),
      });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: findDefaultActiveWindowKindId(app.defaultLayout, app.windowKinds) ?? app.windowKinds[0]?.id ?? null });
      if (appId === S_HOME_APP_ID) {
        openStudioIdRef.current = null;
        openInstanceIdRef.current = null;
      }
      await refreshUi(nextSession);
      return nextSession;
    },
    [loadedPlugins, refreshUi, session, appLabelsOverlay],
  );

  const syncSpawnedPluginDocument = useCallback(async (plugin: PluginWasmHandle, app: AppDefinition, pluginInstanceId: number, documentJson: string, viewState: ViewState) => {
    try {
      const document = JSON.parse(documentJson) as Record<string, unknown>;
      await plugin.handleAction(pluginInstanceId, JSON.stringify({ controllerId: app.controllerId, action: "setDocument", args: { document } }), viewState);
    } catch (syncError) {
      console.error("[DEBUG] spawned plugin document sync failed", syncError);
    }
  }, []);

  const ensureSpawnedPlugin = useCallback(
    async (program: StudioProgramEntry, label?: string, osInstanceId?: string, documentJson?: string) => {
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      if (!pluginEntry || !session) return;
      const app = pluginEntry.manifest.apps.find((candidate) => candidate.id === program.appId);
      const currentPanel = parsePanelState(session.viewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
      const existing = osInstanceId ? currentPanel.spawnedApps.find((entry) => entry.id === osInstanceId) : currentPanel.spawnedApps.find((entry) => entry.appId === program.appId && entry.pluginId === program.pluginId);
      if (existing) {
        if (documentJson && app) {
          await syncSpawnedPluginDocument(pluginEntry.handle, app, existing.instanceId, documentJson, session.viewState);
        }
        updateStudioPanel(buildStudioPanelState(currentPanel.programs, currentPanel.spawnedApps, currentPanel.activePanelTab, existing.id));
        return;
      }
      const instanceId = await pluginEntry.handle.createApp(program.appId);
      if (documentJson && app) {
        await syncSpawnedPluginDocument(pluginEntry.handle, app, instanceId, documentJson, session.viewState);
      }
      const spawnedId = osInstanceId ?? `${program.pluginId}-${instanceId}`;
      updateStudioPanel(
        buildStudioPanelState(
          currentPanel.programs,
          [
            ...currentPanel.spawnedApps,
            {
              id: spawnedId,
              pluginId: program.pluginId,
              instanceId,
              appId: program.appId,
              label: label ?? program.label,
              document: program.document,
            },
          ],
          currentPanel.activePanelTab,
          spawnedId,
        ),
      );
    },
    [loadedPlugins, session, syncSpawnedPluginDocument, updateStudioPanel],
  );

  /**
   * 🐚 Consumes a plugin action's typed `requestedEffects: HostEffect[]` (WS-D's `InvocationResponse`) —
   * replaces the deleted `processPluginOps` string-matching. The legacy `setDocument`-mirror
   * backbone-write block is gone entirely: document content sync now flows through
   * `openDocument`/`closeDocument`'s worker-backed `DocumentHost` lifecycle, not a per-op JS mirror.
   */
  const applyHostEffects = useCallback(
    async (effects: readonly HostEffect[], baseSession: ActiveSession, uiScope: UiDirtyScope = { kind: "full" }) => {
      let nextViewState = baseSession.viewState;
      for (const effect of effects) {
        if (effect === "requestSync") continue;
        if ("setPanel" in effect) {
          nextViewState = { ...nextViewState, panelJson: effect.setPanel.panelJson };
          continue;
        }
        if ("setActiveUtility" in effect) {
          // 🧰 A plugin programmatically switched utility: mirror it into the host-owned store slice and,
          // when it targets the active window, into the view state fed to the follow-up refresh.
          const { windowKindId, utilityId } = effect.setActiveUtility;
          dispatch({ type: "SET_ACTIVE_UTILITY", windowId: windowKindId, utilityId: utilityId || null });
          if (windowKindId === activeWindowIdRef.current) nextViewState = { ...nextViewState, activeUtilityId: utilityId || undefined };
          continue;
        }
        if ("openDialog" in effect) {
          // 🗨️ Renders from the active `baseSession.app` — dialogs opened by spawned plugin
          // instances are v1-out-of-scope, mirroring the introduction's active-session-only scope.
          const { dialogId, args } = effect.openDialog;
          if (baseSession.app.dialogs?.some((entry) => entry.id === dialogId)) {
            dispatch({ type: "SET_DIALOG", value: { dialogId, seedArgs: args as Record<string, unknown> | undefined } });
          } else {
            console.error(`[os-shell] openDialog: app ${baseSession.app.id} declares no dialog "${dialogId}"`);
          }
          continue;
        }
        if ("navigate" in effect) {
          navigateHistory(effect.navigate.uri);
          continue;
        }
        if ("downloadMediaExport" in effect) {
          const { filename, mimeType, data, encoding } = effect.downloadMediaExport;
          downloadMediaExport(filename, mimeType, data, encoding);
          continue;
        }
        if ("iconRenderExport" in effect) {
          for (const item of effect.iconRenderExport.items) {
            try {
              const result = await iconRenderPort.render(item.request as Parameters<typeof iconRenderPort.render>[0]);
              downloadDataUrl(item.filename, result.dataUrl);
            } catch (error) {
              console.error(`icon render export failed for ${item.filename}`, error);
            }
          }
          continue;
        }
        if ("requestFileOpen" in effect) {
          const { accept, readAs, importAction } = effect.requestFileOpen;
          const opened = await requestFileOpen(accept || ".json,.spatial.json", readAs);
          if (opened) {
            const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
            if (pluginEntry) {
              const importResponse = await pluginEntry.handle.handleAction(
                baseSession.instanceId,
                JSON.stringify({
                  controllerId: baseSession.app.controllerId,
                  action: importAction,
                  args: { payload: opened.contents, name: opened.name },
                }),
                baseSession.viewState,
              );
              await applyHostEffects(importResponse.requestedEffects ?? [], baseSession, resolveUiDirtyScope(importResponse.uiScope));
            }
          }
          continue;
        }
        if ("spawnPluginInstance" in effect) {
          const { programId, appId, osInstanceId, label, documentJson } = effect.spawnPluginInstance;
          const currentPanel = parsePanelState(nextViewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
          const program = currentPanel.programs.find((entry) => entry.programId === programId && entry.appId === appId) ?? currentPanel.programs.find((entry) => entry.programId === programId);
          if (program) await ensureSpawnedPlugin(program, label, osInstanceId, documentJson);
          continue;
        }
        if ("openPluginInstance" in effect) {
          const { programId, appId, osInstanceId } = effect.openPluginInstance;
          const currentPanel = parsePanelState(nextViewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
          const program = currentPanel.programs.find((entry) => entry.programId === programId && entry.appId === appId) ?? currentPanel.programs.find((entry) => entry.programId === programId);
          if (program) {
            await ensureSpawnedPlugin(program, undefined, osInstanceId, undefined);
            if (osInstanceId && openStudioIdRef.current) {
              openInstanceIdRef.current = osInstanceId;
              navigateHistory(`/studios/${openStudioIdRef.current}/instances/${osInstanceId}`);
            }
          } else {
            console.warn(
              "[os-shell] openPluginInstance: no program matches",
              { programId, appId },
              "available:",
              currentPanel.programs.map((entry) => `${entry.programId}/${entry.appId}`),
            );
          }
          continue;
        }
      }
      const nextSession = { ...baseSession, viewState: nextViewState };
      const isSpawnedPluginSession = studioMode && session && baseSession.pluginId !== session.pluginId;
      dispatch({
        type: "SET_SESSION",
        value: (current) => {
          if (!current) return nextSession;
          if (isSpawnedPluginSession) return current.viewState === nextViewState ? current : { ...current, viewState: nextViewState };
          if (current.instanceId !== nextSession.instanceId) return current;
          // 🐢 Preserve `current`'s identity when the viewState didn't actually change — otherwise every
          // action mints a new `session` object, which cascades into a new `onAction` identity, which
          // busts every memo keyed on it (windows, panels, the boot-refresh effect below) even when
          // nothing about the session changed.
          return current.viewState === nextViewState ? current : { ...current, viewState: nextViewState };
        },
      });
      if (isSpawnedPluginSession) {
        const spawned = parsePanelState(nextViewState)?.spawnedApps.find((entry) => entry.pluginId === baseSession.pluginId && entry.instanceId === baseSession.instanceId);
        if (spawned) await refreshSpawnedUi(spawned, nextViewState, uiScope);
      } else if (session?.instanceId === nextSession.instanceId || baseSession.instanceId === nextSession.instanceId) {
        await refreshUi(nextSession, uiScope);
      }
    },
    [ensureSpawnedPlugin, loadedPlugins, navigateHistory, refreshSpawnedUi, refreshUi, session, studioMode],
  );

  const applyShellUri = useCallback(
    async (uri: string, preservedViewState?: ViewState) => {
      const currentSession = sessionRef.current;
      if (!studioMode || !currentSession || loadedPlugins.length === 0) return;
      const path = uri.split("?")[0] ?? "/";
      const studioPath = parseStudioShellPath(path);
      const sPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === "s")?.handle;
      if (!sPlugin) return;
      if (!studioPath) {
        openStudioIdRef.current = null;
        openInstanceIdRef.current = null;
        if (currentSession.app.id !== S_HOME_APP_ID) await switchToSApp(S_HOME_APP_ID, preservedViewState);
        return;
      }
      const { studioId, instanceId } = studioPath;
      const studioSession = currentSession.app.id === S_PLAY_APP_ID ? currentSession : await switchToSApp(S_PLAY_APP_ID, preservedViewState);
      if (!studioSession) return;
      if (openStudioIdRef.current !== studioId) {
        openStudioIdRef.current = studioId;
        openInstanceIdRef.current = null;
        await sPlugin.handleAction(studioSession.instanceId, JSON.stringify({ controllerId: S_PLAY_CONTROLLER_ID, action: "openStudio", args: { studioId } }), studioSession.viewState);
        await refreshUi(studioSession);
      }
      if (openInstanceIdRef.current === (instanceId ?? null)) return;
      openInstanceIdRef.current = instanceId ?? null;
      if (instanceId) {
        const response = await sPlugin.handleAction(studioSession.instanceId, JSON.stringify({ controllerId: S_PLAY_CONTROLLER_ID, action: "openInstance", args: { instanceId } }), studioSession.viewState);
        await applyHostEffects(response.requestedEffects ?? [], studioSession, resolveUiDirtyScope(response.uiScope));
      } else {
        const response = await sPlugin.handleAction(studioSession.instanceId, JSON.stringify({ controllerId: S_PLAY_CONTROLLER_ID, action: "closeFocusedInstance" }), studioSession.viewState);
        const currentPanel = parsePanelState(studioSession.viewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
        updateStudioPanel(buildStudioPanelState(currentPanel.programs, currentPanel.spawnedApps, currentPanel.activePanelTab, undefined));
        await applyHostEffects(response.requestedEffects ?? [], studioSession, resolveUiDirtyScope(response.uiScope));
      }
    },
    [applyHostEffects, loadedPlugins, refreshUi, studioMode, switchToSApp, updateStudioPanel],
  );

  useEffect(() => {
    if (!studioMode || loadedPlugins.length === 0) return;
    void applyShellUri(shellUri).catch((uriError) => {
      console.error("[DEBUG] shell uri apply failed", uriError);
    });
  }, [applyShellUri, loadedPlugins.length, shellUri, studioMode]);

  const resolveSyncTargetSession = useCallback((): ActiveSession | null => {
    if (!session) return null;
    if (studioMode && panel?.activeSpawnedId) {
      const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
      if (spawned) {
        const app = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
        if (app) return { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: session.viewState };
      }
    }
    return session;
  }, [loadedPlugins, panel, session, studioMode]);

  /**
   * 🧵 `openDocument(ref, bindings)` — replaces `attachSyncBackbone`'s URI-string mirror. Spins up (or
   * reuses) `backbone-worker.ts`, tells it to open the document, subscribes to its postMessage events,
   * and calls the plugin instance's `attachBackbone`/`loadAppDocument` WIT-exported methods (WS-D) so
   * the plugin-side store starts pumping through the same logical channel. The `actor://<documentId>`
   * uri mirrors `framework/sync`'s `ChannelBackbone::pair` convention on the Rust side.
   *
   * Full loop note: this wires the main-thread half of the contract. The remaining hop — the
   * sandboxed plugin's own `backbone-send`/`backbone-poll` WIT host-import calls relaying through its
   * dedicated plugin worker, through this main thread, into `backbone-worker.ts` — is
   * `framework/product/os/dev/script.ts`'s `pluginWorkerSource` responsibility (dev workflow, deferred
   * per this session's priority order if not otherwise completed); see that file's own notes.
   */
  const openDocument = useCallback(
    async (ref: { readonly documentId: string; readonly schema: string }, bindings: readonly PersistenceBinding[]) => {
      const targetSession = resolveSyncTargetSession();
      if (!targetSession) return;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === targetSession.pluginId)?.handle;
      if (!plugin) return;
      const worker = ensureBackboneWorker();
      openDocumentSessionsRef.current.set(ref.documentId, { session: targetSession, plugin });
      const request: BackboneWorkerRequest = {
        kind: "open",
        documentId: ref.documentId,
        schema: ref.schema,
        bindings,
        watchExternal: true,
        actor: shellActorIdRef.current,
      };
      worker.postMessage(request);
      const uri = `actor://${ref.documentId}`;
      if (plugin.attachBackbone) await plugin.attachBackbone(targetSession.instanceId, uri);
      dispatch({ type: "SET_SYNC_BACKBONE_URI", value: uri });
      dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
    },
    [loadedPlugins, resolveSyncTargetSession],
  );

  const closeDocument = useCallback((documentId: string) => {
    const entry = openDocumentSessionsRef.current.get(documentId);
    if (entry?.plugin.detachBackbone) void entry.plugin.detachBackbone(entry.session.instanceId);
    openDocumentSessionsRef.current.delete(documentId);
    const request: BackboneWorkerRequest = { kind: "close", documentId };
    backboneWorkerRef.current?.postMessage(request);
  }, []);

  /** @deprecated superseded by {@link openDocument}; kept as a thin URI-parsing adapter only for the
   * existing sync-card UI (`onAction`'s `attach` handler below), which still collects a single uri
   * from file/folder/remote pickers — translates that uri into an `OsDocumentRef` + `PersistenceBinding`. */
  const attachSyncBackbone = useCallback(
    async (uri: string) => {
      const targetSession = resolveSyncTargetSession();
      if (!targetSession) return;
      const documentId = syncDocumentId(targetSession, panel, studioMode);
      const bindings: PersistenceBinding[] = uri.startsWith("remote://")
        ? (() => {
            const rest = uri.slice("remote://".length);
            const slash = rest.indexOf("/");
            const baseUrl = slash > 0 ? `http://${rest.slice(0, slash)}` : `http://${rest}`;
            return [{ kind: "hub", baseUrl }];
          })()
        : uri.startsWith("folder://")
          ? [{ kind: "folder", path: uri.slice("folder://".length) }]
          : uri.startsWith("file://")
            ? [{ kind: "folder", path: uri.slice("file://".length).replace(/\/[^/]*$/, "") }]
            : [];
      await openDocument({ documentId, schema: targetSession.app.document.join(".") }, bindings);
    },
    [openDocument, panel, resolveSyncTargetSession, studioMode],
  );

  const detachSyncBackbone = useCallback(() => {
    if (syncBackboneUri) closeDocument(syncBackboneUri.replace(/^actor:\/\//, ""));
    dispatch({ type: "SET_SYNC_BACKBONE_URI", value: null });
    dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
  }, [closeDocument, syncBackboneUri]);

  const spawnProgram = useCallback(
    async (program: StudioProgramEntry) => {
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      if (!pluginEntry || !session) return;
      const instanceId = await pluginEntry.handle.createApp(program.appId);
      const currentPanel = parsePanelState(session.viewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
      const spawnedId = `${program.pluginId}-${instanceId}`;
      updateStudioPanel(
        buildStudioPanelState(
          currentPanel.programs,
          [
            ...currentPanel.spawnedApps,
            {
              id: spawnedId,
              pluginId: program.pluginId,
              instanceId,
              appId: program.appId,
              label: program.label,
              document: program.document,
            },
          ],
          currentPanel.activePanelTab,
          spawnedId,
        ),
      );
    },
    [loadedPlugins, session, updateStudioPanel],
  );

  const onAction = useCallback(
    (action: ActionDescriptor) => {
      if (!session) return;

      // 🎓 First-run walkthrough (mirrors setActiveUtility below): fully shell-intercepted, resets
      // playback to the first step, never forwarded to the plugin.
      if (action.action === START_INTRODUCTION_ACTION_ID) {
        dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
        return;
      }
      const introductionStep = introductionStepIndexRef.current != null ? (session.app.introduction?.steps[introductionStepIndexRef.current] ?? null) : null;
      const advanceIntroductionStep = () => {
        const stepIndex = introductionStepIndexRef.current;
        const introduction = session.app.introduction;
        if (stepIndex == null || !introduction) return;
        if (stepIndex >= introduction.steps.length - 1) {
          dispatch({ type: "SET_INTRODUCTION_STEP", value: null });
          writeStoredIntroductionSeen(session.app.id);
        } else {
          dispatch({ type: "SET_INTRODUCTION_STEP", value: stepIndex + 1 });
        }
      };

      // 🧰 Utility activation (P5): host-owned session state, never a document op. Re-clicking the active
      // utility (or an empty utilityId) deactivates. We resolve the target window from the descriptor's tagged
      // `windowId` (see `tagSetActiveUtilityWindow`), falling back to the active window, update the store,
      // then forward the resolved utility to the plugin so it can clear/prepare scratch.
      if (action.action === SET_ACTIVE_UTILITY_ACTION_ID) {
        const args = typeof action.args === "object" && action.args != null ? (action.args as { utilityId?: unknown; windowId?: unknown }) : {};
        const windowId = typeof args.windowId === "string" && args.windowId ? args.windowId : (activeWindowIdRef.current ?? "");
        if (!windowId) return;
        const requested = typeof args.utilityId === "string" ? args.utilityId : "";
        const next = resolveUtilityActivation(activeUtilityByWindowIdRef.current[windowId], requested);
        dispatch({ type: "SET_ACTIVE_UTILITY", windowId, utilityId: next });
        if (introductionStep?.advance.kind === "utility" && next && introductionStep.advance.id === next) advanceIntroductionStep();
        const pluginEntry = findPluginForAction(action);
        const plugin = pluginEntry?.handle;
        if (plugin) {
          const viewState: ViewState = { ...session.viewState, activeUtilityId: next ?? undefined };
          const forwarded: ActionDescriptor = { controllerId: action.controllerId, action: action.action, args: { utilityId: next } };
          void plugin
            .handleAction(session.instanceId, JSON.stringify(forwarded), viewState)
            .then((response) => applyHostEffects(response.requestedEffects ?? [], { ...session, viewState }, resolveUiDirtyScope(response.uiScope)))
            .catch((utilityError) => console.error("[DEBUG] setActiveUtility failed", utilityError));
        }
        return;
      }

      if (introductionStep?.advance.kind === "action" && introductionStep.advance.id === action.action) advanceIntroductionStep();

      if (action.controllerId === FRAMEWORK_SYNC_CONTROLLER_ID) {
        if (action.action === "selectFile") {
          dispatch({ type: "SET_SYNC_CARD_KIND", value: "file" });
          dispatch({ type: "SET_SYNC_DRAFT_PATH", value: syncBackboneUri?.startsWith("file://") ? syncBackboneUri.slice("file://".length) : "" });
          return;
        }
        if (action.action === "selectFolder") {
          dispatch({ type: "SET_SYNC_CARD_KIND", value: "folder" });
          dispatch({ type: "SET_SYNC_DRAFT_PATH", value: syncBackboneUri?.startsWith("folder://") ? syncBackboneUri.slice("folder://".length) : "" });
          return;
        }
        if (action.action === "selectRemote") {
          dispatch({ type: "SET_SYNC_CARD_KIND", value: "remote" });
          const remote = syncBackboneUri?.startsWith("remote://") ? syncBackboneUri.slice("remote://".length) : "";
          dispatch({ type: "SET_SYNC_DRAFT_PATH", value: remote });
          return;
        }
        if (action.action === "attach") {
          const path = typeof action.args === "object" && action.args != null && "path" in action.args ? String((action.args as { path?: string }).path ?? "") : syncDraftPath;
          if (!path.trim()) return;
          const uri =
            action.args && typeof action.args === "object" && "kind" in action.args
              ? String((action.args as { kind?: string }).kind) === "remote"
                ? buildRemoteBackboneUri(path.split("/")[0] ?? "127.0.0.1:8787", path.split("/").slice(1).join("/") || syncDocumentId(session, panel, studioMode))
                : String((action.args as { kind?: string }).kind) === "folder"
                  ? buildFolderBackboneUri(path)
                  : buildFileBackboneUri(path)
              : buildFileBackboneUri(path);
          void attachSyncBackbone(uri);
          return;
        }
        if (action.action === "detach") {
          void detachSyncBackbone();
          return;
        }
        return;
      }

      if (studioMode && action.controllerId === S_HOME_CONTROLLER_ID && action.action === "importStudio") {
        importStudioInputRef.current?.click();
        return;
      }

      if (studioMode && action.action === "spawnApp" && action.controllerId !== S_PLAY_CONTROLLER_ID) {
        const programId = typeof action.args === "object" && action.args != null && "programId" in action.args ? String((action.args as { programId?: string }).programId ?? "") : "";
        const pluginId = typeof action.args === "object" && action.args != null && "pluginId" in action.args ? String((action.args as { pluginId?: string }).pluginId ?? "") : "";
        const currentPanel = parsePanelState(session.viewState);
        const program = currentPanel?.programs.find((entry) => entry.programId === programId && entry.pluginId === pluginId);
        if (program) void spawnProgram(program);
        return;
      }

      if (studioMode && action.controllerId === S_PLAY_CONTROLLER_ID && action.action === "setActivePanelTab") {
        const tabId = typeof action.args === "object" && action.args != null && "tabId" in action.args ? String((action.args as { tabId?: string }).tabId ?? "s-play-catalogue") : "s-play-catalogue";
        const currentPanel = parsePanelState(session.viewState) ?? buildStudioPanelState(buildStudioPrograms(loadedPlugins), []);
        updateStudioPanel(buildStudioPanelState(currentPanel.programs, currentPanel.spawnedApps, tabId, currentPanel.activeSpawnedId));
        return;
      }

      const pluginEntry = findPluginForAction(action);
      const plugin = pluginEntry?.handle;
      if (!plugin) return;

      const targetSession =
        studioMode && action.controllerId !== session.app.controllerId
          ? (() => {
              const spawned = panel?.spawnedApps.find((entry) => {
                const app = loadedPlugins.find((p) => p.handle.pluginId === entry.pluginId)?.manifest.apps.find((a) => a.id === entry.appId);
                return app?.controllerId === action.controllerId;
              });
              if (!spawned) return session;
              const app = loadedPlugins.find((p) => p.handle.pluginId === spawned.pluginId)?.manifest.apps.find((a) => a.id === spawned.appId);
              if (!app) return session;
              return { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: session.viewState };
            })()
          : session;

      // 🚫 The old `setDocument` → `patchAppSource` mirror (spawned-instance content write-back on the
      // os document) is deleted — app content no longer embeds on the os document at all
      // (`OsAppInstance.document` is now just an `OsDocumentRef` handle). A spawned instance's content
      // sync now goes through its own `openDocument`-opened `DocumentHost` channel, same as any other
      // document; there is no host-side JS mirroring step anymore.
      const dispatchViewState = injectActiveUtility(targetSession.viewState);
      return plugin
        .handleAction(targetSession.instanceId, JSON.stringify(action), dispatchViewState)
        .then((response) => applyHostEffects(response.requestedEffects ?? [], { ...targetSession, viewState: dispatchViewState }, resolveUiDirtyScope(response.uiScope)))
        .catch((actionError) => {
          console.error("[DEBUG] action failed", actionError);
        });
    },
    [applyHostEffects, attachSyncBackbone, detachSyncBackbone, findPluginForAction, injectActiveUtility, loadedPlugins, panel, session, spawnProgram, studioMode, syncBackboneUri, syncDraftPath, updateStudioPanel],
  );

  const onActionRef = useRef(onAction);
  useEffect(() => {
    onActionRef.current = onAction;
  }, [onAction]);

  // 🐢 `onAction`'s own identity churns every action (its deps include `session`, `panel`, …). Render
  // trees built from `UiNode`s only need a *callable* action dispatcher, not a fresh one each time —
  // route them through this permanently-stable ref indirection so `interpretUiNode`'s `React.memo`
  // (and any `useMemo` keyed on the dispatcher passed to it) can actually bail.
  const onActionStable = useCallback((action: Parameters<typeof onAction>[0]) => onActionRef.current(action), []);

  const studioSessionActive = studioMode && session?.app.id === S_PLAY_APP_ID;
  useEffect(() => {
    if (!studioSessionActive || typeof window === "undefined") return;
    const identity = presenceClientIdentity();
    const beat = () => onActionRef.current({ controllerId: S_PLAY_CONTROLLER_ID, action: "presenceHeartbeat", args: identity });
    const initial = window.setTimeout(beat, 1000);
    const timer = window.setInterval(beat, PRESENCE_HEARTBEAT_INTERVAL_MS);
    return () => {
      window.clearTimeout(initial);
      window.clearInterval(timer);
    };
  }, [studioSessionActive]);

  usePanelChromeHotkeys({
    onToggle: (anchor) => dispatch({ type: "SET_PANEL_VISIBLE", anchor, value: (visible) => !visible }),
  });

  useElementsSurfaceChrome({ appearance: uiAppearance, device: uiDevice, expertise: uiExpertise, compact: uiCompact });

  //#region 💾 uiPrefs persistence (skips localStorage writes for any locked preference)
  useEffect(() => {
    if (!locks.appearance) writeStoredUiChromeAppearance(uiAppearance);
    writeStoredUiChromeLayout(uiLayout);
    writeStoredUiChromeCompact(uiCompact);
    writeStoredUiChromeExpertise(uiExpertise);
    if (!locks.locale) writeStoredUiChromeLocale(uiLocale);
    void setUiLocale(uiLocale);
    if (!locks.terminology) writeStoredUiChromeTerminology(uiTerminology);
    setActiveUiTheme(uiTheme);
    if (!locks.themeId) {
      writeStoredUiChromeThemeSnapshot(uiTheme);
      writeStoredUiChromeThemeId(uiThemeId);
    }
    writeStoredUiCustomThemes(uiCustomThemes);
  }, [uiAppearance, uiLayout, uiCompact, uiExpertise, uiLocale, uiTerminology, uiTheme, uiThemeId, uiCustomThemes, locks]);
  //#endregion

  useActionHotkey(
    "mod+[",
    useCallback(() => {
      if (canGoBack) goBack();
    }, [canGoBack, goBack]),
  );
  useActionHotkey(
    "mod+]",
    useCallback(() => {
      if (canGoForward) goForward();
    }, [canGoForward, goForward]),
  );
  useActionHotkey(
    "mod+up",
    useCallback(() => {
      if (canGoUp) goUp();
    }, [canGoUp, goUp]),
  );
  useActionHotkey(
    "mod+p",
    useCallback(() => dispatch({ type: "SET_SEARCH_OPEN", value: (open) => !open }), []),
  );
  useActionHotkey(
    "mod+f",
    useCallback(() => dispatch({ type: "SET_FIND_OPEN", value: (open) => !open }), []),
  );

  const applyNamedLayout = useCallback(
    (layout: WindowLayout) => {
      if (!session) return;
      const windowIds = session.app.windowKinds.map((kind) => kind.id);
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: [] });
      extraWindowCounterRef.current = 0;
      dispatch({ type: "SET_SHELL_LAYOUT", value: convertFrameworkLayoutToModeLayout(layout, windowIds, appLabelsOverlay) });
      const defaultWindowId = findDefaultActiveWindowKindId(layout, session.app.windowKinds);
      if (defaultWindowId) dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: defaultWindowId });
    },
    [session, appLabelsOverlay],
  );

  const applyModeChange = useCallback(
    (modeId: string) => {
      dispatch({
        type: "SET_SESSION",
        value: (current) => {
          if (!current) return current;
          const layout = resolveLayoutForMode(current.app, modeId);
          if (layout) {
            dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: [] });
            extraWindowCounterRef.current = 0;
            dispatch({
              type: "SET_SHELL_LAYOUT",
              value: convertFrameworkLayoutToModeLayout(
                layout,
                current.app.windowKinds.map((kind) => kind.id),
                appLabelsOverlay,
              ),
            });
            const defaultWindowId = findDefaultActiveWindowKindId(layout, current.app.windowKinds);
            if (defaultWindowId) dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: defaultWindowId });
          }
          return { ...current, viewState: { ...current.viewState, activeModeId: modeId } };
        },
      });
    },
    [appLabelsOverlay],
  );

  const handleTemplateDrop = useCallback(
    (payload: WindowTemplateDropPayload, target: ModeCanvasDropTarget) => {
      if (!session) return;
      const kind = session.app.windowKinds.find((entry) => entry.id === payload.windowKindId);
      if (!kind) return;
      extraWindowCounterRef.current += 1;
      const instanceId = `${payload.windowKindId}-${extraWindowCounterRef.current}`;
      dispatch({
        type: "SET_EXTRA_WINDOW_INSTANCES",
        value: (current) => [...current, { id: instanceId, windowKindId: payload.windowKindId, title: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, kind.label) }],
      });
      dispatch({
        type: "SET_SHELL_LAYOUT",
        value: (current) => {
          const base =
            current ??
            convertFrameworkLayoutToModeLayout(
              session.app.defaultLayout,
              session.app.windowKinds.map((entry) => entry.id),
              appLabelsOverlay,
            );
          return insertWindowAtDropZone(base, instanceId, target);
        },
      });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: instanceId });
    },
    [appLabelsOverlay, session],
  );

  const displayHostRef = useRef<DisplayHostApi | null>(null);
  const displayHost = useNamedLayoutHost({
    appId: session?.app.id ?? "framework-os",
    windowKinds: session?.app.windowKinds.map((kind) => ({ ...kind, label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, kind.label) })) ?? [],
    builtinLayouts: session?.app.namedLayouts ?? [],
    currentLayout: captureCurrentFrameworkLayout(shellLayout, session?.app.defaultLayout),
    onApplyLayout: applyNamedLayout,
    namedLayoutStore,
  });
  displayHostRef.current = displayHost;

  //#region 🔖ThemeMutators
  const uiThemeBase = uiThemeDraft ?? uiTheme;
  const uiThemeDirty = uiThemeDraft !== null;
  const uiThemeList = useMemo((): readonly UiTheme[] => [...builtinUiThemes(), ...Object.values(uiCustomThemes)], [uiCustomThemes]);
  const osCommands = useMemo(
    () => buildOsCommands(uiThemeList, [UI_TERMINOLOGY_NATIVE, ...(session?.app.terminologies ?? [])], session?.app.introduction != null, locks),
    [uiThemeList, session?.app.terminologies, session?.app.introduction, uiLocale, uiTerminology, locks],
  );

  const draftThemePatch = useCallback(
    (patch: (next: UiTheme) => void) => {
      const next = structuredClone(uiThemeBase);
      patch(next);
      dispatch({ type: "SET_UI_THEME_DRAFT", value: next });
    },
    [uiThemeBase],
  );

  const setThemeId = useCallback((id: string) => {
    dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
    dispatch({ type: "SET_UI_THEME_ID", value: id });
  }, []);

  const setThemeColor = useCallback(
    (key: string, hex: string) =>
      draftThemePatch((next) => {
        next.colors[key] = hex;
      }),
    [draftThemePatch],
  );
  const setThemeSpacing = useCallback(
    (key: string, value: string) =>
      draftThemePatch((next) => {
        next.spacing[key] = value;
      }),
    [draftThemePatch],
  );
  const setThemeFontStack = useCallback(
    (key: string, value: string) =>
      draftThemePatch((next) => {
        next.fontStacks[key] = value;
      }),
    [draftThemePatch],
  );
  const setThemeStroke = useCallback(
    (key: string, value: number | number[]) =>
      draftThemePatch((next) => {
        next.strokes[key] = value;
      }),
    [draftThemePatch],
  );
  const setThemeRadius = useCallback(
    (key: string, value: number) =>
      draftThemePatch((next) => {
        next.radii[key] = value;
      }),
    [draftThemePatch],
  );
  const setThemeOpacity = useCallback(
    (key: string, value: number) =>
      draftThemePatch((next) => {
        next.opacities[key] = value;
      }),
    [draftThemePatch],
  );
  const setThemeMetric = useCallback(
    (section: string, key: string, value: number | number[]) =>
      draftThemePatch((next) => {
        next.metrics[section] = { ...(next.metrics[section] ?? {}), [key]: value };
      }),
    [draftThemePatch],
  );
  const setThemeAppearancePaint = useCallback(
    (appearance: ThemeAppearanceName, group: ThemePaletteGroup, key: string, hex: string, alpha?: number) =>
      draftThemePatch((next) => {
        next.appearances[appearance][group][key] = alpha === undefined ? { hex } : { hex, alpha };
      }),
    [draftThemePatch],
  );

  const resetTheme = useCallback(() => {
    dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
    dispatch({ type: "SET_UI_THEME_ID", value: "semio" });
  }, []);

  const saveTheme = useCallback(
    (label: string) => {
      const trimmed = label.trim();
      if (!trimmed) return;
      const slug = trimmed
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, "-")
        .replace(/(^-+|-+$)/g, "");
      if (!slug) return;
      const id = `custom.${slug}`;
      const saved: UiTheme = { ...uiThemeBase, id, label: trimmed };
      dispatch({ type: "SET_UI_CUSTOM_THEMES", value: (current) => ({ ...current, [id]: saved }) });
      dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
      dispatch({ type: "SET_UI_THEME_ID", value: id });
    },
    [uiThemeBase],
  );

  const deleteTheme = useCallback((id: string) => {
    if (!id.startsWith("custom.")) return;
    dispatch({
      type: "SET_UI_CUSTOM_THEMES",
      value: (current) => {
        const { [id]: _removed, ...rest } = current;
        return rest;
      },
    });
    dispatch({ type: "SET_UI_THEME_ID", value: (current) => (current === id ? "semio" : current) });
    dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
  }, []);

  const exportTheme = useCallback(() => {
    downloadMediaExport(`${uiThemeBase.id}.theme.json`, "application/json", serializeUiTheme(uiThemeBase));
  }, [uiThemeBase]);

  const importTheme = useCallback(async () => {
    const opened = await requestFileOpen(".theme.json,application/json");
    if (!opened) return;
    try {
      const parsed = parseUiTheme(JSON.parse(opened.contents));
      saveTheme(parsed.label || parsed.id);
    } catch {
      /* invalid theme file, ignore */
    }
  }, [saveTheme]);
  //#endregion 🔖ThemeMutators

  const [themeSaveLabel, setThemeSaveLabel] = useState("");
  const settingsHostRef = useRef<SettingsHostApi | null>(null);
  const settingsHost: SettingsHostApi = useMemo(
    () => ({
      appId: session?.app.id,
      appLabel: session ? appDocumentLabel(session.app.document) : undefined,
      controllerId: session?.app.controllerId,
      pluginId: session?.pluginId,
      compact: uiCompact,
      setCompact: (value: boolean) => dispatch({ type: "SET_UI_COMPACT", value }),
      expertise: uiExpertise,
      setExpertise: (value: string) => dispatch({ type: "SET_UI_EXPERTISE", value: value as Expertise }),
      appearance: uiAppearance,
      setAppearance: (value: string) => dispatch({ type: "SET_UI_APPEARANCE", value: value as ElementsSurfaceAppearance }),
      layout: uiLayout,
      setLayout: (value: UiChromeLayout) => dispatch({ type: "SET_UI_LAYOUT", value }),
      mobileActive: mobile,
      onResetDock: () => {
        dispatch({ type: "RESET_DOCK" });
        dockLayoutStore.reset();
        dockUiStateStore.reset();
      },
      locale: uiLocale,
      setLocale: (value: UiLocale) => dispatch({ type: "SET_UI_LOCALE", value }),
      terminology: uiTerminology,
      setTerminology: (value: string) => dispatch({ type: "SET_UI_TERMINOLOGY", value }),
      terminologies: [UI_TERMINOLOGY_NATIVE, ...(session?.app.terminologies ?? [])],
      theme: uiThemeBase,
      themeId: uiThemeId,
      themeDirty: uiThemeDirty,
      themes: uiThemeList,
      setThemeId,
      setThemeColor,
      setThemeSpacing,
      setThemeFontStack,
      setThemeStroke,
      setThemeRadius,
      setThemeOpacity,
      setThemeMetric,
      setThemeAppearancePaint,
      saveTheme,
      deleteTheme,
      resetTheme,
      exportTheme,
      importTheme,
      themeSaveLabel,
      setThemeSaveLabel,
      locks,
    }),
    [
      session,
      dockLayoutStore,
      uiCompact,
      uiExpertise,
      uiAppearance,
      uiLayout,
      mobile,
      uiLocale,
      uiTerminology,
      uiThemeBase,
      uiThemeId,
      uiThemeDirty,
      uiThemeList,
      locks,
      setThemeId,
      setThemeColor,
      setThemeSpacing,
      setThemeFontStack,
      setThemeStroke,
      setThemeRadius,
      setThemeOpacity,
      setThemeMetric,
      setThemeAppearancePaint,
      saveTheme,
      deleteTheme,
      resetTheme,
      exportTheme,
      importTheme,
      themeSaveLabel,
      setThemeSaveLabel,
    ],
  );
  settingsHostRef.current = settingsHost;

  const frameworkDisplayTabs = useMemo(() => createFrameworkDisplayPanelTabs(() => displayHostRef.current), [displayHost, uiLocale]);
  const frameworkSettingsTabs = useMemo(() => createFrameworkSettingsPanelTabs(() => settingsHostRef.current), [settingsHost]);

  useEffect(() => {
    if (!session) return;
    const parseKeys = (keys: string) =>
      keys
        .split(",")
        .map((key) => key.trim().toLowerCase())
        .filter(Boolean);
    const isEditableTarget = (target: EventTarget | null) => {
      if (!(target instanceof HTMLElement)) return false;
      const tag = target.tagName;
      if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
      if (target.isContentEditable) return true;
      return target.closest("[contenteditable='true'], [role='textbox']") != null;
    };
    const matches = (event: KeyboardEvent, binding: string) => {
      const parts = binding.split("+").map((part) => part.trim());
      const key = parts[parts.length - 1] ?? "";
      const needsCtrl = parts.includes("ctrl") || parts.includes("meta") || parts.includes("mod");
      const needsShift = parts.includes("shift");
      const needsAlt = parts.includes("alt");
      const hasCtrl = event.ctrlKey || event.metaKey;
      if (needsCtrl !== hasCtrl) return false;
      if (needsShift !== event.shiftKey) return false;
      if (needsAlt !== event.altKey) return false;
      return event.key.toLowerCase() === key;
    };
    const actionById = new Map(session.app.actions.map((action) => [action.id, action]));
    const onKeyDown = (event: KeyboardEvent) => {
      if (isEditableTarget(event.target)) return;
      // 🧰 Escape deactivates the active window's active utility (P5) when nothing is being typed.
      if (event.key === "Escape") {
        const windowId = activeWindowIdRef.current;
        if (windowId && activeUtilityByWindowIdRef.current[windowId]) {
          event.preventDefault();
          onAction({ controllerId: session.app.controllerId, action: SET_ACTIVE_UTILITY_ACTION_ID, args: { windowId, utilityId: "" } });
          return;
        }
      }
      for (const binding of session.app.keybindings) {
        for (const chord of parseKeys(binding.keys)) {
          if (!matches(event, chord)) continue;
          event.preventDefault();
          // ✍️ Arg-carrying hotkeys never silent-fire defaults (P4): open the staged form, or — if that
          // form is already expanded in the active window — treat the hotkey as Execute (with validation).
          const definition = actionById.get(binding.action.action);
          if (definition && actionRequiresStagedForm(definition)) {
            const windowId = activeWindowIdRef.current;
            if (!windowId) return;
            const expanded = actionPaneExpandedByWindowIdRef.current[windowId] ?? null;
            const staged = actionPaneStagedArgsByKeyRef.current[actionStageKey(windowId, definition.id)] ?? {};
            const intent = resolveKeybindingIntent(definition, expanded, staged);
            if (intent.kind === "execute") {
              onAction({ controllerId: session.app.controllerId, action: intent.actionId, args: intent.args });
            } else if (intent.kind === "open") {
              dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId, value: false });
              dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId, value: intent.actionId });
            }
            return;
          }
          onAction(binding.action);
          return;
        }
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [onAction, session]);

  const activeRightPanelTab = session?.app.panelTabs.find((tab) => panelAnchorForGroup(tab.group) === "top-right");
  const activePanelTabId = panel?.activePanelTab ?? (activeRightPanelTab ? panelTabKindId(activeRightPanelTab.kind) : undefined) ?? (session?.app.panelTabs[0] ? panelTabKindId(session.app.panelTabs[0].kind) : undefined);

  const workbenchLeftTabs = useMemo((): PanelTabNode[] => {
    if (!session) return [];
    const pluginLeftTabs = session.app.panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === "top-left").map((tab, order) => panelTabDefinitionToNode(tab, tab.group, panelUiByKey, onAction, order, appLabelsOverlay));
    if (studioMode && session.app.id === S_PLAY_APP_ID && pluginLeftTabs.length > 0) return pluginLeftTabs;
    const hasPluginDocumentTab = pluginLeftTabs.some((tab) => tab.id === FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
    if (hasPluginDocumentTab) return pluginLeftTabs;
    const documentTab = singleTreeLeaf({
      id: FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
      icon: shellTabIcon(FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID),
      name: shellLabel("ui.panel.document"),
      order: 0,
      tree: staticTreePanelDefinition({
        sections: [
          {
            id: "document.root",
            label: shellLabel("ui.panel.document"),
            items: [{ id: "document.empty", label: studioMode ? `${panel?.spawnedApps.length ?? 0} ${shellLabel("ui.panel.spawnedAppsSuffix")}` : shellLabel("ui.panel.documentEmpty") }],
          },
        ],
      }),
    });
    return [documentTab, ...pluginLeftTabs];
  }, [appLabelsOverlay, onAction, panel?.spawnedApps.length, panelUiByKey, session, studioMode, uiLocale]);

  const detailsRightTabs = useMemo((): PanelTabNode[] => {
    if (!session) return [];
    return session.app.panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === "top-right").map((tab, order) => panelTabDefinitionToNode(tab, tab.group, panelUiByKey, onAction, order, appLabelsOverlay));
  }, [appLabelsOverlay, onAction, panelUiByKey, session]);

  const settingsRightTabs = useMemo((): PanelTabNode[] => frameworkSettingsTabs, [frameworkSettingsTabs]);

  //#region 🧰FooterUtilityLeaves — bottom-right's History tab, now sourced from the framework-owned History
  // actions in the app registry (the plugin `list-tools` surface is gone; the per-window Actions rail
  // replaces the old footer Actions tab entirely per P6).
  const frameworkUtilitiesHistoryTab = useMemo((): PanelTabNode | null => {
    if (!session) return null;
    const grouped = groupUtilityNodesByCategory(frameworkHistoryUtilityNodes(session.app), ["history"]);
    if (!grouped.length) return null;
    return singleTreeLeaf({
      id: "framework.utilities.history",
      icon: shellTabIcon(UTILITY_CATEGORY_ICON_ID.history),
      name: shellLabel("ui.panel.toolsHistory"),
      order: 1,
      tree: {
        sections: [{ id: "framework.utilities.history.root", label: "", items: [{ id: "framework.utilities.history.tree", label: "", control: <UtilityTree id="ui.toolbar.footer.history" utilities={grouped} onAction={onAction} direction="up" /> }] }],
      },
    });
  }, [onAction, session, uiLocale]);
  //#endregion 🧰FooterUtilityLeaves

  //#region 🔄SyncLeaf — bottom-left's sync tab, replacing the old floating footer SyncAttachCard.
  const frameworkSyncTab = useMemo((): PanelTabNode | null => {
    const syncUtilities = buildFrameworkSyncUtilities(syncBackboneUri) as readonly UtilityNode[];
    if (!syncUtilities.length) return null;
    const syncStatus = syncBackboneUri ? (syncStatusByDocumentId[syncBackboneUri.replace(/^actor:\/\//, "")] ?? null) : null;
    return singleTreeLeaf({
      id: "framework.sync",
      icon: shellTabIcon(UTILITY_CATEGORY_ICON_ID.sync),
      name: shellLabel("ui.panel.sync"),
      order: 0,
      tree: {
        sections: [
          {
            id: "framework.sync.root",
            label: "",
            items: [
              {
                id: "framework.sync.card",
                label: "",
                control: (
                  <SyncAttachCard
                    activeUri={syncBackboneUri}
                    cardKind={syncCardKind}
                    draftPath={syncDraftPath}
                    syncUtilities={syncUtilities}
                    status={syncStatus}
                    onAction={onAction}
                    onDraftPathChange={(value) => dispatch({ type: "SET_SYNC_DRAFT_PATH", value })}
                    onClose={() => dispatch({ type: "SET_SYNC_CARD_KIND", value: null })}
                    onAttach={attachSyncBackbone}
                    onDetach={detachSyncBackbone}
                  />
                ),
              },
            ],
          },
        ],
      },
    });
  }, [attachSyncBackbone, detachSyncBackbone, onAction, syncBackboneUri, syncCardKind, syncDraftPath, syncStatusByDocumentId, uiLocale]);
  //#endregion 🔄SyncLeaf

  const activePluginManifest = useMemo(() => loadedPlugins.find((entry) => entry.handle.pluginId === session?.pluginId)?.manifest, [loadedPlugins, session?.pluginId]);
  const activeModeId = session?.viewState.activeModeId ?? session?.app.modes[0]?.id ?? session?.app.id ?? "";

  const resolvedCommands = useMemo(() => resolveCommands(osCommands, activePluginManifest, session?.app, activeModeId), [osCommands, activePluginManifest, session?.app, activeModeId]);

  const commandCategoryList = useMemo(() => commandCategories(resolvedCommands), [resolvedCommands, uiLocale]);

  /**
   * 🎛 Dispatches a resolved command: os-scope commands are handled locally (no plugin round trip);
   * plugin/app/mode-scope commands route through the active session's plugin `handleCommand`, mirroring
   * `onAction`'s tail. Plugin commands are only resolvable/dispatchable for the active session's plugin
   * instance (no headless-instance routing for non-focused plugins yet).
   */
  const onCommand = useCallback(
    (source: ResolvedCommand["source"], commandId: string, args?: Record<string, unknown>) => {
      if (source.kind === "os") {
        dispatchOsCommand(commandId, args, dispatch, dockLayoutStore, dockUiStateStore, locks);
        return;
      }
      if (!session) return;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
      if (!plugin?.handleCommand) return;
      const dispatchViewState = injectActiveUtility(session.viewState);
      void plugin
        .handleCommand(session.instanceId, JSON.stringify({ command: commandId, args }), dispatchViewState)
        .then((response) => applyHostEffects(response.requestedEffects ?? [], { ...session, viewState: dispatchViewState }, resolveUiDirtyScope(response.uiScope)))
        .catch((commandError) => {
          console.error("[DEBUG] command failed", commandError);
        });
    },
    [applyHostEffects, dockLayoutStore, dockUiStateStore, injectActiveUtility, loadedPlugins, session, locks],
  );

  const commandCategoryTabs = useMemo(() => buildCommandCategoryTabs(resolvedCommands, commandCategoryList, expandedCommandIdRef, commandStagedArgsByCommandIdRef, onCommand, dispatch), [resolvedCommands, commandCategoryList, onCommand]);

  //#region 🧭DockAssembly — default four-corner arrangement (the two middle anchors start empty save the command palette in bottom-middle) + persisted-override reconciliation + drag-and-drop wiring.
  const defaultDock = useMemo((): PanelDock => {
    // 🧭 Top-left (Workbench: Document/Catalogue), top-right (Details: Inspection/Parameters) and bottom-right
    // (Settings: Theme/Settings) render their tabs flat, one level up from where they used to sit — the
    // category-branch wrapper tab is gone, so each leaf is a top-level toggle instead of two clicks deep.
    const topLeft: PanelTabNode[] = [...workbenchLeftTabs];
    const bottomLeft: PanelTabNode[] = [];
    if (frameworkDisplayTabs.length > 0) {
      bottomLeft.push({ kind: "branch", id: FRAMEWORK_CATEGORY_DISPLAY_ID, icon: categoryTabIcon(frameworkDisplayTabs, "layout-grid"), name: shellLabel("ui.panelToggle.display"), order: 0, children: frameworkDisplayTabs });
    }
    if (frameworkSyncTab) bottomLeft.push(frameworkSyncTab);
    const topRight: PanelTabNode[] = [...detailsRightTabs];
    const bottomRight: PanelTabNode[] = [...settingsRightTabs];
    if (frameworkUtilitiesHistoryTab) bottomRight.push(frameworkUtilitiesHistoryTab);
    // 🎛 Command categories stay nested under one expandable Command branch (unlike flat Theme/Settings
    // footer toggles) so the folded bottom-middle chrome shows a single Command toggle, not every
    // category leaf inlined along the footer.
    const bottomMiddle: PanelTabNode[] =
      commandCategoryTabs.length > 0
        ? [{ kind: "branch", id: FRAMEWORK_CATEGORY_COMMAND_ID, icon: categoryTabIcon(commandCategoryTabs, "wrench"), name: shellLabel("ui.panelToggle.command"), order: 0, children: commandCategoryTabs }]
        : [];
    return { anchors: { "top-left": topLeft, "top-middle": [], "top-right": topRight, "bottom-left": bottomLeft, "bottom-middle": bottomMiddle, "bottom-right": bottomRight } };
  }, [commandCategoryTabs, detailsRightTabs, frameworkDisplayTabs, frameworkSyncTab, frameworkUtilitiesHistoryTab, settingsRightTabs, uiLocale, workbenchLeftTabs]);

  useEffect(() => {
    dispatch({ type: "SET_DOCK_OVERRIDE", value: dockLayoutStore.getSnapshot() });
  }, [dockLayoutStore]);

  const dock = useMemo((): PanelDock => applyDockSkeleton(defaultDock, dockOverride), [defaultDock, dockOverride]);

  /** 🗄️ Skips the very first (pre-hydration) commit so a persisted skeleton isn't clobbered with `null` before the seeding effect above has a chance to read and apply it. */
  const dockPersistedOnceRef = useRef(false);
  useEffect(() => {
    if (!dockPersistedOnceRef.current) {
      dockPersistedOnceRef.current = true;
      return;
    }
    const nextSkeleton = dockSkeletonOf(dock);
    const defaultSkeleton = dockSkeletonOf(defaultDock);
    dockLayoutStore.save(dockSkeletonsEqual(nextSkeleton, defaultSkeleton) ? null : nextSkeleton);
  }, [dock, defaultDock, dockLayoutStore]);

  useEffect(() => {
    dispatch({ type: "HYDRATE_DOCK_UI", value: dockUiStateStore.getSnapshot() });
  }, [dockUiStateStore]);

  /** 🗄️ Same first-commit-skip as the dock skeleton effect above, but also re-arms when the store identity itself changes (app switch) — otherwise the new app's pre-hydration state would be written into its own key on the first post-switch commit. */
  const dockUiPersistedOnceRef = useRef(false);
  const dockUiPersistedStoreRef = useRef(dockUiStateStore);
  useEffect(() => {
    if (dockUiPersistedStoreRef.current !== dockUiStateStore) {
      dockUiPersistedStoreRef.current = dockUiStateStore;
      dockUiPersistedOnceRef.current = false;
    }
    if (!dockUiPersistedOnceRef.current) {
      dockUiPersistedOnceRef.current = true;
      return;
    }
    const anchors: Partial<Record<PanelAnchor, DockUiPanelState>> = {};
    for (const anchor of PANEL_ANCHORS) {
      const panelState = panels[anchor];
      const entry: DockUiPanelState = {};
      if (panelState.visible) entry.visible = true;
      if (panelState.size !== DEFAULT_PANEL_SIZES[anchor]) entry.size = panelState.size;
      if (panelState.path.length > 0) entry.path = panelState.path;
      if (Object.keys(entry).length > 0) anchors[anchor] = entry;
    }
    const hasPathMemory = Object.keys(panelPathMemory).length > 0;
    const hasTreeOpen = Object.keys(treeOpenStates).length > 0;
    const isDefault = Object.keys(anchors).length === 0 && !hasPathMemory && !hasTreeOpen;
    dockUiStateStore.save(isDefault ? null : { version: 2, anchors, pathMemory: hasPathMemory ? panelPathMemory : undefined, treeOpen: hasTreeOpen ? treeOpenStates : undefined });
  }, [panels, panelPathMemory, treeOpenStates, dockUiStateStore]);

  const handleTabDockDrop = useCallback(
    (move: PanelTabDockMove) => {
      const nextDock = moveTabInDock(dock, move);
      if (nextDock === dock) return;
      const nextSkeleton = dockSkeletonOf(nextDock);
      const defaultSkeleton = dockSkeletonOf(defaultDock);
      dispatch({ type: "SET_DOCK_OVERRIDE", value: dockSkeletonsEqual(nextSkeleton, defaultSkeleton) ? null : nextSkeleton });
      const targetPath = findPanelTabPath(nextDock.anchors[move.target.anchor], move.tabId);
      if (targetPath) dispatch({ type: "SET_PANEL_PATH", anchor: move.target.anchor, value: targetPath });
      if (move.fromAnchor !== move.target.anchor) {
        const sourceTabs = nextDock.anchors[move.fromAnchor];
        dispatch({ type: "SET_PANEL_PATH", anchor: move.fromAnchor, value: (prev) => reconcileActivePath(sourceTabs, prev, panelTabChildren) });
      }
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: move.target.anchor, value: true });
    },
    [dock, defaultDock],
  );

  const handleTreeUnitDockDrop = useCallback(
    (move: PanelTreeUnitDockMove) => {
      const nextDock = moveTreeUnitInDock(dock, move);
      if (nextDock === dock) return;
      const nextSkeleton = dockSkeletonOf(nextDock);
      const defaultSkeleton = dockSkeletonOf(defaultDock);
      dispatch({ type: "SET_DOCK_OVERRIDE", value: dockSkeletonsEqual(nextSkeleton, defaultSkeleton) ? null : nextSkeleton });
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: move.target.anchor, value: true });
    },
    [dock, defaultDock],
  );

  const studioOverrideTabId = studioMode && session?.app.id === S_PLAY_APP_ID ? (panel?.activePanelTab ?? S_PLAY_CATALOGUE_TAB_ID) : undefined;
  const studioOverrideAnchor = studioOverrideTabId ? findPanelTabInDock(dock, studioOverrideTabId)?.anchor : undefined;
  const detailsOverrideTabId = panel?.activePanelTab;
  const detailsOverrideAnchor = detailsOverrideTabId ? findPanelTabInDock(dock, detailsOverrideTabId)?.anchor : undefined;

  /** @emoji 🎓 The current introduction step's anchor, decomposed by kind — `null` unless that kind is
   * active, so every reveal override below (here and in `modeWindows`) is a plain truthiness check. A
   * folded toolbar/Actions rail/dock panel would otherwise hide the step's anchor from ever mounting (see
   * `useIntroductionAnchorRect`), leaving the step centered with no highlight and no way for the user to
   * find what to do. */
  const activeIntroductionStepAnchor: IntroductionAnchor | null = session?.app.introduction && introductionStepIndex != null ? (session.app.introduction.steps[introductionStepIndex]?.anchor ?? null) : null;
  const introductionUtilityId = activeIntroductionStepAnchor?.kind === "utility" ? activeIntroductionStepAnchor.id : null;
  const introductionActionId = activeIntroductionStepAnchor?.kind === "action" ? activeIntroductionStepAnchor.id : null;
  const introductionPanelTabId = activeIntroductionStepAnchor?.kind === "panelTab" ? activeIntroductionStepAnchor.id : null;
  const introductionPanelTabAnchor = introductionPanelTabId ? findPanelTabInDock(dock, introductionPanelTabId)?.anchor : undefined;
  const introductionUtilityWindowId = useMemo(() => {
    if (!introductionUtilityId || !session) return null;
    for (const kind of session.app.windowKinds) {
      const utilities = resolveUtilityNodes(session.app, kind, null, kind.id, appLabelsOverlay);
      if (utilityNodeTreeContainsId(utilities, introductionUtilityId)) return kind.id;
    }
    return null;
  }, [appLabelsOverlay, introductionUtilityId, session]);
  const introductionActionWindowId = useMemo(() => {
    if (!introductionActionId || !session) return null;
    for (const kind of session.app.windowKinds) {
      const actions = resolveWindowActions(session.app, kind);
      if (actions.some((action) => action.id === introductionActionId)) return kind.id;
    }
    return null;
  }, [introductionActionId, session]);
  const introductionAnchorFallbackSelectors = useMemo((): readonly string[] => {
    if (introductionUtilityWindowId) return [introductionWindowToolbarUnfoldSelector(introductionUtilityWindowId)];
    if (introductionActionWindowId) return [introductionWindowActionPaneUnfoldSelector(introductionActionWindowId)];
    return [];
  }, [introductionActionWindowId, introductionUtilityWindowId]);

  const lastIntroductionPanelTabIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    if (!introductionPanelTabId || !introductionPanelTabAnchor) {
      lastIntroductionPanelTabIdRef.current = undefined;
      return;
    }
    if (lastIntroductionPanelTabIdRef.current === introductionPanelTabId) return;
    lastIntroductionPanelTabIdRef.current = introductionPanelTabId;
    const resolved = findPanelTabPath(dock.anchors[introductionPanelTabAnchor], introductionPanelTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: introductionPanelTabAnchor, value: resolved });
    dispatch({ type: "SET_PANEL_VISIBLE", anchor: introductionPanelTabAnchor, value: true });
  }, [introductionPanelTabId, introductionPanelTabAnchor, dock]);

  /** 🧭 Progressive reveal means a stored path can legitimately end at a branch (or be empty) — this is now a plain per-anchor truncation-validate, no override reassertion (see the write-through effects below). */
  const panelActivePaths = useMemo((): Record<PanelAnchor, readonly string[]> => {
    const result = {} as Record<PanelAnchor, readonly string[]>;
    for (const anchor of PANEL_ANCHORS) result[anchor] = reconcileActivePath(dock.anchors[anchor], panels[anchor].path, panelTabChildren);
    return result;
  }, [panels, dock]);

  /**
   * 🧭 Generalizes the old `leftPanelActivePath`/`rightPanelActivePath` studio/plugin "snap to the active panel
   * tab" overrides across all six anchors. Write-through rather than read-time: each override dispatches
   * `SET_PANEL_PATH` only when its target tab id actually changes, so a user's own collapse/navigation
   * afterward sticks instead of being reasserted on every render (progressive reveal made read-time reassertion
   * fight the user's own collapses). Studio wins over details when both would touch the same anchor.
   **/
  const lastStudioOverrideTabIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    if (!studioOverrideTabId || !studioOverrideAnchor) {
      lastStudioOverrideTabIdRef.current = undefined;
      return;
    }
    if (lastStudioOverrideTabIdRef.current === studioOverrideTabId) return;
    lastStudioOverrideTabIdRef.current = studioOverrideTabId;
    if (panels[studioOverrideAnchor].path[0] === FRAMEWORK_CATEGORY_DISPLAY_ID) return;
    const resolved = findPanelTabPath(dock.anchors[studioOverrideAnchor], studioOverrideTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: studioOverrideAnchor, value: resolved });
  }, [studioOverrideTabId, studioOverrideAnchor, dock, panels]);

  const lastDetailsOverrideTabIdRef = useRef<string | undefined>(undefined);
  useEffect(() => {
    if (!detailsOverrideTabId || !detailsOverrideAnchor) {
      lastDetailsOverrideTabIdRef.current = undefined;
      return;
    }
    if (lastDetailsOverrideTabIdRef.current === detailsOverrideTabId) return;
    lastDetailsOverrideTabIdRef.current = detailsOverrideTabId;
    if (detailsOverrideAnchor === studioOverrideAnchor) return;
    // 🧭 Settings tabs render flat now (no category branch to check against) — skip the override if the
    // anchor's active leaf already belongs to Settings, so browsing Theme/Settings there doesn't get stomped.
    if (settingsRightTabs.some((tab) => tab.id === panels[detailsOverrideAnchor].path[0])) return;
    const resolved = findPanelTabPath(dock.anchors[detailsOverrideAnchor], detailsOverrideTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: detailsOverrideAnchor, value: resolved });
  }, [detailsOverrideTabId, detailsOverrideAnchor, studioOverrideAnchor, dock, panels, settingsRightTabs]);
  //#endregion 🧭DockAssembly

  const mobilePanelTabs = useMemo(
    () => [...defaultDock.anchors["top-left"], ...defaultDock.anchors["top-middle"], ...defaultDock.anchors["top-right"], ...defaultDock.anchors["bottom-left"], ...defaultDock.anchors["bottom-middle"], ...defaultDock.anchors["bottom-right"]],
    [defaultDock],
  );

  const mobilePanel = useMemo(() => {
    if (mobilePanelTabs.length === 0) return undefined;
    return {
      visible: PANEL_ANCHORS.some((anchor) => panels[anchor].visible),
      tabs: mobilePanelTabs,
      activeTabPath: mobilePanelPath,
      onActiveTabPathChange: (path: readonly string[]) => {
        dispatch({ type: "SET_MOBILE_PANEL_PATH", value: path });
        const tabId = path[path.length - 1];
        // 🌱 Progressive paths often end at a branch (or are empty) — only leaves are meaningful "active panel tab" selections.
        if (tabId && studioMode && session?.app.id === S_PLAY_APP_ID && findPanelTabNode(mobilePanelTabs, path)?.kind === "leaf") {
          onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } });
        }
      },
      pathMemory: panelPathMemory,
      onPathMemoryChange: (value: Readonly<Record<string, string>>) => dispatch({ type: "SET_PANEL_PATH_MEMORY", value }),
      treeOpenStates,
      onTreeOpenStateChange: (id: string, open: boolean) => dispatch({ type: "SET_TREE_OPEN_STATE", id, open }),
    };
  }, [panels, mobilePanelPath, mobilePanelTabs, onAction, panelPathMemory, session, studioMode, treeOpenStates]);

  const exampleOptions = useMemo(() => {
    const appId = session?.app.id ?? "";
    if (!appId) return [];
    const seen = new Set<string>();
    return (activePluginManifest?.examples ?? [])
      .filter((example) => example.appId === appId)
      .filter((example) => {
        if (seen.has(example.id)) return false;
        seen.add(example.id);
        return true;
      })
      .map((example) => ({ id: example.id, label: resolveAppLabel(appLabelsOverlay, "example", example.id, example.label) }));
  }, [activePluginManifest, session?.app.id, appLabelsOverlay]);

  useEffect(() => {
    if (exampleOptions.length === 0) return;
    dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: (current) => (!current || exampleOptions.some((option) => option.id === current) ? current : "") });
  }, [exampleOptions, session?.app.id, session?.pluginId]);

  useEffect(() => {
    if (exampleOptions.length === 0 || activeExampleId || !session) return;
    if (noExampleResetInstanceIdRef.current === session.instanceId) return;
    noExampleResetInstanceIdRef.current = session.instanceId;
    onAction({ controllerId: session.app.controllerId, action: "setActiveExample", args: { exampleId: "" } });
  }, [activeExampleId, exampleOptions, onAction, session]);

  //#region 🎛️InlinedPanelToggles — Document/Catalogue (top-left) and Theme/Settings (bottom-right) now toggle
  // straight from the navbar/footer bar instead of through the removed Workbench/Settings category tab.
  const workbenchToggleItems = useMemo(
    (): PanelToggleItem[] =>
      workbenchLeftTabs.map((tab) => {
        const TabIcon = tab.icon;
        return {
          id: tab.id,
          icon: <TabIcon size={16} />,
          text: tab.name,
          pressed: panels["top-left"].visible && panelActivePaths["top-left"][0] === tab.id,
          onPressedChange: (pressed: boolean) => {
            dispatch({ type: "SET_PANEL_VISIBLE", anchor: "top-left", value: pressed });
            if (pressed) dispatch({ type: "SET_PANEL_PATH", anchor: "top-left", value: [tab.id] });
          },
        };
      }),
    [workbenchLeftTabs, panels, panelActivePaths],
  );

  const settingsToggleItems = useMemo(
    (): PanelToggleItem[] =>
      settingsRightTabs.map((tab) => {
        const TabIcon = tab.icon;
        return {
          id: tab.id,
          icon: <TabIcon size={16} />,
          text: tab.name,
          pressed: panels["bottom-right"].visible && panelActivePaths["bottom-right"][0] === tab.id,
          onPressedChange: (pressed: boolean) => {
            dispatch({ type: "SET_PANEL_VISIBLE", anchor: "bottom-right", value: pressed });
            if (pressed) dispatch({ type: "SET_PANEL_PATH", anchor: "bottom-right", value: [tab.id] });
          },
        };
      }),
    [settingsRightTabs, panels, panelActivePaths],
  );
  //#endregion 🎛️InlinedPanelToggles

  const navbarItems = useMemo((): NavbarItem[] => {
    if (!session) return [];
    // Logo/title, example selector, and mode switcher render as one cluster, centered as a group in the navbar
    // (via `centered`) rather than left-anchored with fill spacers pushing the rest toward the trailing edge.
    const centerContent: ReactNode[] = [
      <div key="logoAndTitle" className="flex min-w-0 shrink-0 items-center gap-single">
        <SemioLogo className="size-workbench shrink-0" />
        <span data-slot="app-name" className={cn("px-single", shellChromeTitleClassName)}>
          {appDocumentLabel(session.app.document)}
        </span>
      </div>,
    ];
    if (exampleOptions.length > 0 && !locks.exampleId && (!studioMode || session.app.id !== S_HOME_APP_ID)) {
      centerContent.push(
        <NavbarExampleSelect
          key="fixture"
          id="playground.navbar.fixture"
          value={activeExampleId}
          options={exampleOptions}
          onValueChange={(exampleId) => {
            dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: exampleId });
            onAction({ controllerId: session.app.controllerId, action: "setActiveExample", args: { exampleId: exampleId || "" } });
          }}
        />,
      );
    }
    if (session.app.modes.length > 1) {
      centerContent.push(
        <ButtonGroup key="modes" id="playground.navbar.modes">
          {session.app.modes.map((mode) => {
            const isActive = activeModeId === mode.id;
            return (
              <ButtonGroupItem
                key={mode.id}
                id={`playground.navbar.modes.${mode.id}`}
                className={cn(isActive && interactiveActiveFillClass)}
                data-state={isActive ? "on" : undefined}
                onClick={() => applyModeChange(mode.id)}
                icon={<span className="hidden" />}
                text={resolveAppLabel(appLabelsOverlay, "mode", mode.id, mode.label)}
              />
            );
          })}
        </ButtonGroup>,
      );
    }
    const items: NavbarItem[] = [{ key: "center", centered: true, content: <div className="flex min-w-0 items-center gap-double">{centerContent}</div> }];
    if (workbenchToggleItems.length > 0) {
      items.push({ key: "workbenchToggle", content: <PanelToggleGroup items={workbenchToggleItems} /> });
    }
    return items;
  }, [activeExampleId, activeModeId, appLabelsOverlay, applyModeChange, exampleOptions, locks.exampleId, onAction, session, workbenchToggleItems]);

  const searchItems = useMemo(() => {
    if (!session) return [];
    const items: UISearchItem[] = [];
    for (const tab of flattenPanelTabLeaves(session.app.panelTabs)) {
      const tabId = panelTabKindId(tab.kind);
      items.push({
        id: `panel.${tabId}`,
        label: resolveAppLabel(appLabelsOverlay, "panelTab", tabId, tab.label),
        category: shellLabel("ui.search.category.panels"),
        icon: <Icon icon="panel-left" size="small" />,
        onSelect: () => onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } }),
      });
    }
    for (const kind of session.app.windowKinds) {
      items.push({
        id: `window.${kind.id}`,
        label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, kind.label),
        category: shellLabel("ui.search.category.windows"),
        icon: <Icon icon="app-window" size="small" />,
        onSelect: () => dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: kind.id }),
      });
    }
    const keysByActionId = new Map(session.app.keybindings.map((binding) => [binding.action.action, binding.keys]));
    const declaredActionIds = new Set<string>();
    // 📇 First window kind whose resolved actions include this id (orphan/global actions fall through to
    // the active window, then the first window) — the redirect target for arg-carrying palette entries.
    const hostWindowForAction = (actionId: string): string | undefined => {
      for (const kind of session.app.windowKinds) {
        if (resolveWindowActions(session.app, kind).some((entry) => entry.id === actionId)) return kind.id;
      }
      return activeWindowId ?? session.app.windowKinds[0]?.id;
    };
    for (const action of session.app.actions ?? []) {
      if (!action.inPalette) continue;
      declaredActionIds.add(action.id);
      const argCarrying = actionRequiresStagedForm(action);
      const resolvedActionLabel = resolveAppLabel(appLabelsOverlay, "action", action.id, action.label);
      items.push({
        id: `action.${action.id}`,
        // ✍️ Arg-carrying actions never fire from the palette (P3): the "…" entry activates the hosting
        // window, unfolds its Actions rail, and expands this action's staged form instead of dispatching.
        label: argCarrying ? `${resolvedActionLabel}…` : resolvedActionLabel,
        description: action.keys ?? keysByActionId.get(action.id),
        category: action.category ?? (action.kind === "history" ? shellLabel("ui.toolbar.parent.history") : shellLabel("ui.toolbar.parent.actions")),
        onSelect: () => {
          if (argCarrying) {
            const windowId = hostWindowForAction(action.id);
            if (windowId) {
              dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: windowId });
              dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId, value: false });
              dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId, value: action.id });
            }
            dispatch({ type: "SET_SEARCH_OPEN", value: false });
            return;
          }
          onAction({ controllerId: session.app.controllerId, action: action.id });
        },
      });
    }
    for (const binding of session.app.keybindings) {
      if (declaredActionIds.has(binding.action.action)) continue;
      items.push({
        id: `keybinding.${binding.keys}`,
        label: binding.action.action,
        description: binding.keys,
        category: shellLabel("ui.toolbar.parent.actions"),
        onSelect: () => onAction(binding.action),
      });
    }
    // 🎛️ Commands (os/plugin/app/mode) — the footer twin of the window-rail P3 redirect above: an
    // arg-carrying command never fires from the palette, it opens the bottom-middle command panel at its
    // category and expands its form instead.
    for (const { definition, source } of resolvedCommands) {
      if (!definition.inPalette) continue;
      const argCarrying = (definition.args?.length ?? 0) > 0;
      items.push({
        id: `command.${definition.id}`,
        label: argCarrying ? `${definition.label}…` : definition.label,
        description: definition.keys,
        category: commandCategoryLabel(definition.category),
        onSelect: () => {
          if (argCarrying) {
            dispatch({ type: "SET_PANEL_VISIBLE", anchor: "bottom-middle", value: true });
            dispatch({ type: "SET_PANEL_PATH", anchor: "bottom-middle", value: [FRAMEWORK_CATEGORY_COMMAND_ID, `command.category.${definition.category}`] });
            dispatch({ type: "SET_COMMAND_EXPANDED", value: definition.id });
            dispatch({ type: "SET_SEARCH_OPEN", value: false });
            return;
          }
          onCommand(source, definition.id);
        },
      });
    }
    if (studioMode && panel) {
      for (const program of panel.programs) {
        items.push({
          id: `spawn.${program.programId}`,
          label: `${shellLabel("ui.palette.spawnPrefix")} ${appDocumentLabel(program.document)}`,
          category: shellLabel("ui.search.category.catalogue"),
          onSelect: () => onAction({ controllerId: S_PLAY_CONTROLLER_ID, action: "spawnApp", args: { programId: program.programId } }),
        });
      }
      items.push(
        {
          id: "studio.undo",
          label: shellLabel("ui.palette.undo"),
          category: shellLabel("ui.search.category.studio"),
          icon: <Icon icon="undo-2" size="small" />,
          onSelect: () => onAction({ controllerId: S_PLAY_CONTROLLER_ID, action: "undo" }),
        },
        {
          id: "studio.redo",
          label: shellLabel("ui.palette.redo"),
          category: shellLabel("ui.search.category.studio"),
          icon: <Icon icon="redo-2" size="small" />,
          onSelect: () => onAction({ controllerId: S_PLAY_CONTROLLER_ID, action: "redo" }),
        },
        {
          id: "studio.home",
          label: shellLabel("ui.palette.goHome"),
          category: shellLabel("ui.search.category.navigation"),
          onSelect: () => onAction({ controllerId: S_PLAY_CONTROLLER_ID, action: "goHome" }),
        },
      );
    }
    return items;
  }, [activeWindowId, appLabelsOverlay, onAction, onCommand, panel, resolvedCommands, session, studioMode, uiLocale]);

  const modeWindows = useMemo((): ModeWindowDescriptor[] => {
    if (!session) return [];
    const actionPaneSlice: ActionPaneSlice = { expandedByWindowId: actionPaneExpandedByWindowId, stagedArgsByKey: actionPaneStagedArgsByKey, activeUtilityByWindowId };
    const actionsFoldedFor = (windowId: string, actions: readonly ActionDefinition[]) =>
      introductionActionId && actions.some((action) => action.id === introductionActionId) ? false : (actionPaneFoldedByWindowId[windowId] ?? true);
    const onActionsFoldedFor = (windowId: string) => (folded: boolean) => dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId, value: folded });
    // 🖱️ Window-body cursor follows the active utility's declared `cursor` (P5).
    const cursorFor = (app: AppDefinition, windowId: string): CSSProperties | undefined => {
      const utilityId = activeUtilityByWindowId[windowId];
      const cursor = utilityId ? (app.utilities ?? []).find((utility) => utility.id === utilityId)?.cursor : undefined;
      return cursor ? { cursor } : undefined;
    };
    if (studioMode && spawnedWindowUi && panel?.activeSpawnedId) {
      const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
      if (spawned) {
        const spawnedApp = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
        const windowKind = spawnedApp?.windowKinds[0];
        const chrome = windowKind ? spawnedWindowChromeForKind(windowKind, spawnedWindowEngagements, spawnedWindowMeasures, activeUtilityByWindowId[spawned.id], onActionStable) : undefined;
        const spawnedUtilities = spawnedApp && windowKind ? resolveUtilityNodes(spawnedApp, windowKind, activeUtilityByWindowId[spawned.id], spawned.id, appLabelsOverlay) : [];
        const spawnedActions = spawnedApp && windowKind ? resolveWindowActions(spawnedApp, windowKind) : [];
        return [
          {
            id: spawned.id,
            title: appDocumentLabel(spawned.document),
            fill: true,
            showControls: true,
            measures: chrome?.measures,
            utilityOptions: chrome?.utilityOptions,
            engagement: chrome?.engagement,
            toolbar: spawnedApp && windowKind ? utilityBarNode(spawnedUtilities, spawned.id, onActionStable, introductionUtilityId) : undefined,
            actionPane: spawnedApp && windowKind ? windowActionPaneNode(spawnedApp, windowKind, spawned.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay) : undefined,
            actionsFolded: actionsFoldedFor(spawned.id, spawnedActions),
            onActionsFoldedChange: onActionsFoldedFor(spawned.id),
            children: (
              <ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" style={spawnedApp ? cursorFor(spawnedApp, spawned.id) : undefined}>
                <InterpretedUiNode node={spawnedWindowUi} onAction={onActionStable} />
              </ChromeAwareWindowScrollSurface>
            ),
          },
        ];
      }
    }
    if (Object.keys(windowUiByKind).length === 0) return [];
    const baseWindows = session.app.windowKinds.map((kind) => {
      const utilities = resolveUtilityNodes(session.app, kind, activeUtilityByWindowId[kind.id], kind.id, appLabelsOverlay);
      const actions = resolveWindowActions(session.app, kind);
      return {
        id: kind.id,
        title: appWindowDocumentLabel(session.app, resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, kind.label)),
        fill: true,
        showControls: true,
        ...windowMeasuresChrome(windowMeasuresByKind[kind.id] ?? kind.options.measures, activeUtilityByWindowId[kind.id], kind.id, onActionStable),
        engagement: windowEngagementToSpec(resolveWindowEngagement(kind, windowEngagementsByKind), onActionStable),
        toolbar: utilityBarNode(utilities, kind.id, onActionStable, introductionUtilityId),
        actionPane: windowActionPaneNode(session.app, kind, kind.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay),
        actionsFolded: actionsFoldedFor(kind.id, actions),
        onActionsFoldedChange: onActionsFoldedFor(kind.id),
        children: (
          <ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" data-window-kind-id={kind.id} style={cursorFor(session.app, kind.id)}>
            <InterpretedUiNode node={windowUiByKind[kind.id] ?? { type: "text", value: `${shellLabel("ui.common.missingWindow")}: ${kind.id}` }} onAction={onActionStable} />
          </ChromeAwareWindowScrollSurface>
        ),
      };
    });
    const extraWindows = extraWindowInstances.flatMap((instance) => {
      const kind = session.app.windowKinds.find((entry) => entry.id === instance.windowKindId);
      if (!kind) return [];
      const utilities = resolveUtilityNodes(session.app, kind, activeUtilityByWindowId[instance.id], instance.id, appLabelsOverlay);
      const actions = resolveWindowActions(session.app, kind);
      return [
        {
          id: instance.id,
          title: instance.title,
          fill: true,
          showControls: true,
          ...windowMeasuresChrome(windowMeasuresByKind[kind.id] ?? kind.options.measures, activeUtilityByWindowId[instance.id], instance.id, onActionStable),
          engagement: windowEngagementToSpec(resolveWindowEngagement(kind, windowEngagementsByKind), onActionStable),
          toolbar: utilityBarNode(utilities, instance.id, onActionStable, introductionUtilityId),
          actionPane: windowActionPaneNode(session.app, kind, instance.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay),
          actionsFolded: actionsFoldedFor(instance.id, actions),
          onActionsFoldedChange: onActionsFoldedFor(instance.id),
          children: (
            <ChromeAwareWindowScrollSurface className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" data-window-kind-id={kind.id} style={cursorFor(session.app, instance.id)}>
              <InterpretedUiNode node={windowUiByKind[kind.id] ?? { type: "text", value: `${shellLabel("ui.common.missingWindow")}: ${kind.id}` }} onAction={onActionStable} />
            </ChromeAwareWindowScrollSurface>
          ),
        },
      ];
    });
    return [...baseWindows, ...extraWindows];
  }, [
    actionPaneExpandedByWindowId,
    actionPaneFoldedByWindowId,
    actionPaneStagedArgsByKey,
    activeUtilityByWindowId,
    appLabelsOverlay,
    extraWindowInstances,
    introductionActionId,
    introductionUtilityId,
    loadedPlugins,
    onActionStable,
    panel,
    session,
    spawnedWindowEngagements,
    spawnedWindowMeasures,
    spawnedWindowUi,
    studioMode,
    uiLocale,
    windowEngagementsByKind,
    windowMeasuresByKind,
    windowUiByKind,
  ]);

  const effectiveModeLayout = useMemo(
    () =>
      shellLayout ??
      (session
        ? convertFrameworkLayoutToModeLayout(
            session.app.defaultLayout,
            modeWindows.map((window) => window.id),
            appLabelsOverlay,
          )
        : { kind: "stack" as const, children: [] }),
    [appLabelsOverlay, modeWindows, session, shellLayout],
  );

  const canvas = useMemo(() => {
    if (!session) return <p className="p-double text-sm text-muted-foreground">{shellLabel("ui.common.loadingPlugins")}</p>;
    if (error)
      return (
        <p className="p-double text-sm text-destructive" role="alert">
          {error}
        </p>
      );
    const modes = session.app.modes.length > 0 ? session.app.modes : [{ id: session.app.id, label: appDocumentLabel(session.app.document) }];
    const studioHomeBar =
      studioMode && session.app.id === S_PLAY_APP_ID && !panel?.activeSpawnedId ? (
        <button
          type="button"
          className={cn(borderNormalBottomClass, "px-single py-single text-left text-sm text-muted-foreground hover:bg-muted/40 hover:text-foreground")}
          onClick={() => onAction({ controllerId: S_PLAY_CONTROLLER_ID, action: "goHome" })}
        >
          ← {shellLabel("ui.common.home")}
        </button>
      ) : null;
    const focusedSpawned = panel?.activeSpawnedId ? panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId) : undefined;
    const focusedBar = focusedSpawned ? (
      <div className={cn(borderNormalBottomClass, "flex items-center gap-single px-single py-single text-sm text-muted-foreground")}>
        <button type="button" className="hover:text-foreground" onClick={() => (openStudioIdRef.current ? navigateHistory(`/studios/${openStudioIdRef.current}`) : onAction({ controllerId: S_PLAY_CONTROLLER_ID, action: "closeFocusedInstance" }))}>
          ← {shellLabel("ui.common.backToMediaGraph")}
        </button>
        <span>·</span>
        <span>{appDocumentLabel(focusedSpawned.document)}</span>
      </div>
    ) : null;
    return (
      <div className="flex h-full min-h-0 flex-col overflow-hidden">
        {studioHomeBar}
        {focusedBar}
        <input
          ref={importStudioInputRef}
          type="file"
          accept="application/json,.json"
          className="hidden"
          onChange={(event) => {
            const file = event.target.files?.[0];
            if (!file) return;
            void file.text().then((json) => {
              onAction({ controllerId: S_HOME_CONTROLLER_ID, action: "importStudio", args: { json } });
              event.target.value = "";
            });
          }}
        />
        <div className="min-h-0 flex-1">
          <App
            modes={modes.map((mode) => ({ id: mode.id, label: resolveAppLabel(appLabelsOverlay, "mode", mode.id, mode.label), children: null }))}
            activeModeId={session.viewState.activeModeId ?? modes[0]?.id ?? session.app.id}
            onActiveModeChange={applyModeChange}
            chrome={false}
          >
            <Mode
              className="h-full w-full"
              mobile={mobile}
              windows={modeWindows}
              layout={effectiveModeLayout}
              activeWindowId={activeWindowId}
              onActiveWindowChange={(value) => dispatch({ type: "SET_ACTIVE_WINDOW_ID", value })}
              onLayoutChange={(value) => dispatch({ type: "SET_SHELL_LAYOUT", value })}
              onTemplateDrop={mobile ? undefined : handleTemplateDrop}
              onWindowClose={(windowId) => {
                if (studioMode && panel?.spawnedApps.some((entry) => entry.id === windowId)) {
                  const nextSpawned = panel.spawnedApps.filter((entry) => entry.id !== windowId);
                  updateStudioPanel(buildStudioPanelState(panel.programs, nextSpawned, panel.activePanelTab, nextSpawned[0]?.id));
                }
                dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: (current) => current.filter((entry) => entry.id !== windowId) });
                dispatch({
                  type: "SET_SHELL_LAYOUT",
                  value: (current) =>
                    current ??
                    convertFrameworkLayoutToModeLayout(
                      session.app.defaultLayout,
                      modeWindows.map((window) => window.id),
                      appLabelsOverlay,
                    ),
                });
              }}
            />
          </App>
        </div>
      </div>
    );
  }, [activeWindowId, effectiveModeLayout, error, handleTemplateDrop, mobile, modeWindows, navigateHistory, onAction, panel, session, studioMode, uiLocale, updateStudioPanel]);

  const buildPanelProps = useCallback(
    (anchor: PanelAnchor) => ({
      visible: panels[anchor].visible,
      onVisibleChange: (value: boolean) => dispatch({ type: "SET_PANEL_VISIBLE", anchor, value }),
      size: panels[anchor].size,
      onSizeChange: (value: number) => dispatch({ type: "SET_PANEL_SIZE", anchor, value }),
      // 🎛️ Document/Catalogue and Theme/Settings toggle from the navbar/footer now (see workbenchToggleItems/
      // settingsToggleItems) — their own floating panel no longer needs a redundant tab bar of its own.
      hideTabBar: anchor === "top-left" || anchor === "bottom-right",
      tabs: dock.anchors[anchor],
      activeTabPath: panelActivePaths[anchor],
      onActiveTabPathChange: (path: readonly string[]) => {
        dispatch({ type: "SET_PANEL_PATH", anchor, value: path });
        // 🎛️ Command palette only: switching category leaves always collapses any expanded arg form — the
        // next hierarchy level up only makes sense under its own category's command list (mirrors the old
        // dedicated `SET_COMMAND_CATEGORY` reducer case, now expressed at the generic path-change call site
        // since category-active state itself is just this anchor's `activeTabPath`). Categories sit under
        // the Command branch, so compare the category segment (path[1]), not the shared branch root.
        if (anchor === "bottom-middle" && panels[anchor].path[1] !== path[1]) {
          dispatch({ type: "SET_COMMAND_EXPANDED", value: null });
        }
        const tabId = path[path.length - 1];
        // 🌱 Progressive paths often end at a branch (or are empty) — only leaves are meaningful "active panel tab" selections.
        if (tabId && studioMode && session?.app.id === S_PLAY_APP_ID && findPanelTabNode(dock.anchors[anchor], path)?.kind === "leaf") {
          onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } });
        }
      },
      pathMemory: panelPathMemory,
      onPathMemoryChange: (value: Readonly<Record<string, string>>) => dispatch({ type: "SET_PANEL_PATH_MEMORY", value }),
      treeOpenStates,
      onTreeOpenStateChange: (id: string, open: boolean) => dispatch({ type: "SET_TREE_OPEN_STATE", id, open }),
    }),
    [panelActivePaths, panels, dock, onAction, panelPathMemory, session, studioMode, treeOpenStates],
  );

  return (
    <UIFindProvider>
      <LevelProvider level="window">
        <div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>
          <PanelDockProvider dock={dock} onTabDockDrop={handleTabDockDrop} onTreeUnitDockDrop={handleTreeUnitDockDrop}>
            <Layout
              mobile={mobile}
              mobilePanel={mobilePanel}
              navbar={<Navbar items={navbarItems} showFullscreenToggle />}
              footer={
                <Footer
                  items={[
                    // 🏛️ Fill spacer keeps the funding credit and the Settings corner toggle grouped together at the
                    // right edge (between the centered command palette and the true corner) instead of each fighting
                    // ms-auto for the same pixel where the floating bottom-right Panel also anchors.
                    navbarFillItem("footerTrailingFill"),
                    fundedByZukunftBauFooterItem(),
                    ...(settingsToggleItems.length > 0 ? [{ key: "settingsToggle", content: <PanelToggleGroup items={settingsToggleItems} /> }] : []),
                  ]}
                />
              }
              panels={{
                "top-left": buildPanelProps("top-left"),
                "top-middle": buildPanelProps("top-middle"),
                "top-right": buildPanelProps("top-right"),
                "bottom-left": buildPanelProps("bottom-left"),
                "bottom-middle": buildPanelProps("bottom-middle"),
                "bottom-right": buildPanelProps("bottom-right"),
              }}
              canvas={<ShellRenderErrorBoundary>{canvas}</ShellRenderErrorBoundary>}
            />
          </PanelDockProvider>
        </div>
        <UISearch items={searchItems} open={searchOpen} onOpenChange={(value) => dispatch({ type: "SET_SEARCH_OPEN", value })} />
        <UIFind open={findOpen} onOpenChange={(value) => dispatch({ type: "SET_FIND_OPEN", value })} />
        {session?.app.introduction && introductionStepIndex != null && (
          <UIIntroduction
            introduction={resolveIntroductionDefinition(session.app.introduction, appLabelsOverlay)}
            stepIndex={introductionStepIndex}
            anchorFallbackSelectors={introductionAnchorFallbackSelectors}
            onStepIndexChange={(value) => dispatch({ type: "SET_INTRODUCTION_STEP", value })}
            onDismiss={() => {
              dispatch({ type: "SET_INTRODUCTION_STEP", value: null });
              writeStoredIntroductionSeen(session.app.id);
            }}
          />
        )}
        {session &&
          overlayDialog &&
          (() => {
            const dialog = session.app.dialogs?.find((entry) => entry.id === overlayDialog.dialogId);
            if (!dialog) return null;
            return (
              <UIDialog
                dialog={resolveDialogDefinition(dialog, appLabelsOverlay)}
                seedArgs={overlayDialog.seedArgs}
                renderField={(def, value, onChange) => renderStagedArgControl(def, value, onChange)}
                onSubmit={(args) => {
                  dispatch({ type: "SET_DIALOG", value: null });
                  onAction({ controllerId: session.app.controllerId, action: dialog.submitAction, args });
                }}
                onCancel={() => {
                  dispatch({ type: "SET_DIALOG", value: null });
                  if (dialog.cancelAction) onAction({ controllerId: session.app.controllerId, action: dialog.cancelAction });
                }}
              />
            );
          })()}
      </LevelProvider>
    </UIFindProvider>
  );
}
//#endregion FrameworkOsShell

//#region 🔖plugin-runtime

export type PluginWasmHandle = {
  readonly pluginId: string;
  readonly manifest: PluginManifest;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  readonly handleAction: (instanceId: number, actionJson: string, viewState: ViewState) => Promise<InvocationResponse>;
  /** 🎛️ Dispatches a scoped command (os/plugin/app/mode) — optional since not every plugin declares commands. */
  readonly handleCommand?: (instanceId: number, commandJson: string, viewState: ViewState) => Promise<InvocationResponse>;
  readonly render: (instanceId: number, bodyKey: string, viewState: ViewState) => Promise<UiNode>;
  readonly renderWithDocument?: (instanceId: number, bodyKey: string, viewState: ViewState, documentJson: string) => Promise<UiNode>;
  readonly refreshUi: (instanceId: number, request: PluginUiRefreshRequest) => Promise<PluginUiRefreshResponse>;
  /** 🔗 The `DocumentApp` document-sync surface (WS-D) — optional since not every plugin has migrated onto it yet (WS-F). */
  readonly applyOperations?: (instanceId: number, operationsJson: string) => Promise<void>;
  readonly readAppDocument?: (instanceId: number) => Promise<string>;
  readonly loadAppDocument?: (instanceId: number, documentJson: string) => Promise<void>;
  readonly attachBackbone?: (instanceId: number, uri: string) => Promise<void>;
  readonly detachBackbone?: (instanceId: number) => Promise<void>;
  readonly dispose: () => void;
};

export type { PluginRegistryEntry };
export { DEFAULT_PLUGIN_REGISTRY };

export async function loadPluginModule(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  return adaptPluginHandle(await loadCorePluginModule(pluginId, moduleUrl));
}

export async function loadPluginWasm(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  return adaptPluginHandle(await loadCorePluginWasm(pluginId, moduleUrl));
}

function adaptPluginHandle(handle: CorePluginWasmHandle): PluginWasmHandle {
  return {
    pluginId: handle.pluginId,
    manifest: handle.manifest as unknown as PluginManifest,
    createApp: (appId) => handle.createApp(appId),
    destroyApp: (instanceId) => handle.destroyApp(instanceId),
    handleAction: (instanceId, actionJson, viewState) => handle.handleAction(instanceId, actionJson, viewState),
    handleCommand: handle.handleCommand ? (instanceId, commandJson, viewState) => handle.handleCommand!(instanceId, commandJson, viewState) : undefined,
    render: async (instanceId, bodyKey, viewState) => (await handle.render(instanceId, bodyKey, viewState)) as unknown as UiNode,
    renderWithDocument: handle.renderWithDocument ? async (instanceId, bodyKey, viewState, documentJson) => (await handle.renderWithDocument!(instanceId, bodyKey, viewState, documentJson)) as unknown as UiNode : undefined,
    refreshUi: (instanceId, request) => handle.refreshUi(instanceId, request),
    applyOperations: handle.applyOperations ? (instanceId, operationsJson) => handle.applyOperations!(instanceId, operationsJson) : undefined,
    readAppDocument: handle.readAppDocument ? (instanceId) => handle.readAppDocument!(instanceId) : undefined,
    loadAppDocument: handle.loadAppDocument ? (instanceId, documentJson) => handle.loadAppDocument!(instanceId, documentJson) : undefined,
    attachBackbone: handle.attachBackbone ? (instanceId, uri) => handle.attachBackbone!(instanceId, uri) : undefined,
    detachBackbone: handle.detachBackbone ? (instanceId) => handle.detachBackbone!(instanceId) : undefined,
    dispose: () => handle.dispose(),
  };
}
//#endregion 🔖plugin-runtime

//#region 🔖wasm-session-loader

//#region GraphSession
type GraphSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly GraphSession: new () => GraphWasmSession;
};

let graphSessionPromise: Promise<GraphSessionModule> | null = null;

export async function createGraphSession(): Promise<GraphWasmSession> {
  if (!graphSessionPromise) {
    graphSessionPromise = import("@semio-tech/framework-graph-rs/pkg/framework_graph.js").then(async (mod) => {
      await mod.default();
      return mod as GraphSessionModule;
    });
  }
  const mod = await graphSessionPromise;
  return new mod.GraphSession();
}
//#endregion GraphSession

//#region FlowSession
export type FlowWasmSession = GraphWasmSession & {
  loadFixtureJson(json: string): void;
  fixtureJson(): string;
  syncFromSceneJson?(json: string): void;
  setSelection(json: string): void;
  setPreviewOff(json: string): void;
  setCatalogueJson(json: string): void;
  setNeuronKindInfosJson(json: string): void;
  setComputingProgress(json: string): void;
  setAutomaticLod(enabled: boolean): void;
  setForcedDrawLodLabel(label: string): void;
  setCanvasThemeJson(json: string): void;
  setCamera(x: number, y: number, zoom: number): void;
  pointerDownScreen(sx: number, sy: number, button: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean, pan: boolean): void;
  pointerMoveScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerUpScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  wheelScreen(sx: number, sy: number, deltaX: number, deltaY: number, zoomGesture: boolean): void;
  labelOverlayPaintStateJson(): string;
  paramOverlayPaintStateJson(): string;
  stepperOverlayStateJson(): string;
  sliderOverlayStateJson(): string;
  selectionUnionBoundsScreenJson(): string;
  selectionPreviewPointsJson(): string;
  selectionPreviewCrossing(): boolean;
  selectedWidgetIds(): string;
  hoveredWidgetId(): string | undefined;
  hoveredChannelJson(): string;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  previewText(): string;
  preselectWidgetIdsJson(): string;
  previewOffWidgetIds(): string;
  alignSelection(mode: string): void;
  undo(): boolean;
  redo(): boolean;
  selectAll(): void;
  deleteSelection(): void;
  addWidget(descriptorJson: string, worldX: number, worldY: number): string;
  setGhostWidget(descriptorJson: string, worldX: number, worldY: number): void;
  clearGhostWidget(): void;
  worldFromScreen(sx: number, sy: number): string;
  evaluateSync(): string;
  noteInsertText(chunk: string): void;
  noteBackspace(): void;
  noteDeleteForward(): void;
  noteCommitEdit(): void;
  noteMoveCaret(direction: string, extend: boolean): void;
  setSliderValue(widgetId: string, value: number): void;
  setStepperFieldValue(widgetId: string, fieldKey: string, value: number): void;
  setNeuronParams(widgetId: string, paramsJson: string): void;
  setHover?(widgetId: string | null): void;
  setHoverChannel?(widgetId: string | null, port?: string | null): void;
  cameraJson?(): string;
};

type FlowSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly FlowSession: new () => FlowWasmSession;
};

let flowSessionPromise: Promise<FlowSessionModule> | null = null;

export async function createFlowSession(): Promise<FlowWasmSession> {
  if (!flowSessionPromise) {
    flowSessionPromise = import("@semio-tech/flow-core/pkg/flow_core.js").then(async (mod) => {
      await mod.default();
      return mod as FlowSessionModule;
    });
  }
  const mod = await flowSessionPromise;
  return new mod.FlowSession();
}
//#endregion FlowSession

//#region EditorSession
export type EditorWasmSession = GraphWasmSession & {
  syncFromSceneJson(json: string): void;
  setText(text: string): void;
  text(): string;
  caret(): number;
  anchor(): number;
  pointerDownScreen(sx: number, sy: number, button: number): void;
  pointerMoveScreen(sx: number, sy: number, buttons: number): void;
  pointerUpScreen(sx: number, sy: number, buttons: number): void;
  wheelScrollScreen(deltaY: number): void;
  insertText(text: string): void;
  backspace(): void;
  deleteForward(): void;
  selectAll(): void;
  replaceSelection(text: string): void;
  selectionText(): string;
  setCanvasThemeJson(json: string): void;
  hoverTokenRangeJson(): string;
  setHoverRange(start: number, end: number): void;
  cameraJson(): string;
  moveLeft(extend: boolean): void;
  moveRight(extend: boolean): void;
  moveUp(extend: boolean): void;
  moveDown(extend: boolean): void;
  moveLineStart(extend: boolean): void;
  moveLineEnd(extend: boolean): void;
  tabInsertText(): string;
  setSelectionRange(anchor: number, caret: number): void;
  selectSpanAt(offset: number): void;
  selectSpanAtScreen(sx: number, sy: number): void;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  caretWorldJson(): string;
  worldToScreenJson(wx: number, wy: number): string;
  setSelectionOccurrencesJson(json: string): void;
  setExtraCaretsJson(json: string): void;
  setCaretVisible(visible: boolean): void;
};

type EditorSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly EditorSession: new () => EditorWasmSession;
};

let editorSessionPromise: Promise<EditorSessionModule> | null = null;

export async function createEditorSession(): Promise<EditorWasmSession> {
  if (!editorSessionPromise) {
    editorSessionPromise = import("@semio-tech/framework-editor-rs/pkg/framework_editor.js").then(async (mod) => {
      await mod.default();
      return mod as EditorSessionModule;
    });
  }
  const mod = await editorSessionPromise;
  return new mod.EditorSession();
}
//#endregion EditorSession

//#region RasterSession
export type RasterWasmSession = {
  gpuReady(): boolean;
  attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
  setCamera(x: number, y: number, zoom: number): void;
  wheelScreen(sx: number, sy: number, deltaY: number): void;
  pointerDownScreen(sx: number, sy: number, button: number): void;
  pointerMoveScreen(sx: number, sy: number): void;
  pointerUpScreen(sx: number, sy: number): void;
  syncDocumentJson(json: string): void;
  uploadLayerImage(layerId: string, bytes: Uint8Array): void;
  uploadRasterImageKey(key: string, bytes: Uint8Array): void;
  setActiveUtility(utility: string): void;
  setBrushSize(size: number): void;
  setBrushOpacity(opacity: number): void;
  setHoveredIdSilent(id?: string | null): void;
  setSelectionIdsJson(json: string): void;
  setCanvasThemeJson(json: string): void;
  cameraJson(): string;
  setViewMode(mode: string, layerId?: string | null): void;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  marqueeHitsJson(queryJson: string): string;
  navigatorFitCameraJson(viewportW: number, viewportH: number): string;
  navigatorViewportOverlayJson(contentCameraJson: string, contentViewportJson: string): string;
  free(): void;
};

type RasterSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly RasterSession: new () => RasterWasmSession;
};

let rasterSessionPromise: Promise<RasterSessionModule> | null = null;

export async function createRasterSession(): Promise<RasterWasmSession> {
  if (!rasterSessionPromise) {
    rasterSessionPromise = import("@semio-tech/raster-rs/pkg/raster.js").then(async (mod) => {
      await mod.default();
      return mod as RasterSessionModule;
    });
  }
  const mod = await rasterSessionPromise;
  return new mod.RasterSession();
}
//#endregion RasterSession

//#region MapSession
export type MapWasmSession = {
  attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
  setCamera(x: number, y: number, zoom: number): void;
  cameraJson(): string;
  cameraLimitsJson(): string;
  fitWorldCamera(): void;
  reclampCamera(): void;
  pointerDownScreen(sx: number, sy: number, button: number): void;
  pointerMoveScreen(sx: number, sy: number): void;
  pointerUpScreen(sx: number, sy: number): void;
  wheelScreen(sx: number, sy: number, deltaY: number): void;
  syncMapJson(json: string): void;
  uploadTile(z: number, x: number, y: number, bytes: Uint8Array): void;
  uploadVectorTile(z: number, x: number, y: number, bytes: Uint8Array): void;
  visibleTilesJson(): string;
  visibleVectorTilesJson(): string;
  setRenderMode(mode: string): void;
  setVectorStyle(style: string): void;
  setLodMode(mode: string): void;
  setLayerVisibilityJson(json: string): void;
  setLayerStrokeScaleJson(json: string): void;
  setSelectionJson(json: string): void;
  setHoverJson(json: string): void;
  featuresInRectJson(x0: number, y0: number, x1: number, y1: number, crossing: boolean): string;
  featuresInPolygonJson(pointsJson: string, crossing: boolean): string;
  hitTestFeatureJson(sx: number, sy: number): string;
  featureScreenJson(kind: string, id: string): string;
  positionScreenJson(id: string): string;
  currentLodJson(): string;
  setMapThemeJson(json: string): void;
  gpuReady(): boolean;
  free(): void;
};

type MapSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly MapSession: new () => MapWasmSession;
};

let mapSessionPromise: Promise<MapSessionModule> | null = null;

export async function createMapSession(): Promise<MapWasmSession> {
  if (!mapSessionPromise) {
    mapSessionPromise = import("@semio-tech/gis-2d-rs/pkg/gis_2d.js").then(async (mod) => {
      await mod.default();
      return mod as MapSessionModule;
    });
  }
  const mod = await mapSessionPromise;
  return new mod.MapSession();
}

//#region TerrainSession
export type TerrainWasmSession = {
  set_project_origin(lon: number, lat: number): void;
  set_exaggeration(exaggeration: number): void;
  visible_terrain_tiles_json(cameraJson: string): string;
  upload_elevation_tile(z: number, x: number, y: number, bytes: Uint8Array): boolean;
  evict_terrain_tile(z: number, x: number, y: number): void;
  terrain_tile_mesh_json(z: number, x: number, y: number): string;
};

type TerrainSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly TerrainSession: new () => TerrainWasmSession;
};

let terrainSessionPromise: Promise<TerrainSessionModule> | null = null;

export async function createTerrainSession(): Promise<TerrainWasmSession> {
  if (!terrainSessionPromise) {
    terrainSessionPromise = import("@semio-tech/gis-3d-rs/pkg/gis_3d.js").then(async (mod) => {
      await mod.default();
      return mod as TerrainSessionModule;
    });
  }
  const mod = await terrainSessionPromise;
  return new mod.TerrainSession();
}
//#endregion TerrainSession

export type Puzzle2dBoardWasmSession = {
  attach_canvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown>;
  setSize(width: number, height: number, dpr: number): void;
  renderFrame(): void;
  parseFixtureJson(json: string): boolean;
  syncDescriptorJson(json: string): void;
  setKindCatalogsJson(json: string): void;
  setCamera(x: number, y: number, zoom: number): void;
  setSelectionIdsJson(json: string): void;
  setCanvasThemeJson(json: string): void;
  pointerDownScreen(sx: number, sy: number, button: number, shift: boolean, ctrlOrMeta: boolean): void;
  pointerMoveScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerUpScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  wheelScreen(sx: number, sy: number, deltaY: number): void;
  drainEventsJson(): string;
  cameraJson(): string;
  gpuReady(): boolean;
  setHoveredIdSilent?(id?: string | null): void;
  setActiveUtility?(label: string): void;
  setSelectionOptions?(method: string, mode: string, selectNodes: boolean, selectEdges: boolean, selectHandles: boolean): void;
  setGridSnapEnabled?(enabled: boolean): void;
  setGridFactor?(v: number): void;
  setSuggestionOffset?(distance: number): void;
  setBrushKindWeights?(json: string): void;
  setHandleLinkCompatJson?(json: string): void;
  setAutomaticLod?(enabled: boolean): void;
  setForcedDrawLodLabel?(label: string): void;
  setSelectionIdsJsonSilent?(json: string): void;
  setCameraSilent?(x: number, y: number, zoom: number): void;
  pointerLeaveScreen?(alt: boolean): void;
  pickTargetsAtScreenJson?(sx: number, sy: number): string;
  deleteSelection?(): void;
  cancelAreaSelect?(): boolean;
  brushCycleCandidate?(forward: boolean): void;
  setFixtureDropPreviewJson?(json: string): void;
  clearFixtureDropPreview?(): void;
  defersDescriptorSyncFromJs?(): boolean;
  isDraggingAreaSelect?(): boolean;
  /** @emoji 🐢 Silent cross-pane mirror setters (WS-live-sync round 4) — move nodes/set preselect/set the marquee outline without emitting board events or a fixture reset, so a peer pane can mirror another pane's live gesture without round-tripping through the plugin. */
  setNodePositionsJson?(json: string): void;
  setPreselectStateJsonSilent?(json: string): void;
  setSelectionScreenPreview?(flatXy: readonly number[]): void;
  clearSelectionScreenPreview?(): void;
  free(): void;
};

type Puzzle2dBoardSessionModule = {
  readonly default: (input?: unknown) => Promise<unknown>;
  readonly BoardSession: new () => Puzzle2dBoardWasmSession;
};

let puzzle2dBoardSessionPromise: Promise<Puzzle2dBoardSessionModule> | null = null;

export async function createPuzzle2dBoardSession(): Promise<Puzzle2dBoardWasmSession> {
  if (!puzzle2dBoardSessionPromise) {
    puzzle2dBoardSessionPromise = import("@semio-tech/puzzle-2d-rs/pkg/puzzle_2d.js").then(async (mod) => {
      await mod.default();
      return mod as Puzzle2dBoardSessionModule;
    });
  }
  const mod = await puzzle2dBoardSessionPromise;
  return new mod.BoardSession();
}
//#endregion MapSession

//#region SceneHelpers
export function isFlowGraphScene(capabilitiesJson?: string): boolean {
  if (!capabilitiesJson) return false;
  try {
    const caps = JSON.parse(capabilitiesJson) as { readonly engine?: string; readonly spotlight?: boolean; readonly noteEdit?: boolean };
    return caps.engine === "flow" || caps.spotlight === true || caps.noteEdit === true;
  } catch {
    return false;
  }
}
//#endregion SceneHelpers
//#endregion 🔖wasm-session-loader

//#region 🔖ui-search-find

//#region UISearch
export type UISearchItem = {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly icon?: ReactNode;
  readonly category?: string;
  readonly onSelect: () => void;
};

export function UISearch({
  items,
  open,
  onOpenChange,
  placeholder = shellLabel("ui.search.placeholder"),
  emptyMessage = shellLabel("ui.search.empty"),
}: {
  readonly items: readonly UISearchItem[];
  readonly open: boolean;
  readonly onOpenChange: (open: boolean) => void;
  readonly placeholder?: string;
  readonly emptyMessage?: string;
}) {
  const [query, setQuery] = useState("");
  const fuse = useMemo(
    () =>
      new Fuse(items, {
        keys: [
          { name: "label", weight: 2 },
          { name: "description", weight: 1 },
          { name: "category", weight: 0.5 },
        ],
        threshold: 0.4,
        includeScore: true,
      }),
    [items],
  );
  const results = useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);
    return items.slice(0, 20).map((item, idx) => ({ item, refIndex: idx, score: 0 }) as FuseResult<UISearchItem>);
  }, [fuse, items, query]);
  const grouped = useMemo(() => {
    const groups: Record<string, FuseResult<UISearchItem>[]> = {};
    for (const result of results) {
      const category = result.item.category || "";
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    }
    return groups;
  }, [results]);
  const handleSelect = useCallback(
    (item: UISearchItem) => {
      onOpenChange(false);
      setQuery("");
      item.onSelect();
    },
    [onOpenChange],
  );

  return (
    <CommandDialog title={shellLabel("ui.search.title")} description={shellLabel("ui.search.description")} open={open} onOpenChange={onOpenChange} shouldFilter={false}>
      <CommandInput id="ui.search.input" placeholder={placeholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{emptyMessage}</CommandEmpty>
        {Object.entries(grouped).map(([category, categoryResults]) => (
          <CommandGroup key={category || "__default"} heading={category || undefined}>
            {categoryResults.map((result, idx) => (
              <CommandItem key={`${result.item.id}-${idx}`} value={`${result.item.label} ${result.item.description ?? ""} ${result.item.category ?? ""}`.trim()} onSelect={() => handleSelect(result.item)}>
                <div className="flex items-center gap-single">
                  {result.item.icon}
                  <div className="flex flex-col">
                    <span>{result.item.label}</span>
                    {result.item.description ? <span className="text-xs text-muted-foreground">{result.item.description}</span> : null}
                  </div>
                </div>
              </CommandItem>
            ))}
          </CommandGroup>
        ))}
      </CommandList>
    </CommandDialog>
  );
}
//#endregion UISearch

//#region UIFind
export type UIFindItem = {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly category?: string;
};

export type UIFindContextValue = {
  readonly findItems: readonly UIFindItem[];
  readonly setFindItems: (items: readonly UIFindItem[]) => void;
  readonly setOnFindItem: (callback: ((itemId: string) => void) | undefined) => void;
  readonly triggerFindItem: (itemId: string) => void;
};

const UIFindContext = createContext<UIFindContextValue | null>(null);

function areFindItemsShallowEqual(previousItems: readonly UIFindItem[], nextItems: readonly UIFindItem[]): boolean {
  if (previousItems === nextItems) return true;
  if (previousItems.length !== nextItems.length) return false;
  for (let index = 0; index < nextItems.length; index += 1) {
    const previous = previousItems[index];
    const next = nextItems[index];
    if (!previous || !next || previous.id !== next.id || previous.label !== next.label || previous.description !== next.description || previous.category !== next.category) {
      return false;
    }
  }
  return true;
}

export function UIFindProvider({ children }: { readonly children: ReactNode }) {
  const [findItems, setFindItemsState] = useState<readonly UIFindItem[]>([]);
  const onFindItemCallbackRef = useRef<((itemId: string) => void) | undefined>(undefined);
  const setFindItems = useCallback((items: readonly UIFindItem[]) => {
    setFindItemsState((previousItems) => (areFindItemsShallowEqual(previousItems, items) ? previousItems : items));
  }, []);
  const setOnFindItem = useCallback((callback: ((itemId: string) => void) | undefined) => {
    onFindItemCallbackRef.current = callback;
  }, []);
  const triggerFindItem = useCallback((itemId: string) => {
    onFindItemCallbackRef.current?.(itemId);
  }, []);
  const contextValue = useMemo(() => ({ findItems, setFindItems, setOnFindItem, triggerFindItem }), [findItems, setFindItems, setOnFindItem, triggerFindItem]);
  return <UIFindContext.Provider value={contextValue}>{children}</UIFindContext.Provider>;
}

export function useUIFind(): UIFindContextValue {
  const context = useContext(UIFindContext);
  if (!context) throw new Error("useUIFind must be used within UIFindProvider");
  return context;
}

export function useUIFindSafe(): UIFindContextValue | null {
  return useContext(UIFindContext);
}

export function UIFind({
  open,
  onOpenChange,
  placeholder = shellLabel("ui.find.placeholder"),
  emptyMessage = shellLabel("ui.find.empty"),
}: {
  readonly open: boolean;
  readonly onOpenChange: (open: boolean) => void;
  readonly placeholder?: string;
  readonly emptyMessage?: string;
}) {
  const [query, setQuery] = useState("");
  const findContext = useContext(UIFindContext);
  const findItems = findContext?.findItems ?? [];
  const triggerFindItem = findContext?.triggerFindItem;
  const fuse = useMemo(
    () =>
      new Fuse(findItems, {
        keys: [
          { name: "label", weight: 2 },
          { name: "description", weight: 1 },
          { name: "category", weight: 0.5 },
        ],
        threshold: 0.4,
        includeScore: true,
      }),
    [findItems],
  );
  const results = useMemo(() => {
    if (query.trim()) return fuse.search(query).slice(0, 20);
    return findItems.slice(0, 20).map((item, idx) => ({ item, refIndex: idx, score: 0 }) as FuseResult<UIFindItem>);
  }, [findItems, fuse, query]);
  const grouped = useMemo(() => {
    const groups: Record<string, FuseResult<UIFindItem>[]> = {};
    for (const result of results) {
      const category = result.item.category || "";
      if (!groups[category]) groups[category] = [];
      groups[category].push(result);
    }
    return groups;
  }, [results]);
  const handleSelect = useCallback(
    (item: UIFindItem) => {
      onOpenChange(false);
      setQuery("");
      triggerFindItem?.(item.id);
    },
    [onOpenChange, triggerFindItem],
  );

  if (!findContext) return null;

  return (
    <CommandDialog title={shellLabel("ui.find.title")} description={shellLabel("ui.find.description")} open={open} onOpenChange={onOpenChange} shouldFilter={false}>
      <CommandInput id="ui.find.input" placeholder={placeholder} value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>{emptyMessage}</CommandEmpty>
        {Object.entries(grouped).map(([category, categoryResults]) => (
          <CommandGroup key={category || "__default"} heading={category || undefined}>
            {categoryResults.map((result, idx) => (
              <CommandItem key={`${result.item.id}-${idx}`} value={`${result.item.label} ${result.item.description ?? ""} ${result.item.category ?? ""}`.trim()} onSelect={() => handleSelect(result.item)}>
                <div className="flex flex-col">
                  <span>{result.item.label}</span>
                  {result.item.description ? <span className="text-xs text-muted-foreground">{result.item.description}</span> : null}
                </div>
              </CommandItem>
            ))}
          </CommandGroup>
        ))}
      </CommandList>
    </CommandDialog>
  );
}
//#endregion UIFind
//#endregion 🔖ui-search-find

//#region 🔖sync-attach-card

type SyncAttachCardProps = {
  readonly activeUri: string | null;
  readonly cardKind: SyncCardKind | null;
  readonly draftPath: string;
  readonly syncUtilities: readonly FrameworkSyncUtilityLeaf[];
  readonly status: DocumentSyncStatus | null;
  readonly onAction: (action: ActionDescriptor) => void;
  readonly onDraftPathChange: (value: string) => void;
  readonly onClose: () => void;
  readonly onAttach: (uri: string) => void;
  readonly onDetach: () => void;
};

/** 🚦 Minimal status label for a `DocumentSyncStatus` — matches this file's small-badge-text style
 * (see the `activeUri` line right below it), not a new component system. */
function syncStatusLabel(status: DocumentSyncStatus | null): string | null {
  if (!status) return null;
  const remote =
    status.remote.kind === "live" ? `live · ${status.remote.peerCount} peer${status.remote.peerCount === 1 ? "" : "s"}` : status.remote.kind === "connecting" ? "connecting…" : status.remote.kind === "backoff" ? "reconnecting…" : "offline";
  const persisted = status.persisted ? "saved" : "unsaved";
  const pending = status.pendingOps > 0 ? ` · ${status.pendingOps} pending` : "";
  return `${remote} · ${persisted}${pending}`;
}

function SyncAttachCard({ activeUri, cardKind, draftPath, syncUtilities, status, onAction, onDraftPathChange, onClose, onAttach, onDetach }: SyncAttachCardProps): ReactElement {
  const open = cardKind != null;
  const placeholder = cardKind === "remote" ? "127.0.0.1:8787/demo" : cardKind === "folder" ? "/absolute/project/folder" : "/absolute/document.json";

  const attachFromDraft = () => {
    if (!cardKind || !draftPath.trim()) return;
    if (cardKind === "remote") {
      const slash = draftPath.indexOf("/");
      const hostPort = slash > 0 ? draftPath.slice(0, slash) : draftPath;
      const documentId = slash > 0 ? draftPath.slice(slash + 1) : "document";
      onAttach(buildRemoteBackboneUri(hostPort, documentId));
      return;
    }
    onAttach(cardKind === "folder" ? buildFolderBackboneUri(draftPath) : buildFileBackboneUri(draftPath));
  };

  return (
    <Popover
      open={open}
      onOpenChange={(nextOpen) => {
        if (!nextOpen) onClose();
      }}
    >
      <PopoverAnchor asChild>
        <div>
          <UtilityTree utilities={groupUtilityNodesByCategory(syncUtilities as readonly UtilityNode[], ["sync"])} onAction={onAction} />
        </div>
      </PopoverAnchor>
      {open ? (
        <PopoverContent side="top" align="center" className="w-80 space-y-3 p-3">
          <div className="space-y-1">
            <p className="text-sm font-medium capitalize">{cardKind} backbone</p>
            {activeUri ? <p className="break-all text-xs text-muted-foreground">{activeUri}</p> : null}
            {activeUri && status ? <p className="text-xs text-muted-foreground">{syncStatusLabel(status)}</p> : null}
          </div>
          <Input value={draftPath} placeholder={placeholder} onChange={(event) => onDraftPathChange(event.target.value)} />
          <div className="flex items-center gap-2">
            <Button type="button" onClick={attachFromDraft}>
              Attach
            </Button>
            {activeUri ? (
              <Button type="button" onClick={onDetach}>
                Detach
              </Button>
            ) : null}
          </div>
        </PopoverContent>
      ) : null}
    </Popover>
  );
}
//#endregion 🔖sync-attach-card

//#region 🔖utility-tree

type UtilityTreeProps = {
  readonly utilities: readonly UtilityNode[];
  readonly onAction: (action: ActionDescriptor) => void;
  readonly id?: string;
  /** @emoji 🎀 `up` stacks a new ribbon line above the base row per pressed collection (window toolbar); `inline` keeps the horizontal drill-down (footer). */
  readonly direction?: RibbonDirection;
  /** @emoji 🎓 A utility id the introduction walkthrough is anchored on — when it names a leaf nested inside
   * a collapsed group picker, the picker auto-drills into that group so the leaf actually mounts (see
   * {@link findUtilityGroupPath}). `null`/not-found leaves `activePath` alone. */
  readonly revealUtilityId?: string | null;
};

function resolveLeafAction(node: UtilityLeaf | Extract<UtilityNode, { readonly kind: "button" | "toggle" }>): ActionDescriptor | null {
  if ("onPress" in node && node.onPress) return node.onPress;
  if ("onChange" in node && node.onChange) return node.onChange;
  if (node.kind === "button" || node.kind === "toggle") {
    if (!node.action || !node.controllerId) return null;
    return { controllerId: node.controllerId, action: node.action, args: node.args as Record<string, unknown> | undefined };
  }
  return null;
}

function utilityIcon(iconId: string): IconName {
  return iconId in ICONS ? (iconId as IconName) : "circle";
}

/** @emoji 🔢 Sorts toolbar nodes by `order`. */
export function sortUtilityNodes(nodes: readonly UtilityNode[]): UtilityNode[] {
  return [...nodes].sort((left, right) => (left.order ?? 0) - (right.order ?? 0));
}

//#region 🗂️UtilityCategoryGrouping

const UTILITY_CATEGORY_ORDER: readonly UtilityCategory[] = ["selection", "tools", "history", "sync"];

/** @emoji 🪟 Categories that are scoped to whatever window/pane the user is interacting with — selecting or editing content varies per window, so these live in each window's own bottom-left panel. */
const UTILITY_CATEGORIES: readonly UtilityCategory[] = ["selection", "tools"];

const UTILITY_CATEGORY_ICON_ID: Readonly<Record<UtilityCategory, string>> = {
  selection: "mouse-pointer",
  tools: "wrench",
  history: "undo",
  sync: "cloud",
};

function utilityNodeCategory(node: UtilityNode): UtilityCategory {
  if (node.kind === "separator") return "tools";
  if (node.category) return node.category;
  return "tools";
}

/**
 * 🕰️ Framework-owned History utility nodes derived from an app's registry (the six injected History
 * actions). Sources the bottom-right History footer tab now that the plugin `list-tools` surface is gone.
 */
export function frameworkHistoryUtilityNodes(app: Pick<AppDefinition, "actions" | "controllerId">): UtilityNode[] {
  return (app.actions ?? [])
    .filter((action) => action.kind === "history")
    .map((action, order) => ({
      id: action.id,
      kind: "button" as const,
      iconId: action.iconId ?? "undo",
      label: action.label,
      text: action.label,
      order,
      category: "history" as const,
      onPress: { controllerId: app.controllerId, action: action.id },
    }));
}

/** @emoji 🗂️ Buckets top-level utility nodes into the given categories (default: all) so activating a category expands the panel with another line, matching {@link buildToolbarRibbonSegments}'s one-active-group-per-level picker. A category with a single already-meaningful collection is used as-is instead of being re-wrapped in a synthetic one, avoiding a redundant picker level with a duplicate-looking label (e.g. a lone "Selection" collection nested under a "Selection" category chip). Separators default to `tools` (mirrors Rust `UtilityNode::category()`), so dividers between same-category runs survive; dividers that only separated different categories become redundant once those categories are separate picker lines. */
export function groupUtilityNodesByCategory(nodes: readonly UtilityNode[], categories: readonly UtilityCategory[] = UTILITY_CATEGORY_ORDER): UtilityNode[] {
  const buckets = new Map<UtilityCategory, UtilityNode[]>();
  for (const node of nodes) {
    const category = utilityNodeCategory(node);
    if (!categories.includes(category)) continue;
    const bucket = buckets.get(category) ?? [];
    bucket.push(node);
    buckets.set(category, bucket);
  }
  return categories
    .filter((category) => hasInteractiveUtilityNodes(buckets.get(category)))
    .map((category, order) => {
      const bucket = buckets.get(category)!;
      if (bucket.length === 1 && bucket[0].kind === "collection") return { ...bucket[0], order };
      return { id: category, kind: "collection" as const, iconId: UTILITY_CATEGORY_ICON_ID[category], text: category, order, category, children: bucket };
    });
}

/** @emoji 🦶 Deduplicates utility nodes by id across every window's utility set (mode-wide utilities are attached identically to each window kind when a plugin doesn't differentiate per window), for a single shared footer entry per utility. */
export function dedupeUtilityNodesById(nodeLists: readonly (readonly UtilityNode[])[]): UtilityNode[] {
  const seen = new Map<string, UtilityNode>();
  for (const nodes of nodeLists) {
    for (const node of nodes) {
      if (!seen.has(node.id)) seen.set(node.id, node);
    }
  }
  return [...seen.values()];
}

//#endregion 🗂️UtilityCategoryGrouping

function isInteractiveUtilityNode(node: UtilityNode): boolean {
  if (node.kind === "separator") return false;
  if (node.kind === "collection") return hasInteractiveUtilityNodes(node.children);
  return true;
}

function hasInteractiveUtilityNodes(nodes?: readonly UtilityNode[]): boolean {
  return Boolean(nodes?.some((node) => isInteractiveUtilityNode(node)));
}

function hasInteractiveUtilityLeaves(items: readonly UtilityLeaf[]): boolean {
  return items.some((node) => node.kind !== "separator");
}

type UtilityCollectionNode = Extract<UtilityNode, { readonly kind: "collection" }>;

export type ToolbarRibbonSegment = { readonly kind: "picker"; readonly collections: readonly UtilityCollectionNode[]; readonly depth: number } | { readonly kind: "tools"; readonly items: readonly UtilityLeaf[]; readonly depth: number };

/** @emoji 🎀 Builds drill-down ribbon segments from a toolbar tree and active collection path; `depth` marks how many collections were drilled into to reach a segment. Collections never auto-activate: a level only recurses when `path[depth]` names one of its enabled collections, so at most one group per level is active and an unresolved level simply shows its picker. */
export function buildToolbarRibbonSegments(nodes: readonly UtilityNode[], path: readonly string[], depth = 0): ToolbarRibbonSegment[] {
  const sorted = sortUtilityNodes(nodes);
  const collections = sorted.filter((node): node is UtilityCollectionNode => node.kind === "collection" && !node.disabled);
  const looseLeaves = sorted.filter((node): node is UtilityLeaf => node.kind !== "collection");
  const segments: ToolbarRibbonSegment[] = [];

  if (collections.length > 0) segments.push({ kind: "picker", collections, depth });
  if (hasInteractiveUtilityLeaves(looseLeaves)) segments.push({ kind: "tools", items: looseLeaves, depth });
  if (collections.length === 0) return segments;

  const activeId = path[depth];
  const active = activeId ? collections.find((node) => node.id === activeId) : undefined;
  if (!active) return segments;
  return [...segments, ...buildToolbarRibbonSegments(active.children, path, depth + 1)];
}

/** @emoji 🎀 Validates an active-group path against the current utility tree: keeps each entry only while it still names an enabled collection at that level, truncating at the first miss rather than substituting a default. */
export function reconcileUtilityPath(nodes: readonly UtilityNode[], path: readonly string[]): readonly string[] {
  let current = nodes;
  const reconciled: string[] = [];

  for (const collectionId of path) {
    const collections = sortUtilityNodes(current).filter((node): node is UtilityCollectionNode => node.kind === "collection" && !node.disabled);
    const active = collections.find((node) => node.id === collectionId);
    if (!active) break;
    reconciled.push(collectionId);
    current = active.children;
  }

  return reconciled;
}

/** @emoji 🎓 Finds the group-id path (in {@link reconcileUtilityPath} shape) leading down to a utility leaf,
 * so a folded picker can drill straight to it. Returns `[]` when the id is a top-level (ungrouped) node,
 * `null` when the tree has no node with that id at all. */
export function findUtilityGroupPath(nodes: readonly UtilityNode[], targetId: string, prefix: readonly string[] = []): readonly string[] | null {
  for (const node of nodes) {
    if (node.id === targetId) return prefix;
    if (node.kind === "collection") {
      const nested = findUtilityGroupPath(node.children, targetId, [...prefix, node.id]);
      if (nested) return nested;
    }
  }
  return null;
}

function UtilityToolbarItems({ items, onAction }: { readonly items: readonly UtilityLeaf[]; readonly onAction: (action: ActionDescriptor) => void }): ReactElement {
  const sorted = useMemo(() => sortUtilityNodes(items) as UtilityLeaf[], [items]);
  const nodes = useMemo(() => {
    const rendered: ReactElement[] = [];
    let buttonRun: UtilityLeaf[] = [];
    let toggleRun: UtilityLeaf[] = [];

    const flushButtons = () => {
      if (buttonRun.length === 0) return;
      const run = buttonRun;
      buttonRun = [];
      rendered.push(
        <ToolbarItem key={`buttons-${run.map((entry) => entry.id).join("-")}`}>
          <ButtonGroup>
            {run.map((entry) => {
              const action = resolveLeafAction(entry);
              if (!action) return null;
              return (
                <ButtonGroupItem
                  key={entry.id}
                  id={entry.id}
                  aria-label={entry.title ?? entry.label ?? entry.id}
                  title={entry.title ?? entry.label}
                  disabled={entry.disabled}
                  onClick={() => onAction(action)}
                  icon={<Icon icon={utilityIcon(entry.iconId)} size="small" />}
                  text={entry.text ?? entry.label}
                />
              );
            })}
          </ButtonGroup>
        </ToolbarItem>,
      );
    };

    const flushToggles = () => {
      if (toggleRun.length === 0) return;
      const run = toggleRun;
      toggleRun = [];
      rendered.push(
        <ToolbarItem key={`toggles-${run.map((entry) => entry.id).join("-")}`}>
          <ToggleGroup
            kind="multiple"
            value={run.filter((entry) => entry.pressed).map((entry) => entry.id)}
            onValueChange={(values) => {
              for (const entry of run) {
                const action = resolveLeafAction(entry);
                if (!action) continue;
                const pressed = values.includes(entry.id);
                if ((entry.pressed ?? false) !== pressed) onAction(action);
              }
            }}
            items={run.map((entry) => ({
              value: entry.id,
              id: entry.id,
              icon: <Icon icon={utilityIcon(entry.iconId)} size="small" />,
              text: entry.text ?? entry.label,
            }))}
          />
        </ToolbarItem>,
      );
    };

    const flushRuns = () => {
      flushButtons();
      flushToggles();
    };

    for (const item of sorted) {
      if (item.kind === "separator") {
        flushRuns();
        rendered.push(<ToolbarDivider key={item.id} />);
        continue;
      }
      if (item.kind === "toggle") {
        flushButtons();
        toggleRun.push(item);
        continue;
      }
      flushToggles();
      buttonRun.push(item);
    }
    flushRuns();
    return rendered;
  }, [onAction, sorted]);

  return <ToolbarGroup>{nodes}</ToolbarGroup>;
}

function toolbarRibbonSegmentKey(segment: ToolbarRibbonSegment, index: number): string {
  return segment.kind === "picker" ? `picker-${segment.depth}-${segment.collections.map((entry) => entry.id).join("-")}` : `tools-${index}-${segment.items.map((entry) => entry.id).join("-")}`;
}

export function UtilityTree({ utilities, onAction, id = "ui.toolbar", direction = "inline", revealUtilityId = null }: UtilityTreeProps): ReactElement | null {
  const [activePath, setActivePath] = useState<readonly string[]>([]);

  useEffect(() => {
    setActivePath((previousPath) => reconcileUtilityPath(utilities, previousPath));
  }, [utilities]);

  useEffect(() => {
    if (!revealUtilityId) return;
    const path = findUtilityGroupPath(utilities, revealUtilityId);
    if (path) setActivePath((previousPath) => (previousPath.length === path.length && previousPath.every((entry, index) => entry === path[index]) ? previousPath : path));
  }, [revealUtilityId, utilities]);

  const segments = useMemo(() => buildToolbarRibbonSegments(utilities, activePath), [utilities, activePath]);

  if (!hasInteractiveUtilityNodes(utilities)) return null;

  const renderSegment = (segment: ToolbarRibbonSegment): ReactNode =>
    segment.kind === "picker" ? (
      <ToolbarItem>
        <ToggleGroup
          kind="single"
          value={activePath[segment.depth] ?? ""}
          onValueChange={(value) => {
            setActivePath(value ? reconcileUtilityPath(utilities, [...activePath.slice(0, segment.depth), value]) : activePath.slice(0, segment.depth));
          }}
          items={segment.collections.map((entry) => ({
            value: entry.id,
            id: `${id}.group.${entry.id}`,
            icon: <Icon icon={utilityIcon(entry.iconId)} size="small" />,
            text: entry.text ?? entry.label,
          }))}
        />
      </ToolbarItem>
    ) : (
      <UtilityToolbarItems items={segment.items} onAction={onAction} />
    );

  const windowId = id.startsWith("ui.toolbar.") ? id.slice("ui.toolbar.".length) : "";

  const findPressedSelectionUtility = (nodes: readonly UtilityNode[]): UtilityNode | undefined => {
    for (const node of nodes) {
      if (node.kind === "collection") {
        const found = findPressedSelectionUtility(node.children);
        if (found) return found;
      } else if (node.kind === "toggle" && node.pressed && node.id.startsWith("select")) {
        return node;
      }
    }
    return undefined;
  };

  const activeSelectionUtility = findPressedSelectionUtility(utilities);
  const hasActiveSelection = activeSelectionUtility != null;

  const rows: RibbonRow[] =
    direction === "inline"
      ? segments.map((segment, index) => ({ key: toolbarRibbonSegmentKey(segment, index), content: renderSegment(segment) }))
      : Array.from(
          segments.reduce((byDepth, segment, index) => {
            const zones = byDepth.get(segment.depth) ?? [];
            zones.push(<ToolbarZone key={toolbarRibbonSegmentKey(segment, index)}>{renderSegment(segment)}</ToolbarZone>);
            byDepth.set(segment.depth, zones);
            return byDepth;
          }, new Map<number, ReactElement[]>()),
        )
          .sort(([left], [right]) => left - right)
          .map(([depth, content]) => ({ key: `row-${depth}`, content }));

  if (hasActiveSelection && direction !== "inline") {
    rows.push({
      key: "row-selection-options",
      content: (
        <ToolbarZone>
          <ToolbarItem>
            <SelectionUtilityOptions activeUtilityId={activeSelectionUtility.id} windowId={windowId} onAction={onAction} />
          </ToolbarItem>
        </ToolbarZone>
      ),
    });
  }

  return (
    <UiChromeLabelPolicyProvider policy="always">
      <Ribbon id={id} direction={direction} rows={rows} />
    </UiChromeLabelPolicyProvider>
  );
}
//#endregion 🔖utility-tree

//#region 🔖os-chrome-panels

//#region DisplayPanel
export type DisplayHostApi = {
  readonly windowKinds: readonly { readonly id: string; readonly label: string }[];
  readonly namedLayouts: readonly NamedLayout[];
  readonly userLayouts: readonly NamedLayout[];
  readonly saveCurrentLayout: (label: string) => void;
  readonly applyNamedLayout: (layoutId: string) => void;
  readonly deleteUserLayout: (layoutId: string) => void;
  readonly layoutSaveLabel: string;
  readonly setLayoutSaveLabel: (value: string) => void;
};

const FRAMEWORK_DISPLAY_WINDOWS_TAB_ID = "framework.display.windows";
const FRAMEWORK_DISPLAY_LAYOUT_TAB_ID = "framework.display.layout";
const FRAMEWORK_SETTINGS_GENERAL_TAB_ID = "framework.settings.general";
const FRAMEWORK_SETTINGS_THEME_TAB_ID = "framework.settings.theme";

function groupNamedLayoutsToTreeItems(layouts: readonly NamedLayout[], onApply: (layoutId: string) => void, onDeleteUser?: (layoutId: string) => void): TreeDataItem[] {
  const root: TreeDataItem[] = [];
  const folderByKey = new Map<string, TreeDataItem>();
  const layoutLeaf = (entry: NamedLayout): TreeDataItem => ({
    id: `framework.display.layout.${entry.id}`,
    label: entry.label,
    onClick: () => onApply(entry.id),
    ...(entry.origin === "user" && onDeleteUser
      ? {
          actions: [
            {
              id: `framework.display.delete.${entry.id}`,
              icon: <Icon icon="trash-2" size="small" />,
              onClick: () => onDeleteUser(entry.id),
            },
          ],
        }
      : {}),
  });
  for (const entry of layouts) {
    if (!entry.groupPath?.length) {
      root.push(layoutLeaf(entry));
      continue;
    }
    let siblings = root;
    let pathKey = "";
    for (let index = 0; index < entry.groupPath.length; index += 1) {
      const segment = entry.groupPath[index]!;
      pathKey = pathKey ? `${pathKey}/${segment}` : segment;
      let folder = folderByKey.get(pathKey);
      if (!folder) {
        folder = { id: `framework.display.layout.group.${pathKey}`, label: segment, defaultOpen: false, items: [] };
        folderByKey.set(pathKey, folder);
        siblings.push(folder);
      }
      const folderItems = folder.items ?? (folder.items = []);
      if (index === entry.groupPath.length - 1) folder.items = [...folderItems, layoutLeaf(entry)];
      else siblings = folderItems;
    }
  }
  return root;
}

function buildDisplayWindowsTree(host: DisplayHostApi): TreePanelConfig {
  return {
    dragAndDropController: windowTemplatePaletteTreeDragController(),
    sections: host.windowKinds.length
      ? host.windowKinds.map((kind) => ({
          id: `framework.display.windows.${kind.id}`,
          label: kind.label,
          defaultOpen: false,
          items: [
            {
              id: `framework.display.windows.${kind.id}.kind`,
              label: kind.label,
              dragData: {
                [COMPOSE_WINDOW_TEMPLATE_MIME]: JSON.stringify({ windowKindId: kind.id }),
              },
            },
          ],
        }))
      : [{ id: "framework.display.windows.empty", items: [{ id: "empty", label: "—" }] }],
  };
}

function buildDisplayLayoutTree(host: DisplayHostApi): TreePanelConfig {
  const builtinLayouts = host.namedLayouts.filter((entry) => entry.origin === "builtin");
  const userLayouts = host.userLayouts;
  const builtinItems = groupNamedLayoutsToTreeItems(builtinLayouts, (layoutId) => host.applyNamedLayout(layoutId));
  const userItems = userLayouts.length
    ? [
        {
          id: "framework.display.layout.group.saved",
          label: shellLabel("ui.display.saved"),
          defaultOpen: false,
          items: groupNamedLayoutsToTreeItems(
            userLayouts,
            (layoutId) => host.applyNamedLayout(layoutId),
            (layoutId) => host.deleteUserLayout(layoutId),
          ),
        },
      ]
    : [];
  return {
    sections: [
      {
        id: "framework.display.layout.save",
        label: shellLabel("ui.display.saveLayout"),
        defaultOpen: false,
        items: [
          {
            id: "framework.display.layout.save.label",
            label: shellLabel("ui.common.name"),
            control: <Input id="framework.display.save-label" value={host.layoutSaveLabel} onChange={(event) => host.setLayoutSaveLabel(event.target.value)} placeholder={shellLabel("ui.display.saveLayoutPlaceholder")} />,
          },
          {
            id: "framework.display.layout.save.action",
            label: shellLabel("ui.common.save"),
            control: (
              <Button
                id="framework.display.save"
                size="sm"
                text={shellLabel("ui.display.saveCurrentLayout")}
                disabled={!host.layoutSaveLabel.trim()}
                onClick={() => {
                  const label = host.layoutSaveLabel.trim();
                  if (!label) return;
                  host.saveCurrentLayout(label);
                  host.setLayoutSaveLabel("");
                }}
              />
            ),
          },
        ],
      },
      {
        id: "framework.display.layout.list",
        label: shellLabel("ui.display.layouts"),
        defaultOpen: true,
        items: [...builtinItems, ...userItems],
      },
    ],
  };
}

export function createFrameworkDisplayPanelTabs(getHost: () => DisplayHostApi | null): PanelTabNode[] {
  return [
    singleTreeLeaf({
      id: FRAMEWORK_DISPLAY_WINDOWS_TAB_ID,
      icon: shellTabIcon("framework.display.windows"),
      name: shellLabel("ui.display.tab.windows"),
      order: -100,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildDisplayWindowsTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.display.unavailable") }] }] };
        },
      },
    }),
    singleTreeLeaf({
      id: FRAMEWORK_DISPLAY_LAYOUT_TAB_ID,
      icon: shellTabIcon("framework.display.layout"),
      name: shellLabel("ui.display.tab.layout"),
      order: -99,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildDisplayLayoutTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.display.unavailable") }] }] };
        },
      },
    }),
  ];
}
//#endregion DisplayPanel

//#region SettingsPanel
export type SettingsHostApi = {
  readonly appId?: string;
  readonly appLabel?: string;
  readonly controllerId?: string;
  readonly pluginId?: string;
  readonly compact: boolean;
  readonly setCompact: (compact: boolean) => void;
  readonly expertise: string;
  readonly setExpertise: (expertise: string) => void;
  readonly appearance: string;
  readonly setAppearance: (appearance: string) => void;
  readonly layout: UiChromeLayout;
  readonly setLayout: (layout: UiChromeLayout) => void;
  readonly mobileActive: boolean;
  /** 🧭 Clears the persisted corner-panel arrangement and folds every corner's active path back to its default — undefined when a shell doesn't wire up dock persistence. */
  readonly onResetDock?: () => void;
  readonly locale: UiLocale;
  readonly setLocale: (locale: UiLocale) => void;
  readonly terminology: string;
  readonly setTerminology: (id: string) => void;
  readonly terminologies: readonly string[];
  readonly theme: UiTheme;
  readonly themeId: string;
  readonly themeDirty: boolean;
  readonly themes: readonly UiTheme[];
  readonly setThemeId: (id: string) => void;
  readonly setThemeColor: (key: string, hex: string) => void;
  readonly setThemeSpacing: (key: string, value: string) => void;
  readonly setThemeFontStack: (key: string, value: string) => void;
  readonly setThemeStroke: (key: string, value: number | number[]) => void;
  readonly setThemeRadius: (key: string, value: number) => void;
  readonly setThemeOpacity: (key: string, value: number) => void;
  readonly setThemeMetric: (section: string, key: string, value: number | number[]) => void;
  readonly setThemeAppearancePaint: (appearance: ThemeAppearanceName, group: ThemePaletteGroup, key: string, hex: string, alpha?: number) => void;
  readonly saveTheme: (label: string) => void;
  readonly deleteTheme: (id: string) => void;
  readonly resetTheme: () => void;
  readonly exportTheme: () => void;
  readonly importTheme: () => void;
  readonly themeSaveLabel: string;
  readonly setThemeSaveLabel: (value: string) => void;
  readonly locks: ResolvedShellLocks;
};

function buildSettingsGeneralTree(host: SettingsHostApi): TreePanelConfig {
  return {
    sections: [
      ...(host.appId || host.appLabel || host.controllerId || host.pluginId
        ? [
            {
              id: "framework.settings.app",
              label: shellLabel("ui.settings.tab.app"),
              defaultOpen: true,
              items: [
                ...(host.appLabel ? [{ id: "framework.settings.app.label", label: `${shellLabel("ui.settings.app.name")}: ${host.appLabel}` }] : []),
                ...(host.appId ? [{ id: "framework.settings.app.id", label: `${shellLabel("ui.settings.app.id")}: ${host.appId}` }] : []),
                ...(host.controllerId ? [{ id: "framework.settings.app.controller", label: `${shellLabel("ui.settings.app.controller")}: ${host.controllerId}` }] : []),
                ...(host.pluginId ? [{ id: "framework.settings.app.plugin", label: `${shellLabel("ui.settings.app.plugin")}: ${host.pluginId}` }] : []),
              ],
            },
          ]
        : []),
      {
        id: "framework.settings.general",
        label: shellLabel("ui.settings.tab.general"),
        defaultOpen: true,
        items: [
          ...(host.locks.appearance
            ? []
            : [
                {
                  id: "framework.settings.appearance",
                  label: shellLabel("ui.settings.tab.appearance"),
                  control: (
                    <Select value={host.appearance} onValueChange={(value) => host.setAppearance(value)}>
                      <SelectTrigger id="framework.settings.appearance" className="h-small w-32" size="sm">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="system">{shellLabel("ui.settings.appearance.system")}</SelectItem>
                        <SelectItem value="light">{shellLabel("ui.settings.appearance.light")}</SelectItem>
                        <SelectItem value="dark">{shellLabel("ui.settings.appearance.dark")}</SelectItem>
                      </SelectContent>
                    </Select>
                  ),
                },
              ]),
          {
            id: "framework.settings.layout",
            label: shellLabel("ui.settings.tab.layout"),
            control: host.mobileActive ? (
              <span className="text-sm text-muted-foreground">{shellLabel("settings.layout.mobile")}</span>
            ) : (
              <Select value={host.layout} onValueChange={(value) => host.setLayout(value === "tablet" ? "tablet" : "desktop")}>
                <SelectTrigger id="framework.settings.layout" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="desktop">{shellLabel("settings.layout.desktop")}</SelectItem>
                  <SelectItem value="tablet">{shellLabel("settings.layout.tablet")}</SelectItem>
                </SelectContent>
              </Select>
            ),
          },
          {
            id: "framework.settings.compact",
            label: shellLabel("settings.compact"),
            control: <input id="framework.settings.compact" type="checkbox" checked={host.compact} onChange={(event) => host.setCompact(event.target.checked)} />,
          },
          {
            id: "framework.settings.expertise",
            label: shellLabel("ui.settings.tab.expertise"),
            control: (
              <Select value={host.expertise} onValueChange={(value) => host.setExpertise(value)}>
                <SelectTrigger id="framework.settings.expertise" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="beginner">{shellLabel("settings.expertise.beginner")}</SelectItem>
                  <SelectItem value="normal">{shellLabel("settings.expertise.normal")}</SelectItem>
                  <SelectItem value="expert">{shellLabel("settings.expertise.expert")}</SelectItem>
                </SelectContent>
              </Select>
            ),
          },
          ...(host.locks.locale
            ? []
            : [
                {
                  id: "framework.settings.language",
                  label: shellLabel("ui.settings.tab.language"),
                  control: (
                    <Select value={host.locale} onValueChange={(value) => host.setLocale(value === "de" ? "de" : "en")}>
                      <SelectTrigger id="framework.settings.language" className="h-small w-32" size="sm">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="en">{shellLabel("ui.settings.language.en")}</SelectItem>
                        <SelectItem value="de">{shellLabel("ui.settings.language.de")}</SelectItem>
                      </SelectContent>
                    </Select>
                  ),
                },
              ]),
          ...(host.locks.terminology
            ? []
            : [
                {
                  id: "framework.settings.terminology",
                  label: shellLabel("ui.settings.tab.terminology"),
                  control: (
                    <Select value={host.terminology} onValueChange={(value) => host.setTerminology(value)}>
                      <SelectTrigger id="framework.settings.terminology" className="h-small w-32" size="sm">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {host.terminologies.map((id) => (
                          <SelectItem key={id} value={id}>
                            {shellTerminologyLabel(id)}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  ),
                },
              ]),
          ...(host.onResetDock
            ? [
                {
                  id: "framework.settings.resetDock.action",
                  label: shellLabel("ui.settings.resetDock"),
                  control: <Button id="framework.settings.resetDock" size="sm" icon="rotate-ccw" text={shellLabel("ui.settings.resetDock")} onClick={() => host.onResetDock?.()} />,
                },
              ]
            : []),
        ],
      },
    ],
  };
}

function rgba8ToHex(rgba: readonly [number, number, number, number]): string {
  const [r, g, b] = rgba;
  return `#${r.toString(16).padStart(2, "0")}${g.toString(16).padStart(2, "0")}${b.toString(16).padStart(2, "0")}`;
}

function themeColorInputRow(id: string, label: string, hex: string, onChange: (hex: string) => void): TreeDataItem {
  return {
    id,
    label,
    control: <input id={id} type="color" className={cn(borderElementClass, "h-small w-16 shrink-0 rounded border bg-background")} value={hex} onChange={(event) => onChange(event.target.value)} />,
  };
}

function themeTextInputRow(id: string, label: string, value: string, onCommit: (value: string) => void): TreeDataItem {
  return {
    id,
    label,
    control: <Input id={id} defaultValue={value} onBlur={(event) => onCommit(event.target.value)} className="h-small w-32" />,
  };
}

function themeNumberInputRow(id: string, label: string, value: number | number[], onCommit: (value: number | number[]) => void): TreeDataItem {
  const text = Array.isArray(value) ? value.join(", ") : String(value);
  return {
    id,
    label,
    control: (
      <Input
        id={id}
        defaultValue={text}
        onBlur={(event) => {
          const raw = event.target.value.trim();
          if (raw.includes(",")) {
            const parts = raw
              .split(",")
              .map((part) => Number.parseFloat(part.trim()))
              .filter((n) => !Number.isNaN(n));
            if (parts.length) onCommit(parts);
            return;
          }
          const n = Number.parseFloat(raw);
          if (!Number.isNaN(n)) onCommit(n);
        }}
        className="h-small w-32"
      />
    ),
  };
}

const THEME_PALETTE_GROUP_LABEL_KEYS = {
  board: "ui.settings.theme.group.board",
  map: "ui.settings.theme.group.map",
  canvas: "ui.settings.theme.group.canvas",
  chrome: "ui.settings.theme.group.chrome",
} as const satisfies Record<ThemePaletteGroup, UiTranslationKey>;

function buildThemeAppearanceGroupItems(host: SettingsHostApi, appearance: ThemeAppearanceName, group: ThemePaletteGroup): TreeDataItem[] {
  const refs = host.theme.appearances[appearance][group];
  const resolved = resolveThemeAppearancePalettes(host.theme, appearance)[group];
  return Object.keys(refs)
    .sort()
    .map((paintKey) => {
      const rgba = resolved[paintKey] ?? [0, 0, 0, 255];
      const hex = rgba8ToHex(rgba);
      const alpha = rgba[3] / 255;
      return {
        id: `framework.settings.theme.appearances.${appearance}.${group}.${paintKey}`,
        label: paintKey,
        control: (
          <div className="flex w-full items-center gap-single">
            <input type="color" className={cn(borderElementClass, "h-small w-10 shrink-0 rounded border bg-background")} value={hex} onChange={(event) => host.setThemeAppearancePaint(appearance, group, paintKey, event.target.value, alpha)} />
            <Input
              id={`framework.settings.theme.appearances.${appearance}.${group}.${paintKey}.alpha`}
              defaultValue={alpha.toFixed(2)}
              onBlur={(event) => {
                const nextAlpha = Number.parseFloat(event.target.value);
                if (!Number.isNaN(nextAlpha)) host.setThemeAppearancePaint(appearance, group, paintKey, hex, Math.min(1, Math.max(0, nextAlpha)));
              }}
              className="h-small w-14 shrink-0"
            />
          </div>
        ),
      } satisfies TreeDataItem;
    });
}

function buildSettingsThemeTree(host: SettingsHostApi): TreePanelConfig {
  const colorItems = Object.keys(host.theme.colors)
    .sort()
    .map((key) => themeColorInputRow(`framework.settings.theme.colors.${key}`, key, host.theme.colors[key]!, (hex) => host.setThemeColor(key, hex)));

  const spacingItems = Object.keys(host.theme.spacing)
    .sort()
    .map((key) => themeTextInputRow(`framework.settings.theme.spacing.${key}`, key, host.theme.spacing[key]!, (value) => host.setThemeSpacing(key, value)));

  const fontItems = Object.keys(host.theme.fontStacks)
    .sort()
    .map((key) => themeTextInputRow(`framework.settings.theme.fonts.${key}`, key, host.theme.fontStacks[key]!, (value) => host.setThemeFontStack(key, value)));

  const strokeItems = Object.keys(host.theme.strokes)
    .sort()
    .map((key) => themeNumberInputRow(`framework.settings.theme.strokes.${key}`, key, host.theme.strokes[key]!, (value) => host.setThemeStroke(key, value)));

  const radiusItems = Object.keys(host.theme.radii)
    .sort()
    .map((key) => themeNumberInputRow(`framework.settings.theme.radii.${key}`, key, host.theme.radii[key]!, (value) => host.setThemeRadius(key, typeof value === "number" ? value : value[0]!)));

  const opacityItems = Object.keys(host.theme.opacities)
    .sort()
    .map((key) => themeNumberInputRow(`framework.settings.theme.opacities.${key}`, key, host.theme.opacities[key]!, (value) => host.setThemeOpacity(key, typeof value === "number" ? value : value[0]!)));

  const metricSections = Object.keys(host.theme.metrics)
    .sort()
    .map(
      (section): TreeDataItem => ({
        id: `framework.settings.theme.metrics.${section}`,
        label: section,
        defaultOpen: false,
        items: Object.keys(host.theme.metrics[section]!)
          .sort()
          .map((key) => themeNumberInputRow(`framework.settings.theme.metrics.${section}.${key}`, key, host.theme.metrics[section]![key]!, (value) => host.setThemeMetric(section, key, value))),
      }),
    );

  const appearanceGroups: readonly ThemePaletteGroup[] = ["board", "map", "canvas", "chrome"];
  const appearanceItems: TreeDataItem[] = (["light", "dark"] as const).map((appearance) => ({
    id: `framework.settings.theme.appearances.${appearance}`,
    label: shellLabel(appearance === "light" ? "ui.settings.theme.appearance.light" : "ui.settings.theme.appearance.dark"),
    defaultOpen: false,
    items: appearanceGroups.map((group) => ({
      id: `framework.settings.theme.appearances.${appearance}.${group}`,
      label: shellLabel(THEME_PALETTE_GROUP_LABEL_KEYS[group]),
      defaultOpen: false,
      items: buildThemeAppearanceGroupItems(host, appearance, group),
    })),
  }));

  return {
    sections: [
      {
        id: "framework.settings.theme.select",
        label: `${shellLabel("ui.settings.theme.select")}${host.themeDirty ? ` (${shellLabel("ui.settings.theme.dirty")})` : ""}`,
        defaultOpen: true,
        items: [
          {
            id: "framework.settings.theme.select.picker",
            label: shellLabel("ui.settings.theme.select"),
            control: (
              <Select value={host.themeId} onValueChange={(value) => host.setThemeId(value)}>
                <SelectTrigger id="framework.settings.theme.select" className="h-small w-32" size="sm">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {host.themes.map((theme) => (
                    <SelectItem key={theme.id} value={theme.id}>
                      {theme.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            ),
          },
          {
            id: "framework.settings.theme.save.label",
            label: shellLabel("ui.common.name"),
            control: (
              <Input id="framework.settings.theme.save-label" value={host.themeSaveLabel} onChange={(event) => host.setThemeSaveLabel(event.target.value)} placeholder={shellLabel("ui.settings.theme.savePlaceholder")} className="h-small w-32" />
            ),
          },
          {
            id: "framework.settings.theme.save.action",
            label: shellLabel("ui.settings.theme.save"),
            control: (
              <Button
                id="framework.settings.theme.save"
                size="sm"
                text={shellLabel("ui.settings.theme.save")}
                disabled={!host.themeSaveLabel.trim()}
                onClick={() => {
                  const label = host.themeSaveLabel.trim();
                  if (!label) return;
                  host.saveTheme(label);
                  host.setThemeSaveLabel("");
                }}
              />
            ),
          },
          {
            id: "framework.settings.theme.reset.action",
            label: shellLabel("ui.settings.theme.reset"),
            control: <Button id="framework.settings.theme.reset" size="sm" text={shellLabel("ui.settings.theme.reset")} disabled={!host.themeDirty && host.themeId === "semio"} onClick={() => host.resetTheme()} />,
          },
          {
            id: "framework.settings.theme.export.action",
            label: shellLabel("ui.settings.theme.export"),
            control: <Button id="framework.settings.theme.export" size="sm" text={shellLabel("ui.settings.theme.export")} onClick={() => host.exportTheme()} />,
          },
          {
            id: "framework.settings.theme.import.action",
            label: shellLabel("ui.settings.theme.import"),
            control: <Button id="framework.settings.theme.import" size="sm" text={shellLabel("ui.settings.theme.import")} onClick={() => host.importTheme()} />,
          },
          ...(host.themeId.startsWith("custom.")
            ? [
                {
                  id: "framework.settings.theme.delete.action",
                  label: shellLabel("ui.settings.theme.delete"),
                  control: <Button id="framework.settings.theme.delete" size="sm" text={shellLabel("ui.settings.theme.delete")} onClick={() => host.deleteTheme(host.themeId)} />,
                },
              ]
            : []),
        ],
      },
      { id: "framework.settings.theme.colors", label: shellLabel("ui.settings.theme.colors"), defaultOpen: false, items: colorItems },
      { id: "framework.settings.theme.spacing", label: shellLabel("ui.settings.theme.spacing"), defaultOpen: false, items: spacingItems },
      { id: "framework.settings.theme.fonts", label: shellLabel("ui.settings.theme.fonts"), defaultOpen: false, items: fontItems },
      { id: "framework.settings.theme.strokes", label: shellLabel("ui.settings.theme.strokes"), defaultOpen: false, items: strokeItems },
      { id: "framework.settings.theme.radii", label: shellLabel("ui.settings.theme.radii"), defaultOpen: false, items: radiusItems },
      { id: "framework.settings.theme.opacities", label: shellLabel("ui.settings.theme.opacities"), defaultOpen: false, items: opacityItems },
      { id: "framework.settings.theme.metrics", label: shellLabel("ui.settings.theme.metrics"), defaultOpen: false, items: metricSections },
      { id: "framework.settings.theme.appearances", label: shellLabel("ui.settings.theme.appearances"), defaultOpen: false, items: appearanceItems },
    ],
  };
}

export function createFrameworkSettingsPanelTabs(getHost: () => SettingsHostApi | null): PanelTabNode[] {
  return [
    singleTreeLeaf({
      id: FRAMEWORK_SETTINGS_GENERAL_TAB_ID,
      icon: shellTabIcon("framework.settings.general"),
      name: shellLabel("ui.panelToggle.settings"),
      order: -98,
      tree: {
        resolveTree: () => {
          const host = getHost();
          return host ? buildSettingsGeneralTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.settings.unavailable") }] }] };
        },
      },
    }),
    // 🔒 A locked theme means no theme editing/saving either — drop the whole tab (its footer toggle
    // in `settingsToggleItems` derives from `settingsRightTabs`, so it disappears for free).
    ...(getHost()?.locks.themeId
      ? []
      : [
          singleTreeLeaf({
            id: FRAMEWORK_SETTINGS_THEME_TAB_ID,
            icon: shellTabIcon("paintbrush"),
            name: shellLabel("ui.settings.tab.theme"),
            order: -97,
            tree: {
              resolveTree: () => {
                const host = getHost();
                return host ? buildSettingsThemeTree(host) : { sections: [{ id: "unavailable", items: [{ id: "unavailable", label: shellLabel("ui.settings.unavailable") }] }] };
              },
            },
          }),
        ]),
  ];
}

export function useNamedLayoutHost(options: {
  readonly appId: string;
  readonly windowKinds: readonly { readonly id: string; readonly label: string }[];
  readonly builtinLayouts: readonly NamedLayout[];
  readonly currentLayout: WindowLayout | undefined;
  readonly onApplyLayout: (layout: WindowLayout) => void;
  readonly namedLayoutStore: { getSnapshot: () => readonly NamedLayout[]; save: (layout: NamedLayout) => void; remove: (layoutId: string) => void; subscribe: (listener: () => void) => () => void };
}): DisplayHostApi {
  const userLayouts = useSyncExternalStore(
    (listener) => options.namedLayoutStore.subscribe(listener),
    () => options.namedLayoutStore.getSnapshot(),
    () => options.namedLayoutStore.getSnapshot(),
  );
  const [layoutSaveLabel, setLayoutSaveLabel] = useState("");
  return useMemo(
    (): DisplayHostApi => ({
      windowKinds: options.windowKinds,
      namedLayouts: options.builtinLayouts,
      userLayouts,
      saveCurrentLayout: (label) => {
        if (!options.currentLayout) return;
        const id = `user-${Date.now()}`;
        options.namedLayoutStore.save(createNamedLayout(id, label, options.currentLayout, "user"));
      },
      applyNamedLayout: (layoutId) => {
        const layout = [...options.builtinLayouts, ...userLayouts].find((entry) => entry.id === layoutId);
        if (layout) options.onApplyLayout(layout.layout);
      },
      deleteUserLayout: (layoutId) => options.namedLayoutStore.remove(layoutId),
      layoutSaveLabel,
      setLayoutSaveLabel,
    }),
    [options, userLayouts, layoutSaveLabel],
  );
}
//#endregion SettingsPanel
//#endregion 🔖os-chrome-panels
