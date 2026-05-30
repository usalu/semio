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
  createWindowLayout,
  registerWindowBody,
  type AppTools,
  type ToolItem,
  type WindowBodyViewContext,
  type WindowEngagement,
  type WindowMeasure,
  type UiNode,
  type WindowLayout,
} from "@framework/playground/core";
import {
  DocumentHistory,
  SHAPE_MODEL_DEFINITION_ID,
  applyTransformation,
  buildModelTopologyHierarchy,
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
  type ModelTopologyHierarchyNode,
  type SelectionTarget,
  type SpatialComputeMode,
  type TransformationSpec,
} from "@cad/js/core";

/** @emoji ⚡ Per-window compute mode options for CAD play window measures. */
export const CAD_PLAY_COMPUTE_MODES: readonly SpatialComputeMode[] = ["fast", "precise"];

//#region 🔖Ids
export const CAD_PLAY_APP_ID = "cad-play";
export const CAD_PLAY_CONTROLLER_ID = "cad-play";
export const CAD_PLAY_HIERARCHY_TAB_ID = "cad-play-hierarchy";

/** @emoji 🖱️ Hover owner id when the workbench hierarchy drives shared pointer focus. */
export const CAD_PLAY_HOVER_SOURCE_HIERARCHY = "cad-play-hierarchy";

/** @emoji 🖱️ Hover owner id when the 3D canvas drives shared pointer focus. */
export const CAD_PLAY_HOVER_SOURCE_CANVAS = "cad-play-canvas";

export const CAD_PLAY_BUILDING_MODEL_DEFINITION_ID = "aec.building";
export const CAD_PLAY_ENERGY_MODEL_DEFINITION_ID = "aec.building.energy";
export const CAD_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID = "aec.building.structure.classic";

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
    modelDefinitionId: SHAPE_MODEL_DEFINITION_ID,
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

/** @emoji 🔢 Digest for hierarchy chrome when {@link Model} instances mutate in place (revision, objects, topology counts). */
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
  readonly isSelected: (kind: SelectionTarget["kind"], id: string) => boolean;
  readonly isHighlighted: (kind: SelectionTarget["kind"], id: string) => boolean;
  readonly onSelect: (modelDefinitionId: string, target: SelectionTarget) => void;
  readonly onHover: (modelDefinitionId: string, target: SelectionTarget | null) => void;
};

function cadPlayHierarchyHoverHandlers(ctx: CadPlayHierarchyPickContext, target: SelectionTarget): Pick<TreeDataItem, "onPointerEnter" | "onPointerLeave"> {
  return {
    onPointerEnter: () => ctx.onHover(ctx.modelDefinitionId, target),
    onPointerLeave: () => ctx.onHover(ctx.modelDefinitionId, null),
  };
}

function cadPlayTopologyTreeItem(node: ModelTopologyHierarchyNode, path: string, ctx: CadPlayHierarchyPickContext): TreeDataItem {
  const childItems = node.children.map((child) => cadPlayTopologyTreeItem(child, `${path}.${child.kind}.${child.id}`, ctx));
  const target: SelectionTarget = { kind: node.kind, id: node.id, editable: true };
  return {
    id: `cad-play-hierarchy.topology.${path}`,
    label: `${node.kind} ${node.id}`,
    isSelected: ctx.isSelected(node.kind, node.id),
    isHighlighted: ctx.isHighlighted(node.kind, node.id),
    defaultOpen: node.kind === "solid" || node.kind === "shell" || node.kind === "face",
    onClick: () => ctx.onSelect(ctx.modelDefinitionId, target),
    ...cadPlayHierarchyHoverHandlers(ctx, target),
    ...(childItems.length > 0 ? { items: childItems } : {}),
  };
}

function cadPlayPrimitiveSlotTreeItems(model: Model, modelDefinitionId: string, objectId: string, slot: string, primitiveRef: string, ctx: CadPlayHierarchyPickContext): TreeDataItem {
  const kind = resolvePrimitiveRefKind(model, primitiveRef) ?? "solid";
  const primitiveId = String(primitiveRef);
  const topology = buildModelTopologyHierarchy(model, primitiveId);
  const topologyItems = (topology?.children ?? []).map((child) => cadPlayTopologyTreeItem(child, `${modelDefinitionId}.${objectId}.${slot}.${child.kind}.${child.id}`, ctx));
  const target: SelectionTarget = { kind, id: primitiveId, editable: true };
  return {
    id: `cad-play-hierarchy.primitive.${modelDefinitionId}.${objectId}.${slot}`,
    label: `${slot}: ${kind} ${primitiveId}`,
    isSelected: ctx.isSelected(kind, primitiveId),
    isHighlighted: ctx.isHighlighted(kind, primitiveId),
    defaultOpen: true,
    onClick: () => ctx.onSelect(ctx.modelDefinitionId, target),
    ...cadPlayHierarchyHoverHandlers(ctx, target),
    items: topologyItems.length ? topologyItems : [{ id: `cad-play-hierarchy.primitive.${modelDefinitionId}.${objectId}.${slot}.topology.empty`, label: "(empty)" }],
  };
}

