// #region 🧲Header
/** @emoji 🛝 Sequence app renderer contribution — loaded only via `./play` subpath. */
// #endregion 🧲Header

import { createWriterDocument } from "@semio-tech/writer-core";
import { WriterCanvas } from "@semio-tech/writer-react";
import type { ReactElement } from "react";
import type { AppRendererContribution } from "@semio-tech/framework-platform-core";
import { PlaygroundContext, useApp, PureSidePanelTabDefinition, CallbackTreePanelDefinition, Platform, CommandBus, collectUiTreeItemDragData, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { type SidePanelTabConfig } from "@semio-tech/framework-playground-core";
import {
  SEQUENCE_PLAY_CATALOGUE_TAB_ID,
  SEQUENCE_PLAY_DEFAULT_FIXTURE_JSON,
  SEQUENCE_PLAY_HIERARCHY_TAB_ID,
  SEQUENCE_PLAY_INSPECTION_TAB_ID,
  SEQUENCE_PLAY_SCRIPT_SURFACE_ID,
  SEQUENCE_PLAY_SURFACE_ID,
  SEQUENCE_PLAY_SURFACE_ID_COMPILED_DAG,
  SEQUENCE_PLAY_WINDOW_KIND_ID,
  SequencePlayController,
  buildSequencePlayCatalogueTree,
  buildSequencePlayHierarchyTree,
  buildSequencePlayInspectorTree,
  sequencePlayWindowBodies,
} from "@semio-tech/sequence-core";
import { DAG_LOD_MODE_AUTOMATIC, SequenceCanvas, dagLodCanvasProps, ensureSequenceWasmLoaded, sequenceStepPaletteTreeDragController } from "./index.tsx";

const sequencePlayControllerRef: { current: SequencePlayController | null } = { current: null };

function useSequencePlayController(runtimeOverride?: Platform): SequencePlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as SequencePlayController | undefined;
  sequencePlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useSequencePlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as SequencePlayController | undefined;
      sequencePlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as SequencePlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function SequencePlayPaneSurfaceHost({ node }: { readonly node: import("@semio-tech/framework-platform-core").UiSequenceHostSurfaceNode }): ReactElement {
  const { runtime } = useApp();
  const ctrl = useSequencePlayController();
  const interactionRevision = useSequencePlayInteractionRevision(runtime);
  void interactionRevision;
  const scopeId = node.paneId ?? SEQUENCE_PLAY_WINDOW_KIND_ID;
  const lodProps = dagLodCanvasProps(ctrl?.lodModeForScope(scopeId) ?? DAG_LOD_MODE_AUTOMATIC);
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
  const onSelectionChange = reactHostPort.useCallback(
    (ids: readonly string[]) => {
      ctrl?.run("setSelection", { ids: [...ids] });
    },
    [ctrl],
  );
  const onCompiledTextChange = reactHostPort.useCallback(
    (text: string) => {
      ctrl?.run("setCompiledText", { text });
    },
    [ctrl],
  );
  const onCompiledWireLiteralChange = reactHostPort.useCallback(
    (text: string) => {
      ctrl?.run("setCompiledWireLiteral", { text });
    },
    [ctrl],
  );
  const onRunResult = reactHostPort.useCallback(
    (result: import("@semio-tech/imperative-core").RunResult) => {
      ctrl?.run("setRunResult", { result });
    },
    [ctrl],
  );
  return (
    <SequenceCanvas
      fixtureJson={ctrl?.getFixtureJson() ?? SEQUENCE_PLAY_DEFAULT_FIXTURE_JSON}
      reorganize={ctrl?.getReorganize()}
      runRequest={ctrl?.getRunRequest()}
      runStopRequest={ctrl?.getRunStopRequest()}
      selectedStepIds={ctrl?.getSelectedStepIds() ?? []}
      fixtureDragDrop
      onFixtureChange={onFixtureChange}
      onSelectionChange={onSelectionChange}
      onCompiledTextChange={onCompiledTextChange}
      onCompiledWireLiteralChange={onCompiledWireLiteralChange}
      onRunResult={onRunResult}
      {...lodProps}
      onLodChange={onLodChange}
    />
  );
}

function SequencePlayScriptSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const ctrl = useSequencePlayController();
  const interactionRevision = useSequencePlayInteractionRevision(appCtx?.runtime as Platform);
  const document = reactHostPort.useMemo(
    () =>
      createWriterDocument({
        id: "sequence-compiled-script",
        languageId: "plaintext",
        text: ctrl?.getCompiledText() ?? "",
      }),
    [ctrl?.getCompiledText(), interactionRevision],
  );
  return <WriterCanvas document={document} className="h-full min-h-0" />;
}

function SequencePlayCompiledDagSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const ctrl = useSequencePlayController();
  const interactionRevision = useSequencePlayInteractionRevision(appCtx?.runtime as Platform);
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  const document = reactHostPort.useMemo(
    () => ctrl?.getWriterDocumentCompiledDag() ?? createWriterDocument({ id: "sequence-compiled-dag", languageId: "wire", text: "" }),
    [ctrl, interactionRevision],
  );
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    sequencePlayControllerRef.current?.run("setWireHover", { offset });
  }, []);
  const onSelectionChange = reactHostPort.useCallback((range: { start: number; end: number }) => {
    sequencePlayControllerRef.current?.run("setWireSelect", range);
  }, []);
  return (
    <WriterCanvas
      document={document}
      className="h-full min-h-0"
      onHoverChange={onHoverChange}
      onSelectionChange={onSelectionChange}
      externalHoverOccurrences={ctrl?.getWireHoverOccurrences()}
      externalHoverOccurrencesSignal={ctrl?.getHoverEpoch()}
      externalSelectionOccurrences={ctrl?.getWireSelectOccurrences()}
      externalSelectionOccurrencesSignal={ctrl?.getSelectEpoch()}
    />
  );
}

class SequencePlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: SEQUENCE_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = sequencePlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildSequencePlayHierarchyTree(ctrl?.getFixtureJson() ?? SEQUENCE_PLAY_DEFAULT_FIXTURE_JSON, ctrl?.getSelectedStepIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class SequencePlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: SEQUENCE_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const bus = new CommandBus();
        const treeNode = buildSequencePlayCatalogueTree();
        const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
        return {
          ...config,
          dragAndDropController: sequenceStepPaletteTreeDragController(collectUiTreeItemDragData(treeNode.sections)),
        };
      }),
    };
  }
}

class SequencePlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: SEQUENCE_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = sequencePlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildSequencePlayInspectorTree(
          ctrl?.getFixtureJson() ?? SEQUENCE_PLAY_DEFAULT_FIXTURE_JSON,
          ctrl?.getSelectedStepIds() ?? [],
          ctrl?.getEffectLog() ?? [],
        );
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

/** @emoji 🛝 Sequence app renderer contribution for playground and OS shells. */
export const sequenceAppRenderer: AppRendererContribution = {
  windowBodies: sequencePlayWindowBodies,
  surfaceHosts: {
    [SEQUENCE_PLAY_SURFACE_ID]: SequencePlayPaneSurfaceHost,
    [SEQUENCE_PLAY_SCRIPT_SURFACE_ID]: SequencePlayScriptSurfaceHost,
    [SEQUENCE_PLAY_SURFACE_ID_COMPILED_DAG]: SequencePlayCompiledDagSurfaceHost,
  },
  panelTabs: {
    workbench: [new SequencePlayHierarchyPanelDefinition(), new SequencePlayCataloguePanelDefinition()],
    details: [new SequencePlayInspectionPanelDefinition()],
  },
  preload: ensureSequenceWasmLoaded,
};
//#endregion 🔖SequencePlayHost
