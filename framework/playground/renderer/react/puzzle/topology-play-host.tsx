// #region 🧲Header
/** @emoji 🛝 Puzzle play React chrome in `@framework/playground-renderer-react` (not in play packages). */
// #endregion 🧲Header



import React from "react";
import { useGLTF } from "@react-three/drei";
import { ClipboardList, ListTree } from "lucide-react";
import { LevelProvider, getLevelBgClass } from "@ui/react";
import { ProductRuntime } from "@framework/playground";
import {
	PlaygroundView,
	PureSidePanelTabDefinition,
	StaticTreePanelDefinition,
	mountPlaygroundApp,
	registerUiBoardSurfaceHost,
	registerUiScene3DSurfaceHost,
	registerWindowBody,
	useApp,
	type SidePanelTabConfig,
	type TreeDataSection,
	type UiBoardHostSurfaceNode,
	type UiScene3DHostSurfaceNode,
} from "@framework/playground-renderer-react";
import {
	buildTopologyDualSurfaceBindings,
	topologyMirrorConnectHandlers,
	topologyMirrorProximityHandlers,
	TopologyBoardPane,
	TopologyScenePane,
	topologySceneChromeDefaults,
} from "@puzzle/5d-react";
import type { Playground } from "@framework/playground";
import {
	TOPOLOGY_PLAY_APP_ID,
	TOPOLOGY_PLAY_BOARD_BODY_KEY,
	TOPOLOGY_PLAY_BOARD_SURFACE_ID,
	TOPOLOGY_PLAY_BOARD_WINDOW_ID,
	TOPOLOGY_PLAY_CONTROLLER_ID,
	TOPOLOGY_PLAY_SCENE_BODY_KEY,
	TOPOLOGY_PLAY_SCENE_SURFACE_ID,
	TOPOLOGY_PLAY_HIERARCHY_TAB_ID,
	TopologyPlayShellController,
	buildTopologyBoardDeclarativeBody,
	buildTopologyPlayHierarchySections,
	buildTopologyPlayRuntime,
	buildTopologySceneDeclarativeBody,
	type TopologyPlaySnapshot,
} from "../../../../../puzzle/5d/play/index.ts";


//#region 🔖Snapshot
function useTopologyPlaySnapshot(): { readonly controller: TopologyPlayShellController | undefined; readonly snapshot: TopologyPlaySnapshot | null } {
	const { runtime } = useApp();
	React.useSyncExternalStore(
		(listener) => runtime.subscribe(listener),
		() => runtime.generation,
		() => 0,
	);
	const controller = runtime.getActiveApp()?.controller as TopologyPlayShellController | undefined;
	return { controller, snapshot: controller?.getSnapshot() ?? null };
}
//#endregion 🔖Snapshot

//#region 🔖DetailsPanel
function TopologyPlayStatusPanel(): React.ReactElement {
	const { snapshot } = useTopologyPlaySnapshot();
	if (!snapshot) {
		return <p className="text-muted-foreground p-2 text-xs">No topology snapshot</p>;
	}
	return (
		<dl className="grid gap-2 p-2 text-xs">
			<div>
				<dt className="text-muted-foreground font-medium">Manifest</dt>
				<dd>{snapshot.manifestLabel ?? "—"}</dd>
			</div>
			<div>
				<dt className="text-muted-foreground font-medium">Board selection</dt>
				<dd>{snapshot.boardSelected.size} id(s)</dd>
			</div>
			<div>
				<dt className="text-muted-foreground font-medium">Scene selection</dt>
				<dd>{snapshot.sceneSelected ?? "—"}</dd>
			</div>
			<div>
				<dt className="text-muted-foreground font-medium">Relocate</dt>
				<dd>{snapshot.relocateMode}</dd>
			</div>
			<div>
				<dt className="text-muted-foreground font-medium">Connect events</dt>
				<dd>
					board {snapshot.connectBoard} · scene {snapshot.connectScene}
				</dd>
			</div>
			<div>
				<dt className="text-muted-foreground font-medium">Proximity events</dt>
				<dd>
					board {snapshot.proximityBoard} · scene {snapshot.proximityScene}
				</dd>
			</div>
		</dl>
	);
}

class TopologyPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
	constructor(private readonly buildTree: () => import("@framework/playground").UiTreeNode) {
		super();
	}

	resolveTab(): SidePanelTabConfig {
		return {
			id: TOPOLOGY_PLAY_HIERARCHY_TAB_ID,
			icon: ListTree,
			order: 0,
			tree: new StaticTreePanelDefinition({ sections: this.buildTree().sections as TreeDataSection[] }),
		};
	}
}

class TopologyPlayStatusPanelDefinition extends PureSidePanelTabDefinition {
	resolveTab(): SidePanelTabConfig {
		return {
			id: "topology-play-status",
			icon: ClipboardList,
			order: 0,
			tree: new StaticTreePanelDefinition({
				sections: [
					{
						id: "topology-play-status.section",
						label: "Paired play",
						defaultOpen: true,
						items: [{ id: "topology-play-status.body", label: "Status", description: <TopologyPlayStatusPanel /> }],
					},
				],
			}),
		};
	}
}
//#endregion 🔖DetailsPanel

//#region 🔖Surfaces
function TopologyBoardSurfaceHost({ node }: { readonly node: UiBoardHostSurfaceNode }): React.ReactElement {
	const { controller, snapshot } = useTopologyPlaySnapshot();
	if (node.controllerId !== TOPOLOGY_PLAY_CONTROLLER_ID || node.surfaceId !== TOPOLOGY_PLAY_BOARD_SURFACE_ID || node.paneId !== TOPOLOGY_PLAY_BOARD_WINDOW_ID || !controller || !snapshot?.boardFixture || !snapshot.boardCamera) {
		return <div className="p-2 text-xs text-muted-foreground">Invalid topology board binding</div>;
	}
	const bindings = buildTopologyDualSurfaceBindings({
		...snapshot.sharedKinds,
		onBoardSelect: (snap) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, "setBoardSelection", { ids: snap.ids }),
		onSceneSelect: (snap) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, "setSceneSelection", { objectIds: snap.objectIds }),
		onBoardCamera: (camera) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, "setBoardCamera", { camera }),
		onSceneCamera: (camera) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, "setSceneCamera", { camera }),
		onSceneLodChange: undefined,
		...topologyMirrorConnectHandlers((payload) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, payload.surface === "board" ? "noteBoardConnect" : "noteSceneConnect")),
		...topologyMirrorProximityHandlers((payload) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, payload.surface === "board" ? "noteBoardProximity" : "noteSceneProximity")),
	});
	return (
		<TopologyBoardPane
			fixture={snapshot.boardFixture}
			bindings={bindings}
			selectedIds={snapshot.boardSelected}
			board={{
				camera: snapshot.boardCamera,
				onLodChange: (lod) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, "setBoardLodTag", { lod }),
				...snapshot.boardLodProps,
			}}
		/>
	);
}

function TopologySceneSurfaceHost({ node }: { readonly node: UiScene3DHostSurfaceNode }): React.ReactElement {
	const { controller, snapshot } = useTopologyPlaySnapshot();
	if (node.controllerId !== TOPOLOGY_PLAY_CONTROLLER_ID || node.surfaceId !== TOPOLOGY_PLAY_SCENE_SURFACE_ID || !controller || !snapshot?.sceneFixture || !snapshot.sceneCamera || !snapshot.boardFixture) {
		return <div className="p-2 text-xs text-muted-foreground">Invalid topology scene binding</div>;
	}
	const meshUrls = React.useMemo(() => [...new Set(snapshot.sceneFixture.objects.map((object) => object.meshUrl))], [snapshot.sceneFixture]);
	React.useEffect(() => {
		for (const url of meshUrls) useGLTF.preload(url);
	}, [meshUrls]);
	const bindings = buildTopologyDualSurfaceBindings({
		...snapshot.sharedKinds,
		onBoardSelect: (snap) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, "setBoardSelection", { ids: snap.ids }),
		onSceneSelect: (snap) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, "setSceneSelection", { objectIds: snap.objectIds }),
		onBoardCamera: (camera) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, "setBoardCamera", { camera }),
		onSceneCamera: (camera) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, "setSceneCamera", { camera }),
		onSceneLodChange: undefined,
		...topologyMirrorConnectHandlers((payload) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, payload.surface === "board" ? "noteBoardConnect" : "noteSceneConnect")),
		...topologyMirrorProximityHandlers((payload) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, payload.surface === "board" ? "noteBoardProximity" : "noteSceneProximity")),
	});
	return (
		<TopologyScenePane
			fixture={snapshot.sceneFixture}
			bindings={bindings}
			relocateMode={snapshot.relocateMode}
			selectedObjectId={snapshot.sceneSelected}
			scene={{ ...topologySceneChromeDefaults(), ...snapshot.sceneLodProps, camera: snapshot.sceneCamera ?? snapshot.sceneFixture.camera }}
		/>
	);
}
//#endregion 🔖Surfaces