/** @emoji 🌳 ModelSpace → model definition → object → primitive slot tree for CAD play workbench. */
export function buildCadPlayHierarchySections(
  modelsByDefinitionId: Record<string, Model>,
  activeModelDefinitionId: string,
  selection: readonly SelectionTarget[],
  onSelect: (modelDefinitionId: string, target: SelectionTarget) => void,
  hoveredKey: string | null = null,
  onHover: (modelDefinitionId: string, target: SelectionTarget | null) => void = () => {},
): TreeDataSection[] {
  const selectedKeys = new Set(selection.map(cadPlaySelectionKey));
  const isSelected = (kind: SelectionTarget["kind"], id: string): boolean => selectedKeys.has(`${kind}:${id}`);
  const isHighlighted = (kind: SelectionTarget["kind"], id: string): boolean => hoveredKey === `${kind}:${id}`;
  const modelDefinitionIds = Object.keys(modelsByDefinitionId).sort((a, b) => a.localeCompare(b));
  const modelBranches: TreeDataItem[] = [];
  for (const modelDefinitionId of modelDefinitionIds) {
    const model = modelsByDefinitionId[modelDefinitionId];
    if (!model) {
      continue;
    }
    const pickCtx: CadPlayHierarchyPickContext = { modelDefinitionId, isSelected, isHighlighted, onSelect, onHover };
    const objectItems: TreeDataItem[] = listModelObjectsForModelDefinition(model, modelDefinitionId).map((object) => {
      const objectId = String(object.id);
      const typologyTail = object.typology.split(".").pop() ?? object.typology;
      const primitiveItems: TreeDataItem[] = objectPrimitiveEntries(object).map(([slot, primitiveRef]) => cadPlayPrimitiveSlotTreeItems(model, modelDefinitionId, objectId, slot, primitiveRef, pickCtx));
      const objectTarget: SelectionTarget = { kind: "object", id: objectId, editable: true };
      return {
        id: `cad-play-hierarchy.object.${modelDefinitionId}.${objectId}`,
        label: `${typologyObjectPascalFromLabel(typologyTail.replace(/[._-]+/g, " "))} (${objectId})`,
        description: object.typology,
        isSelected: isSelected("object", objectId),
        isHighlighted: isHighlighted("object", objectId),
        defaultOpen: true,
        onClick: () => onSelect(modelDefinitionId, objectTarget),
        ...cadPlayHierarchyHoverHandlers(pickCtx, objectTarget),
        items: primitiveItems.length ? primitiveItems : [{ id: `cad-play-hierarchy.object.${modelDefinitionId}.${objectId}.primitives.empty`, label: "(none)" }],
      };
    });
    modelBranches.push({
      id: `cad-play-hierarchy.model.${modelDefinitionId}`,
      label: cadPlayModelDefinitionLabel(modelDefinitionId),
      description: modelDefinitionId,
      defaultOpen: modelDefinitionId === activeModelDefinitionId,
      items: objectItems.length ? objectItems : [{ id: `cad-play-hierarchy.model.${modelDefinitionId}.objects.empty`, label: "(no objects)" }],
    });
  }
  const modelSpaceRoot: TreeDataItem = {
    id: "cad-play-hierarchy.modelspace",
    label: "ModelSpace",
    defaultOpen: true,
    items: modelBranches.length ? modelBranches : [{ id: "cad-play-hierarchy.modelspace.empty", label: "(empty)" }],
  };
  return [{ id: "cad-play-hierarchy.root", defaultOpen: true, items: [modelSpaceRoot] }];
}
//#endregion 🔖CadPlayHierarchy

//#region 🔖Toolbar
/** @emoji 🧰 Snapshot for {@link buildCadPlayToolbarTools}. */
export interface CadPlayToolbarState {
  readonly activeModelDefinitionId: string;
  readonly selectionCount: number;
  readonly transformsTo: readonly TransformationSpec[];
  readonly transformsFrom: readonly TransformationSpec[];
}

/** @emoji 🔗 React host bridge for CAD play toolbar commands. */
export interface CadPlayHostBridge {
  getToolbarState(): CadPlayToolbarState;
  runHostCommand(command: string, args?: unknown): void;
}

/** @emoji 🧰 Playground {@link AppTools} for CAD play (view, save, transform). */
export function buildCadPlayToolbarTools(state: CadPlayToolbarState, controllerId: string): AppTools {
  const viewTools: ToolItem[] = listModelDefinitionManifests().map((row, index) => ({
    id: `cad.play.view.${row.id}`,
    kind: "toggle",
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
      label: "Selected",
      order: 0,
      disabled: state.selectionCount === 0,
      controllerId,
      command: "saveSelected",
    },
    {
      id: "cad.play.save.modelspace",
      kind: "button",
      label: "Model space",
      order: 1,
      controllerId,
      command: "saveInPlay",
    },
    {
      id: "cad.play.save.current",
      kind: "button",
      label: "Current",
      order: 2,
      controllerId,
      command: "saveCurrent",
    },
    {
      id: "cad.play.save.load",
      kind: "button",
      label: "Load",
      order: 3,
      controllerId,
      command: "loadRawRequest",
    },
  ];
  const transformTools: ToolItem[] = [
    ...state.transformsTo.map((spec, index) => ({
      id: `cad.play.transform.to.${qualifiedTransformationId(spec.modelDefinitionId, spec.id)}`,
      kind: "button" as const,
      label: `→ ${spec.label}`,
      title: spec.target.modelDefinition,
      order: index,
      controllerId,
      command: "applyTransformation",
      args: { qid: qualifiedTransformationId(spec.modelDefinitionId, spec.id) },
    })),
    ...(state.transformsTo.length > 0 && state.transformsFrom.length > 0 ? [{ id: "cad.play.transform.separator", kind: "separator" as const, order: state.transformsTo.length }] : []),
    ...state.transformsFrom.map((spec, index) => ({
      id: `cad.play.transform.from.${qualifiedTransformationId(spec.modelDefinitionId, spec.id)}`,
      kind: "button" as const,
      label: `← ${spec.label}`,
      title: spec.source.modelDefinition,
      order: state.transformsTo.length + (state.transformsTo.length > 0 && state.transformsFrom.length > 0 ? 1 : 0) + index,
      controllerId,
      command: "applyTransformation",
      args: { qid: qualifiedTransformationId(spec.modelDefinitionId, spec.id) },
    })),
  ];
  return {
    view: viewTools,
    save: saveTools,
    ...(transformTools.length > 0 ? { transform: transformTools } : {}),
  };
}

/** @emoji 🔑 Stable digest for {@link WindowEngagement} equality (skips redundant shell updates). */
export function windowEngagementDigest(engagement: WindowEngagement | undefined): string {
  if (!engagement) return "";
  const options = (engagement.options ?? []).map((row) => `${row.id}\u0001${row.label}\u0001${row.pressed ? 1 : 0}\u0001${row.disabled ? 1 : 0}`).join("\u0002");
  const input = engagement.input ? `${engagement.input.id}\u0001${engagement.input.value}\u0001${engagement.input.placeholder ?? ""}\u0001${engagement.input.disabled ? 1 : 0}` : "";
  const status = (engagement.status ?? []).map((row) => `${row.id}\u0001${row.text}`).join("\u0002");
  const possibles = (engagement.possibleEngagements ?? []).map((row) => `${row.id}\u0001${row.label}\u0001${row.detail ?? ""}`).join("\u0002");
  return [options, input, status, possibles].join("\u0003");
}

/** @emoji ⚖️ Returns whether two neutral engagement snapshots are equivalent for shell sync. */
export function windowEngagementsEqual(left: WindowEngagement | undefined, right: WindowEngagement | undefined): boolean {
  return windowEngagementDigest(left) === windowEngagementDigest(right);
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
      }
    : undefined;
  const status = engagement.status?.map((row) => ({ id: row.id, text: typeof row.content === "string" ? row.content : String(row.content) }));
  const possibleEngagements = engagement.possibleEngagements?.map((row) => ({
    id: row.id,
    label: row.label,
    detail: row.detail,
    command: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementPossibleSelect", args: { pane, possibleId: row.id } },
  }));
  return { options, input, status, possibleEngagements };
}
//#endregion 🔖Toolbar

