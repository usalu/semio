// #region 🧲Header
/** @emoji 🛝 Playground play host for S — loaded only via `./play` subpath. */
// #endregion 🧲Header

import { createWriterDocument } from "@semio-tech/writer-core";
import { WriterCanvas } from "@semio-tech/writer-react";
import type { ReactElement } from "react";
import { type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, PlaygroundView, PlaygroundContext, PureSidePanelTabDefinition, CallbackTreePanelDefinition, registerUiWriterSurfaceHost, registerUiSSurfaceHost, registerTabIcon, Platform, CommandBus, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID, FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL, uiTreeNodeToTreePanelConfig } from "@semio-tech/framework-playground-renderer-react";
import { reactHostPort, createIconComponent } from "@semio-tech/ui-react";
import { type SidePanelTabConfig, UiTreeNode } from "@semio-tech/framework-playground-core";
import type { UiSHostSurfaceNode } from "@semio-tech/framework-platform-core";
import {
	S_PLAY_APP_ID,
	S_PLAY_CONTROLLER_ID,
	S_PLAY_SURFACE_MEDIA_GRAPH,
	S_PLAY_SURFACE_COMPILED_DAG,
	SPlayController,
	appInstanceResourceProjection,
	buildSPlayCatalogueTree,
	buildSPlayInspectorTree,
	buildSPlayParametersTree,
	registerSPlayDeclarativeBodies,
	sResourceDescriptor,
	type SAppInstance,
} from "@semio-tech/s-core";

import { defaultDrawDocument, drawDocumentToJson, type DrawDocument } from "@semio-tech/draw-core";
import { defaultRasterDocument, type RasterDocument } from "@semio-tech/raster-core";
import { PresentationDeck } from "@semio-tech/framework-presentation-renderer-react";
import type { PresentationDeck as PresentationDeckDocument } from "@semio-tech/framework-presentation-core";

let sPlayChromeRegistered = false;
const sPlayControllerRef: { current: SPlayController | null } = { current: null };

const EMPTY_PUZZLE3D_FIXTURE = {
	schema: "puzzle.3d.fixture",
	camera: { position: [4, 4, 4], target: [0, 0, 0], zoom: 1 },
	objects: [],
	attractions: [],
	references: [],
	targetVolumes: [],
} as const;

function SPuzzle3dHost({ fixtureJson }: { readonly fixtureJson: string }): ReactElement {
	const fixture = reactHostPort.useMemo(() => parseFixture(JSON.parse(fixtureJson)) ?? EMPTY_PUZZLE3D_FIXTURE, [fixtureJson]);
	const [selectedId, setSelectedId] = reactHostPort.useState<string | null>(null);
	return (
		<ObjectStateProvider fixture={fixture}>
			<PlayCanvas fixture={fixture} setSelectedId={setSelectedId} selectedId={selectedId} className="h-full" />
		</ObjectStateProvider>
	);
}

function SPresentationDeckHost({ deck }: { readonly deck: PresentationDeckDocument }): ReactElement {
	return <PresentationDeck presentation={deck as never} />;
}

const SCadPlayRoot = reactHostPort.lazy(() =>
	import("@semio-tech/cad-js-renderer-react").then((module) => ({ default: module.CadPlayRoot })),
);

function SUpstreamBadge({
	upstreamInstanceId,
	instances,
}: {
	readonly upstreamInstanceId: string | null;
	readonly instances: readonly SAppInstance[];
}): ReactElement | null {
	if (!upstreamInstanceId) return null;
	const upstream = instances.find((entry) => entry.id === upstreamInstanceId);
	if (!upstream) return null;
	return (
		<div className="border-b border-border/60 bg-muted/40 px-3 py-1 text-xs text-muted-foreground">
			Upstream · {upstream.label} ({upstream.yields})
		</div>
	);
}

