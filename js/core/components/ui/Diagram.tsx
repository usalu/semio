import React, { FC, useCallback, useMemo, useState } from 'react'
import {
  BaseEdge,
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
  useReactFlow,
  ViewportPortal,
  XYPosition,
  InternalNode,
  ReactFlowInstance
} from '@xyflow/react'
import { useDroppable } from '@dnd-kit/core'

import { Connection, Design, DesignId, flattenDesign, ICON_WIDTH, Kit, Piece, Port, Type } from '@semio/js/semio'

// import '@xyflow/react/dist/base.css';
import '@xyflow/react/dist/style.css'
import '@semio/js/globals.css'
import { DesignEditorSelection } from '../..'

//#region Diagram Component

const Diagram: FC<DiagramProps> = ({
  kit,
  designId,
  fullscreen,
  selection,
  onPanelDoubleClick,
  onDesignChange,
  onSelectionChange
}) => {
  if (!kit) return null // Prevents error if kit is undefined
  // Mapping the semio design to react flow nodes and edges
  const nodesAndEdges = useMemo(
    () => mapDesignToNodesAndEdges({ kit, designId, selection }),
    [kit, designId, selection]
  )
  if (!nodesAndEdges) return null
  const { nodes, edges: edges } = nodesAndEdges

  // Drage handling
  const [dragState, setDragState] = useState<{
    origin: XYPosition
    offset: XYPosition
  } | null>(null)

  const { handleNodeDragStart, handleNodeDrag, handleNodeDragStop } = useDragHandle(
    dragState,
    setDragState,
    selection,
    onSelectionChange,
    kit,
    designId,
    onDesignChange
  )

  // Double click handling
  const onDoubleClickCapture = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation()
      onPanelDoubleClick?.()
    },
    [onPanelDoubleClick]
  )

  // Ghost nodes rendering
  const displayNodes = useDisplayGhostNodes(dragState, nodes, selection)

  // Ghost edges rendering
  const displayEdges = useDisplayGhostEdges(displayNodes, nodes, selection, edges)

  return (
    <div id="diagram" className="h-full w-full">
      <ReactFlow
        ref={useDiagramDroppableNodeRef()}
        nodes={displayNodes}
        edges={displayEdges}
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
          toggleNodeSelection(node.id, selection, onSelectionChange, event)
        }}
        onNodeDragStart={handleNodeDragStart}
        onNodeDrag={handleNodeDrag}
        onNodeDragStop={handleNodeDragStop}
        onDoubleClick={onDoubleClickCapture}
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

interface DiagramProps {
  kit: Kit
  designId: DesignId
  fileUrls: Map<string, string>
  fullscreen?: boolean
  selection: DesignEditorSelection
  onDesignChange?: (design: Design) => void
  onSelectionChange: (selection: DesignEditorSelection) => void
  onPanelDoubleClick?: () => void
}

export default Diagram

//#endregion

//#region Proximity Edges

function getProximityEdges(
  displayNodes: PieceNode[],
  nodes: PieceNode[],
  reactFlowInstance: ReactFlowInstance,
  selection: DesignEditorSelection
): Edge[] {
  const ghostNodes = displayNodes.filter((node) => node.data.isGhost)

  if (ghostNodes.length === 0) {
    return []
  }

  const proximityEdges: Edge[] = []
  for (const ghostNode of ghostNodes) {
    const closestEdge = getClosestEdge(ghostNode, nodes, reactFlowInstance.getInternalNode, selection)

    if (closestEdge) {
      proximityEdges.push({
        id: 'ghost-edge-' + ghostNode.id,
        source: closestEdge.source,
        sourceHandle: closestEdge.sourceHandle,
        target: closestEdge.target,
        targetHandle: closestEdge.targetHandle,
        className: 'temp'
      })
    }
  }

  return proximityEdges
}

