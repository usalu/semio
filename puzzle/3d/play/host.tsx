// #region 🧲Header
/** @emoji 🛝 Scene play React host — entry-only; imported from play/main.ts. */
// #endregion 🧲Header

import { LevelProvider, getLevelBgClass } from "@ui/react";
import { useGLTF } from "@react-three/drei";
import { ClipboardList, ListTree, Settings, Tags } from "lucide-react";
import React, { useCallback, useMemo, useSyncExternalStore } from "react";
import {
	registerTabIcon,
	registerUiScene3DSurfaceHost,
	registerWindowBody,
	useApp,
	type UiScene3DHostSurfaceNode,
} from "@framework/playground-renderer-react";
import nakaginSceneFixtureJson from "./fixtures/nakagin-capsule-tower.scene.json";
import {
	PlaySceneCanvas,
	SceneObjectStateProvider,
	parseFixtureV1,
	applyConnectToSceneFixture,
	blockedVortexFullIdsFromAttractions,
	parseKindCatalogs,
	parseKindCompatibility,
	sceneLodCanvasProps,
	sliderValueFromLod,
	DEFAULT_MANUAL_LOD,
	type FixtureV1,
	type RelocatePayload,
} from "../react/index.tsx";
import {
	SCENE_PLAY_BODY_KEY,
	SCENE_PLAY_CONTROLLER_ID,
	SCENE_PLAY_EMPTY_SELECTION,
	SCENE_PLAY_ICON_HIERARCHY,
	SCENE_PLAY_ICON_INSPECTOR,
	SCENE_PLAY_ICON_KINDS,
	SCENE_PLAY_ICON_SETTINGS,
	SCENE_PLAY_SCENE_SURFACE_ID,
	ScenePlayShellController,
	buildScenePlayDeclarativeBody,
	setScenePlaySurfaceHostRegistrar,
	type ScenePlaySnapshot,
} from "./index.ts";


function useScenePlayController(): ScenePlayShellController | undefined {
  const { runtime } = useApp();
  return runtime.getActiveApp()?.controller as ScenePlayShellController | undefined;
}

function useScenePlaySnapshot(): ScenePlaySnapshot {
  const ctrl = useScenePlayController();
  return useSyncExternalStore(
    (onStoreChange) => (ctrl ? ctrl.subscribeSnapshot(onStoreChange) : () => {}),
    () =>
      ctrl?.getSnapshot() ?? {
        fixture: null,
        fixtureRevision: 0,
        lodProps: sceneLodCanvasProps({ automaticLod: true, depthVariableLod: false, manualLod: DEFAULT_MANUAL_LOD }),
        lodTag: DEFAULT_MANUAL_LOD,
        lodSlider: sliderValueFromLod(DEFAULT_MANUAL_LOD),
        automaticLod: true,
        depthVariableLod: false,
        relocateMode: "translate",
        selection: SCENE_PLAY_EMPTY_SELECTION,
        selectedId: null,
        selectedLabel: null,
        selectionMode: "single",
        proximityRadius: 24,
        chunkSize: 256,
        gridFactor: 10,
        showLodGrid: false,
        gridSnapEnabled: true,
        proximityCount: 0,
        connectCount: 0,
        indirectCount: 0,
        compatibleObjectsCount: 0,
        targetRingCount: 0,
      },
    () => ({
      fixture: null,
      fixtureRevision: 0,
      lodProps: sceneLodCanvasProps({ automaticLod: true, depthVariableLod: false, manualLod: DEFAULT_MANUAL_LOD }),
      lodTag: DEFAULT_MANUAL_LOD,
      lodSlider: sliderValueFromLod(DEFAULT_MANUAL_LOD),
      automaticLod: true,
      depthVariableLod: false,
      relocateMode: "translate",
      selection: SCENE_PLAY_EMPTY_SELECTION,
      selectedId: null,
      selectedLabel: null,
      selectionMode: "single",
      proximityRadius: 24,
      chunkSize: 256,
      gridFactor: 10,
      showLodGrid: false,
      gridSnapEnabled: true,
      proximityCount: 0,
      connectCount: 0,
      indirectCount: 0,
      compatibleObjectsCount: 0,
      targetRingCount: 0,
    }),
  );
}

