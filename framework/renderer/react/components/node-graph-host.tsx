import { useCallback, useEffect, useMemo, useRef, useState, type DragEvent } from "react";
import {
	Diagram,
	Handle,
	Position,
	CATALOGUE_DRAG_MIME,
	type Edge,
	type Node,
	type NodeProps,
	type NodeTypes,
} from "@semio-tech/ui-react";
import type { CommandDescriptor, UiComponentSceneNode } from "../types.ts";
import { useUIFindSafe } from "../ui-search-find.tsx";

//#region MediaGraphTypes
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

type DiagramViewport = {
	readonly x: number;
	readonly y: number;
	readonly zoom: number;
};

type GraphFindItem = {
	readonly id: string;
	readonly label: string;
	readonly category?: string;
};

type GraphContextMenuItem = {
	readonly id: string;
	readonly label: string;
	readonly command: string;
	readonly args?: Record<string, unknown>;
};
//#endregion MediaGraphTypes

//#region MediaGraphDiagramNode
function portLabel(port: MediaGraphPort): string {
	if (port.label) return port.label;
	const segments = port.id.split(":");
	return segments[segments.length - 1] ?? port.id;
}

function MediaGraphDiagramNode({ data }: NodeProps<MediaGraphNodeData>) {
	const inputCount = Math.max(data.inputs.length, 1);
	const outputCount = Math.max(data.outputs.length, 1);
	const rowCount = Math.max(inputCount, outputCount);
	const rowHeight = 18;
	const bodyHeight = Math.max(data.height, 56 + rowCount * rowHeight);
	return (
		<div
			className="rounded border border-border bg-panel text-panel-foreground shadow-sm"
			style={{ width: data.width, minHeight: bodyHeight }}
		>
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
									<Handle
										id={input.id}
										type="target"
										position={Position.Left}
										className="!size-2 !border-panel !bg-foreground"
										style={{ top }}
									/>
									<span className="pl-3 text-muted-foreground">{portLabel(input)}</span>
								</>
							) : null}
							{output ? (
								<>
									<Handle
										id={output.id}
										type="source"
										position={Position.Right}
										className="!size-2 !border-panel !bg-foreground"
										style={{ top }}
									/>
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
//#endregion MediaGraphDiagramNode

//#region MediaGraphSceneParsing
function parseViewport(viewportJson: string): DiagramViewport {
	try {
		const parsed = JSON.parse(viewportJson) as Partial<DiagramViewport>;
		return {
			x: Number(parsed.x ?? 0),
			y: Number(parsed.y ?? 0),
			zoom: Number(parsed.zoom ?? 1),
		};
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
//#endregion MediaGraphSceneParsing

//#region NodeGraphHost
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
	const contextMenuItems = useMemo(
		() => parseJsonArray<GraphContextMenuItem>(scene?.contextMenuJson),
		[scene?.contextMenuJson],
	);
	const viewport = useMemo(() => parseViewport(scene?.viewportJson ?? "{}"), [scene?.viewportJson]);
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
			onCommand({
				controllerId: node.controllerId,
				command,
				args: { surfaceId: node.surfaceId, ...args },
			});
		},
		[node.controllerId, node.surfaceId, onCommand],
	);

	const findContext = useUIFindSafe();
	const setFindItems = findContext?.setFindItems;
	const setOnFindItem = findContext?.setOnFindItem;
	const onFindItemRef = useRef<(itemId: string) => void>(() => {});

	onFindItemRef.current = (itemId: string) => {
		const mediaNode = parsedNodes.find((entry) => entry.instanceId === itemId);
		if (!mediaNode) return;
		dispatch("setMediaNodeSelection", { nodeIds: [mediaNode.id] });
		dispatch("selectInstance", { instanceId: mediaNode.instanceId! });
	};

	useEffect(() => {
		if (!setFindItems || findItems.length === 0) return;
		setFindItems(findItems);
	}, [findItems, setFindItems]);

	useEffect(() => {
		if (!setOnFindItem || findItems.length === 0) return;
		setOnFindItem((itemId) => onFindItemRef.current(itemId));
		return () => setOnFindItem(undefined);
	}, [findItems.length, setOnFindItem]);

	const containerRef = useRef<HTMLDivElement>(null);

	const handleCatalogueDrop = useCallback(
		(event: DragEvent<HTMLDivElement>) => {
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
		},
		[dispatch, editable, viewport.x, viewport.y, viewport.zoom],
	);

	const handleNodeDoubleClick = useCallback(
		(_event: React.MouseEvent, clickedNode: Node) => {
			const record = parsedNodes.find((entry) => entry.id === clickedNode.id);
			if (record?.instanceId) dispatch("openInstance", { instanceId: record.instanceId });
		},
		[dispatch, parsedNodes],
	);

	if (!scene) return <div className="semio-node-graph-empty">No graph scene</div>;

	return (
		<div
			ref={containerRef}
			className="semio-node-graph-host relative h-full min-h-[24rem] w-full"
			data-surface-id={node.surfaceId}
			onDragOver={(event) => {
				if (editable && event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) event.preventDefault();
			}}
			onDrop={handleCatalogueDrop}
			onContextMenu={(event) => {
				if (!editable || contextMenuItems.length === 0) return;
				event.preventDefault();
				const first = contextMenuItems[0];
				if (first) dispatch(first.command, first.args);
			}}
		>
			<Diagram
				className="h-full w-full"
				nodeTypes={mediaGraphNodeTypes}
				nodes={nodes}
				edges={edges}
				fitView={false}
				defaultViewport={viewport}
				minZoom={0.1}
				maxZoom={4}
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
								dispatch("moveMediaNode", { nodeId: draggedNode.id, x: draggedNode.position.x, y: draggedNode.position.y });
							}
						: undefined
				}
				onConnect={
					editable
						? (connection) => {
								if (!connection.source || !connection.target || !connection.sourceHandle || !connection.targetHandle) return;
								dispatch("connectMediaPorts", {
									sourceNodeId: connection.source,
									sourcePortId: connection.sourceHandle,
									targetNodeId: connection.target,
									targetPortId: connection.targetHandle,
								});
							}
						: undefined
				}
				onNodeClick={(_event, clickedNode) => {
					const record = parsedNodes.find((entry) => entry.id === clickedNode.id);
					if (record?.instanceId) dispatch("selectInstance", { instanceId: record.instanceId });
					else dispatch("selectNode", { nodeId: clickedNode.id });
				}}
				onNodeDoubleClick={handleNodeDoubleClick}
				onSelectionChange={
					findItems.length > 0
						? (selection) => {
								const nodeIds = selection.nodes.map((entry) => entry.id);
								dispatch("setMediaNodeSelection", { nodeIds });
							}
						: undefined
				}
				onPaneClick={() => dispatch("graphPointerDown")}
			/>
		</div>
	);
}
//#endregion NodeGraphHost
