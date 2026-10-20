/** 🧱️ The flow surface's refs and state: session, scheduler and canvas refs, canvas generation, context menu, wire refusal, overlay state, spotlight state, the app catalogue and scene refs, the introduction resolver registration and the geometry probe.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | FlowGraphCanvasHost}: `useFlowSurfaceState` runs the original statements in order and returns the bindings later hooks and the render read; the component merges them into the shared core object.
 */

import type { DagWireTypeRefusal } from "@semio-tech/framework/infinite/board/schema/dag-input";
import * as React from "../../⚛️react-port/🟦️.ts";
import {
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState
} from "../../⚛️react-port/🟦️.ts";
import { createDemandFrameScheduler } from "@semio-tech/framework/canvas/react-renderer";
import { registerIntroductionSurfaceResolver, useLabel, type CanvasPickTarget } from "@semio-tech/ui-react";
import {
  windowElementId,
  type ActionDescriptor,
  type ContextMenuItemSpec,
  type NodeGraphScene,
  type PluginContextMenuRequest
} from "@semio-tech/framework";
import { type SurfaceContextMenuResult } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/surface-menu";
import { useAppCatalogue, WindowInstanceIdContext } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/scene-host-contract";
import { type FlowWasmSession } from "@semio-tech/framework-os/modules/renderer/engine/elements/WasmSessionLoader";
import { DEFAULT_NODE_GRAPH_VIEWPORT, type GraphKeyboardPort, usePortTypeLabels, wireRefusalLabelOptions } from "../🧰️graph-core/🟦️.ts";
import { type FlowCatalogueSection, type FlowSpotlightState } from "../🪄️catalogue-spotlight/🟦️.tsx";
import { computeDagMarqueeOverlay, dagIntroductionResolver, type FlowGraphSurfaceProbe, parseDagSelectionUnionBoundsScreen } from "../🖼️dag-overlays/🟦️.tsx";
//#region 🔖️NodeGraphHost
//#region 🔖️flow-graph-canvas-host
//#region FlowGraphCanvasHost

/** 🔮️ The closure bindings an earlier hook reads before a later hook declares them. The declaring hook assigns its
 * slot right after the declaration, so a callback built earlier reads the same binding it read as a closure variable. */
export type FlowForwardSlots = {
  openSpotlightAtClient: (clientX: number, clientY: number, target: HTMLElement) => void;
};

export type FlowGraphCanvasHostProps = {
  readonly scene: NodeGraphScene;
  readonly surfaceId: string;
  readonly controllerId: string;
  readonly editable: boolean;
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
  readonly onAction: (action: ActionDescriptor) => void;
  readonly keyboardPort: React.MutableRefObject<GraphKeyboardPort | null>;
};

export type FlowCoreAfterFlowSurfaceState = FlowForwardSlots & ReturnType<typeof useFlowSurfaceState>;

