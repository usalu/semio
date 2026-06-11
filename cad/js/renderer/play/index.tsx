// #region 🧲Header
// 💻 cad/js/renderer/play/index.tsx — CAD play shell (headless + React chrome + Vite entry).
// #endregion 🧲Header

import { AppPointerFocusStore } from "@framework/core";
import {
  CommandBus,
  Controller,
  Platform,
  AppRuntime,
  ModeRuntime,
  WindowKindRuntime,
  buildCadWindowBody,
  createNamedLayout,
  createPlayAppRuntime,
  createWindowLayout,
  PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY,
  namedLayoutsFromOrbitViewDescriptors,
  registerWindowBody,
  type AppTools,
  type ToolItem,
  type WindowBodyViewContext,
  type WindowEngagement,
  type WindowMeasure,
  type UiNode,
  type WindowLayout,
  type WindowTemplate,
  enforcePlaygroundWindowEngagementInput,
  windowEngagementsEqual,
} from "@framework/playground/core";
import {
  ORBIT_CAMERA_VIEW_COMMAND,
  applyWorldReferenceTransform,
  createOrbitCameraViewLayoutDescriptors,
  createOrbitCameraViewTemplates,
  orbitCameraProjectionForView,
  worldEntitySelectable,
  type OrbitCameraViewId,
  type OrbitCameraProjection,
  type WorldReferenceProps,
  type WorldReferenceRelocatePayload,
} from "@infinite/world/r3f";
import {
  DocumentHistory,
  defaultModelDefinitionId,
  applyTransformation,
  buildModelPrimitiveHierarchy,
  countViewObjectsForModelDefinition,
  createInteractionRuntime,
  isEmptyModelDiff,
  isInteractionSessionActive,
  isShapeModelDefinition,
  listModelDefinitionManifests,
  listModelObjectsForModelDefinition,
  listSpatialInteractionsForModelDefinition,
  listTransformationsFromModelDefinition,
  listTransformationsIntoModelDefinition,
  loadSpatialInteraction,
  Model,
  ModelSpace,
  modelDefinitionSelectionEntityKinds,
  modelDefinitionUsesGeometryPicking,
  objectPrimitiveEntries,
  parseModelJson,
  qualifiedTransformationId,
  resolveModelDefinitionScope,
  resolvePrimitiveRefKind,
  typologyObjectPascalFromLabel,
  type InteractionRuntime,
  type InteractionRuntimeOptions,
  type InteractionSnapshot,
  type InteractionSpec,
  type ModelDocument,
  type ModelPrimitiveHierarchyNode,
  type SelectionTarget,
  type SpatialComputeMode,
  type TransformationSpec,
  applyModelDiff,
  type CadGumballConfig,
  type CadGumballGroupKey,
  CAD_GUMBALL_GROUPS,
  CAD_GUMBALL_HIDDEN,
  cadGumballConfigVisible,
  type ModelDiff,
  deleteObjectsFromModel,
  deletableObjectIdsFromSelection,
} from "@cad/js/core";
import { bootstrapCadModules } from "@cad/js/runtime";
import { AEC_BUILDING_MODEL_DEFINITION_ID } from "@cad/js/module/aec-building";
import { AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID } from "@cad/js/module/aec-building-energy";
import {
  AEC_BUILDING_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID,
  AEC_BUILDING_STRUCTURE_MODEL_DEFINITION_ID,
} from "@cad/js/module/aec-building-structure";

bootstrapCadModules();

/** @emoji ⚡ Per-window compute mode options for CAD play window measures. */
export const CAD_PLAY_COMPUTE_MODES: readonly SpatialComputeMode[] = ["fast", "precise"];

const ListTree = createIconComponent("list-tree");
const Shapes = createIconComponent("shapes");

//#region 🔖Ids
export const CAD_PLAY_APP_ID = "cad-play";
export const CAD_PLAY_CONTROLLER_ID = "cad-play";
export const CAD_PLAY_HIERARCHY_TAB_ID = "cad-play-hierarchy";

/** @emoji 🖱️ Hover owner id when the workbench hierarchy drives shared pointer focus. */
export const CAD_PLAY_HOVER_SOURCE_HIERARCHY = "cad-play-hierarchy";

/** @emoji 🖱️ Hover owner id when the 3D canvas drives shared pointer focus. */
export const CAD_PLAY_HOVER_SOURCE_CANVAS = "cad-play-canvas";

export const CAD_PLAY_BUILDING_MODEL_DEFINITION_ID = AEC_BUILDING_MODEL_DEFINITION_ID;
export const CAD_PLAY_ENERGY_MODEL_DEFINITION_ID = AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID;
export const CAD_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID = AEC_BUILDING_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID;

export type CadPlayPaneId = "shape" | "building" | "energy" | "structure-classic";

export const CAD_PLAY_SHAPE_WINDOW_ID = "cad-play-shape";
export const CAD_PLAY_BUILDING_WINDOW_ID = "cad-play-building";
export const CAD_PLAY_ENERGY_WINDOW_ID = "cad-play-energy";
export const CAD_PLAY_STRUCTURE_CLASSIC_WINDOW_ID = "cad-play-structure-classic";

export const CAD_PLAY_SHAPE_WINDOW_LABEL = "Shape";
export const CAD_PLAY_BUILDING_WINDOW_LABEL = "Building";
export const CAD_PLAY_ENERGY_WINDOW_LABEL = "Energy";
export const CAD_PLAY_STRUCTURE_CLASSIC_WINDOW_LABEL = "Structure Classic";

export const CAD_PLAY_SHAPE_BODY_KEY = "cad.play.shape";
export const CAD_PLAY_BUILDING_BODY_KEY = "cad.play.building";
export const CAD_PLAY_ENERGY_BODY_KEY = "cad.play.energy";
export const CAD_PLAY_STRUCTURE_CLASSIC_BODY_KEY = "cad.play.structure-classic";

export const CAD_PLAY_SHAPE_SCENE_SURFACE_ID = "cad.play.scene3d/shape";
export const CAD_PLAY_BUILDING_SCENE_SURFACE_ID = "cad.play.scene3d/building";
export const CAD_PLAY_ENERGY_SCENE_SURFACE_ID = "cad.play.scene3d/energy";
export const CAD_PLAY_STRUCTURE_CLASSIC_SCENE_SURFACE_ID = "cad.play.scene3d/structure-classic";

export const CAD_PLAY_DEFAULT_WORLD_REFERENCES: WorldReferenceProps[] = [
  { id: "ref-sketch", source: { url: "/infinite-fixture/sketch.png", mediaKind: "image" }, origin: [-24, -18, 0.01], widthWorld: 22 },
  { id: "ref-site-pdf", source: { url: "/infinite-fixture/site.pdf", mediaKind: "pdf", page: 1 }, origin: [12, 8, 0.01], widthWorld: 28 },
];

export type CadPlayReferencesByModelDefinitionId = Readonly<Record<string, readonly WorldReferenceProps[]>>;

export interface CadPlaySelectedReference {
  readonly modelDefinitionId: string;
  readonly id: string;
}

/** @emoji 🪪 Stable hover/selection key for a CAD play reference plane. */
export function cadPlayReferenceHoverKey(referenceId: string): string {
  return `reference:${referenceId}`;
}

/** @emoji 🔢 Digest for hierarchy chrome when reference planes mutate. */
export function cadPlayReferencesDigest(referencesByModelDefinitionId: CadPlayReferencesByModelDefinitionId): string {
  return Object.keys(referencesByModelDefinitionId)
    .sort((a, b) => a.localeCompare(b))
    .map((modelDefinitionId) => {
      const rows = referencesByModelDefinitionId[modelDefinitionId] ?? [];
      return `${modelDefinitionId}:${rows.map((row) => [row.id, row.hidden === true, row.locked === true, row.source.url].join(":")).join("|")}`;
    })
    .join(";");
}

function cadPlayDefaultReferencesByModelDefinitionId(): Record<string, WorldReferenceProps[]> {
  return { [defaultModelDefinitionId()]: CAD_PLAY_DEFAULT_WORLD_REFERENCES.map((row) => ({ ...row })) };
}

/** @emoji 🖼️ Patches one reference plane under a model definition. */
export function updateCadPlayReferenceInMap(
  referencesByModelDefinitionId: Record<string, WorldReferenceProps[]>,
  modelDefinitionId: string,
  referenceId: string,
  patch: Partial<Omit<WorldReferenceProps, "id">>,
): Record<string, WorldReferenceProps[]> {
  const rows = referencesByModelDefinitionId[modelDefinitionId];
  if (!rows?.length) {
    return referencesByModelDefinitionId;
  }
  const index = rows.findIndex((row) => row.id === referenceId);
  if (index < 0) {
    return referencesByModelDefinitionId;
  }
  const nextRows = rows.map((row, rowIndex) => (rowIndex === index ? { ...row, ...patch } : row));
  return { ...referencesByModelDefinitionId, [modelDefinitionId]: nextRows };
}

const CAD_PLAY_VIEW_TEMPLATES: readonly WindowTemplate[] = createOrbitCameraViewTemplates({
  controllerId: CAD_PLAY_CONTROLLER_ID,
}) as readonly WindowTemplate[];

/** @emoji 🪟 Quad play layout: shape/building left, energy/structure classic right. */
export const CAD_PLAY_LAYOUT: WindowLayout = {
  root: {
    kind: "row",
    children: [
      {
        kind: "column",
        size: 50,
        children: [
          { kind: "stack", size: 50, children: [createWindowLayout(CAD_PLAY_SHAPE_WINDOW_ID, CAD_PLAY_SHAPE_WINDOW_LABEL)] },
          { kind: "stack", size: 50, children: [createWindowLayout(CAD_PLAY_BUILDING_WINDOW_ID, CAD_PLAY_BUILDING_WINDOW_LABEL)] },
        ],
      },
      {
        kind: "column",
        size: 50,
        children: [
          { kind: "stack", size: 50, children: [createWindowLayout(CAD_PLAY_ENERGY_WINDOW_ID, CAD_PLAY_ENERGY_WINDOW_LABEL)] },
          {
            kind: "stack",
            size: 50,
            children: [createWindowLayout(CAD_PLAY_STRUCTURE_CLASSIC_WINDOW_ID, CAD_PLAY_STRUCTURE_CLASSIC_WINDOW_LABEL)],
          },
        ],
      },
    ],
  },
};

const CAD_PLAY_PANE_SPECS: readonly {
  readonly pane: CadPlayPaneId;
  readonly windowKindId: string;
  readonly label: string;
  readonly bodyKey: string;
  readonly surfaceId: string;
  readonly modelDefinitionId: string;
}[] = [
  {
    pane: "shape",
    windowKindId: CAD_PLAY_SHAPE_WINDOW_ID,
    label: CAD_PLAY_SHAPE_WINDOW_LABEL,
    bodyKey: CAD_PLAY_SHAPE_BODY_KEY,
    surfaceId: CAD_PLAY_SHAPE_SCENE_SURFACE_ID,
    modelDefinitionId: defaultModelDefinitionId(),
  },
  {
    pane: "building",
    windowKindId: CAD_PLAY_BUILDING_WINDOW_ID,
    label: CAD_PLAY_BUILDING_WINDOW_LABEL,
    bodyKey: CAD_PLAY_BUILDING_BODY_KEY,
    surfaceId: CAD_PLAY_BUILDING_SCENE_SURFACE_ID,
    modelDefinitionId: CAD_PLAY_BUILDING_MODEL_DEFINITION_ID,
  },
  {
    pane: "energy",
    windowKindId: CAD_PLAY_ENERGY_WINDOW_ID,
    label: CAD_PLAY_ENERGY_WINDOW_LABEL,
    bodyKey: CAD_PLAY_ENERGY_BODY_KEY,
    surfaceId: CAD_PLAY_ENERGY_SCENE_SURFACE_ID,
    modelDefinitionId: CAD_PLAY_ENERGY_MODEL_DEFINITION_ID,
  },
  {
    pane: "structure-classic",
    windowKindId: CAD_PLAY_STRUCTURE_CLASSIC_WINDOW_ID,
    label: CAD_PLAY_STRUCTURE_CLASSIC_WINDOW_LABEL,
    bodyKey: CAD_PLAY_STRUCTURE_CLASSIC_BODY_KEY,
    surfaceId: CAD_PLAY_STRUCTURE_CLASSIC_SCENE_SURFACE_ID,
    modelDefinitionId: CAD_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID,
  },
];

/** @emoji 🧭 Maps a CAD play scene surface id to its pane id. */
export function cadPlayPaneFromSurfaceId(surfaceId: string): CadPlayPaneId | null {
  return CAD_PLAY_PANE_SPECS.find((row) => row.surfaceId === surfaceId)?.pane ?? null;
}

/** @emoji 🧭 Active model definition for a CAD play pane. */
export function cadPlayModelDefinitionIdForPane(pane: CadPlayPaneId): string {
  return CAD_PLAY_PANE_SPECS.find((row) => row.pane === pane)!.modelDefinitionId;
}

