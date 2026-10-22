// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/NodeGraph/component.tsx
/** 🕸️ `NodeGraph` — the node-graph/flow program scene host: wasm dag-engine canvas surface,
 * the flow-engine (React Flow) canvas host, the catalogue double-click spotlight, label/slider/marquee
 * canvas overlays shared by both engines, and the SSR-safe `Diagram`-based fallback. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { decodeDagSelectionJson } from "@semio-tech/framework/infinite/board/io/text/dag-input";
import type { DagSelectionDomains } from "@semio-tech/framework/infinite/board/schema/dag-input";
import * as React from "../⚛️react-port/🟦️.ts";
import {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState
} from "../⚛️react-port/🟦️.ts";
import {
  CanvasPickMenu,
  cn,
  ContextMenuController,
  DiagramLiveRegion,
  glassClass,
  pickMostSpecificCanvasTarget,
  SelectionMarquee,
  useDiagramTranslate,
  useLabel,
  type CanvasPickTarget
} from "@semio-tech/ui-react";
import {
  nodeGraphActions,
  type ComponentSceneHostProps,
  type NodeGraphScene,
  type PluginContextMenuSurfaceTarget,
  type PresencePeer
} from "@semio-tech/framework";
import { mapContextMenuSpecs, openSurfaceContextMenu } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/surface-menu";
import { parseJsonArray, selectionGroupsFromDomains } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/scene-host-contract";
import { isFlowGraphScene } from "@semio-tech/framework-os/modules/renderer/engine/elements/WasmSessionLoader";
import { useUIFindSafe } from "@semio-tech/framework-os/modules/renderer/engine/elements/ShellSearch";
import {
  flowJsonText,
  type GraphKeyboardPort,
  graphPinchWheelPlan,
  handleGraphKeyboard,
  nodeGraphEditRows,
  nodeGraphInputDecodeControl,
  publishNodeGraphSelection
} from "./🧰️graph-core/🟦️.ts";
import { FlowSpotlight } from "./🪄️catalogue-spotlight/🟦️.tsx";
import { DiagramGraphFallback } from "./🔷️diagram-node/🟦️.tsx";
import { WasmGraphSurface } from "./🧭️wasm-graph-surface/🟦️.tsx";
import { GraphSliderOverlays, parseDagMinimapWidgetOccluder, SelectionAlignChrome } from "./🖼️dag-overlays/🟦️.tsx";
import { issueFlowGestureStep, observeFlowTask, readFlowTask } from "./🔄️flow-session-sync/🟦️.ts";
import { flowGestureIsCameraPan, flowSurfaceCanvasKey } from "./🧷️flow-canvas-contract/🟦️.ts";
import { useFlowSurfaceState, type FlowGraphCanvasHostProps } from "./🧱️flow-surface-state/🟦️.ts";
import { useFlowDispatchAndGestures } from "./🎬️flow-gestures/🟦️.ts";
import { useFlowOverlayAndRender } from "./🎨️flow-overlay-render/🟦️.ts";
import { useFlowCanvasSurface } from "./📏️flow-canvas-surface/🟦️.ts";
import { useFlowInteractionPublication } from "./🧾️flow-interaction-lane/🟦️.ts";
import { useFlowSessionAttach } from "./🔌️flow-session-attach/🟦️.ts";
import { useFlowPickAndSpotlight, type FlowCoreAfterFlowPickAndSpotlight } from "./🎯️flow-pick-spotlight/🟦️.ts";
// #endregion 🔌️Adapters

export {
  flowAddWidgetArgs,
  GRAPH_WHEEL_ZOOM_NOTCH,
  type GraphPinchWheelCall,
  graphPinchWheelPlan,
  nodeGraphConnectionIsValid,
  nodeGraphEditRows,
  nodeGraphHoverActionArgs,
  nodeGraphPickChannel,
  nodeGraphSelectionActionArgs,
  nodeGraphViewportActionArgs,
  parseNodeGraphSessionViewport,
  portValueTypes,
  portValueTypesCompatible,
  resolveHostSnapshotWidgetInstanceId,
  wireRefusalLabelOptions
} from "./🧰️graph-core/🟦️.ts";

export {
  type CatalogueAppDragPayload,
  catalogueGhostDescriptorJson,
  type FlowCatalogueGroup,
  type FlowCatalogueItem,
  flowCatalogueItemDescriptor,
  type FlowCatalogueSection,
  flowRankCatalogueSuggestions,
  flowSpotlightSuggestionListScrollClass,
  parseCatalogueAppDragPayload
} from "./🪄️catalogue-spotlight/🟦️.tsx";

export {
  workflowNodesToDiagramNodes
} from "./🔷️diagram-node/🟦️.tsx";

export {
  alignModeToDag,
  computeDagMarqueeOverlay,
  DAG_CONTENT_FIT_PADDING_PX,
  DAG_CONTENT_FRAMED_MIN_COVERAGE,
  DAG_CONTENT_REFIT_MAX_COVERAGE,
  DAG_LABEL_ELLIPSIS,
  type DagCameraState,
  dagContentBounds,
  type DagContentBounds,
  dagContentCoverage,
  dagElementInteractionChrome,
  dagEllipsizeByMeasure,
  dagFitCamera,
  type DagLabelOverlayInteraction,
  type DagLabelOverlayRow,
  type DagMarqueeOverlay,
  dagOverlayLabelFill,
  dagOverlayLabelFillHex,
  type DagPreselectSnapshot,
  dagScreenToWorld,
  type DagSelectionBounds,
  type DagSliderOverlayRow,
  dagSliderValueText,
  dagStartupCamera,
  dagWorldToScreen,
  type FlowGraphSurfaceProbe,
  GraphSliderOverlays,
  nodeGraphContentSignature,
  paintDagLabelOverlays,
  parseDagCameraState,
  parseDagLabelRows,
  parseDagMinimapWidgetCursor,
  parseDagMinimapWidgetOccluder,
  parseDagNodeIdArray,
  parseDagOverlayCamera,
  parseDagPreselectJson,
  parseDagSelectionUnionBoundsScreen,
  parseDagSliderOverlays,
  resizeCanvasBackingStore,
  sceneToSyncJson,
  sceneToSyncPack,
  SelectionAlignChrome
} from "./🖼️dag-overlays/🟦️.tsx";

export {
  createFlowSharedPayloadCoalescer,
  FLOW_SHARED_PAYLOAD_COALESCE_MS,
  type FlowSharedPayloadCoalescerPorts
} from "./🔄️flow-session-sync/🟦️.ts";

export {
  createNodeGraphInteractionLedger,
  type NodeGraphHoverMark,
  nodeGraphHoverMarkKey,
  nodeGraphSelectionMarkKey,
  type NodeGraphSelectionMarks
} from "./🫱️interaction-publication/🟦️.ts";

export {
  createFlowCameraGesture,
  FLOW_CAMERA_GESTURE_SETTLE_MS,
  type FlowCameraGesturePorts,
  flowGestureIsCameraPan,
  flowSurfaceCanvasKey,
  flowSurfaceRenderAllowed
} from "./🧷️flow-canvas-contract/🟦️.ts";

//#region 🔖️NodeGraphHost
//#region NodeGraphHost
//#region Helpers
// 🚪️ Exported (unlike the rest of this Helpers subregion) — `TextEditorHost` in the sibling `✏️TextEditor`
// element also needs the SSR-safe client-mount gate; shared here rather than duplicated.
export const useClient = () => {
  const [client, setClient] = useState(false);
  useEffect(() => setClient(true), []);
  return client;
};

function PresencePeersOverlay({ peers }: { readonly peers: readonly PresencePeer[] }) {
  if (peers.length === 0) return null;
  return (
    <div className={cn("pointer-events-none absolute right-2 top-2 z-panel flex max-w-[14rem] flex-col gap-1 rounded border border-border/60 px-2 py-1 text-xs shadow-sm", glassClass)} data-level="pane">
      {peers.map((peer) => (
        <div key={peer.clientId} className="flex items-center justify-between gap-2 text-muted-foreground">
          <span className="truncate font-medium text-foreground">{peer.name}</span>
          <span>{peer.selectionCount} selected</span>
        </div>
      ))}
    </div>
  );
}
/**
 * 🪜️ The graph host's root class. `isolate` is load-bearing: it keeps this host's OWN layer numbers —
 * the full-bleed pointer overlay at `z-30`, the label canvas at `z-40`, the marquee at `z-50` — inside
 * its own stacking context.
 *
 * Without it those raw numbers competed directly with the shell's z ladder (`--z-pane: 20`,
 * `🖌️ui/🎨️.css`), so the pointer overlay painted ON TOP of the window's floating pane chrome and ate
 * every click on `Window Options` (which is where the LOD picker lives), `Actions` and `Utilities`.
 * The chips stayed visible and focusable and were completely dead to a pointer, while the same chips
 * on a sibling `World3dHost` window worked — that host has no such overlay. Measured on the React
 * serve 2026-09-14: `elementsFromPoint` over the chip's centre answered the graph overlay, and a
 * programmatic `.click()` opened the rail the pointer could not reach
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 */
export const NODE_GRAPH_HOST_CLASS = "semio-node-graph-host isolate relative h-full min-h-0 w-full overflow-hidden";