function SSketchpadHost({ appId }: { readonly appId: string }): ReactElement {
	const [platform, setPlatform] = reactHostPort.useState<Platform | null>(null);
	reactHostPort.useEffect(() => {
		let active = true;
		void import("@semio-tech/compose-sketchpad").then(({ ensureSketchpadPlatform }) =>
			ensureSketchpadPlatform().then((runtime) => {
				if (!active) return;
				runtime.activeAppId = appId;
				runtime.notify();
				setPlatform(runtime);
			}),
		);
		return () => {
			active = false;
		};
	}, [appId]);
	if (!platform) {
		return <div className="flex h-full items-center justify-center p-6 text-sm text-muted-foreground">Loading sketchpad…</div>;
	}
	return <PlaygroundView runtime={platform} defaultAppId={appId} />;
}

function SAppHostRouter({ instance }: { readonly instance: SAppInstance | null }): ReactElement {
	const ctrl = useSPlayController();
	const store = ctrl?.getStudioStore();
	const generation = store?.getGeneration() ?? 0;
	const projection = store?.projection();
	const resourceBundle = reactHostPort.useMemo(() => {
		if (!instance || !store) return null;
		const current = store.projection();
		return appInstanceResourceProjection(current.mediaGraph, current.appInstances, instance.id);
	}, [instance, store, generation]);
	const materialized = resourceBundle?.projection;
	const upstreamInstanceId = resourceBundle?.upstreamInstanceId ?? null;
	const resource = instance ? sResourceDescriptor(instance.yields) : null;
	const dispatchDraw = reactHostPort.useCallback(
		(document: DrawDocument) => {
			if (!instance || !store) return;
			store.dispatch({
				kind: "patchAppSource",
				instanceId: instance.id,
				inline: drawDocumentToJson(document),
			});
		},
		[instance, store],
	);
	const drawDoc = reactHostPort.useMemo(() => {
		if (instance?.sourceDocument.payloadRef === "fixture:semio.draw.json") return defaultDrawDocument("semio", "Semio Emblem");
		if (materialized && typeof materialized === "object" && (materialized as DrawDocument).schema === "draw.document") return materialized as DrawDocument;
		return defaultDrawDocument(instance?.id ?? "draw");
	}, [instance, materialized]);
	const rasterDoc = reactHostPort.useMemo(() => {
		if (materialized && typeof materialized === "object" && (materialized as RasterDocument).schema === "raster.document") {
			return materialized as RasterDocument;
		}
		return defaultRasterDocument(instance?.id ?? "raster");
	}, [instance, materialized]);
	const formsSpec = reactHostPort.useMemo(() => {
		if (materialized && typeof materialized === "object" && (materialized as FormSpec).schema === "forms.form") {
			return materialized as FormSpec;
		}
		if (instance?.sourceDocument.inline) {
			try {
				return parseFormSpec(JSON.parse(instance.sourceDocument.inline));
			} catch {
				return defaultFormSpec(instance?.id ?? "forms");
			}
		}
		return defaultFormSpec(instance?.id ?? "forms");
	}, [instance, materialized]);
	const dispatchRaster = reactHostPort.useCallback(
		(document: RasterDocument) => {
			if (!instance || !store) return;
			store.dispatch({
				kind: "applyAppOperation",
				instanceId: instance.id,
				forwards: [{ op: "replaceProjection", projection: document }],
				backwards: [{ op: "replaceProjection", projection: rasterDoc }],
			});
		},
		[instance, store, rasterDoc],
	);
	const dispatchForms = reactHostPort.useCallback(
		(spec: FormSpec) => {
			if (!instance || !store) return;
			store.dispatch({
				kind: "applyAppOperation",
				instanceId: instance.id,
				forwards: [{ op: "replaceProjection", projection: spec }],
				backwards: [{ op: "replaceProjection", projection: formsSpec }],
			});
		},
		[instance, store, formsSpec],
	);
	const writerDoc = reactHostPort.useMemo(() => {
		const doc = materialized as { text?: string } | null;
		return createWriterPlayDocument({ id: instance?.id ?? "writer", languageId: "jack", text: doc?.text ?? instance?.sourceDocument.inline ?? "" });
	}, [instance, materialized]);
	const fixtureJson = reactHostPort.useMemo(() => JSON.stringify(materialized ?? {}), [materialized]);
	const hostChrome = <SUpstreamBadge upstreamInstanceId={upstreamInstanceId} instances={projection?.appInstances ?? []} />;
	if (!instance || !resource) {
		return <div className="flex h-full items-center justify-center p-6 text-sm text-muted-foreground">No active app</div>;
	}
	if (instance.programId === "compose.sketchpad") {
		return (
			<div className="flex h-full min-h-0 flex-col overflow-hidden">
				{hostChrome}
				<div className="min-h-0 flex-1">
					<SSketchpadHost appId={instance.appId} />
				</div>
			</div>
		);
	}
	switch (resource.componentKind) {
		case "draw":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<DrawCanvas document={drawDoc} onCommit={(document) => dispatchDraw(document)} className="min-h-0 flex-1" />
				</div>
			);
		case "writer":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<WriterPlayCanvas
						document={writerDoc}
						onChange={(document) => {
							if (!store) return;
							store.dispatch({
								kind: "patchAppSource",
								instanceId: instance.id,
								inline: JSON.stringify(document),
							});
						}}
						createLspTransport={() => ({ dispose() {} } as never)}
						className="min-h-0 flex-1"
					/>
				</div>
			);
		case "raster":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<RasterCanvas
						document={rasterDoc}
						selectedIds={[]}
						hoveredId={null}
						kindHover={null}
						activeTool={rasterDoc.activeTool}
						camera={rasterDoc.camera}
						onSelect={() => {}}
						onHover={() => {}}
						onCameraChange={(camera) => dispatchRaster({ ...rasterDoc, camera })}
						className="min-h-0 flex-1"
						viewMode="composite"
					/>
				</div>
			);
		case "forms":
			return (
				<div className="flex h-full min-h-0 flex-col overflow-hidden">
					{hostChrome}
					<FormEditSurface spec={formsSpec} onChange={(spec) => dispatchForms(spec)} className="min-h-0 flex-1 overflow-auto p-4" />
				</div>
			);
		case "cad":
			return (
				<div className="relative h-full min-h-0 overflow-hidden">
					{hostChrome}
					<reactHostPort.Suspense fallback={<div className="p-6 text-sm text-muted-foreground">Loading CAD…</div>}>
						<SCadPlayRoot />
					</reactHostPort.Suspense>
				</div>
			);
		case "flow":
			return <FlowCanvas fixtureJson={fixtureJson} className="h-full min-h-0" />;
		case "dag":
			return <DagCanvas fixtureJson={fixtureJson} className="h-full min-h-0" reorganize />;
		case "imperative":
			return <ImperativeEditor documentJson={fixtureJson} className="h-full min-h-0" />;
		case "sequence":
			return <SequenceCanvas fixtureJson={fixtureJson} className="h-full min-h-0" />;
		case "trinity":
			return <TrinityCanvas fixtureJson={fixtureJson} className="h-full min-h-0" reorganize />;
		case "gismap":
			return (
				<div className="relative h-full min-h-0">
					<MapCanvas className="h-full" />
				</div>
			);
		case "puzzle2d":
			return (
				<div className="relative h-full min-h-0">
					<Puzzle2dCanvas className="h-full" />
				</div>
			);
		case "puzzle3d":
		case "puzzle5d":
			return (
				<div className="relative h-full min-h-0">
					{hostChrome}
					<SPuzzle3dHost fixtureJson={fixtureJson} />
				</div>
			);
		case "shooting":
			return (
				<div className="relative h-full min-h-0">
					<ShootingModelCanvas fixture={JSON.parse(fixtureJson) as never} className="h-full" />
				</div>
			);
		case "panel":
			if (materialized && typeof materialized === "object" && (materialized as PresentationDeckDocument).schema === "presentation.deck") {
				return (
					<div className="flex h-full min-h-0 flex-col overflow-hidden">
						{hostChrome}
						<SPresentationDeckHost deck={materialized as PresentationDeckDocument} />
					</div>
				);
			}
			return (
				<div className="h-full overflow-auto p-4 text-xs text-muted-foreground">
					<div className="mb-2 font-medium text-foreground">
						{resource.name} ({resource.componentKind})
					</div>
					<pre className="whitespace-pre-wrap">{fixtureJson}</pre>
				</div>
			);
		case "virtualFileSystem":
		case "s":
			return (
				<div className="h-full overflow-auto p-4 text-xs text-muted-foreground">
					<div className="mb-2 font-medium text-foreground">
						{resource.name} ({resource.componentKind})
					</div>
					<pre className="whitespace-pre-wrap">{fixtureJson}</pre>
				</div>
			);
		default:
			return (
				<div className="flex h-full items-center justify-center p-6 text-sm text-muted-foreground">
					{resource.name} ({resource.componentKind})
				</div>
			);
	}
}