/** @emoji 🌌 Resolves one pane's model from the shared play model space record (viewport binding). */
export function cadPlayPaneModel(modelsByDefinitionId: Readonly<Record<string, Model>>, pane: CadPlayPaneId): Model {
  const modelDefinitionId = cadPlayModelDefinitionIdForPane(pane);
  return modelsByDefinitionId[modelDefinitionId] ?? new Model();
}

/** @emoji 🧭 Scene surface id for a CAD play pane. */
export function cadPlaySceneSurfaceIdForPane(pane: CadPlayPaneId): string {
  return CAD_PLAY_PANE_SPECS.find((row) => row.pane === pane)!.surfaceId;
}

/** @emoji 🧭 Maps a CAD play window kind id to its pane id. */
export function cadPlayPaneFromWindowKindId(windowKindId: string): CadPlayPaneId | null {
  return CAD_PLAY_PANE_SPECS.find((row) => row.windowKindId === windowKindId)?.pane ?? null;
}

/** @emoji 🪟 Maps shell instance id (or window kind id on bootstrap) to CAD play pane. */
export function cadPlayPaneFromShellWindowId(shellWindowId: string): CadPlayPaneId | null {
  const direct = cadPlayPaneFromWindowKindId(shellWindowId);
  if (direct) {
    return direct;
  }
  const match = /^win-(cad-play-(?:shape|building|energy|structure-classic))-/.exec(shellWindowId);
  if (!match?.[1]) {
    return null;
  }
  return cadPlayPaneFromWindowKindId(match[1]);
}

/** @emoji 🧭 Maps a model definition id to its CAD play pane id. */
export function cadPlayPaneForModelDefinition(modelDefinitionId: string): CadPlayPaneId | null {
  return CAD_PLAY_PANE_SPECS.find((row) => row.modelDefinitionId === modelDefinitionId)?.pane ?? null;
}

const CAD_PLAY_PANE_IDS: readonly CadPlayPaneId[] = ["shape", "building", "energy", "structure-classic"];

function emptyInteractionIdByPane(): Record<CadPlayPaneId, string> {
  return { shape: "", building: "", energy: "", "structure-classic": "" };
}

function emptyInteractionBootIdByPane(): Record<CadPlayPaneId, number> {
  return { shape: 0, building: 0, energy: 0, "structure-classic": 0 };
}

function emptySnapshotByPane(): Record<CadPlayPaneId, InteractionSnapshot | null> {
  return { shape: null, building: null, energy: null, "structure-classic": null };
}

function isSpatialComputeMode(value: string): value is SpatialComputeMode {
  return value === "fast" || value === "precise";
}
//#endregion 🔖Ids

//#region 🔖CadPlayHierarchy
function cadPlayModelDefinitionLabel(modelDefinitionId: string): string {
  const manifest = listModelDefinitionManifests().find((row) => row.id === modelDefinitionId);
  if (manifest?.label?.trim()) {
    return `${manifest.label}`;
  }
  const tail = modelDefinitionId.split(".").pop() ?? modelDefinitionId;
  return typologyObjectPascalFromLabel(tail.replace(/[._-]+/g, " "));
}

function cadPlaySelectionKey(target: SelectionTarget): string {
  return `${target.kind}:${target.id}`;
}

/** @emoji 🔢 Digest for hierarchy chrome when {@link Model} instances mutate in place (revision, objects, primitive counts). */
export function cadPlayModelsDigest(modelsByDefinitionId: Record<string, Model>): string {
  return Object.keys(modelsByDefinitionId)
    .sort((a, b) => a.localeCompare(b))
    .map((modelDefinitionId) => {
      const model = modelsByDefinitionId[modelDefinitionId];
      if (!model) return `${modelDefinitionId}:missing`;
      return [modelDefinitionId, model.revision, Object.keys(model.objects).length, Object.keys(model.solids).length, Object.keys(model.faces).length, Object.keys(model.vertices).length].join(":");
    })
    .join("|");
}

type CadPlayHierarchyPickContext = {
  readonly modelDefinitionId: string;
  readonly model: Model;
  readonly isSelected: (kind: SelectionTarget["kind"], id: string) => boolean;
  readonly onSelect: (modelDefinitionId: string, target: SelectionTarget) => void;
  readonly onHover: (modelDefinitionId: string, target: SelectionTarget | null) => void;
  readonly onToggleHidden: (modelDefinitionId: string, target: SelectionTarget) => void;
  readonly onToggleLocked: (modelDefinitionId: string, target: SelectionTarget) => void;
  readonly registerHighlight: (target: SelectionTarget, itemId: string, extraKeys?: readonly string[]) => void;
};

type CadPlayHierarchyReferenceContext = {
  readonly modelDefinitionId: string;
  readonly onSelect: (modelDefinitionId: string, referenceId: string) => void;
  readonly onHover: (modelDefinitionId: string, referenceId: string | null) => void;
  readonly onToggleHidden: (modelDefinitionId: string, referenceId: string) => void;
  readonly onToggleLocked: (modelDefinitionId: string, referenceId: string) => void;
  readonly registerHighlight: (referenceId: string, itemId: string) => void;
};

function cadPlayHierarchyReferenceChrome(reference: WorldReferenceProps, ctx: CadPlayHierarchyReferenceContext): Pick<TreeDataItem, "isHidden" | "actions" | "contextMenu"> {
  return {
    isHidden: reference.hidden === true,
    actions: [
      {
        id: "reference.hidden",
        icon: reference.hidden ? "eye-off" : "eye",
        title: reference.hidden ? "Show" : "Hide",
        onClick: () => ctx.onToggleHidden(ctx.modelDefinitionId, reference.id),
        revealOnHover: reference.hidden !== true,
      },
      {
        id: "reference.locked",
        icon: reference.locked ? "lock-open" : "lock",
        title: reference.locked ? "Unlock" : "Lock",
        onClick: () => ctx.onToggleLocked(ctx.modelDefinitionId, reference.id),
        revealOnHover: reference.locked !== true,
      },
    ],
    contextMenu: [
      {
        id: "reference.hidden",
        label: reference.hidden ? "Show" : "Hide",
        icon: reference.hidden ? "eye" : "eye-off",
        onSelect: () => ctx.onToggleHidden(ctx.modelDefinitionId, reference.id),
      },
      {
        id: "reference.locked",
        label: reference.locked ? "Unlock" : "Lock",
        icon: reference.locked ? "lock-open" : "lock",
        onSelect: () => ctx.onToggleLocked(ctx.modelDefinitionId, reference.id),
      },
    ],
  };
}

function cadPlayHierarchyReferenceHoverHandlers(ctx: CadPlayHierarchyReferenceContext, referenceId: string): Pick<TreeDataItem, "onPointerEnter" | "onPointerLeave"> {
  return {
    onPointerEnter: () => ctx.onHover(ctx.modelDefinitionId, referenceId),
    onPointerLeave: () => ctx.onHover(ctx.modelDefinitionId, null),
  };
}

function cadPlayHierarchyEntityChrome(model: Model, target: SelectionTarget, ctx: CadPlayHierarchyPickContext): Pick<TreeDataItem, "isHidden" | "actions" | "contextMenu"> {
  const flags = model.getEntityFlags(target.id);
  return {
    isHidden: flags.hidden === true,
    actions: [
      {
        id: `${target.kind}.hidden`,
        icon: flags.hidden ? "eye-off" : "eye",
        title: flags.hidden ? "Show" : "Hide",
        onClick: () => ctx.onToggleHidden(ctx.modelDefinitionId, target),
        revealOnHover: flags.hidden !== true,
      },
      {
        id: `${target.kind}.locked`,
        icon: flags.locked ? "lock-open" : "lock",
        title: flags.locked ? "Unlock" : "Lock",
        onClick: () => ctx.onToggleLocked(ctx.modelDefinitionId, target),
        revealOnHover: flags.locked !== true,
      },
    ],
    contextMenu: [
      {
        id: `${target.kind}.hidden`,
        label: flags.hidden ? "Show" : "Hide",
        icon: flags.hidden ? "eye" : "eye-off",
        onSelect: () => ctx.onToggleHidden(ctx.modelDefinitionId, target),
      },
      {
        id: `${target.kind}.locked`,
        label: flags.locked ? "Unlock" : "Lock",
        icon: flags.locked ? "lock-open" : "lock",
        onSelect: () => ctx.onToggleLocked(ctx.modelDefinitionId, target),
      },
    ],
  };
}

function cadPlayHierarchyHoverHandlers(ctx: CadPlayHierarchyPickContext, target: SelectionTarget): Pick<TreeDataItem, "onPointerEnter" | "onPointerLeave"> {
  return {
    onPointerEnter: () => ctx.onHover(ctx.modelDefinitionId, target),
    onPointerLeave: () => ctx.onHover(ctx.modelDefinitionId, null),
  };
}

function cadPlayPrimitiveChildTreeItem(node: ModelPrimitiveHierarchyNode, path: string, ctx: CadPlayHierarchyPickContext, objectId?: string): TreeDataItem {
  const childItems = node.children.map((child) => cadPlayPrimitiveChildTreeItem(child, `${path}.${child.kind}.${child.id}`, ctx, objectId));
  const target: SelectionTarget = { kind: node.kind, id: node.id, editable: true };
  const itemId = `cad-play-hierarchy.child.${path}`;
  ctx.registerHighlight(target, itemId, objectId ? [selectionTargetHoverKey({ kind: "object", id: objectId, editable: true })] : []);
  return {
    id: itemId,
    label: `${node.kind} ${node.id}`,
    isSelected: ctx.isSelected(node.kind, node.id),
    defaultOpen: node.kind === "solid" || node.kind === "shell" || node.kind === "face",
    onClick: () => ctx.onSelect(ctx.modelDefinitionId, target),
    ...cadPlayHierarchyHoverHandlers(ctx, target),
    ...cadPlayHierarchyEntityChrome(ctx.model, target, ctx),
    ...(childItems.length > 0 ? { items: childItems } : {}),
  };
}

function cadPlayPrimitiveSlotTreeItems(model: Model, modelDefinitionId: string, objectId: string, slot: string, primitiveRef: string, ctx: CadPlayHierarchyPickContext): TreeDataItem {
  const kind = resolvePrimitiveRefKind(model, primitiveRef) ?? "solid";
  const primitiveId = String(primitiveRef);
  const primitiveHierarchy = buildModelPrimitiveHierarchy(model, primitiveId);
  const childItems = (primitiveHierarchy?.children ?? []).map((child) => cadPlayPrimitiveChildTreeItem(child, `${modelDefinitionId}.${objectId}.${slot}.${child.kind}.${child.id}`, ctx, objectId));
  const target: SelectionTarget = { kind, id: primitiveId, editable: true };
  const objectTarget: SelectionTarget = { kind: "object", id: objectId, editable: true };
  const itemId = `cad-play-hierarchy.primitive.${modelDefinitionId}.${objectId}.${slot}`;
  ctx.registerHighlight(target, itemId, [selectionTargetHoverKey(objectTarget)]);
  return {
    id: itemId,
    label: `${slot}: ${kind} ${primitiveId}`,
    isSelected: ctx.isSelected(kind, primitiveId),
    defaultOpen: true,
    onClick: () => ctx.onSelect(ctx.modelDefinitionId, target),
    ...cadPlayHierarchyHoverHandlers(ctx, target),
    ...cadPlayHierarchyEntityChrome(ctx.model, target, ctx),
    items: childItems.length ? childItems : [{ id: `cad-play-hierarchy.primitive.${modelDefinitionId}.${objectId}.${slot}.child.empty`, label: "(empty)" }],
  };
}

/** @emoji 🌳 ModelSpace → model definition → object → primitive slot tree for CAD play workbench. */
export interface CadPlayHierarchyBuildResult {
  readonly sections: TreeDataSection[];
  readonly highlightKeyToItemIds: Readonly<Record<string, readonly string[]>>;
}

function registerCadPlayHierarchyHighlight(index: Record<string, string[]>, target: SelectionTarget, itemId: string, extraKeys: readonly string[] = []): void {
  const keys = new Set<string>();
  for (const seed of [selectionTargetHoverKey(target), ...extraKeys]) {
    for (const alias of spatialHoverKeyAliases(seed)) keys.add(alias);
  }
  for (const key of keys) {
    const bucket = index[key];
    if (bucket) {
      if (!bucket.includes(itemId)) bucket.push(itemId);
      continue;
    }
    index[key] = [itemId];
  }
}

function buildCadPlayExpandedSelectionKeys(model: Model, modelDefinitionId: string, selection: readonly SelectionTarget[]): ReadonlySet<string> {
  const keys = new Set<string>();
  for (const target of selection) {
    for (const alias of spatialHoverKeyAliases(selectionTargetHoverKey(target))) keys.add(alias);
  }
  for (const row of listModelObjectsForModelDefinition(model, modelDefinitionId)) {
    const objectKey = selectionTargetHoverKey({ kind: "object", id: String(row.id), editable: true });
    for (const [, primitiveRef] of objectPrimitiveEntries(row)) {
      const kind = resolvePrimitiveRefKind(model, primitiveRef) ?? "solid";
      const primitiveKey = selectionTargetHoverKey({ kind, id: String(primitiveRef), editable: true });
      if (keys.has(primitiveKey) || keys.has(`object:${primitiveRef}`)) keys.add(objectKey);
    }
  }
  return keys;
}

