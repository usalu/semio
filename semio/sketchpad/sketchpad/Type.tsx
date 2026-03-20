// #region Header

// js/semio/sketchpad/Type.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

// #region Imports

import { Author, AuthorId, Camera, Connector, Coord, findModel, guid, Guid, Kit, Model, Point, selectBestModel, File as SemioFile, toSemioRotation, toThreeRotation, Type, TypeDiff, Vector } from "@semio/js/semio";
import React, { createContext, FC, Suspense, useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import type { ThreeEvent } from "../../../.elements/ui";
import {
  arrayMove,
  BasicChatPanel,
  Geometry,
  Input,
  Line,
  OBJLoader,
  Ring,
  Scene as SceneComponent,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Slider,
  SortableTreeItems,
  Sphere,
  Stepper,
  Textarea,
  THREE,
  Toggle,
  ToggleGroup,
  ToolbarGroup,
  TransactionProvider,
  Tree,
  TreeItem,
  TreeRow,
  TreeStateProvider,
  useFBX,
  useGLTF,
  useHotkeys,
  useLoader,
  useSearchParams,
  useXStateSelector as useSelector,
} from "../../../.elements/ui";
import { useLabel } from "../i18n";
import type { AppWindowConfig, HookResult, KitCommandContext, KitDiffAppEdit, PanelDefinition, PanelVisibility, Tool, ToolRenderContext, TypeAppId } from "./shared";
import {
  AppConfig,
  applySelectionComposition,
  AppPlugin,
  conditionalHookResult,
  createKeyedTransactionHandlers,
  createPanelDefinition,
  EMPTY_PANEL_VISIBILITY,
  Expertise,
  isSelectionToolKind,
  Mode,
  PanelKind,
  readonlyHookResult,
  registerAppPlugin,
  registerEventHandler,
  registerKeyedAppEventHandlers,
  resolveSelectionCompositionKind,
  Theme,
  ToolKind,
  toSelectionToolKind,
} from "./shared";
import {
  Canvas,
  CollaborativeKitStore,
  createDefaultTypeAppState,
  createTypeActiveToolSelector,
  createTypeAppSelector,
  createTypeCameraSelector,
  createTypeFocusedConnectorSelector,
  createTypeHoverSelector,
  createTypeOthersSelector,
  createTypePanelVisibilitySelector,
  createTypeSelectedModelTagsSelector,
  createTypeSelectionSelector,
  KitScopeProvider,
  LayoutCanvas,
  TypeAppFullscreenWindow as SketchpadTypeAppFullscreenWindow,
  TypeScopeProvider,
  useAddFooterItem,
  useAddPanelSection,
  useAddSidePanelTab,
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
  useRemoveSidePanelTab,
  useSketchpadActor,
  useTheme,
  useTooltip,
  useType,
  useTypeAppXState,
  useTypeScope,
} from "./Sketchpad";

/**
 * [👤semio📚js🗃️sketchpad💻type🔖imports🪨kitsectionlazy](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/d/i/KitSectionLazy)
 * KitSectionLazy holds the data fields for a KitSectionLazy record.
 **/
const KitSectionLazy = React.lazy(() => import("./Kit").then((module) => ({ default: module.KitSection })));

import { AddIcon, AwardIcon, ChatIcon, CheckIcon, CodeIcon, ConnectorIcon, HandIcon, MonitorIcon, MoonIcon, MousePointerIcon, RemoveIcon, SceneIcon, SelectToolIcon, SettingsIcon, SunIcon, TutorialIcon, UserIcon } from "@semio/assets";

// #endregion Imports

// #region Internal State Management

// [👤semio📚js🗃️sketchpad💻typetsx🔖internalstatemanagement](repo://section/SEMIO/JS/SKETCHPAD/TYPE.TSX/INTERNAL-STATE-MANAGEMENT)
// TypeApp state interfaces, enums, and diffing types. MUST define all shared state shapes.

/**
 * Selection state holding selected connector and model GUIDs.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖internalstatemanagement🛠️typeappselection](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Internal%20State%20Management/d/i/TypeAppSelection)
 **/
export interface TypeAppSelection {
  connectors?: Guid[];
  models?: Guid[];
}
/**
 * Diff for added and removed connector selections.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖internalstatemanagement🛠️typeappselectionportsdiff](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Internal%20State%20Management/d/i/TypeAppSelectionPortsDiff)
 **/
export interface TypeAppSelectionPortsDiff {
  added?: Guid[];
  removed?: Guid[];
}
/**
 * Diff for added and removed model selections.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖internalstatemanagement🛠️typeappselectionmodelsdiff](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Internal%20State%20Management/d/i/TypeAppSelectionModelsDiff)
 **/
export interface TypeAppSelectionModelsDiff {
  added?: Guid[];
  removed?: Guid[];
}
/**
 * Combined selection diff for connectors and models.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖internalstatemanagement🛠️typeappselectiondiff](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Internal%20State%20Management/d/i/TypeAppSelectionDiff)
 **/
export interface TypeAppSelectionDiff {
  connectors?: TypeAppSelectionPortsDiff;
  models?: TypeAppSelectionModelsDiff;
}
/**
 * Fullscreen window modes for the TypeApp.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖internalstatemanagement🛠️typeappfullscreenwindow](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Internal%20State%20Management/d/i/TypeAppFullscreenWindow)
 **/
export enum TypeAppFullscreenWindow {
  None = "none",
  Connectors = "connectors",
  Models = "models",
}

/**
 * Window kind identifiers for the TypeApp layout.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖internalstatemanagement🛠️typeappwindowkind](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Internal%20State%20Management/d/i/TypeAppWindowKind)
 **/
export enum TypeAppWindowKind {
  Scene = "scene",
}
/**
 * Presence state including cursor position and camera.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖internalstatemanagement🛠️typeapppresence](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Internal%20State%20Management/d/i/TypeAppPresence)
 **/
export interface TypeAppPresence {
  cursor?: Coord;
  camera?: Camera;
}
/**
 * Hover state tracking which connector or model is hovered.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖internalstatemanagement🛠️typeapphover](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Internal%20State%20Management/d/i/TypeAppHover)
 **/
export interface TypeAppHover {
  connector?: Guid;
  model?: Guid;
}
/**
 * Presence state of another user including their name.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖internalstatemanagement🛠️typeapppresenceother](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Internal%20State%20Management/d/i/TypeAppPresenceOther)
 **/
export interface TypeAppPresenceOther extends TypeAppPresence {
  name: string;
}
/**
 * Diff object describing partial changes to TypeApp state.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖internalstatemanagement🛠️typeappdiff](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Internal%20State%20Management/d/i/TypeAppDiff)
 **/
export interface TypeAppDiff {
  selection?: TypeAppSelectionDiff;
  presence?: TypeAppPresence;
  hover?: TypeAppHover;
  fullscreenWindow?: TypeAppFullscreenWindow;
  panelVisibility?: Partial<PanelVisibility>;
  activeTool?: ToolKind;
  camera?: Camera;
  focusedConnectorGuid?: Guid | null;
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
  focusedConnectorGuid?: Guid;
  selectedModelGuid?: Guid;
  selectedModelTags?: string[];
  windowLayout?: any;
}

/**
 * Context passed to TypeApp commands with kit, typeApp state, and target GUID.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖internalstatemanagement🛠️typeappcommandcontext](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Internal%20State%20Management/d/i/TypeAppCommandContext)
 **/
export interface TypeAppCommandContext extends KitCommandContext {
  typeApp: TypeAppState;
  Guid: Guid;
}
/**
 * Result of a TypeApp command containing optional app and type diffs.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖internalstatemanagement🛠️typeappcommandresult](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Internal%20State%20Management/d/i/TypeAppCommandResult)
 **/
export interface TypeAppCommandResult {
  diff?: TypeAppDiff;
  typeDiff?: TypeDiff;
}

/**
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖internalstatemanagement🪨emptytypeselection](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Internal%20State%20Management/d/i/EMPTY_TYPE_SELECTION)
 * EMPTY_TYPE_SELECTION holds the data fields for a EMPTY_TYPE_SELECTION record.
 **/
const EMPTY_TYPE_SELECTION: TypeAppSelection = {};
/**
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖internalstatemanagement🪨emptymodeltagarray](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Internal%20State%20Management/d/i/EMPTY_MODEL_TAG_ARRAY)
 * EMPTY_MODEL_TAG_ARRAY holds the data fields for a EMPTY_MODEL_TAG_ARRAY record.
 **/
const EMPTY_MODEL_TAG_ARRAY: string[] = [];

// #endregion Internal State Management

// #region Type App Plugin Registration

// [👤semio📚js🗃️sketchpad💻type🔖imports🔖typeapppluginregistration](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Type%20App%20Plugin%20Registration)
// Plugin registration and XState event handlers for the TypeApp. MUST register all event handlers at module load.

/**
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖typeapppluginregistration🪨typeappplugin](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Type%20App%20Plugin%20Registration/d/i/typeAppPlugin)
 * typeAppPlugin holds the data fields for a typeAppPlugin record.
 **/
const typeAppPlugin: AppPlugin = {
  id: "type",
  namespace: "TYPE",
  machine: {
    actions: {},
    guards: {},
    eventHandlers: {},
    selectors: {},
    createDefaultState: (): TypeAppState => ({
      panelVisibility: { toolbar: true, leftSidePanel: true, rightSidePanel: true, details: true },
      activeTool: ToolKind.SELECTION_NORMAL,
      fullscreenWindow: TypeAppFullscreenWindow.None,
      selection: undefined,
      hover: undefined,
      presence: undefined,
      others: [],
      camera: undefined,
      focusedConnectorGuid: undefined,
      selectedModelGuid: undefined,
      selectedModelTags: [],
      windowLayout: undefined,
    }),
  },
};

/**
 * typeAppEventConfig holds the data fields for a typeAppEventConfig record.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖typeapppluginregistration🪨typeappeventconfig](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Type%20App%20Plugin%20Registration/d/i/typeAppEventConfig)
 **/
const typeAppEventConfig = {
  namespace: "TYPE",
  appKey: "typeApps" as const,
  getKey: (event: any) => `${event.kitGuid}:${event.typeGuid}`,
  createDefaultState: createDefaultTypeAppState,
};

if (typeof window !== "undefined") {
  registerAppPlugin(typeAppPlugin);

  registerKeyedAppEventHandlers(typeAppEventConfig, (event) => event.hover);

  registerEventHandler("TYPE.FOCUS_CONNECTOR", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      return { typeApps: { ...apps, [key]: { ...app, focusedConnector: event.connectorGuid } } };
    },
  });

  registerEventHandler("TYPE.CLEAR_FOCUS", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      return { typeApps: { ...apps, [key]: { ...app, focusedConnector: undefined } } };
    },
  });

  registerEventHandler("TYPE.SELECT_CONNECTOR", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      const connectors = [...(app.selection?.connectors || [])];
      if (!connectors.includes(event.connectorGuid)) connectors.push(event.connectorGuid);
      return { typeApps: { ...apps, [key]: { ...app, selection: { ...app.selection, connectors } } } };
    },
  });

  registerEventHandler("TYPE.DESELECT_CONNECTOR", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      const connectors = (app.selection?.connectors || []).filter((p: Guid) => p !== event.connectorGuid);
      return { typeApps: { ...apps, [key]: { ...app, selection: { ...app.selection, connectors } } } };
    },
  });

  registerEventHandler("TYPE.SELECT_MODEL", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      const models = [...(app.selection?.models || [])];
      if (!models.includes(event.modelGuid)) models.push(event.modelGuid);
      return { typeApps: { ...apps, [key]: { ...app, selection: { ...app.selection, models } } } };
    },
  });

  registerEventHandler("TYPE.DESELECT_MODEL", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      const models = (app.selection?.models || []).filter((m: Guid) => m !== event.modelGuid);
      return { typeApps: { ...apps, [key]: { ...app, selection: { ...app.selection, models } } } };
    },
  });

  registerEventHandler("TYPE.HOVER_CONNECTOR", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      return { typeApps: { ...apps, [key]: { ...app, hover: { connector: event.connectorGuid } } } };
    },
  });

  registerEventHandler("TYPE.HOVER_MODEL", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      return { typeApps: { ...apps, [key]: { ...app, hover: { model: event.modelGuid } } } };
    },
  });

  registerEventHandler("TYPE.SET_SELECTED_MODEL", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      return { typeApps: { ...apps, [key]: { ...app, selectedModelGuid: event.modelGuid } } };
    },
  });

  registerEventHandler("TYPE.SET_MODEL_TAGS", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      return { typeApps: { ...apps, [key]: { ...app, selectedModelTags: event.tags } } };
    },
  });

  registerEventHandler("TYPE.ADD_MODEL_TAG", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      const tags = [...(app.selectedModelTags || [])];
      if (!tags.includes(event.tag)) tags.push(event.tag);
      return { typeApps: { ...apps, [key]: { ...app, selectedModelTags: tags } } };
    },
  });

  registerEventHandler("TYPE.REMOVE_MODEL_TAG", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      const tags = (app.selectedModelTags || []).filter((t: string) => t !== event.tag);
      return { typeApps: { ...apps, [key]: { ...app, selectedModelTags: tags } } };
    },
  });

  registerEventHandler("TYPE.CLEAR_MODEL_TAGS", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      return { typeApps: { ...apps, [key]: { ...app, selectedModelTags: [] } } };
    },
  });

  registerEventHandler("TYPE.SELECT_MODEL_TAG", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      const tags = app.selectedModelTags || [];
      if (tags.includes(event.tagGuid)) return {};
      return { typeApps: { ...apps, [key]: { ...app, selectedModelTags: [...tags, event.tagGuid] } } };
    },
  });

  registerEventHandler("TYPE.DESELECT_MODEL_TAG", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      const tags = app.selectedModelTags || [];
      return { typeApps: { ...apps, [key]: { ...app, selectedModelTags: tags.filter((g: Guid) => g !== event.tagGuid) } } };
    },
  });

  registerEventHandler("TYPE.SELECT_ALL", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      return { typeApps: { ...apps, [key]: { ...app, selection: { connectors: [], models: [] } } } };
    },
  });

  registerEventHandler("TYPE.DESELECT_ALL", {
    action: (context: any, event: any) => {
      const key = typeAppEventConfig.getKey(event);
      const apps = context.typeApps || {};
      const app = apps[key] || createDefaultTypeAppState();
      return { typeApps: { ...apps, [key]: { ...app, selection: undefined } } };
    },
  });
  createKeyedTransactionHandlers({
    appKey: "typeApps",
    keyFields: ["kitGuid", "typeGuid"],
    createDefaultState: createDefaultTypeAppState,
  });
}

