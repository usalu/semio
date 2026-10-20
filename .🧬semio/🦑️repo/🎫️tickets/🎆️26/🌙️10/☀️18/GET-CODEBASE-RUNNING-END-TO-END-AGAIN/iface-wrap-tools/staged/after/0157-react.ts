/** 🎯️ Canvas appearance sync, the pick interaction, the catalogue spotlight and its previews, drag and drop onto the canvas and the double-click handler.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | FlowGraphCanvasHost}: `useFlowPickAndSpotlight` runs the original statements in order and returns the bindings later hooks and the render read; the component merges them into the shared core object.
 */

import { decodeDagHoverJson } from "@semio-tech/framework/infinite/board/io/text/dag-input";
import { useCallback, useEffect, type DragEvent, type MouseEvent } from "../../⚛️react-port/🟦️.ts";
import {
  CATALOGUE_DRAG_MIME,
  getActiveCatalogueDragPayload,
  useCanvasAppearanceSync,
  useCanvasPickInteraction,
  useShellScopeOptional,
  type CanvasPickTarget
} from "@semio-tech/ui-react";
import {
  flowAddWidgetArgs,
  flowJsonText,
  nodeGraphInputDecodeControl,
  nodeGraphPickChannel,
  publishNodeGraphHover,
  resolveHostSnapshotWidgetInstanceId
} from "../🧰️graph-core/🟦️.ts";
import {
  catalogueGhostDescriptorJson,
  type FlowCatalogueItem,
  flowCatalogueItemDescriptor,
  type FlowCatalogueSection,
  parseCatalogueAppDragPayload,
  parseFlowCatalogueSections
} from "../🪄️catalogue-spotlight/🟦️.tsx";
import { dagScreenToWorld, parseDagOverlayCamera } from "../🖼️dag-overlays/🟦️.tsx";
import { observeFlowTask, readObservedFlowTask, syncFlowCanvasTheme, syncFlowSessionAppCatalogue } from "../🔄️flow-session-sync/🟦️.ts";
import type { FlowCoreAfterFlowSessionAttach } from "../🔌️flow-session-attach/🟦️.ts";
//#region 🔖️NodeGraphHost
//#region 🔖️flow-graph-canvas-host
//#region FlowGraphCanvasHost

export type FlowCoreAfterFlowPickAndSpotlight = FlowCoreAfterFlowSessionAttach & ReturnType<typeof useFlowPickAndSpotlight>;

