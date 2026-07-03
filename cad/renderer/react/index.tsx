// #region 🧲Header
/** @emoji 📐 CAD play React host — viewport chrome, model space, playground root. */
// #endregion 🧲Header

import "./globals.css";

import {
  Button,
  Label,
  NavbarExampleSelect,
  NAVBAR_NO_EXAMPLE_ID,
  reactHostPort,
  formatNumber,
  type EngagementSpec,
  type TreeDataItem,
  type TreeDataSection,
  type UiTranslationKey,
} from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import { StrictMode, type ChangeEvent, type ReactNode } from "react";
// #endregion 🔌Adapters

//#region 🪁I18n Compile Gate
const _cadPlayToolbarI18nKeys = [
  "ui.toolbar.parent.view",
  "ui.toolbar.parent.save",
  "ui.toolbar.parent.transform",
  "ui.toolbar.parent.transfer",
] as const satisfies readonly UiTranslationKey[];
//#endregion 🪁I18n Compile Gate
import {
  PlaygroundView,
  CallbackTreePanelDefinition,
  PureSidePanelTabDefinition,
  StaticTreePanelDefinition,
  engagementSpecControlMirror,
  mountPlaygroundApp,
  uiTreeNodeToTreePanelConfig,
  type SidePanelTabConfig,
} from "@semio-tech/framework-playground-renderer-react";
import { uiDeclarativeSectionsToTree, type UiNode, type UiTreeNode, Platform, isPlaygroundExampleLocked, playgroundLockedExampleId, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL } from "@semio-tech/framework-playground-core";
import { AppPointerFocusStore } from "@semio-tech/framework-core";
import { registerSurfaceBinding, useShellWindowInstance, type UiCadHostSurfaceNode } from "@semio-tech/framework-platform-renderer-react";
import { defaultConstructRunner } from "@semio-tech/cad-js-query";
import { BrepjsKernel, preciseSpatialKernelMath } from "@semio-tech/cad-js-kernel-brepjs";
import { statelyStateEngineProvider } from "@semio-tech/cad-js-machine-stately";
import {
  defaultModelDefinitionId,
  Model,
  ModelSpace,
  parseModelJson,
  type InteractionSpec,
  type ModelDocument,
  type ModelDiff,
  type SelectionTarget,
  type SpatialComputeMode,
  type TransformationSpec,
  applyTransformation,
  loadSpatialInteraction,
  listSpatialInteractionsForModelDefinition,
  isShapeModelDefinition,
  listTransformationsIntoModelDefinition,
  listModelObjectsForModelDefinition,
  applyModelDiff,
  deleteObjectsFromModel,
  deletableObjectIdsFromSelection,
  type CadGumballConfig,
} from "@semio-tech/cad-js-core";
import {
  InteractionRepl,
  modelHasCommittedSolidsForDisplay,
  modelHasFactoryFaceDisplay,
  replDisplayedSelectionTargets,
  replWithRendererSelectionTargets,
  pruneSelectionTargetsForEntityFlags,
  r3fPreviewKernel,
  canvasHoverKeyForSelectionTarget,
  spatialHoverKeyAliases,
  spatialHoverKeysMatch,
  selectionTargetHoverKey,
  spatialPickTargetKey,
  useDocumentHistory,
  useInteractionRuntime,
  type SpatialInteractionSelectionByState,
  type SpatialPickTarget,
  type SpatialRendererSelectionByModel,
} from "../js/index.tsx";


import {
  CAD_PLAY_APP_ID,
  CAD_PLAY_CONTROLLER_ID,
  CAD_PLAY_HIERARCHY_TAB_ID,
  CAD_PLAY_PANE_SPECS,
  CAD_PLAY_CONCRETE_FOREST_FIXTURE_IDS,
  CadPlayShellController,
  buildCadPlayRuntime,
  buildCadPlayHierarchyPendingSections,
  buildCadPlayHierarchySections,
  buildCadPlayDetailsTree,
  buildCadPlayCatalogTree,
  cadPlaySceneSurfaceIdForPane,
  cadPlayPaneFromSurfaceId,
  cadPlayPaneFromWindowKindId,
  cadPlayPaneFromShellWindowId,
  cadPlayDefaultReferencesByModelDefinitionId,
  cadPlayReferencesForFixture,
  cadPlayIsConcreteForestFixture,
  ensurePlayQuadModelSlots,
  ensureCadPlayQuadModels,
  cadPlayPaneGeometry,
  modelsFromCadJson,
  activeModelDefinitionIdFromSpatialJson,
  flushModelsRecord,
  modelSpaceFromRecord,
  recordFromModelSpace,
  type CadPlayChromeSnapshot,
  type CadPlayPaneId,
  type CadPlayReferencesByModelDefinitionId,
  type CadPlaySelectedReference,
} from "@semio-tech/cad-js-renderer-core";
import { CAD_PLAY_SHAPE_ASSETS, resolveCadPlayFixtureSlug } from "@semio-tech/cad-js-renderer-core";

const CadPlayHierarchyIcon = createIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID);
const CadPlayCatalogueIcon = createIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID);
const CadPlayInspectionIcon = createIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID);

//#region 🔖GeometryCatalog
function modelVertexCount(json: Record<string, unknown>): number {
  const bundle = json.modelSpace ? (json as SpatialExchangeBundle) : null;
  const modelSpace = parseModelSpaceJson(bundle?.modelSpace ?? json);
  if (modelSpace) return Object.values(modelSpace.models).reduce((count, model) => count + Object.keys(model.vertices).length, 0);
  const model = parseModelJson(json);
  if (model) return Object.keys(model.vertices).length;
  const geo = json.geometry;
  if (geo && typeof geo === "object") {
    const nested = (geo as Record<string, unknown>).vertices;
    if (Array.isArray(nested)) return nested.length;
  }
  const verts = json.vertices;
  return Array.isArray(verts) ? verts.length : 0;
}

const PLAY_REPL_SPEC: InteractionSpec = {
  schema: "spatial.interaction/v1",
  id: "",
  version: "1.0.0",
  label: "Play",
  machine: {
    initial: "idle",
    states: [{ name: "idle" }],
  },
  display: {
    states: [{ state: "idle", items: [] }],
  },
  commit: {
    fromStates: [],
    operation: { kind: "action", action: "play.repl.noop" },
  },
};

type ModelJsonSnapshot = ReturnType<Model["toJSON"]>;
type ModelSpaceJsonSnapshot = ReturnType<ModelSpace["toJSON"]>;

interface SpatialExchangeBundle {
  readonly model?: ModelJsonSnapshot;
  readonly modelSpace?: ModelSpaceJsonSnapshot;
  readonly activeModelDefinitionId?: string;
}

interface SaveFilePickerTypeOption {
  readonly description?: string;
  readonly accept: Record<string, readonly string[]>;
}

interface SaveFilePickerOptionsLike {
  readonly suggestedName?: string;
  readonly types?: readonly SaveFilePickerTypeOption[];
  readonly excludeAcceptAllOption?: boolean;
}

interface FileSystemWritableFileStreamLike {
  write(data: string): Promise<void>;
  close(): Promise<void>;
}

interface FileSystemFileHandleLike {
  createWritable(): Promise<FileSystemWritableFileStreamLike>;
}

interface SavePickerWindow extends Window {
  showSaveFilePicker?: (options?: SaveFilePickerOptionsLike) => Promise<FileSystemFileHandleLike>;
}

function emptyPlayModels(): Record<string, Model> {
  return ensurePlayQuadModelSlots({});
}

function fileStem(name: string): string {
  const trimmed = name.trim();
  if (!trimmed) return "spatial";
  return (
    trimmed
      .replace(/\.analytic\.spatial\.json$/i, "")
      .replace(/\.raw\.spatial\.json$/i, "")
      .replace(/\.spatial\.json$/i, "")
      .replace(/\.json$/i, "")
      .replace(/[^a-z0-9._-]+/gi, "-")
      .replace(/^-+|-+$/g, "") || "spatial"
  );
}

function selectRawModel(model: Model, selection: readonly SelectionTarget[]): ModelJsonSnapshot {
  const selectedModel = new Model();
  selectedModel.revision = model.revision;
  const anchors = new Set<string>();
  const vertices = new Set<string>();
  const edges = new Set<string>();
  const wires = new Set<string>();
  const faces = new Set<string>();
  const shells = new Set<string>();
  const solids = new Set<string>();
  const visitById = (id: string): void => {
    if (model.anchors[id]) {
      visitAnchor(id);
      return;
    }
    if (model.vertices[id]) {
      visitVertex(id);
      return;
    }
    if (model.edges[id]) {
      visitEdge(id);
      return;
    }
    if (model.wires[id]) {
      visitWire(id);
      return;
    }
    if (model.faces[id]) {
      visitFace(id);
      return;
    }
    if (model.shells[id]) {
      visitShell(id);
      return;
    }
    if (model.solids[id]) {
      visitSolid(id);
      return;
    }
  };

  const visitAnchor = (id: string): void => {
    if (anchors.has(id)) return;
    const rec = model.anchors[id];
    if (!rec) return;
    anchors.add(id);
    visitById(rec.attachment.id);
  };

  const visitVertex = (id: string): void => {
    if (vertices.has(id) || !model.vertices[id]) return;
    vertices.add(id);
  };

  const visitEdge = (id: string): void => {
    if (edges.has(id)) return;
    const rec = model.edges[id];
    if (!rec) return;
    edges.add(id);
    for (const vertexId of rec.vertexIds) visitVertex(vertexId);
  };

  const visitWire = (id: string): void => {
    if (wires.has(id)) return;
    const rec = model.wires[id];
    if (!rec) return;
    wires.add(id);
    for (const edgeId of rec.edgeIds) visitEdge(edgeId);
  };

  const visitFace = (id: string): void => {
    if (faces.has(id)) return;
    const rec = model.faces[id];
    if (!rec) return;
    faces.add(id);
    for (const wireId of rec.wireIds) visitWire(wireId);
  };

  const visitShell = (id: string): void => {
    if (shells.has(id)) return;
    const rec = model.shells[id];
    if (!rec) return;
    shells.add(id);
    for (const faceId of rec.faceIds) visitFace(faceId);
  };

  const visitSolid = (id: string): void => {
    if (solids.has(id)) return;
    const rec = model.solids[id];
    if (!rec) return;
    solids.add(id);
    for (const shellId of rec.shellIds) visitShell(shellId);
  };

  for (const target of selection) {
    switch (target.kind) {
      case "object": {
        const object = model.objects[target.id];
        if (!object) break;
        selectedModel.objects[object.id] = object;
        for (const primitiveId of Object.values(object.primitives)) visitById(primitiveId);
        break;
      }
      case "anchor":
        visitAnchor(target.id);
        break;
      case "vertex":
        visitVertex(target.id);
        break;
      case "edge":
        visitEdge(target.id);
        break;
      case "wire":
        visitWire(target.id);
        break;
      case "face":
        visitFace(target.id);
        break;
      case "shell":
        visitShell(target.id);
        break;
      case "solid":
        visitSolid(target.id);
        break;
      default:
        break;
    }
  }

  const sortIds = (ids: Set<string>) => [...ids].sort((a, b) => a.localeCompare(b));
  selectedModel.anchors = Object.fromEntries(sortIds(anchors).map((id) => [id, model.anchors[id]!])) as typeof selectedModel.anchors;
  selectedModel.vertices = Object.fromEntries(sortIds(vertices).map((id) => [id, model.vertices[id]!])) as typeof selectedModel.vertices;
  selectedModel.edges = Object.fromEntries(sortIds(edges).map((id) => [id, model.edges[id]!])) as typeof selectedModel.edges;
  selectedModel.wires = Object.fromEntries(sortIds(wires).map((id) => [id, model.wires[id]!])) as typeof selectedModel.wires;
  selectedModel.faces = Object.fromEntries(sortIds(faces).map((id) => [id, model.faces[id]!])) as typeof selectedModel.faces;
  selectedModel.shells = Object.fromEntries(sortIds(shells).map((id) => [id, model.shells[id]!])) as typeof selectedModel.shells;
  selectedModel.solids = Object.fromEntries(sortIds(solids).map((id) => [id, model.solids[id]!])) as typeof selectedModel.solids;
  selectedModel.metadata.loadSnapshot(model.metadata.toJSON(), false);
  return selectedModel.toJSON();
}

