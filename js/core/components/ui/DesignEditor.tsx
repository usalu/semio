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

const COMMAND_STACK_MAX = 50;

import { DndContext, DragEndEvent, DragOverlay, DragStartEvent } from "@dnd-kit/core";
import { ReactFlowProvider, useReactFlow } from "@xyflow/react";
import { Info, MessageCircle, Terminal, Wrench } from "lucide-react";
import { FC, ReactNode, createContext, useCallback, useContext, useEffect, useReducer, useState } from "react";
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
  Piece,
  PieceId,
  Plane,
  Type,
  addConnectionToDesign,
  addConnectionToDesignDiff,
  addConnectionsToDesign,
  addConnectionsToDesignDiff,
  addPieceToDesignDiff,
  addPiecesToDesign,
  addPiecesToDesignDiff,
  applyDesignDiff,
  findDesignInKit,
  isSameConnection,
  isSameDesign,
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
  updateDesignInKit,
} from "@semio/js";
import Diagram from "@semio/js/components/ui/Diagram";
import Model from "@semio/js/components/ui/Model";
import { default as Navbar } from "@semio/js/components/ui/Navbar";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "@semio/js/components/ui/Resizable";
import { Layout, Mode, Theme } from "@semio/js/components/ui/Sketchpad";
import { ToggleGroup, ToggleGroupItem } from "@semio/js/components/ui/ToggleGroup";
import { Generator } from "@semio/js/lib/utils";
import { Camera, orientDesign } from "../../semio";
import Chat from "./Chat";
import { CommandContext, ConsolePanel, commandRegistry } from "./Console";
import Details from "./Details";
import Workbench, { DesignAvatar, TypeAvatar } from "./Workbench";
import { designEditorCommands } from "./designEditorCommands";

// Register all design editor commands
designEditorCommands.forEach((command) => commandRegistry.register(command));

// Helper functions for commands
const addPieceToDesign = (design: Design, piece: Piece): Design => ({
  ...design,
  pieces: [...(design.pieces || []), piece],
});

//#region State

export interface DesignEditorSelection {
  selectedPieceIds: string[];
  selectedConnections: {
    connectingPieceId: string;
    connectedPieceId: string;
  }[];
  selectedPiecePortId?: { pieceId: string; portId: string };
}

export interface OperationStackEntry {
  design: Design;
  selection: DesignEditorSelection;
}

export interface Presence {
  name: string;
  cursor?: DiagramPoint;
  camera?: Camera;
}

export interface DesignEditorState {
  kit: Kit;
  designId: DesignId;
  fileUrls: Map<string, string>;
  fullscreenPanel: FullscreenPanel;
  selection: DesignEditorSelection;
  designDiff: DesignDiff;
  operationStack: OperationStackEntry[];
  operationIndex: number;
  isTransactionActive: boolean;
  cursor?: DiagramPoint;
  camera?: Camera;
  others: Presence[];
}

export enum FullscreenPanel {
  None = "none",
  Diagram = "diagram",
  Model = "model",
}

