// #region 🧲Header
/** @emoji 🛝 Playground play host for Puzzle5d — loaded only via `./play` subpath. */
// #endregion 🧲Header

import { createWriterDocument } from "@semio-tech/writer-core";
import { WriterCanvas } from "@semio-tech/writer-react";
import type { ReactElement } from "react";
import { type Playground, type PlaygroundChromeBoot, type Store, bootPlayground, mountPlaygroundApp, PlaygroundView, useApp, PureSidePanelTabDefinition, CallbackTreePanelDefinition, registerUiPuzzle3dSurfaceHost, registerUiPuzzle2dSurfaceHost, registerUiWriterSurfaceHost, registerTabIcon, Platform, CommandBus, collectUiTreeItemDragData, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig, registerWindowBody, uiDeclarativeSectionsToTree } from "@semio-tech/framework-playground-renderer-react";
import { reactHostPort, createIconComponent } from "@semio-tech/ui-react";
import { type SidePanelTabConfig, UiTreeNode, UiPuzzle2dHostSurfaceNode, UiPuzzle3dHostSurfaceNode } from "@semio-tech/framework-playground-core";
import * as React from "react";
// #region 🔌Adapters
import {
    PUZZLE_5D_PLAY_2D_BODY_KEY,
    PUZZLE_5D_PLAY_2D_SURFACE_ID,
    PUZZLE_5D_PLAY_2D_WINDOW_ID,
    PUZZLE_5D_PLAY_3D_BODY_KEY,
    PUZZLE_5D_PLAY_3D_SURFACE_ID,
    PUZZLE_5D_PLAY_JACK_BODY_KEY,
    PUZZLE_5D_PLAY_JACK_SURFACE_ID,
    PUZZLE_5D_PLAY_JACK_WINDOW_ID,
    PUZZLE_5D_PLAY_APP_ID,
    PUZZLE_5D_PLAY_CONTROLLER_ID,
    PUZZLE_5D_PLAY_HIERARCHY_TAB_ID,
    PUZZLE_5D_PLAY_ICON_KINDS,
    PUZZLE_5D_PLAY_KINDS_TAB_ID,
    PUZZLE_5D_PLAY_STORE_ID,
    Puzzle5dPlayShellController,
    Puzzle5dStoreBridge,
    buildPuzzle5d2dDeclarativeBody,
    buildPuzzle5d3dDeclarativeBody,
    buildPuzzle5dJackDeclarativeBody,
    buildPuzzle5dPlayHierarchySections,
    buildPuzzle5dPlayInspectorTree,
    buildPuzzle5dPlayKindsTree,
    puzzle5dFixturePaletteTreeDragController,
    type Puzzle5dPlayHostBridge,
    type Puzzle5dPlaySnapshot
} from "@semio-tech/puzzle-5d-core";

// #endregion 🔌Adapters

//#region 🔖Snapshot
function usePuzzle5dPlaySnapshot(): { readonly controller: Puzzle5dPlayShellController | undefined; readonly snapshot: Puzzle5dPlaySnapshot | null } {
  const { runtime } = useApp();
  reactHostPort.useSyncExternalStore(
    (listener) => runtime.subscribe(listener),
    () => runtime.generation,
    () => 0,
  );
  const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController | undefined;
  return { controller, snapshot: controller?.getSnapshot() ?? null };
}
//#endregion 🔖Snapshot

