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

import { DndContext, DragEndEvent, DragOverlay, DragStartEvent, useDraggable } from '@dnd-kit/core'
import { arrayMove } from '@dnd-kit/sortable'
import { ReactFlowProvider, useReactFlow } from '@xyflow/react'
import { Info, MessageCircle, Minus, Pin, Plus, Terminal, Trash2, Wrench } from 'lucide-react'
import { FC, ReactNode, createContext, useCallback, useContext, useEffect, useReducer, useState } from 'react'
import { createPortal } from 'react-dom'
import { useHotkeys } from 'react-hotkeys-hook'

import {
  Connection,
  ConnectionId,
  Design, DesignDiff, DesignId,
  DiagramPoint,
  ICON_WIDTH, Kit, Piece, PieceId,
  Plane, Type,
  addConnectionToDesign,
  addConnectionToDesignDiff,
  addConnectionsToDesign,
  addConnectionsToDesignDiff,
  addPieceToDesignDiff,
  addPiecesToDesign,
  addPiecesToDesignDiff,
  applyDesignDiff,
  findConnectionInDesign,
  findDesignInKit,
  findPieceInDesign,
  findReplacableTypesForPieceInDesign,
  findReplacableTypesForPiecesInDesign,
  findTypeInKit,
  isSameConnection,
  isSameDesign,
  mergeDesigns,
  piecesMetadata,
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
  updateDesignInKit
} from '@semio/js'
import { Avatar, AvatarFallback } from '@semio/js/components/ui/Avatar'
import Combobox from '@semio/js/components/ui/Combobox'
import Diagram from '@semio/js/components/ui/Diagram'
import { HoverCard, HoverCardContent, HoverCardTrigger } from '@semio/js/components/ui/HoverCard'
import { Input } from '@semio/js/components/ui/Input'
import Model from '@semio/js/components/ui/Model'
import { default as Navbar } from '@semio/js/components/ui/Navbar'
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '@semio/js/components/ui/Resizable'
import { ScrollArea } from '@semio/js/components/ui/ScrollArea'
import { Layout, Mode, Theme } from '@semio/js/components/ui/Sketchpad'
import { Slider } from '@semio/js/components/ui/Slider'
import Stepper from '@semio/js/components/ui/Stepper'
import { Textarea } from '@semio/js/components/ui/Textarea'
import { ToggleGroup, ToggleGroupItem } from '@semio/js/components/ui/ToggleGroup'
import { SortableTreeItems, Tree, TreeItem, TreeSection } from '@semio/js/components/ui/Tree'
import { Generator } from '@semio/js/lib/utils'
import { orientDesign } from '../../semio'
import Console, { CommandContext, commandRegistry } from './Console'
import { designEditorCommands } from './designEditorCommands'

// Helper functions for commands
const addPieceToDesign = (design: Design, piece: Piece): Design => ({
  ...design,
  pieces: [...(design.pieces || []), piece]
})

//#region State