function ScenePlaySceneSurfaceHost({ node }: { readonly node: UiScene3DHostSurfaceNode }): React.ReactElement {
  const { runtime } = useApp();
  const bus = runtime.commandBus;
  const ctrl = useScenePlayController();
  if (node.controllerId !== SCENE_PLAY_CONTROLLER_ID) {
    return <div className="p-2 text-xs text-muted-foreground">Invalid scene viewport binding</div>;
  }
  const snap = useScenePlaySnapshot();
  if (!snap.fixture) {
    return <div className="p-4 text-destructive">Invalid scene fixture</div>;
  }
  const kindCompatibility = parseKindCompatibility(snap.fixture.meta);
  const kindCatalogs = parseKindCatalogs(snap.fixture.meta);
  const blockedVortexFullIds = blockedVortexFullIdsFromAttractions(snap.fixture.attractions);
  const selectedVortexFullIds = useMemo(() => new Set(snap.selection.vortexIds), [snap.selection.vortexIds]);
  const patchFixture = useCallback(
    (updater: (prev: FixtureV1) => FixtureV1) => {
      ctrl?.patchFixture(updater);
    },
    [ctrl],
  );
  const onRelocatePersist = useCallback(
    (payload: RelocatePayload, attractingByObjectId: ReadonlyMap<string, readonly string[]>) => {
      ctrl?.patchRelocate(payload, attractingByObjectId);
    },
    [ctrl],
  );
  const proximityRelocateEnabled = snap.fixture.attractions.length > 0;
  return (
    <div className="absolute inset-0 min-h-0 min-w-0">
      <SceneObjectStateProvider
        fixture={snap.fixture}
        fixtureRevision={snap.fixtureRevision}
        onConnect={(payload) => {
          patchFixture((fixture) => applyConnectToSceneFixture(fixture, payload));
          bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteConnect");
        }}
        onRelocate={onRelocatePersist}
      >
        <PlaySceneCanvas
          fixture={snap.fixture}
          proximityRelocateEnabled={proximityRelocateEnabled}
          kindCatalogs={kindCatalogs}
          kindCompatibility={kindCompatibility}
          blockedVortexFullIds={blockedVortexFullIds}
          lodTag={snap.lodTag}
          lodProps={snap.lodProps}
          relocateMode={snap.relocateMode}
          selection={snap.selection}
          selectedId={snap.selectedId}
          selectedLabel={snap.selectedLabel}
          selectionMode={snap.selectionMode}
          selectedVortexFullIds={selectedVortexFullIds}
          proximityRadius={snap.proximityRadius}
          chunkSize={snap.chunkSize}
          gridFactor={snap.gridFactor}
          showLodGrid={snap.showLodGrid}
          gridSnapEnabled={snap.gridSnapEnabled}
          setSelectedId={(id) => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "setSelectedId", { id })}
          onSelect={(selection) => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteSelection", selection)}
          onIndirectConnect={() => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteIndirect")}
          onProximityConnect={() => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteProximity")}
          onLodChange={(lod) => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "setEffectiveLod", { lod })}
          onCamera={(camera) => ctrl?.setCamera(camera)}
          onAttractionCompatibleObjects={() => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteCompatibleObjects")}
          onAttractionTargetRing={() => bus.dispatch(SCENE_PLAY_CONTROLLER_ID, "noteTargetRing")}
        />
      </SceneObjectStateProvider>
    </div>
  );
}

let scenePlayChromeRegistered = false;

/** @emoji 🧊 Registers scene play surface host, window body, tab icons, and mesh preload. */
export function registerSceneSurfaceHosts(): void {
  if (scenePlayChromeRegistered) return;
  scenePlayChromeRegistered = true;
  registerUiScene3DSurfaceHost(SCENE_PLAY_SCENE_SURFACE_ID, ScenePlaySceneSurfaceHost);
  registerWindowBody(SCENE_PLAY_BODY_KEY, buildScenePlayDeclarativeBody);
  registerTabIcon(SCENE_PLAY_ICON_INSPECTOR, ClipboardList);
  registerTabIcon(SCENE_PLAY_ICON_KINDS, Tags);
  registerTabIcon(SCENE_PLAY_ICON_HIERARCHY, ListTree);
  registerTabIcon(SCENE_PLAY_ICON_SETTINGS, Settings);
  const fixture = parseFixtureV1(nakaginSceneFixtureJson as unknown);
  if (fixture) {
    const urls = [...new Set(fixture.objects.map((object) => object.meshUrl))];
    for (const url of urls) useGLTF.preload(url);
  }
}

setScenePlaySurfaceHostRegistrar(registerSceneSurfaceHosts);

// #endregion 🛝PlayHost