// #endregion Type App Plugin Registration

// #region XState Hooks

// [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks)
// React hooks that read and write TypeApp XState machine state. MUST use memoized selectors for performance.

/**
 * Selects a slice of TypeApp state for the current kit-type scope.
 *MUST return null when no kit or type scope is available.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🛠️usetypeapp](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/useTypeApp)
 **/
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

/**
 * Returns the current connector and model selection for the TypeApp.
 *MUST return a conditionalHookResult with setter availability.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🛠️usetypeappselection](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/useTypeAppSelection)
 **/
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

/**
 * Returns the current panel visibility state for the TypeApp.
 *MUST return a conditionalHookResult with setter availability.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🛠️usetypeapppanelvisibility](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/useTypeAppPanelVisibility)
 **/
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

/**
 * Returns the list of other users' presence states.
 *MUST return a readonly hook result.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🛠️usetypeappothers](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/useTypeAppOthers)
 **/
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

/**
 * Returns the current camera state for the TypeApp.
 *MUST return a conditionalHookResult with setter availability.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🛠️usetypeappcamera](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/useTypeAppCamera)
 **/
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

/**
 * Returns the GUID of the focused connector for camera targeting.
 *MUST return a conditionalHookResult with setter availability.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🛠️usetypeappfocusedconnectorguid](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/useTypeAppFocusedConnectorGuid)
 **/
export function useTypeAppFocusedConnectorGuid(): HookResult<Guid | undefined> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypeFocusedConnectorSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const value = useSelector(actor, selector);
  const canSetEvent = useMemo(() => ({ type: "TYPE.FOCUS_CONNECTOR" as const, kitGuid, typeGuid, connectorGuid: "" }), [kitGuid, typeGuid]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (connectorGuid: Guid | undefined) => {
      if (connectorGuid) {
        actor.send({ type: "TYPE.FOCUS_CONNECTOR", kitGuid, typeGuid, connectorGuid });
      } else {
        actor.send({ type: "TYPE.CLEAR_FOCUS", kitGuid, typeGuid });
      }
    };
  }, [actor, kitGuid, typeGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

/**
 * Returns the current hover state indicating which connector or model is hovered.
 *MUST return a conditionalHookResult with setter availability.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🛠️usetypeapphover](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/useTypeAppHover)
 **/
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
      if (hover?.connector) {
        actor.send({ type: "TYPE.HOVER_CONNECTOR", kitGuid, typeGuid, connectorGuid: hover.connector });
      } else if (hover?.model) {
        actor.send({ type: "TYPE.HOVER_MODEL", kitGuid, typeGuid, modelGuid: hover.model });
      } else {
        actor.send({ type: "TYPE.CLEAR_HOVER", kitGuid, typeGuid });
      }
    };
  }, [actor, kitGuid, typeGuid, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

/**
 * Returns the currently active tool kind for the TypeApp.
 *MUST return a conditionalHookResult with setter availability.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🛠️usetypeappactivetool](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/useTypeAppActiveTool)
 **/
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

/**
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks✂️transaction](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/Transaction)
 * Transaction holds the data fields for a Transaction record.
 **/
interface Transaction {
  start?: () => void;
  finalize?: () => void;
  abort?: () => void;
}

/**
 * Returns a transaction object with start, finalize, and abort methods.
 *MUST return stub methods until XState transaction events are implemented.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🛠️usetypeapptransaction](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/useTypeAppTransaction)
 **/
export function useTypeAppTransaction(_id?: TypeAppId): Transaction {
  // TODO: Implement transaction via XState events
  return {
    start: () => {},
    finalize: () => {},
    abort: () => {},
  };
}

/**
 * Returns an object of command functions for sending TypeApp XState events.
 *MUST return no-op functions when no kit or type scope is available.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🛠️usetypeappcommands](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/useTypeAppCommands)
 **/
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
        selectConnector: noOp,
        deselectConnector: noOp,
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
      focusPort: (connectorGuid: Guid) => actor.send({ type: "TYPE.FOCUS_CONNECTOR", kitGuid, typeGuid, connectorGuid }),
      clearFocus: () => actor.send({ type: "TYPE.CLEAR_FOCUS", kitGuid, typeGuid }),
      setActiveTool: (tool: ToolKind) => actor.send({ type: "TYPE.SET_ACTIVE_TOOL", kitGuid, typeGuid, tool }),
      selectConnector: (connectorGuid: Guid) => actor.send({ type: "TYPE.SELECT_CONNECTOR", kitGuid, typeGuid, connectorGuid }),
      deselectConnector: (connectorGuid: Guid) => actor.send({ type: "TYPE.DESELECT_CONNECTOR", kitGuid, typeGuid, connectorGuid }),
      selectModel: (modelGuid: Guid) => actor.send({ type: "TYPE.SELECT_MODEL", kitGuid, typeGuid, modelGuid }),
      deselectModel: (modelGuid: Guid) => actor.send({ type: "TYPE.DESELECT_MODEL", kitGuid, typeGuid, modelGuid }),
      hoverPort: (connectorGuid: Guid) => actor.send({ type: "TYPE.HOVER_CONNECTOR", kitGuid, typeGuid, connectorGuid }),
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

/**
 * Returns whether a specific connector is selected and a setter to toggle it.
 *MUST return a conditionalHookResult with setter availability.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🛠️usetypeappisportselected](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/useTypeAppIsPortSelected)
 **/
export function useTypeAppIsPortSelected(connectorId: string): HookResult<boolean> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypeSelectionSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const selection = useSelector(actor, selector);
  const value = selection?.connectors?.includes(connectorId) ?? false;
  const canSetEvent = useMemo(() => ({ type: "TYPE.SELECT_CONNECTOR" as const, kitGuid, typeGuid, connectorGuid: connectorId }), [kitGuid, typeGuid, connectorId]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (isSelected: boolean) => {
      if (isSelected) {
        actor.send({ type: "TYPE.SELECT_CONNECTOR", kitGuid, typeGuid, connectorGuid: connectorId });
      } else {
        actor.send({ type: "TYPE.DESELECT_CONNECTOR", kitGuid, typeGuid, connectorGuid: connectorId });
      }
    };
  }, [actor, kitGuid, typeGuid, connectorId, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

/**
 * Returns whether a specific connector is hovered and a setter to toggle it.
 *MUST return a conditionalHookResult with setter availability.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🛠️usetypeappisporthovered](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/useTypeAppIsPortHovered)
 **/
export function useTypeAppIsPortHovered(connectorId: string): HookResult<boolean> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypeHoverSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const hover = useSelector(actor, selector);
  const value = hover?.connector === connectorId;
  const canSetEvent = useMemo(() => ({ type: "TYPE.HOVER_CONNECTOR" as const, kitGuid, typeGuid, connectorGuid: connectorId }), [kitGuid, typeGuid, connectorId]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setter = useMemo(() => {
    if (!canSet) return undefined;
    return (isHovered: boolean) => {
      if (isHovered) {
        actor.send({ type: "TYPE.HOVER_CONNECTOR", kitGuid, typeGuid, connectorGuid: connectorId });
      } else {
        actor.send({ type: "TYPE.CLEAR_HOVER", kitGuid, typeGuid });
      }
    };
  }, [actor, kitGuid, typeGuid, connectorId, canSet]);
  return conditionalHookResult(canSet, value, setter);
}

/**
 * Returns the GUID of the selected model for mesh display.
 *MUST return a conditionalHookResult with setter availability.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🛠️usetypeappselectedmodelguid](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/useTypeAppSelectedModelGuid)
 **/
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

/**
 * Returns the selected model tags used for model filtering.
 *MUST return a conditionalHookResult with setter availability.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🛠️usetypeappselectedmodeltags](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/useTypeAppSelectedModelTags)
 **/
export function useTypeAppSelectedModelTags(): HookResult<string[]> {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const selector = useMemo(() => createTypeSelectedModelTagsSelector(kitGuid, typeGuid), [kitGuid, typeGuid]);
  const value = useSelector(actor, selector) ?? EMPTY_MODEL_TAG_ARRAY;
  const canSetEvent = useMemo(() => ({ type: "TYPE.SET_MODEL_TAGS" as const, kitGuid, typeGuid, tags: [] as string[] }), [kitGuid, typeGuid]);
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

// [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks)
// Convenience React hooks wrapping state hooks into single-purpose actions. MUST return action-canAct tuples.

/**
 * Tuple type for action hooks returning the action callback and a can-act boolean.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️actionhookresult](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/ActionHookResult)
 **/
export type ActionHookResult<TArgs extends any[]> = readonly [action: ((...args: TArgs) => void) | undefined, canAct: boolean];

/**
 * Selects a single connector replacing the current selection.
 *MUST clear model selection when selecting a connector.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeappselectconnector](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppSelectConnector)
 **/
export function useTypeAppSelectConnector(): ActionHookResult<[connectorGuid: string]> {
  const [, setSelection, canSetSelection] = useTypeAppSelection();
  const [selection] = useTypeAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (connectorGuid: string) => setSelection({ ...selection, connectors: [connectorGuid], models: [] });
  }, [setSelection, canSetSelection, selection]);
  return [action, canSetSelection];
}

/**
 * Removes a connector from the current selection.
 *MUST filter the connector GUID from the selection array.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeappdeselectconnector](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppDeselectConnector)
 **/
export function useTypeAppDeselectConnector(): ActionHookResult<[connectorGuid: string]> {
  const [, setSelection, canSetSelection] = useTypeAppSelection();
  const [selection] = useTypeAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (connectorGuid: string) => {
      const currentConnectors = selection?.connectors ?? [];
      setSelection({ ...selection, connectors: currentConnectors.filter((p) => p !== connectorGuid) });
    };
  }, [setSelection, canSetSelection, selection]);
  return [action, canSetSelection];
}

/**
 * Sets the hover state to a specific connector.
 *MUST delegate to the hover state setter.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeapphoverport](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppHoverPort)
 **/
export function useTypeAppHoverPort(): ActionHookResult<[connectorGuid: string]> {
  const [, setHover, canSetHover] = useTypeAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (connectorGuid: string) => setHover({ connector: connectorGuid });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

/**
 * Sets the hover state to a specific model.
 *MUST delegate to the hover state setter.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeapphovermodel](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppHoverModel)
 **/
export function useTypeAppHoverModel(): ActionHookResult<[modelGuid: string]> {
  const [, setHover, canSetHover] = useTypeAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return (modelGuid: string) => setHover({ model: modelGuid });
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

/**
 * Clears the current hover state.
 *MUST set hover to undefined.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeappclearhover](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppClearHover)
 **/
export function useTypeAppClearHover(): ActionHookResult<[]> {
  const [, setHover, canSetHover] = useTypeAppHover();
  const action = useMemo(() => {
    if (!canSetHover || !setHover) return undefined;
    return () => setHover(undefined);
  }, [setHover, canSetHover]);
  return [action, canSetHover];
}

/**
 * Sets the focused connector GUID for camera targeting.
 *MUST delegate to the focused connector state setter.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeappfocusport](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppFocusPort)
 **/
export function useTypeAppFocusPort(): ActionHookResult<[connectorGuid: string]> {
  const [, setFocusedConnectorGuid, canSetFocusedConnectorGuid] = useTypeAppFocusedConnectorGuid();
  const action = useMemo(() => {
    if (!canSetFocusedConnectorGuid || !setFocusedConnectorGuid) return undefined;
    return (connectorGuid: string) => setFocusedConnectorGuid(connectorGuid);
  }, [setFocusedConnectorGuid, canSetFocusedConnectorGuid]);
  return [action, canSetFocusedConnectorGuid];
}

/**
 * Clears the focused connector allowing the camera to return to default.
 *MUST set focused connector GUID to undefined.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeappclearfocus](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppClearFocus)
 **/
export function useTypeAppClearFocus(): ActionHookResult<[]> {
  const [, setFocusedConnectorGuid, canSetFocusedConnectorGuid] = useTypeAppFocusedConnectorGuid();
  const action = useMemo(() => {
    if (!canSetFocusedConnectorGuid || !setFocusedConnectorGuid) return undefined;
    return () => setFocusedConnectorGuid(undefined);
  }, [setFocusedConnectorGuid, canSetFocusedConnectorGuid]);
  return [action, canSetFocusedConnectorGuid];
}

/**
 * Clears all connector and model selections.
 *MUST set both connector and model arrays to empty.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeappdeselectall](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppDeselectAll)
 **/
export function useTypeAppDeselectAll(): ActionHookResult<[]> {
  const [, setSelection, canSetSelection] = useTypeAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return () => setSelection({ connectors: [], models: [] });
  }, [setSelection, canSetSelection]);
  return [action, canSetSelection];
}