export interface DesignEditorSelection {
  selectedPieceIds: string[];
  selectedConnections: {
    connectingPieceId: string;
    connectedPieceId: string;
  }[];
  selectedPiecePortId?: { pieceId: string; portId: string }
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
  AddPiece = 'ADD_PIECE',
  SetPiece = 'SET_PIECE',
  RemovePiece = 'REMOVE_PIECE',
  AddPieces = 'ADD_PIECES',
  SetPieces = 'SET_PIECES',
  RemovePieces = 'REMOVE_PIECES',
  AddConnection = 'ADD_CONNECTION',
  SetConnection = 'SET_CONNECTION',
  RemoveConnection = 'REMOVE_CONNECTION',
  AddConnections = 'ADD_CONNECTIONS',
  SetConnections = 'SET_CONNECTIONS',
  RemoveConnections = 'REMOVE_CONNECTIONS',
  RemovePiecesAndConnections = 'REMOVE_PIECES_AND_CONNECTIONS',
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
  SelectPiecePort = 'SELECT_PIECE_PORT',
  DeselectPiecePort = 'DESELECT_PIECE_PORT',
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

const selectionConnectionToConnectionId = (selectionConnection: { connectingPieceId: string; connectedPieceId: string }): ConnectionId => ({
  connected: { piece: { id_: selectionConnection.connectedPieceId } }, connecting: { piece: { id_: selectionConnection.connectingPieceId } }
})
const connectionToSelectionConnection = (connection: Connection | ConnectionId): { connectingPieceId: string; connectedPieceId: string } => ({
  connectingPieceId: connection.connecting.piece.id_, connectedPieceId: connection.connected.piece.id_
})

const selectAll = (design: Design): DesignEditorSelection => ({
  selectedPieceIds: design.pieces?.map((p: Piece) => p.id_) || [],
  selectedConnections: design.connections?.map((c: Connection) => ({ connectedPieceId: c.connected.piece.id_, connectingPieceId: c.connecting.piece.id_ })) || [],
  selectedPiecePortId: undefined
})
const deselectAll = (selection: DesignEditorSelection): DesignEditorSelection => ({ selectedPieceIds: [], selectedConnections: [], selectedPiecePortId: undefined })
const addAllPiecesToSelection = (selection: DesignEditorSelection, design: Design): DesignEditorSelection => {
  const existingIds = new Set(selection.selectedPieceIds)
  const allPieceIds = design.pieces?.map((p: Piece) => p.id_) || []
  const newIds = allPieceIds.filter(id => !existingIds.has(id))
  return { selectedPieceIds: [...selection.selectedPieceIds, ...newIds], selectedConnections: selection.selectedConnections, selectedPiecePortId: selection.selectedPiecePortId }
}
const removeAllPiecesFromSelection = (selection: DesignEditorSelection): DesignEditorSelection => ({ selectedPieceIds: [], selectedConnections: selection.selectedConnections, selectedPiecePortId: selection.selectedPiecePortId })
const addAllConnectionsToSelection = (selection: DesignEditorSelection, design: Design): DesignEditorSelection => {
  const allConnections = design.connections?.map((c: Connection) => connectionToSelectionConnection(c)) || []
  const newConnections = allConnections.filter(conn => {
    const connectionId = selectionConnectionToConnectionId(conn)
    return !selection.selectedConnections.some((c) => {
      const existingConnectionId = selectionConnectionToConnectionId(c)
      return isSameConnection(existingConnectionId, connectionId)
    })
  })
  return { selectedPieceIds: selection.selectedPieceIds, selectedConnections: [...selection.selectedConnections, ...newConnections], selectedPiecePortId: selection.selectedPiecePortId }
}
const removeAllConnectionsFromSelection = (selection: DesignEditorSelection): DesignEditorSelection => ({ selectedPieceIds: selection.selectedPieceIds, selectedConnections: [], selectedPiecePortId: selection.selectedPiecePortId })

const selectPiece = (piece: Piece | PieceId): DesignEditorSelection => ({ selectedPieceIds: [typeof piece === 'string' ? piece : piece.id_], selectedConnections: [], selectedPiecePortId: undefined })
const addPieceToSelection = (selection: DesignEditorSelection, piece: Piece | PieceId): DesignEditorSelection => {
  const pieceId = typeof piece === 'string' ? piece : piece.id_
  const existingPieceIds = new Set(selection.selectedPieceIds)
  const newPieceIds = existingPieceIds.has(pieceId) ? selection.selectedPieceIds.filter((id: string) => id !== pieceId) : [...selection.selectedPieceIds, pieceId]
  return ({ selectedPieceIds: newPieceIds, selectedConnections: selection.selectedConnections, selectedPiecePortId: selection.selectedPiecePortId })
}
const removePieceFromSelection = (selection: DesignEditorSelection, piece: Piece | PieceId): DesignEditorSelection => {
  return ({
    selectedPieceIds: selection.selectedPieceIds.filter((id: string) => id !== (typeof piece === 'string' ? piece : piece.id_)),
    selectedConnections: selection.selectedConnections,
    selectedPiecePortId: selection.selectedPiecePortId
  })
}

const selectPieces = (pieces: (Piece | PieceId)[]): DesignEditorSelection => ({ selectedPieceIds: pieces.map(p => typeof p === 'string' ? p : p.id_), selectedConnections: [], selectedPiecePortId: undefined })
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

const selectConnection = (connection: Connection | ConnectionId): DesignEditorSelection => ({ selectedConnections: [connectionToSelectionConnection(connection)], selectedPieceIds: [], selectedPiecePortId: undefined })
const addConnectionToSelection = (selection: DesignEditorSelection, connection: Connection | ConnectionId): DesignEditorSelection => {
  const connectionObj = connectionToSelectionConnection(connection)

  const exists = selection.selectedConnections.some((c) => {
    const existingConnectionId = selectionConnectionToConnectionId(c)
    return isSameConnection(existingConnectionId, connection)
  })

  if (exists) return selection
  return ({ selectedConnections: [...selection.selectedConnections, connectionObj], selectedPieceIds: selection.selectedPieceIds, selectedPiecePortId: selection.selectedPiecePortId })
}
const removeConnectionFromSelection = (selection: DesignEditorSelection, connection: Connection | ConnectionId): DesignEditorSelection => {
  return ({
    selectedConnections: selection.selectedConnections.filter((c: { connectingPieceId: string; connectedPieceId: string }) => {
      const existingConnectionId = selectionConnectionToConnectionId(c)
      return !isSameConnection(existingConnectionId, connection)
    }),
    selectedPieceIds: selection.selectedPieceIds,
    selectedPiecePortId: selection.selectedPiecePortId
  })
}

const selectConnections = (connections: (Connection | ConnectionId)[]): DesignEditorSelection => ({ selectedConnections: connections.map(conn => connectionToSelectionConnection(conn)), selectedPieceIds: [], selectedPiecePortId: undefined })
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
    selectedPieceIds: selection.selectedPieceIds,
    selectedPiecePortId: selection.selectedPiecePortId
  })
}

const selectPiecePort = (pieceId: string, portId: string): DesignEditorSelection => ({ selectedPieceIds: [], selectedConnections: [], selectedPiecePortId: { pieceId, portId } })
const deselectPiecePort = (selection: DesignEditorSelection): DesignEditorSelection => ({ ...selection, selectedPiecePortId: undefined })

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
  return ({ selectedPieceIds: newSelectedPieceIds, selectedConnections: newSelectedConnections, selectedPiecePortId: undefined })
}