function getClosestEdge(
  node: Node,
  nodes: PieceNode[],
  getInternalNode: (id: string) => InternalNode<Node> | undefined,
  selection: DesignEditorSelection
): Edge | null {
  const MIN_DISTANCE = 150
  // We calculate handle positions manually based on the node's data.
  const draggedHandles = getHandlePositions(node)

  let closestHandle = {
    distance: Number.MAX_VALUE,
    source: null as null | { nodeId: string; handleId: string },
    target: null as null | { nodeId: string; handleId: string }
  }

  const originalDraggedId = node.data.isGhost ? node.id.slice(5) : null
  const selectedNodeIds = selection.selectedPieceIds ?? []

  // Iterate over all other nodes in the array
  for (const otherNode of nodes) {
    // Skip the node itself, the original dragged node, and any selected nodes
    if (otherNode.id === node.id || otherNode.id === originalDraggedId || selectedNodeIds.includes(otherNode.id))
      continue
    const otherInternalNode = getInternalNode(otherNode.id)
    if (!otherInternalNode) continue
    // Get all handles on the other node
    const otherHandles = getAbsoluteHandlePositions(otherInternalNode)
    // Compare all handles between dragged node and other node
    for (const draggedHandle of draggedHandles) {
      for (const otherHandle of otherHandles) {
        const dx = draggedHandle.x - otherHandle.x
        const dy = draggedHandle.y - otherHandle.y
        const distance = Math.sqrt(dx * dx + dy * dy)
        if (distance < closestHandle.distance && distance < MIN_DISTANCE) {
          // Always assign the node with the lower id as source for consistency
          closestHandle = createHandle(otherNode, distance, draggedHandle, otherHandle)
        }
      }
    }
  }

  // If no close handle pair found, return null
  if (!closestHandle.source || !closestHandle.target) {
    return null
  }

  // Return an edge connecting the closest handles
  return {
    id: `${closestHandle.source.nodeId}-${closestHandle.source.handleId}__${closestHandle.target.nodeId}-${closestHandle.target.handleId}`,
    source: closestHandle.source.nodeId,
    sourceHandle: closestHandle.source.handleId,
    target: closestHandle.target.nodeId,
    targetHandle: closestHandle.target.handleId
  }

  function createHandle(
    otherNode: PieceNode,
    distance: number,
    draggedHandle: { handleId: string; x: number; y: number },
    otherHandle: { handleId: string; x: number; y: number }
  ) {
    if (node.id < otherNode.id) {
      return {
        distance,
        source: {
          nodeId: node.id,
          handleId: draggedHandle.handleId
        },
        target: {
          nodeId: otherNode.id,
          handleId: otherHandle.handleId
        }
      }
    } else {
      return {
        distance,
        source: {
          nodeId: otherNode.id,
          handleId: otherHandle.handleId
        },
        target: {
          nodeId: node.id,
          handleId: draggedHandle.handleId
        }
      }
    }
  }

  function getHandlePositions(node: Node) {
    const pieceNode = node as PieceNode
    const ports = pieceNode.data.type.ports || []
    const handlePositions = ports
      .filter((port) => port.id_)
      .map((port) => {
        const { x: portX, y: portY } = portPositionStyle(port)
        return {
          handleId: port.id_ as string,
          x: pieceNode.position.x + portX + ICON_WIDTH / 2,
          y: pieceNode.position.y + portY
        }
      })
    return handlePositions
  }

  function getAbsoluteHandlePositions(node: InternalNode<Node>): Array<{
    handleId: string
    x: number
    y: number
  }> {
    const handles = node.internals.handleBounds?.source ?? []
    return handles
      .filter((handle) => handle.id !== null && handle.id !== undefined)
      .map((handle) => ({
        handleId: handle.id as string,
        x: node.internals.positionAbsolute.x + handle.x + handle.width / 2,
        y: node.internals.positionAbsolute.y + handle.y + handle.height / 2
      }))
  }
}

//#endregion

//#region Interaction