/** 🎯️ What THIS graph surface has selected, hovered and highlighted, exactly as the guest published it
 * in `NodeGraphScene` — the graph twin of `🌐️World3dHost`'s `data-selection-json`.
 *
 * A node picked from the Artifact panel's outline row, from the canvas, or by a traversal chord is one
 * and the same `graph`-domain selection, and until this existed the ONLY surface in the shell that
 * published a selection lane was the 3D pane — whose ids are mesh ids (`extrude@solid`), never node ids.
 * So "selecting through the document panel does not select" could not be told from "the surface that
 * holds the selection never says so": measured on generation3d 6018, clicking
 * `panel:procedural-play-graph/height` marked its row and invoked `interactionSelect`, while the only
 * `[data-selection-json]` in the document was the preview's, with `selectedIds: []`
 * (`🗑️generated/react-reds/recon/recon.json` `doc-panel-select`). */
export function nodeGraphSurfaceSelectionDomV1(scene: NodeGraphScene | undefined): Record<string, unknown> {
  const hover = scene?.hover;
  return {
    selectedIds: [...(scene?.selection ?? [])],
    highlightedIds: [...(scene?.highlighted ?? [])],
    hoverTarget: hover?.nodeId ? { nodeId: hover.nodeId, portId: hover.portId ?? null } : null,
    editable: scene?.editable ?? true,
  };
}
//#endregion Helpers