export function buildCadPlayHierarchySections(
  modelsByDefinitionId: Record<string, Model>,
  activeModelDefinitionId: string,
  selection: readonly SelectionTarget[],
  onSelect: (modelDefinitionId: string, target: SelectionTarget) => void,
  onHover: (modelDefinitionId: string, target: SelectionTarget | null) => void = () => {},
  onToggleHidden: (modelDefinitionId: string, target: SelectionTarget) => void = () => {},
  onToggleLocked: (modelDefinitionId: string, target: SelectionTarget) => void = () => {},
  referencesByModelDefinitionId: CadPlayReferencesByModelDefinitionId = {},
  selectedReference: CadPlaySelectedReference | null = null,
  onSelectReference: (modelDefinitionId: string, referenceId: string) => void = () => {},
  onHoverReference: (modelDefinitionId: string, referenceId: string | null) => void = () => {},
  onToggleReferenceHidden: (modelDefinitionId: string, referenceId: string) => void = () => {},
  onToggleReferenceLocked: (modelDefinitionId: string, referenceId: string) => void = () => {},
): CadPlayHierarchyBuildResult {
  const selectedKeys = new Set(selection.map(cadPlaySelectionKey));
  const isSelected = (kind: SelectionTarget["kind"], id: string, expandedKeys?: ReadonlySet<string>): boolean => {
    const key = `${kind}:${id}`;
    if (selectedKeys.has(key)) return true;
    if (!expandedKeys) return false;
    for (const alias of spatialHoverKeyAliases(key)) {
      if (expandedKeys.has(alias)) return true;
    }
    return false;
  };
  const highlightKeyToItemIds: Record<string, string[]> = {};
  const registerHighlight = (target: SelectionTarget, itemId: string, extraKeys: readonly string[] = []) => {
    registerCadPlayHierarchyHighlight(highlightKeyToItemIds, target, itemId, extraKeys);
  };
  const modelDefinitionIds = [
    ...new Set([...CAD_PLAY_PANE_SPECS.map((row) => row.modelDefinitionId), ...Object.keys(modelsByDefinitionId)]),
  ].sort((a, b) => a.localeCompare(b));
  const modelBranches: TreeDataItem[] = [];
  for (const modelDefinitionId of modelDefinitionIds) {
    const model = modelsByDefinitionId[modelDefinitionId] ?? new Model();
    const expandedSelectionKeys = buildCadPlayExpandedSelectionKeys(model, modelDefinitionId, selection);
    const pickCtx: CadPlayHierarchyPickContext = { modelDefinitionId, model, isSelected: (kind, id) => isSelected(kind, id, expandedSelectionKeys), onSelect, onHover, onToggleHidden, onToggleLocked, registerHighlight };
    const objectItems: TreeDataItem[] = listModelObjectsForModelDefinition(model, modelDefinitionId).map((object) => {
      const objectId = String(object.id);
      const typologyTail = object.typology.split(".").pop() ?? object.typology;
      const primitiveItems: TreeDataItem[] = objectPrimitiveEntries(object).map(([slot, primitiveRef]) => cadPlayPrimitiveSlotTreeItems(model, modelDefinitionId, objectId, slot, primitiveRef, pickCtx));
      const objectTarget: SelectionTarget = { kind: "object", id: objectId, editable: true };
      const objectItemId = `cad-play-hierarchy.object.${modelDefinitionId}.${objectId}`;
      registerHighlight(objectTarget, objectItemId);
      return {
        id: objectItemId,
        label: `${typologyObjectPascalFromLabel(typologyTail.replace(/[._-]+/g, " "))} (${objectId})`,
        description: object.typology,
        isSelected: isSelected("object", objectId, expandedSelectionKeys),
        defaultOpen: true,
        onClick: () => onSelect(modelDefinitionId, objectTarget),
        ...cadPlayHierarchyHoverHandlers(pickCtx, objectTarget),
        ...cadPlayHierarchyEntityChrome(model, objectTarget, pickCtx),
        items: primitiveItems.length ? primitiveItems : [{ id: `cad-play-hierarchy.object.${modelDefinitionId}.${objectId}.primitives.empty`, label: "(none)" }],
      };
    });
    const referenceCtx: CadPlayHierarchyReferenceContext = {
      modelDefinitionId,
      onSelect: onSelectReference,
      onHover: onHoverReference,
      onToggleHidden: onToggleReferenceHidden,
      onToggleLocked: onToggleReferenceLocked,
      registerHighlight: (referenceId, itemId) => {
        const key = cadPlayReferenceHoverKey(referenceId);
        const bucket = highlightKeyToItemIds[key];
        if (bucket) {
          if (!bucket.includes(itemId)) bucket.push(itemId);
          return;
        }
        highlightKeyToItemIds[key] = [itemId];
      },
    };
    const referenceItems: TreeDataItem[] = (referencesByModelDefinitionId[modelDefinitionId] ?? []).map((reference) => {
      const itemId = `cad-play-hierarchy.reference.${modelDefinitionId}.${reference.id}`;
      referenceCtx.registerHighlight(reference.id, itemId);
      return {
        id: itemId,
        label: reference.id,
        description: reference.source.url,
        isSelected: selectedReference?.modelDefinitionId === modelDefinitionId && selectedReference.id === reference.id,
        onClick: () => referenceCtx.onSelect(modelDefinitionId, reference.id),
        ...cadPlayHierarchyReferenceHoverHandlers(referenceCtx, reference.id),
        ...cadPlayHierarchyReferenceChrome(reference, referenceCtx),
      };
    });
    const referencesGroup: TreeDataItem = {
      id: `cad-play-hierarchy.references.${modelDefinitionId}`,
      label: "References",
      defaultOpen: true,
      items: referenceItems.length ? referenceItems : [{ id: `cad-play-hierarchy.references.${modelDefinitionId}.empty`, label: "(none)" }],
    };
    modelBranches.push({
      id: `cad-play-hierarchy.model.${modelDefinitionId}`,
      label: cadPlayModelDefinitionLabel(modelDefinitionId),
      description: modelDefinitionId,
      defaultOpen: modelDefinitionId === activeModelDefinitionId,
      items: [
        ...(objectItems.length ? objectItems : [{ id: `cad-play-hierarchy.model.${modelDefinitionId}.objects.empty`, label: "(no objects)" }]),
        referencesGroup,
      ],
    });
  }
  return {
    sections: modelBranches.length
      ? modelBranches.map((branch) => ({
          id: branch.id,
          label: branch.label,
          description: branch.description,
          defaultOpen: branch.defaultOpen,
          items: branch.items ?? [],
        }))
      : [{ id: "cad-play-hierarchy.empty", label: "ModelSpace", defaultOpen: true, items: [{ id: "cad-play-hierarchy.empty.msg", label: "(empty)" }] }],
    highlightKeyToItemIds,
  };
}
//#endregion 🔖CadPlayHierarchy

//#region 🔖Toolbar
/** @emoji 🧰 Snapshot for {@link buildCadPlayToolbarTools}. */
export interface CadPlayToolbarState {
  readonly activeModelDefinitionId: string;
  readonly selectionCount: number;
  readonly transfersTo: readonly TransformationSpec[];
  readonly transfersFrom: readonly TransformationSpec[];
}

/** @emoji 🔗 React host bridge for CAD play toolbar commands. */
export interface CadPlayHostBridge {
  getToolbarState(): CadPlayToolbarState;
  runHostCommand(command: string, args?: unknown): void;
}

const CAD_GUMBALL_GROUP_ICON: Record<CadGumballGroupKey, string> = {
  moveAxes: "move",
  movePlanes: "move-3d",
  rotate: "rotate-cw",
  scaleAxes: "maximize-2",
  scalePlanes: "scaling",
  scaleUniform: "box",
};

/** @emoji 🧰 Playground {@link AppTools} for CAD play (view, save, transfer). */
export function buildCadPlayToolbarTools(state: CadPlayToolbarState, controllerId: string): AppTools {
  const viewTools: ToolItem[] = listModelDefinitionManifests().map((row, index) => ({
    id: `cad.play.view.${row.id}`,
    kind: "toggle",
    iconId: "box",
    text: row.label,
    title: row.id,
    order: index,
    pressed: state.activeModelDefinitionId === row.id,
    controllerId,
    command: "focusModelDefinition",
    args: { modelDefinitionId: row.id },
  }));
  const saveTools: ToolItem[] = [
    {
      id: "cad.play.save.selected",
      kind: "button",
      iconId: "save",
      label: "Selected",
      order: 0,
      disabled: state.selectionCount === 0,
      controllerId,
      command: "saveSelected",
    },
    {
      id: "cad.play.save.modelspace",
      kind: "button",
      iconId: "hard-drive",
      label: "Model space",
      order: 1,
      controllerId,
      command: "saveInPlay",
    },
    {
      id: "cad.play.save.current",
      kind: "button",
      iconId: "save",
      label: "Current",
      order: 2,
      controllerId,
      command: "saveCurrent",
    },
    {
      id: "cad.play.save.load",
      kind: "button",
      iconId: "folder-open",
      label: "Load",
      order: 3,
      controllerId,
      command: "loadRawRequest",
    },
  ];
  const transferTools: ToolItem[] = [
    ...state.transfersTo.map((spec, index) => ({
      id: `cad.play.transfer.to.${qualifiedTransformationId(spec.modelDefinitionId, spec.id)}`,
      kind: "button" as const,
      iconId: "arrow-right",
      label: `→ ${spec.label}`,
      title: spec.target.modelDefinition,
      order: index,
      controllerId,
      command: "applyTransformation",
      args: { qid: qualifiedTransformationId(spec.modelDefinitionId, spec.id) },
    })),
    ...(state.transfersTo.length > 0 && state.transfersFrom.length > 0 ? [{ id: "cad.play.transfer.separator", kind: "separator" as const, order: state.transfersTo.length }] : []),
    ...state.transfersFrom.map((spec, index) => ({
      id: `cad.play.transfer.from.${qualifiedTransformationId(spec.modelDefinitionId, spec.id)}`,
      kind: "button" as const,
      iconId: "arrow-left",
      label: `← ${spec.label}`,
      title: spec.source.modelDefinition,
      order: state.transfersTo.length + (state.transfersTo.length > 0 && state.transfersFrom.length > 0 ? 1 : 0) + index,
      controllerId,
      command: "applyTransformation",
      args: { qid: qualifiedTransformationId(spec.modelDefinitionId, spec.id) },
    })),
  ];
  return {
    view: viewTools,
    save: saveTools,
    ...(transferTools.length > 0 ? { transfer: transferTools } : {}),
  };
}

