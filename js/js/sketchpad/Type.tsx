// #region Header

// Type.tsx

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

import { arrayMove } from "@dnd-kit/sortable";
import { Edges, Line, Sphere, useFBX, useGLTF } from "@react-three/drei";
import { ThreeEvent, useLoader } from "@react-three/fiber";
import React, { createContext, FC, Suspense, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import { useParams } from "react-router";
import * as THREE from "three";
import { OBJLoader } from "three/addons/loaders/OBJLoader.js";
import * as Y from "yjs";
import { useLabel } from "../i18n";
import { Author, AuthorId, Camera, Coord, findModel, guid, Guid, Kit, Model, Point, Port, selectBestModel, File as SemioFile, toSemioRotation, toThreeRotation, Type, TypeDiff, Vector } from "../semio";
import { Geometry, Input, Scene as SceneComponent, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Slider, SortableTreeItems, Stepper, Textarea, Toggle, ToggleGroup, TreeContent, TreeItem } from "./elements";
import type { AppWindowConfig, KitCommandContext, KitDiffAppEdit, PanelDefinition, PanelVisibility, Tool, ToolDefinition, ToolRenderContext, Transact, TypeAppId, YAttributes, YLeafMapNumber, YLeafMapString, YStringArray } from "./shared";
import { AppConfig, createPanelDefinition, Expertise, Mode, PanelKind, Theme, ToolKind } from "./shared";
import type { KitStore, SketchpadStore, TypeStore } from "./Sketchpad";
import {
  Canvas,
  createDefaultLayout,
  identitySelector,
  KitDiffAppStore,
  KitScopeProvider,
  LayoutCanvas,
  registerTypeAppStoreFactory,
  ToolGroup,
  TypeScopeProvider,
  useAddFooterItem,
  useAddPanelSection,
  useAppType,
  useExpertise,
  useFocusSafe,
  useIsInTypeScope,
  useKit,
  useKitCommands,
  useKitFiles,
  useKitScope,
  useKitStore,
  useKitTags,
  useKitTransaction,
  useLanguage,
  useLayout,
  useMode,
  useRemoveFooterItem,
  useRemovePanelSection,
  useSketchpadCommands,
  useSketchpadStore,
  useSyncDeep,
  useSyncField,
  useTheme,
  useTooltip,
  useType,
  useTypeScope,
} from "./Sketchpad";

let kitAppModuleCache: any = null;
if (typeof window !== "undefined" && (window as any).__KIT_APP_MODULE_CACHE__) {
  kitAppModuleCache = (window as any).__KIT_APP_MODULE_CACHE__.kitAppModuleCache;
}
const getKitAppModule = () => {
  if (!kitAppModuleCache) {
    if (typeof window !== "undefined" && (window as any).__KIT_APP_MODULE_CACHE__) {
      kitAppModuleCache = (window as any).__KIT_APP_MODULE_CACHE__.kitAppModuleCache;
    }
    if (!kitAppModuleCache) {
      throw new Error("Kit app module not loaded. This should not happen - ensure kit app is imported.");
    }
  }
  return kitAppModuleCache;
};

const KitSectionLazy = React.lazy(async () => {
  const module = await import("./Kit");
  kitAppModuleCache = module;
  if (typeof window !== "undefined") {
    if (!(window as any).__KIT_APP_MODULE_CACHE__) {
      (window as any).__KIT_APP_MODULE_CACHE__ = {};
    }
    (window as any).__KIT_APP_MODULE_CACHE__.kitAppModuleCache = module;
  }
  return { default: module.KitSection };
});

// Lucide icons used throughout the app
import { AddIcon, AwardIcon, CheckIcon, CodeIcon, HandIcon, MonitorIcon, MoonIcon, MousePointerIcon, PortIcon, RemoveIcon, SelectToolIcon, SunIcon, TutorialIcon, UserIcon } from "@semio/assets";

// #endregion Imports

// #region Store

type YTypeAppVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YTypeApp = Y.Map<YTypeAppVal>;
type YTypeApps = Y.Map<YTypeApp>;

export interface TypeAppSelection {
  ports?: Guid[];
  models?: Guid[];
}
export interface TypeAppSelectionPortsDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface TypeAppSelectionModelsDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface TypeAppSelectionDiff {
  ports?: TypeAppSelectionPortsDiff;
  models?: TypeAppSelectionModelsDiff;
}
export enum TypeAppFullscreenWindow {
  None = "none",
  Ports = "ports",
  Models = "models",
}

export enum TypeAppWindowKind {
  Scene = "scene",
}
export interface TypeAppPresence {
  cursor?: Coord;
  camera?: Camera;
}
export interface TypeAppHover {
  port?: Guid;
  model?: Guid;
}
export interface TypeAppPresenceOther extends TypeAppPresence {
  name: string;
}
export interface TypeAppDiff {
  selection?: TypeAppSelectionDiff;
  presence?: TypeAppPresence;
  hover?: TypeAppHover;
  fullscreenWindow?: TypeAppFullscreenWindow;
  panelVisibility?: Partial<PanelVisibility>;
  activeTool?: ToolKind;
  camera?: Camera;
  focusedPortGuid?: Guid | null;
  selectedModelGuid?: Guid | null;
  selectedModelTags?: string[];
  windowLayout?: any;
}
export interface TypeAppEdit extends KitDiffAppEdit<TypeAppSelectionDiff> {}
export interface TypeAppState {
  fullscreenWindow: TypeAppFullscreenWindow;
  panelVisibility: PanelVisibility;
  activeTool: ToolKind;
  selection?: TypeAppSelection;
  hover?: TypeAppHover;
  presence?: TypeAppPresence;
  others: TypeAppPresenceOther[];
  camera?: Camera;
  focusedPortGuid?: Guid;
  selectedModelGuid?: Guid;
  selectedModelTags?: string[];
  windowLayout?: any;
}

export interface TypeAppCommandContext extends KitCommandContext {
  typeApp: TypeAppState;
  Guid: Guid;
}
export interface TypeAppCommandResult {
  diff?: TypeAppDiff;
  typeDiff?: TypeDiff;
}

function inverseTypeAppSelectionDiff(selection: TypeAppSelection, diff: TypeAppSelectionDiff): TypeAppSelectionDiff {
  const inverse: TypeAppSelectionDiff = {};
  if (diff.ports) {
    inverse.ports = {
      added: diff.ports.removed ?? [],
      removed: diff.ports.added ?? [],
    };
  }
  if (diff.models) {
    inverse.models = {
      added: diff.models.removed ?? [],
      removed: diff.models.added ?? [],
    };
  }
  return inverse;
}

class TypeAppStore extends KitDiffAppStore<TypeAppState, TypeAppDiff, TypeAppSelectionDiff, TypeAppEdit, TypeAppCommandContext, TypeAppCommandResult> {
  private readonly Guid: TypeAppId;
  // PERF: Cache getters to avoid creating new objects on every access
  private _cachedCameraStr?: string;
  private _cachedCamera?: Camera;
  private _cachedSelectionVersion = 0;
  private _cachedSelection?: TypeAppSelection;
  private _cachedHoverVersion = 0;
  private _cachedHover?: TypeAppHover;

  constructor(parent: SketchpadStore, yMap: Y.Map<any>, transact: Transact, id: TypeAppId) {
    super(parent, yMap, transact);
    this.Guid = id;

    transact(() => {
      if (!yMap.has("fullscreenWindow")) {
        yMap.set("fullscreenWindow", TypeAppFullscreenWindow.None);
      }
      if (!yMap.has("activeTool")) {
        yMap.set("activeTool", ToolKind.SELECTION_NORMAL);
      }
      if (!yMap.has("panelVisibility")) {
        const yPanelVisibility = new Y.Map<boolean>();
        yPanelVisibility.set("toolbar", true);
        yPanelVisibility.set("workbench", false);
        yPanelVisibility.set("details", false);
        yPanelVisibility.set("chat", false);
        yPanelVisibility.set("settings", false);
        yMap.set("panelVisibility", yPanelVisibility);
      } else {
        // Ensure toolbar is always visible for existing instances
        const yPanelVisibility = yMap.get("panelVisibility") as Y.Map<boolean>;
        if (yPanelVisibility && yPanelVisibility.get("toolbar") !== true) {
          yPanelVisibility.set("toolbar", true);
        }
      }
      // PERF: Clear corrupted windowLayout that has multiple windows
      // Type app should only have 1 Scene window - more causes severe performance issues
      if (yMap.has("windowLayout")) {
        const layoutStr = yMap.get("windowLayout") as string | undefined;
        if (layoutStr) {
          try {
            const layout = typeof layoutStr === "string" ? JSON.parse(layoutStr) : layoutStr;
            const countWindows = (node: any): number => {
              if (!node) return 0;
              if (node.type === "component") return 1;
              if (node.content && Array.isArray(node.content)) {
                return node.content.reduce((sum: number, child: any) => sum + countWindows(child), 0);
              }
              return 0;
            };
            if (countWindows(layout) > 1) {
              console.warn(`[TypeAppStore] Clearing corrupted layout with ${countWindows(layout)} windows`);
              yMap.delete("windowLayout");
            }
          } catch {
            // If layout is corrupted/unparseable, clear it
            yMap.delete("windowLayout");
          }
        }
      }
    });

    Object.entries(commands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  type(): TypeStore | undefined {
    return this.parent.kit(this.Guid.kit).type(this.Guid.type);
  }

  kit(): KitStore {
    return this.parent.kit(this.Guid.kit);
  }

  // TypeApp-specific getters
  get fullscreenWindow(): TypeAppFullscreenWindow {
    return this.yMap.get("fullscreenWindow") as TypeAppFullscreenWindow;
  }

  get activeTool(): ToolKind {
    const value = this.yMap.get("activeTool") as ToolKind;
    if (value === undefined) {
      this.transact(() => {
        this.yMap.set("activeTool", ToolKind.SELECTION_NORMAL);
      });
      return ToolKind.SELECTION_NORMAL;
    }
    return value;
  }

  get panelVisibility(): PanelVisibility {
    const yPanelVisibility = this.yMap.get("panelVisibility") as Y.Map<boolean>;
    if (!yPanelVisibility) {
      return {
        toolbar: true,
        workbench: false,
        details: false,
        chat: false,
        settings: false,
      };
    }
    return {
      toolbar: yPanelVisibility.get("toolbar") ?? true,
      workbench: yPanelVisibility.get("workbench") ?? false,
      details: yPanelVisibility.get("details") ?? false,
      chat: yPanelVisibility.get("chat") ?? false,
      settings: yPanelVisibility.get("settings") ?? false,
    };
  }

  get selection(): TypeAppSelection {
    const selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) return {};

    // PERF: Cache selection object - only rebuild if Y.Map version changed
    const currentVersion = (selection as any)._clock || 0;
    if (this._cachedSelection && this._cachedSelectionVersion === currentVersion) {
      return this._cachedSelection;
    }

    const result: TypeAppSelection = {};
    const ports = selection.get("ports") as Y.Array<string>;
    if (ports) {
      result.ports = ports.toArray();
    }
    const models = selection.get("models") as Y.Array<string>;
    if (models) {
      result.models = models.toArray();
    }

    this._cachedSelectionVersion = currentVersion;
    this._cachedSelection = result;
    return result;
  }

  get hover(): TypeAppHover | undefined {
    const hover = this.yMap.get("hover") as Y.Map<string>;
    if (!hover) return undefined;

    // PERF: Cache hover object - only rebuild if Y.Map version changed
    const currentVersion = (hover as any)._clock || 0;
    if (this._cachedHover && this._cachedHoverVersion === currentVersion) {
      return this._cachedHover;
    }

    const result: TypeAppHover = {};
    const port = hover.get("port");
    if (port) result.port = port;
    const model = hover.get("model");
    if (model) result.model = model;

    this._cachedHoverVersion = currentVersion;
    this._cachedHover = result;
    return result;
  }

  get presence(): TypeAppPresence | undefined {
    return undefined;
  }

  get others(): TypeAppPresenceOther[] {
    return [];
  }

  get camera(): Camera | undefined {
    const cameraStr = this.yMap.get("camera") as string | undefined;
    // PERF: Cache parsed camera - only rebuild if string changed
    if (cameraStr === this._cachedCameraStr) {
      return this._cachedCamera;
    }
    this._cachedCameraStr = cameraStr;
    this._cachedCamera = cameraStr ? JSON.parse(cameraStr) : undefined;
    return this._cachedCamera;
  }

  get focusedPortGuid(): Guid | undefined {
    return this.yMap.get("focusedPortGuid") as Guid | undefined;
  }

  get selectedModelGuid(): Guid | undefined {
    return this.yMap.get("selectedModelGuid") as Guid | undefined;
  }

  get selectedModelTags(): string[] {
    const yTags = this.yMap.get("selectedModelTags") as Y.Array<string> | undefined;
    return yTags ? yTags.toArray() : [];
  }

  get windowLayout(): any {
    const layoutStr = this.yMap.get("windowLayout") as string | undefined;
    if (!layoutStr) return undefined;
    try {
      const layout = JSON.parse(layoutStr);
      // PERF: Validate layout - Type app should only have 1 Scene window
      const countWindows = (node: any): number => {
        if (!node) return 0;
        if (node.type === "component") return 1;
        if (node.content && Array.isArray(node.content)) {
          return node.content.reduce((sum: number, child: any) => sum + countWindows(child), 0);
        }
        return 0;
      };
      const windowCount = countWindows(layout);
      if (windowCount > 1) {
        console.warn(`[TypeAppStore] Corrupted layout with ${windowCount} windows, returning undefined`);
        // Clear the corrupted layout from storage
        this.transact(() => {
          this.yMap.delete("windowLayout");
        });
        return undefined;
      }
      return layout;
    } catch {
      return undefined;
    }
  }
  set windowLayout(layout: any) {
    if (layout) {
      // PERF: Only save layouts with 1 or fewer windows
      const countWindows = (node: any): number => {
        if (!node) return 0;
        if (node.type === "component") return 1;
        if (node.content && Array.isArray(node.content)) {
          return node.content.reduce((sum: number, child: any) => sum + countWindows(child), 0);
        }
        return 0;
      };
      if (countWindows(layout) > 1) {
        console.warn(`[TypeAppStore] Rejecting layout with ${countWindows(layout)} windows`);
        return;
      }
      this.yMap.set("windowLayout", JSON.stringify(layout));
    } else {
      this.yMap.delete("windowLayout");
    }
  }

  protected hash(state: TypeAppState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): TypeAppState {
    return {
      fullscreenWindow: this.fullscreenWindow,
      panelVisibility: this.panelVisibility,
      activeTool: this.activeTool,
      selection: this.selection,
      hover: this.hover,
      isTransactionActive: this.isTransactionActive,
      canUndo: this.canUndo(),
      canRedo: this.canRedo(),
      presence: this.presence,
      others: this.others,
      // diff: this.diff, // TODO: TypeAppState doesn't have a diff property
      currentTransactionStack: this.currentTransactionStack,
      pastTransactionsStack: this.pastTransactionsStack,
      camera: this.camera,
      focusedPortGuid: this.focusedPortGuid,
      selectedModelGuid: this.selectedModelGuid,
      selectedModelTags: this.selectedModelTags,
      windowLayout: this.windowLayout,
    } as any;
  }

  protected inverseSelectionDiff(selection: TypeAppSelection, diff: TypeAppSelectionDiff): TypeAppSelectionDiff {
    return inverseTypeAppSelectionDiff(selection, diff);
  }

  protected applySelectionDiff = (selectionDiff: TypeAppSelectionDiff) => {
    let selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) {
      selection = new Y.Map();
      this.yMap.set("selection", selection);
    }

    if (selectionDiff.ports) {
      let yPorts = selection.get("ports") as Y.Array<string>;
      if (!yPorts) {
        yPorts = new Y.Array<string>();
        selection.set("ports", yPorts);
      }

      if (selectionDiff.ports.removed) {
        for (const Guid of selectionDiff.ports.removed) {
          const index = yPorts.toArray().indexOf(Guid);
          if (index >= 0) yPorts.delete(index, 1);
        }
      }

      if (selectionDiff.ports.added) {
        for (const Guid of selectionDiff.ports.added) {
          if (!yPorts.toArray().includes(Guid)) {
            yPorts.push([Guid]);
          }
        }
      }
    }

    if (selectionDiff.models) {
      let yModels = selection.get("models") as Y.Array<string>;
      if (!yModels) {
        yModels = new Y.Array<string>();
        selection.set("models", yModels);
      }

      if (selectionDiff.models.removed) {
        for (const repId of selectionDiff.models.removed) {
          const index = yModels.toArray().indexOf(repId);
          if (index >= 0) yModels.delete(index, 1);
        }
      }

      if (selectionDiff.models.added) {
        for (const repId of selectionDiff.models.added) {
          if (!yModels.toArray().includes(repId)) {
            yModels.push([repId]);
          }
        }
      }
    }
  };

  protected getSelection(): TypeAppSelection {
    return this.selection;
  }

  change = (diff: TypeAppDiff) => {
    this.transact(() => {
      if (diff.fullscreenWindow) {
        this.yMap.set("fullscreenWindow", diff.fullscreenWindow);
      }
      if (diff.activeTool !== undefined) {
        this.yMap.set("activeTool", diff.activeTool);
      }
      if (diff.panelVisibility !== undefined) {
        let yPanelVisibility = this.yMap.get("panelVisibility") as Y.Map<boolean>;
        if (!yPanelVisibility) {
          yPanelVisibility = new Y.Map<boolean>();
          this.yMap.set("panelVisibility", yPanelVisibility);
        }
        Object.entries(diff.panelVisibility).forEach(([key, value]) => {
          if (value !== undefined) {
            yPanelVisibility.set(key, value);
          }
        });
      }
      if (diff.selection) {
        this.applySelectionDiff(diff.selection);
      }
      if (diff.hover) {
        let yHover = this.yMap.get("hover") as Y.Map<string>;
        if (!yHover) {
          yHover = new Y.Map<string>();
          this.yMap.set("hover", yHover);
        }
        if (diff.hover.port !== undefined) {
          if (diff.hover.port) {
            yHover.set("port", diff.hover.port);
          } else {
            yHover.delete("port");
          }
        }
        if (diff.hover.model !== undefined) {
          if (diff.hover.model) {
            yHover.set("model", diff.hover.model);
          } else {
            yHover.delete("model");
          }
        }
        if (diff.hover.model !== undefined) {
          if (diff.hover.model) {
            yHover.set("model", diff.hover.model);
          } else {
            yHover.delete("model");
          }
        }
      }
      if (diff.presence) {
        // Handle presence changes if needed
      }
      if (diff.camera) {
        this.yMap.set("camera", JSON.stringify(diff.camera));
      }
      if (diff.focusedPortGuid !== undefined) {
        if (diff.focusedPortGuid === null) {
          this.yMap.delete("focusedPortGuid");
        } else {
          this.yMap.set("focusedPortGuid", diff.focusedPortGuid);
        }
      }
      if (diff.selectedModelGuid !== undefined) {
        if (diff.selectedModelGuid === null) {
          this.yMap.delete("selectedModelGuid");
        } else {
          this.yMap.set("selectedModelGuid", diff.selectedModelGuid);
        }
      }
      if (diff.selectedModelTags !== undefined) {
        let yTags = this.yMap.get("selectedModelTags") as Y.Array<string>;
        if (!yTags) {
          yTags = new Y.Array<string>();
          this.yMap.set("selectedModelTags", yTags);
        }
        yTags.delete(0, yTags.length);
        if (diff.selectedModelTags.length > 0) {
          yTags.push(diff.selectedModelTags);
        }
      }
      if (diff.windowLayout !== undefined) {
        this.windowLayout = diff.windowLayout;
      }
    });
  };

  async executeCommand<T>(command: string, ...args: any[]): Promise<T> {
    let origin: string | undefined;
    let rest: any[];

    // Origins are strings like "semio.sketchpad.app.type.panel.details.name" (starts with semio.sketchpad)
    // Commands are strings like "semio.typeApp.startTransaction" (starts with semio. but NOT semio.sketchpad)
    if (typeof args[0] === "string" && args[0].startsWith("semio.sketchpad.")) {
      origin = args[0];
      rest = args.slice(1);
    } else {
      origin = undefined;
      rest = args;
    }

    if (command === "semio.typeApp.startTransaction") {
      console.group(`[${origin || "unknown"}] Transaction: "${command}"`);
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.typeApp.finalizeTransaction") {
      this.finalizeTransaction();
      console.groupEnd();
      return {} as T;
    }
    if (command === "semio.typeApp.abortTransaction") {
      this.abortTransaction();
      console.groupEnd();
      return {} as T;
    }
    if (command === "semio.typeApp.undo") {
      this.undo();
      return {} as T;
    }
    if (command === "semio.typeApp.redo") {
      this.redo();
      return {} as T;
    }
    console.group(`[${origin || "unknown"}] Executing command: "${command}"`);
    const callback = this.commandRegistry.get(command);
    if (!callback) {
      console.groupEnd();
      throw new Error(`Command "${command}" not found in type app store`);
    }
    const typeApp = this.snapshot();
    const kitStore = this.kit();
    const kit = kitStore.snapshot();
    const typeStore = this.type();
    const typeGuid = typeStore?.guid ?? this.Guid.type;

    const context: TypeAppCommandContext = {
      kit,
      typeApp,
      Guid: typeGuid,
      fileUrls: kitStore.fileUrls,
      origin,
    };
    const result = callback(context, ...rest);
    if (result.diff) {
      this.change(result.diff);
    }
    if (result.typeDiff) {
      // Apply type diff to the type store
      if (typeStore) {
        typeStore.change(result.typeDiff);
      }
    }
    this.recordEdit(result);
    console.groupEnd();
    return result as T;
  }

  async execute<T>(command: string, ...rest: any[]): Promise<T> {
    return this.executeCommand<T>(command, ...rest);
  }
}

if (typeof window !== "undefined") {
  registerTypeAppStoreFactory((parent, yMap, transact, id) => new TypeAppStore(parent, yMap, transact, id));
}

export function useTypeAppStore<T>(selector?: (store: TypeAppStore) => T, id?: TypeAppId): T | TypeAppStore | null {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const resolvedKitId = kitScope?.guid ?? id?.kit;
  const resolvedTypeId = typeScope?.guid ?? id?.type;
  if (!resolvedKitId || !resolvedTypeId) return null;
  const typeAppStore = store.typeApp(resolvedKitId, resolvedTypeId);
  return selector ? selector(typeAppStore) : typeAppStore;
}

// Stable selectors for TypeApp hooks - must be module-level to avoid infinite loops with useSyncExternalStore
const EMPTY_TYPE_SELECTION: TypeAppSelection = {};
const EMPTY_PANEL_VISIBILITY: PanelVisibility = { toolbar: false, workbench: false, details: false, chat: false, settings: false };
const EMPTY_OTHERS: TypeAppPresenceOther[] = [];

const selectTypeAppState = (state: TypeAppState) => state;
const selectTypeAppSelection = (s: TypeAppState) => s.selection ?? EMPTY_TYPE_SELECTION;
const selectTypeAppPanelVisibility = (s: TypeAppState) => s.panelVisibility;
const selectTypeAppOthers = (s: TypeAppState) => s.others;
const selectTypeAppCamera = (s: TypeAppState) => s.camera;
const selectTypeAppFocusedPortGuid = (s: TypeAppState) => s.focusedPortGuid;

export function useTypeApp<T>(selector?: (state: TypeAppState) => T, id?: TypeAppId): T | TypeAppState | null {
  const store = useTypeAppStore(identitySelector, id);
  // PERF: Use the provided selector or fall back to stable identity selector
  const stableSelector = useMemo(() => selector || selectTypeAppState, [selector]);
  if (!store) return null;
  return useSyncDeep<TypeAppState, T>(store as TypeAppStore, stableSelector as (state: TypeAppState) => T);
}

export function useTypeAppSelection(id?: TypeAppId): TypeAppSelection {
  const store = useTypeAppStore(identitySelector, id);
  if (!store) return EMPTY_TYPE_SELECTION;
  return useSyncField<TypeAppState, TypeAppSelection>(store as TypeAppStore, "selection", selectTypeAppSelection);
}

export function useTypeAppPanelVisibility(id?: TypeAppId): PanelVisibility {
  const store = useTypeAppStore(identitySelector, id);
  if (!store) return EMPTY_PANEL_VISIBILITY;
  return useSyncField<TypeAppState, PanelVisibility>(store as TypeAppStore, "panelVisibility", selectTypeAppPanelVisibility);
}

export function useTypeAppOthers(id?: TypeAppId): TypeAppPresenceOther[] {
  const store = useTypeAppStore(identitySelector, id);
  if (!store) return EMPTY_OTHERS;
  return useSyncField<TypeAppState, TypeAppPresenceOther[]>(store as TypeAppStore, "others", selectTypeAppOthers);
}

export function useTypeAppCamera(id?: TypeAppId): Camera | undefined {
  const store = useTypeAppStore(identitySelector, id);
  if (!store) return undefined;
  return useSyncField<TypeAppState, Camera | undefined>(store as TypeAppStore, "camera", selectTypeAppCamera, false);
}

export function useTypeAppFocusedPortGuid(id?: TypeAppId): Guid | undefined {
  const store = useTypeAppStore(identitySelector, id);
  if (!store) return undefined;
  return useSyncField<TypeAppState, Guid | undefined>(store as TypeAppStore, "focusedPortGuid", selectTypeAppFocusedPortGuid, false);
}

export function useTypeAppCommands(id?: TypeAppId) {
  const store = useTypeAppStore(undefined, id) as TypeAppStore | null;
  const noOp = () => {};
  if (!store) {
    return {
      startTransaction: noOp,
      finalizeTransaction: noOp,
      abortTransaction: noOp,
      undo: noOp,
      redo: noOp,
      selectAll: noOp,
      deselectAll: noOp,
      togglePanel: noOp,
      setCamera: noOp,
      focusPort: noOp,
      clearFocus: noOp,
      setActiveTool: noOp,
      selectPort: noOp,
      deselectPort: noOp,
      selectModel: noOp,
      deselectModel: noOp,
      hoverPort: noOp,
      hoverModel: noOp,
      clearHover: noOp,
      setSelectedModel: noOp,
      addModelTag: noOp,
      removeModelTag: noOp,
      clearModelTags: noOp,
      setModelTags: noOp,
      execute: noOp,
    };
  }
  return {
    startTransaction: (origin: string) => store.execute("semio.typeApp.startTransaction", origin),
    finalizeTransaction: (origin: string) => store.execute("semio.typeApp.finalizeTransaction", origin),
    abortTransaction: (origin: string) => store.execute("semio.typeApp.abortTransaction", origin),
    undo: (origin: string) => store.execute("semio.typeApp.undo", origin),
    redo: (origin: string) => store.execute("semio.typeApp.redo", origin),
    selectAll: (origin: string) => store.execute("semio.typeApp.selectAll", origin),
    deselectAll: (origin: string) => store.execute("semio.typeApp.deselectAll", origin),
    togglePanel: (origin: string, panelKey: keyof PanelVisibility) => {
      const current = store.snapshot().panelVisibility;
      store.change({
        panelVisibility: {
          [panelKey]: !current[panelKey],
        },
      });
    },
    setCamera: (origin: string, camera: Camera) => {
      store.change({ camera });
    },
    focusPort: (origin: string, portGuid: Guid) => store.execute("semio.typeApp.focusPort", origin, portGuid),
    clearFocus: (origin: string) => store.execute("semio.typeApp.clearFocus", origin),
    setActiveTool: (origin: string, tool: ToolKind) => {
      store.change({ activeTool: tool });
    },
    selectPort: (origin: string, portId: Guid) => {
      const selection = store.selection;
      const ports = selection.ports || [];
      if (!ports.includes(portId)) {
        store.change({
          selection: {
            ports: {
              added: [portId],
            },
          },
        });
      }
    },
    deselectPort: (origin: string, portId?: Guid) => {
      const selection = store.selection;
      const ports = selection.ports || [];
      if (portId) {
        if (ports.includes(portId)) {
          store.change({
            selection: {
              ports: {
                removed: [portId],
              },
            },
          });
        }
      } else {
        store.change({
          selection: {
            ports: {
              removed: ports,
            },
          },
        });
      }
    },
    selectModel: (origin: string, modelId: Guid) => {
      const selection = store.selection;
      const models = selection.models || [];
      if (!models.includes(modelId)) {
        store.change({
          selection: {
            models: {
              added: [modelId],
            },
          },
        });
      }
    },
    deselectModel: (origin: string, modelId?: Guid) => {
      const selection = store.selection;
      const models = selection.models || [];
      if (modelId) {
        if (models.includes(modelId)) {
          store.change({
            selection: {
              models: {
                removed: [modelId],
              },
            },
          });
        }
      } else {
        store.change({
          selection: {
            models: {
              removed: models,
            },
          },
        });
      }
    },
    hoverPort: (origin: string, portId: Guid) => {
      store.change({
        hover: {
          port: portId,
        },
      });
    },
    hoverModel: (origin: string, modelId: Guid) => {
      store.change({
        hover: {
          model: modelId,
        },
      });
    },
    clearHover: (origin: string) => {
      store.change({
        hover: {
          port: undefined,
          model: undefined,
        },
      });
    },
    setSelectedModel: (origin: string, modelGuid: Guid) => {
      store.change({
        selectedModelGuid: modelGuid,
      });
    },
    addModelTag: (origin: string, tag: string) => store.execute("semio.typeApp.addModelTag", origin, tag),
    removeModelTag: (origin: string, tag: string) => store.execute("semio.typeApp.removeModelTag", origin, tag),
    clearModelTags: (origin: string) => store.execute("semio.typeApp.clearModelTags", origin),
    setModelTags: (origin: string, tags: string[]) => store.execute("semio.typeApp.setModelTags", origin, tags),
    execute: (origin: string, command: string, ...args: any[]) => store.execute(command, origin, ...args),
  };
}

// Stable selectors for TypeApp hover and active tool
const selectTypeAppHover = (s: TypeAppState) => s.hover;
const selectTypeAppActiveTool = (s: TypeAppState) => s.activeTool ?? ToolKind.SELECTION_NORMAL;

export function useTypeAppHover(id?: TypeAppId): TypeAppHover | undefined {
  const store = useTypeAppStore(identitySelector, id);
  if (!store) return undefined;
  return useSyncField<TypeAppState, TypeAppHover | undefined>(store as TypeAppStore, "hover", selectTypeAppHover);
}

export function useTypeAppActiveTool(id?: TypeAppId): ToolKind {
  const store = useTypeAppStore(identitySelector, id);
  if (!store) return ToolKind.SELECTION_NORMAL;
  return useSyncField<TypeAppState, ToolKind>(store as TypeAppStore, "activeTool", selectTypeAppActiveTool, false);
}

interface Transaction {
  start?: () => void;
  finalize?: () => void;
  abort?: () => void;
}

export function useTypeAppTransaction(origin: string, id?: TypeAppId): Transaction {
  const store = useTypeAppStore(undefined, id) as TypeAppStore | null;
  if (!store) return {};
  return {
    start: () => store.execute("semio.typeApp.startTransaction", origin),
    finalize: () => store.execute("semio.typeApp.finalizeTransaction", origin),
    abort: () => store.execute("semio.typeApp.abortTransaction", origin),
  };
}

export function useTypeAppIsPortSelected(id: TypeAppId | undefined, portId: string): boolean {
  const store = useTypeAppStore(identitySelector, id);
  // PERF: Memoize the selector to prevent infinite loops
  const selector = useCallback((s: TypeAppState) => s.selection?.ports?.includes(portId) || false, [portId]);
  if (!store) return false;
  return useSyncField<TypeAppState, boolean>(store as TypeAppStore, "selection", selector);
}

export function useTypeAppIsPortHovered(id: TypeAppId | undefined, portId: string): boolean {
  const store = useTypeAppStore(identitySelector, id);
  // PERF: Memoize the selector to prevent infinite loops
  const selector = useCallback((s: TypeAppState) => s.hover?.port === portId, [portId]);
  if (!store) return false;
  return useSyncField<TypeAppState, boolean>(store as TypeAppStore, "hover", selector);
}

// Stable selectors for TypeApp hooks - must be module-level to avoid infinite loops with useSyncExternalStore
const EMPTY_MODEL_TAG_ARRAY: string[] = [];
const selectSelectedModelGuid = (s: TypeAppState) => s.selectedModelGuid;
const selectSelectedModelTags = (s: TypeAppState) => s.selectedModelTags ?? EMPTY_MODEL_TAG_ARRAY;

export function useTypeAppSelectedModelGuid(id?: TypeAppId): Guid | undefined {
  const store = useTypeAppStore(identitySelector, id);
  if (!store) return undefined;
  return useSyncField<TypeAppState, Guid | undefined>(store as TypeAppStore, "selectedModelGuid", selectSelectedModelGuid, false);
}

export function useTypeAppSelectedModelTags(id?: TypeAppId): string[] {
  const store = useTypeAppStore(identitySelector, id);
  if (!store) return EMPTY_MODEL_TAG_ARRAY;
  return useSyncField<TypeAppState, string[]>(store as TypeAppStore, "selectedModelTags", selectSelectedModelTags);
}

const TypeAppScopeContext = createContext<{ id: string } | undefined>(undefined);
export const TypeAppScopeProvider = (props: { id: string; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(TypeAppScopeContext.Provider, { value }, props.children as any);
};
const useTypeAppScope = () => useContext(TypeAppScopeContext);

// #endregion Store

// #region Commands

export const commands = {
  "semio.typeApp.selectPort": (context: TypeAppCommandContext, portGuid: Guid): TypeAppCommandResult => {
    const currentPorts = context.typeApp.selection?.ports || [];
    return {
      diff: {
        selection: {
          ports: { added: [portGuid], removed: [] },
        },
      },
    };
  },
  "semio.typeApp.deselectPort": (context: TypeAppCommandContext, portGuid: Guid): TypeAppCommandResult => {
    return {
      diff: {
        selection: {
          ports: { added: [], removed: [portGuid] },
        },
      },
    };
  },
  "semio.typeApp.selectModel": (context: TypeAppCommandContext, reprGuid: Guid): TypeAppCommandResult => {
    return {
      diff: {
        selection: {
          models: { added: [reprGuid], removed: [] },
        },
      },
    };
  },
  "semio.typeApp.deselectModel": (context: TypeAppCommandContext, reprGuid: Guid): TypeAppCommandResult => {
    return {
      diff: {
        selection: {
          models: { added: [], removed: [reprGuid] },
        },
      },
    };
  },
  "semio.typeApp.hoverPort": (context: TypeAppCommandContext, portGuid: Guid): TypeAppCommandResult => {
    return {
      diff: {
        hover: { port: portGuid },
      },
    };
  },
  "semio.typeApp.hoverModel": (context: TypeAppCommandContext, reprGuid: Guid): TypeAppCommandResult => {
    return {
      diff: {
        hover: { model: reprGuid },
      },
    };
  },
  "semio.typeApp.clearHover": (context: TypeAppCommandContext): TypeAppCommandResult => {
    return {
      diff: {
        hover: {},
      },
    };
  },
  "semio.typeApp.setCamera": (context: TypeAppCommandContext, camera: Camera): TypeAppCommandResult => {
    return {
      diff: {
        camera,
      },
    };
  },
  "semio.typeApp.focusPort": (context: TypeAppCommandContext, portGuid: Guid): TypeAppCommandResult => {
    return {
      diff: {
        focusedPortGuid: portGuid,
      },
    };
  },
  "semio.typeApp.clearFocus": (context: TypeAppCommandContext): TypeAppCommandResult => {
    return {
      diff: {
        focusedPortGuid: null,
      },
    };
  },
  "semio.typeApp.setActiveTool": (context: TypeAppCommandContext, tool: ToolKind): TypeAppCommandResult => {
    return {
      diff: {
        activeTool: tool,
      },
    };
  },
  "semio.typeApp.togglePanel": (context: TypeAppCommandContext, panel: keyof PanelVisibility): TypeAppCommandResult => {
    return {
      diff: {
        panelVisibility: {
          [panel]: !context.typeApp.panelVisibility[panel],
        },
      },
    };
  },
  "semio.typeApp.deselectAll": (context: TypeAppCommandContext): TypeAppCommandResult => {
    return {
      diff: {
        selection: {
          ports: { removed: context.typeApp.selection?.ports || [] },
          models: { removed: context.typeApp.selection?.models || [] },
        },
      },
    };
  },
  "semio.typeApp.selectAll": (context: TypeAppCommandContext): TypeAppCommandResult => {
    const type = context.kit.types?.find((t) => t.guid === context.Guid);
    const allPorts = type?.ports?.map((p) => p.guid) || [];
    const allModels = type?.models?.map((r) => r.guid) || [];
    return {
      diff: {
        selection: {
          ports: { added: allPorts },
          models: { added: allModels },
        },
      },
    };
  },
  "semio.typeApp.addModelTag": (context: TypeAppCommandContext, tag: string): TypeAppCommandResult => {
    const currentTags = context.typeApp.selectedModelTags || [];
    if (currentTags.includes(tag)) {
      return {};
    }
    return {
      diff: {
        selectedModelTags: [...currentTags, tag],
      },
    };
  },
  "semio.typeApp.removeModelTag": (context: TypeAppCommandContext, tag: string): TypeAppCommandResult => {
    const currentTags = context.typeApp.selectedModelTags || [];
    return {
      diff: {
        selectedModelTags: currentTags.filter((t) => t !== tag),
      },
    };
  },
  "semio.typeApp.clearModelTags": (context: TypeAppCommandContext): TypeAppCommandResult => {
    return {
      diff: {
        selectedModelTags: [],
      },
    };
  },
  "semio.typeApp.setModelTags": (context: TypeAppCommandContext, tags: string[]): TypeAppCommandResult => {
    return {
      diff: {
        selectedModelTags: tags,
      },
    };
  },
};

// #endregion Commands

// #region Scene

/**
 * PortVisual component - renders a Port as a SceneModel.
 *
 * Implements the unified SceneModel abstraction:
 * - Hoverable: changes color on pointer enter/leave
 * - Clickable: handles selection with tool-specific behavior
 * - Focusable: sets userData.id to port.guid for camera zoom
 * - Has a semio plane: derived from Port.point and Port.direction
 *
 * Unlike Pieces which have an explicit Plane property, Ports are defined by
 * a point and direction vector. The SceneModel abstraction allows both to be
 * focusable through the unified focus behavior in Scene.tsx.
 */
const PortVisual: FC<{ port: Port; isSelected: boolean; isHovered: boolean; onHover: () => void; onLeave: () => void; onClick: () => void; onDoubleClick: () => void }> = ({ port, isSelected, isHovered, onHover, onLeave, onClick, onDoubleClick }) => {
  // Transform port position from Semio coordinate system to Three.js coordinate system
  const position = useMemo(() => {
    const semioPos = new THREE.Vector3(port.point.x, port.point.y, port.point.z);
    const threePos = semioPos.applyMatrix4(toThreeRotation());
    return [threePos.x, threePos.y, threePos.z] as [number, number, number];
  }, [port.point]);

  // Transform port direction from Semio coordinate system to Three.js coordinate system
  const direction = useMemo(() => {
    const semioDir = new THREE.Vector3(port.direction.x, port.direction.y, port.direction.z);
    const threeDir = semioDir.applyMatrix4(toThreeRotation()).normalize();
    return [threeDir.x, threeDir.y, threeDir.z] as [number, number, number];
  }, [port.direction]);

  const getComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
  const selectedColor = useMemo(() => getComputedColor("--active-base"), []);
  const hoverColor = useMemo(() => getComputedColor("--hover-base"), []);
  const defaultColor = useMemo(() => getComputedColor("--foreground"), []);

  const color = isSelected ? selectedColor : isHovered ? hoverColor : defaultColor;

  // Calculate arrow points for line
  const arrowLength = 0.5;
  const endPoint = useMemo(() => [position[0] + direction[0] * arrowLength, position[1] + direction[1] * arrowLength, position[2] + direction[2] * arrowLength] as [number, number, number], [position, direction]);
  const points = useMemo(() => [position, endPoint], [position, endPoint]);

  // userData for making the port focusable by guid
  const userData = useMemo(() => ({ id: port.guid }), [port.guid]);

  const handleClick = useCallback(
    (e: ThreeEvent<MouseEvent>) => {
      e.stopPropagation();
      onClick();
    },
    [onClick],
  );

  const handlePointerEnter = useCallback(
    (e: ThreeEvent<PointerEvent>) => {
      e.stopPropagation();
      onHover();
    },
    [onHover],
  );

  const handlePointerLeave = useCallback(
    (e: ThreeEvent<PointerEvent>) => {
      e.stopPropagation();
      onLeave();
    },
    [onLeave],
  );

  const handleDoubleClick = useCallback(
    (e: ThreeEvent<MouseEvent>) => {
      e.stopPropagation();
      onDoubleClick();
    },
    [onDoubleClick],
  );

  return (
    <Geometry hovered={isHovered} onClick={handleClick} onDoubleClick={handleDoubleClick} onPointerEnter={handlePointerEnter} onPointerLeave={handlePointerLeave} userData={userData} showEdges={false}>
      <group>
        <Sphere args={[0.03]} position={position}>
          <meshBasicMaterial color={color} />
        </Sphere>
        <Line points={points} color={color} lineWidth={2} />
        <Sphere args={[0.05]} position={endPoint}>
          <meshBasicMaterial color={color} />
        </Sphere>
      </group>
    </Geometry>
  );
};

const PortPreview: FC<{ position: THREE.Vector3; normal: THREE.Vector3 }> = ({ position, normal }) => {
  const previewColor = "#00ff00";

  // Calculate arrow points for line
  // Note: position and normal are already in Three.js coordinates from the mesh raycasting
  const arrowLength = 0.5;
  const posArray = useMemo(() => [position.x, position.y, position.z] as [number, number, number], [position]);
  const direction = useMemo(() => {
    const dir = normal.clone().normalize();
    return [dir.x, dir.y, dir.z] as [number, number, number];
  }, [normal]);
  const endPoint = useMemo(() => [posArray[0] + direction[0] * arrowLength, posArray[1] + direction[1] * arrowLength, posArray[2] + direction[2] * arrowLength] as [number, number, number], [posArray, direction]);
  const points = useMemo(() => [posArray, endPoint], [posArray, endPoint]);

  return (
    <group>
      <Sphere args={[0.03]} position={posArray}>
        <meshBasicMaterial color={previewColor} />
      </Sphere>
      <Line points={points} color={previewColor} lineWidth={2} />
    </group>
  );
};

// Separate components for each loader type to avoid conditional hook calls
const GLTFMesh: FC<{ url: string; onPointerDown: any; onPointerUp: any; onPointerMove: any; onPointerOut: any }> = ({ url, onPointerDown, onPointerUp, onPointerMove, onPointerOut }) => {
  const gltf = useGLTF(url);
  const clonedScene = useMemo(() => {
    const cloned = gltf.scene.clone();
    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
      }
    });
    return cloned;
  }, [gltf.scene]);
  return <primitive object={clonedScene} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
};

