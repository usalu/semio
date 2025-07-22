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
  InternalNode,
  MiniMap,
  MiniMapNodeProps,
  Node,
  NodeProps,
  Position,
  ReactFlow,
  ReactFlowInstance,
  Connection as RFConnection,
  useReactFlow,
  ViewportPortal,
  XYPosition
} from '@xyflow/react'
import React, { FC, useCallback, useEffect, useMemo, useState } from 'react'

import {
  applyDesignDiff,
  Connection,
  ConnectionsDiff,
  Design,
  DesignDiff,
  DesignId,
  flattenDesign,
  getDesign,
  ICON_WIDTH,
  Kit,
  Piece,
  Port,
  Status,
  Type
} from '@semio/js'

// import '@xyflow/react/dist/base.css';
import '@semio/js/globals.css'
import '@xyflow/react/dist/style.css'
import { DesignEditorAction, DesignEditorDispatcher, DesignEditorSelection, DesignEditorState } from '../..'

//#region Diagram Component

const Diagram: FC<{ designEditorState: DesignEditorState; designEditorDispatcher: DesignEditorDispatcher }> = ({
  designEditorState,
  designEditorDispatcher
}) => {
  const { kit, designId, selection, fullscreenPanel, designDiff } = designEditorState // fullscreen is fullscreenPanel === 'diagram'

  const onDesignChange = (design: Design) =>
    designEditorDispatcher({ type: DesignEditorAction.SetDesign, payload: design })

  const onSelectionChange = (sel: DesignEditorSelection) =>
    designEditorDispatcher({ type: DesignEditorAction.SetSelection, payload: sel })

  const onPanelDoubleClick = () =>
    designEditorDispatcher({
      type: DesignEditorAction.SetFullscreen,
      payload: fullscreenPanel === 'diagram' ? null : 'diagram'
    })

  const fullscreen = fullscreenPanel === 'diagram'

  if (!kit) return null // Prevents error if kit is undefined
  // Mapping the semio design to react flow nodes and edges
  const nodesAndEdges = mapDesignToNodesAndEdges({ kit, designId, selection, designDiff })
  if (!nodesAndEdges) return null
  const { nodes, edges: edges } = nodesAndEdges

  // Drage handling
  const [dragState, setDragState] = useState<{
    origin: XYPosition
    offset: XYPosition | null
  } | null>(null)

  const { handleNodeDragStart, handleNodeDrag, handleNodeDragStop } = useDragHandle(
    dragState,
    setDragState,
    selection,
    onSelectionChange,
    kit,
    designId,
    onDesignChange,
    designDiff,
    designEditorDispatcher
  )

  const reactFlowInstance = useReactFlow()

  const onConnect = useCallback(
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

      const design = getDesign(kit, designId)

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
    },
    [kit, designId, onDesignChange, reactFlowInstance]
  )

  // Double click handling
  const onDoubleClickCapture = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation()
      onPanelDoubleClick?.()
    },
    [onPanelDoubleClick]
  )

  // Intermediate nodes rendering
  const displayNodes = useDisplayIntermediateNodes(dragState, nodes, selection)

  // Intermediate edges rendering
  const displayEdges = useDisplayIntermediateEdges(displayNodes, nodes, selection, edges)

  useEffect(() => {
    const proximityEdges = getProximityEdges(displayNodes, nodes, reactFlowInstance, selection);

    const design = getDesign(kit, designId);
    if (!design) return;

    const originalConnections = design.connections ?? [];
    const currentAdded = designDiff.connections?.added ?? [];

    const isDuplicate = (sourceId: string, targetId: string) => {
      const idsA = [sourceId, targetId].sort().join('--');
      return originalConnections.some(c => {
        const idsB = [c.connecting.piece.id_, c.connected.piece.id_].sort().join('--');
        return idsA === idsB;
      }) || currentAdded.some(c => {
        const idsB = [c.connecting.piece.id_, c.connected.piece.id_].sort().join('--');
        return idsA === idsB;
      });
    };

    const intermediateConns: Connection[] = proximityEdges
      .map(edge => {
        let sourceId = edge.source!;
        let targetId = edge.target!;
        let sourceHandle = edge.sourceHandle ?? '';
        let targetHandle = edge.targetHandle ?? '';

        if (sourceId.startsWith('intermediate')) sourceId = sourceId.substring(12);
        if (targetId.startsWith('intermediate')) targetId = targetId.substring(12);

        if (isDuplicate(sourceId, targetId)) return null;

        const dx = typeof edge.data?.dx === 'number' ? edge.data.dx : 0;
        const dy = typeof edge.data?.dy === 'number' ? edge.data.dy : 0;

        const scaledDx = dx / ICON_WIDTH;
        const scaledDy = -dy / ICON_WIDTH;

        if (Math.abs(scaledDx) < 0.001 && Math.abs(scaledDy) < 0.001) return null;

        // Determine connecting and connected based on original order
        // Assuming source is connecting, target is connected
        return {
          connecting: { piece: { id_: sourceId }, port: { id_: sourceHandle } },
          connected: { piece: { id_: targetId }, port: { id_: targetHandle } },
          description: '',
          gap: 0,
          shift: 0,
          rise: 0,
          rotation: 0,
          turn: 0,
          tilt: 0,
          x: scaledDx,
          y: scaledDy,
          qualities: [{ name: 'semio.intermediate', value: 'true' }]
        } as Connection;
      })
      .filter(Boolean) as Connection[];

    // Filter out previous intermediate from current added
    const nonIntermediateAdded = currentAdded.filter(c =>
      !c.qualities?.some(q => q.name === 'semio.intermediate' && q.value === 'true')
    );

    const newAdded = [...nonIntermediateAdded, ...intermediateConns];

    const newConnectionsDiff: ConnectionsDiff = {
      ...(designDiff.connections ?? {}),
      added: newAdded.length > 0 ? newAdded : undefined
    };

    const newDesignDiff: DesignDiff = {
      ...designDiff,
      connections: newConnectionsDiff
    };

    if (JSON.stringify(newDesignDiff) === JSON.stringify(designDiff)) return;
    designEditorDispatcher({ type: DesignEditorAction.SetDesignDiff, payload: newDesignDiff });
  }, [displayNodes, nodes, selection, kit, designId, designDiff, reactFlowInstance, designEditorDispatcher]);

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
        onPaneClick={() => onSelectionChange({ selectedPieceIds: [], selectedConnections: [] })}
        onDoubleClick={onDoubleClickCapture}
        onConnect={onConnect}
        connectionLineComponent={ConnectionConnectionLine}
      >
        {fullscreenPanel === 'diagram' && <Controls className="border" showZoom={false} showInteractive={false} />}
        {fullscreenPanel === 'diagram' && (
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
  designEditorState: DesignEditorState
  designEditorDispatcher: DesignEditorDispatcher
}

export default Diagram

//#endregion

//#region Data Mapping

function getPieceStatusFromQuality(piece: Piece): Status {
  const statusQuality = piece.qualities?.find((q) => q.name === 'semio.status')
  return (statusQuality?.value as Status) || Status.Unchanged
}

function getConnectionStatusFromQuality(conn: Connection): Status {
  const statusQuality = conn.qualities?.find((q) => q.name === 'semio.status')
  return (statusQuality?.value as Status) || Status.Unchanged
}

function mapDesignToNodesAndEdges({
  kit,
  designId,
  selection,
  designDiff
}: {
  kit: Kit
  designId: DesignId
  selection?: DesignEditorSelection
  designDiff: DesignDiff
}): { nodes: PieceNode[]; edges: ConnectionEdge[] } | null {
  const types = kit?.types ?? []
  const design = getDesign(kit, designId)
  if (!design) return null
  const effectiveDesign = applyDesignDiff(design, designDiff, true)
  const tempKit = {
    ...kit,
    designs: kit.designs!.map((d: Design) => {
      if (getDesign(kit, designId) === d) {
        return effectiveDesign
      }
      return d
    })
  }
  const flatDesign = flattenDesign(tempKit, designId)
  const pieceNodes =
    flatDesign!.pieces?.map((flatPiece) => {
      const type = types.find(
        (t) => t.name === flatPiece.type.name && (t.variant ?? '') === (flatPiece.type.variant ?? '')
      )
      const pieceStatus = getPieceStatusFromQuality(flatPiece)
      return pieceToNode(flatPiece, type!, selection?.selectedPieceIds.includes(flatPiece.id_) ?? false, pieceStatus)
    }) ?? []
  const connectionEdges =
    effectiveDesign.connections?.map((connection) => {
      const connStatus = getConnectionStatusFromQuality(connection)
      return connectionToEdge(
        connection,
        selection?.selectedConnections.some(
          (c) =>
            c.connectingPieceId === connection.connecting.piece.id_ &&
            c.connectedPieceId === connection.connected.piece.id_
        ) ?? false,
        connStatus
      )
    }) ?? []
  return { nodes: pieceNodes, edges: connectionEdges }
}

const pieceToNode = (piece: Piece, type: Type, selected: boolean, status: Status): PieceNode => ({
  type: 'piece',
  id: piece.id_,
  position: {
    x: piece.center!.x * ICON_WIDTH || 0,
    y: -piece.center!.y * ICON_WIDTH || 0
  },
  selected,
  data: { piece, type, isBeingDragged: false, isIntermediate: false, status }
})

const connectionToEdge = (connection: Connection, selected: boolean, status: Status): ConnectionEdge => ({
  type: 'connection',
  id: `${connection.connecting.piece.id_} -- ${connection.connected.piece.id_}`,
  source: connection.connecting.piece.id_,
  sourceHandle: connection.connecting.port.id_,
  target: connection.connected.piece.id_,
  targetHandle: connection.connected.port.id_,
  data: { connection, status }
})

//#endregion

//#region Proximity Edges

function getProximityEdges(
  displayNodes: PieceNode[],
  nodes: PieceNode[],
  reactFlowInstance: ReactFlowInstance,
  selection: DesignEditorSelection
): Edge[] {
  const intermediateNodes = displayNodes.filter((node) => node.data.isIntermediate)

  if (intermediateNodes.length === 0) {
    return []
  }

  const proximityEdges: Edge[] = []
  for (const intermediateNode of intermediateNodes) {
    const closestEdge = getClosestEdge(intermediateNode, nodes, reactFlowInstance.getInternalNode, selection)

    if (closestEdge) {
      proximityEdges.push({
        ...closestEdge,
        id: 'intermediate-edge-' + intermediateNode.id,
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
    target: null as null | { nodeId: string; handleId: string },
    dx: 0,
    dy: 0
  }

  const originalDraggedId = node.data.isIntermediate ? node.id.slice(12) : null
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
          closestHandle = createHandle(otherNode, distance, draggedHandle, otherHandle, dx, dy)
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
    targetHandle: closestHandle.target.handleId,
    data: { dx: closestHandle.dx, dy: closestHandle.dy }
  }

  function createHandle(
    otherNode: PieceNode,
    distance: number,
    draggedHandle: { handleId: string; x: number; y: number },
    otherHandle: { handleId: string; x: number; y: number },
    dx: number,
    dy: number
  ) {
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
      return {
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

  function getHandlePositions(node: Node) {
    const pieceNode = node as PieceNode
    const ports = pieceNode.data.type.ports || []
    const handlePositions = ports.map((port) => {
      const { x: portX, y: portY } = portPositionStyle(port)
      return {
        handleId: port.id_ || '',
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
  dragState: { origin: XYPosition; offset: XYPosition | null } | null,
  setDragState: React.Dispatch<
    React.SetStateAction<{
      origin: XYPosition
      offset: XYPosition | null
    } | null>
  >,
  selection: DesignEditorSelection,
  onSelectionChange: (selection: DesignEditorSelection) => void,
  kit: Kit,
  designId: DesignId,
  onDesignChange: ((design: Design) => void) | undefined,
  designDiff: DesignDiff,
  designEditorDispatcher: DesignEditorDispatcher
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
    if (dragState && dragState.offset && onDesignChange) {
      const nodesAndEdges = mapDesignToNodesAndEdges({ kit, designId, selection, designDiff })
      if (!nodesAndEdges) return

      const { offset } = dragState
      const { nodes } = nodesAndEdges

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

      const proximityEdges = getProximityEdges([...nodes, ...intermediateNodes], nodes, reactFlowInstance, selection)

      const design = getDesign(kit, designId)

      if (design) {
        const scaledOffset = {
          x: offset.x / ICON_WIDTH,
          y: -offset.y / ICON_WIDTH
        }

        // If there are no proximity edges, this is a simple position update
        if (proximityEdges.length === 0) {
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
        const newConnections = proximityEdges
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

function useDisplayIntermediateNodes(
  dragState: {
    origin: XYPosition
    offset: XYPosition | null
  } | null,
  nodes: PieceNode[],
  selection: { selectedPieceIds?: string[] }
) {
  return useMemo(() => {
    if (!dragState || !dragState.offset) return nodes
    const { offset } = dragState
    const selectedNodeIds = selection.selectedPieceIds ?? []

    // Grey out all originals being dragged
    const nodesWithGreyedOut = nodes.map((node) =>
      selectedNodeIds.includes(node.id) ? { ...node, data: { ...node.data, isBeingDragged: true } } : node
    )
    // Add intermediate nodes for all being dragged
    const intermediateNodes: PieceNode[] = selectedNodeIds
      .map((id) => {
        const orig = nodes.find((n) => n.id === id)
        if (!orig) return undefined
        // Copy all properties from the original node, only override what needs to be unique
        return {
          ...orig,
          id: 'intermediate' + id,
          position: {
            x: orig.position.x + offset.x,
            y: orig.position.y + offset.y
          },
          data: { ...orig.data, isIntermediate: true },
          selected: true
        }
      })
      .filter(Boolean) as PieceNode[]

    return [...nodesWithGreyedOut, ...intermediateNodes]
  }, [nodes, dragState, selection])
}

function useDisplayIntermediateEdges(
  displayNodes: PieceNode[],
  nodes: PieceNode[],
  selection: DesignEditorSelection,
  edges: Edge[]
) {
  const reactFlowInstance = useReactFlow()
  return useMemo(() => {
    const intermediateEdges = getProximityEdges(displayNodes, nodes, reactFlowInstance, selection)
    if (intermediateEdges.length > 0) {
      return [...edges, ...intermediateEdges]
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
  isIntermediate: boolean
  status: Status
}

type PieceNode = Node<PieceNodeProps, 'piece'>
type DiagramNode = PieceNode

type ConnectionEdge = Edge<{ connection: Connection; status: Status }, 'connection'>
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

  return (
    <>
      <Handle id={port.id_} type="source" style={{ left: x + ICON_WIDTH / 2, top: y }} position={Position.Top} />
      <Handle id={port.id_} type="target" style={{ left: x + ICON_WIDTH / 2, top: y }} position={Position.Top} />
    </>
  )
}

const PieceNodeComponent: React.FC<NodeProps<PieceNode>> = React.memo(({ id, data, selected }) => {
  const {
    piece: { id_ },
    type: { ports },
    isBeingDragged,
    isIntermediate,
    status // Add status prop
  } = data as PieceNodeProps & { status: Status }

  let fillClass = ''
  let strokeClass = 'stroke-foreground'
  let opacity = isBeingDragged ? 0.5 : 1

  if (status === Status.Added) {
    fillClass = selected ? 'fill-green-600' : 'fill-green-400'
  } else if (status === Status.Removed) {
    fillClass = 'fill-red-400'
    strokeClass = 'stroke-red-600'
    opacity *= 0.5 // transparent
  } else if (status === Status.Modified) {
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
  data
}) => {
  const HANDLE_HEIGHT = 5
  const path = `M ${sourceX} ${sourceY + HANDLE_HEIGHT / 2} L ${targetX} ${targetY + HANDLE_HEIGHT / 2}`

  const isIntermediate = data?.connection?.qualities?.some(q => q.name === 'semio.intermediate') ?? id?.startsWith('intermediate-edge');
  const status = data?.status || Status.Unchanged

  let stroke: string;
  let dasharray: string | undefined;
  let opacity = 1;

  if (isIntermediate) {
    stroke = 'var(--primary)';
    dasharray = '5 5';
  } else {
    stroke = 'var(--foreground)';
    if (status === Status.Added) {
      stroke = 'green';
      dasharray = '5 5';
    } else if (status === Status.Removed) {
      stroke = 'red';
      opacity = 0.5;
      dasharray = '5 5';
    } else if (status === Status.Modified) {
      stroke = 'yellow';
    }
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

//#endregion

//#region Droppable Node

function useDiagramDroppableNodeRef() {
  return useDroppable({ id: 'diagram' }).setNodeRef
}

//#endregion
