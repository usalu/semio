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

import { Connection as RFConnection } from "@xyflow/react";
import React, { createContext, useContext } from "react";
import * as Y from "yjs";
import { areSameKit, Camera, Connection, ConnectionDiff, Coord, DiffStatus, Guid, KitDiff, Piece, PieceDiff } from "../../../semio";
import { DesignStore, KitCommandContext, KitStore, useDesignScope, useKitScope } from "../../kits/store";
import {
  identitySelector,
  KitDiffEditorEdit,
  KitDiffEditorStore,
  PanelVisibility,
  registerDesignEditorStoreFactory,
  SketchpadStore,
  ToolType,
  useSketchpadStore,
  useSync,
  useSyncDeep,
  YAttributes,
  YLeafMapNumber,
  YLeafMapString,
  YStringArray,
} from "../../store";
import { commands as designEditorCommands } from "./commands";

type YDesignEditorVal = string | number | boolean | YLeafMapString | YLeafMapNumber | Y.Map<boolean> | YAttributes | YStringArray;
type YDesignEditor = Y.Map<YDesignEditorVal>;
type YDesignEditors = Y.Map<Y.Map<YDesignEditor>>;

export interface DesignEditorId {
  kit: Guid;
  design: Guid;
}
export interface DesignEditorSelection {
  pieces?: Guid[];
  connections?: Guid[];
  port?: { piece: Guid; designPiece?: Guid; port: Guid };
}
export interface DesignEditorSelectionPiecesDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface DesignEditorSelectionConnectionsDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface DesignEditorSelectionPortDiff {
  piece?: Guid;
  designPiece?: Guid;
  port?: Guid;
}
export interface DesignEditorSelectionDiff {
  pieces?: DesignEditorSelectionPiecesDiff;
  connections?: DesignEditorSelectionConnectionsDiff;
  port?: DesignEditorSelectionPortDiff;
}
export enum DesignEditorFullscreenWindow {
  None = "none",
  Diagram = "diagram",
  Accessl = "accessl",
}
export interface DesignEditorPresence {
  cursor?: Coord;
  camera?: Camera;
  diagramCenter?: Coord;
  diagramScale?: number;
}
export interface DesignEditorHover {
  pieces?: Guid[];
  connections?: Guid[];
  ports?: { piece: Guid; designPiece?: Guid; port: Guid }[];
  types?: Guid[];
  designs?: Guid[];
}
export interface DesignEditorPresenceOther extends DesignEditorPresence {
  name: string;
}
export interface DesignEditorDiff {
  selection?: DesignEditorSelectionDiff;
  presence?: DesignEditorPresence;
  hover?: DesignEditorHover;
  fullscreenWindow?: DesignEditorFullscreenWindow;
  panelVisibility?: Partial<PanelVisibility>;
  activeTool?: ToolType;
  camera?: Camera;
  diagramCenter?: Coord;
  diagramScale?: number;
}
export interface DesignEditorEdit extends KitDiffEditorEdit<DesignEditorSelectionDiff> {}
export interface DesignEditorState {
  fullscreenWindow: DesignEditorFullscreenWindow;
  panelVisibility: PanelVisibility;
  activeTool?: ToolType;
  selection?: DesignEditorSelection;
  hover?: DesignEditorHover;
  presence?: DesignEditorPresence;
  others: DesignEditorPresenceOther[];
  camera?: Camera;
  diagramCenter?: Coord;
  diagramScale?: number;
  currentTransactionStackLength?: number; // Added to trigger re-renders when stack changes
}

export interface DesignEditorCommandContext extends KitCommandContext {
  designEditor: DesignEditorState;
  Guid: Guid;
}
export interface DesignEditorCommandResult {
  diff?: DesignEditorDiff;
  kitDiff?: KitDiff;
}

export const inverseDesignEditorSelectionDiff = (selection: DesignEditorSelection, diff: DesignEditorSelectionDiff): DesignEditorSelectionDiff => {
  const inverseDiff: DesignEditorSelectionDiff = {};

  // Inverse pieces diff
  if (diff.pieces) {
    inverseDiff.pieces = {};
    if (diff.pieces.added) {
      inverseDiff.pieces.removed = diff.pieces.added;
    }
    if (diff.pieces.removed) {
      inverseDiff.pieces.added = diff.pieces.removed;
    }
  }

  // Inverse connections diff
  if (diff.connections) {
    inverseDiff.connections = {};
    if (diff.connections.added) {
      inverseDiff.connections.removed = diff.connections.added;
    }
    if (diff.connections.removed) {
      inverseDiff.connections.added = diff.connections.removed;
    }
  }

  // Inverse port diff - restore the original port from selection
  if (diff.port) {
    inverseDiff.port = {
      piece: selection.port?.piece,
      designPiece: selection.port?.designPiece,
      port: selection.port?.port,
    };
  }

  return inverseDiff;
};
export const areSameDesignEditor = (designEditor: DesignEditorId, other: DesignEditorId): boolean => areSameKit(designEditor.kit, other.kit) && designEditor.design === other.design;
export const hasSameDesignEditor = (designEditor: DesignEditorId, others: DesignEditorId[]): boolean => others.some((other) => areSameDesignEditor(designEditor, other));