const FBXMesh: FC<{ url: string; onPointerDown: any; onPointerUp: any; onPointerMove: any; onPointerOut: any }> = ({ url, onPointerDown, onPointerUp, onPointerMove, onPointerOut }) => {
  const scene = useFBX(url);
  const clonedScene = useMemo(() => {
    const cloned = scene.clone();
    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
      }
    });
    return cloned;
  }, [scene]);
  return <primitive object={clonedScene} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
};

const OBJMesh: FC<{ url: string; onPointerDown: any; onPointerUp: any; onPointerMove: any; onPointerOut: any }> = ({ url, onPointerDown, onPointerUp, onPointerMove, onPointerOut }) => {
  const obj = useLoader(OBJLoader, url);
  const clonedScene = useMemo(() => {
    const cloned = obj.clone();
    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
      }
    });
    return cloned;
  }, [obj]);
  return <primitive object={clonedScene} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
};

const LoadedTypeMesh: FC<{
  url: string;
  fileExtension: string;
  onPointerDown: (e: ThreeEvent<PointerEvent>) => void;
  onPointerUp: (e: ThreeEvent<PointerEvent>) => void;
  onPointerMove: (e: ThreeEvent<PointerEvent>) => void;
  onPointerOut: (e: ThreeEvent<PointerEvent>) => void;
}> = ({ url, fileExtension, onPointerDown, onPointerUp, onPointerMove, onPointerOut }) => {
  const ext = fileExtension.toLowerCase();

  // Use separate components to avoid conditional hook calls
  if (ext === "glb" || ext === "gltf") {
    return <GLTFMesh url={url} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
  } else if (ext === "fbx") {
    return <FBXMesh url={url} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
  } else if (ext === "obj") {
    return <OBJMesh url={url} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
  } else {
    // Default to GLTF for unknown types
    return <GLTFMesh url={url} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
  }
};