function useSPlayController(runtimeOverride?: Platform): SPlayController | undefined {
	const appCtx = reactHostPort.useContext(PlaygroundContext);
	const runtime = runtimeOverride ?? appCtx?.runtime;
	reactHostPort.useSyncExternalStore(
		(listener) => (runtime ? runtime.subscribe(listener) : () => {}),
		() => runtime?.generation ?? 0,
		() => 0,
	);
	const ctrl = runtime?.getActiveApp()?.controller as SPlayController | undefined;
	sPlayControllerRef.current = ctrl ?? null;
	return ctrl;
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

function SPlayInner({ playground }: { readonly playground: Playground }): ReactElement {
	const ctrl = useSPlayController(playground.runtime);
	const bus = playground.runtime.commandBus;
	const focusedInstanceId = ctrl?.getFocusedInstanceId() ?? null;
	const studioGeneration = ctrl?.getStudioStore().getGeneration() ?? 0;
	const focusedInstance = reactHostPort.useMemo(() => {
		if (!ctrl || !focusedInstanceId) return null;
		return ctrl.getStudioStore().projection().appInstances.find((entry) => entry.id === focusedInstanceId) ?? null;
	}, [ctrl, focusedInstanceId, studioGeneration]);
	const detailTabs = reactHostPort.useMemo(
		() =>
			ctrl
				? [
						new SPlayInspectionPanelDefinition(() => buildSPlayInspectorTree(ctrl), bus).resolveTab(),
					]
				: [],
		[ctrl, bus],
	);
	const catalogueTabs = reactHostPort.useMemo(
		() => (ctrl ? [new SPlayCataloguePanelDefinition(() => buildSPlayCatalogueTree(), bus).resolveTab()] : []),
		[ctrl, bus],
	);
	const parameterTabs = reactHostPort.useMemo(
		() => (ctrl ? [new SPlayParametersPanelDefinition(() => buildSPlayParametersTree(ctrl), bus).resolveTab()] : []),
		[ctrl, bus],
	);
	const augmentPanelTabs = reactHostPort.useMemo(
		() => ({ details: detailTabs, workbench: [...catalogueTabs, ...parameterTabs] }),
		[detailTabs, catalogueTabs, parameterTabs],
	);
	if (!ctrl) return <PlaygroundView runtime={playground.runtime} defaultAppId={S_PLAY_APP_ID} />;
	if (focusedInstance) {
		return (
			<SStudioProvider store={ctrl.getStudioStore()}>
				<div className="flex h-full min-h-0 flex-col overflow-hidden bg-background">
					<button
						type="button"
						className="border-b border-border/60 px-3 py-2 text-left text-sm text-muted-foreground hover:bg-muted/40 hover:text-foreground"
						onClick={() => ctrl.run("closeFocusedInstance")}
					>
						← Back to Media Graph · {focusedInstance.label}
					</button>
					<div className="min-h-0 flex-1">
						<SAppHostRouter instance={focusedInstance} />
					</div>
				</div>
			</SStudioProvider>
		);
	}
	return (
		<SStudioProvider store={ctrl.getStudioStore()}>
			<PlaygroundView runtime={playground.runtime} defaultAppId={S_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} />
		</SStudioProvider>
	);
}

class SPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
	constructor(
		private readonly buildTree: () => UiTreeNode,
		private readonly commandBus: CommandBus,
	) {
		super();
	}

	buildTab(): SidePanelTabConfig {
		return {
			id: "s-play-catalogue",
			icon: createIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID),
			name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
			order: 1,
			tree: new CallbackTreePanelDefinition(() => uiTreeNodeToTreePanelConfig(this.buildTree(), this.commandBus)),
		};
	}
}

