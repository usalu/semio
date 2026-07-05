import { useCallback, useEffect, useMemo, useRef, useState, type DragEvent, type KeyboardEvent } from "react";
import {
	CATALOGUE_DRAG_MIME,
	CanvasPickMenu,
	ContextMenuController,
	Diagram,
	Handle,
	Position,
	SelectionMarquee,
	useCanvasPickInteraction,
	type CanvasPickTarget,
	type Edge,
	type Node,
	type NodeProps,
	type NodeTypes,
} from "@semio-tech/ui-react";
import { GraphWasmCanvas, type GraphWasmSession } from "@semio-tech/infinite-cavas-react-renderer";
import type { CommandDescriptor, NodeGraphScene, PresencePeer, UiComponentSceneNode } from "../types.ts";
import { nodeGraphCommands } from "../types.ts";
import { useUIFindSafe } from "../ui-search-find.tsx";
import { FlowGraphCanvasHost } from "./flow-graph-canvas-host.tsx";
import {
	computeDagMarqueeOverlay,
	GraphParamOverlays,
	GraphStepperOverlays,
	paintDagLabelOverlays,
	parseDagSelectionUnionBoundsScreen,
	SelectionAlignChrome,
	alignModeToDag,
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
	paramOverlayPaintStateJson(): string;
	stepperOverlayStateJson(): string;
	selectionUnionBoundsScreenJson(): string;
	selectionPreviewPointsJson(): string;
	selectionPreviewCrossing(): boolean;
	selectedNodeIdsJson(): string;
	hoveredNodeId(): string | null | undefined;
	hoveredChannelJson(): string;
	cameraJson(): string;
	takePendingOpenInstanceId(): string | null | undefined;
	pickTargetsAtScreenJson(sx: number, sy: number): string;
	setHover?(widgetId: string | null): void;
	setHoverChannel?(widgetId: string | null, port?: string | null): void;
	alignSelection?(mode: string): void;
	fixtureJson?(): string;
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

//#region Keyboard
function isEditableGraphKeyTarget(target: EventTarget | null): boolean {
	if (!(target instanceof HTMLElement)) return false;
	const tag = target.tagName;
	if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
	if (target.isContentEditable) return true;
	return target.closest("[contenteditable='true'], [role='textbox']") != null;
}

function handleGraphKeyboard(
	event: KeyboardEvent<HTMLDivElement>,
	editable: boolean,
	parsedNodes: readonly MediaGraphNodeRecord[],
	dispatch: (command: string, args?: Record<string, unknown>) => void,
) {
	if (!editable || isEditableGraphKeyTarget(event.target)) return;
	const mod = event.metaKey || event.ctrlKey;
	if (mod && event.key.toLowerCase() === "a") {
		event.preventDefault();
		dispatch("setMediaNodeSelection", { nodeIds: parsedNodes.map((node) => node.id) });
		return;
	}
	if (event.key === "Escape") {
		event.preventDefault();
		dispatch("setMediaNodeSelection", { nodeIds: [] });
		return;
	}
	if (event.key === "Delete" || event.key === "Backspace") {
		event.preventDefault();
		dispatch("deleteSelection", {});
	}
}
//#endregion Keyboard

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
	const [overlaySize, setOverlaySize] = useState({ w: 0, h: 0 });
	const [paramStateJson, setParamStateJson] = useState("{}");
	const [stepperStateJson, setStepperStateJson] = useState("{}");
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
		try {
			setParamStateJson(session.paramOverlayPaintStateJson());
			setStepperStateJson(session.stepperOverlayStateJson());
		} catch {
			/* session not ready */
		}
		setOverlaySize({ w: rect.width, h: rect.height });
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
			paramOverlayPaintStateJson: () => "{}",
			stepperOverlayStateJson: () => "{}",
			selectionUnionBoundsScreenJson: () => "{}",
			selectionPreviewPointsJson: () => "[]",
			selectionPreviewCrossing: () => false,
			selectedNodeIdsJson: () => "[]",
			hoveredNodeId: () => null,
			hoveredChannelJson: () => "{}",
			cameraJson: () => scene.viewportJson,
			pickTargetsAtScreenJson: () => "[]",
			setHover: () => {},
			setHoverChannel: () => {},
			alignSelection: () => {},
			fixtureJson: () => "{}",
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

	const commitGraphFixture = useCallback(() => {
		const session = sessionRef.current;
		if (!session?.fixtureJson) return;
		try {
			const fixtureJson = session.fixtureJson();
			dispatch(nodeGraphCommands.edit, { ops: [{ op: "setFixture", fixtureJson }] });
		} catch {
			/* session not ready */
		}
	}, [dispatch]);

	const pickInteraction = useCanvasPickInteraction({
		resolveTargetsAtClient: (client) => {
			const session = sessionRef.current;
			const container = containerRef.current;
			if (!session?.pickTargetsAtScreenJson || !container) return [];
			const rect = container.getBoundingClientRect();
			const sx = client.x - rect.left;
			const sy = client.y - rect.top;
			try {
				return JSON.parse(session.pickTargetsAtScreenJson(sx, sy)) as CanvasPickTarget[];
			} catch {
				return [];
			}
		},
		onHoverFocus: (focus) => {
			const session = sessionRef.current;
			if (!session) return;
			const target = focus.target;
			if (!target) {
				session.setHover?.(null);
			} else if (target.portId) {
				session.setHoverChannel?.(target.id, target.portId);
			} else {
				session.setHover?.(target.id);
			}
			session.renderFrame();
			paintOverlays();
		},
		onSelectTarget: () => {
			emitInteractionState();
		},
	});

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
			{marquee ? (
				<SelectionMarquee
					coverage={marquee.coverage ?? "full"}
					shape={
						marquee.kind === "lasso"
							? { shape: "polygon", points: marquee.points ?? [] }
							: { shape: "rect", rect: { x: marquee.x ?? 0, y: marquee.y ?? 0, width: marquee.width ?? 0, height: marquee.height ?? 0 } }
					}
				/>
			) : null}
			<div
				className="absolute inset-0 z-30"
				onPointerDown={(event) => {
					if (!editable) return;
					const session = sessionRef.current;
					if (!session?.pointerDownScreen) return;
					const rect = event.currentTarget.getBoundingClientRect();
					const client = { x: event.clientX, y: event.clientY };
					pickInteraction.onCanvasPointerDown(client);
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
					const client = { x: event.clientX, y: event.clientY };
					pickInteraction.onCanvasPointerMove(client);
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
					const client = { x: event.clientX, y: event.clientY };
					pickInteraction.onCanvasPointerUp(client, { shift: event.shiftKey, ctrlOrMeta: event.metaKey || event.ctrlKey, alt: event.altKey });
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
				onPointerLeave={() => pickInteraction.onCanvasPointerLeave()}
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
			{selectionBounds && editable ? (
				<SelectionAlignChrome
					bounds={selectionBounds}
					onAlign={(mode) => {
						const session = sessionRef.current;
						if (!session?.alignSelection) return;
						session.alignSelection(alignModeToDag(mode));
						commitGraphFixture();
						session.renderFrame();
						emitInteractionState();
					}}
				/>
			) : null}
			<GraphParamOverlays
				stateJson={paramStateJson}
				logicalW={overlaySize.w}
				logicalH={overlaySize.h}
				editable={editable}
				onParamChange={(nodeId, portId, value) =>
					dispatch(nodeGraphCommands.edit, { op: "setParam", nodeId, portId, value })
				}
			/>
			<GraphStepperOverlays
				stateJson={stepperStateJson}
				logicalW={overlaySize.w}
				logicalH={overlaySize.h}
				editable={editable}
				onStepperChange={(widgetId, fieldKey, value) =>
					dispatch(nodeGraphCommands.edit, { op: "setStepper", widgetId, fieldKey, value })
				}
			/>
			<CanvasPickMenu
				request={pickInteraction.pickMenu}
				hoveredKey={pickInteraction.menuHoveredKey}
				onHoverKey={pickInteraction.onMenuHoverKey}
				onPick={pickInteraction.onMenuPick}
				onDismiss={pickInteraction.dismissPickMenu}
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

function PresencePeersOverlay({ peers }: { readonly peers: readonly PresencePeer[] }) {
	if (peers.length === 0) return null;
	return (
		<div className="pointer-events-none absolute right-2 top-2 z-panel flex max-w-[14rem] flex-col gap-1 rounded border border-border/60 bg-window/90 px-2 py-1 text-xs shadow-sm">
			{peers.map((peer) => (
				<div key={peer.clientId} className="flex items-center justify-between gap-2 text-muted-foreground">
					<span className="truncate font-medium text-foreground">{peer.name}</span>
					<span>{peer.selectionCount} selected</span>
				</div>
			))}
		</div>
	);
}

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
	const presencePeers = useMemo(() => parseJsonArray<PresencePeer>(scene?.presencePeersJson), [scene?.presencePeersJson]);
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
		<div
			className="semio-node-graph-host relative h-full min-h-[24rem] w-full"
			data-surface-id={node.surfaceId}
			tabIndex={editable ? 0 : undefined}
			onKeyDown={(event) => handleGraphKeyboard(event, editable, parsedNodes, dispatch)}
		>
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
			<PresencePeersOverlay peers={presencePeers} />
		</div>
	);
}
//#endregion NodeGraphHost