/** @emoji 💬 Mirrors a live ui {@link EngagementSpec} into a React-neutral {@link WindowEngagement} whose option/input commands route back through the host bridge to the InteractionRepl callbacks. */
export function cadPlayEngagementMirror(engagement: EngagementSpec | null, pane: CadPlayPaneId): WindowEngagement | undefined {
  if (!engagement) return undefined;
  const options = engagement.options?.map((option) => ({
    id: option.id,
    label: option.label,
    pressed: option.pressed,
    disabled: option.disabled,
    command: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementOption", args: { pane, optionId: option.id } },
  }));
  const input = engagement.input
    ? {
        id: engagement.input.id,
        value: engagement.input.value,
        placeholder: engagement.input.placeholder,
        disabled: engagement.input.disabled,
        onChange: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementInput", args: { pane } },
        onSubmit: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementSubmit", args: { pane } },
        onRepeatLast: engagement.input.onRepeatLast
          ? { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementRepeatLast", args: { pane } }
          : undefined,
        onAbort: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementAbort", args: { pane } },
      }
    : undefined;
  const status = engagement.status?.map((row) => ({ id: row.id, text: typeof row.content === "string" ? row.content : String(row.content) }));
  const possibleEngagements = engagement.possibleEngagements?.map((row) => ({
    id: row.id,
    label: row.label,
    detail: row.detail,
    command: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementPossibleSelect", args: { pane, possibleId: row.id } },
  }));
  const control = engagementSpecControlMirror(engagement.control, CAD_PLAY_CONTROLLER_ID, { pane });
  return { sessionActive: engagement.sessionActive, options, input, control, status, possibleEngagements };
}

/** @emoji 💬 Placeholder engagement until a pane's {@link InteractionRepl} publishes a live snapshot (requires `input`). */
export function cadPlayPlaceholderPaneEngagement(pane: CadPlayPaneId): WindowEngagement {
  return {
    input: {
      id: "engagement-input",
      value: "",
      placeholder: "Command",
      onChange: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementInput", args: { pane } },
      onSubmit: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementSubmit", args: { pane } },
      onRepeatLast: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementRepeatLast", args: { pane } },
      onAbort: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementAbort", args: { pane } },
    },
  };
}

/** @emoji 💬 Resolves shell engagement for one pane, always exposing a command {@link WindowEngagementInput}. */
export function cadPlayResolvePaneEngagement(pane: CadPlayPaneId, stored?: WindowEngagement | undefined): WindowEngagement {
  const placeholder = cadPlayPlaceholderPaneEngagement(pane);
  if (!stored) return placeholder;
  if (stored.input) return stored;
  return { ...stored, input: placeholder.input };
}
//#endregion 🔖Toolbar

//#region 🔖Controller
/** @emoji 🎛 CAD play shell controller: quad viewports + playground toolbar categories. */
export class CadPlayShellController extends Controller {
  readonly mainMode = new ModeRuntime("main", "CAD", undefined);
  private hostBridge: CadPlayHostBridge | null = null;
  private computeModeByPane: Record<CadPlayPaneId, SpatialComputeMode>;
  private gumballConfigByPane: Record<CadPlayPaneId, CadGumballConfig>;
  private engagementByPane: Record<CadPlayPaneId, WindowEngagement | undefined>;
  private viewSeedByInstance = new Map<string, { readonly view: OrbitCameraViewId; readonly nonce: number }>();
  private projectionByInstance = new Map<string, OrbitCameraProjection>();

  constructor(commandBus: CommandBus, hostNotify: () => void) {
    super(CAD_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.computeModeByPane = {
      shape: "fast",
      building: "fast",
      energy: "fast",
      "structure-classic": "fast",
    };
    this.gumballConfigByPane = {
      shape: { ...CAD_GUMBALL_HIDDEN },
      building: { ...CAD_GUMBALL_HIDDEN },
      energy: { ...CAD_GUMBALL_HIDDEN },
      "structure-classic": { ...CAD_GUMBALL_HIDDEN },
    };
    this.engagementByPane = { shape: undefined, building: undefined, energy: undefined, "structure-classic": undefined };
    this.rebuildShellMode();
  }

  private computeMeasureForPane(pane: CadPlayPaneId): WindowMeasure {
    return {
      kind: "select",
      id: `${pane}-compute`,
      label: "Compute",
      value: this.computeModeByPane[pane],
      items: CAD_PLAY_COMPUTE_MODES.map((mode) => ({
        id: mode,
        value: mode,
        label: mode === "fast" ? "Fast" : "Precise",
      })),
      onChange: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "setComputeModeForPane", args: { pane } },
    };
  }

  private transformMeasureForPane(pane: CadPlayPaneId): WindowMeasure {
    const config = this.gumballConfigByPane[pane];
    return {
      kind: "group",
      id: `${pane}-transform`,
      label: "Transform",
      defaultOpen: true,
      children: CAD_GUMBALL_GROUPS.map((row) => ({
        kind: "toggle" as const,
        id: `${pane}-gumball-${row.key}`,
        iconId: CAD_GUMBALL_GROUP_ICON[row.key],
        label: row.label,
        text: row.label,
        pressed: config[row.key] !== false,
        onChange: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "setGumballConfigToggleForPane", args: { pane, key: row.key } },
      })),
    };
  }

  private paneEngagementForShell(pane: CadPlayPaneId): WindowEngagement {
    return cadPlayResolvePaneEngagement(pane, this.engagementByPane[pane]);
  }

  /** @emoji 📷 Returns the latest orbit-view seed for one shell instance (display templates / layouts). */
  getCameraViewSeedForInstance(instanceId?: string | null): { readonly view: OrbitCameraViewId; readonly seedKey: string } | null {
    const key = instanceId ?? "";
    const entry = this.viewSeedByInstance.get(key);
    if (!entry) {
      return null;
    }
    return { view: entry.view, seedKey: `${key}:${entry.nonce}` };
  }

  /** @emoji 📐 Returns the active orthographic/perspective mode for one shell instance. */
  getOrbitProjectionForInstance(instanceId?: string | null): OrbitCameraProjection {
    return this.projectionByInstance.get(instanceId ?? "") ?? "perspective";
  }

  private setOrbitProjection(projection: OrbitCameraProjection, instanceId?: string): void {
    this.projectionByInstance.set(instanceId ?? "", projection);
  }

  private applyOrbitCameraView(view: OrbitCameraViewId, instanceId?: string): void {
    const key = instanceId ?? "";
    const prev = this.viewSeedByInstance.get(key);
    this.viewSeedByInstance.set(key, { view, nonce: (prev?.nonce ?? 0) + 1 });
    this.projectionByInstance.set(key, orbitCameraProjectionForView(view));
  }

  /** @emoji 🔄 Rebuilds quad window kinds with per-pane compute measures and live interaction engagement per pane. */
  rebuildShellMode(): void {
    this.mainMode.windowKinds = CAD_PLAY_PANE_SPECS.map((row) => {
      const engagement = this.paneEngagementForShell(row.pane);
      enforcePlaygroundWindowEngagementInput(engagement, `CAD play window "${row.windowKindId}"`);
      return new WindowKindRuntime(
        row.windowKindId,
        row.label,
        row.bodyKey,
        undefined,
        [this.computeMeasureForPane(row.pane), this.transformMeasureForPane(row.pane)],
        engagement,
        CAD_PLAY_VIEW_TEMPLATES,
      );
    });
    this.mainMode.namedLayouts = [
      createNamedLayout("cad-play-quad", "Quad", CAD_PLAY_LAYOUT, "builtin", undefined, ["Workspace", "Quad"]),
      ...namedLayoutsFromOrbitViewDescriptors(CAD_PLAY_SHAPE_WINDOW_ID, createOrbitCameraViewLayoutDescriptors()),
    ];
  }

  /** @emoji 💬 Sets one pane's interaction engagement (from the live {@link InteractionRepl} snapshot) and re-renders the shell. */
  setPaneEngagement(pane: CadPlayPaneId, engagement: WindowEngagement | undefined): void {
    if (windowEngagementsEqual(this.engagementByPane[pane], engagement)) return;
    this.engagementByPane = { ...this.engagementByPane, [pane]: engagement };
    const windowKindId = CAD_PLAY_PANE_SPECS.find((row) => row.pane === pane)?.windowKindId;
    const existing = windowKindId ? this.mainMode.windowKinds.find((wk) => wk.id === windowKindId) : undefined;
    if (existing) {
      existing.engagement = this.paneEngagementForShell(pane);
      this.mainMode.windowKinds = [...this.mainMode.windowKinds];
    } else {
      this.rebuildShellMode();
    }
    this.emit();
  }

  /** @emoji ⚡ Returns compute mode for one quad pane. */
  getComputeModeForPane(pane: CadPlayPaneId): SpatialComputeMode {
    return this.computeModeByPane[pane];
  }

  /** @emoji ⚡ Snapshot of compute modes for all quad panes. */
  getComputeModeByPane(): Readonly<Record<CadPlayPaneId, SpatialComputeMode>> {
    return this.computeModeByPane;
  }

  /** @emoji 🎛 Returns gumball config for one quad pane. */
  getTransformGumballConfigForPane(pane: CadPlayPaneId): CadGumballConfig | null {
    const config = this.gumballConfigByPane[pane];
    return cadGumballConfigVisible(config) ? config : null;
  }

  /** @emoji 🎛 Snapshot of gumball configs for all quad panes. */
  getGumballConfigByPane(): Readonly<Record<CadPlayPaneId, CadGumballConfig>> {
    return this.gumballConfigByPane;
  }

  /** @emoji 🔗 Attaches the React host bridge used for toolbar commands and snapshots. */
  setHostBridge(bridge: CadPlayHostBridge | null): void {
    this.hostBridge = bridge;
    this.rebuildToolbarTools();
  }

  /** @emoji 🔄 Rebuilds {@link ModeRuntime.tools} from the latest host toolbar snapshot. */
  rebuildToolbarTools(): void {
    if (!this.hostBridge) {
      this.mainMode.tools = undefined;
      return;
    }
    this.mainMode.tools = buildCadPlayToolbarTools(this.hostBridge.getToolbarState(), this.id);
  }

  override run(command: string, args?: unknown): void {
    switch (command) {
      case "setComputeModeForPane": {
        const { pane, value } = args as { pane?: CadPlayPaneId; value?: string };
        if (!pane || !CAD_PLAY_PANE_SPECS.some((row) => row.pane === pane)) break;
        if (!value || !isSpatialComputeMode(value)) break;
        if (this.computeModeByPane[pane] === value) break;
        this.computeModeByPane = { ...this.computeModeByPane, [pane]: value };
        this.rebuildShellMode();
        break;
      }
      case "setGumballConfigToggleForPane": {
        const { pane, key } = args as { pane?: CadPlayPaneId; key?: CadGumballGroupKey };
        if (!pane || !CAD_PLAY_PANE_SPECS.some((row) => row.pane === pane)) break;
        if (!key || !CAD_GUMBALL_GROUPS.some((row) => row.key === key)) break;
        const current = this.gumballConfigByPane[pane];
        this.gumballConfigByPane = { ...this.gumballConfigByPane, [pane]: { ...current, [key]: current[key] === false } };
        this.rebuildShellMode();
        break;
      }
      case ORBIT_CAMERA_VIEW_COMMAND: {
        const view = (args as { view?: OrbitCameraViewId }).view;
        const instanceId = (args as { instanceId?: string }).instanceId;
        if (!view) break;
        this.applyOrbitCameraView(view, instanceId);
        break;
      }
      case "setOrbitProjection": {
        const projection = (args as { projection?: OrbitCameraProjection }).projection;
        const instanceId = (args as { instanceId?: string }).instanceId;
        if (projection !== "orthographic" && projection !== "perspective") break;
        this.setOrbitProjection(projection, instanceId);
        break;
      }
      case "focusModelDefinition":
      case "applyTransformation":
      case "saveSelected":
      case "saveInPlay":
      case "saveCurrent":
      case "loadRawRequest":
      case "deleteSelection":
      case "engagementOption":
      case "engagementInput":
      case "engagementSubmit":
      case "engagementRepeatLast":
      case "engagementAbort":
      case "engagementPossibleSelect":
      case "engagementControlChange":
      case "engagementControlCommit":
      case "engagementControlSelect":
        this.hostBridge?.runHostCommand(command, args);
        break;
      default:
        break;
    }
    this.rebuildToolbarTools();
    this.emit();
  }
}
//#endregion 🔖Controller

//#region 🔖Runtime
function cadPlayControllerFromContext(ctx: WindowBodyViewContext): CadPlayShellController | undefined {
  return ctx.runtime.getActiveApp()?.controller as CadPlayShellController | undefined;
}

function buildCadPlayDeclarativeBodyForPane(pane: CadPlayPaneId): (ctx: WindowBodyViewContext) => UiNode {
  return (ctx) => {
    if (!cadPlayControllerFromContext(ctx)) {
      return { type: "text", value: "Missing CAD play controller" };
    }
    return buildCadWindowBody(cadPlaySceneSurfaceIdForPane(pane), CAD_PLAY_CONTROLLER_ID);
  };
}

export const buildCadPlayShapeDeclarativeBody = buildCadPlayDeclarativeBodyForPane("shape");
export const buildCadPlayBuildingDeclarativeBody = buildCadPlayDeclarativeBodyForPane("building");
export const buildCadPlayEnergyDeclarativeBody = buildCadPlayDeclarativeBodyForPane("energy");
export const buildCadPlayStructureClassicDeclarativeBody = buildCadPlayDeclarativeBodyForPane("structure-classic");

export function buildCadPlayAppRuntime(controller: CadPlayShellController): AppRuntime {
  const app = createPlayAppRuntime(CAD_PLAY_APP_ID, "CAD play", controller, CAD_PLAY_LAYOUT as never, controller.mainMode);
  app.panelTabs = [];
  app.onActiveWindowChange = (shellWindowId) => {
    const pane = cadPlayPaneFromShellWindowId(shellWindowId);
    if (!pane) return;
    controller.run("focusModelDefinition", { modelDefinitionId: cadPlayModelDefinitionIdForPane(pane) });
  };
  return app;
}

/** @emoji 📝 Registers CAD play window bodies on the playground host. */
export function registerCadPlayDeclarativeBodies(): void {
  registerWindowBody(CAD_PLAY_SHAPE_BODY_KEY, buildCadPlayShapeDeclarativeBody);
  registerWindowBody(CAD_PLAY_BUILDING_BODY_KEY, buildCadPlayBuildingDeclarativeBody);
  registerWindowBody(CAD_PLAY_ENERGY_BODY_KEY, buildCadPlayEnergyDeclarativeBody);
  registerWindowBody(CAD_PLAY_STRUCTURE_CLASSIC_BODY_KEY, buildCadPlayStructureClassicDeclarativeBody);
}

/** @emoji 🚀 Creates CAD play {@link Platform} with declarative viewport body registered. */
export function buildCadPlayRuntime(): Platform {
  registerCadPlayDeclarativeBodies();
  const runtime = new Platform({ initialPanelVisibility: PRODUCT_SHELL_DEFAULT_PANEL_VISIBILITY });
  const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
  runtime.addApp(buildCadPlayAppRuntime(controller));
  return runtime;
}
//#endregion 🔖Runtime