//#region Component
export function NodeGraphHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.nodeGraph;
  const editable = scene?.editable ?? true;
  const parsedNodes = useMemo(() => scene?.nodes ?? [], [scene?.nodes]);
  const parsedEdges = useMemo(() => scene?.edges ?? [], [scene?.edges]);
  const findItems = useMemo(() => scene?.findItems ?? [], [scene?.findItems]);
  const presencePeers = useMemo(() => parseJsonArray<PresencePeer>(scene?.presencePeersJson), [scene?.presencePeersJson]);
  const isClient = useClient();
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const graphLabel = useLabel("ui.diagram.label");
  const graphRoleDescriptionLabel = useLabel("ui.diagram.roleDescription");
  const graphKeyboardHelpLabel = useLabel("ui.diagram.keyboardHelp");
  const graphNodesLabel = useLabel("ui.diagram.nodes");
  const graphEdgesLabel = useLabel("ui.diagram.edges");
  const graphKeyboardHelpId = React.useId();
  const diagramTranslate = useDiagramTranslate();
  const [keyboardFocusedNodeId, setKeyboardFocusedNodeId] = useState<string | null>(null);
  const keyboardPortRef = useRef<GraphKeyboardPort | null>(null);
  const [keyboardAnnouncement, setKeyboardAnnouncement] = useState("");
  const liveKeyboardFocusedNodeId = keyboardFocusedNodeId !== null && parsedNodes.some((record) => record.id === keyboardFocusedNodeId) ? keyboardFocusedNodeId : null;

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  const findContext = useUIFindSafe();
  const onFindItemRef = useRef<(itemId: string) => void>(() => {});
  onFindItemRef.current = (itemId: string) => {
    const mediaNode = parsedNodes.find((entry) => entry.instanceId === itemId);
    if (!mediaNode) return;
    publishNodeGraphSelection(dispatch, scene?.interactionDomain, { nodeIds: [mediaNode.id] });
    dispatch("selectInstance", { instanceId: mediaNode.instanceId! });
  };

  useEffect(() => {
    if (!findContext?.setFindItems || findItems.length === 0) return;
    findContext.setFindItems(findItems);
  }, [findContext?.setFindItems, findItems]);

  useEffect(() => {
    if (!findContext?.setOnFindItem || findItems.length === 0) return;
    findContext.setOnFindItem((itemId) => onFindItemRef.current(itemId));
    return () => findContext.setOnFindItem?.(undefined);
  }, [findContext?.setOnFindItem, findItems.length]);

  if (!scene) return <div className="semio-node-graph-empty">{emptySceneLabel}</div>;

  const useFlowEngine = isFlowGraphScene(scene.capabilitiesJson) || Boolean(scene.hostSnapshotJson);

  return (
    <div
      className={NODE_GRAPH_HOST_CLASS}
      data-surface-id={node.surfaceId}
      data-status-json={scene.statusJson ?? undefined}
      data-host-snapshot-json={scene.hostSnapshotJson ?? undefined}
      data-selection-json={JSON.stringify(nodeGraphSurfaceSelectionDomV1(scene))}
      data-diagram-focused-node={liveKeyboardFocusedNodeId ?? undefined}
      role="application"
      tabIndex={0}
      aria-roledescription={graphRoleDescriptionLabel}
      aria-label={`${graphLabel} — ${parsedNodes.length} ${graphNodesLabel}, ${parsedEdges.length} ${graphEdgesLabel}`}
      aria-describedby={graphKeyboardHelpId}
      onKeyDown={(event) =>
        handleGraphKeyboard(event, editable, scene, liveKeyboardFocusedNodeId, keyboardPortRef.current, dispatch, (focusedId, announcement) => {
          setKeyboardFocusedNodeId(focusedId);
          setKeyboardAnnouncement(announcement);
        }, diagramTranslate)
      }
    >
      <span id={graphKeyboardHelpId} className="sr-only">
        {graphKeyboardHelpLabel}
      </span>
      <DiagramLiveRegion text={keyboardAnnouncement} />
      {isClient ? (
        useFlowEngine ? (
          <FlowGraphCanvasHost scene={scene} surfaceId={node.surfaceId} controllerId={node.controllerId} editable={editable} requestContextMenu={requestContextMenu} onAction={onAction} keyboardPort={keyboardPortRef} />
        ) : (
          <WasmGraphSurface scene={scene} surfaceId={node.surfaceId} controllerId={node.controllerId} editable={editable} requestContextMenu={requestContextMenu} onAction={onAction} keyboardPort={keyboardPortRef} />
        )
      ) : (
        <DiagramGraphFallback scene={scene} node={node} editable={editable} parsedNodes={parsedNodes} parsedEdges={parsedEdges} findItems={findItems} requestContextMenu={requestContextMenu} onAction={onAction} />
      )}
      <PresencePeersOverlay peers={presencePeers} />
    </div>
  );
}
//#endregion Component
//#endregion NodeGraphHost

