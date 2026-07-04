import { useCallback, useEffect, useMemo, useRef, useState, type DragEvent } from "react";
import { CATALOGUE_DRAG_MIME } from "@semio-tech/ui-react";
import {
	FlowCanvas,
	FlowExtensionHost,
	buildFlowContextMenuItems,
	createEphemeralFlowStore,
	type FlowCanvasCommandRequest,
	type FlowCanvasContextMenuContext,
	type FlowContextMenuDispatch,
	type FlowModuleOperatorInfo,
} from "@semio-tech/flow-react";
import type { CommandDescriptor, UiComponentSceneNode } from "../types.ts";
import { OS_MEDIA_FLOW_MODULE_ID, applyFlowFixtureJsonToMediaGraphCommands, parseMediaGraphFromFixture } from "../os-media-graph-flow.ts";

//#region FlowCanvasHost
const OS_MEDIA_FLOW_CTX_SKIP = new Set([
	"flow.ctx.add",
	"flow.ctx.preview",
	"flow.ctx.collapse",
	"flow.ctx.explode",
	"flow.ctx.replaceImage",
]);

function flowScreenToWorld(
	camera: { readonly x: number; readonly y: number; readonly zoom: number },
	sx: number,
	sy: number,
	viewportW: number,
	viewportH: number,
): { readonly x: number; readonly y: number } {
	const cx = viewportW / 2;
	const cy = viewportH / 2;
	return {
		x: (sx - cx) / camera.zoom + camera.x,
		y: (sy - cy) / camera.zoom + camera.y,
	};
}

function buildMediaFlowContextMenu(ctx: FlowCanvasContextMenuContext, dispatch: FlowContextMenuDispatch) {
	return buildFlowContextMenuItems(ctx, dispatch).filter((item) => !item.id || !OS_MEDIA_FLOW_CTX_SKIP.has(item.id));
}

export function FlowCanvasHost({
	node,
	onCommand,
}: {
	readonly node: UiComponentSceneNode;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const scene = node.flowCanvas;
	const containerRef = useRef<HTMLDivElement>(null);
	const cameraRef = useRef({ x: 0, y: 0, zoom: 1 });
	const flowStoreRef = useRef(createEphemeralFlowStore());
	const extensionHostRef = useRef<FlowExtensionHost | null>(null);
	if (!extensionHostRef.current) extensionHostRef.current = new FlowExtensionHost();
	const extensionHost = extensionHostRef.current;
	const [extensionRevision, setExtensionRevision] = useState(() => extensionHost.getRevision());
	const [commandRequest, setCommandRequest] = useState<FlowCanvasCommandRequest | undefined>();
	const editable = scene?.editable ?? true;
	const fixtureJson = scene?.fixtureJson ?? "{}";
	const operators = useMemo((): readonly FlowModuleOperatorInfo[] => {
		if (!scene?.operatorsJson) return [];
		try {
			return JSON.parse(scene.operatorsJson) as FlowModuleOperatorInfo[];
		} catch {
			return [];
		}
	}, [scene?.operatorsJson]);

	useEffect(() => {
		extensionHost.registerContributions(OS_MEDIA_FLOW_MODULE_ID, operators);
		return () => extensionHost.unregisterContributions(OS_MEDIA_FLOW_MODULE_ID);
	}, [extensionHost, operators]);

	useEffect(() => extensionHost.subscribe(() => setExtensionRevision(extensionHost.getRevision())), [extensionHost]);

	const graph = useMemo(() => parseMediaGraphFromFixture(fixtureJson), [fixtureJson]);

	const dispatchCommand = useCallback(
		(command: string, args?: Record<string, unknown>) => {
			onCommand({
				controllerId: node.controllerId,
				command,
				args: { surfaceId: node.surfaceId, ...args },
			});
		},
		[node.controllerId, node.surfaceId, onCommand],
	);

	const handleFixtureChange = useCallback(
		(nextJson: string) => {
			const next = JSON.parse(nextJson) as { readonly camera?: { readonly x: number; readonly y: number; readonly zoom: number } };
			if (next.camera) cameraRef.current = next.camera;
			if (!editable) return;
			for (const command of applyFlowFixtureJsonToMediaGraphCommands(graph, nextJson, node.controllerId)) {
				onCommand(command);
			}
		},
		[editable, graph, node.controllerId, onCommand],
	);

	const handleSelectionChange = useCallback(
		(nodeIds: readonly string[]) => {
			dispatchCommand("setMediaNodeSelection", { nodeIds: [...nodeIds] });
			const nodeId = nodeIds[0];
			if (!nodeId) return;
			const mediaNode = graph.nodes.find((entry) => entry.id === nodeId);
			if (mediaNode) dispatchCommand("selectInstance", { instanceId: mediaNode.instanceId });
		},
		[dispatchCommand, graph.nodes],
	);

	const handleWidgetDoubleClick = useCallback(
		(widgetId: string) => {
			const mediaNode = graph.nodes.find((entry) => entry.id === widgetId);
			if (mediaNode) dispatchCommand("openInstance", { instanceId: mediaNode.instanceId });
		},
		[dispatchCommand, graph.nodes],
	);

	const onCanvasCommand = useCallback((command: string, args?: Record<string, unknown>) => {
		setCommandRequest({
			command,
			argsJson: JSON.stringify(args ?? {}),
			epoch: Date.now(),
		});
	}, []);

	const handleDrop = useCallback(
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
			const position = flowScreenToWorld(cameraRef.current, event.clientX - rect.left, event.clientY - rect.top, rect.width, rect.height);
			dispatchCommand("spawnApp", { programId: payload.programId, appId: payload.appId, position });
		},
		[dispatchCommand, editable],
	);

	if (!scene) return <div className="semio-flow-canvas-empty">No flow scene</div>;

	return (
		<div
			ref={containerRef}
			className="semio-flow-canvas-host relative h-full min-h-[24rem] w-full"
			data-surface-id={node.surfaceId}
			onDragOver={(event) => {
				if (editable && event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) event.preventDefault();
			}}
			onDrop={handleDrop}
		>
			<FlowCanvas
				className="h-full w-full"
				fixtureJson={fixtureJson}
				store={flowStoreRef.current}
				extensionHost={extensionHost}
				extensionRevision={extensionRevision}
				onFixtureChange={editable ? handleFixtureChange : undefined}
				onSelectionChange={handleSelectionChange}
				onWidgetDoubleClick={handleWidgetDoubleClick}
				enableSpotlight={false}
				contextMenu={editable ? (ctx) => buildMediaFlowContextMenu(ctx, onCanvasCommand) : undefined}
				commandRequest={commandRequest}
				automaticLod
			/>
		</div>
	);
}
//#endregion FlowCanvasHost
