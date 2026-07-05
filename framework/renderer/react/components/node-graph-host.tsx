import { useCallback, useEffect, useMemo, useRef, useState, type DragEvent } from "react";
import {
	Diagram,
	Handle,
	Position,
	CATALOGUE_DRAG_MIME,
	ContextMenuController,
	type Edge,
	type Node,
	type NodeProps,
	type NodeTypes,
} from "@semio-tech/ui-react";
import { GraphWasmCanvas, type GraphWasmSession } from "@semio-tech/infinite-cavas-react-renderer";
import type { CommandDescriptor, NodeGraphScene, UiComponentSceneNode } from "../types.ts";
import { nodeGraphCommands } from "../types.ts";
import { useUIFindSafe } from "../ui-search-find.tsx";
import { FlowGraphCanvasHost } from "./flow-graph-canvas-host.tsx";
import {
	computeDagMarqueeOverlay,
	paintDagLabelOverlays,
	parseDagSelectionUnionBoundsScreen,
	sceneToSyncJson,
} from "./graph-canvas-overlays.tsx";
import { createGraphSession, isFlowGraphScene } from "../wasm-session-loader.ts";

//#region Types
type MediaGraphPort = {
	readonly id: string;
	readonly resourceKind?: string;
	readonly direction?: string;
	readonly label?: string;
};

type MediaGraphNodeRecord = {
	readonly id: string;
	readonly instanceId?: string;
	readonly label?: string;
	readonly x?: number;
	readonly y?: number;
	readonly width?: number;
	readonly height?: number;
	readonly inputs?: readonly MediaGraphPort[];
	readonly outputs?: readonly MediaGraphPort[];
};

type MediaGraphEdgeRecord = {
	readonly id: string;
	readonly sourceNodeId: string;
	readonly sourcePortId: string;
	readonly targetNodeId: string;
	readonly targetPortId: string;
};

type MediaGraphNodeData = {
	readonly label: string;
	readonly inputs: readonly MediaGraphPort[];
	readonly outputs: readonly MediaGraphPort[];
	readonly width: number;
	readonly height: number;
};

type DiagramViewport = { readonly x: number; readonly y: number; readonly zoom: number };

type GraphFindItem = { readonly id: string; readonly label: string; readonly category?: string };

type GraphContextMenuItem = {
	readonly id: string;
	readonly label: string;
	readonly command: string;
	readonly args?: Record<string, unknown>;
};