export enum DesignEditorAction {
  SetDesign = "SET_DESIGN",
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
  Undo = "UNDO",
  Redo = "REDO",
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

const selectAll = (design: Design): DesignEditorSelection => ({
  selectedPieceIds: design.pieces?.map((p: Piece) => p.id_) || [],
  selectedConnections:
    design.connections?.map((c: Connection) => ({
      connectedPieceId: c.connected.piece.id_,
      connectingPieceId: c.connecting.piece.id_,
    })) || [],
  selectedPiecePortId: undefined,
});
const deselectAll = (selection: DesignEditorSelection): DesignEditorSelection => ({
  selectedPieceIds: [],
  selectedConnections: [],
  selectedPiecePortId: undefined,
});
const addAllPiecesToSelection = (selection: DesignEditorSelection, design: Design): DesignEditorSelection => {
  const existingIds = new Set(selection.selectedPieceIds);
  const allPieceIds = design.pieces?.map((p: Piece) => p.id_) || [];
  const newIds = allPieceIds.filter((id) => !existingIds.has(id));
  return {
    selectedPieceIds: [...selection.selectedPieceIds, ...newIds],
    selectedConnections: selection.selectedConnections,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};
const removeAllPiecesFromSelection = (selection: DesignEditorSelection): DesignEditorSelection => ({
  selectedPieceIds: [],
  selectedConnections: selection.selectedConnections,
  selectedPiecePortId: selection.selectedPiecePortId,
});
const addAllConnectionsToSelection = (selection: DesignEditorSelection, design: Design): DesignEditorSelection => {
  const allConnections = design.connections?.map((c: Connection) => connectionToSelectionConnection(c)) || [];
  const newConnections = allConnections.filter((conn) => {
    const connectionId = selectionConnectionToConnectionId(conn);
    return !selection.selectedConnections.some((c) => {
      const existingConnectionId = selectionConnectionToConnectionId(c);
      return isSameConnection(existingConnectionId, connectionId);
    });
  });
  return {
    selectedPieceIds: selection.selectedPieceIds,
    selectedConnections: [...selection.selectedConnections, ...newConnections],
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};
const removeAllConnectionsFromSelection = (selection: DesignEditorSelection): DesignEditorSelection => ({
  selectedPieceIds: selection.selectedPieceIds,
  selectedConnections: [],
  selectedPiecePortId: selection.selectedPiecePortId,
});

const selectPiece = (piece: Piece | PieceId): DesignEditorSelection => ({
  selectedPieceIds: [typeof piece === "string" ? piece : piece.id_],
  selectedConnections: [],
  selectedPiecePortId: undefined,
});
const addPieceToSelection = (selection: DesignEditorSelection, piece: Piece | PieceId): DesignEditorSelection => {
  const pieceId = typeof piece === "string" ? piece : piece.id_;
  const existingPieceIds = new Set(selection.selectedPieceIds);
  const newPieceIds = existingPieceIds.has(pieceId) ? selection.selectedPieceIds.filter((id: string) => id !== pieceId) : [...selection.selectedPieceIds, pieceId];
  return {
    selectedPieceIds: newPieceIds,
    selectedConnections: selection.selectedConnections,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};
const removePieceFromSelection = (selection: DesignEditorSelection, piece: Piece | PieceId): DesignEditorSelection => {
  return {
    selectedPieceIds: selection.selectedPieceIds.filter((id: string) => id !== (typeof piece === "string" ? piece : piece.id_)),
    selectedConnections: selection.selectedConnections,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};

const selectPieces = (pieces: (Piece | PieceId)[]): DesignEditorSelection => ({
  selectedPieceIds: pieces.map((p) => (typeof p === "string" ? p : p.id_)),
  selectedConnections: [],
  selectedPiecePortId: undefined,
});
const addPiecesToSelection = (selection: DesignEditorSelection, pieces: (Piece | PieceId)[]): DesignEditorSelection => {
  const existingIds = new Set(selection.selectedPieceIds);
  const newIds = pieces.map((p) => (typeof p === "string" ? p : p.id_)).filter((id) => !existingIds.has(id));
  return {
    ...selection,
    selectedPieceIds: [...selection.selectedPieceIds, ...newIds],
  };
};
const removePiecesFromSelection = (selection: DesignEditorSelection, pieces: (Piece | PieceId)[]): DesignEditorSelection => {
  const idsToRemove = new Set(pieces.map((p) => (typeof p === "string" ? p : p.id_)));
  return {
    ...selection,
    selectedPieceIds: selection.selectedPieceIds.filter((id: string) => !idsToRemove.has(id)),
  };
};

const selectConnection = (connection: Connection | ConnectionId): DesignEditorSelection => ({
  selectedConnections: [connectionToSelectionConnection(connection)],
  selectedPieceIds: [],
  selectedPiecePortId: undefined,
});
const addConnectionToSelection = (selection: DesignEditorSelection, connection: Connection | ConnectionId): DesignEditorSelection => {
  const connectionObj = connectionToSelectionConnection(connection);

  const exists = selection.selectedConnections.some((c) => {
    const existingConnectionId = selectionConnectionToConnectionId(c);
    return isSameConnection(existingConnectionId, connection);
  });

  if (exists) return selection;
  return {
    selectedConnections: [...selection.selectedConnections, connectionObj],
    selectedPieceIds: selection.selectedPieceIds,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};
const removeConnectionFromSelection = (selection: DesignEditorSelection, connection: Connection | ConnectionId): DesignEditorSelection => {
  return {
    selectedConnections: selection.selectedConnections.filter((c: { connectingPieceId: string; connectedPieceId: string }) => {
      const existingConnectionId = selectionConnectionToConnectionId(c);
      return !isSameConnection(existingConnectionId, connection);
    }),
    selectedPieceIds: selection.selectedPieceIds,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};

const selectConnections = (connections: (Connection | ConnectionId)[]): DesignEditorSelection => ({
  selectedConnections: connections.map((conn) => connectionToSelectionConnection(conn)),
  selectedPieceIds: [],
  selectedPiecePortId: undefined,
});
const addConnectionsToSelection = (selection: DesignEditorSelection, connections: (Connection | ConnectionId)[]): DesignEditorSelection => {
  const newConnections = connections
    .map((conn) => connectionToSelectionConnection(conn))
    .filter((conn) => {
      const connectionId = selectionConnectionToConnectionId(conn);
      return !selection.selectedConnections.some((c) => {
        const existingConnectionId = selectionConnectionToConnectionId(c);
        return isSameConnection(existingConnectionId, connectionId);
      });
    });
  return {
    ...selection,
    selectedConnections: [...selection.selectedConnections, ...newConnections],
  };
};
const removeConnectionsFromSelection = (selection: DesignEditorSelection, connections: (Connection | ConnectionId)[]): DesignEditorSelection => {
  return {
    selectedConnections: selection.selectedConnections.filter((c: { connectingPieceId: string; connectedPieceId: string }) => {
      const existingConnectionId = selectionConnectionToConnectionId(c);
      return !connections.some((conn) => isSameConnection(existingConnectionId, conn));
    }),
    selectedPieceIds: selection.selectedPieceIds,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};

const selectPiecePort = (pieceId: string, portId: string): DesignEditorSelection => ({
  selectedPieceIds: [],
  selectedConnections: [],
  selectedPiecePortId: { pieceId, portId },
});
const deselectPiecePort = (selection: DesignEditorSelection): DesignEditorSelection => ({ ...selection, selectedPiecePortId: undefined });

const invertSelection = (selection: DesignEditorSelection, design: Design): DesignEditorSelection => {
  const allPieceIds = design.pieces?.map((p: Piece) => p.id_) || [];
  const allConnections = design.connections?.map((c: Connection) => connectionToSelectionConnection(c)) || [];
  const newSelectedPieceIds = allPieceIds.filter((id) => !selection.selectedPieceIds.includes(id));
  const newSelectedConnections = allConnections.filter((conn) => {
    const connectionId = selectionConnectionToConnectionId(conn);
    return !selection.selectedConnections.some((selected) => {
      const selectedConnectionId = selectionConnectionToConnectionId(selected);
      return isSameConnection(selectedConnectionId, connectionId);
    });
  });
  return {
    selectedPieceIds: newSelectedPieceIds,
    selectedConnections: newSelectedConnections,
    selectedPiecePortId: undefined,
  };
};

const invertPiecesSelection = (selection: DesignEditorSelection, design: Design): DesignEditorSelection => {
  const allPieceIds = design.pieces?.map((p: Piece) => p.id_) || [];
  const newSelectedPieceIds = allPieceIds.filter((id) => !selection.selectedPieceIds.includes(id));
  return {
    selectedPieceIds: newSelectedPieceIds,
    selectedConnections: selection.selectedConnections,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};
const invertConnectionsSelection = (selection: DesignEditorSelection, design: Design): DesignEditorSelection => {
  const allConnections = design.connections?.map((c: Connection) => connectionToSelectionConnection(c)) || [];
  const newSelectedConnections = allConnections.filter((conn) => {
    const connectionId = selectionConnectionToConnectionId(conn);
    return !selection.selectedConnections.some((selected) => {
      const selectedConnectionId = selectionConnectionToConnectionId(selected);
      return isSameConnection(selectedConnectionId, connectionId);
    });
  });
  return {
    selectedPieceIds: selection.selectedPieceIds,
    selectedConnections: newSelectedConnections,
    selectedPiecePortId: selection.selectedPiecePortId,
  };
};

const subDesignFromSelection = (design: Design, selection: DesignEditorSelection): Design => {
  const subPieces = design.pieces?.filter((p) => selection.selectedPieceIds.includes(p.id_));
  const subConnections = design.connections?.filter((c) => selection.selectedConnections.some((sc) => isSameConnection(selectionConnectionToConnectionId(sc), c)));
  return { ...design, pieces: subPieces, connections: subConnections };
};

const copyToClipboard = (design: Design, selection: DesignEditorSelection, plane?: Plane, center?: DiagramPoint): void => {
  navigator.clipboard.writeText(JSON.stringify(orientDesign(subDesignFromSelection(design, selection), plane, center))).then(() => {});
};
const cutToClipboard = (design: Design, selection: DesignEditorSelection, plane?: Plane, center?: DiagramPoint): void => {
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
const deleteSelected = (kit: Kit, designId: DesignId, selection: DesignEditorSelection): Design => {
  const selectedPieces = selection.selectedPieceIds.map((id) => ({ id_: id }));
  const selectedConnections = selection.selectedConnections.map((conn) => ({
    connecting: { piece: { id_: conn.connectingPieceId } },
    connected: { piece: { id_: conn.connectedPieceId } },
  }));
  const updatedDesign = removePiecesAndConnectionsFromDesign(kit, designId, selectedPieces, selectedConnections);
  return updatedDesign;
};

//#endregion Selection

//#region Operation Stack

const pushToOperationStack = (state: DesignEditorState): DesignEditorState => {
  const currentDesign = findDesignInKit(state.kit, state.designId);
  const newEntry: OperationStackEntry = {
    design: JSON.parse(JSON.stringify(currentDesign)),
    selection: JSON.parse(JSON.stringify(state.selection)),
  };

  let newOperationStack: OperationStackEntry[];
  let newOperationIndex: number;

  if (state.operationIndex < state.operationStack.length - 1) {
    newOperationStack = state.operationStack.slice(0, state.operationIndex + 1);
  } else {
    newOperationStack = [...state.operationStack];
  }

  newOperationStack.push(newEntry);

  if (newOperationStack.length > COMMAND_STACK_MAX) {
    newOperationStack.shift();
    newOperationIndex = newOperationStack.length - 1;
  } else {
    newOperationIndex = newOperationStack.length - 1;
  }

  return {
    ...state,
    operationStack: newOperationStack,
    operationIndex: newOperationIndex,
  };
};

const canUndo = (state: DesignEditorState): boolean => {
  if (state.operationStack.length === 0) return false;
  return state.operationIndex > 0 && state.operationStack.length > 1;
};

const canRedo = (state: DesignEditorState): boolean => {
  if (state.operationStack.length === 0) return false;
  return state.operationIndex < state.operationStack.length - 1;
};

const undo = (state: DesignEditorState): DesignEditorState => {
  if (!canUndo(state)) return state;

  const previousEntry = state.operationStack[state.operationIndex - 1];
  const currentDesign = findDesignInKit(state.kit, state.designId);
  const updatedDesigns = (state.kit.designs || []).map((d: Design) => (isSameDesign(d, currentDesign) ? previousEntry.design : d));
  return {
    ...state,
    kit: { ...state.kit, designs: updatedDesigns },
    selection: previousEntry.selection,
    operationIndex: state.operationIndex - 1,
  };
};

const redo = (state: DesignEditorState): DesignEditorState => {
  if (!canRedo(state)) return state;

  const nextEntry = state.operationStack[state.operationIndex + 1];
  const currentDesign = findDesignInKit(state.kit, state.designId);
  const updatedDesigns = (state.kit.designs || []).map((d: Design) => (isSameDesign(d, currentDesign) ? nextEntry.design : d));
  return {
    ...state,
    kit: { ...state.kit, designs: updatedDesigns },
    selection: nextEntry.selection,
    operationIndex: state.operationIndex + 1,
  };
};

//#endregion Operation Stack

//#region DesignDiff Helpers

const updateDesignDiffInState = (state: DesignEditorState, updatedDesignDiff: DesignDiff): DesignEditorState => {
  return { ...state, designDiff: updatedDesignDiff };
};

const resetDesignDiff = (): DesignDiff => ({
  pieces: { added: [], removed: [], updated: [] },
  connections: { added: [], removed: [], updated: [] },
});

//#endregion DesignDiff Helpers

export const DesignEditorContext = createContext<{ state: DesignEditorState; dispatch: DesignEditorDispatcher } | undefined>(undefined);

export const useDesignEditor = () => {
  const context = useContext(DesignEditorContext);
  if (!context) {
    throw new Error("useDesignEditor must be used within a DesignEditorProvider");
  }
  const { state, dispatch } = context;

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
  const setSelection = useCallback((s: DesignEditorSelection) => dispatch({ type: DesignEditorAction.SetSelection, payload: s }), [dispatch]);
  const selectAll = useCallback(() => dispatch({ type: DesignEditorAction.SelectAll, payload: null }), [dispatch]);
  const deselectAll = useCallback(() => dispatch({ type: DesignEditorAction.DeselectAll, payload: null }), [dispatch]);
  const invertSelection = useCallback(() => dispatch({ type: DesignEditorAction.InvertSelection, payload: null }), [dispatch]);
  const invertPiecesSelection = useCallback(
    () =>
      dispatch({
        type: DesignEditorAction.InvertPiecesSelection,
        payload: null,
      }),
    [dispatch],
  );
  const invertConnectionsSelection = useCallback(
    () =>
      dispatch({
        type: DesignEditorAction.InvertConnectionsSelection,
        payload: null,
      }),
    [dispatch],
  );
  const addAllPiecesToSelection = useCallback(
    () =>
      dispatch({
        type: DesignEditorAction.AddAllPiecesToSelection,
        payload: null,
      }),
    [dispatch],
  );
  const removeAllPiecesFromSelection = useCallback(
    () =>
      dispatch({
        type: DesignEditorAction.RemoveAllPiecesFromSelection,
        payload: null,
      }),
    [dispatch],
  );
  const addAllConnectionsToSelection = useCallback(
    () =>
      dispatch({
        type: DesignEditorAction.AddAllConnectionsToSelection,
        payload: null,
      }),
    [dispatch],
  );
  const removeAllConnectionsFromSelection = useCallback(
    () =>
      dispatch({
        type: DesignEditorAction.RemoveAllConnectionsFromSelection,
        payload: null,
      }),
    [dispatch],
  );
  const selectPiece = useCallback((p: Piece | PieceId) => dispatch({ type: DesignEditorAction.SelectPiece, payload: p }), [dispatch]);
  const addPieceToSelection = useCallback((p: Piece | PieceId) => dispatch({ type: DesignEditorAction.AddPieceToSelection, payload: p }), [dispatch]);
  const removePieceFromSelection = useCallback(
    (p: Piece | PieceId) =>
      dispatch({
        type: DesignEditorAction.RemovePieceFromSelection,
        payload: p,
      }),
    [dispatch],
  );
  const selectPieces = useCallback((ps: (Piece | PieceId)[]) => dispatch({ type: DesignEditorAction.SelectPieces, payload: ps }), [dispatch]);
  const addPiecesToSelection = useCallback((ps: (Piece | PieceId)[]) => dispatch({ type: DesignEditorAction.AddPiecesToSelection, payload: ps }), [dispatch]);
  const removePiecesFromSelection = useCallback(
    (ps: (Piece | PieceId)[]) =>
      dispatch({
        type: DesignEditorAction.RemovePiecesFromSelection,
        payload: ps,
      }),
    [dispatch],
  );
  const selectConnection = useCallback((c: Connection | ConnectionId) => dispatch({ type: DesignEditorAction.SelectConnection, payload: c }), [dispatch]);
  const addConnectionToSelection = useCallback(
    (c: Connection | ConnectionId) =>
      dispatch({
        type: DesignEditorAction.AddConnectionToSelection,
        payload: c,
      }),
    [dispatch],
  );
  const removeConnectionFromSelection = useCallback(
    (c: Connection | ConnectionId) =>
      dispatch({
        type: DesignEditorAction.RemoveConnectionFromSelection,
        payload: c,
      }),
    [dispatch],
  );
  const selectConnections = useCallback((cs: (Connection | ConnectionId)[]) => dispatch({ type: DesignEditorAction.SelectConnections, payload: cs }), [dispatch]);
  const addConnectionsToSelection = useCallback(
    (cs: (Connection | ConnectionId)[]) =>
      dispatch({
        type: DesignEditorAction.AddConnectionsToSelection,
        payload: cs,
      }),
    [dispatch],
  );
  const removeConnectionsFromSelection = useCallback(
    (cs: (Connection | ConnectionId)[]) =>
      dispatch({
        type: DesignEditorAction.RemoveConnectionsFromSelection,
        payload: cs,
      }),
    [dispatch],
  );
  const selectPiecePort = useCallback(
    (pieceId: string, portId: string) =>
      dispatch({
        type: DesignEditorAction.SelectPiecePort,
        payload: { pieceId, portId },
      }),
    [dispatch],
  );
  const deselectPiecePort = useCallback(() => dispatch({ type: DesignEditorAction.DeselectPiecePort, payload: null }), [dispatch]);
  const deleteSelected = useCallback(
    (plane?: Plane, center?: DiagramPoint) =>
      dispatch({
        type: DesignEditorAction.DeleteSelected,
        payload: { plane, center },
      }),
    [dispatch],
  );
  const setFullscreen = useCallback((fp: FullscreenPanel) => dispatch({ type: DesignEditorAction.SetFullscreen, payload: fp }), [dispatch]);
  const toggleDiagramFullscreen = useCallback(
    () =>
      dispatch({
        type: DesignEditorAction.ToggleDiagramFullscreen,
        payload: null,
      }),
    [dispatch],
  );
  const toggleModelFullscreen = useCallback(
    () =>
      dispatch({
        type: DesignEditorAction.ToggleModelFullscreen,
        payload: null,
      }),
    [dispatch],
  );
  const copyToClipboard = useCallback(() => dispatch({ type: DesignEditorAction.CopyToClipboard, payload: null }), [dispatch]);
  const pasteFromClipboard = useCallback(
    (plane?: Plane, center?: DiagramPoint) =>
      dispatch({
        type: DesignEditorAction.PasteFromClipboard,
        payload: { plane, center },
      }),
    [dispatch],
  );
  const undo = useCallback(() => dispatch({ type: DesignEditorAction.Undo, payload: null }), [dispatch]);
  const redo = useCallback(() => dispatch({ type: DesignEditorAction.Redo, payload: null }), [dispatch]);

  // Transaction management
  const startTransaction = useCallback(() => dispatch({ type: DesignEditorAction.StartTransaction, payload: null }), [dispatch]);
  const finalizeTransaction = useCallback(() => dispatch({ type: DesignEditorAction.FinalizeTransaction, payload: null }), [dispatch]);
  const abortTransaction = useCallback(() => dispatch({ type: DesignEditorAction.AbortTransaction, payload: null }), [dispatch]);

  // Cursor and camera management
  const setCursor = useCallback((cursor: DiagramPoint | undefined) => dispatch({ type: DesignEditorAction.SetCursor, payload: cursor }), [dispatch]);
  const setCamera = useCallback((camera: Camera | undefined) => dispatch({ type: DesignEditorAction.SetCamera, payload: camera }), [dispatch]);

  // Presence management
  const stepIn = useCallback((presence: Presence) => dispatch({ type: DesignEditorAction.StepIn, payload: presence }), [dispatch]);
  const stepOut = useCallback((presence: Presence) => dispatch({ type: DesignEditorAction.StepOut, payload: presence }), [dispatch]);
  const updatePresence = useCallback((presence: Partial<Presence> & { name: string }) => dispatch({ type: DesignEditorAction.UpdatePresence, payload: presence }), [dispatch]);

  return {
    ...state,
    canUndo: canUndo(state),
    canRedo: canRedo(state),
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
    undo,
    redo,
    // Command system
    executeCommand: useCallback(
      async (commandId: string, payload: Record<string, any> = {}) => {
        const context: CommandContext = {
          kit: state.kit,
          designId: state.designId,
          selection: state.selection,
        };

        const command = commandRegistry.get(commandId);
        if (!command) {
          throw new Error(`Command not found: ${commandId}`);
        }

        // Editor-only commands can always execute, even during transactions
        if (command.editorOnly) {
          const result = await commandRegistry.execute(commandId, context, payload);
          if (result.selection) {
            setSelection(result.selection);
          }
          if (result.fullscreenPanel !== undefined) {
            setFullscreen(result.fullscreenPanel);
          }
          return result;
        }

        // Design-modifying commands only execute when no transaction is active
        if (state.isTransactionActive) {
          console.warn(`Cannot execute design-modifying command "${commandId}" during active transaction`);
          return;
        }

        // Run design commands in transactions
        startTransaction();
        try {
          const result = await commandRegistry.execute(commandId, context, payload);
          if (result.design) {
            setDesign(result.design);
          }
          if (result.selection) {
            setSelection(result.selection);
          }
          if (result.fullscreenPanel !== undefined) {
            setFullscreen(result.fullscreenPanel);
          }
          finalizeTransaction();
          return result;
        } catch (error) {
          abortTransaction();
          console.error("Command execution failed:", error);
          throw error;
        }
      },
      [state.kit, state.designId, state.selection, state.isTransactionActive, startTransaction, setDesign, setSelection, setFullscreen, finalizeTransaction, abortTransaction],
    ),

    getAvailableCommands: useCallback(() => commandRegistry.getAll(), []),
    getCommand: useCallback((commandId: string) => commandRegistry.get(commandId), []),
    // Transaction functions
    startTransaction,
    finalizeTransaction,
    abortTransaction,
    // Cursor and camera functions
    setCursor,
    setCamera,
    // Presence functions
    stepIn,
    stepOut,
    updatePresence,
  };
};

const designEditorReducer = (state: DesignEditorState, action: { type: DesignEditorAction; payload: any }): DesignEditorState => {
  const currentDesign = findDesignInKit(state.kit, state.designId);

  const updateDesignInDesignEditorStateWithOperationStack = (updatedDesign: Design): DesignEditorState => {
    const stateWithOperation = pushToOperationStack(state);
    const updatedDesigns = (stateWithOperation.kit.designs || []).map((d: Design) => (isSameDesign(d, currentDesign) ? updatedDesign : d));
    return {
      ...stateWithOperation,
      kit: { ...stateWithOperation.kit, designs: updatedDesigns },
    };
  };

  const updateDesignInDesignEditorState = (updatedDesign: Design): DesignEditorState => {
    const updatedDesigns = (state.kit.designs || []).map((d: Design) => (isSameDesign(d, currentDesign) ? updatedDesign : d));
    return { ...state, kit: { ...state.kit, designs: updatedDesigns } };
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
      const designWithRemovedPiece = removePieceFromDesign(state.kit, state.designId, action.payload);
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
      const designWithRemovedPieces = removePiecesFromDesign(state.kit, state.designId, action.payload);
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
      const designWithRemovedConnection = removeConnectionFromDesign(state.kit, state.designId, action.payload);
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
      const designWithRemovedConnections = removeConnectionsFromDesign(state.kit, state.designId, action.payload);
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
      const designWithRemovedPiecesAndConnections = removePiecesAndConnectionsFromDesign(state.kit, state.designId, action.payload.pieceIds, action.payload.connectionIds);
      return updateDesignInDesignEditorStateWithOperationStack(designWithRemovedPiecesAndConnections);
    case DesignEditorAction.DeleteSelected:
      const stateWithDeleteOperation = pushToOperationStack(state);
      const selectionToDelete = stateWithDeleteOperation.selection;
      const updatedDesign = deleteSelected(stateWithDeleteOperation.kit, stateWithDeleteOperation.designId, selectionToDelete);
      const entryWithCorrectSelection = stateWithDeleteOperation.operationStack[stateWithDeleteOperation.operationIndex];
      entryWithCorrectSelection.selection = selectionToDelete;
      return {
        ...stateWithDeleteOperation,
        kit: updateDesignInKit(stateWithDeleteOperation.kit, updatedDesign),
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
      const updatedDesigns = (stateWithFinalizedTransaction.kit.designs || []).map((d: Design) => (isSameDesign(d, currentDesign) ? finalDesign : d));
      const entryWithCorrectTransactionState = stateWithFinalizedTransaction.operationStack[stateWithFinalizedTransaction.operationIndex];
      entryWithCorrectTransactionState.selection = state.selection;
      return {
        ...stateWithFinalizedTransaction,
        kit: { ...stateWithFinalizedTransaction.kit, designs: updatedDesigns },
      };
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
      return undo(state);
    case DesignEditorAction.Redo:
      return redo(state);

    // Other (no operation stack)
    case DesignEditorAction.SetFullscreen:
      return { ...state, fullscreenPanel: action.payload };
    case DesignEditorAction.ToggleDiagramFullscreen:
      return {
        ...state,
        fullscreenPanel: state.fullscreenPanel === FullscreenPanel.Diagram ? FullscreenPanel.None : FullscreenPanel.Diagram,
      };
    case DesignEditorAction.ToggleModelFullscreen:
      return {
        ...state,
        fullscreenPanel: state.fullscreenPanel === FullscreenPanel.Model ? FullscreenPanel.None : FullscreenPanel.Model,
      };
    case DesignEditorAction.SetCursor:
      return { ...state, cursor: action.payload };
    case DesignEditorAction.SetCamera:
      return { ...state, camera: action.payload };
    case DesignEditorAction.StepIn:
      return { ...state, others: [...state.others, action.payload] };
    case DesignEditorAction.StepOut:
      return {
        ...state,
        others: state.others.filter((p) => p.name !== action.payload.name),
      };
    case DesignEditorAction.UpdatePresence:
      return {
        ...state,
        others: state.others.map((p) => (p.name === action.payload.name ? { ...p, ...action.payload } : p)),
      };
    default:
      return state;
  }
};

function useControllableReducer(props: DesignEditorProps) {
  const { kit: controlledKit, selection: controlledSelection, initialKit, initialSelection, onDesignChange, onSelectionChange, onUndo, onRedo, designId, fileUrls } = props;

  const isKitControlled = controlledKit !== undefined;
  const isSelectionControlled = controlledSelection !== undefined;

  const initialDesign = findDesignInKit(initialKit!, designId);
  const initialState: DesignEditorState = {
    kit: initialKit!,
    designId: designId,
    fileUrls: fileUrls,
    fullscreenPanel: FullscreenPanel.None,
    selection: initialSelection || {
      selectedPieceIds: [],
      selectedConnections: [],
      selectedPiecePortId: undefined,
    },
    designDiff: {
      pieces: { added: [], removed: [], updated: [] },
      connections: { added: [], removed: [], updated: [] },
    },
    operationStack: [
      {
        design: JSON.parse(JSON.stringify(initialDesign)),
        selection: JSON.parse(
          JSON.stringify(
            initialSelection || {
              selectedPieceIds: [],
              selectedConnections: [],
              selectedPiecePortId: undefined,
            },
          ),
        ),
      },
    ],
    operationIndex: 0,
    isTransactionActive: false,
    others: [],
  };

  const [internalState, dispatch] = useReducer(designEditorReducer, initialState);

  const state: DesignEditorState = {
    ...internalState,
    kit: isKitControlled ? controlledKit : internalState.kit,
    selection: isSelectionControlled ? controlledSelection : internalState.selection,
    operationStack: isKitControlled || isSelectionControlled ? [] : internalState.operationStack,
    operationIndex: isKitControlled || isSelectionControlled ? -1 : internalState.operationIndex,
    isTransactionActive: internalState.isTransactionActive,
  };

  const dispatchWrapper = useCallback(
    (action: { type: DesignEditorAction; payload: any }) => {
      console.log("ACTION:", action.type, action.payload);
      // console.log('OLDSTATE:', state)

      if (action.type === DesignEditorAction.Undo && onUndo) {
        onUndo();
        return;
      }
      if (action.type === DesignEditorAction.Redo && onRedo) {
        onRedo();
        return;
      }

      const newState = designEditorReducer(state, action);
      // console.log('NEWSTATE:', newState)

      if (!isKitControlled || !isSelectionControlled) dispatch(action);
      if (isKitControlled && newState.kit !== state.kit) onDesignChange?.(findDesignInKit(newState.kit, designId));
      if (isSelectionControlled && newState.selection !== state.selection) onSelectionChange?.(newState.selection);
    },
    [state, isKitControlled, isSelectionControlled, designId, onDesignChange, onSelectionChange, onUndo, onRedo],
  );

  return [state, dispatchWrapper] as const;
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

interface ControlledDesignEditorProps {
  kit?: Kit;
  selection?: DesignEditorSelection;
  onDesignChange?: (design: Design) => void;
  onSelectionChange?: (selection: DesignEditorSelection) => void;
  onUndo?: () => void;
  onRedo?: () => void;
}

interface UncontrolledDesignEditorProps {
  initialKit?: Kit;
  initialSelection?: DesignEditorSelection;
}

interface DesignEditorProps extends ControlledDesignEditorProps, UncontrolledDesignEditorProps {
  designId: DesignId;
  fileUrls: Map<string, string>;
  onToolbarChange: (toolbar: ReactNode) => void;
  mode?: Mode;
  layout?: Layout;
  theme?: Theme;
  setLayout?: (layout: Layout) => void;
  setTheme?: (theme: Theme) => void;
  onWindowEvents?: {
    minimize: () => void;
    maximize: () => void;
    close: () => void;
  };
}

const DesignEditorCore: FC<DesignEditorProps> = (props) => {
  const { onToolbarChange, designId, onUndo: controlledOnUndo, onRedo: controlledOnRedo } = props;

  const [state, dispatch] = useControllableReducer(props);
  if (!state.kit) return null;
  const design = findDesignInKit(state.kit, designId);
  if (!design) return null;

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
      e.stopPropagation();
      dispatch({
        type: DesignEditorAction.ToggleDiagramFullscreen,
        payload: null,
      });
    },
    [dispatch],
  );

  const onDoubleClickModel = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      dispatch({
        type: DesignEditorAction.ToggleModelFullscreen,
        payload: null,
      });
    },
    [dispatch],
  );

  const toggleWorkbench = useCallback(() => togglePanel("workbench"), []);
  const toggleConsole = useCallback(() => togglePanel("console"), []);
  const toggleDetails = useCallback(() => togglePanel("details"), []);
  const toggleChat = useCallback(() => togglePanel("chat"), []);

  const onUndo = controlledOnUndo || (() => dispatch({ type: DesignEditorAction.Undo, payload: null }));
  const onRedo = controlledOnRedo || (() => dispatch({ type: DesignEditorAction.Redo, payload: null }));

  // Presence functions using dispatch directly
  const stepIn = useCallback((presence: Presence) => dispatch({ type: DesignEditorAction.StepIn, payload: presence }), [dispatch]);
  const stepOut = useCallback((presence: Presence) => dispatch({ type: DesignEditorAction.StepOut, payload: presence }), [dispatch]);
  const updatePresence = useCallback((presence: Partial<Presence> & { name: string }) => dispatch({ type: DesignEditorAction.UpdatePresence, payload: presence }), [dispatch]);

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
  }, []);

  const designEditorToolbar = (
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
  );

  useEffect(() => {
    onToolbarChange(designEditorToolbar);
    return () => onToolbarChange(null);
  }, [visiblePanels]);

  const { screenToFlowPosition } = useReactFlow();
  const [activeDraggedType, setActiveDraggedType] = useState<Type | null>(null);
  const [activeDraggedDesign, setActiveDraggedDesign] = useState<Design | null>(null);

  // Register built-in commands
  useEffect(() => {
    // Register all centralized commands
    const unregisterFunctions = designEditorCommands.map((command) => commandRegistry.register(command));

    return () => unregisterFunctions.forEach((fn) => fn());
  }, [commandRegistry]);

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

      const context = { kit: state.kit, designId, selection: state.selection };

      // Editor-only commands can always execute, even during transactions
      if (command.editorOnly) {
        try {
          const result = await commandRegistry.execute(commandId, context, payload);
          if (result.selection)
            dispatch({
              type: DesignEditorAction.SetSelection,
              payload: result.selection,
            });
          if (result.fullscreenPanel !== undefined)
            dispatch({
              type: DesignEditorAction.SetFullscreen,
              payload: result.fullscreenPanel,
            });
        } catch (error) {
          console.error(`Error executing editor-only command ${commandId}:`, error);
        }
        return;
      }

      // Design-modifying commands only execute when no transaction is active
      if (state.isTransactionActive) {
        console.warn(`Cannot execute design-modifying command "${commandId}" during active transaction`);
        return;
      }

      // Run design commands in transactions
      dispatch({ type: DesignEditorAction.StartTransaction, payload: null });
      try {
        const result = await commandRegistry.execute(commandId, context, payload);
        if (result.design)
          dispatch({
            type: DesignEditorAction.SetDesign,
            payload: result.design,
          });
        if (result.selection)
          dispatch({
            type: DesignEditorAction.SetSelection,
            payload: result.selection,
          });
        if (result.fullscreenPanel !== undefined)
          dispatch({
            type: DesignEditorAction.SetFullscreen,
            payload: result.fullscreenPanel,
          });
        dispatch({
          type: DesignEditorAction.FinalizeTransaction,
          payload: null,
        });
      } catch (error) {
        dispatch({ type: DesignEditorAction.AbortTransaction, payload: null });
        console.error(`Error executing command ${commandId}:`, error);
      }
    };

    document.addEventListener("semio-command", handleCommand);
    return () => document.removeEventListener("semio-command", handleCommand);
  }, [state.kit, designId, state.selection, state.isTransactionActive, dispatch]);

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
          if (state.isTransactionActive) {
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
    onUndo();
  });

  useHotkeys("mod+y", async (e) => {
    e.preventDefault();
    e.stopPropagation();
    onRedo();
  });

  const onDragStart = (event: DragStartEvent) => {
    const { active } = event;
    const id = active.id.toString();
    if (id.startsWith("type-")) {
      const [_, name, variant] = id.split("-");
      const normalizeVariant = (v: string | undefined | null) => v ?? "";
      const type = state.kit?.types?.find((t: Type) => t.name === name && normalizeVariant(t.variant) === normalizeVariant(variant));
      setActiveDraggedType(type || null);
    } else if (id.startsWith("design-")) {
      const [_, name, variant, view] = id.split("-");
      const draggedDesignId: DesignId = {
        name,
        variant: variant || undefined,
        view: view || undefined,
      };
      const draggedDesign = findDesignInKit(state.kit, draggedDesignId);
      setActiveDraggedDesign(draggedDesign || null);
    }
  };

  const onDragEnd = (event: DragEndEvent) => {
    const { over } = event;
    if (over?.id === "diagram") {
      if (!(event.activatorEvent instanceof PointerEvent)) {
        return;
      }
      if (activeDraggedType) {
        const { x, y } = screenToFlowPosition({
          x: event.activatorEvent.clientX + event.delta.x,
          y: event.activatorEvent.clientY + event.delta.y,
        });
        const piece: Piece = {
          id_: Generator.randomId(),
          type: {
            name: activeDraggedType.name,
            variant: activeDraggedType.variant || undefined,
          },
          plane: {
            origin: { x: 0, y: 0, z: 0 },
            xAxis: { x: 1, y: 0, z: 0 },
            yAxis: { x: 0, y: 1, z: 0 },
          },
          center: { x: x / ICON_WIDTH - 0.5, y: -y / ICON_WIDTH + 0.5 },
        };
        dispatch({
          type: DesignEditorAction.AddPiece,
          payload: piece,
        });
      } else if (activeDraggedDesign) {
        throw new Error("Not implemented");
      }
    }
    setActiveDraggedType(null);
    setActiveDraggedDesign(null);
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

  return (
    <DesignEditorContext.Provider value={{ state, dispatch }}>
      <DndContext onDragStart={onDragStart} onDragEnd={onDragEnd}>
        <div className="canvas flex-1 relative">
          <div id="sketchpad-edgeless" className="h-full">
            <ResizablePanelGroup direction="horizontal">
              <ResizablePanel defaultSize={state.fullscreenPanel === FullscreenPanel.Diagram ? 100 : 50} className={`${state.fullscreenPanel === FullscreenPanel.Model ? "hidden" : "block"}`} onDoubleClick={onDoubleClickDiagram}>
                <Diagram />
              </ResizablePanel>
              <ResizableHandle className={`border-r ${state.fullscreenPanel !== FullscreenPanel.None ? "hidden" : "block"}`} />
              <ResizablePanel defaultSize={state.fullscreenPanel === FullscreenPanel.Model ? 100 : 50} className={`${state.fullscreenPanel === FullscreenPanel.Diagram ? "hidden" : "block"}`} onDoubleClick={onDoubleClickModel}>
                <Model />
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
              {activeDraggedType && <TypeAvatar type={activeDraggedType} />}
              {activeDraggedDesign && <DesignAvatar design={activeDraggedDesign} />}
            </DragOverlay>,
            document.body,
          )}
        </div>
      </DndContext>
    </DesignEditorContext.Provider>
  );
};

const DesignEditor: FC<DesignEditorProps> = ({ mode, layout, theme, setLayout, setTheme, onWindowEvents, designId, kit, selection, initialKit, initialSelection, fileUrls, onDesignChange, onSelectionChange, onUndo, onRedo, onToolbarChange }) => {
  const [toolbarContent, setToolbarContent] = useState<ReactNode>(null);

  useEffect(() => {
    onToolbarChange?.(toolbarContent);
  }, [toolbarContent, onToolbarChange]);

  return (
    <div key={`layout-${layout}`} className="h-full w-full flex flex-col bg-background text-foreground">
      <Navbar mode={mode} toolbarContent={toolbarContent} layout={layout} theme={theme} setLayout={setLayout} setTheme={setTheme} onWindowEvents={onWindowEvents} />
      <ReactFlowProvider>
        <DesignEditorCore
          kit={kit}
          designId={designId}
          fileUrls={fileUrls}
          selection={selection}
          initialKit={initialKit}
          initialSelection={initialSelection}
          onDesignChange={onDesignChange}
          onSelectionChange={onSelectionChange}
          onUndo={onUndo}
          onRedo={onRedo}
          onToolbarChange={setToolbarContent}
          mode={mode}
          layout={layout}
          theme={theme}
          setLayout={setLayout}
          setTheme={setTheme}
          onWindowEvents={onWindowEvents}
        />
      </ReactFlowProvider>
    </div>
  );
};

export default DesignEditor;

//#endregion Components