export function useFlowPickAndSpotlight(core: FlowCoreAfterFlowSessionAttach) {
  const {
    scene, editable, sessionRef, schedulerRef, containerRef, labelStateJson, sessionReady, spotlight, setSpotlight,
    setSpotlightSections, appCatalogue, pickTargetsRef, sceneRef, dispatch, paintOverlays, renderFlow, interactionLedger,
    hoverRequestRef, emitInteractionState
  } = core;
  const flowGraphCanvasHostShellScope = useShellScopeOptional();
  useCanvasAppearanceSync(
    () => {
      if (sessionRef.current) syncFlowCanvasTheme(sessionRef.current);
      renderFlow();
      paintOverlays();
      schedulerRef.current?.invalidate();
    },
    true,
    flowGraphCanvasHostShellScope?.rootRef.current ?? undefined,
  );

  const pickInteraction = useCanvasPickInteraction({
    resolveTargetsAtClient: (client) => {
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return [];
      const rect = container.getBoundingClientRect();
      const sx = client.x - rect.left;
      const sy = client.y - rect.top;
      observeFlowTask(session, "pickTargetsAtScreenJson:pointer", session.pickTargetsAtScreenJson(sx, sy), (value) => {
        try {
          pickTargetsRef.current = JSON.parse(flowJsonText(value)) as CanvasPickTarget[];
        } catch {
          pickTargetsRef.current = [];
        }
      });
      return [...pickTargetsRef.current];
    },
    onHoverFocus: (focus) => {
      const session = sessionRef.current;
      if (!session) return;
      const request=++hoverRequestRef.current;
      const target = focus.target;
      const channel = nodeGraphPickChannel(target);
      if (!target) {
        observeFlowTask(session, "setHover", session.setHover(null));
      } else if (channel) {
        observeFlowTask(session, "setHoverChannel", session.setHoverChannel(channel.nodeId, channel.portId));
      } else {
        observeFlowTask(session, "setHover", session.setHover(target.id));
      }
      void Promise.all([
        readObservedFlowTask(session, "hoveredWidgetId:pointer", session.hoveredWidgetId()),
        readObservedFlowTask(session, "hoveredChannelJson:pointer", session.hoveredChannelJson()),
      ]).then(async ([hoveredValue, channelValue]) => {
        const hoveredId = typeof hoveredValue === "string" ? hoveredValue : undefined;
        const hover=await decodeDagHoverJson(flowJsonText(channelValue),nodeGraphInputDecodeControl(()=>sessionRef.current===session&&request===hoverRequestRef.current,()=>schedulerRef.current?.invalidate()));
        if(sessionRef.current!==session||request!==hoverRequestRef.current)return;
        const portId=hover.channel?.port;
        if (interactionLedger.publishHover({ hoveredId, portId })) publishNodeGraphHover(dispatch, sceneRef.current.interactionDomain, hoveredId, portId);
      }).catch(() => {});
      schedulerRef.current?.invalidate();
    },
    onSelectTarget: () => {
      emitInteractionState();
    },
  });

  const clearGhostPreview = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    observeFlowTask(session, "clearGhostWidget", session.clearGhostWidget());
    renderFlow();
    paintOverlays();
  }, [paintOverlays, renderFlow]);

  const closeSpotlight = useCallback(() => {
    setSpotlight(null);
    clearGhostPreview();
  }, [clearGhostPreview]);

  const previewSpotlightItem = useCallback(
    (item: FlowCatalogueItem | null) => {
      const session = sessionRef.current;
      const open = spotlight;
      if (!session || !open) return;
      if (!item) {
        observeFlowTask(session, "clearGhostWidget", session.clearGhostWidget());
        renderFlow();
        paintOverlays();
        return;
      }
      observeFlowTask(session, "setGhostWidget", session.setGhostWidget(flowCatalogueItemDescriptor(item), open.world.x, open.world.y));
      renderFlow();
      paintOverlays();
    },
    [paintOverlays, renderFlow, spotlight],
  );

  const commitSpotlightItem = useCallback(
    (item: FlowCatalogueItem) => {
      const session = sessionRef.current;
      const open = spotlight;
      if (!session || !open) return;
      dispatch("addWidget", flowAddWidgetArgs(flowCatalogueItemDescriptor(item), open.world.x, open.world.y));
      setSpotlight(null);
      clearGhostPreview();
    },
    [clearGhostPreview, dispatch, spotlight],
  );

  const openSpotlightAtClient = useCallback(
    (clientX: number, clientY: number, target: HTMLElement) => {
      const session = sessionRef.current;
      if (!session || !editable) return;
      const rect = target.getBoundingClientRect();
      const sx = clientX - rect.left;
      const sy = clientY - rect.top;
      const camera = parseDagOverlayCamera(labelStateJson);
      const world = dagScreenToWorld(camera, rect.width, rect.height, sx, sy);
      setSpotlight({ screen: { x: sx, y: sy }, world });
      observeFlowTask(session, "worldFromScreen:spotlight", session.worldFromScreen(sx, sy), (value) => {
        try {
          const parsed = JSON.parse(flowJsonText(value)) as { readonly x?: number; readonly y?: number };
          setSpotlight({ screen: { x: sx, y: sy }, world: { x: parsed.x ?? world.x, y: parsed.y ?? world.y } });
        } catch {}
      });
    },
    [editable, labelStateJson],
  );
  core.openSpotlightAtClient = openSpotlightAtClient;

  const onDragOverCanvas = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!editable) return;
      event.preventDefault();
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return;
      const encoded = getActiveCatalogueDragPayload();
      if (!encoded) return;
      const catalogueApp = parseCatalogueAppDragPayload(encoded);
      if (!catalogueApp) return;
      const rect = container.getBoundingClientRect();
      const sx = event.clientX - rect.left;
      const sy = event.clientY - rect.top;
      const camera = parseDagOverlayCamera(labelStateJson);
      const world = dagScreenToWorld(camera, rect.width, rect.height, sx, sy);
      observeFlowTask(session, "setGhostWidget", session.setGhostWidget(catalogueGhostDescriptorJson(catalogueApp), world.x, world.y));
      observeFlowTask(session, "worldFromScreen:drag", session.worldFromScreen(sx, sy), (value) => {
        try {
          const parsed = JSON.parse(flowJsonText(value)) as { readonly x?: number; readonly y?: number };
          observeFlowTask(session, "setGhostWidget", session.setGhostWidget(catalogueGhostDescriptorJson(catalogueApp), parsed.x ?? world.x, parsed.y ?? world.y));
        } catch {}
      });
      renderFlow();
      paintOverlays();
    },
    [editable, labelStateJson, paintOverlays, renderFlow],
  );

  const onDrop = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!editable) return;
      clearGhostPreview();
      const raw = event.dataTransfer.getData(CATALOGUE_DRAG_MIME) || event.dataTransfer.getData("text/plain") || getActiveCatalogueDragPayload() || "";
      if (!raw) return;
      event.preventDefault();
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return;
      const rect = container.getBoundingClientRect();
      const sx = event.clientX - rect.left;
      const sy = event.clientY - rect.top;
      const camera = parseDagOverlayCamera(labelStateJson);
      const fallback = dagScreenToWorld(camera, rect.width, rect.height, sx, sy);
      observeFlowTask(session, "worldFromScreen:drop", session.worldFromScreen(sx, sy), (value) => {
        let world = fallback;
        try {
          const parsed = JSON.parse(flowJsonText(value)) as { readonly x?: number; readonly y?: number };
          world = { x: parsed.x ?? fallback.x, y: parsed.y ?? fallback.y };
        } catch {}
        const catalogueApp = parseCatalogueAppDragPayload(raw);
        if (catalogueApp) {
          dispatch("spawnApp", { pluginId: catalogueApp.pluginId, appId: catalogueApp.appId, x: world.x, y: world.y });
          return;
        }
        dispatch("addWidget", flowAddWidgetArgs(raw.startsWith("{") ? raw : JSON.stringify({ kind: raw }), world.x, world.y));
      });
    },
    [clearGhostPreview, dispatch, editable, labelStateJson],
  );

  const onCanvasDoubleClick = useCallback(
    (event: MouseEvent<HTMLDivElement>) => {
      if (!editable) return;
      const session = sessionRef.current;
      if (!session) return;
      observeFlowTask(session, "hoveredWidgetId:doubleClick", session.hoveredWidgetId(), (value) => {
        const hovered = typeof value === "string" ? value : undefined;
        if (!hovered) {
          openSpotlightAtClient(event.clientX, event.clientY, event.currentTarget);
          return;
        }
        const instanceId = resolveHostSnapshotWidgetInstanceId(scene.hostSnapshotJson, hovered);
        if (instanceId) {
          dispatch("openInstance", { instanceId });
        }
      });
    },
    [dispatch, editable, openSpotlightAtClient, scene.hostSnapshotJson],
  );

  useEffect(() => clearGhostPreview, [clearGhostPreview]);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session || !sessionReady) return;
    syncFlowSessionAppCatalogue(session, appCatalogue, sceneRef.current);
  }, [appCatalogue, sessionReady]);

  useEffect(() => {
    if (!spotlight || !sessionReady) {
      setSpotlightSections([]);
      return;
    }
    const session = sessionRef.current;
    if (!session) {
      setSpotlightSections((appCatalogue.sections ?? []) as readonly FlowCatalogueSection[]);
      return;
    }
    return observeFlowTask(session, "catalogueJson:spotlight", session.catalogueJson(), (value) => {
      const sections = parseFlowCatalogueSections(flowJsonText(value));
      setSpotlightSections(sections.length > 0 ? sections : ((appCatalogue.sections ?? []) as readonly FlowCatalogueSection[]));
    });
  }, [appCatalogue, sessionReady, spotlight]);

  return {
    pickInteraction, clearGhostPreview, closeSpotlight, previewSpotlightItem, commitSpotlightItem, onDragOverCanvas,
    onDrop, onCanvasDoubleClick
  };
}
//#endregion FlowGraphCanvasHost
//#endregion 🔖️flow-graph-canvas-host
//#endregion 🔖️NodeGraphHost