//#region 🔖HostBridge
function Puzzle5dPlayHostBridgeInstaller(props: { readonly controller: Puzzle5dPlayShellController; readonly store: Puzzle5dStore }): null {
  const { controller, store } = props;
  const selectionMethodRef = reactHostPort.useRef<Puzzle2dSelectionMethod>("rectangle");
  const selectionModeRef = reactHostPort.useRef<Puzzle2dSelectionMode>("default");
  const selectionTargetsRef = reactHostPort.useRef<Puzzle2dSelectionTargets>({ nodes: true, edges: true, handles: true });
  const gridSnapRef = reactHostPort.useRef(true);
  const redrawPlayingRef = reactHostPort.useRef(false);
  const fillSeedRef = reactHostPort.useRef(1);

  reactHostPort.useEffect(() => {
    installPuzzle3dPlayBrushHost(store.read().meta);
  }, [store]);

  reactHostPort.useLayoutEffect(() => {
    const commitBrushPlacement = (payload: Parameters<typeof puzzle5dCommitBrushPlacementToPlay>[1]) => {
      if (puzzle5dCommitBrushPlacementToPlay(store, payload)) {
        controller.setBrushEngagementPossibles([]);
      }
    };
    puzzle2dSetBrushPlaceCommitHandler(commitBrushPlacement);
    return () => {
      puzzle2dSetBrushPlaceCommitHandler(null);
    };
  }, [controller, store]);

  reactHostPort.useEffect(() => {
    const bridge: Puzzle5dPlayHostBridge = {
      getToolbarState: () => ({
        puzzle2dActiveTool: controller.getActiveTool(),
        puzzle2dSuggestionOffset: controller.getSuggestionOffset(),
        puzzle2dSelectionMethod: selectionMethodRef.current,
        puzzle2dSelectionMode: selectionModeRef.current,
        puzzle2dSelectionTargets: selectionTargetsRef.current,
        puzzle2dGridSnapEnabled: gridSnapRef.current,
        puzzle2dRedrawPlaying: redrawPlayingRef.current,
      }),
      runHostCommand: (command, args) => {
        switch (command) {
          case "setActiveTool": {
            const tool = (args as { tool?: string }).tool;
            const prev = (args as { prevTool?: string }).prevTool;
            if (prev === "fill" && tool !== "fill") {
              store.clearFill();
            }
            if (tool === "fill" && prev !== "fill") {
              fillSeedRef.current = (Date.now() ^ Math.floor(Math.random() * 0x7fffffff)) >>> 0;
            }
            break;
          }
          case "setSuggestionOffset": {
            const distance = Number((args as { distance?: number }).distance);
            if (Number.isFinite(distance)) {
              puzzle2dActiveRenderer()?.setSuggestionOffset(distance);
            }
            break;
          }
          case "setBrushOverlapBudget": {
            break;
          }
          case "pickBrushCandidate": {
            const index = Number((args as { index?: number }).index);
            if (Number.isFinite(index)) {
              puzzle5dPickBrushCandidateAtIndex(index);
            }
            break;
          }
          case "engagementPossibleSelect": {
            const possibleId = (args as { possibleId?: string }).possibleId ?? "";
            const brushMatch = possibleId.match(/^puzzle(?:2d|3d|5d)\.brush\.(\d+)$/);
            if (brushMatch) {
              const index = Number(brushMatch[1]);
              if (Number.isFinite(index)) {
                puzzle5dPickBrushCandidateAtIndex(index);
              }
              break;
            }
            break;
          }
          case "setSelectionMethod": {
            const method = (args as { method?: Puzzle2dSelectionMethod }).method;
            if (method) {
              selectionMethodRef.current = method;
              puzzle2dActiveRenderer()?.setSelectionOptions({ method });
            }
            break;
          }
          case "setSelectionMode": {
            const mode = (args as { mode?: Puzzle2dSelectionMode }).mode;
            if (mode) {
              selectionModeRef.current = mode;
              puzzle2dActiveRenderer()?.setSelectionOptions({ mode });
            }
            break;
          }
          case "toggleSelectionTarget": {
            const kind = (args as { kind?: keyof Puzzle2dSelectionTargets }).kind;
            if (kind) {
              selectionTargetsRef.current = { ...selectionTargetsRef.current, [kind]: !selectionTargetsRef.current[kind] };
            }
            break;
          }
          case "toggleGridSnap": {
            gridSnapRef.current = !gridSnapRef.current;
            break;
          }
          case "toggleRedrawPlaying": {
            redrawPlayingRef.current = !redrawPlayingRef.current;
            break;
          }
          case "clearSelection": {
            controller.run("set2dSelection", { ids: [] });
            controller.run("set3dSelection", { objectIds: [] });
            store.setSelection({ partIds: [], gripIds: [] });
            break;
          }
          default:
            break;
        }
      },
    };
    controller.setHostBridge(bridge);
    return () => controller.setHostBridge(null);
  }, [controller, store]);

  return null;
}

