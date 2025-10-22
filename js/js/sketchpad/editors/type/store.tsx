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
  KitDiffEditorEdit,
  KitDiffEditorStore,
  KitStore,
  PanelVisibility,
  registerTypeEditorStoreFactory,
  SketchpadStore,
  ToolType,
  Transact,
  useKitScope,
  useSketchpadStore,
  useSyncDeep,
  useTypeScope,
  YAttributes,
  YLeafMapNumber,
  YLeafMapString,
  YStringArray,
} from "../../store";
import { commands as typeEditorCommands } from "./commands";

type YTypeEditorVal = string | number | boolean | YLeafMapString | YLeafMapNumber | YAttributes | YStringArray;
type YTypeEditor = Y.Map<YTypeEditorVal>;
type YTypeEditors = Y.Map<YTypeEditor>;

export interface TypeEditorId {
  kit: Guid;
  type: Guid;
}
export interface TypeEditorSelection {
  ports?: Guid[];
  representations?: Guid[];
}
export interface TypeEditorSelectionPortsDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface TypeEditorSelectionRepresentationsDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface TypeEditorSelectionDiff {
  ports?: TypeEditorSelectionPortsDiff;
  representations?: TypeEditorSelectionRepresentationsDiff;
}
export enum TypeEditorFullscreenWindow {
  None = "none",
  Ports = "ports",
  Representations = "representations",
}
export interface TypeEditorPresence {
  cursor?: Coord;
  camera?: Camera;
}
export interface TypeEditorHover {
  port?: Guid;
  representation?: Guid;
}
export interface TypeEditorPresenceOther extends TypeEditorPresence {
  name: string;
}
export interface TypeEditorDiff {
  selection?: TypeEditorSelectionDiff;
  presence?: TypeEditorPresence;
  hover?: TypeEditorHover;
  fullscreenWindow?: TypeEditorFullscreenWindow;
  panelVisibility?: Partial<PanelVisibility>;
  activeTool?: ToolType;
  camera?: Camera;
}
export interface TypeEditorEdit extends KitDiffEditorEdit<TypeEditorSelectionDiff> {}
export interface TypeEditorState {
  fullscreenWindow: TypeEditorFullscreenWindow;
  panelVisibility: PanelVisibility;
  activeTool: ToolType;
  selection?: TypeEditorSelection;
  hover?: TypeEditorHover;
  presence?: TypeEditorPresence;
  others: TypeEditorPresenceOther[];
  camera?: Camera;
}

export interface TypeEditorCommandContext extends KitCommandContext {
  typeEditor: TypeEditorState;
  Guid: Guid;
}
export interface TypeEditorCommandResult {
  diff?: TypeEditorDiff;
  typeDiff?: TypeDiff;
}

