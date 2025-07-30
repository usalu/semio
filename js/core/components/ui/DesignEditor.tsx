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

const COMMAND_STACK_MAX = 50

import { DndContext, DragEndEvent, DragOverlay, DragStartEvent, useDraggable, closestCenter } from '@dnd-kit/core'
import { SortableContext, verticalListSortingStrategy, useSortable, arrayMove } from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import { ReactFlowProvider, useReactFlow } from '@xyflow/react'
import { ChevronUp, ChevronDown, GripVertical, Info, MessageCircle, Minus, Pin, Plus, Terminal, Trash2, Wrench } from 'lucide-react'
import { FC, ReactNode, createContext, useCallback, useContext, useEffect, useReducer, useRef, useState } from 'react'
import { createPortal } from 'react-dom'
import { useHotkeys } from 'react-hotkeys-hook'

import {
  Connection,
  ConnectionId,
  Design, DesignDiff, DesignId, Diagram,
  DiagramPoint,
  ICON_WIDTH, Kit, Model, Piece, PieceId,
  Plane, Type,
  addConnectionToDesign,
  addConnectionsToDesign,
  addPieceToDesign,
  addPiecesToDesign,
  findConnectionInDesign,
  findDesignInKit,
  findPieceInDesign,
  findReplacableTypesForPieceInDesign,
  findReplacableTypesForPiecesInDesign,
  fixPiecesInDesign,
  isSameConnection,
  isSameDesign,
  mergeDesigns,
  piecesMetadata,
  removeConnectionFromDesign,
  removeConnectionsFromDesign,
  removePieceFromDesign,
  removePiecesAndConnectionsFromDesign,
  removePiecesFromDesign,
  setConnectionInDesign,
  setConnectionsInDesign,
  setPieceInDesign,
  setPiecesInDesign,
  updateDesignInKit
} from '@semio/js'
import { Avatar, AvatarFallback } from '@semio/js/components/ui/Avatar'
import Combobox from '@semio/js/components/ui/Combobox'
import { HoverCard, HoverCardContent, HoverCardTrigger } from '@semio/js/components/ui/HoverCard'
import { Input } from '@semio/js/components/ui/Input'
import { default as Navbar } from '@semio/js/components/ui/Navbar'
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '@semio/js/components/ui/Resizable'
import { ScrollArea } from '@semio/js/components/ui/ScrollArea'
import { Layout, Mode, Theme } from '@semio/js/components/ui/Sketchpad'
import { Slider } from '@semio/js/components/ui/Slider'
import Stepper from '@semio/js/components/ui/Stepper'
import { Textarea } from '@semio/js/components/ui/Textarea'
import { ToggleGroup, ToggleGroupItem } from '@semio/js/components/ui/ToggleGroup'
import { Tree, TreeItem, TreeSection } from '@semio/js/components/ui/Tree'
import { Generator } from '@semio/js/lib/utils'
import { flattenDesign, orientDesign } from '../../semio'


//#region State

export interface DesignEditorSelection {
  selectedPieceIds: string[];
  selectedConnections: {
    connectingPieceId: string;
    connectedPieceId: string;
  }[];
}

export interface OperationStackEntry {
  design: Design
  selection: DesignEditorSelection
}

export interface DesignEditorState {
  kit: Kit
  designId: DesignId
  fileUrls: Map<string, string>
  fullscreenPanel: FullscreenPanel
  selection: DesignEditorSelection
  designDiff: DesignDiff
  operationStack: OperationStackEntry[]
  operationIndex: number
  isTransactionActive: boolean
}

export enum FullscreenPanel {
  None = 'none',
  Diagram = 'diagram',
  Model = 'model'
}

export enum DesignEditorAction {
  SetDesign = 'SET_DESIGN',
  AddPieceToDesign = 'ADD_PIECE_TO_DESIGN',
  SetPieceInDesign = 'SET_PIECE_IN_DESIGN',
  RemovePieceFromDesign = 'REMOVE_PIECE_FROM_DESIGN',
  AddPiecesToDesign = 'ADD_PIECES_TO_DESIGN',
  SetPiecesInDesign = 'SET_PIECES_IN_DESIGN',
  RemovePiecesFromDesign = 'REMOVE_PIECES_FROM_DESIGN',
  AddConnectionToDesign = 'ADD_CONNECTION_TO_DESIGN',
  SetConnectionInDesign = 'SET_CONNECTION_IN_DESIGN',
  RemoveConnectionFromDesign = 'REMOVE_CONNECTION_FROM_DESIGN',
  AddConnectionsToDesign = 'ADD_CONNECTIONS_TO_DESIGN',
  SetConnectionsInDesign = 'SET_CONNECTIONS_IN_DESIGN',
  RemoveConnectionsFromDesign = 'REMOVE_CONNECTIONS_FROM_DESIGN',
  RemovePiecesAndConnectionsFromDesign = 'REMOVE_PIECES_AND_CONNECTIONS_FROM_DESIGN',
  SetSelection = 'SET_SELECTION',
  SelectAll = 'SELECT_ALL',
  DeselectAll = 'DESELECT_ALL',
  InvertSelection = 'INVERT_SELECTION',
  InvertPiecesSelection = 'INVERT_PIECES_SELECTION',
  InvertConnectionsSelection = 'INVERT_CONNECTIONS_SELECTION',
  AddAllPiecesToSelection = 'ADD_ALL_PIECES_TO_SELECTION',
  RemoveAllPiecesFromSelection = 'REMOVE_ALL_PIECES_FROM_SELECTION',
  AddAllConnectionsToSelection = 'ADD_ALL_CONNECTIONS_TO_SELECTION',
  RemoveAllConnectionsFromSelection = 'REMOVE_ALL_CONNECTIONS_FROM_SELECTION',
  SelectPiece = 'SELECT_PIECE',
  AddPieceToSelection = 'ADD_PIECE_TO_SELECTION',
  RemovePieceFromSelection = 'REMOVE_PIECE_FROM_SELECTION',
  SelectPieces = 'SELECT_PIECES',
  AddPiecesToSelection = 'ADD_PIECES_TO_SELECTION',
  RemovePiecesFromSelection = 'REMOVE_PIECES_FROM_SELECTION',
  SelectConnection = 'SELECT_CONNECTION',
  AddConnectionToSelection = 'ADD_CONNECTION_TO_SELECTION',
  RemoveConnectionFromSelection = 'REMOVE_CONNECTION_FROM_SELECTION',
  SelectConnections = 'SELECT_CONNECTIONS',
  AddConnectionsToSelection = 'ADD_CONNECTIONS_TO_SELECTION',
  RemoveConnectionsFromSelection = 'REMOVE_CONNECTIONS_FROM_SELECTION',
  CopyToClipboard = 'COPY_TO_CliPBOARD',
  CutToClipboard = 'CUT_TO_CliPBOARD',
  PasteFromClipboard = 'PASTE_FROM_CliPBOARD',
  DeleteSelected = 'DELETE_SELECTED',
  SetFullscreen = 'SET_FULLSCREEN',
  Undo = 'UNDO',
  Redo = 'REDO',
  ToggleDiagramFullscreen = 'TOGGLE_DIAGRAM_FULLSCREEN',
  ToggleModelFullscreen = 'TOGGLE_MODEL_FULLSCREEN',
  StartTransaction = 'START_TRANSACTION',
  FinalizeTransaction = 'FINALIZE_TRANSACTION',
  AbortTransaction = 'ABORT_TRANSACTION'
}

export type DesignEditorDispatcher = (action: { type: DesignEditorAction; payload: any }) => void

//#region Selection

// TODO: Implement with proper world to world pattern

const selectionConnectionToConnectionId = (selectionConnection: { connectingPieceId: string; connectedPieceId: string }): ConnectionId => ({
  connected: { piece: { id_: selectionConnection.connectedPieceId } }, connecting: { piece: { id_: selectionConnection.connectingPieceId } }
})
const connectionToSelectionConnection = (connection: Connection | ConnectionId): { connectingPieceId: string; connectedPieceId: string } => ({
  connectingPieceId: connection.connecting.piece.id_, connectedPieceId: connection.connected.piece.id_
})

const selectAll = (design: Design): DesignEditorSelection => ({
  selectedPieceIds: design.pieces?.map((p: Piece) => p.id_) || [],
  selectedConnections: design.connections?.map((c: Connection) => ({ connectedPieceId: c.connected.piece.id_, connectingPieceId: c.connecting.piece.id_ })) || []
})
const deselectAll = (selection: DesignEditorSelection): DesignEditorSelection => ({ selectedPieceIds: [], selectedConnections: [] })
const addAllPiecesToSelection = (selection: DesignEditorSelection, design: Design): DesignEditorSelection => {
  const existingIds = new Set(selection.selectedPieceIds)
  const allPieceIds = design.pieces?.map((p: Piece) => p.id_) || []
  const newIds = allPieceIds.filter(id => !existingIds.has(id))
  return { selectedPieceIds: [...selection.selectedPieceIds, ...newIds], selectedConnections: selection.selectedConnections }
}
const removeAllPiecesFromSelection = (selection: DesignEditorSelection): DesignEditorSelection => ({ selectedPieceIds: [], selectedConnections: selection.selectedConnections })
const addAllConnectionsToSelection = (selection: DesignEditorSelection, design: Design): DesignEditorSelection => {
  const allConnections = design.connections?.map((c: Connection) => connectionToSelectionConnection(c)) || []
  const newConnections = allConnections.filter(conn => {
    const connectionId = selectionConnectionToConnectionId(conn)
    return !selection.selectedConnections.some((c) => {
      const existingConnectionId = selectionConnectionToConnectionId(c)
      return isSameConnection(existingConnectionId, connectionId)
    })
  })
  return { selectedPieceIds: selection.selectedPieceIds, selectedConnections: [...selection.selectedConnections, ...newConnections] }
}
const removeAllConnectionsFromSelection = (selection: DesignEditorSelection): DesignEditorSelection => ({ selectedPieceIds: selection.selectedPieceIds, selectedConnections: [] })

const selectPiece = (piece: Piece | PieceId): DesignEditorSelection => ({ selectedPieceIds: [typeof piece === 'string' ? piece : piece.id_], selectedConnections: [] })
const addPieceToSelection = (selection: DesignEditorSelection, piece: Piece | PieceId): DesignEditorSelection => {
  const pieceId = typeof piece === 'string' ? piece : piece.id_
  const existingPieceIds = new Set(selection.selectedPieceIds)
  const newPieceIds = existingPieceIds.has(pieceId) ? selection.selectedPieceIds.filter((id: string) => id !== pieceId) : [...selection.selectedPieceIds, pieceId]
  return ({ selectedPieceIds: newPieceIds, selectedConnections: selection.selectedConnections })
}
const removePieceFromSelection = (selection: DesignEditorSelection, piece: Piece | PieceId): DesignEditorSelection => {
  return ({
    selectedPieceIds: selection.selectedPieceIds.filter((id: string) => id !== (typeof piece === 'string' ? piece : piece.id_)),
    selectedConnections: selection.selectedConnections
  })
}

const selectPieces = (pieces: (Piece | PieceId)[]): DesignEditorSelection => ({ selectedPieceIds: pieces.map(p => typeof p === 'string' ? p : p.id_), selectedConnections: [] })
const addPiecesToSelection = (selection: DesignEditorSelection, pieces: (Piece | PieceId)[]): DesignEditorSelection => {
  const existingIds = new Set(selection.selectedPieceIds)
  const newIds = pieces
    .map(p => typeof p === 'string' ? p : p.id_)
    .filter(id => !existingIds.has(id))
  return ({ ...selection, selectedPieceIds: [...selection.selectedPieceIds, ...newIds] })
}
const removePiecesFromSelection = (selection: DesignEditorSelection, pieces: (Piece | PieceId)[]): DesignEditorSelection => {
  const idsToRemove = new Set(pieces.map(p => typeof p === 'string' ? p : p.id_))
  return ({ ...selection, selectedPieceIds: selection.selectedPieceIds.filter((id: string) => !idsToRemove.has(id)) })
}

const selectConnection = (connection: Connection | ConnectionId): DesignEditorSelection => ({ selectedConnections: [connectionToSelectionConnection(connection)], selectedPieceIds: [] })
const addConnectionToSelection = (selection: DesignEditorSelection, connection: Connection | ConnectionId): DesignEditorSelection => {
  const connectionObj = connectionToSelectionConnection(connection)

  const exists = selection.selectedConnections.some((c) => {
    const existingConnectionId = selectionConnectionToConnectionId(c)
    return isSameConnection(existingConnectionId, connection)
  })

  if (exists) return selection
  return ({ selectedConnections: [...selection.selectedConnections, connectionObj], selectedPieceIds: selection.selectedPieceIds })
}
const removeConnectionFromSelection = (selection: DesignEditorSelection, connection: Connection | ConnectionId): DesignEditorSelection => {
  return ({
    selectedConnections: selection.selectedConnections.filter((c: { connectingPieceId: string; connectedPieceId: string }) => {
      const existingConnectionId = selectionConnectionToConnectionId(c)
      return !isSameConnection(existingConnectionId, connection)
    }),
    selectedPieceIds: selection.selectedPieceIds
  })
}

const selectConnections = (connections: (Connection | ConnectionId)[]): DesignEditorSelection => ({ selectedConnections: connections.map(conn => connectionToSelectionConnection(conn)), selectedPieceIds: [] })
const addConnectionsToSelection = (selection: DesignEditorSelection, connections: (Connection | ConnectionId)[]): DesignEditorSelection => {
  const newConnections = connections
    .map(conn => connectionToSelectionConnection(conn))
    .filter(conn => {
      const connectionId = selectionConnectionToConnectionId(conn)
      return !selection.selectedConnections.some((c) => {
        const existingConnectionId = selectionConnectionToConnectionId(c)
        return isSameConnection(existingConnectionId, connectionId)
      })
    })
  return ({ ...selection, selectedConnections: [...selection.selectedConnections, ...newConnections] })
}
const removeConnectionsFromSelection = (selection: DesignEditorSelection, connections: (Connection | ConnectionId)[]): DesignEditorSelection => {
  return ({
    selectedConnections: selection.selectedConnections.filter((c: { connectingPieceId: string; connectedPieceId: string }) => {
      const existingConnectionId = selectionConnectionToConnectionId(c)
      return !connections.some(conn => isSameConnection(existingConnectionId, conn))
    }),
    selectedPieceIds: selection.selectedPieceIds
  })
}

const invertSelection = (selection: DesignEditorSelection, design: Design): DesignEditorSelection => {
  const allPieceIds = design.pieces?.map((p: Piece) => p.id_) || []
  const allConnections = design.connections?.map((c: Connection) => connectionToSelectionConnection(c)) || []
  const newSelectedPieceIds = allPieceIds.filter(id => !selection.selectedPieceIds.includes(id))
  const newSelectedConnections = allConnections.filter(conn => {
    const connectionId = selectionConnectionToConnectionId(conn)
    return !selection.selectedConnections.some(selected => {
      const selectedConnectionId = selectionConnectionToConnectionId(selected)
      return isSameConnection(selectedConnectionId, connectionId)
    })
  })
  return ({ selectedPieceIds: newSelectedPieceIds, selectedConnections: newSelectedConnections })
}

const invertPiecesSelection = (selection: DesignEditorSelection, design: Design): DesignEditorSelection => {
  const allPieceIds = design.pieces?.map((p: Piece) => p.id_) || []
  const newSelectedPieceIds = allPieceIds.filter(id => !selection.selectedPieceIds.includes(id))
  return ({ selectedPieceIds: newSelectedPieceIds, selectedConnections: selection.selectedConnections })
}
const invertConnectionsSelection = (selection: DesignEditorSelection, design: Design): DesignEditorSelection => {
  const allConnections = design.connections?.map((c: Connection) => connectionToSelectionConnection(c)) || []
  const newSelectedConnections = allConnections.filter(conn => {
    const connectionId = selectionConnectionToConnectionId(conn)
    return !selection.selectedConnections.some(selected => {
      const selectedConnectionId = selectionConnectionToConnectionId(selected)
      return isSameConnection(selectedConnectionId, connectionId)
    })
  })
  return ({ selectedPieceIds: selection.selectedPieceIds, selectedConnections: newSelectedConnections })
}