function puzzle5dPickBrushCandidateAtIndex(index: number): void {
  const flatSession = puzzle2dGetBrushSessionSnapshot();
  const flatCandidate = flatSession?.candidates[index];
  puzzle5dFlatRendererRef.current?.setBrushCandidateIndex(index);
  if (!flatCandidate) {
    puzzle3dBrushEngagementSourceRef.current.pickCandidate(index);
    return;
  }
  const volumeIndex = puzzle3dBrushEngagementSourceRef.current.candidates.findIndex(
    (row) => row.objectKindId === flatCandidate.nodeKind && row.sourceVortexIndex === flatCandidate.targetHandleIndex,
  );
  if (volumeIndex >= 0) {
    puzzle3dBrushEngagementSourceRef.current.pickCandidate(volumeIndex);
  }
}

function puzzle5dBrushCandidateRows(payload: Puzzle2dBrushCandidatesPayload, kindCatalogs: ReturnType<typeof project2dKindCatalogs>): { readonly id: string; readonly label: string }[] {
  return payload.candidates.map((candidate, index) => {
    const labels = puzzle2dBrushCandidateDisplayLabels(candidate, kindCatalogs ?? undefined);
    return {
      id: `puzzle5d.brush.${index}`,
      label: `${labels.object} · ${labels.handle}`,
    };
  });
}

function usePuzzle5dPlayStore(): Puzzle5dStore {
  return usePuzzle5dStore();
}
//#endregion 🔖HostBridge

const puzzle5dPlayControllerRef: { current: Puzzle5dPlayShellController | null } = { current: null };

function buildPuzzle5dPlayInspectorTreePanel(snapshot: Puzzle5dPlaySnapshot | null): UiTreeNode {
  if (!snapshot) {
    return uiDeclarativeSectionsToTree([
      { type: "section", id: "puzzle-5d-play-inspector.empty", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "No puzzle 5d snapshot" }] },
    ]);
  }
  return buildPuzzle5dPlayInspectorTree(snapshot);
}

//#region 🔖DetailsPanel
class Puzzle5dPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  constructor(
    private readonly buildTree: () => import("@semio-tech/framework-playground-core").UiTreeNode,
    private readonly commandBus: CommandBus,
  ) {
    super();
  }

  buildTab(): SidePanelTabConfig {
    return {
      id: PUZZLE_5D_PLAY_HIERARCHY_TAB_ID,
      icon: createIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const treeNode = this.buildTree();
        return uiTreeNodeToTreePanelConfig(treeNode, this.commandBus);
      }),
    };
  }
}

class Puzzle5dPlayKindsPanelDefinition extends PureSidePanelTabDefinition {
  constructor(
    private readonly buildTree: () => import("@semio-tech/framework-playground-core").UiTreeNode,
    private readonly commandBus: CommandBus,
  ) {
    super();
  }

  buildTab(): SidePanelTabConfig {
    return {
      id: PUZZLE_5D_PLAY_KINDS_TAB_ID,
      icon: createIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const treeNode = this.buildTree();
        const config = uiTreeNodeToTreePanelConfig(treeNode, this.commandBus);
        return {
          ...config,
          dragAndDropController: puzzle5dFixturePaletteTreeDragController(collectUiTreeItemDragData(treeNode.sections)),
        };
      }),
    };
  }
}

class Puzzle5dPlayInspectorPanelDefinition extends PureSidePanelTabDefinition {
  constructor(
    private readonly buildTree: () => import("@semio-tech/framework-playground-core").UiTreeNode,
    private readonly commandBus: CommandBus,
  ) {
    super();
  }

  buildTab(): SidePanelTabConfig {
    return {
      id: "puzzle-5d-play-inspector",
      icon: createIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => uiTreeNodeToTreePanelConfig(this.buildTree(), this.commandBus)),
    };
  }
}
//#endregion 🔖DetailsPanel

