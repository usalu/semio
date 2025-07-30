// #region Header

// Diagram.tsx

// 2025 Ueli Saluz
// 2025 AdrianoCelentano

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

import { useDroppable } from '@dnd-kit/core'
import {
  BaseEdge,
  ConnectionLineComponentProps,
  ConnectionMode,
  Controls,
  Edge,
  EdgeProps,
  Handle,
  MiniMap,
  MiniMapNodeProps,
  Node,
  NodeProps,
  Position,
  ReactFlow,
  Connection as RFConnection,
  useReactFlow,
  ViewportPortal,
  XYPosition
} from '@xyflow/react'
import React, { FC, useCallback, useEffect, useMemo, useState } from 'react'

import {
  arePortsCompatible,
  Connection,
  ConnectionDiff,
  DesignId,
  DiagramPoint,
  DiffStatus,
  findConnectionInDesign,
  findDesignInKit,
  findPortInType,
  findTypeInKit,
  flattenDesign,
  FullscreenPanel,
  ICON_WIDTH,
  isPortInUse,
  isSameConnection,
  isSamePiece,
  Kit,
  Piece,
  PieceDiff,
  piecesMetadata,
  Port,
  Type
} from '@semio/js'

// import '@xyflow/react/dist/base.css';
import '@semio/js/globals.css'
import '@xyflow/react/dist/style.css'
import { DesignEditorSelection } from '../..'
import { useDesignEditor } from './DesignEditor'


//#region React Flow

type HelperLine = {
  type: 'horizontal' | 'vertical' | 'equalDistance'
  position?: number
  relatedPieceId: string
  // For equal distance lines
  x1?: number
  y1?: number
  x2?: number
  y2?: number
  distance?: number
  referencePieceIds?: string[]
}

type PieceNodeProps = {
  piece: Piece
  type: Type
}

type PieceNode = Node<PieceNodeProps, 'piece'>
type DiagramNode = PieceNode

type ConnectionEdge = Edge<{ connection: Connection; isParentConnection?: boolean }, 'connection'>
type DiagramEdge = ConnectionEdge

type PortHandleProps = { port: Port }

const getPortPositionStyle = (port: Port): { x: number; y: number } => {
  // t is normalized in [0,1[ and clockwise and starts at 12 o'clock
  const { t } = port
  if (t === undefined) {
    return { x: 0, y: 0 }
  }
  const angle = t * 2 * Math.PI
  const radius = ICON_WIDTH / 2
  return {
    x: radius * Math.sin(angle),
    y: -(radius * Math.cos(angle) - radius)
  }
}

const PortHandle: React.FC<PortHandleProps> = ({ port }) => {
  const { x, y } = getPortPositionStyle(port)

  return (
    <Handle id={port.id_ ?? ''} type="source" className="left-1/2 top-0" style={{ left: x + ICON_WIDTH / 2, top: y }} position={Position.Top} />
  )
}

const PieceNodeComponent: React.FC<NodeProps<PieceNode>> = React.memo(({ id, data, selected }) => {
  const {
    piece: { id_, qualities },
    type: { ports }
  } = data as PieceNodeProps & { diffStatus: DiffStatus }

  let fillClass = ''
  let strokeClass = 'stroke-dark stroke-2'
  let opacity = 1

  const diff = qualities?.find(q => q.name === 'semio.diffStatus')?.value as DiffStatus || DiffStatus.Unchanged

  if (diff === DiffStatus.Added) {
    fillClass = selected ? 'fill-[color-mix(in_srgb,theme(colors.success)_50%,theme(colors.primary)_50%)]' : 'fill-success'
  } else if (diff === DiffStatus.Removed) {
    fillClass = selected ? 'fill-[color-mix(in_srgb,theme(colors.danger)_50%,theme(colors.primary)_50%)]' : 'fill-danger'
    strokeClass = 'stroke-danger stroke-2'
    opacity = 0.2
  } else if (diff === DiffStatus.Modified) {
    fillClass = selected ? 'fill-[color-mix(in_srgb,theme(colors.warning)_50%,theme(colors.primary)_50%)]' : 'fill-warning'
  } else if (selected) {
    fillClass = 'fill-primary'
  } else {
    fillClass = 'fill-transparent'
  }

  return (
    <div style={{ opacity }}>
      <svg width={ICON_WIDTH} height={ICON_WIDTH} className="cursor-pointer">
        <circle
          cx={ICON_WIDTH / 2}
          cy={ICON_WIDTH / 2}
          r={ICON_WIDTH / 2 - 1}
          className={`${strokeClass} ${fillClass}`}
        />
        <text
          x={ICON_WIDTH / 2}
          y={ICON_WIDTH / 2}
          textAnchor="middle"
          dominantBaseline="middle"
          className="text-xs font-bold fill-dark"
        >
          {id_}
        </text>
      </svg>
      {ports?.map((port: Port) => <PortHandle key={port.id_} port={port} />)}
    </div>
  )
})