//#region 🔖Controller
/** @emoji 🎛 CAD play shell controller: quad viewports + playground toolbar categories. */
export class CadPlayShellController extends Controller {
  readonly mainMode = new ModeRuntime("main", "CAD", undefined);
  private hostBridge: CadPlayHostBridge | null = null;
  private computeModeByPane: Record<CadPlayPaneId, SpatialComputeMode>;
  private engagementByPane: Record<CadPlayPaneId, WindowEngagement | undefined>;

  constructor(commandBus: CommandBus, hostNotify: () => void) {
    super(CAD_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.computeModeByPane = {
      shape: "fast",
      building: "fast",
      energy: "fast",
      "structure-classic": "fast",
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

  /** @emoji 🔄 Rebuilds quad window kinds with per-pane compute measures and live interaction engagement per pane. */
  rebuildShellMode(): void {
    this.mainMode.windowKinds = CAD_PLAY_PANE_SPECS.map(
      (row) => new WindowKindRuntime(row.windowKindId, row.label, row.bodyKey, undefined, [this.computeMeasureForPane(row.pane)], this.engagementByPane[row.pane]),
    );
  }

  /** @emoji 💬 Sets one pane's interaction engagement (from the live {@link InteractionRepl} snapshot) and re-renders the shell. */
  setPaneEngagement(pane: CadPlayPaneId, engagement: WindowEngagement | undefined): void {
    if (windowEngagementsEqual(this.engagementByPane[pane], engagement)) return;
    this.engagementByPane = { ...this.engagementByPane, [pane]: engagement };
    const windowKindId = CAD_PLAY_PANE_SPECS.find((row) => row.pane === pane)?.windowKindId;
    const existing = windowKindId ? this.mainMode.windowKinds.find((wk) => wk.id === windowKindId) : undefined;
    if (existing) {
      existing.engagement = engagement;
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
      case "focusModelDefinition":
      case "applyTransformation":
      case "saveSelected":
      case "saveInPlay":
      case "saveCurrent":
      case "loadRawRequest":
      case "engagementOption":
      case "engagementInput":
      case "engagementSubmit":
      case "engagementPossibleSelect":
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
  const app = new AppRuntime(CAD_PLAY_APP_ID, "CAD play", undefined, controller, CAD_PLAY_LAYOUT as never, controller.mainMode.windowKinds);
  app.defaultModeId = controller.mainMode.id;
  app.addMode(controller.mainMode);
  app.leftTabs = [];
  app.rightTabs = [];
  app.onActiveWindowChange = (windowKindId) => {
    const pane = cadPlayPaneFromWindowKindId(windowKindId);
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
  const runtime = new Platform();
  const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
  runtime.addApp(buildCadPlayAppRuntime(controller));
  return runtime;
}
//#endregion 🔖Runtime

import "./globals.css";
// #region 🔌Adapters
import { Label, reactHostPort, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, type EngagementSpec, type TreeDataItem, type TreeDataSection } from "@ui/react";
import { StrictMode, type ChangeEvent, type ReactNode } from "react";
// #endregion 🔌Adapters
import {
  PlaygroundView,
  CallbackTreePanelDefinition,
  PureSidePanelTabDefinition,
  StaticTreePanelDefinition,
  mountPlaygroundApp,
  playgroundPanelSection,
  type SidePanelTabConfig,
} from "@framework/playground/renderer/react/shell";
import { registerSurfaceBinding, type UiCadHostSurfaceNode } from "@framework/platform/renderer/react";
import { ListTree, Shapes } from "lucide-react";
import { defaultConstructRunner } from "@cad/js/query";
import geometryNakagin from "../../../assets/play/geometry.json";
import geometryLoom from "../../../assets/play/geometry-loom.json";
import geometryRoutes from "../../../assets/play/geometry-routes.json";
import geometrySmallBuilding from "../../../assets/play/small-building.model.json";
import geometryTallBuilding from "../../../assets/play/tall-building.model.json";
import geometryLargeBuilding from "../../../assets/play/large-building.model.json";
import { BrepjsKernel } from "@cad/js/kernel/brepjs";
import { statelyStateEngineProvider } from "@cad/js/machine/stately";
import {
  InteractionRepl,
  SelectionAttributesPanel,
  SelectionPropertiesPanel,
  replDisplayedSelectionTargets,
  replWithRendererSelectionTargets,
  r3fPreviewKernel,
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
  const modelSpace = parseModelSpaceJson(json);
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
  if (models[SHAPE_MODEL_DEFINITION_ID]) return { ...models };
  return { ...models, [SHAPE_MODEL_DEFINITION_ID]: new Model() };
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
  if (modelSpace) return ensurePlayShapeModel(recordFromModelSpace(modelSpace));
  return ensurePlayShapeModel({
    [SHAPE_MODEL_DEFINITION_ID]: parseModelJson(bundle?.model ?? json) ?? new Model(),
  });
}

function activeModelDefinitionIdFromSpatialJson(json: unknown): string {
  const bundle = json && typeof json === "object" ? (json as SpatialExchangeBundle) : null;
  if (typeof bundle?.activeModelDefinitionId === "string") return bundle.activeModelDefinitionId;
  const modelSpace = parseModelSpaceJson(bundle?.modelSpace ?? json);
  return Object.keys(modelSpace?.models ?? {})[0] ?? SHAPE_MODEL_DEFINITION_ID;
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

function ensureDerivedModelInSpace(models: Readonly<Record<string, Model>>, definitionId: string): Record<string, Model> {
  const withShape = ensurePlayShapeModel(models);
  if (withShape[definitionId]) return withShape;
  if (isShapeModelDefinition(definitionId)) return withShape;
  const candidates = listTransformationsIntoModelDefinition(definitionId);
  const fromShape = candidates.find((row) => isShapeModelDefinition(row.source.modelDefinition));
  const shape = withShape[SHAPE_MODEL_DEFINITION_ID];
  if (fromShape && shape) {
    return { ...withShape, [definitionId]: applyTransformation(fromShape, shape) };
  }
  const fromLinked = candidates.find((row) => withShape[row.source.modelDefinition]);
  if (fromLinked) {
    return { ...withShape, [definitionId]: applyTransformation(fromLinked, withShape[fromLinked.source.modelDefinition]!) };
  }
  return withShape;
}

/** @emoji 🌌 Ensures all four CAD play quad models exist and stay derived from shape. */
export function ensureCadPlayQuadModels(models: Readonly<Record<string, Model>>): Record<string, Model> {
  let next = ensurePlayShapeModel(models);
  if (!next[CAD_PLAY_BUILDING_MODEL_DEFINITION_ID]) {
    next = { ...next, [CAD_PLAY_BUILDING_MODEL_DEFINITION_ID]: new Model() };
  }
  next = ensureDerivedModelInSpace(next, CAD_PLAY_ENERGY_MODEL_DEFINITION_ID);
  next = ensureDerivedModelInSpace(next, "aec.building.structure");
  next = ensureDerivedModelInSpace(next, CAD_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID);
  return next;
}

function emptyPlayModels(): Record<string, Model> {
  return ensureCadPlayQuadModels({});
}

function pickShapeForModelDefinition(models: Readonly<Record<string, Model>>, activeModelDefinitionId: string, liveModel: Model): Model {
  if (isShapeModelDefinition(activeModelDefinitionId)) {
    return models[SHAPE_MODEL_DEFINITION_ID] ?? liveModel;
  }
  if (modelDefinitionUsesGeometryPicking(activeModelDefinitionId)) {
    return models[activeModelDefinitionId] ?? models[SHAPE_MODEL_DEFINITION_ID] ?? liveModel;
  }
  return liveModel;
}

//#region 🔖CadPlayChrome
export interface CadPlayChromeSnapshot {
  readonly modelsByDefinitionId: Record<string, Model>;
  readonly activeModelDefinitionId: string;
  readonly selection: readonly SelectionTarget[];
  readonly hoveredKey: string | null;
  readonly selectTarget: (modelDefinitionId: string, target: SelectionTarget) => void;
  readonly hoverTarget: (modelDefinitionId: string, target: SelectionTarget | null) => void;
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
  constructor(private readonly buildSections: () => TreeDataSection[]) {
    super();
  }

  resolveTab(): SidePanelTabConfig {
    return {
      id: CAD_PLAY_HIERARCHY_TAB_ID,
      icon: ListTree,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => this.buildSections()),
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
  const [activeModelDefinitionId, setActiveModelDefinitionId] = reactHostPort.useState(SHAPE_MODEL_DEFINITION_ID);
  const [interactionIdByPane, setInteractionIdByPane] = reactHostPort.useState(emptyInteractionIdByPane);
  const [interactionBootIdByPane, setInteractionBootIdByPane] = reactHostPort.useState(emptyInteractionBootIdByPane);
  const [shapeAssetId, setShapeAssetId] = reactHostPort.useState("");
  const [modelsByDefinitionId, setModelsByDefinitionId] = reactHostPort.useState<Record<string, Model>>(emptyPlayModels);
  const [loadedRawName, setLoadedRawName] = reactHostPort.useState("");
  const [rendererSelectionByModel, setRendererSelectionByModel] = reactHostPort.useState<SpatialRendererSelectionByModel>({});
  const [interactionSelectionByState, setInteractionSelectionByState] = reactHostPort.useState<SpatialInteractionSelectionByState>({});
  const [modelDefinitionRevision, setModelDefinitionRevision] = reactHostPort.useState(0);
  const [snapshotByPane, setSnapshotByPane] = reactHostPort.useState(emptySnapshotByPane);
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
    setModelsByDefinitionId((prev) => ensureCadPlayQuadModels(prev));
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
      setActiveModelDefinitionId(SHAPE_MODEL_DEFINITION_ID);
    } else {
      const asset = SHAPE_ASSETS.find((candidate) => candidate.id === id);
      if (!asset) return;
      setModelsByDefinitionId(modelsFromCadJson(asset.json));
      setActiveModelDefinitionId(activeModelDefinitionIdFromSpatialJson(asset.json));
    }
    setModelDefinitionRevision((r) => r + 1);
  }, []);

  const modelsForActiveDefinition = reactHostPort.useMemo(() => ensureCadPlayQuadModels(modelsByDefinitionId), [activeModelDefinitionId, modelsByDefinitionId]);

  const activeModel = reactHostPort.useMemo(() => {
    const resolved = modelsForActiveDefinition[activeModelDefinitionId];
    if (resolved) return resolved;
    if (isShapeModelDefinition(activeModelDefinitionId)) {
      return modelsForActiveDefinition[SHAPE_MODEL_DEFINITION_ID] ?? new Model();
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
    return ensureCadPlayQuadModels(flushed);
  }, [activeModelDefinitionId, liveModel, liveModel.revision, modelsByDefinitionId]);

  const playModelSpace = reactHostPort.useMemo(() => modelSpaceFromRecord(flushedModelsByDefinitionId), [flushedModelsByDefinitionId]);

  const visibleExportModel = reactHostPort.useMemo(() => flushedModelsByDefinitionId[activeModelDefinitionId] ?? liveModel, [activeModelDefinitionId, flushedModelsByDefinitionId, liveModel]);

  const pickGeometry = reactHostPort.useMemo(() => pickShapeForModelDefinition(flushedModelsByDefinitionId, activeModelDefinitionId, liveModel), [activeModelDefinitionId, flushedModelsByDefinitionId, liveModel]);

  const commitModelForDefinition = reactHostPort.useCallback((modelDefinitionId: string, model: Model) => {
    setModelsByDefinitionId((prev) => ensureCadPlayQuadModels({ ...prev, [modelDefinitionId]: Model.fromJSON(model.toJSON()) }));
    setModelDefinitionRevision((r) => r + 1);
  }, []);

  const handleActiveModelDefinitionChange = reactHostPort.useCallback(
    (nextId: string) => {
      setModelsByDefinitionId((prev) => {
        const flushed = flushModelsRecord(prev, activeModelDefinitionId, liveModel);
        return ensureCadPlayQuadModels(flushed);
      });
      setActiveModelDefinitionId(nextId);
      setModelDefinitionRevision((r) => r + 1);
    },
    [activeModelDefinitionId, liveModel],
  );

  const focusModelDefinition = reactHostPort.useCallback(
    (modelDefinitionId: string) => {
      if (modelDefinitionId === activeModelDefinitionId) return;
      setModelsByDefinitionId((prev) => ensureCadPlayQuadModels(flushModelsRecord(prev, activeModelDefinitionId, liveModel)));
      setActiveModelDefinitionId(modelDefinitionId);
    },
    [activeModelDefinitionId, liveModel],
  );

  const handleModelAttributesChange = reactHostPort.useCallback(
    (model: Model) => {
      commitModelForDefinition(activeModelDefinitionId, model);
    },
    [activeModelDefinitionId, commitModelForDefinition],
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

  const selectHierarchyTarget = reactHostPort.useCallback(
    (modelDefinitionId: string, target: SelectionTarget) => {
      if (modelDefinitionId !== activeModelDefinitionId) {
        handleActiveModelDefinitionChange(modelDefinitionId);
      }
      setRendererSelectionByModel((prev) => replWithRendererSelectionTargets(prev, modelDefinitionId, [target]));
    },
    [activeModelDefinitionId, handleActiveModelDefinitionChange],
  );

  const hoverHierarchyTarget = reactHostPort.useCallback(
    (modelDefinitionId: string, target: SelectionTarget | null) => {
      if (target && modelDefinitionId !== activeModelDefinitionId) {
        handleActiveModelDefinitionChange(modelDefinitionId);
      }
      pointerFocusRef.current!.setHoverFromSource(CAD_PLAY_HOVER_SOURCE_HIERARCHY, target ? selectionTargetHoverKey(target) : null);
    },
    [activeModelDefinitionId, handleActiveModelDefinitionChange],
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
      activeModelDefinitionId,
      selection: selectionInScope,
      hoveredKey: pointerFocus.hover,
      selectTarget: selectHierarchyTarget,
      hoverTarget: hoverHierarchyTarget,
    });
    return () => publishCadPlayChrome(null);
  }, [
    activeModelDefinitionId,
    flushedModelsByDefinitionId,
    flushedModelsDigest,
    hoverHierarchyTarget,
    modelDefinitionRevision,
    pointerFocus.hover,
    publishCadPlayChrome,
    selectHierarchyTarget,
    selectionInScope,
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
        space.transform(spec.source.modelDefinition, spec.target.modelDefinition, spec);
      } catch (error) {
        setFileStatus(`Transform failed: ${String(error)}`);
        return;
      }
      setModelsByDefinitionId(ensureCadPlayQuadModels(recordFromModelSpace(space)));
      setActiveModelDefinitionId(spec.target.modelDefinition);
      setModelDefinitionRevision((r) => r + 1);
      setFileStatus(`Transformed ${spec.source.modelDefinition} → ${spec.target.modelDefinition}.`);
    },
    [activeModelDefinitionId, liveModel, modelsByDefinitionId],
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
        transformsTo: listTransformationsFromModelDefinition(activeModelDefinitionId),
        transformsFrom: listTransformationsIntoModelDefinition(activeModelDefinitionId),
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
          case "engagementPossibleSelect": {
            const pane = (args as { pane?: CadPlayPaneId })?.pane;
            const possibleId = (args as { possibleId?: string })?.possibleId;
            if (!pane || !CAD_PLAY_PANE_IDS.includes(pane) || !possibleId) break;
            engagementSpecRefByPane.current[pane]?.possibleEngagements?.find((row) => row.id === possibleId)?.onSelect?.();
            break;
          }
          default:
            break;
        }
      },
    };
    shellController.setHostBridge(bridge);
    return () => shellController.setHostBridge(null);
  }, [activeModelDefinitionId, focusModelDefinition, handleApplyTransformation, handleLoadRawRequest, handleSaveCurrent, handleSaveInPlay, handleSaveSelected, selectionInScope, shellController]);

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
        setModelsByDefinitionId(recordFromModelSpace(modelSpace));
        setActiveModelDefinitionId(nextActiveModelDefinitionId);
        setModelDefinitionRevision((r) => r + 1);
        setFileStatus(`Loaded model space from ${file.name}.`);
        return;
      }
      const model = parseModelJson(snapshot);
      if (!model) throw new Error("No spatial model found in file.");
      setShapeAssetId("");
      setLoadedRawName(file.name);
      setModelsByDefinitionId(modelsFromCadJson(model.toJSON()));
      setActiveModelDefinitionId(SHAPE_MODEL_DEFINITION_ID);
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
    }),
    [
      activeModelDefinitionId,
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
  if (selectionInScope.length === 0) {
    return <p className="text-muted-foreground leading-snug">Select a primitive or object in the canvas or workbench hierarchy to edit attributes and properties.</p>;
  }
  return (
    <>
      <SelectionAttributesPanel model={liveModel} activeModelDefinitionId={activeModelDefinitionId} selection={selectionInScope} selectionCount={selectionInScope.length} onModelChange={handleModelAttributesChange} />
      <SelectionPropertiesPanel model={liveModel} kernel={brepjsKernel} activeModelDefinitionId={activeModelDefinitionId} selection={selectionInScope} selectionCount={selectionInScope.length} />
    </>
  );
}