//#region 🔖Surfaces
function Puzzle5d2dSurfaceHost({ node }: { readonly node: UiPuzzle2dHostSurfaceNode }): React.ReactElement {
  const { controller, snapshot } = usePuzzle5dPlaySnapshot();
  const store = usePuzzle5dPlayStore();
  const bindingValid =
    node.controllerId === PUZZLE_5D_PLAY_CONTROLLER_ID &&
    node.surfaceId === PUZZLE_5D_PLAY_2D_SURFACE_ID &&
    node.paneId === PUZZLE_5D_PLAY_2D_WINDOW_ID &&
    Boolean(controller && snapshot?.fixture2d);
  const flatCatalogs = reactHostPort.useMemo(() => project2dKindCatalogs(store.read().kindCatalogs), [store, snapshot?.manifestLabel]);
  const controllerRef = reactHostPort.useRef(controller);
  const storeRef = reactHostPort.useRef(store);
  const activeToolRef = reactHostPort.useRef(snapshot?.activeTool ?? "select");
  controllerRef.current = controller;
  storeRef.current = store;
  activeToolRef.current = snapshot?.activeTool ?? "select";
  const onLodChange = reactHostPort.useCallback((lod: Puzzle2dDrawLodKind) => {
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set2dLodTag", { lod });
  }, []);
  const onSelect = reactHostPort.useCallback((snap: { readonly ids: readonly string[] }) => {
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set2dSelection", { ids: snap.ids });
  }, []);
  const onConnect = reactHostPort.useCallback(() => {
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "note2dConnect");
  }, []);
  const onProximityConnect = reactHostPort.useCallback(() => {
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "note2dProximity");
  }, []);
  const onBrushPlace = reactHostPort.useCallback((payload: Parameters<typeof puzzle5dBrushPlacementFromFlat>[0]) => {
    if (puzzle5dCommitBrushPlacementToPlay(storeRef.current, payload)) {
      controllerRef.current?.setBrushEngagementPossibles([]);
    }
  }, []);
  const onBrushCandidates = reactHostPort.useCallback((payload: Puzzle2dBrushCandidatesPayload) => {
    if (activeToolRef.current !== "brush") {
      controllerRef.current?.setBrushEngagementPossibles([]);
      return;
    }
    controllerRef.current?.setBrushEngagementPossibles(puzzle5dBrushCandidateRows(payload, flatCatalogs));
  }, [flatCatalogs]);
  const onDelete = reactHostPort.useCallback(() => {
    const selection = storeRef.current.getSnapshot().selection;
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set2dSelection", { ids: [...selection.partIds] });
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set3dSelection", {
      objectIds: selection.partIds.length > 0 ? [selection.partIds[0]!] : [],
    });
  }, []);
  const onFixtureDrop = reactHostPort.useCallback((detail: Puzzle2dFixtureDropDetail) => {
    const partId = storeRef.current.applyPaletteNodeDrop(detail);
    if (!partId) {
      return;
    }
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set2dSelection", { ids: [partId] });
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set3dSelection", { objectIds: [partId] });
  }, []);
  if (!bindingValid || !controller || !snapshot) {
    return <div className="p-2 text-xs text-muted-foreground">Invalid puzzle 5d 2d binding</div>;
  }
  return (
    <FiveD
      mode="2d"
      instanceId="play-2d"
      activeTool={snapshot.activeTool}
      suggestionOffset={snapshot.suggestionOffset}
      puzzle2d={{
        onLodChange,
        onSelect,
        onConnect,
        onProximityConnect,
        onBrushPlace,
        onBrushCandidates,
        onDelete,
        fixtureDragDrop: true,
        onFixtureDrop,
        selectionMethod: snapshot.selectionMethod,
        selectionMode: snapshot.selectionMode,
        ...snapshot.lod2dProps,
      }}
    />
  );
}