const subDesignFromSelection = (design: Design, selection: DesignEditorSelection): Design => {
  const subPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_))
  const subConnections = design.connections?.filter(c => selection.selectedConnections.some(sc => isSameConnection(selectionConnectionToConnectionId(sc), c)))
  return { ...design, pieces: subPieces, connections: subConnections }
}

const copyToClipboard = (design: Design, selection: DesignEditorSelection, plane?: Plane, center?: DiagramPoint): void => {
  navigator.clipboard.writeText(JSON.stringify(orientDesign(subDesignFromSelection(design, selection), plane, center))).then(() => { })
}
const cutToClipboard = (design: Design, selection: DesignEditorSelection, plane?: Plane, center?: DiagramPoint): void => {
  navigator.clipboard.writeText(JSON.stringify(subDesignFromSelection(orientDesign(design, plane, center), selection))).then(() => { })
}
const pasteFromClipboard = (design: Design, plane?: Plane, center?: DiagramPoint): Design => {
  // TODO: Implement paste from clipboard
  navigator.clipboard.readText().then(text => mergeDesigns([design, orientDesign(JSON.parse(text), plane, center)]))
  return design
}
const deleteSelected = (kit: Kit, designId: DesignId, selection: DesignEditorSelection): Design => {
  const selectedPieces = selection.selectedPieceIds.map(id => ({ id_: id }))
  const selectedConnections = selection.selectedConnections.map(conn => ({ connecting: { piece: { id_: conn.connectingPieceId } }, connected: { piece: { id_: conn.connectedPieceId } } }))
  const updatedDesign = removePiecesAndConnectionsFromDesign(kit, designId, selectedPieces, selectedConnections)
  return updatedDesign
}


//#endregion Selection

//#region Operation Stack

const pushToOperationStack = (state: DesignEditorState): DesignEditorState => {
  const currentDesign = findDesignInKit(state.kit, state.designId)
  const newEntry: OperationStackEntry = {
    design: JSON.parse(JSON.stringify(currentDesign)),
    selection: JSON.parse(JSON.stringify(state.selection))
  }

  let newOperationStack: OperationStackEntry[]
  let newOperationIndex: number

  if (state.operationIndex < state.operationStack.length - 1) {
    newOperationStack = state.operationStack.slice(0, state.operationIndex + 1)
  } else {
    newOperationStack = [...state.operationStack]
  }

  newOperationStack.push(newEntry)

  if (newOperationStack.length > COMMAND_STACK_MAX) {
    newOperationStack.shift()
    newOperationIndex = newOperationStack.length - 1
  } else {
    newOperationIndex = newOperationStack.length - 1
  }

  return {
    ...state,
    operationStack: newOperationStack,
    operationIndex: newOperationIndex
  }
}

const canUndo = (state: DesignEditorState): boolean => {
  if (state.operationStack.length === 0) return false
  return state.operationIndex > 0 && state.operationStack.length > 1
}

const canRedo = (state: DesignEditorState): boolean => {
  if (state.operationStack.length === 0) return false
  return state.operationIndex < state.operationStack.length - 1
}

const undo = (state: DesignEditorState): DesignEditorState => {
  if (!canUndo(state)) return state

  const previousEntry = state.operationStack[state.operationIndex - 1]
  const currentDesign = findDesignInKit(state.kit, state.designId)
  const updatedDesigns = (state.kit.designs || []).map((d: Design) =>
    isSameDesign(d, currentDesign)
      ? previousEntry.design
      : d
  )
  return {
    ...state,
    kit: { ...state.kit, designs: updatedDesigns },
    selection: previousEntry.selection,
    operationIndex: state.operationIndex - 1
  }
}

const redo = (state: DesignEditorState): DesignEditorState => {
  if (!canRedo(state)) return state

  const nextEntry = state.operationStack[state.operationIndex + 1]
  const currentDesign = findDesignInKit(state.kit, state.designId)
  const updatedDesigns = (state.kit.designs || []).map((d: Design) =>
    isSameDesign(d, currentDesign)
      ? nextEntry.design
      : d
  )
  return {
    ...state,
    kit: { ...state.kit, designs: updatedDesigns },
    selection: nextEntry.selection,
    operationIndex: state.operationIndex + 1
  }
}

//#endregion Operation Stack


export const DesignEditorContext = createContext<{ state: DesignEditorState; dispatch: DesignEditorDispatcher } | undefined>(undefined);

export const useDesignEditor = () => {
  const context = useContext(DesignEditorContext);
  if (!context) {
    throw new Error('useDesignEditor must be used within a DesignEditorProvider');
  }
  const { state, dispatch } = context;

  const setDesign = useCallback((d: Design) => dispatch({ type: DesignEditorAction.SetDesign, payload: d }), [dispatch]);
  const addPieceToDesign = useCallback((p: Piece) => dispatch({ type: DesignEditorAction.AddPieceToDesign, payload: p }), [dispatch]);
  const setPieceInDesign = useCallback((p: Piece) => dispatch({ type: DesignEditorAction.SetPieceInDesign, payload: p }), [dispatch]);
  const removePieceFromDesign = useCallback((p: Piece) => dispatch({ type: DesignEditorAction.RemovePieceFromDesign, payload: p }), [dispatch]);
  const addPiecesToDesign = useCallback((ps: Piece[]) => dispatch({ type: DesignEditorAction.AddPiecesToDesign, payload: ps }), [dispatch]);
  const setPiecesInDesign = useCallback((ps: Piece[]) => dispatch({ type: DesignEditorAction.SetPiecesInDesign, payload: ps }), [dispatch]);
  const removePiecesFromDesign = useCallback((ps: Piece[]) => dispatch({ type: DesignEditorAction.RemovePiecesFromDesign, payload: ps }), [dispatch]);
  const addConnectionToDesign = useCallback((c: Connection) => dispatch({ type: DesignEditorAction.AddConnectionToDesign, payload: c }), [dispatch]);
  const setConnectionInDesign = useCallback((c: Connection) => dispatch({ type: DesignEditorAction.SetConnectionInDesign, payload: c }), [dispatch]);
  const removeConnectionFromDesign = useCallback((c: Connection) => dispatch({ type: DesignEditorAction.RemoveConnectionFromDesign, payload: c }), [dispatch]);
  const addConnectionsToDesign = useCallback((cs: Connection[]) => dispatch({ type: DesignEditorAction.AddConnectionsToDesign, payload: cs }), [dispatch]);
  const setConnectionsInDesign = useCallback((cs: Connection[]) => dispatch({ type: DesignEditorAction.SetConnectionsInDesign, payload: cs }), [dispatch]);
  const removeConnectionsFromDesign = useCallback((cs: Connection[]) => dispatch({ type: DesignEditorAction.RemoveConnectionsFromDesign, payload: cs }), [dispatch]);
  const setSelection = useCallback((s: DesignEditorSelection) => dispatch({ type: DesignEditorAction.SetSelection, payload: s }), [dispatch]);
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
  const deleteSelected = useCallback((plane?: Plane, center?: DiagramPoint) => dispatch({ type: DesignEditorAction.DeleteSelected, payload: { plane, center } }), [dispatch]);
  const setFullscreen = useCallback((fp: FullscreenPanel) => dispatch({ type: DesignEditorAction.SetFullscreen, payload: fp }), [dispatch]);
  const toggleDiagramFullscreen = useCallback(() => dispatch({ type: DesignEditorAction.ToggleDiagramFullscreen, payload: null }), [dispatch]);
  const toggleModelFullscreen = useCallback(() => dispatch({ type: DesignEditorAction.ToggleModelFullscreen, payload: null }), [dispatch]);
  const copyToClipboard = useCallback(() => dispatch({ type: DesignEditorAction.CopyToClipboard, payload: null }), [dispatch]);
  const pasteFromClipboard = useCallback((plane?: Plane, center?: DiagramPoint) => dispatch({ type: DesignEditorAction.PasteFromClipboard, payload: { plane, center } }), [dispatch]);
  const undo = useCallback(() => dispatch({ type: DesignEditorAction.Undo, payload: null }), [dispatch]);
  const redo = useCallback(() => dispatch({ type: DesignEditorAction.Redo, payload: null }), [dispatch]);

  // Transaction management
  const startTransaction = useCallback(() => dispatch({ type: DesignEditorAction.StartTransaction, payload: null }), [dispatch]);
  const finalizeTransaction = useCallback(() => dispatch({ type: DesignEditorAction.FinalizeTransaction, payload: null }), [dispatch]);
  const abortTransaction = useCallback(() => dispatch({ type: DesignEditorAction.AbortTransaction, payload: null }), [dispatch]);

  // High-level piece/connection modification functions that handle transactions
  const updatePiece = useCallback((piece: Piece) => {
    dispatch({ type: DesignEditorAction.SetPieceInDesign, payload: piece });
  }, [dispatch]);

  const updateConnection = useCallback((connection: Connection) => {
    dispatch({ type: DesignEditorAction.SetConnectionInDesign, payload: connection });
  }, [dispatch]);

  const updatePieces = useCallback((pieces: Piece[]) => {
    dispatch({ type: DesignEditorAction.SetPiecesInDesign, payload: pieces });
  }, [dispatch]);

  const updateConnections = useCallback((connections: Connection[]) => {
    dispatch({ type: DesignEditorAction.SetConnectionsInDesign, payload: connections });
  }, [dispatch]);

  return {
    ...state,
    canUndo: canUndo(state),
    canRedo: canRedo(state),
    setDesign,
    addPieceToDesign,
    setPieceInDesign,
    removePieceFromDesign,
    addPiecesToDesign,
    setPiecesInDesign,
    removePiecesFromDesign,
    addConnectionToDesign,
    setConnectionInDesign,
    removeConnectionFromDesign,
    addConnectionsToDesign,
    setConnectionsInDesign,
    removeConnectionsFromDesign,
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
    deleteSelected,
    setFullscreen,
    toggleDiagramFullscreen,
    toggleModelFullscreen,
    undo,
    redo,
    // Transaction functions
    startTransaction,
    finalizeTransaction,
    abortTransaction,
    updatePiece,
    updateConnection,
    updatePieces,
    updateConnections
  };
};

const designEditorReducer = (
  state: DesignEditorState,
  action: { type: DesignEditorAction; payload: any }
): DesignEditorState => {
  const currentDesign = findDesignInKit(state.kit, state.designId)

  const updateDesignInDesignEditorStateWithOperationStack = (updatedDesign: Design): DesignEditorState => {
    const stateWithOperation = pushToOperationStack(state)
    const updatedDesigns = (stateWithOperation.kit.designs || []).map((d: Design) => isSameDesign(d, currentDesign) ? updatedDesign : d)
    return { ...stateWithOperation, kit: { ...stateWithOperation.kit, designs: updatedDesigns } }
  }

  const updateDesignInDesignEditorState = (updatedDesign: Design): DesignEditorState => {
    const updatedDesigns = (state.kit.designs || []).map((d: Design) => isSameDesign(d, currentDesign) ? updatedDesign : d)
    return { ...state, kit: { ...state.kit, designs: updatedDesigns } }
  }

  switch (action.type) {
    // Design changes that should push to operation stack (when not in transaction)
    case DesignEditorAction.SetDesign:
      if (state.isTransactionActive) {
        return updateDesignInDesignEditorState(action.payload)
      }
      return updateDesignInDesignEditorStateWithOperationStack(action.payload)
    case DesignEditorAction.AddPieceToDesign:
      const designWithAddedPiece = addPieceToDesign(currentDesign, action.payload)
      if (state.isTransactionActive) {
        return updateDesignInDesignEditorState(designWithAddedPiece)
      }
      return updateDesignInDesignEditorStateWithOperationStack(designWithAddedPiece)
    case DesignEditorAction.SetPieceInDesign:
      const designWithSetPiece = setPieceInDesign(currentDesign, action.payload)
      if (state.isTransactionActive) {
        return updateDesignInDesignEditorState(designWithSetPiece)
      }
      return updateDesignInDesignEditorStateWithOperationStack(designWithSetPiece)
    case DesignEditorAction.RemovePieceFromDesign:
      const designWithRemovedPiece = removePieceFromDesign(state.kit, state.designId, action.payload)
      if (state.isTransactionActive) {
        return updateDesignInDesignEditorState(designWithRemovedPiece)
      }
      return updateDesignInDesignEditorStateWithOperationStack(designWithRemovedPiece)
    case DesignEditorAction.AddPiecesToDesign:
      const designWithAddedPieces = addPiecesToDesign(currentDesign, action.payload)
      if (state.isTransactionActive) {
        return updateDesignInDesignEditorState(designWithAddedPieces)
      }
      return updateDesignInDesignEditorStateWithOperationStack(designWithAddedPieces)
    case DesignEditorAction.SetPiecesInDesign:
      const designWithSetPieces = setPiecesInDesign(currentDesign, action.payload)
      if (state.isTransactionActive) {
        return updateDesignInDesignEditorState(designWithSetPieces)
      }
      return updateDesignInDesignEditorStateWithOperationStack(designWithSetPieces)
    case DesignEditorAction.RemovePiecesFromDesign:
      const designWithRemovedPieces = removePiecesFromDesign(state.kit, state.designId, action.payload)
      if (state.isTransactionActive) {
        return updateDesignInDesignEditorState(designWithRemovedPieces)
      }
      return updateDesignInDesignEditorStateWithOperationStack(designWithRemovedPieces)
    case DesignEditorAction.AddConnectionToDesign:
      const designWithAddedConnection = addConnectionToDesign(currentDesign, action.payload)
      if (state.isTransactionActive) {
        return updateDesignInDesignEditorState(designWithAddedConnection)
      }
      return updateDesignInDesignEditorStateWithOperationStack(designWithAddedConnection)
    case DesignEditorAction.SetConnectionInDesign:
      const designWithSetConnection = setConnectionInDesign(currentDesign, action.payload)
      if (state.isTransactionActive) {
        return updateDesignInDesignEditorState(designWithSetConnection)
      }
      return updateDesignInDesignEditorStateWithOperationStack(designWithSetConnection)
    case DesignEditorAction.RemoveConnectionFromDesign:
      const designWithRemovedConnection = removeConnectionFromDesign(state.kit, state.designId, action.payload)
      if (state.isTransactionActive) {
        return updateDesignInDesignEditorState(designWithRemovedConnection)
      }
      return updateDesignInDesignEditorStateWithOperationStack(designWithRemovedConnection)
    case DesignEditorAction.AddConnectionsToDesign:
      const designWithAddedConnections = addConnectionsToDesign(currentDesign, action.payload)
      if (state.isTransactionActive) {
        return updateDesignInDesignEditorState(designWithAddedConnections)
      }
      return updateDesignInDesignEditorStateWithOperationStack(designWithAddedConnections)
    case DesignEditorAction.SetConnectionsInDesign:
      const designWithSetConnections = setConnectionsInDesign(currentDesign, action.payload)
      if (state.isTransactionActive) {
        return updateDesignInDesignEditorState(designWithSetConnections)
      }
      return updateDesignInDesignEditorStateWithOperationStack(designWithSetConnections)
    case DesignEditorAction.RemoveConnectionsFromDesign:
      const designWithRemovedConnections = removeConnectionsFromDesign(state.kit, state.designId, action.payload)
      if (state.isTransactionActive) {
        return updateDesignInDesignEditorState(designWithRemovedConnections)
      }
      return updateDesignInDesignEditorStateWithOperationStack(designWithRemovedConnections)
    case DesignEditorAction.RemovePiecesAndConnectionsFromDesign:
      const designWithRemovedPiecesAndConnections = removePiecesAndConnectionsFromDesign(state.kit, state.designId, action.payload.pieceIds, action.payload.connectionIds)
      if (state.isTransactionActive) {
        return updateDesignInDesignEditorState(designWithRemovedPiecesAndConnections)
      }
      return updateDesignInDesignEditorStateWithOperationStack(designWithRemovedPiecesAndConnections)
    case DesignEditorAction.DeleteSelected:
      const stateWithDeleteOperation = pushToOperationStack(state)
      const selectionToDelete = stateWithDeleteOperation.selection
      const updatedDesign = deleteSelected(stateWithDeleteOperation.kit, stateWithDeleteOperation.designId, selectionToDelete)
      const entryWithCorrectSelection = stateWithDeleteOperation.operationStack[stateWithDeleteOperation.operationIndex]
      entryWithCorrectSelection.selection = selectionToDelete
      return { ...stateWithDeleteOperation, kit: updateDesignInKit(stateWithDeleteOperation.kit, updatedDesign), selection: deselectAll(stateWithDeleteOperation.selection) }

    // Transaction actions
    case DesignEditorAction.StartTransaction:
      if (state.isTransactionActive) return state // Only one transaction at a time
      return { ...state, isTransactionActive: true }
    case DesignEditorAction.FinalizeTransaction:
      if (!state.isTransactionActive) return state
      // Push current state to operation stack
      const stateWithFinalizedTransaction = pushToOperationStack(state)
      const entryWithCorrectTransactionState = stateWithFinalizedTransaction.operationStack[stateWithFinalizedTransaction.operationIndex]
      entryWithCorrectTransactionState.selection = state.selection
      return { ...stateWithFinalizedTransaction, isTransactionActive: false }
    case DesignEditorAction.AbortTransaction:
      if (!state.isTransactionActive) return state
      // Restore design to the state before transaction started
      const previousEntry = state.operationStack[state.operationIndex]
      const restoredDesigns = (state.kit.designs || []).map((d: Design) =>
        isSameDesign(d, currentDesign) ? previousEntry.design : d
      )
      return { ...state, kit: { ...state.kit, designs: restoredDesigns }, isTransactionActive: false }

    // Selection changes (no operation stack)
    case DesignEditorAction.SetSelection:
      return { ...state, selection: action.payload }
    case DesignEditorAction.SelectAll:
      return { ...state, selection: selectAll(currentDesign) }
    case DesignEditorAction.DeselectAll:
      return { ...state, selection: deselectAll(state.selection) }
    case DesignEditorAction.InvertSelection:
      return { ...state, selection: invertSelection(state.selection, currentDesign) }
    case DesignEditorAction.InvertPiecesSelection:
      return { ...state, selection: invertPiecesSelection(state.selection, currentDesign) }
    case DesignEditorAction.InvertConnectionsSelection:
      return { ...state, selection: invertConnectionsSelection(state.selection, currentDesign) }
    case DesignEditorAction.AddAllPiecesToSelection:
      return { ...state, selection: addAllPiecesToSelection(state.selection, currentDesign) }
    case DesignEditorAction.RemoveAllPiecesFromSelection:
      return { ...state, selection: removeAllPiecesFromSelection(state.selection) }
    case DesignEditorAction.AddAllConnectionsToSelection:
      return { ...state, selection: addAllConnectionsToSelection(state.selection, currentDesign) }
    case DesignEditorAction.RemoveAllConnectionsFromSelection:
      return { ...state, selection: removeAllConnectionsFromSelection(state.selection) }
    case DesignEditorAction.SelectPiece:
      return { ...state, selection: selectPiece(action.payload) }
    case DesignEditorAction.AddPieceToSelection:
      return { ...state, selection: addPieceToSelection(state.selection, action.payload) }
    case DesignEditorAction.RemovePieceFromSelection:
      return { ...state, selection: removePieceFromSelection(state.selection, action.payload) }
    case DesignEditorAction.SelectPieces:
      return { ...state, selection: selectPieces(action.payload) }
    case DesignEditorAction.AddPiecesToSelection:
      return { ...state, selection: addPiecesToSelection(state.selection, action.payload) }
    case DesignEditorAction.RemovePiecesFromSelection:
      return { ...state, selection: removePiecesFromSelection(state.selection, action.payload) }
    case DesignEditorAction.SelectConnection:
      return { ...state, selection: selectConnection(action.payload) }
    case DesignEditorAction.AddConnectionToSelection:
      return { ...state, selection: addConnectionToSelection(state.selection, action.payload) }
    case DesignEditorAction.RemoveConnectionFromSelection:
      return { ...state, selection: removeConnectionFromSelection(state.selection, action.payload) }
    case DesignEditorAction.SelectConnections:
      return { ...state, selection: selectConnections(action.payload) }
    case DesignEditorAction.AddConnectionsToSelection:
      return { ...state, selection: addConnectionsToSelection(state.selection, action.payload) }
    case DesignEditorAction.RemoveConnectionsFromSelection:
      return { ...state, selection: removeConnectionsFromSelection(state.selection, action.payload) }

    // Undo/Redo
    case DesignEditorAction.Undo:
      return undo(state)
    case DesignEditorAction.Redo:
      return redo(state)

    // Other (no operation stack)
    case DesignEditorAction.SetFullscreen:
      return { ...state, fullscreenPanel: action.payload }
    case DesignEditorAction.ToggleDiagramFullscreen:
      return { ...state, fullscreenPanel: state.fullscreenPanel === FullscreenPanel.Diagram ? FullscreenPanel.None : FullscreenPanel.Diagram }
    case DesignEditorAction.ToggleModelFullscreen:
      return { ...state, fullscreenPanel: state.fullscreenPanel === FullscreenPanel.Model ? FullscreenPanel.None : FullscreenPanel.Model }
    default:
      return state
  }
}

