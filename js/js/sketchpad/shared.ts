// #region Header

// sketchpad.ts

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

// #region Imports

import { ChatIcon, DetailsIcon, HudIcon, SettingsIcon, StatsIcon, ToolbarIcon, ToolsIcon, WorkbenchIcon } from "@semio/assets";
import { ComponentType, ReactNode } from "react";
import { AnyActorRef, assign, fromCallback } from "xstate";
import * as Y from "yjs";
import { Guid, Kit, KitDiff } from "../semio";

// #endregion

// #region Types

export type Url = string;

export type Subscribe = (callback: () => void) => () => void;

export type Disposable = () => void;

export type Transact = (fn: () => void, origin?: string) => void;

export type Unsubscribe = () => void;

export type YProviderFactory = (doc: Y.Doc, id: string) => Promise<void>;

export type AppKind = string;

export type Layout = "desktop" | "tablet" | MobileLayout;

export type PanelKey = "details" | "workbench" | "tools" | "hud" | "stats" | "console" | "chat" | "settings" | "toolbar";

export type HotkeyPath = string;

export type HotkeyValue = string;

export type HotkeyOverrides = Record<HotkeyPath, HotkeyValue>;

export type FileProviderFactory = (kitId: string) => Promise<FileProvider>;

export type YUuid = string;

export type YUuidArray = Y.Array<YUuid>;

export type YConcept = string;

export type YConcepts = Y.Array<string>;

export type YStringArray = Y.Array<string>;

export type YLeafMapString = Y.Map<string>;

export type YLeafMapNumber = Y.Map<number>;

export type YAttributes = Y.Array<Y.Map<string>>;

// #endregion Types

// #region Enums

export enum Theme {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}

export enum Expertise {
  BEGINNER = "beginner",
  NORMAL = "normal",
  EXPERT = "expert",
}

export enum Mode {
  USER = "user",
  DEV = "dev",
}

export enum StoreStatus {
  IDLE = "idle",
  LOADING = "loading",
  ERROR = "error",
  READY = "ready",
}

export enum ToolKind {
  SELECTION_NORMAL = "selection-normal",
  SELECTION_ADDITIVE = "selection-additive",
  SELECTION_SUBTRACTIVE = "selection-subtractive",
  LASSO_RECTANGULAR = "lasso-rectangular",
  LASSO_FREEFORM = "lasso-freeform",
  PORT = "port",
}

export enum WindowKind {
  TABLE = "table",
  SCENE = "scene",
  DIAGRAM = "diagram",
  CUSTOM = "custom",
}

export enum PanelPosition {
  LEFT = "left",
  RIGHT = "right",
  MIDDLE = "middle",
  BOTTOM = "bottom",
}

export enum PanelKind {
  WORKBENCH = "workbench",
  TOOLS = "tools",
  TOOLBAR = "toolbar",
  HUD = "hud",
  STATS = "stats",
  DETAILS = "details",
  CHAT = "chat",
  SETTINGS = "settings",
  PARAMS = "params",
}

// #endregion Enums

// #region Interfaces

// #region File Provider

export interface FileProvider {
  upload: (kitId: string, fileId: string, path: string, blob: Blob) => Promise<string>;
  download: (kitId: string, fileId: string, path: string) => Promise<Blob>;
  delete: (kitId: string, fileId: string, path: string) => Promise<void>;
  getUrl: (kitId: string, fileId: string, path: string) => string;
}

export interface MemoryFileProviderConfig { }

export interface LocalFileProviderConfig {
  dbName?: string;
  storeName?: string;
}

export interface RemoteFileProviderConfig {
  baseUrl: string;
  headers?: Record<string, string>;
}

export interface CompositeFileProviderConfig {
  memory?: boolean;
  local?: boolean | LocalFileProviderConfig;
  remote?: RemoteFileProviderConfig;
}

export interface RemoteProviders {
  yProvider: (yDoc: Y.Doc, name: string) => void;
  fileProvider: FileProviderFactory;
}

export interface FileOperation {
  type: "upload" | "download" | "delete";
  kitId: string;
  fileId: string;
  path: string;
  blob?: Blob;
}

// #endregion File Provider

// #region App IDs

export interface DesignAppId {
  kit: Guid;
  design: Guid;
}

export interface KitAppId {
  kit: Guid;
}

export interface TypeAppId {
  kit: Guid;
  type: Guid;
}

export interface QualityAppId {
  kit: Guid;
  quality: Guid;
}

// #endregion App IDs