type FrameworkGraphSession = GraphWasmSession & {
	syncFromSceneJson(json: string): void;
	pointerDownScreen(sx: number, sy: number, button: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
	pointerMoveScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
	pointerUpScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
	wheelScreen(sx: number, sy: number, deltaX: number, deltaY: number, zoomGesture: boolean): void;
	labelOverlayPaintStateJson(): string;
	selectionUnionBoundsScreenJson(): string;
	selectionPreviewPointsJson(): string;
	selectionPreviewCrossing(): boolean;
	selectedNodeIdsJson(): string;
	hoveredNodeId(): string | null | undefined;
	hoveredChannelJson(): string;
	cameraJson(): string;
	takePendingOpenInstanceId(): string | null | undefined;
	setCanvasThemeJson?(json: string): void;
};
//#endregion Types

//#region Parsing
function parseViewport(viewportJson: string): DiagramViewport {
	try {
		const parsed = JSON.parse(viewportJson) as Partial<DiagramViewport>;
		return { x: Number(parsed.x ?? 0), y: Number(parsed.y ?? 0), zoom: Number(parsed.zoom ?? 1) };
	} catch {
		return { x: 0, y: 0, zoom: 1 };
	}
}

function parseJsonArray<T>(json: string | undefined): readonly T[] {
	if (!json) return [];
	try {
		return JSON.parse(json) as T[];
	} catch {
		return [];
	}
}

function portLabel(port: MediaGraphPort): string {
	if (port.label) return port.label;
	const segments = port.id.split(":");
	return segments[segments.length - 1] ?? port.id;
}

function mediaGraphNodesToDiagramNodes(records: readonly MediaGraphNodeRecord[]): Node<MediaGraphNodeData>[] {
	return records.map((record) => ({
		id: record.id,
		type: "mediaGraph",
		position: { x: record.x ?? 0, y: record.y ?? 0 },
		data: {
			label: record.label?.trim() || record.instanceId || record.id,
			inputs: record.inputs ?? [],
			outputs: record.outputs ?? [],
			width: record.width ?? 180,
			height: record.height ?? 72,
		},
	}));
}

function mediaGraphEdgesToDiagramEdges(records: readonly MediaGraphEdgeRecord[]): Edge[] {
	return records.map((record) => ({
		id: record.id,
		source: record.sourceNodeId,
		target: record.targetNodeId,
		sourceHandle: record.sourcePortId,
		targetHandle: record.targetPortId,
	}));
}
//#endregion Parsing

//#region DiagramNode
function MediaGraphDiagramNode({ data }: NodeProps<MediaGraphNodeData>) {
	const inputCount = Math.max(data.inputs.length, 1);
	const outputCount = Math.max(data.outputs.length, 1);
	const rowCount = Math.max(inputCount, outputCount);
	const rowHeight = 18;
	const bodyHeight = Math.max(data.height, 56 + rowCount * rowHeight);
	return (
		<div className="rounded border border-border bg-panel text-panel-foreground shadow-sm" style={{ width: data.width, minHeight: bodyHeight }}>
			<div className="border-b border-border px-2 py-1 text-xs font-medium">{data.label}</div>
			<div className="relative px-2 py-1 text-[10px] leading-[18px]">
				{Array.from({ length: rowCount }, (_, rowIndex) => {
					const input = data.inputs[rowIndex];
					const output = data.outputs[rowIndex];
					const top = 8 + rowIndex * rowHeight;
					return (
						<div key={`${input?.id ?? "in"}:${output?.id ?? "out"}:${rowIndex}`} className="relative h-[18px]">
							{input ? (
								<>
									<Handle id={input.id} type="target" position={Position.Left} className="!size-2 !border-panel !bg-foreground" style={{ top }} />
									<span className="pl-3 text-muted-foreground">{portLabel(input)}</span>
								</>
							) : null}
							{output ? (
								<>
									<Handle id={output.id} type="source" position={Position.Right} className="!size-2 !border-panel !bg-foreground" style={{ top }} />
									<span className="absolute right-3 top-0 text-right text-muted-foreground">{portLabel(output)}</span>
								</>
							) : null}
						</div>
					);
				})}
			</div>
		</div>
	);
}

const mediaGraphNodeTypes: NodeTypes = { mediaGraph: MediaGraphDiagramNode };
//#endregion DiagramNode

//#region WasmGraphSurface
function WasmGraphSurface({
	scene,
	surfaceId,
	controllerId,
	editable,
	contextMenuItems,
	onCommand,
}: {
	readonly scene: NodeGraphScene;
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly editable: boolean;
	readonly contextMenuItems: readonly GraphContextMenuItem[];
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const sessionRef = useRef<FrameworkGraphSession | null>(null);
	const labelCanvasRef = useRef<HTMLCanvasElement>(null);
	const containerRef = useRef<HTMLDivElement>(null);
	const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number } | null>(null);
	const [selectionBounds, setSelectionBounds] = useState<ReturnType<typeof parseDagSelectionUnionBoundsScreen>>(null);
	const [marquee, setMarquee] = useState<ReturnType<typeof computeDagMarqueeOverlay>>(null);
	const sceneJson = useMemo(() => sceneToSyncJson(scene), [scene]);

	const dispatch = useCallback(
		(command: string, args?: Record<string, unknown>) => {
			onCommand({ controllerId, command, args: { surfaceId, ...args } });
		},
		[controllerId, onCommand, surfaceId],
	);

	const paintOverlays = useCallback(() => {
		const session = sessionRef.current;
		const labelCanvas = labelCanvasRef.current;
		const container = containerRef.current;
		if (!session || !labelCanvas || !container) return;
		const rect = container.getBoundingClientRect();
		const dpr = globalThis.devicePixelRatio || 1;
		try {
			paintDagLabelOverlays(session.labelOverlayPaintStateJson(), labelCanvas, rect.width, rect.height, dpr);
		} catch {
			/* gpu not ready */
		}
		setSelectionBounds(parseDagSelectionUnionBoundsScreen(session.selectionUnionBoundsScreenJson()));
		setMarquee(
			computeDagMarqueeOverlay(session.selectionPreviewPointsJson(), session.selectionPreviewCrossing(), "rectangle"),
		);
	}, []);

	useEffect(() => {
		sessionRef.current?.syncFromSceneJson(sceneJson);
		paintOverlays();
	}, [sceneJson, paintOverlays]);

	const onSessionReady = useCallback(
		(session: GraphWasmSession) => {
			sessionRef.current = session as FrameworkGraphSession;
			sessionRef.current.syncFromSceneJson(sceneJson);
			paintOverlays();
		},
		[sceneJson, paintOverlays],
	);

	const [wasmSession, setWasmSession] = useState<FrameworkGraphSession | null>(null);

	useEffect(() => {
		let cancelled = false;
		void createGraphSession().then((session) => {
			if (!cancelled) setWasmSession(session as FrameworkGraphSession);
		});
		return () => {
			cancelled = true;
		};
	}, []);

	const sessionFactory = useCallback(() => {
		if (wasmSession) return wasmSession;
		return {
			attachCanvas: async () => undefined,
			setSize: () => {},
			renderFrame: () => {},
			syncFromSceneJson: () => {},
			pointerDownScreen: () => {},
			pointerMoveScreen: () => {},
			pointerUpScreen: () => {},
			wheelScreen: () => {},
			labelOverlayPaintStateJson: () => '{"labels":[]}',
			selectionUnionBoundsScreenJson: () => "{}",
			selectionPreviewPointsJson: () => "[]",
			selectionPreviewCrossing: () => false,
			selectedNodeIdsJson: () => "[]",
			hoveredNodeId: () => null,
			hoveredChannelJson: () => "{}",
			cameraJson: () => scene.viewportJson,
			takePendingOpenInstanceId: () => null,
		} satisfies FrameworkGraphSession;
	}, [scene.viewportJson, wasmSession]);

	const emitInteractionState = useCallback(() => {
		const session = sessionRef.current;
		if (!session) return;
		try {
			const nodeIds = JSON.parse(session.selectedNodeIdsJson()) as string[];
			dispatch(nodeGraphCommands.select, { nodeIds });
			const hovered = session.hoveredNodeId();
			dispatch(nodeGraphCommands.hover, { hoverJson: hovered ? JSON.stringify({ nodeId: hovered }) : null });
			dispatch(nodeGraphCommands.viewport, { viewportJson: session.cameraJson() });
			const openId = session.takePendingOpenInstanceId?.();
			if (openId) dispatch("openInstance", { instanceId: openId });
		} catch {
			/* session not ready */
		}
		paintOverlays();
	}, [dispatch, paintOverlays]);

	return (
		<div
			ref={containerRef}
			className="relative h-full w-full"
			onContextMenu={(event) => {
				if (!editable || contextMenuItems.length === 0) return;
				event.preventDefault();
				setContextMenu({ x: event.clientX, y: event.clientY });
			}}
			onPointerUp={emitInteractionState}
		>
			<GraphWasmCanvas className="absolute inset-0" sessionFactory={sessionFactory} onSessionReady={onSessionReady} enablePointer={false} />
			<canvas ref={labelCanvasRef} className="pointer-events-none absolute inset-0 z-40" />
			{selectionBounds ? (
				<div
					className="pointer-events-none absolute z-20 border-2 border-accent"
					style={{ left: selectionBounds.x, top: selectionBounds.y, width: selectionBounds.width, height: selectionBounds.height }}
				/>
			) : null}
			{marquee?.kind === "rect" && marquee.x != null && marquee.y != null && marquee.width != null && marquee.height != null ? (
				<div
					className={`pointer-events-none absolute z-20 border ${marquee.coverage === "partial" ? "border-dashed border-muted-foreground/60" : "border-accent/80 bg-accent/10"}`}
					style={{ left: marquee.x, top: marquee.y, width: marquee.width, height: marquee.height }}
				/>
			) : null}
			<div
				className="absolute inset-0 z-30"
				onPointerDown={(event) => {
					if (!editable) return;
					const session = sessionRef.current;
					if (!session?.pointerDownScreen) return;
					const rect = event.currentTarget.getBoundingClientRect();
					session.pointerDownScreen(
						event.clientX - rect.left,
						event.clientY - rect.top,
						event.button,
						event.shiftKey,
						event.metaKey || event.ctrlKey,
						event.altKey,
					);
					session.renderFrame();
					paintOverlays();
				}}
				onPointerMove={(event) => {
					const session = sessionRef.current;
					if (!session?.pointerMoveScreen) return;
					const rect = event.currentTarget.getBoundingClientRect();
					session.pointerMoveScreen(
						event.clientX - rect.left,
						event.clientY - rect.top,
						event.shiftKey,
						event.metaKey || event.ctrlKey,
						event.altKey,
					);
					session.renderFrame();
					paintOverlays();
				}}
				onPointerUp={(event) => {
					const session = sessionRef.current;
					if (!session?.pointerUpScreen) return;
					const rect = event.currentTarget.getBoundingClientRect();
					session.pointerUpScreen(
						event.clientX - rect.left,
						event.clientY - rect.top,
						event.shiftKey,
						event.metaKey || event.ctrlKey,
						event.altKey,
					);
					session.renderFrame();
					emitInteractionState();
				}}
				onWheel={(event) => {
					event.preventDefault();
					const session = sessionRef.current;
					if (!session?.wheelScreen) return;
					const rect = event.currentTarget.getBoundingClientRect();
					const delta = event.deltaMode === 1 ? event.deltaY * 16 : event.deltaMode === 2 ? event.deltaY * 400 : event.deltaY;
					session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, 0, delta, true);
					session.renderFrame();
					emitInteractionState();
				}}
			/>
			<ContextMenuController
				open={contextMenu != null}
				position={contextMenu ?? { x: 0, y: 0 }}
				items={contextMenuItems.map((item) => ({
					id: item.id,
					label: item.label,
					onSelect: () => dispatch(item.command, item.args),
				}))}
				onOpenChange={(open) => {
					if (!open) setContextMenu(null);
				}}
			/>
		</div>
	);
}
//#endregion WasmGraphSurface