const invertPiecesSelection = (selection: DesignEditorSelection, design: Design): DesignEditorSelection => {
  const allPieceIds = design.pieces?.map((p: Piece) => p.id_) || []
  const newSelectedPieceIds = allPieceIds.filter(id => !selection.selectedPieceIds.includes(id))
  return ({ selectedPieceIds: newSelectedPieceIds, selectedConnections: selection.selectedConnections, selectedPiecePortId: selection.selectedPiecePortId })
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
  return ({ selectedPieceIds: selection.selectedPieceIds, selectedConnections: newSelectedConnections, selectedPiecePortId: selection.selectedPiecePortId })
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
const pasteFromClipboard = async (design: Design, plane?: Plane, center?: DiagramPoint): Promise<Design> => {
  try {
    const text = await navigator.clipboard.readText()
    const clipboardDesign = JSON.parse(text)
    return mergeDesigns([design, orientDesign(clipboardDesign, plane, center)])
  } catch (error) {
    console.warn('Failed to paste from clipboard:', error)
    return design
  }
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

//#region DesignDiff Helpers

const updateDesignDiffInState = (state: DesignEditorState, updatedDesignDiff: DesignDiff): DesignEditorState => {
  return { ...state, designDiff: updatedDesignDiff }
}

const resetDesignDiff = (): DesignDiff => ({
  pieces: { added: [], removed: [], updated: [] },
  connections: { added: [], removed: [], updated: [] }
})

//#endregion DesignDiff Helpers


export const DesignEditorContext = createContext<{ state: DesignEditorState; dispatch: DesignEditorDispatcher } | undefined>(undefined);

export const useDesignEditor = () => {
  const context = useContext(DesignEditorContext);
  if (!context) {
    throw new Error('useDesignEditor must be used within a DesignEditorProvider');
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
    executeCommand: useCallback(async (commandId: string, payload: Record<string, any> = {}) => {
      const context: CommandContext = {
        kit: state.kit,
        designId: state.designId,
        selection: state.selection
      }

      // Always run commands in transactions
      startTransaction()
      try {
        const result = await commandRegistry.execute(commandId, context, payload)
        if (result.design) {
          setDesign(result.design)
        }
        if (result.selection) {
          setSelection(result.selection)
        }
        finalizeTransaction()
      } catch (error) {
        abortTransaction()
        console.error('Command execution failed:', error)
        throw error
      }
    }, [state.kit, state.designId, state.selection, startTransaction, setDesign, setSelection, finalizeTransaction, abortTransaction]),

    getAvailableCommands: useCallback(() => commandRegistry.getAll(), []),
    getCommand: useCallback((commandId: string) => commandRegistry.get(commandId), []),
    // Transaction functions
    startTransaction,
    finalizeTransaction,
    abortTransaction
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
    case DesignEditorAction.AddPiece:
      if (state.isTransactionActive) {
        const updatedDesignDiff = addPieceToDesignDiff(state.designDiff, action.payload)
        return updateDesignDiffInState(state, updatedDesignDiff)
      }
      const designWithAddedPiece = addPieceToDesign(currentDesign, action.payload)
      return updateDesignInDesignEditorStateWithOperationStack(designWithAddedPiece)
    case DesignEditorAction.SetPiece:
      if (state.isTransactionActive) {
        const updatedDesignDiff = setPieceInDesignDiff(state.designDiff, { id_: action.payload.id_, ...action.payload })
        return updateDesignDiffInState(state, updatedDesignDiff)
      }
      const designWithSetPiece = setPieceInDesign(currentDesign, action.payload)
      return updateDesignInDesignEditorStateWithOperationStack(designWithSetPiece)
    case DesignEditorAction.RemovePiece:
      if (state.isTransactionActive) {
        const pieceId = typeof action.payload === 'string' ? { id_: action.payload } : { id_: action.payload.id_ }
        const updatedDesignDiff = removePieceFromDesignDiff(state.designDiff, pieceId)
        return updateDesignDiffInState(state, updatedDesignDiff)
      }
      const designWithRemovedPiece = removePieceFromDesign(state.kit, state.designId, action.payload)
      return updateDesignInDesignEditorStateWithOperationStack(designWithRemovedPiece)
    case DesignEditorAction.AddPieces:
      if (state.isTransactionActive) {
        const updatedDesignDiff = addPiecesToDesignDiff(state.designDiff, action.payload)
        return updateDesignDiffInState(state, updatedDesignDiff)
      }
      const designWithAddedPieces = addPiecesToDesign(currentDesign, action.payload)
      return updateDesignInDesignEditorStateWithOperationStack(designWithAddedPieces)
    case DesignEditorAction.SetPieces:
      if (state.isTransactionActive) {
        const pieceDiffs = action.payload.map((piece: Piece) => ({ ...piece, id_: piece.id_ }))
        const updatedDesignDiff = setPiecesInDesignDiff(state.designDiff, pieceDiffs)
        return updateDesignDiffInState(state, updatedDesignDiff)
      }
      const designWithSetPieces = setPiecesInDesign(currentDesign, action.payload)
      return updateDesignInDesignEditorStateWithOperationStack(designWithSetPieces)
    case DesignEditorAction.RemovePieces:
      if (state.isTransactionActive) {
        const pieceIds = action.payload.map((piece: any) =>
          typeof piece === 'string' ? { id_: piece } : { id_: piece.id_ }
        )
        const updatedDesignDiff = removePiecesFromDesignDiff(state.designDiff, pieceIds)
        return updateDesignDiffInState(state, updatedDesignDiff)
      }
      const designWithRemovedPieces = removePiecesFromDesign(state.kit, state.designId, action.payload)
      return updateDesignInDesignEditorStateWithOperationStack(designWithRemovedPieces)
    case DesignEditorAction.AddConnection:
      if (state.isTransactionActive) {
        const updatedDesignDiff = addConnectionToDesignDiff(state.designDiff, action.payload)
        return updateDesignDiffInState(state, updatedDesignDiff)
      }
      const designWithAddedConnection = addConnectionToDesign(currentDesign, action.payload)
      return updateDesignInDesignEditorStateWithOperationStack(designWithAddedConnection)
    case DesignEditorAction.SetConnection:
      if (state.isTransactionActive) {
        const connectionDiff = {
          ...action.payload,
          connected: action.payload.connected,
          connecting: action.payload.connecting
        }
        const updatedDesignDiff = setConnectionInDesignDiff(state.designDiff, connectionDiff)
        return updateDesignDiffInState(state, updatedDesignDiff)
      }
      const designWithSetConnection = setConnectionInDesign(currentDesign, action.payload)
      return updateDesignInDesignEditorStateWithOperationStack(designWithSetConnection)
    case DesignEditorAction.RemoveConnection:
      if (state.isTransactionActive) {
        const connectionId = typeof action.payload === 'object' && 'connected' in action.payload
          ? action.payload
          : { connected: { piece: { id_: action.payload } }, connecting: { piece: { id_: '' } } }
        const updatedDesignDiff = removeConnectionFromDesignDiff(state.designDiff, connectionId)
        return updateDesignDiffInState(state, updatedDesignDiff)
      }
      const designWithRemovedConnection = removeConnectionFromDesign(state.kit, state.designId, action.payload)
      return updateDesignInDesignEditorStateWithOperationStack(designWithRemovedConnection)
    case DesignEditorAction.AddConnections:
      if (state.isTransactionActive) {
        const updatedDesignDiff = addConnectionsToDesignDiff(state.designDiff, action.payload)
        return updateDesignDiffInState(state, updatedDesignDiff)
      }
      const designWithAddedConnections = addConnectionsToDesign(currentDesign, action.payload)
      return updateDesignInDesignEditorStateWithOperationStack(designWithAddedConnections)
    case DesignEditorAction.SetConnections:
      if (state.isTransactionActive) {
        const connectionDiffs = action.payload.map((connection: Connection) => ({
          ...connection,
          connected: connection.connected,
          connecting: connection.connecting
        }))
        const updatedDesignDiff = setConnectionsInDesignDiff(state.designDiff, connectionDiffs)
        return updateDesignDiffInState(state, updatedDesignDiff)
      }
      const designWithSetConnections = setConnectionsInDesign(currentDesign, action.payload)
      return updateDesignInDesignEditorStateWithOperationStack(designWithSetConnections)
    case DesignEditorAction.RemoveConnections:
      if (state.isTransactionActive) {
        const connectionIds = action.payload.map((connection: any) =>
          typeof connection === 'object' && 'connected' in connection
            ? connection
            : { connected: { piece: { id_: connection } }, connecting: { piece: { id_: '' } } }
        )
        const updatedDesignDiff = removeConnectionsFromDesignDiff(state.designDiff, connectionIds)
        return updateDesignDiffInState(state, updatedDesignDiff)
      }
      const designWithRemovedConnections = removeConnectionsFromDesign(state.kit, state.designId, action.payload)
      return updateDesignInDesignEditorStateWithOperationStack(designWithRemovedConnections)
    case DesignEditorAction.RemovePiecesAndConnections:
      if (state.isTransactionActive) {
        // Handle pieces
        const pieceIds = action.payload.pieceIds.map((piece: any) =>
          typeof piece === 'string' ? { id_: piece } : { id_: piece.id_ }
        )
        let updatedDesignDiff = removePiecesFromDesignDiff(state.designDiff, pieceIds)

        // Handle connections
        const connectionIds = action.payload.connectionIds.map((connection: any) =>
          typeof connection === 'object' && 'connected' in connection
            ? connection
            : { connected: { piece: { id_: connection } }, connecting: { piece: { id_: '' } } }
        )
        updatedDesignDiff = removeConnectionsFromDesignDiff(updatedDesignDiff, connectionIds)

        return updateDesignDiffInState(state, updatedDesignDiff)
      }
      const designWithRemovedPiecesAndConnections = removePiecesAndConnectionsFromDesign(state.kit, state.designId, action.payload.pieceIds, action.payload.connectionIds)
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
      return { ...state, isTransactionActive: true, designDiff: resetDesignDiff() }
    case DesignEditorAction.FinalizeTransaction:
      if (!state.isTransactionActive) return state
      // Apply the accumulated diff to the design and push to operation stack
      const finalDesign = applyDesignDiff(currentDesign, state.designDiff)
      const stateWithFinalizedTransaction = pushToOperationStack({ ...state, isTransactionActive: false, designDiff: resetDesignDiff() })
      const updatedDesigns = (stateWithFinalizedTransaction.kit.designs || []).map((d: Design) =>
        isSameDesign(d, currentDesign) ? finalDesign : d
      )
      const entryWithCorrectTransactionState = stateWithFinalizedTransaction.operationStack[stateWithFinalizedTransaction.operationIndex]
      entryWithCorrectTransactionState.selection = state.selection
      return { ...stateWithFinalizedTransaction, kit: { ...stateWithFinalizedTransaction.kit, designs: updatedDesigns } }
    case DesignEditorAction.AbortTransaction:
      if (!state.isTransactionActive) return state
      // Simply reset transaction state and discard the diff
      return { ...state, isTransactionActive: false, designDiff: resetDesignDiff() }

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
    case DesignEditorAction.SelectPiecePort:
      return { ...state, selection: selectPiecePort(action.payload.pieceId, action.payload.portId) }
    case DesignEditorAction.DeselectPiecePort:
      return { ...state, selection: deselectPiecePort(state.selection) }

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
    selection: initialSelection || { selectedPieceIds: [], selectedConnections: [], selectedPiecePortId: undefined },
    designDiff: {
      pieces: { added: [], removed: [], updated: [] },
      connections: { added: [], removed: [], updated: [] }
    },
    operationStack: [{
      design: JSON.parse(JSON.stringify(initialDesign)),
      selection: JSON.parse(JSON.stringify(initialSelection || { selectedPieceIds: [], selectedConnections: [], selectedPiecePortId: undefined }))
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
                <TreeItem key={name} label={name} defaultOpen={false}>
                  <div className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1">
                    {variants.map((type) => (
                      <TypeAvatar key={`${type.name}-${type.variant}`} type={type} showHoverCard={true} />
                    ))}
                  </div>
                </TreeItem>
              ))}
            </TreeSection>
            <TreeSection label="Designs" defaultOpen={true}>
              {Object.entries(designsByName).map(([name, designs]) => (
                <TreeItem key={name} label={name} defaultOpen={false}>
                  <div className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1">
                    {designs.map((design) => (
                      <DesignAvatar
                        key={`${design.name}-${design.variant}-${design.view}`}
                        design={design}
                        showHoverCard={true}
                      />
                    ))}
                  </div>
                </TreeItem>
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

interface ConsoleCommand {
  id: string
  name: string
  description: string
  icon?: string
  execute: (args: any) => Promise<any>
}

interface DetailsProps extends ResizablePanelProps { }

const DesignSection: FC = () => {
  const { kit, designId, setDesign, startTransaction, finalizeTransaction, abortTransaction } = useDesignEditor()
  const design = findDesignInKit(kit, designId)

  const handleChange = (updatedDesign: Design) => {
    setDesign(updatedDesign)
  }

  const addLocation = () => {
    startTransaction()
    handleChange({ ...design, location: { longitude: 0, latitude: 0 } })
    finalizeTransaction()
  }

  const removeLocation = () => {
    startTransaction()
    handleChange({ ...design, location: undefined })
    finalizeTransaction()
  }

  return (
    <>
      <TreeSection label="Design" defaultOpen={true}>
        <TreeItem>
          <Input
            label="Name"
            value={design.name}
            onChange={(e) => handleChange({ ...design, name: e.target.value })}
            onFocus={startTransaction}
            onBlur={finalizeTransaction}
          />
        </TreeItem>
        <TreeItem>
          <Textarea
            label="Description"
            value={design.description || ''}
            placeholder="Enter design description..."
            onChange={(e) => handleChange({ ...design, description: e.target.value })}
            onFocus={startTransaction}
            onBlur={finalizeTransaction}
          />
        </TreeItem>
        <TreeItem>
          <Input
            label="Icon"
            value={design.icon || ''}
            placeholder="Emoji, name, or URL"
            onChange={(e) => handleChange({ ...design, icon: e.target.value })}
            onFocus={startTransaction}
            onBlur={finalizeTransaction}
          />
        </TreeItem>
        <TreeItem>
          <Input
            label="Image URL"
            value={design.image || ''}
            placeholder="URL to design image"
            onChange={(e) => handleChange({ ...design, image: e.target.value })}
            onFocus={startTransaction}
            onBlur={finalizeTransaction}
          />
        </TreeItem>
        <TreeItem>
          <Input
            label="Variant"
            value={design.variant || ''}
            placeholder="Design variant"
            onChange={(e) => handleChange({ ...design, variant: e.target.value })}
            onFocus={startTransaction}
            onBlur={finalizeTransaction}
          />
        </TreeItem>
        <TreeItem>
          <Input
            label="View"
            value={design.view || ''}
            placeholder="Design view"
            onChange={(e) => handleChange({ ...design, view: e.target.value })}
            onFocus={startTransaction}
            onBlur={finalizeTransaction}
          />
        </TreeItem>
        <TreeItem>
          <Input
            label="Unit"
            value={design.unit}
            onChange={(e) => handleChange({ ...design, unit: e.target.value })}
            onFocus={startTransaction}
            onBlur={finalizeTransaction}
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
        <TreeSection
          label="Authors"
          actions={[
            {
              icon: <Plus size={12} />,
              onClick: () => {
                startTransaction()
                handleChange({
                  ...design,
                  authors: [...(design.authors || []), { name: '', email: '' }]
                })
                finalizeTransaction()
              },
              title: "Add author"
            }
          ]}
        >
          <SortableTreeItems
            items={design.authors.map((author, index) => ({ ...author, id: `author-${index}`, index }))}
            onReorder={(oldIndex, newIndex) => {
              startTransaction()
              handleChange({
                ...design,
                authors: arrayMove(design.authors!, oldIndex, newIndex)
              })
              finalizeTransaction()
            }}
          >
            {(author, index) => (
              <TreeItem
                key={`author-${index}`}
                label={author.name || `Author ${index + 1}`}
                sortable={true}
                sortableId={`author-${index}`}
                isDragHandle={true}
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
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
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
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
                  />
                </TreeItem>
                <TreeItem>
                  <button
                    onClick={() => {
                      startTransaction()
                      handleChange({
                        ...design,
                        authors: design.authors?.filter((_, i) => i !== index)
                      })
                      finalizeTransaction()
                    }}
                    className="text-destructive hover:text-destructive/80 p-1"
                    title="Remove author"
                  >
                    <Trash2 size={12} />
                  </button>
                </TreeItem>
              </TreeItem>
            )}
          </SortableTreeItems>
        </TreeSection>
      ) : (
        <TreeSection
          label="Authors"
          actions={[
            {
              icon: <Plus size={12} />,
              onClick: () => {
                startTransaction()
                handleChange({
                  ...design,
                  authors: [...(design.authors || []), { name: '', email: '' }]
                })
                finalizeTransaction()
              },
              title: "Add author"
            }
          ]}
        >
        </TreeSection>
      )}
      {design.qualities && design.qualities.length > 0 ? (
        <TreeSection
          label="Qualities"
          actions={[
            {
              icon: <Plus size={12} />,
              onClick: () => {
                startTransaction()
                handleChange({
                  ...design,
                  qualities: [...(design.qualities || []), { name: '' }]
                })
                finalizeTransaction()
              },
              title: "Add quality"
            }
          ]}
        >
          <SortableTreeItems
            items={design.qualities.map((quality, index) => ({ ...quality, id: `quality-${index}`, index }))}
            onReorder={(oldIndex, newIndex) => {
              startTransaction()
              handleChange({
                ...design,
                qualities: arrayMove(design.qualities!, oldIndex, newIndex)
              })
              finalizeTransaction()
            }}
          >
            {(quality, index) => (
              <TreeItem
                key={`quality-${index}`}
                label={quality.name || `Quality ${index + 1}`}
                sortable={true}
                sortableId={`quality-${index}`}
                isDragHandle={true}
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
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
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
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
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
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
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
                    onFocus={startTransaction}
                    onBlur={finalizeTransaction}
                  />
                </TreeItem>
                <TreeItem>
                  <button
                    onClick={() => {
                      startTransaction()
                      handleChange({
                        ...design,
                        qualities: design.qualities?.filter((_, i) => i !== index)
                      })
                      finalizeTransaction()
                    }}
                    className="text-destructive hover:text-destructive/80 p-1"
                    title="Remove quality"
                  >
                    <Trash2 size={12} />
                  </button>
                </TreeItem>
              </TreeItem>
            )}
          </SortableTreeItems>
        </TreeSection>
      ) : (
        <TreeSection
          label="Qualities"
          actions={[
            {
              icon: <Plus size={12} />,
              onClick: () => {
                startTransaction()
                handleChange({
                  ...design,
                  qualities: [...(design.qualities || []), { name: '' }]
                })
                finalizeTransaction()
              },
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
  const { kit, designId, setPiece, setPieces, setConnection, startTransaction, finalizeTransaction, abortTransaction, executeCommand } = useDesignEditor()
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
    startTransaction()
    if (isSingle) {
      setPiece({ ...piece!, type: { ...piece!.type, name: value } })
    } else {
      const updatedPieces = pieces.map(piece => ({ ...piece, type: { ...piece.type, name: value } }))
      setPieces(updatedPieces)
    }
    finalizeTransaction()
  }

  const handleTypeVariantChange = (value: string) => {
    startTransaction()
    if (isSingle) {
      setPiece({ ...piece!, type: { ...piece!.type, variant: value } })
    } else {
      const updatedPieces = pieces.map(piece => ({ ...piece, type: { ...piece.type, variant: value } }))
      setPieces(updatedPieces)
    }
    finalizeTransaction()
  }

  const fixPieces = async () => {
    try {
      await executeCommand('fix-selected-pieces')
    } catch (error) {
      console.error('Failed to fix pieces:', error)
    }
  }

  const handleCenterXChange = (value: number) => {
    if (isSingle) {
      setPiece(piece!.center ? { ...piece!, center: { ...piece!.center, x: value } } : piece!)
    } else {
      const updatedPieces = pieces.map(piece =>
        piece.center ? { ...piece, center: { ...piece.center, x: value } } : piece
      )
      setPieces(updatedPieces)
    }
  }

  const handleCenterYChange = (value: number) => {
    if (isSingle) {
      setPiece(piece!.center ? { ...piece!, center: { ...piece!.center, y: value } } : piece!)
    } else {
      const updatedPieces = pieces.map(piece =>
        piece.center ? { ...piece, center: { ...piece.center, y: value } } : piece
      )
      setPieces(updatedPieces)
    }
  }

  const handlePlaneOriginXChange = (value: number) => {
    if (isSingle) {
      setPiece(piece!.plane ? { ...piece!, plane: { ...piece!.plane, origin: { ...piece!.plane.origin, x: value } } } : piece!)
    } else {
      const updatedPieces = pieces.map(piece =>
        piece.plane ? { ...piece, plane: { ...piece.plane, origin: { ...piece.plane.origin, x: value } } } : piece
      )
      setPieces(updatedPieces)
    }
  }

  const handlePlaneOriginYChange = (value: number) => {
    if (isSingle) {
      setPiece(piece!.plane ? { ...piece!, plane: { ...piece!.plane, origin: { ...piece!.plane.origin, y: value } } } : piece!)
    } else {
      const updatedPieces = pieces.map(piece =>
        piece.plane ? { ...piece, plane: { ...piece.plane, origin: { ...piece.plane.origin, y: value } } } : piece
      )
      setPieces(updatedPieces)
    }
  }

  const handlePlaneOriginZChange = (value: number) => {
    if (isSingle) {
      setPiece(piece!.plane ? { ...piece!, plane: { ...piece!.plane, origin: { ...piece!.plane.origin, z: value } } } : piece!)
    } else {
      const updatedPieces = pieces.map(piece =>
        piece.plane ? { ...piece, plane: { ...piece.plane, origin: { ...piece.plane.origin, z: value } } } : piece
      )
      setPieces(updatedPieces)
    }
  }

  const handleConnectionChange = (updatedConnection: Connection) => {
    setConnection(updatedConnection)
  }

  const handleMultipleConnectionsChange = (propertyName: keyof Connection, value: any) => {
    const updatedConnections = parentConnections.map(conn => ({
      ...conn,
      [propertyName]: value
    }))
    updatedConnections.forEach(conn => setConnection(conn))
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
            label="Type"
            options={availableTypeNames.map(name => ({ value: name, label: name }))}
            value={isSingle ? piece!.type.name : (commonTypeName || '')}
            placeholder={!isSingle && commonTypeName === undefined ? 'Mixed values' : 'Select type'}
            onValueChange={handleTypeNameChange}
          />
        </TreeItem>
        {(hasVariant || availableVariants.length > 0) && (
          <TreeItem>
            <Combobox
              label="Variant"
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
  const { kit, designId, setConnection, setConnections, startTransaction, finalizeTransaction, abortTransaction } = useDesignEditor()
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

  const handleChange = (updatedConnection: Connection) => setConnection(updatedConnection)

  const handleGapChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, gap: value })
    else setConnections(connectionObjects.map(connection => ({ ...connection, gap: value })))
  }

  const handleShiftChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, shift: value })
    else setConnections(connectionObjects.map(connection => ({ ...connection, shift: value })))
  }

  const handleRiseChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, rise: value })
    else setConnections(connectionObjects.map(connection => ({ ...connection, rise: value })))
  }

  const handleXOffsetChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, x: value })
    else setConnections(connectionObjects.map(connection => ({ ...connection, x: value })))
  }

  const handleYOffsetChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, y: value })
    else setConnections(connectionObjects.map(connection => ({ ...connection, y: value })))
  }

  const handleRotationChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, rotation: value })
    else setConnections(connectionObjects.map(connection => ({ ...connection, rotation: value })))
  }

  const handleTurnChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, turn: value })
    else setConnections(connectionObjects.map(connection => ({ ...connection, turn: value })))
  }

  const handleTiltChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, tilt: value })
    else setConnections(connectionObjects.map(connection => ({ ...connection, tilt: value })))
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
                onFocus={startTransaction}
                onBlur={finalizeTransaction}
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
                onFocus={startTransaction}
                onBlur={finalizeTransaction}
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

