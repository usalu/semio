// #region Header

// DesignEditor.tsx

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

// #region TODOs

// #endregion TODOs

import { DndContext, DragEndEvent, DragOverlay, DragStartEvent } from "@dnd-kit/core";
import { ReactFlowProvider, useReactFlow } from "@xyflow/react";
import { Info, MessageCircle, Terminal, Wrench } from "lucide-react";
import { FC, createContext, useCallback, useContext, useEffect, useMemo, useState } from "react";
import { createPortal } from "react-dom";
import { useHotkeys } from "react-hotkeys-hook";

import {
  Connection,
  ConnectionId,
  Design,
  DesignDiff,
  DesignId,
  DiagramPoint,
  ICON_WIDTH,
  Kit,
  Model,
  Piece,
  PieceId,
  Plane,
  Type,
  addConnectionToDesign,
  addConnectionToDesignDiff,
  addConnectionsToDesign,
  addConnectionsToDesignDiff,
  addPieceToDesign,
  addPieceToDesignDiff,
  addPiecesToDesign,
  addPiecesToDesignDiff,
  applyDesignDiff,
  findDesignInKit,
  mergeDesigns,
  removeConnectionFromDesign,
  removeConnectionFromDesignDiff,
  removeConnectionsFromDesign,
  removeConnectionsFromDesignDiff,
  removePieceFromDesign,
  removePieceFromDesignDiff,
  removePiecesAndConnectionsFromDesign,
  removePiecesFromDesign,
  removePiecesFromDesignDiff,
  setConnectionInDesign,
  setConnectionInDesignDiff,
  setConnectionsInDesign,
  setConnectionsInDesignDiff,
  setPieceInDesign,
  setPieceInDesignDiff,
  setPiecesInDesign,
  setPiecesInDesignDiff,
} from "@semio/js";
import Diagram from "@semio/js/components/ui/Diagram";
import ModelComponent from "@semio/js/components/ui/Model";
import { default as Navbar } from "@semio/js/components/ui/Navbar";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "@semio/js/components/ui/Resizable";
import { ToggleGroup, ToggleGroupItem } from "@semio/js/components/ui/ToggleGroup";
import { Generator } from "@semio/js/lib/utils";
import { Camera, TypeId, orientDesign } from "../../semio";
import {
  DesignEditorStoreFullscreenPanel,
  DesignEditorStorePresence,
  DesignEditorStoreScopeProvider,
  DesignEditorStoreSelection,
  DesignEditorStoreState,
  useCurrentDesignEditorId,
  useDesign,
  useDesignEditorScope,
  useDesignEditorStoreFullscreenPanel,
  useDesignEditorStoreIsTransactionActive,
  useDesignEditorStoreSelection,
  useDesignId,
  useDesigns,
  useKit,
  useSketchpadStore,
  useTypes,
} from "../../store";
import Chat from "./Chat";
import { ConsolePanel, commandRegistry } from "./Console";
import { designEditorCommands } from "./designEditorCommands";
import Details from "./Details";
import { useSketchpad } from "./Sketchpad";
import Workbench, { DesignAvatar, TypeAvatar } from "./Workbench";

designEditorCommands.forEach((command) => commandRegistry.register(command));

//#region State

export enum DesignEditorAction {
  Undo = "UNDO",
  Redo = "REDO",
  SetDesign = "SET_DESIGN",
  AddDesign = "ADD_DESIGN",
  AddPiece = "ADD_PIECE",
  SetPiece = "SET_PIECE",
  RemovePiece = "REMOVE_PIECE",
  AddPieces = "ADD_PIECES",
  SetPieces = "SET_PIECES",
  RemovePieces = "REMOVE_PIECES",
  AddConnection = "ADD_CONNECTION",
  SetConnection = "SET_CONNECTION",
  RemoveConnection = "REMOVE_CONNECTION",
  AddConnections = "ADD_CONNECTIONS",
  SetConnections = "SET_CONNECTIONS",
  RemoveConnections = "REMOVE_CONNECTIONS",
  RemovePiecesAndConnections = "REMOVE_PIECES_AND_CONNECTIONS",
  SetSelection = "SET_SELECTION",
  SelectAll = "SELECT_ALL",
  DeselectAll = "DESELECT_ALL",
  InvertSelection = "INVERT_SELECTION",
  InvertPiecesSelection = "INVERT_PIECES_SELECTION",
  InvertConnectionsSelection = "INVERT_CONNECTIONS_SELECTION",
  AddAllPiecesToSelection = "ADD_ALL_PIECES_TO_SELECTION",
  RemoveAllPiecesFromSelection = "REMOVE_ALL_PIECES_FROM_SELECTION",
  AddAllConnectionsToSelection = "ADD_ALL_CONNECTIONS_TO_SELECTION",
  RemoveAllConnectionsFromSelection = "REMOVE_ALL_CONNECTIONS_FROM_SELECTION",
  SelectPiece = "SELECT_PIECE",
  AddPieceToSelection = "ADD_PIECE_TO_SELECTION",
  RemovePieceFromSelection = "REMOVE_PIECE_FROM_SELECTION",
  SelectPieces = "SELECT_PIECES",
  AddPiecesToSelection = "ADD_PIECES_TO_SELECTION",
  RemovePiecesFromSelection = "REMOVE_PIECES_FROM_SELECTION",
  SelectConnection = "SELECT_CONNECTION",
  AddConnectionToSelection = "ADD_CONNECTION_TO_SELECTION",
  RemoveConnectionFromSelection = "REMOVE_CONNECTION_FROM_SELECTION",
  SelectConnections = "SELECT_CONNECTIONS",
  AddConnectionsToSelection = "ADD_CONNECTIONS_TO_SELECTION",
  RemoveConnectionsFromSelection = "REMOVE_CONNECTIONS_FROM_SELECTION",
  SelectPiecePort = "SELECT_PIECE_PORT",
  DeselectPiecePort = "DESELECT_PIECE_PORT",
  CopyToClipboard = "COPY_TO_CliPBOARD",
  CutToClipboard = "CUT_TO_CliPBOARD",
  PasteFromClipboard = "PASTE_FROM_CliPBOARD",
  DeleteSelected = "DELETE_SELECTED",
  SetFullscreen = "SET_FULLSCREEN",
  ToggleDiagramFullscreen = "TOGGLE_DIAGRAM_FULLSCREEN",
  ToggleModelFullscreen = "TOGGLE_MODEL_FULLSCREEN",
  StartTransaction = "START_TRANSACTION",
  FinalizeTransaction = "FINALIZE_TRANSACTION",
  AbortTransaction = "ABORT_TRANSACTION",
  SetCursor = "SET_CURSOR",
  SetCamera = "SET_CAMERA",
  StepIn = "STEP_IN",
  StepOut = "STEP_OUT",
  UpdatePresence = "UPDATE_PRESENCE",
}

export type DesignEditorDispatcher = (action: { type: DesignEditorAction; payload: any }) => void;

//#region Selection

const selectionConnectionToConnectionId = (selectionConnection: { connectingPieceId: string; connectedPieceId: string }): ConnectionId => ({
  connected: { piece: { id_: selectionConnection.connectedPieceId } },
  connecting: { piece: { id_: selectionConnection.connectingPieceId } },
});
const connectionToSelectionConnection = (connection: Connection | ConnectionId): { connectingPieceId: string; connectedPieceId: string } => ({
  connectingPieceId: connection.connecting.piece.id_,
  connectedPieceId: connection.connected.piece.id_,
});

