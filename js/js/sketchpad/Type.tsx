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
import { Line, Sphere, useFBX, useGLTF } from "@react-three/drei";
import { ThreeEvent, useLoader } from "@react-three/fiber";
import { useSelector } from "@xstate/react";
import React, { createContext, FC, Suspense, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import * as THREE from "three";
import { OBJLoader } from "three/addons/loaders/OBJLoader.js";
import { useLabel } from "../i18n";
import { Author, AuthorId, Camera, Coord, findModel, guid, Guid, Kit, Model, Point, Port, selectBestModel, File as SemioFile, toSemioRotation, toThreeRotation, Type, TypeDiff, Vector } from "../semio";
import { Geometry, Input, Scene as SceneComponent, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Slider, SortableTreeItems, Stepper, Textarea, Toggle, ToggleGroup, TransactionProvider, TreeContent, TreeItem } from "./elements";
import type { AppWindowConfig, HookResult, KitCommandContext, KitDiffAppEdit, PanelDefinition, PanelVisibility, Tool, ToolDefinition, ToolRenderContext, TypeAppId } from "./shared";
import { AppConfig, AppPlugin, conditionalHookResult, createPanelDefinition, Expertise, Mode, PanelKind, readonlyHookResult, registerAppPlugin, registerRuntimeAction, Theme, ToolKind } from "./shared";
import {
  Canvas,
  createDefaultLayout,
  createDefaultTypeAppState,
  createTypeActiveToolSelector,
  createTypeAppSelector,
  createTypeCameraSelector,
  createTypeFocusedPortSelector,
  createTypeFullscreenWindowSelector,
  createTypeHoverSelector,
  createTypeOthersSelector,
  createTypePanelVisibilitySelector,
  createTypeSelectedModelTagsSelector,
  createTypeSelectionSelector,
  KitScopeProvider,
  KitStore,
  LayoutCanvas,
  TypeAppFullscreenWindow as SketchpadTypeAppFullscreenWindow,
  ToolGroup,
  TypeScopeProvider,
  useAddFooterItem,
  useAddPanelSection,
  useAppType,
  useDevice,
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
  useMode,
  useRemoveFooterItem,
  useRemovePanelSection,
  useSketchpadActor,
  useTheme,
  useTooltip,
  useType,
  useTypeAppXState,
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

import { AddIcon, AwardIcon, CheckIcon, CodeIcon, HandIcon, MonitorIcon, MoonIcon, MousePointerIcon, PortIcon, RemoveIcon, SelectToolIcon, SunIcon, TutorialIcon, UserIcon } from "@semio/assets";

// #endregion Imports

// #region Internal State Management

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

const EMPTY_TYPE_SELECTION: TypeAppSelection = {};
const EMPTY_PANEL_VISIBILITY: PanelVisibility = { toolbar: true, workbench: false, details: false, chat: false, settings: false };
const EMPTY_OTHERS: TypeAppPresenceOther[] = [];
const EMPTY_MODEL_TAG_ARRAY: string[] = [];

// #endregion Internal State Management

// #region Type App Plugin Registration

const typeAppPlugin: AppPlugin = {
  id: "type",
  namespace: "TYPE",
  machine: {
    actions: {},
    guards: {},
    eventHandlers: {},
    selectors: {},
    createDefaultState: (): TypeAppState => ({
      panelVisibility: EMPTY_PANEL_VISIBILITY,
      activeTool: ToolKind.SELECTION_NORMAL,
      fullscreenWindow: TypeAppFullscreenWindow.None,
      selection: undefined,
      hover: undefined,
      presence: undefined,
      others: [],
      camera: undefined,
      focusedPortGuid: undefined,
      selectedModelGuid: undefined,
      selectedModelTags: [],
      windowLayout: undefined,
    }),
  },
};

if (typeof window !== "undefined") {
  registerAppPlugin(typeAppPlugin);
  registerRuntimeAction("typeInit", (context: any, event: any) => {
    if (event.type !== "TYPE.INIT") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    return { typeApps: { ...context.typeApps, [key]: event.state } };
  });
  registerRuntimeAction("typeSync", (context: any, event: any) => {
    if (event.type !== "TYPE.SYNC") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, ...event.state } } };
  });
  registerRuntimeAction("typeTogglePanel", (context: any, event: any) => {
    if (event.type !== "TYPE.TOGGLE_PANEL") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, panelVisibility: { ...app.panelVisibility, [event.panel]: !app.panelVisibility[event.panel] } } } };
  });
  registerRuntimeAction("typeSetPanelVisibility", (context: any, event: any) => {
    if (event.type !== "TYPE.SET_PANEL_VISIBILITY") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, panelVisibility: event.panelVisibility } } };
  });
  registerRuntimeAction("typeSetFullscreenWindow", (context: any, event: any) => {
    if (event.type !== "TYPE.SET_FULLSCREEN_WINDOW") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, fullscreenWindow: event.window } } };
  });
  registerRuntimeAction("typeFocusPort", (context: any, event: any) => {
    if (event.type !== "TYPE.FOCUS_PORT") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, focusedPort: event.portGuid } } };
  });
  registerRuntimeAction("typeSelectModelTag", (context: any, event: any) => {
    if (event.type !== "TYPE.SELECT_MODEL_TAG") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    const tags = app.selectedModelTags || [];
    if (tags.includes(event.tagGuid)) return {};
    return { typeApps: { ...context.typeApps, [key]: { ...app, selectedModelTags: [...tags, event.tagGuid] } } };
  });
  registerRuntimeAction("typeDeselectModelTag", (context: any, event: any) => {
    if (event.type !== "TYPE.DESELECT_MODEL_TAG") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    const tags = app.selectedModelTags || [];
    return { typeApps: { ...context.typeApps, [key]: { ...app, selectedModelTags: tags.filter((g: Guid) => g !== event.tagGuid) } } };
  });
  registerRuntimeAction("typeSetCamera", (context: any, event: any) => {
    if (event.type !== "TYPE.SET_CAMERA") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, camera: event.camera } } };
  });
  registerRuntimeAction("typeSetActiveTool", (context: any, event: any) => {
    if (event.type !== "TYPE.SET_ACTIVE_TOOL") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, activeTool: event.tool } } };
  });
  registerRuntimeAction("typeSetSelection", (context: any, event: any) => {
    if (event.type !== "TYPE.SET_SELECTION") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, selection: event.selection } } };
  });
  registerRuntimeAction("typeClearSelection", (context: any, event: any) => {
    if (event.type !== "TYPE.CLEAR_SELECTION") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, selection: undefined } } };
  });
  registerRuntimeAction("typeSelectPort", (context: any, event: any) => {
    if (event.type !== "TYPE.SELECT_PORT") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    const ports = [...(app.selection?.ports || [])];
    if (!ports.includes(event.portGuid)) ports.push(event.portGuid);
    return { typeApps: { ...context.typeApps, [key]: { ...app, selection: { ...app.selection, ports } } } };
  });
  registerRuntimeAction("typeDeselectPort", (context: any, event: any) => {
    if (event.type !== "TYPE.DESELECT_PORT") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    const ports = (app.selection?.ports || []).filter((p: Guid) => p !== event.portGuid);
    return { typeApps: { ...context.typeApps, [key]: { ...app, selection: { ...app.selection, ports } } } };
  });
  registerRuntimeAction("typeSetHover", (context: any, event: any) => {
    if (event.type !== "TYPE.SET_HOVER") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, hover: event.hover } } };
  });
  registerRuntimeAction("typeClearHover", (context: any, event: any) => {
    if (event.type !== "TYPE.CLEAR_HOVER") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, hover: undefined } } };
  });
  registerRuntimeAction("typeSetModelTags", (context: any, event: any) => {
    if (event.type !== "TYPE.SET_MODEL_TAGS") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, selectedModelTags: event.tags } } };
  });
  registerRuntimeAction("typeSelectAll", (context: any, event: any) => {
    if (event.type !== "TYPE.SELECT_ALL") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, selection: { ports: [], models: [] } } } };
  });
  registerRuntimeAction("typeDeselectAll", (context: any, event: any) => {
    if (event.type !== "TYPE.DESELECT_ALL") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, selection: undefined } } };
  });
  registerRuntimeAction("typeClearFocus", (context: any, event: any) => {
    if (event.type !== "TYPE.CLEAR_FOCUS") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, focusedPort: undefined } } };
  });
  registerRuntimeAction("typeSelectModel", (context: any, event: any) => {
    if (event.type !== "TYPE.SELECT_MODEL") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    const models = [...(app.selection?.models || [])];
    if (!models.includes(event.modelGuid)) models.push(event.modelGuid);
    return { typeApps: { ...context.typeApps, [key]: { ...app, selection: { ...app.selection, models } } } };
  });
  registerRuntimeAction("typeDeselectModel", (context: any, event: any) => {
    if (event.type !== "TYPE.DESELECT_MODEL") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    const models = (app.selection?.models || []).filter((m: Guid) => m !== event.modelGuid);
    return { typeApps: { ...context.typeApps, [key]: { ...app, selection: { ...app.selection, models } } } };
  });
  registerRuntimeAction("typeHoverPort", (context: any, event: any) => {
    if (event.type !== "TYPE.HOVER_PORT") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, hover: { port: event.portGuid } } } };
  });
  registerRuntimeAction("typeHoverModel", (context: any, event: any) => {
    if (event.type !== "TYPE.HOVER_MODEL") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, hover: { model: event.modelGuid } } } };
  });
  registerRuntimeAction("typeSetSelectedModel", (context: any, event: any) => {
    if (event.type !== "TYPE.SET_SELECTED_MODEL") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, selectedModelGuid: event.modelGuid } } };
  });
  registerRuntimeAction("typeAddModelTag", (context: any, event: any) => {
    if (event.type !== "TYPE.ADD_MODEL_TAG") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    const tags = [...(app.selectedModelTags || [])];
    if (!tags.includes(event.tag)) tags.push(event.tag);
    return { typeApps: { ...context.typeApps, [key]: { ...app, selectedModelTags: tags } } };
  });
  registerRuntimeAction("typeRemoveModelTag", (context: any, event: any) => {
    if (event.type !== "TYPE.REMOVE_MODEL_TAG") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    const tags = (app.selectedModelTags || []).filter((t: string) => t !== event.tag);
    return { typeApps: { ...context.typeApps, [key]: { ...app, selectedModelTags: tags } } };
  });
  registerRuntimeAction("typeClearModelTags", (context: any, event: any) => {
    if (event.type !== "TYPE.CLEAR_MODEL_TAGS") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    return { typeApps: { ...context.typeApps, [key]: { ...app, selectedModelTags: [] } } };
  });
  registerRuntimeAction("typeTransactionStart", (context: any, event: any) => {
    if (event.type !== "TYPE.TRANSACTION.START") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key] || createDefaultTypeAppState();
    const tx = app.transaction;
    if (tx.isTransactionActive) {
      const pastStack = [...tx.pastTransactionStack];
      if (tx.currentTransactionStack.length > 0) {
        const merged = tx.currentTransactionStack.length === 1 ? tx.currentTransactionStack[0] : { do: tx.currentTransactionStack[tx.currentTransactionStack.length - 1].do, undo: tx.currentTransactionStack[0].undo };
        pastStack.push(merged);
      }
      return { typeApps: { ...context.typeApps, [key]: { ...app, transaction: { isTransactionActive: true, currentTransactionStack: [], pastTransactionStack: pastStack, redoStack: [] } } } };
    }
    return { typeApps: { ...context.typeApps, [key]: { ...app, transaction: { ...tx, isTransactionActive: true, currentTransactionStack: [], redoStack: [] } } } };
  });
  registerRuntimeAction("typeTransactionCommit", (context: any, event: any) => {
    if (event.type !== "TYPE.TRANSACTION.COMMIT") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key];
    if (!app || !app.transaction.isTransactionActive) return {};
    const tx = app.transaction;
    const pastStack = [...tx.pastTransactionStack];
    if (tx.currentTransactionStack.length > 0) {
      const merged = tx.currentTransactionStack.length === 1 ? tx.currentTransactionStack[0] : { do: tx.currentTransactionStack[tx.currentTransactionStack.length - 1].do, undo: tx.currentTransactionStack[0].undo };
      pastStack.push(merged);
    }
    return { typeApps: { ...context.typeApps, [key]: { ...app, transaction: { isTransactionActive: false, currentTransactionStack: [], pastTransactionStack: pastStack, redoStack: [] } } } };
  });
  registerRuntimeAction("typeTransactionAbort", (context: any, event: any) => {
    if (event.type !== "TYPE.TRANSACTION.ABORT") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key];
    if (!app || !app.transaction.isTransactionActive) return {};
    return { typeApps: { ...context.typeApps, [key]: { ...app, transaction: { ...app.transaction, isTransactionActive: false, currentTransactionStack: [] } } } };
  });
  registerRuntimeAction("typeTransactionUndo", (context: any, event: any) => {
    if (event.type !== "TYPE.TRANSACTION.UNDO") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key];
    if (!app) return {};
    const tx = app.transaction;
    if (tx.isTransactionActive && tx.currentTransactionStack.length > 0) {
      const currentStack = [...tx.currentTransactionStack];
      currentStack.pop();
      return { typeApps: { ...context.typeApps, [key]: { ...app, transaction: { ...tx, currentTransactionStack: currentStack } } } };
    } else if (!tx.isTransactionActive && tx.pastTransactionStack.length > 0) {
      const pastStack = [...tx.pastTransactionStack];
      const edit = pastStack.pop()!;
      const redoStack = [...tx.redoStack, edit];
      return { typeApps: { ...context.typeApps, [key]: { ...app, transaction: { ...tx, pastTransactionStack: pastStack, redoStack } } } };
    }
    return {};
  });
  registerRuntimeAction("typeTransactionRedo", (context: any, event: any) => {
    if (event.type !== "TYPE.TRANSACTION.REDO") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key];
    if (!app || app.transaction.isTransactionActive || app.transaction.redoStack.length === 0) return {};
    const tx = app.transaction;
    const redoStack = [...tx.redoStack];
    const edit = redoStack.pop()!;
    const pastStack = [...tx.pastTransactionStack, edit];
    return { typeApps: { ...context.typeApps, [key]: { ...app, transaction: { ...tx, pastTransactionStack: pastStack, redoStack } } } };
  });
  registerRuntimeAction("typeTransactionRecordEdit", (context: any, event: any) => {
    if (event.type !== "TYPE.TRANSACTION.RECORD_EDIT") return {};
    const key = `${event.kitGuid}:${event.typeGuid}`;
    const app = context.typeApps[key];
    if (!app || !app.transaction.isTransactionActive) return {};
    const currentStack = [...app.transaction.currentTransactionStack, event.edit];
    return { typeApps: { ...context.typeApps, [key]: { ...app, transaction: { ...app.transaction, currentTransactionStack: currentStack, redoStack: [] } } } };
  });
}