//#region DiagramFallback
function DiagramGraphFallback({
	scene,
	node,
	editable,
	parsedNodes,
	parsedEdges,
	findItems,
	contextMenuItems,
	onCommand,
}: {
	readonly scene: NodeGraphScene;
	readonly node: UiComponentSceneNode;
	readonly editable: boolean;
	readonly parsedNodes: readonly MediaGraphNodeRecord[];
	readonly parsedEdges: readonly MediaGraphEdgeRecord[];
	readonly findItems: readonly GraphFindItem[];
	readonly contextMenuItems: readonly GraphContextMenuItem[];
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const viewport = useMemo(() => parseViewport(scene.viewportJson ?? "{}"), [scene.viewportJson]);
	const initialNodes = useMemo(() => mediaGraphNodesToDiagramNodes(parsedNodes), [parsedNodes]);
	const initialEdges = useMemo(() => mediaGraphEdgesToDiagramEdges(parsedEdges), [parsedEdges]);
	const [nodes, setNodes] = useState(initialNodes);
	const [edges, setEdges] = useState(initialEdges);
	useEffect(() => {
		setNodes(initialNodes);
		setEdges(initialEdges);
	}, [initialNodes, initialEdges]);

	const dispatch = useCallback(
		(command: string, args?: Record<string, unknown>) => {
			onCommand({ controllerId: node.controllerId, command, args: { surfaceId: node.surfaceId, ...args } });
		},
		[node.controllerId, node.surfaceId, onCommand],
	);

	const containerRef = useRef<HTMLDivElement>(null);
	const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number } | null>(null);

	return (
		<div
			ref={containerRef}
			className="relative h-full w-full"
			onDragOver={(event) => {
				if (editable && event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) event.preventDefault();
			}}
			onDrop={(event: DragEvent<HTMLDivElement>) => {
				if (!editable) return;
				event.preventDefault();
				const raw = event.dataTransfer.getData(CATALOGUE_DRAG_MIME);
				if (!raw) return;
				let payload: { readonly programId?: string; readonly appId?: string };
				try {
					payload = JSON.parse(raw) as { readonly programId?: string; readonly appId?: string };
				} catch {
					return;
				}
				if (!payload.programId || !payload.appId) return;
				const rect = containerRef.current?.getBoundingClientRect();
				if (!rect) return;
				const x = (event.clientX - rect.left - viewport.x) / viewport.zoom;
				const y = (event.clientY - rect.top - viewport.y) / viewport.zoom;
				dispatch("spawnApp", { programId: payload.programId, appId: payload.appId, position: { x, y } });
			}}
			onContextMenu={(event) => {
				if (!editable || contextMenuItems.length === 0) return;
				event.preventDefault();
				setContextMenu({ x: event.clientX, y: event.clientY });
			}}
		>
			<Diagram
				className="h-full w-full"
				nodeTypes={mediaGraphNodeTypes}
				nodes={nodes}
				edges={edges}
				fitView={false}
				defaultViewport={viewport}
				minZoom={0.05}
				maxZoom={32}
				panOnDrag={[0, 1]}
				selectionOnDrag
				elementsSelectable
				nodesDraggable={editable}
				nodesConnectable={editable}
				edgesReconnectable={editable}
				onNodesChange={(nextNodes) => setNodes(nextNodes as Node<MediaGraphNodeData>[])}
				onEdgesChange={(nextEdges) => setEdges(nextEdges)}
				onNodeDragStop={
					editable
						? (_event, draggedNode) => {
								dispatch(nodeGraphCommands.edit, {
									ops: [{ op: "move", nodeId: draggedNode.id, x: draggedNode.position.x, y: draggedNode.position.y }],
								});
							}
						: undefined
				}
				onConnect={
					editable
						? (connection) => {
								if (!connection.source || !connection.target || !connection.sourceHandle || !connection.targetHandle) return;
								dispatch(nodeGraphCommands.edit, {
									ops: [
										{
											op: "connect",
											sourceNodeId: connection.source,
											sourcePortId: connection.sourceHandle,
											targetNodeId: connection.target,
											targetPortId: connection.targetHandle,
										},
									],
								});
							}
						: undefined
				}
				onNodeClick={(_event, clickedNode) => {
					const record = parsedNodes.find((entry) => entry.id === clickedNode.id);
					if (record?.instanceId) dispatch("selectInstance", { instanceId: record.instanceId });
					dispatch(nodeGraphCommands.select, { nodeIds: [clickedNode.id] });
				}}
				onNodeDoubleClick={(_event, clickedNode) => {
					const record = parsedNodes.find((entry) => entry.id === clickedNode.id);
					if (record?.instanceId) dispatch("openInstance", { instanceId: record.instanceId });
				}}
				onSelectionChange={(selection) => {
					const nodeIds = selection.nodes.map((entry) => entry.id);
					dispatch(nodeGraphCommands.select, { nodeIds });
				}}
			/>
			<ContextMenuController
				open={contextMenu != null}
				position={contextMenu ?? { x: 0, y: 0 }}
				items={contextMenuItems.map((item) => ({
					id: item.id,
					label: item.label,
					onSelect: () => dispatch(item.command, item.args),
				}))}
				onOpenChange={(open) => {
					if (!open) setContextMenu(null);
				}}
			/>
		</div>
	);
}
//#endregion DiagramFallback

