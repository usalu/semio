import React, { FC, useCallback, useMemo, useState, useEffect } from 'react';
import { addEdge, Background, BackgroundVariant, BaseEdge, BuiltInNode, ConnectionMode, Controls, Edge, EdgeProps, getBezierPath, getStraightPath, Handle, HandleProps, MiniMap, MiniMapNodeProps, Node, NodeProps, OnConnect, OnEdgesChange, OnNodesChange, Panel, Position, ReactFlow, ReactFlowProvider, useEdgesState, useNodesState, useOnViewportChange, useReactFlow, useStoreApi, useViewport, Viewport, ViewportPortal, useStore } from '@xyflow/react';
import * as THREE from 'three';
import { useDroppable } from '@dnd-kit/core';
import { Canvas } from '@react-three/fiber';
import { OrbitControls, Sphere } from '@react-three/drei';

import { Connection, Design, flattenDesign, ICON_WIDTH, Kit, Piece, Port, Type, TypeID } from '@semio/js/semio'
import { Avatar, AvatarFallback, AvatarImage } from '@semio/js/components/ui/Avatar';

// import '@xyflow/react/dist/base.css';
import '@xyflow/react/dist/style.css';
import "@semio/js/globals.css";
import { DesignEditorSelection } from '../..';

type PieceNodeProps = {
    piece: Piece;
    type: Type;
    selected?: boolean;
};

type PieceNode = Node<PieceNodeProps, 'piece'>;
type DiagramNode = PieceNode;

type ConnectionEdge = Edge<{ connection: Connection }, 'connection'>;
type DiagramEdge = ConnectionEdge;

type PortHandleProps = { port: Port };

const portPositionStyle = (port: Port): { x: number, y: number } => {
    // t is normalized in [0,1[ and clockwise and starts at 12 o'clock
    const { t } = port;
    if (t === undefined) {
        return { x: 0, y: 0 };
    }
    const angle = t * 2 * Math.PI;
    const radius = ICON_WIDTH / 2;
    return {
        x: radius * Math.sin(angle),
        y: -(radius * Math.cos(angle) - radius),
    };
};

const PortHandle: React.FC<PortHandleProps> = ({ port }) => {
    const { x, y } = portPositionStyle(port);

    return (
        <Handle
            id={port.id_}
            type="source"
            style={{ left: x + ICON_WIDTH / 2, top: y }}
            position={Position.Top}
        />
    );
};

const PieceNodeComponent: React.FC<NodeProps<PieceNode>> = ({ id, data, selected }) => {
    // const { zoom } = useViewport();
    const { piece: { id_ }, type: { ports } } = data;
    return (
        <div>
            <svg
                width={ICON_WIDTH}
                height={ICON_WIDTH}
                className="cursor-pointer"
            >
                <circle
                    cx={ICON_WIDTH / 2}
                    cy={ICON_WIDTH / 2}
                    r={ICON_WIDTH / 2 - 1}
                    className={`stroke-foreground stroke-2 ${selected ? 'fill-primary ' : 'fill-transparent'} `}
                />
                {/* {zoom < 10 && */}
                <text
                    x={ICON_WIDTH / 2}
                    y={ICON_WIDTH / 2}
                    textAnchor="middle"
                    dominantBaseline="middle"
                    className={`text-xs font-bold fill-foreground`}
                >
                    {id_}
                </text>
                {/* } */}
            </svg>
            {/* {zoom > 10 && <Canvas className="w-full h-full">
                <OrbitControls
                    mouseButtons={{
                        LEFT: THREE.MOUSE.ROTATE, // Left mouse button for orbit/pan
                        MIDDLE: undefined,
                        RIGHT: undefined // Right button disabled to allow selection
                    }}
                />
                <Sphere args={[1, 100, 100]} position={[0, 0, 0]}>
                    <meshStandardMaterial color="gold" roughness={0} metalness={1} />
                </Sphere>
                <ambientLight intensity={1} />
            </Canvas>} */}
            {ports?.map((port) => (
                <PortHandle key={port.id_} port={port} />
            ))}
        </div>
    );
};