import "./globals.css";
// #region 🔌Adapters
import {
  Button,
  Label,
  NavbarFixtureSelect,
  NAVBAR_NO_FIXTURE_ID,
  reactHostPort,
  type EngagementSpec,
  type TreeDataItem,
  type TreeDataSection,
  type UiTranslationKey,
} from "@ui/react";
import { createIconComponent } from "@ui/react";
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
  playgroundPanelSection,
  type SidePanelTabConfig,
} from "@framework/playground/renderer/react/shell";
import { registerSurfaceBinding, useShellWindowInstance, type UiCadHostSurfaceNode } from "@framework/platform/renderer/react";
import { defaultConstructRunner } from "@cad/js/query";
import geometryNakagin from "../../../asset/play/geometry.json";
import geometryLoom from "../../../asset/play/geometry-loom.json";
import geometryRoutes from "../../../asset/play/geometry-routes.json";
import geometrySmallBuilding from "../../../asset/play/small-building.model.json";
import geometryTallBuilding from "../../../asset/play/tall-building.model.json";
import geometryLargeBuilding from "../../../asset/play/large-building.model.json";
import geometryConcreteForestLeft from "../../../asset/play/hexagonal-cut-concrete-forest-left.model.json";
import geometryConcreteForestRight from "../../../asset/play/hexagonal-cut-concrete-forest-right.model.json";
import { BrepjsKernel, preciseSpatialKernelMath } from "@cad/js/kernel/brepjs";
import { statelyStateEngineProvider } from "@cad/js/machine/stately";
import {
  InteractionRepl,
  modelHasCommittedSolidsForDisplay,
  modelHasFactoryFaceDisplay,
  SelectionAttributesPanel,
  ModelStatsPanel,
  SelectionPropertiesPanel,
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
} from "../index";

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

const SHAPE_ASSETS = [
  { id: "nakagin-slice", key: "a", label: "Nakagin capsule", json: geometryNakagin as Record<string, unknown> },
  { id: "geometry-loom", key: "l", label: "Loom deck + pent loop + rail", json: geometryLoom as Record<string, unknown> },
  { id: "geometry-routes", key: "r", label: "Multi-route lattice", json: geometryRoutes as Record<string, unknown> },
  { id: "small-building", key: "s", label: "Small building", json: geometrySmallBuilding as Record<string, unknown> },
  { id: "tall-building", key: "t", label: "Tall building", json: geometryTallBuilding as Record<string, unknown> },
  { id: "large-building", key: "b", label: "Large building", json: geometryLargeBuilding as Record<string, unknown> },
  { id: "concrete-forest-left", key: "c", label: "Concrete forest (left)", json: { modelSpace: geometryConcreteForestLeft, activeModelDefinitionId: defaultModelDefinitionId() } as Record<string, unknown> },
  { id: "concrete-forest-right", key: "d", label: "Concrete forest (right)", json: geometryConcreteForestRight as Record<string, unknown> },
] as const;

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

function ensurePlayShapeModel(models: Readonly<Record<string, Model>>): Record<string, Model> {
  if (models[defaultModelDefinitionId()]) return { ...models };
  return { ...models, [defaultModelDefinitionId()]: new Model() };
}

function parseModelSpaceJson(raw: unknown): ModelSpace | null {
  if (!raw || typeof raw !== "object") return null;
  const row = raw as Record<string, unknown>;
  if (row.schema !== "spatial.modelspace/v1" || !Array.isArray(row.models)) return null;
  return ModelSpace.fromJSON(row as ModelSpaceJsonSnapshot);
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

function modelsFromCadJson(json: unknown): Record<string, Model> {
  const bundle = json && typeof json === "object" ? (json as SpatialExchangeBundle) : null;
  const modelSpace = parseModelSpaceJson(bundle?.modelSpace ?? json);
  if (modelSpace) return ensurePlayQuadModelSlots(recordFromModelSpace(modelSpace));
  return ensurePlayQuadModelSlots({
    [defaultModelDefinitionId()]: parseModelJson(bundle?.model ?? json) ?? new Model(),
  });
}

function activeModelDefinitionIdFromSpatialJson(json: unknown): string {
  const bundle = json && typeof json === "object" ? (json as SpatialExchangeBundle) : null;
  if (typeof bundle?.activeModelDefinitionId === "string") return bundle.activeModelDefinitionId;
  const modelSpace = parseModelSpaceJson(bundle?.modelSpace ?? json);
  return Object.keys(modelSpace?.models ?? {})[0] ?? defaultModelDefinitionId();
}

function flushModelsRecord(models: Readonly<Record<string, Model>>, activeId: string, live: Model): Record<string, Model> {
  return { ...models, [activeId]: Model.fromJSON(live.toJSON()) };
}

function modelSpaceFromRecord(models: Readonly<Record<string, Model>>): ModelSpace {
  const space = new ModelSpace();
  for (const id of Object.keys(models).sort()) space.link(id, models[id]!);
  return space;
}

function recordFromModelSpace(space: ModelSpace): Record<string, Model> {
  const out: Record<string, Model> = {};
  for (const id of Object.keys(space.models).sort()) {
    const model = space.models[id];
    if (model) out[id] = Model.fromJSON(model.toJSON());
  }
  return out;
}

function transformationSourceScore(models: Readonly<Record<string, Model>>, spec: TransformationSpec): number {
  const source = models[spec.source.modelDefinition];
  if (!source) return 0;
  const scopedObjects = listModelObjectsForModelDefinition(source, spec.source.modelDefinition).length;
  if (scopedObjects === 0) return 0;
  let score = scopedObjects;
  if (isShapeModelDefinition(spec.source.modelDefinition)) score += 50;
  const targetDepth = spec.target.modelDefinition.split(".").length;
  const sourceDepth = spec.source.modelDefinition.split(".").length;
  if (targetDepth > sourceDepth) score += 20 * (targetDepth - sourceDepth);
  return score;
}

function ensureDerivedModelInSpace(models: Readonly<Record<string, Model>>, definitionId: string): Record<string, Model> {
  const withShape = ensurePlayShapeModel(models);
  if (withShape[definitionId]) return withShape;
  if (isShapeModelDefinition(definitionId)) return withShape;
  const candidates = [...listTransformationsIntoModelDefinition(definitionId)].sort(
    (a, b) => transformationSourceScore(withShape, b) - transformationSourceScore(withShape, a),
  );
  for (const spec of candidates) {
    const source = withShape[spec.source.modelDefinition];
    if (!source || listModelObjectsForModelDefinition(source, spec.source.modelDefinition).length === 0) continue;
    try {
      return { ...withShape, [definitionId]: applyTransformation(spec, source, preciseSpatialKernelMath) };
    } catch {
      continue;
    }
  }
  return withShape;
}

/** @emoji 🌌 Ensures play quad model slots exist without running derive transforms. */
export function ensurePlayQuadModelSlots(models: Readonly<Record<string, Model>>): Record<string, Model> {
  const withShape = ensurePlayShapeModel(models);
  let next = { ...withShape };
  for (const row of CAD_PLAY_PANE_SPECS) {
    if (!next[row.modelDefinitionId]) next[row.modelDefinitionId] = new Model();
  }
  return next;
}

/** @emoji 🌌 Derives missing play quad models from linked sources (explicit transfer toolbar only). */
export function ensureCadPlayQuadModels(models: Readonly<Record<string, Model>>): Record<string, Model> {
  let next = ensurePlayQuadModelSlots(models);
  next = ensureDerivedModelInSpace(next, CAD_PLAY_ENERGY_MODEL_DEFINITION_ID);
  next = ensureDerivedModelInSpace(next, AEC_BUILDING_STRUCTURE_MODEL_DEFINITION_ID);
  next = ensureDerivedModelInSpace(next, CAD_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID);
  return next;
}

function emptyPlayModels(): Record<string, Model> {
  return ensurePlayQuadModelSlots({});
}

/** @emoji 🧭 Resolves one pane's geometry without falling back to spatial.shape. */
export function cadPlayPaneGeometry(models: Readonly<Record<string, Model>>, modelDefinitionId: string, liveModel: Model): Model {
  if (isShapeModelDefinition(modelDefinitionId)) {
    return models[defaultModelDefinitionId()] ?? liveModel;
  }
  return models[modelDefinitionId] ?? new Model();
}

//#region 🔖CadPlayChrome
export interface CadPlayChromeSnapshot {
  readonly modelsByDefinitionId: Record<string, Model>;
  readonly referencesByModelDefinitionId: CadPlayReferencesByModelDefinitionId;
  readonly activeModelDefinitionId: string;
  readonly selection: readonly SelectionTarget[];
  readonly selectedReference: CadPlaySelectedReference | null;
  readonly hoveredKey: string | null;
  readonly selectTarget: (modelDefinitionId: string, target: SelectionTarget) => void;
  readonly hoverTarget: (modelDefinitionId: string, target: SelectionTarget | null) => void;
  readonly toggleHidden: (modelDefinitionId: string, target: SelectionTarget) => void;
  readonly toggleLocked: (modelDefinitionId: string, target: SelectionTarget) => void;
  readonly selectReference: (modelDefinitionId: string, referenceId: string) => void;
  readonly hoverReference: (modelDefinitionId: string, referenceId: string | null) => void;
  readonly toggleReferenceHidden: (modelDefinitionId: string, referenceId: string) => void;
  readonly toggleReferenceLocked: (modelDefinitionId: string, referenceId: string) => void;
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

class CadPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  constructor(
    private readonly buildSections: () => TreeDataSection[],
    private readonly buildHighlightedIds: () => readonly string[],
  ) {
    super();
  }

  resolveTab(): SidePanelTabConfig {
    return {
      id: CAD_PLAY_HIERARCHY_TAB_ID,
      icon: ListTree,
      name: "Hierarchy",
      order: 0,
      tree: new CallbackTreePanelDefinition(() => this.buildSections(), () => this.buildHighlightedIds()),
    };
  }

  buildTab(): SidePanelTabConfig {
    return this.resolveTab();
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
      setReferencesByModelDefinitionId(cadPlayDefaultReferencesByModelDefinitionId());
      setSelectedReference(null);
      setActiveModelDefinitionId(defaultModelDefinitionId());
      setModelsLoadEpoch((epoch) => epoch + 1);
    } else {
      const asset = SHAPE_ASSETS.find((candidate) => candidate.id === id);
      if (!asset) return;
      setModelsByDefinitionId(modelsFromCadJson(asset.json));
      setActiveModelDefinitionId(activeModelDefinitionIdFromSpatialJson(asset.json));
      setModelsLoadEpoch((epoch) => epoch + 1);
    }
    setModelDefinitionRevision((r) => r + 1);
  }, []);

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
      if (modelDefinitionId !== activeModelDefinitionId) {
        handleActiveModelDefinitionChange(modelDefinitionId);
      }
      setSelectedReference({ modelDefinitionId, id: referenceId });
      setRendererSelectionByModel((prev) => replWithRendererSelectionTargets(prev, modelDefinitionId, []));
    },
    [activeModelDefinitionId, handleActiveModelDefinitionChange],
  );

  const hoverHierarchyReference = reactHostPort.useCallback(
    (modelDefinitionId: string, referenceId: string | null) => {
      if (referenceId && modelDefinitionId !== activeModelDefinitionId) {
        handleActiveModelDefinitionChange(modelDefinitionId);
      }
      pointerFocusRef.current!.setHoverFromSource(CAD_PLAY_HOVER_SOURCE_HIERARCHY, referenceId ? cadPlayReferenceHoverKey(referenceId) : null);
    },
    [activeModelDefinitionId, handleActiveModelDefinitionChange],
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
      selectHierarchyReference(modelDefinitionId, referenceId);
      pointerFocusRef.current!.setHoverFromSource(CAD_PLAY_HOVER_SOURCE_CANVAS, cadPlayReferenceHoverKey(referenceId));
    },
    [selectHierarchyReference],
  );

  const handleReferenceHover = reactHostPort.useCallback(
    (modelDefinitionId: string, referenceId: string | null) => {
      if (referenceId) {
        pointerFocusRef.current!.setHoverFromSource(CAD_PLAY_HOVER_SOURCE_CANVAS, cadPlayReferenceHoverKey(referenceId));
        return;
      }
      if (pointerFocusRef.current!.getSnapshot().hover?.startsWith("reference:")) {
        pointerFocusRef.current!.setHoverFromSource(CAD_PLAY_HOVER_SOURCE_CANVAS, null);
      }
    },
    [],
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

  const hoveredReferenceId = reactHostPort.useMemo(() => {
    const hoverKey = pointerFocus.hover;
    if (!hoverKey?.startsWith("reference:")) {
      return null;
    }
    return hoverKey.slice("reference:".length);
  }, [pointerFocus.hover]);

  const revealedReferenceIds = reactHostPort.useMemo(() => {
    if (!hoveredReferenceId) {
      return new Set<string>();
    }
    const hiddenReference = Object.values(referencesByModelDefinitionId)
      .flat()
      .find((row) => row.id === hoveredReferenceId && row.hidden === true);
    return hiddenReference ? new Set([hoveredReferenceId]) : new Set<string>();
  }, [hoveredReferenceId, referencesByModelDefinitionId]);

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
    publishCadPlayChrome({
      modelsByDefinitionId: flushedModelsByDefinitionId,
      referencesByModelDefinitionId,
      activeModelDefinitionId,
      selection: selectionInScope,
      selectedReference,
      hoveredKey: pointerFocus.hover,
      selectTarget: selectHierarchyTarget,
      hoverTarget: hoverHierarchyTarget,
      toggleHidden: toggleHierarchyHidden,
      toggleLocked: toggleHierarchyLocked,
      selectReference: selectHierarchyReference,
      hoverReference: hoverHierarchyReference,
      toggleReferenceHidden: toggleHierarchyReferenceHidden,
      toggleReferenceLocked: toggleHierarchyReferenceLocked,
    });
    return () => publishCadPlayChrome(null);
  }, [
    activeModelDefinitionId,
    flushedModelsByDefinitionId,
    flushedModelsDigest,
    hoverHierarchyReference,
    hoverHierarchyTarget,
    modelDefinitionRevision,
    pointerFocus.hover,
    publishCadPlayChrome,
    referencesByModelDefinitionId,
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
    const asset = SHAPE_ASSETS.find((g) => g.id === shapeAssetId);
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
  }, [activeModelDefinitionId, focusModelDefinition, handleApplyTransformation, handleDeleteSelection, handleLoadRawRequest, handleSaveCurrent, handleSaveInPlay, handleSaveSelected, selectionInScope, shellController]);

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