//#region NodeGraphHost
const useClient = () => {
	const [client, setClient] = useState(false);
	useEffect(() => setClient(true), []);
	return client;
};

export function NodeGraphHost({
	node,
	onCommand,
}: {
	readonly node: UiComponentSceneNode;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const scene = node.nodeGraph;
	const editable = scene?.editable ?? true;
	const parsedNodes = useMemo(() => parseJsonArray<MediaGraphNodeRecord>(scene?.nodesJson), [scene?.nodesJson]);
	const parsedEdges = useMemo(() => parseJsonArray<MediaGraphEdgeRecord>(scene?.edgesJson), [scene?.edgesJson]);
	const findItems = useMemo(() => parseJsonArray<GraphFindItem>(scene?.findItemsJson), [scene?.findItemsJson]);
	const contextMenuItems = useMemo(() => parseJsonArray<GraphContextMenuItem>(scene?.contextMenuJson), [scene?.contextMenuJson]);
	const isClient = useClient();

	const dispatch = useCallback(
		(command: string, args?: Record<string, unknown>) => {
			onCommand({ controllerId: node.controllerId, command, args: { surfaceId: node.surfaceId, ...args } });
		},
		[node.controllerId, node.surfaceId, onCommand],
	);

	const findContext = useUIFindSafe();
	const onFindItemRef = useRef<(itemId: string) => void>(() => {});
	onFindItemRef.current = (itemId: string) => {
		const mediaNode = parsedNodes.find((entry) => entry.instanceId === itemId);
		if (!mediaNode) return;
		dispatch(nodeGraphCommands.select, { nodeIds: [mediaNode.id] });
		dispatch("selectInstance", { instanceId: mediaNode.instanceId! });
	};

	useEffect(() => {
		if (!findContext?.setFindItems || findItems.length === 0) return;
		findContext.setFindItems(findItems);
	}, [findContext?.setFindItems, findItems]);

	useEffect(() => {
		if (!findContext?.setOnFindItem || findItems.length === 0) return;
		findContext.setOnFindItem((itemId) => onFindItemRef.current(itemId));
		return () => findContext.setOnFindItem?.(undefined);
	}, [findContext?.setOnFindItem, findItems.length]);

	if (!scene) return <div className="semio-node-graph-empty">No graph scene</div>;

	const useFlowEngine = isFlowGraphScene(scene.capabilitiesJson) || Boolean(scene.fixtureJson);

	return (
		<div className="semio-node-graph-host relative h-full min-h-[24rem] w-full" data-surface-id={node.surfaceId}>
			{isClient ? (
				useFlowEngine ? (
					<FlowGraphCanvasHost
						scene={scene}
						surfaceId={node.surfaceId}
						controllerId={node.controllerId}
						editable={editable}
						contextMenuItems={contextMenuItems}
						onCommand={onCommand}
					/>
				) : (
					<WasmGraphSurface
						scene={scene}
						surfaceId={node.surfaceId}
						controllerId={node.controllerId}
						editable={editable}
						contextMenuItems={contextMenuItems}
						onCommand={onCommand}
					/>
				)
			) : (
				<DiagramGraphFallback
					scene={scene}
					node={node}
					editable={editable}
					parsedNodes={parsedNodes}
					parsedEdges={parsedEdges}
					findItems={findItems}
					contextMenuItems={contextMenuItems}
					onCommand={onCommand}
				/>
			)}
		</div>
	);
}
//#endregion NodeGraphHost
