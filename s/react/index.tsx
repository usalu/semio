// #region 🧲Header
/** @emoji 🖥️ `@semio-tech/s-react` — studio provider and media graph canvas. */
// #endregion 🧲Header

import React, { createContext, useCallback, useContext, useEffect, useMemo, useRef, useState, useSyncExternalStore, type ReactElement } from "react";
import { createWriterDocument } from "@semio-tech/writer-core";
import { WriterCanvas } from "@semio-tech/writer-react";
import type { AppInstanceHostComponent, AppRendererContribution, PlaygroundMountProps, UiSHostSurfaceNode } from "@semio-tech/framework-platform-core";
import {
	ensureOsAppContribution,
	OsInstanceHostBridgeProvider,
	useOsShellHistory,
	type OsInstanceHostBridge,
} from "@semio-tech/framework-os-renderer-react";
import {
	PlaygroundView,
	PlaygroundContext,
	usePlayController,
	controllerBackedExampleContribution,
} from "@semio-tech/framework-playground-renderer-react";
import { reactHostPort, CATALOGUE_DRAG_MIME } from "@semio-tech/ui-react";
import { Platform } from "@semio-tech/framework-playground-core";
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
	S_HOME_APP_ID,
	S_PLAY_APP_ID,
	S_PLAY_CONTROLLER_ID,
	S_PLAY_EXAMPLE_OPTIONS,
	S_PLAY_SURFACE_MEDIA_GRAPH,
	S_PLAY_SURFACE_COMPILED_DAG,
	S_PLAY_PARAMETERS_TAB_ID,
	SPlayController,
	appInstanceResourceProjection,
	sResourceDescriptor,
	type SAppInstance,
	type SMediaGraph,
	type SParameter,
	type StudioCommand,
	type StudioStore,
	applyFlowFixtureJsonToSMediaGraph,
	buildSMediaFlowOperatorInfos,
	sMediaGraphToFlowFixtureJson,
	sPlaySidePanelBodies,
	sPlayWindowBodies,
} from "@semio-tech/s-core";
import type { PresencePeer } from "@semio-tech/framework-os-core";

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

//#region 🔖PlayHost
function useSPlayController(runtimeOverride?: Platform): SPlayController | undefined {
	const appCtx = reactHostPort.useContext(PlaygroundContext);
	const runtime = runtimeOverride ?? appCtx?.runtime;
	const ctrl = usePlayController<SPlayController>(runtime);
	if (runtime?.getActiveApp()?.id === S_HOME_APP_ID) return undefined;
	return ctrl;
}
function SAppHostRouter({ instance }: { readonly instance: SAppInstance | null }): ReactElement {
	const [InstanceHost, setInstanceHost] = reactHostPort.useState<AppInstanceHostComponent | null>(null);
	const [loading, setLoading] = reactHostPort.useState(false);
	const [fallbackLabel, setFallbackLabel] = reactHostPort.useState<string | null>(null);

	reactHostPort.useEffect(() => {
		if (!instance) {
			setInstanceHost(null);
			setFallbackLabel(null);
			return;
		}
		let active = true;
		setLoading(true);
		void ensureOsAppContribution(instance.programId).then((contribution) => {
			if (!active) return;
			if (contribution?.instanceHost) {
				setInstanceHost(() => contribution.instanceHost!);
				setFallbackLabel(null);
			} else {
				setInstanceHost(null);
				const resource = sResourceDescriptor(instance.yields);
				setFallbackLabel(resource ? `${resource.name} (${resource.componentKind})` : instance.programId);
			}
			setLoading(false);
		});
		return () => {
			active = false;
		};
	}, [instance]);

	if (!instance) {
		return <div className="flex h-full items-center justify-center p-6 text-sm text-muted-foreground">No active app</div>;
	}
	if (loading || !InstanceHost) {
		return (
			<div className="flex h-full items-center justify-center p-6 text-sm text-muted-foreground">
				{loading ? "Loading app…" : (fallbackLabel ?? "Unsupported app")}
			</div>
		);
	}
	return (
		<reactHostPort.Suspense fallback={<div className="flex h-full items-center justify-center p-6 text-sm text-muted-foreground">Loading app…</div>}>
			<InstanceHost instance={instance} />
		</reactHostPort.Suspense>
	);
}

function SPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
	useOsShellHistory(runtime);
	const activeAppId = runtime.getActiveApp()?.id ?? runtime.activeAppId;
	const hasHomeApp = runtime.apps.some((app) => app.id === S_HOME_APP_ID);
	const ctrl = useSPlayController(runtime);
	const focusedInstanceId = ctrl?.getFocusedInstanceId() ?? null;
	const studioGeneration = ctrl?.getStudioStore().getGeneration() ?? 0;
	const focusedInstance = reactHostPort.useMemo(() => {
		if (!ctrl || !focusedInstanceId) return null;
		return ctrl.getStudioStore().projection().appInstances.find((entry) => entry.id === focusedInstanceId) ?? null;
	}, [ctrl, focusedInstanceId, studioGeneration]);
	const instanceHostBridge = reactHostPort.useMemo<OsInstanceHostBridge | null>(() => {
		if (!ctrl) return null;
		const store = ctrl.getStudioStore();
		return {
			subscribe: (listener) => store.subscribe(listener),
			getGeneration: () => store.getGeneration(),
			getInstances: () => store.projection().appInstances,
			projectInstance: (instanceId) => appInstanceResourceProjection(store.projection().mediaGraph, store.projection().appInstances, instanceId),
			dispatch: (command) => store.dispatch(command as StudioCommand),
		};
	}, [ctrl, studioGeneration]);
	const exampleContribution = reactHostPort.useMemo(
		() => controllerBackedExampleContribution(S_PLAY_CONTROLLER_ID, S_PLAY_EXAMPLE_OPTIONS()),
		[],
	);
	if (activeAppId === S_HOME_APP_ID) {
		return <PlaygroundView runtime={runtime} defaultAppId={S_HOME_APP_ID} />;
	}
	if (!ctrl) return <PlaygroundView runtime={runtime} defaultAppId={S_PLAY_APP_ID} exampleContribution={exampleContribution} />;
	if (focusedInstance && instanceHostBridge) {
		return (
			<SStudioProvider store={ctrl.getStudioStore()}>
				<OsInstanceHostBridgeProvider bridge={instanceHostBridge}>
					<div className="flex h-full min-h-0 flex-col overflow-hidden bg-background">
						<div className="flex items-center gap-2 border-b border-border/60 px-3 py-2 text-sm text-muted-foreground">
							<button
								type="button"
								className="hover:text-foreground"
								onClick={() => ctrl.run("goHome")}
							>
								← Home
							</button>
							<span>·</span>
							<button
								type="button"
								className="hover:text-foreground"
								onClick={() => ctrl.run("closeFocusedInstance")}
							>
								← Back to Media Graph · {focusedInstance.label}
							</button>
						</div>
						<div className="min-h-0 flex-1">
							<SAppHostRouter instance={focusedInstance} />
						</div>
					</div>
				</OsInstanceHostBridgeProvider>
			</SStudioProvider>
		);
	}
	return (
		<SStudioProvider store={ctrl.getStudioStore()}>
			<div className="flex h-full min-h-0 flex-col overflow-hidden">
				{hasHomeApp ? (
					<button
						type="button"
						className="border-b border-border/60 px-3 py-2 text-left text-sm text-muted-foreground hover:bg-muted/40 hover:text-foreground"
						onClick={() => ctrl.run("goHome")}
					>
						← Home
					</button>
				) : null}
				<div className="min-h-0 flex-1">
					<PlaygroundView runtime={runtime} defaultAppId={S_PLAY_APP_ID} exampleContribution={exampleContribution} />
				</div>
			</div>
		</SStudioProvider>
	);
}

