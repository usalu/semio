import React, { FC, useCallback, useMemo } from 'react';
import { addEdge, Background, BackgroundVariant, BaseEdge, BuiltInNode, ConnectionMode, Controls, Edge, EdgeProps, getBezierPath, Handle, HandleProps, MiniMap, MiniMapNodeProps, Node, NodeProps, OnConnect, OnEdgesChange, OnNodesChange, Panel, Position, ReactFlow, ReactFlowProvider, useEdgesState, useNodesState, useOnViewportChange, useReactFlow, useViewport, Viewport, ViewportPortal } from '@xyflow/react';
import { Connection, Design, ICON_WIDTH, Kit, Piece, Port, Type } from '@semio/core/semio'
import { Avatar, AvatarFallback, AvatarImage } from '@semio/core/components/ui/Avatar';

// import '@xyflow/react/dist/base.css';
import '@xyflow/react/dist/style.css';

type PieceNode = Node<{ piece: Piece; selected: boolean }, 'piece'>;
type DiagramNode = PieceNode;

type ConnectionEdge = Edge<{ connection: Connection }, 'connection'>;
type DiagramEdge = ConnectionEdge;
// type PortHandleProps = HandleProps & { port: Port };
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
    }
}

const PortHandle: React.FC<PortHandleProps> = ({ port }) => {
    // TODO: If port is default port then t is undefined and the whole piece should be clickable
    const { x, y } = portPositionStyle(port);

    return <Handle
        id={port.id_}
        type="source"
        // style={portPositionStyle(port)}
        style={{ left: x + ICON_WIDTH / 2, top: y }
        }
        position={Position.Top}
    />
};

const PieceNodeComponent: React.FC<NodeProps<PieceNode>> = ({ id, data, selected }) => {
    const { piece: { id_ } } = data;
    return (
        <div className={`h-[${ICON_WIDTH}] w-[${ICON_WIDTH}]`
        }>
            <Avatar className={`cursor-pointer ${selected ? 'bg-primary text-light' : 'bg-light text-darkGrey'}`}>
                {/* <AvatarImage src="https://github.com/usalu.png" /> */}
                <AvatarFallback>Pc</AvatarFallback>
            </Avatar>
            < PortHandle port={{ id_: 'top', t: 0, point: { x: 0, y: 0, z: 0 }, direction: { x: 1, y: 0, z: 0 } }} />
            < PortHandle port={{ id_: 'e', t: 0.25, point: { x: 0, y: 0, z: 0 }, direction: { x: 1, y: 0, z: 0 } }} />
            < PortHandle port={{ id_: 'bottom', t: 0.5, point: { x: 0, y: 0, z: 0 }, direction: { x: 1, y: 0, z: 0 } }} />
            < PortHandle port={{ id_: 'sw', t: 0.66, point: { x: 0, y: 0, z: 0 }, direction: { x: 1, y: 0, z: 0 } }} />
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
    const path = `M ${sourceX} ${sourceY} L ${targetX} ${targetY}`;
    return <BaseEdge path={path} />;
};

export const MiniMapNode: React.FC<MiniMapNodeProps> = ({ x, y, selected }) => {
    return <circle className={selected ? 'fill-primary' : 'fill-lightGrey'} cx={x} cy={y} r="10" />;
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

const DiagramCore: FC = () => {

    const initialNodes = [
        { type: 'piece', id: 'base', position: { x: 0, y: 100 }, data: { piece: { id_: 'base' } } },
        { type: 'piece', id: 'tambour', position: { x: 0, y: 0 }, data: { piece: { id_: 'tambour' } } },
    ];
    const initialEdges = [{ type: 'connection', id: 'base:top -- bottom:tambour', source: 'base', sourceHandle: 'top', target: 'tambour', targetHandle: 'bottom' }];

    // const setPresence = usePresenceSetter()
    // const presenceMap = usePresence();

    const viewport = useViewport();

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

    const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
    const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

    const onConnect = useCallback(
        (params: any) => setEdges((eds) => addEdge(params, eds)),
        [setEdges],
    );
    return (
        <ReactFlow
            colorMode="dark"
            nodes={nodes}
            edges={edges}
            nodeTypes={nodeTypes}
            edgeTypes={edgeTypes}
            connectionMode={ConnectionMode.Loose}
            elementsSelectable
            fitView
            minZoom={0.1}
            maxZoom={5}
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            onConnect={onConnect}
            // onMoveEnd={onUpdateCursor}
            // onPointerLeave={() =>
            //     setPresence({
            //         cursor: null,
            //     })
            // }
            proOptions={{ hideAttribution: true }
            }
        >
            {/* <ViewportPortal>
                {
                    Array.from(presenceMap.entries()).map(([user, presence]) => {
                        if (presence.cursor === null || presence.cursor === undefined) {
                            return null;
                        }
                        const { x, y } = presence.cursor;
                        return (
                            <div
                                key={user}
                                style={{ transform: `translate(${x}px, ${y}px)`, position: 'absolute' }
                                }
                            >
                                <Cursor color={colors.light} />
                            </div>
                        );
                    })}
            </ViewportPortal> */}
            {/* {others.map((user) => {
                if (user.presence.cursor === null || user.presence.cursor === undefined) {
                    return null;
                }
                return (
                    <Cursor
                        key={user.connectionId}
                        color={colors.light}
                        x={user.presence.cursor.x}
                        y={user.presence.cursor.y}
                    ></Cursor>
                );
            })} */}
            <Controls />
            < MiniMap nodeComponent={MiniMapNode} />
            <ViewportPortal>
                <div>
                    x
                </div>
            </ViewportPortal>
            {/* <Background
                    id="2"
                    gap={100}
                    lineWidth={3}
                    color={colors.lightGrey}
                    variant={BackgroundVariant.Lines}
                />
                <Background
                    id="1"
                    color={colors.light}
                    gap={50}
                    lineWidth={1}
                    variant={BackgroundVariant.Lines}
                /> */}

        </ReactFlow>
    )
}

const Diagram: FC = () => {

    // const { isOver, setNodeRef } = useDroppable({
    //     id: 'diagram',
    // });

    // return (
    //     <ReactFlowProvider >
    //         <DiagramCore />
    //     </ReactFlowProvider>
    //     // <div ref={setNodeRef}>
    //     // </div>
    // );
    const initialNodes = [
        { id: '1', position: { x: 0, y: 0 }, data: { label: '1' } },
        { id: '2', position: { x: 0, y: 100 }, data: { label: '2' } },
    ];
    const initialEdges = [{ id: 'e1-2', source: '1', target: '2' }];
    return (
        <div className="bg-pink-600" style={{ height: '100vh' }}>
            <text className="p-6 text-9xl text-pink-600">Hello</text>
            <ReactFlow nodes={initialNodes} edges={initialEdges} />
        </div>
    );
};

export default Diagram;