// #region Panel

export interface PanelKindConfig {
  icon: ComponentType<{ size?: number }>;
  position: PanelPosition;
  group?: string;
  isTransparent?: boolean;
  isGroupable?: boolean;
  hotkey?: string;
}

export const panelKindConfigs: Record<PanelKind, PanelKindConfig> = {
  [PanelKind.WORKBENCH]: {
    icon: WorkbenchIcon,
    position: PanelPosition.LEFT,
    group: "workbench",
    isGroupable: true,
    hotkey: "ctrl+j",
  },
  [PanelKind.TOOLS]: {
    icon: ToolsIcon,
    position: PanelPosition.LEFT,
    group: "workbench",
    isGroupable: true,
    hotkey: "ctrl+j",
  },
  [PanelKind.TOOLBAR]: {
    icon: ToolbarIcon,
    position: PanelPosition.BOTTOM,
  },
  [PanelKind.HUD]: {
    icon: HudIcon,
    position: PanelPosition.MIDDLE,
    group: "hud",
    isGroupable: true,
    isTransparent: true,
    hotkey: "ctrl+k",
  },
  [PanelKind.STATS]: {
    icon: StatsIcon,
    position: PanelPosition.MIDDLE,
    group: "hud",
    isGroupable: true,
    isTransparent: true,
    hotkey: "ctrl+k",
  },
  [PanelKind.DETAILS]: {
    icon: DetailsIcon,
    position: PanelPosition.RIGHT,
    group: "right",
    isGroupable: true,
    hotkey: "ctrl+l",
  },
  [PanelKind.CHAT]: {
    icon: ChatIcon,
    position: PanelPosition.RIGHT,
    group: "right",
    isGroupable: true,
    hotkey: "ctrl+l",
  },
  [PanelKind.SETTINGS]: {
    icon: SettingsIcon,
    position: PanelPosition.RIGHT,
    group: "right",
    isGroupable: true,
    hotkey: "ctrl+l",
  },
  [PanelKind.PARAMS]: {
    icon: SettingsIcon,
    position: PanelPosition.RIGHT,
    group: "right",
    isGroupable: true,
    hotkey: "ctrl+l",
  },
};

export interface PanelVisibility {
  toolbar?: boolean;
  workbench?: boolean;
  tools?: boolean;
  hud?: boolean;
  stats?: boolean;
  details?: boolean;
  chat?: boolean;
  settings?: boolean;
  params?: boolean;
}

export interface PanelSizes {
  toolbarHeight: number;
  workbenchWidth: number;
  toolsWidth: number;
  hudWidth: number;
  statsWidth: number;
  detailsWidth: number;
  chatWidth: number;
  settingsWidth: number;
  consoleHeight: number;
}

export interface PanelSection {
  id: string;
  content: ReactNode | (() => ReactNode);
  specificity?: number;
  defaultOpen?: boolean;
  order?: number;
  actions?: Array<{
    id: string;
    icon: ReactNode;
    onClick: () => void;
  }>;
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onDoubleClick?: () => void;
}

export interface PanelSections {
  details: PanelSection[];
  workbench: PanelSection[];
  tools: PanelSection[];
  hud: PanelSection[];
  stats: PanelSection[];
  console: PanelSection[];
  chat: PanelSection[];
  settings: PanelSection[];
  toolbar: PanelSection[];
}

export interface PanelDefinition {
  id: string;
  kind: PanelKind;
  hotkey?: string;
  tooltip?: {
    labelKey?: string;
    manualPath?: string;
  };
}

export interface EnrichedPanelDefinition extends PanelDefinition {
  key: string;
  icon: ComponentType<{ size?: number }>;
  position: PanelPosition;
  group?: string;
  isTransparent?: boolean;
  isGroupable?: boolean;
  hotkey?: string;
}

export function createPanelDefinition(kind: PanelKind, id: string, hotkey?: string, tooltip?: { labelKey?: string; manualPath?: string }): PanelDefinition {
  const config = panelKindConfigs[kind];
  return {
    id,
    kind,
    hotkey: hotkey ?? config.hotkey,
    tooltip,
  };
}

export function enrichPanelDefinition(panel: PanelDefinition): EnrichedPanelDefinition {
  const config = panelKindConfigs[panel.kind];
  return {
    ...panel,
    key: panel.kind,
    icon: config.icon,
    position: config.position,
    group: config.group,
    isTransparent: config.isTransparent,
    isGroupable: config.isGroupable,
    hotkey: panel.hotkey ?? config.hotkey,
  };
}

