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
  arePortsCompatible,
  Connection,
  ConnectionDiff,
  Design,
  DesignDiff,
  DesignId,
  DiagramPoint,
  DiffStatus,
  findConnection,
  findDesign,
  findPort,
  findType,
  flattenDesign,
  ICON_WIDTH,
  isPortInUse,
  Kit,
  Piece,
  PieceDiff,
  piecesMetadata,
  Port,
  sameConnection,
  sameDesign,
  samePiece,
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
    <Handle id={port.id_ ?? ''} type="source" style={{ left: x + ICON_WIDTH / 2, top: y }} position={Position.Top} />
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

  const diff = data?.connection?.qualities?.find(q => q.name === 'semio.diffStatus')?.value as DiffStatus || DiffStatus.Unchanged;

  let stroke = 'var(--color-foreground)';
  let dasharray: string | undefined;
  let opacity = 1;

  if (selected) {
    stroke = 'var(--color-primary)';
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

const designToNodesAndEdges = (kit: Kit, designId: DesignId, selection: DesignEditorSelection) => {
  const design = findDesign(kit, designId)
  if (!design) return null
  const centers = flattenDesign(kit, designId).pieces?.map(p => p.center)
  const pieceNodes = design.pieces?.map(
    (piece, i) => pieceToNode(piece, findType(kit, piece.type)!, centers![i]!, selection?.selectedPieceIds.includes(piece.id_) ?? false)) ?? []
  const connectionEdges =
    design.connections?.map((connection) => connectionToEdge(connection, selection?.selectedConnections.some(
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
    start: XYPosition
    last: XYPosition
  } | null>(null)
  const fullscreen = fullscreenPanel === 'diagram'
  const onDesignChange = (d: Design) => designEditorDispatcher({ type: DesignEditorAction.SetDesign, payload: d })
  const onDesignDiffChange = (d: DesignDiff) => designEditorDispatcher({ type: DesignEditorAction.SetDesignDiff, payload: d })
  const onSelectionChange = (s: DesignEditorSelection) => designEditorDispatcher({ type: DesignEditorAction.SetSelection, payload: s })
  if (!kit) return null
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
  //#endregion

  const { nodes, edges } = designToNodesAndEdges(effectiveKit, designId, selection) ?? { nodes: [], edges: [] }
  const reactFlowInstance = useReactFlow()

  //#region Dragging
  const onNodeDragStart =
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
        start: { x: node.position.x, y: node.position.y },
        last: { x: node.position.x, y: node.position.y }
      })
    }

  const onNodeDrag = (event: any, node: Node) => {
    // TODO: Fix the reset after proximity connect is estabilished
    const MIN_DISTANCE = 150
    const { start, last } = dragState!
    const metadata = piecesMetadata(effectiveKit, designId)

    const addedConnections: Connection[] = []
    let updatedPieces: PieceDiff[] = []
    let updatedConnections: ConnectionDiff[] = []

    for (const selectedNode of nodes.filter((n) => selection?.selectedPieceIds.includes(n.id))) {
      const piece = selectedNode.data.piece
      const type = selectedNode.data.type
      let closestConnection: Connection | null = null
      let closestDistance = Number.MAX_VALUE
      const selectedInternalNode = reactFlowInstance.getInternalNode(selectedNode.id)!
      for (const otherNode of nodes.filter((n) => !(selection.selectedPieceIds ?? []).includes(n.id))) {
        const existingConnection = design.connections?.find((c) => sameConnection(c, { connected: { piece: { id_: selectedNode.id } }, connecting: { piece: { id_: otherNode.id } } }))
        if (existingConnection) continue
        const otherInternalNode = reactFlowInstance.getInternalNode(otherNode.id)!
        for (const handle of selectedInternalNode.internals.handleBounds?.source ?? []) {
          const port = findPort(type, { id_: handle.id! })
          for (const otherHandle of otherInternalNode.internals.handleBounds?.source ?? []) {
            const otherPort = findPort(otherNode.data.type, { id_: otherHandle.id! })
            // check if ports are compatible and other port is not already connected
            if (!arePortsCompatible(port, otherPort) || isPortInUse(effectiveDesign, otherNode.data.piece, otherPort)) continue
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
        updatedPieces.push({ ...selectedNode.data.piece, center: undefined, plane: undefined })
      }
      else {
        if (!piece.center) {

          const parentPieceId = metadata.get(selectedNode.id)!.parentPieceId!
          const parentInternalNode = reactFlowInstance.getInternalNode(parentPieceId)
          if (!parentInternalNode) throw new Error(`Internal node not found for ${parentPieceId}`)
          const parentConnection = findConnection(effectiveDesign, { connected: { piece: { id_: selectedNode.id } }, connecting: { piece: { id_: parentPieceId } } })
          // const isSelectedConnecting = parentConnection.connecting.piece.id_ === selectedNode.id
          // const selectedInternalNode = reactFlowInstance.getInternalNode(selectedNode.id)!
          // const handle = selectedInternalNode.internals.handleBounds?.source?.find((h) => h.id === (isSelectedConnecting ? parentConnection.connected.port.id_ : parentConnection.connecting.port.id_))
          // if (!handle) throw new Error(`Handle not found for ${parentConnection.connecting.port.id_}`)
          // const parentHandle = parentInternalNode.internals.handleBounds?.source?.find((h) => h.id === (isSelectedConnecting ? parentConnection.connecting.port.id_ : parentConnection.connected.port.id_))
          // if (!parentHandle) throw new Error(`Handle not found for ${parentConnection.connected.port.id_}`)
          updatedConnections.push({
            ...parentConnection,
            x: (parentConnection.x ?? 0) + ((node.position.x - last.x) / ICON_WIDTH),
            y: (parentConnection.y ?? 0) - ((node.position.y - last.y) / ICON_WIDTH)
          })
        }
        else {
          const scaledOffset = { x: (node.position.x - last.x) / ICON_WIDTH, y: -(node.position.y - last.y) / ICON_WIDTH }
          updatedPieces.push({ ...piece, center: { x: piece.center!.x + scaledOffset.x, y: piece.center!.y + scaledOffset.y } })
        }
      }

      onDesignDiffChange({ pieces: { updated: updatedPieces }, connections: { added: addedConnections, updated: updatedConnections } })
      setDragState({ ...dragState!, last: node.position })

    }

  }

  const handleNodeDragStop = () => {
    const updatedDesign = applyDesignDiff(design, designDiff)
    onDesignChange(updatedDesign)
    onDesignDiffChange({})
    setDragState(null)
  }
  //#endregion

  const onConnect =
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

      const design = findDesign(kit, designId)
      if ((design.connections ?? []).find((c) => sameConnection(c, newConnection))) return
      const newConnections = [...(design.connections ?? []), newConnection]
      const updatedPieces = design.pieces?.map((piece) => (samePiece(piece, { id_: params.source! })) ? { ...piece, center: undefined, plane: undefined } : piece)
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
        onNodeDragStart={onNodeDragStart}
        onNodeDrag={onNodeDrag}
        onNodeDragStop={handleNodeDragStop}
        onEdgeClick={(event, edge) => {
          event?.stopPropagation()
          onSelectionChange({
            selectedPieceIds: [],
            selectedConnections: [{ connectingPieceId: edge.source!, connectedPieceId: edge.target! }]
          })
        }}
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