// #region 🧲Header
/** @emoji 🖥️ `@semio-tech/s-react` — studio provider and media graph canvas. */
// #endregion 🧲Header

import React, { createContext, useCallback, useContext, useEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import {
	FlowCanvas,
	FlowExtensionHost,
	buildFlowContextMenuItems,
	createEphemeralFlowStore,
	type FlowCanvasCommandRequest,
	type FlowCanvasContextMenuContext,
	type FlowContextMenuDispatch,
	type FlowSession,
} from "@semio-tech/flow-react";
import {
	OS_MEDIA_FLOW_MODULE_ID,
	type SAppInstance,
	type SMediaGraph,
	type SParameter,
	type StudioCommand,
	type StudioStore,
	applyFlowFixtureJsonToSMediaGraph,
	buildSMediaFlowOperatorInfos,
	sMediaGraphToFlowFixtureJson,
} from "@semio-tech/s-core";
import type { PresencePeer } from "@semio-tech/framework-os-core";
import { CATALOGUE_DRAG_MIME } from "@semio-tech/ui-react";

//#region 🔖StudioContext
const StudioStoreContext = createContext<StudioStore | null>(null);

export function SStudioProvider({ store, children }: { readonly store: StudioStore; readonly children: React.ReactNode }): React.ReactElement {
	return <StudioStoreContext.Provider value={store}>{children}</StudioStoreContext.Provider>;
}

export function useStudioStore(): StudioStore {
	const store = useContext(StudioStoreContext);
	if (!store) throw new Error("SStudioProvider is required");
	return store;
}

export function useStudioProjection(): ReturnType<StudioStore["projection"]> {
	const store = useStudioStore();
	return useSyncExternalStore(
		store.subscribe.bind(store),
		() => {
			void store.getGeneration();
			return store.projection();
		},
		() => store.projection(),
	);
}

export function useStudioGeneration(): number {
	const store = useStudioStore();
	return useSyncExternalStore(store.subscribe.bind(store), () => store.getGeneration(), () => store.getGeneration());
}

export function useDispatchStudioCommand(): (command: StudioCommand) => void {
	const store = useStudioStore();
	return useCallback((command: StudioCommand) => store.dispatch(command), [store]);
}
//#endregion 🔖StudioContext

//#region 🔖MediaGraphFlowExtension
const OS_MEDIA_FLOW_CTX_SKIP = new Set([
	"flow.ctx.add",
	"flow.ctx.preview",
	"flow.ctx.collapse",
	"flow.ctx.explode",
	"flow.ctx.replaceImage",
]);

/** @emoji 🖱️ Flow play context menu without spotlight-only entries for OS media graph neurons. */
export function buildSMediaFlowContextMenu(
	ctx: FlowCanvasContextMenuContext,
	dispatch: FlowContextMenuDispatch,
): ReturnType<typeof buildFlowContextMenuItems> {
	return buildFlowContextMenuItems(ctx, dispatch).filter((item) => !item.id || !OS_MEDIA_FLOW_CTX_SKIP.has(item.id));
}

function useOsMediaFlowExtension(
	extensionHost: FlowExtensionHost,
	graph: SMediaGraph,
	instances: readonly SAppInstance[],
	parameters: readonly SParameter[],
): number {
	const [revision, setRevision] = useState(() => extensionHost.getRevision());
	useEffect(() => {
		const operators = buildSMediaFlowOperatorInfos(graph, instances, parameters);
		extensionHost.registerContributions(OS_MEDIA_FLOW_MODULE_ID, operators);
		setRevision(extensionHost.getRevision());
		return () => extensionHost.unregisterContributions(OS_MEDIA_FLOW_MODULE_ID);
	}, [extensionHost, graph, instances, parameters]);
	useEffect(() => extensionHost.subscribe(() => setRevision(extensionHost.getRevision())), [extensionHost]);
	return revision;
}

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
//#endregion 🔖MediaGraphFlowExtension

//#region 🔖MediaGraphCanvas
export interface SMediaGraphCanvasProps {
	readonly graph: SMediaGraph;
	readonly instances: readonly SAppInstance[];
	readonly parameters?: readonly SParameter[];
	readonly projectionGeneration?: number;
	readonly activeInstanceId?: string | null;
	readonly onSelectInstance?: (instanceId: string) => void;
	readonly onOpenInstance?: (instanceId: string) => void;
	readonly onMoveNode?: (nodeId: string, x: number, y: number) => void;
	readonly onConnectPorts?: (sourceNodeId: string, sourcePortId: string, targetNodeId: string, targetPortId: string) => void;
	readonly onRemoveInstance?: (instanceId: string) => void;
	readonly onDisconnectEdge?: (edgeId: string) => void;
	readonly onSpawnApp?: (programId: string, appId: string, position: { readonly x: number; readonly y: number }) => void;
	readonly editable?: boolean;
	readonly peers?: readonly PresencePeer[];
}

export function SMediaGraphCanvas({
	graph,
	instances,
	parameters = [],
	projectionGeneration = 0,
	activeInstanceId: _activeInstanceId,
	onSelectInstance,
	onOpenInstance,
	onMoveNode,
	onConnectPorts,
	onRemoveInstance,
	onSpawnApp,
	onDisconnectEdge,
	editable = false,
	peers = [],
}: SMediaGraphCanvasProps): React.ReactElement {
	const sessionRef = useRef<FlowSession | null>(null);
	const containerRef = useRef<HTMLDivElement>(null);
	const cameraRef = useRef({ x: 0, y: 0, zoom: 1 });
	const graphRef = useRef(graph);
	const instancesRef = useRef(instances);
	const commandEpochRef = useRef(0);
	const flowStoreRef = useRef(createEphemeralFlowStore());
	const extensionHostRef = useRef<FlowExtensionHost | null>(null);
	if (!extensionHostRef.current) extensionHostRef.current = new FlowExtensionHost();
	const extensionHost = extensionHostRef.current;
	const extensionRevision = useOsMediaFlowExtension(extensionHost, graph, instances, parameters);
	const [commandRequest, setCommandRequest] = useState<FlowCanvasCommandRequest | undefined>();
	graphRef.current = graph;
	instancesRef.current = instances;

	const dispatchProxy = useCallback(
		(command: StudioCommand) => {
			if (command.kind === "moveMediaNode") onMoveNode?.(command.nodeId, command.x, command.y);
			if (command.kind === "connectMediaPorts") {
				onConnectPorts?.(command.sourceNodeId, command.sourcePortId, command.targetNodeId, command.targetPortId);
			}
			if (command.kind === "disconnectMediaEdge") onDisconnectEdge?.(command.edgeId);
			if (command.kind === "removeAppInstance") onRemoveInstance?.(command.instanceId);
		},
		[onConnectPorts, onDisconnectEdge, onMoveNode, onRemoveInstance],
	);

	const fixtureJson = useMemo(
		() => sMediaGraphToFlowFixtureJson(graph, instances, cameraRef.current, parameters),
		[graph, instances, parameters, projectionGeneration],
	);

	const handleFixtureChange = useCallback(
		(nextJson: string) => {
			const next = JSON.parse(nextJson) as { readonly camera?: { readonly x: number; readonly y: number; readonly zoom: number } };
			if (next.camera) cameraRef.current = next.camera;
			if (!editable) return;
			applyFlowFixtureJsonToSMediaGraph(graphRef.current, nextJson, dispatchProxy);
		},
		[dispatchProxy, editable],
	);

	const handleSelectionChange = useCallback(
		(nodeIds: readonly string[]) => {
			const nodeId = nodeIds[0];
			if (!nodeId) return;
			const node = graphRef.current.nodes.find((entry) => entry.id === nodeId);
			if (node) onSelectInstance?.(node.instanceId);
		},
		[onSelectInstance],
	);

	const handleWidgetDoubleClick = useCallback(
		(widgetId: string) => {
			const node = graphRef.current.nodes.find((entry) => entry.id === widgetId);
			if (node) onOpenInstance?.(node.instanceId);
		},
		[onOpenInstance],
	);

	const onCanvasCommand = useCallback((command: string, args?: Record<string, unknown>) => {
		commandEpochRef.current += 1;
		setCommandRequest({
			command,
			argsJson: JSON.stringify(args ?? {}),
			epoch: commandEpochRef.current,
		});
	}, []);

	const handleDrop = useCallback(
		(event: React.DragEvent<HTMLDivElement>) => {
			if (!editable || !onSpawnApp) return;
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
			const sx = event.clientX - rect.left;
			const sy = event.clientY - rect.top;
			const position = flowScreenToWorld(cameraRef.current, sx, sy, rect.width, rect.height);
			onSpawnApp(payload.programId, payload.appId, position);
		},
		[editable, onSpawnApp],
	);

	return (
		<div
			ref={containerRef}
			className="relative h-full w-full"
			onDragOver={(event) => {
				if (editable && onSpawnApp) event.preventDefault();
			}}
			onDrop={handleDrop}
		>
			{peers.length > 0 ? (
				<div className="pointer-events-none absolute right-2 top-2 z-10 flex flex-col gap-1">
					{peers.map((peer) => (
						<div key={peer.clientId} className="rounded bg-[var(--semio-surface-elevated)] px-2 py-1 text-[10px] shadow">
							{peer.name}
							{peer.selection?.length ? ` · ${peer.selection.length} selected` : ""}
						</div>
					))}
				</div>
			) : null}
			<FlowCanvas
				className="h-full w-full"
				fixtureJson={fixtureJson}
				store={flowStoreRef.current}
				extensionHost={extensionHost}
				extensionRevision={extensionRevision}
				onFixtureChange={editable ? handleFixtureChange : undefined}
				onSessionReady={(session) => {
					sessionRef.current = session;
				}}
				onSelectionChange={handleSelectionChange}
				onWidgetDoubleClick={onOpenInstance ? handleWidgetDoubleClick : undefined}
				enableSpotlight={false}
				contextMenu={editable ? (ctx) => buildSMediaFlowContextMenu(ctx, onCanvasCommand) : undefined}
				commandRequest={commandRequest}
				automaticLod
			/>
		</div>
	);
}
//#endregion 🔖MediaGraphCanvas