class DesignEditorStore extends KitDiffEditorStore<DesignEditorState, DesignEditorDiff, DesignEditorSelectionDiff, DesignEditorEdit, DesignEditorCommandContext, DesignEditorCommandResult> {
  constructor(parent: SketchpadStore, yMap: YDesignEditor, transact: (fn: () => void) => void, id: DesignEditorId, state?: DesignEditorState) {
    super(parent, yMap, transact);

    const kit = this.parent.kit(id.kit);
    const design = kit.design(id.design);
    yMap.set("kit", kit.guid);
    yMap.set("design", design.guid);

    // Only initialize if not already set (preserve existing values when reopening)
    if (!yMap.has("fullscreenWindow")) {
      yMap.set("fullscreenWindow", state?.fullscreenWindow || DesignEditorFullscreenWindow.None);
    }

    // Only initialize selection if not already set
    if (!yMap.has("selection")) {
      const selection = new Y.Map<any>();
      const selectedPieces = new Y.Array<Guid>();
      if (state?.selection?.pieces) {
        selectedPieces.push(state.selection.pieces);
      }
      const selectedConnections = new Y.Array<Guid>();
      if (state?.selection?.connections) {
        selectedConnections.push(state.selection.connections);
      }
      const selectionPort = new Y.Map<any>();
      if (state?.selection?.port) {
        selectionPort.set("piece", state.selection.port.piece);
        selectionPort.set("port", state.selection.port.port);
        if (state.selection.port.designPiece) {
          selectionPort.set("designPiece", state.selection.port.designPiece);
        }
      }
      selection.set("pieces", selectedPieces);
      selection.set("connections", selectedConnections);
      selection.set("port", selectionPort);
      yMap.set("selection", selection);
    }

    // Only initialize these if not already set
    if (!yMap.has("isTransactionActive")) {
      yMap.set("isTransactionActive", false);
    }
    if (!yMap.has("presence")) {
      yMap.set("presence", new Y.Map<any>());
    }
    if (!yMap.has("others")) {
      yMap.set("others", new Y.Array<any>());
    }
    if (!yMap.has("diff")) {
      yMap.set("diff", new Y.Map<any>());
    }
    if (!yMap.has("currentTransactionStack")) {
      yMap.set("currentTransactionStack", new Y.Array<any>());
    }
    if (!yMap.has("pastTransactionsStack")) {
      yMap.set("pastTransactionsStack", new Y.Array<any>());
    }
    if (!yMap.has("panelVisibility")) {
      const yPanelVisibility = new Y.Map<boolean>();
      yPanelVisibility.set("toolbar", true);
      yPanelVisibility.set("workbench", true);
      yPanelVisibility.set("details", true);
      yPanelVisibility.set("chat", false);
      yPanelVisibility.set("settings", false);
      yMap.set("panelVisibility", yPanelVisibility as any);
    }

    // Camera, diagramCenter, and diagramScale are already handled by their getters/setters
    // and will be preserved automatically if they exist in the yMap

    Object.entries(designEditorCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  get fullscreenWindow(): DesignEditorFullscreenWindow {
    return this.yMap.get("fullscreenWindow") as DesignEditorFullscreenWindow;
  }
  set fullscreenWindow(panel: DesignEditorFullscreenWindow) {
    this.yMap.set("fullscreenWindow", panel);
  }
  get activeTool(): ToolType {
    return (this.yMap.get("activeTool") as ToolType) ?? ToolType.SELECTION_NORMAL;
  }
  set activeTool(tool: ToolType) {
    this.yMap.set("activeTool", tool);
  }
  get panelVisibility(): PanelVisibility {
    const yPanelVisibility = this.yMap.get("panelVisibility") as Y.Map<boolean>;
    if (!yPanelVisibility) {
      return {
        toolbar: true,
        workbench: true,
        details: true,
        chat: false,
        settings: false,
      };
    }
    return {
      toolbar: yPanelVisibility.get("toolbar") ?? true,
      workbench: yPanelVisibility.get("workbench") ?? true,
      details: yPanelVisibility.get("details") ?? true,
      chat: yPanelVisibility.get("chat") ?? false,
      settings: yPanelVisibility.get("settings") ?? false,
    };
  }
  get selection(): DesignEditorSelection {
    const selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) return {};

    const result: DesignEditorSelection = {};

    // Get pieces
    const pieces = selection.get("pieces") as Y.Array<string>;
    if (pieces && pieces.length > 0) {
      result.pieces = pieces.toArray();
    }

    // Get connections
    const connections = selection.get("connections") as Y.Array<string>;
    if (connections && connections.length > 0) {
      result.connections = connections.toArray();
    }

    // Get port
    const port = selection.get("port") as Y.Map<string>;
    if (port) {
      const piece = port.get("piece");
      const designPiece = port.get("designPiece");
      const portId = port.get("port");

      if (piece && portId) {
        result.port = {
          piece: piece,
          designPiece: designPiece,
          port: portId,
        };
      }
    }

    return result;
  }
  get presence(): DesignEditorPresence {
    return {
      cursor: {
        x: (this.yMap.get("presenceCursorX") as number) || 0,
        y: (this.yMap.get("presenceCursorY") as number) || 0,
      },
    };
  }
  get others(): DesignEditorPresenceOther[] {
    return [];
  }
  get diff(): KitDiff {
    return {};
  }
  get hover(): DesignEditorHover | undefined {
    const hover = this.yMap.get("hover") as Y.Map<any> | undefined;
    if (!hover) return undefined;
    const result: DesignEditorHover = {};
    const pieces = hover.get("pieces") as Y.Array<Guid> | undefined;
    const connections = hover.get("connections") as Y.Array<Guid> | undefined;
    const ports = hover.get("ports") as Y.Array<Y.Map<string>> | undefined;
    const types = hover.get("types") as Y.Array<Guid> | undefined;
    const designs = hover.get("designs") as Y.Array<Guid> | undefined;
    if (pieces && pieces.length > 0) result.pieces = pieces.toArray();
    if (connections && connections.length > 0) result.connections = connections.toArray();
    if (ports && ports.length > 0) {
      result.ports = ports.toArray().map((yPort) => ({
        piece: yPort.get("piece") as Guid,
        designPiece: yPort.get("designPiece") as Guid | undefined,
        port: yPort.get("port") as Guid,
      }));
    }
    if (types && types.length > 0) result.types = types.toArray();
    if (designs && designs.length > 0) result.designs = designs.toArray();
    return Object.keys(result).length > 0 ? result : undefined;
  }

  get camera(): Camera | undefined {
    const cameraStr = this.yMap.get("camera") as string | undefined;
    return cameraStr ? JSON.parse(cameraStr) : undefined;
  }

  get diagramCenter(): Coord | undefined {
    const centerStr = this.yMap.get("diagramCenter") as string | undefined;
    return centerStr ? JSON.parse(centerStr) : undefined;
  }

  get diagramScale(): number | undefined {
    return this.yMap.get("diagramScale") as number | undefined;
  }

  kit(): KitStore {
    return this.parent.kit(this.yMap.get("kit") as string);
  }

  design(): DesignStore {
    return this.kit().design(this.yMap.get("design") as string);
  }

  protected getSelection(): DesignEditorSelection {
    return this.selection;
  }

  protected hash(state: DesignEditorState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): DesignEditorState {
    return {
      fullscreenWindow: this.fullscreenWindow,
      panelVisibility: this.panelVisibility,
      activeTool: this.activeTool,
      selection: this.selection,
      hover: this.hover,
      presence: this.presence,
      others: this.others,
      camera: this.camera,
      diagramCenter: this.diagramCenter,
      diagramScale: this.diagramScale,
      currentTransactionStackLength: this.currentTransactionStack.length, // Include stack length in snapshot
    };
  }

  protected inverseSelectionDiff(selection: DesignEditorSelection, diff: DesignEditorSelectionDiff): DesignEditorSelectionDiff {
    return inverseDesignEditorSelectionDiff(selection, diff);
  }

  protected applySelectionDiff = (selectionDiff: DesignEditorSelectionDiff) => {
    let selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) {
      selection = new Y.Map();
      this.yMap.set("selection", selection);
    }

    // Apply pieces diff
    if (selectionDiff.pieces) {
      let pieces = (selection.get("pieces") as Y.Array<Guid>) || new Y.Array<Guid>();
      if (!selection.has("pieces")) {
        selection.set("pieces", pieces);
      }

      if (selectionDiff.pieces.added) {
        for (const piece of selectionDiff.pieces.added) {
          if (!pieces.toArray().includes(piece)) {
            pieces.push([piece]);
          }
        }
      }
      if (selectionDiff.pieces.removed) {
        for (const piece of selectionDiff.pieces.removed) {
          const index = pieces.toArray().indexOf(piece);
          if (index !== -1) {
            pieces.delete(index, 1);
          }
        }
      }
    }

    // Apply connections diff
    if (selectionDiff.connections) {
      let connections = (selection.get("connections") as Y.Array<Guid>) || new Y.Array<Guid>();
      if (!selection.has("connections")) {
        selection.set("connections", connections);
      }

      if (selectionDiff.connections.added) {
        for (const connectionGuid of selectionDiff.connections.added) {
          if (!connections.toArray().includes(connectionGuid)) {
            connections.push([connectionGuid]);
          }
        }
      }
      if (selectionDiff.connections.removed) {
        for (const connectionGuid of selectionDiff.connections.removed) {
          const index = connections.toArray().indexOf(connectionGuid);
          if (index !== -1) {
            connections.delete(index, 1);
          }
        }
      }
    }

    // Apply port diff
    if (selectionDiff.port) {
      const portSelection = new Y.Map();
      if (selectionDiff.port.piece !== undefined) {
        portSelection.set("piece", selectionDiff.port.piece);
      }
      if (selectionDiff.port.designPiece !== undefined) {
        portSelection.set("designPiece", selectionDiff.port.designPiece);
      }
      if (selectionDiff.port.port !== undefined) {
        portSelection.set("port", selectionDiff.port.port);
      }
      selection.set("port", portSelection);
    }
  };

