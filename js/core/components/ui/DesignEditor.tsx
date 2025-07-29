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
import { ReactFlowProvider, useReactFlow } from '@xyflow/react'
import { Info, MessageCircle, Terminal, Wrench } from 'lucide-react'
import { FC, ReactNode, createContext, useCallback, useContext, useEffect, useReducer, useState } from 'react'
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
  addConnectionToDesignDiff,
  addConnectionsToDesign,
  addConnectionsToDesignDiff,
  addPieceToDesign,
  addPieceToDesignDiff,
  addPiecesToDesign,
  addPiecesToDesignDiff,
  applyDesignDiff,
  findConnectionInDesign,
  findDesignInKit,
  findPieceInDesign,
  findReplacableTypesForPieceInDesign,
  findReplacableTypesForPiecesInDesign,
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
import { Button } from '@semio/js/components/ui/Button'
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
import { Tree, TreeItem, TreeNode, TreeSection } from '@semio/js/components/ui/Tree'
import { Generator } from '@semio/js/lib/utils'
import { orientDesign } from '../../semio'

//#region State

export interface DesignEditorSelection {
  selectedPieceIds: string[];
  selectedConnections: {
    connectingPieceId: string;
    connectedPieceId: string;
  }[];
}

export interface CommandStackEntry {
  design: Design
  selection: DesignEditorSelection
  designDiff: DesignDiff
}

export interface DesignEditorState {
  kit: Kit
  designId: DesignId
  fileUrls: Map<string, string>
  fullscreenPanel: FullscreenPanel
  selection: DesignEditorSelection
  designDiff: DesignDiff
  commandStack: CommandStackEntry[]
  commandIndex: number
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
  SetDesignDiff = 'SET_DESIGN_DIFF',
  AddPieceToDesignDiff = 'ADD_PIECE_TO_DESIGN_DIFF',
  SetPieceInDesignDiff = 'SET_PIECE_IN_DESIGN_DIFF',
  RemovePieceFromDesignDiff = 'REMOVE_PIECE_FROM_DESIGN_DIFF',
  AddPiecesToDesignDiff = 'ADD_PIECES_TO_DESIGN_DIFF',
  SetPiecesInDesignDiff = 'SET_PIECES_IN_DESIGN_DIFF',
  RemovePiecesFromDesignDiff = 'REMOVE_PIECES_FROM_DESIGN_DIFF',
  AddConnectionToDesignDiff = 'ADD_CONNECTION_TO_DESIGN_DIFF',
  SetConnectionInDesignDiff = 'SET_CONNECTION_IN_DESIGN_DIFF',
  RemoveConnectionFromDesignDiff = 'REMOVE_CONNECTION_FROM_DESIGN_DIFF',
  AddConnectionsToDesignDiff = 'ADD_CONNECTIONS_TO_DESIGN_DIFF',
  SetConnectionsInDesignDiff = 'SET_CONNECTIONS_IN_DESIGN_DIFF',
  RemoveConnectionsFromDesignDiff = 'REMOVE_CONNECTIONS_FROM_DESIGN_DIFF',
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
  CopyToClipboard = 'COPY_TO_CLIPBOARD',
  CutToClipboard = 'CUT_TO_CLIPBOARD',
  PasteFromClipboard = 'PASTE_FROM_CLIPBOARD',
  DeleteSelected = 'DELETE_SELECTED',
  SetFullscreen = 'SET_FULLSCREEN',
  Undo = 'UNDO',
  Redo = 'REDO',
  ToggleDiagramFullscreen = 'TOGGLE_DIAGRAM_FULLSCREEN',
  ToggleModelFullscreen = 'TOGGLE_MODEL_FULLSCREEN'
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

//#region Command Stack

const pushToCommandStack = (state: DesignEditorState): DesignEditorState => {
  const currentDesign = findDesignInKit(state.kit, state.designId)
  const newEntry: CommandStackEntry = {
    design: JSON.parse(JSON.stringify(currentDesign)),
    selection: JSON.parse(JSON.stringify(state.selection)),
    designDiff: JSON.parse(JSON.stringify(state.designDiff))
  }

  let newCommandStack: CommandStackEntry[]
  let newCommandIndex: number

  if (state.commandIndex < state.commandStack.length - 1) {
    newCommandStack = state.commandStack.slice(0, state.commandIndex + 1)
  } else {
    newCommandStack = [...state.commandStack]
  }

  newCommandStack.push(newEntry)

  if (newCommandStack.length > COMMAND_STACK_MAX) {
    newCommandStack.shift()
    newCommandIndex = newCommandStack.length - 1
  } else {
    newCommandIndex = newCommandStack.length - 1
  }

  return {
    ...state,
    commandStack: newCommandStack,
    commandIndex: newCommandIndex
  }
}

const canUndo = (state: DesignEditorState): boolean => {
  if (state.commandStack.length === 0) return false
  return state.commandIndex > 0 && state.commandStack.length > 1
}

const canRedo = (state: DesignEditorState): boolean => {
  if (state.commandStack.length === 0) return false
  return state.commandIndex < state.commandStack.length - 1
}

const undo = (state: DesignEditorState): DesignEditorState => {
  if (!canUndo(state)) return state

  const previousEntry = state.commandStack[state.commandIndex - 1]
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
    designDiff: previousEntry.designDiff,
    commandIndex: state.commandIndex - 1
  }
}

const redo = (state: DesignEditorState): DesignEditorState => {
  if (!canRedo(state)) return state

  const nextEntry = state.commandStack[state.commandIndex + 1]
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
    designDiff: nextEntry.designDiff,
    commandIndex: state.commandIndex + 1
  }
}