const ConnectionEdgeComponent: React.FC<EdgeProps<ConnectionEdge>> = ({
  id,
  source,
  target,
  sourceX,
  sourceY,
  targetX,
  targetY,
  sourceHandleId,
  targetHandleId,
  data,
  selected
}) => {
  const HANDLE_HEIGHT = 5
  const path = `M ${sourceX} ${sourceY + HANDLE_HEIGHT / 2} L ${targetX} ${targetY + HANDLE_HEIGHT / 2}`

  const diff = data?.connection?.qualities?.find(q => q.name === 'semio.diffStatus')?.value as DiffStatus || DiffStatus.Unchanged;
  const isParentConnection = data?.isParentConnection ?? false;

  let stroke = 'var(--color-dark)';
  let strokeWidth = 2;
  let dasharray: string | undefined;
  let opacity = 1;

  if (diff === DiffStatus.Added) {
    stroke = selected ? 'color-mix(in srgb, var(--color-success) 50%, var(--color-primary) 50%)' : 'var(--color-success)';
    dasharray = '5 5';
  } else if (diff === DiffStatus.Removed) {
    stroke = selected ? 'color-mix(in srgb, var(--color-danger) 50%, var(--color-primary) 50%)' : 'var(--color-danger)';
    opacity = 0.25;
  } else if (diff === DiffStatus.Modified) {
    stroke = selected ? 'color-mix(in srgb, var(--color-warning) 50%, var(--color-primary) 50%)' : 'var(--color-warning)';
  } else if (selected) {
    stroke = 'var(--color-primary)';
  } else if (isParentConnection) {
    stroke = 'var(--color-secondary)';
    strokeWidth = 3;
  }

  return (
    <BaseEdge
      path={path}
      style={{
        stroke,
        strokeWidth,
        strokeDasharray: dasharray,
        opacity
      }}
      className="transition-colors duration-200"
    />
  )
}

const ConnectionConnectionLine: React.FC<ConnectionLineComponentProps> = (props: ConnectionLineComponentProps) => {
  const { fromX, fromY, toX, toY } = props
  const HANDLE_HEIGHT = 5
  const path = `M ${fromX} ${fromY + HANDLE_HEIGHT / 2} L ${toX} ${toY + HANDLE_HEIGHT / 2}`
  return <BaseEdge path={path} style={{ stroke: 'grey' }} className="opacity-70" />
}

export const MiniMapNode: React.FC<MiniMapNodeProps> = ({ x, y, selected }: MiniMapNodeProps) => {
  return <circle className={`${selected ? 'fill-primary' : 'fill-foreground'} transition-colors duration-200`} cx={x} cy={y} r="10" />
}

const HelperLines: React.FC<{ lines: HelperLine[], nodes: { id: string, position: { x: number, y: number } }[] }> = ({ lines, nodes }) => {
  const { getViewport } = useReactFlow()

  if (lines.length === 0) return null

  const viewport = getViewport()

  return (
    <div className="absolute inset-0 w-full h-full pointer-events-none z-[1000] overflow-hidden">
      {lines.map((line, index) => {
        if (line.type === 'horizontal' && line.position !== undefined) {
          const screenY = line.position * viewport.zoom + viewport.y
          return (
            <div
              key={`h-${line.relatedPieceId}-${index}`}
              className="absolute left-0 w-full h-px border-t border-dashed border-primary opacity-60"
              style={{ top: screenY }}
            />
          )
        } else if (line.type === 'vertical' && line.position !== undefined) {
          const screenX = line.position * viewport.zoom + viewport.x
          return (
            <div
              key={`v-${line.relatedPieceId}-${index}`}
              className="absolute top-0 w-px h-full border-l border-dashed border-primary opacity-60"
              style={{ left: screenX }}
            />
          )
        } else if (line.type === 'equalDistance' && line.x1 !== undefined && line.y1 !== undefined && line.x2 !== undefined && line.y2 !== undefined) {
          const screenX1 = line.x1 * viewport.zoom + viewport.x
          const screenY1 = line.y1 * viewport.zoom + viewport.y
          const screenX2 = line.x2 * viewport.zoom + viewport.x
          const screenY2 = line.y2 * viewport.zoom + viewport.y

          // Different styles based on line type
          const isMidLine = line.relatedPieceId.startsWith('mid-')
          const strokeColor = 'var(--color-primary)'
          const strokeWidth = isMidLine ? '3' : '2'
          const opacity = isMidLine ? 1 : 0.7
          const dashArray = isMidLine ? '4 4' : '8 4'

          return (
            <svg
              key={`eq-${line.relatedPieceId}-${index}`}
              className="absolute inset-0 w-full h-full pointer-events-none"
            >
              <line
                x1={screenX1}
                y1={screenY1}
                x2={screenX2}
                y2={screenY2}
                stroke={strokeColor}
                strokeWidth={strokeWidth}
                strokeDasharray={dashArray}
                opacity={opacity}
              />
            </svg>
          )
        }
        return null
      })}
    </div>
  )
}