async function writeTextFile(name: string, text: string, types: readonly SaveFilePickerTypeOption[], fallbackMime = "application/octet-stream"): Promise<void> {
  const pickerWindow = window as SavePickerWindow;
  if (pickerWindow.showSaveFilePicker) {
    const handle = await pickerWindow.showSaveFilePicker({ suggestedName: name, types });
    const writable = await handle.createWritable();
    await writable.write(text);
    await writable.close();
    return;
  }
  const href = URL.createObjectURL(new Blob([text], { type: fallbackMime }));
  const link = document.createElement("a");
  link.href = href;
  link.download = name;
  link.click();
  URL.revokeObjectURL(href);
}

async function writeJsonFile(name: string, payload: SpatialExchangeBundle): Promise<void> {
  await writeTextFile(name, `${JSON.stringify(payload, null, 2)}\n`, [{ description: "Spatial JSON", accept: { "application/json": [".json", ".spatial.json"] } }], "application/json");
}

async function writeStepFile(name: string, stepText: string): Promise<void> {
  await writeTextFile(name, stepText, [{ description: "STEP AP242", accept: { "application/step": [".stp", ".step"], "model/step": [".stp", ".step"] } }], "application/step");
}

function sanitizeModelDefinitionFileStem(modelDefinitionId: string): string {
  return modelDefinitionId.replace(/[^a-z0-9._-]+/gi, "-").replace(/^-+|-+$/g, "") || "model";
}

interface CadPlayChromeContextValue {
  readonly snapshot: CadPlayChromeSnapshot | null;
  readonly publishSnapshot: (snapshot: CadPlayChromeSnapshot | null) => void;
}

const CadPlayChromeContext = reactHostPort.createContext<CadPlayChromeContextValue | null>(null);

function useCadPlayChrome(): CadPlayChromeContextValue {
  const value = reactHostPort.useContext(CadPlayChromeContext);
  if (!value) {
    throw new Error("useCadPlayChrome must be used inside CadPlayChromeContext.");
  }
  return value;
}

function useCadPlayChromePublish(): (snapshot: CadPlayChromeSnapshot | null) => void {
  return useCadPlayChrome().publishSnapshot;
}

const cadPlayChromeSnapshotRef: { current: CadPlayChromeSnapshot | null } = { current: null };

class CadPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: CAD_PLAY_HIERARCHY_TAB_ID,
      icon: CadPlayHierarchyIcon,
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const snapshot = cadPlayChromeSnapshotRef.current;
        if (!snapshot) return buildCadPlayHierarchyPendingSections();
        const build = buildCadPlayHierarchySections(
          snapshot.modelsByDefinitionId,
          snapshot.activeModelDefinitionId,
          snapshot.selection,
          snapshot.selectTarget,
          snapshot.hoverTarget,
          snapshot.toggleHidden,
          snapshot.toggleLocked,
          snapshot.referencesByModelDefinitionId,
          snapshot.selectedReference,
          snapshot.selectReference,
          snapshot.hoverReference,
          snapshot.toggleReferenceHidden,
          snapshot.toggleReferenceLocked,
        );
        const hoveredKey = snapshot.hoveredKey;
        const highlightedIds: string[] = [];
        if (hoveredKey) {
          const ids = new Set<string>();
          for (const alias of spatialHoverKeyAliases(hoveredKey)) {
            for (const itemId of build.highlightKeyToItemIds[alias] ?? []) ids.add(itemId);
          }
          highlightedIds.push(...ids);
        }
        return { sections: build.sections, highlightedIds };
      }),
    };
  }
}
//#endregion 🔖CadPlayChrome

//#region 🔖PlaySession
interface PlaySessionProps {
  readonly interactionId: string;
  readonly spec: InteractionSpec;
  readonly onInteractionId: (id: string) => void;
  readonly documentModel: ModelDocument;
  readonly history: DocumentHistory;
  readonly kernel: InteractionRuntimeOptions["kernel"];
  readonly mode: SpatialComputeMode;
  readonly asideExtra: ReactNode;
  readonly sessionRestartNonce: number;
  readonly activeModelDefinitionId: string;
  readonly onActiveModelDefinitionId: (value: string) => void;
  readonly rendererSelectionByModel: SpatialRendererSelectionByModel;
  readonly onRendererSelectionByModel: (value: SpatialRendererSelectionByModel) => void;
  readonly interactionSelectionByState: SpatialInteractionSelectionByState;
  readonly onInteractionSelectionByState: (value: SpatialInteractionSelectionByState) => void;
  readonly modelDefinitionRevision: number;
  readonly onModelDefinitionRevision: (revision: number) => void;
  readonly onApplyTransformation: (spec: TransformationSpec) => void;
  readonly pickGeometry: Model;
  readonly onDocumentModelChange: (model: Model) => void;
  readonly onSnapshot: (snapshot: InteractionSnapshot) => void;
  readonly onEngagementChange: (engagement: EngagementSpec | null) => void;
  readonly captureGlobalKeys: boolean;
  readonly hoveredPickKey: string | null;
  readonly onHoveredPickKeyChange: (key: string | null) => void;
  readonly onCanvasHoverTarget: (target: SpatialPickTarget | null) => void;
  readonly autoFitMeshes?: boolean;
  readonly autoFitBehavior?: "initial" | "always";
  readonly transformGumballConfig: CadGumballConfig | null;
  readonly onTransformGumballCommit: (diff: ModelDiff) => void;
  readonly onDeleteSelection: () => boolean;
  readonly committedMeshesKeepPrevious?: boolean;
  readonly cameraView?: OrbitCameraViewId;
  readonly cameraViewSeedKey?: string | number;
  readonly orbitProjection?: OrbitCameraProjection;
  readonly onOrbitProjectionChange?: (projection: OrbitCameraProjection) => void;
  readonly worldReferences?: readonly WorldReferenceProps[];
  readonly selectedReferenceIds?: ReadonlySet<string>;
  readonly hoveredReferenceId?: string | null;
  readonly revealedReferenceIds?: ReadonlySet<string>;
  readonly referenceRelocateActive?: boolean;
  readonly onReferenceSelect?: (id: string, modifiers: { readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean }) => void;
  readonly onReferenceHover?: (id: string | null) => void;
  readonly onReferenceRelocate?: (payload: WorldReferenceRelocatePayload) => void;
}

/** @emoji 🎮 Hosts `useInteractionRuntime` + `InteractionRepl`; same-interaction restarts use `sessionRestartNonce` without remounting GL. */
function PlaySession({
  interactionId,
  spec,
  onInteractionId,
  documentModel,
  history,
  kernel,
  mode,
  asideExtra,
  sessionRestartNonce,
  activeModelDefinitionId,
  onActiveModelDefinitionId,
  rendererSelectionByModel,
  onRendererSelectionByModel,
  interactionSelectionByState,
  onInteractionSelectionByState,
  modelDefinitionRevision,
  onModelDefinitionRevision,
  onApplyTransformation,
  pickGeometry,
  onDocumentModelChange,
  onSnapshot,
  onEngagementChange,
  captureGlobalKeys,
  hoveredPickKey,
  onHoveredPickKeyChange,
  onCanvasHoverTarget,
  autoFitMeshes = false,
  autoFitBehavior = "initial",
  committedMeshesKeepPrevious = true,
  transformGumballConfig,
  onTransformGumballCommit,
  onDeleteSelection,
  cameraView,
  cameraViewSeedKey,
  orbitProjection,
  onOrbitProjectionChange,
  worldReferences = [],
  selectedReferenceIds,
  hoveredReferenceId = null,
  revealedReferenceIds,
  referenceRelocateActive = true,
  onReferenceSelect,
  onReferenceHover,
  onReferenceRelocate,
}: PlaySessionProps) {
  const rtOpts = reactHostPort.useMemo(
    (): InteractionRuntimeOptions => ({
      kernel,
      previewKernel: r3fPreviewKernel,
      mode,
      document: documentModel,
      history,
      stateEngine: statelyStateEngineProvider,
      query: defaultConstructRunner,
      activeModelDefinitionId,
    }),
    [kernel, mode, documentModel, history, activeModelDefinitionId],
  );
  const rt = useInteractionRuntime(spec, rtOpts);
  reactHostPort.useEffect(() => {
    return rt.subscribe(() => {
      const snap = rt.getSnapshot();
      onSnapshot(snap);
      const res = snap.lastResponse;
      if (res?.ok && res.diff && !isEmptyModelDiff(res.diff)) {
        onDocumentModelChange(Model.fromJSON(documentModel.model.toJSON()));
        onModelDefinitionRevision((revision) => revision + 1);
      }
    });
  }, [rt, documentModel, onSnapshot, onDocumentModelChange, onModelDefinitionRevision]);
  return (
    <InteractionRepl
      fillHost
      showAside={false}
      showEngagement
      interactionId={interactionId}
      spec={spec}
      onInteractionId={onInteractionId}
      runtime={rt}
      history={history}
      document={documentModel}
      geometry={documentModel.model}
      pickGeometry={pickGeometry}
      onDocumentModelChange={onDocumentModelChange}
      asideExtra={asideExtra}
      sessionRestartNonce={sessionRestartNonce}
      activeModelDefinitionId={activeModelDefinitionId}
      onActiveModelDefinitionIdChange={onActiveModelDefinitionId}
      rendererSelectionByModel={rendererSelectionByModel}
      onRendererSelectionByModelChange={onRendererSelectionByModel}
      interactionSelectionByState={interactionSelectionByState}
      onInteractionSelectionByStateChange={onInteractionSelectionByState}
      modelDefinitionRevision={modelDefinitionRevision}
      onModelDefinitionRevisionChange={onModelDefinitionRevision}
      onApplyTransformation={onApplyTransformation}
      hideModelDefinitionControls
      onSnapshotChange={onSnapshot}
      onEngagementChange={onEngagementChange}
      captureGlobalKeys={captureGlobalKeys}
      hoveredPickKey={hoveredPickKey}
      onHoveredPickKeyChange={onHoveredPickKeyChange}
      onHoverTarget={onCanvasHoverTarget}
      autoFitMeshes={autoFitMeshes}
      autoFitBehavior={autoFitBehavior}
      committedMeshesKeepPrevious={committedMeshesKeepPrevious}
      transformGumballConfig={transformGumballConfig}
      onTransformGumballCommit={onTransformGumballCommit}
      onDeleteSelection={onDeleteSelection}
      spatialView={{
        ...(cameraView !== undefined && cameraViewSeedKey !== undefined ? { cameraView, cameraViewSeedKey } : {}),
        orbitProjection,
        onOrbitProjectionChange,
      }}
      worldReferences={worldReferences}
      selectedReferenceIds={selectedReferenceIds}
      hoveredReferenceId={hoveredReferenceId}
      revealedReferenceIds={revealedReferenceIds}
      referenceRelocateActive={referenceRelocateActive}
      onReferenceSelect={onReferenceSelect}
      onReferenceHover={onReferenceHover}
      onReferenceRelocate={onReferenceRelocate}
    />
  );
}
//#endregion