//#endregion Command Stack


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
  const setDesignDiff = useCallback((d: DesignDiff) => dispatch({ type: DesignEditorAction.SetDesignDiff, payload: d }), [dispatch]);
  const addPieceToDesignDiff = useCallback((p: Piece) => dispatch({ type: DesignEditorAction.AddPieceToDesignDiff, payload: p }), [dispatch]);
  const setPieceInDesignDiff = useCallback((p: Piece) => dispatch({ type: DesignEditorAction.SetPieceInDesignDiff, payload: p }), [dispatch]);
  const removePieceFromDesignDiff = useCallback((p: Piece) => dispatch({ type: DesignEditorAction.RemovePieceFromDesignDiff, payload: p }), [dispatch]);
  const addPiecesToDesignDiff = useCallback((ps: Piece[]) => dispatch({ type: DesignEditorAction.AddPiecesToDesignDiff, payload: ps }), [dispatch]);
  const setPiecesInDesignDiff = useCallback((ps: Piece[]) => dispatch({ type: DesignEditorAction.SetPiecesInDesignDiff, payload: ps }), [dispatch]);
  const removePiecesFromDesignDiff = useCallback((ps: Piece[]) => dispatch({ type: DesignEditorAction.RemovePiecesFromDesignDiff, payload: ps }), [dispatch]);
  const addConnectionToDesignDiff = useCallback((c: Connection) => dispatch({ type: DesignEditorAction.AddConnectionToDesignDiff, payload: c }), [dispatch]);
  const setConnectionInDesignDiff = useCallback((c: Connection) => dispatch({ type: DesignEditorAction.SetConnectionInDesignDiff, payload: c }), [dispatch]);
  const removeConnectionFromDesignDiff = useCallback((c: Connection) => dispatch({ type: DesignEditorAction.RemoveConnectionFromDesignDiff, payload: c }), [dispatch]);
  const addConnectionsToDesignDiff = useCallback((cs: Connection[]) => dispatch({ type: DesignEditorAction.AddConnectionsToDesignDiff, payload: cs }), [dispatch]);
  const setConnectionsInDesignDiff = useCallback((cs: Connection[]) => dispatch({ type: DesignEditorAction.SetConnectionsInDesignDiff, payload: cs }), [dispatch]);
  const removeConnectionsFromDesignDiff = useCallback((cs: Connection[]) => dispatch({ type: DesignEditorAction.RemoveConnectionsFromDesignDiff, payload: cs }), [dispatch]);
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
    setDesignDiff,
    addPieceToDesignDiff,
    setPieceInDesignDiff,
    removePieceFromDesignDiff,
    addPiecesToDesignDiff,
    setPiecesInDesignDiff,
    removePiecesFromDesignDiff,
    addConnectionToDesignDiff,
    setConnectionInDesignDiff,
    removeConnectionFromDesignDiff,
    addConnectionsToDesignDiff,
    setConnectionsInDesignDiff,
    removeConnectionsFromDesignDiff,
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
    redo
  };
};