// #endregion Type App Plugin Registration

// #region XState Hooks

/**
 * Get Type app state from XState.
 * This is the new XState-based hook.
 */
export function useTypeApp<T>(selector?: (state: TypeAppState) => T, id?: TypeAppId): T | TypeAppState | null {
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? id?.kit;
  const typeGuid = typeScope?.guid ?? id?.type;

  if (!kitGuid || !typeGuid) return null;

  const state = useTypeAppXState(kitGuid, typeGuid);
  if (selector) {
    return selector(state as unknown as TypeAppState) as T;
  }
  return state as unknown as TypeAppState;
}

export function useTypeAppSelection(): HookResult<TypeAppSelection> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypeSelectionSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const value = useSelector(actor, selector) ?? EMPTY_TYPE_SELECTION;
  const canSetEvent = useMemo(() => ({ type: "TYPE.SET_SELECTION" as const, kitGuid, typeGuid, selection: {} as TypeAppSelection }), [kitGuid, typeGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (selection: TypeAppSelection) => {
      actor.send({ type: "TYPE.SET_SELECTION", kitGuid, typeGuid, selection });
    };
  }, [actor, kitGuid, typeGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

export function useTypeAppPanelVisibility(): HookResult<PanelVisibility> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypePanelVisibilitySelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const value = useSelector(actor, selector) ?? EMPTY_PANEL_VISIBILITY;
  const canSetEvent = useMemo(() => ({ type: "TYPE.SET_PANEL_VISIBILITY" as const, kitGuid, typeGuid, panelVisibility: {} as PanelVisibility }), [kitGuid, typeGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (visibility: PanelVisibility) => {
      actor.send({ type: "TYPE.SET_PANEL_VISIBILITY", kitGuid, typeGuid, panelVisibility: visibility });
    };
  }, [actor, kitGuid, typeGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

export function useTypeAppOthers(): HookResult<TypeAppPresenceOther[]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypeOthersSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const value = useSelector(actor, selector) ?? EMPTY_OTHERS;
  return readonlyHookResult(value);
}

export function useTypeAppCamera(): HookResult<Camera | undefined> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypeCameraSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const value = useSelector(actor, selector);
  const canSetEvent = useMemo(() => ({ type: "TYPE.SET_CAMERA" as const, kitGuid, typeGuid, camera: undefined as Camera | undefined }), [kitGuid, typeGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (camera: Camera | undefined) => {
      actor.send({ type: "TYPE.SET_CAMERA", kitGuid, typeGuid, camera: camera! });
    };
  }, [actor, kitGuid, typeGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

export function useTypeAppFocusedPortGuid(): HookResult<Guid | undefined> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypeFocusedPortSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const value = useSelector(actor, selector);
  const canSetEvent = useMemo(() => ({ type: "TYPE.FOCUS_PORT" as const, kitGuid, typeGuid, portGuid: "" }), [kitGuid, typeGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (portGuid: Guid | undefined) => {
      if (portGuid) {
        actor.send({ type: "TYPE.FOCUS_PORT", kitGuid, typeGuid, portGuid });
      } else {
        actor.send({ type: "TYPE.CLEAR_FOCUS", kitGuid, typeGuid });
      }
    };
  }, [actor, kitGuid, typeGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

export function useTypeAppHover(): HookResult<TypeAppHover | undefined> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypeHoverSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const value = useSelector(actor, selector);
  const canSetEvent = useMemo(() => ({ type: "TYPE.SET_HOVER" as const, kitGuid, typeGuid, hover: {} as TypeAppHover }), [kitGuid, typeGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (hover: TypeAppHover | undefined) => {
      if (hover?.port) {
        actor.send({ type: "TYPE.HOVER_PORT", kitGuid, typeGuid, portGuid: hover.port });
      } else if (hover?.model) {
        actor.send({ type: "TYPE.HOVER_MODEL", kitGuid, typeGuid, modelGuid: hover.model });
      } else {
        actor.send({ type: "TYPE.CLEAR_HOVER", kitGuid, typeGuid });
      }
    };
  }, [actor, kitGuid, typeGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

export function useTypeAppActiveTool(): HookResult<ToolKind> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypeActiveToolSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const value = useSelector(actor, selector) ?? ToolKind.SELECTION_NORMAL;
  const canSetEvent = useMemo(() => ({ type: "TYPE.SET_ACTIVE_TOOL" as const, kitGuid, typeGuid, tool: ToolKind.SELECTION_NORMAL }), [kitGuid, typeGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (tool: ToolKind) => {
      actor.send({ type: "TYPE.SET_ACTIVE_TOOL", kitGuid, typeGuid, tool });
    };
  }, [actor, kitGuid, typeGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

interface Transaction {
  start?: () => void;
  finalize?: () => void;
  abort?: () => void;
}

export function useTypeAppTransaction(_id?: TypeAppId): Transaction {
  // TODO: Implement transaction via XState events
  return {
    start: () => {},
    finalize: () => {},
    abort: () => {},
  };
}

export function useTypeAppCommands(id?: TypeAppId) {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? id?.kit ?? "";
  const typeGuid = typeScope?.guid ?? id?.type ?? "";

  return useMemo(() => {
    const noOp = () => {};
    if (!kitGuid || !typeGuid) {
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
      startTransaction: () => actor.send({ type: "TYPE.TRANSACTION.START", kitGuid, typeGuid }),
      finalizeTransaction: () => actor.send({ type: "TYPE.TRANSACTION.COMMIT", kitGuid, typeGuid }),
      abortTransaction: () => actor.send({ type: "TYPE.TRANSACTION.ABORT", kitGuid, typeGuid }),
      undo: () => actor.send({ type: "TYPE.TRANSACTION.UNDO", kitGuid, typeGuid }),
      redo: () => actor.send({ type: "TYPE.TRANSACTION.REDO", kitGuid, typeGuid }),
      selectAll: () => actor.send({ type: "TYPE.SELECT_ALL", kitGuid, typeGuid }),
      deselectAll: () => actor.send({ type: "TYPE.DESELECT_ALL", kitGuid, typeGuid }),
      togglePanel: (_origin: string, panelKey: keyof PanelVisibility) => actor.send({ type: "TYPE.TOGGLE_PANEL", kitGuid, typeGuid, panel: panelKey }),
      setCamera: (camera: Camera) => actor.send({ type: "TYPE.SET_CAMERA", kitGuid, typeGuid, camera }),
      focusPort: (portGuid: Guid) => actor.send({ type: "TYPE.FOCUS_PORT", kitGuid, typeGuid, portGuid }),
      clearFocus: () => actor.send({ type: "TYPE.CLEAR_FOCUS", kitGuid, typeGuid }),
      setActiveTool: (tool: ToolKind) => actor.send({ type: "TYPE.SET_ACTIVE_TOOL", kitGuid, typeGuid, tool }),
      selectPort: (portGuid: Guid) => actor.send({ type: "TYPE.SELECT_PORT", kitGuid, typeGuid, portGuid }),
      deselectPort: (portGuid: Guid) => actor.send({ type: "TYPE.DESELECT_PORT", kitGuid, typeGuid, portGuid }),
      selectModel: (modelGuid: Guid) => actor.send({ type: "TYPE.SELECT_MODEL", kitGuid, typeGuid, modelGuid }),
      deselectModel: (modelGuid: Guid) => actor.send({ type: "TYPE.DESELECT_MODEL", kitGuid, typeGuid, modelGuid }),
      hoverPort: (portGuid: Guid) => actor.send({ type: "TYPE.HOVER_PORT", kitGuid, typeGuid, portGuid }),
      hoverModel: (modelGuid: Guid) => actor.send({ type: "TYPE.HOVER_MODEL", kitGuid, typeGuid, modelGuid }),
      clearHover: () => actor.send({ type: "TYPE.CLEAR_HOVER", kitGuid, typeGuid }),
      setSelectedModel: (modelGuid: Guid) => actor.send({ type: "TYPE.SET_SELECTED_MODEL", kitGuid, typeGuid, modelGuid }),
      addModelTag: (tag: string) => actor.send({ type: "TYPE.ADD_MODEL_TAG", kitGuid, typeGuid, tag }),
      removeModelTag: (tag: string) => actor.send({ type: "TYPE.REMOVE_MODEL_TAG", kitGuid, typeGuid, tag }),
      clearModelTags: () => actor.send({ type: "TYPE.CLEAR_MODEL_TAGS", kitGuid, typeGuid }),
      setModelTags: (tags: string[]) => actor.send({ type: "TYPE.SET_MODEL_TAGS", kitGuid, typeGuid, tags }),
      execute: (command: string, ..._args: any[]) => {
        console.warn(`Type app execute not yet migrated for command: ${command}`);
      },
    };
  }, [actor, kitGuid, typeGuid]);
}

export function useTypeAppIsPortSelected(portId: string): HookResult<boolean> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypeSelectionSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const selection = useSelector(actor, selector);
  const value = selection?.ports?.includes(portId) ?? false;
  const canSetEvent = useMemo(() => ({ type: "TYPE.SELECT_PORT" as const, kitGuid, typeGuid, portGuid: portId }), [kitGuid, typeGuid, portId]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (isSelected: boolean) => {
      if (isSelected) {
        actor.send({ type: "TYPE.SELECT_PORT", kitGuid, typeGuid, portGuid: portId });
      } else {
        actor.send({ type: "TYPE.DESELECT_PORT", kitGuid, typeGuid, portGuid: portId });
      }
    };
  }, [actor, kitGuid, typeGuid, portId, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

export function useTypeAppIsPortHovered(portId: string): HookResult<boolean> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypeHoverSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const hover = useSelector(actor, selector);
  const value = hover?.port === portId;
  const canSetEvent = useMemo(() => ({ type: "TYPE.HOVER_PORT" as const, kitGuid, typeGuid, portGuid: portId }), [kitGuid, typeGuid, portId]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (isHovered: boolean) => {
      if (isHovered) {
        actor.send({ type: "TYPE.HOVER_PORT", kitGuid, typeGuid, portGuid: portId });
      } else {
        actor.send({ type: "TYPE.CLEAR_HOVER", kitGuid, typeGuid });
      }
    };
  }, [actor, kitGuid, typeGuid, portId, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

export function useTypeAppSelectedModelGuid(): HookResult<Guid | undefined> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypeAppSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const state = useSelector(actor, selector);
  const value = state?.selectedModelGuid;
  const canSetEvent = useMemo(() => ({ type: "TYPE.SET_SELECTED_MODEL" as const, kitGuid, typeGuid, modelGuid: "" }), [kitGuid, typeGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (modelGuid: Guid | undefined) => {
      if (modelGuid) {
        actor.send({ type: "TYPE.SET_SELECTED_MODEL", kitGuid, typeGuid, modelGuid });
      }
    };
  }, [actor, kitGuid, typeGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

export function useTypeAppSelectedModelTags(): HookResult<string[]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypeSelectedModelTagsSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const value = useSelector(actor, selector) ?? EMPTY_MODEL_TAG_ARRAY;
  const canSetEvent = useMemo(() => ({ type: "TYPE.SET_MODEL_TAGS" as const, kitGuid, typeGuid, tags: [] }), [kitGuid, typeGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (tags: string[]) => {
      actor.send({ type: "TYPE.SET_MODEL_TAGS", kitGuid, typeGuid, tags });
    };
  }, [actor, kitGuid, typeGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

//#region Action Hooks

export type ActionHookResult<TArgs extends any[]> = readonly [action: ((...args: TArgs) => void) | undefined, canAct: boolean];

export function useTypeAppSelectPort(): ActionHookResult<[portGuid: string]> {
  const [, setSelection, canSetSelection] = useTypeAppSelection();
  const [selection] = useTypeAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (portGuid: string) => setSelection({ ...selection, ports: [portGuid], models: [] });
  }, [setSelection, canSetSelection, selection]);
  return [action, canSetSelection];
}

export function useTypeAppDeselectPort(): ActionHookResult<[portGuid: string]> {
  const [, setSelection, canSetSelection] = useTypeAppSelection();
  const [selection] = useTypeAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (portGuid: string) => {
      const currentPorts = selection?.ports ?? [];
      setSelection({ ...selection, ports: currentPorts.filter((p) => p !== portGuid) });
    };
  }, [setSelection, canSetSelection, selection]);
  return [action, canSetSelection];
}

export function useTypeAppHoverPort(): ActionHookResult<[portGuid: string]> {
  const [, setHover, canSetHover] = useTypeAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (portGuid: string) => setHover({ port: portGuid });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

export function useTypeAppHoverModel(): ActionHookResult<[modelGuid: string]> {
  const [, setHover, canSetHover] = useTypeAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (modelGuid: string) => setHover({ model: modelGuid });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

export function useTypeAppClearHover(): ActionHookResult<[]> {
  const [, setHover, canSetHover] = useTypeAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return () => setHover(undefined);
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

export function useTypeAppFocusPort(): ActionHookResult<[portGuid: string]> {
  const [, setFocusedPortGuid, canSetFocusedPortGuid] = useTypeAppFocusedPortGuid();
  const action = useMemo(() => {
    if (!canSetFocusedPortGuid || !setFocusedPortGuid) return undefined;
    return (portGuid: string) => setFocusedPortGuid(portGuid);
  }, [setFocusedPortGuid, canSetFocusedPortGuid]);
  return [action, canSetFocusedPortGuid];
}

export function useTypeAppClearFocus(): ActionHookResult<[]> {
  const [, setFocusedPortGuid, canSetFocusedPortGuid] = useTypeAppFocusedPortGuid();
  const action = useMemo(() => {
    if (!canSetFocusedPortGuid || !setFocusedPortGuid) return undefined;
    return () => setFocusedPortGuid(undefined);
  }, [setFocusedPortGuid, canSetFocusedPortGuid]);
  return [action, canSetFocusedPortGuid];
}

export function useTypeAppDeselectAll(): ActionHookResult<[]> {
  const [, setSelection, canSetSelection] = useTypeAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return () => setSelection({ ports: [], models: [] });
  }, [setSelection, canSetSelection]);
  return [action, canSetSelection];
}

export function useTypeAppSelectModel(): ActionHookResult<[modelGuid: string]> {
  const [, setSelection, canSetSelection] = useTypeAppSelection();
  const [selection] = useTypeAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (modelGuid: string) => setSelection({ ...selection, models: [modelGuid], ports: [] });
  }, [setSelection, canSetSelection, selection]);
  return [action, canSetSelection];
}

export function useTypeAppDeselectModel(): ActionHookResult<[modelGuid: string]> {
  const [, setSelection, canSetSelection] = useTypeAppSelection();
  const [selection] = useTypeAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (modelGuid: string) => {
      const currentModels = selection?.models ?? [];
      setSelection({ ...selection, models: currentModels.filter((m) => m !== modelGuid) });
    };
  }, [setSelection, canSetSelection, selection]);
  return [action, canSetSelection];
}

export function useTypeAppSetActiveTool(): ActionHookResult<[tool: ToolKind]> {
  const [, setActiveTool, canSetActiveTool] = useTypeAppActiveTool();
  const action = useMemo(() => {
    if (!canSetActiveTool || !setActiveTool) return undefined;
    return (tool: ToolKind) => setActiveTool(tool);
  }, [setActiveTool, canSetActiveTool]);
  return [action, canSetActiveTool];
}

export function useTypeAppSetCamera(): ActionHookResult<[camera: Camera]> {
  const [, setCamera, canSetCamera] = useTypeAppCamera();
  const action = useMemo(() => {
    if (!canSetCamera || !setCamera) return undefined;
    return (camera: Camera) => setCamera(camera);
  }, [setCamera, canSetCamera]);
  return [action, canSetCamera];
}

export function useTypeAppTogglePanel(): ActionHookResult<[panelKey: keyof PanelVisibility]> {
  const [panelVisibility, setPanelVisibility, canSetPanelVisibility] = useTypeAppPanelVisibility();
  const action = useMemo(() => {
    if (!canSetPanelVisibility || !setPanelVisibility) return undefined;
    return (panelKey: keyof PanelVisibility) => {
      setPanelVisibility({ ...panelVisibility, [panelKey]: !panelVisibility[panelKey] });
    };
  }, [setPanelVisibility, canSetPanelVisibility, panelVisibility]);
  return [action, canSetPanelVisibility];
}

export function useTypeAppAddModelTag(): ActionHookResult<[tag: string]> {
  const [selectedTags, setSelectedTags, canSetSelectedTags] = useTypeAppSelectedModelTags();
  const action = useMemo(() => {
    if (!canSetSelectedTags || !setSelectedTags) return undefined;
    return (tag: string) => {
      if (!selectedTags.includes(tag)) {
        setSelectedTags([...selectedTags, tag]);
      }
    };
  }, [setSelectedTags, canSetSelectedTags, selectedTags]);
  return [action, canSetSelectedTags];
}

export function useTypeAppRemoveModelTag(): ActionHookResult<[tag: string]> {
  const [selectedTags, setSelectedTags, canSetSelectedTags] = useTypeAppSelectedModelTags();
  const action = useMemo(() => {
    if (!canSetSelectedTags || !setSelectedTags) return undefined;
    return (tag: string) => {
      setSelectedTags(selectedTags.filter((t) => t !== tag));
    };
  }, [setSelectedTags, canSetSelectedTags, selectedTags]);
  return [action, canSetSelectedTags];
}

export function useTypeAppSetSelectedModel(): ActionHookResult<[modelGuid: string]> {
  const [, setSelectedModel, canSetSelectedModel] = useTypeAppSelectedModelGuid();
  const action = useMemo(() => {
    if (!canSetSelectedModel || !setSelectedModel) return undefined;
    return (modelGuid: string) => setSelectedModel(modelGuid);
  }, [setSelectedModel, canSetSelectedModel]);
  return [action, canSetSelectedModel];
}

//#endregion Action Hooks

const TypeAppScopeContext = createContext<{ id: string } | undefined>(undefined);
export const TypeAppScopeProvider = (props: { id: string; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(TypeAppScopeContext.Provider, { value }, props.children as any);
};
const useTypeAppScope = () => useContext(TypeAppScopeContext);

// #endregion Internal State Management

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
const getComputedColorForMesh = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();

const GLTFMesh: FC<{ url: string; onPointerDown: any; onPointerUp: any; onPointerMove: any; onPointerOut: any }> = ({ url, onPointerDown, onPointerUp, onPointerMove, onPointerOut }) => {
  const gltf = useGLTF(url);
  const plasterColor = useMemo(() => new THREE.Color(getComputedColorForMesh("--plaster")), []);
  const plasterEdgeColor = useMemo(() => new THREE.Color(getComputedColorForMesh("--plaster-edge")), []);

  const clonedScene = useMemo(() => {
    const cloned = gltf.scene.clone();
    const plasterMaterial = new THREE.MeshStandardMaterial({
      color: plasterColor,
      flatShading: false,
      metalness: 0,
      roughness: 0.8,
    });
    const edgeMaterial = new THREE.LineBasicMaterial({ color: plasterEdgeColor });

    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
        if (Array.isArray(child.material)) {
          child.material = child.material.map(() => plasterMaterial.clone());
        } else {
          child.material = plasterMaterial.clone();
        }
      } else if (child instanceof THREE.Line || child instanceof THREE.LineSegments || child instanceof THREE.Points) {
        (child as any).material = edgeMaterial.clone();
      }
    });
    return cloned;
  }, [gltf.scene, plasterColor, plasterEdgeColor]);
  return <primitive object={clonedScene} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
};

const FBXMesh: FC<{ url: string; onPointerDown: any; onPointerUp: any; onPointerMove: any; onPointerOut: any }> = ({ url, onPointerDown, onPointerUp, onPointerMove, onPointerOut }) => {
  const scene = useFBX(url);
  const plasterColor = useMemo(() => new THREE.Color(getComputedColorForMesh("--plaster")), []);
  const plasterEdgeColor = useMemo(() => new THREE.Color(getComputedColorForMesh("--plaster-edge")), []);

  const clonedScene = useMemo(() => {
    const cloned = scene.clone();
    const plasterMaterial = new THREE.MeshStandardMaterial({
      color: plasterColor,
      flatShading: false,
      metalness: 0,
      roughness: 0.8,
    });
    const edgeMaterial = new THREE.LineBasicMaterial({ color: plasterEdgeColor });

    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
        if (Array.isArray(child.material)) {
          child.material = child.material.map(() => plasterMaterial.clone());
        } else {
          child.material = plasterMaterial.clone();
        }
      } else if (child instanceof THREE.Line || child instanceof THREE.LineSegments || child instanceof THREE.Points) {
        (child as any).material = edgeMaterial.clone();
      }
    });
    return cloned;
  }, [scene, plasterColor, plasterEdgeColor]);
  return <primitive object={clonedScene} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
};

const OBJMesh: FC<{ url: string; onPointerDown: any; onPointerUp: any; onPointerMove: any; onPointerOut: any }> = ({ url, onPointerDown, onPointerUp, onPointerMove, onPointerOut }) => {
  const obj = useLoader(OBJLoader, url);
  const plasterColor = useMemo(() => new THREE.Color(getComputedColorForMesh("--plaster")), []);
  const plasterEdgeColor = useMemo(() => new THREE.Color(getComputedColorForMesh("--plaster-edge")), []);

  const clonedScene = useMemo(() => {
    const cloned = obj.clone();
    const plasterMaterial = new THREE.MeshStandardMaterial({
      color: plasterColor,
      flatShading: false,
      metalness: 0,
      roughness: 0.8,
    });
    const edgeMaterial = new THREE.LineBasicMaterial({ color: plasterEdgeColor });

    cloned.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.raycast = THREE.Mesh.prototype.raycast;
        if (Array.isArray(child.material)) {
          child.material = child.material.map(() => plasterMaterial.clone());
        } else {
          child.material = plasterMaterial.clone();
        }
      } else if (child instanceof THREE.Line || child instanceof THREE.LineSegments || child instanceof THREE.Points) {
        (child as any).material = edgeMaterial.clone();
      }
    });
    return cloned;
  }, [obj, plasterColor, plasterEdgeColor]);
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
  const kitDataSource = useKitStore() as KitStore;
  const [selectedModelGuid] = useTypeAppSelectedModelGuid();
  const [selectedModelTags] = useTypeAppSelectedModelTags();
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
    const url = kitDataSource.getFileUrl(file.guid);
    if (url) {
      return { modelUrl: url, fileExtension: ext, fileGuid: file.guid, modelGuid: model.guid, selectionReason: reason };
    }

    // No direct URL - will try blob URL in useEffect
    return { modelUrl: null, fileExtension: ext, fileGuid: file.guid, modelGuid: model.guid, selectionReason: reason };
  }, [typeModels, typeConcepts, files, kitDataSource, selectedModelGuid, selectedModelTags]);

  useEffect(() => {
    if (modelGuid && modelGuid !== prevModelGuidRef.current) {
      prevModelGuidRef.current = modelGuid;
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
        const url = await kitDataSource.getFileBlobUrl(fileGuid);
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
    // Note: We do NOT revoke the blob URL here because it's owned by KitStore's regularFiles cache.
    // The KitStore revokes blob URLs when files are removed from the kit.
    return () => {
      cancelled = true;
    };
  }, [fileGuid, kitDataSource]);

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
    return null;
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
  const [activeTool] = useTypeAppActiveTool();
  // PERF: Use targeted selectors instead of fetching full type/kit
  // Only subscribe to the specific fields we actually need
  const typePorts = useType(selectTypePorts) as Port[] | undefined;
  const typeGuid = useType(selectTypeGuid) as string | undefined;
  // PERF: useKit was only used to check existence - kitCommands being non-null serves same purpose
  const kitCommands = useKitCommands();
  const [selection] = useTypeAppSelection();
  const [hover] = useTypeAppHover();
  // PERF: Removed useTypeApp((s) => s) - was causing full re-renders on every state change
  // Tools (SelectionNormalTool, PortTool, etc.) return null/empty scene content anyway
  const [selectPort] = useTypeAppSelectPort();
  const [deselectPort] = useTypeAppDeselectPort();
  const [hoverPort] = useTypeAppHoverPort();
  const [clearHover] = useTypeAppClearHover();
  const [focusPort] = useTypeAppFocusPort();
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
      if (focusPort) focusPort(itemId);
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

        kitCommands.updateType(typeGuid, {
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
    if (clearHover) clearHover();
  }, [clearHover]);

  const handlePortClick = useCallback(
    (portId: string) => {
      const isSelected = selection?.ports?.includes(portId) || false;
      if (activeTool === ToolKind.SELECTION_ADDITIVE) {
        if (!isSelected && selectPort) selectPort(portId);
      } else if (activeTool === ToolKind.SELECTION_SUBTRACTIVE) {
        if (isSelected && deselectPort) deselectPort(portId);
      } else {
        const currentPorts = selection?.ports ?? [];
        if (currentPorts.length > 0) {
          currentPorts.forEach((id) => deselectPort && deselectPort(id));
        }
        if (!isSelected || currentPorts.length > 1) {
          if (selectPort) selectPort(portId);
        }
      }
    },
    [selection, selectPort, deselectPort, activeTool],
  );

  const handlePortHover = useCallback(
    (portId: string) => {
      if (hoverPort) hoverPort(portId);
    },
    [hoverPort],
  );

  const handlePortLeave = useCallback(() => {
    if (clearHover) clearHover();
  }, [clearHover]);

  const handlePortDoubleClick = useCallback(
    (portId: string) => {
      if (focusPort) focusPort(portId);
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
  const [setCamera] = useTypeAppSetCamera();
  const [deselectAll] = useTypeAppDeselectAll();
  const [clearFocus] = useTypeAppClearFocus();
  const [camera] = useTypeAppCamera();
  const [focusedPortGuid] = useTypeAppFocusedPortGuid();

  const onCameraChange = useCallback(
    (newCamera: Camera) => {
      if (setCamera) setCamera(newCamera);
    },
    [setCamera],
  );

  const onPointerMissed = useCallback(
    (event: MouseEvent) => {
      if (!(event.ctrlKey || event.metaKey) && !event.shiftKey && deselectAll) deselectAll();
    },
    [deselectAll],
  );

  const onFocusComplete = useCallback(() => {
    if (clearFocus) clearFocus();
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

  const updateTypeField = (diff: any) => {
    kitCommands?.updateType(type.guid, diff);
  };

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input lazy id="semio.sketchpad.app.type.panel.details.section.type.name" value={type.name} onLazyChange={(value) => updateTypeField({ name: value })} showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea
            lazy
            id="semio.sketchpad.app.type.panel.details.section.type.description"
            value={type.description || ""}
            placeholderId="semio.sketchpad.app.type.descriptionPlaceholder.label"
            onLazyChange={(value) => updateTypeField({ description: value })}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input lazy id="semio.sketchpad.app.type.panel.details.section.type.icon" value={type.icon || ""} placeholderId="semio.sketchpad.app.type.iconPlaceholder.label" onLazyChange={(value) => updateTypeField({ icon: value })} showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input lazy id="semio.sketchpad.app.type.panel.details.section.type.image" value={type.image || ""} placeholderId="semio.sketchpad.app.type.imagePlaceholder.label" onLazyChange={(value) => updateTypeField({ image: value })} showLabel />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input
            lazy
            id="semio.sketchpad.app.type.panel.details.section.type.parent"
            value={type.parent?.guid || ""}
            placeholderId="semio.sketchpad.app.type.parentPlaceholder.label"
            onLazyChange={(value) => updateTypeField({ parent: value ? { guid: value } : undefined })}
            showLabel
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Toggle id="semio.sketchpad.app.type.panel.details.section.type.abstract" pressed={type.isAbstract || false} onPressedChange={(value) => updateTypeField({ isAbstract: value })} showLabel icon={<CheckIcon />} />
        </TreeContent>
      </TreeItem>
      {type.unit !== undefined && (
        <TreeItem>
          <TreeContent>
            <Input lazy id="semio.sketchpad.app.type.panel.details.section.type.unit" value={type.unit} onLazyChange={(value) => updateTypeField({ unit: value })} showLabel />
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
  const [selectModel] = useTypeAppSelectModel();
  const [deselectModel] = useTypeAppDeselectModel();
  const [hoverModel] = useTypeAppHoverModel();
  const [clearHover] = useTypeAppClearHover();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;
  const [selection] = useTypeAppSelection();
  const [hover] = useTypeAppHover();

  const applyDiff = (diff: any) => {
    kitCommands?.updateType(type.guid, diff);
  };

  const updateModel = (id: string, modelDiff: any) => {
    applyDiff({
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
              applyDiff({
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
              applyDiff({
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
                  onPointerEnter={() => hoverModel && hoverModel(model.guid)}
                  onPointerLeave={() => clearHover && clearHover()}
                  onClick={() => (isSelected ? deselectModel && deselectModel(model.guid) : selectModel && selectModel(model.guid))}
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
                          applyDiff({
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
                            updateModel(model.guid, { url: e.target.value });
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
                            updateModel(model.guid, { description: e.target.value });
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
                            updateModel(model.guid, {
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
  const [selectPort] = useTypeAppSelectPort();
  const [deselectPort] = useTypeAppDeselectPort();
  const [hoverPort] = useTypeAppHoverPort();
  const [clearHover] = useTypeAppClearHover();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;
  const [selection] = useTypeAppSelection();
  const [hover] = useTypeAppHover();

  const applyDiff = (diff: any) => {
    kitCommands?.updateType(type.guid, diff);
  };

  const updatePort = (id: string, portDiff: any) => {
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
    applyDiff({
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
              applyDiff({
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
              applyDiff({
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
                  if (deselectPort) deselectPort(port.guid);
                } else {
                  if (selectPort) selectPort(port.guid);
                }
              };

              const handleHover = () => {
                if (hoverPort) hoverPort(port.guid);
              };

              const handleLeave = () => {
                if (clearHover) clearHover();
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
                          applyDiff({
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
                            updatePort(port.guid, { interface: value });
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
                            updatePort(port.guid, { description: value });
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
                            updatePort(port.guid, { t: value });
                          }}
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
                              updatePort(port.guid, { point: { x: value } });
                            }}
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
                              updatePort(port.guid, { point: { y: value } });
                            }}
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
                              updatePort(port.guid, { point: { z: value } });
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
                            value={port.direction.x}
                            onChange={(value: number) => {
                              updatePort(port.guid, { direction: { x: value } });
                            }}
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
                              updatePort(port.guid, { direction: { y: value } });
                            }}
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
                              updatePort(port.guid, { direction: { z: value } });
                            }}
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
                            updatePort(port.guid, {
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

  const updateAuthors = (authors: string[]) => {
    kitCommands?.updateType(type.guid, { authors: authors.map((a) => ({ guid: a })) });
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
              const newAuthorGuid = guid();
              kitCommands?.createAuthor({
                guid: newAuthorGuid,
                name: "",
                email: "",
              });
              updateAuthors([...(type.authors || []).map((a) => a.guid), newAuthorGuid]);
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
              updateAuthors(arrayMove(type.authors!, oldIndex, newIndex).map((a) => a.guid));
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
                      updateAuthors((type.authors || []).filter((_, i: number) => i !== index).map((a) => a.guid));
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
                        kitCommands?.updateAuthor(item.guid, { name: e.target.value });
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
                        kitCommands?.updateAuthor(item.guid, { email: e.target.value });
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

  const applyDiff = (diff: any) => {
    kitCommands?.updateType(type.guid, diff);
  };

  const updateAttribute = (id: string, attributeDiff: any) => {
    applyDiff({
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
              applyDiff({
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
              applyDiff({
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
                      applyDiff({
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
                        updateAttribute(attribute.guid, { key: e.target.value });
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
                        updateAttribute(attribute.guid, { value: e.target.value });
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
                        updateAttribute(attribute.guid, { definition: e.target.value });
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

  const updatePort = (id: string, portDiff: any) => {
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
    kitCommands?.updateType(type.guid, {
      ports: {
        updated: [{ port: { guid: id }, diff }],
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
            value={port.interface?.guid || ""}
            placeholderId="semio.sketchpad.app.type.portInterfacePlaceholder.label"
            onLazyChange={(value: string) => {
              updatePort(port.guid, { interface: value ? { guid: value } : undefined });
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
              updatePort(port.guid, { description: value });
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
              updatePort(port.guid, { t: value });
            }}
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
                updatePort(port.guid, { point: { x: value } });
              }}
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
                updatePort(port.guid, { point: { y: value } });
              }}
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
                updatePort(port.guid, { point: { z: value } });
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
              value={port.direction.x}
              onChange={(value: number) => {
                updatePort(port.guid, { direction: { x: value } });
              }}
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
                updatePort(port.guid, { direction: { y: value } });
              }}
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
                updatePort(port.guid, { direction: { z: value } });
              }}
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
            value={((port as any).compatibleInterfaces || []).join(", ")}
            placeholderId="semio.sketchpad.app.type.portCompatibleInterfacesPlaceholder.label"
            onLazyChange={(value: string) => {
              updatePort(port.guid, {
                compatibleInterfaces: value
                  .split(",")
                  .map((interface_) => interface_.trim())
                  .filter((interface_) => interface_),
              } as any);
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
      kitCommands?.updateType(type.guid, {
        ports: {
          updated: [{ port: { guid: port.guid }, diff }],
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

const TypeSettingsContent: FC = () => {
  const [theme, setTheme, canSetTheme] = useTheme();
  const [language, setLanguage, canSetLanguage] = useLanguage();
  const [device, setDevice, canSetDevice] = useDevice();
  const [expertise, setExpertise, canSetExpertise] = useExpertise();
  const [mode, setMode, canSetMode] = useMode();

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
            onValueChange={(value: string) => setTheme?.(value as Theme)}
            showLabel
            kind="single"
            disabled={!canSetTheme}
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
          <Select id="semio.sketchpad.settings.language" value={language || "en"} onValueChange={(value: string) => setLanguage?.(value)} showLabel disabled={!canSetLanguage}>
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
            id="semio.sketchpad.settings.device"
            value={typeof device === "object" ? "desktop" : device}
            onValueChange={(value: string) => setDevice?.(value as "desktop" | "tablet")}
            showLabel
            kind="single"
            disabled={!canSetDevice}
            items={[
              { value: "desktop", id: "semio.sketchpad.settings.device.desktop", icon: <MousePointerIcon className="size-small" /> },
              { value: "tablet", id: "semio.sketchpad.settings.device.tablet", icon: <HandIcon className="size-small" /> },
            ]}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <ToggleGroup
            id="semio.sketchpad.settings.expertise"
            value={expertise}
            onValueChange={(value: string) => setExpertise?.(value as Expertise)}
            showLabel
            kind="single"
            disabled={!canSetExpertise}
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
            onValueChange={(value: string) => setMode?.(value as Mode)}
            showLabel
            kind="single"
            disabled={!canSetMode}
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
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kit = kitScope?.guid;
  const type = typeScope?.guid;
  const [activeTool, , canSetActiveTool] = useTypeAppActiveTool();
  const [setActiveTool] = useTypeAppSetActiveTool();

  if (!kit || !type || !canSetActiveTool) return null;

  return <ToolGroup tools={getTypeTools()} activeTool={activeTool} onToolChange={(tool) => setActiveTool && setActiveTool(tool as ToolKind)} />;
};

// #endregion Tools

// #endregion Canvas

// #region App

const App: FC = () => {
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const appType = useAppType();
  const [setActiveTool] = useTypeAppSetActiveTool();
  // PERF: Use targeted hook instead of full state subscription
  const [activeTool] = useTypeAppActiveTool();
  const [selection] = useTypeAppSelection();
  const [isDragOver, setIsDragOver] = useState(false);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (activeTool === ToolKind.SELECTION_NORMAL) {
        if (e.shiftKey && !e.ctrlKey && !e.metaKey) {
          if (setActiveTool) setActiveTool(ToolKind.SELECTION_ADDITIVE);
        } else if ((e.ctrlKey || e.metaKey) && !e.shiftKey) {
          if (setActiveTool) setActiveTool(ToolKind.SELECTION_SUBTRACTIVE);
        }
      }
    };
    const handleKeyUp = (e: KeyboardEvent) => {
      if (activeTool === ToolKind.SELECTION_ADDITIVE && !e.shiftKey) {
        if (setActiveTool) setActiveTool(ToolKind.SELECTION_NORMAL);
      } else if (activeTool === ToolKind.SELECTION_SUBTRACTIVE && !e.ctrlKey && !e.metaKey) {
        if (setActiveTool) setActiveTool(ToolKind.SELECTION_NORMAL);
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
  const [setSelectedModel] = useTypeAppSetSelectedModel();

  // Add settings sections
  useEffect(() => {
    if (appType !== "type") return;

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
      content: () => <TypeSettingsContent />,
    });

    // Add global Sketchpad settings (least specific)
    addSection("settings", {
      id: "semio.sketchpad.settings",
      specificity: 0,
      order: 0,
      content: () => <TypeSettingsContent />,
    });

    return () => {
      removeSection("settings", "semio.sketchpad.app.type.settings");
      removeSection("settings", "semio.sketchpad.app.kit.settings");
      removeSection("settings", "semio.sketchpad.settings");
    };
  }, [appType, addSection, removeSection]);

  // Handle file drops
  useEffect(() => {
    if (appType !== "type") return;

    const handleDrop = async (event: DragEvent) => {
      event.preventDefault();
      event.stopPropagation();
      setIsDragOver(false);

      const files = event.dataTransfer?.files;
      if (!files || files.length === 0 || !type || !kitCommands || !setSelectedModel) return;

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
        await kitCommands.addFile(newFile, file);

        // Add model to type
        await kitCommands.updateType(type.guid, {
          models: {
            added: [newModel],
          },
        });

        // Select the new model
        setSelectedModel(newModelGuid);
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
  }, [appType, type, kitCommands, setSelectedModel]);

  // Window layout is managed by XState - persistedWindowLayout comes from the machine
  const persistedWindowLayout = useTypeApp((s) => s?.windowLayout);

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

  const handleLayoutChange = useCallback((_config: any) => {
    // TODO: Add TYPE.SET_WINDOW_LAYOUT event to XState machine
    // Layout changes are currently not persisted via XState
  }, []);

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
  const appType = useAppType();
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();

  useEffect(() => {
    if (appType !== "type") return;

    addSection("toolbar", {
      id: "semio.sketchpad.app.type.tools",
      specificity: 20,
      order: 0,
      content: <ToolsToggleGroup />,
    });

    return () => {
      removeSection("toolbar", "semio.sketchpad.app.type.tools");
    };
  }, [appType, addSection, removeSection]);

  useTypeAppInitialize();

  const transaction = useKitTransaction();
  return (
    <TransactionProvider transaction={transaction}>
      <App />
    </TransactionProvider>
  );
};

// Initialize Type app state in XState when entering the app
function useTypeAppInitialize() {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const hasInitialized = useRef(false);

  useLayoutEffect(() => {
    if (hasInitialized.current || !kitGuid || !typeGuid) return;

    actor.send({
      type: "TYPE.INIT",
      kitGuid,
      typeGuid,
      state: {
        panelVisibility: { toolbar: true, workbench: false, details: false, chat: false, settings: false },
        selection: undefined,
        hover: undefined,
        focusedPort: undefined,
        camera: undefined,
        activeTool: ToolKind.SELECTION_NORMAL,
        fullscreenWindow: SketchpadTypeAppFullscreenWindow.None,
        selectedModelTags: [],
        transaction: {
          isTransactionActive: false,
          currentTransactionStack: [],
          pastTransactionStack: [],
          redoStack: [],
        },
      },
    });
    hasInitialized.current = true;
  }, [actor, kitGuid, typeGuid]);
}

export default TypeApp;

// #endregion App

// #region Footer

export const TypeAppFooter: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const appType = useAppType();
  const type = useType() as Type | undefined;
  const tags = useKitTags();
  const [selectedModelTags] = useTypeAppSelectedModelTags();
  const [addModelTag] = useTypeAppAddModelTag();
  const [removeModelTag] = useTypeAppRemoveModelTag();

  // Controller refs for callbacks to avoid recreating them in useEffect
  const addModelTagRef = useRef(addModelTag);
  const removeModelTagRef = useRef(removeModelTag);
  const selectedModelTagsRef = useRef(selectedModelTags);

  useEffect(() => {
    addModelTagRef.current = addModelTag;
    removeModelTagRef.current = removeModelTag;
    selectedModelTagsRef.current = selectedModelTags;
  }, [addModelTag, removeModelTag, selectedModelTags]);

  // Get all unique tag guids and names from the type's models
  const { allModelTagGuids, tagNameMap } = useMemo(() => {
    if (!type?.models) return { allModelTagGuids: [], tagNameMap: new Map<string, string>() };
    const tagGuids = new Set<string>();
    const nameMap = new Map<string, string>();
    type.models.forEach((model) => {
      model.tags?.forEach((tag) => {
        tagGuids.add(tag.guid);
      });
    });
    // Fallback to kit tags for any missing names
    tags.forEach((tag) => {
      if (!nameMap.has(tag.guid)) {
        nameMap.set(tag.guid, tag.name);
      }
    });
    return { allModelTagGuids: Array.from(tagGuids), tagNameMap: nameMap };
  }, [type?.models, tags]);

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
        text: tagName,
        className: isSelected ? "bg-active-base text-active-foreground" : "text-muted-foreground hover:text-foreground",
        onClick: () => {
          // Use refs in onClick to get current values at click time
          const currentSelected = isTagSelected(tagGuid);
          if (currentSelected) {
            if (removeModelTagRef.current) removeModelTagRef.current(tagGuid);
          } else {
            if (addModelTagRef.current) addModelTagRef.current(tagGuid);
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
    createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
    createPanelDefinition(PanelKind.CHAT, "semio.sketchpad.navbar.panelToggle.chat.show"),
    createPanelDefinition(PanelKind.SETTINGS, "semio.sketchpad.navbar.panelToggle.settings.show"),
  ],
  matchesPath: (pathParts: string[]) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 4 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]) && pathParts[2] === "types" && isUuidPattern(pathParts[3]);
  },
  order: 30,
};

// #endregion Config
