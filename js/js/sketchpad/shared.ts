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

// #region YPath Types


export type YPathSegment =
  | { kind: "mapKey"; key: string }
  | { kind: "arrayIndex"; index: number }
  | { kind: "arrayItemById"; id: string; idKey: string };


export type YPath = YPathSegment[];

// #endregion YPath Types

// #region Granular Hook Types


export type HookResult<T> = readonly [
  T,
  ((value: T) => void) | undefined,
  boolean
];


export type HookNoSetResult<T> = readonly [
  T,
  undefined,
  boolean
];


export const READONLY_SETTER = undefined as undefined;
export const READONLY_CAN = false;


export function readonlyHookResult<T>(value: T): HookResult<T> {
  return [value, READONLY_SETTER, READONLY_CAN] as const;
}


export function writableHookResult<T>(
  value: T,
  setter: (value: T) => void,
  canSet: boolean = true
): HookResult<T> {
  return [value, canSet ? setter : undefined, canSet] as const;
}


export function conditionalHookResult<T>(
  canSet: boolean,
  value: T,
  setter: ((value: T) => void) | undefined
): HookResult<T> {
  return [value, canSet ? setter : undefined, canSet] as const;
}

export interface Field<T> {
  value: T;
  canSet: boolean;
  set: (next: T) => void;
}

export interface ActionField {
  canExecute: boolean;
  execute: () => void;
}

const NOOP_SETTER = () => {
  if (process.env.NODE_ENV === "development") {
    console.warn("[DEBUG] Attempted to set a disabled field");
  }
};

export function createField<T>(value: T, setter: (next: T) => void, canSet: boolean): Field<T> {
  return {
    value,
    canSet,
    set: canSet ? setter : NOOP_SETTER,
  };
}

export function createReadonlyField<T>(value: T): Field<T> {
  return {
    value,
    canSet: false,
    set: NOOP_SETTER,
  };
}

export function createAction(execute: () => void, canExecute: boolean): ActionField {
  return {
    canExecute,
    execute: canExecute ? execute : () => {
      if (process.env.NODE_ENV === "development") {
        console.warn("[DEBUG] Attempted to execute a disabled action");
      }
    },
  };
}

export function fieldToHookResult<T>(field: Field<T>): HookResult<T> {
  return [field.value, field.canSet ? field.set : undefined, field.canSet] as const;
}

export function hookResultToField<T>(result: HookResult<T>): Field<T> {
  const [value, setter, canSet] = result;
  return {
    value,
    canSet,
    set: setter ?? NOOP_SETTER,
  };
}

// #endregion Granular Hook Types

export type Url = string;

export type Subscribe = (callback: () => void) => () => void;

export type Disposable = () => void;

export type Transact = (fn: () => void, origin?: string) => void;

export type Unsubscribe = () => void;

export type YProviderFactory = (doc: Y.Doc, id: string) => Promise<void>;

export type AppKind = string;

export type Device = "desktop" | "tablet" | MobileDevice;

export type PanelKey = "details" | "workbench" | "tools" | "hud" | "stats" | "console" | "chat" | "settings" | "toolbar" | "leftSidePanel" | "rightSidePanel" | "hudPanel";

export type SidePanelKey = "leftSidePanel" | "rightSidePanel";

export type HudPanelKey = "hudPanel";

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

export enum SidePanelPosition {
  LEFT = "left",
  RIGHT = "right",
}

export interface SidePanelTab {
  id: string;
  icon: ComponentType<{ size?: number }>;
  order?: number;
  content: ReactNode | (() => ReactNode);
}

export interface HudPanelTab {
  id: string;
  icon: ComponentType<{ size?: number }>;
  order?: number;
  content: ReactNode | (() => ReactNode);
}

export interface SidePanelVisibility {
  left: boolean;
  right: boolean;
}

export interface HudPanelVisibility {
  visible: boolean;
}

