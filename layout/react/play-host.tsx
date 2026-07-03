// #region 🧲Header
/** @emoji 🛝 Playground play host for Layout — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import { type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, PlaygroundView, PlaygroundContext, PureSidePanelTabDefinition, CallbackTreePanelDefinition, registerUiLayoutSurfaceHost, Platform, CommandBus, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { CANVAS_HOVER_SOURCE_CANVAS, CANVAS_HOVER_SOURCE_CATALOG, CANVAS_HOVER_SOURCE_HIERARCHY } from "@semio-tech/framework-core";
import { type SidePanelTabConfig } from "@semio-tech/framework-playground-core";
import {
  LAYOUT_PLAY_APP_ID,
  LAYOUT_PLAY_CATALOGUE_TAB_ID,
  LAYOUT_PLAY_CONTROLLER_ID,
  LAYOUT_PLAY_HIERARCHY_TAB_ID,
  LAYOUT_PLAY_INSPECTION_TAB_ID,
  LAYOUT_PLAY_PREFLIGHT_TAB_ID,
  LAYOUT_PLAY_SURFACE_BLUEPRINT,
  LAYOUT_PLAY_SURFACE_PREVIEW,
  LAYOUT_PLAY_WINDOW_BLUEPRINT,
  LAYOUT_PLAY_WINDOW_PREVIEW,
  LayoutPlayController,
  buildLayoutPlayCatalogueTree,
  buildLayoutPlayHierarchyTree,
  buildLayoutPlayInspectorTree,
  buildLayoutPlayPreflightTree,
  findPage,
  registerLayoutPlayDeclarativeBodies,
  type LayoutHoverPayload,
} from "@semio-tech/layout-core";

import { DEFAULT_LAYOUT_DOCUMENT_JSON } from "@semio-tech/layout-core";

let layoutPlayChromeRegistered = false;
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

function useLayoutPlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as LayoutPlayController | undefined;
      layoutPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as LayoutPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
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

export function registerLayoutPlaySurfaceHosts(): void {
  if (layoutPlayChromeRegistered) return;
  layoutPlayChromeRegistered = true;
  registerUiLayoutSurfaceHost(LAYOUT_PLAY_SURFACE_BLUEPRINT, LayoutPlayPaneSurfaceHost);
  registerUiLayoutSurfaceHost(LAYOUT_PLAY_SURFACE_PREVIEW, LayoutPlayPaneSurfaceHost);
  registerLayoutPlayDeclarativeBodies();
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
          const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
          return config;
        },
        () => {
          const ctrl = layoutPlayControllerRef.current;
          const treeNode = buildLayoutPlayHierarchyTree(
            ctrl?.getDocumentJson() ?? DEFAULT_LAYOUT_DOCUMENT_JSON,
            [],
            ctrl?.getHoveredId() ?? null,
          );
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
        const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
        return {
          ...config,
          dragAndDropController: createLayoutPlayCatalogueTreeDragController(),
        };
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
        const treeNode = buildLayoutPlayPreflightTree(ctrl?.getDocumentJson() ?? DEFAULT_LAYOUT_DOCUMENT_JSON);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
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
        const treeNode = buildLayoutPlayInspectorTree(ctrl?.getDocumentJson() ?? DEFAULT_LAYOUT_DOCUMENT_JSON, ctrl?.getSelectedIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

const layoutPlayHierarchyPanel = new LayoutPlayHierarchyPanelDefinition();
const layoutPlayCataloguePanel = new LayoutPlayCataloguePanelDefinition();
const layoutPlayPreflightPanel = new LayoutPlayPreflightPanelDefinition();
const layoutPlayInspectionPanel = new LayoutPlayInspectionPanelDefinition();

function LayoutPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  const interactionRevision = useLayoutPlayInteractionRevision(runtime);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [layoutPlayHierarchyPanel, layoutPlayCataloguePanel, layoutPlayPreflightPanel],
      details: [layoutPlayInspectionPanel],
    }),
    [interactionRevision],
  );
  return <PlaygroundView runtime={runtime} defaultAppId={LAYOUT_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} />;
}

function LayoutPlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <LayoutPlayInner runtime={runtime} />;
}

export function mountLayoutPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<LayoutPlayChrome runtime={playground.runtime} />, rootId);
}

const layoutPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerLayoutPlaySurfaceHosts,
  mount: mountLayoutPlayChrome,
};

export async function bootLayoutPlay(playground: Playground, rootId = "root"): Promise<void> {
  await ensureLayoutWasm();
  bootPlayground(playground, layoutPlayChromeBoot, rootId);
}
//#endregion 🔖LayoutPlayHost