//#region 🔖CadPlayModelSpace
interface CadPlayModelSpaceValue {
  readonly activeModelDefinitionId: string;
  readonly setActiveModelDefinitionId: (value: string) => void;
  readonly focusModelDefinition: (modelDefinitionId: string) => void;
  readonly interactionIdForPane: (pane: CadPlayPaneId) => string;
  readonly handleInteractionPickForPane: (pane: CadPlayPaneId, id: string) => void;
  readonly specForPane: (pane: CadPlayPaneId) => InteractionSpec;
  readonly documentModel: ModelDocument;
  readonly history: DocumentHistory;
  readonly kernel: InteractionRuntimeOptions["kernel"];
  readonly computeModeForPane: (pane: CadPlayPaneId) => SpatialComputeMode;
  readonly transformGumballConfigForPane: (pane: CadPlayPaneId) => CadGumballConfig | null;
  readonly sessionRestartNonceForPane: (pane: CadPlayPaneId) => number;
  readonly rendererSelectionByModel: SpatialRendererSelectionByModel;
  readonly setRendererSelectionByModel: (value: SpatialRendererSelectionByModel) => void;
  readonly interactionSelectionByState: SpatialInteractionSelectionByState;
  readonly setInteractionSelectionByState: (value: SpatialInteractionSelectionByState) => void;
  readonly modelDefinitionRevision: number;
  readonly setModelDefinitionRevision: (value: number | ((revision: number) => number)) => void;
  readonly handleApplyTransformation: (spec: TransformationSpec) => void;
  readonly pickGeometry: Model;
  readonly handleModelAttributesChange: (model: Model) => void;
  readonly commitModelForDefinition: (modelDefinitionId: string, model: Model) => void;
  readonly handleSnapshotChangeForPane: (pane: CadPlayPaneId, snapshot: InteractionSnapshot) => void;
  readonly handleEngagementChangeForPane: (pane: CadPlayPaneId, engagement: EngagementSpec | null) => void;
  readonly flushedModelsByDefinitionId: Record<string, Model>;
  readonly playModelSpace: ModelSpace;
  readonly viewObjectCount: number;
  readonly selectionInScope: readonly SelectionTarget[];
  readonly hoveredPickKey: string | null;
  readonly onHoveredPickKeyChange: (key: string | null) => void;
  readonly onCanvasHoverTarget: (target: SpatialPickTarget | null) => void;
  readonly shapeAssetId: string;
  readonly handleShapeAssetChange: (id: string) => void;
  readonly fileStatus: string;
  readonly loadInputRef: React.RefObject<HTMLInputElement | null>;
  readonly exportBaseName: string;
  readonly handleSaveSelected: () => Promise<void>;
  readonly handleSaveInPlay: () => Promise<void>;
  readonly handleSaveCurrent: () => Promise<void>;
  readonly handleLoadRawRequest: () => void;
  readonly handleLoadRaw: (event: ChangeEvent<HTMLInputElement>) => Promise<void>;
  readonly liveModel: Model;
  readonly brepjsKernel: BrepjsKernel;
  readonly handleTransformGumballCommit: (modelDefinitionId: string, diff: ModelDiff) => void;
  readonly handleDeleteSelection: () => boolean;
  readonly cameraViewSeedForInstance: (instanceId?: string | null) => { readonly view: OrbitCameraViewId; readonly seedKey: string } | null;
  readonly orbitProjectionForInstance: (instanceId?: string | null) => OrbitCameraProjection;
  readonly applyOrbitViewForInstance: (view: OrbitCameraViewId, instanceId?: string | null) => void;
  readonly setOrbitProjectionForInstance: (projection: OrbitCameraProjection, instanceId?: string | null) => void;
  readonly modelsLoadEpoch: number;
  readonly referencesByModelDefinitionId: CadPlayReferencesByModelDefinitionId;
  readonly referencesForModelDefinition: (modelDefinitionId: string) => readonly WorldReferenceProps[];
  readonly selectedReference: CadPlaySelectedReference | null;
  readonly hoveredReferenceId: string | null;
  readonly revealedReferenceIds: ReadonlySet<string>;
  readonly handleReferenceSelect: (modelDefinitionId: string, referenceId: string) => void;
  readonly handleReferenceHover: (modelDefinitionId: string, referenceId: string | null) => void;
  readonly handleReferenceRelocate: (modelDefinitionId: string, payload: WorldReferenceRelocatePayload) => void;
}

const CadPlayModelSpaceContext = reactHostPort.createContext<CadPlayModelSpaceValue | null>(null);

function useCadPlayModelSpace(): CadPlayModelSpaceValue {
  const value = reactHostPort.useContext(CadPlayModelSpaceContext);
  if (!value) {
    throw new Error("useCadPlayModelSpace must be used inside CadPlayModelSpaceProvider.");
  }
  return value;
}