/**
 * Selects a single model replacing the current selection.
 *MUST clear connector selection when selecting a model.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeappselectmodel](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppSelectModel)
 **/
export function useTypeAppSelectModel(): ActionHookResult<[modelGuid: string]> {
  const [, setSelection, canSetSelection] = useTypeAppSelection();
  const [selection] = useTypeAppSelection();
  const action = useMemo(() => {
    if (!canSetSelection || !setSelection) return undefined;
    return (modelGuid: string) => setSelection({ ...selection, models: [modelGuid], connectors: [] });
  }, [setSelection, canSetSelection, selection]);
  return [action, canSetSelection];
}

/**
 * Removes a model from the current selection.
 *MUST filter the model GUID from the selection array.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeappdeselectmodel](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppDeselectModel)
 **/
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

/**
 * Sets the currently active tool kind.
 *MUST delegate to the active tool state setter.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeappsetactivetool](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppSetActiveTool)
 **/
export function useTypeAppSetActiveTool(): ActionHookResult<[tool: ToolKind]> {
  const [, setActiveTool, canSetActiveTool] = useTypeAppActiveTool();
  const action = useMemo(() => {
    if (!canSetActiveTool || !setActiveTool) return undefined;
    return (tool: ToolKind) => setActiveTool(tool);
  }, [setActiveTool, canSetActiveTool]);
  return [action, canSetActiveTool];
}

/**
 * Sets the camera state for the TypeApp scene.
 *MUST delegate to the camera state setter.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeappsetcamera](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppSetCamera)
 **/
export function useTypeAppSetCamera(): ActionHookResult<[camera: Camera]> {
  const [, setCamera, canSetCamera] = useTypeAppCamera();
  const action = useMemo(() => {
    if (!canSetCamera || !setCamera) return undefined;
    return (camera: Camera) => setCamera(camera);
  }, [setCamera, canSetCamera]);
  return [action, canSetCamera];
}

/**
 * Toggles a specific panel's visibility.
 *MUST flip the boolean value of the given panel key.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeapptogglepanel](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppTogglePanel)
 **/
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

/**
 * Adds a tag to the selected model tags if not already present.
 *MUST avoid duplicate tags.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeappaddmodeltag](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppAddModelTag)
 **/
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

/**
 * Removes a tag from the selected model tags.
 *MUST filter the tag string from the tags array.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeappremovemodeltag](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppRemoveModelTag)
 **/
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

/**
 * Sets the selected model GUID for mesh display.
 *MUST delegate to the selected model state setter.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🔖actionhooks🛠️usetypeappsetselectedmodel](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/s/Action%20Hooks/d/i/useTypeAppSetSelectedModel)
 **/
export function useTypeAppSetSelectedModel(): ActionHookResult<[modelGuid: string]> {
  const [, setSelectedModel, canSetSelectedModel] = useTypeAppSelectedModelGuid();
  const action = useMemo(() => {
    if (!canSetSelectedModel || !setSelectedModel) return undefined;
    return (modelGuid: string) => setSelectedModel(modelGuid);
  }, [setSelectedModel, canSetSelectedModel]);
  return [action, canSetSelectedModel];
}

//#endregion Action Hooks

/**
 * TypeAppScopeContext holds the data fields for a TypeAppScopeContext record.
 **/
// [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🪨typeappscopecontext](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/TypeAppScopeContext)
/**
 * TypeAppScopeContext holds the data fields for a TypeAppScopeContext record.
 **/
// [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🪨typeappscopecontext](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/TypeAppScopeContext)
/**
 * TypeAppScopeContext holds the data fields for a TypeAppScopeContext record.
 **/
// [👤semio📚js🗃️sketchpad💻type🔖xstatehooks🪨typeappscopecontext](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/XState%20Hooks/d/i/TypeAppScopeContext)
const TypeAppScopeContext = createContext<{ id: string } | undefined>(undefined);
/** TypeAppScopeProvider holds the data fields for a TypeAppScopeProvider record.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖xstatehooks🪨typeappscopeprovider](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/XState%20Hooks/d/i/TypeAppScopeProvider)
export const TypeAppScopeProvider = (props: { id: string; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(TypeAppScopeContext.Provider, { value }, props.children as any);
};
 **/
// [👤semio📚js🗃️sketchpad💻type🔖xstatehooks🪨usetypeappscope](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/XState%20Hooks/d/i/useTypeAppScope)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖xstatehooks🪨usetypeappscope](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/XState%20Hooks/d/i/useTypeAppScope)
 * useTypeAppScope holds the data fields for a useTypeAppScope record.

 **/
// #endregion XState Hooks

// #region Commands

// [👤semio📚js🗃️sketchpad💻type🔖imports🔖commands](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Commands)
// Command definitions for the TypeApp producing diffs from context. MUST return TypeAppCommandResult.

/**
 * Command map producing TypeApp and type diffs from command context and arguments.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖commands🪨commands](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Commands/d/i/commands)
 **/
