// #region 🧲Header
/** @emoji 🛝 Playground play host for Forms — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import { type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, PlaygroundView, PlaygroundContext, PureSidePanelTabDefinition, CallbackTreePanelDefinition, registerUiFormsSurfaceHost, Platform, CommandBus, collectUiTreeItemDragData, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { type SidePanelTabConfig } from "@semio-tech/framework-playground-core";
import {
  FORMS_PLAY_APP_ID,
  FORMS_PLAY_CATALOGUE_TAB_ID,
  FORMS_PLAY_CONTROLLER_ID,
  FORMS_PLAY_HIERARCHY_TAB_ID,
  FORMS_PLAY_INSPECTION_TAB_ID,
  FORMS_PLAY_SURFACE_ID_EDIT,
  FORMS_PLAY_SURFACE_ID_TRY,
  FormsPlayController,
  buildFormsPlayCatalogueTree,
  buildFormsPlayHierarchyTree,
  buildFormsPlayInspectorTree,
  commitFormsPlayQuestionDropAtClient,
  createFormsPlayHierarchyTreeDragController,
  registerFormsPlayDeclarativeBodies,
} from "@semio-tech/forms-core";

let formsPlayChromeRegistered = false;
const formsPlayControllerRef: { current: FormsPlayController | null } = { current: null };

function useFormsPlayController(runtimeOverride?: Platform): FormsPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribe(listener) : () => {}),
    () => runtime?.generation ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as FormsPlayController | undefined;
  formsPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useFormsPlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as FormsPlayController | undefined;
      formsPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as FormsPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function useFormsPlayExtensionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as FormsPlayController | undefined;
      formsPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as FormsPlayController | undefined)?.getExtensionRevision() ?? 0,
    () => 0,
  );
}

function FormsEditSurfaceHost({ node: _node }: { readonly node: UiFormsHostSurfaceNode }): ReactElement {
  const ctrl = useFormsPlayController();
  const spec = ctrl?.getSpec() ?? defaultFormSpec("empty");
  const selectedIds = ctrl?.getSelectedIds() ?? [];
  return (
    <FormEditSurface
      spec={spec}
      className="h-full"
      selectedIds={selectedIds}
      onSelectionChange={(ids) => ctrl?.run("setSelection", { ids: [...ids] })}
      onChange={(next) => ctrl?.run("setSpecJson", { json: JSON.stringify(next) })}
    />
  );
}

function FormsTrySurfaceHost({ node }: { readonly node: UiFormsHostSurfaceNode }): ReactElement {
  const ctrl = useFormsPlayController();
  const spec = ctrl?.getSpec();
  const tryValues = ctrl?.getTryValues() ?? {};
  if (!spec) return <div className="p-double text-sm text-muted-foreground">No form loaded</div>;
  return (
    <FormRenderer
      spec={spec}
      values={tryValues}
      className="h-full"
      onChange={(values) => ctrl?.run("setTryValues", { values })}
      onSubmit={(values) => console.log("[DEBUG] forms try submit", values)}
    />
  );
}

class FormsPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: FORMS_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = formsPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildFormsPlayHierarchyTree(ctrl?.getSpec() ?? defaultFormSpec("empty"), ctrl?.getSelectedIds() ?? []);
        const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
        return {
          ...config,
          dragAndDropController: createFormsPlayHierarchyTreeDragController(() => formsPlayControllerRef.current ?? undefined),
        };
      }),
    };
  }
}

class FormsPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: FORMS_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const bus = new CommandBus();
        const treeNode = buildFormsPlayCatalogueTree();
        const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
        return {
          ...config,
          dragAndDropController: formsQuestionPaletteTreeDragController(collectUiTreeItemDragData(treeNode.sections)),
        };
      }),
    };
  }
}

class FormsPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: FORMS_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = formsPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildFormsPlayInspectorTree(ctrl?.getSpec() ?? defaultFormSpec("empty"), ctrl?.getSelectedIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function FormsPlayInner({ playground }: { readonly playground: Playground }): ReactElement {
  useFormsPlayController(playground.runtime);
  const interactionRevision = useFormsPlayInteractionRevision(playground.runtime);
  const extensionRevision = useFormsPlayExtensionRevision(playground.runtime);
  const formsPlayHierarchyPanel = reactHostPort.useMemo(() => new FormsPlayHierarchyPanelDefinition(), []);
  const formsPlayCataloguePanel = reactHostPort.useMemo(() => new FormsPlayCataloguePanelDefinition(), []);
  const formsPlayInspectionPanel = reactHostPort.useMemo(() => new FormsPlayInspectionPanelDefinition(), []);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [formsPlayHierarchyPanel, formsPlayCataloguePanel],
      details: [formsPlayInspectionPanel],
    }),
    [interactionRevision, extensionRevision, formsPlayCataloguePanel, formsPlayHierarchyPanel, formsPlayInspectionPanel],
  );
  const onPaletteDrop = reactHostPort.useCallback(
    (detail: { kind: string; clientX: number; clientY: number }) =>
      commitFormsPlayQuestionDropAtClient(formsPlayControllerRef.current ?? undefined, detail.clientX, detail.clientY, detail.kind),
    [],
  );
  return (
    <>
      <FormsQuestionPaletteDragBridge enabled onCommitDrop={onPaletteDrop} />
      <FormsQuestionPaletteDragGhost />
      <PlaygroundView runtime={playground.runtime} defaultAppId={FORMS_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} playgroundKeybindings={playground.keybindings} />
    </>
  );
}

export function registerFormsPlaySurfaceHosts(): void {
  if (formsPlayChromeRegistered) return;
  formsPlayChromeRegistered = true;
  registerUiFormsSurfaceHost(FORMS_PLAY_SURFACE_ID_EDIT, FormsEditSurfaceHost);
  registerUiFormsSurfaceHost(FORMS_PLAY_SURFACE_ID_TRY, FormsTrySurfaceHost);
  registerFormsPlayDeclarativeBodies();
}

function FormsPlayChrome({ playground }: { readonly playground: Playground }): ReactElement {
  return <FormsPlayInner playground={playground} />;
}

export function mountFormsPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<FormsPlayChrome playground={playground} />, rootId);
}

const formsPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerFormsPlaySurfaceHosts,
  mount: mountFormsPlayChrome,
};

export function bootFormsPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, formsPlayChromeBoot, rootId);
}
//#endregion 🔖FormsPlayHost