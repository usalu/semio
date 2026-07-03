// #region 🧲Header
/** @emoji 🛝 Forms app renderer contribution — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import type { AppRendererContribution } from "@semio-tech/framework-platform-core";
import type { OsAppInstance } from "@semio-tech/framework-os-core";
import { OsUpstreamBadge, useOsInstanceHostBridge, useOsInstanceMaterialization } from "@semio-tech/framework-os-renderer-react";
import { PlaygroundContext, PureSidePanelTabDefinition, CallbackTreePanelDefinition, Platform, CommandBus, collectUiTreeItemDragData, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig, controllerBackedExampleContribution } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { type SidePanelTabConfig } from "@semio-tech/framework-playground-core";
import type { UiFormsHostSurfaceNode } from "@semio-tech/framework-platform-core";
import {
  FORMS_PLAY_CATALOGUE_TAB_ID,
  FORMS_PLAY_CONTROLLER_ID,
  FORMS_PLAY_EXAMPLE_OPTIONS,
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
  parseFormSpec,
  type FormSpec,
  formsPlayWindowBodies,
} from "@semio-tech/forms-core";
import {
  FormEditSurface,
  FormRenderer,
  defaultFormSpec,
  formsQuestionPaletteTreeDragController,
  FormsQuestionPaletteDragBridge,
  FormsQuestionPaletteDragGhost,
} from "./index.tsx";

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

function FormsEditSurfaceHost({ node: _node }: { readonly node: UiFormsHostSurfaceNode }): ReactElement {
  const ctrl = useFormsPlayController();
  const spec = ctrl?.getSpec() ?? defaultFormSpec("empty");
  const selectedIds = ctrl?.getSelectedIds() ?? [];
  const onPaletteDrop = reactHostPort.useCallback(
    (detail: { kind: string; clientX: number; clientY: number }) =>
      commitFormsPlayQuestionDropAtClient(ctrl ?? undefined, detail.clientX, detail.clientY, detail.kind),
    [ctrl],
  );
  return (
    <>
      <FormsQuestionPaletteDragBridge enabled onCommitDrop={onPaletteDrop} />
      <FormsQuestionPaletteDragGhost />
      <FormEditSurface
      spec={spec}
      className="h-full"
      selectedIds={selectedIds}
      onSelectionChange={(ids) => ctrl?.run("setSelection", { ids: [...ids] })}
      onChange={(next) => ctrl?.run("setSpecJson", { json: JSON.stringify(next) })}
    />
    </>
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

function FormsOsInstanceHost({ instance }: { readonly instance: OsAppInstance }): ReactElement {
  const bridge = useOsInstanceHostBridge();
  const bundle = useOsInstanceMaterialization(instance);
  const materialized = bundle.projection;
  const formsSpec = reactHostPort.useMemo(() => {
    if (materialized && typeof materialized === "object" && (materialized as FormSpec).schema === "forms.form") return materialized as FormSpec;
    if (instance.sourceDocument.inline) {
      try {
        return parseFormSpec(JSON.parse(instance.sourceDocument.inline));
      } catch {
        return defaultFormSpec(instance.id);
      }
    }
    return defaultFormSpec(instance.id);
  }, [instance, materialized]);
  const dispatchForms = reactHostPort.useCallback(
    (spec: FormSpec) => {
      bridge.dispatch({
        kind: "applyAppOperation",
        instanceId: instance.id,
        forwards: [{ op: "replaceProjection", projection: spec }],
        backwards: [{ op: "replaceProjection", projection: formsSpec }],
      });
    },
    [bridge, instance.id, formsSpec],
  );
  return (
    <div className="flex h-full min-h-0 flex-col overflow-hidden">
      <OsUpstreamBadge upstreamInstanceId={bundle.upstreamInstanceId} />
      <FormEditSurface spec={formsSpec} onChange={(spec) => dispatchForms(spec)} className="min-h-0 flex-1 overflow-auto p-4" />
    </div>
  );
}

/** @emoji 🛝 forms app renderer for playground and OS shells. */
export const formsAppRenderer: AppRendererContribution = {
  windowBodies: formsPlayWindowBodies,
  surfaceHosts: {
    [FORMS_PLAY_SURFACE_ID_EDIT]: FormsEditSurfaceHost,
    [FORMS_PLAY_SURFACE_ID_TRY]: FormsTrySurfaceHost,
  },
  panelTabs: {
    workbench: [new FormsPlayHierarchyPanelDefinition(), new FormsPlayCataloguePanelDefinition()],
    details: [new FormsPlayInspectionPanelDefinition()],
  },
  instanceHost: FormsOsInstanceHost,
  examples: controllerBackedExampleContribution(FORMS_PLAY_CONTROLLER_ID, FORMS_PLAY_EXAMPLE_OPTIONS),
};