function useDragHandle(
  dragState: { origin: XYPosition; offset: XYPosition } | null,
  setDragState: React.Dispatch<
    React.SetStateAction<{
      origin: XYPosition
      offset: XYPosition
    } | null>
  >,
  selection: DesignEditorSelection,
  onSelectionChange: (selection: DesignEditorSelection) => void,
  kit: Kit,
  designId: DesignId,
  onDesignChange: ((design: Design) => void) | undefined
) {
  const reactFlowInstance = useReactFlow()
  const handleNodeDragStart = useCallback(
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
        offset: { x: 0, y: 0 }
      })
    },
    [setDragState, selection, onSelectionChange]
  )

  const handleNodeDrag = useCallback(
    (event: any, node: Node) => {
      setDragState((prev) =>
        prev
          ? {
              ...prev,
              offset: {
                x: node.position.x - prev.origin.x,
                y: node.position.y - prev.origin.y
              }
            }
          : null
      )
    },
    [setDragState]
  )

  const handleNodeDragStop = useCallback(() => {
    updateDesign()
    setDragState(null)
  }, [dragState, onDesignChange, kit, designId, selection, setDragState, reactFlowInstance])
  return { handleNodeDragStart, handleNodeDrag, handleNodeDragStop }

  function updateDesign() {
    if (dragState && onDesignChange) {
      const nodesAndEdges = mapDesignToNodesAndEdges({ kit, designId, selection })
      if (!nodesAndEdges) return

      const { offset } = dragState
      const { nodes } = nodesAndEdges

      // Recreate ghost nodes to calculate proximity edges
      const selectedNodeIds = new Set(selection.selectedPieceIds ?? [])
      const ghostNodes: PieceNode[] = Array.from(selectedNodeIds)
        .map((id) => {
          const orig = nodes.find((n) => n.id === id)
          if (!orig) return undefined
          return {
            ...orig,
            id: 'ghost' + id,
            position: {
              x: orig.position.x + offset.x,
              y: orig.position.y + offset.y
            },
            data: { ...orig.data, isGhost: true }
          }
        })
        .filter(Boolean) as PieceNode[]

      const proximityEdges = getProximityEdges([...nodes, ...ghostNodes], nodes, reactFlowInstance, selection)

      const normalize = (value: string | undefined) => (value === undefined ? '' : value)
      const design = kit.designs?.find(
        (d) =>
          d.name === designId.name &&
          normalize(d.variant) === normalize(designId.variant) &&
          normalize(d.view) === normalize(designId.view)
      )

      if (design) {
        // Update pieces that have moved
        const scaledOffset = {
          x: offset.x / ICON_WIDTH,
          y: -offset.y / ICON_WIDTH
        }

        const newChildPieceIds = new Set<string>()

        // Convert proximity edges to new Semio connections
        const newConnections = proximityEdges.map((edge): Connection => {
          const sourceId = edge.source!.startsWith('ghost') ? edge.source!.substring(5) : edge.source!
          const targetId = edge.target!.startsWith('ghost') ? edge.target!.substring(5) : edge.target!

          // Identify which piece was dragged to become a child
          if (selectedNodeIds.has(sourceId)) newChildPieceIds.add(sourceId)
          if (selectedNodeIds.has(targetId)) newChildPieceIds.add(targetId)

          return {
            connecting: { piece: { id_: sourceId }, port: { id_: edge.sourceHandle! } },
            connected: { piece: { id_: targetId }, port: { id_: edge.targetHandle! } },
            description: '',
            gap: 0,
            shift: 0,
            raise_: 0,
            rotation: 0,
            turn: 0,
            tilt: 0,
            x: 0,
            y: 0
          }
        })

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
                y: piece.center.y + scaledOffset.y,
                z: (piece.center as any).z ?? 0
              }
            }
          }

          // Other pieces are unchanged.
          return piece
        })

        // Filter out old connections of moved pieces and add new ones
        const originalConnections = design.connections ?? []

        const updatedConnections = [...originalConnections, ...newConnections]

        onDesignChange({
          ...design,
          pieces: updatedPieces,
          connections: updatedConnections
        })
      }
    }
  }
}

function toggleNodeSelection(
  nodeId: string,
  selection: { selectedPieceIds?: string[] },
  onSelectionChange: (sel: { selectedPieceIds: string[]; selectedConnections: any[] }) => void,
  event?: React.MouseEvent
) {
  event?.stopPropagation()
  const currentSelectedIds = selection?.selectedPieceIds ?? []
  const isNodeSelected = currentSelectedIds.includes(nodeId)

  // Multi-select: toggle node in selection
  if (isNodeSelected) {
    onSelectionChange({
      selectedPieceIds: currentSelectedIds.filter((id) => id !== nodeId),
      selectedConnections: []
    })
  } else {
    onSelectionChange({
      selectedPieceIds: [...currentSelectedIds, nodeId],
      selectedConnections: []
    })
  }
}

//#endregion

//#region Display Nodes and Edges

function useDisplayGhostNodes(
  dragState: {
    origin: XYPosition
    offset: XYPosition
  } | null,
  nodes: PieceNode[],
  selection: { selectedPieceIds?: string[] }
) {
  return useMemo(() => {
    if (!dragState) return nodes
    const { offset } = dragState
    const selectedNodeIds = selection.selectedPieceIds ?? []

    // Grey out all originals being dragged
    const nodesWithGreyedOut = nodes.map((node) =>
      selectedNodeIds.includes(node.id) ? { ...node, data: { ...node.data, isBeingDragged: true } } : node
    )
    // Add ghost nodes for all being dragged
    const ghostNodes: PieceNode[] = selectedNodeIds
      .map((id) => {
        const orig = nodes.find((n) => n.id === id)
        if (!orig) return undefined
        // Copy all properties from the original node, only override what needs to be unique
        return {
          ...orig,
          id: 'ghost' + id,
          position: {
            x: orig.position.x + offset.x,
            y: orig.position.y + offset.y
          },
          data: { ...orig.data, isGhost: true },
          selected: true
        }
      })
      .filter(Boolean) as PieceNode[]

    return [...nodesWithGreyedOut, ...ghostNodes]
  }, [nodes, dragState, selection])
}

function useDisplayGhostEdges(
  displayNodes: PieceNode[],
  nodes: PieceNode[],
  selection: DesignEditorSelection,
  edges: Edge[]
) {
  const reactFlowInstance = useReactFlow()
  return useMemo(() => {
    const ghostEdges = getProximityEdges(displayNodes, nodes, reactFlowInstance, selection)
    if (ghostEdges.length > 0) {
      return [...edges, ...ghostEdges]
    }
    return edges
  }, [displayNodes, nodes, reactFlowInstance, selection, edges])
}