function CadPlayModelSpaceProvider({ children, runtime, shellController }: { readonly children: ReactNode; readonly runtime: Platform; readonly shellController: CadPlayShellController }) {
  const shellGeneration = reactHostPort.useSyncExternalStore(
    (onStoreChange) => runtime.subscribe(onStoreChange),
    () => runtime.generation,
    () => 0,
  );
  void shellGeneration;
  const computeModeForPane = reactHostPort.useCallback((pane: CadPlayPaneId) => shellController.getComputeModeForPane(pane), [shellController, shellGeneration]);
  const transformGumballConfigForPane = reactHostPort.useCallback((pane: CadPlayPaneId) => shellController.getTransformGumballConfigForPane(pane), [shellController, shellGeneration]);
  const cameraViewSeedForInstance = reactHostPort.useCallback(
    (instanceId?: string | null) => shellController.getCameraViewSeedForInstance(instanceId),
    [shellController, shellGeneration],
  );
  const orbitProjectionForInstance = reactHostPort.useCallback(
    (instanceId?: string | null) => shellController.getOrbitProjectionForInstance(instanceId),
    [shellController, shellGeneration],
  );
  const applyOrbitViewForInstance = reactHostPort.useCallback(
    (view: OrbitCameraViewId, instanceId?: string | null) => {
      shellController.run(ORBIT_CAMERA_VIEW_COMMAND, { view, instanceId: instanceId ?? undefined });
    },
    [shellController],
  );
  const setOrbitProjectionForInstance = reactHostPort.useCallback(
    (projection: OrbitCameraProjection, instanceId?: string | null) => {
      shellController.run("setOrbitProjection", { projection, instanceId: instanceId ?? undefined });
    },
    [shellController],
  );
  const publishCadPlayChrome = useCadPlayChromePublish();
  const pointerFocusRef = reactHostPort.useRef<AppPointerFocusStore<string> | null>(null);
  if (!pointerFocusRef.current) {
    pointerFocusRef.current = new AppPointerFocusStore<string>();
  }
  const pointerFocus = reactHostPort.useSyncExternalStore(
    (onStoreChange) => pointerFocusRef.current!.subscribe(onStoreChange),
    () => pointerFocusRef.current!.getSnapshot(),
    () => pointerFocusRef.current!.getSnapshot(),
  );
  const [activeModelDefinitionId, setActiveModelDefinitionId] = reactHostPort.useState(defaultModelDefinitionId());
  const [interactionIdByPane, setInteractionIdByPane] = reactHostPort.useState(emptyInteractionIdByPane);
  const [interactionBootIdByPane, setInteractionBootIdByPane] = reactHostPort.useState(emptyInteractionBootIdByPane);
  const [shapeAssetId, setShapeAssetId] = reactHostPort.useState("");
  const [modelsByDefinitionId, setModelsByDefinitionId] = reactHostPort.useState<Record<string, Model>>(emptyPlayModels);
  const [loadedRawName, setLoadedRawName] = reactHostPort.useState("");
  const [rendererSelectionByModel, setRendererSelectionByModel] = reactHostPort.useState<SpatialRendererSelectionByModel>({});
  const [interactionSelectionByState, setInteractionSelectionByState] = reactHostPort.useState<SpatialInteractionSelectionByState>({});
  const [modelDefinitionRevision, setModelDefinitionRevision] = reactHostPort.useState(0);
  const [modelsLoadEpoch, setModelsLoadEpoch] = reactHostPort.useState(0);
  const [snapshotByPane, setSnapshotByPane] = reactHostPort.useState(emptySnapshotByPane);
  const [referencesByModelDefinitionId, setReferencesByModelDefinitionId] = reactHostPort.useState<Record<string, WorldReferenceProps[]>>(cadPlayDefaultReferencesByModelDefinitionId);
  const [selectedReference, setSelectedReference] = reactHostPort.useState<CadPlaySelectedReference | null>(null);
  const engagementSpecRefByPane = reactHostPort.useRef<Partial<Record<CadPlayPaneId, EngagementSpec | null>>>({});
  const [fileStatus, setFileStatus] = reactHostPort.useState<string>("");
  const loadInputRef = reactHostPort.useRef<HTMLInputElement>(null);
  const specForPane = reactHostPort.useCallback((pane: CadPlayPaneId): InteractionSpec => {
    const interactionId = interactionIdByPane[pane];
    return interactionId ? (loadSpatialInteraction(interactionId) ?? PLAY_REPL_SPEC) : PLAY_REPL_SPEC;
  }, [interactionIdByPane]);
  const history = useDocumentHistory();
  const brepjsKernel = reactHostPort.useMemo(() => new BrepjsKernel(), []);
  const kernel = reactHostPort.useMemo<InteractionRuntimeOptions["kernel"]>(() => brepjsKernel as unknown as InteractionRuntimeOptions["kernel"], [brepjsKernel]);

  reactHostPort.useEffect(() => {
    setInteractionIdByPane((prev) => {
      let next: Record<CadPlayPaneId, string> | null = null;
      for (const row of CAD_PLAY_PANE_SPECS) {
        const interactionId = prev[row.pane];
        if (!interactionId) continue;
        const scoped = listSpatialInteractionsForModelDefinition(row.modelDefinitionId);
        if (scoped.some((candidate) => candidate.id === interactionId)) continue;
        if (!next) next = { ...prev };
        next[row.pane] = "";
      }
      return next ?? prev;
    });
  }, [interactionIdByPane]);

  reactHostPort.useEffect(() => {
    setModelsByDefinitionId((prev) => ensurePlayQuadModelSlots(prev));
  }, [activeModelDefinitionId]);

  const handleInteractionPickForPane = reactHostPort.useCallback((pane: CadPlayPaneId, id: string) => {
    setInteractionIdByPane((prev) => {
      if (id === prev[pane]) {
        setInteractionBootIdByPane((boot) => ({ ...boot, [pane]: boot[pane] + 1 }));
        return prev;
      }
      setInteractionBootIdByPane((boot) => ({ ...boot, [pane]: 0 }));
      return { ...prev, [pane]: id };
    });
  }, []);

  const handleShapeAssetChange = reactHostPort.useCallback((id: string) => {
    setShapeAssetId(id);
    setLoadedRawName("");
    setFileStatus("");
    if (!id) {
      pointerFocusRef.current?.clearHover();
      setModelsByDefinitionId(emptyPlayModels());
      setReferencesByModelDefinitionId(cadPlayEmptyReferencesByModelDefinitionId());
      setSelectedReference(null);
      setActiveModelDefinitionId(defaultModelDefinitionId());
      setModelsLoadEpoch((epoch) => epoch + 1);
    } else {
      const asset = CAD_PLAY_SHAPE_ASSETS.find((candidate) => candidate.id === id);
      if (!asset) return;
      setModelsByDefinitionId(modelsFromCadJson(asset.json));
      setReferencesByModelDefinitionId(cadPlayReferencesForFixture(id));
      setSelectedReference(null);
      setActiveModelDefinitionId(activeModelDefinitionIdFromSpatialJson(asset.json));
      setModelsLoadEpoch((epoch) => epoch + 1);
    }
    setModelDefinitionRevision((r) => r + 1);
  }, []);

  reactHostPort.useEffect(() => {
    const locked = playgroundLockedExampleId();
    if (locked) handleShapeAssetChange(locked);
  }, [handleShapeAssetChange]);

  const modelsForActiveDefinition = reactHostPort.useMemo(() => ensurePlayQuadModelSlots(modelsByDefinitionId), [activeModelDefinitionId, modelsByDefinitionId]);

  const activeModel = reactHostPort.useMemo(() => {
    const resolved = modelsForActiveDefinition[activeModelDefinitionId];
    if (resolved) return resolved;
    if (isShapeModelDefinition(activeModelDefinitionId)) {
      return modelsForActiveDefinition[defaultModelDefinitionId()] ?? new Model();
    }
    throw new Error(`Play model space missing model for ${activeModelDefinitionId}.`);
  }, [activeModelDefinitionId, modelsForActiveDefinition]);

  const documentModel = reactHostPort.useMemo((): ModelDocument => {
    const model = Model.fromJSON(activeModel.toJSON());
    return { model: model, nodes: [] };
  }, [activeModel, modelDefinitionRevision]);
  const liveModel = documentModel.model;

  const flushedModelsByDefinitionId = reactHostPort.useMemo(() => {
    const flushed = flushModelsRecord(modelsByDefinitionId, activeModelDefinitionId, liveModel);
    return ensurePlayQuadModelSlots(flushed);
  }, [activeModelDefinitionId, liveModel, liveModel.revision, modelsByDefinitionId, modelsLoadEpoch]);

  const playModelSpace = reactHostPort.useMemo(() => modelSpaceFromRecord(flushedModelsByDefinitionId), [flushedModelsByDefinitionId]);

  const visibleExportModel = reactHostPort.useMemo(() => flushedModelsByDefinitionId[activeModelDefinitionId] ?? liveModel, [activeModelDefinitionId, flushedModelsByDefinitionId, liveModel]);

  const pickGeometry = reactHostPort.useMemo(() => cadPlayPaneGeometry(flushedModelsByDefinitionId, activeModelDefinitionId, liveModel), [activeModelDefinitionId, flushedModelsByDefinitionId, liveModel]);

  const commitModelForDefinition = reactHostPort.useCallback((modelDefinitionId: string, model: Model) => {
    setModelsByDefinitionId((prev) => ensurePlayQuadModelSlots({ ...prev, [modelDefinitionId]: Model.fromJSON(model.toJSON()) }));
    setModelDefinitionRevision((r) => r + 1);
  }, []);

  const handleActiveModelDefinitionChange = reactHostPort.useCallback(
    (nextId: string) => {
      setModelsByDefinitionId((prev) => {
        const flushed = flushModelsRecord(prev, activeModelDefinitionId, liveModel);
        return ensurePlayQuadModelSlots(flushed);
      });
      setActiveModelDefinitionId(nextId);
      setModelDefinitionRevision((r) => r + 1);
    },
    [activeModelDefinitionId, liveModel],
  );

  const focusModelDefinition = reactHostPort.useCallback(
    (modelDefinitionId: string) => {
      if (modelDefinitionId === activeModelDefinitionId) return;
      setModelsByDefinitionId((prev) => ensurePlayQuadModelSlots(flushModelsRecord(prev, activeModelDefinitionId, liveModel)));
      setActiveModelDefinitionId(modelDefinitionId);
      setModelDefinitionRevision((r) => r + 1);
    },
    [activeModelDefinitionId, liveModel],
  );

  const handleModelAttributesChange = reactHostPort.useCallback(
    (model: Model) => {
      commitModelForDefinition(activeModelDefinitionId, model);
    },
    [activeModelDefinitionId, commitModelForDefinition],
  );

  const handleTransformGumballCommit = reactHostPort.useCallback(
    (modelDefinitionId: string, diff: ModelDiff) => {
      if (isEmptyModelDiff(diff)) return;
      const model = Model.fromJSON((flushedModelsByDefinitionId[modelDefinitionId] ?? liveModel).toJSON());
      applyModelDiff(model, diff);
      commitModelForDefinition(modelDefinitionId, model);
    },
    [commitModelForDefinition, flushedModelsByDefinitionId, liveModel],
  );

  const activePane = reactHostPort.useMemo(() => cadPlayPaneForModelDefinition(activeModelDefinitionId), [activeModelDefinitionId]);
  const activeInteractionId = activePane ? interactionIdByPane[activePane] : "";
  const activeSnapshot = activePane ? snapshotByPane[activePane] : null;
  const activeSpec = activePane ? specForPane(activePane) : PLAY_REPL_SPEC;
  const interactionActive = reactHostPort.useMemo(
    () => Boolean(activeSnapshot) && isInteractionSessionActive(activeSpec, activeSnapshot?.state ?? "idle"),
    [activeSnapshot, activeSpec],
  );
  const boundInteractionSession = Boolean(activeInteractionId) && interactionActive;
  const handleSnapshotChangeForPane = reactHostPort.useCallback((pane: CadPlayPaneId, next: InteractionSnapshot) => {
    setSnapshotByPane((prev) => {
      const current = prev[pane];
      if (current && current.revision === next.revision && current.state === next.state) return prev;
      return { ...prev, [pane]: next };
    });
  }, []);
  const currentSelection = reactHostPort.useMemo(
    () => replDisplayedSelectionTargets(boundInteractionSession, activeModelDefinitionId, activeSnapshot?.state ?? "idle", rendererSelectionByModel, interactionSelectionByState),
    [boundInteractionSession, activeModelDefinitionId, activeSnapshot?.state, rendererSelectionByModel, interactionSelectionByState],
  );
  const selectionKinds = reactHostPort.useMemo(() => new Set(modelDefinitionSelectionEntityKinds(activeModelDefinitionId)), [activeModelDefinitionId]);
  const viewObjectCount = reactHostPort.useMemo(() => countViewObjectsForModelDefinition(liveModel, activeModelDefinitionId), [liveModel, activeModelDefinitionId, modelDefinitionRevision]);

  const selectionInScope = reactHostPort.useMemo(
    () =>
      currentSelection.filter((target) => {
        if (target.kind === "object" && target.editable === false) return selectionKinds.has("object");
        return selectionKinds.has(target.kind);
      }),
    [currentSelection, selectionKinds],
  );

  reactHostPort.useEffect(() => {
    if (selectionInScope.length > 0) {
      setSelectedReference(null);
    }
  }, [selectionInScope]);

  const handleDeleteSelection = reactHostPort.useCallback((): boolean => {
    if (boundInteractionSession) return false;
    const objectIds = deletableObjectIdsFromSelection(selectionInScope);
    if (objectIds.length === 0) return false;
    const model = Model.fromJSON(liveModel.toJSON());
    deleteObjectsFromModel(model, objectIds);
    commitModelForDefinition(activeModelDefinitionId, model);
    setRendererSelectionByModel((prev) => replWithRendererSelectionTargets(prev, activeModelDefinitionId, []));
    return true;
  }, [activeModelDefinitionId, boundInteractionSession, commitModelForDefinition, liveModel, selectionInScope]);

  const selectHierarchyTarget = reactHostPort.useCallback(
    (modelDefinitionId: string, target: SelectionTarget) => {
      if (modelDefinitionId !== activeModelDefinitionId) {
        handleActiveModelDefinitionChange(modelDefinitionId);
      }
      setSelectedReference(null);
      setRendererSelectionByModel((prev) => replWithRendererSelectionTargets(prev, modelDefinitionId, [target]));
    },
    [activeModelDefinitionId, handleActiveModelDefinitionChange],
  );

  const hoverHierarchyTarget = reactHostPort.useCallback(
    (modelDefinitionId: string, target: SelectionTarget | null) => {
      if (target && modelDefinitionId !== activeModelDefinitionId) {
        handleActiveModelDefinitionChange(modelDefinitionId);
      }
      const model = flushedModelsByDefinitionId[modelDefinitionId] ?? new Model();
      const hoverKey = target ? canvasHoverKeyForSelectionTarget(model, modelDefinitionId, target) : null;
      pointerFocusRef.current!.setHoverFromSource(CAD_PLAY_HOVER_SOURCE_HIERARCHY, hoverKey);
    },
    [activeModelDefinitionId, flushedModelsByDefinitionId, handleActiveModelDefinitionChange],
  );

  const selectHierarchyReference = reactHostPort.useCallback(
    (modelDefinitionId: string, referenceId: string) => {
      const reference = (referencesByModelDefinitionId[modelDefinitionId] ?? []).find((row) => row.id === referenceId);
      if (!reference || !worldEntitySelectable(reference)) {
        return;
      }
      if (modelDefinitionId !== activeModelDefinitionId) {
        handleActiveModelDefinitionChange(modelDefinitionId);
      }
      setSelectedReference({ modelDefinitionId, id: referenceId });
      setRendererSelectionByModel((prev) => replWithRendererSelectionTargets(prev, modelDefinitionId, []));
    },
    [activeModelDefinitionId, handleActiveModelDefinitionChange, referencesByModelDefinitionId],
  );

  const hoverHierarchyReference = reactHostPort.useCallback(
    (modelDefinitionId: string, referenceId: string | null) => {
      if (referenceId && modelDefinitionId !== activeModelDefinitionId) {
        handleActiveModelDefinitionChange(modelDefinitionId);
      }
      if (!referenceId) {
        pointerFocusRef.current!.setHoverFromSource(CAD_PLAY_HOVER_SOURCE_HIERARCHY, null);
        return;
      }
      const reference = (referencesByModelDefinitionId[modelDefinitionId] ?? []).find((row) => row.id === referenceId);
      if (!reference || (reference.hidden !== true && !worldEntitySelectable(reference))) {
        pointerFocusRef.current!.setHoverFromSource(CAD_PLAY_HOVER_SOURCE_HIERARCHY, null);
        return;
      }
      pointerFocusRef.current!.setHoverFromSource(CAD_PLAY_HOVER_SOURCE_HIERARCHY, cadPlayReferenceHoverKey(referenceId));
    },
    [activeModelDefinitionId, handleActiveModelDefinitionChange, referencesByModelDefinitionId],
  );

  const toggleHierarchyReferenceFlag = reactHostPort.useCallback((modelDefinitionId: string, referenceId: string, flag: "hidden" | "locked") => {
    setReferencesByModelDefinitionId((prev) => {
      const reference = (prev[modelDefinitionId] ?? []).find((row) => row.id === referenceId);
      if (!reference) {
        return prev;
      }
      const nextReference = { ...reference, [flag]: !(reference[flag] === true) };
      setSelectedReference((selected) => {
        if (!selected || selected.modelDefinitionId !== modelDefinitionId || selected.id !== referenceId) {
          return selected;
        }
        return worldEntitySelectable(nextReference) ? selected : null;
      });
      return updateCadPlayReferenceInMap(prev, modelDefinitionId, referenceId, { [flag]: !(reference[flag] === true) });
    });
    console.log("[DEBUG] cad toggleReferenceFlag", flag, referenceId);
  }, []);

  const toggleHierarchyReferenceHidden = reactHostPort.useCallback(
    (modelDefinitionId: string, referenceId: string) => toggleHierarchyReferenceFlag(modelDefinitionId, referenceId, "hidden"),
    [toggleHierarchyReferenceFlag],
  );

  const toggleHierarchyReferenceLocked = reactHostPort.useCallback(
    (modelDefinitionId: string, referenceId: string) => toggleHierarchyReferenceFlag(modelDefinitionId, referenceId, "locked"),
    [toggleHierarchyReferenceFlag],
  );

  const referencesForModelDefinition = reactHostPort.useCallback(
    (modelDefinitionId: string) => referencesByModelDefinitionId[modelDefinitionId] ?? [],
    [referencesByModelDefinitionId],
  );

  const handleReferenceSelect = reactHostPort.useCallback(
    (modelDefinitionId: string, referenceId: string) => {
      const reference = (referencesByModelDefinitionId[modelDefinitionId] ?? []).find((row) => row.id === referenceId);
      if (!reference || !worldEntitySelectable(reference)) {
        return;
      }
      selectHierarchyReference(modelDefinitionId, referenceId);
      pointerFocusRef.current!.setHoverFromSource(CAD_PLAY_HOVER_SOURCE_CANVAS, cadPlayReferenceHoverKey(referenceId));
    },
    [referencesByModelDefinitionId, selectHierarchyReference],
  );

  const handleReferenceHover = reactHostPort.useCallback(
    (modelDefinitionId: string, referenceId: string | null) => {
      if (referenceId) {
        const reference = (referencesByModelDefinitionId[modelDefinitionId] ?? []).find((row) => row.id === referenceId);
        if (!reference || !worldEntitySelectable(reference)) {
          return;
        }
        pointerFocusRef.current!.setHoverFromSource(CAD_PLAY_HOVER_SOURCE_CANVAS, cadPlayReferenceHoverKey(referenceId));
        return;
      }
      if (pointerFocusRef.current!.getSnapshot().hover?.startsWith("reference:")) {
        pointerFocusRef.current!.setHoverFromSource(CAD_PLAY_HOVER_SOURCE_CANVAS, null);
      }
    },
    [referencesByModelDefinitionId],
  );

  const handleReferenceRelocate = reactHostPort.useCallback((modelDefinitionId: string, payload: WorldReferenceRelocatePayload) => {
    setReferencesByModelDefinitionId((prev) => {
      const rows = prev[modelDefinitionId] ?? [];
      const index = rows.findIndex((row) => row.id === payload.referenceId);
      if (index < 0) {
        return prev;
      }
      const nextRows = rows.map((row, rowIndex) => (rowIndex === index ? applyWorldReferenceTransform(row, payload.after) : row));
      return { ...prev, [modelDefinitionId]: nextRows };
    });
    console.log("[DEBUG] cad referenceRelocate", payload.referenceId);
  }, []);

  const hoveredReferenceKey = reactHostPort.useMemo(() => {
    const hoverKey = pointerFocus.hover;
    if (!hoverKey?.startsWith("reference:")) {
      return null;
    }
    return hoverKey.slice("reference:".length);
  }, [pointerFocus.hover]);

  const hoveredReferenceId = reactHostPort.useMemo(() => {
    if (!hoveredReferenceKey) {
      return null;
    }
    const reference = Object.values(referencesByModelDefinitionId)
      .flat()
      .find((row) => row.id === hoveredReferenceKey);
    return reference && worldEntitySelectable(reference) ? hoveredReferenceKey : null;
  }, [hoveredReferenceKey, referencesByModelDefinitionId]);

  const revealedReferenceIds = reactHostPort.useMemo(() => {
    if (!hoveredReferenceKey) {
      return new Set<string>();
    }
    const hiddenReference = Object.values(referencesByModelDefinitionId)
      .flat()
      .find((row) => row.id === hoveredReferenceKey && row.hidden === true);
    return hiddenReference ? new Set([hoveredReferenceKey]) : new Set<string>();
  }, [hoveredReferenceKey, referencesByModelDefinitionId]);

  const toggleHierarchyEntityFlag = reactHostPort.useCallback(
    (modelDefinitionId: string, target: SelectionTarget, flag: "hidden" | "locked") => {
      const model = modelDefinitionId === activeModelDefinitionId ? liveModel : flushedModelsByDefinitionId[modelDefinitionId];
      if (!model) {
        return;
      }
      const next = Model.fromJSON(model.toJSON());
      const current = next.getEntityFlags(target.id);
      next.setEntityFlag(target.id, flag, !(current[flag] === true));
      commitModelForDefinition(modelDefinitionId, next);
      setRendererSelectionByModel((prev) =>
        replWithRendererSelectionTargets(prev, modelDefinitionId, pruneSelectionTargetsForEntityFlags(prev[modelDefinitionId] ?? [], (id) => next.getEntityFlags(id))),
      );
      console.log("[DEBUG] cad toggleEntityFlag", flag, target);
    },
    [activeModelDefinitionId, commitModelForDefinition, flushedModelsByDefinitionId, liveModel],
  );

  const toggleHierarchyHidden = reactHostPort.useCallback(
    (modelDefinitionId: string, target: SelectionTarget) => toggleHierarchyEntityFlag(modelDefinitionId, target, "hidden"),
    [toggleHierarchyEntityFlag],
  );

  const toggleHierarchyLocked = reactHostPort.useCallback(
    (modelDefinitionId: string, target: SelectionTarget) => toggleHierarchyEntityFlag(modelDefinitionId, target, "locked"),
    [toggleHierarchyEntityFlag],
  );

  const onCanvasHoverTarget = reactHostPort.useCallback((target: SpatialPickTarget | null) => {
    pointerFocusRef.current!.setHoverFromSource(CAD_PLAY_HOVER_SOURCE_CANVAS, target ? spatialPickTargetKey(target) : null);
  }, []);

  const onHoveredPickKeyChange = reactHostPort.useCallback((key: string | null) => {
    pointerFocusRef.current!.setHoverFromSource(CAD_PLAY_HOVER_SOURCE_CANVAS, key);
  }, []);

  const flushedModelsDigest = reactHostPort.useMemo(() => cadPlayModelsDigest(flushedModelsByDefinitionId), [flushedModelsByDefinitionId, liveModel.revision, modelDefinitionRevision]);

  reactHostPort.useEffect(() => {
    const snapshot: CadPlayChromeSnapshot = {
      modelsByDefinitionId: flushedModelsByDefinitionId,
      referencesByModelDefinitionId,
      activeModelDefinitionId,
      selection: selectionInScope,
      selectedReference,
      hoveredKey: pointerFocus.hover,
      fileStatus,
      selectTarget: selectHierarchyTarget,
      hoverTarget: hoverHierarchyTarget,
      toggleHidden: toggleHierarchyHidden,
      toggleLocked: toggleHierarchyLocked,
      selectReference: selectHierarchyReference,
      hoverReference: hoverHierarchyReference,
      toggleReferenceHidden: toggleHierarchyReferenceHidden,
      toggleReferenceLocked: toggleHierarchyReferenceLocked,
    };
    cadPlayChromeSnapshotRef.current = snapshot;
    publishCadPlayChrome(snapshot);
    runtime.notifyChrome();
    return () => {
      cadPlayChromeSnapshotRef.current = null;
      publishCadPlayChrome(null);
      runtime.notifyChrome();
    };
  }, [
    activeModelDefinitionId,
    flushedModelsByDefinitionId,
    fileStatus,
    flushedModelsDigest,
    hoverHierarchyReference,
    hoverHierarchyTarget,
    modelDefinitionRevision,
    pointerFocus.hover,
    publishCadPlayChrome,
    referencesByModelDefinitionId,
    runtime,
    selectHierarchyReference,
    selectHierarchyTarget,
    selectedReference,
    selectionInScope,
    toggleHierarchyHidden,
    toggleHierarchyLocked,
    toggleHierarchyReferenceHidden,
    toggleHierarchyReferenceLocked,
  ]);

  const exportBaseName = reactHostPort.useMemo(() => {
    if (loadedRawName) return fileStem(loadedRawName);
    const asset = CAD_PLAY_SHAPE_ASSETS.find((g) => g.id === shapeAssetId);
    return fileStem(asset?.id ?? "spatial");
  }, [shapeAssetId, loadedRawName]);

  const handleApplyTransformation = reactHostPort.useCallback(
    (spec: TransformationSpec) => {
      const space = modelSpaceFromRecord(flushModelsRecord(modelsByDefinitionId, activeModelDefinitionId, liveModel));
      try {
        space.transfer(spec.source.modelDefinition, spec.target.modelDefinition, spec, brepjsKernel);
      } catch (error) {
        setFileStatus(`Transfer failed: ${String(error)}`);
        return;
      }
      setModelsByDefinitionId(ensurePlayQuadModelSlots(recordFromModelSpace(space)));
      setActiveModelDefinitionId(spec.target.modelDefinition);
      setModelDefinitionRevision((r) => r + 1);
      setFileStatus(`Transferred ${spec.source.modelDefinition} → ${spec.target.modelDefinition}.`);
    },
    [activeModelDefinitionId, brepjsKernel, liveModel, modelsByDefinitionId],
  );

  reactHostPort.useEffect(() => {
    history.clear();
    setSnapshotByPane(emptySnapshotByPane());
  }, [history, interactionIdByPane, interactionBootIdByPane, shapeAssetId]);

  const saveBundle = reactHostPort.useCallback(async (name: string, payload: SpatialExchangeBundle, message: string) => {
    try {
      await writeJsonFile(name, payload);
      setFileStatus(message);
    } catch (error) {
      setFileStatus(`Save failed: ${String(error)}`);
    }
  }, []);

  const handleSaveSelected = reactHostPort.useCallback(async () => {
    const selectedModel = Model.fromJSON(selectRawModel(liveModel, selectionInScope));
    const selectedModelSpace = new ModelSpace();
    selectedModelSpace.link(activeModelDefinitionId, selectedModel);
    await saveBundle(`${exportBaseName}.selected.spatial.json`, { model: selectedModel.toJSON(), modelSpace: selectedModelSpace.toJSON(), activeModelDefinitionId }, `Saved ${selectionInScope.length} selected item(s) for ${activeModelDefinitionId}.`);
  }, [activeModelDefinitionId, exportBaseName, liveModel, saveBundle, selectionInScope]);

  const handleSaveInPlay = reactHostPort.useCallback(async () => {
    try {
      const stepText = await brepjsKernel.exportModelSpaceToStep(playModelSpace, exportBaseName);
      await writeStepFile(`${exportBaseName}.modelspace.stp`, stepText);
      setFileStatus(`Saved model space (${Object.keys(playModelSpace.models).length} model(s)) to STEP.`);
    } catch (error) {
      setFileStatus(`Save failed: ${String(error)}`);
    }
  }, [brepjsKernel, exportBaseName, playModelSpace]);

  const handleSaveCurrent = reactHostPort.useCallback(async () => {
    try {
      const modelId = activeModelDefinitionId;
      const stepText = await brepjsKernel.exportModelToStep(visibleExportModel, modelId);
      const stem = sanitizeModelDefinitionFileStem(modelId);
      await writeStepFile(`${exportBaseName}.${stem}.stp`, stepText);
      setFileStatus(`Saved ${modelId} to STEP.`);
    } catch (error) {
      setFileStatus(`Save failed: ${String(error)}`);
    }
  }, [activeModelDefinitionId, brepjsKernel, exportBaseName, visibleExportModel]);

  const handleLoadRawRequest = reactHostPort.useCallback(() => {
    loadInputRef.current?.click();
  }, []);

  const handleEngagementChangeForPane = reactHostPort.useCallback(
    (pane: CadPlayPaneId, engagement: EngagementSpec | null) => {
      engagementSpecRefByPane.current[pane] = engagement;
      shellController.setPaneEngagement(pane, cadPlayEngagementMirror(engagement, pane));
    },
    [shellController],
  );

  reactHostPort.useEffect(() => {
    const bridge = {
      getToolbarState: () => ({
        activeModelDefinitionId,
        selectionCount: selectionInScope.length,
        transfersTo: listTransformationsFromModelDefinition(activeModelDefinitionId),
        transfersFrom: listTransformationsIntoModelDefinition(activeModelDefinitionId),
      }),
      runHostCommand: (command: string, args?: unknown) => {
        switch (command) {
          case "focusModelDefinition": {
            const modelDefinitionId = (args as { modelDefinitionId?: string })?.modelDefinitionId;
            if (modelDefinitionId) focusModelDefinition(modelDefinitionId);
            break;
          }
          case "applyTransformation": {
            const qid = (args as { qid?: string })?.qid;
            if (!qid) break;
            const spec =
              listTransformationsFromModelDefinition(activeModelDefinitionId).find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid) ??
              listTransformationsIntoModelDefinition(activeModelDefinitionId).find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid);
            if (spec) void handleApplyTransformation(spec);
            break;
          }
          case "saveSelected":
            void handleSaveSelected();
            break;
          case "saveInPlay":
            void handleSaveInPlay();
            break;
          case "saveCurrent":
            void handleSaveCurrent();
            break;
          case "loadRawRequest":
            handleLoadRawRequest();
            break;
          case "deleteSelection":
            handleDeleteSelection();
            break;
          case "patchCadPlayReference": {
            const { modelDefinitionId, referenceId, field, value } = args as {
              modelDefinitionId?: string;
              referenceId?: string;
              field?: CadPlayReferencePatchField;
              value?: unknown;
            };
            if (!modelDefinitionId || !referenceId || !field) {
              break;
            }
            setReferencesByModelDefinitionId((prev) => {
              const rows = prev[modelDefinitionId] ?? [];
              const index = rows.findIndex((row) => row.id === referenceId);
              if (index < 0) {
                return prev;
              }
              const reference = rows[index]!;
              const patched = patchWorldReferenceProps(reference, field, value);
              if (!patched) {
                return prev;
              }
              const nextRows = rows.map((row, rowIndex) => (rowIndex === index ? patched : row));
              return { ...prev, [modelDefinitionId]: nextRows };
            });
            break;
          }
          case "patchCadPlaySelection": {
            const { modelDefinitionId, kind, id, targets, field, value } = args as {
              modelDefinitionId?: string;
              kind?: SelectionTarget["kind"];
              id?: string;
              targets?: readonly { readonly kind?: SelectionTarget["kind"]; readonly id?: string }[];
              field?: CadPlaySelectionPatchField;
              value?: unknown;
            };
            if (!modelDefinitionId || !field) {
              break;
            }
            const selectionRows: SelectionTarget[] = Array.isArray(targets)
              ? targets
                  .filter((row): row is { readonly kind: SelectionTarget["kind"]; readonly id: string } => Boolean(row?.kind && row?.id))
                  .map((row) => ({ kind: row.kind, id: row.id, editable: true }))
              : kind && id
                ? [{ kind, id, editable: true }]
                : [];
            if (!selectionRows.length) {
              break;
            }
            let currentModel = modelDefinitionId === activeModelDefinitionId ? liveModel : flushedModelsByDefinitionId[modelDefinitionId];
            if (!currentModel) {
              break;
            }
            let patchedModel: Model | null = null;
            for (const target of selectionRows) {
              const patched = patchCadPlaySelectionTarget(currentModel, target, field, value);
              if (!patched) {
                break;
              }
              patchedModel = patched;
              currentModel = patched;
            }
            if (!patchedModel) {
              break;
            }
            commitModelForDefinition(modelDefinitionId, patchedModel);
            if (field === "hidden" || field === "locked") {
              setRendererSelectionByModel((prev) =>
                replWithRendererSelectionTargets(
                  prev,
                  modelDefinitionId,
                  pruneSelectionTargetsForEntityFlags(prev[modelDefinitionId] ?? [], (entityId) => patchedModel!.getEntityFlags(entityId)),
                ),
              );
            }
            break;
          }
          case "engagementOption": {
            const pane = (args as { pane?: CadPlayPaneId })?.pane;
            const optionId = (args as { optionId?: string })?.optionId;
            if (!pane || !CAD_PLAY_PANE_IDS.includes(pane)) break;
            engagementSpecRefByPane.current[pane]?.options?.find((option) => option.id === optionId)?.onPress?.();
            break;
          }
          case "engagementInput": {
            const pane = (args as { pane?: CadPlayPaneId })?.pane;
            if (!pane || !CAD_PLAY_PANE_IDS.includes(pane)) break;
            engagementSpecRefByPane.current[pane]?.input?.onChange?.((args as { value?: string })?.value ?? "");
            break;
          }
          case "engagementSubmit": {
            const pane = (args as { pane?: CadPlayPaneId })?.pane;
            if (!pane || !CAD_PLAY_PANE_IDS.includes(pane)) break;
            engagementSpecRefByPane.current[pane]?.input?.onSubmit?.((args as { value?: string })?.value ?? "");
            break;
          }
          case "engagementRepeatLast": {
            const pane = (args as { pane?: CadPlayPaneId })?.pane;
            if (!pane || !CAD_PLAY_PANE_IDS.includes(pane)) break;
            engagementSpecRefByPane.current[pane]?.input?.onRepeatLast?.();
            break;
          }
          case "engagementAbort": {
            const pane = (args as { pane?: CadPlayPaneId })?.pane;
            if (!pane || !CAD_PLAY_PANE_IDS.includes(pane)) break;
            engagementSpecRefByPane.current[pane]?.input?.onAbort?.();
            break;
          }
          case "engagementPossibleSelect": {
            const pane = (args as { pane?: CadPlayPaneId })?.pane;
            const possibleId = (args as { possibleId?: string })?.possibleId;
            if (!pane || !CAD_PLAY_PANE_IDS.includes(pane) || !possibleId) break;
            engagementSpecRefByPane.current[pane]?.possibleEngagements?.find((row) => row.id === possibleId)?.onSelect?.();
            break;
          }
          case "engagementControlChange": {
            const pane = (args as { pane?: CadPlayPaneId })?.pane;
            const value = (args as { value?: number })?.value;
            const control = pane && CAD_PLAY_PANE_IDS.includes(pane) ? engagementSpecRefByPane.current[pane]?.control : undefined;
            if (value === undefined || !control || control.kind === "ring") break;
            control.onChange?.(value);
            break;
          }
          case "engagementControlCommit": {
            const pane = (args as { pane?: CadPlayPaneId })?.pane;
            const value = (args as { value?: number })?.value;
            const control = pane && CAD_PLAY_PANE_IDS.includes(pane) ? engagementSpecRefByPane.current[pane]?.control : undefined;
            if (value === undefined || !control || control.kind === "ring") break;
            control.onCommit?.(value);
            break;
          }
          case "engagementControlSelect": {
            const pane = (args as { pane?: CadPlayPaneId })?.pane;
            const id = (args as { id?: string })?.id;
            const control = pane && CAD_PLAY_PANE_IDS.includes(pane) ? engagementSpecRefByPane.current[pane]?.control : undefined;
            if (!id || !control || control.kind !== "ring") break;
            control.onSelect?.(id);
            break;
          }
          default:
            break;
        }
      },
    };
    shellController.setHostBridge(bridge);
    return () => shellController.setHostBridge(null);
  }, [activeModelDefinitionId, focusModelDefinition, handleApplyTransformation, handleDeleteSelection, handleLoadRawRequest, handleSaveCurrent, handleSaveInPlay, handleSaveSelected, selectionInScope, shellController, setReferencesByModelDefinitionId, flushedModelsByDefinitionId, liveModel, commitModelForDefinition]);

  const handleLoadRaw = reactHostPort.useCallback(async (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;
    try {
      const parsed = JSON.parse(await file.text()) as unknown;
      const envelope = parsed as Record<string, unknown>;
      const snapshot =
        envelope && typeof envelope === "object" && "modelSpace" in envelope
          ? envelope.modelSpace
          : envelope && typeof envelope === "object" && "model" in envelope
            ? envelope.model
            : envelope && typeof envelope === "object" && "raw" in envelope
              ? envelope.raw
              : parsed;
      const modelSpace = parseModelSpaceJson(snapshot);
      if (modelSpace) {
        const nextActiveModelDefinitionId = typeof envelope.activeModelDefinitionId === "string" && modelSpace.get(envelope.activeModelDefinitionId) ? envelope.activeModelDefinitionId : activeModelDefinitionIdFromSpatialJson(snapshot);
        setShapeAssetId("");
        setLoadedRawName(file.name);
        setModelsByDefinitionId(ensurePlayQuadModelSlots(recordFromModelSpace(modelSpace)));
        setReferencesByModelDefinitionId(cadPlayEmptyReferencesByModelDefinitionId());
        setSelectedReference(null);
        setActiveModelDefinitionId(nextActiveModelDefinitionId);
        setModelsLoadEpoch((epoch) => epoch + 1);
        setModelDefinitionRevision((r) => r + 1);
        setFileStatus(`Loaded model space from ${file.name}.`);
        return;
      }
      const model = parseModelJson(snapshot);
      if (!model) throw new Error("No spatial model found in file.");
      setShapeAssetId("");
      setLoadedRawName(file.name);
      setModelsByDefinitionId(modelsFromCadJson(model.toJSON()));
      setReferencesByModelDefinitionId(cadPlayEmptyReferencesByModelDefinitionId());
      setSelectedReference(null);
      setActiveModelDefinitionId(defaultModelDefinitionId());
      setModelsLoadEpoch((epoch) => epoch + 1);
      setModelDefinitionRevision((r) => r + 1);
      setFileStatus(`Loaded model from ${file.name}.`);
    } catch (error) {
      setFileStatus(`Load failed: ${String(error)}`);
    } finally {
      event.target.value = "";
    }
  }, []);

  const modelSpaceValue = reactHostPort.useMemo<CadPlayModelSpaceValue>(
    () => ({
      activeModelDefinitionId,
      setActiveModelDefinitionId,
      focusModelDefinition,
      interactionIdForPane: (pane) => interactionIdByPane[pane],
      handleInteractionPickForPane,
      specForPane,
      documentModel,
      history,
      kernel,
      computeModeForPane,
      transformGumballConfigForPane,
      sessionRestartNonceForPane: (pane) => interactionBootIdByPane[pane],
      rendererSelectionByModel,
      setRendererSelectionByModel,
      interactionSelectionByState,
      setInteractionSelectionByState,
      modelDefinitionRevision,
      setModelDefinitionRevision,
      handleApplyTransformation,
      pickGeometry,
      handleModelAttributesChange,
      commitModelForDefinition,
      handleSnapshotChangeForPane,
      handleEngagementChangeForPane,
      flushedModelsByDefinitionId,
      playModelSpace,
      viewObjectCount,
      selectionInScope,
      hoveredPickKey: pointerFocus.hover,
      onCanvasHoverTarget,
      onHoveredPickKeyChange,
      shapeAssetId,
      handleShapeAssetChange,
      fileStatus,
      loadInputRef,
      exportBaseName,
      handleSaveSelected,
      handleSaveInPlay,
      handleSaveCurrent,
      handleLoadRawRequest,
      handleLoadRaw,
      liveModel,
      brepjsKernel,
      handleTransformGumballCommit,
      handleDeleteSelection,
      cameraViewSeedForInstance,
      orbitProjectionForInstance,
      applyOrbitViewForInstance,
      setOrbitProjectionForInstance,
      modelsLoadEpoch,
      referencesByModelDefinitionId,
      referencesForModelDefinition,
      selectedReference,
      hoveredReferenceId,
      revealedReferenceIds,
      handleReferenceSelect,
      handleReferenceHover,
      handleReferenceRelocate,
    }),
    [
      activeModelDefinitionId,
      applyOrbitViewForInstance,
      cameraViewSeedForInstance,
      documentModel,
      exportBaseName,
      fileStatus,
      flushedModelsByDefinitionId,
      focusModelDefinition,
      handleApplyTransformation,
      handleInteractionPickForPane,
      handleLoadRaw,
      handleLoadRawRequest,
      handleModelAttributesChange,
      commitModelForDefinition,
      handleSaveCurrent,
      handleSaveInPlay,
      handleSaveSelected,
      handleShapeAssetChange,
      handleSnapshotChangeForPane,
      handleEngagementChangeForPane,
      history,
      interactionBootIdByPane,
      interactionIdByPane,
      interactionSelectionByState,
      kernel,
      liveModel,
      computeModeForPane,
      transformGumballConfigForPane,
      modelDefinitionRevision,
      pickGeometry,
      playModelSpace,
      rendererSelectionByModel,
      selectionInScope,
      pointerFocus.hover,
      onCanvasHoverTarget,
      onHoveredPickKeyChange,
      shapeAssetId,
      specForPane,
      viewObjectCount,
      brepjsKernel,
      handleTransformGumballCommit,
      handleDeleteSelection,
      orbitProjectionForInstance,
      setOrbitProjectionForInstance,
      modelsLoadEpoch,
      referencesByModelDefinitionId,
      referencesForModelDefinition,
      selectedReference,
      hoveredReferenceId,
      revealedReferenceIds,
      handleReferenceSelect,
      handleReferenceHover,
      handleReferenceRelocate,
    ],
  );

  return <CadPlayModelSpaceContext.Provider value={modelSpaceValue}>{children}</CadPlayModelSpaceContext.Provider>;
}