const pieceToNode = (piece: Piece, type: Type, center: DiagramPoint, selected: boolean): PieceNode => ({
  type: 'piece',
  id: piece.id_,
  position: {
    x: center.x * ICON_WIDTH || 0,
    y: -center.y * ICON_WIDTH || 0
  },
  selected,
  data: { piece, type },
  className: selected ? 'selected' : ''
})

const connectionToEdge = (connection: Connection, selected: boolean, isParentConnection: boolean = false): ConnectionEdge => ({
  type: 'connection',
  id: `${connection.connecting.piece.id_} -- ${connection.connected.piece.id_}`,
  source: connection.connecting.piece.id_,
  sourceHandle: connection.connecting.port.id_,
  target: connection.connected.piece.id_,
  targetHandle: connection.connected.port.id_,
  data: { connection, isParentConnection },
  selected
})

const designToNodesAndEdges = (kit: Kit, designId: DesignId, selection: DesignEditorSelection) => {
  const design = findDesignInKit(kit, designId)
  if (!design) return null
  const centers = flattenDesign(kit, designId).pieces?.map(p => p.center)
  const metadata = piecesMetadata(kit, designId)

  const pieceNodes = design.pieces?.map(
    (piece, i) => pieceToNode(piece, findTypeInKit(kit, piece.type)!, centers![i]!, selection?.selectedPieceIds.includes(piece.id_) ?? false)) ?? []

  const parentConnectionId = selection?.selectedPieceIds.length === 1 && selection.selectedConnections.length === 0
    ? (() => {
      const selectedPieceId = selection.selectedPieceIds[0]
      const pieceMetadata = metadata.get(selectedPieceId)
      if (pieceMetadata?.parentPieceId) {
        return `${pieceMetadata.parentPieceId} -- ${selectedPieceId}`
      }
      return null
    })()
    : null

  const connectionEdges =
    design.connections?.map((connection) => {
      const isSelected = selection?.selectedConnections.some(
        (c) =>
          c.connectingPieceId === connection.connecting.piece.id_ &&
          c.connectedPieceId === connection.connected.piece.id_
      ) ?? false

      const connectionId = `${connection.connecting.piece.id_} -- ${connection.connected.piece.id_}`
      const isParentConnection = parentConnectionId === connectionId || parentConnectionId === `${connection.connected.piece.id_} -- ${connection.connecting.piece.id_}`

      return connectionToEdge(connection, isSelected, isParentConnection)
    }) ?? []
  return { nodes: pieceNodes, edges: connectionEdges }
}

//#endregion