/** 🧷️ The one core object `FlowGraphCanvasHost` threads through its hooks: each hook reads the bindings of the hooks before it and its own bag is merged in call order, so the hooks together run the statements the single closure used to run. A binding an earlier hook reads before a later hook declares it travels through `FlowForwardSlots`, assigned the moment its declaration runs. */
type FlowCore = FlowCoreAfterFlowPickAndSpotlight;

//#region 🔖️flow-graph-canvas-host
//#region FlowGraphCanvasHost
export function FlowGraphCanvasHost(props: FlowGraphCanvasHostProps) {
  const core = {} as FlowCore;
  Object.assign(core, useFlowSurfaceState(props));
  Object.assign(core, useFlowDispatchAndGestures(core));
  Object.assign(core, useFlowOverlayAndRender(core));
  Object.assign(core, useFlowCanvasSurface(core));
  Object.assign(core, useFlowInteractionPublication(core));
  Object.assign(core, useFlowSessionAttach(core));
  Object.assign(core, useFlowPickAndSpotlight(core));

  const {
    surfaceId, controllerId, editable, requestContextMenu, windowInstanceId, sessionRef, schedulerRef, gpuCanvasRef,
    labelCanvasRef, containerRef, canvasGeneration, contextMenu, setContextMenu, contextMenuTitleLabel, wireRefusal,
    setWireRefusal, wireRefusalText, selectionBounds, marquee, labelStateJson, sliderStateJson, containerSize, spotlight,
    spotlightSections, dispatch, flowMenuKeysByActionId, shellContextMenuFallback, buildFlowMenuDispatch, cameraPanRef,
    cameraOnlyGestureEndRef, endGesture, publishCameraRef, wheelGesture, handleGesturePointerDown, sliderLane,
    gestureRecognizer, pinchLogScaleRef, paintOverlays, renderFlow, contextMenuRequestRef, handleGesturePointerUp,
    emitInteractionState, pickInteraction, clearGhostPreview, closeSpotlight, previewSpotlightItem, commitSpotlightItem,
    onDragOverCanvas, onDrop, onCanvasDoubleClick
  } = core;

  return (
    <div
      ref={containerRef}
      className="relative h-full w-full"
      style={{ contain: "layout paint" }}
      onDragOver={onDragOverCanvas}
      onDragLeave={() => {
        if (!editable) return;
        clearGhostPreview();
      }}
      onDrop={onDrop}
      onContextMenu={(event) => {
        if (!editable || !requestContextMenu) return;
        event.preventDefault();
        event.stopPropagation();
        const request=++contextMenuRequestRef.current;
        void (async () => {
          const session = sessionRef.current;
          const container = containerRef.current;
          let widgetId: string | undefined;
          let hits: NonNullable<PluginContextMenuSurfaceTarget["hits"]> = [];
          let domains:DagSelectionDomains={nodes:[],edges:[],handles:[]};
          if (session) {
            try {
              domains=await decodeDagSelectionJson(flowJsonText(await readFlowTask(session.selectionDomainsJson())),nodeGraphInputDecodeControl(()=>sessionRef.current===session&&containerRef.current===container&&request===contextMenuRequestRef.current,()=>schedulerRef.current?.invalidate()));
            } catch {
              return;
            }
          }
          if (session && container) {
            const rect = container.getBoundingClientRect();
            const sx = event.clientX - rect.left;
            const sy = event.clientY - rect.top;
            try {
              const targets = JSON.parse(flowJsonText(await readFlowTask(session.pickTargetsAtScreenJson(sx, sy)))) as CanvasPickTarget[];
              hits = targets.map((target) => ({ domain: target.domain, id: target.id, label: target.label }));
              widgetId = pickMostSpecificCanvasTarget(targets)?.id;
            } catch {
              widgetId = undefined;
            }
          }
          if (!widgetId) {
            const hovered = session ? await readFlowTask(session.hoveredWidgetId()).catch(() => undefined) : undefined;
            widgetId = typeof hovered === "string" ? hovered : undefined;
          }
          if(sessionRef.current!==session||containerRef.current!==container||request!==contextMenuRequestRef.current)return;
          if (widgetId && !domains.nodes.includes(widgetId)) {
            domains = { nodes: [widgetId], edges: [], handles: [] };
            if (session) {
              observeFlowTask(session, "setSelection", session.setSelection(JSON.stringify(domains)));
              renderFlow();
            }
            dispatch("contextMenuAt", { id: widgetId });
          } else if (widgetId) {
            dispatch("contextMenuAt", { id: widgetId });
          }
          const menu = await openSurfaceContextMenu(
            requestContextMenu,
            {
              menu: { id: "nodeGraph", args: null },
              surface: {
                surfaceId,
                kind: "nodeGraph",
                hits,
                selection: selectionGroupsFromDomains(domains),
              },
              windowInstanceId: windowInstanceId ?? undefined,
              point: { x: event.clientX, y: event.clientY },
            },
            (specs) => mapContextMenuSpecs(specs, buildFlowMenuDispatch(widgetId, event.clientX, event.clientY), flowMenuKeysByActionId),
            shellContextMenuFallback,
          );
          if(sessionRef.current!==session||containerRef.current!==container||request!==contextMenuRequestRef.current)return;
          setContextMenu({ x: event.clientX, y: event.clientY, widgetId, ...menu });
          paintOverlays();
        })();
      }}
    >
      <canvas key={flowSurfaceCanvasKey(surfaceId, canvasGeneration)} ref={gpuCanvasRef} className="absolute inset-0 block h-full w-full" />
      <canvas ref={labelCanvasRef} className="pointer-events-none absolute inset-0 z-40" />
      <GraphSliderOverlays
        scopeId={JSON.stringify([windowInstanceId, controllerId, surfaceId])}
        stateJson={sliderStateJson}
        logicalW={containerSize.w}
        logicalH={containerSize.h}
        editable={editable}
        occluderRect={parseDagMinimapWidgetOccluder(labelStateJson)}
        onSliderChange={(widgetId, value) => {
          const session = sessionRef.current;
          if (!session) return;
          observeFlowTask(session, "setSliderValue", session.setSliderValue(widgetId, value));
          sliderLane(widgetId).offer(value);
          renderFlow();
          paintOverlays();
        }}
        onSliderCommit={(widgetId, value) => sliderLane(widgetId).commit(value)}
        onSliderAbort={(widgetId, reason) => sliderLane(widgetId).abort(reason)}
        onSliderPointerDown={() => handleGesturePointerDown()}
        onSliderPointerUp={() => handleGesturePointerUp()}
      />
      {selectionBounds ? (
        <>
          <div className="pointer-events-none absolute z-20 border-2 border-accent" style={{ left: selectionBounds.x, top: selectionBounds.y, width: selectionBounds.width, height: selectionBounds.height }} />
          {editable ? (
            <SelectionAlignChrome
              bounds={selectionBounds}
              onAlign={(mode) => {
                const session = sessionRef.current;
                if (session)
                  observeFlowTask(session, "alignSelection", session.alignSelection(mode), (value) => {
                    const operations = nodeGraphEditRows(flowJsonText(value));
                    if (operations.length > 0) dispatch(nodeGraphActions.edit, { operations });
                  });
                paintOverlays();
              }}
            />
          ) : null}
        </>
      ) : null}
      {marquee ? (
        marquee.kind === "lasso" ? (
          <SelectionMarquee className="z-50" coverage={marquee.coverage ?? "full"} shape="polygon" points={marquee.points ?? []} />
        ) : (
          <SelectionMarquee className="z-50" coverage={marquee.coverage ?? "full"} shape="rect" rect={{ x: marquee.x ?? 0, y: marquee.y ?? 0, width: marquee.width ?? 0, height: marquee.height ?? 0 }} />
        )
      ) : null}
      <div
        className="absolute inset-0 z-30 touch-none"
        data-gesture-surface="flow"
        onPointerDown={(event) => {
          const rect = event.currentTarget.getBoundingClientRect();
          const verdict = gestureRecognizer.down({ pointerId: event.pointerId, x: event.clientX - rect.left, y: event.clientY - rect.top });
          if (verdict.kind === "pinchBegin") {
            const session = sessionRef.current;
            pinchLogScaleRef.current = 0;
            cameraPanRef.current = false;
            cameraOnlyGestureEndRef.current = false;
            setWireRefusal(null);
            pickInteraction.onCanvasPointerLeave();
            for (const tracked of gestureRecognizer.pointers) event.currentTarget.setPointerCapture?.(tracked.pointerId);
            endGesture("gesture");
            if (session) issueFlowGestureStep(session.pointerCancelScreen(), () => schedulerRef.current?.invalidate());
            return;
          }
          if (verdict.kind !== "single") return;
          if (!editable) return;
          if (event.button === 2) return;
          const session = sessionRef.current;
          if (!session) return;
          const client = { x: event.clientX, y: event.clientY };
          try {
            event.currentTarget.setPointerCapture(event.pointerId);
          } catch {}
          pickInteraction.onCanvasPointerDown(client);
          cameraPanRef.current = flowGestureIsCameraPan(event.button, event.buttons);
          issueFlowGestureStep(session.pointerDownScreen(event.clientX - rect.left, event.clientY - rect.top, event.button, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey, cameraPanRef.current));
          handleGesturePointerDown();
        }}
        onPointerMove={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const verdict = gestureRecognizer.move({ pointerId: event.pointerId, x: event.clientX - rect.left, y: event.clientY - rect.top });
          if (verdict.kind === "pinch") {
            const plan = graphPinchWheelPlan(verdict.step, pinchLogScaleRef.current);
            pinchLogScaleRef.current = plan.pendingLogScale;
            if (plan.calls.length === 0) return;
            for (const call of plan.calls) issueFlowGestureStep(session.wheelScreen(call.sx, call.sy, call.deltaX, call.deltaY, call.zoomGesture));
            wheelGesture.tick();
            return;
          }
          if (verdict.kind !== "single") return;
          pickInteraction.onCanvasPointerMove({ x: event.clientX, y: event.clientY });
          issueFlowGestureStep(session.pointerMoveScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey));
          schedulerRef.current?.invalidate();
        }}
        onPointerUp={(event) => {
          const verdict = gestureRecognizer.up(event.pointerId);
          if (verdict.kind === "pinchEnd") pinchLogScaleRef.current = 0;
          if (verdict.kind !== "single") return;
          if (event.button === 2) return;
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          try {
            if (event.currentTarget.hasPointerCapture(event.pointerId)) event.currentTarget.releasePointerCapture(event.pointerId);
          } catch {}
          pickInteraction.onCanvasPointerUp(client, { shift: event.shiftKey, ctrlOrMeta: event.metaKey || event.ctrlKey, alt: event.altKey });
          setWireRefusal(null);
          const wasCameraPan = cameraPanRef.current;
          cameraPanRef.current = false;
          cameraOnlyGestureEndRef.current = wasCameraPan;
          issueFlowGestureStep(session.pointerUpScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey), (value) => {
            const operations = nodeGraphEditRows(flowJsonText(value));
            if (operations.length > 0) dispatch(nodeGraphActions.edit, { operations });
          });
          handleGesturePointerUp();
          if (wasCameraPan) publishCameraRef.current();
          else emitInteractionState();
        }}
        onPointerCancel={(event) => {
          const verdict = gestureRecognizer.up(event.pointerId);
          if (verdict.kind === "pinchEnd") pinchLogScaleRef.current = 0;
          if (verdict.kind !== "single") return;
          const session = sessionRef.current;
          try {
            if (event.currentTarget.hasPointerCapture(event.pointerId)) event.currentTarget.releasePointerCapture(event.pointerId);
          } catch {}
          setWireRefusal(null);
          cameraPanRef.current = false;
          cameraOnlyGestureEndRef.current = false;
          pickInteraction.onCanvasPointerLeave();
          endGesture("gesture");
          if (!session) return;
          issueFlowGestureStep(session.pointerCancelScreen(), () => {
            schedulerRef.current?.invalidate();
            paintOverlays();
          });
        }}
        onPointerLeave={() => {
          setWireRefusal(null);
          pickInteraction.onCanvasPointerLeave();
        }}
        onDoubleClick={onCanvasDoubleClick}
        onWheel={(event) => {
          event.preventDefault();
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const delta = event.deltaMode === 1 ? event.deltaY * 16 : event.deltaMode === 2 ? event.deltaY * 400 : event.deltaY;
          issueFlowGestureStep(session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, 0, delta, true));
          wheelGesture.tick();
        }}
      />
      {wireRefusal ? (
        <div
          role="status"
          aria-live="polite"
          data-wire-refusal-json={JSON.stringify(wireRefusal)}
          className="pointer-events-none absolute bottom-2 left-1/2 z-30 -translate-x-1/2 rounded border border-destructive bg-panel px-2 py-1 text-[0.6875rem] text-destructive shadow-sm"
        >
          {wireRefusalText}
        </div>
      ) : null}
      <CanvasPickMenu request={pickInteraction.pickMenu} hoveredKey={pickInteraction.menuHoveredKey} onHoverKey={pickInteraction.onMenuHoverKey} onPick={pickInteraction.onMenuPick} onDismiss={pickInteraction.dismissPickMenu} />
      {spotlight ? (
        <FlowSpotlight state={spotlight} sections={spotlightSections} onPreview={previewSpotlightItem} onCommit={commitSpotlightItem} onClose={closeSpotlight} />
      ) : null}
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.items ?? []}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion FlowGraphCanvasHost
//#endregion 🔖️flow-graph-canvas-host
//#endregion 🔖️NodeGraphHost
