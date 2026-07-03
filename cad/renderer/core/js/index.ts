// #region 🧲Header
/** @emoji 📐 CAD play app core — controller, hierarchy, inspector, runtime. */
// #endregion 🧲Header

import {
	createPlaygroundApp,
	createProductPlaygroundPlatform,
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
  type ToolLeaf,
  toolCollection,
  type WindowBodyViewContext,
  type WindowEngagement,
  type WindowMeasure,
  type UiNode,
  type WindowLayout,
  type WindowTemplate,
  enforcePlaygroundWindowEngagementInput,
  windowEngagementsEqual,
  type CommandDescriptor,
  isPlaygroundExampleLocked,
  playgroundLockedFixtureId,
  FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
  FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
  FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
  uiDeclarativeSectionsToTree,
  type UiTreeNode} from "@semio-tech/framework-playground-core";
import { registerOsMediaExportHandler } from "@semio-tech/framework-os-core";
import { exportModelSpaceToGlb, exportModelSpaceToObj } from "@semio-tech/cad-js-kernel-brepjs";
import {
  ORBIT_CAMERA_VIEW_COMMAND,
  applyWorldReferenceTransform,
  createOrbitCameraViewLayoutDescriptors,
  createOrbitCameraViewTemplates,
  orbitCameraProjectionForView,
  patchWorldReferenceProps,
  worldEntitySelectable,
  worldQuatToEulerDegrees,
  worldReferenceOrientation,
  worldReferenceScaleVec,
  type OrbitCameraViewId,
  type OrbitCameraProjection,
  type WorldReferenceProps,
  type WorldReferenceRelocatePayload,
} from "@semio-tech/infinite-world-r3f";
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
  type ObjectRef,
  deleteObjectsFromModel,
  deletableObjectIdsFromSelection,
} from "@semio-tech/cad-js-core";
import { bootstrapCadModules } from "@semio-tech/cad-js-runtime";
import { AEC_BUILDING_MODEL_DEFINITION_ID } from "@semio-tech/cad-js-module-aec-building";
import { AEC_BUILDING_ENERGY_MODEL_DEFINITION_ID } from "@semio-tech/cad-js-module-aec-building-energy";
import {
  AEC_BUILDING_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID,
  AEC_BUILDING_STRUCTURE_MODEL_DEFINITION_ID,
} from "@semio-tech/cad-js-module-aec-building-structure";
import { preciseSpatialKernelMath } from "@semio-tech/cad-js-kernel-brepjs";
import type { TreeDataItem, TreeDataSection } from "@semio-tech/ui-react";
import { formatNumber, type EngagementControl, type EngagementSpec } from "@semio-tech/ui-react";

bootstrapCadModules();

/** @emoji ⚡ Per-window compute mode options for CAD play window measures. */
export const CAD_PLAY_COMPUTE_MODES: readonly SpatialComputeMode[] = ["fast", "precise"];

function selectionTargetHoverKey(target: SelectionTarget): string {
  return `${target.kind}:${target.id}`;
}

function spatialHoverKeyAliases(key: string | null | undefined): ReadonlySet<string> {
  if (!key) return new Set();
  const out = new Set<string>([key]);
  const colon = key.indexOf(":");
  if (colon <= 0) return out;
  const kind = key.slice(0, colon);
  const id = key.slice(colon + 1);
  if (kind === "object") out.add(`solid:${id}`);
  if (kind === "solid") out.add(`object:${id}`);
  return out;
}

function engagementSpecControlMirror(
  control: EngagementControl | undefined,
  controllerId: string,
  commandArgs: Record<string, unknown>,
): WindowEngagement["control"] {
  if (!control) return undefined;
  if (control.kind === "ring") {
    return {
      kind: "ring",
      id: control.id,
      label: control.label,
      value: control.value,
      disabled: control.disabled,
      options: control.options.map((row) => ({ id: row.id, label: row.label, disabled: row.disabled })),
      onSelect: control.onSelect ? { controllerId, command: "engagementControlSelect", args: { ...commandArgs } } : undefined,
    };
  }
  return {
    kind: control.kind,
    id: control.id,
    label: control.label,
    value: control.value,
    min: control.min,
    max: control.max,
    step: control.step,
    unit: control.unit,
    disabled: control.disabled,
    onChange: control.onChange ? { controllerId, command: "engagementControlChange", args: { ...commandArgs, controlId: control.id } } : undefined,
    onCommit: control.onCommit ? { controllerId, command: "engagementControlCommit", args: { ...commandArgs, controlId: control.id } } : undefined,
  };
}

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

export const CAD_PLAY_CONCRETE_FOREST_FIXTURE_IDS = ["concrete-forest-left", "concrete-forest-right"] as const;

export const CAD_PLAY_CONCRETE_FOREST_WORLD_REFERENCES: WorldReferenceProps[] = [
  { id: "ref-concrete-forest", source: { url: "/cad-fixture/concrete-forest-reference.png", mediaKind: "image" }, origin: [-24, -18, 0.01], widthWorld: 22 },
];

/** @emoji 🧪 Whether a shape fixture id loads the concrete forest reference planes. */
export function cadPlayIsConcreteForestFixture(fixtureId: string): boolean {
  return (CAD_PLAY_CONCRETE_FOREST_FIXTURE_IDS as readonly string[]).includes(fixtureId);
}