export interface PanelConfig {
  id: string;
  key: "workbench" | "details" | "settings" | "tools" | "hud" | "stats" | "toolbar" | "chat";
  label: string;
  order?: number;
  defaultOpen?: boolean;
  content: ReactNode | (() => ReactNode);
}

export interface AppPanels {
  panels: PanelConfig[];
}

// #endregion Panel

// #region App Registry

export interface RouteSegment {
  path: string;
  paramName?: string;
  scopeProvider?: ComponentType<{ guid: string; children: ReactNode }>;
}

export interface AppConfig {
  id: string;
  component: ComponentType;
  routeSegments: RouteSegment[];
  additionalPaths?: string[];
  getPanels: (() => PanelDefinition[]) | ((getLabelFn: (key: string) => string) => PanelDefinition[]) | ((getLabelFn: (key: string) => string, getHotkeyFn: (key: string) => string) => PanelDefinition[]);
  matchesPath?: (pathParts: string[]) => boolean;
  order?: number;
}

export interface AppRegistration extends AppConfig { }

// #endregion App Registry

// #region Sketchpad State

export interface MobileLayout {
  isNavbarExpanded: boolean;
  isFooterExpanded: boolean;
}

export interface SketchpadChangableState {
  navigation: string;
  navigationHistory: string[];
  navigationHistoryIndex: number;
  recentSearches: string[];
  recentFocusItems: Record<string, string[]>;
  theme: Theme;
  language: string;
  layout: Layout;
  expertise: Expertise;
  mode: Mode;
  settings: {
    apps: Record<string, any>;
  };
  panelSizes: PanelSizes;
  isFullscreen: boolean;
  isMobile: boolean;
  activeInteraction?: string;
  hotkeyOverrides?: Record<string, string>;
  activeHotkeySetting?: string;
}

export interface SketchpadState extends SketchpadChangableState {
  id?: string;
  persisted?: boolean;
}

export interface SketchpadDiff {
  navigation?: string;
  navigationHistory?: string[];
  navigationHistoryIndex?: number;
  recentSearches?: string[];
  recentFocusItems?: Record<string, string[]>;
  theme?: Theme;
  language?: string;
  layout?: Layout;
  expertise?: Expertise;
  mode?: Mode;
  settings?: {
    apps?: Record<string, any>;
  };
  panelSizes?: Partial<PanelSizes>;
  isFullscreen?: boolean;
  isMobile?: boolean;
  activeInteraction?: string;
  hotkeyOverrides?: Record<string, string>;
  activeHotkeySetting?: string;
}

export interface InitialStateKit {
  kit: Kit;
  local?: boolean;
  remote?: boolean;
}

export interface ExtendedInitialState extends Partial<SketchpadState> {
  kits?: InitialStateKit[];
}

export type WindowEvents = {
  minimize: () => void;
  maximize: () => void;
  close: () => void;
};

export type SketchpadScope = { id: string; remote?: RemoteProviders; onWindowEvents?: WindowEvents };

// #endregion Sketchpad State

// #region Commands

export interface KitCommandContext {
  kit: Kit;
  fileUrls: Map<Url, Url>;
  origin?: string;
}

export interface KitCommandResult {
  diff?: KitDiff;
  files?: File[];
  origin?: string;
}

export interface SketchpadCommandContext {
  sketchpad: SketchpadState;
  origin?: string;
}

export interface SketchpadCommandResult {
  diff?: SketchpadDiff;
  origin?: string;
}

// #endregion Commands

// #region Store

export interface Synchronizable<TAccessl> {
  onChanged: (subscribe: Subscribe) => Unsubscribe;
  onChangedDeep: (subscribe: Subscribe) => Unsubscribe;
  snapshot: () => TAccessl;
}

export interface StoreState<TState> {
  status: StoreStatus;
  data?: TState;
  error?: Error;
}

export interface AppStep<TSelectionDiff = any> {
  selectionDiff?: TSelectionDiff;
}

export interface AppEdit<TSelectionDiff = any> {
  do: AppStep<TSelectionDiff>;
  undo: AppStep<TSelectionDiff>;
}

export interface AppDiff<TSelectionDiff = any> {
  selection?: TSelectionDiff;
  presence?: any;
  hover?: any;
  fullscreenWindow?: any;
  panelVisibility?: Partial<PanelVisibility>;
}

export interface AppCommandResult<TDiff = any> {
  diff?: TDiff;
  origin?: string;
}