/** @emoji 🎯 Details panel: attribute and property editors for the current selection only. */
function CadPlayDetailsAside(): ReactNode {
  const { activeModelDefinitionId, liveModel, selectionInScope, handleModelAttributesChange, brepjsKernel } = useCadPlayModelSpace();
  return (
    <>
      <ModelStatsPanel model={liveModel} kernel={brepjsKernel} activeModelDefinitionId={activeModelDefinitionId} selection={selectionInScope} selectionCount={selectionInScope.length} />
      {selectionInScope.length === 0 ? (
        <p className="text-muted-foreground leading-snug">Select a primitive or object in the canvas or workbench hierarchy to edit attributes and properties.</p>
      ) : (
        <>
          <SelectionAttributesPanel model={liveModel} activeModelDefinitionId={activeModelDefinitionId} selection={selectionInScope} selectionCount={selectionInScope.length} onModelChange={handleModelAttributesChange} />
          <SelectionPropertiesPanel model={liveModel} kernel={brepjsKernel} activeModelDefinitionId={activeModelDefinitionId} selection={selectionInScope} selectionCount={selectionInScope.length} />
        </>
      )}
    </>
  );
}

/** @emoji 🧪 Navbar fixture dropdown for CAD play shape sources (replaces workbench catalog picker). */
function CadPlayFixtureNavbarSelect(): ReactNode {
  const { shapeAssetId, handleShapeAssetChange } = useCadPlayModelSpace();
  return (
    <NavbarFixtureSelect
      id="cad.play.fixture"
      value={shapeAssetId || NAVBAR_NO_FIXTURE_ID}
      options={SHAPE_ASSETS.map((row) => ({
        id: row.id,
        label: `[${row.key}] ${row.label} (${modelVertexCount(row.json)} verts)`,
      }))}
      onValueChange={(fixtureId) => handleShapeAssetChange(fixtureId === NAVBAR_NO_FIXTURE_ID ? "" : fixtureId)}
    />
  );
}

