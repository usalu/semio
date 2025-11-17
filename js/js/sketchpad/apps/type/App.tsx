// #region Header

// App.tsx

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
import React, { createContext, FC, Suspense, useCallback, useContext, useEffect, useMemo, useRef, useState } from "react";
import { useParams } from "react-router";
import * as THREE from "three";
import { OBJLoader } from "three/addons/loaders/OBJLoader.js";
import * as Y from "yjs";
import i18n from "../../../i18n";
import { Author, Camera, Coord, guid, Guid, Kit, Point, Port, Representation, selectBestRepresentation, File as SemioFile, Type, TypeDiff, Vector } from "../../../semio";
import type { KitStore, SketchpadStore, TypeStore } from "../../App";
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
  useFocusSafe,
  useIsInTypeScope,
  useKit,
  useKitCommands,
  useKitScope,
  useKitStore,
  useRemoveFooterItem,
  useRemovePanelSection,
  useSketchpadStore,
  useSyncDeep,
  useTooltip,
  useType,
  useTypeScope,
} from "../../App";
import { Input, Model, Scene as SceneComponent, Slider, SortableTreeItems, Stepper, Textarea, Toggle, TreeContent, TreeItem } from "../../elements";
import type { AppWindowConfig, KitCommandContext, KitDiffAppEdit, PanelDefinition, PanelVisibility, Tool, ToolDefinition, ToolRenderContext, Transact, TypeAppId, YAttributes, YLeafMapNumber, YLeafMapString, YStringArray } from "../../sketchpad";
import { createPanelDefinition, PanelKind, ToolKind } from "../../sketchpad";
import { AppConfig } from "../index";

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
  const module = await import("../kit/App");
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
import { AddIcon, CheckIcon, PortIcon, RemoveIcon, SelectToolIcon } from "@semio/assets";

// #endregion Imports

// #region Store

type YTypeAppVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YTypeApp = Y.Map<YTypeAppVal>;
type YTypeApps = Y.Map<YTypeApp>;

export interface TypeAppSelection {
  ports?: Guid[];
  representations?: Guid[];
}
export interface TypeAppSelectionPortsDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface TypeAppSelectionRepresentationsDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface TypeAppSelectionDiff {
  ports?: TypeAppSelectionPortsDiff;
  representations?: TypeAppSelectionRepresentationsDiff;
}
export enum TypeAppFullscreenWindow {
  None = "none",
  Ports = "ports",
  Representations = "representations",
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
  representation?: Guid;
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
  selectedRepresentationGuid?: Guid | null;
  selectedRepresentationTags?: string[];
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
  selectedRepresentationGuid?: Guid;
  selectedRepresentationTags?: string[];
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
  if (diff.representations) {
    inverse.representations = {
      added: diff.representations.removed ?? [],
      removed: diff.representations.added ?? [],
    };
  }
  return inverse;
}

