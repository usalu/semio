// #region 🧲Header
/** @emoji 🛝 Playground play host for Lowpoly — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import { type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, PlaygroundView, PlaygroundContext, useApp, PureSidePanelTabDefinition, CallbackTreePanelDefinition, registerUiPuzzle3dSurfaceHost, Platform, CommandBus, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { type SidePanelTabConfig, UiPuzzle3dHostSurfaceNode } from "@semio-tech/framework-playground-core";
import {
  LOWPOLY_PLAY_APP_ID,
  LOWPOLY_PLAY_CATALOGUE_TAB_ID,
  LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON,
  LOWPOLY_PLAY_HIERARCHY_TAB_ID,
  LOWPOLY_PLAY_INSPECTION_TAB_ID,
  LOWPOLY_PLAY_LAYERS_TAB_ID,
  LOWPOLY_PLAY_SURFACE_ID,
  LOWPOLY_PLAY_UV_SURFACE_ID,
  LOWPOLY_PLAY_WINDOW_KIND_ID,
  LowpolyPlayController,
  buildLowpolyPlayCatalogueTree,
  buildLowpolyPlayHierarchyTree,
  buildLowpolyPlayInspectorTree,
  buildLowpolyPlayLayersTree,
  registerLowpolyPlayDeclarativeBodies,
} from "@semio-tech/lowpoly-core";

import { decodeLowpolySelectionTargets, isLowpolyFixtureReady, parseLowpolyFixtureJson, type LowpolySceneObject } from "@semio-tech/lowpoly-core";

let lowpolyPlayChromeRegistered = false;
const lowpolyPlayControllerRef: { current: LowpolyPlayController | null } = { current: null };

type LowpolySharedPlaySnapshot = {
	readonly session: LowpolySessionWasm | null;
	readonly sceneObjects: readonly LowpolySceneObject[];
	readonly paintTextureRevision: number;
	readonly generation: number;
};

const lowpolySharedPlaySnapshot: LowpolySharedPlaySnapshot = {
	session: null,
	sceneObjects: [],
	paintTextureRevision: 0,
	generation: 0,
};
const lowpolySharedPlayListeners = new Set<() => void>();
const lowpolyPaintStrokeHandlersRef: { current: { onBegin?: () => void; onEnd?: () => void } } = { current: {} };

function notifyLowpolySharedPlay(next?: Partial<Pick<LowpolySharedPlaySnapshot, "session" | "sceneObjects" | "paintTextureRevision">>): void {
	if (next?.session !== undefined) (lowpolySharedPlaySnapshot as { session: LowpolySessionWasm | null }).session = next.session;
	if (next?.sceneObjects !== undefined) (lowpolySharedPlaySnapshot as { sceneObjects: readonly LowpolySceneObject[] }).sceneObjects = next.sceneObjects;
	if (next?.paintTextureRevision !== undefined) (lowpolySharedPlaySnapshot as { paintTextureRevision: number }).paintTextureRevision = next.paintTextureRevision;
	(lowpolySharedPlaySnapshot as { generation: number }).generation += 1;
	for (const listener of lowpolySharedPlayListeners) listener();
}

function bumpLowpolyPaintTextureRevision(): void {
	notifyLowpolySharedPlay({ paintTextureRevision: lowpolySharedPlaySnapshot.paintTextureRevision + 1 });
}

function useLowpolySharedPlaySnapshot(): LowpolySharedPlaySnapshot {
	reactHostPort.useSyncExternalStore(
		(listener) => {
			lowpolySharedPlayListeners.add(listener);
			return () => lowpolySharedPlayListeners.delete(listener);
		},
		() => lowpolySharedPlaySnapshot.generation,
		() => 0,
	);
	return lowpolySharedPlaySnapshot;
}

function useLowpolyPlayController(runtimeOverride?: Platform): LowpolyPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as LowpolyPlayController | undefined;
  lowpolyPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useLowpolyPlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as LowpolyPlayController | undefined;
      lowpolyPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as LowpolyPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function useLowpolyPlayHoverTarget(runtime: Platform): import("@semio-tech/lowpoly-core").LowpolyTarget | null {
	reactHostPort.useSyncExternalStore(
		(listener) => {
			const ctrl = runtime.getActiveApp()?.controller as LowpolyPlayController | undefined;
			lowpolyPlayControllerRef.current = ctrl ?? null;
			const unsubscribeRuntime = runtime.subscribe(listener);
			const unsubscribeHover = ctrl?.subscribeHover(listener);
			return () => {
				unsubscribeRuntime();
				unsubscribeHover?.();
			};
		},
		() => (runtime.getActiveApp()?.controller as LowpolyPlayController | undefined)?.getHoverRevision() ?? 0,
		() => 0,
	);
	const ctrl = runtime.getActiveApp()?.controller as LowpolyPlayController | undefined;
	return ctrl?.getHoveredTargetSnapshot() ?? null;
}

function syncLowpolyControllerFromSession(ctrl: LowpolyPlayController, session: LowpolySessionWasm): void {
  const json = session.fixtureJson();
  ctrl.run("setFixtureJson", { json });
  const fixture = parseLowpolyFixtureJson(json);
  if (fixture) {
    ctrl.run("setSelection", {
      keys: [...fixture.selection.keys],
      activeObjectId: fixture.activeObjectId,
    });
  }
}

function lowpolyMirrorAxis(toolParams: Record<string, number>): string {
  const axisIndex = toolParams.mirrorAxis ?? 0;
  return axisIndex === 1 ? "y" : axisIndex === 2 ? "z" : "x";
}

function LowpolyPlaySessionBridge({ runtime }: { readonly runtime: Platform }): null {
  const ctrl = useLowpolyPlayController(runtime);
  const interactionRevision = useLowpolyPlayInteractionRevision(runtime);
  const meshEpoch = ctrl?.getMeshCommandEpoch() ?? 0;
  const toolParams = ctrl?.getToolParams() ?? {};
  const paintStrokeBeforeRef = reactHostPort.useRef<Uint8Array | null>(null);
  const activeObjectId = lowpolySharedPlaySnapshot.sceneObjects.find((object) => object.active)?.id;

  reactHostPort.useEffect(() => {
    let cancelled = false;
    void (async () => {
      if (lowpolySharedPlaySnapshot.session) return;
      const session = await createLowpolySession();
      if (cancelled) return;
      const json = ctrl?.getFixtureJson() ?? LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON;
      if (isLowpolyFixtureReady(json)) {
        safeLoadLowpolyFixture(session, json);
      } else {
        const defaultJson = await loadDefaultLowpolyFixtureJson();
        safeLoadLowpolyFixture(session, defaultJson);
        if (ctrl) syncLowpolyControllerFromSession(ctrl, session);
      }
      notifyLowpolySharedPlay({
        session,
        sceneObjects: tessellateAllLowpolySession(session),
      });
    })();
    return () => {
      cancelled = true;
    };
  }, [ctrl]);

  const controllerFixtureJson = ctrl?.getFixtureJson() ?? "";
  reactHostPort.useEffect(() => {
    const session = lowpolySharedPlaySnapshot.session;
    if (!session || !ctrl) return;
    const json = ctrl.getFixtureJson();
    if (!isLowpolyFixtureReady(json)) return;
    safeLoadLowpolyFixture(session, json);
    const fixture = parseLowpolyFixtureJson(json);
    syncLowpolySessionSelection(session, ctrl.getSelectionTargets(), ctrl.getSelectedTargets(), fixture?.activeObjectId);
    notifyLowpolySharedPlay({ sceneObjects: tessellateAllLowpolySession(session) });
  }, [ctrl, controllerFixtureJson]);

  reactHostPort.useEffect(() => {
    const session = lowpolySharedPlaySnapshot.session;
    if (!session || !ctrl) return;
    const fixture = parseLowpolyFixtureJson(ctrl.getFixtureJson());
    syncLowpolySessionSelection(session, ctrl.getSelectionTargets(), ctrl.getSelectedTargets(), fixture?.activeObjectId);
  }, [ctrl, interactionRevision]);

  reactHostPort.useEffect(() => {
    const session = lowpolySharedPlaySnapshot.session;
    if (!session || !ctrl || meshEpoch === 0) return;
    const pending = ctrl.getPendingMeshCommand();
    const paintPending = ctrl.getPendingPaintCommand();
    if (!pending && !paintPending) return;
    try {
      if (pending?.startsWith("addPrimitive:")) {
        const kind = pending.slice("addPrimitive:".length);
        session.addPrimitive(kind);
      } else if (pending?.startsWith("flipFace:")) {
        const [, objectId, faceId] = pending.split(":");
        if (objectId && faceId != null) {
          session.setActiveObject(objectId);
          session.flipFaces([Number(faceId)]);
        }
      } else if (pending === "extrude") session.extrudeFaces(toolParams.extrudeDistance ?? 0.25);
      else if (pending === "inset") session.insetFaces(toolParams.insetAmount ?? 0.1);
      else if (pending === "flipFaces") session.flipFaces([...ctrl.getSelectedIds("face")]);
      else if (pending === "bevel") session.bevelEdges(toolParams.bevelAmount ?? 0.05, toolParams.bevelSegments ?? 1);
      else if (pending === "loopCut") session.loopCut(toolParams.loopCuts ?? 1);
      else if (pending === "merge") session.mergeVertices();
      else if (pending === "dissolve") session.dissolveEdges();
      else if (pending === "subdivide") session.subdivideFaces();
      else if (pending === "triangulate") session.triangulate();
      else if (pending === "mirror") session.mirror(lowpolyMirrorAxis(toolParams), 0.001);
      else if (pending === "decimate") session.decimate(toolParams.decimateRatio ?? 0.5);
      else if (pending === "snap") session.snapToGrid(toolParams.snapGrid ?? 0.25);
      else if (pending === "toggleSmooth") session.setSmoothShading(!ctrl.getSmoothShading());
      else if (paintPending?.command === "unwrapActive") session.unwrapActive();
      else if (paintPending?.command === "markUvSeam") {
        session.markUvSeam(Boolean(paintPending.args?.seam), [...ctrl.getSelectedIds("edge")]);
      }
      syncLowpolyControllerFromSession(ctrl, session);
      notifyLowpolySharedPlay({ sceneObjects: tessellateAllLowpolySession(session) });
    } catch {
      /* mesh command may fail on empty selection */
    } finally {
      ctrl.clearPendingMeshCommand();
      ctrl.clearPendingPaintCommand();
    }
  }, [meshEpoch, ctrl, toolParams]);

  const paintVcsGeneration = reactHostPort.useSyncExternalStore(
    (listener) => ctrl?.subscribePaintVcs(listener) ?? (() => {}),
    () => ctrl?.getPaintVcsGeneration() ?? 0,
    () => 0,
  );

  reactHostPort.useEffect(() => {
    const session = lowpolySharedPlaySnapshot.session;
    if (!session || !ctrl) return;
    const projection = ctrl.getPaintProjection();
    const expected = 1024 * 1024 * 4;
    if (projection.pixels.length !== expected) return;
    session.setPaintLayerPixels(projection.objectId, projection.layerIndex, new Uint8Array(projection.pixels));
    bumpLowpolyPaintTextureRevision();
  }, [paintVcsGeneration, ctrl]);

  reactHostPort.useEffect(() => {
    lowpolyPaintStrokeHandlersRef.current.onBegin = () => {
      const session = lowpolySharedPlaySnapshot.session;
      if (!session || !activeObjectId) return;
      const layerIndex = ctrl?.getActivePaintLayerIndex() ?? 0;
      paintStrokeBeforeRef.current = new Uint8Array(session.paintLayerPixels(activeObjectId, layerIndex));
    };
    lowpolyPaintStrokeHandlersRef.current.onEnd = () => {
      const session = lowpolySharedPlaySnapshot.session;
      if (!session || !ctrl || !activeObjectId) return;
      const layerIndex = ctrl.getActivePaintLayerIndex();
      const before = paintStrokeBeforeRef.current;
      const after = session.paintLayerPixels(activeObjectId, layerIndex);
      if (before) {
        ctrl.dispatchPaintVcs({
          kind: "apply",
          operations: [
            {
              kind: "layerPixels",
              objectId: activeObjectId,
              layerIndex,
              before: [...before],
              after: [...after],
            },
          ],
        });
      }
      paintStrokeBeforeRef.current = null;
      bumpLowpolyPaintTextureRevision();
    };
    return () => {
      lowpolyPaintStrokeHandlersRef.current = {};
    };
  }, [activeObjectId, ctrl]);

  return null;
}