export interface PanelVisibility {
  toolbar?: boolean;
  leftSidePanel?: boolean;
  rightSidePanel?: boolean;
  hudPanel?: boolean;
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
  leftSidePanelWidth: number;
  rightSidePanelWidth: number;
  hudPanelWidth: number;
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

export interface SidePanelTabs {
  left: SidePanelTab[];
  right: SidePanelTab[];
}

export interface HudPanelTabs {
  tabs: HudPanelTab[];
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
  leftSidePanel: SidePanelTab[];
  rightSidePanel: SidePanelTab[];
  hudPanel: HudPanelTab[];
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

export interface MobileDevice {
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
  device: Device;
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
  device?: Device;
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
  label?: string | any;
  icon?: ReactNode;
  component: (props: any) => ReactNode;
  controls?: WindowControl[];
  variants?: {
    id: string;
    icon?: ReactNode;
    componentProps?: Record<string, any>;
  }[];
}

export interface AppWindowConfig {
  windowKinds: WindowKindDefinition[];
  defaultLayout?: any;
}

export function parseWindowLayout(layout: unknown): any | undefined {
  if (layout === undefined || layout === null) return undefined;
  if (typeof layout === "string") {
    const trimmed = layout.trim();
    if (!trimmed) return undefined;
    try {
      return JSON.parse(trimmed);
    } catch {
      return undefined;
    }
  }
  if (typeof layout === "object") return layout;
  return undefined;
}

/**
 * Sanitizes a GoldenLayout config to ensure only one instance of each component type exists.
 * Removes duplicate components keeping only the first occurrence.
 */
export function deduplicateWindowLayout(layout: any, allowedWindowIds: string[]): any | undefined {
  if (!layout || typeof layout !== "object") return layout;

  const seenComponents = new Set<string>();

  const deduplicateContent = (content: any[]): any[] => {
    if (!Array.isArray(content)) return content;

    return content
      .map((item) => {
        if (!item || typeof item !== "object") return item;

        if (item.type === "component") {
          const componentName = item.componentName;
          if (seenComponents.has(componentName)) {
            return null; // Remove duplicate
          }
          if (!allowedWindowIds.includes(componentName)) {
            return null; // Remove unknown component types
          }
          seenComponents.add(componentName);
          return item;
        }

        if (item.content && Array.isArray(item.content)) {
          const deduped = deduplicateContent(item.content);
          if (deduped.length === 0) return null;
          return { ...item, content: deduped };
        }

        return item;
      })
      .filter((item) => item !== null);
  };

  const root = layout.root;
  if (!root || typeof root !== "object") return layout;

  if (root.content && Array.isArray(root.content)) {
    const dedupedContent = deduplicateContent(root.content);
    return { ...layout, root: { ...root, content: dedupedContent } };
  }

  return layout;
}

export function stringifyWindowLayout(layout: unknown): string | undefined {
  if (layout === undefined || layout === null) return undefined;
  try {
    return JSON.stringify(layout);
  } catch {
    return undefined;
  }
}

export interface AppWindowProps {
  kind: WindowKind;
  children: ReactNode;
  className?: string;
}

export function createDefaultLayout(windowIds: string[], direction: "row" | "column" = "row", sizes?: number[], titles?: string[]): any {
  return {
    root: {
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
    },
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
  icon?: ReactNode;
  text?: string;
  content?: ReactNode;
  onClick?: () => void;
  order?: number;
  className?: string;
  disabled?: boolean;
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
  device: Device;
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
  | { type: "SET_DEVICE"; device: Device }
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

// #region YPath Helpers

/**
 * Creates a YPath segment for accessing a map key.
 */
export function yPathMapKey(key: string): YPathSegment {
  return { kind: "mapKey", key };
}

/**
 * Creates a YPath segment for accessing an array index.
 */
export function yPathArrayIndex(index: number): YPathSegment {
  return { kind: "arrayIndex", index };
}

/**
 * Creates a YPath segment for finding an item in an array by its id.
 */
export function yPathArrayItemById(id: string, idKey: string = "guid"): YPathSegment {
  return { kind: "arrayItemById", id, idKey };
}

/**
 * Gets the value at a path in a Y.js structure.
 * Returns undefined if the path doesn't exist.
 */
export function getValueAtPath(root: Y.Map<any> | Y.Array<any>, path: YPath): any {
  let current: any = root;
  for (const segment of path) {
    if (current === undefined || current === null) return undefined;
    if (segment.kind === "mapKey") {
      if (!(current instanceof Y.Map)) return undefined;
      current = current.get(segment.key);
    } else if (segment.kind === "arrayIndex") {
      if (!(current instanceof Y.Array)) return undefined;
      current = current.get(segment.index);
    } else if (segment.kind === "arrayItemById") {
      if (!(current instanceof Y.Array)) return undefined;
      const arr = current.toArray();
      const item = arr.find((item: any) => {
        if (item instanceof Y.Map) return item.get(segment.idKey) === segment.id;
        return item?.[segment.idKey] === segment.id;
      });
      current = item;
    }
  }
  return current;
}

/**
 * Creates an observer for a specific path in a Y.js structure.
 * Only fires when the value at the path changes.
 */
export function createPathObserver(root: Y.Map<any>, path: YPath, subscribe: Subscribe): Disposable {
  if (path.length === 0) {
    const callback = () => subscribe(() => { });
    root.observeDeep(callback);
    return () => root.unobserveDeep(callback);
  }
  const disposables: Disposable[] = [];
  let lastValue = getValueAtPath(root, path);
  const notifyIfChanged = () => {
    const newValue = getValueAtPath(root, path);
    const lastJson = JSON.stringify(lastValue instanceof Y.Map || lastValue instanceof Y.Array ? lastValue.toJSON() : lastValue);
    const newJson = JSON.stringify(newValue instanceof Y.Map || newValue instanceof Y.Array ? newValue.toJSON() : newValue);
    if (lastJson !== newJson) {
      lastValue = newValue;
      subscribe(() => { });
    }
  };
  const setupObservers = (current: any, remainingPath: YPath, depth: number) => {
    if (!current || remainingPath.length === 0) return;
    const segment = remainingPath[0];
    const rest = remainingPath.slice(1);
    if (segment.kind === "mapKey" && current instanceof Y.Map) {
      const mapCallback = (event: Y.YMapEvent<any>) => {
        if (event.keysChanged.has(segment.key)) {
          disposables.slice(depth + 1).forEach((d) => d());
          disposables.length = depth + 1;
          const next = current.get(segment.key);
          if (rest.length > 0 && next) setupObservers(next, rest, depth + 1);
          notifyIfChanged();
        }
      };
      current.observe(mapCallback);
      disposables.push(() => current.unobserve(mapCallback));
      const next = current.get(segment.key);
      if (rest.length > 0 && next) setupObservers(next, rest, depth + 1);
      else if (rest.length === 0 && next instanceof Y.Map) {
        const deepCallback = () => notifyIfChanged();
        next.observeDeep(deepCallback);
        disposables.push(() => next.unobserveDeep(deepCallback));
      } else if (rest.length === 0 && next instanceof Y.Array) {
        const deepCallback = () => notifyIfChanged();
        next.observeDeep(deepCallback);
        disposables.push(() => next.unobserveDeep(deepCallback));
      }
    } else if (segment.kind === "arrayIndex" && current instanceof Y.Array) {
      const arrayCallback = () => notifyIfChanged();
      current.observe(arrayCallback);
      disposables.push(() => current.unobserve(arrayCallback));
      const next = current.get(segment.index);
      if (rest.length > 0 && next) setupObservers(next, rest, depth + 1);
    } else if (segment.kind === "arrayItemById" && current instanceof Y.Array) {
      const arrayCallback = () => {
        disposables.slice(depth + 1).forEach((d) => d());
        disposables.length = depth + 1;
        const arr = current.toArray();
        const item = arr.find((item: any) => {
          if (item instanceof Y.Map) return item.get(segment.idKey) === segment.id;
          return item?.[segment.idKey] === segment.id;
        });
        if (rest.length > 0 && item) setupObservers(item, rest, depth + 1);
        notifyIfChanged();
      };
      current.observe(arrayCallback);
      disposables.push(() => current.unobserve(arrayCallback));
      const arr = current.toArray();
      const item = arr.find((item: any) => {
        if (item instanceof Y.Map) return item.get(segment.idKey) === segment.id;
        return item?.[segment.idKey] === segment.id;
      });
      if (rest.length > 0 && item) setupObservers(item, rest, depth + 1);
    }
  };
  setupObservers(root, path, 0);
  return () => disposables.forEach((d) => d());
}

// #endregion YPath Helpers

// #region Derived Store

/**
 * A base dependency for a derived node - a path in a specific store.
 */
export interface BaseDependency {
  store: { onPathChanged: (path: YPath, subscribe: Subscribe) => Disposable; getPathSnapshot: (path: YPath) => any };
  path: YPath;
}

/**
 * A node in the derived dependency graph.
 * Caches computed values and only recomputes when dependencies change.
 */
export class DerivedNode<T> {
  private deps: BaseDependency[];
  private compute: () => T;
  private value: T | undefined;
  private valueJson?: string;
  private subscribers = new Set<() => void>();
  private unsubscribers: Disposable[] = [];
  private initialized = false;

  constructor(deps: BaseDependency[], compute: () => T) {
    this.deps = deps;
    this.compute = compute;
  }

  private init() {
    if (this.initialized) return;
    this.initialized = true;
    this.unsubscribers = this.deps.map((d) =>
      d.store.onPathChanged(d.path, () => {
        this.recompute();
        return () => { };
      }),
    );
    this.recompute();
  }

  private recompute() {
    const next = this.compute();
    const nextJson = JSON.stringify(next);
    if (nextJson !== this.valueJson) {
      this.value = next;
      this.valueJson = nextJson;
      this.subscribers.forEach((cb) => cb());
    }
  }

  snapshot(): T {
    if (!this.initialized) this.init();
    if (this.value === undefined) this.recompute();
    return this.value!;
  }

  subscribe(cb: () => void): Disposable {
    if (!this.initialized) this.init();
    this.subscribers.add(cb);
    return () => {
      this.subscribers.delete(cb);
      if (this.subscribers.size === 0) {
        this.unsubscribers.forEach((u) => u());
        this.unsubscribers = [];
        this.initialized = false;
        this.value = undefined;
        this.valueJson = undefined;
      }
    };
  }

  dispose() {
    this.unsubscribers.forEach((u) => u());
    this.unsubscribers = [];
    this.subscribers.clear();
    this.initialized = false;
    this.value = undefined;
    this.valueJson = undefined;
  }
}

/**
 * A store for managing derived nodes.
 * Provides caching and lazy initialization of computed values.
 */
export class DerivedStore {
  private nodes = new Map<string, DerivedNode<any>>();

  getOrCreate<T>(key: string, deps: BaseDependency[], compute: () => T): DerivedNode<T> {
    if (!this.nodes.has(key)) {
      this.nodes.set(key, new DerivedNode<T>(deps, compute));
    }
    return this.nodes.get(key)! as DerivedNode<T>;
  }

  get<T>(key: string): DerivedNode<T> | undefined {
    return this.nodes.get(key) as DerivedNode<T> | undefined;
  }

  delete(key: string): boolean {
    const node = this.nodes.get(key);
    if (node) {
      node.dispose();
      this.nodes.delete(key);
      return true;
    }
    return false;
  }

  clear() {
    this.nodes.forEach((node) => node.dispose());
    this.nodes.clear();
  }

  has(key: string): boolean {
    return this.nodes.has(key);
  }

  keys(): IterableIterator<string> {
    return this.nodes.keys();
  }
}

// #endregion Derived Store

// #region Store Factory Registry

// Factory types - using any to avoid circular type dependencies
export type DesignAppStoreFactory = (parent: any, id: any, state?: any) => any;
export type KitAppStoreFactory = (parent: any, yMap: any, transact: (fn: () => void) => void, id: any, state?: any) => any;
export type TypeAppStoreFactory = (parent: any, id: any, state?: any) => any;
export type QualityAppStoreFactory = (parent: any, id: any, state?: any) => any;

// Global factory registry - lives in shared.ts to avoid circular dependencies
let designAppStoreFactory: DesignAppStoreFactory | undefined;
let kitAppStoreFactory: KitAppStoreFactory | undefined;
let typeAppStoreFactory: TypeAppStoreFactory | undefined;
let qualityAppStoreFactory: QualityAppStoreFactory | undefined;

export function registerDesignAppStoreFactory(factory: DesignAppStoreFactory) {
  designAppStoreFactory = factory;
}

export function registerKitAppStoreFactory(factory: KitAppStoreFactory) {
  kitAppStoreFactory = factory;
}

export function registerTypeAppStoreFactory(factory: TypeAppStoreFactory) {
  typeAppStoreFactory = factory;
}

export function registerQualityAppStoreFactory(factory: QualityAppStoreFactory) {
  qualityAppStoreFactory = factory;
}

export function getDesignAppStoreFactory(): DesignAppStoreFactory {
  if (!designAppStoreFactory) throw new Error("Design app store factory not registered");
  return designAppStoreFactory;
}

export function getKitAppStoreFactory(): KitAppStoreFactory {
  if (!kitAppStoreFactory) throw new Error("Kit app store factory not registered");
  return kitAppStoreFactory;
}

export function getTypeAppStoreFactory(): TypeAppStoreFactory {
  if (!typeAppStoreFactory) throw new Error("Type app store factory not registered");
  return typeAppStoreFactory;
}

export function getQualityAppStoreFactory(): QualityAppStoreFactory {
  if (!qualityAppStoreFactory) throw new Error("Quality app store factory not registered");
  return qualityAppStoreFactory;
}

// #endregion Store Factory Registry

// #region App Plugin Registry

/**
 * Machine contribution from an app plugin.
 * Apps provide their own events, actions, guards, and selectors.
 */
export interface AppMachineContribution {
  /** Event type definitions (for TypeScript union type) */
  eventTypes?: Record<string, any>;
  /** Action implementations keyed by action name */
  actions?: Record<string, (context: any, event: any) => any>;
  /** Guard implementations keyed by guard name */
  guards?: Record<string, (context: any, event: any) => boolean>;
  /** Event-to-action/guard mappings for the machine's `on` config */
  eventHandlers?: Record<string, { guard?: string; actions?: string | string[] }>;
  /** Selector functions for reading app state */
  selectors?: Record<string, (context: any, ...args: any[]) => any>;
  /** Default state factory for this app */
  createDefaultState?: () => any;
}

/**
 * App plugin that contributes to the sketchpad machine.
 * Each app registers a plugin on module import to provide its machine contributions.
 */
export interface AppPlugin {
  /** Unique identifier for the app (e.g., "home", "kit", "type", "design") */
  id: string;
  /** Event namespace prefix (e.g., "HOME", "KIT", "TYPE", "DESIGN") */
  namespace: string;
  /** Machine contributions (events, actions, guards) */
  machine: AppMachineContribution;
  /** Optional store factory registration */
  registerStores?: () => void;
  /** Optional initialization hook called when plugin is registered */
  onRegister?: () => void;
}

// Global plugin registry
const appPlugins: Map<string, AppPlugin> = new Map();

/**
 * Register an app plugin.
 * Should be called as a side-effect when the app module is imported.
 */
export function registerAppPlugin(plugin: AppPlugin): void {
  if (appPlugins.has(plugin.id)) {
    console.warn(`App plugin "${plugin.id}" already registered, replacing...`);
  }
  appPlugins.set(plugin.id, plugin);

  // Call store registration if provided
  if (plugin.registerStores) {
    plugin.registerStores();
  }

  // Call initialization hook if provided
  if (plugin.onRegister) {
    plugin.onRegister();
  }
}

/**
 * Get all registered app plugins.
 */
export function getAppPlugins(): AppPlugin[] {
  return Array.from(appPlugins.values());
}

/**
 * Get a specific app plugin by ID.
 */
export function getAppPlugin(id: string): AppPlugin | undefined {
  return appPlugins.get(id);
}

/**
 * Check if an app plugin is registered.
 */
export function hasAppPlugin(id: string): boolean {
  return appPlugins.has(id);
}

/**
 * Compose all machine contributions from registered plugins.
 * Returns merged actions, guards, and event handlers.
 */
export function composePluginContributions(): {
  actions: Record<string, (context: any, event: any) => any>;
  guards: Record<string, (context: any, event: any) => boolean>;
  eventHandlers: Record<string, { guard?: string; actions?: string | string[] }>;
  selectors: Record<string, (context: any, ...args: any[]) => any>;
} {
  const actions: Record<string, any> = {};
  const guards: Record<string, any> = {};
  const eventHandlers: Record<string, any> = {};
  const selectors: Record<string, any> = {};

  for (const plugin of appPlugins.values()) {
    const contribution = plugin.machine;

    // Merge actions with namespace prefix
    if (contribution.actions) {
      for (const [name, fn] of Object.entries(contribution.actions)) {
        actions[name] = fn;
      }
    }

    // Merge guards with namespace prefix
    if (contribution.guards) {
      for (const [name, fn] of Object.entries(contribution.guards)) {
        guards[name] = fn;
      }
    }

    // Merge event handlers
    if (contribution.eventHandlers) {
      for (const [eventType, handler] of Object.entries(contribution.eventHandlers)) {
        eventHandlers[eventType] = handler;
      }
    }

    // Merge selectors with namespace prefix
    if (contribution.selectors) {
      for (const [name, fn] of Object.entries(contribution.selectors)) {
        selectors[`${plugin.id}.${name}`] = fn;
      }
    }
  }

  return { actions, guards, eventHandlers, selectors };
}

export function getPluginDefaultStates(): Record<string, any> {
  const defaults: Record<string, any> = {};
  for (const plugin of appPlugins.values()) {
    const createDefaultState = plugin.machine.createDefaultState;
    if (!createDefaultState) continue;
    defaults[plugin.id] = createDefaultState();
  }
  return defaults;
}

// #endregion App Plugin Registry

// #region Runtime Action Registry

export type RuntimeActionHandler<TContext = any, TEvent = any> = (context: TContext, event: TEvent) => Partial<TContext>;

const runtimeActionHandlers: Map<string, RuntimeActionHandler> = new Map();

export function registerRuntimeAction<TContext = any, TEvent = any>(actionName: string, handler: RuntimeActionHandler<TContext, TEvent>): void {
  runtimeActionHandlers.set(actionName, handler);
}

export function unregisterRuntimeAction(actionName: string): void {
  runtimeActionHandlers.delete(actionName);
}

export function getRuntimeAction(actionName: string): RuntimeActionHandler | undefined {
  return runtimeActionHandlers.get(actionName);
}

export function hasRuntimeAction(actionName: string): boolean {
  return runtimeActionHandlers.has(actionName);
}

export function executeRuntimeAction<TContext = any, TEvent = any>(actionName: string, context: TContext, event: TEvent): Partial<TContext> {
  const handler = runtimeActionHandlers.get(actionName);
  if (!handler) return {};
  return handler(context, event);
}

// #endregion Runtime Action Registry

// #region Dynamic Event Dispatch Registry

/**
 * Event handler configuration for dynamic dispatch.
 * Apps register handlers that are invoked when events matching their namespace are received.
 */
export interface EventHandlerConfig<TContext = any, TEvent = any> {
  /** Guard function - returns true if the event should be handled */
  guard?: (context: TContext, event: TEvent) => boolean;
  /** Action function - returns partial context updates */
  action: (context: TContext, event: TEvent) => Partial<TContext>;
}

/**
 * Registry mapping event types to their handlers.
 * Key is the full event type (e.g., "HOME.TOGGLE_PANEL", "DESIGN.SET_HOVER")
 */
const eventHandlerRegistry: Map<string, EventHandlerConfig> = new Map();

/**
 * Registry mapping guard names to guard functions.
 * Used by the sketchpad machine for dynamic guard lookup.
 */
const guardRegistry: Map<string, (context: any, event: any) => boolean> = new Map();

/**
 * Register an event handler for a specific event type.
 * Called by app modules to register their event handlers.
 * 
 * @example
 * ```ts
 * registerEventHandler("HOME.TOGGLE_PANEL", {
 *   action: (context, event) => {
 *     const { panel } = event;
 *     return {
 *       homeApp: {
 *         ...context.homeApp,
 *         panelVisibility: { ...context.homeApp.panelVisibility, [panel]: !context.homeApp.panelVisibility[panel] }
 *       }
 *     };
 *   }
 * });
 * ```
 */
export function registerEventHandler<TContext = any, TEvent = any>(
  eventType: string,
  config: EventHandlerConfig<TContext, TEvent>
): void {
  eventHandlerRegistry.set(eventType, config as EventHandlerConfig);
}

/**
 * Unregister an event handler.
 */
export function unregisterEventHandler(eventType: string): void {
  eventHandlerRegistry.delete(eventType);
}

/**
 * Check if an event handler is registered.
 */
export function hasEventHandler(eventType: string): boolean {
  return eventHandlerRegistry.has(eventType);
}

/**
 * Get an event handler configuration.
 */
export function getEventHandler(eventType: string): EventHandlerConfig | undefined {
  return eventHandlerRegistry.get(eventType);
}

/**
 * Execute a registered event handler.
 * Returns empty object if no handler is registered for the event type.
 */
export function executeEventHandler<TContext = any, TEvent extends { type: string } = any>(
  context: TContext,
  event: TEvent
): Partial<TContext> {
  const handler = eventHandlerRegistry.get(event.type);
  if (!handler) return {};

  // Check guard if present
  if (handler.guard && !handler.guard(context, event)) {
    return {};
  }

  return handler.action(context, event);
}

/**
 * Register a guard function.
 * Guards are used by the sketchpad machine to conditionally handle events.
 */
export function registerGuard(name: string, guard: (context: any, event: any) => boolean): void {
  guardRegistry.set(name, guard);
}

/**
 * Unregister a guard function.
 */
export function unregisterGuard(name: string): void {
  guardRegistry.delete(name);
}

/**
 * Get a guard function by name.
 */
export function getGuard(name: string): ((context: any, event: any) => boolean) | undefined {
  return guardRegistry.get(name);
}

/**
 * Check if a guard is registered.
 */
export function hasGuard(name: string): boolean {
  return guardRegistry.has(name);
}

/**
 * Execute a guard by name.
 * Returns false if guard is not found.
 */
export function executeGuard(name: string, context: any, event: any): boolean {
  const guard = guardRegistry.get(name);
  if (!guard) return false;
  return guard(context, event);
}

/**
 * Get all registered event types for a namespace.
 * Useful for debugging and documentation.
 */
export function getEventTypesForNamespace(namespace: string): string[] {
  const prefix = `${namespace}.`;
  return Array.from(eventHandlerRegistry.keys()).filter(key => key.startsWith(prefix));
}

/**
 * Get all registered namespaces.
 */
export function getRegisteredNamespaces(): string[] {
  const namespaces = new Set<string>();
  for (const eventType of eventHandlerRegistry.keys()) {
    const dotIndex = eventType.indexOf(".");
    if (dotIndex > 0) {
      namespaces.add(eventType.substring(0, dotIndex));
    }
  }
  return Array.from(namespaces);
}

export function getRegisteredEventTypes(): string[] {
  return Array.from(eventHandlerRegistry.keys());
}

// #endregion Dynamic Event Dispatch Registry