/** @emoji 📦 Workbench catalog: file I/O status (fixture picker lives in the navbar; toolbar handles save/load). */
function CadPlayCatalogAside(): ReactNode {
  const { activeModelDefinitionId, fileStatus } = useCadPlayModelSpace();
  const statusTone = fileStatus.startsWith("Load failed") || fileStatus.startsWith("Save failed") ? "text-destructive" : "text-muted-foreground";
  return (
    <>
      {!isShapeModelDefinition(activeModelDefinitionId) ? (
        <p className="text-muted-foreground leading-snug">
          Shape fixtures apply to <code className="text-foreground">{defaultModelDefinitionId()}</code>. Use the navbar fixture menu or focus the Shape pane.
        </p>
      ) : null}
      {fileStatus ? <p className={statusTone}>{fileStatus}</p> : null}
    </>
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
  resolveTab(): SidePanelTabConfig {
    return {
      id: "cad-play-catalog",
      icon: Shapes,
      name: "Catalog",
      order: 1,
      tree: new StaticTreePanelDefinition({
        sections: [playgroundPanelSection("cad-play-catalog.section", "Catalog", <CadPlayCatalogAside />)],
      }),
    };
  }
}

class CadPlayDetailsPanelDefinition extends PureSidePanelTabDefinition {
  resolveTab(): SidePanelTabConfig {
    return {
      id: "cad-play-details",
      icon: ListTree,
      name: "Selection",
      order: 0,
      tree: new StaticTreePanelDefinition({
        sections: [playgroundPanelSection("cad-play-details.section", "Selection", <CadPlayDetailsAside />)],
      }),
    };
  }
}

function CadPlayRoot(): ReactNode {
  const runtimeRef = reactHostPort.useRef<Platform | null>(null);
  const shellControllerRef = reactHostPort.useRef<CadPlayShellController | null>(null);
  const [chromeSnapshot, setChromeSnapshot] = reactHostPort.useState<CadPlayChromeSnapshot | null>(null);
  if (!runtimeRef.current) {
    registerCadPlayChrome();
    runtimeRef.current = buildCadPlayRuntime();
    runtimeRef.current.setActiveAppId(CAD_PLAY_APP_ID);
    shellControllerRef.current = runtimeRef.current.getActiveApp()?.controller as CadPlayShellController;
  }
  const shellController = shellControllerRef.current;
  if (!shellController) {
    return null;
  }
  const chromeContextValue = reactHostPort.useMemo<CadPlayChromeContextValue>(() => ({ snapshot: chromeSnapshot, publishSnapshot: setChromeSnapshot }), [chromeSnapshot]);
  const chromeKey = chromeSnapshot
    ? `${chromeSnapshot.activeModelDefinitionId}\u0001${chromeSnapshot.selection.map((row) => `${row.kind}:${row.id}`).join(",")}\u0001${chromeSnapshot.selectedReference?.id ?? ""}\u0001${cadPlayModelsDigest(chromeSnapshot.modelsByDefinitionId)}\u0001${cadPlayReferencesDigest(chromeSnapshot.referencesByModelDefinitionId)}`
    : "";
  const hierarchyBuild = reactHostPort.useMemo(() => {
    if (!chromeSnapshot) {
      return { sections: [] as TreeDataSection[], highlightKeyToItemIds: {} as Readonly<Record<string, readonly string[]>> };
    }
    return buildCadPlayHierarchySections(
      chromeSnapshot.modelsByDefinitionId,
      chromeSnapshot.activeModelDefinitionId,
      chromeSnapshot.selection,
      chromeSnapshot.selectTarget,
      chromeSnapshot.hoverTarget,
      chromeSnapshot.toggleHidden,
      chromeSnapshot.toggleLocked,
      chromeSnapshot.referencesByModelDefinitionId,
      chromeSnapshot.selectedReference,
      chromeSnapshot.selectReference,
      chromeSnapshot.hoverReference,
      chromeSnapshot.toggleReferenceHidden,
      chromeSnapshot.toggleReferenceLocked,
    );
  }, [chromeKey, chromeSnapshot]);
  const hierarchyHighlightedIds = reactHostPort.useMemo(() => {
    const hoveredKey = chromeSnapshot?.hoveredKey;
    if (!hoveredKey) {
      return [];
    }
    const ids = new Set<string>();
    for (const alias of spatialHoverKeyAliases(hoveredKey)) {
      for (const itemId of hierarchyBuild.highlightKeyToItemIds[alias] ?? []) ids.add(itemId);
    }
    return [...ids];
  }, [chromeSnapshot?.hoveredKey, hierarchyBuild.highlightKeyToItemIds]);
  const workbenchTabs = reactHostPort.useMemo(
    () => [
      new CadPlayCatalogPanelDefinition().resolveTab(),
      ...(chromeSnapshot
        ? [
            new CadPlayHierarchyPanelDefinition(
              () => hierarchyBuild.sections,
              () => hierarchyHighlightedIds,
            ).resolveTab(),
          ]
        : []),
    ],
    [chromeSnapshot, hierarchyBuild.sections, hierarchyHighlightedIds],
  );
  const detailsTabs = reactHostPort.useMemo(() => [new CadPlayDetailsPanelDefinition().resolveTab()], []);
  const slotNavbarCenter = reactHostPort.useMemo(() => <CadPlayFixtureNavbarSelect />, []);
  return (
    <CadPlayChromeContext.Provider value={chromeContextValue}>
      <CadPlayModelSpaceProvider runtime={runtimeRef.current} shellController={shellController}>
        <CadPlayLoadInput />
        <PlaygroundView runtime={runtimeRef.current} defaultAppId={CAD_PLAY_APP_ID} augmentPanelTabs={{ workbench: workbenchTabs, details: detailsTabs }} slotNavbarCenter={slotNavbarCenter} />
      </CadPlayModelSpaceProvider>
    </CadPlayChromeContext.Provider>
  );
}

if (typeof document !== "undefined" && !import.meta.vitest) {
  const el = document.getElementById("root");
  if (el) {
    mountPlaygroundApp(
      <StrictMode>
        <CadPlayRoot />
      </StrictMode>,
    );
  }
}
//#endregion 🔖PlaygroundHost

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("cad play display shell", () => {
    it("resolves pane from shell instance ids", () => {
      expect(cadPlayPaneFromShellWindowId(CAD_PLAY_SHAPE_WINDOW_ID)).toBe("shape");
      expect(cadPlayPaneFromShellWindowId("win-cad-play-energy-abc")).toBe("energy");
      expect(cadPlayPaneFromShellWindowId("win-unknown")).toBeNull();
    });

    it("registers a window kind per quad pane for the display panel", () => {
      const runtime = new Platform();
      const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
      expect(controller.mainMode.windowKinds.map((row) => row.id)).toEqual([
        CAD_PLAY_SHAPE_WINDOW_ID,
        CAD_PLAY_BUILDING_WINDOW_ID,
        CAD_PLAY_ENERGY_WINDOW_ID,
        CAD_PLAY_STRUCTURE_CLASSIC_WINDOW_ID,
      ]);
    });

    it("attaches the orbit view template tree to each window kind", () => {
      const runtime = new Platform();
      const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
      expect(controller.mainMode.windowKinds[0]?.templates.map((row) => row.id)).toEqual(["orthographic", "perspective"]);
      expect(controller.mainMode.namedLayouts.some((row) => row.id === "view-quad-standard")).toBe(true);
    });

    it("applies setOrbitCameraView per shell instance", () => {
      const runtime = new Platform();
      const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
      controller.run(ORBIT_CAMERA_VIEW_COMMAND, { view: "top", instanceId: "win-a" });
      const first = controller.getCameraViewSeedForInstance("win-a");
      expect(first?.view).toBe("top");
      controller.run(ORBIT_CAMERA_VIEW_COMMAND, { view: "isometricNe", instanceId: "win-a" });
      const second = controller.getCameraViewSeedForInstance("win-a");
      expect(second?.view).toBe("isometricNe");
      expect(second?.seedKey).not.toBe(first?.seedKey);
    });
  });

  describe("CadPlayShellController compute mode", () => {
    it("stores independent compute modes per quad pane", () => {
      const runtime = new Platform();
      const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
      expect(controller.getComputeModeForPane("shape")).toBe("fast");
      controller.run("setComputeModeForPane", { pane: "energy", value: "precise" });
      expect(controller.getComputeModeForPane("energy")).toBe("precise");
      expect(controller.getComputeModeForPane("shape")).toBe("fast");
      const energyWindow = controller.mainMode.windowKinds.find((row) => row.id === CAD_PLAY_ENERGY_WINDOW_ID);
      expect(energyWindow?.measures[0]).toMatchObject({ kind: "select", value: "precise" });
    });
  });

  describe("CadPlayShellController transform gumball toggles", () => {
    it("stores independent gumball configs per quad pane", () => {
      const runtime = new Platform();
      const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
      expect(controller.getTransformGumballConfigForPane("shape")).toBeNull();
      controller.run("setGumballConfigToggleForPane", { pane: "energy", key: "rotate" });
      expect(controller.getTransformGumballConfigForPane("energy")).toMatchObject({ rotate: true });
      expect(controller.getTransformGumballConfigForPane("shape")).toBeNull();
      const energyWindow = controller.mainMode.windowKinds.find((row) => row.id === CAD_PLAY_ENERGY_WINDOW_ID);
      expect(energyWindow?.measures[1]).toMatchObject({ kind: "group", id: "energy-transform", label: "Transform" });
    });
  });

  describe("CadPlayShellController delete selection", () => {
    it("forwards deleteSelection to the host bridge", () => {
      const runtime = new Platform();
      const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
      const calls: string[] = [];
      controller.setHostBridge({
        getToolbarState: () => ({
          activeModelDefinitionId: defaultModelDefinitionId(),
          selectionCount: 1,
          transfersTo: [],
          transfersFrom: [],
        }),
        runHostCommand: (command) => calls.push(command),
      });
      controller.run("deleteSelection");
      expect(calls).toEqual(["deleteSelection"]);
    });
  });

  describe("cad play delete selection", () => {
    it("deleteObjectsFromModel removes selected objects but keeps solid primitives", async () => {
      const { preciseSpatialKernelMath: M } = await import("@cad/js/kernel/brepjs");
      const { applyModelDiff, solidRef } = await import("@cad/js/core");
      const model = new Model();
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solidRef("box-solid")));
      model.objects["box-a"] = { id: "box-a" as never, typology: "spatial.shape.primitive.box", primitives: { solid: "box-solid" } };
      model.objects["box-b"] = { id: "box-b" as never, typology: "spatial.shape.primitive.box", primitives: { solid: "box-solid" } };
      const selection: SelectionTarget[] = [
        { kind: "object", id: "box-a", editable: true },
        { kind: "solid", id: "box-solid", editable: true },
      ];
      const objectIds = deletableObjectIdsFromSelection(selection);
      deleteObjectsFromModel(model, objectIds);
      expect(model.objects["box-a"]).toBeUndefined();
      expect(model.objects["box-b"]).toBeTruthy();
      expect(Object.keys(model.solids)).toEqual(["box-solid"]);
    });
  });

  describe("interaction spec identity", () => {
    it("loadSpatialInteraction returns one compiled instance per interaction id", () => {
      const first = loadSpatialInteraction("primitive.box");
      const second = loadSpatialInteraction("primitive.box");
      expect(first).toBe(second);
    });
  });

  describe("windowEngagementsEqual", () => {
    it("treats engagement snapshots with the same visible fields and commands as equal", () => {
      const command = { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementPossibleSelect", args: { pane: "shape", possibleId: "primitive.box" } };
      const left: WindowEngagement = {
        input: { id: "engagement-input", value: "box", placeholder: "Command" },
        status: [{ id: "engagement-step", text: "Step: Idle" }],
        possibleEngagements: [{ id: "primitive.box", label: "Box", detail: "b", command }],
      };
      const right: WindowEngagement = {
        input: { id: "engagement-input", value: "box", placeholder: "Command" },
        status: [{ id: "engagement-step", text: "Step: Idle" }],
        possibleEngagements: [{ id: "primitive.box", label: "Box", detail: "b", command }],
      };
      expect(windowEngagementsEqual(left, right)).toBe(true);
    });

    it("differs when suggestion routing commands are added", () => {
      const left: WindowEngagement = {
        input: { id: "engagement-input", value: "box", placeholder: "Command" },
        possibleEngagements: [{ id: "surface.extrudeCrv", label: "ExtrudeCrv", detail: "e" }],
      };
      const right: WindowEngagement = {
        input: { id: "engagement-input", value: "box", placeholder: "Command" },
        possibleEngagements: [
          {
            id: "surface.extrudeCrv",
            label: "ExtrudeCrv",
            detail: "e",
            command: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementPossibleSelect", args: { pane: "shape", possibleId: "surface.extrudeCrv" } },
          },
        ],
      };
      expect(windowEngagementsEqual(left, right)).toBe(false);
    });
  });

  describe("cadPlayEngagementMirror", () => {
    it("returns undefined for a null engagement", () => {
      expect(cadPlayEngagementMirror(null, "shape")).toBeUndefined();
    });

    it("mirrors a ui engagement spec into neutral commands routed to the controller", () => {
      const mirror = cadPlayEngagementMirror(
        {
          options: [{ id: "confirm", label: "Confirm", onPress: () => {} }],
          input: { id: "in", value: "box", placeholder: "Command", onSubmit: () => {} },
          status: [{ id: "state", content: "Step: Idle" }],
        },
        "energy",
      );
      expect(mirror?.options?.[0]).toMatchObject({
        id: "confirm",
        label: "Confirm",
        command: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementOption", args: { pane: "energy", optionId: "confirm" } },
      });
      expect(mirror?.input).toMatchObject({ value: "box", onSubmit: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementSubmit", args: { pane: "energy" } } });
      expect(mirror?.status?.[0]).toEqual({ id: "state", text: "Step: Idle" });
    });

    it("mirrors possible engagements for autocomplete routing", () => {
      const mirror = cadPlayEngagementMirror(
        {
          possibleEngagements: [{ id: "primitive.box", label: "Box", detail: "b", onSelect: () => {} }],
        },
        "shape",
      );
      expect(mirror?.possibleEngagements?.[0]).toMatchObject({
        id: "primitive.box",
        label: "Box",
        command: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementPossibleSelect", args: { pane: "shape", possibleId: "primitive.box" } },
      });
    });
  });

  describe("CadPlayShellController engagement", () => {
    it("skips shell notify when mirrored engagement content is unchanged", () => {
      const runtime = new Platform();
      const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
      let generation = runtime.generation;
      const engagement: WindowEngagement = {
        input: { id: "engagement-input", value: "", placeholder: "Command" },
        possibleEngagements: [
          {
            id: "primitive.box",
            label: "Box",
            detail: "b",
            command: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementPossibleSelect", args: { pane: "shape", possibleId: "primitive.box" } },
          },
        ],
      };
      controller.setPaneEngagement("shape", engagement);
      const afterFirst = runtime.generation;
      controller.setPaneEngagement("shape", { ...engagement, input: { ...engagement.input!, value: "" } });
      expect(runtime.generation).toBe(afterFirst);
      expect(afterFirst).toBeGreaterThan(generation);
    });

    it("notifies shell when mirrored engagement gains suggestion routing commands", () => {
      const runtime = new Platform();
      const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
      const engagement: WindowEngagement = {
        input: { id: "engagement-input", value: "Ex", placeholder: "Command" },
        possibleEngagements: [{ id: "surface.extrudeCrv", label: "ExtrudeCrv", detail: "e" }],
      };
      controller.setPaneEngagement("shape", engagement);
      const afterFirst = runtime.generation;
      controller.setPaneEngagement("shape", {
        ...engagement,
        possibleEngagements: [
          {
            ...engagement.possibleEngagements![0]!,
            command: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementPossibleSelect", args: { pane: "shape", possibleId: "surface.extrudeCrv" } },
          },
        ],
      });
      expect(runtime.generation).toBeGreaterThan(afterFirst);
    });

    it("attaches engagement per pane and routes pane-scoped engagement commands to the host bridge", () => {
      const runtime = new Platform();
      const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
      const calls: { command: string; args?: unknown }[] = [];
      controller.setHostBridge({
        getToolbarState: () => ({ activeModelDefinitionId: defaultModelDefinitionId(), selectionCount: 0, transfersTo: [], transfersFrom: [] }),
        runHostCommand: (command, args) => calls.push({ command, args }),
      });
      controller.setPaneEngagement("shape", { options: [{ id: "confirm", label: "Confirm" }] });
      controller.setPaneEngagement("energy", { options: [{ id: "wall", label: "Wall" }] });
      const shapeWindow = controller.mainMode.windowKinds.find((row) => row.id === CAD_PLAY_SHAPE_WINDOW_ID);
      const energyWindow = controller.mainMode.windowKinds.find((row) => row.id === CAD_PLAY_ENERGY_WINDOW_ID);
      const buildingWindow = controller.mainMode.windowKinds.find((row) => row.id === CAD_PLAY_BUILDING_WINDOW_ID);
      expect(shapeWindow?.engagement?.options?.[0]?.id).toBe("confirm");
      expect(energyWindow?.engagement?.options?.[0]?.id).toBe("wall");
      expect(buildingWindow?.engagement?.input?.id).toBe("engagement-input");
      expect(buildingWindow?.engagement?.options).toBeUndefined();
      controller.run("engagementOption", { pane: "shape", optionId: "confirm" });
      controller.run("engagementSubmit", { pane: "energy", value: "box" });
      expect(calls).toEqual([
        { command: "engagementOption", args: { pane: "shape", optionId: "confirm" } },
        { command: "engagementSubmit", args: { pane: "energy", value: "box" } },
      ]);
    });
  });

  describe("buildCadPlayAppRuntime", () => {
    it("requires engagement.input on every quad window kind", () => {
      const runtime = new Platform();
      const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
      const app = buildCadPlayAppRuntime(controller);
      for (const windowKind of app.windowKinds) {
        expect(windowKind.engagement?.input?.id).toBe("engagement-input");
      }
    });

    it("focuses the pane model definition when the active window changes", () => {
      const runtime = new Platform();
      const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
      const calls: { command: string; args?: unknown }[] = [];
      controller.setHostBridge({
        getToolbarState: () => ({ activeModelDefinitionId: defaultModelDefinitionId(), selectionCount: 0, transfersTo: [], transfersFrom: [] }),
        runHostCommand: (command, args) => calls.push({ command, args }),
      });
      const app = buildCadPlayAppRuntime(controller);
      app.onActiveWindowChange?.(CAD_PLAY_ENERGY_WINDOW_ID);
      expect(calls).toEqual([
        {
          command: "focusModelDefinition",
          args: { modelDefinitionId: cadPlayModelDefinitionIdForPane("energy") },
        },
      ]);
    });
  });

  describe("cadPlayPaneForModelDefinition", () => {
    it("maps each quad model definition to its pane", () => {
      expect(cadPlayPaneForModelDefinition(defaultModelDefinitionId())).toBe("shape");
      expect(cadPlayPaneForModelDefinition(CAD_PLAY_BUILDING_MODEL_DEFINITION_ID)).toBe("building");
      expect(cadPlayPaneForModelDefinition(CAD_PLAY_ENERGY_MODEL_DEFINITION_ID)).toBe("energy");
      expect(cadPlayPaneForModelDefinition(CAD_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID)).toBe("structure-classic");
    });
  });

  describe("buildCadPlayToolbarTools", () => {
    it("registers view, save, and transfer categories", () => {
      const tools = buildCadPlayToolbarTools(
        {
          activeModelDefinitionId: defaultModelDefinitionId(),
          selectionCount: 0,
          transfersTo: [],
          transfersFrom: [],
        },
        CAD_PLAY_CONTROLLER_ID,
      );
      expect(tools.view?.length).toBeGreaterThan(0);
      expect(tools.save?.map((row) => row.id)).toEqual(["cad.play.save.selected", "cad.play.save.modelspace", "cad.play.save.current", "cad.play.save.load"]);
      expect(tools.save?.[0]?.disabled).toBe(true);
      expect(tools.transform).toBeUndefined();
    });
  });

  describe("CAD play runtime", () => {
    it("builds quad viewport bodies for each pane", () => {
      const runtime = buildCadPlayRuntime();
      const ctx = { runtime, activeModeId: "main", generation: 0 } as const;
      expect(
        buildCadPlayShapeDeclarativeBody({
          ...ctx,
          windowKindId: CAD_PLAY_SHAPE_WINDOW_ID,
          bodyKey: CAD_PLAY_SHAPE_BODY_KEY,
        }),
      ).toEqual(buildCadWindowBody(CAD_PLAY_SHAPE_SCENE_SURFACE_ID, CAD_PLAY_CONTROLLER_ID));
      expect(
        buildCadPlayEnergyDeclarativeBody({
          ...ctx,
          windowKindId: CAD_PLAY_ENERGY_WINDOW_ID,
          bodyKey: CAD_PLAY_ENERGY_BODY_KEY,
        }),
      ).toEqual(buildCadWindowBody(CAD_PLAY_ENERGY_SCENE_SURFACE_ID, CAD_PLAY_CONTROLLER_ID));
    });

    it("registers four window kinds in quad layout", () => {
      const app = buildCadPlayRuntime().getActiveApp();
      expect(app?.defaultLayout).toEqual(CAD_PLAY_LAYOUT);
      expect(app?.windowKinds.map((row) => row.id)).toEqual([CAD_PLAY_SHAPE_WINDOW_ID, CAD_PLAY_BUILDING_WINDOW_ID, CAD_PLAY_ENERGY_WINDOW_ID, CAD_PLAY_STRUCTURE_CLASSIC_WINDOW_ID]);
    });

    it("uses empty declarative side tab slots", () => {
      const app = buildCadPlayRuntime().getActiveApp();
      expect(app?.panelTabs).toEqual([]);
    });

    it("cadPlayModelsDigest changes when object rows are added", () => {
      const model = parseModelJson({
        schema: "spatial.model/v1",
        revision: 0,
        objects: {},
        geometry: { anchors: [], vertices: [], edges: [], wires: [], faces: [], shells: [], solids: [] },
      });
      expect(model).not.toBeNull();
      const before = cadPlayModelsDigest({ "spatial.shape": model! });
      model!.objects["box1"] = {
        id: "box1",
        typology: "spatial.shape.primitive.box",
        primitives: { solid: "solid-1" },
      };
      model!.bump();
      const after = cadPlayModelsDigest({ "spatial.shape": model! });
      expect(after).not.toBe(before);
    });

    it("buildCadPlayHierarchySections lists objects after box commit object binding", async () => {
      const { BrepjsKernel } = await import("@cad/js/kernel/brepjs");
      const spec = loadSpatialInteraction("primitive.box")!;
      const model = new Model();
      const kernel = new BrepjsKernel() as never;
      const rt = createInteractionRuntime(spec, {
        kernel,
        document: { model, nodes: [] },
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0], modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 3, 0], modifiers: {} });
      await rt.send({ kind: "set.height", value: 4, modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      const build = buildCadPlayHierarchySections({ [defaultModelDefinitionId()]: model }, defaultModelDefinitionId(), [], () => {});
      const modelBranch = build.sections[0]?.items?.[0];
      expect(modelBranch?.items?.some((row) => row.label !== "(no objects)")).toBe(true);
    });

    it("buildCadPlayHierarchySections maps hover keys to tree item ids", async () => {
      const { preciseSpatialKernelMath: M } = await import("@cad/js/kernel/brepjs");
      const { applyModelDiff, solidRef } = await import("@cad/js/core");
      const model = new Model();
      const solid = solidRef("solid-1");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
      model.objects["box1"] = {
        id: "box1",
        typology: "spatial.shape.primitive.box",
        primitives: { solid: String(solid) },
      };
      const build = buildCadPlayHierarchySections({ "spatial.shape": model }, "spatial.shape", [], () => {});
      expect(build.highlightKeyToItemIds["object:box1"]).toEqual(["cad-play-hierarchy.object.spatial.shape.box1"]);
      expect(build.highlightKeyToItemIds["solid:solid-1"]).toContain("cad-play-hierarchy.primitive.spatial.shape.box1.solid");
    });

    it("buildCadPlayHierarchySections nests child primitives under primitive slots", async () => {
      const { preciseSpatialKernelMath: M } = await import("@cad/js/kernel/brepjs");
      const { applyModelDiff, solidRef } = await import("@cad/js/core");
      const model = new Model();
      const solid = solidRef("solid-1");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
      model.objects["box1"] = {
        id: "box1",
        typology: "spatial.shape.primitive.box",
        primitives: { solid: String(solid) },
      };
      const build = buildCadPlayHierarchySections({ "spatial.shape": model }, "spatial.shape", [], () => {});
      const primitiveNode = build.sections[0]?.items?.[0]?.items?.[0];
      expect(primitiveNode?.label).toContain("solid:");
      const shellNode = primitiveNode?.items?.[0];
      expect(shellNode?.label).toContain("shell");
      const faceNode = shellNode?.items?.[0];
      expect(faceNode?.label).toContain("face");
      const wireNode = faceNode?.items?.[0];
      expect(wireNode?.label).toContain("wire");
      const edgeNode = wireNode?.items?.[0];
      expect(edgeNode?.label).toContain("edge");
      expect(edgeNode?.items?.some((row) => row.label.includes("vertex"))).toBe(true);
    });

    it("buildCadPlayHierarchySections lists references under model definitions with hide and lock chrome", () => {
      const references = { [defaultModelDefinitionId()]: CAD_PLAY_DEFAULT_WORLD_REFERENCES };
      const build = buildCadPlayHierarchySections({ [defaultModelDefinitionId()]: new Model() }, defaultModelDefinitionId(), [], () => {}, () => {}, () => {}, () => {}, references);
      const modelBranch = build.sections.find((row) => row.id === `cad-play-hierarchy.model.${defaultModelDefinitionId()}`);
      const referencesGroup = modelBranch?.items?.find((row) => row.id === `cad-play-hierarchy.references.${defaultModelDefinitionId()}`);
      expect(referencesGroup?.items?.some((row) => row.id === `cad-play-hierarchy.reference.${defaultModelDefinitionId()}.ref-sketch`)).toBe(true);
      const referenceRow = referencesGroup?.items?.find((row) => row.id === `cad-play-hierarchy.reference.${defaultModelDefinitionId()}.ref-sketch`);
      expect(referenceRow?.actions?.some((row) => row.id === "reference.hidden")).toBe(true);
      expect(referenceRow?.actions?.some((row) => row.id === "reference.locked")).toBe(true);
      expect(build.highlightKeyToItemIds[cadPlayReferenceHoverKey("ref-sketch")]).toContain(`cad-play-hierarchy.reference.${defaultModelDefinitionId()}.ref-sketch`);
    });

    it("updateCadPlayReferenceInMap toggles reference flags", () => {
      const initial = cadPlayDefaultReferencesByModelDefinitionId();
      const next = updateCadPlayReferenceInMap(initial, defaultModelDefinitionId(), "ref-sketch", { hidden: true, locked: true });
      expect(next[defaultModelDefinitionId()]?.find((row) => row.id === "ref-sketch")).toEqual(expect.objectContaining({ hidden: true, locked: true }));
    });

    it("buildCadPlayHierarchySections lists kernel-imported STEP objects", async () => {
      const { preciseSpatialKernelMath: M } = await import("@cad/js/kernel/brepjs");
      const { applyModelDiff, defaultModelDefinitionId, solidRef } = await import("@cad/js/core");
      const model = new Model();
      const solid = solidRef("imported-solid");
      applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
      model.objects["object-imported"] = {
        id: "object-imported",
        typology: "spatial.shape.kernel.solid",
        primitives: { solid: String(solid) },
      };
      const mdId = defaultModelDefinitionId();
      const build = buildCadPlayHierarchySections({ [mdId]: model }, mdId, [], () => {});
      const objectNode = build.sections[0]?.items?.[0];
      expect(objectNode?.label).toContain("object-imported");
      expect(objectNode?.description).toBe("spatial.shape.kernel.solid");
    });
  });

  describe("CAD play typology chrome", () => {
    it("lists energy typologies from model definition scope", () => {
      const scope = resolveModelDefinitionScope("aec.building.energy");
      const labels = scope.typologies.map((row) => typologyObjectPascalFromLabel(row.label));
      expect(labels).toContain("BasePlate");
      expect(labels).toContain("ExternalWall");
      expect(labels).toContain("Roof");
    });
  });

  describe("CAD play shared model space", () => {
    it("cadPlayPaneModel keeps each pane bound to its model definition", () => {
      const shape = new Model();
      shape.objects["box1"] = { id: "box1", typology: "spatial.shape.primitive.box", primitives: { solid: "solid-1" } };
      const building = new Model();
      building.objects["site1"] = { id: "site1", typology: "aec.building.site", primitives: {} };
      const models = { [defaultModelDefinitionId()]: shape, [CAD_PLAY_BUILDING_MODEL_DEFINITION_ID]: building };
      expect(cadPlayPaneModel(models, "shape").objects["box1"]).toBeDefined();
      expect(cadPlayPaneModel(models, "building").objects["site1"]).toBeDefined();
      expect(cadPlayPaneModel(models, "shape")).not.toBe(cadPlayPaneModel(models, "building"));
    });

    it("cadPlayPaneGeometry never borrows spatial.shape for building pane", () => {
      const models = modelsFromCadJson({
        modelSpace: geometryConcreteForestLeft,
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      const building = cadPlayPaneGeometry(models, CAD_PLAY_BUILDING_MODEL_DEFINITION_ID, new Model());
      const shape = cadPlayPaneGeometry(models, defaultModelDefinitionId(), new Model());
      expect(listModelObjectsForModelDefinition(building, CAD_PLAY_BUILDING_MODEL_DEFINITION_ID)).toHaveLength(12);
      expect(listModelObjectsForModelDefinition(shape, defaultModelDefinitionId())).toHaveLength(1);
      const shapeOnly = ensurePlayQuadModelSlots({ [defaultModelDefinitionId()]: shape });
      const emptyBuilding = cadPlayPaneGeometry(shapeOnly, CAD_PLAY_BUILDING_MODEL_DEFINITION_ID, new Model());
      expect(listModelObjectsForModelDefinition(emptyBuilding, CAD_PLAY_BUILDING_MODEL_DEFINITION_ID)).toHaveLength(0);
      expect(Object.keys(emptyBuilding.solids).length).toBe(0);
    });
  });

  describe("CAD play model bootstrap", () => {
    it("emptyPlayModels always seeds spatial.shape", () => {
      expect(emptyPlayModels()[defaultModelDefinitionId()]).toBeInstanceOf(Model);
    });

    it("modelsFromCadJson on empty model space still seeds spatial.shape", () => {
      const models = modelsFromCadJson(new ModelSpace().toJSON());
      expect(models[defaultModelDefinitionId()]).toBeInstanceOf(Model);
    });

    it("modelsFromCadJson loads fixture models under spatial.shape", () => {
      const models = modelsFromCadJson(geometrySmallBuilding);
      expect(models[defaultModelDefinitionId()]?.objects).not.toEqual({});
    });

    it("modelsFromCadJson loads concrete forest left building BIM model", () => {
      const models = modelsFromCadJson({
        modelSpace: geometryConcreteForestLeft,
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      const building = models[CAD_PLAY_BUILDING_MODEL_DEFINITION_ID];
      expect(building).toBeInstanceOf(Model);
      expect(Object.keys(building!.objects).length).toBeGreaterThan(0);
      const listed = listModelObjectsForModelDefinition(building!, CAD_PLAY_BUILDING_MODEL_DEFINITION_ID);
      expect(listed.length).toBe(12);
      const typologies = new Set(listed.map((row) => row.typology));
      expect(typologies.has("building.building.slab")).toBe(true);
      expect(typologies.has("building.building.beam")).toBe(true);
      expect(typologies.has("building.building.column")).toBe(true);
      expect(models[CAD_PLAY_ENERGY_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
      expect(models[CAD_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
      expect(listModelObjectsForModelDefinition(models[CAD_PLAY_ENERGY_MODEL_DEFINITION_ID]!, CAD_PLAY_ENERGY_MODEL_DEFINITION_ID).length).toBe(1);
      expect(
        listModelObjectsForModelDefinition(models[CAD_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID]!, CAD_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID).length,
      ).toBe(11);
    });

    it("ensureDerivedModelInSpace keeps spatial.shape for shape definition", () => {
      const models = ensureDerivedModelInSpace({}, defaultModelDefinitionId());
      expect(models[defaultModelDefinitionId()]).toBeInstanceOf(Model);
    });

    it("ensurePlayQuadModelSlots seeds all play panes without derive transforms", () => {
      const models = ensurePlayQuadModelSlots({});
      expect(models[defaultModelDefinitionId()]).toBeInstanceOf(Model);
      expect(models[CAD_PLAY_BUILDING_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
      expect(models[CAD_PLAY_ENERGY_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
      expect(models[CAD_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
      expect(Object.keys(models[CAD_PLAY_ENERGY_MODEL_DEFINITION_ID]!.objects).length).toBe(0);
    });

    it("ensureCadPlayQuadModels seeds all four play panes", () => {
      const models = ensureCadPlayQuadModels({});
      expect(models[defaultModelDefinitionId()]).toBeInstanceOf(Model);
      expect(models[CAD_PLAY_BUILDING_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
      expect(models[CAD_PLAY_ENERGY_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
      expect(models[CAD_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
    });

    it("ensureCadPlayQuadModels derives missing models only when called explicitly", () => {
      const loaded = modelsFromCadJson({
        modelSpace: geometryConcreteForestLeft,
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      expect(listModelObjectsForModelDefinition(loaded[CAD_PLAY_ENERGY_MODEL_DEFINITION_ID]!, CAD_PLAY_ENERGY_MODEL_DEFINITION_ID).length).toBe(1);
      const derived = ensureCadPlayQuadModels(loaded);
      expect(
        listModelObjectsForModelDefinition(derived[CAD_PLAY_ENERGY_MODEL_DEFINITION_ID]!, CAD_PLAY_ENERGY_MODEL_DEFINITION_ID).length,
      ).toBe(1);
    });

    it("buildCadPlayHierarchySections lists concrete forest BIM objects across play definitions", () => {
      const models = modelsFromCadJson({
        modelSpace: geometryConcreteForestLeft,
        activeModelDefinitionId: defaultModelDefinitionId(),
      });
      const build = buildCadPlayHierarchySections(models, defaultModelDefinitionId(), [], () => {});
      const buildingBranch = build.sections.find((row) => row.id === `cad-play-hierarchy.model.${CAD_PLAY_BUILDING_MODEL_DEFINITION_ID}`);
      expect(buildingBranch?.items?.length).toBe(12);
      const energyBranch = build.sections.find((row) => row.id === `cad-play-hierarchy.model.${CAD_PLAY_ENERGY_MODEL_DEFINITION_ID}`);
      expect(energyBranch?.items?.length).toBe(1);
      const structureBranch = build.sections.find(
        (row) => row.id === `cad-play-hierarchy.model.${CAD_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID}`,
      );
      expect(structureBranch?.items?.length).toBe(11);
      expect(build.sections.some((row) => row.id === `cad-play-hierarchy.model.${defaultModelDefinitionId()}`)).toBe(true);
    });
  });
}
//#endregion 🧪Tests