class SPlayParametersPanelDefinition extends PureSidePanelTabDefinition {
	constructor(
		private readonly buildTree: () => UiTreeNode,
		private readonly commandBus: CommandBus,
	) {
		super();
	}

	buildTab(): SidePanelTabConfig {
		return {
			id: "s-play-parameters",
			icon: createIconComponent(FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID),
			name: FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL,
			order: 2,
			tree: new CallbackTreePanelDefinition(() => uiTreeNodeToTreePanelConfig(this.buildTree(), this.commandBus)),
		};
	}
}

class SPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
	constructor(
		private readonly buildTree: () => UiTreeNode,
		private readonly commandBus: CommandBus,
	) {
		super();
	}

	buildTab(): SidePanelTabConfig {
		return {
			id: "s-play-inspector",
			icon: createIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID),
			name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
			order: 0,
			tree: new CallbackTreePanelDefinition(() => uiTreeNodeToTreePanelConfig(this.buildTree(), this.commandBus)),
		};
	}
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

export function registerSPlaySurfaceHosts(): void {
	if (sPlayChromeRegistered) return;
	sPlayChromeRegistered = true;
	registerTabIcon(FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID, "sliders-horizontal");
	registerUiSSurfaceHost(S_PLAY_SURFACE_MEDIA_GRAPH, SSSurfaceHost);
	registerUiWriterSurfaceHost(S_PLAY_SURFACE_COMPILED_DAG, SPlayCompiledDagSurfaceHost);
	registerSPlayDeclarativeBodies();
	registerDrawPlaySurfaceHosts();
	registerWriterPlaySurfaceHosts();
	registerRasterPlaySurfaceHosts();
	registerFlowPlaySurfaceHosts();
	registerDagPlaySurfaceHosts();
	registerMapPlaySurfaceHosts();
	registerPuzzle2dPlaySurfaceHosts();
	registerPuzzle3dPlaySurfaceHosts();
	registerPuzzle5dPlaySurfaceHosts();
	registerTrinityJackPlaySurfaceHosts();
	registerTrinityRewritePlaySurfaceHosts();
	registerProceduralPlaySurfaceHosts();
	registerProcedural2dPlaySurfaceHosts();
	registerShootingPlaySurfaceHosts();
	registerFormsPlaySurfaceHosts();
	registerPresentationPlaySurfaceHosts();
	void import("@semio-tech/cad-js-renderer-react").then((module) => module.registerCadPlaySurfaceHosts());
}

function SPlayChrome({ playground }: { readonly playground: Playground }): ReactElement {
	return <SPlayInner playground={playground} />;
}

export function mountSPlayChrome(playground: Playground, rootId = "root"): void {
	mountPlaygroundApp(<SPlayChrome playground={playground} />, rootId);
}

const sPlayChromeBoot: PlaygroundChromeBoot = {
	registerHosts: registerSPlaySurfaceHosts,
	mount: mountSPlayChrome,
};

export function bootSPlay(playground: Playground, rootId = "root"): void {
	bootPlayground(playground, sPlayChromeBoot, rootId);
}
//#endregion 🔖SPlayHost