/** @emoji 📂 Hidden file input for playground Save → Load. */
function CadPlayLoadInput(): ReactNode {
  const { loadInputRef, handleLoadRaw } = useCadPlayModelSpace();
  return <input ref={loadInputRef} type="file" accept=".json,.spatial.json" hidden onChange={(event) => void handleLoadRaw(event)} />;
}

/** @emoji 🧪 Navbar example dropdown for CAD play shape sources (replaces workbench catalog picker). */
function CadPlayExampleNavbarSelect(): ReactNode {
  const { shapeAssetId, handleShapeAssetChange } = useCadPlayModelSpace();
  return (
    <NavbarExampleSelect
      id="cad.play.example"
      value={shapeAssetId || NAVBAR_NO_EXAMPLE_ID}
      options={CAD_PLAY_SHAPE_ASSETS.map((row) => ({
        id: row.id,
        label: `[${row.key}] ${row.label} (${modelVertexCount(row.json)} verts)`,
      }))}
      onValueChange={(exampleId) => handleShapeAssetChange(exampleId === NAVBAR_NO_EXAMPLE_ID ? "" : exampleId)}
    />
  );
}

/** @emoji 🎮 One quad pane: interaction editing for its model definition with window engagement. */
function CadPlayInteractionPane({ pane, instanceId }: { readonly pane: CadPlayPaneId; readonly instanceId?: string }): ReactNode {
  const {
    interactionIdForPane,
    specForPane,
    handleInteractionPickForPane,
    history,
    kernel,
    computeModeForPane,
    sessionRestartNonceForPane,
      focusModelDefinition,
      activeModelDefinitionId,
      flushedModelsByDefinitionId,
    rendererSelectionByModel,
    setRendererSelectionByModel,
    interactionSelectionByState,
    setInteractionSelectionByState,
    setModelDefinitionRevision,
    handleApplyTransformation,
    commitModelForDefinition,
    handleSnapshotChangeForPane,
    handleEngagementChangeForPane,
    hoveredPickKey,
    onCanvasHoverTarget,
    onHoveredPickKeyChange,
    transformGumballConfigForPane,
    handleTransformGumballCommit,
    handleDeleteSelection,
    cameraViewSeedForInstance,
    orbitProjectionForInstance,
    setOrbitProjectionForInstance,
    modelsLoadEpoch,
    referencesForModelDefinition,
    selectedReference,
    hoveredReferenceId,
    revealedReferenceIds,
    handleReferenceSelect,
    handleReferenceHover,
    handleReferenceRelocate,
  } = useCadPlayModelSpace();
  const cameraSeed = cameraViewSeedForInstance(instanceId);
  const orbitProjection = orbitProjectionForInstance(instanceId);
  const onOrbitProjectionChange = reactHostPort.useCallback(
    (projection: OrbitCameraProjection) => setOrbitProjectionForInstance(projection, instanceId),
    [instanceId, setOrbitProjectionForInstance],
  );
  const modelDefinitionId = cadPlayModelDefinitionIdForPane(pane);
  const captureGlobalKeys = activeModelDefinitionId === modelDefinitionId;
  const interactionId = interactionIdForPane(pane);
  const spec = reactHostPort.useMemo(
    () => (interactionId ? (loadSpatialInteraction(interactionId) ?? PLAY_REPL_SPEC) : PLAY_REPL_SPEC),
    [interactionId],
  );
  const paneModel = cadPlayPaneModel(flushedModelsByDefinitionId, pane);
  const viewModel = paneModel;
  const paneModelRevision = viewModel.revision;
  const pickGeometry = reactHostPort.useMemo(
    () => cadPlayPaneGeometry(flushedModelsByDefinitionId, modelDefinitionId, paneModel),
    [flushedModelsByDefinitionId, modelDefinitionId, paneModel, paneModelRevision, modelsLoadEpoch],
  );
  const documentModel = reactHostPort.useMemo(
    (): ModelDocument => ({ model: Model.fromJSON(viewModel.toJSON()), nodes: [] }),
    [viewModel, paneModelRevision, modelsLoadEpoch],
  );
  const commitPaneModel = reactHostPort.useCallback((model: Model) => commitModelForDefinition(modelDefinitionId, model), [commitModelForDefinition, modelDefinitionId]);
  const onTransformGumballCommit = reactHostPort.useCallback((diff: ModelDiff) => handleTransformGumballCommit(modelDefinitionId, diff), [handleTransformGumballCommit, modelDefinitionId]);
  const onSnapshot = reactHostPort.useCallback((snapshot: InteractionSnapshot) => handleSnapshotChangeForPane(pane, snapshot), [handleSnapshotChangeForPane, pane]);
  const onEngagementChange = reactHostPort.useCallback((engagement: EngagementSpec | null) => handleEngagementChangeForPane(pane, engagement), [handleEngagementChangeForPane, pane]);
  const onInteractionId = reactHostPort.useCallback((id: string) => handleInteractionPickForPane(pane, id), [handleInteractionPickForPane, pane]);

  if (interactionId && !loadSpatialInteraction(interactionId)) {
    return (
      <div className="flex flex-col gap-double p-double text-destructive text-xs">
        <p>
          Unknown interaction <code className="text-foreground">{interactionId}</code>.
        </p>
        <Button type="button" variant="outline" size="sm" className="w-fit" onClick={() => onInteractionId("")}>
          Reset
        </Button>
      </div>
    );
  }

  const mode = computeModeForPane(pane);
  const transformGumballConfig = transformGumballConfigForPane(pane);
  const autoFitMeshes = modelHasCommittedSolidsForDisplay(viewModel) || modelHasFactoryFaceDisplay(viewModel, modelDefinitionId);
  const paneReferences = referencesForModelDefinition(modelDefinitionId);
  const selectedReferenceIds = reactHostPort.useMemo(
    () => (selectedReference?.modelDefinitionId === modelDefinitionId ? new Set([selectedReference.id]) : new Set<string>()),
    [modelDefinitionId, selectedReference],
  );
  const onReferenceRelocate = reactHostPort.useCallback(
    (payload: WorldReferenceRelocatePayload) => handleReferenceRelocate(modelDefinitionId, payload),
    [handleReferenceRelocate, modelDefinitionId],
  );
  const onReferenceSelect = reactHostPort.useCallback(
    (id: string) => handleReferenceSelect(modelDefinitionId, id),
    [handleReferenceSelect, modelDefinitionId],
  );
  const onReferenceHover = reactHostPort.useCallback(
    (id: string | null) => handleReferenceHover(modelDefinitionId, id),
    [handleReferenceHover, modelDefinitionId],
  );

  return (
    <div className="absolute inset-0 min-h-0 min-w-0" onPointerDown={() => focusModelDefinition(modelDefinitionId)}>
      <PlaySession
        interactionId={interactionId}
        spec={spec}
        onInteractionId={onInteractionId}
        documentModel={documentModel}
        history={history}
        kernel={kernel}
        mode={mode}
        asideExtra={null}
        sessionRestartNonce={sessionRestartNonceForPane(pane)}
        activeModelDefinitionId={modelDefinitionId}
        onActiveModelDefinitionId={() => focusModelDefinition(modelDefinitionId)}
        rendererSelectionByModel={rendererSelectionByModel}
        onRendererSelectionByModel={setRendererSelectionByModel}
        interactionSelectionByState={interactionSelectionByState}
        onInteractionSelectionByState={setInteractionSelectionByState}
        modelDefinitionRevision={paneModelRevision}
        onModelDefinitionRevision={setModelDefinitionRevision}
        onApplyTransformation={handleApplyTransformation}
        pickGeometry={pickGeometry}
        onDocumentModelChange={commitPaneModel}
        onSnapshot={onSnapshot}
        onEngagementChange={onEngagementChange}
        captureGlobalKeys={captureGlobalKeys}
        hoveredPickKey={hoveredPickKey}
        onHoveredPickKeyChange={onHoveredPickKeyChange}
        onCanvasHoverTarget={onCanvasHoverTarget}
        autoFitMeshes={autoFitMeshes}
        autoFitBehavior="changes"
        transformGumballConfig={transformGumballConfig}
        onTransformGumballCommit={onTransformGumballCommit}
        onDeleteSelection={handleDeleteSelection}
        cameraView={cameraSeed?.view}
        cameraViewSeedKey={cameraSeed?.seedKey}
        orbitProjection={orbitProjection}
        onOrbitProjectionChange={onOrbitProjectionChange}
        worldReferences={paneReferences}
        selectedReferenceIds={selectedReferenceIds}
        hoveredReferenceId={hoveredReferenceId}
        revealedReferenceIds={revealedReferenceIds}
        referenceRelocateActive={pane === "shape"}
        onReferenceSelect={onReferenceSelect}
        onReferenceHover={onReferenceHover}
        onReferenceRelocate={onReferenceRelocate}
      />
    </div>
  );
}
//#endregion