const PortSection: FC<{ pieceId: string; portId: string }> = ({ pieceId, portId }) => {
  const { kit, designId } = useDesignEditor()
  const design = findDesignInKit(kit, designId)
  const piece = design?.pieces?.find(p => p.id_ === pieceId)
  const type = piece ? findTypeInKit(kit, piece.type) : null
  const port = type?.ports?.find((p: any) => p.id_ === portId)

  if (!piece || !type || !port) {
    return (
      <TreeSection label="Port" defaultOpen={true}>
        <TreeItem>
          <p className="text-sm text-muted-foreground">Port not found</p>
        </TreeItem>
      </TreeSection>
    )
  }

  return (
    <TreeSection label="Port" defaultOpen={true}>
      <Input label="ID" value={port.id_ || '~default~'} disabled />
      {port.description && (
        <Textarea label="Description" value={port.description} disabled />
      )}
      {port.family && (
        <Input label="Family" value={port.family} disabled />
      )}
      {port.mandatory !== undefined && (
        <Input label="Mandatory" value={port.mandatory ? 'Yes' : 'No'} disabled />
      )}
      <Input label="Position" value={`(${port.point.x.toFixed(2)}, ${port.point.y.toFixed(2)}, ${port.point.z.toFixed(2)})`} disabled />
      <Input label="Direction" value={`(${port.direction.x.toFixed(2)}, ${port.direction.y.toFixed(2)}, ${port.direction.z.toFixed(2)})`} disabled />
      {port.compatibleFamilies && port.compatibleFamilies.map((family: string) => <TreeItem><Input label="Compatible Families" value={family} disabled /></TreeItem>)}
      {port.qualities && port.qualities.map((quality: any) => <TreeItem><Input label="Qualities" value={`${quality.name}: ${quality.value || 'N/A'} ${quality.unit && `(${quality.unit})`}`} disabled /></TreeItem>)}
    </TreeSection >
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
  const hasPortSelected = selection.selectedPiecePortId !== undefined
  const hasSelection = hasPieces || hasConnections || hasPortSelected

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
            {hasPortSelected && <PortSection pieceId={selection.selectedPiecePortId!.pieceId} portId={selection.selectedPiecePortId!.portId} />}
            {hasPieces && !hasPortSelected && <PiecesSection pieceIds={selection.selectedPieceIds} />}
            {hasConnections && !hasPortSelected && <ConnectionsSection connections={selection.selectedConnections} />}
            {hasPieces && hasConnections && !hasPortSelected && (
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




//#region Main Component

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

const DesignEditorCore: FC<DesignEditorProps> = (props) => {
  const { onToolbarChange, designId, onUndo: controlledOnUndo, onRedo: controlledOnRedo } = props

  const [state, dispatch] = useControllableReducer(props)
  if (!state.kit) return null
  const design = findDesignInKit(state.kit, designId)
  if (!design) return null

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

  const toggleWorkbench = useCallback(() => togglePanel('workbench'), [])
  const toggleConsole = useCallback(() => togglePanel('console'), [])
  const toggleDetails = useCallback(() => togglePanel('details'), [])
  const toggleChat = useCallback(() => togglePanel('chat'), [])

  const onUndo = controlledOnUndo || (() => dispatch({ type: DesignEditorAction.Undo, payload: null }))
  const onRedo = controlledOnRedo || (() => dispatch({ type: DesignEditorAction.Redo, payload: null }))

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


  // Register built-in commands
  useEffect(() => {
    // Register all centralized commands
    const unregisterFunctions = designEditorCommands.map(command =>
      commandRegistry.register(command)
    )

    return () => unregisterFunctions.forEach(fn => fn())
  }, [commandRegistry])

  // Global command event handler
  useEffect(() => {
    const handleCommand = async (event: Event) => {
      const customEvent = event as CustomEvent
      const { commandId, payload = {} } = customEvent.detail

      const context = { kit: state.kit, designId, selection: state.selection }
      dispatch({ type: DesignEditorAction.StartTransaction, payload: null })
      try {
        const result = await commandRegistry.execute(commandId, context, payload)
        if (result.design) dispatch({ type: DesignEditorAction.SetDesign, payload: result.design })
        if (result.selection) dispatch({ type: DesignEditorAction.SetSelection, payload: result.selection })
        dispatch({ type: DesignEditorAction.FinalizeTransaction, payload: null })
      } catch (error) {
        dispatch({ type: DesignEditorAction.AbortTransaction, payload: null })
        console.error(`Error executing command ${commandId}:`, error)
      }
    }

    document.addEventListener('semio-command', handleCommand)
    return () => document.removeEventListener('semio-command', handleCommand)
  }, [state.kit, designId, state.selection, dispatch])

  // Register hotkeys for all commands automatically
  designEditorCommands.forEach(command => {
    if (command.hotkey) {
      useHotkeys(command.hotkey, (e) => {
        e.preventDefault()
        e.stopPropagation()
        const event = new CustomEvent('semio-command', { detail: { commandId: command.id } })
        document.dispatchEvent(event)
      }, { enableOnContentEditable: false })
    }
  })

  useHotkeys('mod+x', (e) => {
    e.preventDefault()
    e.stopPropagation()
    const event = new CustomEvent('semio-command', { detail: { commandId: 'cut-to-clipboard' } })
    document.dispatchEvent(event)
  })

  // Keep special hotkeys for undo/redo since they're handled differently
  useHotkeys('mod+z', async (e) => {
    e.preventDefault()
    e.stopPropagation()
    onUndo()
  })

  useHotkeys('mod+y', async (e) => {
    e.preventDefault()
    e.stopPropagation()
    onRedo()
  })

  useHotkeys('mod+shift+d', async (e) => {
    e.preventDefault()
    e.stopPropagation()
    const context = { kit: state.kit, designId, selection: state.selection }
    try {
      await commandRegistry.execute('select-all-similar-pieces', context, {})
    } catch (error) {
      console.error('Error selecting all similar pieces:', error)
    }
  })

  const onDragStart = (event: DragStartEvent) => {
    const { active } = event
    const id = active.id.toString()
    if (id.startsWith('type-')) {
      const [_, name, variant] = id.split('-')
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
          type: DesignEditorAction.AddPiece,
          payload: piece
        })
      } else if (activeDraggedDesign) {
        throw new Error('Not implemented')
      }
    }
    setActiveDraggedType(null)
    setActiveDraggedDesign(null)
  }

  // Panel hotkeys (not commands)
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

  const rightPanelVisible = visiblePanels.details || visiblePanels.chat

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
          <div
            className={`absolute z-30 bg-slate-900 text-green-400 border border-slate-700 ${visiblePanels.console ? '' : 'hidden'}`}
            style={{
              left: visiblePanels.workbench ? `${workbenchWidth + 32}px` : '16px',
              right: rightPanelVisible ? `${detailsWidth + 32}px` : '16px',
              bottom: '16px',
              height: `${consoleHeight}px`
            }}
          >
            <div
              className="absolute top-0 left-0 right-0 h-1 cursor-ns-resize bg-slate-700 hover:bg-blue-500"
              onMouseDown={(e) => {
                e.preventDefault()
                const startY = e.clientY
                const startHeight = consoleHeight

                const handleMouseMove = (e: MouseEvent) => {
                  const deltaY = startY - e.clientY
                  const newHeight = Math.max(200, Math.min(800, startHeight + deltaY))
                  setConsoleHeight(newHeight)
                }

                const handleMouseUp = () => {
                  document.removeEventListener('mousemove', handleMouseMove)
                  document.removeEventListener('mouseup', handleMouseUp)
                }

                document.addEventListener('mousemove', handleMouseMove)
                document.addEventListener('mouseup', handleMouseUp)
              }}
            />
            <div className="h-full pt-1">
              <Console />
            </div>
          </div>
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
  onRedo,
  onToolbarChange
}) => {
  const [toolbarContent, setToolbarContent] = useState<ReactNode>(null)

  useEffect(() => {
    onToolbarChange?.(toolbarContent)
  }, [toolbarContent, onToolbarChange])

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
          mode={mode}
          layout={layout}
          theme={theme}
          setLayout={setLayout}
          setTheme={setTheme}
          onWindowEvents={onWindowEvents}
        />
      </ReactFlowProvider>
    </div>
  )
}

export default DesignEditor

//#endregion
