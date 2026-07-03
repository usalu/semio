// #region 🧲Header
/** @emoji 🛝 Playground play host for Flow — loaded only via `./play` subpath. */
// #endregion 🧲Header

import { createWriterDocument } from "@semio-tech/writer-core";
import { WriterCanvas } from "@semio-tech/writer-react";
import type { ReactElement } from "react";
import { type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, PlaygroundView, PlaygroundContext, useApp, PureSidePanelTabDefinition, CallbackTreePanelDefinition, registerUiFlowSurfaceHost, registerUiFormsSurfaceHost, registerUiWriterSurfaceHost, Platform, CommandBus, collectUiTreeItemDragData, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { downloadMediaExportResult } from "@semio-tech/framework-core";
import { type SidePanelTabConfig } from "@semio-tech/framework-playground-core";
import {
    FLOW_PLAY_APP_ID,
    FLOW_PLAY_CATALOGUE_TAB_ID,
    FLOW_PLAY_DEFAULT_FIXTURE_JSON,
    FLOW_PLAY_HIERARCHY_TAB_ID,
    FLOW_PLAY_INSPECTION_TAB_ID,
    FLOW_PLAY_SURFACE_ID,
    FLOW_PLAY_SURFACE_ID_GENERATE,
    FLOW_PLAY_SURFACE_ID_COMPILED_DAG,
    FLOW_PLAY_WINDOW_KIND_ID,
    FlowPlayController,
    buildFlowPlayCanvasContextMenu,
    buildFlowPlayCatalogueTree,
    buildFlowPlayHierarchyTree,
    buildFlowPlayInspectorTree,
    registerFlowPlayDeclarativeBodies
} from "@semio-tech/flow-core";

import { canvasDrawingPngExportPort } from "@semio-tech/procedural-2d-react";
import { FlowGenerateSurface } from "@semio-tech/forms-react";
import { parseFormSpec, type FormSpec } from "@semio-tech/forms-core";
import type { UiFlowHostSurfaceNode, UiFormsHostSurfaceNode } from "@semio-tech/framework-platform-core";

let flowPlayChromeRegistered = false;
const flowPlayControllerRef: { current: FlowPlayController | null } = { current: null };

type FlowExportMesh = {
  readonly position?: readonly number[];
  readonly index?: readonly number[];
  readonly error?: string;
};

function flowExportCollectHandle(value: unknown): string | null {
  if (typeof value === "string" && value.length > 0) return value;
  if (!value || typeof value !== "object") return null;
  const record = value as Record<string, unknown>;
  if (typeof record.handle === "string" && record.handle.length > 0) return record.handle;
  for (const nested of Object.values(record)) {
    const handle = flowExportCollectHandle(nested);
    if (handle) return handle;
  }
  return null;
}

function flowExportMeshToObj(mesh: FlowExportMesh): string {
  const position = mesh.position ?? [];
  const index = mesh.index ?? [];
  const lines = ["# semio flow export"];
  for (let i = 0; i + 2 < position.length; i += 3) {
    lines.push(`v ${position[i]} ${position[i + 1]} ${position[i + 2]}`);
  }
  for (let i = 0; i + 2 < index.length; i += 3) {
    lines.push(`f ${index[i]! + 1} ${index[i + 1]! + 1} ${index[i + 2]! + 1}`);
  }
  return `${lines.join("\n")}\n`;
}

async function downloadFlowOutputExport(format: string, resolvedValueJson: string, widgetId: string): Promise<void> {
  await ensureFlowWasmLoaded();
  const { export_drawing_svg, render_drawing_scene, tessellate } = await import("@semio-tech/flow-core/pkg/flow_core.js");
  const parsed = JSON.parse(resolvedValueJson) as unknown;
  const handle = flowExportCollectHandle(parsed);
  const normalized = format.trim().toLowerCase();
  const baseName = `flow-export-${widgetId}`;
  if (!handle) {
    console.log(`[DEBUG] flow export skipped: no media handle in payload for ${widgetId}`);
    return;
  }
  if (normalized === "svg") {
    const payload = JSON.parse(export_drawing_svg(handle)) as { svg?: string; error?: string };
    if (!payload.svg) throw new Error(payload.error ?? "svg export failed");
    downloadMediaExportResult({ data: payload.svg, mimeType: "image/svg+xml", fileName: `${baseName}.svg` });
    return;
  }
  if (normalized === "png") {
    const sceneJson = render_drawing_scene(handle);
    const scene = JSON.parse(sceneJson) as { error?: string };
    if (scene.error) throw new Error(scene.error);
    const png = canvasDrawingPngExportPort.exportPng(scene);
    downloadMediaExportResult({ data: png, mimeType: "image/png", fileName: `${baseName}.png` });
    return;
  }
  const mesh = JSON.parse(tessellate(handle, 0.25)) as FlowExportMesh;
  if (mesh.error) throw new Error(mesh.error);
  if (normalized === "obj") {
    downloadMediaExportResult({ data: flowExportMeshToObj(mesh), mimeType: "text/plain", fileName: `${baseName}.obj` });
    return;
  }
  if (normalized === "glb") {
    downloadMediaExportResult({ data: JSON.stringify(mesh), mimeType: "application/json", fileName: `${baseName}.mesh.json` });
    return;
  }
  throw new Error(`unsupported export format: ${format}`);
}

/** @emoji 🔔 Re-renders flow play workbench kinds when WASM catalogue sections arrive. */
function useFlowPlaySnapshotRevision(runtime: Platform, selector: (ctrl: FlowPlayController) => number): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as FlowPlayController | undefined;
      flowPlayControllerRef.current = ctrl ?? null;
      const unsubscribeChrome = runtime.subscribeChrome(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeChrome();
        unsubscribeSnapshot?.();
      };
    },
    () => {
      const ctrl = runtime.getActiveApp()?.controller as FlowPlayController | undefined;
      flowPlayControllerRef.current = ctrl ?? null;
      return ctrl ? selector(ctrl) : 0;
    },
    () => 0,
  );
}