//#endregion

//#region React Flow Types

type PieceNodeProps = {
  piece: Piece
  type: Type
  isBeingDragged: boolean
  isGhost: boolean
}

type PieceNode = Node<PieceNodeProps, 'piece'>
type DiagramNode = PieceNode

type ConnectionEdge = Edge<{ connection: Connection }, 'connection'>
type DiagramEdge = ConnectionEdge

type PortHandleProps = { port: Port }

const portPositionStyle = (port: Port): { x: number; y: number } => {
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
  const { x, y } = portPositionStyle(port)

  return <Handle id={port.id_} type="source" style={{ left: x + ICON_WIDTH / 2, top: y }} position={Position.Top} />
}

const PieceNodeComponent: React.FC<NodeProps<PieceNode>> = React.memo(({ id, data, selected }) => {
  const {
    piece: { id_ },
    type: { ports },
    isBeingDragged,
    isGhost
  } = data as any
  return (
    <div style={{ opacity: isBeingDragged ? 0.5 : 1 }}>
      <svg width={ICON_WIDTH} height={ICON_WIDTH} className="cursor-pointer">
        <circle
          cx={ICON_WIDTH / 2}
          cy={ICON_WIDTH / 2}
          r={ICON_WIDTH / 2 - 1}
          className={`stroke-foreground stroke-2 ${selected ? 'fill-primary ' : 'fill-transparent'} `}
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
  targetHandleId
}) => {
  const { getNode } = useReactFlow()
  const HANDLE_HEIGHT = 5
  const path = `M ${sourceX} ${sourceY + HANDLE_HEIGHT / 2} L ${targetX} ${targetY + HANDLE_HEIGHT / 2}`

  const isGhost = id?.startsWith('ghost-edge')

  return (
    <BaseEdge
      path={path}
      style={{
        strokeDasharray: isGhost ? '5 5' : undefined,
        stroke: isGhost ? 'var(--primary)' : 'var(--foreground)'
      }}
    />
  )
}

export const MiniMapNode: React.FC<MiniMapNodeProps> = ({ x, y, selected }) => {
  return <circle className={selected ? 'fill-primary' : 'fill-foreground'} cx={x} cy={y} r="10" />
}

const nodeTypes = { piece: PieceNodeComponent }
const edgeTypes = { connection: ConnectionEdgeComponent }

//#endregion

//#region Data Mapping

function mapDesignToNodesAndEdges({
  kit,
  designId,
  selection
}: {
  kit: Kit
  designId: DesignId
  selection?: DesignEditorSelection
}): { nodes: PieceNode[]; edges: ConnectionEdge[] } | null {
  const types = kit?.types ?? []
  const normalize = (value: string | undefined) => (value === undefined ? '' : value)
  const design = kit.designs?.find(
    (design) =>
      design.name === designId.name &&
      normalize(design.variant) === normalize(designId.variant) &&
      normalize(design.view) === normalize(designId.view)
  )
  if (!design) return null
  const flatDesign = flattenDesign(kit, designId)
  const pieceNodes =
    flatDesign!.pieces?.map((flatPiece) =>
      pieceToNode(
        flatPiece,
        types.find((t) => t.name === flatPiece.type.name && (t.variant ?? '') === (flatPiece.type.variant ?? ''))!,
        selection?.selectedPieceIds.includes(flatPiece.id_) ?? false
      )
    ) ?? []
  const connectionEdges =
    design.connections?.map((connection) =>
      connectionToEdge(
        connection,
        selection?.selectedConnections.some(
          (c) =>
            c.connectingPieceId === connection.connecting.piece.id_ &&
            c.connectedPieceId === connection.connected.piece.id_
        ) ?? false
      )
    ) ?? []
  return { nodes: pieceNodes, edges: connectionEdges }
}

const pieceToNode = (piece: Piece, type: Type, selected: boolean): PieceNode => ({
  type: 'piece',
  id: piece.id_,
  position: {
    x: piece.center!.x * ICON_WIDTH || 0,
    y: -piece.center!.y * ICON_WIDTH || 0
  },
  selected,
  data: { piece, type, isBeingDragged: false, isGhost: false }
})

const connectionToEdge = (connection: Connection, selected: boolean): ConnectionEdge => ({
  type: 'connection',
  id: `${connection.connecting.piece.id_} -- ${connection.connected.piece.id_}`,
  source: connection.connecting.piece.id_,
  sourceHandle: connection.connecting.port.id_,
  target: connection.connected.piece.id_,
  targetHandle: connection.connected.port.id_,
  data: { connection } // removed 'selected' property
})

//#endregion

//#region Droppable Node

function useDiagramDroppableNodeRef() {
  return useDroppable({ id: 'diagram' }).setNodeRef
}

//#endregion