  change = (diff: DesignEditorDiff) => {
    this.transact(() => {
      if (diff.fullscreenWindow) this.fullscreenWindow = diff.fullscreenWindow;
      if (diff.activeTool) this.activeTool = diff.activeTool;
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
      if (diff.presence) {
        // Handle presence changes if needed
      }
      if (diff.hover) {
        if (Object.keys(diff.hover).length === 0) {
          this.yMap.delete("hover");
        } else {
          let yHover = this.yMap.get("hover") as Y.Map<any>;
          if (!yHover) {
            yHover = new Y.Map<any>();
            this.yMap.set("hover", yHover);
          }
          if (Object.prototype.hasOwnProperty.call(diff.hover, "pieces")) {
            const piecesValue = diff.hover.pieces;
            if (piecesValue && piecesValue.length > 0) {
              const yPieces = new Y.Array<Guid>();
              yPieces.push(piecesValue);
              yHover.set("pieces", yPieces);
            } else {
              yHover.delete("pieces");
            }
          }
          if (Object.prototype.hasOwnProperty.call(diff.hover, "connections")) {
            const connectionsValue = diff.hover.connections;
            if (connectionsValue && connectionsValue.length > 0) {
              const yConnections = new Y.Array<Guid>();
              yConnections.push(connectionsValue);
              yHover.set("connections", yConnections);
            } else {
              yHover.delete("connections");
            }
          }
          if (Object.prototype.hasOwnProperty.call(diff.hover, "ports")) {
            const portsValue = diff.hover.ports;
            if (portsValue && portsValue.length > 0) {
              const yPorts = new Y.Array<any>();
              portsValue.forEach((port) => {
                const yPort = new Y.Map<string>();
                yPort.set("piece", port.piece);
                if (port.designPiece) yPort.set("designPiece", port.designPiece);
                yPort.set("port", port.port);
                yPorts.push([yPort]);
              });
              yHover.set("ports", yPorts);
            } else {
              yHover.delete("ports");
            }
          }
          if (Object.prototype.hasOwnProperty.call(diff.hover, "types")) {
            const typesValue = diff.hover.types;
            if (typesValue && typesValue.length > 0) {
              const yTypes = new Y.Array<Guid>();
              yTypes.push(typesValue);
              yHover.set("types", yTypes);
            } else {
              yHover.delete("types");
            }
          }
          if (Object.prototype.hasOwnProperty.call(diff.hover, "designs")) {
            const designsValue = diff.hover.designs;
            if (designsValue && designsValue.length > 0) {
              const yDesigns = new Y.Array<Guid>();
              yDesigns.push(designsValue);
              yHover.set("designs", yDesigns);
            } else {
              yHover.delete("designs");
            }
          }
        }
      }
      if (diff.camera) {
        this.yMap.set("camera", JSON.stringify(diff.camera));
      }
      if (diff.diagramCenter) {
        this.yMap.set("diagramCenter", JSON.stringify(diff.diagramCenter));
      }
      if (diff.diagramScale !== undefined) {
        this.yMap.set("diagramScale", diff.diagramScale);
      }
    });
  };

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    if (command === "semio.designEditor.startTransaction") {
      console.log(`Executing (special) command: "${command}"`);
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.designEditor.finalizeTransaction") {
      console.log(`Executing (special) command: "${command}"`);
      this.finalizeTransaction();
      return {} as T;
    }
    if (command === "semio.designEditor.abortTransaction") {
      console.log(`Executing (special) command: "${command}"`);
      this.abortTransaction();
      return {} as T;
    }
    if (command === "semio.designEditor.undo") {
      console.log(`Executing (special) command: "${command}"`);
      this.undo();
      return {} as T;
    }
    if (command === "semio.designEditor.redo") {
      console.log(`Executing (special) command: "${command}"`);
      this.redo();
      return {} as T;
    }