function LowpolyPlaySurfaceHost({ node: _node }: { readonly node: UiPuzzle3dHostSurfaceNode }): ReactElement {
  const { activeModeId, runtime } = useApp();
  const ctrl = useLowpolyPlayController(runtime);
  const interactionRevision = useLowpolyPlayInteractionRevision(runtime);
  const hoveredTarget = useLowpolyPlayHoverTarget(runtime);
  const shared = useLowpolySharedPlaySnapshot();
  const session = shared.session;
  const sceneObjects = shared.sceneObjects;
  const toolParams = ctrl?.getToolParams() ?? {};
  const selectedTargets = reactHostPort.useMemo(
    () => [...(ctrl?.getSelectedTargets() ?? [])],
    [ctrl, interactionRevision],
  );
  const selectionTargets = ctrl?.getSelectionTargets() ?? { mesh: true, vertex: false, edge: false, face: false };
  const controllerFixtureJson = ctrl?.getFixtureJson() ?? LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON;
  const fixtureJson =
    session && isLowpolyFixtureReady(controllerFixtureJson)
      ? controllerFixtureJson
      : session?.fixtureJson() ?? controllerFixtureJson;
  const interactionMode = activeModeId === "paint" ? "paint" : "model";

  const onFixtureChange = reactHostPort.useCallback(
    (json: string) => {
      ctrl?.run("setFixtureJson", { json });
    },
    [ctrl],
  );
  const onSelectionChange = reactHostPort.useCallback(
    (keys: readonly string[], activeObjectId?: string) => {
      if (session) {
        syncLowpolySessionSelection(session, selectionTargets, decodeLowpolySelectionTargets(keys), activeObjectId);
      }
      ctrl?.run("setSelection", { keys: [...keys], activeObjectId });
    },
    [ctrl, selectionTargets, session],
  );
  const onPaintStrokeBegin = reactHostPort.useCallback(() => {
    lowpolyPaintStrokeHandlersRef.current.onBegin?.();
  }, []);
  const onPaintStrokeEnd = reactHostPort.useCallback(() => {
    lowpolyPaintStrokeHandlersRef.current.onEnd?.();
  }, []);
  const onSceneChange = reactHostPort.useCallback((objects: readonly LowpolySceneObject[]) => {
    notifyLowpolySharedPlay({ sceneObjects: objects });
  }, []);

  return (
    <div className="absolute inset-0 min-h-0 min-w-0">
      <LowpolyCanvas
        fixtureJson={fixtureJson}
        sceneObjects={sceneObjects}
        selectionTargets={selectionTargets}
        selectedTargets={selectedTargets}
        hoveredTarget={hoveredTarget}
        transformTool={ctrl?.getTransformTool() ?? "move"}
        session={session}
        interactionMode={interactionMode}
        paintTool={ctrl?.getPaintTool() ?? "brush"}
        paintLayerIndex={ctrl?.getActivePaintLayerIndex() ?? 0}
        paintColor={ctrl?.getPaintColor() ?? [255, 64, 64, 255]}
        paintBrushSize={toolParams.brushSize ?? 16}
        paintBrushOpacity={toolParams.brushOpacity ?? 1}
        paintBrushHardness={toolParams.brushHardness ?? 0.5}
        paintTextureRevision={shared.paintTextureRevision}
        onFixtureChange={onFixtureChange}
        onSelectionChange={onSelectionChange}
        onHoverChange={(target) => ctrl?.run("setHover", { target })}
        onSceneChange={onSceneChange}
        onPaintStrokeBegin={onPaintStrokeBegin}
        onPaintStrokeEnd={onPaintStrokeEnd}
        onPaintTextureRefresh={bumpLowpolyPaintTextureRevision}
        className="h-full w-full"
      />
    </div>
  );
}