export interface KitDiffAppStep<TSelectionDiff = any> extends AppStep<TSelectionDiff> {
  kitDiff?: KitDiff;
}

export interface KitDiffAppEdit<TSelectionDiff = any> {
  do: KitDiffAppStep<TSelectionDiff>;
  undo: KitDiffAppStep<TSelectionDiff>;
}

export interface KitDiffAppCommandResult<TDiff = any> extends AppCommandResult<TDiff> {
  kitDiff?: KitDiff;
}

export interface Synchronizable<TAccessl> {
  onChanged: (subscribe: Subscribe) => Unsubscribe;
  onChangedDeep: (subscribe: Subscribe) => Unsubscribe;
  snapshot: () => TAccessl;
}

// #endregion Store

// #region Complete State

export interface CompleteState {
  sketchpad: SketchpadState;
  kits: Array<{
    guid: string;
    local: boolean;
    remote: boolean;
    kit: Kit;
  }>;
  kitApps: Record<string, any>;
  typeApps: Record<string, any>;
  qualityApps: Record<string, any>;
  designApps: Record<string, Record<string, any>>;
  home?: any;
  tutorials: any;
}

// #endregion Complete State

// #region Window

export interface WindowConfig {
  id: string;
  title?: string;
  icon?: ReactNode;
  component: ComponentType<any>;
  componentProps?: any;
  defaultSize?: number;
}

export interface WindowControl {
  kind: "toggle" | "dropdown";
  id: string;
  icon?: ReactNode;
  value?: string;
  options?: {
    id: string;
    value: string;
    icon?: ReactNode;
  }[];
  onChange?: (value: string) => void;
}

export interface WindowKindDefinition {
  id: string;
  icon?: ReactNode;
  component: (props: any) => ReactNode;
  controls?: WindowControl[];
  variants?: {
    id: string;
    icon?: ReactNode;
    componentProps?: any;
  }[];
}

export interface AppWindowConfig {
  windowKinds: WindowKindDefinition[];
  defaultLayout: any;
}

export interface AppWindowProps {
  kind: WindowKind;
  children: ReactNode;
  className?: string;
}

export function createDefaultLayout(windowIds: string[], direction: "row" | "column" = "row", sizes?: number[], titles?: string[]): any {
  return {
    type: direction === "row" ? "row" : "column",
    content: windowIds.map((id, index) => ({
      type: "stack",
      content: [
        {
          type: "component",
          componentName: id,
          title: titles && titles[index] ? titles[index] : id,
          componentState: {},
        },
      ],
      ...(sizes && sizes[index] !== undefined ? { size: `${sizes[index]}%` } : {}),
    })),
  };
}

// #endregion Window

// #region Tool

export interface Tool<TState = any> {
  id: ToolKind | string;
  icon?: ReactNode;
  render: (context: ToolRenderContext<TState>) => { scene?: ReactNode; diagram?: ReactNode | null; table?: ReactNode | null };
}

export interface ToolMode {
  id: string;
  icon?: ReactNode;
  label?: string;
  tooltipId?: string;
}

export interface ToolDefinition {
  id: string;
  defaultMode: ToolKind | string;
  modes: ToolMode[];
}

export interface ToolRenderContext<TState = any> {
  state: TState;
}

export interface ToolGroupProps {
  tools: ToolDefinition[];
  activeTool: ToolKind | string;
  onToolChange: (tool: ToolKind | string) => void;
  level?: "panel" | "toolbar";
}

// #endregion Tool

// #region Focus

export interface FocusItem {
  id: string;
  label: string;
  description?: string;
  category?: string;
}

// #endregion Focus

// #region Footer

export interface FooterItem {
  id: string;
  icon?: any;
  content?: ReactNode;
  onClick?: () => void;
  order?: number;
}

// #endregion Footer

// #region Panel Props

export interface ResizablePanelProps {
  visible: boolean;
  onWidthChange?: (width: number) => void;
  width: number;
}

// #endregion Panel Props

// #endregion Interfaces

// #region XState Integration

// #region XState Types

/**
 * Base context for all XState machines that sync with Y.js
 */
export interface YjsSyncContext {
  /** Whether the Y.js data has changed since last snapshot */
  dirty: boolean;
  /** Cached snapshot of the Y.js data */
  cache?: any;
}

/**
 * Context for the root Sketchpad machine
 */
