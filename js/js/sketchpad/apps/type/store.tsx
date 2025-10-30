// #region Header

// store.tsx

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

import React, { createContext, useContext } from "react";
import * as Y from "yjs";
import { Camera, Coord, Guid, TypeDiff } from "../../../semio";
import { TypeStore } from "../../kits/store";
import {
  identitySelector,
  KitCommandContext,
  KitDiffAppEdit,
  KitDiffAppStore,
  KitStore,
  PanelVisibility,
  registerTypeAppStoreFactory,
  SketchpadStore,
  ToolType,
  Transact,
  TypeAppId,
  useKitScope,
  useSketchpadStore,
  useSyncDeep,
  useTypeScope,
  YAttributes,
  YLeafMapNumber,
  YLeafMapString,
  YStringArray,
} from "../../store";
import { commands as typeAppCommands } from "./commands";

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
  activeTool?: ToolType;
  camera?: Camera;
  focusedPortGuid?: Guid | null; // null to clear focus
  selectedRepresentationGuid?: Guid | null; // null to clear selection
}
export interface TypeAppEdit extends KitDiffAppEdit<TypeAppSelectionDiff> {}
export interface TypeAppState {
  fullscreenWindow: TypeAppFullscreenWindow;
  panelVisibility: PanelVisibility;
  activeTool: ToolType;
  selection?: TypeAppSelection;
  hover?: TypeAppHover;
  presence?: TypeAppPresence;
  others: TypeAppPresenceOther[];
  camera?: Camera;
  focusedPortGuid?: Guid; // Currently focused port for camera zoom
  selectedRepresentationGuid?: Guid; // Currently selected representation for display
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
        yMap.set("activeTool", ToolType.SELECTION_NORMAL);
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

    Object.entries(typeAppCommands).forEach(([commandId, command]) => {
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

  get activeTool(): ToolType {
    const value = this.yMap.get("activeTool") as ToolType;
    if (value === undefined) {
      this.transact(() => {
        this.yMap.set("activeTool", ToolType.SELECTION_NORMAL);
      });
      return ToolType.SELECTION_NORMAL;
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

// Register the factory - deferred to avoid circular dependency issues
export function initializeTypeAppStore() {
  registerTypeAppStoreFactory((parent, yMap, transact, id) => new TypeAppStore(parent, yMap, transact, id));
}

// Auto-initialize if this module is imported
if (typeof window !== "undefined") {
  setTimeout(() => initializeTypeAppStore(), 0);
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
  return useSyncDeep<TypeAppState, T>(store as TypeAppStore, selector ? selector : identitySelector);
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
    setActiveTool: (origin: string, tool: ToolType) => {
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
    execute: (origin: string, command: string, ...args: any[]) => store.execute(command, origin, ...args),
  };
}

export function useTypeAppHover(): TypeAppHover | undefined {
  return useTypeApp((s) => s.hover) as TypeAppHover | undefined;
}

export function useTypeAppActiveTool(): ToolType {
  return useTypeApp((s) => s.activeTool) as ToolType;
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

const TypeAppScopeContext = createContext<{ id: string } | undefined>(undefined);
export const TypeAppScopeProvider = (props: { id: string; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(TypeAppScopeContext.Provider, { value }, props.children as any);
};
const useTypeAppScope = () => useContext(TypeAppScopeContext);
