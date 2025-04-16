import React, { FC, useCallback, useMemo, useState, useEffect } from 'react';
import { addEdge, Background, BackgroundVariant, BaseEdge, BuiltInNode, ConnectionMode, Controls, Edge, EdgeProps, getBezierPath, getStraightPath, Handle, HandleProps, MiniMap, MiniMapNodeProps, Node, NodeProps, OnConnect, OnEdgesChange, OnNodesChange, Panel, Position, ReactFlow, ReactFlowProvider, useEdgesState, useNodesState, useOnViewportChange, useReactFlow, useStoreApi, useViewport, Viewport, ViewportPortal } from '@xyflow/react';
import { Connection, Design, ICON_WIDTH, Kit, Piece, Port, Type } from '@semio/js/semio'
import { Avatar, AvatarFallback, AvatarImage } from '@semio/js/components/ui/Avatar';

// import '@xyflow/react/dist/base.css';
import '@xyflow/react/dist/style.css';
import "@semio/js/globals.css";
import { useDroppable } from '@dnd-kit/core';

type PieceNodeProps = {
    piece: Piece;
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
    const { piece: { id_ } } = data;
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
            <PortHandle port={{ id_: 't', t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 1, y: 0, z: 0 } }} />
            <PortHandle port={{ id_: 'e', t: 0.25, point: { x: 0, y: 0, z: 0 }, direction: { x: 1, y: 0, z: 0 } }} />
            <PortHandle port={{ id_: 's', t: 0.5, point: { x: 0, y: 0, z: 0 }, direction: { x: 1, y: 0, z: 0 } }} />
            <PortHandle port={{ id_: 'sw', t: 0.66, point: { x: 0, y: 0, z: 0 }, direction: { x: 1, y: 0, z: 0 } }} />
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
    const HANDLE_HEIGHT = 5;
    const path = `M ${sourceX} ${sourceY + HANDLE_HEIGHT / 2} L ${targetX} ${targetY + + HANDLE_HEIGHT / 2}`;
    return <BaseEdge path={path} />;
};

export const MiniMapNode: React.FC<MiniMapNodeProps> = ({ x, y, selected }) => {
    return <circle className={selected ? 'fill-primary' : 'fill-foreground'} cx={x} cy={y} r="10" />;
}


type CursorProps = {
    color: string;
    x: number;
    y: number;
};

function Cursor({ color, x = 0, y = 0 }: CursorProps) {
    return (
        <svg
            style={{
                position: "absolute",
                left: 0,
                top: 0,
                transform: `translateX(${x}px) translateY(${y}px)`,
            }
            }
            width="24"
            height="36"
            viewBox="0 0 24 36"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M5.65376 12.3673H5.46026L5.31717 12.4976L0.500002 16.8829L0.500002 1.19841L11.7841 12.3673H5.65376Z"
                fill={color}
            />
        </svg>
    );
}

const DiagramCore: FC<DiagramProps> = ({ fullscreen, onPanelDoubleClick, design }) => {


    const { setNodeRef } = useDroppable({
        id: 'diagram',
    });

    const initialNodes: PieceNode[] = design.pieces?.map((piece) => ({
        type: 'piece',
        id: piece.id_ || '',
        position: { x: piece.center?.x || 0, y: piece.center?.y || 0 },
        data: { piece },
    })) || [];

    const initialEdges: ConnectionEdge[] = design.connections?.map((connection) => ({
        type: 'connection',
        id: `${connection.connecting.piece.id_ || ''} -- ${connection.connected.piece.id_ || ''}`,
        source: connection.connecting.piece.id_ || '',
        target: connection.connected.piece.id_ || '',
    })) || [];

    const nodeTypes = useMemo(() => ({ piece: PieceNodeComponent }), []);
    const edgeTypes = useMemo(() => ({ connection: ConnectionEdgeComponent }), []);

    const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
    const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

    const onConnect = useCallback(
        (params: any) => setEdges((eds) => addEdge(params, eds)),
        [setEdges],
    );

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
            maxZoom={5}
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            onConnect={onConnect}
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
    fullscreen?: boolean;
    onPanelDoubleClick?: () => void;
    design: Design;
}


const Diagram: FC<DiagramProps> = ({ fullscreen, onPanelDoubleClick, design }) => {

    // const { isOver, setNodeRef } = useDroppable({
    //     id: 'diagram',
    // });

    return (
        <div className="h-full w-full">
            <ReactFlowProvider >
                <DiagramCore fullscreen={fullscreen} onPanelDoubleClick={onPanelDoubleClick} design={design} />
            </ReactFlowProvider>
            {/* // <div ref={setNodeRef}>
        // </div> */}
        </div>
    );

    // return (
    //     <div className="bg-pink-600" style={{ height: '100vh' }}>
    //         <DiagramCore />
    //     </div>
    // );
};

export default Diagram;