/** @emoji 🖼️ Reference planes for the active shape fixture, replicated per quad model definition. */
export function cadPlayReferencesForFixture(fixtureId: string): Record<string, WorldReferenceProps[]> {
  if (!cadPlayIsConcreteForestFixture(fixtureId)) {
    return {};
  }
  return Object.fromEntries(
    CAD_PLAY_PANE_SPECS.map((row) => [
      row.modelDefinitionId,
      CAD_PLAY_CONCRETE_FOREST_WORLD_REFERENCES.map((reference) => ({ ...reference })),
    ]),
  );
}

function cadPlayEmptyReferencesByModelDefinitionId(): Record<string, WorldReferenceProps[]> {
  return {};
}

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

export function cadPlayDefaultReferencesByModelDefinitionId(): Record<string, WorldReferenceProps[]> {
  return cadPlayEmptyReferencesByModelDefinitionId();
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

export const CAD_PLAY_PANE_SPECS: readonly {
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
    defaultOpen: false,
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
        defaultOpen: false,
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
        isSelected:
          selectedReference?.modelDefinitionId === modelDefinitionId &&
          selectedReference.id === reference.id &&
          worldEntitySelectable(reference),
        onClick: () => referenceCtx.onSelect(modelDefinitionId, reference.id),
        ...cadPlayHierarchyReferenceHoverHandlers(referenceCtx, reference.id),
        ...cadPlayHierarchyReferenceChrome(reference, referenceCtx),
      };
    });
    const referencesGroup: TreeDataItem = {
      id: `cad-play-hierarchy.references.${modelDefinitionId}`,
      label: "References",
      defaultOpen: false,
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
      : [{ id: "cad-play-hierarchy.empty", label: "ModelSpace", defaultOpen: false, items: [{ id: "cad-play-hierarchy.empty.msg", label: "(empty)" }] }],
    highlightKeyToItemIds,
  };
}