//#region 🔖Mount
let topologyPlayChromeRegistered = false;

/** @emoji 🧊 Registers topology play board+scene surface hosts (called from `@framework/playground-renderer-react`). */
export function registerTopologyPlaySurfaceHosts(): void {
	if (topologyPlayChromeRegistered) return;
	topologyPlayChromeRegistered = true;
	registerUiBoardSurfaceHost(TOPOLOGY_PLAY_BOARD_SURFACE_ID, TopologyBoardSurfaceHost);
	registerUiScene3DSurfaceHost(TOPOLOGY_PLAY_SCENE_SURFACE_ID, TopologySceneSurfaceHost);
	registerWindowBody(TOPOLOGY_PLAY_BOARD_BODY_KEY, buildTopologyBoardDeclarativeBody);
	registerWindowBody(TOPOLOGY_PLAY_SCENE_BODY_KEY, buildTopologySceneDeclarativeBody);
}

function TopologyPlayChrome({ runtime }: { readonly runtime: ProductRuntime }): React.ReactElement {
	const generation = React.useSyncExternalStore(
		(listener) => runtime.subscribe(listener),
		() => runtime.generation,
		() => 0,
	);
	void generation;
	const controller = runtime.getActiveApp()?.controller as TopologyPlayShellController | undefined;
	const snapshot = controller?.getSnapshot() ?? null;
	const bus = runtime.commandBus;
	const snapshotKey = snapshot
		? `${snapshot.manifestLabel ?? ""}\u0001${snapshot.sceneSelected ?? ""}\u0001${[...snapshot.boardSelected].sort().join(",")}`
		: "";
	const workbenchTabs = React.useMemo(
		() =>
			snapshot && controller
				? [
						new TopologyPlayHierarchyPanelDefinition(() =>
							buildTopologyPlayHierarchySections(snapshot, {
								onSelectBoard: (id) => bus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, "setBoardSelection", { ids: [id] }),
								onSelectSceneObject: (objectId) =>
									bus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, "setSceneSelection", { objectIds: [objectId] }),
								onSelectSceneVortex: () => {},
								onSelectSceneAttraction: () => {},
							}),
						).resolveTab(),
					]
				: [],
		[snapshot, snapshotKey, controller, bus],
	);
	const detailTabs = React.useMemo(() => [new TopologyPlayStatusPanelDefinition().resolveTab()], []);
	return (
		<LevelProvider level="window">
			<div className={`flex h-screen min-h-0 w-screen flex-col ${getLevelBgClass("window")}`}>
				<PlaygroundView
					runtime={runtime}
					defaultAppId={TOPOLOGY_PLAY_APP_ID}
					augmentPanelTabs={{ workbench: workbenchTabs, details: detailTabs }}
					initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }}
				/>
			</div>
		</LevelProvider>
	);
}

/** @emoji 🚀 Mounts topology play chrome for a {@link Playground} (called from {@link renderPlayground}). */
export function mountTopologyPlayChrome(playground: Playground, rootId = "root"): void {
	mountPlaygroundApp(<TopologyPlayChrome runtime={playground.runtime} />, rootId);
}

//#endregion 🔖Mount

// #endregion 🛝PlayHost