function LowpolyUvSurfaceHost({ node: _node }: { readonly node: UiPuzzle3dHostSurfaceNode }): ReactElement {
  const ctrl = useLowpolyPlayController();
  const shared = useLowpolySharedPlaySnapshot();
  const session = shared.session;
  const sceneObjects = shared.sceneObjects;
  const toolParams = ctrl?.getToolParams() ?? {};
  const fixtureJson = ctrl?.getFixtureJson() ?? LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON;
  const activeObject = sceneObjects.find((object) => object.active) ?? sceneObjects[0] ?? null;
  const paintStrokeBeforeRef = reactHostPort.useRef<Uint8Array | null>(null);

  const onPaintStrokeBegin = reactHostPort.useCallback(() => {
    if (!session || !activeObject) return;
    const layerIndex = ctrl?.getActivePaintLayerIndex() ?? 0;
    paintStrokeBeforeRef.current = new Uint8Array(session.paintLayerPixels(activeObject.id, layerIndex));
  }, [activeObject, ctrl, session]);

  const onPaintStrokeEnd = reactHostPort.useCallback(() => {
    if (!session || !ctrl || !activeObject) return;
    const layerIndex = ctrl.getActivePaintLayerIndex();
    const before = paintStrokeBeforeRef.current;
    const after = session.paintLayerPixels(activeObject.id, layerIndex);
    if (before) {
      ctrl.dispatchPaintVcs({
        kind: "apply",
        operations: [
          {
            kind: "layerPixels",
            objectId: activeObject.id,
            layerIndex,
            before: [...before],
            after: [...after],
          },
        ],
      });
    }
    paintStrokeBeforeRef.current = null;
    bumpLowpolyPaintTextureRevision();
    ctrl.run("setFixtureJson", { json: session.fixtureJson() });
  }, [activeObject, ctrl, session]);

  return (
    <div className="absolute inset-0 min-h-0 min-w-0">
      <LowpolyUvCanvas
        sceneObject={activeObject}
        session={session}
        paintTool={ctrl?.getPaintTool() ?? "brush"}
        paintLayerIndex={ctrl?.getActivePaintLayerIndex() ?? 0}
        paintColor={ctrl?.getPaintColor() ?? [255, 64, 64, 255]}
        paintBrushSize={toolParams.brushSize ?? 16}
        paintBrushOpacity={toolParams.brushOpacity ?? 1}
        paintBrushHardness={toolParams.brushHardness ?? 0.5}
        paintTextureRevision={shared.paintTextureRevision}
        onFixtureChange={(json) => ctrl?.run("setFixtureJson", { json })}
        onPaintStrokeBegin={onPaintStrokeBegin}
        onPaintStrokeEnd={onPaintStrokeEnd}
        onPaintTextureRefresh={bumpLowpolyPaintTextureRevision}
        className="h-full w-full"
      />
    </div>
  );
}