// PERF: Stable selectors for TypeMesh - return existing references from Type
const selectTypeModels = (type: Type) => type.models;
const selectTypeConcepts = (type: Type) => type.concepts;
const selectTypeMeshGuid = (type: Type) => type.guid;

const TypeMesh: FC<{ activeTool: ToolKind; onPortPreview: (position: THREE.Vector3, normal: THREE.Vector3) => void; onPortCreate: (position: THREE.Vector3, normal: THREE.Vector3) => void; onClearPreview: () => void }> = ({
  activeTool,
  onPortPreview,
  onPortCreate,
  onClearPreview,
}) => {
  // PERF: Use targeted selectors instead of full type subscription
  // Each selector returns an existing reference from the Type object
  const typeModels = useType(selectTypeModels) as Model[] | undefined;
  const typeConcepts = useType(selectTypeConcepts) as any[] | undefined;
  const typeGuid = useType(selectTypeMeshGuid) as string | undefined;
  // Use targeted hook instead of deep kit subscription - we only need files
  const files = useKitFiles();
  const kitStore = useKitStore() as KitStore;
  const selectedModelGuid = useTypeAppSelectedModelGuid();
  const selectedModelTags = useTypeAppSelectedModelTags();
  const [isPointerDown, setIsPointerDown] = useState(false);
  const pointerDownTimeRef = useRef<number>(0);
  const pointerDownPositionRef = useRef<{ x: number; y: number }>({ x: 0, y: 0 });
  // Track previous model guid to avoid redundant logging
  const prevModelGuidRef = useRef<string | null>(null);

  const [blobUrl, setBlobUrl] = useState<string | null>(null);

  const { modelUrl, fileExtension, fileGuid, modelGuid, selectionReason } = useMemo(() => {
    if (!typeModels || typeModels.length === 0) {
      return { modelUrl: null, fileExtension: "", fileGuid: null, modelGuid: null, selectionReason: "no-models" };
    }

    let model: Model | undefined;
    let reason = "";

    if (selectedModelGuid) {
      // Use explicitly selected model GUID
      model = typeModels.find((r) => r.guid === selectedModelGuid);
      reason = "explicit-guid";
    } else if (selectedModelTags.length > 0) {
      // Use manually selected tags with strict filtering
      model = selectBestModel(typeModels, selectedModelTags);
      reason = "manual-tags";
    } else {
      // Use type's concepts as default tags for jaccard-based selection
      const conceptGuids = typeConcepts?.map((c) => c.guid) ?? [];
      if (conceptGuids.length > 0) {
        // Use findModel directly (jaccard) instead of selectBestModel (which filters first)
        // This finds the model with highest jaccard similarity to the type's concepts
        model = findModel(typeModels, conceptGuids);
        reason = "type-concepts";
      } else {
        // Fallback to default model (one with no tags) or first model
        const defaultRep = typeModels.find((r) => !r.tags || r.tags.length === 0);
        model = defaultRep ?? typeModels[0];
        reason = "default/first";
      }
    }

    if (!model) {
      return { modelUrl: null, fileExtension: "", fileGuid: null, modelGuid: null, selectionReason: "no-model-found" };
    }

    const fileId = typeof model.file === "string" ? model.file : model.file?.guid;
    const file = files.find((f) => f.guid === fileId);
    if (!file) {
      return { modelUrl: null, fileExtension: "", fileGuid: null, modelGuid: model.guid, selectionReason: "file-not-found" };
    }

    const ext = file.name?.split(".").pop() || "";

    // Try to get URL (for remote files or file provider)
    const url = kitStore.getFileUrl(file.guid);
    if (url) {
      return { modelUrl: url, fileExtension: ext, fileGuid: file.guid, modelGuid: model.guid, selectionReason: reason };
    }

    // No direct URL - will try blob URL in useEffect
    return { modelUrl: null, fileExtension: ext, fileGuid: file.guid, modelGuid: model.guid, selectionReason: reason };
  }, [typeModels, typeConcepts, files, kitStore, selectedModelGuid, selectedModelTags]);

  // PERF: Log model selection only once when the model actually changes
  useEffect(() => {
    if (modelGuid && modelGuid !== prevModelGuidRef.current) {
      prevModelGuidRef.current = modelGuid;
      console.log(`[TypeMesh] Selected ${selectionReason} model:`, modelGuid);
    } else if (!modelGuid && !typeModels) {
      console.warn("[TypeMesh] No models available for type:", typeGuid);
    } else if (!modelGuid && selectionReason === "no-model-found") {
      console.warn("[TypeMesh] No model found for type:", typeGuid);
    } else if (selectionReason === "file-not-found" && modelGuid !== prevModelGuidRef.current) {
      prevModelGuidRef.current = modelGuid;
      console.warn("[TypeMesh] File not found in kit for model:", modelGuid);
    }
  }, [modelGuid, selectionReason, typeGuid, typeModels]);

  // Convert file provider URLs to blob URLs that Three.js can load
  useEffect(() => {
    if (!fileGuid) {
      setBlobUrl(null);
      return;
    }

    let cancelled = false;
    let currentBlobUrl: string | null = null;

    (async () => {
      try {
        const url = await kitStore.getFileBlobUrl(fileGuid);
        if (!cancelled && url) {
          currentBlobUrl = url;
          setBlobUrl(url);
        } else if (!cancelled && !url) {
          // Only warn if blob URL also fails
          console.warn("[TypeMesh] No URL available for file:", fileGuid);
        }
      } catch (error) {
        console.error("[TypeMesh] Failed to get blob URL:", error);
      }
    })();

    // Cleanup on unmount or when fileGuid changes
    return () => {
      cancelled = true;
      if (currentBlobUrl && currentBlobUrl.startsWith("blob:")) {
        URL.revokeObjectURL(currentBlobUrl);
      }
    };
  }, [fileGuid, kitStore]);

  const handlePointerDown = useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (activeTool === ToolKind.PORT) {
        setIsPointerDown(true);
        pointerDownTimeRef.current = Date.now();
        pointerDownPositionRef.current = { x: event.clientX, y: event.clientY };
      }
    },
    [activeTool],
  );

  const handlePointerUp = useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (activeTool === ToolKind.PORT && isPointerDown) {
        const timeDiff = Date.now() - pointerDownTimeRef.current;
        const distance = Math.sqrt(Math.pow(event.clientX - pointerDownPositionRef.current.x, 2) + Math.pow(event.clientY - pointerDownPositionRef.current.y, 2));

        if (timeDiff < 300 && distance < 5 && event.face) {
          event.stopPropagation();
          const position = new THREE.Vector3().copy(event.point);
          const normal = event.face.normal.clone();
          const normalMatrix = new THREE.Matrix3().getNormalMatrix((event.object as THREE.Mesh).matrixWorld);
          normal.applyMatrix3(normalMatrix).normalize();
          onPortCreate(position, normal);
        }
        setIsPointerDown(false);
      }
    },
    [activeTool, isPointerDown, onPortCreate],
  );

  const handlePointerMove = useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (activeTool === ToolKind.PORT && event.face && !isPointerDown) {
        event.stopPropagation();
        const position = new THREE.Vector3().copy(event.point);
        const normal = event.face.normal.clone();
        const normalMatrix = new THREE.Matrix3().getNormalMatrix((event.object as THREE.Mesh).matrixWorld);
        normal.applyMatrix3(normalMatrix).normalize();
        onPortPreview(position, normal);
      }
    },
    [activeTool, isPointerDown, onPortPreview],
  );

  const handlePointerOut = useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (activeTool === ToolKind.PORT) {
        onClearPreview();
        setIsPointerDown(false);
      }
    },
    [activeTool, onClearPreview],
  );

  const getComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
  const foregroundColor = useMemo(() => getComputedColor("--foreground"), []);

  if (!blobUrl) {
    // Render placeholder box when no model is loaded (same as design pieces)
    return (
      <mesh>
        <boxGeometry args={[1, 1, 1]} />
        <meshStandardMaterial color={foregroundColor} emissive={foregroundColor} emissiveIntensity={0.45} />
        <Edges scale={1.001} color={foregroundColor} />
      </mesh>
    );
  }

  return (
    <Suspense fallback={null}>
      <LoadedTypeMesh url={blobUrl} fileExtension={fileExtension} onPointerDown={handlePointerDown} onPointerUp={handlePointerUp} onPointerMove={handlePointerMove} onPointerOut={handlePointerOut} />
    </Suspense>
  );
};