function Puzzle5d3dSurfaceHost({ node }: { readonly node: UiPuzzle3dHostSurfaceNode }): React.ReactElement {
  const { controller, snapshot } = usePuzzle5dPlaySnapshot();
  const store = usePuzzle5dPlayStore();
  const modelPartCount = reactHostPort.useSyncExternalStore(store.subscribe, () => store.read().parts.length, () => store.read().parts.length);
  const fillSeedRef = reactHostPort.useRef(1);
  const fillBaseCaptureRef = reactHostPort.useRef<ReturnType<Puzzle5dStore["read"]> | null>(null);
  const fillPrepareTimerRef = reactHostPort.useRef<ReturnType<typeof setTimeout> | null>(null);
  const fillSessionPreparedRef = reactHostPort.useRef(false);
  const prevActiveToolRef = reactHostPort.useRef(snapshot?.activeTool);
  const bindingValid =
    node.controllerId === PUZZLE_5D_PLAY_CONTROLLER_ID &&
    node.surfaceId === PUZZLE_5D_PLAY_3D_SURFACE_ID &&
    Boolean(controller && snapshot?.fixture3d && snapshot.fixture2d);
  const controllerRef = reactHostPort.useRef(controller);
  const storeRef = reactHostPort.useRef(store);
  const brushOverlapBudgetRef = reactHostPort.useRef(snapshot?.brushOverlapBudget ?? 0);
  controllerRef.current = controller;
  storeRef.current = store;
  brushOverlapBudgetRef.current = snapshot?.brushOverlapBudget ?? 0;
  reactHostPort.useEffect(() => {
    if (!bindingValid) return;
    const urls = [...new Set(store.read().parts.flatMap((part) => (part["3d"] ? [part["3d"].meshUrl] : [])))];
    for (const url of urls) sceneHostPort.drei.useGLTF.preload(url);
  }, [bindingValid, modelPartCount, store]);
  reactHostPort.useLayoutEffect(() => {
    const prev = prevActiveToolRef.current;
    prevActiveToolRef.current = snapshot?.activeTool;
    if (snapshot?.activeTool === "fill" && prev !== "fill") {
      fillBaseCaptureRef.current = structuredClone(store.read());
      fillSessionPreparedRef.current = false;
      fillSeedRef.current = (Date.now() ^ Math.floor(Math.random() * 0x7fffffff)) >>> 0;
    }
    if (snapshot?.activeTool !== "fill") {
      fillBaseCaptureRef.current = null;
      fillSessionPreparedRef.current = false;
    }
  }, [snapshot?.activeTool, store]);
  const volumeKindCatalogs = reactHostPort.useMemo(
    () => project3dKindCatalogs(snapshot?.kindCatalogs ?? snapshot?.sharedKinds.kindCatalogs),
    [snapshot?.kindCatalogs, snapshot?.sharedKinds.kindCatalogs],
  );
  reactHostPort.useEffect(() => {
    if (!bindingValid || snapshot?.activeTool !== "fill" || !snapshot.fixture3d) {
      return;
    }
    const urls = brushMeshUrlsForFillSession(snapshot.fixture3d, volumeKindCatalogs, snapshot.model.kindCompatibility);
    for (const url of urls) {
      if (isLoadableMeshUrl(url)) {
        sceneHostPort.drei.useGLTF.preload(url);
      }
    }
  }, [bindingValid, snapshot?.activeTool, snapshot?.fixture3d, snapshot?.model.kindCompatibility, volumeKindCatalogs]);
  const onSelect = reactHostPort.useCallback((selection: { readonly objectIds: readonly string[] }) => {
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set3dSelection", { objectIds: selection.objectIds });
  }, []);
  const onConnect = reactHostPort.useCallback(() => {
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "note3dConnect");
  }, []);
  const onProximityConnect = reactHostPort.useCallback(() => {
    controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "note3dProximity");
  }, []);
  const onBrushPlace = reactHostPort.useCallback((payload: Parameters<typeof puzzle5dBrushPlacementFromVolume>[0]) => {
    if (puzzle5dCommitVolumeBrushPlacementToPlay(storeRef.current, payload)) {
      controllerRef.current?.setBrushEngagementPossibles([]);
    }
  }, []);
  const onFillMeshesReady = reactHostPort.useCallback(() => {
    if (fillPrepareTimerRef.current !== null) {
      clearTimeout(fillPrepareTimerRef.current);
    }
    fillPrepareTimerRef.current = setTimeout(() => {
      fillPrepareTimerRef.current = null;
      const base = fillBaseCaptureRef.current;
      if (!base || fillSessionPreparedRef.current) {
        return;
      }
      const activeStore = storeRef.current;
      const activeController = controllerRef.current;
      activeStore.setFillBuildDone(false);
      const sequence = buildPuzzle5dFillSequence({
        model: base,
        seed: fillSeedRef.current,
        overlapBudget: brushOverlapBudgetRef.current,
        meshRootForUrl: puzzle3dBrushMeshRootForFill,
      });
      activeStore.prepareFillSession(sequence, base, fillSeedRef.current);
      activeStore.setFillBuildDone(true);
      fillSessionPreparedRef.current = true;
      if (sequence.length > 0) {
        activeController?.run("setFillCount", { count: 1 });
      }
    }, 0);
  }, []);
  reactHostPort.useEffect(
    () => () => {
      if (fillPrepareTimerRef.current !== null) {
        clearTimeout(fillPrepareTimerRef.current);
      }
    },
    [],
  );
  const onFixtureDrop = reactHostPort.useCallback(
    (detail: Puzzle3dFixtureDropDetail) => {
      const sceneFixture = project3d(storeRef.current.read());
      const result = resolvePuzzle3dFixtureDrop(detail, volumeKindCatalogs, sceneFixture);
      if (result.kind !== "palette-object") {
        return;
      }
      const partId = storeRef.current.applyPaletteObjectDrop(result.object);
      if (!partId) {
        return;
      }
      controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set2dSelection", { ids: [partId] });
      controllerRef.current?.commandBus.dispatch(PUZZLE_5D_PLAY_CONTROLLER_ID, "set3dSelection", { objectIds: [partId] });
    },
    [volumeKindCatalogs],
  );
  if (!bindingValid || !controller || !snapshot?.fixture3d) {
    return <div className="p-2 text-xs text-muted-foreground">Invalid puzzle 5d 3d binding</div>;
  }
  return (
    <FiveD
      mode="3d"
      instanceId="play-3d"
      activeTool={snapshot.activeTool}
      brushOverlapBudget={snapshot.brushOverlapBudget}
      gumballConfig={snapshot.gumballConfig}
      puzzle3d={{
        ...snapshot.lod3dProps,
        onSelect,
        onConnect,
        onProximityConnect,
        onBrushPlace,
        onFillMeshesReady,
        fixtureDragDrop: true,
        onFixtureDrop,
        selectionMethod: snapshot.selectionMethod,
        selectionMode: snapshot.selectionMode,
      }}
    />
  );
}
//#endregion 🔖Surfaces