export interface SketchpadMachineContext extends YjsSyncContext {
  navigation: string;
  navigationHistory: string[];
  navigationHistoryIndex: number;
  recentSearches: string[];
  recentFocusItems: Record<string, string[]>;
  theme: Theme;
  language: string;
  layout: Layout;
  expertise: Expertise;
  mode: Mode;
  settings: {
    apps: Record<string, any>;
  };
  panelSizes: PanelSizes;
  isFullscreen: boolean;
  isMobile: boolean;
  activeInteraction?: string;
  hotkeyOverrides?: Record<string, string>;
  activeHotkeySetting?: string;
  /** Map of kit guids to their actor refs */
  kits: Record<Guid, AnyActorRef>;
  /** Home app actor ref */
  homeRef?: AnyActorRef;
  /** Docs app actor ref */
  docsRef?: AnyActorRef;
}

/**
 * Events for the Sketchpad machine
 */
export type SketchpadMachineEvent =
  | { type: "NAVIGATE"; path: string }
  | { type: "NAVIGATE_BACK" }
  | { type: "NAVIGATE_FORWARD" }
  | { type: "SET_THEME"; theme: Theme }
  | { type: "SET_LANGUAGE"; language: string }
  | { type: "SET_EXPERTISE"; expertise: Expertise }
  | { type: "SET_MODE"; mode: Mode }
  | { type: "SET_LAYOUT"; layout: Layout }
  | { type: "TOGGLE_FULLSCREEN" }
  | { type: "SET_PANEL_SIZE"; panel: keyof PanelSizes; size: number }
  | { type: "CREATE_KIT"; kit: Kit }
  | { type: "DELETE_KIT"; guid: Guid }
  | { type: "Y_UPDATE"; data: any }
  | { type: "Y_FIELD_UPDATE"; field: string; value: any };

/**
 * Context for Kit machines (spawned actors)
 */
export interface KitMachineContext extends YjsSyncContext {
  guid: Guid;
  kit: Kit;
  /** Map of type guids to their stores */
  types: Record<Guid, any>;
  /** Map of design guids to their stores */
  designs: Record<Guid, any>;
  /** File URL cache */
  fileUrls: Map<string, string>;
  /** Whether this kit is local */
  local: boolean;
  /** Whether this kit is remote */
  remote: boolean;
}

/**
 * Events for Kit machines
 */
export type KitMachineEvent =
  | { type: "LOAD" }
  | { type: "CHANGE"; diff: KitDiff }
  | { type: "CREATE_TYPE"; typeData: any }
  | { type: "UPDATE_TYPE"; guid: Guid; diff: any }
  | { type: "DELETE_TYPE"; guid: Guid }
  | { type: "CREATE_DESIGN"; design: any }
  | { type: "UPDATE_DESIGN"; guid: Guid; diff: any }
  | { type: "DELETE_DESIGN"; guid: Guid }
  | { type: "Y_UPDATE"; data: any };

/**
 * Generic App machine context
 */
export interface AppMachineContext<TSelection = any> extends YjsSyncContext {
  panelVisibility: PanelVisibility;
  selection?: TSelection;
  hover?: any;
  presence?: any;
  others: any[];
  /** Transaction state */
  isTransactionActive: boolean;
  currentTransactionStack: any[];
  pastTransactionsStack: any[];
  redoStack: any[];
}

/**
 * Generic App machine events
 */
export type AppMachineEvent<TSelectionDiff = any, TDiff = any> =
  | { type: "START_TRANSACTION" }
  | { type: "FINALIZE_TRANSACTION" }
  | { type: "ABORT_TRANSACTION" }
  | { type: "UNDO" }
  | { type: "REDO" }
  | { type: "TOGGLE_PANEL"; panel: keyof PanelVisibility }
  | { type: "SELECT"; diff: TSelectionDiff }
  | { type: "DESELECT" }
  | { type: "HOVER"; data: any }
  | { type: "CLEAR_HOVER" }
  | { type: "CHANGE"; diff: TDiff }
  | { type: "Y_UPDATE"; data: any };

/**
 * KitDiff App machine context (for apps that can modify kits)
 */
export interface KitDiffAppMachineContext<TSelection = any> extends AppMachineContext<TSelection> {
  kitGuid: Guid;
}

// #endregion XState Types

// #region Y.js-XState Bridge

