// #region 🧲Header
/** @emoji 🛝 Procedural app renderer contribution — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import type { AppRendererContribution } from "@semio-tech/framework-platform-core";
import { PlaygroundContext, useApp, PureSidePanelTabDefinition, CallbackTreePanelDefinition, Platform, CommandBus, collectUiTreeItemDragData, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig, controllerBackedExampleContribution } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { type SidePanelTabConfig, UiPuzzle3dHostSurfaceNode } from "@semio-tech/framework-playground-core";
import type { UiFlowHostSurfaceNode, UiFormsHostSurfaceNode } from "@semio-tech/framework-platform-core";

import {
    PROCEDURAL_PLAY_CATALOGUE_TAB_ID,
    PROCEDURAL_3D_PLAY_CONTROLLER_ID,
    PROCEDURAL_PLAY_EXAMPLE_OPTIONS,
    PROCEDURAL_PLAY_HIERARCHY_TAB_ID,
    PROCEDURAL_PLAY_INSPECTION_TAB_ID,
    PROCEDURAL_PLAY_SURFACE_ID,
    PROCEDURAL_PLAY_SURFACE_ID_GENERATE,
    PROCEDURAL_PLAY_SURFACE_ID_PREVIEW,
    PROCEDURAL_PLAY_WINDOW_KIND_ID,
    ProceduralPlayController,
    buildProceduralPlayCanvasContextMenu,
    buildProceduralPlayCatalogueTree,
    buildProceduralPlayHierarchyTree,
    buildProceduralPlayInspectorTree,
    type ProceduralPlayHostBridge,
    proceduralPlayWindowBodies,
} from "@semio-tech/procedural-3d-core";
import { PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON } from "@semio-tech/procedural-3d-core";
import { FlowGenerateSurface } from "@semio-tech/forms-react";
import { parseFormSpec } from "@semio-tech/forms-core";
import { downloadFlowOutputExport } from "@semio-tech/flow-react/play";
import {
  DAG_LOD_MODE_AUTOMATIC as PROCEDURAL_3D_DAG_LOD_MODE_AUTOMATIC,
  FLOW_DEFAULT_PROXIMITY_DISTANCE as PROCEDURAL_3D_DEFAULT_PROXIMITY_DISTANCE,
  dagLodCanvasProps as procedural3dDagLodCanvasProps,
  flowWidgetPaletteTreeDragController as procedural3dWidgetPaletteTreeDragController,
} from "@semio-tech/flow-react";
import { ProceduralFlowEditor, ProceduralPreview, useProceduralBrepBridge } from "./index.tsx";

const proceduralPlayControllerRef: { current: ProceduralPlayController | null } = { current: null };

/** @emoji 🔔 Re-renders procedural play workbench kinds when WASM catalogue sections arrive. */
function useProceduralPlaySnapshotRevision(runtime: Platform, selector: (ctrl: ProceduralPlayController) => number): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as ProceduralPlayController | undefined;
      proceduralPlayControllerRef.current = ctrl ?? null;
      const unsubscribeChrome = runtime.subscribeChrome(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeChrome();
        unsubscribeSnapshot?.();
      };
    },
    () => {
      const ctrl = runtime.getActiveApp()?.controller as ProceduralPlayController | undefined;
      proceduralPlayControllerRef.current = ctrl ?? null;
      return ctrl ? selector(ctrl) : 0;
    },
    () => 0,
  );
}

function useProceduralPlayExtensionRevision(runtime: Platform): number {
  return useProceduralPlaySnapshotRevision(runtime, (c) => c.getExtensionRevision());
}

function useProceduralPlayInteractionRevision(runtime: Platform): number {
  return useProceduralPlaySnapshotRevision(runtime, (c) => c.getInteractionRevision());
}

function useProceduralPlayController(runtimeOverride?: Platform): ProceduralPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as ProceduralPlayController | undefined;
  proceduralPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

async function downloadProceduralFixtureJson(name: string, text: string): Promise<void> {
  const pickerWindow = window as Window & { showSaveFilePicker?: (options?: { suggestedName?: string; types?: { description: string; accept: Record<string, string[]> }[] }) => Promise<FileSystemFileHandle> };
  if (pickerWindow.showSaveFilePicker) {
    const handle = await pickerWindow.showSaveFilePicker({
      suggestedName: name,
      types: [{ description: "Flow fixture JSON", accept: { "application/json": [".json"] } }],
    });
    const writable = await handle.createWritable();
    await writable.write(`${text}\n`);
    await writable.close();
    return;
  }
  const href = URL.createObjectURL(new Blob([`${text}\n`], { type: "application/json" }));
  const link = document.createElement("a");
  link.href = href;
  link.download = name;
  link.click();
  URL.revokeObjectURL(href);
}

