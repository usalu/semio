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
import React, { FC, useState } from 'react'

import {
  applyDesignDiff,
  Connection,
  Design,
  DesignDiff,
  DesignId,
  DiagramPoint,
  DiffStatus,
  findDesign,
  findType,
  flattenDesign,
  ICON_WIDTH,
  Kit,
  Piece,
  Port,
  sameDesign,
  Type
} from '@semio/js'

// import '@xyflow/react/dist/base.css';
import '@semio/js/globals.css'
import '@xyflow/react/dist/style.css'
import { DesignEditorAction, DesignEditorDispatcher, DesignEditorSelection, DesignEditorState } from '../..'


//#region React Flow

type PieceNodeProps = {
  piece: Piece
  type: Type
}

type PieceNode = Node<PieceNodeProps, 'piece'>
type DiagramNode = PieceNode

type ConnectionEdge = Edge<{ connection: Connection }, 'connection'>
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
    <Handle id={port.id_} type="source" style={{ left: x + ICON_WIDTH / 2, top: y }} position={Position.Top} />
  )
}

const PieceNodeComponent: React.FC<NodeProps<PieceNode>> = React.memo(({ id, data, selected }) => {
  const {
    piece: { id_, qualities },
    type: { ports }
  } = data as PieceNodeProps & { diffStatus: DiffStatus }

  let fillClass = ''
  let strokeClass = 'stroke-foreground'
  let opacity = 1

  const diff = qualities?.find(q => q.name === 'semio.diffStatus')?.value as DiffStatus || DiffStatus.Unchanged

  if (diff === DiffStatus.Added) {
    fillClass = selected ? 'fill-green-600' : 'fill-green-400'
  } else if (diff === DiffStatus.Removed) {
    fillClass = 'fill-red-400'
    strokeClass = 'stroke-red-600'
    opacity *= 0.5 // transparent
  } else if (diff === DiffStatus.Modified) {
    fillClass = selected ? 'fill-yellow-600' : 'fill-yellow-400'
  } else {
    fillClass = selected ? 'fill-primary' : 'fill-transparent'
  }

  return (
    <div style={{ opacity }}>
      <svg width={ICON_WIDTH} height={ICON_WIDTH} className="cursor-pointer">
        <circle
          cx={ICON_WIDTH / 2}
          cy={ICON_WIDTH / 2}
          r={ICON_WIDTH / 2 - 1}
          className={`${strokeClass} stroke-2 ${fillClass}`}
        />
        <text
          x={ICON_WIDTH / 2}
          y={ICON_WIDTH / 2}
          textAnchor="middle"
          dominantBaseline="middle"
          className={`text-xs font-bold fill-foreground`}
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

  const isIntermediate = data?.connection?.qualities?.some(q => q.name === 'semio.intermediate') ?? id?.startsWith('intermediate-edge');
  const diff = data?.connection?.qualities?.find(q => q.name === 'semio.diffStatus')?.value as DiffStatus || DiffStatus.Unchanged;

  let stroke = 'var(--foreground)';
  let dasharray: string | undefined;
  let opacity = isIntermediate ? 0.5 : 1;

  if (selected) {
    stroke = 'var(--primary)';
  } else if (diff === DiffStatus.Added) {
    stroke = 'green';
    dasharray = '5 5';
  } else if (diff === DiffStatus.Removed) {
    stroke = 'red';
    opacity = 0.5;
  } else if (diff === DiffStatus.Modified) {
    stroke = 'yellow';
  }

  return (
    <BaseEdge
      path={path}
      style={{
        strokeDasharray: dasharray,
        stroke,
        opacity
      }}
    />
  )
}

const ConnectionConnectionLine: React.FC<ConnectionLineComponentProps> = (props: ConnectionLineComponentProps) => {
  const { fromX, fromY, toX, toY } = props
  const HANDLE_HEIGHT = 5
  const path = `M ${fromX} ${fromY + HANDLE_HEIGHT / 2} L ${toX} ${toY + HANDLE_HEIGHT / 2}`
  return <BaseEdge path={path} style={{ stroke: 'grey' }} />
}

export const MiniMapNode: React.FC<MiniMapNodeProps> = ({ x, y, selected }: MiniMapNodeProps) => {
  return <circle className={selected ? 'fill-primary' : 'fill-foreground'} cx={x} cy={y} r="10" />
}

const nodeTypes = { piece: PieceNodeComponent }
const edgeTypes = { connection: ConnectionEdgeComponent }

const pieceToNode = (piece: Piece, type: Type, center: DiagramPoint, selected: boolean): PieceNode => ({
  type: 'piece',
  id: piece.id_,
  position: {
    x: center.x * ICON_WIDTH || 0,
    y: -center.y * ICON_WIDTH || 0
  },
  selected,
  data: { piece, type }
})

const connectionToEdge = (connection: Connection, selected: boolean): ConnectionEdge => ({
  type: 'connection',
  id: `${connection.connecting.piece.id_} -- ${connection.connected.piece.id_}`,
  source: connection.connecting.piece.id_,
  sourceHandle: connection.connecting.port.id_,
  target: connection.connected.piece.id_,
  targetHandle: connection.connected.port.id_,
  data: { connection },
  selected
})

const designToNodesAndEdges = (kit: Kit, designId: DesignId, selection: DesignEditorSelection, designDiff: DesignDiff) => {
  const design = findDesign(kit, designId)
  if (!design) return null
  const effectiveDesign = applyDesignDiff(design, designDiff, true)
  const effectiveKit = {
    ...kit,
    designs: kit.designs!.map((d: Design) => {
      if (sameDesign(design, d)) {
        return effectiveDesign
      }
      return d
    })
  }
  const centers = flattenDesign(effectiveKit, designId).pieces?.map(p => p.center)
  const pieceNodes = effectiveDesign.pieces?.map(
    (piece, i) => pieceToNode(piece, findType(kit, piece.type)!, centers![i]!, selection?.selectedPieceIds.includes(piece.id_) ?? false)) ?? []
  const connectionEdges =
    effectiveDesign.connections?.map((connection) => connectionToEdge(connection, selection?.selectedConnections.some(
      (c) =>
        c.connectingPieceId === connection.connecting.piece.id_ &&
        c.connectedPieceId === connection.connected.piece.id_
    ) ?? false)) ?? []
  return { nodes: pieceNodes, edges: connectionEdges }
}

//#endregion


interface DiagramProps {
  designEditorState: DesignEditorState
  designEditorDispatcher: DesignEditorDispatcher
}

const Diagram: FC<DiagramProps> = ({
  designEditorState,
  designEditorDispatcher
}) => {

  //#region State
  const { kit, designId, selection, fullscreenPanel, designDiff } = designEditorState
  const [dragState, setDragState] = useState<{
    origin: XYPosition
    offset: XYPosition | null
    diff: DesignDiff | null
  } | null>(null)
  const fullscreen = fullscreenPanel === 'diagram'
  const onDesignChange = (d: Design) => designEditorDispatcher({ type: DesignEditorAction.SetDesign, payload: d })
  const onSelectionChange = (s: DesignEditorSelection) => designEditorDispatcher({ type: DesignEditorAction.SetSelection, payload: s })
  if (!kit) return null
  //#endregion

  const { nodes, edges } = designToNodesAndEdges(kit, designId, selection, designDiff) ?? { nodes: [], edges: [] }
  const reactFlowInstance = useReactFlow()

  //#region Dragging
  const handleNodeDragStart =
    (event: any, node: Node) => {
      const currentSelectedIds = selection?.selectedPieceIds ?? []
      const isNodeSelected = currentSelectedIds.includes(node.id)

      if (!isNodeSelected) {
        onSelectionChange({
          selectedPieceIds: [...currentSelectedIds, node.id],
          selectedConnections: []
        })
      }

      setDragState({
        origin: { x: node.position.x, y: node.position.y },
        offset: { x: 0, y: 0 },
        diff: null
      })
    }

  const handleNodeDrag = (event: any, node: Node) => {

    const proximityConnections: Connection[] = []
    for (const selectedNode of nodes.filter((n) => selection?.selectedPieceIds.includes(n.id))) {

      const MIN_DISTANCE = 150
      const pieceNode = selectedNode as PieceNode
      const ports = pieceNode.data.type.ports || []
      const handlePositions = ports.map((port) => {
        const { x: portX, y: portY } = getPortPositionStyle(port)
        const draggedHandles = {
          handleId: port.id_ || '',
          x: pieceNode.position.x + portX + ICON_WIDTH / 2,
          y: pieceNode.position.y + portY
        }
      })

      let closestHandle = {
        distance: Number.MAX_VALUE,
        source: null as null | { nodeId: string; handleId: string },
        target: null as null | { nodeId: string; handleId: string },
        dx: 0,
        dy: 0
      }

      const originalDraggedId = selectedNode.id.slice(12)
      const selectedNodeIds = selection.selectedPieceIds ?? []

      // Iterate over all other nodes in the array
      for (const otherNode of nodes) {
        // Skip the selectedNode itself, the original dragged selectedNode, and any selected nodes
        if (|| otherNode.id === originalDraggedId || selectedNodeIds.includes(otherNode.id))
          continue
        const otherInternalNode = reactFlowInstance.getInternalNode(otherNode.id)
        if (!otherInternalNode) continue
        // Get all handles on the other selectedNode
        const handles = otherInternalNode.internals.handleBounds?.source ?? []
        const otherHandles = handles
          .filter((handle) => handle.id !== null && handle.id !== undefined)
          .map((handle) => ({
            handleId: handle.id as string,
            x: otherInternalNode.internals.positionAbsolute.x + handle.x + handle.width / 2,
            y: otherInternalNode.internals.positionAbsolute.y + handle.y + handle.height / 2
          }))
        // Compare all handles between dragged selectedNode and other selectedNode
        for (const draggedHandle of draggedHandles) {
          for (const otherHandle of otherHandles) {
            const dx = draggedHandle.x - otherHandle.x
            const dy = draggedHandle.y - otherHandle.y
            const distance = Math.sqrt(dx * dx + dy * dy)
            if (distance < closestHandle.distance && distance < MIN_DISTANCE) {
              // Always assign the selectedNode with the lower id as source for consistency
              if (node.id < otherNode.id) {
                // source: node, target: otherNode. Vector from source to target is other - node = -dx, -dy
                return {
                  distance,
                  source: {
                    nodeId: node.id,
                    handleId: draggedHandle.handleId
                  },
                  target: {
                    nodeId: otherNode.id,
                    handleId: otherHandle.handleId
                  },
                  dx: -dx,
                  dy: -dy
                }
              } else {
                // source: otherNode, target: node. Vector from source to target is node - other = dx, dy
                closestHandle = {
                  distance,
                  source: {
                    nodeId: otherNode.id,
                    handleId: otherHandle.handleId
                  },
                  target: {
                    nodeId: node.id,
                    handleId: draggedHandle.handleId
                  },
                  dx: dx,
                  dy: dy
                }
              }
            }
          }
        }

        const closestEdge = {
          id: `${closestHandle.source.nodeId}-${closestHandle.source.handleId}__${closestHandle.target.nodeId}-${closestHandle.target.handleId}`,
          source: closestHandle.source.nodeId,
          sourceHandle: closestHandle.source.handleId,
          target: closestHandle.target.nodeId,
          targetHandle: closestHandle.target.handleId,
          data: { dx: closestHandle.dx, dy: closestHandle.dy }
        }

        proximityEdges.push({
          ...closestEdge,
          id: 'intermediate-edge-' + selectedNode.id,
          className: 'temp'
        })

      }
    }

    // Recreate intermediate nodes to calculate proximity edges
    const selectedNodeIds = new Set(selection.selectedPieceIds ?? [])
    const intermediateNodes: PieceNode[] = Array.from(selectedNodeIds)
      .map((id) => {
        const orig = nodes.find((n) => n.id === id)
        if (!orig) return undefined
        return {
          ...orig,
          id: 'intermediate' + id,
          position: {
            x: orig.position.x + offset.x,
            y: orig.position.y + offset.y
          },
          data: { ...orig.data, isIntermediate: true }
        }
      })
      .filter(Boolean) as PieceNode[]

    const design = getDesign(kit, designId)

    if (design) {
      const scaledOffset = {
        x: offset.x / ICON_WIDTH,
        y: -offset.y / ICON_WIDTH
      }

      // If there are no proximity edges, this is a simple position update
      if (dragState.diff) {
        const updatedPieces = design.pieces?.map((piece) => {
          // If a dragged piece, just update its position
          if (selectedNodeIds.has(piece.id_) && piece.center) {
            return {
              ...piece,
              center: {
                x: piece.center.x + scaledOffset.x,
                y: piece.center.y + scaledOffset.y
              }
            }
          }
          // Other pieces are unchanged
          return piece
        })

        const newDesignDiffAfter = {
          ...designDiff,
          pieces: updatedPieces
        };

        if (JSON.stringify(newDesignDiffAfter) === JSON.stringify(designDiff)) return;
        onDesignChange({
          ...design,
          pieces: updatedPieces
        })
        return
      }

      // Handle connection creation when proximity edges exist
      const newChildPieceIds = new Set<string>()

      // Convert proximity edges to new Semio connections
      const newConnections = dragState.diff!.connections.
          .map((edge): Connection | null => {
        const sourceId = edge.source!.startsWith('intermediate') ? edge.source!.substring(12) : edge.source!
        const targetId = edge.target!.startsWith('intermediate') ? edge.target!.substring(12) : edge.target!

        // Validate connection: no self-connections
        if (sourceId === targetId) return null

        // Validate handles exist
        if (!edge.sourceHandle || !edge.targetHandle) return null

        // Ensure we don't connect two dragged pieces to each other
        const sourceDragged = selectedNodeIds.has(sourceId)
        const targetDragged = selectedNodeIds.has(targetId)
        if (sourceDragged && targetDragged) return null

        // Identify which piece was dragged to become a child
        if (sourceDragged) newChildPieceIds.add(sourceId)
        if (targetDragged) newChildPieceIds.add(targetId)

        const dx = typeof edge.data?.dx === 'number' ? edge.data.dx : 0
        const dy = typeof edge.data?.dy === 'number' ? edge.data.dy : 0

        const scaledDx = dx / ICON_WIDTH
        const scaledDy = -dy / ICON_WIDTH

        // Validate that the offset is not zero (required by the backend)
        if (Math.abs(scaledDx) < 0.001 && Math.abs(scaledDy) < 0.001) {
          console.warn('Skipping connection with zero offset:', { sourceId, targetId, dx, dy })
          return null
        }

        const newConnection = {
          connecting: { piece: { id_: sourceId }, port: { id_: edge.sourceHandle } },
          connected: { piece: { id_: targetId }, port: { id_: edge.targetHandle } },
          description: '',
          gap: 0,
          shift: 0,
          rise: 0,
          rotation: 0,
          turn: 0,
          tilt: 0,
          x: scaledDx,
          y: scaledDy
        }

        // Debug log to help identify issues
        console.log('Creating proximity connection:', {
          source: sourceId,
          target: targetId,
          sourceHandle: edge.sourceHandle,
          targetHandle: edge.targetHandle,
          dx: scaledDx,
          dy: scaledDy
        })

        return newConnection
      })
        .filter(Boolean) as Connection[]

      // If no valid connections were created, treat this as a simple position update
      if (newConnections.length === 0) {
        const updatedPieces = design.pieces?.map((piece) => {
          // If a dragged piece, just update its position
          if (selectedNodeIds.has(piece.id_) && piece.center) {
            return {
              ...piece,
              center: {
                x: piece.center.x + scaledOffset.x,
                y: piece.center.y + scaledOffset.y
              }
            }
          }
          // Other pieces are unchanged
          return piece
        })

        const newDesignDiffAfter = {
          ...designDiff,
          pieces: updatedPieces
        };

        if (JSON.stringify(newDesignDiffAfter) === JSON.stringify(designDiff)) return;
        onDesignChange({
          ...design,
          pieces: updatedPieces
        })
        return
      }

      const updatedPieces = design.pieces?.map((piece) => {
        // If a dragged piece is now connected, it becomes a child.
        // Its absolute position is removed, to be derived from its parent.
        if (newChildPieceIds.has(piece.id_)) {
          const { center, plane, ...rest } = piece
          return rest
        }

        // If a dragged piece was NOT connected, it just moves.
        if (selectedNodeIds.has(piece.id_) && piece.center) {
          return {
            ...piece,
            center: {
              x: piece.center.x + scaledOffset.x,
              y: piece.center.y + scaledOffset.y
            }
          }
        }

        // Other pieces are unchanged.
        return piece
      })

      // Filter out old connections of moved pieces and add new ones
      const originalConnections = design.connections ?? []
      const preservedConnections = originalConnections.filter((c) => {
        const sourceSelected = selectedNodeIds.has(c.connecting.piece.id_)
        const targetSelected = selectedNodeIds.has(c.connected.piece.id_)
        // Keep connections that are not attached to moved pieces OR are internal to the selection
        return (!sourceSelected && !targetSelected) || (sourceSelected && targetSelected)
      })

      // Check for duplicates before adding new connections
      const validNewConnections: Connection[] = []
      const processedConnectionKeys = new Set<string>()

      for (const newConn of newConnections) {
        const newIds = [newConn.connecting.piece.id_, newConn.connected.piece.id_].sort().join('--')

        // Check against existing preserved connections
        const isDuplicatePreserved = preservedConnections.some((c) => {
          const existingIds = [c.connecting.piece.id_, c.connected.piece.id_].sort().join('--')
          return newIds === existingIds
        })

        // Check if we've already processed this connection pair
        if (!isDuplicatePreserved && !processedConnectionKeys.has(newIds)) {
          validNewConnections.push(newConn)
          processedConnectionKeys.add(newIds)
        }
      }

      const allConnections = [...preservedConnections, ...validNewConnections]
      const newDesignDiffAfter = {
        ...designDiff,
        pieces: updatedPieces,
        connections: allConnections
      };

      if (JSON.stringify(newDesignDiffAfter) === JSON.stringify(designDiff)) return;
      onDesignChange({
        ...design,
        pieces: updatedPieces,
        connections: allConnections
      })
    }
  }

  const handleNodeDragStop = () => {
    setDragState(null)
  }

  //#endregion

  const onConnect =
    (params: RFConnection) => {
      if (params.source === params.target) return

      const sourceInternal = reactFlowInstance.getInternalNode(params.source)
      const targetInternal = reactFlowInstance.getInternalNode(params.target)

      if (!sourceInternal || !targetInternal) return

      const sourceHandles = sourceInternal.internals.handleBounds?.source ?? []
      const targetHandles = targetInternal.internals.handleBounds?.target ?? []

      const sourceHandle = sourceHandles.find((h) => h.id === params.sourceHandle)
      const targetHandle = targetHandles.find((h) => h.id === params.targetHandle)

      if (!sourceHandle || !targetHandle) return

      const sourcePos = {
        x: sourceInternal.internals.positionAbsolute.x + sourceHandle.x + sourceHandle.width / 2,
        y: sourceInternal.internals.positionAbsolute.y + sourceHandle.y + sourceHandle.height / 2
      }

      const targetPos = {
        x: targetInternal.internals.positionAbsolute.x + targetHandle.x + targetHandle.width / 2,
        y: targetInternal.internals.positionAbsolute.y + targetHandle.y + targetHandle.height / 2
      }

      const dx = targetPos.x - sourcePos.x
      const dy = targetPos.y - sourcePos.y

      const scaledX = dx / ICON_WIDTH
      const scaledY = -dy / ICON_WIDTH

      const design = findDesign(kit, designId)

      if (!design || !onDesignChange) return

      const newConnection = {
        connecting: { piece: { id_: params.source! }, port: { id_: params.sourceHandle! } },
        connected: { piece: { id_: params.target! }, port: { id_: params.targetHandle! } },
        description: '',
        gap: 0,
        shift: 0,
        rise: 0,
        rotation: 0,
        turn: 0,
        tilt: 0,
        x: scaledX,
        y: scaledY
      }

      const originalConnections = design.connections ?? []

      const idsA = [newConnection.connecting.piece.id_, newConnection.connected.piece.id_].sort().join('--')

      const isDuplicate = originalConnections.some((c) => {
        const idsB = [c.connecting.piece.id_, c.connected.piece.id_].sort().join('--')
        return idsA === idsB
      })

      if (isDuplicate) return

      const newConnections = [...originalConnections, newConnection]

      // Update the target piece to remove center and plane (it becomes a child)
      const updatedPieces = design.pieces?.map((piece) => {
        if (piece.id_ === params.target) {
          const { center, plane, ...rest } = piece
          return rest
        }
        return piece
      })

      onDesignChange({ ...design, connections: newConnections, pieces: updatedPieces })
    }


  return (
    <div id="diagram" className="h-full w-full">
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
        zoomOnDoubleClick={false}
        panOnDrag={[0]} //left mouse button
        proOptions={{ hideAttribution: true }}
        selectionOnDrag={true}
        onNodeClick={(event, node) => {
          event?.stopPropagation()
          const currentSelectedIds = selection?.selectedPieceIds ?? []
          const isNodeSelected = currentSelectedIds.includes(node.id)

          // Multi-select: toggle node in selection
          if (isNodeSelected) {
            onSelectionChange({
              selectedPieceIds: currentSelectedIds.filter((id) => id !== node.id),
              selectedConnections: []
            })
          } else {
            onSelectionChange({
              selectedPieceIds: [...currentSelectedIds, node.id],
              selectedConnections: []
            })
          }
        }}
        onNodeDragStart={handleNodeDragStart}
        onNodeDrag={handleNodeDrag}
        onNodeDragStop={handleNodeDragStop}
        onPaneClick={() => onSelectionChange({ selectedPieceIds: [], selectedConnections: [] })}
        onDoubleClick={(e: React.MouseEvent) => {
          e.stopPropagation()
          designEditorDispatcher({
            type: DesignEditorAction.SetFullscreen,
            payload: fullscreen ? null : 'diagram'
          })
        }}
        onConnect={onConnect}
        connectionLineComponent={ConnectionConnectionLine}
      >
        {fullscreen && <Controls className="border" showZoom={false} showInteractive={false} />}
        {fullscreen && (
          <MiniMap
            className="border"
            maskColor="var(--accent)"
            bgColor="var(--background)"
            nodeComponent={MiniMapNode}
          />
        )}
        <ViewportPortal>⌞</ViewportPortal>
      </ReactFlow>
    </div>
  )
}

export default Diagram