function useControllableReducer(props: DesignEditorProps) {
  const {
    kit: controlledKit,
    selection: controlledSelection,
    initialKit,
    initialSelection,
    onDesignChange,
    onSelectionChange,
    onUndo,
    onRedo,
    designId,
    fileUrls
  } = props

  const isKitControlled = controlledKit !== undefined
  const isSelectionControlled = controlledSelection !== undefined

  const initialDesign = findDesignInKit(initialKit!, designId)
  const initialState: DesignEditorState = {
    kit: initialKit!,
    designId: designId,
    fileUrls: fileUrls,
    fullscreenPanel: FullscreenPanel.None,
    selection: initialSelection || { selectedPieceIds: [], selectedConnections: [] },
    designDiff: {
      pieces: { added: [], removed: [], updated: [] },
      connections: { added: [], removed: [], updated: [] }
    },
    operationStack: [{
      design: JSON.parse(JSON.stringify(initialDesign)),
      selection: JSON.parse(JSON.stringify(initialSelection || { selectedPieceIds: [], selectedConnections: [] }))
    }],
    operationIndex: 0,
    isTransactionActive: false
  }

  const [internalState, dispatch] = useReducer(designEditorReducer, initialState)

  const state: DesignEditorState = {
    ...internalState,
    kit: isKitControlled ? controlledKit : internalState.kit,
    selection: isSelectionControlled ? controlledSelection : internalState.selection,
    operationStack: isKitControlled || isSelectionControlled ? [] : internalState.operationStack,
    operationIndex: isKitControlled || isSelectionControlled ? -1 : internalState.operationIndex,
    isTransactionActive: internalState.isTransactionActive
  }

  const dispatchWrapper = useCallback((action: { type: DesignEditorAction; payload: any }) => {
    console.log('ACTION:', action.type, action.payload)
    console.log('OLDSTATE:', state)

    if (action.type === DesignEditorAction.Undo && onUndo) {
      onUndo()
      return
    }
    if (action.type === DesignEditorAction.Redo && onRedo) {
      onRedo()
      return
    }

    const newState = designEditorReducer(state, action)
    console.log('NEWSTATE:', newState)

    if (!isKitControlled || !isSelectionControlled) dispatch(action)
    if (isKitControlled && newState.kit !== state.kit) onDesignChange?.(findDesignInKit(newState.kit, designId))
    if (isSelectionControlled && newState.selection !== state.selection) onSelectionChange?.(newState.selection)
  }, [state, isKitControlled, isSelectionControlled, designId, onDesignChange, onSelectionChange, onUndo, onRedo])

  return [state, dispatchWrapper] as const
}

//#endregion State

//#region Panels

interface PanelToggles {
  workbench: boolean
  console: boolean
  details: boolean
  chat: boolean
}

interface PanelProps {
  visible: boolean
}

interface ResizablePanelProps extends PanelProps {
  onWidthChange?: (width: number) => void
  width: number
}

interface TypeAvatarProps {
  type: Type
  showHoverCard?: boolean
}

const TypeAvatar: FC<TypeAvatarProps> = ({ type, showHoverCard = false }) => {
  const { attributes, listeners, setNodeRef } = useDraggable({
    id: `type-${type.name}-${type.variant || ''}`
  })

  const displayVariant = type.variant || type.name
  const avatar = (
    <Avatar ref={setNodeRef} {...listeners} {...attributes} className="cursor-grab">
      {/* <AvatarImage src="https://github.com/semio-tech.png" /> */}
      <AvatarFallback>{displayVariant.substring(0, 2).toUpperCase()}</AvatarFallback>
    </Avatar>
  )

  if (!showHoverCard) {
    return avatar
  }

  return (
    <HoverCard>
      <HoverCardTrigger asChild>{avatar}</HoverCardTrigger>
      <HoverCardContent className="w-80">
        <div className="space-y-1">
          {type.variant ? (
            <>
              <h4 className="text-sm font-semibold">{type.variant}</h4>
              <p className="text-sm">{type.description || 'No description available.'}</p>
            </>
          ) : (
            <p className="text-sm">{type.description || 'No description available.'}</p>
          )}
        </div>
      </HoverCardContent>
    </HoverCard>
  )
}

interface DesignAvatarProps {
  design: Design
  showHoverCard?: boolean
}

const DesignAvatar: FC<DesignAvatarProps> = ({ design, showHoverCard = false }) => {
  const { attributes, listeners, setNodeRef } = useDraggable({
    id: `design-${design.name}-${design.variant || ''}-${design.view || ''}`
  })

  // Determine if this is the default variant and view
  const isDefault = (!design.variant || design.variant === design.name) && (!design.view || design.view === 'Default')

  const displayVariant = design.variant || design.name
  const avatar = (
    <Avatar ref={setNodeRef} {...listeners} {...attributes} className="cursor-grab">
      {/* <AvatarImage src="https://github.com/semio-tech.png" /> */}
      <AvatarFallback>{displayVariant.substring(0, 2).toUpperCase()}</AvatarFallback>
    </Avatar>
  )

  if (!showHoverCard) {
    return avatar
  }

  return (
    <HoverCard>
      <HoverCardTrigger asChild>{avatar}</HoverCardTrigger>
      <HoverCardContent className="w-80">
        <div className="space-y-1">
          {!isDefault && (
            <h4 className="text-sm font-semibold">
              {design.variant || design.name}
              {design.view && design.view !== 'Default' && ` (${design.view})`}
            </h4>
          )}
          <p className="text-sm">{design.description || 'No description available.'}</p>
        </div>
      </HoverCardContent>
    </HoverCard>
  )
}

interface WorkbenchProps extends ResizablePanelProps {
  kit: Kit
}