const Diagram: FC = () => {
  const { kit, designId, selection, fullscreenPanel, setDesign, deselectAll, selectPiece, addPieceToSelection, removePieceFromSelection, selectConnection, addConnectionToSelection, removeConnectionFromSelection, toggleDiagramFullscreen, startTransaction, finalizeTransaction, abortTransaction, updatePiece, updatePieces, addConnectionsToDesign, updateConnection, updateConnections } = useDesignEditor();
  if (!kit) return null
  const design = findDesignInKit(kit, designId)
  if (!design) return null
  const { nodes, edges } = designToNodesAndEdges(kit, designId, selection) ?? { nodes: [], edges: [] }
  const reactFlowInstance = useReactFlow()
  const [dragState, setDragState] = useState<{
    lastPostition: XYPosition
  } | null>(null)
  const [helperLines, setHelperLines] = useState<HelperLine[]>([])
  const fullscreen = fullscreenPanel === FullscreenPanel.Diagram

  // Handle escape key to abort transactions
  useEffect(() => {
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === 'Escape' && dragState) {
        abortTransaction()
        setDragState(null)
      }
    }

    document.addEventListener('keydown', handleEscape)
    return () => document.removeEventListener('keydown', handleEscape)
  }, [dragState, abortTransaction])

  //#endregion State

  //#region Actions
  const onNodeClick = (e: React.MouseEvent, node: DiagramNode) => {
    e.stopPropagation()
    if (e.ctrlKey || e.metaKey) removePieceFromSelection({ id_: node.data.piece.id_ })
    else if (e.shiftKey) addPieceToSelection({ id_: node.data.piece.id_ })
    else selectPiece({ id_: node.data.piece.id_ })
  }

  const onEdgeClick = (e: React.MouseEvent, edge: DiagramEdge) => {
    e.stopPropagation()
    if (e.ctrlKey || e.metaKey) removeConnectionFromSelection(edge.data!.connection)
    else if (e.shiftKey) addConnectionToSelection(edge.data!.connection)
    else selectConnection(edge.data!.connection)
  }

  const onPaneClick = (e: React.MouseEvent) => { if (!(e.ctrlKey || e.metaKey) && !e.shiftKey) deselectAll() }

  const onDoubleClick = (e: React.MouseEvent) => {
    e.stopPropagation()
    toggleDiagramFullscreen()
  }

  //#region Selection
  const onNodeDragStart =
    useCallback(
      (event: any, node: Node) => {
        const currentSelectedIds = selection?.selectedPieceIds ?? []
        const isNodeSelected = currentSelectedIds.includes(node.id)
        const ctrlKey = event.ctrlKey || event.metaKey
        const shiftKey = event.shiftKey

        if (ctrlKey) {
          if (isNodeSelected) {
            removePieceFromSelection({ id_: node.id })
          } else {
            addPieceToSelection({ id_: node.id })
          }
        } else if (shiftKey) {
          if (!isNodeSelected) {
            addPieceToSelection({ id_: node.id })
          }
        } else {
          if (!isNodeSelected) {
            selectPiece({ id_: node.id })
          }
        }

        // Start transaction for drag operation
        startTransaction()

        setDragState({
          lastPostition: { x: node.position.x, y: node.position.y }
        })
        setHelperLines([])
      },
      [selectPiece, removePieceFromSelection, addPieceToSelection, startTransaction]
    )

  const onNodeDrag = useCallback((event: any, node: Node) => {
    // TODO: Fix the reset after proximity connect is estabilished
    const MIN_DISTANCE = 150
    const SNAP_THRESHOLD = 20 // Distance in pixels for snapping to helper lines
    if (!dragState) return
    const { lastPostition } = dragState
    const metadata = piecesMetadata(kit, designId)

    // Calculate helper lines from non-selected pieces
    const currentHelperLines: HelperLine[] = []
    const viewport = reactFlowInstance.getViewport()

    const nonSelectedNodes = nodes.filter((n) => !(selection?.selectedPieceIds ?? []).includes(n.id))

    // Find the closest piece in each direction (positive x, negative x, positive y, negative y)
    const draggedCenterX = node.position.x + ICON_WIDTH / 2
    const draggedCenterY = node.position.y + ICON_WIDTH / 2

    const rightPiece = nonSelectedNodes
      .filter(n => n.position.x + ICON_WIDTH / 2 > draggedCenterX)
      .reduce((closest, current) => {
        const closestDistance = closest ? (closest.position.x + ICON_WIDTH / 2) - draggedCenterX : Infinity
        const currentDistance = (current.position.x + ICON_WIDTH / 2) - draggedCenterX
        return currentDistance < closestDistance ? current : closest
      }, null as typeof nonSelectedNodes[0] | null)

    const leftPiece = nonSelectedNodes
      .filter(n => n.position.x + ICON_WIDTH / 2 < draggedCenterX)
      .reduce((closest, current) => {
        const closestDistance = closest ? draggedCenterX - (closest.position.x + ICON_WIDTH / 2) : Infinity
        const currentDistance = draggedCenterX - (current.position.x + ICON_WIDTH / 2)
        return currentDistance < closestDistance ? current : closest
      }, null as typeof nonSelectedNodes[0] | null)

    const bottomPiece = nonSelectedNodes
      .filter(n => n.position.y + ICON_WIDTH / 2 > draggedCenterY)
      .reduce((closest, current) => {
        const closestDistance = closest ? (closest.position.y + ICON_WIDTH / 2) - draggedCenterY : Infinity
        const currentDistance = (current.position.y + ICON_WIDTH / 2) - draggedCenterY
        return currentDistance < closestDistance ? current : closest
      }, null as typeof nonSelectedNodes[0] | null)

    const topPiece = nonSelectedNodes
      .filter(n => n.position.y + ICON_WIDTH / 2 < draggedCenterY)
      .reduce((closest, current) => {
        const closestDistance = closest ? draggedCenterY - (closest.position.y + ICON_WIDTH / 2) : Infinity
        const currentDistance = draggedCenterY - (current.position.y + ICON_WIDTH / 2)
        return currentDistance < closestDistance ? current : closest
      }, null as typeof nonSelectedNodes[0] | null)

    const closestNodes = [rightPiece, leftPiece, bottomPiece, topPiece].filter(Boolean) as typeof nonSelectedNodes

    for (const otherNode of closestNodes) {
      const centerX = otherNode.position.x + ICON_WIDTH / 2
      const centerY = otherNode.position.y + ICON_WIDTH / 2

      // Add horizontal line through center
      currentHelperLines.push({
        type: 'horizontal',
        position: centerY,
        relatedPieceId: otherNode.id
      })

      // Add vertical line through center
      currentHelperLines.push({
        type: 'vertical',
        position: centerX,
        relatedPieceId: otherNode.id
      })
    }

    const addedConnections: Connection[] = []
    let updatedPieces: PieceDiff[] = []
    let updatedConnections: ConnectionDiff[] = []

    for (const selectedNode of nodes.filter((n) => selection?.selectedPieceIds.includes(n.id))) {
      const piece = selectedNode.data.piece
      const type = selectedNode.data.type
      let closestConnection: Connection | null = null
      let closestDistance = Number.MAX_VALUE
      const selectedInternalNode = reactFlowInstance.getInternalNode(selectedNode.id)!

      // Calculate current drag position with potential snapping
      let draggedX = node.position.x
      let draggedY = node.position.y

      // Check for snapping to helper lines
      const draggedCenterX = draggedX + ICON_WIDTH / 2
      const draggedCenterY = draggedY + ICON_WIDTH / 2

      // Check for equal distance snapping opportunities FIRST (before regular snapping)
      const EQUAL_DISTANCE_THRESHOLD = 15 // Pixels for snapping threshold
      let equalDistanceHelperLines: HelperLine[] = []

      for (let i = 0; i < closestNodes.length; i++) {
        for (let j = i + 1; j < closestNodes.length; j++) {
          const node1 = closestNodes[i]
          const node2 = closestNodes[j]

          const center1 = { x: node1.position.x + ICON_WIDTH / 2, y: node1.position.y + ICON_WIDTH / 2 }
          const center2 = { x: node2.position.x + ICON_WIDTH / 2, y: node2.position.y + ICON_WIDTH / 2 }

          // Check for horizontal alignment with equal vertical spacing
          if (Math.abs(center1.x - center2.x) < 5) { // Vertically aligned pieces
            const distance = Math.abs(center2.y - center1.y)
            const minY = Math.min(center1.y, center2.y)
            const maxY = Math.max(center1.y, center2.y)
            const midY = (center1.y + center2.y) / 2

            if (distance > 40) {
              // Check for middle position snapping
              if (Math.abs(draggedCenterY - midY) < EQUAL_DISTANCE_THRESHOLD) {
                draggedY = midY - ICON_WIDTH / 2

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `upper-${node1.id}-${node2.id}`,
                  x1: center1.x - 50,
                  y1: minY,
                  x2: center1.x + 50,
                  y2: minY,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `lower-${node1.id}-${node2.id}`,
                  x1: center1.x - 50,
                  y1: maxY,
                  x2: center1.x + 50,
                  y2: maxY,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `mid-${node1.id}-${node2.id}`,
                  x1: center1.x - 30,
                  y1: midY,
                  x2: center1.x + 30,
                  y2: midY,
                  referencePieceIds: [node1.id, node2.id]
                })
              }

              // Check for extending the sequence (placing before first or after last)
              const extendedMinY = minY - distance
              const extendedMaxY = maxY + distance

              if (Math.abs(draggedCenterY - extendedMinY) < EQUAL_DISTANCE_THRESHOLD) {
                draggedY = extendedMinY - ICON_WIDTH / 2

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `extend-before-${node1.id}-${node2.id}`,
                  x1: center1.x - 30,
                  y1: extendedMinY,
                  x2: center1.x + 30,
                  y2: extendedMinY,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `ref1-${node1.id}-${node2.id}`,
                  x1: center1.x - 50,
                  y1: minY,
                  x2: center1.x + 50,
                  y2: minY,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `ref2-${node1.id}-${node2.id}`,
                  x1: center1.x - 50,
                  y1: maxY,
                  x2: center1.x + 50,
                  y2: maxY,
                  referencePieceIds: [node1.id, node2.id]
                })
              }

              if (Math.abs(draggedCenterY - extendedMaxY) < EQUAL_DISTANCE_THRESHOLD) {
                draggedY = extendedMaxY - ICON_WIDTH / 2

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `extend-after-${node1.id}-${node2.id}`,
                  x1: center1.x - 30,
                  y1: extendedMaxY,
                  x2: center1.x + 30,
                  y2: extendedMaxY,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `ref1-${node1.id}-${node2.id}`,
                  x1: center1.x - 50,
                  y1: minY,
                  x2: center1.x + 50,
                  y2: minY,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `ref2-${node1.id}-${node2.id}`,
                  x1: center1.x - 50,
                  y1: maxY,
                  x2: center1.x + 50,
                  y2: maxY,
                  referencePieceIds: [node1.id, node2.id]
                })
              }

              // Check for perpendicular equal distance (horizontal placement relative to vertical alignment)
              const extendedLeftX = center1.x - distance
              const extendedRightX = center1.x + distance

              if (Math.abs(draggedCenterX - extendedLeftX) < EQUAL_DISTANCE_THRESHOLD) {
                draggedX = extendedLeftX - ICON_WIDTH / 2

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `perp-left-${node1.id}-${node2.id}`,
                  x1: extendedLeftX,
                  y1: midY - 30,
                  x2: extendedLeftX,
                  y2: midY + 30,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `perp-ref-${node1.id}-${node2.id}`,
                  x1: center1.x,
                  y1: midY - 50,
                  x2: center1.x,
                  y2: midY + 50,
                  referencePieceIds: [node1.id, node2.id]
                })
              }

              if (Math.abs(draggedCenterX - extendedRightX) < EQUAL_DISTANCE_THRESHOLD) {
                draggedX = extendedRightX - ICON_WIDTH / 2

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `perp-right-${node1.id}-${node2.id}`,
                  x1: extendedRightX,
                  y1: midY - 30,
                  x2: extendedRightX,
                  y2: midY + 30,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `perp-ref-${node1.id}-${node2.id}`,
                  x1: center1.x,
                  y1: midY - 50,
                  x2: center1.x,
                  y2: midY + 50,
                  referencePieceIds: [node1.id, node2.id]
                })
              }
            }
          }          // Check for vertical alignment with equal horizontal spacing
          if (Math.abs(center1.y - center2.y) < 5) { // Horizontally aligned pieces
            const distance = Math.abs(center2.x - center1.x)
            const minX = Math.min(center1.x, center2.x)
            const maxX = Math.max(center1.x, center2.x)
            const midX = (center1.x + center2.x) / 2

            if (distance > 40) {
              // Check for middle position snapping
              if (Math.abs(draggedCenterX - midX) < EQUAL_DISTANCE_THRESHOLD) {
                draggedX = midX - ICON_WIDTH / 2

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `left-${node1.id}-${node2.id}`,
                  x1: minX,
                  y1: center1.y - 50,
                  x2: minX,
                  y2: center1.y + 50,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `right-${node1.id}-${node2.id}`,
                  x1: maxX,
                  y1: center1.y - 50,
                  x2: maxX,
                  y2: center1.y + 50,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `mid-${node1.id}-${node2.id}`,
                  x1: midX,
                  y1: center1.y - 30,
                  x2: midX,
                  y2: center1.y + 30,
                  referencePieceIds: [node1.id, node2.id]
                })
              }

              // Check for extending the sequence (placing before first or after last)
              const extendedMinX = minX - distance
              const extendedMaxX = maxX + distance

              if (Math.abs(draggedCenterX - extendedMinX) < EQUAL_DISTANCE_THRESHOLD) {
                draggedX = extendedMinX - ICON_WIDTH / 2

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `extend-before-${node1.id}-${node2.id}`,
                  x1: extendedMinX,
                  y1: center1.y - 30,
                  x2: extendedMinX,
                  y2: center1.y + 30,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `ref1-${node1.id}-${node2.id}`,
                  x1: minX,
                  y1: center1.y - 50,
                  x2: minX,
                  y2: center1.y + 50,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `ref2-${node1.id}-${node2.id}`,
                  x1: maxX,
                  y1: center1.y - 50,
                  x2: maxX,
                  y2: center1.y + 50,
                  referencePieceIds: [node1.id, node2.id]
                })
              }

              if (Math.abs(draggedCenterX - extendedMaxX) < EQUAL_DISTANCE_THRESHOLD) {
                draggedX = extendedMaxX - ICON_WIDTH / 2

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `extend-after-${node1.id}-${node2.id}`,
                  x1: extendedMaxX,
                  y1: center1.y - 30,
                  x2: extendedMaxX,
                  y2: center1.y + 30,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `ref1-${node1.id}-${node2.id}`,
                  x1: minX,
                  y1: center1.y - 50,
                  x2: minX,
                  y2: center1.y + 50,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `ref2-${node1.id}-${node2.id}`,
                  x1: maxX,
                  y1: center1.y - 50,
                  x2: maxX,
                  y2: center1.y + 50,
                  referencePieceIds: [node1.id, node2.id]
                })
              }

              // Check for perpendicular equal distance (vertical placement relative to horizontal alignment)
              const extendedUpY = center1.y - distance
              const extendedDownY = center1.y + distance

              if (Math.abs(draggedCenterY - extendedUpY) < EQUAL_DISTANCE_THRESHOLD) {
                draggedY = extendedUpY - ICON_WIDTH / 2

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `perp-up-${node1.id}-${node2.id}`,
                  x1: midX - 30,
                  y1: extendedUpY,
                  x2: midX + 30,
                  y2: extendedUpY,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `perp-ref-${node1.id}-${node2.id}`,
                  x1: midX - 50,
                  y1: center1.y,
                  x2: midX + 50,
                  y2: center1.y,
                  referencePieceIds: [node1.id, node2.id]
                })
              }

              if (Math.abs(draggedCenterY - extendedDownY) < EQUAL_DISTANCE_THRESHOLD) {
                draggedY = extendedDownY - ICON_WIDTH / 2

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `perp-down-${node1.id}-${node2.id}`,
                  x1: midX - 30,
                  y1: extendedDownY,
                  x2: midX + 30,
                  y2: extendedDownY,
                  referencePieceIds: [node1.id, node2.id]
                })

                equalDistanceHelperLines.push({
                  type: 'equalDistance',
                  relatedPieceId: `perp-ref-${node1.id}-${node2.id}`,
                  x1: midX - 50,
                  y1: center1.y,
                  x2: midX + 50,
                  y2: center1.y,
                  referencePieceIds: [node1.id, node2.id]
                })
              }
            }
          }
        }
      }

      // Always check regular snapping (combining with equal distance snapping)
      // Update drag position based on current snapped position
      const updatedDraggedCenterX = draggedX + ICON_WIDTH / 2
      const updatedDraggedCenterY = draggedY + ICON_WIDTH / 2

      // Find closest horizontal line for snapping
      for (const line of currentHelperLines.filter(l => l.type === 'horizontal' && l.position !== undefined)) {
        const distance = Math.abs(updatedDraggedCenterY - line.position!)
        if (distance < SNAP_THRESHOLD) {
          draggedY = line.position! - ICON_WIDTH / 2
          break
        }
      }

      // Find closest vertical line for snapping
      for (const line of currentHelperLines.filter(l => l.type === 'vertical' && l.position !== undefined)) {
        const distance = Math.abs(updatedDraggedCenterX - line.position!)
        if (distance < SNAP_THRESHOLD) {
          draggedX = line.position! - ICON_WIDTH / 2
          break
        }
      }

      // Add equal distance helper lines to the current helper lines
      currentHelperLines.push(...equalDistanceHelperLines)

      // Update helper lines with equal distance lines
      setHelperLines(currentHelperLines)

      // Update the node position with snapping applied
      if (selectedNode.id === node.id) {
        selectedInternalNode.internals.positionAbsolute.x = draggedX
        selectedInternalNode.internals.positionAbsolute.y = draggedY
        node.position.x = draggedX
        node.position.y = draggedY
      }

      for (const otherNode of nodes.filter((n) => !(selection.selectedPieceIds ?? []).includes(n.id))) {
        const existingConnection = design.connections?.find((c) => isSameConnection(c, { connected: { piece: { id_: selectedNode.id } }, connecting: { piece: { id_: otherNode.id } } }))
        if (existingConnection) continue
        const otherInternalNode = reactFlowInstance.getInternalNode(otherNode.id)!
        for (const handle of selectedInternalNode.internals.handleBounds?.source ?? []) {
          const port = findPortInType(type, { id_: handle.id! })
          for (const otherHandle of otherInternalNode.internals.handleBounds?.source ?? []) {
            const otherPort = findPortInType(otherNode.data.type, { id_: otherHandle.id! })
            if (!arePortsCompatible(port, otherPort) || isPortInUse(design, otherNode.data.piece, otherPort)) continue
            const dx = (selectedInternalNode.internals.positionAbsolute.x + handle.x) - (otherInternalNode.internals.positionAbsolute.x + otherHandle.x)
            const dy = (selectedInternalNode.internals.positionAbsolute.y + handle.y) - (otherInternalNode.internals.positionAbsolute.y + otherHandle.y)
            const distance = Math.sqrt(dx * dx + dy * dy)
            if (distance < closestDistance && distance < MIN_DISTANCE) {
              closestConnection = {
                connected: { piece: { id_: otherNode.id }, port: { id_: otherHandle.id! } },
                connecting: { piece: { id_: selectedInternalNode.id }, port: { id_: handle.id! } },
                x: ((selectedInternalNode.internals.positionAbsolute.x + handle.x) - (otherInternalNode.internals.positionAbsolute.x + otherHandle.x)) / ICON_WIDTH,
                y: -(((selectedInternalNode.internals.positionAbsolute.y + handle.y) - (otherInternalNode.internals.positionAbsolute.y + otherHandle.y)) / ICON_WIDTH)
              }
              closestDistance = distance
            }
          }
        }
      }

      if (closestConnection) {
        addedConnections.push(closestConnection)
        const updatedPiece = { ...selectedNode.data.piece, center: undefined, plane: undefined }
        if (updatedPiece.type) {
          updatedPieces.push(updatedPiece as Piece)
        }
      }
      else {
        if (!piece.center) {

          const parentPieceId = metadata.get(selectedNode.id)!.parentPieceId!
          const parentInternalNode = reactFlowInstance.getInternalNode(parentPieceId)
          if (!parentInternalNode) throw new Error(`Internal node not found for ${parentPieceId}`)
          const parentConnection = findConnectionInDesign(design, { connected: { piece: { id_: selectedNode.id } }, connecting: { piece: { id_: parentPieceId } } })
          // const isSelectedConnecting = parentConnection.connecting.piece.id_ === selectedNode.id
          // const selectedInternalNode = reactFlowInstance.getInternalNode(selectedNode.id)!
          // const handle = selectedInternalNode.internals.handleBounds?.source?.find((h) => h.id === (isSelectedConnecting ? parentConnection.connected.port.id_ : parentConnection.connecting.port.id_))
          // if (!handle) throw new Error(`Handle not found for ${parentConnection.connecting.port.id_}`)
          // const parentHandle = parentInternalNode.internals.handleBounds?.source?.find((h) => h.id === (isSelectedConnecting ? parentConnection.connecting.port.id_ : parentConnection.connected.port.id_))
          // if (!parentHandle) throw new Error(`Handle not found for ${parentConnection.connected.port.id_}`)
          updatedConnections.push({
            ...parentConnection,
            x: (parentConnection.x ?? 0) + ((draggedX - lastPostition.x) / ICON_WIDTH),
            y: (parentConnection.y ?? 0) - ((draggedY - lastPostition.y) / ICON_WIDTH)
          })
        }
        else {
          const scaledOffset = { x: (draggedX - lastPostition.x) / ICON_WIDTH, y: -(draggedY - lastPostition.y) / ICON_WIDTH }
          const updatedPiece = { ...piece, center: { x: piece.center!.x + scaledOffset.x, y: piece.center!.y + scaledOffset.y } }
          if (updatedPiece.type) {
            updatedPieces.push(updatedPiece as Piece)
          }
        }
      }

      if (updatedPieces.length > 0) {
        updatedPieces.forEach(piece => {
          if (piece.type) {
            updatePiece(piece as Piece)
          }
        })
      }

      if (addedConnections.length > 0) {
        addConnectionsToDesign(addedConnections)
      }

      if (updatedConnections.length > 0) {
        updatedConnections.forEach(connection => {
          updateConnection(connection as Connection)
        })
      }

      setDragState({ ...dragState!, lastPostition: { x: draggedX, y: draggedY } })

    }

  }, [updatePiece, addConnectionsToDesign, updateConnection, design, reactFlowInstance, selection, nodes])

  const onNodeDragStop = useCallback(() => {
    // Finalize transaction instead of manually applying diff
    finalizeTransaction()
    setDragState(null)
    setHelperLines([])
  }, [finalizeTransaction])
  //#endregion

  const onConnect = useCallback(
    (params: RFConnection) => {
      if (params.source === params.target) return

      const sourceInternalNode = reactFlowInstance.getInternalNode(params.source)
      const targetInternalNode = reactFlowInstance.getInternalNode(params.target)
      if (!sourceInternalNode || !targetInternalNode) return

      const sourceHandle = (sourceInternalNode.internals.handleBounds?.source ?? []).find((h) => h.id === params.sourceHandle)
      const targetHandle = (targetInternalNode.internals.handleBounds?.source ?? []).find((h) => h.id === params.targetHandle)
      if (!sourceHandle || !targetHandle) return

      const newConnection = {
        connected: { piece: { id_: params.source! }, port: { id_: params.sourceHandle! } },
        connecting: { piece: { id_: params.target! }, port: { id_: params.targetHandle! } },
        x: ((sourceInternalNode.internals.positionAbsolute.x + sourceHandle.x) - (targetInternalNode.internals.positionAbsolute.x + targetHandle.x)) / ICON_WIDTH,
        y: -(((sourceInternalNode.internals.positionAbsolute.y + sourceHandle.y) - (targetInternalNode.internals.positionAbsolute.y + targetHandle.y)) / ICON_WIDTH)
      }

      const design = findDesignInKit(kit, designId)
      if ((design.connections ?? []).find((c) => isSameConnection(c, newConnection))) return
      const newConnections = [...(design.connections ?? []), newConnection]
      const updatedPieces = design.pieces?.map((piece) => (isSamePiece(piece, { id_: params.source! })) ? { ...piece, center: undefined, plane: undefined } : piece)
      setDesign({ ...design, connections: newConnections, pieces: updatedPieces })
    },
    [setDesign, kit, designId, reactFlowInstance, design]
  )
  //#endregion Actions

  const nodeTypes = useMemo(() => ({ piece: PieceNodeComponent }), [])
  const edgeTypes = useMemo(() => ({ connection: ConnectionEdgeComponent }), [])

  return (
    <div id="diagram" className="h-full w-full relative">
      <ReactFlow
        ref={useDroppable({ id: 'diagram' }).setNodeRef}
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        connectionMode={ConnectionMode.Loose}
        elementsSelectable={false}
        minZoom={0.1}
        maxZoom={12}
        fitView
        zoomOnDoubleClick={false}
        panOnDrag={[0]}
        proOptions={{ hideAttribution: true }}
        onNodeClick={onNodeClick}
        onEdgeClick={onEdgeClick}
        onNodeDragStart={onNodeDragStart}
        onNodeDrag={onNodeDrag}
        onNodeDragStop={onNodeDragStop}
        onPaneClick={onPaneClick}
        onDoubleClick={onDoubleClick}
        onConnect={onConnect}
        connectionLineComponent={ConnectionConnectionLine}
        className="bg-background"
      >
        {fullscreen && <Controls className="border border-border bg-background rounded-md shadow-sm" showZoom={false} showInteractive={false} />}
        {fullscreen && (
          <MiniMap
            className="border border-border bg-background rounded-md shadow-sm"
            maskColor="var(--accent)"
            bgColor="var(--background)"
            nodeComponent={MiniMapNode}
          />
        )}
        <ViewportPortal>⌞</ViewportPortal>
      </ReactFlow>
      <HelperLines lines={helperLines} nodes={nodes} />
    </div>
  )
}

export default Diagram