    console.group(`Executing command: "${command}"`);
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in design editor store`);

    const kitStore = this.kit();
    const state = this.snapshot();
    const kitState = kitStore.snapshot();

    const context: DesignEditorCommandContext = {
      designEditor: state,
      kit: kitState,
      Guid: this.design().guid,
      fileUrls: kitStore.fileUrls,
    };
    const result = callback(context, ...rest);
    if (result.diff) {
      this.change(result.diff);
    }
    if (result.kitDiff) {
      kitStore.change(result.kitDiff);
    }
    this.recordEdit(result);
    console.groupEnd();
    return result as T;
  }

  execute<T>(command: string, ...rest: any[]): Promise<T> {
    return this.executeCommand(command, ...rest);
  }
}

registerDesignEditorStoreFactory((parent, yMap, transact, id, state) => new DesignEditorStore(parent, yMap as any, transact, id, state));

type DesignEditorScope = { id: string };
const DesignEditorScopeContext = createContext<DesignEditorScope | null>(null);
export const DesignEditorScopeProvider = (props: { id: string; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(DesignEditorScopeContext.Provider, { value }, props.children as any);
};
const useDesignEditorScope = () => useContext(DesignEditorScopeContext);

export function useDesignEditorStore<T>(selector?: (store: DesignEditorStore) => T, id?: DesignEditorId): T | DesignEditorStore | null {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const resolvedKitId = kitScope?.guid ?? id?.kit;
  const resolvedDesignId = designScope?.guid ?? id?.design;
  if (!resolvedKitId || !resolvedDesignId) {
    return null;
  }
  const designEditorStore = store.designEditor(resolvedKitId, resolvedDesignId);
  return selector ? selector(designEditorStore) : designEditorStore;
}

export function useDesignEditor<T>(selector?: (state: DesignEditorState) => T, id?: DesignEditorId): T | DesignEditorState | null {
  const store = useDesignEditorStore(identitySelector, id);
  return useSyncDeep<DesignEditorState, T>(store as DesignEditorStore, selector ? selector : identitySelector);
}

export function useDesignEditorSafe<T>(selector?: (state: DesignEditorState) => T, id?: DesignEditorId): T | DesignEditorState | null {
  try {
    return useDesignEditor(selector, id);
  } catch {
    return null;
  }
}

export function useDesignEditorSelection(): DesignEditorSelection {
  return useDesignEditor((s) => s.selection) as DesignEditorSelection;
}

export function useDesignEditorFullscreen(): DesignEditorFullscreenWindow {
  return useDesignEditor((s) => s.fullscreenWindow) as DesignEditorFullscreenWindow;
}

// TODO: DesignEditorState doesn't have a diff property - this needs to be rethought
export function useDesignEditorDiff(): KitDiff | undefined {
  // return useDesignEditor((s) => s.diff) as KitDiff;
  return undefined;
}

export function useDesignEditorOthers(): DesignEditorPresenceOther[] {
  return useDesignEditor((s) => s.others) as DesignEditorPresenceOther[];
}

export function useDesignEditorCamera(): Camera | undefined {
  return useDesignEditor((s) => s.camera) as Camera | undefined;
}

export function useDesignEditorDiagramCenter(): Coord | undefined {
  return useDesignEditor((s) => s.diagramCenter) as Coord | undefined;
}

export function useDesignEditorDiagramScale(): number | undefined {
  return useDesignEditor((s) => s.diagramScale) as number | undefined;
}

export function useDesignEditorHover(): DesignEditorHover | undefined {
  return useDesignEditor((s) => s.hover) as DesignEditorHover | undefined;
}

export function useDesignEditorSelectionSafe(): DesignEditorSelection | undefined {
  try {
    return useDesignEditorSelection();
  } catch {
    return undefined;
  }
}

export function useDesignEditorHoverSafe(): DesignEditorHover | undefined {
  try {
    return useDesignEditorHover();
  } catch {
    return undefined;
  }
}

export function useDesignEditorCommandsSafe(id?: DesignEditorId) {
  try {
    return useDesignEditorCommands(id);
  } catch {
    return {
      startTransaction: () => {},
      finalizeTransaction: () => {},
      abortTransaction: () => {},
      undo: () => {},
      redo: () => {},
      selectAll: () => Promise.resolve(),
      deselectAll: () => Promise.resolve(),
      selectPiece: () => Promise.resolve(),
      selectPieces: () => Promise.resolve(),
      addPieceToSelection: () => Promise.resolve(),
      removePieceFromSelection: () => Promise.resolve(),
      selectConnection: () => Promise.resolve(),
      addConnectionToSelection: () => Promise.resolve(),
      removeConnectionFromSelection: () => Promise.resolve(),
      selectPiecePort: () => Promise.resolve(),
      deselectPiecePort: () => Promise.resolve(),
      deleteSelected: () => Promise.resolve(),
      toggleDiagramFullscreen: () => Promise.resolve(),
      toggleAccesslFullscreen: () => Promise.resolve(),
      setActiveTool: () => Promise.resolve(),
      addPiece: () => Promise.resolve(),
      addPieces: () => Promise.resolve(),
      removePiece: () => Promise.resolve(),
      removePieces: () => Promise.resolve(),
      addConnection: () => Promise.resolve(),
      addConnections: () => Promise.resolve(),
      removeConnection: () => Promise.resolve(),
      removeConnections: () => Promise.resolve(),
      updatePiece: () => Promise.resolve(),
      updatePieces: () => Promise.resolve(),
      updateConnection: () => Promise.resolve(),
      updateConnections: () => Promise.resolve(),
      setCamera: () => Promise.resolve(),
      setDiagramCenter: () => Promise.resolve(),
      setDiagramScale: () => Promise.resolve(),
      hoverPiece: () => Promise.resolve(),
      hoverPieces: () => Promise.resolve(),
      hoverConnection: () => Promise.resolve(),
      hoverConnections: () => Promise.resolve(),
      hoverPort: () => Promise.resolve(),
      hoverType: () => Promise.resolve(),
      hoverTypes: () => Promise.resolve(),
      hoverDesign: () => Promise.resolve(),
      hoverDesigns: () => Promise.resolve(),
      clearHover: () => Promise.resolve(),
      togglePanel: () => {},
      execute: () => Promise.resolve(),
    };
  }
}

export function useDesignEditorCommands(id?: DesignEditorId) {
  const store = useDesignEditorStore(undefined, id) as DesignEditorStore;
  return {
    startTransaction: () => store.startTransaction(),
    finalizeTransaction: () => store.finalizeTransaction(),
    abortTransaction: () => store.abortTransaction(),
    undo: () => store.undo(),
    redo: () => store.redo(),
    selectAll: () => store.execute("semio.designEditor.selectAll"),
    deselectAll: () => store.execute("semio.designEditor.deselectAll"),
    selectPiece: (guid: Guid) => store.execute("semio.designEditor.selectPiece", guid),
    selectPieces: (guids: Guid[]) => store.execute("semio.designEditor.selectPieces", guids),
    addPieceToSelection: (guid: Guid) => store.execute("semio.designEditor.addPieceToSelection", guid),
    removePieceFromSelection: (guid: Guid) => store.execute("semio.designEditor.removePieceFromSelection", guid),
    selectConnection: (connectionGuid: Guid) => store.execute("semio.designEditor.selectConnection", connectionGuid),
    addConnectionToSelection: (connectionGuid: Guid) => store.execute("semio.designEditor.addConnectionToSelection", connectionGuid),
    removeConnectionFromSelection: (connectionGuid: Guid) => store.execute("semio.designEditor.removeConnectionFromSelection", connectionGuid),
    selectPiecePort: (piece: Guid, port: Guid) => store.execute("semio.designEditor.selectPiecePort", piece, port),
    deselectPiecePort: () => store.execute("semio.designEditor.deselectPiecePort"),
    deleteSelected: () => store.execute("semio.designEditor.deleteSelected"),
    toggleDiagramFullscreen: () => store.execute("semio.designEditor.toggleDiagramFullscreen"),
    toggleAccesslFullscreen: () => store.execute("semio.designEditor.toggleAccesslFullscreen"),
    setActiveTool: (tool: ToolType) => store.execute("semio.designEditor.setActiveTool", tool),
    addPiece: (piece: Piece) => store.execute("semio.designEditor.addPiece", piece),
    addPieces: (pieces: Piece[]) => store.execute("semio.designEditor.addPieces", pieces),
    removePiece: (piece: Guid) => store.execute("semio.designEditor.removePiece", piece),
    removePieces: (pieces: Guid[]) => store.execute("semio.designEditor.removePieces", pieces),
    addConnection: (connection: Connection) => store.execute("semio.designEditor.addConnection", connection),
    addConnections: (connections: Connection[]) => store.execute("semio.designEditor.addConnections", connections),
    removeConnection: (connection: Guid) => store.execute("semio.designEditor.removeConnection", connection),
    removeConnections: (connections: Guid[]) => store.execute("semio.designEditor.removeConnections", connections),
    updatePiece: (piece: Guid, pieceDiff: PieceDiff) => store.execute("semio.designEditor.updatePiece", piece, pieceDiff),
    updatePieces: (updates: { id: Guid; diff: PieceDiff }[]) => store.execute("semio.designEditor.updatePieces", updates),
    updateConnection: (connection: Guid, connectionDiff: ConnectionDiff) => store.execute("semio.designEditor.updateConnection", connection, connectionDiff),
    updateConnections: (updates: { id: Guid; diff: ConnectionDiff }[]) => store.execute("semio.designEditor.updateConnections", updates),
    setCamera: (camera: Camera) => store.execute("semio.designEditor.setCamera", camera),
    setDiagramCenter: (center: Coord) => store.execute("semio.designEditor.setDiagramCenter", center),
    setDiagramScale: (scale: number) => store.execute("semio.designEditor.setDiagramScale", scale),
    hoverPiece: (guid: Guid) => store.execute("semio.designEditor.hoverPiece", guid),
    hoverPieces: (guids: Guid[]) => store.execute("semio.designEditor.hoverPieces", guids),
    hoverConnection: (guid: Guid) => store.execute("semio.designEditor.hoverConnection", guid),
    hoverConnections: (guids: Guid[]) => store.execute("semio.designEditor.hoverConnections", guids),
    hoverPort: (pieceGuid: Guid, portGuid: Guid) => store.execute("semio.designEditor.hoverPort", pieceGuid, portGuid),
    hoverType: (guid: Guid) => store.execute("semio.designEditor.hoverType", guid),
    hoverTypes: (guids: Guid[]) => store.execute("semio.designEditor.hoverTypes", guids),
    hoverDesign: (guid: Guid) => store.execute("semio.designEditor.hoverDesign", guid),
    hoverDesigns: (guids: Guid[]) => store.execute("semio.designEditor.hoverDesigns", guids),
    clearHover: () => store.execute("semio.designEditor.clearHover"),
    togglePanel: (panelKey: keyof PanelVisibility) => {
      const current = store.snapshot().panelVisibility;
      store.change({
        panelVisibility: {
          [panelKey]: !current[panelKey],
        },
      });
    },
    execute: (command: string, ...args: any[]) => store.execute(command, ...args),
  };
}

export function useIsDesignPieceChangedInTransaction(id: DesignEditorId | undefined, pieceId: string) {
  const store = useDesignEditorStore(identitySelector, id) as DesignEditorStore;
  return useSync<DesignEditorState, boolean>(
    store,
    (state) => {
      const currentStack = store?.currentTransactionStack;
      if (!currentStack || currentStack.length === 0) {
        return false;
      }

      // Check if piece is in any edit in current transaction
      for (const edit of currentStack) {
        if (edit.do?.kitDiff?.designs) {
          for (const designUpdate of edit.do.kitDiff.designs.updated || []) {
            // Check updated pieces
            if (designUpdate.diff.pieces?.updated) {
              for (const pieceUpdate of designUpdate.diff.pieces.updated) {
                if (pieceUpdate.id === pieceId) {
                  return true;
                }
              }
            }
            // Check added pieces
            if (designUpdate.diff.pieces?.added) {
              for (const piece of designUpdate.diff.pieces.added) {
                if (piece.guid === pieceId) {
                  return true;
                }
              }
            }
            // Check removed pieces
            if (designUpdate.diff.pieces?.removed) {
              for (const removedPieceId of designUpdate.diff.pieces.removed) {
                if (removedPieceId === pieceId) {
                  return true;
                }
              }
            }
          }
        }
      }
      return false;
    },
    true,
  );
}

// #region Design Editor - Piece Hooks

/**
 * Check if a specific piece is directly hovered
 */
export function useDesignEditorIsPieceHovered(id: DesignEditorId | undefined, pieceId: string): boolean {
  const store = useDesignEditorStore(identitySelector, id) as DesignEditorStore;
  return useSync<DesignEditorState, boolean>(
    store,
    (state) => {
      const hover = state.hover;
      return hover?.pieces?.includes(pieceId) ?? false;
    },
    true,
  ) as boolean;
}

/**
 * Check if a piece is transitively hovered (piece itself, its type, or its design is hovered in the same editor)
 */
export function useDesignEditorIsPieceTransitiveHovered(id: DesignEditorId | undefined, pieceId: string): boolean {
  const store = useDesignEditorStore(identitySelector, id) as DesignEditorStore;

  return useSync<DesignEditorState, boolean>(
    store,
    (state) => {
      const hover = state.hover;

      // Direct piece hover
      if (hover?.pieces?.includes(pieceId)) return true;

      // Get the piece from the design
      const design = store.design().snapshot();
      const piece = design?.pieces?.find((p) => p.guid === pieceId);
      if (!piece) return false;

      // Check if the piece's type is hovered in this editor
      if (piece.type && hover?.types?.includes(piece.type)) return true;

      // Check if the piece's design is hovered in this editor
      if (piece.design && hover?.designs?.includes(piece.design)) return true;

      return false;
    },
    true,
  ) as boolean;
}

/**
 * Check if a type is transitively hovered (type itself or any piece of that type is hovered in the same editor)
 */
export function useDesignEditorIsTypeTransitiveHovered(id: DesignEditorId | undefined, typeId: string): boolean {
  const store = useDesignEditorStore(identitySelector, id) as DesignEditorStore;

  return useSync<DesignEditorState, boolean>(
    store,
    (state) => {
      const hover = state.hover;

      // Direct type hover
      if (hover?.types?.includes(typeId)) return true;

      // Check if any hovered piece has this type
      if (hover?.pieces && hover.pieces.length > 0) {
        const design = store.design().snapshot();
        return hover.pieces.some((pieceId) => {
          const piece = design?.pieces?.find((p) => p.guid === pieceId);
          return piece?.type === typeId;
        });
      }

      return false;
    },
    true,
  ) as boolean;
}

/**
 * Get the diff status of a piece from the current kit diff
 */
export function useDesignEditorPieceStatus(id: DesignEditorId | undefined, pieceId: string): DiffStatus {
  const store = useDesignEditorStore(identitySelector, id) as DesignEditorStore;
  return useSync<DesignEditorState, DiffStatus>(
    store,
    (state) => {
      const currentStack = store?.currentTransactionStack;

      // Check current transaction stack for status
      if (currentStack && currentStack.length > 0) {
        for (const edit of currentStack) {
          if (edit.do?.kitDiff?.designs) {
            for (const designUpdate of edit.do.kitDiff.designs.updated || []) {
              // Check added pieces
              if (designUpdate.diff.pieces?.added) {
                for (const piece of designUpdate.diff.pieces.added) {
                  if (piece.guid === pieceId) {
                    return DiffStatus.Added;
                  }
                }
              }
              // Check removed pieces
              if (designUpdate.diff.pieces?.removed) {
                for (const removedId of designUpdate.diff.pieces.removed) {
                  if (removedId === pieceId) {
                    return DiffStatus.Removed;
                  }
                }
              }
              // Check modified pieces
              if (designUpdate.diff.pieces?.updated) {
                for (const pieceUpdate of designUpdate.diff.pieces.updated) {
                  if (pieceUpdate.id === pieceId) {
                    return DiffStatus.Modified;
                  }
                }
              }
            }
          }
        }
      }

      return DiffStatus.Unchanged;
    },
    true,
  ) as DiffStatus;
}

/**
 * Check if a piece is selected
 */
export function useDesignEditorIsPieceSelected(id: DesignEditorId | undefined, pieceId: string): boolean {
  const store = useDesignEditorStore(identitySelector, id) as DesignEditorStore;
  return useSync<DesignEditorState, boolean>(
    store,
    (state) => {
      return state.selection?.pieces?.includes(pieceId) ?? false;
    },
    true,
  ) as boolean;
}

/**
 * Get the color for a piece based on its state (status, selection, hover)
 */
export function useDesignEditorPieceColor(id: DesignEditorId | undefined, pieceId: string): { fill: string; stroke: string; opacity: number } {
  const isSelected = useDesignEditorIsPieceSelected(id, pieceId);
  const isHovered = useDesignEditorIsPieceTransitiveHovered(id, pieceId);
  const status = useDesignEditorPieceStatus(id, pieceId);
  const isChangedInTransaction = useIsDesignPieceChangedInTransaction(id, pieceId) as boolean;

  let fill = "var(--foreground)";
  let stroke = "var(--foreground)";
  let opacity = 1;

  // Base state colors
  if (status === DiffStatus.Added) {
    fill = "var(--color-success)";
    stroke = "var(--color-success)";
  } else if (status === DiffStatus.Removed) {
    fill = "var(--color-danger)";
    stroke = "var(--color-danger)";
    opacity = 0.2;
  } else if (status === DiffStatus.Modified) {
    fill = "var(--color-warning)";
    stroke = "var(--color-warning)";
  } else if (isChangedInTransaction) {
    fill = "var(--color-changed-base)";
    stroke = "var(--color-changed-base)";
  } else {
    fill = "transparent";
    stroke = "var(--foreground)";
  }

  // Hover state (overrides base when not selected)
  if (isHovered && !isSelected) {
    fill = "var(--hover-base)";
    stroke = "var(--foreground)";
    opacity = 1;
  }

  // Selected state with mixed colors for transaction changes
  if (isSelected) {
    if (isChangedInTransaction) {
      fill = "var(--color-selected-changed)";
      stroke = "var(--foreground)";
    } else if (status === DiffStatus.Added) {
      fill = "var(--color-selected-added)";
      stroke = "var(--foreground)";
    } else if (status === DiffStatus.Removed) {
      fill = "var(--color-selected-removed)";
      stroke = "var(--foreground)";
    } else if (status === DiffStatus.Modified) {
      fill = "var(--color-selected-changed)";
      stroke = "var(--foreground)";
    } else {
      fill = "var(--active-base)";
      stroke = "var(--foreground)";
    }
    opacity = 1;
  }

  return { fill, stroke, opacity };
}

// #endregion Design Editor - Piece Hooks

// #region Design Editor - Connection Hooks

/**
 * Check if a specific connection is directly hovered
 */
export function useDesignEditorIsConnectionHovered(id: DesignEditorId | undefined, connectionId: string): boolean {
  const store = useDesignEditorStore(identitySelector, id) as DesignEditorStore;
  return useSync<DesignEditorState, boolean>(
    store,
    (state) => {
      return state.hover?.connections?.includes(connectionId) ?? false;
    },
    true,
  ) as boolean;
}

/**
 * Check if a connection is selected
 */
export function useDesignEditorIsConnectionSelected(id: DesignEditorId | undefined, connectionId: string): boolean {
  const store = useDesignEditorStore(identitySelector, id) as DesignEditorStore;
  return useSync<DesignEditorState, boolean>(
    store,
    (state) => {
      return state.selection?.connections?.includes(connectionId) ?? false;
    },
    true,
  ) as boolean;
}

/**
 * Get the diff status of a connection from the current kit diff
 */
export function useDesignEditorConnectionStatus(id: DesignEditorId | undefined, connectionId: string): DiffStatus {
  const store = useDesignEditorStore(identitySelector, id) as DesignEditorStore;
  return useSync<DesignEditorState, DiffStatus>(
    store,
    (state) => {
      const currentStack = store?.currentTransactionStack;

      if (currentStack && currentStack.length > 0) {
        for (const edit of currentStack) {
          if (edit.do?.kitDiff?.designs) {
            for (const designUpdate of edit.do.kitDiff.designs.updated || []) {
              // Check added connections
              if (designUpdate.diff.connections?.added) {
                for (const conn of designUpdate.diff.connections.added) {
                  if (conn.guid === connectionId) {
                    return DiffStatus.Added;
                  }
                }
              }
              // Check removed connections (removed is an array of connection IDs)
              if (designUpdate.diff.connections?.removed) {
                for (const removedConn of designUpdate.diff.connections.removed) {
                  if (typeof removedConn === "string" && removedConn === connectionId) {
                    return DiffStatus.Removed;
                  } else if (typeof removedConn === "object" && removedConn.connected && removedConn.connecting) {
                    // Handle case where removed is a Side comparison
                    continue;
                  }
                }
              }
              // Check modified connections
              if (designUpdate.diff.connections?.updated) {
                for (const connUpdate of designUpdate.diff.connections.updated) {
                  if (typeof connUpdate.id === "string" && connUpdate.id === connectionId) {
                    return DiffStatus.Modified;
                  } else if (typeof connUpdate.id === "object" && connUpdate.id.connected && connUpdate.id.connecting) {
                    // Handle case where id is a Side-based identifier
                    continue;
                  }
                }
              }
            }
          }
        }
      }

      return DiffStatus.Unchanged;
    },
    true,
  ) as DiffStatus;
}

/**
 * Get the color for a connection based on its state
 */
export function useDesignEditorConnectionColor(id: DesignEditorId | undefined, connectionId: string): { fill: string; stroke: string; opacity: number } {
  const isSelected = useDesignEditorIsConnectionSelected(id, connectionId);
  const isHovered = useDesignEditorIsConnectionHovered(id, connectionId);
  const status = useDesignEditorConnectionStatus(id, connectionId);

  let fill = "var(--foreground)";
  let stroke = "var(--foreground)";
  let opacity = 1;

  // Base state colors
  if (status === DiffStatus.Added) {
    fill = "var(--color-success)";
    stroke = "var(--color-success)";
  } else if (status === DiffStatus.Removed) {
    fill = "var(--color-danger)";
    stroke = "var(--color-danger)";
    opacity = 0.2;
  } else if (status === DiffStatus.Modified) {
    fill = "var(--color-warning)";
    stroke = "var(--color-warning)";
  }

  // Hover state
  if (isHovered && !isSelected) {
    fill = "var(--hover-base)";
    stroke = "var(--foreground)";
    opacity = 1;
  }

  // Selected state
  if (isSelected) {
    if (status === DiffStatus.Added) {
      fill = "var(--color-selected-added)";
      stroke = "var(--foreground)";
    } else if (status === DiffStatus.Removed) {
      fill = "var(--color-selected-removed)";
      stroke = "var(--foreground)";
    } else if (status === DiffStatus.Modified) {
      fill = "var(--color-selected-changed)";
      stroke = "var(--foreground)";
    } else {
      fill = "var(--active-base)";
      stroke = "var(--foreground)";
    }
    opacity = 1;
  }

  return { fill, stroke, opacity };
}

// #endregion Design Editor - Connection Hooks

// #endregion Design Editor