const Workbench: FC<WorkbenchProps> = ({ visible, onWidthChange, width, kit }) => {
  if (!visible) return null
  const [isResizeHovered, setIsResizeHovered] = useState(false)
  const [isResizing, setIsResizing] = useState(false)

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault()
    setIsResizing(true)

    const startX = e.clientX
    const startWidth = width

    const handleMouseMove = (e: MouseEvent) => {
      const newWidth = startWidth + (e.clientX - startX)
      if (newWidth >= 150 && newWidth <= 500) {
        onWidthChange?.(newWidth)
      }
    }

    const handleMouseUp = () => {
      setIsResizing(false)
      document.removeEventListener('mousemove', handleMouseMove)
      document.removeEventListener('mouseup', handleMouseUp)
    }

    document.addEventListener('mousemove', handleMouseMove)
    document.addEventListener('mouseup', handleMouseUp)
  }

  // const { kit } = useKit();
  if (!kit?.types || !kit?.designs) return null

  const typesByName = kit.types.reduce(
    (acc, type) => {
      acc[type.name] = acc[type.name] || []
      acc[type.name].push(type)
      return acc
    },
    {} as Record<string, Type[]>
  )

  const designsByName = kit.designs.reduce(
    (acc, design) => {
      const nameKey = design.name
      acc[nameKey] = acc[nameKey] || []
      acc[nameKey].push(design)
      return acc
    },
    {} as Record<string, Design[]>
  )

  return (
    <div
      className={`absolute top-4 left-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isResizing || isResizeHovered ? 'border-r-primary' : 'border-r'}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">
        <div className="p-1">
          <Tree>
            <TreeSection label="Types" defaultOpen={true}>
              {Object.entries(typesByName).map(([name, variants]) => (
                <TreeSection key={name} label={name} defaultOpen={false}>
                  <div
                    className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1"
                    style={{ paddingLeft: `${(1 + 1) * 1.25}rem` }}
                  >
                    {variants.map((type) => (
                      <TypeAvatar key={`${type.name}-${type.variant}`} type={type} showHoverCard={true} />
                    ))}
                  </div>
                </TreeSection>
              ))}
            </TreeSection>
            <TreeSection label="Designs" defaultOpen={true}>
              {Object.entries(designsByName).map(([name, designs]) => (
                <TreeSection key={name} label={name} defaultOpen={false}>
                  <div
                    className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1"
                    style={{ paddingLeft: `${(1 + 1) * 1.25}rem` }}
                  >
                    {designs.map((design) => (
                      <DesignAvatar
                        key={`${design.name}-${design.variant}-${design.view}`}
                        design={design}
                        showHoverCard={true}
                      />
                    ))}
                  </div>
                </TreeSection>
              ))}
            </TreeSection>
          </Tree>
        </div>
      </ScrollArea>
      <div
        className="absolute top-0 bottom-0 right-0 w-1 cursor-ew-resize"
        onMouseDown={handleMouseDown}
        onMouseEnter={() => setIsResizeHovered(true)}
        onMouseLeave={() => !isResizing && setIsResizeHovered(false)}
      />
    </div>
  )
}

interface ConsoleProps {
  visible: boolean
  leftPanelVisible: boolean
  rightPanelVisible: boolean
  leftPanelWidth?: number
  rightPanelWidth?: number
  height: number
  setHeight: (height: number) => void
}

const Console: FC<ConsoleProps> = ({
  visible,
  leftPanelVisible,
  rightPanelVisible,
  leftPanelWidth = 230,
  rightPanelWidth = 230,
  height,
  setHeight
}) => {
  if (!visible) return null

  const [isResizeHovered, setIsResizeHovered] = useState(false)
  const [isResizing, setIsResizing] = useState(false)
  const designEditor = useDesignEditor()

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault()
    setIsResizing(true)

    const startY = e.clientY
    const startHeight = height

    const handleMouseMove = (e: MouseEvent) => {
      const newHeight = startHeight - (e.clientY - startY)
      if (newHeight >= 100 && newHeight <= 600) {
        setHeight(newHeight)
      }
    }

    const handleMouseUp = () => {
      setIsResizing(false)
      document.removeEventListener('mousemove', handleMouseMove)
      document.removeEventListener('mouseup', handleMouseUp)
    }

    document.addEventListener('mousemove', handleMouseMove)
    document.addEventListener('mouseup', handleMouseUp)
  }

  return (
    <div
      className={`absolute z-30 bg-background-level-2 text-foreground border ${isResizing || isResizeHovered ? 'border-t-primary' : ''}`}
      style={{
        left: leftPanelVisible ? `calc(${leftPanelWidth}px + calc(var(--spacing) * 8))` : `calc(var(--spacing) * 4)`,
        right: rightPanelVisible ? `calc(${rightPanelWidth}px + calc(var(--spacing) * 8))` : `calc(var(--spacing) * 4)`,
        bottom: `calc(var(--spacing) * 4)`,
        height: `${height}px`
      }}
    >
      <div
        className={`absolute top-0 left-0 right-0 h-1 cursor-ns-resize`}
        onMouseDown={handleMouseDown}
        onMouseEnter={() => setIsResizeHovered(true)}
        onMouseLeave={() => !isResizing && setIsResizeHovered(false)}
      />
      <Cli designEditor={designEditor} />
    </div>
  )
}

interface DetailsProps extends ResizablePanelProps { }

interface SortableTreeSectionProps {
  id: string
  index: number
  totalItems: number
  onMoveUp: () => void
  onMoveDown: () => void
  onRemove: () => void
  children: ReactNode
  label: string
}

const SortableTreeSection: FC<SortableTreeSectionProps> = ({ 
  id, 
  index,
  totalItems,
  onMoveUp,
  onMoveDown,
  onRemove,
  children, 
  label 
}) => {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id })

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  }

  const actions = [
    {
      icon: <div {...attributes} {...listeners} className="cursor-grab hover:text-primary"><GripVertical size={12} /></div>,
      onClick: () => {},
      title: "Drag to reorder"
    },
    {
      icon: <ChevronUp size={12} className={index === 0 ? 'opacity-30' : ''} />,
      onClick: index === 0 ? () => {} : onMoveUp,
      title: index === 0 ? "Already at top" : "Move up"
    },
    {
      icon: <ChevronDown size={12} className={index === totalItems - 1 ? 'opacity-30' : ''} />,
      onClick: index === totalItems - 1 ? () => {} : onMoveDown,
      title: index === totalItems - 1 ? "Already at bottom" : "Move down"
    },
    {
      icon: <Trash2 size={12} />,
      onClick: onRemove,
      title: "Remove"
    }
  ]

  return (
    <div ref={setNodeRef} style={style}>
      <TreeSection
        label={label}
        actions={actions}
      >
        {children}
      </TreeSection>
    </div>
  )
}

const DesignSection: FC = () => {
  const { kit, designId, setDesign, startTransaction, finalizeTransaction, abortTransaction } = useDesignEditor()
  const design = findDesignInKit(kit, designId)

  const handleChange = (updatedDesign: Design) => {
    setDesign(updatedDesign)
  }

  const addLocation = () => {
    handleChange({ ...design, location: { longitude: 0, latitude: 0 } })
  }

  const removeLocation = () => {
    handleChange({ ...design, location: undefined })
  }

  return (
    <>
      <TreeSection label="Design" defaultOpen={true}>
        <TreeItem>
          <Input
            label="Name"
            value={design.name}
            onChange={(e) => handleChange({ ...design, name: e.target.value })}
          />
        </TreeItem>
        <TreeItem>
          <Textarea
            label="Description"
            value={design.description || ''}
            placeholder="Enter design description..."
            onChange={(e) => handleChange({ ...design, description: e.target.value })}
          />
        </TreeItem>
        <TreeItem>
          <Input
            label="Icon"
            value={design.icon || ''}
            placeholder="Emoji, name, or URL"
            onChange={(e) => handleChange({ ...design, icon: e.target.value })}
          />
        </TreeItem>
        <TreeItem>
          <Input
            label="Image URL"
            value={design.image || ''}
            placeholder="URL to design image"
            onChange={(e) => handleChange({ ...design, image: e.target.value })}
          />
        </TreeItem>
        <TreeItem>
          <Input
            label="Variant"
            value={design.variant || ''}
            placeholder="Design variant"
            onChange={(e) => handleChange({ ...design, variant: e.target.value })}
          />
        </TreeItem>
        <TreeItem>
          <Input
            label="View"
            value={design.view || ''}
            placeholder="Design view"
            onChange={(e) => handleChange({ ...design, view: e.target.value })}
          />
        </TreeItem>
        <TreeItem>
          <Input
            label="Unit"
            value={design.unit}
            onChange={(e) => handleChange({ ...design, unit: e.target.value })}
          />
        </TreeItem>
      </TreeSection>
      {design.location ? (
        <TreeSection
          label="Location"

          actions={[
            {
              icon: <Minus size={12} />,
              onClick: removeLocation,
              title: "Remove location"
            }
          ]}
        >
          <TreeItem>
            <Stepper
              label="Longitude"
              value={design.location.longitude}
              onChange={(value) => handleChange({
                ...design,
                location: { ...design.location!, longitude: value }
              })}
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              step={0.000001}
            />
          </TreeItem>
          <TreeItem>
            <Stepper
              label="Latitude"
              value={design.location.latitude}
              onChange={(value) => handleChange({
                ...design,
                location: { ...design.location!, latitude: value }
              })}
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              step={0.000001}
            />
          </TreeItem>
        </TreeSection>
      ) : (
        <TreeSection
          label="Location"

          actions={[
            {
              icon: <Plus size={12} />,
              onClick: addLocation,
              title: "Add location"
            }
          ]}
        >
        </TreeSection>
      )}
      {design.authors && design.authors.length > 0 ? (
        <DndContext
          collisionDetection={closestCenter}
          onDragEnd={(event) => {
            const { active, over } = event
            if (over && active.id !== over.id) {
              const oldIndex = design.authors!.findIndex((_, i) => `author-${i}` === active.id)
              const newIndex = design.authors!.findIndex((_, i) => `author-${i}` === over.id)
              handleChange({
                ...design,
                authors: arrayMove(design.authors!, oldIndex, newIndex)
              })
            }
          }}
        >
          <TreeSection
            label="Authors"
            actions={[
              {
                icon: <Plus size={12} />,
                onClick: () => handleChange({
                  ...design,
                  authors: [...(design.authors || []), { name: '', email: '' }]
                }),
                title: "Add author"
              }
            ]}
          >
            <SortableContext items={design.authors.map((_, i) => `author-${i}`)} strategy={verticalListSortingStrategy}>
              {design.authors.map((author, index) => (
                <SortableTreeSection
                  key={`author-${index}`}
                  id={`author-${index}`}
                  index={index}
                  totalItems={design.authors!.length}
                  label={author.name || `Author ${index + 1}`}
                  onMoveUp={() => {
                    if (index > 0) {
                      handleChange({
                        ...design,
                        authors: arrayMove(design.authors!, index, index - 1)
                      })
                    }
                  }}
                  onMoveDown={() => {
                    if (index < design.authors!.length - 1) {
                      handleChange({
                        ...design,
                        authors: arrayMove(design.authors!, index, index + 1)
                      })
                    }
                  }}
                  onRemove={() => handleChange({
                    ...design,
                    authors: design.authors?.filter((_, i) => i !== index)
                  })}
                >
                  <TreeItem>
                    <Input
                      label="Name"
                      value={author.name}
                      onChange={(e) => {
                        const updatedAuthors = [...(design.authors || [])]
                        updatedAuthors[index] = { ...author, name: e.target.value }
                        handleChange({ ...design, authors: updatedAuthors })
                      }}
                    />
                  </TreeItem>
                  <TreeItem>
                    <Input
                      label="Email"
                      value={author.email}
                      onChange={(e) => {
                        const updatedAuthors = [...(design.authors || [])]
                        updatedAuthors[index] = { ...author, email: e.target.value }
                        handleChange({ ...design, authors: updatedAuthors })
                      }}
                    />
                  </TreeItem>
                </SortableTreeSection>
              ))}
            </SortableContext>
          </TreeSection>
        </DndContext>
      ) : (
        <TreeSection
          label="Authors"
          actions={[
            {
              icon: <Plus size={12} />,
              onClick: () => handleChange({
                ...design,
                authors: [...(design.authors || []), { name: '', email: '' }]
              }),
              title: "Add author"
            }
          ]}
        >
        </TreeSection>
      )}
      {design.qualities && design.qualities.length > 0 ? (
        <DndContext
          collisionDetection={closestCenter}
          onDragEnd={(event) => {
            const { active, over } = event
            if (over && active.id !== over.id) {
              const oldIndex = design.qualities!.findIndex((_, i) => `quality-${i}` === active.id)
              const newIndex = design.qualities!.findIndex((_, i) => `quality-${i}` === over.id)
              handleChange({
                ...design,
                qualities: arrayMove(design.qualities!, oldIndex, newIndex)
              })
            }
          }}
        >
          <TreeSection
            label="Qualities"
            actions={[
              {
                icon: <Plus size={12} />,
                onClick: () => handleChange({
                  ...design,
                  qualities: [...(design.qualities || []), { name: '' }]
                }),
                title: "Add quality"
              }
            ]}
          >
            <SortableContext items={design.qualities.map((_, i) => `quality-${i}`)} strategy={verticalListSortingStrategy}>
              {design.qualities.map((quality, index) => (
                <SortableTreeSection
                  key={`quality-${index}`}
                  id={`quality-${index}`}
                  index={index}
                  totalItems={design.qualities!.length}
                  label={quality.name || `Quality ${index + 1}`}
                  onMoveUp={() => {
                    if (index > 0) {
                      handleChange({
                        ...design,
                        qualities: arrayMove(design.qualities!, index, index - 1)
                      })
                    }
                  }}
                  onMoveDown={() => {
                    if (index < design.qualities!.length - 1) {
                      handleChange({
                        ...design,
                        qualities: arrayMove(design.qualities!, index, index + 1)
                      })
                    }
                  }}
                  onRemove={() => handleChange({
                    ...design,
                    qualities: design.qualities?.filter((_, i) => i !== index)
                  })}
                >
                  <TreeItem>
                    <Input
                      label="Name"
                      value={quality.name}
                      onChange={(e) => {
                        const updatedQualities = [...(design.qualities || [])]
                        updatedQualities[index] = { ...quality, name: e.target.value }
                        handleChange({ ...design, qualities: updatedQualities })
                      }}
                    />
                  </TreeItem>
                  <TreeItem>
                    <Input
                      label="Value"
                      value={quality.value || ''}
                      placeholder="Optional value"
                      onChange={(e) => {
                        const updatedQualities = [...(design.qualities || [])]
                        updatedQualities[index] = { ...quality, value: e.target.value }
                        handleChange({ ...design, qualities: updatedQualities })
                      }}
                    />
                  </TreeItem>
                  <TreeItem>
                    <Input
                      label="Unit"
                      value={quality.unit || ''}
                      placeholder="Optional unit"
                      onChange={(e) => {
                        const updatedQualities = [...(design.qualities || [])]
                        updatedQualities[index] = { ...quality, unit: e.target.value }
                        handleChange({ ...design, qualities: updatedQualities })
                      }}
                    />
                  </TreeItem>
                  <TreeItem>
                    <Input
                      label="Definition"
                      value={quality.definition || ''}
                      placeholder="Optional definition (text or URL)"
                      onChange={(e) => {
                        const updatedQualities = [...(design.qualities || [])]
                        updatedQualities[index] = { ...quality, definition: e.target.value }
                        handleChange({ ...design, qualities: updatedQualities })
                      }}
                    />
                  </TreeItem>
                </SortableTreeSection>
              ))}
            </SortableContext>
          </TreeSection>
        </DndContext>
      ) : (
        <TreeSection
          label="Qualities"
          actions={[
            {
              icon: <Plus size={12} />,
              onClick: () => handleChange({
                ...design,
                qualities: [...(design.qualities || []), { name: '' }]
              }),
              title: "Add quality"
            }
          ]}
        >
        </TreeSection>
      )}
      <TreeSection label="Metadata" >
        {design.created && (
          <TreeItem>
            <Input label="Created" value={design.created.toISOString().split('T')[0]} disabled />
          </TreeItem>
        )}
        {design.updated && (
          <TreeItem>
            <Input label="Updated" value={design.updated.toISOString().split('T')[0]} disabled />
          </TreeItem>
        )}
        {design.pieces && design.pieces.length > 0 && (
          <TreeItem>
            <Input label="Pieces" value={`${design.pieces.length} pieces`} disabled />
          </TreeItem>
        )}
        {design.connections && design.connections.length > 0 && (
          <TreeItem>
            <Input label="Connections" value={`${design.connections.length} connections`} disabled />
          </TreeItem>
        )}
      </TreeSection>
    </>
  )
}

const PiecesSection: FC<{ pieceIds: string[] }> = ({ pieceIds }) => {
  const { kit, designId, updatePiece, updatePieces, updateConnection, setDesign, removeConnectionFromDesign, startTransaction, finalizeTransaction, abortTransaction } = useDesignEditor()
  const design = findDesignInKit(kit, designId)
  const pieces = pieceIds.map(id => findPieceInDesign(design, id))
  const metadata = piecesMetadata(kit, designId)

  const isSingle = pieceIds.length === 1
  const piece = isSingle ? pieces[0] : null

  const getCommonValue = <T,>(getter: (piece: Piece) => T | undefined): T | undefined => {
    const values = pieces.map(getter).filter(v => v !== undefined)
    if (values.length === 0) return undefined
    const firstValue = values[0]
    return values.every(v => JSON.stringify(v) === JSON.stringify(firstValue)) ? firstValue : undefined
  }

  const handleTypeNameChange = (value: string) => {
    if (isSingle) {
      updatePiece({ ...piece!, type: { ...piece!.type, name: value } })
    } else {
      const updatedPieces = pieces.map(piece => ({ ...piece, type: { ...piece.type, name: value } }))
      updatePieces(updatedPieces)
    }
  }

  const handleTypeVariantChange = (value: string) => {
    if (isSingle) {
      updatePiece({ ...piece!, type: { ...piece!.type, variant: value } })
    } else {
      const updatedPieces = pieces.map(piece => ({ ...piece, type: { ...piece.type, variant: value } }))
      updatePieces(updatedPieces)
    }
  }

  const fixPieces = () => fixPiecesInDesign(kit, designId, pieceIds)

  const handleCenterXChange = (value: number) => {
    if (isSingle) {
      updatePiece(piece!.center ? { ...piece!, center: { ...piece!.center, x: value } } : piece!)
    } else {
      const updatedPieces = pieces.map(piece =>
        piece.center ? { ...piece, center: { ...piece.center, x: value } } : piece
      )
      updatePieces(updatedPieces)
    }
  }

  const handleCenterYChange = (value: number) => {
    if (isSingle) {
      updatePiece(piece!.center ? { ...piece!, center: { ...piece!.center, y: value } } : piece!)
    } else {
      const updatedPieces = pieces.map(piece =>
        piece.center ? { ...piece, center: { ...piece.center, y: value } } : piece
      )
      updatePieces(updatedPieces)
    }
  }

  const handlePlaneOriginXChange = (value: number) => {
    if (isSingle) {
      updatePiece(piece!.plane ? { ...piece!, plane: { ...piece!.plane, origin: { ...piece!.plane.origin, x: value } } } : piece!)
    } else {
      const updatedPieces = pieces.map(piece =>
        piece.plane ? { ...piece, plane: { ...piece.plane, origin: { ...piece.plane.origin, x: value } } } : piece
      )
      updatePieces(updatedPieces)
    }
  }

  const handlePlaneOriginYChange = (value: number) => {
    if (isSingle) {
      updatePiece(piece!.plane ? { ...piece!, plane: { ...piece!.plane, origin: { ...piece!.plane.origin, y: value } } } : piece!)
    } else {
      const updatedPieces = pieces.map(piece =>
        piece.plane ? { ...piece, plane: { ...piece.plane, origin: { ...piece.plane.origin, y: value } } } : piece
      )
      updatePieces(updatedPieces)
    }
  }

  const handlePlaneOriginZChange = (value: number) => {
    if (isSingle) {
      updatePiece(piece!.plane ? { ...piece!, plane: { ...piece!.plane, origin: { ...piece!.plane.origin, z: value } } } : piece!)
    } else {
      const updatedPieces = pieces.map(piece =>
        piece.plane ? { ...piece, plane: { ...piece.plane, origin: { ...piece.plane.origin, z: value } } } : piece
      )
      updatePieces(updatedPieces)
    }
  }

  const handleConnectionChange = (updatedConnection: Connection) => {
    updateConnection(updatedConnection)
  }

  const handleMultipleConnectionsChange = (propertyName: keyof Connection, value: any) => {
    const updatedConnections = parentConnections.map(conn => ({
      ...conn,
      [propertyName]: value
    }))
    updatedConnections.forEach(conn => updateConnection(conn))
  }

  const getCommonConnectionValue = <T,>(getter: (connection: Connection) => T | undefined): T | undefined => {
    if (parentConnections.length === 0) return undefined
    const values = parentConnections.map(getter).filter(v => v !== undefined)
    if (values.length === 0) return undefined
    const firstValue = values[0]
    return values.every(v => JSON.stringify(v) === JSON.stringify(firstValue)) ? firstValue : undefined
  }

  const commonTypeName = getCommonValue(p => p.type.name)
  const commonTypeVariant = getCommonValue(p => p.type.variant)
  const commonCenterX = getCommonValue(p => p.center?.x)
  const commonCenterY = getCommonValue(p => p.center?.y)
  const commonPlaneOriginX = getCommonValue(p => p.plane?.origin.x)
  const commonPlaneOriginY = getCommonValue(p => p.plane?.origin.y)
  const commonPlaneOriginZ = getCommonValue(p => p.plane?.origin.z)

  const hasCenter = pieces.some(p => p.center)
  const hasPlane = pieces.some(p => p.plane)
  const hasVariant = pieces.some(p => p.type.variant)
  const hasUnfixedPieces = pieces.some(p => !p.plane || !p.center)

  const selectedVariants = [...new Set(pieces.map(p => p.type.variant).filter((v): v is string => Boolean(v)))]
  const availableTypes = isSingle
    ? findReplacableTypesForPieceInDesign(kit, designId, pieceIds[0], selectedVariants)
    : findReplacableTypesForPiecesInDesign(kit, designId, pieceIds, selectedVariants)
  const availableTypeNames = [...new Set(availableTypes.map(t => t.name))]
  const availableVariants = commonTypeName
    ? [...new Set((isSingle
      ? findReplacableTypesForPieceInDesign(kit, designId, pieceIds[0])
      : findReplacableTypesForPiecesInDesign(kit, designId, pieceIds)
    ).filter(t => t.name === commonTypeName).map(t => t.variant).filter((v): v is string => Boolean(v)))]
    : []

  let parentConnection: Connection | null = null
  let parentConnections: Connection[] = []

  if (isSingle && piece) {
    const pieceMetadata = metadata.get(piece.id_)
    if (pieceMetadata?.parentPieceId) {
      try {
        parentConnection = findConnectionInDesign(design, {
          connected: { piece: { id_: piece.id_ } },
          connecting: { piece: { id_: pieceMetadata.parentPieceId } }
        })
      } catch { }
    }
  } else if (!isSingle) {
    // For multiple pieces, find all their parent connections
    parentConnections = pieces.map(piece => {
      const pieceMetadata = metadata.get(piece.id_)
      if (pieceMetadata?.parentPieceId) {
        try {
          return findConnectionInDesign(design, {
            connected: { piece: { id_: piece.id_ } },
            connecting: { piece: { id_: pieceMetadata.parentPieceId } }
          })
        } catch {
          return null
        }
      }
      return null
    }).filter((conn): conn is Connection => conn !== null)
  }

  const isFixed = isSingle ? (piece!.plane && piece!.center) : false

  return (
    <>
      <TreeSection
        label={isSingle ? "Piece" : `Multiple Pieces (${pieceIds.length})`}
        defaultOpen={true}
        actions={hasUnfixedPieces ? [
          {
            icon: <Pin size={12} />,
            onClick: fixPieces,
            title: isSingle ? "Fix piece" : "Fix pieces"
          }
        ] : undefined}
      >
        {isSingle && (
          <TreeItem>
            <Input label="ID" value={piece!.id_} disabled />
          </TreeItem>
        )}
        <TreeItem>
          <Combobox
            label="Type Name"
            options={availableTypeNames.map(name => ({ value: name, label: name }))}
            value={isSingle ? piece!.type.name : (commonTypeName || '')}
            placeholder={!isSingle && commonTypeName === undefined ? 'Mixed values' : 'Select type name'}
            onValueChange={handleTypeNameChange}
          />
        </TreeItem>
        {(hasVariant || availableVariants.length > 0) && (
          <TreeItem>
            <Combobox
              label="Type Variant"
              options={availableVariants.map(variant => ({ value: variant, label: variant }))}
              value={isSingle ? (piece!.type.variant || '') : (commonTypeVariant || '')}
              placeholder={!isSingle && commonTypeVariant === undefined ? 'Mixed values' : 'Select variant'}
              onValueChange={handleTypeVariantChange}
              allowClear={true}
            />
          </TreeItem>
        )}
      </TreeSection>
      {hasCenter && (
        <TreeSection label="Center">
          <TreeItem>
            <Stepper
              label="X"
              value={isSingle ? piece!.center?.x : commonCenterX}
              onChange={handleCenterXChange}
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              step={0.1}
            />
          </TreeItem>
          <TreeItem>
            <Stepper
              label="Y"
              value={isSingle ? piece!.center?.y : commonCenterY}
              onChange={handleCenterYChange}
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              step={0.1}
            />
          </TreeItem>
        </TreeSection>
      )}
      {hasPlane && (
        <TreeSection label="Plane">
          <TreeSection label="Origin" defaultOpen={true}>
            <TreeItem>
              <Stepper
                label="X"
                value={isSingle ? piece!.plane?.origin.x : commonPlaneOriginX}
                onChange={handlePlaneOriginXChange}
                onPointerDown={startTransaction}
                onPointerUp={finalizeTransaction}
                onPointerCancel={abortTransaction}
                step={0.1}
              />
            </TreeItem>
            <TreeItem>
              <Stepper
                label="Y"
                value={isSingle ? piece!.plane?.origin.y : commonPlaneOriginY}
                onChange={handlePlaneOriginYChange}
                onPointerDown={startTransaction}
                onPointerUp={finalizeTransaction}
                onPointerCancel={abortTransaction}
                step={0.1}
              />
            </TreeItem>
            <TreeItem>
              <Stepper
                label="Z"
                value={isSingle ? piece!.plane?.origin.z : commonPlaneOriginZ}
                onChange={handlePlaneOriginZChange}
                onPointerDown={startTransaction}
                onPointerUp={finalizeTransaction}
                onPointerCancel={abortTransaction}
                step={0.1}
              />
            </TreeItem>
          </TreeSection>
        </TreeSection>
      )}
      {(parentConnection || parentConnections.length > 0) && (
        <TreeSection
          label={isSingle ? "Parent Connection" : `Parent Connections (${parentConnections.length})`}
          defaultOpen={true}
        >
          <TreeItem>
            <Stepper
              label="Gap"
              value={isSingle ? (parentConnection?.gap ?? 0) : (getCommonConnectionValue(c => c.gap) ?? 0)}
              onChange={(value) => {
                if (isSingle && parentConnection) {
                  handleConnectionChange({ ...parentConnection, gap: value })
                } else {
                  handleMultipleConnectionsChange('gap', value)
                }
              }}
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              step={0.1}
            />
          </TreeItem>
          <TreeItem>
            <Stepper
              label="Shift"
              value={isSingle ? (parentConnection?.shift ?? 0) : (getCommonConnectionValue(c => c.shift) ?? 0)}
              onChange={(value) => {
                if (isSingle && parentConnection) {
                  handleConnectionChange({ ...parentConnection, shift: value })
                } else {
                  handleMultipleConnectionsChange('shift', value)
                }
              }}
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              step={0.1}
            />
          </TreeItem>
          <TreeItem>
            <Stepper
              label="Rise"
              value={isSingle ? (parentConnection?.rise ?? 0) : (getCommonConnectionValue(c => c.rise) ?? 0)}
              onChange={(value) => {
                if (isSingle && parentConnection) {
                  handleConnectionChange({ ...parentConnection, rise: value })
                } else {
                  handleMultipleConnectionsChange('rise', value)
                }
              }}
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              step={0.1}
            />
          </TreeItem>
          <TreeItem>
            <Stepper
              label="X Offset"
              value={isSingle ? (parentConnection?.x ?? 0) : (getCommonConnectionValue(c => c.x) ?? 0)}
              onChange={(value) => {
                if (isSingle && parentConnection) {
                  handleConnectionChange({ ...parentConnection, x: value })
                } else {
                  handleMultipleConnectionsChange('x', value)
                }
              }}
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              step={0.1}
            />
          </TreeItem>
          <TreeItem>
            <Stepper
              label="Y Offset"
              value={isSingle ? (parentConnection?.y ?? 0) : (getCommonConnectionValue(c => c.y) ?? 0)}
              onChange={(value) => {
                if (isSingle && parentConnection) {
                  handleConnectionChange({ ...parentConnection, y: value })
                } else {
                  handleMultipleConnectionsChange('y', value)
                }
              }}
              onPointerDown={startTransaction}
              onPointerUp={finalizeTransaction}
              onPointerCancel={abortTransaction}
              step={0.1}
            />
          </TreeItem>
        </TreeSection>
      )}
    </>
  )
}

const ConnectionsSection: FC<{ connections: { connectingPieceId: string; connectedPieceId: string }[] }> = ({ connections }) => {
  const { kit, designId, updateConnection, updateConnections, startTransaction, finalizeTransaction, abortTransaction } = useDesignEditor()
  const design = findDesignInKit(kit, designId)
  const connectionObjects = connections.map(conn => {
    const connectionId = {
      connecting: { piece: { id_: conn.connectingPieceId } },
      connected: { piece: { id_: conn.connectedPieceId } }
    }
    return findConnectionInDesign(design, connectionId)
  })

  const isSingle = connections.length === 1
  const connection = isSingle ? connectionObjects[0] : null

  const getCommonValue = <T,>(getter: (connection: Connection) => T | undefined): T | undefined => {
    const values = connectionObjects.map(getter).filter(v => v !== undefined)
    if (values.length === 0) return undefined
    const firstValue = values[0]
    return values.every(v => JSON.stringify(v) === JSON.stringify(firstValue)) ? firstValue : undefined
  }

  const handleChange = (updatedConnection: Connection) => {
    updateConnection(updatedConnection)
  }

  const handleGapChange = (value: number) => {
    if (isSingle) {
      handleChange({ ...connection!, gap: value })
    } else {
      const updatedConnections = connectionObjects.map(connection => ({ ...connection, gap: value }))
      updateConnections(updatedConnections)
    }
  }

  const handleShiftChange = (value: number) => {
    if (isSingle) {
      handleChange({ ...connection!, shift: value })
    } else {
      const updatedConnections = connectionObjects.map(connection => ({ ...connection, shift: value }))
      updateConnections(updatedConnections)
    }
  }

  const handleRiseChange = (value: number) => {
    if (isSingle) {
      handleChange({ ...connection!, rise: value })
    } else {
      const updatedConnections = connectionObjects.map(connection => ({ ...connection, rise: value }))
      updateConnections(updatedConnections)
    }
  }

  const handleXOffsetChange = (value: number) => {
    if (isSingle) {
      handleChange({ ...connection!, x: value })
    } else {
      const updatedConnections = connectionObjects.map(connection => ({ ...connection, x: value }))
      updateConnections(updatedConnections)
    }
  }

  const handleYOffsetChange = (value: number) => {
    if (isSingle) {
      handleChange({ ...connection!, y: value })
    } else {
      const updatedConnections = connectionObjects.map(connection => ({ ...connection, y: value }))
      updateConnections(updatedConnections)
    }
  }

  const handleRotationChange = (value: number) => {
    if (isSingle) {
      handleChange({ ...connection!, rotation: value })
    } else {
      const updatedConnections = connectionObjects.map(connection => ({ ...connection, rotation: value }))
      updateConnections(updatedConnections)
    }
  }

  const handleTurnChange = (value: number) => {
    if (isSingle) {
      handleChange({ ...connection!, turn: value })
    } else {
      const updatedConnections = connectionObjects.map(connection => ({ ...connection, turn: value }))
      updateConnections(updatedConnections)
    }
  }

  const handleTiltChange = (value: number) => {
    if (isSingle) {
      handleChange({ ...connection!, tilt: value })
    } else {
      const updatedConnections = connectionObjects.map(connection => ({ ...connection, tilt: value }))
      updateConnections(updatedConnections)
    }
  }

  const commonGap = getCommonValue(c => c.gap)
  const commonShift = getCommonValue(c => c.shift)
  const commonRise = getCommonValue(c => c.rise)
  const commonXOffset = getCommonValue(c => c.x)
  const commonYOffset = getCommonValue(c => c.y)
  const commonRotation = getCommonValue(c => c.rotation)
  const commonTurn = getCommonValue(c => c.turn)
  const commonTilt = getCommonValue(c => c.tilt)

  return (
    <>
      <TreeSection label={isSingle ? "Connection" : `Multiple Connections (${connections.length})`} defaultOpen={true}>
        {isSingle && (
          <>
            <TreeItem>
              <Input label="Connecting Piece ID" value={connection!.connecting.piece.id_} disabled />
            </TreeItem>
            <TreeItem>
              <Input
                label="Connecting Port ID"
                value={connection!.connecting.port.id_}
                onChange={(e) =>
                  handleChange({ ...connection!, connecting: { ...connection!.connecting, port: { id_: e.target.value } } })
                }
              />
            </TreeItem>
            <TreeItem>
              <Input label="Connected Piece ID" value={connection!.connected.piece.id_} disabled />
            </TreeItem>
            <TreeItem>
              <Input
                label="Connected Port ID"
                value={connection!.connected.port.id_}
                onChange={(e) =>
                  handleChange({ ...connection!, connected: { ...connection!.connected, port: { id_: e.target.value } } })
                }
              />
            </TreeItem>
          </>
        )}
        {!isSingle && (
          <TreeItem>
            <p className="text-sm text-muted-foreground">Editing {connections.length} connections simultaneously</p>
          </TreeItem>
        )}
      </TreeSection>
      <TreeSection label="Translation" defaultOpen={true}>
        <TreeItem>
          <Stepper
            label="Gap"
            value={isSingle ? (connection!.gap ?? 0) : (commonGap ?? 0)}
            onChange={handleGapChange}
            onPointerDown={startTransaction}
            onPointerUp={finalizeTransaction}
            onPointerCancel={abortTransaction}
            step={0.1}
          />
        </TreeItem>
        <TreeItem>
          <Stepper
            label="Shift"
            value={isSingle ? (connection!.shift ?? 0) : (commonShift ?? 0)}
            onChange={handleShiftChange}
            onPointerDown={startTransaction}
            onPointerUp={finalizeTransaction}
            onPointerCancel={abortTransaction}
            step={0.1}
          />
        </TreeItem>
        <TreeItem>
          <Stepper
            label="Rise"
            value={isSingle ? (connection!.rise ?? 0) : (commonRise ?? 0)}
            onChange={handleRiseChange}
            onPointerDown={startTransaction}
            onPointerUp={finalizeTransaction}
            onPointerCancel={abortTransaction}
            step={0.1}
          />
        </TreeItem>
        <TreeItem>
          <Stepper
            label="X Offset"
            value={isSingle ? (connection!.x ?? 0) : (commonXOffset ?? 0)}
            onChange={handleXOffsetChange}
            onPointerDown={startTransaction}
            onPointerUp={finalizeTransaction}
            onPointerCancel={abortTransaction}
            step={0.1}
          />
        </TreeItem>
        <TreeItem>
          <Stepper
            label="Y Offset"
            value={isSingle ? (connection!.y ?? 0) : (commonYOffset ?? 0)}
            onChange={handleYOffsetChange}
            onPointerDown={startTransaction}
            onPointerUp={finalizeTransaction}
            onPointerCancel={abortTransaction}
            step={0.1}
          />
        </TreeItem>
      </TreeSection>
      <TreeSection label="Rotation">
        <TreeItem>
          <Slider
            label="Rotation"
            value={[isSingle ? (connection!.rotation ?? 0) : (commonRotation ?? 0)]}
            onValueChange={([value]) => handleRotationChange(value)}
            onPointerDown={startTransaction}
            onPointerUp={finalizeTransaction}
            onPointerCancel={abortTransaction}
            min={-180}
            max={180}
            step={1}
          />
        </TreeItem>
        <TreeItem>
          <Slider
            label="Turn"
            value={[isSingle ? (connection!.turn ?? 0) : (commonTurn ?? 0)]}
            onValueChange={([value]) => handleTurnChange(value)}
            onPointerDown={startTransaction}
            onPointerUp={finalizeTransaction}
            onPointerCancel={abortTransaction}
            min={-180}
            max={180}
            step={1}
          />
        </TreeItem>
        <TreeItem>
          <Slider
            label="Tilt"
            value={[isSingle ? (connection!.tilt ?? 0) : (commonTilt ?? 0)]}
            onValueChange={([value]) => handleTiltChange(value)}
            onPointerDown={startTransaction}
            onPointerUp={finalizeTransaction}
            onPointerCancel={abortTransaction}
            min={-180}
            max={180}
            step={1}
          />
        </TreeItem>
      </TreeSection>
    </>
  )
}



const Details: FC<DetailsProps> = ({ visible, onWidthChange, width }) => {
  if (!visible) return null
  const [isResizeHovered, setIsResizeHovered] = useState(false)
  const [isResizing, setIsResizing] = useState(false)

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault()
    setIsResizing(true)

    const startX = e.clientX
    const startWidth = width

    const handleMouseMove = (e: MouseEvent) => {
      const newWidth = startWidth - (e.clientX - startX)
      if (newWidth >= 150 && newWidth <= 500) {
        onWidthChange?.(newWidth)
      }
    }

    const handleMouseUp = () => {
      setIsResizing(false)
      document.removeEventListener('mousemove', handleMouseMove)
      document.removeEventListener('mouseup', handleMouseUp)
    }

    document.addEventListener('mousemove', handleMouseMove)
    document.addEventListener('mouseup', handleMouseUp)
  }

  const { selection } = useDesignEditor()

  const hasPieces = selection.selectedPieceIds.length > 0
  const hasConnections = selection.selectedConnections.length > 0
  const hasSelection = hasPieces || hasConnections

  return (
    <div
      className={`absolute top-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isResizing || isResizeHovered ? 'border-l-primary' : 'border-l'}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">
        <div className="p-1">
          <Tree>
            {!hasSelection && <DesignSection />}
            {hasPieces && <PiecesSection pieceIds={selection.selectedPieceIds} />}
            {hasConnections && <ConnectionsSection connections={selection.selectedConnections} />}
            {hasPieces && hasConnections && (
              <TreeSection label="Mixed Selection" defaultOpen={true}>
                <TreeItem>
                  <p className="text-sm text-muted-foreground">
                    Select only pieces or only connections to edit details.
                  </p>
                </TreeItem>
              </TreeSection>
            )}
          </Tree>
        </div>
      </ScrollArea>
      <div
        className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize"
        onMouseDown={handleMouseDown}
        onMouseEnter={() => setIsResizeHovered(true)}
        onMouseLeave={() => !isResizing && setIsResizeHovered(false)}
      />
    </div>
  )
}