function Puzzle5dPlayJackSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): React.ReactElement {
  const { controller } = usePuzzle5dPlaySnapshot();
  void controller?.getHoverEpoch();
  void controller?.getSelectEpoch();
  const document = controller?.getWriterDocumentJack() ?? createWriterDocument({ id: "puzzle-5d-jack", languageId: "jack", text: "" });
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    puzzle5dPlayControllerRef.current?.run("setJackHover", { offset });
  }, []);
  const onSelectionChange = reactHostPort.useCallback((range: { start: number; end: number }) => {
    puzzle5dPlayControllerRef.current?.run("setJackSelect", range);
  }, []);
  return (
    <WriterCanvas
      document={document}
      className="h-full"
      onHoverChange={onHoverChange}
      onSelectionChange={onSelectionChange}
      externalHoverOccurrences={controller?.getJackHoverOccurrences()}
      externalHoverOccurrencesSignal={controller?.getHoverEpoch()}
      externalSelectionOccurrences={controller?.getJackSelectOccurrences()}
      externalSelectionOccurrencesSignal={controller?.getSelectEpoch()}
    />
  );
}

//#region 🔖Mount
let topologyPlayChromeRegistered = false;

/** @emoji 🧊 Registers topology play flat+volume surface hosts (called from `@semio-tech/framework-playground-renderer-react`). */
export function registerPuzzle5dPlaySurfaceHosts(): void {
  if (topologyPlayChromeRegistered) return;
  topologyPlayChromeRegistered = true;
  registerUiPuzzle2dSurfaceHost(PUZZLE_5D_PLAY_2D_SURFACE_ID, Puzzle5d2dSurfaceHost);
  registerUiPuzzle3dSurfaceHost(PUZZLE_5D_PLAY_3D_SURFACE_ID, Puzzle5d3dSurfaceHost);
  registerUiWriterSurfaceHost(PUZZLE_5D_PLAY_JACK_SURFACE_ID, Puzzle5dPlayJackSurfaceHost);
  registerTabIcon(PUZZLE_5D_PLAY_ICON_KINDS, "tags");
  registerWindowBody(PUZZLE_5D_PLAY_2D_BODY_KEY, buildPuzzle5d2dDeclarativeBody);
  registerWindowBody(PUZZLE_5D_PLAY_3D_BODY_KEY, buildPuzzle5d3dDeclarativeBody);
  registerWindowBody(PUZZLE_5D_PLAY_JACK_BODY_KEY, buildPuzzle5dJackDeclarativeBody);
}

