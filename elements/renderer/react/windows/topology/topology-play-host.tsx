// #region ­ƒº▓Header
// ­ƒÆ╗ elements/renderer/react/windows/topology/topology-play-host.tsx ÔÇö React host surfaces for the declarative topology play bundle.
// #endregion ­ƒº▓Header

import { useGLTF } from "@react-three/drei";
import * as React from "react";

import {
	LevelProvider,
	WorkbenchView,
	getLevelBgClass,
	mountReactApp,
	registerDeclarativeWindowBody,
	registerUiBoardSurfaceHost,
	registerUiScene3DSurfaceHost,
	useApp,
	type UiBoardHostSurfaceNode,
	type UiScene3DHostSurfaceNode,
} from "@elements/ui";

import {
	buildTopologyDualSurfaceBindings,
	TopologyBoardPane,
	TopologyScenePane,
	topologyMirrorConnectHandlers,
	topologyMirrorProximityHandlers,
	topologySceneChromeDefaults,
} from "./react/index.tsx";
import {
	TOPOLOGY_PLAY_APP_ID,
	TOPOLOGY_PLAY_BOARD_BODY_KEY,
	TOPOLOGY_PLAY_BOARD_SURFACE_ID,
	TOPOLOGY_PLAY_BOARD_WINDOW_ID,
	TOPOLOGY_PLAY_CONTROLLER_ID,
	TOPOLOGY_PLAY_SCENE_BODY_KEY,
	TOPOLOGY_PLAY_SCENE_SURFACE_ID,
	TopologyPlayShellController,
	buildTopologyBoardDeclarativeBody,
	buildTopologyPlayWorkbench,
	buildTopologySceneDeclarativeBody,
	type TopologyPlaySnapshot,
} from "./play/index.ts";
import "./play/globals.css";

//#region ­ƒöûSnapshot
function useTopologyPlaySnapshot(): { readonly controller: TopologyPlayShellController | undefined; readonly snapshot: TopologyPlaySnapshot | null } {
	const { workbench } = useApp();
	React.useSyncExternalStore(
		(listener) => workbench.subscribe(listener),
		() => workbench.generation,
		() => 0,
	);
	const controller = workbench.getActiveApp()?.controller as TopologyPlayShellController | undefined;
	return { controller, snapshot: controller?.getSnapshot() ?? null };
}
//#endregion ­ƒöûSnapshot

//#region ­ƒöûSurfaces
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
		onSceneLodChange: (lod) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, "setSceneLodTag", { lod }),
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
		onSceneLodChange: (lod) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, "setSceneLodTag", { lod }),
		...topologyMirrorConnectHandlers((payload) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, payload.surface === "board" ? "noteBoardConnect" : "noteSceneConnect")),
		...topologyMirrorProximityHandlers((payload) => controller.commandBus.dispatch(TOPOLOGY_PLAY_CONTROLLER_ID, payload.surface === "board" ? "noteBoardProximity" : "noteSceneProximity")),
	});
	return (
		<TopologyScenePane
			fixture={snapshot.sceneFixture}
			bindings={bindings}
			relocateMode={snapshot.relocateMode}
			selectedObjectId={snapshot.sceneSelected}
			scene={{ ...topologySceneChromeDefaults(), ...snapshot.sceneLodProps }}
		/>
	);
}
//#endregion ­ƒöûSurfaces

//#region ­ƒöûMount
let topologyPlayChromeRegistered = false;

function registerTopologyPlayChrome(): void {
	if (topologyPlayChromeRegistered) return;
	topologyPlayChromeRegistered = true;
	registerUiBoardSurfaceHost(TOPOLOGY_PLAY_BOARD_SURFACE_ID, TopologyBoardSurfaceHost);
	registerUiScene3DSurfaceHost(TOPOLOGY_PLAY_SCENE_SURFACE_ID, TopologySceneSurfaceHost);
	registerDeclarativeWindowBody(TOPOLOGY_PLAY_BOARD_BODY_KEY, buildTopologyBoardDeclarativeBody);
	registerDeclarativeWindowBody(TOPOLOGY_PLAY_SCENE_BODY_KEY, buildTopologySceneDeclarativeBody);
}

export function createTopologyPlayElement(): React.ReactElement {
	registerTopologyPlayChrome();
	return (
		<LevelProvider>
			<WorkbenchView workbench={buildTopologyPlayWorkbench()} defaultAppId={TOPOLOGY_PLAY_APP_ID} className={getLevelBgClass(0)} />
		</LevelProvider>
	);
}

/** @emoji ­ƒÜÇ Vite host entry: mounts topology play into `#root`. */
export function mountTopologyPlay(): void {
	mountReactApp(createTopologyPlayElement());
}
//#endregion ­ƒöûMount
