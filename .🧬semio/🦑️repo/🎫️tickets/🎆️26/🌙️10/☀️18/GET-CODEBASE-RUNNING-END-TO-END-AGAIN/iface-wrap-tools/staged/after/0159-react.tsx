/** 🔶️ The window instance id phase of WasmGraphSurface. */
import { useCallback, useContext, useEffect, useMemo, useRef, useState } from "../../../⚛️react-port/🟦️.ts";
import { type WasmCanvasSession } from "@semio-tech/framework/canvas/react-renderer";
import { registerIntroductionSurfaceResolver, useCanvasAppearanceSync, useLabel, useShellScopeOptional } from "@semio-tech/ui-react";
import { GestureRecognizer, windowElementId, type ActionDescriptor, type NodeGraphScene } from "@semio-tech/framework";
import { useMapContextMenuSpecs, useShellContextMenuFallback, type SurfaceContextMenuResult } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/surface-menu";
import { WindowInstanceIdContext } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/scene-host-contract";
import { createGraphSession } from "@semio-tech/framework-os/modules/renderer/engine/elements/WasmSessionLoader";
import { DEFAULT_NODE_GRAPH_VIEWPORT, type FrameworkGraphSession, publishNodeGraphHover, publishNodeGraphSelection, syncOptionalGraphCanvasTheme } from "../../🧰️graph-core/🟦️.ts";
import { computeDagMarqueeOverlay, dagIntroductionResolver, paintDagLabelOverlays, parseDagNodeIdArray, parseDagSelectionUnionBoundsScreen, sceneToSyncPack, useGraphSliderLanes } from "../../🖼️dag-overlays/🟦️.tsx";