//#region 🔖PlaygroundHost
let cadPlayChromeRegistered = false;

function registerCadPlayChrome(): void {
  if (cadPlayChromeRegistered) return;
  cadPlayChromeRegistered = true;
  for (const pane of ["shape", "building", "energy", "structure-classic"] as const) {
    registerSurfaceBinding(cadPlaySceneSurfaceIdForPane(pane), CadPlaySurfaceHost);
  }
}

/** @emoji 🧊 Viewport for one CAD play quad pane. */
function CadPlaySurfaceHost({ node }: { readonly node: UiCadHostSurfaceNode }): ReactNode {
  if (node.controllerId !== CAD_PLAY_CONTROLLER_ID) {
    return <div className="p-single text-destructive text-xs">Invalid CAD play surface binding</div>;
  }
  const shellInstance = useShellWindowInstance();
  const paneFromSurface = cadPlayPaneFromSurfaceId(node.surfaceId);
  const paneFromKind = shellInstance?.windowKindId ? cadPlayPaneFromWindowKindId(shellInstance.windowKindId) : null;
  const pane = paneFromKind ?? paneFromSurface;
  if (!pane) {
    return <div className="p-single text-destructive text-xs">Unknown CAD play surface</div>;
  }
  return (
    <div className="absolute inset-0 flex min-h-0 min-w-0 flex-col overflow-hidden">
      <CadPlayInteractionPane pane={pane} instanceId={shellInstance?.instanceId} />
    </div>
  );
}


class CadPlayCatalogPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: "cad-play-catalog",
      icon: CadPlayCatalogueIcon,
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() =>
        uiTreeNodeToTreePanelConfig(buildCadPlayCatalogTree(cadPlayChromeSnapshotRef.current), new CommandBus()),
      ),
    };
  }
}

class CadPlayDetailsPanelDefinition extends PureSidePanelTabDefinition {
  constructor(private readonly commandBus: CommandBus) {
    super();
  }

  buildTab(): SidePanelTabConfig {
    return {
      id: "cad-play-details",
      icon: CadPlayInspectionIcon,
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() =>
        uiTreeNodeToTreePanelConfig(buildCadPlayDetailsTree(cadPlayChromeSnapshotRef.current), this.commandBus),
      ),
    };
  }
}

/** @emoji 🧊 CAD play chrome root for standalone boot and s nested host. */
export function CadPlayRoot({ runtime: runtimeOverride }: { readonly runtime?: Platform } = {}): ReactNode {
  const runtimeRef = reactHostPort.useRef<Platform | null>(runtimeOverride ?? null);
  const shellControllerRef = reactHostPort.useRef<CadPlayShellController | null>(null);
  const [chromeSnapshot, setChromeSnapshot] = reactHostPort.useState<CadPlayChromeSnapshot | null>(null);
  if (!runtimeRef.current) {
    registerCadPlayChrome();
    runtimeRef.current = buildCadPlayRuntime();
    runtimeRef.current.setActiveAppId(CAD_PLAY_APP_ID);
    shellControllerRef.current = runtimeRef.current.getActiveApp()?.controller as CadPlayShellController;
  } else if (!shellControllerRef.current) {
    registerCadPlayChrome();
    shellControllerRef.current = runtimeRef.current.getActiveApp()?.controller as CadPlayShellController;
  }
  const shellController = shellControllerRef.current;
  if (!shellController) {
    return null;
  }
  const chromeContextValue = reactHostPort.useMemo<CadPlayChromeContextValue>(() => ({ snapshot: chromeSnapshot, publishSnapshot: setChromeSnapshot }), [chromeSnapshot]);
  const cadPlayHierarchyPanel = reactHostPort.useMemo(() => new CadPlayHierarchyPanelDefinition(), []);
  const cadPlayCatalogPanel = reactHostPort.useMemo(() => new CadPlayCatalogPanelDefinition(), []);
  const cadPlayDetailsPanel = reactHostPort.useMemo(
    () => new CadPlayDetailsPanelDefinition(runtimeRef.current!.commandBus),
    [],
  );
  const workbenchTabs = reactHostPort.useMemo(
    () => [cadPlayHierarchyPanel.resolveTab(), cadPlayCatalogPanel.resolveTab()],
    [cadPlayHierarchyPanel, cadPlayCatalogPanel],
  );
  const detailsTabs = reactHostPort.useMemo(() => [cadPlayDetailsPanel.resolveTab()], [cadPlayDetailsPanel]);
  const slotNavbarCenter = reactHostPort.useMemo(
    () => (isPlaygroundExampleLocked() ? null : <CadPlayExampleNavbarSelect />),
    [],
  );
  return (
    <CadPlayChromeContext.Provider value={chromeContextValue}>
      <CadPlayModelSpaceProvider runtime={runtimeRef.current} shellController={shellController}>
        <CadPlayLoadInput />
        <PlaygroundView runtime={runtimeRef.current} defaultAppId={CAD_PLAY_APP_ID} augmentPanelTabs={{ workbench: workbenchTabs, details: detailsTabs }} slotNavbarCenter={slotNavbarCenter} />
      </CadPlayModelSpaceProvider>
    </CadPlayChromeContext.Provider>
  );
}

/** @emoji 🛝 Registers CAD play surface hosts for s and playground boot. */
export function registerCadPlaySurfaceHosts(): void {
  registerCadPlayChrome();
}