function inverseTypeEditorSelectionDiff(selection: TypeEditorSelection, diff: TypeEditorSelectionDiff): TypeEditorSelectionDiff {
  const inverse: TypeEditorSelectionDiff = {};
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

class TypeEditorStore extends KitDiffEditorStore<TypeEditorState, TypeEditorDiff, TypeEditorSelectionDiff, TypeEditorEdit, TypeEditorCommandContext, TypeEditorCommandResult> {
  private readonly Guid: TypeEditorId;

  constructor(parent: SketchpadStore, yMap: Y.Map<any>, transact: Transact, id: TypeEditorId) {
    super(parent, yMap, transact);
    this.Guid = id;

    transact(() => {
      if (!yMap.has("fullscreenWindow")) {
        yMap.set("fullscreenWindow", TypeEditorFullscreenWindow.None);
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
      }
    });

    Object.entries(typeEditorCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  type(): TypeStore | undefined {
    return this.parent.kit(this.Guid.kit).type(this.Guid.type);
  }

  kit(): KitStore {
    return this.parent.kit(this.Guid.kit);
  }

  // TypeEditor-specific getters
  get fullscreenWindow(): TypeEditorFullscreenWindow {
    return this.yMap.get("fullscreenWindow") as TypeEditorFullscreenWindow;
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

  get selection(): TypeEditorSelection {
    const selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) return {};

    const result: TypeEditorSelection = {};

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

  get hover(): TypeEditorHover | undefined {
    const hover = this.yMap.get("hover") as Y.Map<string>;
    if (!hover) return undefined;

    const result: TypeEditorHover = {};
    const port = hover.get("port");
    if (port) result.port = port;
    const representation = hover.get("representation");
    if (representation) result.representation = representation;

    return result;
  }

  get presence(): TypeEditorPresence | undefined {
    return undefined;
  }

  get others(): TypeEditorPresenceOther[] {
    return [];
  }

  get camera(): Camera | undefined {
    const cameraStr = this.yMap.get("camera") as string | undefined;
    return cameraStr ? JSON.parse(cameraStr) : undefined;
  }

  protected hash(state: TypeEditorState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): TypeEditorState {
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
      diff: this.diff,
      currentTransactionStack: this.currentTransactionStack,
      pastTransactionsStack: this.pastTransactionsStack,
      camera: this.camera,
    } as any;
  }

  protected inverseSelectionDiff(selection: TypeEditorSelection, diff: TypeEditorSelectionDiff): TypeEditorSelectionDiff {
    return inverseTypeEditorSelectionDiff(selection, diff);
  }

  protected applySelectionDiff = (selectionDiff: TypeEditorSelectionDiff) => {
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

  protected getSelection(): TypeEditorSelection {
    return this.selection;
  }

  change = (diff: TypeEditorDiff) => {
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
    });
  };

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    if (command === "semio.typeEditor.startTransaction") {
      console.log(`Executing (special) command: "${command}"`);
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.typeEditor.finalizeTransaction") {
      console.log(`Executing (special) command: "${command}"`);
      this.finalizeTransaction();
      return {} as T;
    }
    if (command === "semio.typeEditor.abortTransaction") {
      console.log(`Executing (special) command: "${command}"`);
      this.abortTransaction();
      return {} as T;
    }
    if (command === "semio.typeEditor.undo") {
      console.log(`Executing (special) command: "${command}"`);
      this.undo();
      return {} as T;
    }
    if (command === "semio.typeEditor.redo") {
      console.log(`Executing (special) command: "${command}"`);
      this.redo();
      return {} as T;
    }
    console.group(`Executing command: "${command}"`);
    const callback = this.commandRegistry.get(command);
    if (!callback) {
      console.groupEnd();
      throw new Error(`Command "${command}" not found in type editor store`);
    }
    const typeEditor = this.snapshot();
    const kitStore = this.kit();
    const kit = kitStore.snapshot();
    const typeStore = this.type();
    const typeGuid = typeStore?.guid ?? this.Guid.type;

    const context: TypeEditorCommandContext = {
      kit,
      typeEditor,
      Guid: typeGuid,
      fileUrls: kitStore.fileUrls,
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

registerTypeEditorStoreFactory((parent, yMap, transact, id) => new TypeEditorStore(parent, yMap, transact, id));

export function useTypeEditorStore<T>(selector?: (store: TypeEditorStore) => T, id?: TypeEditorId): T | TypeEditorStore | null {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const resolvedKitId = kitScope?.guid ?? id?.kit;
  if (!resolvedKitId) return null;
  const typeScope = useTypeScope();
  const resolvedTypeId = typeScope?.guid ?? id?.type;
  if (!resolvedTypeId) return null;
  const typeEditorStore = store.typeEditor(resolvedKitId, resolvedTypeId);
  return selector ? selector(typeEditorStore) : typeEditorStore;
}

export function useTypeEditor<T>(selector?: (state: TypeEditorState) => T, id?: TypeEditorId): T | TypeEditorState | null {
  const store = useTypeEditorStore(identitySelector, id);
  return useSyncDeep<TypeEditorState, T>(store as TypeEditorStore, selector ? selector : identitySelector);
}

export function useTypeEditorSafe<T>(selector?: (state: TypeEditorState) => T, id?: TypeEditorId): T | TypeEditorState | null {
  try {
    return useTypeEditor(selector, id);
  } catch {
    return null;
  }
}

export function useTypeEditorSelection(): TypeEditorSelection {
  return useTypeEditor((s) => s.selection) as TypeEditorSelection;
}

export function useTypeEditorPanelVisibility(): PanelVisibility {
  return useTypeEditor((s) => s.panelVisibility) as PanelVisibility;
}

export function useTypeEditorOthers(): TypeEditorPresenceOther[] {
  return useTypeEditor((s) => s.others) as TypeEditorPresenceOther[];
}

export function useTypeEditorCamera(): Camera | undefined {
  return useTypeEditor((s) => s.camera) as Camera | undefined;
}

export function useTypeEditorCommands(id?: TypeEditorId) {
  const store = useTypeEditorStore(undefined, id) as TypeEditorStore | null;
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
    startTransaction: () => store.execute("semio.typeEditor.startTransaction"),
    finalizeTransaction: () => store.execute("semio.typeEditor.finalizeTransaction"),
    abortTransaction: () => store.execute("semio.typeEditor.abortTransaction"),
    undo: () => store.execute("semio.typeEditor.undo"),
    redo: () => store.execute("semio.typeEditor.redo"),
    selectAll: () => store.execute("semio.typeEditor.selectAll"),
    deselectAll: () => store.execute("semio.typeEditor.deselectAll"),
    togglePanel: (panelKey: keyof PanelVisibility) => {
      const current = store.snapshot().panelVisibility;
      store.change({
        panelVisibility: {
          [panelKey]: !current[panelKey],
        },
      });
    },
    setCamera: (camera: Camera) => {
      store.change({ camera });
    },
    setActiveTool: (tool: ToolType) => {
      store.change({ activeTool: tool });
    },
    selectPort: (portId: Guid) => {
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
    deselectPort: (portId?: Guid) => {
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
    selectRepresentation: (representationId: Guid) => {
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
    deselectRepresentation: (representationId?: Guid) => {
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
    hoverPort: (portId: Guid) => {
      store.change({
        hover: {
          port: portId,
        },
      });
    },
    hoverRepresentation: (representationId: Guid) => {
      store.change({
        hover: {
          representation: representationId,
        },
      });
    },
    clearHover: () => {
      store.change({
        hover: {
          port: undefined,
          representation: undefined,
        },
      });
    },
    execute: (command: string, ...args: any[]) => store.execute(command, ...args),
  };
}

export function useTypeEditorHover(): TypeEditorHover | undefined {
  return useTypeEditor((s) => s.hover) as TypeEditorHover | undefined;
}

export function useTypeEditorActiveTool(): ToolType {
  return useTypeEditor((s) => s.activeTool) as ToolType;
}

export function useTypeEditorIsPortSelected(id: TypeEditorId | undefined, portId: string): boolean {
  return useTypeEditor((s) => s.selection?.ports?.includes(portId) || false, id) as boolean;
}

export function useTypeEditorIsPortHovered(id: TypeEditorId | undefined, portId: string): boolean {
  return useTypeEditor((s) => s.hover?.port === portId, id) as boolean;
}

const TypeEditorScopeContext = createContext<{ id: string } | undefined>(undefined);
export const TypeEditorScopeProvider = (props: { id: string; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(TypeEditorScopeContext.Provider, { value }, props.children as any);
};
const useTypeEditorScope = () => useContext(TypeEditorScopeContext);