// PERF: Stable selectors for SceneContent - only fetch the specific fields needed
// These return existing array/object references from the Type, not new objects
const selectTypePorts = (type: Type) => type.ports;
const selectTypeGuid = (type: Type) => type.guid;

// PERF: SceneContent is memoized to prevent re-renders when Scene re-renders due to camera changes
const SceneContent: FC = React.memo(() => {
  const activeTool = useTypeAppActiveTool();
  // PERF: Use targeted selectors instead of fetching full type/kit
  // Only subscribe to the specific fields we actually need
  const typePorts = useType(selectTypePorts) as Port[] | undefined;
  const typeGuid = useType(selectTypeGuid) as string | undefined;
  // PERF: useKit was only used to check existence - kitCommands being non-null serves same purpose
  const kitCommands = useKitCommands();
  const selection = useTypeAppSelection();
  const hover = useTypeAppHover();
  // PERF: Removed useTypeApp((s) => s) - was causing full re-renders on every state change
  // Tools (SelectionNormalTool, PortTool, etc.) return null/empty scene content anyway
  const { selectPort, deselectPort, hoverPort, clearHover, focusPort } = useTypeAppCommands();
  const [portPreview, setPortPreview] = useState<{ position: THREE.Vector3; normal: THREE.Vector3 } | null>(null);
  const focusContext = useFocusSafe();
  const prevItemsRef = useRef<string>("");

  // Set focus items for navbar
  useEffect(() => {
    if (!focusContext || !typePorts) return;
    const items = typePorts.map((port) => ({
      id: port.guid,
      label: port.description || `Port ${port.guid.substring(0, 8)}`,
      category: "Ports",
    }));
    const itemsKey = items.map((item) => `${item.id}:${item.label}`).join("|");
    if (prevItemsRef.current !== itemsKey) {
      prevItemsRef.current = itemsKey;
      focusContext.setFocusItems(items);
    }
  }, [focusContext, typePorts]);

  // Register focus handler for navbar focus
  useEffect(() => {
    if (!focusContext) return;
    const handleFocus = (itemId: string) => {
      focusPort(itemId, "");
    };
    focusContext.setOnFocusItem(handleFocus);
    return () => {
      if (focusContext) focusContext.setOnFocusItem(undefined);
    };
  }, [focusContext, focusPort]);

  // PERF: Removed tool contribution computation - tools return null/empty scene content
  // The actual tool behavior is handled by activeTool-specific logic in TypeMesh and SceneContent

  const handlePortPreview = useCallback((position: THREE.Vector3, normal: THREE.Vector3) => {
    setPortPreview({ position, normal });
  }, []);

  const handlePortCreate = useCallback(
    (position: THREE.Vector3, normal: THREE.Vector3) => {
      // PERF: Check typeGuid and kitCommands instead of full type/kit objects
      if (typeGuid && kitCommands) {
        // Convert position and normal from Three.js coordinate system back to Semio coordinate system
        const semioPosition = position.clone().applyMatrix4(toSemioRotation());
        const semioNormal = normal.clone().applyMatrix4(toSemioRotation()).normalize();

        const newPort: Port = {
          guid: guid(),
          point: {
            x: semioPosition.x,
            y: semioPosition.y,
            z: semioPosition.z,
          } as Point,
          direction: {
            x: semioNormal.x,
            y: semioNormal.y,
            z: semioNormal.z,
          } as Vector,
          t: 0,
          mandatory: false,
        };

        kitCommands.updateType("semio.sketchpad.app.type.canvas.scene.addPort", typeGuid, {
          ports: {
            added: [newPort],
          },
        });
      }
    },
    [typeGuid, kitCommands],
  );

  const handleClearPreview = useCallback(() => {
    setPortPreview(null);
    clearHover("");
  }, [clearHover]);

  const handlePortClick = useCallback(
    (portId: string) => {
      const isSelected = selection?.ports?.includes(portId) || false;
      if (activeTool === ToolKind.SELECTION_ADDITIVE) {
        if (!isSelected) selectPort(portId, "");
      } else if (activeTool === ToolKind.SELECTION_SUBTRACTIVE) {
        if (isSelected) deselectPort(portId);
      } else {
        const currentPorts = selection?.ports ?? [];
        if (currentPorts.length > 0) {
          currentPorts.forEach((id) => deselectPort(id));
        }
        if (!isSelected || currentPorts.length > 1) {
          selectPort(portId, "");
        }
      }
    },
    [selection, selectPort, deselectPort, activeTool],
  );

  const handlePortHover = useCallback(
    (portId: string) => {
      hoverPort(portId, "");
    },
    [hoverPort],
  );

  const handlePortLeave = useCallback(() => {
    clearHover("");
  }, [clearHover]);

  const handlePortDoubleClick = useCallback(
    (portId: string) => {
      focusPort(portId, "");
    },
    [focusPort],
  );

  return (
    <>
      <TypeMesh activeTool={activeTool} onPortPreview={handlePortPreview} onPortCreate={handlePortCreate} onClearPreview={handleClearPreview} />
      {typePorts?.map((port) => {
        const isSelected = selection?.ports?.includes(port.guid) || false;
        const isHovered = hover?.port === port.guid;
        return (
          <PortVisual
            key={port.guid}
            port={port}
            isSelected={isSelected}
            isHovered={isHovered}
            onHover={() => handlePortHover(port.guid)}
            onLeave={handlePortLeave}
            onClick={() => handlePortClick(port.guid)}
            onDoubleClick={() => handlePortDoubleClick(port.guid)}
          />
        );
      })}
      {portPreview && <PortPreview position={portPreview.position} normal={portPreview.normal} />}
    </>
  );
});