export function registerLowpolyPlaySurfaceHosts(): void {
  if (lowpolyPlayChromeRegistered) return;
  lowpolyPlayChromeRegistered = true;
  registerUiPuzzle3dSurfaceHost(LOWPOLY_PLAY_SURFACE_ID, LowpolyPlaySurfaceHost);
  registerUiPuzzle3dSurfaceHost(LOWPOLY_PLAY_UV_SURFACE_ID, LowpolyUvSurfaceHost);
  registerLowpolyPlayDeclarativeBodies();
}

class LowpolyPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: LOWPOLY_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = lowpolyPlayControllerRef.current;
        const bus = ctrl?.commandBus ?? new CommandBus();
        const fixture = ctrl?.getFixtureJson() ?? LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON;
        const treeNode = buildLowpolyPlayHierarchyTree(
          fixture,
          ctrl?.getSelectedTargets() ?? [],
          {
            hoveredTarget: ctrl?.getHoveredTarget() ?? null,
            onHover: (target) => ctrl?.run("setHover", { target }),
            onFlipFace: (objectId, faceId) => ctrl?.run("flipFace", { objectId, faceId }),
          },
        );
        return { ...uiTreeNodeToTreePanelConfig(treeNode, bus), selectionMode: "multiple" };
      }),
    };
  }
}

class LowpolyPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: LOWPOLY_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const bus = lowpolyPlayControllerRef.current?.commandBus ?? new CommandBus();
        const treeNode = buildLowpolyPlayCatalogueTree();
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class LowpolyPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: LOWPOLY_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = lowpolyPlayControllerRef.current;
        const bus = ctrl?.commandBus ?? new CommandBus();
        const treeNode = buildLowpolyPlayInspectorTree(ctrl?.getFixtureJson() ?? LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON, { ...(ctrl?.getToolParams() ?? {}) });
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class LowpolyPlayLayersPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: LOWPOLY_PLAY_LAYERS_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: "Layers",
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = lowpolyPlayControllerRef.current;
        const bus = ctrl?.commandBus ?? new CommandBus();
        const treeNode = buildLowpolyPlayLayersTree(ctrl?.getFixtureJson() ?? LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON, ctrl?.getActivePaintLayerIndex() ?? 0);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function LowpolyPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useLowpolyPlayController(runtime);
  const interactionRevision = useLowpolyPlayInteractionRevision(runtime);
  const hierarchyPanel = reactHostPort.useMemo(() => new LowpolyPlayHierarchyPanelDefinition(), []);
  const cataloguePanel = reactHostPort.useMemo(() => new LowpolyPlayCataloguePanelDefinition(), []);
  const inspectionPanel = reactHostPort.useMemo(() => new LowpolyPlayInspectionPanelDefinition(), []);
  const layersPanel = reactHostPort.useMemo(() => new LowpolyPlayLayersPanelDefinition(), []);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [hierarchyPanel, cataloguePanel],
      details: [inspectionPanel, layersPanel],
    }),
    [interactionRevision, cataloguePanel, hierarchyPanel, inspectionPanel, layersPanel],
  );
  return (
    <>
      <LowpolyPlaySessionBridge runtime={runtime} />
      <PlaygroundView runtime={runtime} defaultAppId={LOWPOLY_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} />
    </>
  );
}

function LowpolyPlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <LowpolyPlayInner runtime={runtime} />;
}

export function mountLowpolyPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<LowpolyPlayChrome runtime={playground.runtime} />, rootId);
}

const lowpolyPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerLowpolyPlaySurfaceHosts,
  mount: mountLowpolyPlayChrome,
};

export async function bootLowpolyPlay(playground: Playground, rootId = "root"): Promise<void> {
  const ctrl = playground.runtime.getActiveApp()?.controller as LowpolyPlayController | undefined;
  if (ctrl && !isLowpolyFixtureReady(ctrl.getFixtureJson())) {
    const json = await loadDefaultLowpolyFixtureJson();
    ctrl.run("setFixtureJson", { json });
    const fixture = parseLowpolyFixtureJson(json);
    if (fixture) {
      ctrl.run("setSelection", { mode: fixture.selection.mode, ids: [...fixture.selection.ids] });
    }
  }
  bootPlayground(playground, lowpolyPlayChromeBoot, rootId);
}
//#endregion 🔖LowpolyPlayHost