// #region 🧲Header
/** @emoji 🛝 Procedural2d app renderer contribution — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import type { AppRendererContribution } from "@semio-tech/framework-platform-core";
import { PlaygroundContext, useApp, PureSidePanelTabDefinition, CallbackTreePanelDefinition, Platform, CommandBus, collectUiTreeItemDragData, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig, controllerBackedExampleContribution } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { type SidePanelTabConfig, UiPuzzle2dHostSurfaceNode } from "@semio-tech/framework-playground-core";
import type { UiFlowHostSurfaceNode, UiFormsHostSurfaceNode } from "@semio-tech/framework-platform-core";

import { flowWidgetPaletteTreeDragController as procedural2dWidgetPaletteTreeDragController } from "@semio-tech/flow-react";
import {
    PROCEDURAL_2D_PLAY_CATALOGUE_TAB_ID,
    PROCEDURAL_2D_PLAY_CONTROLLER_ID,
    PROCEDURAL_2D_PLAY_EXAMPLE_OPTIONS,
    PROCEDURAL_2D_PLAY_HIERARCHY_TAB_ID,
    PROCEDURAL_2D_PLAY_INSPECTION_TAB_ID,
    PROCEDURAL_2D_PLAY_SURFACE_ID,
    PROCEDURAL_2D_PLAY_SURFACE_ID_GENERATE,
    PROCEDURAL_2D_PLAY_SURFACE_ID_PREVIEW,
    Procedural2dPlayController,
    buildProcedural2dPlayCanvasContextMenu,
    buildProcedural2dPlayCatalogueTree,
    buildProcedural2dPlayHierarchyTree,
    buildProcedural2dPlayInspectorTree,
    type Procedural2dPlayHostBridge,
    procedural2dPlayWindowBodies,
} from "@semio-tech/procedural-2d-core";
import { PROCEDURAL_2D_PLAY_EMPTY_FIXTURE_JSON } from "@semio-tech/procedural-2d-core";
import { FlowGenerateSurface } from "@semio-tech/forms-react";
import { parseFormSpec } from "@semio-tech/forms-core";
import { downloadFlowOutputExport } from "@semio-tech/flow-react/play";
import {
  Procedural2dFlowEditor,
  Procedural2dPreview,
  useProcedural2dDrawingBridge,
  canvasDrawingPngExportPort,
} from "./index.tsx";

const procedural2dPlayControllerRef: { current: Procedural2dPlayController | null } = { current: null };

function useProcedural2dPlaySnapshotRevision(runtime: Platform, selector: (ctrl: Procedural2dPlayController) => number): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as Procedural2dPlayController | undefined;
      procedural2dPlayControllerRef.current = ctrl ?? null;
      const unsubscribeChrome = runtime.subscribeChrome(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeChrome();
        unsubscribeSnapshot?.();
      };
    },
    () => {
      const ctrl = runtime.getActiveApp()?.controller as Procedural2dPlayController | undefined;
      procedural2dPlayControllerRef.current = ctrl ?? null;
      return ctrl ? selector(ctrl) : 0;
    },
    () => 0,
  );
}

function useProcedural2dPlayExtensionRevision(runtime: Platform): number {
  return useProcedural2dPlaySnapshotRevision(runtime, (c) => c.getExtensionRevision());
}

function useProcedural2dPlayInteractionRevision(runtime: Platform): number {
  return useProcedural2dPlaySnapshotRevision(runtime, (c) => c.getInteractionRevision());
}

function useProcedural2dPlayController(runtimeOverride?: Platform): Procedural2dPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as Procedural2dPlayController | undefined;
  procedural2dPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

async function downloadProcedural2dExport(name: string, data: BlobPart, mime: string): Promise<void> {
  const pickerWindow = window as Window & { showSaveFilePicker?: (options?: { suggestedName?: string; types?: { description: string; accept: Record<string, string[]> }[] }) => Promise<FileSystemFileHandle> };
  const ext = name.includes(".") ? name.slice(name.lastIndexOf(".")) : "";
  if (pickerWindow.showSaveFilePicker) {
    const handle = await pickerWindow.showSaveFilePicker({
      suggestedName: name,
      types: [{ description: "Export", accept: { [mime]: [ext] } }],
    });
    const writable = await handle.createWritable();
    await writable.write(data);
    await writable.close();
    return;
  }
  const href = URL.createObjectURL(new Blob([data], { type: mime }));
  const link = document.createElement("a");
  link.href = href;
  link.download = name;
  link.click();
  URL.revokeObjectURL(href);
}

function Procedural2dPlayToolbarHostBridge({ runtime, ctrl }: { readonly runtime: Platform; readonly ctrl: Procedural2dPlayController | undefined }): ReactElement {
  const interactionRevision = useProcedural2dPlayInteractionRevision(runtime);
  const loadInputRef = reactHostPort.useRef<HTMLInputElement>(null);
  const drawingBridge = useProcedural2dDrawingBridge();
  const downloadFixture = reactHostPort.useCallback(async () => {
    const json = ctrl?.getFixtureJson() ?? PROCEDURAL_2D_PLAY_EMPTY_FIXTURE_JSON;
    try {
      await downloadProcedural2dExport("procedural2d.fixture.json", `${json}\n`, "application/json");
      console.log("[DEBUG] procedural 2d play downloaded fixture");
    } catch (error) {
      console.log(`[DEBUG] procedural 2d play download failed: ${String(error)}`);
    }
  }, [ctrl]);
  const handleLoadFile = reactHostPort.useCallback(
    (event: reactHostPort.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      event.target.value = "";
      if (!file || !ctrl) return;
      void file.text().then((text) => {
        if (!text.includes("flow.fixture")) {
          console.log("[DEBUG] procedural 2d play load rejected: not a flow fixture");
          return;
        }
        ctrl.run("setFixtureJson", { json: text, resetInteraction: true });
        console.log("[DEBUG] procedural 2d play loaded fixture from file");
      });
    },
    [ctrl],
  );
  reactHostPort.useEffect(() => {
    if (!ctrl) return;
    const bridge: Procedural2dPlayHostBridge = {
      getToolbarState: () => ({
        selectionMethod: ctrl.getSelectionMethod(),
        selectionMode: ctrl.getSelectionMode(),
        showMode: ctrl.getShowMode(),
        selectionCount: ctrl.getSelectedNodeIds().length,
        hasStoredFixture: ctrl.hasStoredFixture(),
      }),
      runHostCommand: (command, args) => {
        if (command === "saveDownload") {
          void downloadFixture();
          return;
        }
        if (command === "loadRequest") {
          loadInputRef.current?.click();
          return;
        }
        if (command === "exportSvg" || command === "exportPdf" || command === "exportPng") {
          const handle = (args as { handle?: string } | undefined)?.handle ?? ctrl.getPrimaryDrawingHandle();
          const primaryItem = ctrl.getPreviewItems().find((item) => item.kind === "drawing" && (!handle || item.handle === handle));
          if (!handle && !primaryItem?.scene) {
            console.log(`[DEBUG] procedural 2d play ${command} skipped: no drawing handle or scene`);
            return;
          }
          void (async () => {
            try {
              if (command === "exportPng" && primaryItem?.scene) {
                const png = canvasDrawingPngExportPort.exportPng(primaryItem.scene);
                await downloadProcedural2dExport("procedural2d.export.png", png, "image/png");
              } else if (!drawingBridge || !handle) {
                console.log(`[DEBUG] procedural 2d play ${command} skipped: no drawing bridge`);
                return;
              } else if (command === "exportSvg") {
                const svg = drawingBridge.exportSvg(handle);
                await downloadProcedural2dExport("procedural2d.export.svg", svg, "image/svg+xml");
              } else if (command === "exportPdf") {
                const pdfBase64 = drawingBridge.exportPdf(handle);
                const binary = atob(pdfBase64);
                const bytes = Uint8Array.from(binary, (char) => char.charCodeAt(0));
                await downloadProcedural2dExport("procedural2d.export.pdf", bytes, "application/pdf");
              } else {
                const png = drawingBridge.exportPng(handle);
                await downloadProcedural2dExport("procedural2d.export.png", png, "image/png");
              }
              console.log(`[DEBUG] procedural 2d play ${command} completed`);
            } catch (error) {
              console.log(`[DEBUG] procedural 2d play ${command} failed: ${String(error)}`);
            }
          })();
        }
      },
    };
    ctrl.setHostBridge(bridge);
    return () => ctrl.setHostBridge(null);
  }, [ctrl, downloadFixture, drawingBridge, interactionRevision]);
  return <input ref={loadInputRef} type="file" accept=".json,application/json" className="hidden" onChange={handleLoadFile} />;
}

function Procedural2dPlayPaneSurfaceHost({ node }: { readonly node: UiFlowHostSurfaceNode }): ReactElement {
  const { runtime } = useApp();
  const ctrl = useProcedural2dPlayController();
  const extensionRevision = useProcedural2dPlayExtensionRevision(runtime);
  const interactionRevision = useProcedural2dPlayInteractionRevision(runtime);
  void interactionRevision;
  const onPreviewText = reactHostPort.useCallback(
    (text: string) => {
      console.log(`[DEBUG] procedural 2d play preview: ${text}`);
      ctrl?.run("setPreviewText", { text });
    },
    [ctrl],
  );
  const onEvalOutputs = reactHostPort.useCallback(
    (outputsJson: string, previewMeshes?: Readonly<Record<string, unknown>>) => {
      console.log(`[DEBUG] procedural 2d play eval outputs: ${outputsJson.slice(0, 120)}`);
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
    (channel: import("@semio-tech/procedural-2d-react").ProceduralChannelRef | null) => {
      ctrl?.run("setHover", { id: channel?.widgetId ?? null, channel });
    },
    [ctrl],
  );
  const onSelectedChannelsChange = reactHostPort.useCallback(
    (channels: readonly import("@semio-tech/procedural-2d-react").ProceduralChannelRef[]) => {
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
        console.log(`[DEBUG] procedural 2d play export failed: ${String(error)}`);
      });
    },
    [],
  );
  return (
    <>
      <Procedural2dPlayToolbarHostBridge runtime={runtime} ctrl={ctrl} />
      <Procedural2dFlowEditor
      fixtureJson={ctrl?.getFixtureJson() ?? PROCEDURAL_2D_PLAY_EMPTY_FIXTURE_JSON}
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
      contextMenu={(ctx) => buildProcedural2dPlayCanvasContextMenu(ctx, onCanvasCommand)}
      className="h-full w-full"
    />
    </>
  );
}

function Procedural2dPreviewSurfaceHost({ node: _node }: { readonly node: UiPuzzle2dHostSurfaceNode }): ReactElement {
  const { runtime } = useApp();
  const ctrl = useProcedural2dPlayController();
  const drawingBridge = useProcedural2dDrawingBridge();
  const interactionRevision = useProcedural2dPlayInteractionRevision(runtime);
  void interactionRevision;
  const onHover = reactHostPort.useCallback(
    (channel: import("@semio-tech/procedural-2d-react").ProceduralChannelRef | null) => {
      ctrl?.run("setHover", { id: channel?.widgetId ?? null, channel });
    },
    [ctrl],
  );
  const onSelect = reactHostPort.useCallback(
    (channel: import("@semio-tech/procedural-2d-react").ProceduralChannelRef) => {
      ctrl?.run("setSelectChannels", { channels: [channel] });
    },
    [ctrl],
  );
  const onSelectionChange = reactHostPort.useCallback(
    (ids: readonly string[], mode: import("@semio-tech/procedural-2d-react").ProceduralSelectionMode) => {
      ctrl?.run("setSelection", { ids: [...ids], mode });
    },
    [ctrl],
  );
  return (
    <div className="absolute inset-0 min-h-0 min-w-0">
      <Procedural2dPreview
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
        onHover={onHover}
        onSelect={onSelect}
        onSelectionChange={onSelectionChange}
        kernel={drawingBridge ?? undefined}
        className="h-full w-full"
      />
    </div>
  );
}

class Procedural2dPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: PROCEDURAL_2D_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = procedural2dPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildProcedural2dPlayHierarchyTree(ctrl?.getFixtureJson() ?? PROCEDURAL_2D_PLAY_EMPTY_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class Procedural2dPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: PROCEDURAL_2D_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = procedural2dPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildProcedural2dPlayCatalogueTree(ctrl?.getCatalogueSections() ?? [], ctrl?.getExtensionEntries() ?? []);
        const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
        return {
          ...config,
          dragAndDropController: procedural2dWidgetPaletteTreeDragController(collectUiTreeItemDragData(treeNode.sections)),
        };
      }),
    };
  }
}

class Procedural2dPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: PROCEDURAL_2D_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = procedural2dPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildProcedural2dPlayInspectorTree(ctrl?.getFixtureJson() ?? PROCEDURAL_2D_PLAY_EMPTY_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}


function Procedural2dGenerateSurfaceHost({ node }: { readonly node: UiFormsHostSurfaceNode }): ReactElement {
  const ctrl = useProcedural2dPlayController();
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

/** @emoji 🛝 procedural2d app renderer for playground and OS shells. */
export const procedural2dAppRenderer: AppRendererContribution = {
  windowBodies: procedural2dPlayWindowBodies,
  surfaceHosts: {
    [PROCEDURAL_2D_PLAY_SURFACE_ID]: Procedural2dPlayPaneSurfaceHost,
    [PROCEDURAL_2D_PLAY_SURFACE_ID_PREVIEW]: Procedural2dPreviewSurfaceHost,
    [PROCEDURAL_2D_PLAY_SURFACE_ID_GENERATE]: Procedural2dGenerateSurfaceHost,
  },
  panelTabs: {
    workbench: [new Procedural2dPlayHierarchyPanelDefinition(), new Procedural2dPlayCataloguePanelDefinition()],
    details: [new Procedural2dPlayInspectionPanelDefinition()],
  },
  examples: controllerBackedExampleContribution(PROCEDURAL_2D_PLAY_CONTROLLER_ID, PROCEDURAL_2D_PLAY_EXAMPLE_OPTIONS),
};