const Scene: FC<{ isDragOver?: boolean }> = ({ isDragOver = false }) => {
  const { setCamera, deselectAll, clearFocus } = useTypeAppCommands();
  const camera = useTypeAppCamera();
  const focusedPortGuid = useTypeAppFocusedPortGuid();

  const onCameraChange = useCallback(
    (newCamera: Camera) => {
      setCamera("", newCamera);
    },
    [setCamera],
  );

  const onPointerMissed = useCallback(
    (event: MouseEvent) => {
      if (!(event.ctrlKey || event.metaKey) && !event.shiftKey) deselectAll("");
    },
    [deselectAll],
  );

  const onFocusComplete = useCallback(() => {
    clearFocus("");
  }, [clearFocus]);

  return (
    <SceneComponent camera={camera} onCameraChange={onCameraChange} onPointerMissed={onPointerMissed} focusedItemId={focusedPortGuid} onFocusComplete={onFocusComplete}>
      <SceneContent />
      {isDragOver && (
        <mesh position={[0, 0, 0]}>
          <planeGeometry args={[100, 100]} />
          <meshBasicMaterial color="#4f46e5" opacity={0.2} transparent />
        </mesh>
      )}
    </SceneComponent>
  );
};

// #endregion Scene

// #region Node

// #endregion Node

// #endregion Windows

// #region Panels

// #region Left

// #region Workbench

// #endregion Workbench

// #endregion Left

// #region Middle

// #region Hud

// #endregion Hud

// #region Stats

// #endregion Stats

// #endregion Middle

// #region Right

// #region Details

export const TypeDetails: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <TypeDetailsForm />;
};

const TypeDetailsForm: FC = () => {
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;

  const updateTypeField = (origin: string, diff: any) => {
    kitCommands?.updateType(origin, type.guid, diff);
  };

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input lazy id="semio.sketchpad.app.type.panel.details.section.type.name" value={type.name} onLazyChange={(value) => updateTypeField("semio.sketchpad.app.type.panel.details.section.type.name", { name: value })} showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea
            lazy
            id="semio.sketchpad.app.type.panel.details.section.type.description"
            value={type.description || ""}
            placeholderId="semio.sketchpad.app.type.descriptionPlaceholder.label"
            onLazyChange={(value) => updateTypeField("semio.sketchpad.app.type.panel.details.section.type.description", { description: value })}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.type.panel.details.section.type.icon"
            value={type.icon || ""}
            placeholderId="semio.sketchpad.app.type.iconPlaceholder.label"
            onLazyChange={(value) => updateTypeField("semio.sketchpad.app.type.panel.details.section.type.icon", { icon: value })}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.type.panel.details.section.type.image"
            value={type.image || ""}
            placeholderId="semio.sketchpad.app.type.imagePlaceholder.label"
            onLazyChange={(value) => updateTypeField("semio.sketchpad.app.type.panel.details.section.type.image", { image: value })}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.type.panel.details.section.type.parent"
            value={type.parent || ""}
            placeholderId="semio.sketchpad.app.type.parentPlaceholder.label"
            onLazyChange={(value) => updateTypeField("semio.sketchpad.app.type.panel.details.section.type.parent", { parent: value })}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Toggle
            id="semio.sketchpad.app.type.panel.details.section.type.abstract"
            pressed={type.isAbstract || false}
            onPressedChange={(value) => updateTypeField("semio.sketchpad.app.type.panel.details.section.type.abstract", { isAbstract: value })}
            showLabel
            icon={<CheckIcon />}
          />
        </TreeContent>
      </TreeItem>
      {type.unit !== undefined && (
        <TreeItem>
          <TreeContent>
            <Input lazy id="semio.sketchpad.app.type.panel.details.section.type.unit" value={type.unit} onLazyChange={(value) => updateTypeField("semio.sketchpad.app.type.panel.details.section.type.unit", { unit: value })} showLabel />
          </TreeContent>
        </TreeItem>
      )}
    </>
  );
};

export const ModelsSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <ModelsSectionForm />;
};