const ConnectionEdgeComponent: React.FC<EdgeProps<ConnectionEdge>> = ({
    source,
    target,
    sourceX,
    sourceY,
    targetX,
    targetY,
    sourceHandleId,
    targetHandleId,
}) => {
    const { getNode } = useReactFlow();

    // const sourceHandle = getNode(source)?.handles.find((handle) => handle.id === sourceHandleId);
    // const targetHandle = getNode(target)?.handles.find((handle) => handle.id === targetHandleId);

    // const centerX = (sourceX + targetX) / 2;
    // const centerY = (sourceY + targetY) / 2;
    // unit vector pointing to center
    // const length = Math.sqrt((centerX) ** 2 + (centerY) ** 2);
    // const x = (centerX / length) * ICON_WIDTH;
    // const y = (centerY / length) * ICON_WIDTH;

    // const path = `M ${sourceX} ${sourceY} C ${sourceX + x} ${sourceY + y}, ${targetX - x} ${targetY - y}, ${targetX} ${targetY}`;
    const HANDLE_HEIGHT = 5;
    const path = `M ${sourceX} ${sourceY + HANDLE_HEIGHT / 2} L ${targetX} ${targetY + + HANDLE_HEIGHT / 2}`;
    return <BaseEdge path={path} />;
};

export const MiniMapNode: React.FC<MiniMapNodeProps> = ({ x, y, selected }) => {
    return <circle className={selected ? 'fill-primary' : 'fill-foreground'} cx={x} cy={y} r="10" />;
}


