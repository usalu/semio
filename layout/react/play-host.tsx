// #region 🧲Header
/** @emoji 🛝 Layout app renderer contribution — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import type { AppRendererContribution } from "@semio-tech/framework-platform-core";
import type { OsAppInstance } from "@semio-tech/framework-os-core";
import { OsUpstreamBadge, useOsInstanceMaterialization } from "@semio-tech/framework-os-renderer-react";
import { PureSidePanelTabDefinition, CallbackTreePanelDefinition, PlaygroundContext, Platform, CommandBus, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { CANVAS_HOVER_SOURCE_CANVAS, CANVAS_HOVER_SOURCE_CATALOG, CANVAS_HOVER_SOURCE_HIERARCHY } from "@semio-tech/framework-core";
import { type SidePanelTabConfig } from "@semio-tech/framework-playground-core";
import { LayoutCanvas, createLayoutPlayCatalogueTreeDragController, ensureLayoutWasm } from "./index.tsx";
import {
  LAYOUT_PLAY_CATALOGUE_TAB_ID,
  LAYOUT_PLAY_HIERARCHY_TAB_ID,
  LAYOUT_PLAY_INSPECTION_TAB_ID,
  LAYOUT_PLAY_PREFLIGHT_TAB_ID,
  LAYOUT_PLAY_SURFACE_BLUEPRINT,
  LAYOUT_PLAY_SURFACE_PREVIEW,
  LAYOUT_PLAY_WINDOW_PREVIEW,
  LayoutPlayController,
  buildLayoutPlayCatalogueTree,
  buildLayoutPlayHierarchyTree,
  buildLayoutPlayInspectorTree,
  buildLayoutPlayPreflightTree,
  findPage,
  type LayoutHoverPayload,
  layoutPlayWindowBodies,
} from "@semio-tech/layout-core";
import { DEFAULT_LAYOUT_DOCUMENT_JSON } from "@semio-tech/layout-core";

const layoutPlayControllerRef: { current: LayoutPlayController | null } = { current: null };

function useLayoutPlayController(runtimeOverride?: Platform): LayoutPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as LayoutPlayController | undefined;
  layoutPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function LayoutPlayPaneSurfaceHost({ node }: { readonly node: import("@semio-tech/framework-platform-core").UiLayoutHostSurfaceNode }): ReactElement {
  const ctrl = useLayoutPlayController();
  const chromeMode = node.chromeMode ?? (node.paneId === LAYOUT_PLAY_WINDOW_PREVIEW ? "preview" : "blueprint");
  const onSelectionChange = reactHostPort.useCallback(
    (objectId: string | null) => {
      if (objectId) ctrl?.run("setSelection", { ids: [objectId] });
    },
    [ctrl],
  );
  const onHover = reactHostPort.useCallback(
    (objectId: string | null) => {
      ctrl?.run("setHover", { id: objectId, sourceId: CANVAS_HOVER_SOURCE_CANVAS });
    },
    [ctrl],
  );
  const onCatalogueDrop = reactHostPort.useCallback(
    (kind: import("@semio-tech/layout-core").LayoutCatalogueKind, worldX: number, worldY: number) => {
      if (!ctrl) return;
      if (kind === "page") {
        const spreadId = findPage(ctrl.getDocument(), ctrl.getActivePageId())?.spreadId ?? ctrl.getDocument().spreads[0]?.id;
        ctrl.run("addPage", { spreadId });
        return;
      }
      const pageId = ctrl.getActivePageId();
      const layerId = ctrl.getDocument().pages.find((page) => page.id === pageId)?.layerIds[0];
      ctrl.run("addFrame", { kind, pageId, layerId, x: worldX, y: worldY });
    },
    [ctrl],
  );
  return (
    <LayoutCanvas
      chromeMode={chromeMode}
      documentJson={ctrl?.getDocumentJson() ?? DEFAULT_LAYOUT_DOCUMENT_JSON}
      pageId={ctrl?.getActivePageId() ?? "page-1"}
      selectedIds={ctrl?.getSelectedIds() ?? []}
      hoveredId={ctrl?.getHoveredId() ?? null}
      onHit={chromeMode === "blueprint" ? onSelectionChange : undefined}
      onHover={chromeMode === "blueprint" ? onHover : undefined}
      onCatalogueDrop={chromeMode === "blueprint" ? onCatalogueDrop : undefined}
      className="h-full min-h-0"
    />
  );
}

class LayoutPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: LAYOUT_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(
        () => {
          const ctrl = layoutPlayControllerRef.current;
          const bus = new CommandBus();
          const hoverSink = (payload: LayoutHoverPayload) => ctrl?.run("setHover", { id: payload.id, sourceId: CANVAS_HOVER_SOURCE_HIERARCHY });
          const treeNode = buildLayoutPlayHierarchyTree(
            ctrl?.getDocumentJson() ?? DEFAULT_LAYOUT_DOCUMENT_JSON,
            ctrl?.getSelectedIds() ?? [],
            ctrl?.getHoveredId() ?? null,
            hoverSink,
          );
          return uiTreeNodeToTreePanelConfig(treeNode, bus);
        },
        () => {
          const ctrl = layoutPlayControllerRef.current;
          const treeNode = buildLayoutPlayHierarchyTree(ctrl?.getDocumentJson() ?? DEFAULT_LAYOUT_DOCUMENT_JSON, [], ctrl?.getHoveredId() ?? null);
          return [...(treeNode.type === "tree" ? (treeNode.highlightedIds ?? []) : [])];
        },
      ),
    };
  }
}

class LayoutPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: LAYOUT_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = layoutPlayControllerRef.current;
        const bus = new CommandBus();
        const hoverSink = (payload: LayoutHoverPayload) => ctrl?.run("setHover", { id: payload.id, sourceId: CANVAS_HOVER_SOURCE_CATALOG });
        const treeNode = buildLayoutPlayCatalogueTree(hoverSink);
        return { ...uiTreeNodeToTreePanelConfig(treeNode, bus), dragAndDropController: createLayoutPlayCatalogueTreeDragController() };
      }),
    };
  }
}

class LayoutPlayPreflightPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: LAYOUT_PLAY_PREFLIGHT_TAB_ID,
      icon: shellTabIconComponent("alert-triangle", "workbench"),
      name: "Preflight",
      order: 2,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = layoutPlayControllerRef.current;
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(buildLayoutPlayPreflightTree(ctrl?.getDocumentJson() ?? DEFAULT_LAYOUT_DOCUMENT_JSON), bus);
      }),
    };
  }
}

class LayoutPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: LAYOUT_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = layoutPlayControllerRef.current;
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(buildLayoutPlayInspectorTree(ctrl?.getDocumentJson() ?? DEFAULT_LAYOUT_DOCUMENT_JSON, ctrl?.getSelectedIds() ?? []), bus);
      }),
    };
  }
}

function LayoutOsInstanceHost({ instance }: { readonly instance: OsAppInstance }): ReactElement {
  const bundle = useOsInstanceMaterialization(instance);
  const materialized = bundle.projection;
  const documentJson = reactHostPort.useMemo(() => {
    if (materialized && typeof materialized === "object") return JSON.stringify(materialized);
    return instance.sourceDocument.inline || DEFAULT_LAYOUT_DOCUMENT_JSON;
  }, [instance.sourceDocument.inline, materialized]);
  const pageId = reactHostPort.useMemo(() => {
    try {
      const parsed = JSON.parse(documentJson) as { pages?: readonly { id: string }[] };
      return parsed.pages?.[0]?.id ?? "page-1";
    } catch {
      return "page-1";
    }
  }, [documentJson]);
  return (
    <div className="flex h-full min-h-0 flex-col overflow-hidden">
      <OsUpstreamBadge upstreamInstanceId={bundle.upstreamInstanceId} />
      <LayoutCanvas documentJson={documentJson} pageId={pageId} className="min-h-0 flex-1" chromeMode="blueprint" />
    </div>
  );
}

/** @emoji 🛝 Layout app renderer contribution for playground and OS shells. */
export const layoutAppRenderer: AppRendererContribution = {
  windowBodies: layoutPlayWindowBodies,
  surfaceHosts: {
    [LAYOUT_PLAY_SURFACE_BLUEPRINT]: LayoutPlayPaneSurfaceHost,
    [LAYOUT_PLAY_SURFACE_PREVIEW]: LayoutPlayPaneSurfaceHost,
  },
  panelTabs: {
    workbench: [new LayoutPlayHierarchyPanelDefinition(), new LayoutPlayCataloguePanelDefinition(), new LayoutPlayPreflightPanelDefinition()],
    details: [new LayoutPlayInspectionPanelDefinition()],
  },
  preload: ensureLayoutWasm,
  instanceHost: LayoutOsInstanceHost,
};