const ModelsSectionForm: FC = () => {
  const tooltip = useTooltip();
  const { selectModel, deselectModel, hoverModel, clearHover } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;
  const selection = useTypeAppSelection();
  const hover = useTypeAppHover();

  const applyDiff = (origin: string, diff: any) => {
    kitCommands?.updateType(origin, type.guid, diff);
  };

  const updateModel = (origin: string, id: string, modelDiff: any) => {
    applyDiff(origin, {
      models: {
        updated: [{ id, diff: modelDiff }],
      },
    });
  };

  const hasModels = type.models && type.models.length > 0;

  return (
    <>
      <TreeItem
        id="semio.sketchpad.app.type.models"
        actions={[
          {
            icon: <AddIcon />,
            onClick: () => {
              const origin = "semio.sketchpad.app.type.panel.details.models.add";
              applyDiff(origin, {
                models: {
                  added: [{ guid: guid(), url: "", tags: [] }],
                },
              });
            },
            id: "semio.sketchpad.common.add",
          },
        ]}
      >
        {hasModels && (
          <SortableTreeItems
            items={(type.models || []).map((model: any, index: number) => ({
              ...model,
              id: `model-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              if (!type.models) return;
              const origin = "semio.sketchpad.app.type.panel.details.models.reorder";
              applyDiff(origin, {
                models: {
                  removed: type.models.map((model: any) => model.guid),
                  added: arrayMove(type.models, oldIndex, newIndex),
                },
              });
            }}
          >
            {(model, index) => {
              const isSelected = selection?.models?.includes(model.guid) || false;
              const isHovered = hover?.model === model.guid;
              return (
                <div
                  key={`model-${index}`}
                  onPointerEnter={() => hoverModel("semio.sketchpad.app.type.panel.details.model.hover", model.guid)}
                  onPointerLeave={() => clearHover("semio.sketchpad.app.type.panel.details.model.leave")}
                  onClick={() => (isSelected ? deselectModel("semio.sketchpad.app.type.panel.details.model.deselect", model.guid) : selectModel("semio.sketchpad.app.type.panel.details.model.select", model.guid))}
                >
                  <TreeItem
                    key={`model-${index}`}
                    id="semio.sketchpad.app.type.model"
                    label={model.url}
                    sortable={true}
                    sortableId={`model-${index}`}
                    isDragHandle={true}
                    className={`${isSelected ? "bg-accent/20" : ""} ${isHovered ? "bg-hover" : ""}`}
                    actions={[
                      {
                        icon: <RemoveIcon />,
                        onClick: () => {
                          const origin = "semio.sketchpad.app.type.panel.details.models.remove";
                          applyDiff(origin, {
                            models: {
                              removed: [model.guid],
                            },
                          });
                        },
                        id: "semio.sketchpad.common.remove",
                      },
                    ]}
                  >
                    <TreeItem>
                      <TreeContent>
                        <Input
                          id="semio.sketchpad.app.type.panel.details.section.models.url"
                          value={model.url}
                          onChange={(e) => {
                            updateModel("semio.sketchpad.app.type.panel.details.section.models.url", model.guid, { url: e.target.value });
                          }}
                          showLabel
                        />
                      </TreeContent>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <Textarea
                          id="semio.sketchpad.app.type.panel.details.section.models.description"
                          value={model.description || ""}
                          placeholderId="semio.sketchpad.app.type.modelDescriptionPlaceholder.label"
                          onChange={(e) => {
                            updateModel("semio.sketchpad.app.type.panel.details.section.models.description", model.guid, { description: e.target.value });
                          }}
                          showLabel
                        />
                      </TreeContent>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <Input
                          id="semio.sketchpad.app.type.panel.details.section.models.tags"
                          value={(model.tags || []).join(", ")}
                          placeholderId="semio.sketchpad.app.type.modelTagsPlaceholder.label"
                          onChange={(e) => {
                            updateModel("semio.sketchpad.app.type.panel.details.section.models.tags", model.guid, {
                              tags: e.target.value
                                .split(",")
                                .map((tag) => tag.trim())
                                .filter((tag) => tag),
                            });
                          }}
                          showLabel
                        />
                      </TreeContent>
                    </TreeItem>
                  </TreeItem>
                </div>
              );
            }}
          </SortableTreeItems>
        )}
      </TreeItem>
    </>
  );
};

export const PortsListSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <PortsListSectionForm />;
};

const PortsListSectionForm: FC = () => {
  const tooltip = useTooltip();
  const { selectPort, deselectPort, hoverPort, clearHover } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;
  const selection = useTypeAppSelection();
  const hover = useTypeAppHover();

  const applyDiff = (origin: string, diff: any) => {
    kitCommands?.updateType(origin, type.guid, diff);
  };

  const updatePort = (origin: string, id: string, portDiff: any) => {
    const port = type.ports?.find((existingPort) => existingPort.guid === id);
    const diff: any = { ...portDiff };
    if (port) {
      if (portDiff.point) {
        diff.point = {};
        if (portDiff.point.x !== undefined) diff.point.x = portDiff.point.x - port.point.x;
        if (portDiff.point.y !== undefined) diff.point.y = portDiff.point.y - port.point.y;
        if (portDiff.point.z !== undefined) diff.point.z = portDiff.point.z - port.point.z;
      }
      if (portDiff.direction) {
        diff.direction = {};
        if (portDiff.direction.x !== undefined) diff.direction.x = portDiff.direction.x - port.direction.x;
        if (portDiff.direction.y !== undefined) diff.direction.y = portDiff.direction.y - port.direction.y;
        if (portDiff.direction.z !== undefined) diff.direction.z = portDiff.direction.z - port.direction.z;
      }
    }
    applyDiff(origin, {
      ports: {
        updated: [{ id, diff }],
      },
    });
  };

  const hasPorts = type.ports && type.ports.length > 0;

  return (
    <>
      <TreeItem
        id="semio.sketchpad.app.type.ports"
        actions={[
          {
            icon: <AddIcon />,
            onClick: () => {
              const origin = "semio.sketchpad.app.type.panel.details.ports.add";
              applyDiff(origin, {
                ports: {
                  added: [
                    {
                      guid: guid(),
                      t: 0,
                      point: { x: 0, y: 0, z: 0 },
                      direction: { x: 0, y: 0, z: 1 },
                    },
                  ],
                },
              });
            },
            id: "semio.sketchpad.common.add",
          },
        ]}
      >
        {hasPorts && (
          <SortableTreeItems
            items={(type.ports || []).map((port: any, index: number) => ({
              ...port,
              id: `port-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              if (!type.ports) return;
              const origin = "semio.sketchpad.app.type.panel.details.ports.reorder";
              applyDiff(origin, {
                ports: {
                  removed: type.ports.map((existingPort: any) => existingPort.guid),
                  added: arrayMove(type.ports, oldIndex, newIndex),
                },
              });
            }}
          >
            {(port, index) => {
              const isSelected = selection?.ports?.includes(port.guid) || false;
              const isHovered = hover?.port === port.guid;
              const handleClick = (event: React.MouseEvent) => {
                event.stopPropagation();
                if (isSelected) {
                  deselectPort("semio.sketchpad.app.type.panel.details.section.ports.deselect", port.guid);
                } else {
                  selectPort("semio.sketchpad.app.type.panel.details.section.ports.select", port.guid);
                }
              };

              const handleHover = () => {
                hoverPort("semio.sketchpad.app.type.panel.details.section.ports.hover", port.guid);
              };

              const handleLeave = () => {
                clearHover("semio.sketchpad.app.type.panel.details.section.ports.leave");
              };

              return (
                <div onPointerEnter={handleHover} onPointerLeave={handleLeave} onClick={handleClick}>
                  <TreeItem
                    key={`port-${index}`}
                    id="semio.sketchpad.app.type.port"
                    label={port.interface}
                    sortable={true}
                    sortableId={`port-${index}`}
                    isDragHandle={true}
                    className={`cursor-selectable ${isSelected ? "ring-1 ring-[color:var(--active-base)]" : ""} ${isHovered ? "bg-[color:var(--hover-base)]" : ""}`}
                    actions={[
                      {
                        icon: <RemoveIcon />,
                        onClick: () => {
                          const origin = "semio.sketchpad.app.type.panel.details.ports.remove";
                          applyDiff(origin, {
                            ports: {
                              removed: [port.guid],
                            },
                          });
                        },
                        id: "semio.sketchpad.common.remove",
                      },
                    ]}
                  >
                    <TreeItem>
                      <TreeContent>
                        <Input
                          lazy
                          id="semio.sketchpad.app.type.panel.details.section.ports.interface"
                          value={port.interface || ""}
                          placeholderId="semio.sketchpad.app.type.portInterfacePlaceholder.label"
                          onLazyChange={(value: string) => {
                            updatePort("semio.sketchpad.app.type.panel.details.section.ports.interface", port.guid, { interface: value });
                          }}
                          showLabel
                        />
                      </TreeContent>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <Textarea
                          lazy
                          id="semio.sketchpad.app.type.panel.details.section.ports.description"
                          value={port.description || ""}
                          placeholderId="semio.sketchpad.app.type.portDescriptionPlaceholder.label"
                          onLazyChange={(value: string) => {
                            updatePort("semio.sketchpad.app.type.panel.details.section.ports.description", port.guid, { description: value });
                          }}
                          showLabel
                        />
                      </TreeContent>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <Slider
                          id="semio.sketchpad.app.type.panel.details.section.ports.t"
                          value={[port.t ?? 0]}
                          onValueChange={([value]) => {
                            updatePort("semio.sketchpad.app.type.panel.details.section.ports.t", port.guid, { t: value });
                          }}
                          transaction={useKitTransaction("semio.sketchpad.app.type.panel.details.section.ports.t")}
                          min={0}
                          max={1}
                          step={0.01}
                          showLabel
                        />
                      </TreeContent>
                    </TreeItem>
                    <TreeItem id="semio.sketchpad.app.type.portPoint">
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            id="semio.sketchpad.app.type.panel.details.section.ports.point.x"
                            value={port.point.x}
                            onChange={(value: number) => {
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.x", port.guid, { point: { x: value } });
                            }}
                            transaction={useKitTransaction("semio.sketchpad.app.type.panel.details.section.ports.point.x")}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            id="semio.sketchpad.app.type.panel.details.section.ports.point.y"
                            value={port.point.y}
                            onChange={(value: number) => {
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.y", port.guid, { point: { y: value } });
                            }}
                            transaction={useKitTransaction("semio.sketchpad.app.type.panel.details.section.ports.point.y")}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            id="semio.sketchpad.app.type.panel.details.section.ports.point.z"
                            value={port.point.z}
                            onChange={(value: number) => {
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.z", port.guid, { point: { z: value } });
                            }}
                            transaction={useKitTransaction("semio.sketchpad.app.type.panel.details.section.ports.point.z")}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                    </TreeItem>
                    <TreeItem id="semio.sketchpad.app.type.portDirection">
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            id="semio.sketchpad.app.type.panel.details.section.ports.direction.x"
                            value={port.direction.x}
                            onChange={(value: number) => {
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.x", port.guid, { direction: { x: value } });
                            }}
                            transaction={useKitTransaction("semio.sketchpad.app.type.panel.details.section.ports.direction.x")}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            id="semio.sketchpad.app.type.panel.details.section.ports.direction.y"
                            value={port.direction.y}
                            onChange={(value: number) => {
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.y", port.guid, { direction: { y: value } });
                            }}
                            transaction={useKitTransaction("semio.sketchpad.app.type.panel.details.section.ports.direction.y")}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                      <TreeItem>
                        <TreeContent>
                          <Stepper
                            id="semio.sketchpad.app.type.panel.details.section.ports.direction.z"
                            value={port.direction.z}
                            onChange={(value: number) => {
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.z", port.guid, { direction: { z: value } });
                            }}
                            transaction={useKitTransaction("semio.sketchpad.app.type.panel.details.section.ports.direction.z")}
                            step={0.1}
                          />
                        </TreeContent>
                      </TreeItem>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <Input
                          lazy
                          id="semio.sketchpad.app.type.panel.details.section.ports.compatibleInterfaces"
                          value={(port.compatibleInterfaces || []).join(", ")}
                          placeholderId="semio.sketchpad.app.type.portCompatibleInterfacesPlaceholder.label"
                          onLazyChange={(value: string) => {
                            updatePort("semio.sketchpad.app.type.panel.details.section.ports.compatibleInterfaces", port.guid, {
                              compatibleInterfaces: value
                                .split(",")
                                .map((interface_) => interface_.trim())
                                .filter((interface_) => interface_),
                            });
                          }}
                          showLabel
                        />
                      </TreeContent>
                    </TreeItem>
                  </TreeItem>
                </div>
              );
            }}
          </SortableTreeItems>
        )}
      </TreeItem>
    </>
  );
};

export const AuthorsSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <AuthorsSectionForm />;
};

const AuthorsSectionForm: FC = () => {
  const tooltip = useTooltip();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;
  const kit = useKit() as Kit;

  const updateAuthors = (origin: string, authors: string[]) => {
    kitCommands?.updateType(origin, type.guid, { authors: authors.map((a) => ({ guid: a })) });
  };

  const hasAuthors = type.authors && type.authors.length > 0;

  return (
    <>
      <TreeItem
        id="semio.sketchpad.app.type.authors"
        actions={[
          {
            icon: <AddIcon />,
            onClick: () => {
              const origin = "semio.sketchpad.app.type.panel.details.authors.add";
              const newAuthorGuid = guid();
              kitCommands?.createAuthor(origin, {
                guid: newAuthorGuid,
                name: "",
                email: "",
              });
              updateAuthors(origin, [...(type.authors || []).map((a) => a.guid), newAuthorGuid]);
            },
            id: "semio.sketchpad.common.add",
          },
        ]}
      >
        {hasAuthors && (
          <SortableTreeItems
            items={(type.authors || []).map((authorId: AuthorId, index: number) => {
              const author = kit.authors?.find((a: Author) => a.guid === authorId.guid);
              return {
                id: `author-${index}`,
                index,
                guid: authorId.guid,
                name: author?.name || "",
                email: author?.email || "",
              };
            })}
            onReorder={(oldIndex, newIndex) => {
              const origin = "semio.sketchpad.app.type.panel.details.authors.reorder";
              updateAuthors(
                origin,
                arrayMove(type.authors!, oldIndex, newIndex).map((a) => a.guid),
              );
            }}
          >
            {(item, index) => (
              <TreeItem
                key={`author-${index}`}
                id="semio.sketchpad.app.type.author"
                label={item.name}
                sortable={true}
                sortableId={`author-${index}`}
                isDragHandle={true}
                actions={[
                  {
                    icon: <RemoveIcon />,
                    onClick: () => {
                      const origin = "semio.sketchpad.app.type.panel.details.authors.remove";
                      updateAuthors(
                        origin,
                        (type.authors || []).filter((_, i: number) => i !== index).map((a) => a.guid),
                      );
                    },
                    id: "semio.sketchpad.common.remove",
                  },
                ]}
              >
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.type.panel.details.section.authors.name"
                      value={item.name}
                      onChange={(e) => {
                        kitCommands?.updateAuthor("semio.sketchpad.app.type.panel.details.section.authors.name", item.guid, { name: e.target.value });
                      }}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.type.panel.details.section.authors.email"
                      value={item.email}
                      onChange={(e) => {
                        kitCommands?.updateAuthor("semio.sketchpad.app.type.panel.details.section.authors.email", item.guid, { email: e.target.value });
                      }}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
              </TreeItem>
            )}
          </SortableTreeItems>
        )}
      </TreeItem>
    </>
  );
};

export const AttributesSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <AttributesSectionForm />;
};

