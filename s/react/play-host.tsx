// #region 🧲Header
/** @emoji 🛝 S app renderer contribution — loaded only via `./play` subpath. */
// #endregion 🧲Header

import { createWriterDocument } from "@semio-tech/writer-core";
import { WriterCanvas } from "@semio-tech/writer-react";
import type { ReactElement } from "react";
import type { AppInstanceHostComponent, AppRendererContribution } from "@semio-tech/framework-platform-core";
import type { PlaygroundMountProps } from "@semio-tech/framework-platform-core";
import {
	applyAllOsSurfaceContributions,
	ensureOsAppContribution,
	OsInstanceHostBridgeProvider,
	useOsShellHistory,
	type OsInstanceHostBridge,
} from "@semio-tech/framework-os-renderer-react";
import {
	PlaygroundView,
	PlaygroundContext,
	PureSidePanelTabDefinition,
	CallbackTreePanelDefinition,
	Platform,
	CommandBus,
	FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
	FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
	FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
	FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
	FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
	FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
	uiTreeNodeToTreePanelConfig,
	controllerBackedExampleContribution,
} from "@semio-tech/framework-playground-renderer-react";
import { reactHostPort, createIconComponent } from "@semio-tech/ui-react";
import { type SidePanelTabConfig } from "@semio-tech/framework-playground-core";
import type { UiSHostSurfaceNode } from "@semio-tech/framework-platform-core";
import {
	S_HOME_APP_ID,
	S_PLAY_APP_ID,
	S_PLAY_CONTROLLER_ID,
	S_PLAY_EXAMPLE_OPTIONS,
	S_PLAY_SURFACE_MEDIA_GRAPH,
	S_PLAY_SURFACE_COMPILED_DAG,
	SPlayController,
	appInstanceResourceProjection,
	buildSPlayCatalogueTree,
	buildSPlayInspectorTree,
	buildSPlayParametersTree,
	sResourceDescriptor,
	type SAppInstance,
	type StudioCommand,
	sPlayWindowBodies,
} from "@semio-tech/s-core";
import { SMediaGraphCanvas, SStudioProvider } from "./index.tsx";

const sPlayControllerRef: { current: SPlayController | null } = { current: null };

function useSPlayController(runtimeOverride?: Platform): SPlayController | undefined {
	const appCtx = reactHostPort.useContext(PlaygroundContext);
	const runtime = runtimeOverride ?? appCtx?.runtime;
	reactHostPort.useSyncExternalStore(
		(listener) => (runtime ? runtime.subscribe(listener) : () => {}),
		() => runtime?.generation ?? 0,
		() => 0,
	);
	const activeApp = runtime?.getActiveApp();
	if (activeApp?.id === S_HOME_APP_ID) return undefined;
	const ctrl = activeApp?.controller as SPlayController | undefined;
	sPlayControllerRef.current = ctrl ?? null;
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
	const augmentPanelTabs = reactHostPort.useMemo(
		() =>
			ctrl
				? {
						details: [new SPlayInspectionPanelDefinition().resolveTab()],
						workbench: [new SPlayCataloguePanelDefinition().resolveTab(), new SPlayParametersPanelDefinition().resolveTab()],
					}
				: undefined,
		[ctrl],
	);
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
					<PlaygroundView runtime={runtime} defaultAppId={S_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} exampleContribution={exampleContribution} />
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
		const current = sPlayControllerRef.current;
		if (!current) return;
		const node = current.getStudioStore().projection().mediaGraph.nodes.find((row) => row.instanceId === instanceId);
		current.run("setMediaNodeSelection", { nodeIds: node ? [node.id] : [] });
		current.run("selectInstance", { instanceId });
	}, []);
	return (
		<SMediaGraphCanvas
			graph={projection.mediaGraph}
			instances={projection.appInstances}
			parameters={projection.parameters}
			projectionGeneration={generation}
			activeInstanceId={activeInstanceId}
			onSelectInstance={onSelect}
			onOpenInstance={(instanceId) => sPlayControllerRef.current?.run("openInstance", { instanceId })}
			editable
			onMoveNode={(nodeId, x, y) => store?.dispatch({ kind: "moveMediaNode", nodeId, x, y })}
			onConnectPorts={(sourceNodeId, sourcePortId, targetNodeId, targetPortId) =>
				store?.dispatch({ kind: "connectMediaPorts", sourceNodeId, sourcePortId, targetNodeId, targetPortId })
			}
			onDisconnectEdge={(edgeId) => store?.dispatch({ kind: "disconnectMediaEdge", edgeId })}
			onRemoveInstance={(instanceId) => store?.dispatch({ kind: "removeAppInstance", instanceId })}
			onSpawnApp={(programId, appId, position) => sPlayControllerRef.current?.run("spawnApp", { programId, appId, position })}
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

class SPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
	buildTab(): SidePanelTabConfig {
		return {
			id: "s-play-catalogue",
			icon: createIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID),
			name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
			order: 1,
			tree: new CallbackTreePanelDefinition(() => uiTreeNodeToTreePanelConfig(buildSPlayCatalogueTree(), new CommandBus())),
		};
	}
}

class SPlayParametersPanelDefinition extends PureSidePanelTabDefinition {
	buildTab(): SidePanelTabConfig {
		return {
			id: "s-play-parameters",
			icon: createIconComponent(FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID),
			name: FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
			order: 2,
			tree: new CallbackTreePanelDefinition(() => {
				const ctrl = sPlayControllerRef.current;
				const bus = new CommandBus();
				if (!ctrl) {
					return uiTreeNodeToTreePanelConfig(
						{ type: "tree", id: "s-play-parameters.loading", label: FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL, sections: [] },
						bus,
					);
				}
				return uiTreeNodeToTreePanelConfig(buildSPlayParametersTree(ctrl), bus);
			}),
		};
	}
}

class SPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
	buildTab(): SidePanelTabConfig {
		return {
			id: "s-play-inspector",
			icon: createIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID),
			name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
			order: 0,
			tree: new CallbackTreePanelDefinition(() => {
				const ctrl = sPlayControllerRef.current;
				const bus = new CommandBus();
				if (!ctrl) {
					return uiTreeNodeToTreePanelConfig(
						{ type: "tree", id: "s-play-inspector.loading", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, sections: [] },
						bus,
					);
				}
				return uiTreeNodeToTreePanelConfig(buildSPlayInspectorTree(ctrl), bus);
			}),
		};
	}
}

function mountSPlayChrome({ runtime }: PlaygroundMountProps): ReactElement {
	return <SPlayInner runtime={runtime} />;
}

/** @emoji 🛝 S app renderer for playground and OS shells. */
export const sAppRenderer: AppRendererContribution = {
	windowBodies: sPlayWindowBodies,
	surfaceHosts: {
		[S_PLAY_SURFACE_MEDIA_GRAPH]: SSSurfaceHost,
		[S_PLAY_SURFACE_COMPILED_DAG]: SPlayCompiledDagSurfaceHost,
	},
	panelTabs: {
		workbench: [new SPlayCataloguePanelDefinition(), new SPlayParametersPanelDefinition()],
		details: [new SPlayInspectionPanelDefinition()],
	},
	tabIcons: {
		[FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID]: "sliders-horizontal",
	},
	preload: applyAllOsSurfaceContributions,
	mountChrome: mountSPlayChrome,
	examples: controllerBackedExampleContribution(S_PLAY_CONTROLLER_ID, S_PLAY_EXAMPLE_OPTIONS()),
};