function useFlowPlayCatalogueRevision(runtime: Platform): number {
  return useFlowPlaySnapshotRevision(runtime, (c) => c.getCatalogueRevision());
}

function useFlowPlayExtensionRevision(runtime: Platform): number {
  return useFlowPlaySnapshotRevision(runtime, (c) => c.getExtensionRevision());
}

function useFlowPlayInteractionRevision(runtime: Platform): number {
  return useFlowPlaySnapshotRevision(runtime, (c) => c.getInteractionRevision());
}

function useFlowPlayHoverRevision(runtime: Platform): number {
  return useFlowPlaySnapshotRevision(runtime, (c) => c.getHoverEpoch());
}

function useFlowPlaySelectRevision(runtime: Platform): number {
  return useFlowPlaySnapshotRevision(runtime, (c) => c.getSelectEpoch());
}

function useFlowPlayController(runtimeOverride?: Platform): FlowPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as FlowPlayController | undefined;
  flowPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function FlowPlayPaneSurfaceHost({ node }: { readonly node: UiFlowHostSurfaceNode }): ReactElement {
  const { runtime } = useApp();
  const ctrl = useFlowPlayController();
  const extensionRevision = useFlowPlayExtensionRevision(runtime);
  const hoverRevision = useFlowPlayHoverRevision(runtime);
  const selectRevision = useFlowPlaySelectRevision(runtime);
  const scopeId = node.paneId ?? FLOW_PLAY_WINDOW_KIND_ID;
  const lodProps = dagLodCanvasProps(ctrl?.lodModeForScope(scopeId) ?? DAG_LOD_MODE_AUTOMATIC);
  const proximityDistance = ctrl?.proximityDistanceValue() ?? FLOW_DEFAULT_PROXIMITY_DISTANCE;
  const onLodChange = reactHostPort.useCallback(
    (lod: import("@semio-tech/flow-react").DagDrawLodKind) => {
      ctrl?.run("setEffectiveLod", { lod, instanceId: scopeId });
    },
    [ctrl, scopeId],
  );
  const onPreviewText = reactHostPort.useCallback(
    (text: string) => {
      console.log(`[DEBUG] flow play preview: ${text}`);
      ctrl?.run("setPreviewText", { text });
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
  const onCanvasCommand = reactHostPort.useCallback(
    (command: string, args?: Record<string, unknown>) => {
      ctrl?.run(command, args);
    },
    [ctrl],
  );
  const onSelectionChange = reactHostPort.useCallback(
    (ids: readonly string[]) => {
      ctrl?.run("setSelection", { ids: [...ids] });
    },
    [ctrl],
  );
  const onHoverChange = reactHostPort.useCallback(
    (id: string | null) => {
      ctrl?.run("setGraphHover", { id });
    },
    [ctrl],
  );
  const onChannelHoverChange = reactHostPort.useCallback(
    (channel: import("@semio-tech/flow-react").FlowChannelRef | null) => {
      ctrl?.run("setGraphChannelHover", { channel });
    },
    [ctrl],
  );
  const onSelectedChannelsChange = reactHostPort.useCallback(
    (channels: readonly import("@semio-tech/flow-react").FlowChannelRef[]) => {
      ctrl?.run("setGraphChannelSelect", { channels: [...channels] });
    },
    [ctrl],
  );
  const onCompiledWireLiteralChange = reactHostPort.useCallback(
    (text: string) => {
      ctrl?.run("setCompiledWireLiteral", { text });
    },
    [ctrl],
  );
  const onOutputExport = reactHostPort.useCallback(
    (widgetId: string, format: string, resolvedValueJson: string) => {
      void downloadFlowOutputExport(format, resolvedValueJson, widgetId).catch((error) => {
        console.log(`[DEBUG] flow play export failed: ${String(error)}`);
      });
    },
    [],
  );
  return (
    <FlowCanvas
      fixtureJson={ctrl?.getFixtureJson() ?? FLOW_PLAY_DEFAULT_FIXTURE_JSON}
      fixtureDragDrop
      reorganize={ctrl?.getReorganize()}
      commandRequest={ctrl?.getCommandRequest()}
      extensionRevision={extensionRevision}
      onPreviewText={onPreviewText}
      onCatalogueReady={onCatalogueReady}
      onFixtureChange={onFixtureChange}
      onCompiledWireLiteralChange={onCompiledWireLiteralChange}
      onOutputExport={onOutputExport}
      contextMenu={(ctx) => buildFlowPlayCanvasContextMenu(ctx, onCanvasCommand)}
      selectedNodeIds={ctrl?.getSelectedNodeIds()}
      hoveredNodeId={hoverRevision >= 0 && !ctrl?.getHoveredChannel() ? (ctrl?.getGraphHighlightedNodeIds()[0] ?? null) : null}
      hoveredChannel={hoverRevision >= 0 ? (ctrl?.getHoveredChannel() ?? null) : null}
      selectedChannels={selectRevision >= 0 ? [...(ctrl?.getSelectedChannels() ?? [])] : []}
      onSelectionChange={onSelectionChange}
      onHoverChange={onHoverChange}
      onChannelHoverChange={onChannelHoverChange}
      onSelectedChannelsChange={onSelectedChannelsChange}
      {...lodProps}
      onLodChange={onLodChange}
      proximityDistance={proximityDistance}
    />
  );
}

function FlowPlayCompiledDagSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const { runtime } = useApp();
  const ctrl = useFlowPlayController();
  const interactionRevision = useFlowPlayInteractionRevision(runtime);
  const hoverRevision = useFlowPlayHoverRevision(runtime);
  const selectRevision = useFlowPlaySelectRevision(runtime);
  const document = reactHostPort.useMemo(
    () => ctrl?.getWriterDocumentCompiledDag() ?? createWriterDocument({ id: "flow-compiled-dag", languageId: "wire", text: "" }),
    [ctrl, interactionRevision],
  );
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    flowPlayControllerRef.current?.run("setWireHover", { offset });
  }, []);
  const onSelectionChange = reactHostPort.useCallback((range: { start: number; end: number }) => {
    flowPlayControllerRef.current?.run("setWireSelect", range);
  }, []);
  return (
    <WriterCanvas
      document={document}
      className="h-full min-h-0"
      onHoverChange={onHoverChange}
      onSelectionChange={onSelectionChange}
      externalHoverOccurrences={ctrl?.getWireHoverOccurrences()}
      externalHoverOccurrencesSignal={hoverRevision}
      externalSelectionOccurrences={ctrl?.getWireSelectOccurrences()}
      externalSelectionOccurrencesSignal={selectRevision}
    />
  );
}

class FlowPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: FLOW_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = flowPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildFlowPlayHierarchyTree(ctrl?.getFixtureJson() ?? FLOW_PLAY_DEFAULT_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class FlowPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: FLOW_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = flowPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildFlowPlayCatalogueTree(ctrl?.getCatalogueSections() ?? [], ctrl?.getExtensionEntries() ?? []);
        const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
        return {
          ...config,
          dragAndDropController: flowWidgetPaletteTreeDragController(collectUiTreeItemDragData(treeNode.sections)),
        };
      }),
    };
  }
}

class FlowPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: FLOW_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = flowPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildFlowPlayInspectorTree(ctrl?.getFixtureJson() ?? FLOW_PLAY_DEFAULT_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function FlowPlayGenerateSurfaceHost({ node }: { readonly node: UiFormsHostSurfaceNode }): ReactElement {
  const ctrl = useFlowPlayController();
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

function FlowPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useFlowPlayController(runtime);
  const catalogueRevision = useFlowPlayCatalogueRevision(runtime);
  const extensionRevision = useFlowPlayExtensionRevision(runtime);
  const interactionRevision = useFlowPlayInteractionRevision(runtime);
  const flowPlayHierarchyPanel = reactHostPort.useMemo(() => new FlowPlayHierarchyPanelDefinition(), []);
  const flowPlayCataloguePanel = reactHostPort.useMemo(() => new FlowPlayCataloguePanelDefinition(), []);
  const flowPlayInspectionPanel = reactHostPort.useMemo(() => new FlowPlayInspectionPanelDefinition(), []);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [flowPlayHierarchyPanel, flowPlayCataloguePanel],
      details: [flowPlayInspectionPanel],
    }),
    [catalogueRevision, extensionRevision, interactionRevision, flowPlayCataloguePanel, flowPlayHierarchyPanel, flowPlayInspectionPanel],
  );
  return <PlaygroundView runtime={runtime} defaultAppId={FLOW_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} />;
}

export function registerFlowPlaySurfaceHosts(): void {
  if (flowPlayChromeRegistered) return;
  flowPlayChromeRegistered = true;
  registerUiFlowSurfaceHost(FLOW_PLAY_SURFACE_ID, FlowPlayPaneSurfaceHost);
  registerUiFormsSurfaceHost(FLOW_PLAY_SURFACE_ID_GENERATE, FlowPlayGenerateSurfaceHost);
  registerUiWriterSurfaceHost(FLOW_PLAY_SURFACE_ID_COMPILED_DAG, FlowPlayCompiledDagSurfaceHost);
  registerFlowPlayDeclarativeBodies();
}

function FlowPlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <FlowPlayInner runtime={runtime} />;
}

export function mountFlowPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<FlowPlayChrome runtime={playground.runtime} />, rootId);
}

const flowPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerFlowPlaySurfaceHosts,
  mount: mountFlowPlayChrome,
};

export function bootFlowPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, flowPlayChromeBoot, rootId);
}
//#endregion 🔖FlowPlayHost