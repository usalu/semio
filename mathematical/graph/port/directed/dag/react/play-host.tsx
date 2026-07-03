// #region 🧲Header
/** @emoji 🛝 Playground play host for Dag — loaded only via `./play` subpath. */
// #endregion 🧲Header

import { createWriterDocument } from "@semio-tech/writer-core";
import { WriterCanvas } from "@semio-tech/writer-react";
import type { ReactElement } from "react";
import { type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, PlaygroundView, PlaygroundContext, PureSidePanelTabDefinition, CallbackTreePanelDefinition, registerUiDagSurfaceHost, registerUiWriterSurfaceHost, Platform, CommandBus, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { type SidePanelTabConfig } from "@semio-tech/framework-playground-core";
import {
    DAG_PLAY_APP_ID,
    DAG_PLAY_CATALOGUE_TAB_ID,
    DAG_PLAY_DEFAULT_FIXTURE_JSON,
    DAG_PLAY_HIERARCHY_TAB_ID,
    DAG_PLAY_INSPECTION_TAB_ID,
    DAG_PLAY_SURFACE_ID,
    DAG_PLAY_SURFACE_ID_JACK,
    DAG_PLAY_WINDOW_KIND_ID,
    DagPlayController,
    buildDagPlayCatalogueTree,
    buildDagPlayHierarchyTree,
    buildDagPlayInspectorTree,
    registerDagPlayDeclarativeBodies
} from "@semio-tech/dag-host-core";

import type { UiDagHostSurfaceNode } from "@semio-tech/framework-platform-core";

let dagPlayChromeRegistered = false;
const dagPlayControllerRef: { current: DagPlayController | null } = { current: null };

function useDagPlayController(runtimeOverride?: Platform): DagPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as DagPlayController | undefined;
  dagPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useDagPlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as DagPlayController | undefined;
      dagPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as DagPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function DagPlayPaneSurfaceHost({ node }: { readonly node: UiDagHostSurfaceNode }): ReactElement {
  const ctrl = useDagPlayController();
  const scopeId = node.paneId ?? DAG_PLAY_WINDOW_KIND_ID;
  const lodProps = dagLodCanvasProps(ctrl?.lodModeForScope(scopeId) ?? DAG_HOST_LOD_AUTOMATIC);
  const onLodChange = reactHostPort.useCallback(
    (lod: import("@semio-tech/dag-react").DagDrawLodKind) => {
      ctrl?.run("setEffectiveLod", { lod, instanceId: scopeId });
    },
    [ctrl, scopeId],
  );
  const onFixtureChange = reactHostPort.useCallback(
    (json: string) => {
      ctrl?.run("setFixtureJson", { json });
    },
    [ctrl],
  );
  console.log("[DEBUG] dag play surface mount");
  return (
    <DagCanvas
      fixtureJson={ctrl?.getFixtureJson() ?? DAG_PLAY_DEFAULT_FIXTURE_JSON}
      reorganize={ctrl?.getReorganize()}
      onFixtureChange={onFixtureChange}
      {...lodProps}
      onLodChange={onLodChange}
    />
  );
}

function DagPlayJackSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const ctrl = useDagPlayController();
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  const document = ctrl?.getWriterDocumentJack() ?? createWriterDocument({ id: "dag-jack", languageId: "jack", text: "" });
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    dagPlayControllerRef.current?.run("setJackHover", { offset });
  }, []);
  const onSelectionChange = reactHostPort.useCallback((range: { start: number; end: number }) => {
    dagPlayControllerRef.current?.run("setJackSelect", range);
  }, []);
  return (
    <WriterCanvas
      document={document}
      className="h-full"
      onHoverChange={onHoverChange}
      onSelectionChange={onSelectionChange}
      externalHoverOccurrences={ctrl?.getJackHoverOccurrences()}
      externalHoverOccurrencesSignal={ctrl?.getHoverEpoch()}
      externalSelectionOccurrences={ctrl?.getJackSelectOccurrences()}
      externalSelectionOccurrencesSignal={ctrl?.getSelectEpoch()}
    />
  );
}

export function registerDagPlaySurfaceHosts(): void {
  if (dagPlayChromeRegistered) return;
  dagPlayChromeRegistered = true;
  registerUiDagSurfaceHost(DAG_PLAY_SURFACE_ID, DagPlayPaneSurfaceHost);
  registerUiWriterSurfaceHost(DAG_PLAY_SURFACE_ID_JACK, DagPlayJackSurfaceHost);
  registerDagPlayDeclarativeBodies();
}

class DagPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: DAG_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = dagPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildDagPlayHierarchyTree(ctrl?.getFixtureJson() ?? DAG_PLAY_DEFAULT_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class DagPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: DAG_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(buildDagPlayCatalogueTree(), bus);
      }),
    };
  }
}

class DagPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: DAG_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = dagPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildDagPlayInspectorTree(ctrl?.getFixtureJson() ?? DAG_PLAY_DEFAULT_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function DagPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useDagPlayController(runtime);
  const interactionRevision = useDagPlayInteractionRevision(runtime);
  const dagPlayHierarchyPanel = reactHostPort.useMemo(() => new DagPlayHierarchyPanelDefinition(), []);
  const dagPlayCataloguePanel = reactHostPort.useMemo(() => new DagPlayCataloguePanelDefinition(), []);
  const dagPlayInspectionPanel = reactHostPort.useMemo(() => new DagPlayInspectionPanelDefinition(), []);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [dagPlayHierarchyPanel, dagPlayCataloguePanel],
      details: [dagPlayInspectionPanel],
    }),
    [interactionRevision, dagPlayCataloguePanel, dagPlayHierarchyPanel, dagPlayInspectionPanel],
  );
  return <PlaygroundView runtime={runtime} defaultAppId={DAG_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} />;
}

function DagPlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <DagPlayInner runtime={runtime} />;
}

export function mountDagPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<DagPlayChrome runtime={playground.runtime} />, rootId);
}

const dagPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerDagPlaySurfaceHosts,
  mount: mountDagPlayChrome,
};

export function bootDagPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, dagPlayChromeBoot, rootId);
}
//#endregion 🔖DagPlayHost