const AttributesSectionForm: FC = () => {
  const tooltip = useTooltip();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;

  const applyDiff = (origin: string, diff: any) => {
    kitCommands?.updateType(origin, type.guid, diff);
  };

  const updateAttribute = (origin: string, id: string, attributeDiff: any) => {
    applyDiff(origin, {
      attributes: {
        updated: [{ id, diff: attributeDiff }],
      },
    });
  };

  const hasAttributes = type.attributes && type.attributes.length > 0;

  return (
    <>
      <TreeItem
        id="semio.sketchpad.app.type.attributes"
        actions={[
          {
            icon: <AddIcon />,
            onClick: () => {
              const origin = "semio.sketchpad.app.type.panel.details.attributes.add";
              applyDiff(origin, {
                attributes: {
                  added: [{ guid: guid(), key: "" }],
                },
              });
            },
            id: "semio.sketchpad.common.add",
          },
        ]}
      >
        {hasAttributes && (
          <SortableTreeItems
            items={(type.attributes || []).map((attribute: any, index: number) => ({
              ...attribute,
              id: `attribute-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              if (!type.attributes) return;
              const origin = "semio.sketchpad.app.type.panel.details.attributes.reorder";
              applyDiff(origin, {
                attributes: {
                  removed: type.attributes.map((attribute: any) => attribute.guid),
                  added: arrayMove(type.attributes, oldIndex, newIndex),
                },
              });
            }}
          >
            {(attribute, index) => (
              <TreeItem
                key={`attribute-${index}`}
                id="semio.sketchpad.app.type.attribute"
                label={attribute.key}
                sortable={true}
                sortableId={`attribute-${index}`}
                isDragHandle={true}
                actions={[
                  {
                    icon: <RemoveIcon />,
                    onClick: () => {
                      const origin = "semio.sketchpad.app.type.panel.details.attributes.remove";
                      applyDiff(origin, {
                        attributes: {
                          removed: [attribute.guid],
                        },
                      });
                    },
                    id: "semio.sketchpad.common.remove",
                  },
                ]}
              >
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.type.panel.details.section.attributes.name"
                      value={attribute.key}
                      onChange={(e) => {
                        updateAttribute("semio.sketchpad.app.type.panel.details.section.attributes.name", attribute.guid, { key: e.target.value });
                      }}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.type.panel.details.section.attributes.value"
                      value={attribute.value || ""}
                      placeholderId="semio.sketchpad.app.type.attributeValuePlaceholder.label"
                      onChange={(e) => {
                        updateAttribute("semio.sketchpad.app.type.panel.details.section.attributes.value", attribute.guid, { value: e.target.value });
                      }}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      id="semio.sketchpad.app.type.panel.details.section.attributes.definition"
                      value={attribute.definition || ""}
                      placeholderId="semio.sketchpad.app.type.attributeDefinitionPlaceholder.label"
                      onChange={(e) => {
                        updateAttribute("semio.sketchpad.app.type.panel.details.section.attributes.definition", attribute.guid, { definition: e.target.value });
                      }}
                      showLabel
                    />
                  </TreeContent>
                </TreeItem>
              </TreeItem>
            )}
          </SortableTreeItems>
        )}
      </TreeItem>
    </>
  );
};

export const PortSection: FC<{ portGuid: Guid }> = ({ portGuid }) => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <PortSectionForm portGuid={portGuid} />;
};

const PortSectionForm: FC<{ portGuid: Guid }> = ({ portGuid }) => {
  const tooltip = useTooltip();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;

  const port = type.ports?.find((p) => p.guid === portGuid);

  if (!port) {
    return (
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.type.portNotFound")}</p>
        </TreeContent>
      </TreeItem>
    );
  }

  const updatePort = (origin: string, id: string, portDiff: any) => {
    const port = type.ports?.find((existingPort) => existingPort.guid === id);
    const diff: any = { ...portDiff };
    if (port) {
      if (portDiff.point) {
        diff.point = {};
        if (portDiff.point.x !== undefined) diff.point.x = portDiff.point.x - port.point.x;
        if (portDiff.point.y !== undefined) diff.point.y = portDiff.point.y - port.point.y;
        if (portDiff.point.z !== undefined) diff.point.z = portDiff.point.z - port.point.z;
      }
      if (portDiff.direction) {
        diff.direction = {};
        if (portDiff.direction.x !== undefined) diff.direction.x = portDiff.direction.x - port.direction.x;
        if (portDiff.direction.y !== undefined) diff.direction.y = portDiff.direction.y - port.direction.y;
        if (portDiff.direction.z !== undefined) diff.direction.z = portDiff.direction.z - port.direction.z;
      }
    }
    kitCommands?.updateType(origin, type.guid, {
      ports: {
        updated: [{ id, diff }],
      },
    });
  };

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.type.panel.details.section.ports.interface"
            value={port.interface || ""}
            placeholderId="semio.sketchpad.app.type.portInterfacePlaceholder.label"
            onLazyChange={(value: string) => {
              updatePort("semio.sketchpad.app.type.panel.details.section.ports.interface", port.guid, { interface: value });
            }}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea
            lazy
            id="semio.sketchpad.app.type.panel.details.section.ports.description"
            value={port.description || ""}
            placeholderId="semio.sketchpad.app.type.portDescriptionPlaceholder.label"
            onLazyChange={(value: string) => {
              updatePort("semio.sketchpad.app.type.panel.details.section.ports.description", port.guid, { description: value });
            }}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Slider
            id="semio.sketchpad.app.type.panel.details.section.ports.t"
            value={[port.t ?? 0]}
            onValueChange={([value]) => {
              updatePort("semio.sketchpad.app.type.panel.details.section.ports.t", port.guid, { t: value });
            }}
            transaction={useKitTransaction("semio.sketchpad.app.type.panel.details.section.ports.t")}
            min={0}
            max={1}
            step={0.01}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.type.portPoint">
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.point.x"
              value={port.point.x}
              onChange={(value: number) => {
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.x", port.guid, { point: { x: value } });
              }}
              transaction={useKitTransaction("semio.sketchpad.app.type.panel.details.section.ports.point.x")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.point.y"
              value={port.point.y}
              onChange={(value: number) => {
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.y", port.guid, { point: { y: value } });
              }}
              transaction={useKitTransaction("semio.sketchpad.app.type.panel.details.section.ports.point.y")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.point.z"
              value={port.point.z}
              onChange={(value: number) => {
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.z", port.guid, { point: { z: value } });
              }}
              transaction={useKitTransaction("semio.sketchpad.app.type.panel.details.section.ports.point.z")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.type.portDirection">
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.direction.x"
              value={port.direction.x}
              onChange={(value: number) => {
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.x", port.guid, { direction: { x: value } });
              }}
              transaction={useKitTransaction("semio.sketchpad.app.type.panel.details.section.ports.direction.x")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.direction.y"
              value={port.direction.y}
              onChange={(value: number) => {
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.y", port.guid, { direction: { y: value } });
              }}
              transaction={useKitTransaction("semio.sketchpad.app.type.panel.details.section.ports.direction.y")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.direction.z"
              value={port.direction.z}
              onChange={(value: number) => {
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.z", port.guid, { direction: { z: value } });
              }}
              transaction={useKitTransaction("semio.sketchpad.app.type.panel.details.section.ports.direction.z")}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.type.panel.details.section.ports.compatibleInterfaces"
            value={(port.compatibleInterfaces || []).join(", ")}
            placeholderId="semio.sketchpad.app.type.portCompatibleInterfacesPlaceholder.label"
            onLazyChange={(value: string) => {
              updatePort("semio.sketchpad.app.type.panel.details.section.ports.compatibleInterfaces", port.guid, {
                compatibleInterfaces: value
                  .split(",")
                  .map((interface_) => interface_.trim())
                  .filter((interface_) => interface_),
              });
            }}
            showLabel
          />
        </TreeContent>
      </TreeItem>
    </>
  );
};

export const PortsMultipleSection: FC<{ portGuids: Guid[] }> = ({ portGuids }) => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <PortsMultipleSectionForm portGuids={portGuids} />;
};

const PortsMultipleSectionForm: FC<{ portGuids: Guid[] }> = ({ portGuids }) => {
  const tooltip = useTooltip();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;

  const ports = type.ports?.filter((p) => portGuids.includes(p.guid)) || [];

  if (ports.length === 0) {
    return (
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.type.portsNotFound")}</p>
        </TreeContent>
      </TreeItem>
    );
  }

  const getCommonValue = <T,>(getter: (port: any) => T | undefined): T | undefined => {
    const values = ports.map(getter).filter((v) => v !== undefined);
    if (values.length === 0) return undefined;
    const firstValue = values[0];
    return values.every((v) => JSON.stringify(v) === JSON.stringify(firstValue)) ? firstValue : undefined;
  };

  const updatePorts = (origin: string, portDiff: any) => {
    ports.forEach((port) => {
      const diff: any = { ...portDiff };
      if (portDiff.point) {
        diff.point = {};
        if (portDiff.point.x !== undefined) diff.point.x = portDiff.point.x - port.point.x;
        if (portDiff.point.y !== undefined) diff.point.y = portDiff.point.y - port.point.y;
        if (portDiff.point.z !== undefined) diff.point.z = portDiff.point.z - port.point.z;
      }
      if (portDiff.direction) {
        diff.direction = {};
        if (portDiff.direction.x !== undefined) diff.direction.x = portDiff.direction.x - port.direction.x;
        if (portDiff.direction.y !== undefined) diff.direction.y = portDiff.direction.y - port.direction.y;
        if (portDiff.direction.z !== undefined) diff.direction.z = portDiff.direction.z - port.direction.z;
      }
      kitCommands?.updateType(origin, type.guid, {
        ports: {
          updated: [{ id: port.guid, diff }],
        },
      });
    });
  };

  const commonInterface = getCommonValue((p) => p.interface);
  const commonT = getCommonValue((p) => p.t);
  const commonPointX = getCommonValue((p) => p.point?.x);
  const commonPointY = getCommonValue((p) => p.point?.y);
  const commonPointZ = getCommonValue((p) => p.point?.z);
  const commonDirectionX = getCommonValue((p) => p.direction?.x);
  const commonDirectionY = getCommonValue((p) => p.direction?.y);
  const commonDirectionZ = getCommonValue((p) => p.direction?.z);

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.type.panel.details.section.ports.interface"
            value={commonInterface || ""}
            placeholderId={commonInterface === undefined ? "semio.sketchpad.common.mixedValues" : "semio.sketchpad.app.type.portInterfacePlaceholder.label"}
            onLazyChange={(value) => updatePorts("semio.sketchpad.app.type.panel.details.section.ports.interface", { interface: value })}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Slider
            id="semio.sketchpad.app.type.panel.details.section.ports.t"
            value={[commonT ?? 0]}
            onValueChange={([value]) => {
              updatePorts("semio.sketchpad.app.type.panel.details.section.ports.t", { t: value });
            }}
            transaction={useKitTransaction("semio.sketchpad.app.type.panel.details.section.ports.t")}
            min={0}
            max={1}
            step={0.01}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.type.portPoint">
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.point.x"
              value={commonPointX}
              onChange={(value: number) => {
                updatePorts("semio.sketchpad.app.type.panel.details.section.ports.point.x", { point: { x: value } });
              }}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.point.y"
              value={commonPointY}
              onChange={(value: number) => {
                updatePorts("semio.sketchpad.app.type.panel.details.section.ports.point.y", { point: { y: value } });
              }}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.point.z"
              value={commonPointZ}
              onChange={(value: number) => {
                updatePorts("semio.sketchpad.app.type.panel.details.section.ports.point.z", { point: { z: value } });
              }}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.type.portDirection">
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.direction.x"
              value={commonDirectionX}
              onChange={(value: number) => {
                updatePorts("semio.sketchpad.app.type.panel.details.section.ports.direction.x", { direction: { x: value } });
              }}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.direction.y"
              value={commonDirectionY}
              onChange={(value: number) => {
                updatePorts("semio.sketchpad.app.type.panel.details.section.ports.direction.y", { direction: { y: value } });
              }}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
        <TreeItem>
          <TreeContent>
            <Stepper
              id="semio.sketchpad.app.type.panel.details.section.ports.direction.z"
              value={commonDirectionZ}
              onChange={(value: number) => {
                updatePorts("semio.sketchpad.app.type.panel.details.section.ports.direction.z", { direction: { z: value } });
              }}
              step={0.1}
            />
          </TreeContent>
        </TreeItem>
      </TreeItem>
    </>
  );
};

// #endregion Details

// #region Params

// #endregion Params

// #region Chat

// #endregion Chat

// #region Settings

// #endregion Settings

// #endregion Right

// #endregion Panels

// #region Tools

// Tools Registry
const toolModules = import.meta.glob<Record<string, Tool<TypeAppState>>>("./*Tool.tsx", { eager: true });

const PortToolContent: FC<ToolRenderContext<TypeAppState>> = () => {
  return null;
};

export const PortTool: Tool<TypeAppState> = {
  id: ToolKind.PORT,
  icon: <PortIcon className="size-tiny" />,
  render: (context: ToolRenderContext<TypeAppState>) => ({
    scene: <PortToolContent {...context} />,
  }),
};

// Selection Tool
export const SelectionNormalTool: Tool<TypeAppState> = {
  id: ToolKind.SELECTION_NORMAL,
  icon: <SelectToolIcon className="size-tiny" />,
  render: (context: ToolRenderContext<TypeAppState>) => ({}),
};

export const SelectionAdditiveTool: Tool<TypeAppState> = {
  id: ToolKind.SELECTION_ADDITIVE,
  icon: <AddIcon className="size-tiny" />,
  render: (context: ToolRenderContext<TypeAppState>) => ({}),
};

export const SelectionSubtractiveTool: Tool<TypeAppState> = {
  id: ToolKind.SELECTION_SUBTRACTIVE,
  icon: <RemoveIcon className="size-tiny" />,
  render: (context: ToolRenderContext<TypeAppState>) => ({}),
};

export const TypeAppTools: Tool<TypeAppState>[] = [SelectionNormalTool, SelectionAdditiveTool, SelectionSubtractiveTool, PortTool];

// Tools Toggle Group
const getTypeTools = (): ToolDefinition[] => [
  {
    id: "selection",
    defaultMode: ToolKind.SELECTION_NORMAL,
    modes: TypeAppTools.filter((tool) => tool.id.startsWith("selection")).map((tool) => ({
      id: tool.id,
      icon: tool.icon,
    })),
  },
  {
    id: "port",
    defaultMode: ToolKind.PORT,
    modes: TypeAppTools.filter((tool) => tool.id === ToolKind.PORT).map((tool) => ({
      id: tool.id,
      icon: tool.icon,
    })),
  },
];

export const ToolsToggleGroup: FC = () => {
  const { kit, type } = useParams();
  const typeAppId: TypeAppId | undefined = kit && type ? { kit, type } : undefined;
  // PERF: Use targeted hook instead of full state subscription
  const activeTool = useTypeAppActiveTool(typeAppId);
  const { setActiveTool } = useTypeAppCommands(typeAppId);

  if (!kit || !type) return null;

  return <ToolGroup tools={getTypeTools()} activeTool={activeTool} onToolChange={(tool) => setActiveTool("toolbar", tool as ToolKind)} level="panel" />;
};

// #endregion Tools

// #endregion Canvas

// #region App

const App: FC = () => {
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const appType = useAppType();
  const { setActiveTool } = useTypeAppCommands();
  // PERF: Use targeted hook instead of full state subscription
  const activeTool = useTypeAppActiveTool();
  const selection = useTypeAppSelection();
  const [isDragOver, setIsDragOver] = useState(false);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (activeTool === ToolKind.SELECTION_NORMAL) {
        if (e.shiftKey && !e.ctrlKey && !e.metaKey) {
          setActiveTool("semio.sketchpad.app.type.keydown.shift", ToolKind.SELECTION_ADDITIVE);
        } else if ((e.ctrlKey || e.metaKey) && !e.shiftKey) {
          setActiveTool("semio.sketchpad.app.type.keydown.ctrl", ToolKind.SELECTION_SUBTRACTIVE);
        }
      }
    };
    const handleKeyUp = (e: KeyboardEvent) => {
      if (activeTool === ToolKind.SELECTION_ADDITIVE && !e.shiftKey) {
        setActiveTool("semio.sketchpad.app.type.keyup.shift", ToolKind.SELECTION_NORMAL);
      } else if (activeTool === ToolKind.SELECTION_SUBTRACTIVE && !e.ctrlKey && !e.metaKey) {
        setActiveTool("semio.sketchpad.app.type.keyup.ctrl", ToolKind.SELECTION_NORMAL);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [activeTool, setActiveTool]);

  // Toolbar section is now registered in TypeApp component for earlier initialization

  // Dynamic details panel based on selection
  useEffect(() => {
    if (appType !== "type") return;

    const hasPorts = selection?.ports && selection.ports.length > 0;
    const hasMultiplePorts = selection?.ports && selection.ports.length > 1;
    const hasSinglePort = selection?.ports && selection.ports.length === 1;

    // Remove all previous sections
    const portsMultipleId = "semio.sketchpad.app.type.panel.details.section.ports.multipleTitle";

    removeSection("details", "semio.sketchpad.app.type.properties");
    removeSection("details", "semio.sketchpad.app.type.port.properties");
    removeSection("details", portsMultipleId);
    removeSection("details", "semio.sketchpad.app.kit.properties");

    if (hasSinglePort) {
      // Single port selected: show Port section then Type section
      addSection("details", {
        id: "semio.sketchpad.app.type.port.properties",
        specificity: 30,
        order: 0,
        content: () => <PortSection portGuid={selection.ports![0]} />,
      });
    } else if (hasMultiplePorts) {
      // Multiple ports selected: show Ports section then Type section
      addSection("details", {
        id: portsMultipleId,
        specificity: 30,
        order: 0,
        content: () => <PortsMultipleSection portGuids={selection.ports!} />,
      });
    }

    // Always show Type section (with all subsections)
    addSection("details", {
      id: "semio.sketchpad.app.type.properties",
      specificity: 20,
      order: 50,
      content: () => (
        <>
          <TypeDetails />
          <ModelsSection />
          <PortsListSection />
          <AuthorsSection />
          <AttributesSection />
        </>
      ),
    });

    // Always add Kit section at the bottom
    addSection("details", {
      id: "semio.sketchpad.app.kit.properties",
      specificity: 10,
      order: 100,
      content: () => (
        <React.Suspense fallback={null}>
          <KitSectionLazy />
        </React.Suspense>
      ),
    });

    return () => {
      removeSection("details", "semio.sketchpad.app.type.properties");
      removeSection("details", "semio.sketchpad.app.type.port.properties");
      removeSection("details", portsMultipleId);
      removeSection("details", "semio.sketchpad.app.kit.properties");
    };
  }, [addSection, removeSection, appType, selection]);

  const type = useType() as Type | undefined;
  const kitCommands = useKitCommands();
  const typeAppCommands = useTypeAppCommands();
  const sketchpadCommands = useSketchpadCommands();

  // Add settings sections
  useEffect(() => {
    if (appType !== "type") return;

    const { setTheme, setLanguage, setLayout, setExpertise, setMode } = sketchpadCommands;

    const SketchpadSettingsContent = () => {
      const theme = useTheme();
      const language = useLanguage();
      const layout = useLayout();
      const expertise = useExpertise();
      const mode = useMode();

      const languageEnLabel = useLabel("semio.sketchpad.settings.language.en");
      const languageDeLabel = useLabel("semio.sketchpad.settings.language.de");
      const languagePlaceholder = useLabel("semio.sketchpad.app.home.settings.language.placeholder");

      return (
        <>
          <TreeItem>
            <TreeContent>
              <ToggleGroup
                id="semio.sketchpad.settings.theme"
                value={theme}
                onValueChange={(value: string) => setTheme("semio.sketchpad.settings.theme", value as Theme)}
                showLabel
                kind="single"
                items={[
                  { value: Theme.SYSTEM, id: "semio.sketchpad.settings.theme.system", icon: <MonitorIcon className="size-small" /> },
                  { value: Theme.LIGHT, id: "semio.sketchpad.settings.theme.light", icon: <SunIcon className="size-small" /> },
                  { value: Theme.DARK, id: "semio.sketchpad.settings.theme.dark", icon: <MoonIcon className="size-small" /> },
                ]}
              />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Select id="semio.sketchpad.settings.language" value={language || "en"} onValueChange={(value: string) => setLanguage("semio.sketchpad.settings.language", value)} showLabel>
                <SelectTrigger>
                  <SelectValue placeholder={languagePlaceholder} />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="en">{languageEnLabel}</SelectItem>
                  <SelectItem value="de">{languageDeLabel}</SelectItem>
                </SelectContent>
              </Select>
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <ToggleGroup
                id="semio.sketchpad.settings.layout"
                value={typeof layout === "object" ? "desktop" : layout}
                onValueChange={(value: string) => setLayout("semio.sketchpad.settings.layout", value as "desktop" | "tablet")}
                showLabel
                kind="single"
                items={[
                  { value: "desktop", id: "semio.sketchpad.settings.layout.desktop", icon: <MousePointerIcon className="size-small" /> },
                  { value: "tablet", id: "semio.sketchpad.settings.layout.tablet", icon: <HandIcon className="size-small" /> },
                ]}
              />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <ToggleGroup
                id="semio.sketchpad.settings.expertise"
                value={expertise}
                onValueChange={(value: string) => setExpertise("semio.sketchpad.settings.expertise", value as Expertise)}
                showLabel
                kind="single"
                items={[
                  { value: Expertise.BEGINNER, id: "semio.sketchpad.settings.expertise.beginner", icon: <TutorialIcon className="size-small" /> },
                  { value: Expertise.NORMAL, id: "semio.sketchpad.settings.expertise.normal", icon: <UserIcon className="size-small" /> },
                  { value: Expertise.EXPERT, id: "semio.sketchpad.settings.expertise.expert", icon: <AwardIcon className="size-small" /> },
                ]}
              />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <ToggleGroup
                id="semio.sketchpad.settings.mode"
                value={mode}
                onValueChange={(value: string) => setMode("semio.sketchpad.settings.mode", value as Mode)}
                showLabel
                kind="single"
                items={[
                  { value: Mode.USER, id: "semio.sketchpad.settings.mode.user", icon: <UserIcon className="size-small" /> },
                  { value: Mode.DEV, id: "semio.sketchpad.settings.mode.dev", icon: <CodeIcon className="size-small" /> },
                ]}
              />
            </TreeContent>
          </TreeItem>
        </>
      );
    };

    // Add Type-specific settings (most specific)
    addSection("settings", {
      id: "semio.sketchpad.app.type.settings",
      specificity: 30,
      order: 0,
      content: () => <>{/* Type-specific settings can be added here in the future */}</>,
    });

    // Add Kit settings (middle specificity)
    addSection("settings", {
      id: "semio.sketchpad.app.kit.settings",
      specificity: 10,
      order: 0,
      content: SketchpadSettingsContent,
    });

    // Add global Sketchpad settings (least specific)
    addSection("settings", {
      id: "semio.sketchpad.settings",
      specificity: 0,
      order: 0,
      content: SketchpadSettingsContent,
    });

    return () => {
      removeSection("settings", "semio.sketchpad.app.type.settings");
      removeSection("settings", "semio.sketchpad.app.kit.settings");
      removeSection("settings", "semio.sketchpad.settings");
    };
  }, [appType, addSection, removeSection, sketchpadCommands]);

  // Handle file drops
  useEffect(() => {
    if (appType !== "type") return;

    const handleDrop = async (event: DragEvent) => {
      event.preventDefault();
      event.stopPropagation();
      setIsDragOver(false);

      const files = event.dataTransfer?.files;
      if (!files || files.length === 0 || !type || !kitCommands || !typeAppCommands) return;

      for (let i = 0; i < files.length; i++) {
        const file = files[i];

        // Create File object
        const newFileGuid = guid();
        const newFile: SemioFile = {
          guid: newFileGuid,
          name: file.name,
          size: file.size,
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
        };

        // Create Model that references the file
        const newModelGuid = guid();
        const newModel: Model = {
          guid: newModelGuid,
          file: { guid: newFileGuid },
          description: file.name,
        };

        // Add file to kit with blob
        await kitCommands.addFile("semio.sketchpad.app.type.panel.details.addFile", newFile, file);

        // Add model to type
        await kitCommands.updateType("semio.sketchpad.app.type.panel.details.addModel", type.guid, {
          models: {
            added: [newModel],
          },
        });

        // Select the new model
        typeAppCommands.setSelectedModel("semio.sketchpad.app.type.dropModel", newModelGuid);
      }
    };

    const handleDragOver = (event: DragEvent) => {
      event.preventDefault();
      event.stopPropagation();
      if (event.dataTransfer?.types.includes("Files")) {
        setIsDragOver(true);
      }
    };

    const handleDragLeave = (event: DragEvent) => {
      event.preventDefault();
      event.stopPropagation();
      // Only set to false if we're leaving the document entirely
      if (event.relatedTarget === null) {
        setIsDragOver(false);
      }
    };

    document.addEventListener("drop", handleDrop);
    document.addEventListener("dragover", handleDragOver);
    document.addEventListener("dragleave", handleDragLeave);

    return () => {
      document.removeEventListener("drop", handleDrop);
      document.removeEventListener("dragover", handleDragOver);
      document.removeEventListener("dragleave", handleDragLeave);
    };
  }, [appType, type, kitCommands, typeAppCommands]);

  const store = useTypeAppStore() as TypeAppStore | null;
  const persistedWindowLayout = useTypeApp((s) => s.windowLayout);

  const defaultLayout = useMemo(() => {
    return createDefaultLayout([TypeAppWindowKind.Scene], "row", undefined);
  }, []);

  // PERF: Validate persisted layout - if corrupted with multiple windows, reset to default
  // This prevents accumulated windows from causing massive performance issues
  const windowLayout = useMemo(() => {
    if (!persistedWindowLayout) return undefined;
    // Count windows in layout - Type app should only have 1 Scene window
    const countWindows = (node: any): number => {
      if (!node) return 0;
      if (node.type === "component") return 1;
      if (node.content && Array.isArray(node.content)) {
        return node.content.reduce((sum: number, child: any) => sum + countWindows(child), 0);
      }
      return 0;
    };
    const windowCount = countWindows(persistedWindowLayout);
    // If more than 1 window, layout is corrupted - use default
    if (windowCount > 1) {
      console.warn(`[TypeApp] Corrupted layout detected (${windowCount} windows), resetting to default`);
      return undefined;
    }
    return persistedWindowLayout;
  }, [persistedWindowLayout]);

  const windowConfig: AppWindowConfig = useMemo(() => {
    return {
      windowKinds: [
        {
          id: TypeAppWindowKind.Scene,
          label: "Scene",
          component: (props: any) => <Scene isDragOver={isDragOver} />,
        },
      ],
      defaultLayout,
    };
  }, [defaultLayout, isDragOver]);

  const handleLayoutChange = useCallback(
    (config: any) => {
      if (store && typeof store.change === "function") {
        // Only persist valid layouts with 1 window
        const countWindows = (node: any): number => {
          if (!node) return 0;
          if (node.type === "component") return 1;
          if (node.content && Array.isArray(node.content)) {
            return node.content.reduce((sum: number, child: any) => sum + countWindows(child), 0);
          }
          return 0;
        };
        if (countWindows(config) <= 1) {
          store.change({ windowLayout: config });
        }
      }
    },
    [store],
  );

  return (
    <>
      <TypeAppFooter />
      <Canvas>
        {/* PERF: Always use default layout to prevent window accumulation performance issues */}
        <LayoutCanvas windowConfig={windowConfig} layoutState={undefined} onLayoutChange={handleLayoutChange} />
      </Canvas>
    </>
  );
};

const TypeApp: FC = () => {
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();

  useLayoutEffect(() => {
    addSection("toolbar", {
      id: "semio.sketchpad.app.type.tools",
      specificity: 20,
      order: 0,
      content: <ToolsToggleGroup />,
    });

    return () => {
      removeSection("toolbar", "semio.sketchpad.app.type.tools");
    };
  }, [addSection, removeSection]);

  return <App />;
};

export default TypeApp;

// #endregion App

// #region Footer

export const TypeAppFooter: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const appType = useAppType();
  const type = useType() as Type | undefined;
  const tags = useKitTags();
  const selectedModelTags = useTypeAppSelectedModelTags();
  const { addModelTag, removeModelTag } = useTypeAppCommands();

  // Store refs for callbacks to avoid recreating them in useEffect
  const addModelTagRef = useRef(addModelTag);
  const removeModelTagRef = useRef(removeModelTag);
  const selectedModelTagsRef = useRef(selectedModelTags);

  useEffect(() => {
    addModelTagRef.current = addModelTag;
    removeModelTagRef.current = removeModelTag;
    selectedModelTagsRef.current = selectedModelTags;
  }, [addModelTag, removeModelTag, selectedModelTags]);

  // Get all unique tag guids from the type's models
  const allModelTagGuids = useMemo(() => {
    if (!type?.models) return [];
    const tagGuids = new Set<string>();
    type.models.forEach((model) => {
      model.tags?.forEach((tag) => tagGuids.add(tag.guid));
    });
    return Array.from(tagGuids);
  }, [type?.models]);

  // Get tag names from kit
  const tagNameMap = useMemo(() => {
    const map = new Map<string, string>();
    tags.forEach((tag) => {
      map.set(tag.guid, tag.name);
    });
    return map;
  }, [tags]);

  useEffect(() => {
    if (appType !== "type") return;

    // Helper function using ref to check selection at click time
    const isTagSelected = (tagGuid: string): boolean => {
      return selectedModelTagsRef.current.includes(tagGuid);
    };

    // Remove previous tag items
    allModelTagGuids.forEach((tagGuid) => {
      removeFooterItem(`semio.sketchpad.app.type.footer.tag.${tagGuid}`);
    });

    // Add footer items for each tag
    allModelTagGuids.forEach((tagGuid, index) => {
      const tagName = tagNameMap.get(tagGuid) || tagGuid.slice(0, 8);
      const isSelected = selectedModelTags.includes(tagGuid);

      addFooterItem({
        id: `semio.sketchpad.app.type.footer.tag.${tagGuid}`,
        content: <span className={`cursor-pointer transition-colors ${isSelected ? "text-foreground font-medium" : "text-muted-foreground hover:text-foreground"}`}>{tagName}</span>,
        onClick: () => {
          // Use refs in onClick to get current values at click time
          const currentSelected = isTagSelected(tagGuid);
          if (currentSelected) {
            removeModelTagRef.current("semio.sketchpad.app.type.footer.tag.remove", tagGuid);
          } else {
            addModelTagRef.current("semio.sketchpad.app.type.footer.tag.add", tagGuid);
          }
        },
        order: index,
      });
    });

    return () => {
      allModelTagGuids.forEach((tagGuid) => {
        removeFooterItem(`semio.sketchpad.app.type.footer.tag.${tagGuid}`);
      });
    };
    // Note: Intentionally excluding addModelTag, removeModelTag, selectedModelTags from deps
    // because they change on every render. We use refs to access current values.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [appType, addFooterItem, removeFooterItem, allModelTagGuids, tagNameMap]);

  return null;
};

// #endregion Footer

// #region Config

export const config: AppConfig = {
  id: "type",
  component: TypeApp,
  routeSegments: [
    {
      path: "kits/:kit",
      paramName: "kit",
      scopeProvider: KitScopeProvider,
    },
    {
      path: "types/:type",
      paramName: "type",
      scopeProvider: TypeScopeProvider,
    },
  ],
  getPanels: (): PanelDefinition[] => [
    createPanelDefinition(PanelKind.WORKBENCH, "semio.sketchpad.navbar.panelToggle.workbench.show"),
    createPanelDefinition(PanelKind.TOOLS, "semio.sketchpad.navbar.panelToggle.tools.show"),
    createPanelDefinition(PanelKind.TOOLBAR, "semio.sketchpad.navbar.panelToggle.toolbar.show"),
    createPanelDefinition(PanelKind.HUD, "semio.sketchpad.navbar.panelToggle.hud.show"),
    createPanelDefinition(PanelKind.STATS, "semio.sketchpad.navbar.panelToggle.stats.show"),
    createPanelDefinition(PanelKind.SETTINGS, "semio.sketchpad.navbar.panelToggle.settings.show"),
    createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
    createPanelDefinition(PanelKind.CHAT, "semio.sketchpad.navbar.panelToggle.chat.show"),
  ],
  matchesPath: (pathParts: string[]) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 4 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]) && pathParts[2] === "types" && isUuidPattern(pathParts[3]);
  },
  order: 30,
};

// #endregion Config