export const commands = {
  "semio.typeApp.selectConnector": (context: TypeAppCommandContext, connectorGuid: Guid): TypeAppCommandResult => {
    const currentConnectors = context.typeApp.selection?.connectors || [];
    return {
      diff: {
        selection: {
          connectors: { added: [connectorGuid], removed: [] },
        },
      },
    };
  },
  "semio.typeApp.deselectConnector": (context: TypeAppCommandContext, connectorGuid: Guid): TypeAppCommandResult => {
    return {
      diff: {
        selection: {
          connectors: { added: [], removed: [connectorGuid] },
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
  "semio.typeApp.hoverPort": (context: TypeAppCommandContext, connectorGuid: Guid): TypeAppCommandResult => {
    return {
      diff: {
        hover: { connector: connectorGuid },
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
  "semio.typeApp.focusPort": (context: TypeAppCommandContext, connectorGuid: Guid): TypeAppCommandResult => {
    return {
      diff: {
        focusedConnectorGuid: connectorGuid,
      },
    };
  },
  "semio.typeApp.clearFocus": (context: TypeAppCommandContext): TypeAppCommandResult => {
    return {
      diff: {
        focusedConnectorGuid: null,
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
          connectors: { removed: context.typeApp.selection?.connectors || [] },
          models: { removed: context.typeApp.selection?.models || [] },
        },
      },
    };
  },
  "semio.typeApp.selectAll": (context: TypeAppCommandContext): TypeAppCommandResult => {
    const type = context.kit.types?.find((t) => t.guid === context.Guid);
    const allConnectors = type?.connectors?.map((p) => p.guid) || [];
    const allModels = type?.models?.map((r) => r.guid) || [];
    return {
      diff: {
        selection: {
          connectors: { added: allConnectors },
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

// [👤semio📚js🗃️sketchpad💻type🔖imports🔖scene](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Scene)
// Three.js scene components for connectors, meshes, and the 3D viewport. MUST render inside a React Three Fiber canvas.

/**
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖scene🪨gltfmesh](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Scene/d/i/GLTFMesh)
 * ConnectorVisual holds the data fields for a ConnectorVisual record.
 **/
const ConnectorVisual: FC<{ connector: Connector; isSelected: boolean; isHovered: boolean; onHover: () => void; onLeave: () => void; onClick: () => void; onDoubleClick: () => void }> = ({
  connector,
  isSelected,
  isHovered,
  onHover,
  onLeave,
  onClick,
  onDoubleClick,
}) => {
  const position = useMemo(() => {
    const semioPos = new THREE.Vector3(connector.point.x, connector.point.y, connector.point.z);
    const threePos = semioPos.applyMatrix4(toThreeRotation());
    return [threePos.x, threePos.y, threePos.z] as [number, number, number];
  }, [connector.point]);

  const direction = useMemo(() => {
    const semioDir = new THREE.Vector3(connector.direction.x, connector.direction.y, connector.direction.z);
    const threeDir = semioDir.applyMatrix4(toThreeRotation()).normalize();
    return [threeDir.x, threeDir.y, threeDir.z] as [number, number, number];
  }, [connector.direction]);

  const getComputedColor = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();
  const selectedColor = useMemo(() => getComputedColor("--active-base"), []);
  const hoverColor = useMemo(() => getComputedColor("--hover-base"), []);
  const defaultColor = useMemo(() => getComputedColor("--foreground"), []);

  const color = isSelected ? selectedColor : isHovered ? hoverColor : defaultColor;

  const arrowLength = 0.5;
  const endPoint = useMemo(() => [position[0] + direction[0] * arrowLength, position[1] + direction[1] * arrowLength, position[2] + direction[2] * arrowLength] as [number, number, number], [position, direction]);
  const points = useMemo(() => [position, endPoint], [position, endPoint]);

  const userData = useMemo(() => ({ id: connector.guid }), [connector.guid]);

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

// [👤semio📚js🗃️sketchpad💻type🔖scene🪨connectorpreview](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/ConnectorPreview)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖scene🪨connectorpreview](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/ConnectorPreview)
 * ConnectorPreview holds the data fields for a ConnectorPreview record.
 **/
const ConnectorPreview: FC<{ position: THREE.Vector3; normal: THREE.Vector3 }> = ({ position, normal }) => {
  const previewColor = "#00ff00";

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

// [👤semio📚js🗃️sketchpad💻type🔖scene🪨getcomputedcolorformesh](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/getComputedColorForMesh)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖scene🪨getcomputedcolorformesh](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/getComputedColorForMesh)
 * getComputedColorForMesh holds the data fields for a getComputedColorForMesh record.
 **/
const getComputedColorForMesh = (variable: string): string => getComputedStyle(document.documentElement).getPropertyValue(variable).trim();

// [👤semio📚js🗃️sketchpad💻type🔖scene🪨gltfmesh](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/GLTFMesh)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖scene🪨gltfmesh](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/GLTFMesh)
 * GLTFMesh holds the data fields for a GLTFMesh record.
 **/
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

/** FBXMesh holds the data fields for a FBXMesh record.
 **/
// [👤semio📚js🗃️sketchpad💻type🔖imports🔖scene🪨fbxmesh](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Scene/d/i/FBXMesh)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖scene🪨fbxmesh](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/FBXMesh)
 **/
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

/** OBJMesh holds the data fields for a OBJMesh record.
 **/
// [👤semio📚js🗃️sketchpad💻type🔖imports🔖scene🪨objmesh](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Scene/d/i/OBJMesh)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖scene🪨objmesh](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/OBJMesh)
 **/
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

/**
 * LoadedTypeMesh holds the data fields for a LoadedTypeMesh record.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖scene🪨loadedtypemesh](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Scene/d/i/LoadedTypeMesh)
 **/
const LoadedTypeMesh: FC<{
  url: string;
  fileExtension: string;
  onPointerDown: (e: ThreeEvent<PointerEvent>) => void;
  onPointerUp: (e: ThreeEvent<PointerEvent>) => void;
  onPointerMove: (e: ThreeEvent<PointerEvent>) => void;
  onPointerOut: (e: ThreeEvent<PointerEvent>) => void;
}> = ({ url, fileExtension, onPointerDown, onPointerUp, onPointerMove, onPointerOut }) => {
  const ext = fileExtension.toLowerCase();

  if (ext === "glb" || ext === "gltf") {
    return <GLTFMesh url={url} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
  } else if (ext === "fbx") {
    return <FBXMesh url={url} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
  } else if (ext === "obj") {
    return <OBJMesh url={url} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
  } else {
    return <GLTFMesh url={url} onPointerDown={onPointerDown} onPointerUp={onPointerUp} onPointerMove={onPointerMove} onPointerOut={onPointerOut} />;
  }
};

/** selectTypeModels holds the data fields for a selectTypeModels record.
 **/
// [👤semio📚js🗃️sketchpad💻type🔖imports🔖scene🪨selecttypemodels](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Scene/d/i/selectTypeModels)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖scene🪨selecttypemodels](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/selectTypeModels)
 **/
const selectTypeModels = (type: Type) => type.models;
// [👤semio📚js🗃️sketchpad💻type🔖scene🪨selecttypeconcepts](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/selectTypeConcepts)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖scene🪨selecttypeconcepts](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/selectTypeConcepts)
 * selectTypeConcepts holds the data fields for a selectTypeConcepts record.
 **/
const selectTypeConcepts = (type: Type) => type.concepts;
/** selectTypeMeshGuid holds the data fields for a selectTypeMeshGuid record.
 **/
// [👤semio📚js🗃️sketchpad💻type🔖imports🔖scene🪨selecttypemeshguid](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Scene/d/i/selectTypeMeshGuid)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖scene🪨selecttypemeshguid](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/selectTypeMeshGuid)
 **/
const selectTypeMeshGuid = (type: Type) => type.guid;

/**
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖scene🪨selecttypeguid](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Scene/d/i/selectTypeGuid)
 * TypeMesh holds the data fields for a TypeMesh record.
 **/
const TypeMesh: FC<{ activeTool: ToolKind; onPortPreview: (position: THREE.Vector3, normal: THREE.Vector3) => void; onPortCreate: (position: THREE.Vector3, normal: THREE.Vector3) => void; onClearPreview: () => void }> = ({
  activeTool,
  onPortPreview,
  onPortCreate,
  onClearPreview,
}) => {
  const typeModels = useType(selectTypeModels) as Model[] | undefined;
  const typeConcepts = useType(selectTypeConcepts) as any[] | undefined;
  const typeGuid = useType(selectTypeMeshGuid) as string | undefined;

  const files = useKitFiles();
  const kitDataSource = useKitStore() as CollaborativeKitStore;
  const [selectedModelGuid] = useTypeAppSelectedModelGuid();
  const [selectedModelTags] = useTypeAppSelectedModelTags();
  const [isPointerDown, setIsPointerDown] = useState(false);
  const pointerDownTimeRef = useRef<number>(0);
  const pointerDownPositionRef = useRef<{ x: number; y: number }>({ x: 0, y: 0 });

  const prevModelGuidRef = useRef<string | null>(null);

  const [blobUrl, setBlobUrl] = useState<string | null>(null);

  const { modelUrl, fileExtension, fileGuid, modelGuid, selectionReason } = useMemo(() => {
    if (!typeModels || typeModels.length === 0) {
      return { modelUrl: null, fileExtension: "", fileGuid: null, modelGuid: null, selectionReason: "no-models" };
    }

    let model: Model | undefined;
    let reason = "";

    if (selectedModelGuid) {
      model = typeModels.find((r) => r.guid === selectedModelGuid);
      reason = "explicit-guid";
    } else if (selectedModelTags.length > 0) {
      model = selectBestModel(typeModels, selectedModelTags);
      reason = "manual-tags";
    } else {
      const conceptGuids = typeConcepts?.map((c) => c.guid) ?? [];
      if (conceptGuids.length > 0) {
        model = findModel(typeModels, conceptGuids);
        reason = "type-concepts";
      } else {
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

    const url = kitDataSource.getFileUrl(file.guid);
    if (url) {
      return { modelUrl: url, fileExtension: ext, fileGuid: file.guid, modelGuid: model.guid, selectionReason: reason };
    }

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
          console.warn("[TypeMesh] No URL available for file:", fileGuid);
        }
      } catch (error) {
        console.error("[TypeMesh] Failed to get blob URL:", error);
      }
    })();

    return () => {
      cancelled = true;
    };
  }, [fileGuid, kitDataSource]);

  const handlePointerDown = useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (activeTool === ToolKind.CONNECTOR) {
        setIsPointerDown(true);
        pointerDownTimeRef.current = Date.now();
        pointerDownPositionRef.current = { x: event.clientX, y: event.clientY };
      }
    },
    [activeTool],
  );

  const handlePointerUp = useCallback(
    (event: ThreeEvent<PointerEvent>) => {
      if (activeTool === ToolKind.CONNECTOR && isPointerDown) {
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
      if (activeTool === ToolKind.CONNECTOR && event.face && !isPointerDown) {
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
      if (activeTool === ToolKind.CONNECTOR) {
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

// [👤semio📚js🗃️sketchpad💻type🔖scene🪨selecttypeports](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/selectTypePorts)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖scene🪨selecttypeports](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/selectTypePorts)
 * selectTypePorts holds the data fields for a selectTypePorts record.
 **/
const selectTypePorts = (type: Type) => type.connectors;
// [👤semio📚js🗃️sketchpad💻type🔖scene🪨selecttypeguid](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/selectTypeGuid)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖scene🪨selecttypeguid](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Scene/d/i/selectTypeGuid)
 * selectTypeGuid holds the data fields for a selectTypeGuid record.
 **/
const selectTypeGuid = (type: Type) => type.guid;

/**
 * SceneContent holds the data fields for a SceneContent record.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖scene🪨scenecontent](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Scene/d/i/SceneContent)
 **/
const SceneContent: FC = React.memo(() => {
  const [activeTool] = useTypeAppActiveTool();
  const typeFilters = useTypeFilters();

  const typePorts = useType(selectTypePorts) as Connector[] | undefined;
  const typeGuid = useType(selectTypeGuid) as string | undefined;

  const kitCommands = useKitCommands();
  const [selection, setSelection] = useTypeAppSelection();
  const [hover] = useTypeAppHover();

  const [hoverPort] = useTypeAppHoverPort();
  const [clearHover] = useTypeAppClearHover();
  const [focusPort] = useTypeAppFocusPort();
  const [connectorPreview, setConnectorPreview] = useState<{ position: THREE.Vector3; normal: THREE.Vector3 } | null>(null);
  const focusContext = useFocusSafe();
  const prevItemsRef = useRef<string>("");
  const visibleTypePorts = useMemo(() => (typeFilters.showConnectors ? typePorts : []), [typeFilters.showConnectors, typePorts]);

  useEffect(() => {
    if (!focusContext) return;
    const items = (visibleTypePorts || []).map((connector) => ({
      id: connector.guid,
      label: connector.description || `Connector ${connector.guid.substring(0, 8)}`,
      category: "Connectors",
    }));
    const itemsKey = items.map((item) => `${item.id}:${item.label}`).join("|");
    if (prevItemsRef.current !== itemsKey) {
      prevItemsRef.current = itemsKey;
      focusContext.setFocusItems(items);
    }
  }, [focusContext, visibleTypePorts]);

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

  const handlePortPreview = useCallback((position: THREE.Vector3, normal: THREE.Vector3) => {
    setConnectorPreview({ position, normal });
  }, []);

  const handlePortCreate = useCallback(
    (position: THREE.Vector3, normal: THREE.Vector3) => {
      if (typeGuid && kitCommands) {
        const semioPosition = position.clone().applyMatrix4(toSemioRotation());
        const semioNormal = normal.clone().applyMatrix4(toSemioRotation()).normalize();

        const newPort: Connector = {
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
          connectors: {
            added: [newPort],
          },
        });
      }
    },
    [typeGuid, kitCommands],
  );

  const handleClearPreview = useCallback(() => {
    setConnectorPreview(null);
    if (clearHover) clearHover();
  }, [clearHover]);

  useEffect(() => {
    if (typeFilters.showConnectors) return;
    setConnectorPreview(null);
    if (clearHover) clearHover();
  }, [typeFilters.showConnectors, clearHover]);

  const handlePortClick = useCallback(
    (connectorId: string) => {
      if (!setSelection) return;
      const compositionKind = resolveSelectionCompositionKind(activeTool);
      setSelection({
        ...(selection || {}),
        connectors: applySelectionComposition(selection?.connectors, [connectorId], compositionKind),
        models: compositionKind === "replace" ? [] : selection?.models || [],
      });
    },
    [selection, setSelection, activeTool],
  );

  const handlePortHover = useCallback(
    (connectorId: string) => {
      if (hoverPort) hoverPort(connectorId);
    },
    [hoverPort],
  );

  const handlePortLeave = useCallback(() => {
    if (clearHover) clearHover();
  }, [clearHover]);

  const handlePortDoubleClick = useCallback(
    (connectorId: string) => {
      if (focusPort) focusPort(connectorId);
    },
    [focusPort],
  );

  return (
    <>
      {typeFilters.showModels && <TypeMesh activeTool={activeTool} onPortPreview={handlePortPreview} onPortCreate={handlePortCreate} onClearPreview={handleClearPreview} />}
      {visibleTypePorts?.map((connector) => {
        const isSelected = selection?.connectors?.includes(connector.guid) || false;
        const isHovered = hover?.connector === connector.guid;
        return (
          <ConnectorVisual
            key={connector.guid}
            connector={connector}
            isSelected={isSelected}
            isHovered={isHovered}
            onHover={() => handlePortHover(connector.guid)}
            onLeave={handlePortLeave}
            onClick={() => handlePortClick(connector.guid)}
            onDoubleClick={() => handlePortDoubleClick(connector.guid)}
          />
        );
      })}
      {typeFilters.showConnectors && connectorPreview && <ConnectorPreview position={connectorPreview.position} normal={connectorPreview.normal} />}
    </>
  );
});

/**
 * Scene holds the data fields for a Scene record.
 *[👤semio📚js🗃️sketchpad💻type🔖imports🔖scene🪨scene](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Scene/d/i/Scene)
 **/
const Scene: FC<{ isDragOver?: boolean }> = ({ isDragOver = false }) => {
  const [setCamera] = useTypeAppSetCamera();
  const [camera] = useTypeAppCamera();
  const [focusedConnectorGuid] = useTypeAppFocusedConnectorGuid();
  const [deselectAll] = useTypeAppDeselectAll();
  const [clearFocus] = useTypeAppClearFocus();

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
    <SceneComponent camera={camera} onCameraChange={onCameraChange} onPointerMissed={onPointerMissed} focusedItemId={focusedConnectorGuid} onFocusComplete={onFocusComplete}>
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

// #region Panels

// #region Right

// #region Details

// [👤semio📚js🗃️sketchpad💻typetsx🔖panels](repo://section/SEMIO/JS/SKETCHPAD/TYPE.TSX/PANELS)
// Detail panel sections for editing type properties, connectors, models, authors, and attributes. MUST render within tree items.

/**
 * Detail panel section displaying editable type properties.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖right🔖details🪨typedetails](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Right/s/Details/d/i/TypeDetails)
 **/
export const TypeDetails: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <TypeDetailsForm />;
};

/** TypeDetailsForm holds the data fields for a TypeDetailsForm record.
 **/
// [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖right🔖details🪨typedetailsform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Right/s/Details/d/i/TypeDetailsForm)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖panels🔖right🔖details🪨typedetailsform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Panels/s/Right/s/Details/d/i/TypeDetailsForm)
 **/
const TypeDetailsForm: FC = () => {
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;

  const updateTypeField = (diff: any) => {
    kitCommands?.updateType(type.guid, diff);
  };

  return (
    <>
      <TreeRow>
        <Input lazy id="semio.sketchpad.app.type.panel.details.section.type.name" value={type.name} onLazyChange={(value) => updateTypeField({ name: value })} showLabel />
      </TreeRow>
      <TreeRow>
        <Textarea
          lazy
          id="semio.sketchpad.app.type.panel.details.section.type.description"
          value={type.description || ""}
          placeholderId="semio.sketchpad.app.type.descriptionPlaceholder.label"
          onLazyChange={(value) => updateTypeField({ description: value })}
          showLabel
        />
      </TreeRow>
      <TreeRow>
        <Input lazy id="semio.sketchpad.app.type.panel.details.section.type.icon" value={type.icon || ""} placeholderId="semio.sketchpad.app.type.iconPlaceholder.label" onLazyChange={(value) => updateTypeField({ icon: value })} showLabel />
      </TreeRow>
      <TreeRow>
        <Input lazy id="semio.sketchpad.app.type.panel.details.section.type.image" value={type.image || ""} placeholderId="semio.sketchpad.app.type.imagePlaceholder.label" onLazyChange={(value) => updateTypeField({ image: value })} showLabel />
      </TreeRow>
      <TreeRow>
        <Input
          lazy
          id="semio.sketchpad.app.type.panel.details.section.type.parent"
          value={type.parent?.guid || ""}
          placeholderId="semio.sketchpad.app.type.parentPlaceholder.label"
          onLazyChange={(value) => updateTypeField({ parent: value ? { guid: value } : undefined })}
          showLabel
        />
      </TreeRow>
      <TreeRow>
        <Toggle id="semio.sketchpad.app.type.panel.details.section.type.abstract" pressed={type.isAbstract || false} onPressedChange={(value) => updateTypeField({ isAbstract: value })} showLabel icon={<CheckIcon />} />
      </TreeRow>
      {type.unit !== undefined && (
        <TreeRow>
          <Input lazy id="semio.sketchpad.app.type.panel.details.section.type.unit" value={type.unit} onLazyChange={(value) => updateTypeField({ unit: value })} showLabel />
        </TreeRow>
      )}
    </>
  );
};

/**
 * Detail panel section for managing type models with add, remove, and reorder.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖right🔖details🪨modelssection](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Right/s/Details/d/i/ModelsSection)
 **/
export const ModelsSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <ModelsSectionForm />;
};

/** ModelsSectionForm holds the data fields for a ModelsSectionForm record.
 **/
// [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖right🔖details🪨modelssectionform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Right/s/Details/d/i/ModelsSectionForm)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖panels🔖right🔖details🪨modelssectionform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Panels/s/Right/s/Details/d/i/ModelsSectionForm)
 **/
const ModelsSectionForm: FC = () => {
  const tooltip = useTooltip();
  const [hoverModel] = useTypeAppHoverModel();
  const [clearHover] = useTypeAppClearHover();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;
  const [selection, setSelection] = useTypeAppSelection();
  const [hover] = useTypeAppHover();
  const [activeTool] = useTypeAppActiveTool();

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
                  onClick={(e: React.MouseEvent) => {
                    if (!setSelection) return;
                    const compositionKind = resolveSelectionCompositionKind(activeTool, {
                      shiftKey: e.shiftKey,
                      altKey: e.altKey,
                      ctrlKey: e.ctrlKey,
                      metaKey: e.metaKey,
                    });
                    setSelection({
                      ...(selection || {}),
                      models: applySelectionComposition(selection?.models, [model.guid], compositionKind),
                      connectors: compositionKind === "replace" ? [] : selection?.connectors || [],
                    });
                  }}
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
                    <TreeRow>
                      <Input
                        id="semio.sketchpad.app.type.panel.details.section.models.url"
                        value={model.url}
                        onChange={(e) => {
                          updateModel(model.guid, { url: e.target.value });
                        }}
                        showLabel
                      />
                    </TreeRow>
                    <TreeRow>
                      <Textarea
                        id="semio.sketchpad.app.type.panel.details.section.models.description"
                        value={model.description || ""}
                        placeholderId="semio.sketchpad.app.type.modelDescriptionPlaceholder.label"
                        onChange={(e) => {
                          updateModel(model.guid, { description: e.target.value });
                        }}
                        showLabel
                      />
                    </TreeRow>
                    <TreeRow>
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
                    </TreeRow>
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

/**
 * Detail panel section listing all type connectors with inline editing.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖right🔖details🪨connectorslistsection](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Right/s/Details/d/i/ConnectorsListSection)
 **/
export const ConnectorsListSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <ConnectorsListSectionForm />;
};

/** ConnectorsListSectionForm holds the data fields for a ConnectorsListSectionForm record.
 **/
// [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖right🔖details🪨connectorslistsectionform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Right/s/Details/d/i/ConnectorsListSectionForm)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖panels🔖right🔖details🪨connectorslistsectionform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Panels/s/Right/s/Details/d/i/ConnectorsListSectionForm)
 **/
const ConnectorsListSectionForm: FC = () => {
  const tooltip = useTooltip();
  const [hoverPort] = useTypeAppHoverPort();
  const [clearHover] = useTypeAppClearHover();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;
  const [selection, setSelection] = useTypeAppSelection();
  const [hover] = useTypeAppHover();
  const [activeTool] = useTypeAppActiveTool();

  const applyDiff = (diff: any) => {
    kitCommands?.updateType(type.guid, diff);
  };

  const updatePort = (id: string, connectorDiff: any) => {
    const connector = type.connectors?.find((existingConnector) => existingConnector.guid === id);
    const diff: any = { ...connectorDiff };
    if (connector) {
      if (connectorDiff.point) {
        diff.point = {};
        if (connectorDiff.point.x !== undefined) diff.point.x = connectorDiff.point.x - connector.point.x;
        if (connectorDiff.point.y !== undefined) diff.point.y = connectorDiff.point.y - connector.point.y;
        if (connectorDiff.point.z !== undefined) diff.point.z = connectorDiff.point.z - connector.point.z;
      }
      if (connectorDiff.direction) {
        diff.direction = {};
        if (connectorDiff.direction.x !== undefined) diff.direction.x = connectorDiff.direction.x - connector.direction.x;
        if (connectorDiff.direction.y !== undefined) diff.direction.y = connectorDiff.direction.y - connector.direction.y;
        if (connectorDiff.direction.z !== undefined) diff.direction.z = connectorDiff.direction.z - connector.direction.z;
      }
    }
    applyDiff({
      connectors: {
        updated: [{ id, diff }],
      },
    });
  };

  const hasPorts = type.connectors && type.connectors.length > 0;

  return (
    <>
      <TreeItem
        id="semio.sketchpad.app.type.connectors"
        actions={[
          {
            icon: <AddIcon />,
            onClick: () => {
              const origin = "semio.sketchpad.app.type.panel.details.connectors.add";
              applyDiff({
                connectors: {
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
          <TreeRow>
            <Ring
              id="semio.sketchpad.app.type.panel.details.section.connectors.ring"
              orbs={(type.connectors || []).map((connector: any) => {
                const isSelected = selection?.connectors?.includes(connector.guid) || false;
                return {
                  id: connector.guid,
                  t: connector.t ?? 0,
                  selected: isSelected,
                  hovered: hover?.connector === connector.guid,
                  disabled: !isSelected,
                };
              })}
              onOrbChange={(orbId, _oldT, newT) => {
                updatePort(orbId, { t: newT });
              }}
              onOrbSelect={(orbId) => {
                if (!setSelection) return;
                setSelection({
                  ...(selection || {}),
                  connectors: [orbId],
                  models: [],
                });
              }}
              onOrbHoverChange={(orbId, hovered) => {
                if (hovered) hoverPort?.(orbId);
                else clearHover?.();
              }}
              showLabel
            />
          </TreeRow>
        )}
        {hasPorts && (
          <SortableTreeItems
            items={(type.connectors || []).map((connector: any, index: number) => ({
              ...connector,
              id: `connector-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              if (!type.connectors) return;
              const origin = "semio.sketchpad.app.type.panel.details.connectors.reorder";
              applyDiff({
                connectors: {
                  removed: type.connectors.map((existingConnector: any) => existingConnector.guid),
                  added: arrayMove(type.connectors, oldIndex, newIndex),
                },
              });
            }}
          >
            {(connector, index) => {
              const isSelected = selection?.connectors?.includes(connector.guid) || false;
              const isHovered = hover?.connector === connector.guid;
              const handleClick = (event: React.MouseEvent) => {
                event.stopPropagation();
                if (!setSelection) return;
                const compositionKind = resolveSelectionCompositionKind(activeTool, {
                  shiftKey: event.shiftKey,
                  altKey: event.altKey,
                  ctrlKey: event.ctrlKey,
                  metaKey: event.metaKey,
                });
                setSelection({
                  ...(selection || {}),
                  connectors: applySelectionComposition(selection?.connectors, [connector.guid], compositionKind),
                  models: compositionKind === "replace" ? [] : selection?.models || [],
                });
              };

              const handleHover = () => {
                if (hoverPort) hoverPort(connector.guid);
              };

              const handleLeave = () => {
                if (clearHover) clearHover();
              };

              return (
                <div onPointerEnter={handleHover} onPointerLeave={handleLeave} onClick={handleClick}>
                  <TreeItem
                    key={`connector-${index}`}
                    id="semio.sketchpad.app.type.connector"
                    label={typeof connector.port === "string" ? connector.port : connector.port?.guid || ""}
                    sortable={true}
                    sortableId={`connector-${index}`}
                    isDragHandle={true}
                    className={`cursor-selectable ${isSelected ? "ring-1 ring-[color:var(--active-base)]" : ""} ${isHovered ? "bg-[color:var(--hover-base)]" : ""}`}
                    actions={[
                      {
                        icon: <RemoveIcon />,
                        onClick: () => {
                          const origin = "semio.sketchpad.app.type.panel.details.connectors.remove";
                          applyDiff({
                            connectors: {
                              removed: [connector.guid],
                            },
                          });
                        },
                        id: "semio.sketchpad.common.remove",
                      },
                    ]}
                  >
                    <TreeRow>
                      <Input
                        lazy
                        id="semio.sketchpad.app.type.panel.details.section.connectors.port"
                        value={typeof connector.port === "string" ? connector.port : connector.port?.guid || ""}
                        placeholderId="semio.sketchpad.app.type.connectorPortPlaceholder.label"
                        onLazyChange={(value: string) => {
                          updatePort(connector.guid, { port: value });
                        }}
                        showLabel
                      />
                    </TreeRow>
                    <TreeRow>
                      <Textarea
                        lazy
                        id="semio.sketchpad.app.type.panel.details.section.connectors.description"
                        value={connector.description || ""}
                        placeholderId="semio.sketchpad.app.type.connectorDescriptionPlaceholder.label"
                        onLazyChange={(value: string) => {
                          updatePort(connector.guid, { description: value });
                        }}
                        showLabel
                      />
                    </TreeRow>
                    <TreeItem id="semio.sketchpad.app.type.connectorPoint">
                      <TreeRow>
                        <Stepper
                          id="semio.sketchpad.app.type.panel.details.section.connectors.point.x"
                          value={connector.point.x}
                          onChange={(value: number) => {
                            updatePort(connector.guid, { point: { x: value } });
                          }}
                          step={0.1}
                        />
                      </TreeRow>
                      <TreeRow>
                        <Stepper
                          id="semio.sketchpad.app.type.panel.details.section.connectors.point.y"
                          value={connector.point.y}
                          onChange={(value: number) => {
                            updatePort(connector.guid, { point: { y: value } });
                          }}
                          step={0.1}
                        />
                      </TreeRow>
                      <TreeRow>
                        <Stepper
                          id="semio.sketchpad.app.type.panel.details.section.connectors.point.z"
                          value={connector.point.z}
                          onChange={(value: number) => {
                            updatePort(connector.guid, { point: { z: value } });
                          }}
                          step={0.1}
                        />
                      </TreeRow>
                    </TreeItem>
                    <TreeItem id="semio.sketchpad.app.type.connectorDirection">
                      <TreeRow>
                        <Stepper
                          id="semio.sketchpad.app.type.panel.details.section.connectors.direction.x"
                          value={connector.direction.x}
                          onChange={(value: number) => {
                            updatePort(connector.guid, { direction: { x: value } });
                          }}
                          step={0.1}
                        />
                      </TreeRow>
                      <TreeRow>
                        <Stepper
                          id="semio.sketchpad.app.type.panel.details.section.connectors.direction.y"
                          value={connector.direction.y}
                          onChange={(value: number) => {
                            updatePort(connector.guid, { direction: { y: value } });
                          }}
                          step={0.1}
                        />
                      </TreeRow>
                      <TreeRow>
                        <Stepper
                          id="semio.sketchpad.app.type.panel.details.section.connectors.direction.z"
                          value={connector.direction.z}
                          onChange={(value: number) => {
                            updatePort(connector.guid, { direction: { z: value } });
                          }}
                          step={0.1}
                        />
                      </TreeRow>
                    </TreeItem>
                    <TreeRow>
                      <Input
                        lazy
                        id="semio.sketchpad.app.type.panel.details.section.connectors.compatiblePorts"
                        value={(connector.compatiblePorts || []).join(", ")}
                        placeholderId="semio.sketchpad.app.type.connectorCompatiblePortsPlaceholder.label"
                        onLazyChange={(value: string) => {
                          updatePort(connector.guid, {
                            compatiblePorts: value
                              .split(",")
                              .map((port_) => port_.trim())
                              .filter((port_) => port_),
                          });
                        }}
                        showLabel
                      />
                    </TreeRow>
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

/**
 * Detail panel section for managing type authors.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖right🔖details🪨authorssection](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Right/s/Details/d/i/AuthorsSection)
 **/
export const AuthorsSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <AuthorsSectionForm />;
};

/** AuthorsSectionForm holds the data fields for a AuthorsSectionForm record.
 **/
// [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖right🔖details🪨authorssectionform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Right/s/Details/d/i/AuthorsSectionForm)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖panels🔖right🔖details🪨authorssectionform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Panels/s/Right/s/Details/d/i/AuthorsSectionForm)
 **/
const AuthorsSectionForm: FC = () => {
  const tooltip = useTooltip();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;
  const kit = useKit() as Kit | null;

  const updateAuthors = (authors: string[]) => {
    kitCommands?.updateType(type.guid, { authors: authors.map((a) => ({ guid: a })) });
  };

  const hasAuthors = type?.authors && type.authors.length > 0;

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
            items={(type.authors || [])
              .filter((authorId): authorId is AuthorId => !!authorId?.guid)
              .map((authorId: AuthorId, index: number) => {
                const author = kit?.authors?.find((a: Author) => a?.guid === authorId.guid);
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
                <TreeRow>
                  <Input
                    id="semio.sketchpad.app.type.panel.details.section.authors.name"
                    value={item.name}
                    onChange={(e) => {
                      kitCommands?.updateAuthor(item.guid, { name: e.target.value });
                    }}
                    showLabel
                  />
                </TreeRow>
                <TreeRow>
                  <Input
                    id="semio.sketchpad.app.type.panel.details.section.authors.email"
                    value={item.email}
                    onChange={(e) => {
                      kitCommands?.updateAuthor(item.guid, { email: e.target.value });
                    }}
                    showLabel
                  />
                </TreeRow>
              </TreeItem>
            )}
          </SortableTreeItems>
        )}
      </TreeItem>
    </>
  );
};

/**
 * Detail panel section for managing type key-value attributes.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖right🔖details🪨attributessectionform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Right/s/Details/d/i/AttributesSectionForm)
 **/
export const AttributesSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <AttributesSectionForm />;
};

// [👤semio📚js🗃️sketchpad💻type🔖panels🔖right🔖details🪨attributessectionform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Panels/s/Right/s/Details/d/i/AttributesSectionForm)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖panels🔖right🔖details🪨attributessectionform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Panels/s/Right/s/Details/d/i/AttributesSectionForm)
 * AttributesSectionForm holds the data fields for a AttributesSectionForm record.
 **/
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
                <TreeRow>
                  <Input
                    id="semio.sketchpad.app.type.panel.details.section.attributes.name"
                    value={attribute.key}
                    onChange={(e) => {
                      updateAttribute(attribute.guid, { key: e.target.value });
                    }}
                    showLabel
                  />
                </TreeRow>
                <TreeRow>
                  <Input
                    id="semio.sketchpad.app.type.panel.details.section.attributes.value"
                    value={attribute.value || ""}
                    placeholderId="semio.sketchpad.app.type.attributeValuePlaceholder.label"
                    onChange={(e) => {
                      updateAttribute(attribute.guid, { value: e.target.value });
                    }}
                    showLabel
                  />
                </TreeRow>
                <TreeRow>
                  <Input
                    id="semio.sketchpad.app.type.panel.details.section.attributes.definition"
                    value={attribute.definition || ""}
                    placeholderId="semio.sketchpad.app.type.attributeDefinitionPlaceholder.label"
                    onChange={(e) => {
                      updateAttribute(attribute.guid, { definition: e.target.value });
                    }}
                    showLabel
                  />
                </TreeRow>
              </TreeItem>
            )}
          </SortableTreeItems>
        )}
      </TreeItem>
    </>
  );
};

/**
 * Detail panel section for editing a single selected connector.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖right🔖details🪨connectorsection](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Right/s/Details/d/i/ConnectorSection)
 **/
export const ConnectorSection: FC<{ connectorGuid: Guid }> = ({ connectorGuid }) => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <ConnectorSectionForm connectorGuid={connectorGuid} />;
};

// [👤semio📚js🗃️sketchpad💻type🔖panels🔖right🔖details🪨connectorsectionform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Panels/s/Right/s/Details/d/i/ConnectorSectionForm)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖panels🔖right🔖details🪨connectorsectionform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Panels/s/Right/s/Details/d/i/ConnectorSectionForm)
 * ConnectorSectionForm holds the data fields for a ConnectorSectionForm record.
 **/
const ConnectorSectionForm: FC<{ connectorGuid: Guid }> = ({ connectorGuid }) => {
  const tooltip = useTooltip();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;

  const connector = type.connectors?.find((p) => p.guid === connectorGuid);

  if (!connector) {
    return (
      <TreeRow>
        <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.type.connectorNotFound")}</p>
      </TreeRow>
    );
  }

  const updatePort = (id: string, connectorDiff: any) => {
    const connector = type.connectors?.find((existingConnector) => existingConnector.guid === id);
    const diff: any = { ...connectorDiff };
    if (connector) {
      if (connectorDiff.point) {
        diff.point = {};
        if (connectorDiff.point.x !== undefined) diff.point.x = connectorDiff.point.x - connector.point.x;
        if (connectorDiff.point.y !== undefined) diff.point.y = connectorDiff.point.y - connector.point.y;
        if (connectorDiff.point.z !== undefined) diff.point.z = connectorDiff.point.z - connector.point.z;
      }
      if (connectorDiff.direction) {
        diff.direction = {};
        if (connectorDiff.direction.x !== undefined) diff.direction.x = connectorDiff.direction.x - connector.direction.x;
        if (connectorDiff.direction.y !== undefined) diff.direction.y = connectorDiff.direction.y - connector.direction.y;
        if (connectorDiff.direction.z !== undefined) diff.direction.z = connectorDiff.direction.z - connector.direction.z;
      }
    }
    kitCommands?.updateType(type.guid, {
      connectors: {
        updated: [{ connector: { guid: id }, diff }],
      },
    });
  };

  return (
    <>
      <TreeRow>
        <Input
          lazy
          id="semio.sketchpad.app.type.panel.details.section.connectors.port"
          value={connector.port?.guid || ""}
          placeholderId="semio.sketchpad.app.type.connectorPortPlaceholder.label"
          onLazyChange={(value: string) => {
            updatePort(connector.guid, { port: value ? { guid: value } : undefined });
          }}
          showLabel
        />
      </TreeRow>
      <TreeRow>
        <Textarea
          lazy
          id="semio.sketchpad.app.type.panel.details.section.connectors.description"
          value={connector.description || ""}
          placeholderId="semio.sketchpad.app.type.connectorDescriptionPlaceholder.label"
          onLazyChange={(value: string) => {
            updatePort(connector.guid, { description: value });
          }}
          showLabel
        />
      </TreeRow>
      <TreeRow>
        <Ring
          id="semio.sketchpad.app.type.panel.details.section.connector.ring"
          orbs={(type.connectors || []).map((c: Connector) => ({
            id: c.guid,
            t: c.t ?? 0,
            selected: c.guid === connector.guid,
            disabled: c.guid !== connector.guid,
          }))}
          onOrbChange={(_orbId, _oldT, newT) => {
            updatePort(connector.guid, { t: newT });
          }}
          showLabel
        />
      </TreeRow>
      <TreeItem id="semio.sketchpad.app.type.connectorPoint">
        <TreeRow>
          <Stepper
            id="semio.sketchpad.app.type.panel.details.section.connectors.point.x"
            value={connector.point.x}
            onChange={(value: number) => {
              updatePort(connector.guid, { point: { x: value } });
            }}
            step={0.1}
          />
        </TreeRow>
        <TreeRow>
          <Stepper
            id="semio.sketchpad.app.type.panel.details.section.connectors.point.y"
            value={connector.point.y}
            onChange={(value: number) => {
              updatePort(connector.guid, { point: { y: value } });
            }}
            step={0.1}
          />
        </TreeRow>
        <TreeRow>
          <Stepper
            id="semio.sketchpad.app.type.panel.details.section.connectors.point.z"
            value={connector.point.z}
            onChange={(value: number) => {
              updatePort(connector.guid, { point: { z: value } });
            }}
            step={0.1}
          />
        </TreeRow>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.type.connectorDirection">
        <TreeRow>
          <Stepper
            id="semio.sketchpad.app.type.panel.details.section.connectors.direction.x"
            value={connector.direction.x}
            onChange={(value: number) => {
              updatePort(connector.guid, { direction: { x: value } });
            }}
            step={0.1}
          />
        </TreeRow>
        <TreeRow>
          <Stepper
            id="semio.sketchpad.app.type.panel.details.section.connectors.direction.y"
            value={connector.direction.y}
            onChange={(value: number) => {
              updatePort(connector.guid, { direction: { y: value } });
            }}
            step={0.1}
          />
        </TreeRow>
        <TreeRow>
          <Stepper
            id="semio.sketchpad.app.type.panel.details.section.connectors.direction.z"
            value={connector.direction.z}
            onChange={(value: number) => {
              updatePort(connector.guid, { direction: { z: value } });
            }}
            step={0.1}
          />
        </TreeRow>
      </TreeItem>
      <TreeRow>
        <Input
          lazy
          id="semio.sketchpad.app.type.panel.details.section.connectors.compatiblePorts"
          value={((connector as any).compatiblePorts || []).join(", ")}
          placeholderId="semio.sketchpad.app.type.connectorCompatiblePortsPlaceholder.label"
          onLazyChange={(value: string) => {
            updatePort(connector.guid, {
              compatiblePorts: value
                .split(",")
                .map((port_) => port_.trim())
                .filter((port_) => port_),
            } as any);
          }}
          showLabel
        />
      </TreeRow>
    </>
  );
};

/**
 * Detail panel section for batch-editing multiple selected connectors.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖right🔖details🪨connectorsmultiplesectionform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Right/s/Details/d/i/ConnectorsMultipleSectionForm)
 **/
export const ConnectorsMultipleSection: FC<{ connectorGuids: Guid[] }> = ({ connectorGuids }) => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <ConnectorsMultipleSectionForm connectorGuids={connectorGuids} />;
};

// [👤semio📚js🗃️sketchpad💻type🔖panels🔖right🔖details🪨connectorsmultiplesectionform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Panels/s/Right/s/Details/d/i/ConnectorsMultipleSectionForm)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖panels🔖right🔖details🪨connectorsmultiplesectionform](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Panels/s/Right/s/Details/d/i/ConnectorsMultipleSectionForm)
 * ConnectorsMultipleSectionForm holds the data fields for a ConnectorsMultipleSectionForm record.
 **/
const ConnectorsMultipleSectionForm: FC<{ connectorGuids: Guid[] }> = ({ connectorGuids }) => {
  const tooltip = useTooltip();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;

  const connectors = type.connectors?.filter((p) => connectorGuids.includes(p.guid)) || [];

  if (connectors.length === 0) {
    return (
      <TreeRow>
        <p className="text-sm text-muted-foreground">{useLabel("semio.sketchpad.app.type.connectorsNotFound")}</p>
      </TreeRow>
    );
  }

  const getCommonValue = <T,>(getter: (connector: any) => T | undefined): T | undefined => {
    const values = connectors.map(getter).filter((v) => v !== undefined);
    if (values.length === 0) return undefined;
    const firstValue = values[0];
    return values.every((v) => JSON.stringify(v) === JSON.stringify(firstValue)) ? firstValue : undefined;
  };

  const updatePorts = (origin: string, connectorDiff: any) => {
    connectors.forEach((connector) => {
      const diff: any = { ...connectorDiff };
      if (connectorDiff.point) {
        diff.point = {};
        if (connectorDiff.point.x !== undefined) diff.point.x = connectorDiff.point.x - connector.point.x;
        if (connectorDiff.point.y !== undefined) diff.point.y = connectorDiff.point.y - connector.point.y;
        if (connectorDiff.point.z !== undefined) diff.point.z = connectorDiff.point.z - connector.point.z;
      }
      if (connectorDiff.direction) {
        diff.direction = {};
        if (connectorDiff.direction.x !== undefined) diff.direction.x = connectorDiff.direction.x - connector.direction.x;
        if (connectorDiff.direction.y !== undefined) diff.direction.y = connectorDiff.direction.y - connector.direction.y;
        if (connectorDiff.direction.z !== undefined) diff.direction.z = connectorDiff.direction.z - connector.direction.z;
      }
      kitCommands?.updateType(type.guid, {
        connectors: {
          updated: [{ connector: { guid: connector.guid }, diff }],
        },
      });
    });
  };

  const commonPort = getCommonValue((p) => p.port);
  const commonT = getCommonValue((p) => p.t);
  const commonPointX = getCommonValue((p) => p.point?.x);
  const commonPointY = getCommonValue((p) => p.point?.y);
  const commonPointZ = getCommonValue((p) => p.point?.z);
  const commonDirectionX = getCommonValue((p) => p.direction?.x);
  const commonDirectionY = getCommonValue((p) => p.direction?.y);
  const commonDirectionZ = getCommonValue((p) => p.direction?.z);

  return (
    <>
      <TreeRow>
        <Input
          lazy
          id="semio.sketchpad.app.type.panel.details.section.connectors.port"
          value={commonPort || ""}
          placeholderId={commonPort === undefined ? "semio.sketchpad.common.mixedValues" : "semio.sketchpad.app.type.connectorPortPlaceholder.label"}
          onLazyChange={(value) => updatePorts("semio.sketchpad.app.type.panel.details.section.connectors.port", { port: value })}
          showLabel
        />
      </TreeRow>
      <TreeRow>
        <Slider
          id="semio.sketchpad.app.type.panel.details.section.connectors.t"
          value={[commonT ?? 0]}
          onValueChange={([value]) => {
            updatePorts("semio.sketchpad.app.type.panel.details.section.connectors.t", { t: value });
          }}
          min={0}
          max={1}
          step={0.01}
          showLabel
        />
      </TreeRow>
      <TreeItem id="semio.sketchpad.app.type.connectorPoint">
        <TreeRow>
          <Stepper
            id="semio.sketchpad.app.type.panel.details.section.connectors.point.x"
            value={commonPointX}
            onChange={(value: number) => {
              updatePorts("semio.sketchpad.app.type.panel.details.section.connectors.point.x", { point: { x: value } });
            }}
            step={0.1}
          />
        </TreeRow>
        <TreeRow>
          <Stepper
            id="semio.sketchpad.app.type.panel.details.section.connectors.point.y"
            value={commonPointY}
            onChange={(value: number) => {
              updatePorts("semio.sketchpad.app.type.panel.details.section.connectors.point.y", { point: { y: value } });
            }}
            step={0.1}
          />
        </TreeRow>
        <TreeRow>
          <Stepper
            id="semio.sketchpad.app.type.panel.details.section.connectors.point.z"
            value={commonPointZ}
            onChange={(value: number) => {
              updatePorts("semio.sketchpad.app.type.panel.details.section.connectors.point.z", { point: { z: value } });
            }}
            step={0.1}
          />
        </TreeRow>
      </TreeItem>
      <TreeItem id="semio.sketchpad.app.type.connectorDirection">
        <TreeRow>
          <Stepper
            id="semio.sketchpad.app.type.panel.details.section.connectors.direction.x"
            value={commonDirectionX}
            onChange={(value: number) => {
              updatePorts("semio.sketchpad.app.type.panel.details.section.connectors.direction.x", { direction: { x: value } });
            }}
            step={0.1}
          />
        </TreeRow>
        <TreeRow>
          <Stepper
            id="semio.sketchpad.app.type.panel.details.section.connectors.direction.y"
            value={commonDirectionY}
            onChange={(value: number) => {
              updatePorts("semio.sketchpad.app.type.panel.details.section.connectors.direction.y", { direction: { y: value } });
            }}
            step={0.1}
          />
        </TreeRow>
        <TreeRow>
          <Stepper
            id="semio.sketchpad.app.type.panel.details.section.connectors.direction.z"
            value={commonDirectionZ}
            onChange={(value: number) => {
              updatePorts("semio.sketchpad.app.type.panel.details.section.connectors.direction.z", { direction: { z: value } });
            }}
            step={0.1}
          />
        </TreeRow>
      </TreeItem>
    </>
  );
};

// #endregion Details

// #region Settings

// [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖right🔖settings](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Right/s/Settings)
// Settings panel for theme, language, device, expertise, and mode selection. MUST use toggle groups and select elements.

// [👤semio📚js🗃️sketchpad💻type🔖panels🔖right🔖settings🪨typesettingscontent](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Panels/s/Right/s/Settings/d/i/TypeSettingsContent)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖panels🔖right🔖settings🪨typesettingscontent](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Panels/s/Right/s/Settings/d/i/TypeSettingsContent)
 * TypeSettingsContent holds the data fields for a TypeSettingsContent record.
 **/
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
      <TreeRow>
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
      </TreeRow>
      <TreeRow>
        <Select id="semio.sketchpad.settings.language" value={language || "en"} onValueChange={(value: string) => setLanguage?.(value)} showLabel disabled={!canSetLanguage}>
          <SelectTrigger>
            <SelectValue placeholder={languagePlaceholder} />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="en">{languageEnLabel}</SelectItem>
          </SelectContent>
        </Select>
      </TreeRow>
      <TreeRow>
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
      </TreeRow>
      <TreeRow>
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
      </TreeRow>
      <TreeRow>
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
      </TreeRow>
    </>
  );
};

// #endregion Settings

// #endregion Right

// #endregion Panels

// #region Tools

// [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖tools](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Tools)
// Tool definitions for selection modes and connector creation. MUST export tool objects and settings components.

/**
 * toolModules holds the data fields for a toolModules record.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖tools🪨toolmodules](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Tools/d/i/toolModules)
 **/
const toolModules = import.meta.glob<Record<string, Tool<TypeAppState>>>("./*Tool.tsx", { eager: true });

/** ConnectorToolContent holds the data fields for a ConnectorToolContent record.
 **/
// [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖tools🪨connectortoolcontent](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Tools/d/i/ConnectorToolContent)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖tools🪨connectortoolcontent](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Tools/d/i/ConnectorToolContent)
 **/
const ConnectorToolContent: FC<ToolRenderContext<TypeAppState>> = () => {
  return null;
};

/**
 * Tool definition for the connector creation tool.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖tools🪨connectortool](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Tools/d/i/ConnectorTool)
 **/
export const ConnectorTool: Tool<TypeAppState> = {
  id: ToolKind.CONNECTOR,
  icon: <ConnectorIcon className="size-tiny" />,
  render: (context: ToolRenderContext<TypeAppState>) => ({
    scene: <ConnectorToolContent {...context} />,
  }),
};

/**
 * Tool definition for the normal selection tool.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖tools🪨selectionnormaltool](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Tools/d/i/SelectionNormalTool)
 **/
export const SelectionNormalTool: Tool<TypeAppState> = {
  id: ToolKind.SELECTION_NORMAL,
  icon: <SelectToolIcon className="size-tiny" />,
  render: (context: ToolRenderContext<TypeAppState>) => ({}),
};

/**
 * Tool definition for the additive selection tool.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖tools🪨selectionadditivetool](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Tools/d/i/SelectionAdditiveTool)
 **/
export const SelectionAdditiveTool: Tool<TypeAppState> = {
  id: ToolKind.SELECTION_ADDITIVE,
  icon: <AddIcon className="size-tiny" />,
  render: (context: ToolRenderContext<TypeAppState>) => ({}),
};

/**
 * Tool definition for the subtractive selection tool.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖tools🪨typeselectsettings](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Tools/d/i/TypeSelectSettings)
 **/
export const SelectionSubtractiveTool: Tool<TypeAppState> = {
  id: ToolKind.SELECTION_SUBTRACTIVE,
  icon: <RemoveIcon className="size-tiny" />,
  render: (context: ToolRenderContext<TypeAppState>) => ({}),
};

/**
 * Tool definition for hand/pan tool.
 * [👤semio📚js🗃️sketchpad💻typetsx🔖tools🪨handtool](repo://definition/SEMIO/JS/SKETCHPAD/TYPE.TSX/TOOLS/HAND-TOOL)
 **/
export const HandTool: Tool<TypeAppState> = {
  id: ToolKind.HAND,
  icon: <HandIcon className="size-tiny" />,
  render: (context: ToolRenderContext<TypeAppState>) => ({}),
};

/**
 * Settings component for the selection tool group with mode toggles.
 *MUST render toggle buttons for each selection sub-mode.
 * [👤semio📚js🗃️sketchpad💻typetsx🔖tools🪨typeselectsettings](repo://definition/SEMIO/JS/SKETCHPAD/TYPE.TSX/TOOLS/TYPE-SELECT-SETTINGS)
 **/
export const TypeSelectSettings: FC = () => {
  const [activeTool, setActiveTool] = useTypeAppActiveTool();
  const additiveLabel = useLabel("semio.sketchpad.app.type.tools.select.additive");
  const subtractiveLabel = useLabel("semio.sketchpad.app.type.tools.select.subtractive");

  return (
    <div className="flex shrink-0 items-center gap-single h-full px-single">
      <Toggle
        id="semio.sketchpad.app.type.tools.select.additive"
        icon={<AddIcon className="size-tiny" />}
        text={additiveLabel}
        pressed={activeTool === ToolKind.SELECTION_ADDITIVE}
        onPressedChange={(pressed) => setActiveTool && setActiveTool(pressed ? ToolKind.SELECTION_ADDITIVE : ToolKind.SELECTION_NORMAL)}
      />
      <Toggle
        id="semio.sketchpad.app.type.tools.select.subtractive"
        icon={<RemoveIcon className="size-tiny" />}
        text={subtractiveLabel}
        pressed={activeTool === ToolKind.SELECTION_SUBTRACTIVE}
        onPressedChange={(pressed) => setActiveTool && setActiveTool(pressed ? ToolKind.SELECTION_SUBTRACTIVE : ToolKind.SELECTION_NORMAL)}
      />
    </div>
  );
};

/**
 * Settings component for the hand tool.
 *MUST activate the hand tool on mount.
 * [👤semio📚js🗃️sketchpad💻typetsx🔖tools🪨typehandsettings](repo://definition/SEMIO/JS/SKETCHPAD/TYPE.TSX/TOOLS/TYPE-HAND-SETTINGS)
 **/
export const TypeHandSettings: FC = () => {
  const [activeTool, setActiveTool] = useTypeAppActiveTool();

  useEffect(() => {
    if (activeTool !== ToolKind.HAND && setActiveTool) {
      setActiveTool(ToolKind.HAND);
    }
  }, [setActiveTool]);

  return null;
};

/**
 * Settings component for toggling the connector creation tool.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖tools🪨typeconnectorsettings](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Tools/d/i/TypeConnectorSettings)
 **/
export const TypeConnectorSettings: FC = () => {
  const [activeTool, setActiveTool] = useTypeAppActiveTool();
  const connectorLabel = useLabel("semio.sketchpad.app.type.tools.connector");

  return (
    <div className="flex shrink-0 items-center gap-single h-full px-single">
      <Toggle id="semio.sketchpad.app.type.tools.connector" pressed={activeTool === ToolKind.CONNECTOR} onPressedChange={() => setActiveTool && setActiveTool(ToolKind.CONNECTOR)} icon={<ConnectorIcon className="size-tiny" />} text={connectorLabel} />
    </div>
  );
};

/**
 * Array of all Type app tool configurations.
 *
 *
 **/
export const TypeAppTools: Tool<TypeAppState>[] = [SelectionNormalTool, SelectionAdditiveTool, SelectionSubtractiveTool, HandTool, ConnectorTool];

// #endregion Tools

// #region App

// [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖app](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/App)
// Main TypeApp component orchestrating panels, scene, keyboard shortcuts, and drag-and-drop. MUST register sections on mount.

// [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖app🪨app](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/App/d/i/App)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖app🪨app](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/App/d/i/App)
 * App holds the data fields for a App record.
 **/
const App: FC = () => {
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const appType = useAppType();
  const [setActiveTool] = useTypeAppSetActiveTool();
  const { undo, redo } = useTypeAppCommands();

  const [activeTool] = useTypeAppActiveTool();
  const [selection] = useTypeAppSelection();
  const [isDragOver, setIsDragOver] = useState(false);

  useHotkeys("ctrl+z", () => undo?.(), { enableOnFormTags: true });
  useHotkeys("ctrl+y", () => redo?.(), { enableOnFormTags: true });
  useHotkeys("ctrl+shift+z", () => redo?.(), { enableOnFormTags: true });

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!setActiveTool || !isSelectionToolKind(activeTool)) return;
      const nextToolKind = toSelectionToolKind(
        resolveSelectionCompositionKind(ToolKind.SELECTION_NORMAL, {
          shiftKey: e.shiftKey,
          altKey: e.altKey,
          ctrlKey: e.ctrlKey,
          metaKey: e.metaKey,
        }),
      );
      if (nextToolKind !== ToolKind.SELECTION_NORMAL && nextToolKind !== activeTool) setActiveTool(nextToolKind);
    };
    const handleKeyUp = (e: KeyboardEvent) => {
      if (!setActiveTool || !isSelectionToolKind(activeTool)) return;
      const nextToolKind = toSelectionToolKind(
        resolveSelectionCompositionKind(ToolKind.SELECTION_NORMAL, {
          shiftKey: e.shiftKey,
          altKey: e.altKey,
          ctrlKey: e.ctrlKey,
          metaKey: e.metaKey,
        }),
      );
      if (nextToolKind === ToolKind.SELECTION_NORMAL && activeTool !== ToolKind.SELECTION_NORMAL) setActiveTool(ToolKind.SELECTION_NORMAL);
      if (nextToolKind !== ToolKind.SELECTION_NORMAL && nextToolKind !== activeTool) setActiveTool(nextToolKind);
    };
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [activeTool, setActiveTool]);

  useEffect(() => {
    if (appType !== "type") return;

    const hasPorts = selection?.connectors && selection.connectors.length > 0;
    const hasMultiplePorts = selection?.connectors && selection.connectors.length > 1;
    const hasSinglePort = selection?.connectors && selection.connectors.length === 1;

    const connectorsMultipleId = "semio.sketchpad.app.type.panel.details.section.connectors.multipleTitle";

    removeSection("details", "semio.sketchpad.app.type.properties");
    removeSection("details", "semio.sketchpad.app.type.connector.properties");
    removeSection("details", connectorsMultipleId);
    removeSection("details", "semio.sketchpad.app.kit.properties");

    if (hasSinglePort) {
      addSection("details", {
        id: "semio.sketchpad.app.type.connector.properties",
        specificity: 30,
        order: 0,
        content: () => <ConnectorSection connectorGuid={selection.connectors![0]} />,
      });
    } else if (hasMultiplePorts) {
      addSection("details", {
        id: connectorsMultipleId,
        specificity: 30,
        order: 0,
        content: () => <ConnectorsMultipleSection connectorGuids={selection.connectors!} />,
      });
    }

    addSection("details", {
      id: "semio.sketchpad.app.type.properties",
      specificity: 20,
      order: 50,
      content: () => (
        <>
          <TypeDetails />
          <ModelsSection />
          <ConnectorsListSection />
          <AuthorsSection />
          <AttributesSection />
        </>
      ),
    });

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
      removeSection("details", "semio.sketchpad.app.type.connector.properties");
      removeSection("details", connectorsMultipleId);
      removeSection("details", "semio.sketchpad.app.kit.properties");
    };
  }, [addSection, removeSection, appType, selection]);

  const type = useType() as Type | undefined;
  const kitCommands = useKitCommands();
  const [setSelectedModel] = useTypeAppSetSelectedModel();

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

        const newFileGuid = guid();
        const newFile: SemioFile = {
          guid: newFileGuid,
          name: file.name,
          size: file.size,
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
        };

        const newModelGuid = guid();
        const newModel: Model = {
          guid: newModelGuid,
          file: { guid: newFileGuid },
          description: file.name,
        };

        await kitCommands.addFile(newFile, file);

        await kitCommands.updateType(type.guid, {
          models: {
            added: [newModel],
          },
        });

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

  const persistedWindowLayout = useTypeApp((s) => s?.windowLayout);
  const addSidePanelTab = useAddSidePanelTab();
  const removeSidePanelTab = useRemoveSidePanelTab();

  useEffect(() => {
    if (appType !== "type") return;
    addSidePanelTab("right", {
      id: "semio.sketchpad.app.type.settings",
      icon: SettingsIcon,
      order: 100,
      content: () => (
        <TreeStateProvider>
          <Tree className="min-w-0 overflow-hidden p-double" sections={[{ id: "semio.sketchpad.app.type.settings.content", label: null, content: <TypeSettingsContent /> }]} />
        </TreeStateProvider>
      ),
    });
    addSidePanelTab("right", {
      id: "semio.sketchpad.app.type.chat",
      icon: ChatIcon,
      order: 101,
      content: () => <BasicChatPanel id="semio.sketchpad.app.type.chat" title="Type" />,
    });
    return () => {
      removeSidePanelTab("right", "semio.sketchpad.app.type.settings");
      removeSidePanelTab("right", "semio.sketchpad.app.type.chat");
    };
  }, [appType, addSidePanelTab, removeSidePanelTab]);

  const defaultLayout = useMemo(
    () => ({
      root: {
        type: "row",
        content: [
          {
            type: "stack",
            size: "100%",
            content: [
              {
                type: "component",
                componentName: TypeAppWindowKind.Scene,
                title: "scene",
                componentState: {},
              },
            ],
          },
        ],
      },
    }),
    [],
  );

  const windowLayout = useMemo(() => {
    if (!persistedWindowLayout) return undefined;

    const countWindows = (node: any): number => {
      if (!node) return 0;
      if (node.type === "component") return 1;
      if (node.content && Array.isArray(node.content)) {
        return node.content.reduce((sum: number, child: any) => sum + countWindows(child), 0);
      }
      return 0;
    };
    const windowCount = countWindows(persistedWindowLayout);

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

// #region Filters

// [👤semio📚js🗃️sketchpad💻typetsx🔖filters](repo://section/SEMIO/JS/SKETCHPAD/TYPE.TSX/FILTERS)
// Type filter context and toolbar toggles MUST control connector and model visibility via URL search params.

type TypeFilterKind = "connectors" | "models";

interface TypeFilterState {
  showConnectors: boolean;
  showModels: boolean;
}

const isTypeFilterKind = (value: string): value is TypeFilterKind => value === "connectors" || value === "models";
const parseTypeFilterState = (searchParams: URLSearchParams): TypeFilterState => {
  const kinds = searchParams.getAll("filter").filter(isTypeFilterKind);
  if (kinds.length === 0) {
    return { showConnectors: true, showModels: true };
  }
  return {
    showConnectors: kinds.includes("connectors"),
    showModels: kinds.includes("models"),
  };
};

const createTypeFilterStore = () => {
  let state: TypeFilterState = { showConnectors: true, showModels: true };
  const listeners = new Set<() => void>();

  const getState = () => state;
  const subscribe = (listener: () => void) => {
    listeners.add(listener);
    return () => {
      listeners.delete(listener);
    };
  };
  const setState = (nextState: TypeFilterState) => {
    if (state.showConnectors === nextState.showConnectors && state.showModels === nextState.showModels) {
      return;
    }
    state = nextState;
    listeners.forEach((listener) => listener());
  };

  return { getState, subscribe, setState };
};

const typeFilterStore = createTypeFilterStore();

const useTypeFilters = (): TypeFilterState => useSyncExternalStore(typeFilterStore.subscribe, typeFilterStore.getState, typeFilterStore.getState);

const TypeKindToggles: FC = () => {
  const [searchParams, setSearchParams] = useSearchParams();

  const selectedKindsFromUrl = useMemo(() => searchParams.getAll("filter").filter(isTypeFilterKind), [searchParams]);
  const selectedKinds = useMemo(() => new Set(selectedKindsFromUrl), [selectedKindsFromUrl]);

  useEffect(() => {
    typeFilterStore.setState(parseTypeFilterState(searchParams));
  }, [searchParams]);

  const toggleKind = (kind: TypeFilterKind) => {
    const allKinds: TypeFilterKind[] = ["connectors", "models"];
    const newParams = new URLSearchParams(searchParams);
    const kinds = newParams.getAll("filter").filter(isTypeFilterKind);

    if (kinds.length === 0) {
      newParams.delete("filter");
      allKinds.filter((k) => k !== kind).forEach((k) => newParams.append("filter", k));
    } else if (kinds.includes(kind)) {
      const remaining = kinds.filter((k) => k !== kind);
      newParams.delete("filter");
      remaining.forEach((k) => newParams.append("filter", k));
    } else {
      const updated = [...kinds, kind];
      newParams.delete("filter");
      if (updated.length < allKinds.length) {
        updated.forEach((k) => newParams.append("filter", k));
      }
    }

    typeFilterStore.setState(parseTypeFilterState(newParams));
    setSearchParams(newParams);
  };
  const isActive = (kind: TypeFilterKind) => selectedKindsFromUrl.length === 0 || selectedKinds.has(kind);

  const labelConnectors = useLabel("semio.sketchpad.app.type.toolbar.showConnectors");
  const labelModels = useLabel("semio.sketchpad.app.type.toolbar.showModels");

  return (
    <ToolbarGroup>
      <Toggle pressed={isActive("connectors")} onPressedChange={() => toggleKind("connectors")} id="semio.sketchpad.app.type.toolbar.showConnectors" icon={<ConnectorIcon />} text={labelConnectors} />
      <Toggle pressed={isActive("models")} onPressedChange={() => toggleKind("models")} id="semio.sketchpad.app.type.toolbar.showModels" icon={<SceneIcon />} text={labelModels} />
    </ToolbarGroup>
  );
};

// #endregion Filters

const TypeApp: FC = () => {
  const appType = useAppType();
  const addSection = useAddPanelSection();
  const removeSection = useRemovePanelSection();
  const commands = useTypeAppCommands();
  const commandsRef = useRef(commands);
  commandsRef.current = commands;

  useEffect(() => {
    if (appType !== "type") return;

    addSection("toolbar", {
      id: "semio.sketchpad.app.type.toolbar.filters",
      specificity: 10,
      order: 0,
      toolbarGroup: {
        id: "filter",
        labelId: "semio.sketchpad.toolbar.parent.filter",
        order: 5,
      },
      content: <TypeKindToggles />,
    });

    addSection("toolbar", {
      id: "semio.sketchpad.app.type.tools.selection",
      specificity: 20,
      order: 0,
      toolbarGroup: {
        id: "selection",
        labelId: "semio.sketchpad.toolbar.parent.selection",
        order: 10,
      },
      content: <TypeSelectSettings />,
    });

    addSection("toolbar", {
      id: "semio.sketchpad.app.type.tools.hand",
      specificity: 20,
      order: 2,
      toolbarGroup: {
        id: "hand",
        labelId: "semio.sketchpad.toolbar.parent.hand",
        order: 30,
        subToolId: ToolKind.HAND,
        subToolLabelId: "semio.sketchpad.toolbar.subtool.hand",
        subToolIcon: <HandIcon className="size-tiny" />,
      },
      content: <TypeHandSettings />,
    });

    addSection("toolbar", {
      id: "semio.sketchpad.app.type.tools.connector",
      specificity: 20,
      order: 10,
      toolbarGroup: {
        id: "create",
        labelId: "semio.sketchpad.toolbar.parent.create",
        order: 40,
        subToolId: ToolKind.CONNECTOR,
        subToolLabelId: "semio.sketchpad.toolbar.subtool.connector",
        subToolIcon: <ConnectorIcon className="size-tiny" />,
        onActivate: () => commandsRef.current.setActiveTool(ToolKind.CONNECTOR),
      },
      content: <TypeConnectorSettings />,
    });

    return () => {
      removeSection("toolbar", "semio.sketchpad.app.type.toolbar.filters");
      removeSection("toolbar", "semio.sketchpad.app.type.tools.selection");
      removeSection("toolbar", "semio.sketchpad.app.type.tools.hand");
      removeSection("toolbar", "semio.sketchpad.app.type.tools.connector");
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

// [👤semio📚js🗃️sketchpad💻type🔖app🛠️usetypeappinitialize](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/App/d/i/useTypeAppInitialize)
/**
 * [👤semio📚js🗃️sketchpad💻type🔖app🪨usetypeappinitialize](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/App/d/i/useTypeAppInitialize)
 * useTypeAppInitialize holds the data fields for a useTypeAppInitialize record.
 **/
function useTypeAppInitialize() {
  const actor = useSketchpadActor();
  const kitScope = useKitScope();
  const typeScope = useTypeScope();
  const kitGuid = kitScope?.guid ?? "";
  const typeGuid = typeScope?.guid ?? "";
  const initializedKeyRef = useRef<string | null>(null);

  useLayoutEffect(() => {
    if (!kitGuid || !typeGuid) return;
    const initKey = `${kitGuid}:${typeGuid}`;
    if (initializedKeyRef.current === initKey) return;

    actor.send({
      type: "TYPE.INIT",
      kitGuid,
      typeGuid,
      state: {
        panelVisibility: { toolbar: true, leftSidePanel: true, rightSidePanel: true, details: true },
        selection: undefined,
        hover: undefined,
        focusedConnector: undefined,
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
    initializedKeyRef.current = initKey;
  }, [kitGuid, typeGuid, actor]);
}

export default TypeApp;

// #endregion App

// #region Footer

// [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖footer](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Footer)
// Footer component displaying model tag toggles. MUST update footer items when tags change.

/**
 * Footer component rendering model tag toggle buttons.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖footer🪨typeappfooter](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Footer/d/i/TypeAppFooter)
 **/
export const TypeAppFooter: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const appType = useAppType();
  const type = useType() as Type | undefined;
  const tags = useKitTags();
  const [selectedModelTags] = useTypeAppSelectedModelTags();
  const [addModelTag] = useTypeAppAddModelTag();
  const [removeModelTag] = useTypeAppRemoveModelTag();

  const addModelTagRef = useRef(addModelTag);
  const removeModelTagRef = useRef(removeModelTag);
  const selectedModelTagsRef = useRef(selectedModelTags);

  useEffect(() => {
    addModelTagRef.current = addModelTag;
    removeModelTagRef.current = removeModelTag;
    selectedModelTagsRef.current = selectedModelTags;
  }, [addModelTag, removeModelTag, selectedModelTags]);

  const { allModelTagGuids, tagNameMap } = useMemo(() => {
    if (!type?.models) return { allModelTagGuids: [], tagNameMap: new Map<string, string>() };
    const tagGuids = new Set<string>();
    const nameMap = new Map<string, string>();
    type.models.forEach((model) => {
      model.tags?.forEach((tag) => {
        tagGuids.add(tag.guid);
      });
    });

    tags.forEach((tag) => {
      if (!nameMap.has(tag.guid)) {
        nameMap.set(tag.guid, tag.name);
      }
    });
    return { allModelTagGuids: Array.from(tagGuids), tagNameMap: nameMap };
  }, [type?.models, tags]);

  useEffect(() => {
    if (appType !== "type") return;

    const isTagSelected = (tagGuid: string): boolean => {
      return selectedModelTagsRef.current.includes(tagGuid);
    };

    allModelTagGuids.forEach((tagGuid) => {
      removeFooterItem(`semio.sketchpad.app.type.footer.tag.${tagGuid}`);
    });

    allModelTagGuids.forEach((tagGuid, index) => {
      const tagName = tagNameMap.get(tagGuid) || tagGuid.slice(0, 8);
      const isSelected = selectedModelTags.includes(tagGuid);

      addFooterItem({
        id: `semio.sketchpad.app.type.footer.tag.${tagGuid}`,
        text: tagName,
        className: isSelected ? "bg-active-base text-active-foreground" : "text-muted-foreground hover:text-foreground",
        onClick: () => {
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
  }, [appType, allModelTagGuids, tagNameMap, selectedModelTags, addFooterItem, removeFooterItem]);

  return null;
};

// #endregion Footer

// #region Config

// [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖config](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Config)
// App configuration for the TypeApp including route segments, panels, and path matching. MUST define all route segments.

/**
 * TypeApp configuration defining routes, panels, and path matching.
 * [👤semio📚js🗃️sketchpad💻type🔖imports🔖panels🔖config🪨config](repo://p/u/semio/b/l/js/fd/org/sketchpad/f/Type.tsx/s/Imports/s/Panels/s/Config/d/i/config)
 **/
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
    createPanelDefinition(PanelKind.STATS, "semio.sketchpad.navbar.panelToggle.stats.show"),
    createPanelDefinition(PanelKind.DETAILS, "semio.sketchpad.navbar.panelToggle.details.show"),
  ],
  matchesPath: (pathParts: string[]) => {
    const isUuidPattern = (str: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(str);
    return pathParts.length === 4 && pathParts[0] === "kits" && isUuidPattern(pathParts[1]) && pathParts[2] === "types" && isUuidPattern(pathParts[3]);
  },
  order: 30,
};

// #endregion Config