function Puzzle5dPlayChrome({
  runtime,
  playgroundKeybindings,
}: {
  readonly runtime: Platform;
  readonly playgroundKeybindings?: readonly { readonly key: string; readonly controllerId: string; readonly command: string }[];
}): React.ReactElement {
  const generation = reactHostPort.useSyncExternalStore(
    (listener) => runtime.subscribe(listener),
    () => runtime.generation,
    () => 0,
  );
  void generation;
  const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController | undefined;
  puzzle5dPlayControllerRef.current = controller ?? null;
  const snapshot = controller?.getSnapshot() ?? null;
  const bus = runtime.commandBus;
  const puzzle5dStore = controller?.puzzle5dStore;
  const snapshotKey = snapshot
    ? `${snapshot.manifestLabel ?? ""}\u0001${snapshot.selection.partIds.join(",")}\u0001${snapshot.selection.gripIds.join(",")}`
    : "";
  const workbenchTabs = reactHostPort.useMemo(
    () =>
      snapshot && controller
        ? [
            new Puzzle5dPlayHierarchyPanelDefinition(() => buildPuzzle5dPlayHierarchySections(snapshot), bus).resolveTab(),
            new Puzzle5dPlayKindsPanelDefinition(() => buildPuzzle5dPlayKindsTree(snapshot), bus).resolveTab(),
          ]
        : [],
    [snapshot, snapshotKey, controller, bus, puzzle5dStore],
  );
  const detailTabs = reactHostPort.useMemo(
    () => (snapshot ? [new Puzzle5dPlayInspectorPanelDefinition(() => buildPuzzle5dPlayInspectorTreePanel(snapshot), bus).resolveTab()] : []),
    [snapshot, snapshotKey, bus],
  );
  const shell = (
    <PlaygroundView
      runtime={runtime}
      defaultAppId={PUZZLE_5D_PLAY_APP_ID}
      playgroundKeybindings={playgroundKeybindings}
      augmentPanelTabs={{ workbench: workbenchTabs, details: detailTabs }}
    />
  );
  if (!controller) {
    return shell;
  }
  const puzzle5dBridge = controller.getStore(PUZZLE_5D_PLAY_STORE_ID) as Puzzle5dStoreBridge | undefined;
  const storeForProvider = puzzle5dBridge?.inner ?? controller.puzzle5dStore;
  return (
    <StoreProvider store={storeForProvider}>
      <Puzzle5dBrushPairedSync />
      <Puzzle5dPlayHostBridgeInstaller controller={controller} store={storeForProvider} />
      {shell}
    </StoreProvider>
  );
}

/** @emoji 🚀 Mounts puzzle 5d play chrome for a {@link Playground}. */
export function mountPuzzle5dPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<Puzzle5dPlayChrome runtime={playground.runtime} playgroundKeybindings={playground.keybindings} />, rootId);
}

const topologyPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerPuzzle5dPlaySurfaceHosts,
  mount: mountPuzzle5dPlayChrome,
};

/** @emoji 🛝 Puzzle 5d play entry: register hosts, bodies, mount chrome (from `puzzle/5d/play/index.ts`). */
export function boot5dPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, topologyPlayChromeBoot, rootId);
}

//#endregion 🔖Mount

// #endregion 🛝PlayHost
//#endregion 🔖Puzzle5dPlayHost