const designEditorReducer = (
  state: DesignEditorState,
  action: { type: DesignEditorAction; payload: any }
): DesignEditorState => {
  const currentDesign = findDesignInKit(state.kit, state.designId)

  const updateDesignInDesignEditorStateWithCommandStack = (updatedDesign: Design): DesignEditorState => {
    const stateWithCommand = pushToCommandStack(state)
    const updatedDesigns = (stateWithCommand.kit.designs || []).map((d: Design) => isSameDesign(d, currentDesign) ? updatedDesign : d)
    return { ...stateWithCommand, kit: { ...stateWithCommand.kit, designs: updatedDesigns } }
  }

  switch (action.type) {
    // Design changes that should push to command stack
    case DesignEditorAction.SetDesign:
      return updateDesignInDesignEditorStateWithCommandStack(action.payload)
    case DesignEditorAction.AddPieceToDesign:
      return updateDesignInDesignEditorStateWithCommandStack(addPieceToDesign(currentDesign, action.payload))
    case DesignEditorAction.SetPieceInDesign:
      return updateDesignInDesignEditorStateWithCommandStack(setPieceInDesign(currentDesign, action.payload))
    case DesignEditorAction.RemovePieceFromDesign:
      return updateDesignInDesignEditorStateWithCommandStack(removePieceFromDesign(state.kit, state.designId, action.payload))
    case DesignEditorAction.AddPiecesToDesign:
      return updateDesignInDesignEditorStateWithCommandStack(addPiecesToDesign(currentDesign, action.payload))
    case DesignEditorAction.SetPiecesInDesign:
      return updateDesignInDesignEditorStateWithCommandStack(setPiecesInDesign(currentDesign, action.payload))
    case DesignEditorAction.RemovePiecesFromDesign:
      return updateDesignInDesignEditorStateWithCommandStack(removePiecesFromDesign(state.kit, state.designId, action.payload))
    case DesignEditorAction.AddConnectionToDesign:
      return updateDesignInDesignEditorStateWithCommandStack(addConnectionToDesign(currentDesign, action.payload))
    case DesignEditorAction.SetConnectionInDesign:
      return updateDesignInDesignEditorStateWithCommandStack(setConnectionInDesign(currentDesign, action.payload))
    case DesignEditorAction.RemoveConnectionFromDesign:
      return updateDesignInDesignEditorStateWithCommandStack(removeConnectionFromDesign(state.kit, state.designId, action.payload))
    case DesignEditorAction.AddConnectionsToDesign:
      return updateDesignInDesignEditorStateWithCommandStack(addConnectionsToDesign(currentDesign, action.payload))
    case DesignEditorAction.SetConnectionsInDesign:
      return updateDesignInDesignEditorStateWithCommandStack(setConnectionsInDesign(currentDesign, action.payload))
    case DesignEditorAction.RemoveConnectionsFromDesign:
      return updateDesignInDesignEditorStateWithCommandStack(removeConnectionsFromDesign(state.kit, state.designId, action.payload))
    case DesignEditorAction.RemovePiecesAndConnectionsFromDesign:
      return updateDesignInDesignEditorStateWithCommandStack(removePiecesAndConnectionsFromDesign(state.kit, state.designId, action.payload.pieceIds, action.payload.connectionIds))
    case DesignEditorAction.DeleteSelected:
      const stateWithDeleteCommand = pushToCommandStack(state)
      const updatedDesign = deleteSelected(stateWithDeleteCommand.kit, stateWithDeleteCommand.designId, stateWithDeleteCommand.selection)
      return { ...stateWithDeleteCommand, kit: updateDesignInKit(stateWithDeleteCommand.kit, updatedDesign), selection: deselectAll(stateWithDeleteCommand.selection) }

    // DesignDiff changes (no command stack)
    case DesignEditorAction.SetDesignDiff:
      return { ...state, designDiff: action.payload }
    case DesignEditorAction.AddPieceToDesignDiff:
      return { ...state, designDiff: addPieceToDesignDiff(state.designDiff, action.payload) }
    case DesignEditorAction.SetPieceInDesignDiff:
      return { ...state, designDiff: setPieceInDesignDiff(state.designDiff, action.payload) }
    case DesignEditorAction.RemovePieceFromDesignDiff:
      return { ...state, designDiff: removePieceFromDesignDiff(state.designDiff, action.payload) }
    case DesignEditorAction.AddPiecesToDesignDiff:
      return { ...state, designDiff: addPiecesToDesignDiff(state.designDiff, action.payload) }
    case DesignEditorAction.SetPiecesInDesignDiff:
      return { ...state, designDiff: setPiecesInDesignDiff(state.designDiff, action.payload) }
    case DesignEditorAction.RemovePiecesFromDesignDiff:
      return { ...state, designDiff: removePiecesFromDesignDiff(state.designDiff, action.payload) }
    case DesignEditorAction.AddConnectionToDesignDiff:
      return { ...state, designDiff: addConnectionToDesignDiff(state.designDiff, action.payload) }
    case DesignEditorAction.SetConnectionInDesignDiff:
      return { ...state, designDiff: setConnectionInDesignDiff(state.designDiff, action.payload) }
    case DesignEditorAction.RemoveConnectionFromDesignDiff:
      return { ...state, designDiff: removeConnectionFromDesignDiff(state.designDiff, action.payload) }
    case DesignEditorAction.AddConnectionsToDesignDiff:
      return { ...state, designDiff: addConnectionsToDesignDiff(state.designDiff, action.payload) }
    case DesignEditorAction.SetConnectionsInDesignDiff:
      return { ...state, designDiff: setConnectionsInDesignDiff(state.designDiff, action.payload) }
    case DesignEditorAction.RemoveConnectionsFromDesignDiff:
      return { ...state, designDiff: removeConnectionsFromDesignDiff(state.designDiff, action.payload) }

    // Selection changes (no command stack)
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

    // Other (no command stack)
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
    designDiff: controlledDesignDiff,
    selection: controlledSelection,
    initialKit,
    initialDesignDiff,
    initialSelection,
    onDesignChange,
    onDesignDiffChange,
    onSelectionChange,
    onUndo,
    onRedo,
    designId,
    fileUrls
  } = props

  const isKitControlled = controlledKit !== undefined
  const isDesignDiffControlled = controlledDesignDiff !== undefined
  const isSelectionControlled = controlledSelection !== undefined

  const initialDesign = findDesignInKit(initialKit!, designId)
  const initialState: DesignEditorState = {
    kit: initialKit!,
    designId: designId,
    fileUrls: fileUrls,
    fullscreenPanel: FullscreenPanel.None,
    selection: initialSelection || { selectedPieceIds: [], selectedConnections: [] },
    designDiff: initialDesignDiff || {
      pieces: { added: [], removed: [], updated: [] },
      connections: { added: [], removed: [], updated: [] }
    },
    commandStack: [{
      design: JSON.parse(JSON.stringify(initialDesign)),
      selection: JSON.parse(JSON.stringify(initialSelection || { selectedPieceIds: [], selectedConnections: [] })),
      designDiff: JSON.parse(JSON.stringify(initialDesignDiff || {
        pieces: { added: [], removed: [], updated: [] },
        connections: { added: [], removed: [], updated: [] }
      }))
    }],
    commandIndex: 0
  }

  const [internalState, dispatch] = useReducer(designEditorReducer, initialState)

  const state: DesignEditorState = {
    ...internalState,
    kit: isKitControlled ? controlledKit : internalState.kit,
    designDiff: isDesignDiffControlled ? controlledDesignDiff : internalState.designDiff,
    selection: isSelectionControlled ? controlledSelection : internalState.selection,
    commandStack: isKitControlled || isDesignDiffControlled || isSelectionControlled ? [] : internalState.commandStack,
    commandIndex: isKitControlled || isDesignDiffControlled || isSelectionControlled ? -1 : internalState.commandIndex
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

    if (!isKitControlled || !isDesignDiffControlled || !isSelectionControlled) dispatch(action)
    if (isKitControlled && newState.kit !== state.kit) onDesignChange?.(findDesignInKit(newState.kit, designId))
    if (isDesignDiffControlled && newState.designDiff !== state.designDiff) onDesignDiffChange?.(newState.designDiff)
    if (isSelectionControlled && newState.selection !== state.selection) onSelectionChange?.(newState.selection)
  }, [state, isKitControlled, isDesignDiffControlled, isSelectionControlled, designId, onDesignChange, onDesignDiffChange, onSelectionChange, onUndo, onRedo])

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
                <TreeNode key={name} label={name} collapsible={true} level={1} defaultOpen={false}>
                  <div
                    className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1"
                    style={{ paddingLeft: `${(1 + 1) * 1.25}rem` }}
                  >
                    {variants.map((type) => (
                      <TypeAvatar key={`${type.name}-${type.variant}`} type={type} showHoverCard={true} />
                    ))}
                  </div>
                </TreeNode>
              ))}
            </TreeSection>
            <TreeSection label="Designs" defaultOpen={true}>
              {Object.entries(designsByName).map(([name, designs]) => (
                <TreeNode key={name} label={name} collapsible={true} level={1} defaultOpen={false}>
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
                </TreeNode>
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
      <ScrollArea className="h-full">
        <div className="p-1">
          <Tree>
            <TreeSection label="Messages" defaultOpen={true}>
              <TreeItem label="System started" level={1} />
              <TreeItem label="Design loaded successfully" level={1} />
            </TreeSection>
            <TreeSection label="Errors" defaultOpen={false}>
              <TreeItem label="No errors" level={1} />
            </TreeSection>
            <TreeSection label="Warnings" defaultOpen={false}>
              <TreeItem label="No warnings" level={1} />
            </TreeSection>
          </Tree>
        </div>
      </ScrollArea>
    </div>
  )
}

interface DetailsProps extends ResizablePanelProps { }

const PieceDetails: FC<{ pieceId: string }> = ({ pieceId }) => {
  const { kit, designId, designDiff, setPieceInDesignDiff, setConnectionInDesignDiff } = useDesignEditor()
  const design = findDesignInKit(kit, designId)
  const effectiveDesign = applyDesignDiff(design, designDiff, false)
  const piece = findPieceInDesign(effectiveDesign, pieceId)
  const metadata = piecesMetadata(kit, designId)
  const pieceMetadata = metadata.get(pieceId)

  const handlePieceChange = (updatedPiece: Piece) => {
    setPieceInDesignDiff(updatedPiece)
  }

  const handleConnectionChange = (updatedConnection: Connection) => {
    setConnectionInDesignDiff(updatedConnection)
  }

  const pieceVariants = piece.type.variant ? [piece.type.variant] : []
  const availableTypes = findReplacableTypesForPieceInDesign(kit, designId, pieceId, pieceVariants)
  const availableTypeNames = [...new Set(availableTypes.map(t => t.name))]
  const availableVariants = findReplacableTypesForPieceInDesign(kit, designId, pieceId).filter(t => t.name === piece.type.name).map(t => t.variant).filter((v): v is string => Boolean(v))

  let parentConnection: Connection | null = null
  if (pieceMetadata?.parentPieceId) {
    try {
      parentConnection = findConnectionInDesign(effectiveDesign, {
        connected: { piece: { id_: pieceId } },
        connecting: { piece: { id_: pieceMetadata.parentPieceId } }
      })
    } catch {
      try {
        parentConnection = findConnectionInDesign(effectiveDesign, {
          connected: { piece: { id_: pieceMetadata.parentPieceId } },
          connecting: { piece: { id_: pieceId } }
        })
      } catch {
        // No parent connection found
      }
    }
  }

  return (
    <div className="p-1">
      <Tree>
        <TreeSection label="Piece Details" defaultOpen={true}>
          <div className="space-y-2 px-2">
            <div className="space-y-1">
              <div className="flex items-center justify-between">
                <label className="text-sm">ID</label>
              </div>
              <Input value={piece.id_} disabled />
            </div>
            <div className="space-y-1">
              <div className="flex items-center justify-between">
                <label className="text-sm">Type Name</label>
              </div>
              <Combobox
                options={availableTypeNames.map(name => ({ value: name, label: name }))}
                value={piece.type.name}
                placeholder="Select type name"
                onValueChange={(value) => handlePieceChange({ ...piece, type: { ...piece.type, name: value } })}
              />
            </div>
            {(piece.type.variant || availableVariants.length > 0) && (
              <div className="space-y-1">
                <div className="flex items-center justify-between">
                  <label className="text-sm">Type Variant</label>
                </div>
                <Combobox
                  options={availableVariants.map(variant => ({ value: variant, label: variant }))}
                  value={piece.type.variant || ''}
                  placeholder="Select variant"
                  onValueChange={(value) => handlePieceChange({ ...piece, type: { ...piece.type, variant: value } })}
                  allowClear={true}
                />
              </div>
            )}
          </div>
        </TreeSection>
        {piece.center && (
          <TreeSection label="Center" defaultOpen={false}>
            <div className="space-y-2 px-2">
              <div className="space-y-1">
                <div className="flex items-center justify-between">
                  <label className="text-sm">X</label>
                  <span className="text-xs text-muted-foreground">{piece.center.x}</span>
                </div>
                <Stepper
                  value={piece.center.x}
                  onChange={(value) => handlePieceChange({ ...piece, center: { ...piece.center!, x: value } })}
                  step={0.1}
                />
              </div>
              <div className="space-y-1">
                <div className="flex items-center justify-between">
                  <label className="text-sm">Y</label>
                  <span className="text-xs text-muted-foreground">{piece.center.y}</span>
                </div>
                <Stepper
                  value={piece.center.y}
                  onChange={(value) => handlePieceChange({ ...piece, center: { ...piece.center!, y: value } })}
                  step={0.1}
                />
              </div>
            </div>
          </TreeSection>
        )}
        {piece.plane && (
          <TreeSection label="Plane" defaultOpen={false}>
            <div className="space-y-2 px-2">
              <TreeNode label="Origin" collapsible={true} level={1} defaultOpen={true}>
                <div className="space-y-2 px-2">
                  <div className="space-y-1">
                    <div className="flex items-center justify-between">
                      <label className="text-sm">X</label>
                      <span className="text-xs text-muted-foreground">{piece.plane.origin.x}</span>
                    </div>
                    <Stepper
                      value={piece.plane.origin.x}
                      onChange={(value) =>
                        handlePieceChange({
                          ...piece,
                          plane: { ...piece.plane!, origin: { ...piece.plane!.origin, x: value } }
                        })
                      }
                      step={0.1}
                    />
                  </div>
                  <div className="space-y-1">
                    <div className="flex items-center justify-between">
                      <label className="text-sm">Y</label>
                      <span className="text-xs text-muted-foreground">{piece.plane.origin.y}</span>
                    </div>
                    <Stepper
                      value={piece.plane.origin.y}
                      onChange={(value) =>
                        handlePieceChange({
                          ...piece,
                          plane: { ...piece.plane!, origin: { ...piece.plane!.origin, y: value } }
                        })
                      }
                      step={0.1}
                    />
                  </div>
                  <div className="space-y-1">
                    <div className="flex items-center justify-between">
                      <label className="text-sm">Z</label>
                      <span className="text-xs text-muted-foreground">{piece.plane.origin.z}</span>
                    </div>
                    <Stepper
                      value={piece.plane.origin.z}
                      onChange={(value) =>
                        handlePieceChange({
                          ...piece,
                          plane: { ...piece.plane!, origin: { ...piece.plane!.origin, z: value } }
                        })
                      }
                      step={0.1}
                    />
                  </div>
                </div>
              </TreeNode>
            </div>
          </TreeSection>
        )}
        {parentConnection && (
          <TreeSection label="Parent Connection" defaultOpen={true}>
            <div className="space-y-2 px-2">
              <div className="space-y-1">
                <div className="flex items-center justify-between">
                  <label className="text-sm">Gap</label>
                  <span className="text-xs text-muted-foreground">{parentConnection.gap ?? 0}</span>
                </div>
                <Stepper
                  value={parentConnection.gap ?? 0}
                  onChange={(value) => handleConnectionChange({ ...parentConnection, gap: value })}
                  step={0.1}
                />
              </div>
              <div className="space-y-1">
                <div className="flex items-center justify-between">
                  <label className="text-sm">Shift</label>
                  <span className="text-xs text-muted-foreground">{parentConnection.shift ?? 0}</span>
                </div>
                <Stepper
                  value={parentConnection.shift ?? 0}
                  onChange={(value) => handleConnectionChange({ ...parentConnection, shift: value })}
                  step={0.1}
                />
              </div>
              <div className="space-y-1">
                <div className="flex items-center justify-between">
                  <label className="text-sm">Rise</label>
                  <span className="text-xs text-muted-foreground">{parentConnection.rise ?? 0}</span>
                </div>
                <Stepper
                  value={parentConnection.rise ?? 0}
                  onChange={(value) => handleConnectionChange({ ...parentConnection, rise: value })}
                  step={0.1}
                />
              </div>
              <div className="space-y-1">
                <div className="flex items-center justify-between">
                  <label className="text-sm">Rotation</label>
                  <span className="text-xs text-muted-foreground">{parentConnection.rotation ?? 0}°</span>
                </div>
                <Slider
                  value={[parentConnection.rotation ?? 0]}
                  onValueChange={([value]) => handleConnectionChange({ ...parentConnection, rotation: value })}
                  min={-180}
                  max={180}
                  step={1}
                />
              </div>
              <div className="space-y-1">
                <div className="flex items-center justify-between">
                  <label className="text-sm">Turn</label>
                  <span className="text-xs text-muted-foreground">{parentConnection.turn ?? 0}°</span>
                </div>
                <Slider
                  value={[parentConnection.turn ?? 0]}
                  onValueChange={([value]) => handleConnectionChange({ ...parentConnection, turn: value })}
                  min={-180}
                  max={180}
                  step={1}
                />
              </div>
              <div className="space-y-1">
                <div className="flex items-center justify-between">
                  <label className="text-sm">Tilt</label>
                  <span className="text-xs text-muted-foreground">{parentConnection.tilt ?? 0}°</span>
                </div>
                <Slider
                  value={[parentConnection.tilt ?? 0]}
                  onValueChange={([value]) => handleConnectionChange({ ...parentConnection, tilt: value })}
                  min={-180}
                  max={180}
                  step={1}
                />
              </div>
              <div className="space-y-1">
                <div className="flex items-center justify-between">
                  <label className="text-sm">X Offset</label>
                  <span className="text-xs text-muted-foreground">{parentConnection.x ?? 0}</span>
                </div>
                <Stepper
                  value={parentConnection.x ?? 0}
                  onChange={(value) => handleConnectionChange({ ...parentConnection, x: value })}
                  step={0.1}
                />
              </div>
              <div className="space-y-1">
                <div className="flex items-center justify-between">
                  <label className="text-sm">Y Offset</label>
                  <span className="text-xs text-muted-foreground">{parentConnection.y ?? 0}</span>
                </div>
                <Stepper
                  value={parentConnection.y ?? 0}
                  onChange={(value) => handleConnectionChange({ ...parentConnection, y: value })}
                  step={0.1}
                />
              </div>
            </div>
          </TreeSection>
        )}
      </Tree>
    </div>
  )
}

const MultiPieceDetails: FC<{ pieceIds: string[] }> = ({ pieceIds }) => {
  const { kit, designId, designDiff, setPiecesInDesignDiff } = useDesignEditor()
  const design = findDesignInKit(kit, designId)
  const effectiveDesign = applyDesignDiff(design, designDiff, false)
  const pieces = pieceIds.map(id => findPieceInDesign(effectiveDesign, id))

  const getCommonValue = <T,>(getter: (piece: Piece) => T | undefined): T | undefined => {
    const values = pieces.map(getter).filter(v => v !== undefined)
    if (values.length === 0) return undefined
    const firstValue = values[0]
    return values.every(v => JSON.stringify(v) === JSON.stringify(firstValue)) ? firstValue : undefined
  }

  const handleTypeNameChange = (value: string) => {
    const updatedPieces = pieces.map(piece => ({ ...piece, type: { ...piece.type, name: value } }))
    setPiecesInDesignDiff(updatedPieces)
  }

  const handleTypeVariantChange = (value: string) => {
    const updatedPieces = pieces.map(piece => ({ ...piece, type: { ...piece.type, variant: value } }))
    setPiecesInDesignDiff(updatedPieces)
  }

  const handleCenterXChange = (value: number) => {
    const updatedPieces = pieces.map(piece =>
      piece.center ? { ...piece, center: { ...piece.center, x: value } } : piece
    )
    setPiecesInDesignDiff(updatedPieces)
  }

  const handleCenterYChange = (value: number) => {
    const updatedPieces = pieces.map(piece =>
      piece.center ? { ...piece, center: { ...piece.center, y: value } } : piece
    )
    setPiecesInDesignDiff(updatedPieces)
  }

  const handlePlaneOriginXChange = (value: number) => {
    const updatedPieces = pieces.map(piece =>
      piece.plane ? { ...piece, plane: { ...piece.plane, origin: { ...piece.plane.origin, x: value } } } : piece
    )
    setPiecesInDesignDiff(updatedPieces)
  }

  const handlePlaneOriginYChange = (value: number) => {
    const updatedPieces = pieces.map(piece =>
      piece.plane ? { ...piece, plane: { ...piece.plane, origin: { ...piece.plane.origin, y: value } } } : piece
    )
    setPiecesInDesignDiff(updatedPieces)
  }

  const handlePlaneOriginZChange = (value: number) => {
    const updatedPieces = pieces.map(piece =>
      piece.plane ? { ...piece, plane: { ...piece.plane, origin: { ...piece.plane.origin, z: value } } } : piece
    )
    setPiecesInDesignDiff(updatedPieces)
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

  const selectedVariants = [...new Set(pieces.map(p => p.type.variant).filter((v): v is string => Boolean(v)))]
  const availableTypes = findReplacableTypesForPiecesInDesign(kit, designId, pieceIds, selectedVariants)
  const availableTypeNames = [...new Set(availableTypes.map(t => t.name))]
  const availableVariants = commonTypeName ? [...new Set(findReplacableTypesForPiecesInDesign(kit, designId, pieceIds).filter(t => t.name === commonTypeName).map(t => t.variant).filter((v): v is string => Boolean(v)))] : []

  return (
    <div className="space-y-2 p-1">
      <div className="text-sm font-medium mb-3">{pieceIds.length} pieces selected</div>

      <div className="space-y-1">
        <div className="flex items-center justify-between">
          <label className="text-sm">Type Name</label>
        </div>
        <Combobox
          options={availableTypeNames.map(name => ({ value: name, label: name }))}
          value={commonTypeName || ''}
          placeholder={commonTypeName === undefined ? 'Mixed values' : 'Select type name'}
          onValueChange={handleTypeNameChange}
        />
      </div>

      {(hasVariant || availableVariants.length > 0) && (
        <div className="space-y-1">
          <div className="flex items-center justify-between">
            <label className="text-sm">Type Variant</label>
          </div>
          <Combobox
            options={availableVariants.map(variant => ({ value: variant, label: variant }))}
            value={commonTypeVariant || ''}
            placeholder={commonTypeVariant === undefined ? 'Mixed values' : 'Select variant'}
            onValueChange={handleTypeVariantChange}
            allowClear={true}
          />
        </div>
      )}

      {hasCenter && (
        <>
          <div className="space-y-1">
            <div className="flex items-center justify-between">
              <label className="text-sm">Center X</label>
              <span className="text-xs text-muted-foreground">{commonCenterX !== undefined ? commonCenterX : 'Mixed'}</span>
            </div>
            <Stepper
              value={commonCenterX}
              onChange={handleCenterXChange}
              step={0.1}
            />
          </div>
          <div className="space-y-1">
            <div className="flex items-center justify-between">
              <label className="text-sm">Center Y</label>
              <span className="text-xs text-muted-foreground">{commonCenterY !== undefined ? commonCenterY : 'Mixed'}</span>
            </div>
            <Stepper
              value={commonCenterY}
              onChange={handleCenterYChange}
              step={0.1}
            />
          </div>
        </>
      )}

      {hasPlane && (
        <>
          <div className="space-y-1">
            <div className="flex items-center justify-between">
              <label className="text-sm">Plane Origin X</label>
              <span className="text-xs text-muted-foreground">{commonPlaneOriginX !== undefined ? commonPlaneOriginX : 'Mixed'}</span>
            </div>
            <Stepper
              value={commonPlaneOriginX}
              onChange={handlePlaneOriginXChange}
              step={0.1}
            />
          </div>
          <div className="space-y-1">
            <div className="flex items-center justify-between">
              <label className="text-sm">Plane Origin Y</label>
              <span className="text-xs text-muted-foreground">{commonPlaneOriginY !== undefined ? commonPlaneOriginY : 'Mixed'}</span>
            </div>
            <Stepper
              value={commonPlaneOriginY}
              onChange={handlePlaneOriginYChange}
              step={0.1}
            />
          </div>
          <div className="space-y-1">
            <div className="flex items-center justify-between">
              <label className="text-sm">Plane Origin Z</label>
              <span className="text-xs text-muted-foreground">{commonPlaneOriginZ !== undefined ? commonPlaneOriginZ : 'Mixed'}</span>
            </div>
            <Stepper
              value={commonPlaneOriginZ}
              onChange={handlePlaneOriginZChange}
              step={0.1}
            />
          </div>
        </>
      )}
    </div>
  )
}

const ConnectionDetails: FC<{ connectingPieceId: string; connectedPieceId: string }> = ({
  connectingPieceId,
  connectedPieceId
}) => {
  const { kit, designId, designDiff, setConnectionInDesignDiff } = useDesignEditor()
  const design = findDesignInKit(kit, designId)
  const effectiveDesign = applyDesignDiff(design, designDiff, false)
  const connectionId = {
    connecting: { piece: { id_: connectingPieceId } },
    connected: { piece: { id_: connectedPieceId } }
  }
  const connection = findConnectionInDesign(effectiveDesign, connectionId)

  const handleChange = (updatedConnection: Connection) => {
    setConnectionInDesignDiff(updatedConnection)
  }

  return (
    <div className="space-y-2 p-1">
      <div className="space-y-1">
        <div className="flex items-center justify-between">
          <label className="text-sm">Connecting Piece ID</label>
        </div>
        <Input value={connection.connecting.piece.id_} disabled />
      </div>
      <div className="space-y-1">
        <div className="flex items-center justify-between">
          <label className="text-sm">Connecting Port ID</label>
        </div>
        <Input
          value={connection.connecting.port.id_}
          onChange={(e) =>
            handleChange({ ...connection, connecting: { ...connection.connecting, port: { id_: e.target.value } } })
          }
        />
      </div>
      <div className="space-y-1">
        <div className="flex items-center justify-between">
          <label className="text-sm">Connected Piece ID</label>
        </div>
        <Input value={connection.connected.piece.id_} disabled />
      </div>
      <div className="space-y-1">
        <div className="flex items-center justify-between">
          <label className="text-sm">Connected Port ID</label>
        </div>
        <Input
          value={connection.connected.port.id_}
          onChange={(e) =>
            handleChange({ ...connection, connected: { ...connection.connected, port: { id_: e.target.value } } })
          }
        />
      </div>
      <div className="space-y-1">
        <div className="flex items-center justify-between">
          <label className="text-sm">Gap</label>
          <span className="text-xs text-muted-foreground">{connection.gap ?? 0}</span>
        </div>
        <Input
          type="number"
          value={connection.gap ?? 0}
          onChange={(e) => handleChange({ ...connection, gap: parseFloat(e.target.value) })}
        />
      </div>
    </div>
  )
}

const DesignDetails: FC = () => {
  const { kit, designId, setDesign } = useDesignEditor()
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
    <div className="space-y-2 p-1">
      <div className="text-sm font-medium mb-3">Design Properties</div>

      <div className="space-y-1">
        <div className="flex items-center justify-between">
          <label className="text-sm">Name</label>
        </div>
        <Input
          value={design.name}
          onChange={(e) => handleChange({ ...design, name: e.target.value })}
        />
      </div>

      <div className="space-y-1">
        <div className="flex items-center justify-between">
          <label className="text-sm">Description</label>
        </div>
        <Textarea
          value={design.description || ''}
          placeholder="Enter design description..."
          onChange={(e) => handleChange({ ...design, description: e.target.value })}
        />
      </div>

      <div className="space-y-1">
        <div className="flex items-center justify-between">
          <label className="text-sm">Icon</label>
        </div>
        <Input
          value={design.icon || ''}
          placeholder="Emoji, name, or URL"
          onChange={(e) => handleChange({ ...design, icon: e.target.value })}
        />
      </div>

      <div className="space-y-1">
        <div className="flex items-center justify-between">
          <label className="text-sm">Image URL</label>
        </div>
        <Input
          value={design.image || ''}
          placeholder="URL to design image"
          onChange={(e) => handleChange({ ...design, image: e.target.value })}
        />
      </div>

      <div className="space-y-1">
        <div className="flex items-center justify-between">
          <label className="text-sm">Variant</label>
        </div>
        <Input
          value={design.variant || ''}
          placeholder="Design variant"
          onChange={(e) => handleChange({ ...design, variant: e.target.value })}
        />
      </div>

      <div className="space-y-1">
        <div className="flex items-center justify-between">
          <label className="text-sm">View</label>
        </div>
        <Input
          value={design.view || ''}
          placeholder="Design view"
          onChange={(e) => handleChange({ ...design, view: e.target.value })}
        />
      </div>

      <div className="space-y-1">
        <div className="flex items-center justify-between">
          <label className="text-sm">Unit</label>
        </div>
        <Input
          value={design.unit}
          onChange={(e) => handleChange({ ...design, unit: e.target.value })}
        />
      </div>

      {design.location ? (
        <>
          <div className="flex items-center justify-between">
            <label className="text-sm">Location</label>
            <Button
              variant="ghost"
              size="sm"
              onClick={removeLocation}
              className="text-destructive hover:text-destructive"
            >
              Remove
            </Button>
          </div>
          <div className="space-y-1">
            <div className="flex items-center justify-between">
              <label className="text-xs text-muted-foreground">Longitude</label>
              <span className="text-xs text-muted-foreground">{design.location.longitude}</span>
            </div>
            <Stepper
              value={design.location.longitude}
              onChange={(value) => handleChange({
                ...design,
                location: { ...design.location!, longitude: value }
              })}
              step={0.000001}
            />
          </div>
          <div className="space-y-1">
            <div className="flex items-center justify-between">
              <label className="text-xs text-muted-foreground">Latitude</label>
              <span className="text-xs text-muted-foreground">{design.location.latitude}</span>
            </div>
            <Stepper
              value={design.location.latitude}
              onChange={(value) => handleChange({
                ...design,
                location: { ...design.location!, latitude: value }
              })}
              step={0.000001}
            />
          </div>
        </>
      ) : (
        <div className="space-y-1">
          <div className="flex items-center justify-between">
            <label className="text-sm">Location</label>
          </div>
          <Button
            variant="ghost"
            size="sm"
            onClick={addLocation}
            className="text-muted-foreground hover:text-foreground"
          >
            Add location
          </Button>
        </div>
      )}

      {design.created && (
        <div className="space-y-1">
          <div className="flex items-center justify-between">
            <label className="text-sm">Created</label>
          </div>
          <Input value={design.created.toISOString().split('T')[0]} disabled />
        </div>
      )}

      {design.updated && (
        <div className="space-y-1">
          <div className="flex items-center justify-between">
            <label className="text-sm">Updated</label>
          </div>
          <Input value={design.updated.toISOString().split('T')[0]} disabled />
        </div>
      )}

      {design.pieces && design.pieces.length > 0 && (
        <div className="space-y-1">
          <div className="flex items-center justify-between">
            <label className="text-sm">Pieces</label>
          </div>
          <Input value={`${design.pieces.length} pieces`} disabled />
        </div>
      )}

      {design.connections && design.connections.length > 0 && (
        <div className="space-y-1">
          <div className="flex items-center justify-between">
            <label className="text-sm">Connections</label>
          </div>
          <Input value={`${design.connections.length} connections`} disabled />
        </div>
      )}

      {design.authors && design.authors.length > 0 && (
        <div className="space-y-1">
          <div className="flex items-center justify-between">
            <label className="text-sm">Authors</label>
          </div>
          <Input value={`${design.authors.length} authors`} disabled />
        </div>
      )}

      {design.qualities && design.qualities.length > 0 && (
        <div className="space-y-1">
          <div className="flex items-center justify-between">
            <label className="text-sm">Qualities</label>
          </div>
          <Input value={`${design.qualities.length} qualities`} disabled />
        </div>
      )}
    </div>
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

  const { kit, designId, selection, designDiff } = useDesignEditor()
  const design = findDesignInKit(kit, designId)
  const effectiveDesign = applyDesignDiff(design, designDiff, false)

  let content: ReactNode
  if (selection.selectedPieceIds.length === 1 && selection.selectedConnections.length === 0) {
    content = <PieceDetails pieceId={selection.selectedPieceIds[0]} />
  } else if (selection.selectedPieceIds.length > 1 && selection.selectedConnections.length === 0) {
    content = <MultiPieceDetails pieceIds={selection.selectedPieceIds} />
  } else if (selection.selectedPieceIds.length === 0 && selection.selectedConnections.length === 1) {
    const { connectingPieceId, connectedPieceId } = selection.selectedConnections[0]
    content = <ConnectionDetails connectingPieceId={connectingPieceId} connectedPieceId={connectedPieceId} />
  } else if (selection.selectedPieceIds.length === 0 && selection.selectedConnections.length === 0) {
    content = <DesignDetails />
  } else {
    content = <div className="p-1">Multiple selection types - select pieces or connections to edit details.</div>
  }

  return (
    <div
      className={`absolute top-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isResizing || isResizeHovered ? 'border-l-primary' : 'border-l'}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">{content}</ScrollArea>
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
              <TreeNode label="Design Session #1" collapsible={true} level={1} defaultOpen={false}>
                <TreeItem label="How can I add a new piece?" level={2} />
                <TreeItem label="Can you help with connections?" level={2} />
              </TreeNode>
              <TreeNode label="Design Session #2" collapsible={true} level={1} defaultOpen={false}>
                <TreeItem label="What are the available types?" level={2} />
              </TreeNode>
            </TreeSection>
            <TreeSection label="Quick Actions" defaultOpen={true}>
              <TreeItem label="Add random piece" level={1} />
              <TreeItem label="Connect all pieces" level={1} />
              <TreeItem label="Generate layout suggestions" level={1} />
            </TreeSection>
            <TreeSection label="Templates" defaultOpen={false}>
              <TreeNode label="Common Questions" collapsible={true} level={1} defaultOpen={true}>
                <TreeItem label="How do I create a connection?" level={2} />
                <TreeItem label="How do I delete a piece?" level={2} />
                <TreeItem label="How do I change piece properties?" level={2} />
              </TreeNode>
              <TreeNode label="Advanced Workflows" collapsible={true} level={1} defaultOpen={false}>
                <TreeItem label="Batch operations" level={2} />
                <TreeItem label="Complex layouts" level={2} />
                <TreeItem label="Export/Import" level={2} />
              </TreeNode>
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
  designDiff?: DesignDiff
  onDesignChange?: (design: Design) => void
  onDesignDiffChange?: (designDiff: DesignDiff) => void
  onSelectionChange?: (selection: DesignEditorSelection) => void
  onUndo?: () => void
  onRedo?: () => void
}

interface UncontrolledDesignEditorProps {
  initialKit?: Kit
  initialDesignDiff?: DesignDiff
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
  designDiff,
  selection,
  initialKit,
  initialDesignDiff,
  initialSelection,
  fileUrls,
  onDesignChange,
  onDesignDiffChange,
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
          designDiff={designDiff}
          selection={selection}
          initialKit={initialKit}
          initialDesignDiff={initialDesignDiff}
          initialSelection={initialSelection}
          onDesignChange={onDesignChange}
          onDesignDiffChange={onDesignDiffChange}
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