function SMediaGraphSurfaceHost({ node: _node }: { readonly node: UiSHostSurfaceNode }): ReactElement {
	const ctrl = useSPlayController();
	const generation = ctrl?.getStudioStore().getGeneration() ?? 0;
	void generation;
	const projection = ctrl?.getStudioStore().projection() ?? {
		activeProgramId: null,
		activeAlternativeId: null,
		appInstances: [],
		mediaGraph: { schema: "s.media-graph", nodes: [], edges: [] },
		parameters: [],
		parameterBindings: [],
	};
	const activeInstanceId = ctrl?.getActiveInstanceId() ?? null;
	const store = ctrl?.getStudioStore();
	const onSelect = reactHostPort.useCallback((instanceId: string) => {
		if (!ctrl) return;
		const node = ctrl.getStudioStore().projection().mediaGraph.nodes.find((row) => row.instanceId === instanceId);
		ctrl.run("setMediaNodeSelection", { nodeIds: node ? [node.id] : [] });
		ctrl.run("selectInstance", { instanceId });
	}, [ctrl]);
	return (
		<SMediaGraphCanvas
			graph={projection.mediaGraph}
			instances={projection.appInstances}
			parameters={projection.parameters}
			projectionGeneration={generation}
			activeInstanceId={activeInstanceId}
			onSelectInstance={onSelect}
			onOpenInstance={(instanceId) => ctrl?.run("openInstance", { instanceId })}
			editable
			onMoveNode={(nodeId, x, y) => store?.dispatch({ kind: "moveMediaNode", nodeId, x, y })}
			onConnectPorts={(sourceNodeId, sourcePortId, targetNodeId, targetPortId) =>
				store?.dispatch({ kind: "connectMediaPorts", sourceNodeId, sourcePortId, targetNodeId, targetPortId })
			}
			onDisconnectEdge={(edgeId) => store?.dispatch({ kind: "disconnectMediaEdge", edgeId })}
			onRemoveInstance={(instanceId) => store?.dispatch({ kind: "removeAppInstance", instanceId })}
			onSpawnApp={(programId, appId, position) => ctrl?.run("spawnApp", { programId, appId, position })}
			peers={store?.getPresencePeers() ?? []}
		/>
	);
}

function SSSurfaceHost({ node }: { readonly node: UiSHostSurfaceNode }): ReactElement {
	return <SMediaGraphSurfaceHost node={node} />;
}

function SPlayCompiledDagSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
	const ctrl = useSPlayController();
	const [revision, setRevision] = reactHostPort.useState(0);
	reactHostPort.useEffect(() => ctrl?.subscribeSnapshot(() => setRevision((value) => value + 1)) ?? undefined, [ctrl]);
	const document = reactHostPort.useMemo(
		() => ctrl?.getWriterDocumentCompiledDag() ?? createWriterDocument({ id: "s-compiled-dag", languageId: "wire", text: "" }),
		[ctrl, revision],
	);
	return <WriterCanvas document={document} className="h-full min-h-0" />;
}

function mountSPlayChrome({ runtime }: PlaygroundMountProps): ReactElement {
	return <SPlayInner runtime={runtime} />;
}

/** @emoji 🛝 S app renderer for playground and OS shells. */
export const sAppRenderer: AppRendererContribution = {
	windowBodies: sPlayWindowBodies,
	sidePanelBodies: sPlaySidePanelBodies,
	surfaceHosts: {
		[S_PLAY_SURFACE_MEDIA_GRAPH]: SSSurfaceHost,
		[S_PLAY_SURFACE_COMPILED_DAG]: SPlayCompiledDagSurfaceHost,
	},
	tabIcons: {
		[S_PLAY_PARAMETERS_TAB_ID]: "sliders-horizontal",
	},
	mountChrome: mountSPlayChrome,
	examples: controllerBackedExampleContribution(S_PLAY_CONTROLLER_ID, S_PLAY_EXAMPLE_OPTIONS()),
};
//#endregion 🔖PlayHost
