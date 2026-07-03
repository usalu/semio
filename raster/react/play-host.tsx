// #region 🧲Header
/** @emoji 🛝 Raster app renderer contribution — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import type { AppRendererContribution } from "@semio-tech/framework-platform-core";
import type { OsAppInstance } from "@semio-tech/framework-os-core";
import { OsUpstreamBadge, useOsInstanceHostBridge, useOsInstanceMaterialization } from "@semio-tech/framework-os-renderer-react";
import { PlaygroundContext, PureSidePanelTabDefinition, CallbackTreePanelDefinition, Platform, CommandBus, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig, controllerBackedExampleContribution } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { CANVAS_HOVER_SOURCE_CANVAS, CANVAS_HOVER_SOURCE_CATALOG, CANVAS_HOVER_SOURCE_HIERARCHY } from "@semio-tech/framework-core";
import { type SidePanelTabConfig } from "@semio-tech/framework-playground-core";
import * as React from "react";
import type { UiRasterHostSurfaceNode } from "@semio-tech/framework-platform-core";
import {
  RASTER_PLAY_CATALOGUE_TAB_ID,
  RASTER_PLAY_CONTROLLER_ID,
  RASTER_PLAY_EXAMPLE_OPTIONS,
  RASTER_PLAY_LAYERS_TAB_ID,
  RASTER_PLAY_MASKS_TAB_ID,
  RASTER_PLAY_PROPERTIES_TAB_ID,
  RASTER_PLAY_SURFACE_ID_COMPOSITE,
  RASTER_PLAY_SURFACE_ID_NAVIGATOR,
  RasterPlayController,
  buildRasterPlayCatalogueTree,
  buildRasterPlayInspectorTree,
  buildRasterPlayLayersTree,
  buildRasterPlayMasksTree,
  createRasterPlayHierarchyTreeDragController,
  defaultRasterDocument,
  type RasterDocument,
  type RasterPlayHierarchyBuildOptions,
  type RasterPlayHostBridge,
  rasterPlayWindowBodies,
} from "@semio-tech/raster-core";
import { RasterCanvas, RasterLayerView, RasterMaskView } from "./index.tsx";

const rasterPlayControllerRef: { current: RasterPlayController | null } = { current: null };

function useRasterPlayController(runtimeOverride?: Platform): RasterPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => {
      const unsubscribeRuntime = runtime ? runtime.subscribe(listener) : () => {};
      const ctrl = runtime?.getActiveApp()?.controller as RasterPlayController | undefined;
      const unsubscribeCtrl = ctrl?.subscribe(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeCtrl?.();
      };
    },
    () => {
      const ctrl = runtime?.getActiveApp()?.controller as RasterPlayController | undefined;
      return ctrl?.getInteractionRevision() ?? runtime?.generation ?? 0;
    },
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as RasterPlayController | undefined;
  rasterPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function rasterPlayHierarchyOptions(ctrl: RasterPlayController | undefined): RasterPlayHierarchyBuildOptions {
  return {
    onToggleVisible: (layerId) => ctrl?.run("toggleLayerVisible", { layerId }),
    onDeleteLayer: (layerId) => ctrl?.run("deleteLayer", { layerId }),
    onDuplicateLayer: (layerId) => ctrl?.run("duplicateLayer", { layerId }),
    onAddMask: (layerId) => ctrl?.run("addLayerMask", { layerId }),
  };
}

function RasterPlayPaneSurfaceHost({ node }: { readonly node: UiRasterHostSurfaceNode }): ReactElement {
  const ctrl = useRasterPlayController();
  const doc = ctrl?.getDocument();
  if (!doc) return <div className="p-double text-sm text-muted-foreground">No raster document</div>;
  const selectedIds = ctrl?.getSelectedIds() ?? [];
  const hoveredId = ctrl?.getHoveredId() ?? null;
  const kindHover = ctrl?.getHoveredKind() ?? null;
  const onHover = reactHostPort.useCallback((payload: import("@semio-tech/raster-core").RasterHoverPayload) => {
    ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_CANVAS });
  }, [ctrl]);
  const onViewportChange = reactHostPort.useCallback((viewport: import("@semio-tech/raster-core").RasterViewport) => {
    ctrl?.run("setCompositeViewport", viewport);
  }, [ctrl]);
  const common = {
    document: doc,
    selectedIds,
    hoveredId,
    kindHover,
    activeTool: doc.activeTool,
    camera: doc.camera,
    contentViewport: ctrl.getCompositeViewport(),
    onViewportChange: node.view === "composite" ? onViewportChange : undefined,
    onHover,
    onSelect: (ids: readonly string[]) => ctrl?.run("setSelection", { ids: [...ids] }),
    onCommit: (document: typeof doc, selectLayerId?: string) => ctrl?.run("commitDocument", { document, selectLayerId }),
    onCameraChange: (camera: typeof doc.camera) => ctrl?.run("setCamera", { camera }),
    className: "h-full",
  };
  if (node.view === "layer") {
    return <RasterLayerView {...common} isolatedLayerId={node.layerId ?? selectedIds[0] ?? null} />;
  }
  if (node.view === "mask") {
    return <RasterMaskView {...common} isolatedLayerId={node.layerId ?? selectedIds[0] ?? null} />;
  }
  if (node.view === "navigator") {
    return (
      <>
        <RasterPlayFileBridge />
        <RasterCanvas {...common} viewMode="navigator" />
      </>
    );
  }
  return (
    <>
      <RasterPlayFileBridge />
      <RasterCanvas {...common} viewMode="composite" />
    </>
  );
}

class RasterPlayLayersPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: RASTER_PLAY_LAYERS_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(
        () => {
          const ctrl = rasterPlayControllerRef.current;
          const doc = ctrl?.getDocument();
          const bus = new CommandBus();
          if (!doc) return { sections: [{ id: "raster-empty", items: [{ id: "empty", label: "No document" }] }] };
          const treeNode = buildRasterPlayLayersTree(
            doc,
            ctrl?.getSelectedIds() ?? [],
            ctrl?.getHoveredId() ?? null,
            ctrl?.getHoveredKind() ?? null,
            (payload) => ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_HIERARCHY }),
            rasterPlayHierarchyOptions(ctrl),
          );
          const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
          return {
            ...config,
            dragAndDropController: createRasterPlayHierarchyTreeDragController(() => rasterPlayControllerRef.current ?? undefined),
          };
        },
        () => {
          const ctrl = rasterPlayControllerRef.current;
          const doc = ctrl?.getDocument();
          if (!doc) return [];
          return [...(buildRasterPlayLayersTree(doc, [], ctrl?.getHoveredId() ?? null, ctrl?.getHoveredKind() ?? null).highlightedIds ?? [])];
        },
      ),
    };
  }
}

class RasterPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: RASTER_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = rasterPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildRasterPlayCatalogueTree(
          ctrl?.getSelectedIds() ?? [],
          (payload) => ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_CATALOG }),
        );
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class RasterPlayMasksPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: RASTER_PLAY_MASKS_TAB_ID,
      icon: shellTabIconComponent("square-dashed", "workbench"),
      name: "Masks",
      order: 1,
      tree: new CallbackTreePanelDefinition(
        () => {
          const ctrl = rasterPlayControllerRef.current;
          const doc = ctrl?.getDocument();
          const bus = new CommandBus();
          if (!doc) return { sections: [{ id: "raster-masks-empty", items: [{ id: "empty", label: "No masks" }] }] };
          const treeNode = buildRasterPlayMasksTree(
            doc,
            ctrl?.getSelectedIds() ?? [],
            ctrl?.getHoveredId() ?? null,
            ctrl?.getHoveredKind() ?? null,
            (payload) => ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_HIERARCHY }),
          );
          return uiTreeNodeToTreePanelConfig(treeNode, bus);
        },
        () => {
          const ctrl = rasterPlayControllerRef.current;
          const doc = ctrl?.getDocument();
          if (!doc) return [];
          return [...(buildRasterPlayMasksTree(doc, [], ctrl?.getHoveredId() ?? null, ctrl?.getHoveredKind() ?? null).highlightedIds ?? [])];
        },
      ),
    };
  }
}

class RasterPlayPropertiesPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: RASTER_PLAY_PROPERTIES_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = rasterPlayControllerRef.current;
        const doc = ctrl?.getDocument();
        const bus = new CommandBus();
        if (!doc) return { sections: [{ id: "raster-props-empty", items: [{ id: "empty", label: "No document" }] }] };
        const treeNode = buildRasterPlayInspectorTree(doc, ctrl?.getSelectedIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function RasterPlayFileBridge(): ReactElement | null {
  const ctrl = useRasterPlayController();
  const loadInputRef = reactHostPort.useRef<HTMLInputElement | null>(null);
  const downloadFixture = reactHostPort.useCallback(async () => {
    if (!ctrl) return;
    const text = ctrl.getDocumentJson();
    const blob = new Blob([text], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = "semio.raster.json";
    anchor.click();
    URL.revokeObjectURL(url);
    console.log("[DEBUG] raster play exported document");
  }, [ctrl]);
  const handleLoadFile = reactHostPort.useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      event.target.value = "";
      if (!file || !ctrl) return;
      void file.text().then((text) => {
        ctrl.run("setFixtureJson", { json: text, resetInteraction: true });
        console.log("[DEBUG] raster play imported document from file");
      });
    },
    [ctrl],
  );
  reactHostPort.useEffect(() => {
    if (!ctrl) return;
    const bridge: RasterPlayHostBridge = {
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
  return <input ref={loadInputRef} type="file" accept=".json,.raster.json,application/json" className="hidden" onChange={handleLoadFile} />;
}

function RasterOsInstanceHost({ instance }: { readonly instance: OsAppInstance }): ReactElement {
  const bridge = useOsInstanceHostBridge();
  const bundle = useOsInstanceMaterialization(instance);
  const materialized = bundle.projection;
  const rasterDoc = reactHostPort.useMemo(() => {
    if (materialized && typeof materialized === "object" && (materialized as RasterDocument).schema === "raster.document") return materialized as RasterDocument;
    return defaultRasterDocument(instance.id);
  }, [instance.id, materialized]);
  const dispatchRaster = reactHostPort.useCallback(
    (document: RasterDocument) => {
      bridge.dispatch({
        kind: "applyAppOperation",
        instanceId: instance.id,
        forwards: [{ op: "replaceProjection", projection: document }],
        backwards: [{ op: "replaceProjection", projection: rasterDoc }],
      });
    },
    [bridge, instance.id, rasterDoc],
  );
  return (
    <div className="flex h-full min-h-0 flex-col overflow-hidden">
      <OsUpstreamBadge upstreamInstanceId={bundle.upstreamInstanceId} />
      <RasterCanvas
        document={rasterDoc}
        selectedIds={[]}
        hoveredId={null}
        kindHover={null}
        activeTool={rasterDoc.activeTool}
        camera={rasterDoc.camera}
        onSelect={() => {}}
        onHover={() => {}}
        onCameraChange={(camera) => dispatchRaster({ ...rasterDoc, camera })}
        className="min-h-0 flex-1"
        viewMode="composite"
      />
    </div>
  );
}

/** @emoji 🛝 raster app renderer for playground and OS shells. */
export const rasterAppRenderer: AppRendererContribution = {
  windowBodies: rasterPlayWindowBodies,
  surfaceHosts: {
    [RASTER_PLAY_SURFACE_ID_COMPOSITE]: RasterPlayPaneSurfaceHost,
    [RASTER_PLAY_SURFACE_ID_NAVIGATOR]: RasterPlayPaneSurfaceHost,
  },
  panelTabs: {
    workbench: [new RasterPlayLayersPanelDefinition(), new RasterPlayCataloguePanelDefinition(), new RasterPlayMasksPanelDefinition()],
    details: [new RasterPlayPropertiesPanelDefinition()],
  },
  instanceHost: RasterOsInstanceHost,
  examples: controllerBackedExampleContribution(RASTER_PLAY_CONTROLLER_ID, RASTER_PLAY_EXAMPLE_OPTIONS),
};