/**
 * Creates an XState actor that observes a Y.js Map and sends Y_UPDATE events
 * when the data changes. This is the core bridge between Y.js and XState.
 * 
 * @param yMap - The Y.js Map to observe
 * @returns An actor logic that can be invoked in a machine
 * 
 * @example
 * ```ts
 * const machine = createMachine({
 *   invoke: {
 *     id: 'yjsSync',
 *     src: createYjsSyncActor(yMap)
 *   },
 *   on: {
 *     Y_UPDATE: { actions: 'handleYjsUpdate' }
 *   }
 * });
 * ```
 */
export function createYjsSyncActor(yMap: Y.Map<any>) {
  return fromCallback<{ type: "Y_UPDATE"; data: any }>(({ sendBack }: { sendBack: (event: { type: "Y_UPDATE"; data: any }) => void }) => {
    const observer = () => {
      sendBack({ type: "Y_UPDATE", data: yMap.toJSON() });
    };

    // Send initial state
    observer();

    // Observe deep changes
    yMap.observeDeep(observer);

    // Return cleanup function
    return () => {
      yMap.unobserveDeep(observer);
    };
  });
}

/**
 * Creates an XState actor that observes a specific field in a Y.js Map
 * and sends Y_FIELD_UPDATE events when that field changes.
 * 
 * @param yMap - The Y.js Map to observe
 * @param field - The field name to observe
 * @returns An actor logic that can be invoked in a machine
 */
export function createYjsFieldSyncActor(yMap: Y.Map<any>, field: string) {
  return fromCallback<{ type: "Y_FIELD_UPDATE"; field: string; value: any }>(({ sendBack }: { sendBack: (event: { type: "Y_FIELD_UPDATE"; field: string; value: any }) => void }) => {
    const observer = (events: Y.YMapEvent<any>[]) => {
      for (const event of events) {
        if (event.keysChanged.has(field)) {
          sendBack({ type: "Y_FIELD_UPDATE", field, value: yMap.get(field) });
        }
      }
    };

    // Send initial state
    sendBack({ type: "Y_FIELD_UPDATE", field, value: yMap.get(field) });

    // Observe changes
    yMap.observe(observer as any);

    // Return cleanup function
    return () => {
      yMap.unobserve(observer as any);
    };
  });
}

/**
 * Wraps a Y.js transaction in a function that can be called from XState actions.
 * This ensures Y.js changes are atomic and can be properly synced.
 * 
 * @param yDoc - The Y.js document
 * @param fn - The function to execute within the transaction
 * @param origin - Optional origin string for the transaction
 */
export function yTransact(yDoc: Y.Doc, fn: () => void, origin?: string): void {
  yDoc.transact(fn, origin);
}

/**
 * Creates an assign action that updates Y.js data and marks cache as dirty.
 * This is used to handle Y_UPDATE events in XState machines.
 * 
 * @returns An assign action configuration
 */
export function createYjsUpdateAssign() {
  return assign({
    dirty: () => true,
    cache: ({ event }: { event: { type: "Y_UPDATE"; data: any } }) => (event as any).data,
  });
}

/**
 * Helper to create a selector that accesses cached Y.js data with dirty checking.
 * If dirty, rebuilds the cache; otherwise returns cached data.
 * 
 * @param buildSnapshot - Function to build snapshot from Y.js data
 * @returns A function that returns the snapshot
 */
export function createYjsSelector<TContext extends YjsSyncContext, TSnapshot>(
  buildSnapshot: (context: TContext) => TSnapshot
): (context: TContext) => TSnapshot {
  return (context: TContext): TSnapshot => {
    if (!context.dirty && context.cache) {
      return context.cache as TSnapshot;
    }
    return buildSnapshot(context);
  };
}

// #endregion Y.js-XState Bridge

// #region Machine Factories

/**
 * Input type for creating app machines
 */
export interface AppMachineInput {
  yMap: Y.Map<any>;
  transact: Transact;
}

/**
 * Input type for creating kit-diff app machines
 */
export interface KitDiffAppMachineInput extends AppMachineInput {
  kitGuid: Guid;
}

/**
 * Configuration for transaction machine
 */
export interface TransactionMachineConfig<TEdit = any> {
  /** Function to apply an edit's selection diff */
  applySelectionDiff: (selectionDiff: any) => void;
  /** Function to compute inverse of a selection diff */
  inverseSelectionDiff: (selection: any, diff: any) => any;
  /** Optional function to apply kit diff (for KitDiff apps) */
  applyKitDiff?: (kitDiff: KitDiff) => void;
  /** Optional function to compute inverse of kit diff */
  inverseKitDiff?: (kit: Kit, diff: KitDiff) => KitDiff;
}

// #endregion Machine Factories

// #endregion XState Integration