interface ChatProps extends ResizablePanelProps { }

const Chat: FC<ChatProps> = ({ visible, onWidthChange, width }) => {
  if (!visible) return null
  const [isResizeHovered, setIsResizeHovered] = useState(false)
  const [isResizing, setIsResizing] = useState(false)

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault()
    setIsResizing(true)

    const startX = e.clientX
    const startWidth = width

    const handleMouseMove = (e: MouseEvent) => {
      const newWidth = startWidth - (e.clientX - startX)
      if (newWidth >= 150 && newWidth <= 500) {
        onWidthChange?.(newWidth)
      }
    }

    const handleMouseUp = () => {
      setIsResizing(false)
      document.removeEventListener('mousemove', handleMouseMove)
      document.removeEventListener('mouseup', handleMouseUp)
    }

    document.addEventListener('mousemove', handleMouseMove)
    document.addEventListener('mouseup', handleMouseUp)
  }

  return (
    <div
      className={`absolute top-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isResizing || isResizeHovered ? 'border-l-primary' : 'border-l'}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">
        <div className="p-1">
          <Tree>
            <TreeSection label="Conversation History" defaultOpen={true}>
              <TreeSection label="Design Session #1">
                <TreeItem label="How can I add a new piece?" />
                <TreeItem label="Can you help with connections?" />
              </TreeSection>
              <TreeSection label="Design Session #2">
                <TreeItem label="What are the available types?" />
              </TreeSection>
            </TreeSection>
            <TreeSection label="Quick Actions" defaultOpen={true}>
              <TreeItem label="Add random piece" />
              <TreeItem label="Connect all pieces" />
              <TreeItem label="Generate layout suggestions" />
            </TreeSection>
            <TreeSection label="Templates">
              <TreeSection label="Common Questions" defaultOpen={true}>
                <TreeItem label="How do I create a connection?" />
                <TreeItem label="How do I delete a piece?" />
                <TreeItem label="How do I change piece properties?" />
              </TreeSection>
              <TreeSection label="Advanced Workflows">
                <TreeItem label="Batch operations" />
                <TreeItem label="Complex layouts" />
                <TreeItem label="Export/Import" />
              </TreeSection>
            </TreeSection>
          </Tree>
        </div>
        <div className="p-4 border-t">
          <Textarea placeholder="Ask a question about the design..." />
        </div>
      </ScrollArea>
      <div
        className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize"
        onMouseDown={handleMouseDown}
        onMouseEnter={() => setIsResizeHovered(true)}
        onMouseLeave={() => !isResizing && setIsResizeHovered(false)}
      />
    </div>
  )
}

// Cli Component and interfaces
interface CliCommand {
  name: string
  description: string
  execute: (args: string[], cli: CliState) => Promise<void>
}

interface CliState {
  commands: CliCommand[]
  history: string[]
  currentInput: string
  suggestions: string[]
  isAwaitingInput: boolean
  awaitingPrompt?: string
  awaitingOptions?: { value: string; label: string }[]
  awaitingType?: 'text' | 'select' | 'confirm'
  onInputReceived?: (value: string) => void
  designEditor: ReturnType<typeof useDesignEditor>
}

const Cli: FC<{ designEditor: ReturnType<typeof useDesignEditor> }> = ({ designEditor }) => {
  const [state, setState] = useState<CliState>({
    commands: [],
    history: [],
    currentInput: '',
    suggestions: [],
    isAwaitingInput: false,
    designEditor
  })

  // Helper functions for interactive input
  const askForInput = (prompt: string, type: 'text' | 'select' | 'confirm' = 'text', options?: { value: string; label: string }[]): Promise<string> => {
    return new Promise((resolve) => {
      setState(prev => ({
        ...prev,
        history: [prompt],
        isAwaitingInput: true,
        awaitingPrompt: prompt,
        awaitingType: type,
        awaitingOptions: options,
        onInputReceived: resolve,
        currentInput: ''
      }))
    })
  }

  const askForText = (prompt: string, defaultValue?: string): Promise<string> => {
    return askForInput(`${prompt}${defaultValue ? ` (default: ${defaultValue})` : ''}:`)
  }

  const askForSelect = (prompt: string, options: { value: string; label: string }[]): Promise<string> => {
    const optionsText = options.map((opt, index) => `${index + 1}. ${opt.label}`).join('\n')
    return askForInput(`${prompt}:\n${optionsText}\nSelect (1-${options.length}):`, 'select', options)
  }

  const askForConfirm = (prompt: string): Promise<boolean> => {
    return askForInput(`${prompt} (y/n):`, 'confirm').then(answer =>
      answer.toLowerCase().startsWith('y')
    )
  }

  // Helper function to parse command arguments
  const parseArgs = (args: string[]) => {
    const parsed: { positional: string[], flags: Record<string, string | boolean> } = {
      positional: [],
      flags: {}
    }

    for (let i = 0; i < args.length; i++) {
      const arg = args[i]

      if (arg.startsWith('--')) {
        // Long flag: --flag or --flag=value
        const [key, value] = arg.slice(2).split('=', 2)
        if (value !== undefined) {
          parsed.flags[key] = value
        } else if (i + 1 < args.length && !args[i + 1].startsWith('-')) {
          // Next arg is the value
          parsed.flags[key] = args[++i]
        } else {
          // Boolean flag
          parsed.flags[key] = true
        }
      } else if (arg.startsWith('-') && arg.length > 1) {
        // Short flag: -f or -f value
        const key = arg.slice(1)
        if (i + 1 < args.length && !args[i + 1].startsWith('-')) {
          parsed.flags[key] = args[++i]
        } else {
          parsed.flags[key] = true
        }
      } else {
        // Positional argument
        parsed.positional.push(arg)
      }
    }

    return parsed
  }

  const getArgValue = (parsed: ReturnType<typeof parseArgs>, key: string, positionalIndex?: number): string | undefined => {
    // Check flags first (both long and short versions)
    if (parsed.flags[key] !== undefined) {
      return typeof parsed.flags[key] === 'string' ? parsed.flags[key] as string : undefined
    }

    // Check short flag version
    const shortKey = key.length > 1 ? key[0] : key
    if (parsed.flags[shortKey] !== undefined) {
      return typeof parsed.flags[shortKey] === 'string' ? parsed.flags[shortKey] as string : undefined
    }

    // Check positional
    if (positionalIndex !== undefined && parsed.positional[positionalIndex]) {
      return parsed.positional[positionalIndex]
    }

    return undefined
  }

  const hasFlag = (parsed: ReturnType<typeof parseArgs>, key: string): boolean => {
    return parsed.flags[key] === true || parsed.flags[key.length > 1 ? key[0] : key] === true
  }

  const addPieceCommand: CliCommand = {
    name: 'add-piece',
    description: '➕ Add a new piece to the design',
    execute: async (args, cli) => {
      cli.designEditor.startTransaction()

      try {
        // Get available types
        const types = cli.designEditor.kit.types || []
        const typeNames = [...new Set(types.map(t => t.name))]

        if (typeNames.length === 0) {
          throw new Error('No types available in kit')
        }

        const parsed = parseArgs(args)
        const isInteractive = parsed.positional.length === 0 && Object.keys(parsed.flags).length === 0

        // Piece ID
        let pieceId: string
        const idArg = getArgValue(parsed, 'id', 0)
        if (idArg) {
          pieceId = idArg === 'random' ? Generator.randomId() : idArg
        } else if (!isInteractive) {
          pieceId = Generator.randomId()
        } else {
          const idInput = await askForText('Enter piece ID (press Enter for random)', 'random')
          pieceId = idInput.trim() === '' || idInput.toLowerCase() === 'random' ? Generator.randomId() : idInput
        }

        // Type selection
        let selectedTypeName: string
        const typeArg = getArgValue(parsed, 'type', 1)
        if (typeArg && typeNames.includes(typeArg)) {
          selectedTypeName = typeArg
        } else if (!isInteractive && !typeArg) {
          throw new Error('Type must be specified in non-interactive mode')
        } else {
          const typeOptions = typeNames.map(name => ({ value: name, label: name }))
          const typeIndex = await askForSelect('Select type', typeOptions)
          const selectedIndex = parseInt(typeIndex) - 1
          if (selectedIndex < 0 || selectedIndex >= typeOptions.length) {
            throw new Error('Invalid type selection')
          }
          selectedTypeName = typeOptions[selectedIndex].value
        }

        // Variant selection
        const availableVariants = types
          .filter(t => t.name === selectedTypeName && t.variant)
          .map(t => t.variant!)

        let selectedVariant: string | undefined
        const variantArg = getArgValue(parsed, 'variant', 2)
        if (variantArg && (variantArg === '' || availableVariants.includes(variantArg))) {
          selectedVariant = variantArg === '' ? undefined : variantArg
        } else if (availableVariants.length > 0 && (isInteractive || !variantArg)) {
          const variantOptions = [
            { value: '', label: 'None (default)' },
            ...availableVariants.map(variant => ({ value: variant, label: variant }))
          ]
          const variantIndex = await askForSelect('Select variant', variantOptions)
          const selectedIndex = parseInt(variantIndex) - 1
          if (selectedIndex < 0 || selectedIndex >= variantOptions.length) {
            throw new Error('Invalid variant selection')
          }
          selectedVariant = variantOptions[selectedIndex].value || undefined
        }

        // Position input
        let centerX = 0, centerY = 0
        const xArg = getArgValue(parsed, 'x')
        if (xArg) {
          centerX = parseFloat(xArg) || 0
        } else if (isInteractive) {
          const xInput = await askForText('Enter X position', '0')
          centerX = parseFloat(xInput) || 0
        }

        const yArg = getArgValue(parsed, 'y')
        if (yArg) {
          centerY = parseFloat(yArg) || 0
        } else if (isInteractive) {
          const yInput = await askForText('Enter Y position', '0')
          centerY = parseFloat(yInput) || 0
        }

        // Ask if piece should be fixed
        let shouldFix = false
        if (hasFlag(parsed, 'fixed')) {
          shouldFix = true
        } else if (isInteractive) {
          shouldFix = await askForConfirm('Fix piece at position?')
        }

        const piece = {
          id_: pieceId,
          type: { name: selectedTypeName, variant: selectedVariant },
          ...(shouldFix && {
            plane: {
              origin: { x: centerX, y: centerY, z: 0 },
              xAxis: { x: 1, y: 0, z: 0 },
              yAxis: { x: 0, y: 1, z: 0 }
            }
          }),
          center: { x: centerX, y: centerY }
        }

        cli.designEditor.addPieceToDesign(piece)
        cli.designEditor.finalizeTransaction()

        setState(prev => ({
          ...prev,
          history: [`✓ Added piece ${piece.id_} of type ${selectedTypeName}${selectedVariant ? ` (${selectedVariant})` : ''} at (${centerX}, ${centerY})${shouldFix ? ' [FIXED]' : ''}`]
        }))
      } catch (error) {
        cli.designEditor.abortTransaction()
        setState(prev => ({
          ...prev,
          history: [`✗ Error adding piece: ${error}`]
        }))
      }
    }
  }

  const connectPiecesCommand: CliCommand = {
    name: 'connect-pieces',
    description: '🔗 Connect two pieces',
    execute: async (args, cli) => {
      cli.designEditor.startTransaction()

      try {
        const design = findDesignInKit(cli.designEditor.kit, cli.designEditor.designId)
        const availablePieces = design.pieces || []

        if (availablePieces.length < 2) {
          throw new Error('Need at least 2 pieces to create a connection')
        }

        const parsed = parseArgs(args)
        const isInteractive = parsed.positional.length === 0 && Object.keys(parsed.flags).length === 0

        // Select connecting piece
        let connectingPieceId: string
        const connectingArg = getArgValue(parsed, 'from', 0)
        if (connectingArg && availablePieces.some(p => p.id_ === connectingArg)) {
          connectingPieceId = connectingArg
        } else if (!isInteractive && !connectingArg) {
          throw new Error('Connecting piece ID must be specified in non-interactive mode')
        } else {
          const pieceOptions = availablePieces.map(piece => ({
            value: piece.id_,
            label: `${piece.id_} (${piece.type.name}${piece.type.variant ? ` - ${piece.type.variant}` : ''})`
          }))
          const connectingIndex = await askForSelect('Select connecting piece', pieceOptions)
          const selectedIndex = parseInt(connectingIndex) - 1
          if (selectedIndex < 0 || selectedIndex >= pieceOptions.length) {
            throw new Error('Invalid connecting piece selection')
          }
          connectingPieceId = pieceOptions[selectedIndex].value
        }

        // Select connected piece
        let connectedPieceId: string
        const connectedArg = getArgValue(parsed, 'to', 1)
        if (connectedArg && availablePieces.some(p => p.id_ === connectedArg) && connectedArg !== connectingPieceId) {
          connectedPieceId = connectedArg
        } else if (!isInteractive && !connectedArg) {
          throw new Error('Connected piece ID must be specified in non-interactive mode')
        } else {
          const pieceOptions = availablePieces
            .filter(piece => piece.id_ !== connectingPieceId)
            .map(piece => ({
              value: piece.id_,
              label: `${piece.id_} (${piece.type.name}${piece.type.variant ? ` - ${piece.type.variant}` : ''})`
            }))
          const connectedIndex = await askForSelect('Select connected piece', pieceOptions)
          const selectedIndex = parseInt(connectedIndex) - 1
          if (selectedIndex < 0 || selectedIndex >= pieceOptions.length) {
            throw new Error('Invalid connected piece selection')
          }
          connectedPieceId = pieceOptions[selectedIndex].value
        }

        // Connection parameters
        let gap = 0, shift = 0, rise = 0

        const gapArg = getArgValue(parsed, 'gap')
        if (gapArg !== undefined) {
          gap = parseFloat(gapArg) || 0
        } else if (isInteractive) {
          const gapInput = await askForText('Enter gap', '0')
          gap = parseFloat(gapInput) || 0
        }

        const shiftArg = getArgValue(parsed, 'shift')
        if (shiftArg !== undefined) {
          shift = parseFloat(shiftArg) || 0
        } else if (isInteractive) {
          const shiftInput = await askForText('Enter shift', '0')
          shift = parseFloat(shiftInput) || 0
        }

        const riseArg = getArgValue(parsed, 'rise')
        if (riseArg !== undefined) {
          rise = parseFloat(riseArg) || 0
        } else if (isInteractive) {
          const riseInput = await askForText('Enter rise', '0')
          rise = parseFloat(riseInput) || 0
        }

        const connection = {
          connecting: { piece: { id_: connectingPieceId }, port: { id_: '' } },
          connected: { piece: { id_: connectedPieceId }, port: { id_: '' } },
          gap,
          shift,
          rise,
          x: 0,
          y: 0,
          rotation: 0,
          turn: 0,
          tilt: 0
        }

        cli.designEditor.addConnectionToDesign(connection)
        cli.designEditor.finalizeTransaction()

        setState(prev => ({
          ...prev,
          history: [`✓ Connected piece ${connectingPieceId} to ${connectedPieceId} (gap: ${gap}, shift: ${shift}, rise: ${rise})`]
        }))
      } catch (error) {
        cli.designEditor.abortTransaction()
        setState(prev => ({
          ...prev,
          history: [`✗ Error connecting pieces: ${error}`]
        }))
      }
    }
  }

  const flattenDesignCommand: CliCommand = {
    name: 'flatten-design',
    description: '⬇️ Flatten the current design',
    execute: async (args, cli) => {
      cli.designEditor.startTransaction()

      try {
        const flattened = flattenDesign(cli.designEditor.kit, cli.designEditor.designId)
        cli.designEditor.setDesign(flattened)
        cli.designEditor.finalizeTransaction()

        setState(prev => ({
          ...prev,
          history: [`✓ Design flattened successfully`]
        }))
      } catch (error) {
        cli.designEditor.abortTransaction()
        setState(prev => ({
          ...prev,
          history: [`✗ Error flattening design: ${error}`]
        }))
      }
    }
  }

  const selectAllCommand: CliCommand = {
    name: 'select-all',
    description: 'Select all pieces in the design',
    execute: async (args, cli) => {
      cli.designEditor.selectAll()
      setState(prev => ({
        ...prev,
        history: [`✓ Selected all pieces`]
      }))
    }
  }

  const deselectAllCommand: CliCommand = {
    name: 'deselect-all',
    description: 'Deselect all pieces in the design',
    execute: async (args, cli) => {
      cli.designEditor.deselectAll()
      setState(prev => ({
        ...prev,
        history: [`✓ Deselected all pieces`]
      }))
    }
  }

  const deleteSelectedCommand: CliCommand = {
    name: 'delete-selected',
    description: 'Delete currently selected pieces and connections',
    execute: async (args, cli) => {
      cli.designEditor.startTransaction()

      try {
        cli.designEditor.deleteSelected()
        cli.designEditor.finalizeTransaction()

        setState(prev => ({
          ...prev,
          history: [`✓ Deleted selected pieces and connections`]
        }))
      } catch (error) {
        cli.designEditor.abortTransaction()
        setState(prev => ({
          ...prev,
          history: [`✗ Error deleting selection: ${error}`]
        }))
      }
    }
  }

  const listPiecesCommand: CliCommand = {
    name: 'list-pieces',
    description: '📋 List all pieces in the design',
    execute: async (args, cli) => {
      const design = findDesignInKit(cli.designEditor.kit, cli.designEditor.designId)
      const pieces = design.pieces || []

      if (pieces.length === 0) {
        setState(prev => ({
          ...prev,
          history: ['No pieces in design']
        }))
        return
      }

      const parsed = parseArgs(args)
      const showDetails = hasFlag(parsed, 'details') || hasFlag(parsed, 'd')
      const filterType = getArgValue(parsed, 'type') || getArgValue(parsed, 't')
      const filterVariant = getArgValue(parsed, 'variant') || getArgValue(parsed, 'v')

      let filteredPieces = pieces
      if (filterType) {
        filteredPieces = filteredPieces.filter(p => p.type.name === filterType)
      }
      if (filterVariant) {
        filteredPieces = filteredPieces.filter(p => p.type.variant === filterVariant)
      }

      if (filteredPieces.length === 0) {
        setState(prev => ({
          ...prev,
          history: [`No pieces found${filterType ? ` with type "${filterType}"` : ''}${filterVariant ? ` and variant "${filterVariant}"` : ''}`]
        }))
        return
      }

      let output = `Found ${filteredPieces.length} pieces${filterType || filterVariant ? ' (filtered)' : ''}:\n`
      filteredPieces.forEach((piece, index) => {
        const position = piece.center ? `(${piece.center.x.toFixed(2)}, ${piece.center.y.toFixed(2)})` : 'No position'
        const fixed = piece.plane ? '[FIXED]' : '[LINKED]'

        if (showDetails) {
          output += `${index + 1}. ${piece.id_}\n`
          output += `   Type: ${piece.type.name}${piece.type.variant ? ` - ${piece.type.variant}` : ''}\n`
          output += `   Position: ${position} ${fixed}\n`
        } else {
          output += `${index + 1}. ${piece.id_} (${piece.type.name}${piece.type.variant ? ` - ${piece.type.variant}` : ''}) ${position} ${fixed}\n`
        }
      })

      setState(prev => ({
        ...prev,
        history: [output.trim()]
      }))
    }
  }

  const selectPiecesCommand: CliCommand = {
    name: 'select-pieces',
    description: '🎯 Select specific pieces by ID, type, or variant',
    execute: async (args, cli) => {
      const design = findDesignInKit(cli.designEditor.kit, cli.designEditor.designId)
      const pieces = design.pieces || []

      if (pieces.length === 0) {
        setState(prev => ({
          ...prev,
          history: ['No pieces in design to select']
        }))
        return
      }

      const parsed = parseArgs(args)
      const isInteractive = parsed.positional.length === 0 && Object.keys(parsed.flags).length === 0

      let selectedPieceIds: string[] = []

      if (isInteractive) {
        // Interactive mode - ask what to select
        const selectionOptions = [
          { value: 'all', label: 'Select all pieces' },
          { value: 'by-type', label: 'Select by type' },
          { value: 'by-variant', label: 'Select by variant' },
          { value: 'by-id', label: 'Select by ID' },
          { value: 'by-status', label: 'Select by status (fixed/linked)' }
        ]
        const selectionIndex = await askForSelect('What would you like to select?', selectionOptions)
        const selectedIndex = parseInt(selectionIndex) - 1
        if (selectedIndex < 0 || selectedIndex >= selectionOptions.length) {
          throw new Error('Invalid selection')
        }

        const selectionType = selectionOptions[selectedIndex].value

        switch (selectionType) {
          case 'all':
            selectedPieceIds = pieces.map(p => p.id_)
            break
          case 'by-type':
            const typeNames = [...new Set(pieces.map(p => p.type.name))]
            const typeOptions = typeNames.map(name => ({ value: name, label: name }))
            const typeIndex = await askForSelect('Select type', typeOptions)
            const selectedTypeIndex = parseInt(typeIndex) - 1
            if (selectedTypeIndex >= 0 && selectedTypeIndex < typeOptions.length) {
              const selectedType = typeOptions[selectedTypeIndex].value
              selectedPieceIds = pieces.filter(p => p.type.name === selectedType).map(p => p.id_)
            }
            break
          case 'by-variant':
            const variants = [...new Set(pieces.map(p => p.type.variant).filter(v => v))]
            if (variants.length === 0) {
              throw new Error('No variants available')
            }
            const variantOptions = variants.map(variant => ({ value: variant!, label: variant! }))
            const variantIndex = await askForSelect('Select variant', variantOptions)
            const selectedVariantIndex = parseInt(variantIndex) - 1
            if (selectedVariantIndex >= 0 && selectedVariantIndex < variantOptions.length) {
              const selectedVariant = variantOptions[selectedVariantIndex].value
              selectedPieceIds = pieces.filter(p => p.type.variant === selectedVariant).map(p => p.id_)
            }
            break
          case 'by-id':
            const pieceOptions = pieces.map(piece => ({
              value: piece.id_,
              label: `${piece.id_} (${piece.type.name}${piece.type.variant ? ` - ${piece.type.variant}` : ''})`
            }))
            const pieceIndex = await askForSelect('Select piece', pieceOptions)
            const selectedPieceIndex = parseInt(pieceIndex) - 1
            if (selectedPieceIndex >= 0 && selectedPieceIndex < pieceOptions.length) {
              selectedPieceIds = [pieceOptions[selectedPieceIndex].value]
            }
            break
          case 'by-status':
            const statusOptions = [
              { value: 'fixed', label: 'Fixed pieces' },
              { value: 'linked', label: 'Linked pieces' }
            ]
            const statusIndex = await askForSelect('Select status', statusOptions)
            const selectedStatusIndex = parseInt(statusIndex) - 1
            if (selectedStatusIndex >= 0 && selectedStatusIndex < statusOptions.length) {
              const isFixed = statusOptions[selectedStatusIndex].value === 'fixed'
              selectedPieceIds = pieces.filter(p => !!p.plane === isFixed).map(p => p.id_)
            }
            break
        }
      } else {
        // Non-interactive mode
        const typeFilter = getArgValue(parsed, 'type') || getArgValue(parsed, 't')
        const variantFilter = getArgValue(parsed, 'variant') || getArgValue(parsed, 'v')
        const statusFilter = getArgValue(parsed, 'status') || getArgValue(parsed, 's')
        const addMode = hasFlag(parsed, 'add') || hasFlag(parsed, 'a')

        // Use positional arguments as piece IDs
        if (parsed.positional.length > 0) {
          selectedPieceIds = parsed.positional.filter(id => pieces.some(p => p.id_ === id))
        } else {
          // Filter by flags
          let filteredPieces = pieces
          if (typeFilter) {
            filteredPieces = filteredPieces.filter(p => p.type.name === typeFilter)
          }
          if (variantFilter) {
            filteredPieces = filteredPieces.filter(p => p.type.variant === variantFilter)
          }
          if (statusFilter) {
            const isFixed = statusFilter.toLowerCase() === 'fixed'
            filteredPieces = filteredPieces.filter(p => !!p.plane === isFixed)
          }
          selectedPieceIds = filteredPieces.map(p => p.id_)
        }

        // If add mode, add to existing selection, otherwise replace
        if (addMode) {
          cli.designEditor.addPiecesToSelection(selectedPieceIds)
        } else {
          cli.designEditor.selectPieces(selectedPieceIds)
        }
      }

      if (selectedPieceIds.length === 0) {
        setState(prev => ({
          ...prev,
          history: ['No pieces matched the selection criteria']
        }))
        return
      }

      // Apply selection
      if (!isInteractive && (hasFlag(parsed, 'add') || hasFlag(parsed, 'a'))) {
        cli.designEditor.addPiecesToSelection(selectedPieceIds)
      } else {
        cli.designEditor.selectPieces(selectedPieceIds)
      }

      setState(prev => ({
        ...prev,
        history: [`✓ Selected ${selectedPieceIds.length} piece${selectedPieceIds.length === 1 ? '' : 's'}: ${selectedPieceIds.join(', ')}`]
      }))
    }
  }

  const listTypesCommand: CliCommand = {
    name: 'list-types',
    description: 'List all available types in the kit',
    execute: async (args, cli) => {
      const types = cli.designEditor.kit.types || []
      const typeNames = [...new Set(types.map(t => t.name))]

      setState(prev => ({
        ...prev,
        history: [`Available types: ${typeNames.join(', ')}`]
      }))
    }
  }

  const helpCommand: CliCommand = {
    name: 'help',
    description: 'Show available commands',
    execute: async (args, cli) => {
      const commandList = cli.commands.map(cmd => `  ${cmd.name.padEnd(20)} - ${cmd.description}`).join('\n')
      const usage = `
Usage examples:

Positional arguments:
  add-piece Wall                       - Add piece with type Wall (interactive for rest)
  add-piece Wall Exterior              - Add piece with type Wall, variant Exterior
  connect-pieces piece1 piece2         - Connect two pieces by ID
  select-pieces piece1 piece2 piece3   - Select specific pieces by ID

Flagged arguments:
  add-piece --type Wall --variant Exterior --x 5 --y 3 --fixed
  add-piece -t Wall -v Exterior -x 5 -y 3 --fixed
  connect-pieces --from piece1 --to piece2 --gap 1.5 --shift 0
  list-pieces --details --type Wall --variant Exterior
  list-pieces -d -t Wall -v Exterior
  select-pieces --type Wall --add      - Add Wall pieces to current selection

Interactive mode (no arguments):
  add-piece                           - Interactive guided setup
  connect-pieces                      - Interactive piece selection
  select-pieces                       - Interactive selection options

Mixed arguments:
  add-piece Wall --x 5 --y 3          - Type as positional, position as flags
  connect-pieces piece1 --to piece2 --gap 1.5

All commands support both short (-t) and long (--type) flag formats.
Commands without arguments automatically enter interactive mode.`

      setState(prev => ({
        ...prev,
        history: [`Available commands:\n${commandList}\n${usage}`]
      }))
    }
  }

  const clearCommand: CliCommand = {
    name: 'clear',
    description: 'Clear command history',
    execute: async (args, cli) => {
      setState(prev => ({
        ...prev,
        history: []
      }))
    }
  }

  // Initialize commands
  useEffect(() => {
    if (state.commands.length === 0) {
      setState(prev => ({
        ...prev,
        commands: [addPieceCommand, connectPiecesCommand, flattenDesignCommand, selectAllCommand, deselectAllCommand, selectPiecesCommand, deleteSelectedCommand, listPiecesCommand, listTypesCommand, helpCommand, clearCommand]
      }))
    }
  }, [])

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      e.preventDefault()
      handleInput(state.currentInput)
    } else if (e.key === 'Tab') {
      e.preventDefault()
      if (state.suggestions.length > 0) {
        if (state.isAwaitingInput && state.awaitingType === 'select') {
          // For select mode, cycle through number suggestions
          const currentIndex = state.suggestions.indexOf(state.currentInput)
          const nextIndex = (currentIndex + 1) % state.suggestions.length
          setState(prev => ({ ...prev, currentInput: state.suggestions[nextIndex] }))
        } else {
          // For normal commands or text input, use first suggestion
          setState(prev => ({ ...prev, currentInput: prev.suggestions[0] }))
        }
      }
    } else if (e.key === 'Escape' && state.isAwaitingInput) {
      // Allow escape to cancel interactive input
      setState(prev => ({
        ...prev,
        history: ['✗ Cancelled'],
        isAwaitingInput: false,
        awaitingPrompt: undefined,
        awaitingType: undefined,
        awaitingOptions: undefined,
        onInputReceived: undefined,
        currentInput: ''
      }))
    }

    // Update cursor position after key handling
    setTimeout(() => {
      const input = inputRef.current
      if (input) {
        setCursorPosition(input.selectionStart || 0)
      }
    }, 0)
  }

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const input = e.target.value
    updateSuggestions(input)

    // Update cursor position
    setTimeout(() => {
      const inputEl = inputRef.current
      if (inputEl) {
        setCursorPosition(inputEl.selectionStart || 0)
      }
    }, 0)
  }

  const handleInput = (input: string) => {
    // If awaiting input, handle the response
    if (state.isAwaitingInput && state.onInputReceived) {
      let processedInput = input.trim()

      // Handle different input types
      if (state.awaitingType === 'select' && state.awaitingOptions) {
        const index = parseInt(processedInput) - 1
        if (index >= 0 && index < state.awaitingOptions.length) {
          processedInput = (index + 1).toString()
        }
      }

      setState(prev => ({
        ...prev,
        history: [],
        currentInput: '',
        isAwaitingInput: false,
        awaitingPrompt: undefined,
        awaitingType: undefined,
        awaitingOptions: undefined
      }))

      state.onInputReceived(processedInput)
      return
    }

    // Parse command
    const parts = input.trim().split(' ')
    const commandName = parts[0]
    const args = parts.slice(1)

    if (!commandName) return

    const command = state.commands.find(cmd => cmd.name === commandName)
    if (command) {
      setState(prev => ({
        ...prev,
        history: [],
        currentInput: ''
      }))
      command.execute(args, state)
    } else {
      setState(prev => ({
        ...prev,
        history: [`✗ Unknown command: ${commandName}. Type 'help' for available commands.`],
        currentInput: ''
      }))
    }
  }

  const updateSuggestions = (input: string) => {
    // If awaiting input, show context-specific suggestions
    if (state.isAwaitingInput) {
      if (state.awaitingType === 'select' && state.awaitingOptions) {
        const suggestions = state.awaitingOptions
          .map((opt, index) => `${index + 1}`)
          .filter(num => num.startsWith(input))
        setState(prev => ({ ...prev, suggestions, currentInput: input }))
      } else if (state.awaitingType === 'confirm') {
        const suggestions = ['y', 'yes', 'n', 'no'].filter(opt => opt.startsWith(input.toLowerCase()))
        setState(prev => ({ ...prev, suggestions, currentInput: input }))
      } else {
        setState(prev => ({ ...prev, suggestions: [], currentInput: input }))
      }
      return
    }

    // Normal command suggestions
    const suggestions = state.commands
      .filter(cmd => cmd.name.startsWith(input.toLowerCase()))
      .map(cmd => cmd.name)
    setState(prev => ({ ...prev, suggestions, currentInput: input }))
  }

  const [cursorPosition, setCursorPosition] = useState(0)
  const [isFocused, setIsFocused] = useState(true)
  const inputRef = useRef<HTMLInputElement>(null)

  // Update cursor position when input changes or cursor moves
  useEffect(() => {
    const input = inputRef.current
    if (input) {
      const updateCursorPosition = () => {
        setCursorPosition(input.selectionStart || 0)
      }

      input.addEventListener('select', updateCursorPosition)
      input.addEventListener('click', updateCursorPosition)
      input.addEventListener('keyup', updateCursorPosition)

      return () => {
        input.removeEventListener('select', updateCursorPosition)
        input.removeEventListener('click', updateCursorPosition)
        input.removeEventListener('keyup', updateCursorPosition)
      }
    }
  }, [])

  const handleOptionClick = (optionIndex: number) => {
    if (state.isAwaitingInput && state.awaitingType === 'select') {
      const optionNumber = (optionIndex + 1).toString()
      setState(prev => ({ ...prev, currentInput: optionNumber }))
      handleInput(optionNumber)
    }
  }

  const renderInputWithCursor = () => {
    const text = state.currentInput
    const beforeCursor = text.slice(0, cursorPosition)
    const atCursor = text[cursorPosition] || ' '
    const afterCursor = text.slice(cursorPosition + 1)

    return (
      <div className="flex-1 relative">
        <input
          ref={inputRef}
          type="text"
          value={state.currentInput}
          onChange={handleInputChange}
          onKeyDown={handleKeyDown}
          onFocus={() => setIsFocused(true)}
          onBlur={() => setIsFocused(false)}
          className="w-full outline-none bg-transparent text-transparent caret-transparent"
          placeholder={
            state.isAwaitingInput
              ? (state.awaitingType === 'select' ? "Enter number or click option..." :
                state.awaitingType === 'confirm' ? "Enter y/n..." :
                  "Enter value...")
              : "Type a command..."
          }
          style={{ caretColor: 'transparent' }}
          autoFocus
        />
        <div className="absolute inset-0 pointer-events-none flex items-center px-3">
          <span className="text-primary">{beforeCursor}</span>
          <span
            className={`cli-cursor inline-block ${isFocused
              ? (atCursor === ' '
                ? 'bg-foreground w-2'
                : 'bg-foreground text-background')
              : (atCursor === ' '
                ? 'border border-foreground w-2'
                : 'border border-foreground text-foreground bg-transparent')
              }`}
            style={{
              minWidth: '0.5rem',
              height: '1.25rem'
            }}
          >
            {atCursor === ' ' ? '\u00A0' : atCursor}
          </span>
          <span className="text-primary">{afterCursor}</span>
        </div>
      </div>
    )
  }

  return (
    <div
      className="h-full flex flex-col text-sm p-2"
      onClick={() => {
        // Focus input when clicking anywhere in the console
        if (inputRef.current) {
          inputRef.current.focus()
        }
      }}
    >
      <div className="flex-1 overflow-y-auto mb-2">
        {state.history.map((line, index) => {
          // Check if this is a select prompt with options
          if (state.isAwaitingInput && state.awaitingType === 'select' && state.awaitingOptions) {
            const lines = line.split('\n')
            return (
              <div key={index} className="whitespace-pre-wrap text-primary">
                {lines.map((singleLine, lineIndex) => {
                  // Check if this line is an option (starts with number and dot)
                  const optionMatch = singleLine.match(/^(\d+)\. (.+)$/)
                  if (optionMatch) {
                    const optionNumber = parseInt(optionMatch[1])
                    const optionText = optionMatch[2]
                    return (
                      <div
                        key={lineIndex}
                        className="cursor-pointer hover:underline hover:text-primary-foreground"
                        onClick={() => handleOptionClick(optionNumber - 1)}
                      >
                        {optionNumber}. {optionText}
                      </div>
                    )
                  }
                  return <div key={lineIndex}>{singleLine}</div>
                })}
              </div>
            )
          }

          return (
            <div key={index} className="whitespace-pre-wrap text-primary">
              {line}
            </div>
          )
        })}
      </div>

      <div className="flex items-center space-x-1">
        <span className="text-foreground">
          {state.isAwaitingInput ? "Input:" : "Command:"}
        </span>
        {renderInputWithCursor()}
      </div>

      {state.suggestions.length > 0 && (
        <div className="mt-1 text-gray text-xs">
          {state.isAwaitingInput
            ? `Options: ${state.suggestions.join(', ')}`
            : `Suggestions: ${state.suggestions.join(', ')}`
          }
        </div>
      )}

      {state.awaitingType === 'select' && state.awaitingOptions && (
        <div className="mt-1 text-gray text-xs">
          Tip: Type the number, click an option, use Tab to cycle, or press Escape to cancel
        </div>
      )}

      {state.isAwaitingInput && state.awaitingType !== 'select' && (
        <div className="mt-1 text-gray text-xs">
          Tip: Press Escape to cancel
        </div>
      )}
    </div>
  )
}

//#endregion Panels

const DesignEditorCore: FC<DesignEditorProps> = (props) => {
  const { onToolbarChange, designId, onUndo: controlledOnUndo, onRedo: controlledOnRedo } = props

  // #region State

  const [state, dispatch] = useControllableReducer(props)
  if (!state.kit) return null
  const design = findDesignInKit(state.kit, designId)
  if (!design) return null

  // #endregion State

  // #region Panels

  const [visiblePanels, setVisiblePanels] = useState<PanelToggles>({
    workbench: false,
    console: false,
    details: false,
    chat: false
  })
  const [workbenchWidth, setWorkbenchWidth] = useState(230)
  const [detailsWidth, setDetailsWidth] = useState(230)
  const [consoleHeight, setConsoleHeight] = useState(200)
  const [chatWidth, setChatWidth] = useState(230)

  const togglePanel = (panel: keyof PanelToggles) => {
    setVisiblePanels((prev) => {
      const newState = { ...prev }
      if (panel === 'chat' && !prev.chat) {
        newState.details = false
      }
      if (panel === 'details' && !prev.details) {
        newState.chat = false
      }
      newState[panel] = !prev[panel]
      return newState
    })
  }

  const onDoubleClickDiagram = useCallback((e: React.MouseEvent) => {
    e.stopPropagation()
    dispatch({ type: DesignEditorAction.ToggleDiagramFullscreen, payload: null })
  }, [dispatch])

  const onDoubleClickModel = useCallback((e: React.MouseEvent) => {
    e.stopPropagation()
    dispatch({ type: DesignEditorAction.ToggleModelFullscreen, payload: null })
  }, [dispatch])

  // #endregion Panels

  // #region Hotkeys
  useHotkeys('mod+j', (e) => {
    e.preventDefault()
    e.stopPropagation()
    togglePanel('workbench')
  })
  useHotkeys('mod+k', (e) => {
    e.preventDefault()
    e.stopPropagation()
    togglePanel('console')
  })
  useHotkeys('mod+l', (e) => {
    e.preventDefault()
    e.stopPropagation()
    togglePanel('details')
  })
  useHotkeys(['mod+[', 'mod+semicolon', 'mod+ö'], (e) => {
    e.preventDefault()
    e.stopPropagation()
    togglePanel('chat')
  })
  useHotkeys('mod+a', (e) => {
    e.preventDefault()
    dispatch({ type: DesignEditorAction.SelectAll, payload: null })
  })
  useHotkeys('mod+i', (e) => {
    e.preventDefault()
    dispatch({ type: DesignEditorAction.InvertSelection, payload: null })
  })
  useHotkeys('mod+d', (e) => {
    e.preventDefault()
    console.log('Select closest piece with same variant')
  })
  useHotkeys('mod+shift+d', (e) => {
    e.preventDefault()
    console.log('Select all pieces with same variant')
  })
  useHotkeys('mod+c', (e) => {
    e.preventDefault()
    copyToClipboard(design, state.selection)
  })
  useHotkeys('mod+v', (e) => {
    e.preventDefault()
    pasteFromClipboard(design)
  })
  useHotkeys('mod+x', (e) => {
    e.preventDefault()
    cutToClipboard(design, state.selection)
  })
  useHotkeys('delete', (e) => {
    e.preventDefault()
    dispatch({ type: DesignEditorAction.DeleteSelected, payload: null })
  })
  useHotkeys('mod+z', (e) => {
    e.preventDefault()
    dispatch({ type: DesignEditorAction.Redo, payload: null })
  })
  useHotkeys('mod+y', (e) => {
    e.preventDefault()
    dispatch({ type: DesignEditorAction.Undo, payload: null })
  })
  useHotkeys('mod+w', (e) => {
    e.preventDefault()
    console.log('Close design')
  })
  // #endregion Hotkeys

  const rightPanelVisible = visiblePanels.details || visiblePanels.chat

  const designEditorToolbar = (
    <ToggleGroup
      type="multiple"
      value={Object.entries(visiblePanels)
        .filter(([_, isVisible]) => isVisible)
        .map(([key]) => key)}
      onValueChange={(values) => {
        Object.keys(visiblePanels).forEach((key) => {
          const isCurrentlyVisible = visiblePanels[key as keyof PanelToggles]
          const shouldBeVisible = values.includes(key)
          if (isCurrentlyVisible !== shouldBeVisible) {
            togglePanel(key as keyof PanelToggles)
          }
        })
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
  )

  useEffect(() => {
    onToolbarChange(designEditorToolbar)
    return () => onToolbarChange(null)
  }, [visiblePanels])

  const { screenToFlowPosition } = useReactFlow()
  const [activeDraggedType, setActiveDraggedType] = useState<Type | null>(null)
  const [activeDraggedDesign, setActiveDraggedDesign] = useState<Design | null>(null)

  // #region Drag and Drop

  const onDragStart = (event: DragStartEvent) => {
    const { active } = event
    const id = active.id.toString()
    if (id.startsWith('type-')) {
      const [_, name, variant] = id.split('-')
      // Normalize variants so that undefined, null and empty string are treated the same
      const normalizeVariant = (v: string | undefined | null) => v ?? ''
      const type = state.kit?.types?.find(
        (t: Type) => t.name === name && normalizeVariant(t.variant) === normalizeVariant(variant)
      )
      setActiveDraggedType(type || null)
    } else if (id.startsWith('design-')) {
      const [_, name, variant, view] = id.split('-')
      const draggedDesignId: DesignId = { name, variant: variant || undefined, view: view || undefined }
      const draggedDesign = findDesignInKit(state.kit, draggedDesignId)
      setActiveDraggedDesign(draggedDesign || null)
    }
  }

  const onDragEnd = (event: DragEndEvent) => {
    const { over } = event
    if (over?.id === 'diagram') {
      if (!(event.activatorEvent instanceof PointerEvent)) {
        return
      }
      if (activeDraggedType) {
        const { x, y } = screenToFlowPosition({
          x: event.activatorEvent.clientX + event.delta.x,
          y: event.activatorEvent.clientY + event.delta.y
        })
        const piece: Piece = {
          id_: Generator.randomId(),
          type: {
            name: activeDraggedType.name,
            variant: activeDraggedType.variant || undefined
          },
          plane: {
            origin: { x: 0, y: 0, z: 0 },
            xAxis: { x: 1, y: 0, z: 0 },
            yAxis: { x: 0, y: 1, z: 0 }
          },
          center: { x: x / ICON_WIDTH - 0.5, y: -y / ICON_WIDTH + 0.5 }
        }
        dispatch({
          type: DesignEditorAction.AddPieceToDesign,
          payload: piece
        })
      } else if (activeDraggedDesign) {
        throw new Error('Not implemented')
      }
    }
    setActiveDraggedType(null)
    setActiveDraggedDesign(null)
  }

  // #endregion Drag and Drop

  return (
    <DesignEditorContext.Provider value={{ state, dispatch }}>
      <DndContext onDragStart={onDragStart} onDragEnd={onDragEnd}>
        <div className="canvas flex-1 relative">
          <div id="sketchpad-edgeless" className="h-full">
            <ResizablePanelGroup direction="horizontal">
              <ResizablePanel
                defaultSize={state.fullscreenPanel === FullscreenPanel.Diagram ? 100 : 50}
                className={`${state.fullscreenPanel === FullscreenPanel.Model ? 'hidden' : 'block'}`}
                onDoubleClick={onDoubleClickDiagram}
              >
                <Diagram />
              </ResizablePanel>
              <ResizableHandle
                className={`border-r ${state.fullscreenPanel !== FullscreenPanel.None ? 'hidden' : 'block'}`}
              />
              <ResizablePanel
                defaultSize={state.fullscreenPanel === FullscreenPanel.Model ? 100 : 50}
                className={`${state.fullscreenPanel === FullscreenPanel.Diagram ? 'hidden' : 'block'}`}
                onDoubleClick={onDoubleClickModel}
              >
                <Model />
              </ResizablePanel>
            </ResizablePanelGroup>
          </div>
          <Workbench
            visible={visiblePanels.workbench}
            onWidthChange={setWorkbenchWidth}
            width={workbenchWidth}
            kit={state.kit!}
          />
          <Details visible={visiblePanels.details} onWidthChange={setDetailsWidth} width={detailsWidth} />
          <Console
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
            document.body
          )}
        </div>
      </DndContext>
    </DesignEditorContext.Provider>
  )
}

interface ControlledDesignEditorProps {
  kit?: Kit
  selection?: DesignEditorSelection
  onDesignChange?: (design: Design) => void
  onSelectionChange?: (selection: DesignEditorSelection) => void
  onUndo?: () => void
  onRedo?: () => void
}

interface UncontrolledDesignEditorProps {
  initialKit?: Kit
  initialSelection?: DesignEditorSelection
}

interface DesignEditorProps extends ControlledDesignEditorProps, UncontrolledDesignEditorProps {
  designId: DesignId
  fileUrls: Map<string, string>
  onToolbarChange: (toolbar: ReactNode) => void
  mode?: Mode
  layout?: Layout
  theme?: Theme
  setLayout?: (layout: Layout) => void
  setTheme?: (theme: Theme) => void
  onWindowEvents?: {
    minimize: () => void
    maximize: () => void
    close: () => void
  }
}

const DesignEditor: FC<DesignEditorProps> = ({
  mode,
  layout,
  theme,
  setLayout,
  setTheme,
  onWindowEvents,
  designId,
  kit,
  selection,
  initialKit,
  initialSelection,
  fileUrls,
  onDesignChange,
  onSelectionChange,
  onUndo,
  onRedo
}) => {
  const [toolbarContent, setToolbarContent] = useState<ReactNode>(null)

  return (
    <div key={`layout-${layout}`} className="h-full w-full flex flex-col bg-background text-foreground">
      <Navbar
        mode={mode}
        toolbarContent={toolbarContent}
        layout={layout}
        theme={theme}
        setLayout={setLayout}
        setTheme={setTheme}
        onWindowEvents={onWindowEvents}
      />
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
        />
      </ReactFlowProvider>
    </div>
  )
}

export default DesignEditor

//#endregion