function ProceduralPlayToolbarHostBridge({ runtime, ctrl }: { readonly runtime: Platform; readonly ctrl: ProceduralPlayController | undefined }): ReactElement {
  const interactionRevision = useProceduralPlayInteractionRevision(runtime);
  const loadInputRef = reactHostPort.useRef<HTMLInputElement>(null);
  const downloadFixture = reactHostPort.useCallback(async () => {
    const json = ctrl?.getFixtureJson() ?? PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON;
    try {
      await downloadProceduralFixtureJson("procedural.fixture.json", json);
      console.log("[DEBUG] procedural play downloaded fixture");
    } catch (error) {
      console.log(`[DEBUG] procedural play download failed: ${String(error)}`);
    }
  }, [ctrl]);
  const handleLoadFile = reactHostPort.useCallback(
    (event: reactHostPort.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      event.target.value = "";
      if (!file || !ctrl) return;
      void file.text().then((text) => {
        if (!text.includes("flow.fixture")) {
          console.log("[DEBUG] procedural play load rejected: not a flow fixture");
          return;
        }
        ctrl.run("setFixtureJson", { json: text, resetInteraction: true });
        console.log("[DEBUG] procedural play loaded fixture from file");
      });
    },
    [ctrl],
  );
  reactHostPort.useEffect(() => {
    if (!ctrl) return;
    const bridge: ProceduralPlayHostBridge = {
      getToolbarState: () => ({
        selectionMethod: ctrl.getSelectionMethod(),
        selectionMode: ctrl.getSelectionMode(),
        showMode: ctrl.getShowMode(),
        selectionCount: ctrl.getSelectedNodeIds().length,
        hasStoredFixture: ctrl.hasStoredFixture(),
      }),
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
  }, [ctrl, downloadFixture, interactionRevision]);
  return <input ref={loadInputRef} type="file" accept=".json,application/json" className="hidden" onChange={handleLoadFile} />;
}

function ProceduralPlayPaneSurfaceHost({ node }: { readonly node: UiFlowHostSurfaceNode }): ReactElement {
  const { runtime } = useApp();
  const ctrl = useProceduralPlayController();
  const extensionRevision = useProceduralPlayExtensionRevision(runtime);
  const interactionRevision = useProceduralPlayInteractionRevision(runtime);
  void interactionRevision;
  const scopeId = node.paneId ?? PROCEDURAL_PLAY_WINDOW_KIND_ID;
  const lodProps = procedural3dDagLodCanvasProps(ctrl?.lodModeForScope(scopeId) ?? PROCEDURAL_3D_DAG_LOD_MODE_AUTOMATIC);
  const proximityDistance = ctrl?.proximityDistanceValue() ?? PROCEDURAL_3D_DEFAULT_PROXIMITY_DISTANCE;
  const onLodChange = reactHostPort.useCallback(
    (lod: import("@semio-tech/flow-react").DagDrawLodKind) => {
      ctrl?.run("setEffectiveLod", { lod, instanceId: scopeId });
    },
    [ctrl, scopeId],
  );
  const onPreviewText = reactHostPort.useCallback(
    (text: string) => {
      console.log(`[DEBUG] procedural play preview: ${text}`);
      ctrl?.run("setPreviewText", { text });
    },
    [ctrl],
  );
  const onEvalOutputs = reactHostPort.useCallback(
    (outputsJson: string, previewMeshes?: Readonly<Record<string, unknown>>) => {
      console.log(`[DEBUG] procedural play eval outputs: ${outputsJson.slice(0, 120)}`);
      ctrl?.run("setEvalOutputs", { outputsJson, previewMeshes });
    },
    [ctrl],
  );
  const onCatalogueReady = reactHostPort.useCallback(
    (sections: readonly import("@semio-tech/flow-react").CatalogueSection[]) => {
      ctrl?.run("setCatalogueSections", { sections: [...sections] });
    },
    [ctrl],
  );
  const onFixtureChange = reactHostPort.useCallback(
    (json: string) => {
      ctrl?.run("setFixtureJson", { json });
    },
    [ctrl],
  );
  const onSelectionChange = reactHostPort.useCallback(
    (ids: readonly string[]) => {
      ctrl?.run("setSelection", { ids: [...ids], mode: "default", fromFlow: true });
    },
    [ctrl],
  );
  const onPreselectChange = reactHostPort.useCallback(
    (snapshot: { readonly ids: readonly string[]; readonly removedIds: readonly string[] }) => {
      ctrl?.run("setPreselect", { ids: [...snapshot.ids], removedIds: [...snapshot.removedIds] });
    },
    [ctrl],
  );
  const onHoverChange = reactHostPort.useCallback(
    (id: string | null) => {
      ctrl?.run("setHover", { id, channel: null });
    },
    [ctrl],
  );
  const onChannelHoverChange = reactHostPort.useCallback(
    (channel: import("@semio-tech/procedural-3d-react").ProceduralChannelRef | null) => {
      ctrl?.run("setHover", { id: channel?.widgetId ?? null, channel });
    },
    [ctrl],
  );
  const onSelectedChannelsChange = reactHostPort.useCallback(
    (channels: readonly import("@semio-tech/procedural-3d-react").ProceduralChannelRef[]) => {
      ctrl?.run("setSelectedChannels", { channels: [...channels] });
    },
    [ctrl],
  );
  const onPreviewOffChange = reactHostPort.useCallback(
    (ids: readonly string[]) => {
      ctrl?.run("setPreviewOff", { ids: [...ids], fromFlow: true });
    },
    [ctrl],
  );
  const onCanvasCommand = reactHostPort.useCallback(
    (command: string, args?: Record<string, unknown>) => {
      ctrl?.run(command, args);
    },
    [ctrl],
  );
  const onOutputExport = reactHostPort.useCallback(
    (widgetId: string, format: string, resolvedValueJson: string) => {
      void downloadFlowOutputExport(format, resolvedValueJson, widgetId).catch((error) => {
        console.log(`[DEBUG] procedural play export failed: ${String(error)}`);
      });
    },
    [],
  );
  return (
    <>
      <ProceduralPlayToolbarHostBridge runtime={runtime} ctrl={ctrl} />
      <ProceduralFlowEditor
      fixtureJson={ctrl?.getFixtureJson() ?? PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON}
      reorganize={ctrl?.getReorganize()}
      commandRequest={ctrl?.getCommandRequest()}
      extensionRevision={extensionRevision}
      onPreviewText={onPreviewText}
      onEvalOutputs={onEvalOutputs}
      onOutputExport={onOutputExport}
      onCatalogueReady={onCatalogueReady}
      onFixtureChange={onFixtureChange}
      onSelectionChange={onSelectionChange}
      onPreselectChange={onPreselectChange}
      onHoverChange={onHoverChange}
      onChannelHoverChange={onChannelHoverChange}
      onSelectedChannelsChange={onSelectedChannelsChange}
      onPreviewOffChange={onPreviewOffChange}
      selectedNodeIds={ctrl?.getSelectedNodeIds()}
      selectedChannels={ctrl?.getSelectedChannels()}
      preselectNodeIds={ctrl?.getPreselectNodeIds()}
      preselectRemovedNodeIds={ctrl?.getPreselectRemovedNodeIds()}
      hoveredNodeId={ctrl?.getHoveredNodeId()}
      hoveredChannel={ctrl?.getHoveredChannel()}
      previewOffNodeIds={ctrl?.getPreviewOffNodeIds()}
      selectionMode={ctrl?.getSelectionMode()}
      selectionMethod={ctrl?.getSelectionMethod()}
      contextMenu={(ctx) => buildProceduralPlayCanvasContextMenu(ctx, onCanvasCommand)}
      {...lodProps}
      onLodChange={onLodChange}
      proximityDistance={proximityDistance}
      className="h-full w-full"
    />
    </>
  );
}

function ProceduralPreviewSurfaceHost({ node: _node }: { readonly node: UiPuzzle3dHostSurfaceNode }): ReactElement {
  const { runtime } = useApp();
  const ctrl = useProceduralPlayController();
  const brepBridge = useProceduralBrepBridge();
  const interactionRevision = useProceduralPlayInteractionRevision(runtime);
  void interactionRevision;
  const onHover = reactHostPort.useCallback(
    (channel: import("@semio-tech/procedural-3d-react").ProceduralChannelRef | null) => {
      ctrl?.run("setHover", { id: channel?.widgetId ?? null, channel });
    },
    [ctrl],
  );
  const onSelect = reactHostPort.useCallback(
    (channel: import("@semio-tech/procedural-3d-react").ProceduralChannelRef) => {
      ctrl?.run("setSelectChannels", { channels: [channel] });
    },
    [ctrl],
  );
  const onSelectionChange = reactHostPort.useCallback(
    (ids: readonly string[], mode: import("@semio-tech/procedural-3d-react").ProceduralSelectionMode) => {
      ctrl?.run("setSelection", { ids: [...ids], mode });
    },
    [ctrl],
  );
  const onGumballTransform = reactHostPort.useCallback(
    (request: import("@semio-tech/procedural-3d-react").ProceduralGumballTransformRequest) => {
      ctrl?.run("applyGumballTransform", request);
    },
    [ctrl],
  );
  return (
    <div className="absolute inset-0 min-h-0 min-w-0">
      <ProceduralPreview
        items={ctrl?.getPreviewItems() ?? []}
        selectedNodeIds={ctrl?.getSelectedNodeIds()}
        preselectNodeIds={ctrl?.getPreselectNodeIds()}
        preselectRemovedNodeIds={ctrl?.getPreselectRemovedNodeIds()}
        hoveredNodeId={ctrl?.getHoveredNodeId()}
        hoveredChannel={ctrl?.getHoveredChannel()}
        hoveredGeometryTargets={ctrl?.getHoveredGeometryTargets()}
        selectedChannels={ctrl?.getSelectedChannels()}
        selectedGeometryTargets={ctrl?.getSelectedGeometryTargets()}
        previewOffNodeIds={ctrl?.getPreviewOffNodeIds()}
        showMode={ctrl?.getShowMode() ?? "everything"}
        selectionMode={ctrl?.getSelectionMode()}
        selectionMethod={ctrl?.getSelectionMethod()}
        transformGranularity={ctrl?.getTransformGranularity() ?? "full"}
        onGumballTransform={onGumballTransform}
        gumballActiveWidgetIds={ctrl?.getGumballActiveWidgetIds()}
        onHover={onHover}
        onSelect={onSelect}
        onSelectionChange={onSelectionChange}
        kernel={brepBridge ?? undefined}
        className="h-full w-full"
      />
    </div>
  );
}

class ProceduralPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: PROCEDURAL_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = proceduralPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildProceduralPlayHierarchyTree(ctrl?.getFixtureJson() ?? PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class ProceduralPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: PROCEDURAL_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = proceduralPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildProceduralPlayCatalogueTree(ctrl?.getCatalogueSections() ?? [], ctrl?.getExtensionEntries() ?? []);
        const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
        return {
          ...config,
          dragAndDropController: procedural3dWidgetPaletteTreeDragController(collectUiTreeItemDragData(treeNode.sections)),
        };
      }),
    };
  }
}

class ProceduralPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: PROCEDURAL_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = proceduralPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildProceduralPlayInspectorTree(ctrl?.getFixtureJson() ?? PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}


function Procedural3dGenerateSurfaceHost({ node }: { readonly node: UiFormsHostSurfaceNode }): ReactElement {
  const ctrl = useProceduralPlayController();
  const spec = reactHostPort.useMemo(() => {
    try {
      return parseFormSpec(JSON.parse(ctrl?.getGenerateFormSpecJson() ?? "{}"));
    } catch {
      return parseFormSpec({ schema: "forms.form", id: "empty", version: "1", steps: [{ id: "s", title: "Inputs", questions: [] }] });
    }
  }, [ctrl]);
  return (
    <FlowGenerateSurface
      formSpec={spec}
      generations={[...(ctrl?.getGenerations() ?? [])]}
      selectedGenerationId={ctrl?.getSelectedGenerationId() ?? null}
      previewText={ctrl?.getGeneratePreviewText() ?? "—"}
      onSelectGeneration={(id) => ctrl?.run("selectGeneration", { id })}
      onAddGeneration={() => ctrl?.run("addGeneration")}
      onRemoveGeneration={(id) => ctrl?.run("removeGeneration", { id })}
      onGenerationValuesChange={(id, values) => ctrl?.run("updateGenerationValues", { id, values })}
      onRenameGeneration={(id, name) => ctrl?.run("renameGeneration", { id, name })}
      className="h-full"
    />
  );
}

/** @emoji 🛝 procedural app renderer for playground and OS shells. */
export const proceduralAppRenderer: AppRendererContribution = {
  windowBodies: proceduralPlayWindowBodies,
  surfaceHosts: {
    [PROCEDURAL_PLAY_SURFACE_ID]: ProceduralPlayPaneSurfaceHost,
    [PROCEDURAL_PLAY_SURFACE_ID_PREVIEW]: ProceduralPreviewSurfaceHost,
    [PROCEDURAL_PLAY_SURFACE_ID_GENERATE]: Procedural3dGenerateSurfaceHost,
  },
  panelTabs: {
    workbench: [new ProceduralPlayHierarchyPanelDefinition(), new ProceduralPlayCataloguePanelDefinition()],
    details: [new ProceduralPlayInspectionPanelDefinition()],
  },
  examples: controllerBackedExampleContribution(PROCEDURAL_3D_PLAY_CONTROLLER_ID, PROCEDURAL_PLAY_EXAMPLE_OPTIONS),
};