/** @emoji 📦 Workbench catalog: shape fixtures and file I/O status (toolbar handles save/load). */
function CadPlayCatalogAside(): ReactNode {
  const { activeModelDefinitionId, shapeAssetId, handleShapeAssetChange, fileStatus } = useCadPlayModelSpace();
  const statusTone = fileStatus.startsWith("Load failed") || fileStatus.startsWith("Save failed") ? "text-destructive" : "text-muted-foreground";
  return (
    <>
      {isShapeModelDefinition(activeModelDefinitionId) ? (
        <Label id="cad.play.catalog.shape" label="Shape asset">
          <Select value={shapeAssetId || "__none__"} onValueChange={(value) => handleShapeAssetChange(value === "__none__" ? "" : value)}>
            <SelectTrigger className="h-medium w-full" id="cad.play.catalog.shape.trigger" size="sm">
              <SelectValue placeholder="No asset" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="__none__">No asset</SelectItem>
              {SHAPE_ASSETS.map((g) => (
                <SelectItem key={g.id} value={g.id}>
                  [{g.key}] {g.label} ({modelVertexCount(g.json)} verts)
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </Label>
      ) : (
        <p className="text-muted-foreground leading-snug">
          Shape assets apply to <code className="text-foreground">spatial.shape</code>. Focus the Shape pane to load source geometry.
        </p>
      )}
      {fileStatus ? <p className={statusTone}>{fileStatus}</p> : null}
    </>
  );
}

/** @emoji 🎮 One quad pane: interaction editing for its model definition with window engagement. */
function CadPlayInteractionPane({ pane }: { readonly pane: CadPlayPaneId }): ReactNode {
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
    modelDefinitionRevision,
    setModelDefinitionRevision,
    handleApplyTransformation,
    commitModelForDefinition,
    handleSnapshotChangeForPane,
    handleEngagementChangeForPane,
    hoveredPickKey,
    onCanvasHoverTarget,
    onHoveredPickKeyChange,
  } = useCadPlayModelSpace();
  const modelDefinitionId = cadPlayModelDefinitionIdForPane(pane);
  const captureGlobalKeys = activeModelDefinitionId === modelDefinitionId;
  const interactionId = interactionIdForPane(pane);
  const spec = specForPane(pane);
  const paneModel = cadPlayPaneModel(flushedModelsByDefinitionId, pane);
  const documentModel = reactHostPort.useMemo((): ModelDocument => ({ model: Model.fromJSON(paneModel.toJSON()), nodes: [] }), [paneModel, modelDefinitionRevision]);
  const pickGeometry = reactHostPort.useMemo(
    () => pickShapeForModelDefinition(flushedModelsByDefinitionId, modelDefinitionId, paneModel),
    [flushedModelsByDefinitionId, modelDefinitionId, paneModel, modelDefinitionRevision],
  );
  const commitPaneModel = reactHostPort.useCallback((model: Model) => commitModelForDefinition(modelDefinitionId, model), [commitModelForDefinition, modelDefinitionId]);
  const onSnapshot = reactHostPort.useCallback((snapshot: InteractionSnapshot) => handleSnapshotChangeForPane(pane, snapshot), [handleSnapshotChangeForPane, pane]);
  const onEngagementChange = reactHostPort.useCallback((engagement: EngagementSpec | null) => handleEngagementChangeForPane(pane, engagement), [handleEngagementChangeForPane, pane]);
  const onInteractionId = reactHostPort.useCallback((id: string) => handleInteractionPickForPane(pane, id), [handleInteractionPickForPane, pane]);

  if (interactionId && !loadSpatialInteraction(interactionId)) {
    return (
      <div className="flex flex-col gap-double p-double text-destructive text-xs">
        <p>
          Unknown interaction <code className="text-foreground">{interactionId}</code>.
        </p>
        <button type="button" className="w-fit rounded-md border border-border bg-background px-double py-single text-sm text-foreground" onClick={() => onInteractionId("")}>
          Reset
        </button>
      </div>
    );
  }

  const mode = computeModeForPane(pane);
  const autoFitMeshes = pane !== "shape";

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
        modelDefinitionRevision={modelDefinitionRevision}
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
  const pane = cadPlayPaneFromSurfaceId(node.surfaceId);
  if (!pane) {
    return <div className="p-single text-destructive text-xs">Unknown CAD play surface</div>;
  }
  return (
    <div className="absolute inset-0 flex min-h-0 min-w-0 flex-col overflow-hidden">
      <CadPlayInteractionPane pane={pane} />
    </div>
  );
}

class CadPlayCatalogPanelDefinition extends PureSidePanelTabDefinition {
  resolveTab(): SidePanelTabConfig {
    return {
      id: "cad-play-catalog",
      icon: Shapes,
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
  const chromeSnapshotRef = reactHostPort.useRef(chromeSnapshot);
  chromeSnapshotRef.current = chromeSnapshot;
  const chromeKey = chromeSnapshot
    ? `${chromeSnapshot.activeModelDefinitionId}\u0001${chromeSnapshot.selection.map((row) => `${row.kind}:${row.id}`).join(",")}\u0001${chromeSnapshot.hoveredKey ?? ""}\u0001${cadPlayModelsDigest(chromeSnapshot.modelsByDefinitionId)}`
    : "";
  const workbenchTabs = reactHostPort.useMemo(
    () => [
      new CadPlayCatalogPanelDefinition().resolveTab(),
      ...(chromeSnapshot
        ? [
            new CadPlayHierarchyPanelDefinition(() => {
              const snap = chromeSnapshotRef.current;
              if (!snap) return [];
              return buildCadPlayHierarchySections(snap.modelsByDefinitionId, snap.activeModelDefinitionId, snap.selection, snap.selectTarget, snap.hoveredKey, snap.hoverTarget);
            }).resolveTab(),
          ]
        : []),
    ],
    [chromeSnapshot, chromeKey],
  );
  const detailsTabs = reactHostPort.useMemo(() => [new CadPlayDetailsPanelDefinition().resolveTab()], []);
  return (
    <CadPlayChromeContext.Provider value={chromeContextValue}>
      <CadPlayModelSpaceProvider runtime={runtimeRef.current} shellController={shellController}>
        <CadPlayLoadInput />
        <PlaygroundView runtime={runtimeRef.current} defaultAppId={CAD_PLAY_APP_ID} augmentPanelTabs={{ workbench: workbenchTabs, details: detailsTabs }} />
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

  describe("windowEngagementsEqual", () => {
    it("treats engagement snapshots with the same visible fields as equal", () => {
      const left: WindowEngagement = {
        input: { id: "engagement-input", value: "box", placeholder: "Type an interaction" },
        status: [{ id: "engagement-state", text: "State: idle" }],
        possibleEngagements: [{ id: "primitive.box", label: "Box", detail: "b" }],
      };
      const right: WindowEngagement = {
        input: { id: "engagement-input", value: "box", placeholder: "Type an interaction" },
        status: [{ id: "engagement-state", text: "State: idle" }],
        possibleEngagements: [{ id: "primitive.box", label: "Box", detail: "b", command: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementPossibleSelect", args: { pane: "shape", possibleId: "primitive.box" } } }],
      };
      expect(windowEngagementsEqual(left, right)).toBe(true);
      expect(windowEngagementDigest(left)).toBe(windowEngagementDigest(right));
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
          input: { id: "in", value: "box", placeholder: "Type an interaction", onSubmit: () => {} },
          status: [{ id: "state", content: "State: idle" }],
        },
        "energy",
      );
      expect(mirror?.options?.[0]).toMatchObject({
        id: "confirm",
        label: "Confirm",
        command: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementOption", args: { pane: "energy", optionId: "confirm" } },
      });
      expect(mirror?.input).toMatchObject({ value: "box", onSubmit: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementSubmit", args: { pane: "energy" } } });
      expect(mirror?.status?.[0]).toEqual({ id: "state", text: "State: idle" });
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
        input: { id: "engagement-input", value: "", placeholder: "Type an interaction" },
        possibleEngagements: [{ id: "primitive.box", label: "Box", detail: "b" }],
      };
      controller.setPaneEngagement("shape", engagement);
      const afterFirst = runtime.generation;
      controller.setPaneEngagement("shape", { ...engagement, possibleEngagements: [{ ...engagement.possibleEngagements![0]!, command: { controllerId: CAD_PLAY_CONTROLLER_ID, command: "engagementPossibleSelect", args: { pane: "shape", possibleId: "primitive.box" } } }] });
      expect(runtime.generation).toBe(afterFirst);
      expect(afterFirst).toBeGreaterThan(generation);
    });

    it("attaches engagement per pane and routes pane-scoped engagement commands to the host bridge", () => {
      const runtime = new Platform();
      const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
      const calls: { command: string; args?: unknown }[] = [];
      controller.setHostBridge({
        getToolbarState: () => ({ activeModelDefinitionId: SHAPE_MODEL_DEFINITION_ID, selectionCount: 0, transformsTo: [], transformsFrom: [] }),
        runHostCommand: (command, args) => calls.push({ command, args }),
      });
      controller.setPaneEngagement("shape", { options: [{ id: "confirm", label: "Confirm" }] });
      controller.setPaneEngagement("energy", { options: [{ id: "wall", label: "Wall" }] });
      const shapeWindow = controller.mainMode.windowKinds.find((row) => row.id === CAD_PLAY_SHAPE_WINDOW_ID);
      const energyWindow = controller.mainMode.windowKinds.find((row) => row.id === CAD_PLAY_ENERGY_WINDOW_ID);
      const buildingWindow = controller.mainMode.windowKinds.find((row) => row.id === CAD_PLAY_BUILDING_WINDOW_ID);
      expect(shapeWindow?.engagement?.options?.[0]?.id).toBe("confirm");
      expect(energyWindow?.engagement?.options?.[0]?.id).toBe("wall");
      expect(buildingWindow?.engagement).toBeUndefined();
      controller.run("engagementOption", { pane: "shape", optionId: "confirm" });
      controller.run("engagementSubmit", { pane: "energy", value: "box" });
      expect(calls).toEqual([
        { command: "engagementOption", args: { pane: "shape", optionId: "confirm" } },
        { command: "engagementSubmit", args: { pane: "energy", value: "box" } },
      ]);
    });
  });

  describe("buildCadPlayAppRuntime", () => {
    it("focuses the pane model definition when the active window changes", () => {
      const runtime = new Platform();
      const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
      const calls: { command: string; args?: unknown }[] = [];
      controller.setHostBridge({
        getToolbarState: () => ({ activeModelDefinitionId: SHAPE_MODEL_DEFINITION_ID, selectionCount: 0, transformsTo: [], transformsFrom: [] }),
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
      expect(cadPlayPaneForModelDefinition(SHAPE_MODEL_DEFINITION_ID)).toBe("shape");
      expect(cadPlayPaneForModelDefinition(CAD_PLAY_BUILDING_MODEL_DEFINITION_ID)).toBe("building");
      expect(cadPlayPaneForModelDefinition(CAD_PLAY_ENERGY_MODEL_DEFINITION_ID)).toBe("energy");
      expect(cadPlayPaneForModelDefinition(CAD_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID)).toBe("structure-classic");
    });
  });

  describe("buildCadPlayToolbarTools", () => {
    it("registers view, save, and transform categories", () => {
      const tools = buildCadPlayToolbarTools(
        {
          activeModelDefinitionId: SHAPE_MODEL_DEFINITION_ID,
          selectionCount: 0,
          transformsTo: [],
          transformsFrom: [],
        },
        CAD_PLAY_CONTROLLER_ID,
      );
      expect(tools.view?.length).toBeGreaterThan(0);
      expect(tools.save?.map((row) => row.id)).toEqual(["cad.play.save.selected", "cad.play.save.modelspace", "cad.play.save.current", "cad.play.save.load"]);
      expect(tools.save?.[0]?.disabled).toBe(true);
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
      expect(app?.leftTabs).toEqual([]);
      expect(app?.rightTabs).toEqual([]);
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
        activeModelDefinitionId: SHAPE_MODEL_DEFINITION_ID,
      });
      await rt.send({ kind: "pointer.down", point: [0, 0, 0], modifiers: {} });
      await rt.send({ kind: "pointer.down", point: [2, 3, 0], modifiers: {} });
      await rt.send({ kind: "set.height", value: 4, modifiers: {} });
      await rt.send({ kind: "confirm", modifiers: {} });
      const sections = buildCadPlayHierarchySections({ [SHAPE_MODEL_DEFINITION_ID]: model }, SHAPE_MODEL_DEFINITION_ID, [], () => {});
      const modelBranch = sections[0]?.items?.[0]?.items?.[0];
      expect(modelBranch?.items?.some((row) => row.label !== "(no objects)")).toBe(true);
    });

    it("buildCadPlayHierarchySections highlights hovered topology keys", async () => {
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
      const sections = buildCadPlayHierarchySections({ "spatial.shape": model }, "spatial.shape", [], () => {}, "object:box1");
      const modelBranch = sections[0]?.items?.[0]?.items?.[0];
      const objectNode = modelBranch?.items?.[0];
      expect(objectNode?.isHighlighted).toBe(true);
    });

    it("buildCadPlayHierarchySections nests topology under primitive slots", async () => {
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
      const sections = buildCadPlayHierarchySections({ "spatial.shape": model }, "spatial.shape", [], () => {});
      const primitiveNode = sections[0]?.items?.[0]?.items?.[0]?.items?.[0]?.items?.[0];
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
      const models = { [SHAPE_MODEL_DEFINITION_ID]: shape, [CAD_PLAY_BUILDING_MODEL_DEFINITION_ID]: building };
      expect(cadPlayPaneModel(models, "shape").objects["box1"]).toBeDefined();
      expect(cadPlayPaneModel(models, "building").objects["site1"]).toBeDefined();
      expect(cadPlayPaneModel(models, "shape")).not.toBe(cadPlayPaneModel(models, "building"));
    });
  });

  describe("CAD play model bootstrap", () => {
    it("emptyPlayModels always seeds spatial.shape", () => {
      expect(emptyPlayModels()[SHAPE_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
    });

    it("modelsFromCadJson on empty model space still seeds spatial.shape", () => {
      const models = modelsFromCadJson(new ModelSpace().toJSON());
      expect(models[SHAPE_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
    });

    it("modelsFromCadJson loads fixture models under spatial.shape", () => {
      const models = modelsFromCadJson(geometrySmallBuilding);
      expect(models[SHAPE_MODEL_DEFINITION_ID]?.objects).not.toEqual({});
    });

    it("ensureDerivedModelInSpace keeps spatial.shape for shape definition", () => {
      const models = ensureDerivedModelInSpace({}, SHAPE_MODEL_DEFINITION_ID);
      expect(models[SHAPE_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
    });

    it("ensureCadPlayQuadModels seeds all four play panes", () => {
      const models = ensureCadPlayQuadModels({});
      expect(models[SHAPE_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
      expect(models[CAD_PLAY_BUILDING_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
      expect(models[CAD_PLAY_ENERGY_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
      expect(models[CAD_PLAY_STRUCTURE_CLASSIC_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
    });
  });
}
//#endregion 🧪Tests
