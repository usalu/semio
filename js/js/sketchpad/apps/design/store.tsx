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
import { areSameKit, Camera, Connection, ConnectionDiff, Coord, DiffStatus, Guid, KitDiff, Piece, PieceDiff } from "../../../semio";
import { DesignStore, KitCommandContext, KitStore, useDesignScope, useKitScope } from "../../kits/store";
import {
  DesignAppId,
  identitySelector,
  KitDiffAppEdit,
  KitDiffAppStore,
  PanelVisibility,
  registerDesignAppStoreFactory,
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
import { commands as designAppCommands } from "./commands";

type YDesignAppVal = string | number | boolean | YLeafMapString | YLeafMapNumber | Y.Map<boolean> | YAttributes | YStringArray;
type YDesignApp = Y.Map<YDesignAppVal>;
type YDesignApps = Y.Map<Y.Map<YDesignApp>>;

export interface DesignAppSelection {
  pieces?: Guid[];
  connections?: Guid[];
  port?: { piece: Guid; designPiece?: Guid; port: Guid };
}
export interface DesignAppSelectionPiecesDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface DesignAppSelectionConnectionsDiff {
  added?: Guid[];
  removed?: Guid[];
}
export interface DesignAppSelectionPortDiff {
  piece?: Guid;
  designPiece?: Guid;
  port?: Guid;
}
export interface DesignAppSelectionDiff {
  pieces?: DesignAppSelectionPiecesDiff;
  connections?: DesignAppSelectionConnectionsDiff;
  port?: DesignAppSelectionPortDiff;
}
export enum DesignAppFullscreenWindow {
  None = "none",
  Diagram = "diagram",
  Accessl = "accessl",
}
export interface DesignAppPresence {
  cursor?: Coord;
  camera?: Camera;
  diagramCenter?: Coord;
  diagramScale?: number;
}
export interface DesignAppHover {
  pieces?: Guid[];
  connections?: Guid[];
  ports?: { piece: Guid; designPiece?: Guid; port: Guid }[];
  types?: Guid[];
  designs?: Guid[];
}
export interface DesignAppPresenceOther extends DesignAppPresence {
  name: string;
}
export interface DesignAppDiff {
  selection?: DesignAppSelectionDiff;
  presence?: DesignAppPresence;
  hover?: DesignAppHover;
  fullscreenWindow?: DesignAppFullscreenWindow;
  panelVisibility?: Partial<PanelVisibility>;
  activeTool?: ToolType;
  camera?: Camera;
  diagramCenter?: Coord;
  diagramScale?: number;
  focusedPieceGuid?: Guid | null; // null to clear focus
}
export interface DesignAppEdit extends KitDiffAppEdit<DesignAppSelectionDiff> {}
export interface DesignAppState {
  fullscreenWindow: DesignAppFullscreenWindow;
  panelVisibility: PanelVisibility;
  activeTool?: ToolType;
  selection?: DesignAppSelection;
  hover?: DesignAppHover;
  presence?: DesignAppPresence;
  others: DesignAppPresenceOther[];
  camera?: Camera;
  diagramCenter?: Coord;
  diagramScale?: number;
  focusedPieceGuid?: Guid; // Currently focused piece for camera zoom
  currentTransactionStackLength?: number; // Added to trigger re-renders when stack changes
}

export interface DesignAppCommandContext extends KitCommandContext {
  designApp: DesignAppState;
  Guid: Guid;
}
export interface DesignAppCommandResult {
  diff?: DesignAppDiff;
  kitDiff?: KitDiff;
}

export const inverseDesignAppSelectionDiff = (selection: DesignAppSelection, diff: DesignAppSelectionDiff): DesignAppSelectionDiff => {
  const inverseDiff: DesignAppSelectionDiff = {};

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
export const areSameDesignApp = (designApp: DesignAppId, other: DesignAppId): boolean => areSameKit(designApp.kit, other.kit) && designApp.design === other.design;
export const hasSameDesignApp = (designApp: DesignAppId, others: DesignAppId[]): boolean => others.some((other) => areSameDesignApp(designApp, other));

class DesignAppStore extends KitDiffAppStore<DesignAppState, DesignAppDiff, DesignAppSelectionDiff, DesignAppEdit, DesignAppCommandContext, DesignAppCommandResult> {
  constructor(parent: SketchpadStore, yMap: YDesignApp, transact: (fn: () => void) => void, id: DesignAppId, state?: DesignAppState) {
    super(parent, yMap, transact);

    const kit = this.parent.kit(id.kit);
    const design = kit.design(id.design);
    yMap.set("kit", kit.guid);
    yMap.set("design", design.guid);

    // Only initialize if not already set (preserve existing values when reopening)
    if (!yMap.has("fullscreenWindow")) {
      yMap.set("fullscreenWindow", state?.fullscreenWindow || DesignAppFullscreenWindow.None);
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

    Object.entries(designAppCommands).forEach(([commandId, command]) => {
      this.registerCommand(commandId, command);
    });
  }

  get fullscreenWindow(): DesignAppFullscreenWindow {
    return this.yMap.get("fullscreenWindow") as DesignAppFullscreenWindow;
  }
  set fullscreenWindow(panel: DesignAppFullscreenWindow) {
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
  get selection(): DesignAppSelection {
    const selection = this.yMap.get("selection") as Y.Map<any>;
    if (!selection) return {};

    const result: DesignAppSelection = {};

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
  get presence(): DesignAppPresence {
    return {
      cursor: {
        x: (this.yMap.get("presenceCursorX") as number) || 0,
        y: (this.yMap.get("presenceCursorY") as number) || 0,
      },
    };
  }
  get others(): DesignAppPresenceOther[] {
    return [];
  }
  get diff(): KitDiff {
    return {};
  }
  get hover(): DesignAppHover | undefined {
    const hover = this.yMap.get("hover") as Y.Map<any> | undefined;
    if (!hover) return undefined;
    const result: DesignAppHover = {};
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

  get focusedPieceGuid(): Guid | undefined {
    return this.yMap.get("focusedPieceGuid") as Guid | undefined;
  }

  kit(): KitStore {
    return this.parent.kit(this.yMap.get("kit") as string);
  }

  design(): DesignStore {
    return this.kit().design(this.yMap.get("design") as string);
  }

  protected getSelection(): DesignAppSelection {
    return this.selection;
  }

  protected hash(state: DesignAppState): string {
    return JSON.stringify(state);
  }

  protected buildSnapshot(): DesignAppState {
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
      focusedPieceGuid: this.focusedPieceGuid,
      currentTransactionStackLength: this.currentTransactionStack.length, // Include stack length in snapshot
    };
  }

  protected inverseSelectionDiff(selection: DesignAppSelection, diff: DesignAppSelectionDiff): DesignAppSelectionDiff {
    return inverseDesignAppSelectionDiff(selection, diff);
  }

  protected applySelectionDiff = (selectionDiff: DesignAppSelectionDiff) => {
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

  change = (diff: DesignAppDiff) => {
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
      if (diff.focusedPieceGuid !== undefined) {
        if (diff.focusedPieceGuid === null) {
          this.yMap.delete("focusedPieceGuid");
        } else {
          this.yMap.set("focusedPieceGuid", diff.focusedPieceGuid);
        }
      }
    });
  };

  async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
    if (command === "semio.designApp.startTransaction") {
      console.log(`Executing (special) command: "${command}"`);
      this.startTransaction();
      return {} as T;
    }
    if (command === "semio.designApp.finalizeTransaction") {
      console.log(`Executing (special) command: "${command}"`);
      this.finalizeTransaction();
      return {} as T;
    }
    if (command === "semio.designApp.abortTransaction") {
      console.log(`Executing (special) command: "${command}"`);
      this.abortTransaction();
      return {} as T;
    }
    if (command === "semio.designApp.undo") {
      console.log(`Executing (special) command: "${command}"`);
      this.undo();
      return {} as T;
    }
    if (command === "semio.designApp.redo") {
      console.log(`Executing (special) command: "${command}"`);
      this.redo();
      return {} as T;
    }

    console.group(`Executing command: "${command}"`);
    const callback = this.commandRegistry.get(command);
    if (!callback) throw new Error(`Command "${command}" not found in design app store`);

    const kitStore = this.kit();
    const state = this.snapshot();
    const kitState = kitStore.snapshot();

    const context: DesignAppCommandContext = {
      designApp: state,
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

// Register the factory - deferred to avoid circular dependency issues
export function initializeDesignAppStore() {
  registerDesignAppStoreFactory((parent, yMap, transact, id, state) => new DesignAppStore(parent, yMap as any, transact, id, state));
}

// Auto-initialize if this module is imported
if (typeof window !== 'undefined') {
  // Use setTimeout to defer execution until after module initialization
  setTimeout(() => initializeDesignAppStore(), 0);
}

type DesignAppScope = { id: string };
const DesignAppScopeContext = createContext<DesignAppScope | null>(null);
export const DesignAppScopeProvider = (props: { id: string; children: React.ReactNode }) => {
  const value = { id: props.id };
  return React.createElement(DesignAppScopeContext.Provider, { value }, props.children as any);
};
const useDesignAppScope = () => useContext(DesignAppScopeContext);

export function useDesignAppStore<T>(selector?: (store: DesignAppStore) => T, id?: DesignAppId): T | DesignAppStore | null {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const designScope = useDesignScope();
  const resolvedKitId = kitScope?.guid ?? id?.kit;
  const resolvedDesignId = designScope?.guid ?? id?.design;
  if (!resolvedKitId || !resolvedDesignId) {
    return null;
  }
  const designAppStore = store.designApp(resolvedKitId, resolvedDesignId);
  return selector ? selector(designAppStore) : designAppStore;
}

export function useDesignApp<T>(selector?: (state: DesignAppState) => T, id?: DesignAppId): T | DesignAppState | null {
  const store = useDesignAppStore(identitySelector, id);
  return useSyncDeep<DesignAppState, T>(store as DesignAppStore, selector ? selector : identitySelector);
}

export function useDesignAppSelection(): DesignAppSelection {
  return useDesignApp((s) => s.selection) as DesignAppSelection;
}

export function useDesignAppFullscreen(): DesignAppFullscreenWindow {
  return useDesignApp((s) => s.fullscreenWindow) as DesignAppFullscreenWindow;
}

// TODO: DesignAppState doesn't have a diff property - this needs to be rethought
export function useDesignAppDiff(): KitDiff | undefined {
  // return useDesignApp((s) => s.diff) as KitDiff;
  return undefined;
}

export function useDesignAppOthers(): DesignAppPresenceOther[] {
  return useDesignApp((s) => s.others) as DesignAppPresenceOther[];
}

export function useDesignAppCamera(): Camera | undefined {
  return useDesignApp((s) => s.camera) as Camera | undefined;
}

export function useDesignAppDiagramCenter(): Coord | undefined {
  return useDesignApp((s) => s.diagramCenter) as Coord | undefined;
}

export function useDesignAppDiagramScale(): number | undefined {
  return useDesignApp((s) => s.diagramScale) as number | undefined;
}

export function useDesignAppFocusedPieceGuid(): Guid | undefined {
  return useDesignApp((s) => s.focusedPieceGuid) as Guid | undefined;
}

export function useDesignAppHover(): DesignAppHover | undefined {
  return useDesignApp((s) => s.hover) as DesignAppHover | undefined;
}

export function useDesignAppCommands(id?: DesignAppId) {
  const store = useDesignAppStore(undefined, id) as DesignAppStore;
  return {
    startTransaction: () => {
      void store.execute("semio.designApp.startTransaction");
    },
    finalizeTransaction: () => {
      void store.execute("semio.designApp.finalizeTransaction");
    },
    abortTransaction: () => {
      void store.execute("semio.designApp.abortTransaction");
    },
    undo: () => {
      void store.execute("semio.designApp.undo");
    },
    redo: () => {
      void store.execute("semio.designApp.redo");
    },
    selectAll: () => store.execute("semio.designApp.selectAll"),
    deselectAll: () => store.execute("semio.designApp.deselectAll"),
    selectPiece: (guid: Guid) => store.execute("semio.designApp.selectPiece", guid),
    selectPieces: (guids: Guid[]) => store.execute("semio.designApp.selectPieces", guids),
    addPieceToSelection: (guid: Guid) => store.execute("semio.designApp.addPieceToSelection", guid),
    removePieceFromSelection: (guid: Guid) => store.execute("semio.designApp.removePieceFromSelection", guid),
    selectConnection: (connectionGuid: Guid) => store.execute("semio.designApp.selectConnection", connectionGuid),
    addConnectionToSelection: (connectionGuid: Guid) => store.execute("semio.designApp.addConnectionToSelection", connectionGuid),
    removeConnectionFromSelection: (connectionGuid: Guid) => store.execute("semio.designApp.removeConnectionFromSelection", connectionGuid),
    selectPiecePort: (piece: Guid, port: Guid) => store.execute("semio.designApp.selectPiecePort", piece, port),
    deselectPiecePort: () => store.execute("semio.designApp.deselectPiecePort"),
    deleteSelected: () => store.execute("semio.designApp.deleteSelected"),
    toggleDiagramFullscreen: () => store.execute("semio.designApp.toggleDiagramFullscreen"),
    toggleAccesslFullscreen: () => store.execute("semio.designApp.toggleAccesslFullscreen"),
    setActiveTool: (tool: ToolType) => store.execute("semio.designApp.setActiveTool", tool),
    addPiece: (piece: Piece) => store.execute("semio.designApp.addPiece", piece),
    addPieces: (pieces: Piece[]) => store.execute("semio.designApp.addPieces", pieces),
    removePiece: (piece: Guid) => store.execute("semio.designApp.removePiece", piece),
    removePieces: (pieces: Guid[]) => store.execute("semio.designApp.removePieces", pieces),
    addConnection: (connection: Connection) => store.execute("semio.designApp.addConnection", connection),
    addConnections: (connections: Connection[]) => store.execute("semio.designApp.addConnections", connections),
    removeConnection: (connection: Guid) => store.execute("semio.designApp.removeConnection", connection),
    removeConnections: (connections: Guid[]) => store.execute("semio.designApp.removeConnections", connections),
    updatePiece: (piece: Guid, pieceDiff: PieceDiff) => store.execute("semio.designApp.updatePiece", piece, pieceDiff),
    updatePieces: (updates: { id: Guid; diff: PieceDiff }[]) => store.execute("semio.designApp.updatePieces", updates),
    updateConnection: (connection: Guid, connectionDiff: ConnectionDiff) => store.execute("semio.designApp.updateConnection", connection, connectionDiff),
    updateConnections: (updates: { id: Guid; diff: ConnectionDiff }[]) => store.execute("semio.designApp.updateConnections", updates),
    setCamera: (camera: Camera) => store.execute("semio.designApp.setCamera", camera),
    focusPiece: (pieceGuid: Guid) => store.execute("semio.designApp.focusPiece", pieceGuid),
    clearFocus: () => store.execute("semio.designApp.clearFocus"),
    setDiagramCenter: (center: Coord) => store.execute("semio.designApp.setDiagramCenter", center),
    setDiagramScale: (scale: number) => store.execute("semio.designApp.setDiagramScale", scale),
    hoverPiece: (guid: Guid) => store.execute("semio.designApp.hoverPiece", guid),
    hoverPieces: (guids: Guid[]) => store.execute("semio.designApp.hoverPieces", guids),
    hoverConnection: (guid: Guid) => store.execute("semio.designApp.hoverConnection", guid),
    hoverConnections: (guids: Guid[]) => store.execute("semio.designApp.hoverConnections", guids),
    hoverPort: (pieceGuid: Guid, portGuid: Guid) => store.execute("semio.designApp.hoverPort", pieceGuid, portGuid),
    hoverType: (guid: Guid) => store.execute("semio.designApp.hoverType", guid),
    hoverTypes: (guids: Guid[]) => store.execute("semio.designApp.hoverTypes", guids),
    hoverDesign: (guid: Guid) => store.execute("semio.designApp.hoverDesign", guid),
    hoverDesigns: (guids: Guid[]) => store.execute("semio.designApp.hoverDesigns", guids),
    clearHover: () => store.execute("semio.designApp.clearHover"),
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

export function useIsDesignPieceChangedInTransaction(id: DesignAppId | undefined, pieceId: string) {
  const store = useDesignAppStore(identitySelector, id) as DesignAppStore;
  return useSync<DesignAppState, boolean>(
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

// #region Design App - Piece Hooks

/**
 * Check if a specific piece is directly hovered
 */
export function useDesignAppIsPieceHovered(id: DesignAppId | undefined, pieceId: string): boolean {
  const store = useDesignAppStore(identitySelector, id) as DesignAppStore;
  return useSync<DesignAppState, boolean>(
    store,
    (state) => {
      const hover = state.hover;
      return hover?.pieces?.includes(pieceId) ?? false;
    },
    true,
  ) as boolean;
}

/**
 * Check if a piece is transitively hovered (piece itself, its type, or its design is hovered in the same app)
 */
export function useDesignAppIsPieceTransitiveHovered(id: DesignAppId | undefined, pieceId: string): boolean {
  const store = useDesignAppStore(identitySelector, id) as DesignAppStore;

  return useSync<DesignAppState, boolean>(
    store,
    (state) => {
      const hover = state.hover;

      // Direct piece hover
      if (hover?.pieces?.includes(pieceId)) return true;

      // Get the piece from the design
      const design = store.design().snapshot();
      const piece = design?.pieces?.find((p) => p.guid === pieceId);
      if (!piece) return false;

      // Check if the piece's type is hovered in this app
      if (piece.type && hover?.types?.includes(piece.type)) return true;

      // Check if the piece's design is hovered in this app
      if (piece.design && hover?.designs?.includes(piece.design)) return true;

      return false;
    },
    true,
  ) as boolean;
}

/**
 * Check if a type is transitively hovered (type itself or any piece of that type is hovered in the same app)
 */
export function useDesignAppIsTypeTransitiveHovered(id: DesignAppId | undefined, typeId: string): boolean {
  const store = useDesignAppStore(identitySelector, id) as DesignAppStore;

  return useSync<DesignAppState, boolean>(
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
export function useDesignAppPieceStatus(id: DesignAppId | undefined, pieceId: string): DiffStatus {
  const store = useDesignAppStore(identitySelector, id) as DesignAppStore;
  return useSync<DesignAppState, DiffStatus>(
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
export function useDesignAppIsPieceSelected(id: DesignAppId | undefined, pieceId: string): boolean {
  const store = useDesignAppStore(identitySelector, id) as DesignAppStore;
  return useSync<DesignAppState, boolean>(
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
export function useDesignAppPieceColor(id: DesignAppId | undefined, pieceId: string): { fill: string; stroke: string; opacity: number } {
  const isSelected = useDesignAppIsPieceSelected(id, pieceId);
  const isHovered = useDesignAppIsPieceTransitiveHovered(id, pieceId);
  const status = useDesignAppPieceStatus(id, pieceId);
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

// #endregion Design App - Piece Hooks

// #region Design App - Connection Hooks

/**
 * Check if a specific connection is directly hovered
 */
export function useDesignAppIsConnectionHovered(id: DesignAppId | undefined, connectionId: string): boolean {
  const store = useDesignAppStore(identitySelector, id) as DesignAppStore;
  return useSync<DesignAppState, boolean>(
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
export function useDesignAppIsConnectionSelected(id: DesignAppId | undefined, connectionId: string): boolean {
  const store = useDesignAppStore(identitySelector, id) as DesignAppStore;
  return useSync<DesignAppState, boolean>(
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
export function useDesignAppConnectionStatus(id: DesignAppId | undefined, connectionId: string): DiffStatus {
  const store = useDesignAppStore(identitySelector, id) as DesignAppStore;
  return useSync<DesignAppState, DiffStatus>(
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
export function useDesignAppConnectionColor(id: DesignAppId | undefined, connectionId: string): { fill: string; stroke: string; opacity: number } {
  const isSelected = useDesignAppIsConnectionSelected(id, connectionId);
  const isHovered = useDesignAppIsConnectionHovered(id, connectionId);
  const status = useDesignAppConnectionStatus(id, connectionId);

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

// #endregion Design App - Connection Hooks

// #endregion Design App