/** 🔷 Builds windowInstanceId, sessionRef, labelCanvasRef, containerRef of WasmGraphSurface. */
export function useWindowInstanceIdState({ scene, onAction, controllerId, surfaceId }: { readonly scene: NodeGraphScene; readonly onAction: (action: ActionDescriptor) => void; readonly controllerId: string; readonly surfaceId: string }) {
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const sessionRef = useRef<FrameworkGraphSession | null>(null);
  const labelCanvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.node");
  const [selectionBounds, setSelectionBounds] = useState<ReturnType<typeof parseDagSelectionUnionBoundsScreen>>(null);
  const [marquee, setMarquee] = useState<ReturnType<typeof computeDagMarqueeOverlay>>(null);
  const [overlaySize, setOverlaySize] = useState({ w: 0, h: 0 });
  const [sliderStateJson, setSliderStateJson] = useState("{}");
  const scenePack = useMemo(() => sceneToSyncPack(scene), [scene]);
  const sceneRef = useRef(scene);
  sceneRef.current = scene;

  const dispatch = useCallback((action: string, args?: Record<string, unknown>) => onAction({ controllerId, action, args: { surfaceId, ...args } }), [controllerId, onAction, surfaceId]);
  const dispatchRef = useRef(dispatch);
  dispatchRef.current = dispatch;
  const { sliderLane } = useGraphSliderLanes(surfaceId, dispatchRef);
  const [gestureRecognizer] = useState(() => new GestureRecognizer());
  const pinchLogScaleRef = useRef(0);

  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();

  const paintOverlays = useCallback(() => {
    const session = sessionRef.current;
    const labelCanvas = labelCanvasRef.current;
    const container = containerRef.current;
    if (!session || !labelCanvas || !container) return;
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    try {
      paintDagLabelOverlays(session.labelOverlayPaintStateJson(), labelCanvas, rect.width, rect.height, dpr, {
        hoveredId: session.hoveredNodeId() ?? null,
        selectedIds: parseDagNodeIdArray(session.selectedNodeIdsJson()),
        preselect: { ids: [], removedIds: [] },
        dimmedIds: [],
        highlightIds: sceneRef.current.highlighted ?? [],
      });
    } catch {}
    setSelectionBounds(parseDagSelectionUnionBoundsScreen(session.selectionUnionBoundsScreenJson()));
    setMarquee(computeDagMarqueeOverlay(session.selectionPreviewPointsJson(), session.selectionPreviewCrossing(), session.selectionPreviewMethod?.() ?? "rectangle"));
    try {
      const nextSliderJson = session.sliderOverlayStateJson();
      setSliderStateJson((prev) => (prev === nextSliderJson ? prev : nextSliderJson));
    } catch {}
    setOverlaySize((prev) => (prev.w === rect.width && prev.h === rect.height ? prev : { w: rect.width, h: rect.height }));
    try {
      container.setAttribute("data-viewport-camera-json", JSON.stringify(session.viewport()));
      container.setAttribute("data-session-selection-json", session.selectedNodeIdsJson());
    } catch {}
  }, []);

  useEffect(() => {
    try {
      sessionRef.current?.syncFromScenePack?.(scenePack);
      paintOverlays();
    } catch (error) {
    }
  }, [scenePack, paintOverlays]);

  const onSessionReady = useCallback(
    (session: WasmCanvasSession) => {
      sessionRef.current = session as FrameworkGraphSession;
      syncOptionalGraphCanvasTheme(sessionRef.current);
      try {
        sessionRef.current.syncFromScenePack?.(scenePack);
        paintOverlays();
      } catch (error) {
      }
    },
    [scenePack, paintOverlays],
  );

  const wasmGraphSurfaceShellScope = useShellScopeOptional();
  useCanvasAppearanceSync(
    () => {
      syncOptionalGraphCanvasTheme(sessionRef.current);
      try {
        sessionRef.current?.renderFrame();
      } catch {}
      paintOverlays();
    },
    true,
    wasmGraphSurfaceShellScope?.rootRef.current ?? undefined,
  );

  const [wasmSession, setWasmSession] = useState<FrameworkGraphSession | null>(null);

  useEffect(() => {
    let cancelled = false;
    void createGraphSession().then((session) => {
      if (!cancelled) setWasmSession(session as FrameworkGraphSession);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    if (!windowInstanceId) return;
    return registerIntroductionSurfaceResolver(windowElementId(windowInstanceId), dagIntroductionResolver(sessionRef, containerRef));
  }, [windowInstanceId]);

  const sessionFactory = useCallback(() => {
    if (wasmSession) return wasmSession;
    return {
      attachCanvas: async () => undefined,
      setSize: () => {},
      renderFrame: () => {},
      syncFromSceneJson: () => {},
      syncFromScenePack: () => {},
      setCanvasThemeJson: () => {},
      pointerDownScreen: () => {},
      pointerMoveScreen: () => {},
      pointerUpScreen: () => {},
      pointerCancelScreen: () => {},
      wheelScreen: () => {},
      labelOverlayPaintStateJson: () => '{"labels":[]}',
      sliderOverlayStateJson: () => "{}",
      selectionUnionBoundsScreenJson: () => "{}",
      selectionPreviewPointsJson: () => "[]",
      selectionPreviewCrossing: () => false,
      selectionPreviewMethod: () => "rectangle",
      selectedNodeIdsJson: () => "[]",
      hoveredNodeId: () => null,
      hoveredChannelJson: () => '{"channel":null,"refusal":null}',
      syncInteraction: () => {},
      viewport: () => scene.viewport ?? DEFAULT_NODE_GRAPH_VIEWPORT,
      pickTargetsAtScreenJson: () => "[]",
      setHover: () => {},
      setHoverChannel: () => {},
      alignSelection: () => {},
      takeGraphEditsJson: () => '{"operations":[]}',
      takePendingOpenInstanceId: () => null,
    } satisfies FrameworkGraphSession;
  }, [scene.viewport, wasmSession]);

  const emitInteractionState = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const nodeIds = JSON.parse(session.selectedNodeIdsJson()) as string[];
      publishNodeGraphSelection(dispatch, sceneRef.current.interactionDomain, { nodeIds });
      const hovered = session.hoveredNodeId();
      publishNodeGraphHover(dispatch, sceneRef.current.interactionDomain, hovered);
      const openId = session.takePendingOpenInstanceId?.();
      if (openId) dispatch("openInstance", { instanceId: openId });
    } catch {}
    paintOverlays();
  }, [dispatch, paintOverlays]);
  return { sessionRef, paintOverlays, emitInteractionState, dispatch, containerRef, sceneRef, windowInstanceId, mapContextMenu, shellContextMenuFallback, setContextMenu, sessionFactory, onSessionReady, labelCanvasRef, selectionBounds, marquee, gestureRecognizer, pinchLogScaleRef, sliderStateJson, overlaySize, sliderLane, contextMenuTitleLabel, contextMenu };
}