const selectAll = (design: Design): DesignEditorStoreSelection => ({
  selectedPieceIds: design.pieces?.map((p: Piece) => ({ id_: p.id_ })) || [],
  selectedConnections:
    design.connections?.map((c: Connection) => ({
      connected: { piece: { id_: c.connected.piece.id_ } },
      connecting: { piece: { id_: c.connecting.piece.id_ } },
    })) || [],
  selectedPiecePortId: undefined,
});
const deselectAll = (selection: DesignEditorStoreSelection): DesignEditorStoreSelection => ({
  selectedPieceIds: [],
  selectedConnections: [],
  selectedPiecePortId: undefined,
});
const addAllPiecesToSelection = (selection: DesignEditorStoreSelection, design: Design): DesignEditorStoreSelection => {
  const existingIds = new Set(selection.selectedPieceIds.map((p) => p.id_));
  const allPieceIds = design.pieces?.map((p: Piece) => p.id_) || [];
  const newIds = allPieceIds.filter((id: string) => !existingIds.has(id));
  return {
    selectedPieceIds: [...selection.selectedPieceIds, ...newIds.map((id) => ({ id_: id }))],
    selectedConnections: selection.selectedConnections,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};
const removeAllPiecesFromSelection = (selection: DesignEditorStoreSelection): DesignEditorStoreSelection => ({
  selectedPieceIds: [],
  selectedConnections: selection.selectedConnections,
  selectedPiecePortId: selection.selectedPiecePortId,
});
const addAllConnectionsToSelection = (selection: DesignEditorStoreSelection, design: Design): DesignEditorStoreSelection => {
  const allConnections =
    design.connections?.map((c: Connection) => ({
      connected: { piece: { id_: c.connected.piece.id_ } },
      connecting: { piece: { id_: c.connecting.piece.id_ } },
    })) || [];
  const newConnections = allConnections.filter((conn) => {
    return !selection.selectedConnections.some((c) => {
      return c.connected.piece.id_ === conn.connected.piece.id_ && c.connecting.piece.id_ === conn.connecting.piece.id_;
    });
  });
  return {
    selectedPieceIds: selection.selectedPieceIds,
    selectedConnections: [...selection.selectedConnections, ...newConnections],
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};
const removeAllConnectionsFromSelection = (selection: DesignEditorStoreSelection): DesignEditorStoreSelection => ({
  selectedPieceIds: selection.selectedPieceIds,
  selectedConnections: [],
  selectedPiecePortId: selection.selectedPiecePortId,
});

const selectPiece = (piece: Piece | PieceId): DesignEditorStoreSelection => ({
  selectedPieceIds: [typeof piece === "string" ? { id_: piece } : piece],
  selectedConnections: [],
  selectedPiecePortId: undefined,
});
const addPieceToSelection = (selection: DesignEditorStoreSelection, piece: Piece | PieceId): DesignEditorStoreSelection => {
  const pieceId = typeof piece === "string" ? piece : piece.id_;
  const existingPieceIds = new Set(selection.selectedPieceIds.map((p) => p.id_));
  const newPieceIds = existingPieceIds.has(pieceId) ? selection.selectedPieceIds.filter((p) => p.id_ !== pieceId) : [...selection.selectedPieceIds, { id_: pieceId }];
  return {
    selectedPieceIds: newPieceIds,
    selectedConnections: selection.selectedConnections,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};
const removePieceFromSelection = (selection: DesignEditorStoreSelection, piece: Piece | PieceId): DesignEditorStoreSelection => {
  const pieceId = typeof piece === "string" ? piece : piece.id_;
  return {
    selectedPieceIds: selection.selectedPieceIds.filter((p) => p.id_ !== pieceId),
    selectedConnections: selection.selectedConnections,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};

const selectPieces = (pieces: (Piece | PieceId)[]): DesignEditorStoreSelection => ({
  selectedPieceIds: pieces.map((p) => (typeof p === "string" ? { id_: p } : p)),
  selectedConnections: [],
  selectedPiecePortId: undefined,
});
const addPiecesToSelection = (selection: DesignEditorStoreSelection, pieces: (Piece | PieceId)[]): DesignEditorStoreSelection => {
  const existingIds = new Set(selection.selectedPieceIds.map((p) => p.id_));
  const newIds = pieces.map((p) => (typeof p === "string" ? p : p.id_)).filter((id) => !existingIds.has(id));
  return {
    ...selection,
    selectedPieceIds: [...selection.selectedPieceIds, ...newIds.map((id) => ({ id_: id }))],
  };
};
const removePiecesFromSelection = (selection: DesignEditorStoreSelection, pieces: (Piece | PieceId)[]): DesignEditorStoreSelection => {
  const idsToRemove = new Set(pieces.map((p) => (typeof p === "string" ? p : p.id_)));
  return {
    ...selection,
    selectedPieceIds: selection.selectedPieceIds.filter((p) => !idsToRemove.has(p.id_)),
  };
};

const selectConnection = (connection: Connection | ConnectionId): DesignEditorStoreSelection => ({
  selectedConnections: [
    {
      connected: { piece: { id_: connection.connected.piece.id_ } },
      connecting: { piece: { id_: connection.connecting.piece.id_ } },
    },
  ],
  selectedPieceIds: [],
  selectedPiecePortId: undefined,
});
const addConnectionToSelection = (selection: DesignEditorStoreSelection, connection: Connection | ConnectionId): DesignEditorStoreSelection => {
  const connectionObj = {
    connected: { piece: { id_: connection.connected.piece.id_ } },
    connecting: { piece: { id_: connection.connecting.piece.id_ } },
  };

  const exists = selection.selectedConnections.some((c) => {
    return c.connected.piece.id_ === connectionObj.connected.piece.id_ && c.connecting.piece.id_ === connectionObj.connecting.piece.id_;
  });

  if (exists) return selection;
  return {
    selectedConnections: [...selection.selectedConnections, connectionObj],
    selectedPieceIds: selection.selectedPieceIds,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};
const removeConnectionFromSelection = (selection: DesignEditorStoreSelection, connection: Connection | ConnectionId): DesignEditorStoreSelection => {
  return {
    selectedConnections: selection.selectedConnections.filter((c) => {
      return !(c.connected.piece.id_ === connection.connected.piece.id_ && c.connecting.piece.id_ === connection.connecting.piece.id_);
    }),
    selectedPieceIds: selection.selectedPieceIds,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};

const selectConnections = (connections: (Connection | ConnectionId)[]): DesignEditorStoreSelection => ({
  selectedConnections: connections.map((conn) => ({
    connected: { piece: { id_: conn.connected.piece.id_ } },
    connecting: { piece: { id_: conn.connecting.piece.id_ } },
  })),
  selectedPieceIds: [],
  selectedPiecePortId: undefined,
});
const addConnectionsToSelection = (selection: DesignEditorStoreSelection, connections: (Connection | ConnectionId)[]): DesignEditorStoreSelection => {
  const newConnections = connections
    .map((conn) => ({
      connected: { piece: { id_: conn.connected.piece.id_ } },
      connecting: { piece: { id_: conn.connecting.piece.id_ } },
    }))
    .filter((conn) => {
      return !selection.selectedConnections.some((c) => {
        return c.connected.piece.id_ === conn.connected.piece.id_ && c.connecting.piece.id_ === conn.connecting.piece.id_;
      });
    });
  return {
    ...selection,
    selectedConnections: [...selection.selectedConnections, ...newConnections],
  };
};
const removeConnectionsFromSelection = (selection: DesignEditorStoreSelection, connections: (Connection | ConnectionId)[]): DesignEditorStoreSelection => {
  return {
    selectedConnections: selection.selectedConnections.filter((c) => {
      return !connections.some((conn) => c.connected.piece.id_ === conn.connected.piece.id_ && c.connecting.piece.id_ === conn.connecting.piece.id_);
    }),
    selectedPieceIds: selection.selectedPieceIds,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};

const selectPiecePort = (pieceId: string, portId: string): DesignEditorStoreSelection => ({
  selectedPieceIds: [],
  selectedConnections: [],
  selectedPiecePortId: { pieceId: { id_: pieceId }, portId: { id_: portId } },
});
const deselectPiecePort = (selection: DesignEditorStoreSelection): DesignEditorStoreSelection => ({ ...selection, selectedPiecePortId: undefined });

const invertSelection = (selection: DesignEditorStoreSelection, design: Design): DesignEditorStoreSelection => {
  const allPieceIds = design.pieces?.map((p: Piece) => p.id_) || [];
  const allConnections =
    design.connections?.map((c: Connection) => ({
      connected: { piece: { id_: c.connected.piece.id_ } },
      connecting: { piece: { id_: c.connecting.piece.id_ } },
    })) || [];
  const selectedPieceIdSet = new Set(selection.selectedPieceIds.map((p) => p.id_));
  const newSelectedPieceIds = allPieceIds.filter((id: string) => !selectedPieceIdSet.has(id));
  const newSelectedConnections = allConnections.filter((conn) => {
    return !selection.selectedConnections.some((selected) => {
      return selected.connected.piece.id_ === conn.connected.piece.id_ && selected.connecting.piece.id_ === conn.connecting.piece.id_;
    });
  });
  return {
    selectedPieceIds: newSelectedPieceIds.map((id) => ({ id_: id })),
    selectedConnections: newSelectedConnections,
    selectedPiecePortId: undefined,
  };
};

const invertPiecesSelection = (selection: DesignEditorStoreSelection, design: Design): DesignEditorStoreSelection => {
  const allPieceIds = design.pieces?.map((p: Piece) => p.id_) || [];
  const selectedPieceIdSet = new Set(selection.selectedPieceIds.map((p) => p.id_));
  const newSelectedPieceIds = allPieceIds.filter((id: string) => !selectedPieceIdSet.has(id));
  return {
    selectedPieceIds: newSelectedPieceIds.map((id) => ({ id_: id })),
    selectedConnections: selection.selectedConnections,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};
const invertConnectionsSelection = (selection: DesignEditorStoreSelection, design: Design): DesignEditorStoreSelection => {
  const allConnections =
    design.connections?.map((c: Connection) => ({
      connected: { piece: { id_: c.connected.piece.id_ } },
      connecting: { piece: { id_: c.connecting.piece.id_ } },
    })) || [];
  const newSelectedConnections = allConnections.filter((conn) => {
    return !selection.selectedConnections.some((selected) => {
      return selected.connected.piece.id_ === conn.connected.piece.id_ && selected.connecting.piece.id_ === conn.connecting.piece.id_;
    });
  });
  return {
    selectedPieceIds: selection.selectedPieceIds,
    selectedConnections: newSelectedConnections,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};

const subDesignFromSelection = (design: Design, selection: DesignEditorStoreSelection): Design => {
  const selectedPieceIdSet = new Set(selection.selectedPieceIds.map((p) => p.id_));
  const subPieces = design.pieces?.filter((p: Piece) => selectedPieceIdSet.has(p.id_));
  const subConnections = design.connections?.filter((c: Connection) => selection.selectedConnections.some((sc) => sc.connected.piece.id_ === c.connected.piece.id_ && sc.connecting.piece.id_ === c.connecting.piece.id_));
  return { ...design, pieces: subPieces, connections: subConnections };
};

const copyToClipboard = (design: Design, selection: DesignEditorStoreSelection, plane?: Plane, center?: DiagramPoint): void => {
  navigator.clipboard.writeText(JSON.stringify(orientDesign(subDesignFromSelection(design, selection), plane, center))).then(() => {});
};
const cutToClipboard = (design: Design, selection: DesignEditorStoreSelection, plane?: Plane, center?: DiagramPoint): void => {
  navigator.clipboard.writeText(JSON.stringify(subDesignFromSelection(orientDesign(design, plane, center), selection))).then(() => {});
};
const pasteFromClipboard = async (design: Design, plane?: Plane, center?: DiagramPoint): Promise<Design> => {
  try {
    const text = await navigator.clipboard.readText();
    const clipboardDesign = JSON.parse(text);
    return mergeDesigns([design, orientDesign(clipboardDesign, plane, center)]);
  } catch (error) {
    console.warn("Failed to paste from clipboard:", error);
    return design;
  }
};
const deleteSelected = (kit: Kit, designId: DesignId, selection: DesignEditorStoreSelection): Design => {
  const selectedPieces = selection.selectedPieceIds;
  const selectedConnections = selection.selectedConnections.map((conn) => ({
    connecting: { piece: { id_: conn.connecting.piece.id_ } },
    connected: { piece: { id_: conn.connected.piece.id_ } },
  }));
  const updatedDesign = removePiecesAndConnectionsFromDesign(kit, designId, selectedPieces, selectedConnections);
  return updatedDesign;
};

//#endregion Selection

//#region DesignDiff Helpers

const updateDesignDiffInState = (state: DesignEditorStoreState, updatedDesignDiff: DesignDiff): DesignEditorStoreState => {
  return { ...state, designDiff: updatedDesignDiff };
};

const resetDesignDiff = (): DesignDiff => ({
  pieces: { added: [], removed: [], updated: [] },
  connections: { added: [], removed: [], updated: [] },
});

const pushToOperationStack = (state: DesignEditorStoreState): DesignEditorStoreState => {
  // This is a simplified implementation - in a real scenario, this would capture the current operation
  // and add it to the operation stack for undo/redo functionality
  return {
    ...state,
    operationStack: [...state.operationStack],
    operationIndex: state.operationIndex,
  };
};

//#endregion DesignDiff Helpers

export const DesignEditorCommandsContext = createContext<{ dispatch: DesignEditorDispatcher } | undefined>(undefined);

export const DesignEditorCommandsProvider = (props: { children: React.ReactNode }) => {
  const store = useSketchpadStore();
  const designEditorScope = useDesignEditorScope();
  const kit = useKit();
  const designId = useDesignId();

  if (!designEditorScope?.id) {
    throw new Error("DesignEditorCommandsProvider must be used within a DesignEditorScopeProvider");
  }

  const designEditorStore = store.getDesignEditorStoreStore(designEditorScope.id);
  if (!designEditorStore) {
    throw new Error(`Design editor store not found for id: ${designEditorScope.id}`);
  }

  // Don't render children until all required dependencies are available
  if (!store || !kit || !designId) {
    return null;
  }

  const dispatch = useCallback(
    (action: { type: DesignEditorAction; payload: any }) => {
      if (!kit || !designId) {
        console.warn("Cannot dispatch action: kit or designId not available");
        return;
      }

      // For now, we'll use a simplified implementation that directly calls store methods
      // This will be expanded to handle all the DesignEditorAction types
      const currentDesign = findDesignInKit(kit, designId);
      if (!currentDesign) {
        console.warn("Cannot dispatch action: design not found");
        return;
      }

      const selection = designEditorStore.getState().selection;

      switch (action.type) {
        case DesignEditorAction.SelectAll:
          const allPieces = currentDesign.pieces?.map((p: Piece) => ({ id_: p.id_ })) || [];
          const allConnections =
            currentDesign.connections?.map((c: Connection) => ({
              connected: { piece: { id_: c.connected.piece.id_ } },
              connecting: { piece: { id_: c.connecting.piece.id_ } },
            })) || [];
          designEditorStore.updateDesignEditorStoreSelection({
            selectedPieceIds: allPieces,
            selectedConnections: allConnections,
            selectedPiecePortId: undefined,
          });
          break;

        case DesignEditorAction.DeselectAll:
          designEditorStore.updateDesignEditorStoreSelection({
            selectedPieceIds: [],
            selectedConnections: [],
            selectedPiecePortId: undefined,
          });
          break;

        case DesignEditorAction.SetSelection:
          designEditorStore.updateDesignEditorStoreSelection(action.payload);
          break;

        default:
          console.warn(`Unhandled action type: ${action.type}`);
      }
    },
    [kit, designId, designEditorStore],
  );

  return <DesignEditorCommandsContext.Provider value={{ dispatch }}>{props.children}</DesignEditorCommandsContext.Provider>;
};

export const useDesignEditorCommands = () => {
  const context = useContext(DesignEditorCommandsContext);
  if (!context) {
    throw new Error("useDesignEditorCommands must be used within a DesignEditorCommandsProvider");
  }

  // Provide a safe default dispatch function to avoid React hook issues
  const { dispatch = () => console.warn("dispatch not available") } = context;

  // TODO: These functions need to be implemented properly
  const clusterDesign = useCallback(() => console.warn("clusterDesign not implemented"), []);
  const expandDesign = useCallback(() => console.warn("expandDesign not implemented"), []);

  const setDesign = useCallback((d: Design) => dispatch({ type: DesignEditorAction.SetDesign, payload: d }), [dispatch]);
  const addPiece = useCallback((p: Piece) => dispatch({ type: DesignEditorAction.AddPiece, payload: p }), [dispatch]);
  const setPiece = useCallback((p: Piece) => dispatch({ type: DesignEditorAction.SetPiece, payload: p }), [dispatch]);
  const removePiece = useCallback((p: Piece) => dispatch({ type: DesignEditorAction.RemovePiece, payload: p }), [dispatch]);
  const addPieces = useCallback((ps: Piece[]) => dispatch({ type: DesignEditorAction.AddPieces, payload: ps }), [dispatch]);
  const setPieces = useCallback((ps: Piece[]) => dispatch({ type: DesignEditorAction.SetPieces, payload: ps }), [dispatch]);
  const removePieces = useCallback((ps: Piece[]) => dispatch({ type: DesignEditorAction.RemovePieces, payload: ps }), [dispatch]);
  const addConnection = useCallback((c: Connection) => dispatch({ type: DesignEditorAction.AddConnection, payload: c }), [dispatch]);
  const setConnection = useCallback((c: Connection) => dispatch({ type: DesignEditorAction.SetConnection, payload: c }), [dispatch]);
  const removeConnection = useCallback((c: Connection) => dispatch({ type: DesignEditorAction.RemoveConnection, payload: c }), [dispatch]);
  const addConnections = useCallback((cs: Connection[]) => dispatch({ type: DesignEditorAction.AddConnections, payload: cs }), [dispatch]);
  const setConnections = useCallback((cs: Connection[]) => dispatch({ type: DesignEditorAction.SetConnections, payload: cs }), [dispatch]);
  const removeConnections = useCallback((cs: Connection[]) => dispatch({ type: DesignEditorAction.RemoveConnections, payload: cs }), [dispatch]);
  const setSelection = useCallback((s: DesignEditorStoreSelection) => dispatch({ type: DesignEditorAction.SetSelection, payload: s }), [dispatch]);
  const selectAll = useCallback(() => dispatch({ type: DesignEditorAction.SelectAll, payload: null }), [dispatch]);
  const deselectAll = useCallback(() => dispatch({ type: DesignEditorAction.DeselectAll, payload: null }), [dispatch]);
  const invertSelection = useCallback(() => dispatch({ type: DesignEditorAction.InvertSelection, payload: null }), [dispatch]);
  const invertPiecesSelection = useCallback(() => dispatch({ type: DesignEditorAction.InvertPiecesSelection, payload: null }), [dispatch]);
  const invertConnectionsSelection = useCallback(() => dispatch({ type: DesignEditorAction.InvertConnectionsSelection, payload: null }), [dispatch]);
  const addAllPiecesToSelection = useCallback(() => dispatch({ type: DesignEditorAction.AddAllPiecesToSelection, payload: null }), [dispatch]);
  const removeAllPiecesFromSelection = useCallback(() => dispatch({ type: DesignEditorAction.RemoveAllPiecesFromSelection, payload: null }), [dispatch]);
  const addAllConnectionsToSelection = useCallback(() => dispatch({ type: DesignEditorAction.AddAllConnectionsToSelection, payload: null }), [dispatch]);
  const removeAllConnectionsFromSelection = useCallback(() => dispatch({ type: DesignEditorAction.RemoveAllConnectionsFromSelection, payload: null }), [dispatch]);
  const selectPiece = useCallback((p: Piece | PieceId) => dispatch({ type: DesignEditorAction.SelectPiece, payload: p }), [dispatch]);
  const addPieceToSelection = useCallback((p: Piece | PieceId) => dispatch({ type: DesignEditorAction.AddPieceToSelection, payload: p }), [dispatch]);
  const removePieceFromSelection = useCallback((p: Piece | PieceId) => dispatch({ type: DesignEditorAction.RemovePieceFromSelection, payload: p }), [dispatch]);
  const selectPieces = useCallback((ps: (Piece | PieceId)[]) => dispatch({ type: DesignEditorAction.SelectPieces, payload: ps }), [dispatch]);
  const addPiecesToSelection = useCallback((ps: (Piece | PieceId)[]) => dispatch({ type: DesignEditorAction.AddPiecesToSelection, payload: ps }), [dispatch]);
  const removePiecesFromSelection = useCallback((ps: (Piece | PieceId)[]) => dispatch({ type: DesignEditorAction.RemovePiecesFromSelection, payload: ps }), [dispatch]);
  const selectConnection = useCallback((c: Connection | ConnectionId) => dispatch({ type: DesignEditorAction.SelectConnection, payload: c }), [dispatch]);
  const addConnectionToSelection = useCallback((c: Connection | ConnectionId) => dispatch({ type: DesignEditorAction.AddConnectionToSelection, payload: c }), [dispatch]);
  const removeConnectionFromSelection = useCallback((c: Connection | ConnectionId) => dispatch({ type: DesignEditorAction.RemoveConnectionFromSelection, payload: c }), [dispatch]);
  const selectConnections = useCallback((cs: (Connection | ConnectionId)[]) => dispatch({ type: DesignEditorAction.SelectConnections, payload: cs }), [dispatch]);
  const addConnectionsToSelection = useCallback((cs: (Connection | ConnectionId)[]) => dispatch({ type: DesignEditorAction.AddConnectionsToSelection, payload: cs }), [dispatch]);
  const removeConnectionsFromSelection = useCallback((cs: (Connection | ConnectionId)[]) => dispatch({ type: DesignEditorAction.RemoveConnectionsFromSelection, payload: cs }), [dispatch]);
  const selectPiecePort = useCallback((pieceId: string, portId: string) => dispatch({ type: DesignEditorAction.SelectPiecePort, payload: { pieceId, portId } }), [dispatch]);
  const deselectPiecePort = useCallback(() => dispatch({ type: DesignEditorAction.DeselectPiecePort, payload: null }), [dispatch]);
  const deleteSelected = useCallback((plane?: Plane, center?: DiagramPoint) => dispatch({ type: DesignEditorAction.DeleteSelected, payload: { plane, center } }), [dispatch]);
  const setFullscreen = useCallback((fp: DesignEditorStoreFullscreenPanel) => dispatch({ type: DesignEditorAction.SetFullscreen, payload: fp }), [dispatch]);
  const toggleDiagramFullscreen = useCallback(() => dispatch({ type: DesignEditorAction.ToggleDiagramFullscreen, payload: null }), [dispatch]);
  const toggleModelFullscreen = useCallback(() => dispatch({ type: DesignEditorAction.ToggleModelFullscreen, payload: null }), [dispatch]);
  const copyToClipboard = useCallback(() => dispatch({ type: DesignEditorAction.CopyToClipboard, payload: null }), [dispatch]);
  const pasteFromClipboard = useCallback((plane?: Plane, center?: DiagramPoint) => dispatch({ type: DesignEditorAction.PasteFromClipboard, payload: { plane, center } }), [dispatch]);
  const undo = useCallback(() => dispatch({ type: DesignEditorAction.Undo, payload: null }), [dispatch]);
  const redo = useCallback(() => dispatch({ type: DesignEditorAction.Redo, payload: null }), [dispatch]);
  const startTransaction = useCallback(() => dispatch({ type: DesignEditorAction.StartTransaction, payload: null }), [dispatch]);
  const finalizeTransaction = useCallback(() => dispatch({ type: DesignEditorAction.FinalizeTransaction, payload: null }), [dispatch]);
  const abortTransaction = useCallback(() => dispatch({ type: DesignEditorAction.AbortTransaction, payload: null }), [dispatch]);
  const setCursor = useCallback((cursor: DiagramPoint | undefined) => dispatch({ type: DesignEditorAction.SetCursor, payload: cursor }), [dispatch]);
  const setCamera = useCallback((camera: Camera | undefined) => dispatch({ type: DesignEditorAction.SetCamera, payload: camera }), [dispatch]);
  const stepIn = useCallback((presence: DesignEditorStorePresence) => dispatch({ type: DesignEditorAction.StepIn, payload: presence }), [dispatch]);
  const stepOut = useCallback((presence: DesignEditorStorePresence) => dispatch({ type: DesignEditorAction.StepOut, payload: presence }), [dispatch]);
  const updatePresence = useCallback((presence: Partial<DesignEditorStorePresence> & { name: string }) => dispatch({ type: DesignEditorAction.UpdatePresence, payload: presence }), [dispatch]);
  const executeCommand = useCallback(
    async (commandId: string, payload: Record<string, any> = {}) => {
      // Note: This is a placeholder implementation
      // The context needs to be passed from outside since we can't call hooks here
      console.warn(`executeCommand not fully implemented: ${commandId}`, payload);
    },
    [dispatch],
  );
  const getAvailableCommands = useCallback(() => commandRegistry.getAll(), []);
  const getCommand = useCallback((commandId: string) => commandRegistry.get(commandId), []);

  return {
    undo,
    redo,
    setDesign,
    addPiece,
    setPiece,
    removePiece,
    addPieces,
    setPieces,
    removePieces,
    addConnection,
    setConnection,
    removeConnection,
    addConnections,
    setConnections,
    removeConnections,
    setSelection,
    selectAll,
    deselectAll,
    invertSelection,
    invertPiecesSelection,
    invertConnectionsSelection,
    addAllPiecesToSelection,
    removeAllPiecesFromSelection,
    addAllConnectionsToSelection,
    removeAllConnectionsFromSelection,
    selectPiece,
    addPieceToSelection,
    removePieceFromSelection,
    selectPieces,
    addPiecesToSelection,
    removePiecesFromSelection,
    selectConnection,
    addConnectionToSelection,
    removeConnectionFromSelection,
    selectConnections,
    addConnectionsToSelection,
    removeConnectionsFromSelection,
    selectPiecePort,
    deselectPiecePort,
    deleteSelected,
    setFullscreen,
    toggleDiagramFullscreen,
    toggleModelFullscreen,
    getAvailableCommands,
    getCommand,
    executeCommand,
    startTransaction,
    finalizeTransaction,
    abortTransaction,
    setCursor,
    setCamera,
    stepIn,
    stepOut,
    updatePresence,
  };
};

const designEditorReducer = (state: DesignEditorStoreState, action: { type: DesignEditorAction; payload: any }, kit: Kit, designId: DesignId): DesignEditorStoreState => {
  const currentDesign = findDesignInKit(kit, designId);

  const updateDesignInDesignEditorStateWithOperationStack = (updatedDesign: Design): DesignEditorStoreState => {
    return pushToOperationStack(state);
  };

  const updateDesignInDesignEditorState = (updatedDesign: Design): DesignEditorStoreState => {
    return state;
  };

  switch (action.type) {
    // Design changes that should push to operation stack (when not in transaction)
    case DesignEditorAction.SetDesign:
      if (state.isTransactionActive) {
        return updateDesignInDesignEditorState(action.payload);
      }
      return updateDesignInDesignEditorStateWithOperationStack(action.payload);
    case DesignEditorAction.AddPiece:
      if (state.isTransactionActive) {
        const updatedDesignDiff = addPieceToDesignDiff(state.designDiff, action.payload);
        return updateDesignDiffInState(state, updatedDesignDiff);
      }
      const designWithAddedPiece = addPieceToDesign(currentDesign, action.payload);
      return updateDesignInDesignEditorStateWithOperationStack(designWithAddedPiece);
    case DesignEditorAction.SetPiece:
      if (state.isTransactionActive) {
        const updatedDesignDiff = setPieceInDesignDiff(state.designDiff, {
          id_: action.payload.id_,
          ...action.payload,
        });
        return updateDesignDiffInState(state, updatedDesignDiff);
      }
      const designWithSetPiece = setPieceInDesign(currentDesign, action.payload);
      return updateDesignInDesignEditorStateWithOperationStack(designWithSetPiece);
    case DesignEditorAction.RemovePiece:
      if (state.isTransactionActive) {
        const pieceId = typeof action.payload === "string" ? { id_: action.payload } : { id_: action.payload.id_ };
        const updatedDesignDiff = removePieceFromDesignDiff(state.designDiff, pieceId);
        return updateDesignDiffInState(state, updatedDesignDiff);
      }
      const designWithRemovedPiece = removePieceFromDesign(kit, designId, action.payload);
      return updateDesignInDesignEditorStateWithOperationStack(designWithRemovedPiece);
    case DesignEditorAction.AddPieces:
      if (state.isTransactionActive) {
        const updatedDesignDiff = addPiecesToDesignDiff(state.designDiff, action.payload);
        return updateDesignDiffInState(state, updatedDesignDiff);
      }
      const designWithAddedPieces = addPiecesToDesign(currentDesign, action.payload);
      return updateDesignInDesignEditorStateWithOperationStack(designWithAddedPieces);
    case DesignEditorAction.SetPieces:
      if (state.isTransactionActive) {
        const pieceDiffs = action.payload.map((piece: Piece) => ({
          ...piece,
          id_: piece.id_,
        }));
        const updatedDesignDiff = setPiecesInDesignDiff(state.designDiff, pieceDiffs);
        return updateDesignDiffInState(state, updatedDesignDiff);
      }
      const designWithSetPieces = setPiecesInDesign(currentDesign, action.payload);
      return updateDesignInDesignEditorStateWithOperationStack(designWithSetPieces);
    case DesignEditorAction.RemovePieces:
      if (state.isTransactionActive) {
        const pieceIds = action.payload.map((piece: any) => (typeof piece === "string" ? { id_: piece } : { id_: piece.id_ }));
        const updatedDesignDiff = removePiecesFromDesignDiff(state.designDiff, pieceIds);
        return updateDesignDiffInState(state, updatedDesignDiff);
      }
      const designWithRemovedPieces = removePiecesFromDesign(kit, designId, action.payload);
      return updateDesignInDesignEditorStateWithOperationStack(designWithRemovedPieces);
    case DesignEditorAction.AddConnection:
      if (state.isTransactionActive) {
        const updatedDesignDiff = addConnectionToDesignDiff(state.designDiff, action.payload);
        return updateDesignDiffInState(state, updatedDesignDiff);
      }
      const designWithAddedConnection = addConnectionToDesign(currentDesign, action.payload);
      return updateDesignInDesignEditorStateWithOperationStack(designWithAddedConnection);
    case DesignEditorAction.SetConnection:
      if (state.isTransactionActive) {
        const connectionDiff = {
          ...action.payload,
          connected: action.payload.connected,
          connecting: action.payload.connecting,
        };
        const updatedDesignDiff = setConnectionInDesignDiff(state.designDiff, connectionDiff);
        return updateDesignDiffInState(state, updatedDesignDiff);
      }
      const designWithSetConnection = setConnectionInDesign(currentDesign, action.payload);
      return updateDesignInDesignEditorStateWithOperationStack(designWithSetConnection);
    case DesignEditorAction.RemoveConnection:
      if (state.isTransactionActive) {
        const connectionId =
          typeof action.payload === "object" && "connected" in action.payload
            ? action.payload
            : {
                connected: { piece: { id_: action.payload } },
                connecting: { piece: { id_: "" } },
              };
        const updatedDesignDiff = removeConnectionFromDesignDiff(state.designDiff, connectionId);
        return updateDesignDiffInState(state, updatedDesignDiff);
      }
      const designWithRemovedConnection = removeConnectionFromDesign(kit, designId, action.payload);
      return updateDesignInDesignEditorStateWithOperationStack(designWithRemovedConnection);
    case DesignEditorAction.AddConnections:
      if (state.isTransactionActive) {
        const updatedDesignDiff = addConnectionsToDesignDiff(state.designDiff, action.payload);
        return updateDesignDiffInState(state, updatedDesignDiff);
      }
      const designWithAddedConnections = addConnectionsToDesign(currentDesign, action.payload);
      return updateDesignInDesignEditorStateWithOperationStack(designWithAddedConnections);
    case DesignEditorAction.SetConnections:
      if (state.isTransactionActive) {
        const connectionDiffs = action.payload.map((connection: Connection) => ({
          ...connection,
          connected: connection.connected,
          connecting: connection.connecting,
        }));
        const updatedDesignDiff = setConnectionsInDesignDiff(state.designDiff, connectionDiffs);
        return updateDesignDiffInState(state, updatedDesignDiff);
      }
      const designWithSetConnections = setConnectionsInDesign(currentDesign, action.payload);
      return updateDesignInDesignEditorStateWithOperationStack(designWithSetConnections);
    case DesignEditorAction.RemoveConnections:
      if (state.isTransactionActive) {
        const connectionIds = action.payload.map((connection: any) =>
          typeof connection === "object" && "connected" in connection
            ? connection
            : {
                connected: { piece: { id_: connection } },
                connecting: { piece: { id_: "" } },
              },
        );
        const updatedDesignDiff = removeConnectionsFromDesignDiff(state.designDiff, connectionIds);
        return updateDesignDiffInState(state, updatedDesignDiff);
      }
      const designWithRemovedConnections = removeConnectionsFromDesign(kit, designId, action.payload);
      return updateDesignInDesignEditorStateWithOperationStack(designWithRemovedConnections);
    case DesignEditorAction.RemovePiecesAndConnections:
      if (state.isTransactionActive) {
        // Handle pieces
        const pieceIds = action.payload.pieceIds.map((piece: any) => (typeof piece === "string" ? { id_: piece } : { id_: piece.id_ }));
        let updatedDesignDiff = removePiecesFromDesignDiff(state.designDiff, pieceIds);

        // Handle connections
        const connectionIds = action.payload.connectionIds.map((connection: any) =>
          typeof connection === "object" && "connected" in connection
            ? connection
            : {
                connected: { piece: { id_: connection } },
                connecting: { piece: { id_: "" } },
              },
        );
        updatedDesignDiff = removeConnectionsFromDesignDiff(updatedDesignDiff, connectionIds);

        return updateDesignDiffInState(state, updatedDesignDiff);
      }
      const designWithRemovedPiecesAndConnections = removePiecesAndConnectionsFromDesign(kit, designId, action.payload.pieceIds, action.payload.connectionIds);
      return updateDesignInDesignEditorStateWithOperationStack(designWithRemovedPiecesAndConnections);
    case DesignEditorAction.DeleteSelected:
      const stateWithDeleteOperation = pushToOperationStack(state);
      const selectionToDelete = stateWithDeleteOperation.selection;
      const updatedDesign = deleteSelected(kit, designId, selectionToDelete);
      const entryWithCorrectSelection = stateWithDeleteOperation.operationStack[stateWithDeleteOperation.operationIndex];
      entryWithCorrectSelection.selection = selectionToDelete;
      return {
        ...stateWithDeleteOperation,
        selection: deselectAll(stateWithDeleteOperation.selection),
      };

    // Transaction actions
    case DesignEditorAction.StartTransaction:
      if (state.isTransactionActive) return state; // Only one transaction at a time
      return {
        ...state,
        isTransactionActive: true,
        designDiff: resetDesignDiff(),
      };
    case DesignEditorAction.FinalizeTransaction:
      if (!state.isTransactionActive) return state;
      // Apply the accumulated diff to the design and push to operation stack
      const finalDesign = applyDesignDiff(currentDesign, state.designDiff);
      const stateWithFinalizedTransaction = pushToOperationStack({
        ...state,
        isTransactionActive: false,
        designDiff: resetDesignDiff(),
      });
      const entryWithCorrectTransactionState = stateWithFinalizedTransaction.operationStack[stateWithFinalizedTransaction.operationIndex];
      entryWithCorrectTransactionState.selection = state.selection;
      return stateWithFinalizedTransaction;
    case DesignEditorAction.AbortTransaction:
      if (!state.isTransactionActive) return state;
      // Simply reset transaction state and discard the diff
      return {
        ...state,
        isTransactionActive: false,
        designDiff: resetDesignDiff(),
      };

    // Selection changes (no operation stack)
    case DesignEditorAction.SetSelection:
      return { ...state, selection: action.payload };
    case DesignEditorAction.SelectAll:
      return { ...state, selection: selectAll(currentDesign) };
    case DesignEditorAction.DeselectAll:
      return { ...state, selection: deselectAll(state.selection) };
    case DesignEditorAction.InvertSelection:
      return {
        ...state,
        selection: invertSelection(state.selection, currentDesign),
      };
    case DesignEditorAction.InvertPiecesSelection:
      return {
        ...state,
        selection: invertPiecesSelection(state.selection, currentDesign),
      };
    case DesignEditorAction.InvertConnectionsSelection:
      return {
        ...state,
        selection: invertConnectionsSelection(state.selection, currentDesign),
      };
    case DesignEditorAction.AddAllPiecesToSelection:
      return {
        ...state,
        selection: addAllPiecesToSelection(state.selection, currentDesign),
      };
    case DesignEditorAction.RemoveAllPiecesFromSelection:
      return {
        ...state,
        selection: removeAllPiecesFromSelection(state.selection),
      };
    case DesignEditorAction.AddAllConnectionsToSelection:
      return {
        ...state,
        selection: addAllConnectionsToSelection(state.selection, currentDesign),
      };
    case DesignEditorAction.RemoveAllConnectionsFromSelection:
      return {
        ...state,
        selection: removeAllConnectionsFromSelection(state.selection),
      };
    case DesignEditorAction.SelectPiece:
      return { ...state, selection: selectPiece(action.payload) };
    case DesignEditorAction.AddPieceToSelection:
      return {
        ...state,
        selection: addPieceToSelection(state.selection, action.payload),
      };
    case DesignEditorAction.RemovePieceFromSelection:
      return {
        ...state,
        selection: removePieceFromSelection(state.selection, action.payload),
      };
    case DesignEditorAction.SelectPieces:
      return { ...state, selection: selectPieces(action.payload) };
    case DesignEditorAction.AddPiecesToSelection:
      return {
        ...state,
        selection: addPiecesToSelection(state.selection, action.payload),
      };
    case DesignEditorAction.RemovePiecesFromSelection:
      return {
        ...state,
        selection: removePiecesFromSelection(state.selection, action.payload),
      };
    case DesignEditorAction.SelectConnection:
      return { ...state, selection: selectConnection(action.payload) };
    case DesignEditorAction.AddConnectionToSelection:
      return {
        ...state,
        selection: addConnectionToSelection(state.selection, action.payload),
      };
    case DesignEditorAction.RemoveConnectionFromSelection:
      return {
        ...state,
        selection: removeConnectionFromSelection(state.selection, action.payload),
      };
    case DesignEditorAction.SelectConnections:
      return { ...state, selection: selectConnections(action.payload) };
    case DesignEditorAction.AddConnectionsToSelection:
      return {
        ...state,
        selection: addConnectionsToSelection(state.selection, action.payload),
      };
    case DesignEditorAction.RemoveConnectionsFromSelection:
      return {
        ...state,
        selection: removeConnectionsFromSelection(state.selection, action.payload),
      };
    case DesignEditorAction.SelectPiecePort:
      return {
        ...state,
        selection: selectPiecePort(action.payload.pieceId, action.payload.portId),
      };
    case DesignEditorAction.DeselectPiecePort:
      return { ...state, selection: deselectPiecePort(state.selection) };

    // Undo/Redo
    case DesignEditorAction.Undo:
      if (state.operationIndex >= 0 && state.operationStack.length > 0) {
        const op = state.operationStack[state.operationIndex];
        return {
          ...state,
          selection: op.selection,
          operationIndex: state.operationIndex - 1,
        };
      }
      return state;
    case DesignEditorAction.Redo:
      if (state.operationIndex + 1 < state.operationStack.length) {
        return {
          ...state,
          operationIndex: state.operationIndex + 1,
        };
      }
      return state;

    // Other (no operation stack)
    case DesignEditorAction.SetFullscreen:
      return { ...state, fullscreenPanel: action.payload };
    case DesignEditorAction.ToggleDiagramFullscreen:
      return {
        ...state,
        fullscreenPanel: state.fullscreenPanel === DesignEditorStoreFullscreenPanel.Diagram ? DesignEditorStoreFullscreenPanel.None : DesignEditorStoreFullscreenPanel.Diagram,
      };
    case DesignEditorAction.ToggleModelFullscreen:
      return {
        ...state,
        fullscreenPanel: state.fullscreenPanel === DesignEditorStoreFullscreenPanel.Model ? DesignEditorStoreFullscreenPanel.None : DesignEditorStoreFullscreenPanel.Model,
      };
    case DesignEditorAction.SetCursor:
      return state; // Cursor state is handled externally
    case DesignEditorAction.SetCamera:
      return state; // Camera state is handled externally
    case DesignEditorAction.StepIn:
      return { ...state, others: [...(state.others || []), action.payload] };
    case DesignEditorAction.StepOut:
      return {
        ...state,
        others: (state.others || []).filter((p) => p.name !== action.payload.name),
      };
    case DesignEditorAction.UpdatePresence:
      return {
        ...state,
        others: (state.others || []).map((p) => (p.name === action.payload.name ? { ...p, ...action.payload } : p)),
      };
    default:
      return state;
  }
};

// Export a pure selection reducer so the store-driven Sketchpad can compute selection updates
export function reduceSelection(selection: DesignEditorStoreSelection, design: Design, action: { type: DesignEditorAction; payload: any }): DesignEditorStoreSelection {
  switch (action.type) {
    case DesignEditorAction.SetSelection:
      return action.payload as DesignEditorStoreSelection;
    case DesignEditorAction.SelectAll:
      return selectAll(design);
    case DesignEditorAction.DeselectAll:
      return deselectAll(selection);
    case DesignEditorAction.InvertSelection:
      return invertSelection(selection, design);
    case DesignEditorAction.InvertPiecesSelection:
      return invertPiecesSelection(selection, design);
    case DesignEditorAction.InvertConnectionsSelection:
      return invertConnectionsSelection(selection, design);
    case DesignEditorAction.AddAllPiecesToSelection:
      return addAllPiecesToSelection(selection, design);
    case DesignEditorAction.RemoveAllPiecesFromSelection:
      return removeAllPiecesFromSelection(selection);
    case DesignEditorAction.AddAllConnectionsToSelection:
      return addAllConnectionsToSelection(selection, design);
    case DesignEditorAction.RemoveAllConnectionsFromSelection:
      return removeAllConnectionsFromSelection(selection);
    case DesignEditorAction.SelectPiece:
      return selectPiece(action.payload);
    case DesignEditorAction.AddPieceToSelection:
      return addPieceToSelection(selection, action.payload);
    case DesignEditorAction.RemovePieceFromSelection:
      return removePieceFromSelection(selection, action.payload);
    case DesignEditorAction.SelectPieces:
      return selectPieces(action.payload);
    case DesignEditorAction.AddPiecesToSelection:
      return addPiecesToSelection(selection, action.payload);
    case DesignEditorAction.RemovePiecesFromSelection:
      return removePiecesFromSelection(selection, action.payload);
    case DesignEditorAction.SelectConnection:
      return selectConnection(action.payload);
    case DesignEditorAction.AddConnectionToSelection:
      return addConnectionToSelection(selection, action.payload);
    case DesignEditorAction.RemoveConnectionFromSelection:
      return removeConnectionFromSelection(selection, action.payload);
    case DesignEditorAction.SelectConnections:
      return selectConnections(action.payload);
    case DesignEditorAction.AddConnectionsToSelection:
      return addConnectionsToSelection(selection, action.payload);
    case DesignEditorAction.RemoveConnectionsFromSelection:
      return removeConnectionsFromSelection(selection, action.payload);
    case DesignEditorAction.SelectPiecePort:
      return selectPiecePort(action.payload.pieceId, action.payload.portId);
    case DesignEditorAction.DeselectPiecePort:
      return deselectPiecePort(selection);
    default:
      return selection;
  }
}

//#endregion State

//#region Components

interface PanelToggles {
  workbench: boolean;
  console: boolean;
  details: boolean;
  chat: boolean;
}

interface PanelProps {
  visible: boolean;
}

export interface ResizablePanelProps extends PanelProps {
  onWidthChange?: (width: number) => void;
  width: number;
}

interface DesignEditorProps {}

const DesignEditorCore: FC<DesignEditorProps> = () => {
  const { setNavbarToolbar } = useSketchpad();
  const { toggleDiagramFullscreen, toggleModelFullscreen, undo, redo, stepIn, stepOut, setSelection, startTransaction, setDesign, finalizeTransaction, abortTransaction, addPiece } = useDesignEditorCommands();

  const [visiblePanels, setVisiblePanels] = useState<PanelToggles>({
    workbench: false,
    console: false,
    details: false,
    chat: false,
  });
  const [workbenchWidth, setWorkbenchWidth] = useState(230);
  const [detailsWidth, setDetailsWidth] = useState(230);
  const [consoleHeight, setConsoleHeight] = useState(200);
  const [chatWidth, setChatWidth] = useState(230);

  const togglePanel = (panel: keyof PanelToggles) => {
    setVisiblePanels((prev) => {
      const newState = { ...prev };
      if (panel === "chat" && !prev.chat) {
        newState.details = false;
      }
      if (panel === "details" && !prev.details) {
        newState.chat = false;
      }
      newState[panel] = !prev[panel];
      return newState;
    });
  };

  const onDoubleClickDiagram = useCallback(
    (e: React.MouseEvent) => {
      toggleDiagramFullscreen();
    },
    [toggleDiagramFullscreen],
  );

  const onDoubleClickModel = useCallback(
    (e: React.MouseEvent) => {
      toggleModelFullscreen();
    },
    [toggleModelFullscreen],
  );

  // Debug presence simulation - add initial users but disable continuous updates
  useEffect(() => {
    const users = [
      {
        name: "Alice",
        cursor: { x: 2, y: 3 },
        camera: {
          position: { x: 10, y: 5, z: 8 },
          forward: { x: -1, y: 0, z: 0 },
          up: { x: 0, y: 0, z: 1 },
        },
      },
      {
        name: "Bob",
        cursor: { x: -1, y: 25 },
        camera: {
          position: { x: -5, y: 10, z: 55 },
          forward: { x: 0, y: -1, z: 0 },
          up: { x: 0, y: 0, z: 1 },
        },
      },
      {
        name: "Charlie",
        cursor: { x: 5, y: -2 },
        camera: {
          position: { x: 15, y: -8, z: 6 },
          forward: { x: -0.7, y: 0.7, z: 0 },
          up: { x: 0, y: 0, z: 1 },
        },
      },
    ];

    const stepInDelay = 1000;
    users.forEach((user, index) => {
      setTimeout(
        () => {
          stepIn(user);
        },
        stepInDelay * (index + 1),
      );
    });

    // Disabled continuous updates to prevent recursive rendering
    // const updateInterval = setInterval(() => {
    //   users.forEach(user => {
    //     const deltaX = (Math.random() - 0.5) * 0.5
    //     const deltaY = (Math.random() - 0.5) * 0.5
    //     updatePresence({
    //       name: user.name,
    //       cursor: {
    //         x: user.cursor!.x + deltaX,
    //         y: user.cursor!.y + deltaY
    //       }
    //     })
    //     user.cursor!.x += deltaX
    //     user.cursor!.y += deltaY
    //   })
    // }, 2000)

    // return () => {
    //   clearInterval(updateInterval)
    // }
  }, [stepIn]);

  const designEditorToolbar = useMemo(
    () => (
      <ToggleGroup
        type="multiple"
        value={Object.entries(visiblePanels)
          .filter(([_, isVisible]) => isVisible)
          .map(([key]) => key)}
        onValueChange={(values) => {
          Object.keys(visiblePanels).forEach((key) => {
            const isCurrentlyVisible = visiblePanels[key as keyof PanelToggles];
            const shouldBeVisible = values.includes(key);
            if (isCurrentlyVisible !== shouldBeVisible) {
              togglePanel(key as keyof PanelToggles);
            }
          });
        }}
      >
        <ToggleGroupItem value="workbench" tooltip="Workbench" hotkey="⌘J">
          <Wrench />
        </ToggleGroupItem>
        <ToggleGroupItem value="console" tooltip="Console" hotkey="⌘K">
          <Terminal />
        </ToggleGroupItem>
        <ToggleGroupItem value="details" tooltip="Details" hotkey="⌘L">
          <Info />
        </ToggleGroupItem>
        <ToggleGroupItem value="chat" tooltip="Chat" hotkey="⌘[">
          <MessageCircle />
        </ToggleGroupItem>
      </ToggleGroup>
    ),
    [visiblePanels, togglePanel],
  );

  useEffect(() => {
    setNavbarToolbar(designEditorToolbar);
    return () => setNavbarToolbar(null);
  }, [designEditorToolbar, setNavbarToolbar]);

  const { screenToFlowPosition } = useReactFlow();
  const [activeDraggedTypeId, setActiveDraggedTypeId] = useState<TypeId | null>(null);
  const [activeDraggedDesignId, setActiveDraggedDesignId] = useState<DesignId | null>(null);

  // Register built-in commands
  useEffect(() => {
    // Register all centralized commands
    const unregisterFunctions = designEditorCommands.map((command) => commandRegistry.register(command));

    return () => unregisterFunctions.forEach((fn) => fn());
  }, []);

  // Get hook values at component level
  const kit = useKit();
  const designId = useDesignId();
  const selection = useDesignEditorStoreSelection();
  const isTransactionActive = useDesignEditorStoreIsTransactionActive();

  // Global command event handler
  useEffect(() => {
    const handleCommand = async (event: Event) => {
      const customEvent = event as CustomEvent;
      const { commandId, payload = {} } = customEvent.detail;

      const command = commandRegistry.get(commandId);
      if (!command) {
        console.error(`Command not found: ${commandId}`);
        return;
      }

      const context = { kit, designId, selection };

      // Editor-only commands can always execute, even during transactions
      if (command.editorOnly) {
        try {
          const result = await commandRegistry.execute(commandId, context, payload);
          if (result.selection) setSelection(result.selection);
        } catch (error) {
          console.error(`Error executing editor-only command ${commandId}:`, error);
        }
        return;
      }

      // Design-modifying commands only execute when no transaction is active
      if (isTransactionActive) {
        console.warn(`Cannot execute design-modifying command "${commandId}" during active transaction`);
        return;
      }

      // Run design commands in transactions
      startTransaction();
      try {
        const result = await commandRegistry.execute(commandId, context, payload);
        if (result.design) setDesign(result.design);
        if (result.selection) setSelection(result.selection);
        finalizeTransaction();
      } catch (error) {
        abortTransaction();
        console.error(`Error executing command ${commandId}:`, error);
      }
    };

    document.addEventListener("semio-command", handleCommand);
    return () => document.removeEventListener("semio-command", handleCommand);
  }, [kit, designId, selection, isTransactionActive, setSelection, startTransaction, setDesign, finalizeTransaction, abortTransaction]);

  // Register hotkeys for all commands automatically from the command registry
  const allCommands = commandRegistry.getAll();
  allCommands.forEach((command) => {
    if (command.hotkey) {
      useHotkeys(
        command.hotkey,
        (e) => {
          e.preventDefault();
          e.stopPropagation();

          // Editor-only commands can always execute
          if (command.editorOnly) {
            const event = new CustomEvent("semio-command", {
              detail: { commandId: command.id },
            });
            document.dispatchEvent(event);
            return;
          }

          // Design-modifying commands only execute when no transaction is active
          if (useDesignEditorStoreIsTransactionActive()) {
            console.warn(`Cannot execute design-modifying command "${command.id}" during active transaction (hotkey: ${command.hotkey})`);
            return;
          }

          const event = new CustomEvent("semio-command", {
            detail: { commandId: command.id },
          });
          document.dispatchEvent(event);
        },
        { enableOnContentEditable: false },
      );
    }
  });

  // Special hotkeys for undo/redo (handled outside command system)
  useHotkeys("mod+z", async (e) => {
    e.preventDefault();
    e.stopPropagation();
    undo();
  });

  useHotkeys("mod+y", async (e) => {
    e.preventDefault();
    e.stopPropagation();
    redo();
  });

  const onDragStart = (event: DragStartEvent) => {
    const { active } = event;
    const id = active.id.toString();
    if (id.startsWith("type-")) {
      const [_, name, variant] = id.split("-");
      const normalizeVariant = (v: string | undefined | null) => v ?? "";
      const type = useTypes()?.find((t: Type) => t.name === name && normalizeVariant(t.variant) === normalizeVariant(variant));
      setActiveDraggedTypeId(type || null);
    } else if (id.startsWith("design-")) {
      const [_, name, variant, view] = id.split("-");
      const draggedDesignId: DesignId = {
        name,
        variant: variant || undefined,
        view: view || undefined,
      };
      const draggedDesign = useDesigns()?.find((d: Design) => d.name === name && d.variant === variant && d.view === view);
      setActiveDraggedDesignId(draggedDesign || null);
    }
  };

  const onDragEnd = (event: DragEndEvent) => {
    const { over } = event;
    if (over?.id === "diagram") {
      if (!(event.activatorEvent instanceof PointerEvent)) {
        return;
      }
      if (activeDraggedTypeId) {
        const { x, y } = screenToFlowPosition({
          x: event.activatorEvent.clientX + event.delta.x,
          y: event.activatorEvent.clientY + event.delta.y,
        });
        const piece: Piece = {
          id_: Generator.randomId(),
          type: {
            name: activeDraggedTypeId.name,
            variant: activeDraggedTypeId.variant || undefined,
          },
          plane: {
            origin: { x: 0, y: 0, z: 0 },
            xAxis: { x: 1, y: 0, z: 0 },
            yAxis: { x: 0, y: 1, z: 0 },
          },
          center: { x: x / ICON_WIDTH - 0.5, y: -y / ICON_WIDTH + 0.5 },
        };
        addPiece(piece);
      } else if (activeDraggedDesignId) {
        const { x, y } = screenToFlowPosition({
          x: event.activatorEvent.clientX + event.delta.x,
          y: event.activatorEvent.clientY + event.delta.y,
        });
        const current = useDesigns()?.find((d: Design) => d.name === activeDraggedDesignId.name && d.variant === activeDraggedDesignId.variant && d.view === activeDraggedDesignId.view);
        if (current) {
          const newEntry = {
            designId: {
              name: activeDraggedDesignId.name,
              variant: activeDraggedDesignId.variant,
              view: activeDraggedDesignId.view,
            },
            plane: {
              origin: { x: 0, y: 0, z: 0 },
              xAxis: { x: 1, y: 0, z: 0 },
              yAxis: { x: 0, y: 1, z: 0 },
            },
            center: { x: x / ICON_WIDTH - 0.5, y: -y / ICON_WIDTH + 0.5 },
          };
          const updated: Design = {
            ...current,
            fixedDesigns: [...(current.fixedDesigns || []), newEntry],
          };
          setDesign(updated);
        }
      }
    }
    setActiveDraggedTypeId(null);
    setActiveDraggedDesignId(null);
  };

  // Panel hotkeys (not commands)
  useHotkeys("mod+j", (e) => {
    e.preventDefault();
    e.stopPropagation();
    togglePanel("workbench");
  });
  useHotkeys("mod+k", (e) => {
    e.preventDefault();
    e.stopPropagation();
    togglePanel("console");
  });
  useHotkeys("mod+l", (e) => {
    e.preventDefault();
    e.stopPropagation();
    togglePanel("details");
  });
  useHotkeys(["mod+[", "mod+semicolon", "mod+ö"], (e) => {
    e.preventDefault();
    e.stopPropagation();
    togglePanel("chat");
  });

  const rightPanelVisible = visiblePanels.details || visiblePanels.chat;

  const fullscreenPanel = useDesignEditorStoreFullscreenPanel();

  return (
    <DndContext onDragStart={onDragStart} onDragEnd={onDragEnd}>
      <div className="canvas flex-1 relative">
        <div id="sketchpad-edgeless" className="h-full">
          <ResizablePanelGroup direction="horizontal">
            <ResizablePanel defaultSize={fullscreenPanel === DesignEditorStoreFullscreenPanel.Diagram ? 100 : 50} className={`${fullscreenPanel === DesignEditorStoreFullscreenPanel.Model ? "hidden" : "block"}`} onDoubleClick={onDoubleClickDiagram}>
              <Diagram />
            </ResizablePanel>
            <ResizableHandle className={`border-r ${fullscreenPanel !== DesignEditorStoreFullscreenPanel.None ? "hidden" : "block"}`} />
            <ResizablePanel defaultSize={fullscreenPanel === DesignEditorStoreFullscreenPanel.Model ? 100 : 50} className={`${fullscreenPanel === DesignEditorStoreFullscreenPanel.Diagram ? "hidden" : "block"}`} onDoubleClick={onDoubleClickModel}>
              <ModelComponent />
            </ResizablePanel>
          </ResizablePanelGroup>
        </div>
        <Workbench visible={visiblePanels.workbench} onWidthChange={setWorkbenchWidth} width={workbenchWidth} />
        <Details visible={visiblePanels.details} onWidthChange={setDetailsWidth} width={detailsWidth} />
        <ConsolePanel
          visible={visiblePanels.console}
          leftPanelVisible={visiblePanels.workbench}
          rightPanelVisible={rightPanelVisible}
          leftPanelWidth={workbenchWidth}
          rightPanelWidth={detailsWidth}
          height={consoleHeight}
          setHeight={setConsoleHeight}
        />
        <Chat visible={visiblePanels.chat} onWidthChange={setChatWidth} width={chatWidth} />
        {createPortal(
          <DragOverlay>
            {activeDraggedTypeId && <TypeAvatar typeId={activeDraggedTypeId} />}
            {activeDraggedDesignId && <DesignAvatar designId={activeDraggedDesignId} />}
          </DragOverlay>,
          document.body,
        )}
      </div>
    </DndContext>
  );
};

const DesignEditor: FC<DesignEditorProps> = () => {
  const currentDesignEditorId = useCurrentDesignEditorId();

  if (!currentDesignEditorId) {
    return <div className="h-full w-full flex items-center justify-center">Loading design editor...</div>;
  }

  return (
    <DesignEditorStoreScopeProvider id={currentDesignEditorId}>
      <div className="h-full w-full flex flex-col bg-background text-foreground">
        <Navbar />
        <ReactFlowProvider>
          <DesignEditorCommandsProvider>
            <DesignEditorCore />
          </DesignEditorCommandsProvider>
        </ReactFlowProvider>
      </div>
    </DesignEditorStoreScopeProvider>
  );
};

export default DesignEditor;

// Export the reducer for state management
export { designEditorReducer };

//#endregion Components
