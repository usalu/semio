// #region 🧲Header
/** @emoji 🛝 Draw app renderer contribution — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import type { AppRendererContribution, UiDrawHostSurfaceNode } from "@semio-tech/framework-platform-core";
import type { OsAppInstance } from "@semio-tech/framework-os-core";
import { OsUpstreamBadge, useOsInstanceHostBridge, useOsInstanceMaterialization } from "@semio-tech/framework-os-renderer-react";
import { PlaygroundContext, PureSidePanelTabDefinition, CallbackTreePanelDefinition, Platform, CommandBus, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig, controllerBackedExampleContribution } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { CANVAS_HOVER_SOURCE_CANVAS, CANVAS_HOVER_SOURCE_HIERARCHY } from "@semio-tech/framework-core";
import { type SidePanelTabConfig } from "@semio-tech/framework-playground-core";
import * as React from "react";
import {
  DRAW_PLAY_CATALOGUE_TAB_ID,
  DRAW_PLAY_CONTROLLER_ID,
  DRAW_PLAY_EXAMPLE_DEFAULT_ID,
  DRAW_PLAY_LAYERS_TAB_ID,
  DRAW_PLAY_PROPERTIES_TAB_ID,
  DRAW_PLAY_SURFACE_ID_COMPOSITE,
  DrawPlayController,
  buildDrawPlayCatalogueTree,
  buildDrawPlayInspectorTree,
  buildDrawPlayLayersTree,
  createDrawPlayHierarchyTreeDragController,
  defaultDrawDocument,
  drawDocumentToJson,
  type DrawDocument,
  type DrawPlayHierarchyBuildOptions,
  type DrawPlayHostBridge,
  drawPlayWindowBodies,
} from "@semio-tech/draw-core";
import { DrawCanvas } from "./index.tsx";

const drawPlayControllerRef: { current: DrawPlayController | null } = { current: null };

function useDrawPlayController(runtimeOverride?: Platform): DrawPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => {
      const unsubscribeRuntime = runtime ? runtime.subscribe(listener) : () => {};
      const ctrl = runtime?.getActiveApp()?.controller as DrawPlayController | undefined;
      const unsubscribeCtrl = ctrl?.subscribe(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeCtrl?.();
      };
    },
    () => {
      const generation = runtime?.generation ?? 0;
      const revision = (runtime?.getActiveApp()?.controller as DrawPlayController | undefined)?.getInteractionRevision() ?? 0;
      return generation * 1_000_000 + revision;
    },
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as DrawPlayController | undefined;
  drawPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function drawPlayHierarchyOptions(ctrl: DrawPlayController | undefined): DrawPlayHierarchyBuildOptions {
  return {
    onToggleVisible: (layerId) => ctrl?.run("toggleLayerVisible", { layerId }),
    onDeleteLayer: (layerId) => ctrl?.run("deleteLayer", { layerId }),
    onDuplicateLayer: (layerId) => ctrl?.run("duplicateLayer", { layerId }),
  };
}

function DrawPlayFileBridge(): ReactElement | null {
  const ctrl = useDrawPlayController();
  const loadInputRef = reactHostPort.useRef<HTMLInputElement | null>(null);
  const downloadFixture = reactHostPort.useCallback(async () => {
    if (!ctrl) return;
    const text = ctrl.getDocumentJson();
    const blob = new Blob([text], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = "semio.draw.json";
    anchor.click();
    URL.revokeObjectURL(url);
    console.log("[DEBUG] draw play exported document");
  }, [ctrl]);
  const handleLoadFile = reactHostPort.useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      event.target.value = "";
      if (!file || !ctrl) return;
      void file.text().then((text) => {
        ctrl.run("setFixtureJson", { json: text, resetInteraction: true });
        console.log("[DEBUG] draw play imported document from file");
      });
    },
    [ctrl],
  );
  reactHostPort.useEffect(() => {
    if (!ctrl) return;
    const bridge: DrawPlayHostBridge = {
      runHostCommand: (command) => {
        if (command === "saveDownload") {
          void downloadFixture();
          return;
        }
        if (command === "loadRequest") {
          loadInputRef.current?.click();
        }
      },
    };
    ctrl.setHostBridge(bridge);
    return () => ctrl.setHostBridge(null);
  }, [ctrl, downloadFixture]);
  return <input ref={loadInputRef} type="file" accept=".json,.draw.json,application/json" className="hidden" onChange={handleLoadFile} />;
}

function DrawPlayPaneSurfaceHost({ node: _node }: { readonly node: UiDrawHostSurfaceNode }): ReactElement {
  const ctrl = useDrawPlayController();
  const doc = ctrl?.getDocument();
  const onHover = reactHostPort.useCallback((payload: import("@semio-tech/draw-core").DrawHoverPayload) => {
    ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_CANVAS });
  }, [ctrl]);
  if (!doc) {
    return (
      <>
        <DrawPlayFileBridge />
        <div className="p-double text-sm text-muted-foreground">No draw document</div>
      </>
    );
  }
  const selectedIds = ctrl?.getSelectedIds() ?? [];
  const hoveredId = ctrl?.getHoveredId() ?? null;
  const kindHover = ctrl?.getHoveredKind() ?? null;
  const common = {
    document: doc,
    selectedIds,
    hoveredId,
    kindHover,
    activeTool: doc.activeTool,
    camera: doc.camera,
    onHover,
    onSelect: (ids: readonly string[]) => ctrl?.run("setSelection", { ids: [...ids] }),
    onCommit: (document: typeof doc, selectLayerId?: string) => ctrl?.run("commitDocument", { document, selectLayerId }),
    onCameraChange: (camera: typeof doc.camera) => ctrl?.run("setCamera", { camera }),
    className: "h-full",
  };
  return (
    <>
      <DrawPlayFileBridge />
      <DrawCanvas {...common} />
    </>
  );
}

class DrawPlayLayersPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: DRAW_PLAY_LAYERS_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(
        () => {
          const ctrl = drawPlayControllerRef.current;
          const doc = ctrl?.getDocument();
          const bus = new CommandBus();
          if (!doc) return { sections: [{ id: "draw-empty", items: [{ id: "empty", label: "No document" }] }] };
          const treeNode = buildDrawPlayLayersTree(
            doc,
            ctrl?.getSelectedIds() ?? [],
            ctrl?.getHoveredId() ?? null,
            ctrl?.getHoveredKind() ?? null,
            (payload) => ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_HIERARCHY }),
            drawPlayHierarchyOptions(ctrl),
          );
          const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
          return {
            ...config,
            dragAndDropController: createDrawPlayHierarchyTreeDragController(() => drawPlayControllerRef.current ?? undefined),
          };
        },
        () => {
          const ctrl = drawPlayControllerRef.current;
          const doc = ctrl?.getDocument();
          if (!doc) return [];
          return [...(buildDrawPlayLayersTree(doc, [], ctrl?.getHoveredId() ?? null, ctrl?.getHoveredKind() ?? null).highlightedIds ?? [])];
        },
      ),
    };
  }
}

class DrawPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: DRAW_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = drawPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildDrawPlayCatalogueTree(
          ctrl?.getSelectedIds() ?? [],
          (payload) => ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_HIERARCHY }),
        );
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class DrawPlayPropertiesPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: DRAW_PLAY_PROPERTIES_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = drawPlayControllerRef.current;
        const doc = ctrl?.getDocument();
        const bus = new CommandBus();
        if (!doc) return { sections: [{ id: "draw-props-empty", items: [{ id: "empty", label: "No document" }] }] };
        const treeNode = buildDrawPlayInspectorTree(doc, ctrl?.getSelectedIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function DrawOsInstanceHost({ instance }: { readonly instance: OsAppInstance }): ReactElement {
  const bridge = useOsInstanceHostBridge();
  const bundle = useOsInstanceMaterialization(instance);
  const materialized = bundle.projection;
  const drawDoc = reactHostPort.useMemo(() => {
    if (instance.sourceDocument.payloadRef === "fixture:semio.draw.json") return defaultDrawDocument("semio", "Semio Emblem");
    if (materialized && typeof materialized === "object" && (materialized as DrawDocument).schema === "draw.document") return materialized as DrawDocument;
    return defaultDrawDocument(instance.id);
  }, [instance, materialized]);
  const dispatchDraw = reactHostPort.useCallback(
    (document: DrawDocument) => {
      bridge.dispatch({ kind: "patchAppSource", instanceId: instance.id, inline: drawDocumentToJson(document) });
    },
    [bridge, instance.id],
  );
  return (
    <div className="flex h-full min-h-0 flex-col overflow-hidden">
      <OsUpstreamBadge upstreamInstanceId={bundle.upstreamInstanceId} />
      <DrawCanvas document={drawDoc} onCommit={(document) => dispatchDraw(document)} className="min-h-0 flex-1" />
    </div>
  );
}

/** @emoji 🛝 Draw app renderer contribution for playground and OS shells. */
export const drawAppRenderer: AppRendererContribution = {
  windowBodies: drawPlayWindowBodies,
  surfaceHosts: {
    [DRAW_PLAY_SURFACE_ID_COMPOSITE]: DrawPlayPaneSurfaceHost,
  },
  panelTabs: {
    workbench: [new DrawPlayLayersPanelDefinition(), new DrawPlayCataloguePanelDefinition()],
    details: [new DrawPlayPropertiesPanelDefinition()],
  },
  instanceHost: DrawOsInstanceHost,
  examples: controllerBackedExampleContribution(DRAW_PLAY_CONTROLLER_ID, [{ id: DRAW_PLAY_EXAMPLE_DEFAULT_ID, label: "Semio" }]),
};
//#endregion 🔖DrawPlayHost