export function buildCadPlayHierarchyPendingSections(): TreeDataSection[] {
  return [{ id: "cad-play-hierarchy.pending", label: "Hierarchy", items: [{ id: "cad-play-hierarchy.pending.item", label: "(empty)" }] }];
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
  const viewTools: ToolLeaf[] = listModelDefinitionManifests().map((row, index) => ({
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
  const saveTools: ToolLeaf[] = [
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
  const transferTools: ToolLeaf[] = [
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
  return [
    toolCollection("view", "layout-grid", viewTools),
    toolCollection("save", "save", saveTools),
    ...(transferTools.length > 0 ? [toolCollection("transfer", "arrow-right-left", transferTools)] : []),
  ];
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
  readonly mainMode = new ModeRuntime("main", "Edit", undefined);
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
      defaultOpen: false,
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
      case "patchCadPlayReference":
      case "patchCadPlaySelection":
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
  const app = createPlayAppRuntime(CAD_PLAY_APP_ID, "CAD", controller, CAD_PLAY_LAYOUT as never, controller.mainMode);
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

type ModelJsonSnapshot = ReturnType<Model["toJSON"]>;
type ModelSpaceJsonSnapshot = ReturnType<ModelSpace["toJSON"]>;

interface SpatialExchangeBundle {
  readonly model?: ModelJsonSnapshot;
  readonly modelSpace?: ModelSpaceJsonSnapshot;
  readonly activeModelDefinitionId?: string;
}

function ensurePlayShapeModel(models: Readonly<Record<string, Model>>): Record<string, Model> {
  if (models[defaultModelDefinitionId()]) return { ...models };
  return { ...models, [defaultModelDefinitionId()]: new Model() };
}

function parseModelSpaceJson(raw: unknown): ModelSpace | null {
  if (!raw || typeof raw !== "object") return null;
  const row = raw as Record<string, unknown>;
  if (row.schema !== "spatial.modelspace" || !Array.isArray(row.models)) return null;
  return ModelSpace.fromJSON(row as ModelSpaceJsonSnapshot);
}

/** @emoji 📦 Loads play models from spatial exchange JSON. */
export function modelsFromCadJson(json: unknown): Record<string, Model> {
  const bundle = json && typeof json === "object" ? (json as SpatialExchangeBundle) : null;
  const modelSpace = parseModelSpaceJson(bundle?.modelSpace ?? json);
  if (modelSpace) return ensurePlayQuadModelSlots(recordFromModelSpace(modelSpace));
  return ensurePlayQuadModelSlots({
    [defaultModelDefinitionId()]: parseModelJson(bundle?.model ?? json) ?? new Model(),
  });
}

/** @emoji 🎯 Resolves active model definition id from spatial JSON. */
export function activeModelDefinitionIdFromSpatialJson(json: unknown): string {
  const bundle = json && typeof json === "object" ? (json as SpatialExchangeBundle) : null;
  if (typeof bundle?.activeModelDefinitionId === "string") return bundle.activeModelDefinitionId;
  const modelSpace = parseModelSpaceJson(bundle?.modelSpace ?? json);
  return Object.keys(modelSpace?.models ?? {})[0] ?? defaultModelDefinitionId();
}

/** @emoji 💾 Flushes one live model into a models record. */
export function flushModelsRecord(models: Readonly<Record<string, Model>>, activeId: string, live: Model): Record<string, Model> {
  return { ...models, [activeId]: Model.fromJSON(live.toJSON()) };
}

/** @emoji 🌌 Builds a model space from play models record. */
export function modelSpaceFromRecord(models: Readonly<Record<string, Model>>): ModelSpace {
  const space = new ModelSpace();
  for (const id of Object.keys(models).sort()) space.link(id, models[id]!);
  return space;
}

/** @emoji 🌌 Materializes a models record from model space. */
export function recordFromModelSpace(space: ModelSpace): Record<string, Model> {
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
  readonly fileStatus: string;
  readonly selectTarget: (modelDefinitionId: string, target: SelectionTarget) => void;
  readonly hoverTarget: (modelDefinitionId: string, target: SelectionTarget | null) => void;
  readonly toggleHidden: (modelDefinitionId: string, target: SelectionTarget) => void;
  readonly toggleLocked: (modelDefinitionId: string, target: SelectionTarget) => void;
  readonly selectReference: (modelDefinitionId: string, referenceId: string) => void;
  readonly hoverReference: (modelDefinitionId: string, referenceId: string | null) => void;
  readonly toggleReferenceHidden: (modelDefinitionId: string, referenceId: string) => void;
  readonly toggleReferenceLocked: (modelDefinitionId: string, referenceId: string) => void;
}

//#region 🔖CadPlayDetails
function cadPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
  return { controllerId: CAD_PLAY_CONTROLLER_ID, command, args: args as never };
}

type CadPlayReferencePatchField = "origin" | "rotation" | "scale" | "scaleUniform" | "widthWorld" | "opacity";

export type CadPlaySelectionPatchField = "typology" | "hidden" | "locked" | "name";

function cadPlaySelectionFlagInputValue(value: unknown): boolean {
  return value === true || value === "true";
}

function cadPlaySelectionAllEqual<T>(values: readonly T[]): boolean {
  if (values.length <= 1) {
    return true;
  }
  const first = values[0];
  for (let index = 1; index < values.length; index += 1) {
    if (values[index] !== first) {
      return false;
    }
  }
  return true;
}

function cadPlaySelectionFlagSelectItems(): readonly { readonly id: string; readonly label: string; readonly value: string }[] {
  return [
    { id: "false", label: "false", value: "false" },
    { id: "true", label: "true", value: "true" },
  ];
}

/** @emoji 🎯 Patches one geometry selection target on a cloned model. */
export function patchCadPlaySelectionTarget(
  model: Model,
  target: SelectionTarget,
  field: CadPlaySelectionPatchField,
  value: unknown,
): Model | null {
  if (target.editable === false && field === "typology") {
    return null;
  }
  const next = Model.fromJSON(model.toJSON());
  if (field === "typology") {
    if (target.kind !== "object") {
      return null;
    }
    const trimmed = typeof value === "string" ? value.trim() : "";
    if (!trimmed) {
      return null;
    }
    const object = next.objects[target.id as ObjectRef];
    if (!object) {
      return null;
    }
    next.objects[target.id as ObjectRef] = { ...object, typology: trimmed };
    return next;
  }
  if (field === "hidden" || field === "locked") {
    next.setEntityFlag(target.id, field, cadPlaySelectionFlagInputValue(value));
    return next;
  }
  if (field === "name") {
    const trimmed = typeof value === "string" ? value.trim() : "";
    if (!trimmed) {
      return null;
    }
    next.metadata.setField(target.id, "name", trimmed);
    return next;
  }
  return null;
}

function cadPlaySelectionTargetRows(
  snapshot: CadPlayChromeSnapshot,
  targets: readonly SelectionTarget[],
): readonly UiNode[] {
  const model = snapshot.modelsByDefinitionId[snapshot.activeModelDefinitionId];
  const modelDefinitionId = snapshot.activeModelDefinitionId;
  const patch = (kind: SelectionTarget["kind"], id: string, field: CadPlaySelectionPatchField) =>
    cadPlayCmd("patchCadPlaySelection", { modelDefinitionId, kind, id, field });
  const rows: UiNode[] = [];
  for (const [index, target] of targets.entries()) {
    const flags = model?.getEntityFlags(target.id) ?? {};
    const metadataName = model?.metadata.get(target.id)?.name;
    const displayName = typeof metadataName === "string" && metadataName.trim() ? metadataName : target.id;
    const typology = target.kind === "object" ? (model?.objects[target.id as ObjectRef]?.typology ?? "") : "";
    const prefix = `cad-play-details.selection.target.${index}`;
    rows.push({
      type: "field",
      id: `${prefix}.kind`,
      label: targets.length === 1 ? "Kind" : `Kind ${index + 1}`,
      child: { type: "text", value: target.kind },
    });
    rows.push({
      type: "field",
      id: `${prefix}.id`,
      label: targets.length === 1 ? "Id" : `Id ${index + 1}`,
      child: { type: "text", value: target.id },
    });
    rows.push({
      type: "field",
      id: `${prefix}.name`,
      label: "Name",
      child: {
        type: "input",
        id: `${prefix}.name.input`,
        inputKind: "text",
        value: displayName,
        commit: "blur",
        onChange: patch(target.kind, target.id, "name"),
      },
    });
    if (target.kind === "object") {
      rows.push({
        type: "field",
        id: `${prefix}.typology`,
        label: "Typology",
        child: {
          type: "input",
          id: `${prefix}.typology.input`,
          inputKind: "text",
          value: typology,
          commit: "blur",
          onChange: patch(target.kind, target.id, "typology"),
        },
      });
    }
    rows.push({
      type: "field",
      id: `${prefix}.hidden`,
      label: "Hidden",
      child: {
        type: "select",
        id: `${prefix}.hidden.select`,
        value: String(flags.hidden === true),
        items: cadPlaySelectionFlagSelectItems(),
        onChange: patch(target.kind, target.id, "hidden"),
      },
    });
    rows.push({
      type: "field",
      id: `${prefix}.locked`,
      label: "Locked",
      child: {
        type: "select",
        id: `${prefix}.locked.select`,
        value: String(flags.locked === true),
        items: cadPlaySelectionFlagSelectItems(),
        onChange: patch(target.kind, target.id, "locked"),
      },
    });
  }
  return rows;
}

/** @emoji 🖼️ Builds editable reference inspector rows matching puzzle 3d play. */
export function buildCadPlayReferenceInspectorChildren(
  reference: WorldReferenceProps,
  modelDefinitionId: string,
): readonly UiNode[] {
  const referenceId = reference.id;
  const orientation = worldReferenceOrientation(reference);
  const tilt = worldQuatToEulerDegrees(orientation);
  const scaleVec = worldReferenceScaleVec(reference.scale);
  const patch = (field: CadPlayReferencePatchField) => cadPlayCmd("patchCadPlayReference", { modelDefinitionId, referenceId, field });
  return [
    {
      type: "field",
      id: `cad-play-details.reference.${referenceId}.id`,
      label: "Id",
      child: { type: "text", value: referenceId },
    },
    {
      type: "field",
      id: `cad-play-details.reference.${referenceId}.source`,
      label: "Source",
      child: { type: "text", value: reference.source.url },
    },
    {
      type: "field",
      id: `cad-play-details.reference.${referenceId}.mediaKind`,
      label: "Media kind",
      child: { type: "text", value: reference.source.mediaKind },
    },
    ...(typeof reference.source.page === "number"
      ? [
          {
            type: "field" as const,
            id: `cad-play-details.reference.${referenceId}.page`,
            label: "Page",
            child: { type: "text" as const, value: String(reference.source.page) },
          },
        ]
      : []),
    {
      type: "field",
      id: `cad-play-details.reference.${referenceId}.origin`,
      label: "Position",
      child: {
        type: "vec3",
        id: `cad-play-details.reference.${referenceId}.origin.vec3`,
        value: reference.origin as [number, number, number],
        onChange: patch("origin"),
      },
    },
    {
      type: "field",
      id: `cad-play-details.reference.${referenceId}.tilt`,
      label: "Tilt (°)",
      child: {
        type: "vec3",
        id: `cad-play-details.reference.${referenceId}.tilt.vec3`,
        value: tilt as [number, number, number],
        onChange: patch("rotation"),
      },
    },
    {
      type: "keyValue",
      entries: [
        { label: "Quaternion X", value: formatNumber(orientation[0]) },
        { label: "Quaternion Y", value: formatNumber(orientation[1]) },
        { label: "Quaternion Z", value: formatNumber(orientation[2]) },
        { label: "Quaternion W", value: formatNumber(orientation[3]) },
      ],
    },
    {
      type: "field",
      id: `cad-play-details.reference.${referenceId}.scale`,
      label: "Scale (X, Y, Z)",
      child: {
        type: "vec3",
        id: `cad-play-details.reference.${referenceId}.scale.vec3`,
        value: scaleVec,
        onChange: patch("scale"),
      },
    },
    {
      type: "field",
      id: `cad-play-details.reference.${referenceId}.scaleUniform`,
      label: "Scale factor",
      child: {
        type: "input",
        id: `cad-play-details.reference.${referenceId}.scaleUniform.input`,
        inputKind: "number",
        value: String(typeof reference.scale === "number" ? reference.scale : scaleVec[0]),
        onChange: patch("scaleUniform"),
      },
    },
    {
      type: "field",
      id: `cad-play-details.reference.${referenceId}.widthWorld`,
      label: "Width (world)",
      child: {
        type: "input",
        id: `cad-play-details.reference.${referenceId}.widthWorld.input`,
        inputKind: "number",
        value: String(reference.widthWorld ?? 10),
        onChange: patch("widthWorld"),
      },
    },
    {
      type: "field",
      id: `cad-play-details.reference.${referenceId}.opacity`,
      label: "Opacity",
      child: {
        type: "input",
        id: `cad-play-details.reference.${referenceId}.opacity.input`,
        inputKind: "number",
        value: String(reference.opacity ?? 1),
        onChange: patch("opacity"),
      },
    },
    {
      type: "keyValue",
      entries: [
        { label: "Hidden", value: String(reference.hidden === true) },
        { label: "Locked", value: String(reference.locked === true) },
      ],
    },
  ];
}

export function buildCadPlayCatalogTree(snapshot: CadPlayChromeSnapshot | null): UiTreeNode {
  if (!snapshot) {
    return uiDeclarativeSectionsToTree([{ type: "section", id: "cad-play-catalog.loading", label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, children: [{ type: "text", value: "…" }] }]);
  }
  const children: UiNode[] = [];
  if (!isShapeModelDefinition(snapshot.activeModelDefinitionId)) {
    children.push({
      type: "text",
      value: `Shape fixtures apply to ${defaultModelDefinitionId()}. Use the navbar fixture menu or focus the Shape pane.`,
    });
  }
  if (snapshot.fileStatus) {
    children.push({ type: "text", value: snapshot.fileStatus });
  }
  if (children.length === 0) {
    children.push({ type: "text", value: "Use the navbar fixture menu or toolbar save/load." });
  }
  return uiDeclarativeSectionsToTree([{ type: "section", id: "cad-play-catalog.section", label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, children }]);
}

export function buildCadPlayDetailsTree(snapshot: CadPlayChromeSnapshot | null): UiTreeNode {
  if (!snapshot) {
    return uiDeclarativeSectionsToTree([{ type: "section", id: "cad-play-details.loading", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "…" }] }]);
  }
  const sections: UiNode[] = [];
  const selectedReference = snapshot.selectedReference;
  if (selectedReference) {
    const reference = (snapshot.referencesByModelDefinitionId[selectedReference.modelDefinitionId] ?? []).find((row) => row.id === selectedReference.id);
    if (reference && worldEntitySelectable(reference)) {
      sections.push({
        type: "section",
        id: "cad-play-details.reference",
        label: reference.id,
        children: [...buildCadPlayReferenceInspectorChildren(reference, selectedReference.modelDefinitionId)],
      });
    }
  }
  if (snapshot.selection.length > 0) {
    sections.push({
      type: "section",
      id: "cad-play-details.selection",
      label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      children: [...buildCadPlaySelectionInspectorChildren(snapshot)],
    });
  }
  if (sections.length === 0) {
    return uiDeclarativeSectionsToTree([
      {
        type: "section",
        id: "cad-play-details.empty",
        label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
        children: [{ type: "text", value: "Select a primitive, object, or reference in the canvas or workbench hierarchy to edit details." }],
      },
    ]);
  }
  return uiDeclarativeSectionsToTree(sections);
}

/** @emoji 🎯 Builds editable selection rows for the CAD play inspection tree. */
export function buildCadPlaySelectionInspectorChildren(snapshot: CadPlayChromeSnapshot): readonly UiNode[] {
  const editableTargets = snapshot.selection.filter((target) => target.editable !== false);
  const children: UiNode[] = [
    {
      type: "field",
      id: "cad-play-details.selection.modelDefinition",
      label: "Model definition",
      child: { type: "text", value: snapshot.activeModelDefinitionId },
    },
    {
      type: "field",
      id: "cad-play-details.selection.count",
      label: "Selected targets",
      child: { type: "text", value: String(snapshot.selection.length) },
    },
  ];
  if (editableTargets.length === 0) {
    if (snapshot.selection.length > 0) {
      children.push({ type: "text", value: "Selected targets are locked and cannot be edited here." });
    }
    return children;
  }
  if (editableTargets.length === 1) {
    children.push(...cadPlaySelectionTargetRows(snapshot, editableTargets));
    return children;
  }
  const model = snapshot.modelsByDefinitionId[snapshot.activeModelDefinitionId];
  const typologies = editableTargets
    .filter((target) => target.kind === "object")
    .map((target) => model?.objects[target.id as ObjectRef]?.typology ?? "");
  const hiddens = editableTargets.map((target) => model?.getEntityFlags(target.id).hidden === true);
  const lockeds = editableTargets.map((target) => model?.getEntityFlags(target.id).locked === true);
  const names = editableTargets.map((target) => {
    const metadataName = model?.metadata.get(target.id)?.name;
    return typeof metadataName === "string" && metadataName.trim() ? metadataName : target.id;
  });
  const modelDefinitionId = snapshot.activeModelDefinitionId;
  const patchAll = (field: CadPlaySelectionPatchField) =>
    cadPlayCmd("patchCadPlaySelection", {
      modelDefinitionId,
      targets: editableTargets.map((target) => ({ kind: target.kind, id: target.id })),
      field,
    });
  if (typologies.length > 0) {
    const typologyUniform = cadPlaySelectionAllEqual(typologies);
    children.push({
      type: "field",
      id: "cad-play-details.selection.typology",
      label: "Typology",
      child: {
        type: "input",
        id: "cad-play-details.selection.typology.input",
        inputKind: "text",
        value: typologyUniform ? (typologies[0] ?? "") : "",
        placeholder: typologyUniform ? undefined : "Mixed",
        commit: "blur",
        onChange: patchAll("typology"),
      },
    });
  }
  children.push({
    type: "field",
    id: "cad-play-details.selection.name",
    label: "Name",
    child: {
      type: "input",
      id: "cad-play-details.selection.name.input",
      inputKind: "text",
      value: cadPlaySelectionAllEqual(names) ? (names[0] ?? "") : "",
      placeholder: cadPlaySelectionAllEqual(names) ? undefined : "Mixed",
      commit: "blur",
      onChange: patchAll("name"),
    },
  });
  children.push(
    {
      type: "field",
      id: "cad-play-details.selection.hidden",
      label: "Hidden",
      child: {
        type: "select",
        id: "cad-play-details.selection.hidden.select",
        value: cadPlaySelectionAllEqual(hiddens) ? String(hiddens[0] === true) : "",
        placeholder: cadPlaySelectionAllEqual(hiddens) ? undefined : "Mixed",
        items: cadPlaySelectionFlagSelectItems(),
        onChange: patchAll("hidden"),
      },
    },
    {
      type: "field",
      id: "cad-play-details.selection.locked",
      label: "Locked",
      child: {
        type: "select",
        id: "cad-play-details.selection.locked.select",
        value: cadPlaySelectionAllEqual(lockeds) ? String(lockeds[0] === true) : "",
        placeholder: cadPlaySelectionAllEqual(lockeds) ? undefined : "Mixed",
        items: cadPlaySelectionFlagSelectItems(),
        onChange: patchAll("locked"),
      },
    },
  );
  for (const [index, target] of editableTargets.entries()) {
    children.push({
      type: "section",
      id: `cad-play-details.selection.target-section.${index}`,
      label: `${target.kind} · ${target.id}`,
      children: [...cadPlaySelectionTargetRows(snapshot, [target])],
    });
  }
  return children;
}

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for cad. */
export function buildCadProgramDefinition(): PlatformDefinition {
	return {
		id: "cad",
		name: "CAD",
		apiVersion: "1",
		apps: [{ id: "cad", label: "CAD", controllerId: CAD_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension


//#region 🧪Tests
import geometryConcreteForestLeft from "../../../asset/play/hexagonal-cut-concrete-forest-left.model.json";
import geometryConcreteForestRight from "../../../asset/play/hexagonal-cut-concrete-forest-right.model.json";

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
      const { preciseSpatialKernelMath: M } = await import("@semio-tech/cad-js-kernel-brepjs");
      const { applyModelDiff, solidRef } = await import("@semio-tech/cad-js-core");
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
      const view = tools.find((node) => node.id === "view" && node.kind === "collection");
      const save = tools.find((node) => node.id === "save" && node.kind === "collection");
      expect(view?.kind === "collection" ? view.children.length : 0).toBeGreaterThan(0);
      expect(save?.kind === "collection" ? save.children.map((row) => row.id) : []).toEqual([
        "cad.play.save.selected",
        "cad.play.save.modelspace",
        "cad.play.save.current",
        "cad.play.save.load",
      ]);
      const firstSave = save?.kind === "collection" ? save.children[0] : undefined;
      expect(firstSave?.kind === "button" ? firstSave.disabled : undefined).toBe(true);
      expect(tools.some((node) => node.id === "transfer")).toBe(false);
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
        schema: "spatial.model",
        revision: 0,
        objects: [],
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

    it("buildCadPlayHierarchyPendingSections declares an item for initial mount", () => {
      const sections = buildCadPlayHierarchyPendingSections();
      expect(sections).toHaveLength(1);
      expect(sections[0]?.items?.length).toBeGreaterThan(0);
    });

    it("buildCadPlayHierarchySections lists objects after box commit object binding", async () => {
      const { BrepjsKernel } = await import("@semio-tech/cad-js-kernel-brepjs");
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
      const { preciseSpatialKernelMath: M } = await import("@semio-tech/cad-js-kernel-brepjs");
      const { applyModelDiff, solidRef } = await import("@semio-tech/cad-js-core");
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
      const { preciseSpatialKernelMath: M } = await import("@semio-tech/cad-js-kernel-brepjs");
      const { applyModelDiff, solidRef } = await import("@semio-tech/cad-js-core");
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

    it("cadPlayReferencesForFixture loads concrete forest reference in every model definition", () => {
      expect(cadPlayIsConcreteForestFixture("concrete-forest-left")).toBe(true);
      expect(cadPlayIsConcreteForestFixture("concrete-forest-right")).toBe(true);
      expect(cadPlayIsConcreteForestFixture("unknown-fixture")).toBe(false);
      expect(cadPlayReferencesForFixture("unknown-fixture")).toEqual({});
      const references = cadPlayReferencesForFixture("concrete-forest-left");
      for (const row of CAD_PLAY_PANE_SPECS) {
        expect(references[row.modelDefinitionId]?.map((reference) => reference.id)).toEqual(["ref-concrete-forest"]);
      }
      expect(cadPlayDefaultReferencesByModelDefinitionId()).toEqual({});
    });

    it("buildCadPlayHierarchySections lists references under model definitions with hide and lock chrome", () => {
      const references = cadPlayReferencesForFixture("concrete-forest-left");
      const build = buildCadPlayHierarchySections({ [defaultModelDefinitionId()]: new Model() }, defaultModelDefinitionId(), [], () => {}, () => {}, () => {}, () => {}, references);
      const modelBranch = build.sections.find((row) => row.id === `cad-play-hierarchy.model.${defaultModelDefinitionId()}`);
      const referencesGroup = modelBranch?.items?.find((row) => row.id === `cad-play-hierarchy.references.${defaultModelDefinitionId()}`);
      expect(referencesGroup?.items?.some((row) => row.id === `cad-play-hierarchy.reference.${defaultModelDefinitionId()}.ref-concrete-forest`)).toBe(true);
      const referenceRow = referencesGroup?.items?.find((row) => row.id === `cad-play-hierarchy.reference.${defaultModelDefinitionId()}.ref-concrete-forest`);
      expect(referenceRow?.actions?.some((row) => row.id === "reference.hidden")).toBe(true);
      expect(referenceRow?.actions?.some((row) => row.id === "reference.locked")).toBe(true);
      expect(build.highlightKeyToItemIds[cadPlayReferenceHoverKey("ref-concrete-forest")]).toContain(`cad-play-hierarchy.reference.${defaultModelDefinitionId()}.ref-concrete-forest`);
    });

    it("buildCadPlayReferenceInspectorChildren exposes editable reference transform fields", () => {
      const reference: WorldReferenceProps = {
        id: "ref-a",
        source: { url: "/plan.png", mediaKind: "image" },
        origin: [1, 2, 3],
        orientation: [0, 0, 0, 1],
        scale: [2, 3, 4],
        widthWorld: 42,
        opacity: 0.8,
      };
      const children = buildCadPlayReferenceInspectorChildren(reference, defaultModelDefinitionId());
      const positionField = children.find((row) => row.type === "field" && row.id === "cad-play-details.reference.ref-a.origin");
      const tiltField = children.find((row) => row.type === "field" && row.id === "cad-play-details.reference.ref-a.tilt");
      const scaleField = children.find((row) => row.type === "field" && row.id === "cad-play-details.reference.ref-a.scale");
      const widthField = children.find((row) => row.type === "field" && row.id === "cad-play-details.reference.ref-a.widthWorld");
      expect(positionField?.type).toBe("field");
      expect(positionField?.child.type).toBe("vec3");
      expect(positionField?.child.value).toEqual([1, 2, 3]);
      expect(tiltField?.child.type).toBe("vec3");
      expect(scaleField?.child.type).toBe("vec3");
      expect(scaleField?.child.value).toEqual([2, 3, 4]);
      expect(widthField?.child.type).toBe("input");
      expect(widthField?.child.value).toBe("42");
      expect(positionField?.child.onChange?.command).toBe("patchCadPlayReference");
    });

    it("buildCadPlayDetailsTree renders reference section for selected reference", () => {
      const modelDefinitionId = defaultModelDefinitionId();
      const reference: WorldReferenceProps = {
        id: "ref-a",
        source: { url: "/plan.png", mediaKind: "image" },
        origin: [1, 2, 3],
        scale: 2,
        widthWorld: 42,
      };
      const snapshot: CadPlayChromeSnapshot = {
        modelsByDefinitionId: { [modelDefinitionId]: new Model() },
        referencesByModelDefinitionId: { [modelDefinitionId]: [reference] },
        activeModelDefinitionId: modelDefinitionId,
        selection: [],
        selectedReference: { modelDefinitionId, id: "ref-a" },
        hoveredKey: null,
        fileStatus: "",
        selectTarget: () => {},
        hoverTarget: () => {},
        toggleHidden: () => {},
        toggleLocked: () => {},
        selectReference: () => {},
        hoverReference: () => {},
        toggleReferenceHidden: () => {},
        toggleReferenceLocked: () => {},
      };
      const tree = buildCadPlayDetailsTree(snapshot);
      const referenceSection = tree.sections.find((section) => section.label === "ref-a");
      expect(referenceSection).toBeDefined();
      const positionField = referenceSection!.items.find((item) => item.id === "cad-play-details.reference.ref-a.origin");
      expect(positionField?.control?.type).toBe("vec3");
      expect((positionField?.control as { value?: readonly number[] } | undefined)?.value).toEqual([1, 2, 3]);
    });

    it("buildCadPlayDetailsTree renders selection section for canvas targets", () => {
      const modelDefinitionId = defaultModelDefinitionId();
      const snapshot: CadPlayChromeSnapshot = {
        modelsByDefinitionId: { [modelDefinitionId]: new Model() },
        referencesByModelDefinitionId: {},
        activeModelDefinitionId: modelDefinitionId,
        selection: [{ kind: "object", id: "wall-a", editable: true }],
        selectedReference: null,
        hoveredKey: null,
        fileStatus: "",
        selectTarget: () => {},
        hoverTarget: () => {},
        toggleHidden: () => {},
        toggleLocked: () => {},
        selectReference: () => {},
        hoverReference: () => {},
        toggleReferenceHidden: () => {},
        toggleReferenceLocked: () => {},
      };
      const tree = buildCadPlayDetailsTree(snapshot);
      const selectionSection = tree.sections.find((section) => section.id === "cad-play-details.selection");
      expect(selectionSection).toBeDefined();
      const targetField = selectionSection!.items.find((item) => item.id === "cad-play-details.selection.target.0.kind");
      expect(targetField?.label).toBe("Kind");
      const typologyField = selectionSection!.items.find((item) => item.id === "cad-play-details.selection.target.0.typology");
      expect(typologyField?.control?.type).toBe("input");
      expect(typologyField?.control?.onChange?.command).toBe("patchCadPlaySelection");
    });

    it("buildCadPlaySelectionInspectorChildren batches shared name field for multi-select", () => {
      const modelDefinitionId = defaultModelDefinitionId();
      const model = new Model();
      model.metadata.setField("wall-a", "name", "Wall A");
      model.metadata.setField("wall-b", "name", "Wall B");
      const snapshot: CadPlayChromeSnapshot = {
        modelsByDefinitionId: { [modelDefinitionId]: model },
        referencesByModelDefinitionId: {},
        activeModelDefinitionId: modelDefinitionId,
        selection: [
          { kind: "object", id: "wall-a", editable: true },
          { kind: "object", id: "wall-b", editable: true },
        ],
        selectedReference: null,
        hoveredKey: null,
        fileStatus: "",
        selectTarget: () => {},
        hoverTarget: () => {},
        toggleHidden: () => {},
        toggleLocked: () => {},
        selectReference: () => {},
        hoverReference: () => {},
        toggleReferenceHidden: () => {},
        toggleReferenceLocked: () => {},
      };
      const rows = buildCadPlaySelectionInspectorChildren(snapshot);
      const nameField = rows.find((row) => row.id === "cad-play-details.selection.name") as { child?: { type?: string; placeholder?: string } } | undefined;
      expect(nameField?.child?.type).toBe("input");
      expect(nameField?.child?.placeholder).toBe("Mixed");
    });

    it("patchCadPlaySelectionTarget stores display name in metadata", () => {
      const model = new Model();
      const patched = patchCadPlaySelectionTarget(model, { kind: "object", id: "wall-a", editable: true }, "name", "Renamed");
      expect(patched?.metadata.get("wall-a")?.name).toBe("Renamed");
    });

    it("updateCadPlayReferenceInMap toggles reference flags", () => {
      const initial = cadPlayReferencesForFixture("concrete-forest-left");
      const next = updateCadPlayReferenceInMap(initial, defaultModelDefinitionId(), "ref-concrete-forest", { hidden: true, locked: true });
      expect(next[defaultModelDefinitionId()]?.find((row) => row.id === "ref-concrete-forest")).toEqual(expect.objectContaining({ hidden: true, locked: true }));
    });

    it("buildCadPlayHierarchySections does not mark locked references selected", () => {
      const references = {
        [defaultModelDefinitionId()]: [{ ...CAD_PLAY_CONCRETE_FOREST_WORLD_REFERENCES[0]!, locked: true }],
      };
      const build = buildCadPlayHierarchySections(
        { [defaultModelDefinitionId()]: new Model() },
        defaultModelDefinitionId(),
        [],
        () => {},
        () => {},
        () => {},
        () => {},
        references,
        { modelDefinitionId: defaultModelDefinitionId(), id: "ref-concrete-forest" },
      );
      const referencesGroup = build.sections[0]?.items?.find((row) => row.id === `cad-play-hierarchy.references.${defaultModelDefinitionId()}`);
      const referenceRow = referencesGroup?.items?.find((row) => row.id === `cad-play-hierarchy.reference.${defaultModelDefinitionId()}.ref-concrete-forest`);
      expect(referenceRow?.isSelected).not.toBe(true);
    });

    it("buildCadPlayHierarchySections lists kernel-imported STEP objects", async () => {
      const { preciseSpatialKernelMath: M } = await import("@semio-tech/cad-js-kernel-brepjs");
      const { applyModelDiff, defaultModelDefinitionId, solidRef } = await import("@semio-tech/cad-js-core");
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
      const models = modelsFromCadJson(geometryConcreteForestRight);
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
      expect(buildingBranch?.items?.length).toBe(13);
      const energyBranch = build.sections.find((row) => row.id === `cad-play-hierarchy.model.${CAD_PLAY_ENERGY_MODEL_DEFINITION_ID}`);
      expect(energyBranch?.items?.length).toBe(2);
      const structureBranch = build.sections.find(
        (row) => row.id === `cad-play-hierarchy.model.${CAD_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID}`,
      );
      expect(structureBranch?.items?.length).toBe(12);
      expect(build.sections.some((row) => row.id === `cad-play-hierarchy.model.${defaultModelDefinitionId()}`)).toBe(true);
    });
  });
}
//#endregion 🧪Tests

//#region 🔖Play
import { CAD_PLAY_SHAPE_ASSET_IDS, resolveCadPlayExampleSlug } from "./example-slugs.ts";

export { CAD_PLAY_SHAPE_ASSET_IDS, resolveCadPlayExampleSlug } from "./example-slugs.ts";

/** @emoji 🧩 Shape fixture assets for CAD play navbar. */
export const CAD_PLAY_SHAPE_ASSETS = [
	{ id: "concrete-forest-left", key: "c", label: "Concrete forest (left)", json: { modelSpace: geometryConcreteForestLeft, activeModelDefinitionId: defaultModelDefinitionId() } as Record<string, unknown> },
	{ id: "concrete-forest-right", key: "d", label: "Concrete forest (right)", json: geometryConcreteForestRight as Record<string, unknown> },
] as const;

//#region 🔖MediaExport
function cadExportModelSpace(doc: unknown): ModelSpace {
  if (doc instanceof ModelSpace) return doc;
  const parsed = parseModelSpaceJson(doc);
  if (parsed) return parsed;
  const models = modelsFromCadJson(doc);
  return modelSpaceFromRecord(models);
}

/** @emoji 💾 Registers CAD model space OBJ/GLB export handlers for the OS media graph. */
export function registerCadMediaExportHandlers(): void {
  registerOsMediaExportHandler("3d.cad", "obj", async (doc) => ({
    data: await exportModelSpaceToObj(cadExportModelSpace(doc)),
    mimeType: "text/plain",
    fileName: "cad.obj",
  }));
  registerOsMediaExportHandler("3d.cad", "glb", async (doc) => ({
    data: await exportModelSpaceToGlb(cadExportModelSpace(doc)),
    mimeType: "model/gltf-binary",
    fileName: "cad.glb",
  }));
}
//#endregion 🔖MediaExport

/** @emoji 🛝 CAD playground app. */


export const cadPlayAppDefinition = createPlaygroundApp({
	id: CAD_PLAY_APP_ID,
	label: "CAD",
	controllerId: CAD_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "cad",
		resolveDedupe: ["react", "react-dom", "three"],
		optimizeDeps: {
			include: [
				"react",
				"react-dom",
				"react/jsx-runtime",
				"react/jsx-dev-runtime",
				"three",
				"@react-three/fiber",
				"@react-three/drei",
				"@semio-tech/infinite-world-r3f",
				"brepjs",
				"brepjs-opencascade",
				"golden-layout",
				"lucide-react",
				"chevrotain",
			],
		},
	},
	createRuntime: () => {
		const runtime = createProductPlaygroundPlatform(CAD_PLAY_APP_ID);
			registerCadPlayDeclarativeBodies();
			const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
			runtime.addApp(buildCadPlayAppRuntime(controller));
			return runtime;
	},
	registerBodies: () => {
		registerCadPlayDeclarativeBodies();
	},
	bootRenderer: async (pg) => {
		const { bootCadPlay } = await import("@semio-tech/cad-js-renderer-react/play");
		bootCadPlay(pg);
	},
});
//#endregion 🔖Play