class TypeAppStore extends KitDiffAppStore<TypeAppState, TypeAppDiff, TypeAppSelectionDiff, TypeAppEdit, TypeAppCommandContext, TypeAppCommandResult> {
  private readonly Guid: TypeAppId;

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
        yPanelVisibility.set("details", true);
        yPanelVisibility.set("chat", false);
        yPanelVisibility.set("settings", false);
        yMap.set("panelVisibility", yPanelVisibility);
      } else {
        // Ensure toolbar field exists for existing instances
        const yPanelVisibility = yMap.get("panelVisibility") as Y.Map<boolean>;
        if (yPanelVisibility && !yPanelVisibility.has("toolbar")) {
          yPanelVisibility.set("toolbar", true);
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
        details: true,
        chat: false,
        settings: false,
      };
    }
    return {
      toolbar: yPanelVisibility.get("toolbar") ?? true,
      workbench: yPanelVisibility.get("workbench") ?? false,
      details: yPanelVisibility.get("details") ?? true,
      chat: yPanelVisibility.get("chat") ?? false,
      settings: yPanelVisibility.get("settings") ?? false,
    };
  }

  get selection(): TypeAppSelection {
    const selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) return {};

    const result: TypeAppSelection = {};

    const ports = selection.get("ports") as Y.Array<string>;
    if (ports) {
      result.ports = ports.toArray();
    }

    const representations = selection.get("representations") as Y.Array<string>;
    if (representations) {
      result.representations = representations.toArray();
    }

    return result;
  }

  get hover(): TypeAppHover | undefined {
    const hover = this.yMap.get("hover") as Y.Map<string>;
    if (!hover) return undefined;

    const result: TypeAppHover = {};
    const port = hover.get("port");
    if (port) result.port = port;
    const representation = hover.get("representation");
    if (representation) result.representation = representation;

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
    return cameraStr ? JSON.parse(cameraStr) : undefined;
  }

  get focusedPortGuid(): Guid | undefined {
    return this.yMap.get("focusedPortGuid") as Guid | undefined;
  }

  get selectedRepresentationGuid(): Guid | undefined {
    return this.yMap.get("selectedRepresentationGuid") as Guid | undefined;
  }

  get selectedRepresentationTags(): string[] {
    const yTags = this.yMap.get("selectedRepresentationTags") as Y.Array<string> | undefined;
    return yTags ? yTags.toArray() : [];
  }

  get windowLayout(): any {
    const layoutStr = this.yMap.get("windowLayout") as string | undefined;
    return layoutStr ? JSON.parse(layoutStr) : undefined;
  }
  set windowLayout(layout: any) {
    if (layout) {
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
      selectedRepresentationGuid: this.selectedRepresentationGuid,
      selectedRepresentationTags: this.selectedRepresentationTags,
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

    if (selectionDiff.representations) {
      let yRepresentations = selection.get("representations") as Y.Array<string>;
      if (!yRepresentations) {
        yRepresentations = new Y.Array<string>();
        selection.set("representations", yRepresentations);
      }

      if (selectionDiff.representations.removed) {
        for (const repId of selectionDiff.representations.removed) {
          const index = yRepresentations.toArray().indexOf(repId);
          if (index >= 0) yRepresentations.delete(index, 1);
        }
      }

      if (selectionDiff.representations.added) {
        for (const repId of selectionDiff.representations.added) {
          if (!yRepresentations.toArray().includes(repId)) {
            yRepresentations.push([repId]);
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
        if (diff.hover.representation !== undefined) {
          if (diff.hover.representation) {
            yHover.set("representation", diff.hover.representation);
          } else {
            yHover.delete("representation");
          }
        }
        if (diff.hover.representation !== undefined) {
          if (diff.hover.representation) {
            yHover.set("representation", diff.hover.representation);
          } else {
            yHover.delete("representation");
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
      if (diff.selectedRepresentationGuid !== undefined) {
        if (diff.selectedRepresentationGuid === null) {
          this.yMap.delete("selectedRepresentationGuid");
        } else {
          this.yMap.set("selectedRepresentationGuid", diff.selectedRepresentationGuid);
        }
      }
      if (diff.selectedRepresentationTags !== undefined) {
        let yTags = this.yMap.get("selectedRepresentationTags") as Y.Array<string>;
        if (!yTags) {
          yTags = new Y.Array<string>();
          this.yMap.set("selectedRepresentationTags", yTags);
        }
        yTags.delete(0, yTags.length);
        if (diff.selectedRepresentationTags.length > 0) {
          yTags.push(diff.selectedRepresentationTags);
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
      console.log(`[${origin || "unknown"}] Executing (special) command: "${command}"`);
      this.undo();
      return {} as T;
    }
    if (command === "semio.typeApp.redo") {
      console.log(`[${origin || "unknown"}] Executing (special) command: "${command}"`);
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

export function useTypeApp<T>(selector?: (state: TypeAppState) => T, id?: TypeAppId): T | TypeAppState | null {
  const store = useTypeAppStore(identitySelector, id);
  if (!store) return null;
  return useSyncDeep<TypeAppState, T>(store as TypeAppStore, selector || ((state: TypeAppState) => state as T));
}

export function useTypeAppSelection(): TypeAppSelection {
  return useTypeApp((s) => s.selection) as TypeAppSelection;
}

export function useTypeAppPanelVisibility(): PanelVisibility {
  return useTypeApp((s) => s.panelVisibility) as PanelVisibility;
}

export function useTypeAppOthers(): TypeAppPresenceOther[] {
  return useTypeApp((s) => s.others) as TypeAppPresenceOther[];
}

export function useTypeAppCamera(): Camera | undefined {
  return useTypeApp((s) => s.camera) as Camera | undefined;
}

export function useTypeAppFocusedPortGuid(): Guid | undefined {
  return useTypeApp((s) => s.focusedPortGuid) as Guid | undefined;
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
      selectRepresentation: noOp,
      deselectRepresentation: noOp,
      hoverPort: noOp,
      hoverRepresentation: noOp,
      clearHover: noOp,
      setSelectedRepresentation: noOp,
      addRepresentationTag: noOp,
      removeRepresentationTag: noOp,
      clearRepresentationTags: noOp,
      setRepresentationTags: noOp,
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
    selectRepresentation: (origin: string, representationId: Guid) => {
      const selection = store.selection;
      const representations = selection.representations || [];
      if (!representations.includes(representationId)) {
        store.change({
          selection: {
            representations: {
              added: [representationId],
            },
          },
        });
      }
    },
    deselectRepresentation: (origin: string, representationId?: Guid) => {
      const selection = store.selection;
      const representations = selection.representations || [];
      if (representationId) {
        if (representations.includes(representationId)) {
          store.change({
            selection: {
              representations: {
                removed: [representationId],
              },
            },
          });
        }
      } else {
        store.change({
          selection: {
            representations: {
              removed: representations,
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
    hoverRepresentation: (origin: string, representationId: Guid) => {
      store.change({
        hover: {
          representation: representationId,
        },
      });
    },
    clearHover: (origin: string) => {
      store.change({
        hover: {
          port: undefined,
          representation: undefined,
        },
      });
    },
    setSelectedRepresentation: (origin: string, representationGuid: Guid) => {
      store.change({
        selectedRepresentationGuid: representationGuid,
      });
    },
    addRepresentationTag: (origin: string, tag: string) => store.execute("semio.typeApp.addRepresentationTag", origin, tag),
    removeRepresentationTag: (origin: string, tag: string) => store.execute("semio.typeApp.removeRepresentationTag", origin, tag),
    clearRepresentationTags: (origin: string) => store.execute("semio.typeApp.clearRepresentationTags", origin),
    setRepresentationTags: (origin: string, tags: string[]) => store.execute("semio.typeApp.setRepresentationTags", origin, tags),
    execute: (origin: string, command: string, ...args: any[]) => store.execute(command, origin, ...args),
  };
}

export function useTypeAppHover(): TypeAppHover | undefined {
  return useTypeApp((s) => s.hover) as TypeAppHover | undefined;
}

export function useTypeAppActiveTool(): ToolKind {
  return useTypeApp((s) => s.activeTool) as ToolKind;
}

export function useTypeAppIsPortSelected(id: TypeAppId | undefined, portId: string): boolean {
  return useTypeApp((s) => s.selection?.ports?.includes(portId) || false, id) as boolean;
}

export function useTypeAppIsPortHovered(id: TypeAppId | undefined, portId: string): boolean {
  return useTypeApp((s) => s.hover?.port === portId, id) as boolean;
}

export function useTypeAppSelectedRepresentationGuid(): Guid | undefined {
  return useTypeApp((s) => s.selectedRepresentationGuid) as Guid | undefined;
}

export function useTypeAppSelectedRepresentationTags(): string[] {
  return useTypeApp((s) => s.selectedRepresentationTags ?? []) as string[];
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
  "semio.typeApp.selectRepresentation": (context: TypeAppCommandContext, reprGuid: Guid): TypeAppCommandResult => {
    return {
      diff: {
        selection: {
          representations: { added: [reprGuid], removed: [] },
        },
      },
    };
  },
  "semio.typeApp.deselectRepresentation": (context: TypeAppCommandContext, reprGuid: Guid): TypeAppCommandResult => {
    return {
      diff: {
        selection: {
          representations: { added: [], removed: [reprGuid] },
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
  "semio.typeApp.hoverRepresentation": (context: TypeAppCommandContext, reprGuid: Guid): TypeAppCommandResult => {
    return {
      diff: {
        hover: { representation: reprGuid },
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
          representations: { removed: context.typeApp.selection?.representations || [] },
        },
      },
    };
  },
  "semio.typeApp.selectAll": (context: TypeAppCommandContext): TypeAppCommandResult => {
    const type = context.kit.types?.find((t) => t.guid === context.Guid);
    const allPorts = type?.ports?.map((p) => p.guid) || [];
    const allRepresentations = type?.representations?.map((r) => r.guid) || [];
    return {
      diff: {
        selection: {
          ports: { added: allPorts },
          representations: { added: allRepresentations },
        },
      },
    };
  },
  "semio.typeApp.addRepresentationTag": (context: TypeAppCommandContext, tag: string): TypeAppCommandResult => {
    const currentTags = context.typeApp.selectedRepresentationTags || [];
    if (currentTags.includes(tag)) {
      return {};
    }
    return {
      diff: {
        selectedRepresentationTags: [...currentTags, tag],
      },
    };
  },
  "semio.typeApp.removeRepresentationTag": (context: TypeAppCommandContext, tag: string): TypeAppCommandResult => {
    const currentTags = context.typeApp.selectedRepresentationTags || [];
    return {
      diff: {
        selectedRepresentationTags: currentTags.filter((t) => t !== tag),
      },
    };
  },
  "semio.typeApp.clearRepresentationTags": (context: TypeAppCommandContext): TypeAppCommandResult => {
    return {
      diff: {
        selectedRepresentationTags: [],
      },
    };
  },
  "semio.typeApp.setRepresentationTags": (context: TypeAppCommandContext, tags: string[]): TypeAppCommandResult => {
    return {
      diff: {
        selectedRepresentationTags: tags,
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
  const position = useMemo(() => [port.point.x, port.point.y, port.point.z] as [number, number, number], [port.point]);
  const direction = useMemo(() => {
    const dir = new THREE.Vector3(port.direction.x, port.direction.y, port.direction.z).normalize();
    return [dir.x, dir.y, dir.z] as [number, number, number];
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
    <Model hovered={isHovered} onClick={handleClick} onDoubleClick={handleDoubleClick} onPointerEnter={handlePointerEnter} onPointerLeave={handlePointerLeave} userData={userData} showEdges={false}>
      <group>
        <Sphere args={[0.03]} position={position}>
          <meshBasicMaterial color={color} />
        </Sphere>
        <Line points={points} color={color} lineWidth={2} />
        <Sphere args={[0.05]} position={endPoint}>
          <meshBasicMaterial color={color} />
        </Sphere>
      </group>
    </Model>
  );
};

const PortPreview: FC<{ position: THREE.Vector3; normal: THREE.Vector3 }> = ({ position, normal }) => {
  const previewColor = "#00ff00";

  // Calculate arrow points for line
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

const TypeMesh: FC<{ activeTool: ToolKind; onPortPreview: (position: THREE.Vector3, normal: THREE.Vector3) => void; onPortCreate: (position: THREE.Vector3, normal: THREE.Vector3) => void; onClearPreview: () => void }> = ({
  activeTool,
  onPortPreview,
  onPortCreate,
  onClearPreview,
}) => {
  const type = useType(undefined, undefined, true) as Type | undefined;
  const kit = useKit(undefined, undefined, true) as Kit | undefined;
  const kitStore = useKitStore() as KitStore;
  const selectedRepresentationGuid = useTypeAppSelectedRepresentationGuid();
  const selectedRepresentationTags = useTypeAppSelectedRepresentationTags();
  const [isPointerDown, setIsPointerDown] = useState(false);
  const pointerDownTimeRef = useRef<number>(0);
  const pointerDownPositionRef = useRef<{ x: number; y: number }>({ x: 0, y: 0 });

  const [blobUrl, setBlobUrl] = useState<string | null>(null);

  const { representationUrl, fileExtension, fileGuid } = useMemo(() => {
    if (!type?.representations || type.representations.length === 0) {
      return { representationUrl: null, fileExtension: "", fileGuid: null };
    }

    let representation: Representation | undefined;

    if (selectedRepresentationGuid) {
      representation = type.representations.find((r) => r.guid === selectedRepresentationGuid);
    } else if (selectedRepresentationTags.length > 0) {
      representation = selectBestRepresentation(type.representations, selectedRepresentationTags);
    } else {
      const defaultRep = type.representations.find((r) => !r.tags || r.tags.length === 0);
      representation = defaultRep ?? type.representations[0];
    }

    if (!representation) {
      return { representationUrl: null, fileExtension: "", fileGuid: null };
    }

    const file = kit?.files?.find((f) => f.guid === representation.file);
    if (!file) {
      return { representationUrl: null, fileExtension: "", fileGuid: null };
    }

    const ext = file.name?.split(".").pop() || "";

    const url = kitStore.getFileUrl(file.guid);
    if (!url) {
      return { representationUrl: null, fileExtension: ext, fileGuid: file.guid };
    }

    return { representationUrl: url, fileExtension: ext, fileGuid: file.guid };
  }, [type, kit, kitStore, selectedRepresentationGuid, selectedRepresentationTags]);

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

  if (!blobUrl) {
    return null; // No placeholder - just render nothing if no valid blob URL yet
  }

  return (
    <Suspense fallback={null}>
      <LoadedTypeMesh url={blobUrl} fileExtension={fileExtension} onPointerDown={handlePointerDown} onPointerUp={handlePointerUp} onPointerMove={handlePointerMove} onPointerOut={handlePointerOut} />
    </Suspense>
  );
};

const SceneContent: FC = () => {
  const activeTool = useTypeAppActiveTool();
  const type = useType() as Type | undefined;
  const kit = useKit();
  const kitCommands = useKitCommands();
  const selection = useTypeAppSelection();
  const hover = useTypeAppHover();
  const appState = useTypeApp((s) => s);
  const { selectPort, deselectPort, hoverPort, clearHover, focusPort } = useTypeAppCommands();
  const [portPreview, setPortPreview] = useState<{ position: THREE.Vector3; normal: THREE.Vector3 } | null>(null);
  const focusContext = useFocusSafe();
  const prevItemsRef = useRef<string>("");

  // Set focus items for navbar
  useEffect(() => {
    if (!focusContext || !type?.ports) return;
    const items = type.ports.map((port) => ({
      id: port.guid,
      label: port.description || `Port ${port.guid.substring(0, 8)}`,
      category: "Ports",
    }));
    const itemsKey = items.map((item) => `${item.id}:${item.label}`).join("|");
    if (prevItemsRef.current !== itemsKey) {
      prevItemsRef.current = itemsKey;
      focusContext.setFocusItems(items);
    }
  }, [focusContext, type?.ports]);

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

  const currentTool = useMemo(() => TypeAppTools.find((tool) => tool.id === activeTool), [activeTool]);

  const toolContribution = useMemo(() => {
    if (!currentTool || !appState) return null;
    return currentTool.render({
      state: appState,
    });
  }, [currentTool, appState]);

  const handlePortPreview = useCallback((position: THREE.Vector3, normal: THREE.Vector3) => {
    setPortPreview({ position, normal });
  }, []);

  const handlePortCreate = useCallback(
    (position: THREE.Vector3, normal: THREE.Vector3) => {
      if (type && kit) {
        const newPort: Port = {
          guid: guid(),
          point: {
            x: position.x,
            y: position.y,
            z: position.z,
          } as Point,
          direction: {
            x: normal.x,
            y: normal.y,
            z: normal.z,
          } as Vector,
          t: 0,
          mandatory: false,
        };

        if (kitCommands) {
          kitCommands.updateType("semio.sketchpad.app.type.canvas.scene.addPort", type.guid, {
            ports: {
              added: [newPort],
            },
          });
        }
      }
    },
    [type, kit, kitCommands],
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
      {toolContribution?.scene || (
        <>
          <TypeMesh activeTool={activeTool} onPortPreview={handlePortPreview} onPortCreate={handlePortCreate} onClearPreview={handleClearPreview} />
          {type?.ports?.map((port) => {
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
      )}
    </>
  );
};

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

export const RepresentationsSection: FC = () => {
  const isInTypeScope = useIsInTypeScope();
  if (!isInTypeScope) return null;
  return <RepresentationsSectionForm />;
};

const RepresentationsSectionForm: FC = () => {
  const tooltip = useTooltip();
  const { selectRepresentation, deselectRepresentation, hoverRepresentation, clearHover } = useTypeAppCommands();
  const kitCommands = useKitCommands();
  const type = useType(undefined, undefined, true) as Type;
  const selection = useTypeAppSelection();
  const hover = useTypeAppHover();

  const applyDiff = (origin: string, diff: any) => {
    kitCommands?.updateType(origin, type.guid, diff);
  };

  const updateRepresentation = (origin: string, id: string, representationDiff: any) => {
    applyDiff(origin, {
      representations: {
        updated: [{ id, diff: representationDiff }],
      },
    });
  };

  const hasRepresentations = type.representations && type.representations.length > 0;

  return (
    <>
      <TreeItem
        id="semio.sketchpad.app.type.representations"
        actions={[
          {
            icon: <AddIcon />,
            onClick: () => {
              const origin = "semio.sketchpad.app.type.panel.details.representations.add";
              applyDiff(origin, {
                representations: {
                  added: [{ guid: guid(), url: "", tags: [] }],
                },
              });
            },
            id: "semio.sketchpad.common.add",
          },
        ]}
      >
        {hasRepresentations && (
          <SortableTreeItems
            items={(type.representations || []).map((representation: any, index: number) => ({
              ...representation,
              id: `representation-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              if (!type.representations) return;
              const origin = "semio.sketchpad.app.type.panel.details.representations.reorder";
              applyDiff(origin, {
                representations: {
                  removed: type.representations.map((representation: any) => representation.guid),
                  added: arrayMove(type.representations, oldIndex, newIndex),
                },
              });
            }}
          >
            {(representation, index) => {
              const isSelected = selection?.representations?.includes(representation.guid) || false;
              const isHovered = hover?.representation === representation.guid;
              return (
                <div
                  key={`representation-${index}`}
                  onPointerEnter={() => hoverRepresentation("semio.sketchpad.app.type.panel.details.representation.hover", representation.guid)}
                  onPointerLeave={() => clearHover("semio.sketchpad.app.type.panel.details.representation.leave")}
                  onClick={() =>
                    isSelected ? deselectRepresentation("semio.sketchpad.app.type.panel.details.representation.deselect", representation.guid) : selectRepresentation("semio.sketchpad.app.type.panel.details.representation.select", representation.guid)
                  }
                >
                  <TreeItem
                    key={`representation-${index}`}
                    id="semio.sketchpad.app.type.representation"
                    label={representation.url}
                    sortable={true}
                    sortableId={`representation-${index}`}
                    isDragHandle={true}
                    className={`${isSelected ? "bg-accent/20" : ""} ${isHovered ? "bg-hover" : ""}`}
                    actions={[
                      {
                        icon: <RemoveIcon />,
                        onClick: () => {
                          const origin = "semio.sketchpad.app.type.panel.details.representations.remove";
                          applyDiff(origin, {
                            representations: {
                              removed: [representation.guid],
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
                          id="semio.sketchpad.app.type.panel.details.section.representations.url"
                          value={representation.url}
                          onChange={(e) => {
                            updateRepresentation("semio.sketchpad.app.type.panel.details.section.representations.url", representation.guid, { url: e.target.value });
                          }}
                          showLabel
                        />
                      </TreeContent>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <Textarea
                          id="semio.sketchpad.app.type.panel.details.section.representations.description"
                          value={representation.description || ""}
                          placeholderId="semio.sketchpad.app.type.representationDescriptionPlaceholder.label"
                          onChange={(e) => {
                            updateRepresentation("semio.sketchpad.app.type.panel.details.section.representations.description", representation.guid, { description: e.target.value });
                          }}
                          showLabel
                        />
                      </TreeContent>
                    </TreeItem>
                    <TreeItem>
                      <TreeContent>
                        <Input
                          id="semio.sketchpad.app.type.panel.details.section.representations.tags"
                          value={(representation.tags || []).join(", ")}
                          placeholderId="semio.sketchpad.app.type.representationTagsPlaceholder.label"
                          onChange={(e) => {
                            updateRepresentation("semio.sketchpad.app.type.panel.details.section.representations.tags", representation.guid, {
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
  const { startTransaction, finalizeTransaction, abortTransaction } = kitCommands || {};

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
                    label={port.family}
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
                          id="semio.sketchpad.app.type.panel.details.section.ports.family"
                          value={port.family || ""}
                          placeholderId="semio.sketchpad.app.type.portFamilyPlaceholder.label"
                          onLazyChange={(value: string) => {
                            updatePort("semio.sketchpad.app.type.panel.details.section.ports.family", port.guid, { family: value });
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
                          transaction={{
                            start: () => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t"),
                            finalize: () => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t"),
                            abort: () => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t"),
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
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.x", port.guid, { point: { x: value } });
                            }}
                            transaction={{
                              start: () => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.x"),
                              finalize: () => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.x"),
                              abort: () => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.x"),
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
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.y", port.guid, { point: { y: value } });
                            }}
                            transaction={{
                              start: () => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.y"),
                              finalize: () => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.y"),
                              abort: () => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.y"),
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
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.z", port.guid, { point: { z: value } });
                            }}
                            transaction={{
                              start: () => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.z"),
                              finalize: () => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.z"),
                              abort: () => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.z"),
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
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.x", port.guid, { direction: { x: value } });
                            }}
                            transaction={{
                              start: () => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.x"),
                              finalize: () => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.x"),
                              abort: () => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.x"),
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
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.y", port.guid, { direction: { y: value } });
                            }}
                            transaction={{
                              start: () => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.y"),
                              finalize: () => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.y"),
                              abort: () => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.y"),
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
                              updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.z", port.guid, { direction: { z: value } });
                            }}
                            transaction={{
                              start: () => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.z"),
                              finalize: () => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.z"),
                              abort: () => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.z"),
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
                          id="semio.sketchpad.app.type.panel.details.section.ports.compatibleFamilies"
                          value={(port.compatibleFamilies || []).join(", ")}
                          placeholderId="semio.sketchpad.app.type.portCompatibleFamiliesPlaceholder.label"
                          onLazyChange={(value: string) => {
                            updatePort("semio.sketchpad.app.type.panel.details.section.ports.compatibleFamilies", port.guid, {
                              compatibleFamilies: value
                                .split(",")
                                .map((family) => family.trim())
                                .filter((family) => family),
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
    kitCommands?.updateType(origin, type.guid, { authors });
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
              updateAuthors(origin, [...(type.authors || []), newAuthorGuid]);
            },
            id: "semio.sketchpad.common.add",
          },
        ]}
      >
        {hasAuthors && (
          <SortableTreeItems
            items={(type.authors || []).map((authorGuid: string, index: number) => {
              const author = kit.authors?.find((a: Author) => a.guid === authorGuid);
              return {
                id: `author-${index}`,
                index,
                guid: authorGuid,
                name: author?.name || "",
                email: author?.email || "",
              };
            })}
            onReorder={(oldIndex, newIndex) => {
              const origin = "semio.sketchpad.app.type.panel.details.authors.reorder";
              updateAuthors(origin, arrayMove(type.authors!, oldIndex, newIndex));
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
                        (type.authors || []).filter((_, i: number) => i !== index),
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
  const { startTransaction, finalizeTransaction, abortTransaction } = kitCommands || {};

  const port = type.ports?.find((p) => p.guid === portGuid);

  if (!port) {
    return (
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{i18n.t("semio.sketchpad.app.type.portNotFound")}</p>
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
            id="semio.sketchpad.app.type.panel.details.section.ports.family"
            value={port.family || ""}
            placeholderId="semio.sketchpad.app.type.portFamilyPlaceholder.label"
            onLazyChange={(value: string) => {
              updatePort("semio.sketchpad.app.type.panel.details.section.ports.family", port.guid, { family: value });
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
            transaction={{
              start: () => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t"),
              finalize: () => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t"),
              abort: () => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t"),
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
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.x", port.guid, { point: { x: value } });
              }}
              transaction={{
                start: () => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.x"),
                finalize: () => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.x"),
                abort: () => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.x"),
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
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.y", port.guid, { point: { y: value } });
              }}
              transaction={{
                start: () => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.y"),
                finalize: () => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.y"),
                abort: () => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.y"),
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
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.point.z", port.guid, { point: { z: value } });
              }}
              transaction={{
                start: () => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.z"),
                finalize: () => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.z"),
                abort: () => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.point.z"),
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
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.x", port.guid, { direction: { x: value } });
              }}
              transaction={{
                start: () => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.x"),
                finalize: () => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.x"),
                abort: () => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.x"),
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
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.y", port.guid, { direction: { y: value } });
              }}
              transaction={{
                start: () => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.y"),
                finalize: () => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.y"),
                abort: () => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.y"),
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
                updatePort("semio.sketchpad.app.type.panel.details.section.ports.direction.z", port.guid, { direction: { z: value } });
              }}
              transaction={{
                start: () => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.z"),
                finalize: () => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.z"),
                abort: () => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.direction.z"),
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
            id="semio.sketchpad.app.type.panel.details.section.ports.compatibleFamilies"
            value={(port.compatibleFamilies || []).join(", ")}
            placeholderId="semio.sketchpad.app.type.portCompatibleFamiliesPlaceholder.label"
            onLazyChange={(value: string) => {
              updatePort("semio.sketchpad.app.type.panel.details.section.ports.compatibleFamilies", port.guid, {
                compatibleFamilies: value
                  .split(",")
                  .map((family) => family.trim())
                  .filter((family) => family),
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
  const { startTransaction, finalizeTransaction, abortTransaction } = kitCommands || {};

  const ports = type.ports?.filter((p) => portGuids.includes(p.guid)) || [];

  if (ports.length === 0) {
    return (
      <TreeItem>
        <TreeContent>
          <p className="text-sm text-muted-foreground">{i18n.t("semio.sketchpad.app.type.portsNotFound")}</p>
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

  const commonFamily = getCommonValue((p) => p.family);
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
            id="semio.sketchpad.app.type.panel.details.section.ports.family"
            value={commonFamily || ""}
            placeholderId={commonFamily === undefined ? "semio.sketchpad.common.mixedValues" : "semio.sketchpad.app.type.portFamilyPlaceholder.label"}
            onLazyChange={(value) => updatePorts("semio.sketchpad.app.type.panel.details.section.ports.family", { family: value })}
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
            transaction={{
              start: () => startTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t"),
              finalize: () => finalizeTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t"),
              abort: () => abortTransaction?.("semio.sketchpad.app.type.panel.details.section.ports.t"),
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
  const app = useTypeApp((s) => s, typeAppId);
  const { setActiveTool } = useTypeAppCommands(typeAppId);

  if (!kit || !type || !app) return null;

  const activeTool = (app as TypeAppState).activeTool ?? ToolKind.SELECTION_NORMAL;

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
  const app = useTypeApp((s) => s);
  const activeTool = app?.activeTool ?? ToolKind.SELECTION_NORMAL;
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

  useEffect(() => {
    if (appType !== "type") return;

    addSection("toolbar", {
      id: "semio.sketchpad.app.type.tools",
      order: 0,
      content: <ToolsToggleGroup />,
    });

    return () => {
      removeSection("toolbar", "semio.sketchpad.app.type.tools");
    };
  }, [addSection, removeSection, appType]);

  // Dynamic details panel based on selection
  useEffect(() => {
    if (appType !== "type") return;

    const hasPorts = selection?.ports && selection.ports.length > 0;
    const hasMultiplePorts = selection?.ports && selection.ports.length > 1;
    const hasSinglePort = selection?.ports && selection.ports.length === 1;

    // Remove all previous sections
    const portsMultipleId = "semio.sketchpad.app.type.panel.details.section.ports.multipleTitle";

    removeSection("details", "semio.sketchpad.app.type.title");
    removeSection("details", "semio.sketchpad.app.type.port.title");
    removeSection("details", portsMultipleId);
    removeSection("details", "semio.sketchpad.app.kit.title");

    if (hasSinglePort) {
      // Single port selected: show Port section then Type section
      addSection("details", {
        id: "semio.sketchpad.app.type.port.title",
        order: 0,
        content: () => <PortSection portGuid={selection.ports![0]} />,
      });
    } else if (hasMultiplePorts) {
      // Multiple ports selected: show Ports section then Type section
      addSection("details", {
        id: portsMultipleId,
        order: 0,
        content: () => <PortsMultipleSection portGuids={selection.ports!} />,
      });
    }

    // Always show Type section (with all subsections)
    addSection("details", {
      id: "semio.sketchpad.app.type.title",
      order: 50,
      content: () => (
        <>
          <TypeDetails />
          <RepresentationsSection />
          <PortsListSection />
          <AuthorsSection />
          <AttributesSection />
        </>
      ),
    });

    // Always add Kit section at the bottom
    addSection("details", {
      id: "semio.sketchpad.app.kit.title",
      order: 100,
      content: () => (
        <React.Suspense fallback={null}>
          <KitSectionLazy />
        </React.Suspense>
      ),
    });

    return () => {
      removeSection("details", "semio.sketchpad.app.type.title");
      removeSection("details", "semio.sketchpad.app.type.port.title");
      removeSection("details", portsMultipleId);
      removeSection("details", "semio.sketchpad.app.kit.title");
    };
  }, [addSection, removeSection, appType, selection]);

  const type = useType() as Type | undefined;
  const kitCommands = useKitCommands();
  const typeAppCommands = useTypeAppCommands();

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
          createdAt: new Date(),
          updatedAt: new Date(),
        };

        // Create Representation that references the file
        const newRepresentationGuid = guid();
        const newRepresentation: Representation = {
          guid: newRepresentationGuid,
          file: newFileGuid,
          description: file.name,
        };

        // Add file to kit with blob
        await kitCommands.addFile("semio.sketchpad.app.type.panel.details.addFile", newFile, file);

        // Add representation to type
        await kitCommands.updateType("semio.sketchpad.app.type.panel.details.addRepresentation", type.guid, {
          representations: {
            added: [newRepresentation],
          },
        });

        // Select the new representation
        typeAppCommands.setSelectedRepresentation("semio.sketchpad.app.type.dropRepresentation", newRepresentationGuid);
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
  const windowLayout = useTypeApp((s) => s.windowLayout);

  const defaultLayout = useMemo(() => {
    return createDefaultLayout([TypeAppWindowKind.Scene]);
  }, []);

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
        store.change({ windowLayout: config });
      }
    },
    [store],
  );

  return (
    <>
      <TypeAppFooter />
      <Canvas>
        <LayoutCanvas windowConfig={windowConfig} layoutState={windowLayout} onLayoutChange={handleLayoutChange} />
      </Canvas>
    </>
  );
};

const TypeApp: FC = () => {
  return <App />;
};

export default TypeApp;

// #endregion App

// #region Footer

export const TypeAppFooter: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const appType = useAppType();

  useEffect(() => {
    if (appType !== "type") return;

    // TODO: Add type-specific footer items here
    // Example:
    // addFooterItem({
    //   id: "semio.sketchpad.app.type.footer.someAction",
    //   icon: SomeIcon,
    //   label: "Action",
    //   onClick: () => { /* action */ },
    //   order: 0,
    // });

    return () => {
      // removeFooterItem("semio.sketchpad.app.type.footer.someAction");
    };
  }, [appType, addFooterItem, removeFooterItem]);

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