export function useFlowSurfaceState({
  scene,
  surfaceId,
  controllerId,
  editable,
  requestContextMenu,
  onAction,
  keyboardPort,
}: FlowGraphCanvasHostProps) {
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const sessionRef = useRef<FlowWasmSession | null>(null);
  const schedulerRef = useRef<ReturnType<typeof createDemandFrameScheduler> | null>(null);
  const surfaceReadyRef = useRef(false);
  const drewOnceRef = useRef(false);
  const gpuCanvasRef = useRef<HTMLCanvasElement | null>(null);
  const labelCanvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [canvasGeneration, setCanvasGeneration] = useState(0);
  const framedGraphSignatureRef = useRef<string | null>(null);
  const attachedSurfaceRef = useRef<{ readonly surface: number; readonly surfaceGeneration: number; readonly presentsOnGpu: boolean } | null>(null);
  const replaceUnpresentableCanvasRef = useRef<() => void>(() => {});
  const [contextMenu, setContextMenu] = useState<SurfaceContextMenuResult & {
    readonly x: number;
    readonly y: number;
    readonly widgetId?: string;
  } | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.flow");
  const [wireRefusal, setWireRefusal] = useState<DagWireTypeRefusal | null>(null);
  const portTypeLabels = usePortTypeLabels();
  const wireRefusalText = useLabel("ui.nodeGraph.incompatiblePorts", wireRefusalLabelOptions(wireRefusal, portTypeLabels));
  const [selectionBounds, setSelectionBounds] = useState<ReturnType<typeof parseDagSelectionUnionBoundsScreen>>(null);
  const [marquee, setMarquee] = useState<ReturnType<typeof computeDagMarqueeOverlay>>(null);
  const [labelStateJson, setLabelStateJson] = useState("{}");
  const [sliderStateJson, setSliderStateJson] = useState("{}");
  const [containerSize, setContainerSize] = useState({ w: 800, h: 600 });
  const [sessionReady, setSessionReady] = useState(false);
  const overlayRequestRef = useRef(0);
  const [spotlight, setSpotlight] = useState<FlowSpotlightState | null>(null);
  const [spotlightSections, setSpotlightSections] = useState<readonly FlowCatalogueSection[]>([]);
  const appCatalogue = useAppCatalogue();
  const appCatalogueRef = useRef(appCatalogue);
  appCatalogueRef.current = appCatalogue;
  const pickTargetsRef = useRef<readonly CanvasPickTarget[]>([]);
  const sceneSignature = useMemo(() => JSON.stringify(scene), [scene]);
  const sceneRef = useRef(scene);
  sceneRef.current = scene;

  useEffect(() => {
    if (!windowInstanceId) return;
    return registerIntroductionSurfaceResolver(windowElementId(windowInstanceId), dagIntroductionResolver(sessionRef, containerRef));
  }, [windowInstanceId]);

  useEffect(() => {
    const resolver = dagIntroductionResolver(sessionRef, containerRef);
    const host = window as unknown as { __semioFlowGraphProbe?: Record<string, FlowGraphSurfaceProbe> };
    const registry = (host.__semioFlowGraphProbe ??= {});
    registry[surfaceId] = {
      entity: (domain, id) => resolver.entity?.(domain, id) ?? null,
      nodeIds: () => (sceneRef.current.nodes ?? []).map((node) => node.id),
      nodeLayout: () => Object.fromEntries((sceneRef.current.nodes ?? []).map((node) => [node.id, { x: node.x, y: node.y }])),
      viewport: () => sceneRef.current.viewport ?? DEFAULT_NODE_GRAPH_VIEWPORT,
      hostSnapshotJson: () => sceneRef.current.hostSnapshotJson ?? null,
      rect: () => {
        const measured = containerRef.current?.getBoundingClientRect();
        return measured ? { x: measured.x, y: measured.y, width: measured.width, height: measured.height } : null;
      },
    };
    return () => {
      delete registry[surfaceId];
    };
  }, [surfaceId]);

  return {
    scene, surfaceId, controllerId, editable, requestContextMenu, onAction, keyboardPort, windowInstanceId, sessionRef,
    schedulerRef, surfaceReadyRef, drewOnceRef, gpuCanvasRef, labelCanvasRef, containerRef, canvasGeneration,
    setCanvasGeneration, framedGraphSignatureRef, attachedSurfaceRef, replaceUnpresentableCanvasRef, contextMenu,
    setContextMenu, contextMenuTitleLabel, wireRefusal, setWireRefusal, wireRefusalText, selectionBounds,
    setSelectionBounds, marquee, setMarquee, labelStateJson, setLabelStateJson, sliderStateJson, setSliderStateJson,
    containerSize, setContainerSize, sessionReady, setSessionReady, overlayRequestRef, spotlight, setSpotlight,
    spotlightSections, setSpotlightSections, appCatalogue, appCatalogueRef, pickTargetsRef, sceneSignature, sceneRef
  };
}
//#endregion FlowGraphCanvasHost
//#endregion 🔖️flow-graph-canvas-host
//#endregion 🔖️NodeGraphHost