const DiagramCore: FC<DiagramProps> = ({ design, types, fullscreen, selection, onPanelDoubleClick, onSelectionChange }) => {
    const { setNodeRef } = useDroppable({
        id: 'diagram',
    });

    const flatDesign = design ? flattenDesign(design, types) : null;

    const nodes = flatDesign?.pieces?.map((piece) => ({
        type: 'piece',
        id: piece.id_,
        position: { x: piece.center?.x * ICON_WIDTH || 0, y: -piece.center?.y * ICON_WIDTH || 0 },
        data: { piece, type: types.find((t) => t.name === piece.type.name && (t.variant ?? '') === (piece.type.variant ?? '')), selected: selection?.selectedPieceIds.includes(piece.id_) },
    }));

    const edges = design.connections?.map((connection) => ({
        type: 'connection',
        id: `${connection.connecting.piece.id_} -- ${connection.connected.piece.id_}`,
        source: connection.connecting.piece.id_,
        sourceHandle: connection.connecting.port.id_,
        target: connection.connected.piece.id_,
        targetHandle: connection.connected.port.id_,
        data: { connection, selected: selection?.selectedConnections.some((c) => c.connectingPieceId === connection.connecting.piece.id_ && c.connectedPieceId === connection.connected.piece.id_) },
    }));

    // const edges = design.connections?.map((connection) => ({
    //     type: 'connection',
    //     id: connection.id_,
    //     source: connection.source.id_,
    //     target: connection.target.id_,
    // }));


    // const types: Type[] = [
    //     { name: 'base', ports: [{ id_: 't', t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 1, y: 0, z: 0 } }] },
    //     { name: 'tambour', ports: [{ id_: 't', t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 1, y: 0, z: 0 } }] },
    // ];
    // const initialNodes: PieceNode[] = [
    //     { type: 'piece', id: 'b', position: { x: 0, y: 100 }, data: { piece: { id_: 'b', type: { name: "base" } } } },
    //     { type: 'piece', id: 't', position: { x: 0, y: 0 }, data: { piece: { id_: 't', type: { name: "tambour" } } } },
    // ];
    // const initialEdges: ConnectionEdge[] = [{ type: 'connection', id: 'base:top -- bottom:tambour', source: 'b', sourceHandle: 't', target: 't', targetHandle: 'sw' }];

    // const setPresence = usePresenceSetter()
    // const presenceMap = usePresence();

    // const viewport = useViewport();

    // const onUpdateCursor = (event) => {
    //     if (event === null || event === undefined) {
    //         return;
    //     }
    //     // TODO: Figure out how to get the x and y of the mouse pointer in the viewport coordinate system
    //     const x = Math.round((event.clientX) - viewport.x);
    //     const y = Math.round((event.clientY) - viewport.y);
    //     // console.log(`mX: ${event.clientX}, mY: ${event.clientY}`);
    //     // console.log(`vX: ${viewport.x}, vY: ${viewport.y}, z: ${viewport.zoom}`);
    //     setPresence({
    //         cursor: { x, y },
    //     });
    // }

    const nodeTypes = useMemo(() => ({ piece: PieceNodeComponent }), []);
    const edgeTypes = useMemo(() => ({ connection: ConnectionEdgeComponent }), []);

    // const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
    // const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

    // const onConnect = useCallback(
    //     (params: any) => setEdges((eds) => addEdge(params, eds)),
    //     [setEdges],
    // );

    const MIN_DISTANCE = 100;
    // const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
    // const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);
    // const { getInternalNode } = useReactFlow();

    // const onConnect = useCallback(
    //     (params) => setEdges((eds) => addEdge(params, eds)),
    //     [setEdges],
    // );

    // const getClosestEdge = useCallback((node) => {
    //     const { nodeLookup } = store.getState();
    //     const internalNode = getInternalNode(node.id);

    //     const closestNode = Array.from(nodeLookup.values()).reduce(
    //         (res, n) => {
    //             if (n.id !== internalNode.id) {
    //                 const dx =
    //                     n.internals.positionAbsolute.x -
    //                     internalNode.internals.positionAbsolute.x;
    //                 const dy =
    //                     n.internals.positionAbsolute.y -
    //                     internalNode.internals.positionAbsolute.y;
    //                 const d = Math.sqrt(dx * dx + dy * dy);

    //                 if (d < res.distance && d < MIN_DISTANCE) {
    //                     res.distance = d;
    //                     res.node = n;
    //                 }
    //             }

    //             return res;
    //         },
    //         {
    //             distance: Number.MAX_VALUE,
    //             node: null,
    //         },
    //     );

    //     if (!closestNode.node) {
    //         return null;
    //     }

    //     // '// Find the closest source and target handles
    //     // let closestInternalHandle = null;
    //     // let closestClosestHandle = null;
    //     // let minHandleDistance = MIN_DISTANCE + ICON_WIDTH;

    //     // console.log('closestNode', internalNode['type'], closestNode.node['type']);
    //     // const internalHandles = nodeTypes[].handles || [];
    //     // const closestHandles = nodeTypes[closestNode['type']].handles || [];

    //     // internalHandles.forEach((sourceHandle) => {
    //     //     closestHandles.forEach((targetHandle) => {
    //     //         const dx =
    //     //             targetHandle.positionAbsolute.x -
    //     //             sourceHandle.positionAbsolute.x;
    //     //         const dy =
    //     //             targetHandle.positionAbsolute.y -
    //     //             sourceHandle.positionAbsolute.y;
    //     //         const handleDistance = Math.sqrt(dx * dx + dy * dy);
    //     //         console.log('handleDistance', handleDistance, sourceHandle.id, targetHandle.id);

    //     //         if (handleDistance < minHandleDistance) {
    //     //             minHandleDistance = handleDistance;
    //     //             closestInternalHandle = sourceHandle;
    //     //             closestClosestHandle = targetHandle;
    //     //         }
    //     //     });
    //     // });

    //     // if (!closestInternalHandle || !closestClosestHandle) {
    //     //     return null;
    //     // }

    //     const closeNodeIsSource =
    //         closestNode.node.internals.positionAbsolute.x <
    //         internalNode.internals.positionAbsolute.x;

    //     return {
    //         id: closeNodeIsSource
    //             ? `${closestNode.node.id}-${node.id}`
    //             : `${node.id}-${closestNode.node.id}`,
    //         type: 'connection',
    //         source: closeNodeIsSource ? closestNode.node.id : node.id,
    //         // sourceHandle: closeNodeIsSource ? closestInternalHandle.id : closestClosestHandle.id,
    //         target: closeNodeIsSource ? node.id : closestNode.node.id,
    //         // targetHandle: closeNodeIsSource ? closestClosestHandle.id : closestInternalHandle.id,
    //     };
    // }, []);

    // const onNodeDrag = useCallback(
    //     (_, node) => {
    //         const closeEdge = getClosestEdge(node);

    //         setEdges((es) => {
    //             const nextEdges = es.filter((e) => e.className !== 'temp');

    //             if (
    //                 closeEdge &&
    //                 !nextEdges.find(
    //                     (ne) =>
    //                         ne.source === closeEdge.source && ne.target === closeEdge.target,
    //                 )
    //             ) {
    //                 closeEdge.className = 'temp';
    //                 nextEdges.push(closeEdge);
    //             }

    //             return nextEdges;
    //         });
    //     },
    //     [getClosestEdge, setEdges],
    // );

    // const onNodeDragStop = useCallback(
    //     (_, node) => {
    //         const closeEdge = getClosestEdge(node);

    //         setEdges((es) => {
    //             const nextEdges = es.filter((e) => e.className !== 'temp');

    //             if (
    //                 closeEdge &&
    //                 !nextEdges.find(
    //                     (ne) =>
    //                         ne.source === closeEdge.source && ne.target === closeEdge.target,
    //                 )
    //             ) {
    //                 nextEdges.push(closeEdge);
    //             }

    //             return nextEdges;
    //         });
    //     },
    //     [getClosestEdge],
    // );

    return (
        <ReactFlow
            ref={setNodeRef}
            nodes={nodes}
            edges={edges}
            nodeTypes={nodeTypes}
            edgeTypes={edgeTypes}
            connectionMode={ConnectionMode.Loose}
            elementsSelectable
            minZoom={0.1}
            maxZoom={12}
            // onNodesChange={onNodesChange}
            // onEdgesChange={onEdgesChange}
            // onNodeDrag={onNodeDrag}
            // onNodeDragStop={onNodeDragStop}
            // onConnect={onConnect}
            onSelectionChange={({ nodes, edges }) => {
                const selectedPieceIds = nodes.map((node) => node.id);
                const selectedConnections = edges.map((edge) => {
                    return {
                        connectedPieceId: edge.source,
                        connectingPieceId: edge.target,
                    }
                });

                // Only trigger onSelectionChange if the selection actually changed
                const currentSelection = selection || { selectedPieceIds: [], selectedConnections: [] };
                const piecesChanged =
                    selectedPieceIds.length !== currentSelection.selectedPieceIds.length ||
                    selectedPieceIds.some(id => !currentSelection.selectedPieceIds.includes(id));
                const connectionsChanged =
                    selectedConnections.length !== currentSelection.selectedConnections.length ||
                    selectedConnections.some(conn =>
                        !currentSelection.selectedConnections.some(
                            currConn => currConn.connectedPieceId === conn.connectedPieceId &&
                                currConn.connectingPieceId === conn.connectingPieceId
                        )
                    );

                if (piecesChanged || connectionsChanged) {
                    onSelectionChange?.({
                        selectedPieceIds,
                        selectedConnections
                    });
                }
            }}
            zoomOnDoubleClick={false}
            onDoubleClickCapture={(e) => {
                e.stopPropagation();
                onPanelDoubleClick?.();
            }}
            panOnDrag={[0]} //left mouse button
            proOptions={{ hideAttribution: true }}
            multiSelectionKeyCode="Shift"
        >
            {fullscreen && <Controls className="border" showZoom={false} showInteractive={false} />}
            {fullscreen && < MiniMap className="border" maskColor='var(--accent)' bgColor='var(--background)' nodeComponent={MiniMapNode} />}
            <ViewportPortal>
                <div>
                    x
                </div>
            </ViewportPortal>
        </ReactFlow>
    )
}


interface DiagramProps {
    design: Design;
    types: Type[];
    fullscreen?: boolean;
    selection?: DesignEditorSelection;
    fileUrl: string;
    onPanelDoubleClick?: () => void;
    onSelectionChange?: (selection: DesignEditorSelection) => void;
}


const Diagram: FC<DiagramProps> = ({ design, types, fullscreen, selection, fileUrl, onPanelDoubleClick, onSelectionChange }) => {

    return (
        <div id="diagram" className="h-full w-full">
            <DiagramCore fullscreen={fullscreen} onPanelDoubleClick={onPanelDoubleClick} design={design} types={types} selection={selection} onSelectionChange={onSelectionChange} />
        </div>
    );
};

export default Diagram;