// #region 🧲Header
/** @emoji 👯 Puzzle 5D play app — unified flat + volume puzzle editor. */
// #endregion 🧲Header

import {
	createPlaygroundApp,
	createProductPlaygroundPlatform,
  CommandBus,
  Controller,
  Store,
  Platform,
  AppRuntime,
  ModeRuntime,
  WindowKindRuntime,
  buildPuzzle2dWindowBody,
  buildPuzzle3dWindowBody,
  createDefaultLayout,
  createJackPlayWindowEngagement,
  type ToolLeaf,
  type ToolNode,
  toolCollection,
  type WindowBodyViewContext,
  type CommandDescriptor,
  type WindowEngagement,
  type WindowEngagementControl,
  type WindowMeasure,
  type UiNode,
  PLAYGROUND_NO_EXAMPLE_ID,
  type PlaygroundExampleCatalog,
  type PlaygroundExampleHost,
  type PlaygroundKeybinding,
  isPlaygroundExampleLocked,
  isPlaygroundNoExampleId,
  playgroundResolvedExampleId,
  playgroundTreePanelRootItems,
  platformFromViewContext,
  type UiTreeItemNode,
  type UiTreeNode,
  type UiTreeSectionNode,
  enforcePlaygroundWindowEngagementInput,
  collectUiTreeItemDragData,
  uiDeclarativeSectionsToTree,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
  FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
  FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID,
  FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
  FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
  buildControllerTreeSidePanelBody,
  registerSidePanelBody,
  type SideTabSpec,
  uiInspectorAllEqual,
  JackHoverBridge,
	buildWriterWindowBody,
	registerWindowBody,
} from "@semio-tech/framework-playground-core";
import { registerOsMediaExportHandler } from "@semio-tech/framework-os-core";
import { exportPuzzle3dFixtureGlb, exportPuzzle3dFixtureObj } from "@semio-tech/puzzle-3d-core";
import { createWriterDocument, type WriterDocument } from "@semio-tech/writer-core";
import { runJackOnBoardFixture } from "@semio-tech/graph-dsl-core";
import {
  DocumentVcsStore,
  createDocumentVcsEnvelope,
  recordProjectionChange,
} from "@semio-tech/vcs-core/internal";

import {
  buildPuzzle2dPlayToolbarTools,
  puzzle2dPlayViewportCameraForFixtureId,
  puzzle2dPlayViewportCameraFromFixture,
  type Puzzle2dPlayToolbarState,
} from "../../../2d/core/js/index.ts";
import {
  PUZZLE_2D_FIXTURE_DRAG_MIME,
  beginPuzzle2dFixturePalettePointerDrag,
  cancelPuzzle2dFixturePalettePointerDrag,
  puzzle2dFixturePaletteTreeDragController,
} from "../../../2d/react/index.tsx";
import {
  PUZZLE_2D_LOD_MODE_AUTOMATIC,
  puzzle2dLodAutomaticSelectLabel,
  puzzle2dLodCanvasProps,
  isPuzzle2dDrawLodKind,
  puzzle2dFixtureNodeDisplayLabel,
  parsePuzzle2dFixture,
  DEFAULT_PUZZLE_2D_SUGGESTION_OFFSET_PX,
  type Puzzle2dDrawLodKind,
  type Puzzle2dFixture,
  type Puzzle2dLodModeKind,
  type CameraState,
  type Puzzle2dSelectionMethod,
  type Puzzle2dSelectionMode,
  type Puzzle2dSelectionTargets,
  puzzle2dPlayNodeKindDragData,
} from "../../../2d/react/index.tsx";
import { bootstrapElementsSurfaceChromeDocument, type GumballConfig } from "@semio-tech/ui-react";
import { PUZZLE_3D_GUMBALL_CONFIG, PUZZLE_3D_GUMBALL_GROUPS, type Puzzle3dGumballGroupKey } from "@semio-tech/puzzle-3d-core";
import {
  FIXTURE_DRAG_MIME,
  beginPuzzle3dFixturePalettePointerDrag,
  cancelPuzzle3dFixturePalettePointerDrag,
  puzzle3dFixturePaletteTreeDragController,
} from "../../../3d/react/index.tsx";
import {
  DEFAULT_MANUAL_LOD,
  PUZZLE_3D_LOD_SLIDER_MAX,
  PUZZLE_3D_LOD_SLIDER_MIN,
  formatLod,
  lodFromSliderValue,
  parseFixture,
  puzzle3dLodCanvasProps,
  sliderValueFromLod,
  type Fixture as Puzzle3dFixture,
  DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET,
  BRUSH_PLACEMENT_OVERLAP_BUDGET_MAX,
  BRUSH_PLACEMENT_OVERLAP_BUDGET_STEP,
  isLoadableMeshUrl,
  puzzle3dPlayObjectKindDragData,
  resolveObjectKindMeshUrl,
} from "../../../3d/react/index.tsx";
import {
  createStore,
  parseModel,
  project2d,
  project3d,
  project2dKindCatalogs,
  project3dKindCatalogs,
  compose5d,
  sharedKindsFromMetas,
  PUZZLE_5D_SCHEMA,
  type KindCatalogBundle as Puzzle5dKindCatalogBundle,
  PUZZLE_5D_ENGAGEMENT_TOOL_BRUSH_ID,
  PUZZLE_5D_ENGAGEMENT_TOOL_FILL_ID,
  PUZZLE_5D_ENGAGEMENT_TOOL_SELECT_ID,
  PUZZLE_5D_FILL_COUNT_MAX,
  type Puzzle5dActiveTool,
  type Puzzle5dBrushPlacement,
  type Store as Puzzle5dStore,
  type StoreSnapshot as Puzzle5dStoreSnapshot,
  type Model as Puzzle5dModel,
  gripFullId,
  parseGripFullId,
  type Puzzle5dPartPatchField,
  type Puzzle5dGripPatchField,
  applyBrushPlacementToModel,
  applyPuzzle5dModelEditOp,
  backwardsPuzzle5dModelEditOp,
  diffPuzzle5dModelEditOp,
  type Puzzle5dModelEditOp,
  type FastenerKind,
  type GripKind,
  type PartKind,
  type RopeKind,
  type SelectionSnapshot as Puzzle5dSelectionSnapshot,
} from "../../react/index.tsx";

//#region 🔖Ids
export const PUZZLE_5D_PLAY_APP_ID = "puzzle-5d-play";
export const PUZZLE_5D_PLAY_CONTROLLER_ID = "puzzle-5d-play";
export const PUZZLE_5D_PLAY_2D_WINDOW_ID = "puzzle-5d-2d";
export const PUZZLE_5D_PLAY_3D_WINDOW_ID = "puzzle-5d-3d";
export const PUZZLE_5D_PLAY_JACK_WINDOW_ID = "puzzle-5d-jack";
export const PUZZLE_5D_PLAY_2D_WINDOW_LABEL = "Puzzle 2d";
export const PUZZLE_5D_PLAY_3D_WINDOW_LABEL = "Puzzle 3d";
export const PUZZLE_5D_PLAY_JACK_WINDOW_LABEL = "Jack";
export const PUZZLE_5D_PLAY_2D_BODY_KEY = "puzzle.5d.play.2d";
export const PUZZLE_5D_PLAY_3D_BODY_KEY = "puzzle.5d.play.3d";
export const PUZZLE_5D_PLAY_JACK_BODY_KEY = "puzzle.5d.play.jack";
export const PUZZLE_5D_PLAY_2D_SURFACE_ID = "puzzle.5d.play.2d";
export const PUZZLE_5D_PLAY_3D_SURFACE_ID = "puzzle.5d.play.3d";
export const PUZZLE_5D_PLAY_JACK_SURFACE_ID = "puzzle.5d.play.jack";
export const PUZZLE_5D_PLAY_DEFAULT_JACK_QUERY = "MATCH (n:part) RETURN n.name";
export const PUZZLE_5D_PLAY_HIERARCHY_TAB_ID = "puzzle-5d-play-hierarchy";
export const PUZZLE_5D_PLAY_KINDS_TAB_ID = "puzzle-5d-play-kinds";
export const PUZZLE_5D_PLAY_INSPECTOR_TAB_ID = "puzzle-5d-play-inspector";
export const PUZZLE_5D_PLAY_HIERARCHY_BODY_KEY = "puzzle.5d.play.hierarchy";
export const PUZZLE_5D_PLAY_KINDS_BODY_KEY = "puzzle.5d.play.kinds";
export const PUZZLE_5D_PLAY_INSPECTOR_BODY_KEY = "puzzle.5d.play.inspector";
export const PUZZLE_5D_PLAY_ICON_KINDS = "puzzle.5d-play.icon.kinds";

const PUZZLE_5D_PLAY_LOD_TIERS_2D: readonly Puzzle2dDrawLodKind[] = ["minimap", "overview", "compact", "normal", "detail", "micro"];

function puzzle5dPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command, args: args as never };
}

/** @emoji 🃏 Normalizes a puzzle 5d model projection into board-shaped JSON for Jack queries. */
export function puzzle5dPlayJackBoardJson(model: Puzzle5dModel): string {
	const fixture2d = project2d(model);
	return JSON.stringify({
		schema: fixture2d.schema,
		nodes: fixture2d.nodes.map((node) => ({
			id: node.id,
			nodeKind: "part",
			text: puzzle2dFixtureNodeDisplayLabel(node),
		})),
		edges: fixture2d.edges,
	});
}

const PUZZLE_5D_SUGGESTION_OFFSET_MIN = 0;
const PUZZLE_5D_SUGGESTION_OFFSET_MAX = 160;
const PUZZLE_5D_SUGGESTION_OFFSET_STEP = 4;

/** @emoji 🔗 React host bridge: toolbar snapshot + commands that need canvas/fixture context. */
export interface Puzzle5dPlayHostBridge {
  getToolbarState(): Puzzle2dPlayToolbarState;
  runHostCommand(command: string, args?: unknown): void;
}
//#endregion 🔖Ids

//#region 🔖Puzzle5dPlayHierarchy
function puzzle5dPartDisplayLabel(part: Puzzle5dModel["parts"][number]): string {
  const kind = part.partKind?.trim();
  if (kind) return kind;
  const flatText = part["2d"]?.text?.trim();
  if (flatText) return flatText;
  const volumeLabel = part["3d"]?.label?.trim();
  if (volumeLabel) return volumeLabel;
  return part.id;
}

function puzzle5dGripDisplayLabel(grip: Puzzle5dModel["parts"][number]["grips"][number]): string {
  const kind = grip.gripKind?.trim();
  if (kind) return kind;
  const volumeLabel = grip["3d"]?.label?.trim();
  if (volumeLabel) return volumeLabel;
  return grip.id;
}

function puzzle5dFastenerDisplayLabel(fastener: Puzzle5dModel["fasteners"][number], model: Puzzle5dModel): string {
  if (fastener.fastenerKind?.trim()) return fastener.fastenerKind;
  const sourcePart = model.parts.find((part) => part.grips.some((grip) => gripFullId(part.id, grip.id) === fastener.source));
  const targetPart = model.parts.find((part) => part.grips.some((grip) => gripFullId(part.id, grip.id) === fastener.target));
  if (sourcePart && targetPart) return `${puzzle5dPartDisplayLabel(sourcePart)} → ${puzzle5dPartDisplayLabel(targetPart)}`;
  return fastener.id;
}

//#region 🔖Puzzle5dPlayHierarchy
/** @emoji 🖼️ Default tree-row icons for puzzle 5D entity kinds (Lucide catalog ids). */
export const PUZZLE5D_PLAY_ENTITY_TREE_ICON = {
  part: "box",
  grip: "circle-dot",
  fastener: "link",
  rope: "plug",
} as const;

type Puzzle5dPlayEntityTreeKind = keyof typeof PUZZLE5D_PLAY_ENTITY_TREE_ICON;

/** @emoji 🖼️ Resolves the tree-row icon id for a puzzle 5D entity kind. */
export function puzzle5dPlayEntityTreeIcon(kind: Puzzle5dPlayEntityTreeKind): string {
  return PUZZLE5D_PLAY_ENTITY_TREE_ICON[kind];
}

function puzzle5dPlayKindSectionTreeIcon(sectionId: string): string | undefined {
  if (sectionId === "puzzle-5d-play-kinds.parts") {
    return puzzle5dPlayEntityTreeIcon("part");
  }
  if (sectionId === "puzzle-5d-play-kinds.grips") {
    return puzzle5dPlayEntityTreeIcon("grip");
  }
  if (sectionId === "puzzle-5d-play-kinds.fasteners") {
    return puzzle5dPlayEntityTreeIcon("fastener");
  }
  if (sectionId === "puzzle-5d-play-kinds.ropes") {
    return puzzle5dPlayEntityTreeIcon("rope");
  }
  return undefined;
}

/** @emoji 🌳 Puzzle 5d hierarchy: Parts (with nested Grips) and Fasteners. */
export function buildPuzzle5dPlayHierarchySections(snapshot: Puzzle5dPlaySnapshot): UiTreeNode {
  const model = snapshot.model;
  const selectedPartIds = new Set(snapshot.selection.partIds);
  const selectedGripIds = new Set(snapshot.selection.gripIds);
  const partItems: UiTreeItemNode[] = model.parts.map((part) => {
    const gripItems: UiTreeItemNode[] = part.grips.map((grip) => {
      const fullId = gripFullId(part.id, grip.id);
      return {
        id: `puzzle-5d-play-hierarchy.grip.${fullId}`,
        label: puzzle5dGripDisplayLabel(grip),
        description: fullId,
        icon: puzzle5dPlayEntityTreeIcon("grip"),
        isSelected: selectedGripIds.has(fullId),
        command: puzzle5dPlayCmd("hierarchySelectGrip", { gripFullId: fullId }),
      };
    });
    return {
      id: `puzzle-5d-play-hierarchy.part.${part.id}`,
      label: puzzle5dPartDisplayLabel(part),
      description: part.id,
      icon: puzzle5dPlayEntityTreeIcon("part"),
      defaultOpen: gripItems.length > 0,
      isSelected: selectedPartIds.has(part.id),
      command: puzzle5dPlayCmd("hierarchySelectPart", { partId: part.id }),
      ...(gripItems.length ? { items: gripItems } : {}),
    };
  });
  const fastenerItems: UiTreeItemNode[] = model.fasteners.map((fastener) => ({
    id: `puzzle-5d-play-hierarchy.fastener.${fastener.id}`,
    label: puzzle5dFastenerDisplayLabel(fastener, model),
    description: `${fastener.source} → ${fastener.target}`,
    icon: puzzle5dPlayEntityTreeIcon("fastener"),
    command: puzzle5dPlayCmd("hierarchySelectFastener", { fastenerId: fastener.id }),
  }));
  if (!partItems.length && !fastenerItems.length) {
    return playgroundTreePanelRootItems("puzzle-5d-play-hierarchy.root", [{ id: "puzzle-5d-play-hierarchy.empty", label: "(no parts)" }]);
  }
  return {
    type: "tree",
    sections: [
      {
        id: "puzzle-5d-play-hierarchy.parts",
        label: "Parts",
        defaultOpen: false,
        items: partItems.length ? partItems : [{ id: "puzzle-5d-play-hierarchy.parts.empty", label: "(none)" }],
      },
      {
        id: "puzzle-5d-play-hierarchy.fasteners",
        label: "Fasteners",
        defaultOpen: false,
        items: fastenerItems.length ? fastenerItems : [{ id: "puzzle-5d-play-hierarchy.fasteners.empty", label: "(none)" }],
      },
    ],
  };
}
//#endregion 🔖Puzzle5dPlayHierarchy

//#region 🔖Puzzle5dPlayKinds
type Puzzle5dCatalogKind = PartKind | GripKind | RopeKind | FastenerKind;

function puzzle5dCatalogKindLabel(entry: Puzzle5dCatalogKind): string {
  const display = entry.label?.trim() || entry.name?.trim();
  return display && display.length > 0 ? display : entry.id;
}

function puzzle5dCatalogGripKindLabel(gripKindId: string, gripKinds: readonly GripKind[] | undefined): string {
  const entry = gripKinds?.find((row) => row.id === gripKindId);
  return entry ? puzzle5dCatalogKindLabel(entry) : gripKindId;
}

function puzzle5dPartKindGripCatalogItems(sectionId: string, partIndex: number, partKindId: string, gripKinds: readonly GripKind[] | undefined): readonly UiTreeItemNode[] {
  return (gripKinds ?? []).map((grip, gripIndex) => ({
    id: `${sectionId}.${partIndex}.${partKindId}.grip.${gripIndex}`,
    label: puzzle5dCatalogGripKindLabel(grip.id, gripKinds),
    description: grip.id,
    icon: puzzle5dPlayEntityTreeIcon("grip"),
  }));
}

function puzzle5dPlayKindCatalogSection(
  sectionId: string,
  label: string,
  entries: readonly Puzzle5dCatalogKind[] | undefined,
  gripKinds?: readonly GripKind[],
  sectionDefaultOpen = false,
  bundle?: Puzzle5dKindCatalogBundle,
  fixture3d?: Puzzle3dFixture | null,
): UiTreeSectionNode | null {
  if (!entries?.length) {
    return null;
  }
  const catalogs2d = project2dKindCatalogs(bundle);
  const catalogs3d = project3dKindCatalogs(bundle);
  const isPartPalette = sectionId === "puzzle-5d-play-kinds.parts";
  const sectionTreeIcon = puzzle5dPlayKindSectionTreeIcon(sectionId);
  const items: UiTreeItemNode[] = [...entries]
    .sort((a, b) => puzzle5dCatalogKindLabel(a).localeCompare(puzzle5dCatalogKindLabel(b)))
    .map((entry, index) => {
      const partKind = isPartPalette ? (entry as PartKind) : null;
      const gripItems = partKind ? puzzle5dPartKindGripCatalogItems(sectionId, index, entry.id, gripKinds) : [];
      const flatDrag = isPartPalette ? puzzle2dPlayNodeKindDragData(entry.id, catalogs2d) : undefined;
      const volumeDrag =
        isPartPalette && isLoadableMeshUrl(resolveObjectKindMeshUrl(entry.id, catalogs3d, fixture3d ?? undefined))
          ? puzzle3dPlayObjectKindDragData(entry.id, fixture3d?.domain)
          : undefined;
      const dragData = flatDrag || volumeDrag ? { ...(flatDrag ?? {}), ...(volumeDrag ?? {}) } : undefined;
      return {
        id: `${sectionId}.${index}.${entry.id}`,
        label: puzzle5dCatalogKindLabel(entry),
        description: entry.id,
        icon: sectionTreeIcon,
        defaultOpen: gripItems.length === 0,
        ...(gripItems.length ? { items: gripItems } : {}),
        ...(dragData ? { draggable: true, dragData } : {}),
      };
    });
  return { id: sectionId, label, defaultOpen: sectionDefaultOpen, items };
}

/** @emoji 🏷️ Workbench kinds tab: Parts, Grips, Fasteners, Ropes. */
export function buildPuzzle5dPlayKindsTree(snapshot: Puzzle5dPlaySnapshot): UiTreeNode {
  const bundle: Puzzle5dKindCatalogBundle | undefined = snapshot.kindCatalogs ?? snapshot.sharedKinds.kindCatalogs;
  const sections = [
    puzzle5dPlayKindCatalogSection("puzzle-5d-play-kinds.parts", "Parts", bundle?.parts, bundle?.grips, false, bundle, snapshot.fixture3d),
    puzzle5dPlayKindCatalogSection("puzzle-5d-play-kinds.grips", "Grips", bundle?.grips, undefined, false, bundle, snapshot.fixture3d),
    puzzle5dPlayKindCatalogSection("puzzle-5d-play-kinds.fasteners", "Fasteners", bundle?.fasteners, undefined, false, bundle, snapshot.fixture3d),
    puzzle5dPlayKindCatalogSection("puzzle-5d-play-kinds.ropes", "Ropes", bundle?.ropes, undefined, false, bundle, snapshot.fixture3d),
  ].filter((section): section is UiTreeSectionNode => section !== null);
  if (!sections.length) {
    return {
      type: "tree",
      sections: [
        {
          id: "puzzle-5d-play-kinds.empty",
          label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
          defaultOpen: false,
          items: [{ id: "puzzle-5d-play-kinds.empty.msg", label: "No kind catalogs in this fixture" }],
        },
      ],
    };
  }
  return { type: "tree", sections };
}
//#endregion 🔖Puzzle5dPlayKinds

//#region 🔖Puzzle5dPlayInspection
function puzzle5dPlayInspectorVec3Value(value: readonly [number, number, number] | null | undefined): readonly [number, number, number] | null {
  if (value == null || !Array.isArray(value) || value.length < 3) {
    return null;
  }
  const x = value[0];
  const y = value[1];
  const z = value[2];
  if (!Number.isFinite(x) || !Number.isFinite(y) || !Number.isFinite(z)) {
    return null;
  }
  return [x, y, z];
}

function puzzle5dPlayInspectorUniformVec3(values: readonly (readonly [number, number, number] | null | undefined)[]): readonly [number, number, number] | null {
  if (!values.length || !uiInspectorAllEqual(values)) {
    return null;
  }
  return puzzle5dPlayInspectorVec3Value(values[0]);
}

function puzzle5dPlayInspectorKindItems(
  rows: readonly { readonly id?: string; readonly name?: string; readonly label?: string }[] | undefined,
): readonly { readonly id: string; readonly label: string; readonly value: string }[] {
  if (!rows?.length) {
    return [];
  }
  return rows.map((row) => ({
    id: row.id ?? row.name ?? row.label ?? "",
    label: row.label ?? row.name ?? row.id ?? "",
    value: row.id ?? row.name ?? row.label ?? "",
  }));
}

/** @emoji 🔎 Declarative inspection panel for puzzle 5d play selection. */
export function buildPuzzle5dPlayInspectorTree(snapshot: Puzzle5dPlaySnapshot): UiTreeNode {
  const { selection, model, kindCatalogs } = snapshot;
  const bundle = kindCatalogs ?? snapshot.sharedKinds.kindCatalogs;
  const children: UiNode[] = [
    {
      type: "section",
      id: "puzzle-5d-play-inspector.header",
      label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      children: [
        {
          type: "text",
          value: `${selection.partIds.length} parts · ${selection.gripIds.length} grips · tool ${snapshot.activeTool}`,
        },
        ...(selection.partIds.length === 0 && selection.gripIds.length === 0
          ? [{ type: "text" as const, value: "Select parts or grips in the canvas or workbench hierarchy." }]
          : []),
        {
          type: "button",
          id: "puzzle-5d-play-inspector.delete",
          label: "Delete selection",
          command: puzzle5dPlayCmd("deleteSelection"),
        },
      ],
    },
  ];
  if (selection.partIds.length > 0) {
    const parts = selection.partIds
      .map((partId) => model.parts.find((part) => part.id === partId))
      .filter((part): part is Puzzle5dModel["parts"][number] => Boolean(part));
    if (parts.length === 0) {
      children.push({
        type: "section",
        id: "puzzle-5d-play-inspector.parts",
        label: `Parts (${selection.partIds.length})`,
        children: [{ type: "text", value: "Selected parts are not available in the model yet." }],
      });
    } else {
    const partKinds = parts.map((part) => part.partKind ?? "");
    const partKindUniform = uiInspectorAllEqual(partKinds);
    const labels = parts.map((part) => part["3d"]?.label ?? part["2d"]?.text ?? "");
    const labelUniform = uiInspectorAllEqual(labels);
    const xs = parts.map((part) => part["2d"]?.x ?? Number.NaN);
    const ys = parts.map((part) => part["2d"]?.y ?? Number.NaN);
    const xUniform = uiInspectorAllEqual(xs);
    const yUniform = uiInspectorAllEqual(ys);
    const origins = parts.map((part) => part["3d"]?.origin ?? null);
    const partFields: UiNode[] = [
      {
        type: "field",
        id: "puzzle-5d-play-inspector.part.kind",
        label: "Part kind",
        child: {
          type: "select",
          id: "puzzle-5d-play-inspector.part.kind.select",
          value: partKindUniform ? (partKinds[0] ?? "") : "",
          placeholder: partKindUniform ? "kind" : "Mixed",
          items: puzzle5dPlayInspectorKindItems(bundle?.parts),
          onChange: puzzle5dPlayCmd("patchPuzzle5dParts", { partIds: selection.partIds, field: "partKind" satisfies Puzzle5dPartPatchField }),
        },
      },
      {
        type: "field",
        id: "puzzle-5d-play-inspector.part.label",
        label: "Label",
        child: {
          type: "input",
          id: "puzzle-5d-play-inspector.part.label.input",
          inputKind: "text",
          value: labelUniform ? (labels[0] ?? "") : "",
          placeholder: labelUniform ? undefined : "Mixed",
          onChange: puzzle5dPlayCmd("patchPuzzle5dParts", { partIds: selection.partIds, field: "label" satisfies Puzzle5dPartPatchField }),
        },
      },
      {
        type: "field",
        id: "puzzle-5d-play-inspector.part.text",
        label: "Flat text",
        child: {
          type: "input",
          id: "puzzle-5d-play-inspector.part.text.input",
          inputKind: "text",
          value: labelUniform ? (parts[0]?.["2d"]?.text ?? "") : "",
          placeholder: labelUniform ? undefined : "Mixed",
          onChange: puzzle5dPlayCmd("patchPuzzle5dParts", { partIds: selection.partIds, field: "text" satisfies Puzzle5dPartPatchField }),
        },
      },
      {
        type: "field",
        id: "puzzle-5d-play-inspector.part.x",
        label: "Flat x",
        child: {
          type: "numberStepper",
          id: "puzzle-5d-play-inspector.part.x.stepper",
          value: xUniform ? xs[0]! : Number.NaN,
          step: 1,
          uniform: xUniform,
          onAbsolute: puzzle5dPlayCmd("patchPuzzle5dParts", { partIds: selection.partIds, field: "x" satisfies Puzzle5dPartPatchField }),
        },
      },
      {
        type: "field",
        id: "puzzle-5d-play-inspector.part.y",
        label: "Flat y",
        child: {
          type: "numberStepper",
          id: "puzzle-5d-play-inspector.part.y.stepper",
          value: yUniform ? ys[0]! : Number.NaN,
          step: 1,
          uniform: yUniform,
          onAbsolute: puzzle5dPlayCmd("patchPuzzle5dParts", { partIds: selection.partIds, field: "y" satisfies Puzzle5dPartPatchField }),
        },
      },
      {
        type: "field",
        id: "puzzle-5d-play-inspector.part.origin",
        label: "Volume origin",
        child: {
          type: "vec3",
          id: "puzzle-5d-play-inspector.part.origin.vec3",
          value: puzzle5dPlayInspectorUniformVec3(origins),
          onChange: puzzle5dPlayCmd("patchPuzzle5dParts", { partIds: selection.partIds, field: "origin" satisfies Puzzle5dPartPatchField }),
        },
      },
    ];
    children.push({
      type: "section",
      id: "puzzle-5d-play-inspector.parts",
      label: `Parts (${selection.partIds.length})`,
      children: partFields,
    });
    }
  }
  if (selection.gripIds.length === 1) {
    const gripFull = selection.gripIds[0]!;
    const parsed = parseGripFullId(gripFull);
    const hostPart = parsed ? model.parts.find((part) => part.id === parsed.partId) : undefined;
    const grip = hostPart?.grips.find((row) => row.id === parsed?.gripId);
    if (grip && parsed) {
      const gripFields: UiNode[] = [
        {
          type: "field",
          id: "puzzle-5d-play-inspector.grip.id",
          label: "Full id",
          child: { type: "text", value: gripFull },
        },
        {
          type: "field",
          id: "puzzle-5d-play-inspector.grip.kind",
          label: "Grip kind",
          child: {
            type: "select",
            id: "puzzle-5d-play-inspector.grip.kind.select",
            value: grip.gripKind ?? "",
            items: puzzle5dPlayInspectorKindItems(bundle?.grips),
            onChange: puzzle5dPlayCmd("patchPuzzle5dGrips", { gripFullIds: [gripFull], field: "gripKind" satisfies Puzzle5dGripPatchField }),
          },
        },
        {
          type: "field",
          id: "puzzle-5d-play-inspector.grip.label",
          label: "Label",
          child: {
            type: "input",
            id: "puzzle-5d-play-inspector.grip.label.input",
            inputKind: "text",
            value: grip["3d"]?.label ?? "",
            onChange: puzzle5dPlayCmd("patchPuzzle5dGrips", { gripFullIds: [gripFull], field: "label" satisfies Puzzle5dGripPatchField }),
          },
        },
        {
          type: "field",
          id: "puzzle-5d-play-inspector.grip.angle",
          label: "Flat angle",
          child: {
            type: "numberStepper",
            id: "puzzle-5d-play-inspector.grip.angle.stepper",
            value: grip["2d"]?.angle ?? Number.NaN,
            step: 1,
            uniform: true,
            onAbsolute: puzzle5dPlayCmd("patchPuzzle5dGrips", { gripFullIds: [gripFull], field: "angle" satisfies Puzzle5dGripPatchField }),
          },
        },
        {
          type: "field",
          id: "puzzle-5d-play-inspector.grip.radius",
          label: "Radius",
          child: {
            type: "input",
            id: "puzzle-5d-play-inspector.grip.radius.input",
            inputKind: "number",
            value: String(grip["3d"]?.radius ?? grip["2d"]?.radius ?? ""),
            onChange: puzzle5dPlayCmd("patchPuzzle5dGrips", { gripFullIds: [gripFull], field: "radius" satisfies Puzzle5dGripPatchField }),
          },
        },
      ];
      if (grip["3d"]) {
        gripFields.push(
          {
            type: "field",
            id: "puzzle-5d-play-inspector.grip.position",
            label: "Position",
            child: {
              type: "vec3",
              id: "puzzle-5d-play-inspector.grip.position.vec3",
              value: puzzle5dPlayInspectorVec3Value(grip["3d"].position),
              onChange: puzzle5dPlayCmd("patchPuzzle5dGrips", { gripFullIds: [gripFull], field: "position" satisfies Puzzle5dGripPatchField }),
            },
          },
          ...(grip["3d"].direction
            ? [
                {
                  type: "field" as const,
                  id: "puzzle-5d-play-inspector.grip.direction",
                  label: "Direction",
                  child: {
                    type: "vec3" as const,
                    id: "puzzle-5d-play-inspector.grip.direction.vec3",
                    value: puzzle5dPlayInspectorVec3Value(grip["3d"]!.direction),
                    onChange: puzzle5dPlayCmd("patchPuzzle5dGrips", { gripFullIds: [gripFull], field: "direction" satisfies Puzzle5dGripPatchField }),
                  },
                },
              ]
            : []),
        );
      }
      children.push({
        type: "section",
        id: "puzzle-5d-play-inspector.grips",
        label: "Grip",
        children: gripFields,
      });
    }
  } else if (selection.gripIds.length > 1) {
    const gripRows: { readonly gripFull: string; readonly grip: (typeof model.parts)[number]["grips"][number] }[] = [];
    for (const gripFull of selection.gripIds) {
      const parsed = parseGripFullId(gripFull);
      const hostPart = parsed ? model.parts.find((part) => part.id === parsed.partId) : undefined;
      const grip = hostPart?.grips.find((row) => row.id === parsed?.gripId);
      if (grip && parsed) gripRows.push({ gripFull, grip });
    }
    if (gripRows.length === 0) {
      children.push({
        type: "section",
        id: "puzzle-5d-play-inspector.grips",
        label: `Grips (${selection.gripIds.length})`,
        children: [{ type: "text", value: "Selected grips are not available in the model yet." }],
      });
    } else {
      const gripFullIds = gripRows.map((row) => row.gripFull);
      const gripKinds = gripRows.map((row) => row.grip.gripKind ?? "");
      const labels = gripRows.map((row) => row.grip["3d"]?.label ?? "");
      const angles = gripRows.map((row) => row.grip["2d"]?.angle ?? Number.NaN);
      const radii = gripRows.map((row) => row.grip["3d"]?.radius ?? row.grip["2d"]?.radius ?? Number.NaN);
      const positions = gripRows.map((row) => row.grip["3d"]?.position ?? null);
      const gripKindUniform = uiInspectorAllEqual(gripKinds);
      const labelUniform = uiInspectorAllEqual(labels);
      const angleUniform = uiInspectorAllEqual(angles);
      const radiusUniform = uiInspectorAllEqual(radii);
      const gripFields: UiNode[] = [
        {
          type: "field",
          id: "puzzle-5d-play-inspector.grip.kind",
          label: "Grip kind",
          child: {
            type: "select",
            id: "puzzle-5d-play-inspector.grip.kind.select",
            value: gripKindUniform ? (gripKinds[0] ?? "") : "",
            placeholder: gripKindUniform ? "kind" : "Mixed",
            items: puzzle5dPlayInspectorKindItems(bundle?.grips),
            onChange: puzzle5dPlayCmd("patchPuzzle5dGrips", { gripFullIds, field: "gripKind" satisfies Puzzle5dGripPatchField }),
          },
        },
        {
          type: "field",
          id: "puzzle-5d-play-inspector.grip.label",
          label: "Label",
          child: {
            type: "input",
            id: "puzzle-5d-play-inspector.grip.label.input",
            inputKind: "text",
            value: labelUniform ? (labels[0] ?? "") : "",
            placeholder: labelUniform ? undefined : "Mixed",
            onChange: puzzle5dPlayCmd("patchPuzzle5dGrips", { gripFullIds, field: "label" satisfies Puzzle5dGripPatchField }),
          },
        },
        {
          type: "field",
          id: "puzzle-5d-play-inspector.grip.angle",
          label: "Flat angle",
          child: {
            type: "numberStepper",
            id: "puzzle-5d-play-inspector.grip.angle.stepper",
            value: angleUniform ? angles[0]! : Number.NaN,
            step: 1,
            uniform: angleUniform,
            onAbsolute: puzzle5dPlayCmd("patchPuzzle5dGrips", { gripFullIds, field: "angle" satisfies Puzzle5dGripPatchField }),
          },
        },
        {
          type: "field",
          id: "puzzle-5d-play-inspector.grip.radius",
          label: "Radius",
          child: {
            type: "input",
            id: "puzzle-5d-play-inspector.grip.radius.input",
            inputKind: "number",
            value: radiusUniform ? String(radii[0]) : "",
            placeholder: radiusUniform ? undefined : "Mixed",
            onChange: puzzle5dPlayCmd("patchPuzzle5dGrips", { gripFullIds, field: "radius" satisfies Puzzle5dGripPatchField }),
          },
        },
        {
          type: "field",
          id: "puzzle-5d-play-inspector.grip.position",
          label: "Position",
          child: {
            type: "vec3",
            id: "puzzle-5d-play-inspector.grip.position.vec3",
            value: puzzle5dPlayInspectorUniformVec3(positions),
            onChange: puzzle5dPlayCmd("patchPuzzle5dGrips", { gripFullIds, field: "position" satisfies Puzzle5dGripPatchField }),
          },
        },
      ];
      children.push({
        type: "section",
        id: "puzzle-5d-play-inspector.grips",
        label: `Grips (${gripRows.length})`,
        children: gripFields,
      });
    }
  }
  return uiDeclarativeSectionsToTree(children);
}
//#endregion 🔖Puzzle5dPlayInspection

function puzzle5dPaletteDragDomainFromEncoded(encoded: string): "2d" | "3d" | null {
  try {
    const parsed = JSON.parse(encoded) as { readonly schema?: string };
    if (parsed.schema === "puzzle.2d.fixture") {
      return "2d";
    }
    if (parsed.schema === "puzzle.3d.fixture") {
      return "3d";
    }
  } catch {
    return null;
  }
  return null;
}

function puzzle5dPaletteDragDomainFromDragData(dragData: Record<string, string> | undefined): "2d" | "3d" | null {
  if (!dragData) {
    return null;
  }
  if (dragData[PUZZLE_2D_FIXTURE_DRAG_MIME]?.trim()) {
    return "2d";
  }
  if (dragData[FIXTURE_DRAG_MIME]?.trim()) {
    return "3d";
  }
  return null;
}

/** @emoji 🖱️ Tree drag controller for merged flat + volume palette rows in puzzle 5d play. */
export function puzzle5dFixturePaletteTreeDragController(dragDataByItemId: ReadonlyMap<string, Record<string, string>>) {
  const flatDragByItemId = new Map<string, Record<string, string>>();
  const volumeDragByItemId = new Map<string, Record<string, string>>();
  for (const [itemId, dragData] of dragDataByItemId) {
    const domain = puzzle5dPaletteDragDomainFromDragData(dragData);
    if (domain === "2d") {
      flatDragByItemId.set(itemId, dragData);
    } else if (domain === "3d") {
      volumeDragByItemId.set(itemId, dragData);
    }
  }
  const flatController = puzzle2dFixturePaletteTreeDragController(flatDragByItemId);
  const volumeController = puzzle3dFixturePaletteTreeDragController(volumeDragByItemId);
  const readEncoded = (dragData: Record<string, string>): string | undefined =>
    dragData[PUZZLE_2D_FIXTURE_DRAG_MIME]?.trim() || dragData[FIXTURE_DRAG_MIME]?.trim() || undefined;
  return {
    getDragData: ({ sourceItem }: { readonly sourceItem: { readonly id: string } }) => dragDataByItemId.get(sourceItem.id),
    pointerPaletteDrag: {
      readEncodedDragPayload: readEncoded,
      begin: (encoded: string) => {
        const domain = puzzle5dPaletteDragDomainFromEncoded(encoded);
        if (domain === "2d") {
          beginPuzzle2dFixturePalettePointerDrag(encoded);
          return;
        }
        if (domain === "3d") {
          beginPuzzle3dFixturePalettePointerDrag(encoded);
        }
      },
      cancel: () => {
        cancelPuzzle2dFixturePalettePointerDrag();
        cancelPuzzle3dFixturePalettePointerDrag();
      },
    },
    onDragStart: (ctx: { readonly sourceItem: { readonly id: string } }) => {
      const domain = puzzle5dPaletteDragDomainFromDragData(dragDataByItemId.get(ctx.sourceItem.id));
      if (domain === "2d") {
        flatController.onDragStart?.(ctx as never);
        return;
      }
      if (domain === "3d") {
        volumeController.onDragStart?.(ctx as never);
      }
    },
    onDragEnd: (ctx: { readonly sourceItem: { readonly id: string } }) => {
      const domain = puzzle5dPaletteDragDomainFromDragData(dragDataByItemId.get(ctx.sourceItem.id));
      if (domain === "2d") {
        flatController.onDragEnd?.(ctx as never);
        return;
      }
      if (domain === "3d") {
        volumeController.onDragEnd?.(ctx as never);
      }
    },
  };
}
//#endregion 🔖Puzzle5dPlayKinds

//#region 🔖Helpers
function puzzle5dPlayLodTierMenuLabel(tier: string): string {
  return tier.charAt(0).toUpperCase() + tier.slice(1);
}

function puzzle5dControllerFromContext(ctx: WindowBodyViewContext): Puzzle5dPlayShellController | undefined {
  return platformFromViewContext(ctx)?.getActiveApp()?.controller as Puzzle5dPlayShellController | undefined;
}

function sameCamera(a: CameraState | null, b: CameraState): boolean {
  return Boolean(a && a.x === b.x && a.y === b.y && a.zoom === b.zoom);
}
//#endregion 🔖Helpers


//#region 🔖LiveForceGraph
import {
  puzzle2dApplyLiveForceGraphLayoutTick,
  type Puzzle2dLiveForceGraphDragState,
  type Puzzle2dRedrawLayoutOptions,
} from "../../../2d/react/index.tsx";

const FIVE_D_LIVE_FORCE_ITERS_PER_FRAME = 24;

/** @emoji 🧷 Carries settled flat centers and camera forward when live force topology gains nodes (e.g. kit VFS unfold). */
export function mergeLiveForceGraphTopologyModel(incoming: Puzzle5dModel, existing: Puzzle5dModel): Puzzle5dModel {
  const existingParts = new Map(existing.parts.map((part) => [part.id, part] as const));
  const childOrdinal = new Map<string, number>();
  const parentByChild = new Map<string, string>();
  for (const fastener of incoming.fasteners) {
    if (!parentByChild.has(fastener.target)) {
      parentByChild.set(fastener.target, fastener.source);
    }
  }
  const parts = incoming.parts.map((part) => {
    const prev = existingParts.get(part.id);
    if (prev?.["2d"] && part["2d"]) {
      return { ...part, "2d": { ...part["2d"], x: prev["2d"].x, y: prev["2d"].y } };
    }
    if (!part["2d"]) {
      return part;
    }
    const parentId = parentByChild.get(part.id);
    const parent = parentId ? existingParts.get(parentId) : undefined;
    if (!parent?.["2d"]) {
      return part;
    }
    const ordinal = childOrdinal.get(parentId) ?? 0;
    childOrdinal.set(parentId, ordinal + 1);
    const angle = -Math.PI / 2 + ordinal * (Math.PI / 6);
    const radius = 96;
    return {
      ...part,
      "2d": {
        ...part["2d"],
        x: parent["2d"].x + Math.cos(angle) * radius,
        y: parent["2d"].y + Math.sin(angle) * radius,
      },
    };
  });
  return { ...incoming, parts, camera2d: { ...existing.camera2d }, camera3d: { ...existing.camera3d } };
}

/** @emoji 🕸️ Applies one WASM force-graph tick to a puzzle 5d store snapshot (same path as WIRES play). */
export function fiveDApplyLiveForceGraphStep(store: Puzzle5dStore, _instanceId: string, drag?: Puzzle2dLiveForceGraphDragState): void {
  const fixture = project2d(store.read());
  if (fixture.nodes.length === 0) {
    return;
  }
  const laid = puzzle2dApplyLiveForceGraphLayoutTick(
    fixture,
    {
      forceGraph: {
        gravity: 0,
        idealEdgeLength: 64,
        iterations: FIVE_D_LIVE_FORCE_ITERS_PER_FRAME,
        repulsionStrength: 80,
      },
      mode: "force-graph",
      redrawHandlesAfter: false,
    } satisfies Puzzle2dRedrawLayoutOptions,
    drag,
  );
  store.applyPart2dCenters(new Map(laid.nodes.map((node) => [node.id, { x: node.x, y: node.y }])));
}
//#endregion 🔖LiveForceGraph

//#region 🔖Controller
export interface Puzzle5dPlaySnapshot {
  readonly model: Puzzle5dModel;
  readonly selection: Puzzle5dSelectionSnapshot;
  readonly manifestLabel: string | undefined;
  readonly fixture2d: Puzzle2dFixture | null;
  readonly fixture3d: Puzzle3dFixture | null;
  readonly selected2d: ReadonlySet<string>;
  readonly camera2d: CameraState | null;
  readonly camera3d: CameraState | null;
  readonly selected3d: string | null;
  readonly gumballConfig: GumballConfig;
  readonly lod3dTag: number;
  readonly lod2dTag: Puzzle2dDrawLodKind;
  readonly lod2dProps: ReturnType<typeof puzzle2dLodCanvasProps>;
  readonly lod3dProps: ReturnType<typeof puzzle3dLodCanvasProps>;
  readonly automaticLod3d: boolean;
  readonly depthVariableLod3d: boolean;
  readonly lod3dSlider: number;
  readonly sharedKinds: ReturnType<typeof sharedKindsFromMetas>;
  readonly kindCatalogs: Puzzle5dKindCatalogBundle | undefined;
  readonly connect2d: number;
  readonly connect3d: number;
  readonly proximity2d: number;
  readonly proximity3d: number;
  readonly activeTool: Puzzle5dActiveTool;
  readonly suggestionOffset: number;
  readonly brushOverlapBudget: number;
  readonly fillCount: number;
  readonly fillBuildDone: boolean;
  readonly selectionMethod: Puzzle2dSelectionMethod;
  readonly selectionMode: Puzzle2dSelectionMode;
}

export const PUZZLE_5D_PLAY_STORE_ID = "puzzle-5d";

/** @emoji 🔗 Adapts {@link Puzzle5dStore} to {@link Store} for controller-owned registration. */
export class Puzzle5dStoreBridge extends Store<Puzzle5dStoreSnapshot> {
  private detach?: () => void;

  constructor(readonly inner: Puzzle5dStore) {
    super();
    this.detach = inner.subscribe(() => this.notify());
  }

  override getSnapshot(): Puzzle5dStoreSnapshot {
    return this.inner.getSnapshot();
  }

  override dispose(): void {
    this.detach?.();
    super.dispose();
  }
}

function puzzle5d2dPointerKey(id: string): string {
  return `puzzle5d:2d:${id}`;
}

function puzzle5d3dPointerKey(id: string): string {
  return `puzzle5d:3d:${id}`;
}

function puzzle5dSelected2dFromPointerKeys(keys: readonly string[]): ReadonlySet<string> {
  return new Set(keys.filter((key) => key.startsWith("puzzle5d:2d:")).map((key) => key.slice("puzzle5d:2d:".length)));
}

function puzzle5dSelected3dFromPointerKeys(keys: readonly string[]): string | null {
  const key = keys.find((entry) => entry.startsWith("puzzle5d:3d:"));
  return key ? key.slice("puzzle5d:3d:".length) : null;
}


/** @emoji 🎛 Puzzle 5d play shell controller shared by declarative 2d and 3d windows. */
export class Puzzle5dPlayShellController extends Controller implements PlaygroundExampleHost {
  readonly mainMode = new ModeRuntime("main", "Edit", undefined);
  private activeExampleId = playgroundResolvedExampleId(PUZZLE_5D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
  readonly puzzle5dStore: Puzzle5dStore = createStore(puzzle5dPlayEmptyModel());
  readonly puzzle5dStoreBridge: Puzzle5dStoreBridge;
  private readonly modelDocStore = new DocumentVcsStore<Puzzle5dModel, Puzzle5dModelEditOp>({
    envelope: createDocumentVcsEnvelope("puzzle.5d", "puzzle5d-play", puzzle5dPlayEmptyModel()),
    applyOp: applyPuzzle5dModelEditOp,
    backwardsOp: backwardsPuzzle5dModelEditOp,
    diffOp: diffPuzzle5dModelEditOp,
  });
  private gumballConfig: GumballConfig = { ...PUZZLE_3D_GUMBALL_CONFIG };
  private camera2d: CameraState | null = { ...this.puzzle5dStore.read().camera2d };
  private camera3d: CameraState | null = { ...this.puzzle5dStore.read().camera3d };
  private lod3dTag = DEFAULT_MANUAL_LOD;
  private automaticLod3d = true;
  private depthVariableLod3d = false;
  private manualLod3d = DEFAULT_MANUAL_LOD;
  private lod3dSlider = sliderValueFromLod(DEFAULT_MANUAL_LOD);
  private lod2dTag: Puzzle2dDrawLodKind = "normal";
  private lod2dMode: Puzzle2dLodModeKind = PUZZLE_2D_LOD_MODE_AUTOMATIC;
  private connect2d = 0;
  private connect3d = 0;
  private proximity2d = 0;
  private proximity3d = 0;
  private engagementInputByWindow: Record<string, string> = {
    [PUZZLE_5D_PLAY_2D_WINDOW_ID]: "",
    [PUZZLE_5D_PLAY_3D_WINDOW_ID]: "",
  };
  private hostBridge: Puzzle5dPlayHostBridge | null = null;
  private activeTool: Puzzle5dActiveTool = "select";
  private suggestionOffset = DEFAULT_PUZZLE_2D_SUGGESTION_OFFSET_PX;
  private brushOverlapBudget = DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET;
  private brushEngagementPossibles: { readonly id: string; readonly label: string }[] = [];
  private puzzle2dSelectionMethod: Puzzle2dSelectionMethod = "rectangle";
  private puzzle2dSelectionMode: Puzzle2dSelectionMode = "default";
  private puzzle2dSelectionTargets: Puzzle2dSelectionTargets = { nodes: true, edges: true, handles: true };
  private puzzle2dGridSnapEnabled = true;
  private puzzle2dRedrawPlaying = false;
  private readonly jackBridge = new JackHoverBridge();
  private jackEngagementInput = "";
  private readonly snapshotListeners = new Set<() => void>();

  private lastStoreShellModel: Puzzle5dModel | null = null;
  private lastStoreShellSelectionKey = "";
  private lastStoreShellFillCount = 0;
  get selected2d(): ReadonlySet<string> {
    return puzzle5dSelected2dFromPointerKeys(this.pointerFocus.getSnapshot().selection);
  }

  get selected3d(): string | null {
    return puzzle5dSelected3dFromPointerKeys(this.pointerFocus.getSnapshot().selection);
  }

  private setPointerFocus5dSelection(selected2d: ReadonlySet<string>, selected3d: string | null): void {
    const keys = [...selected2d].map(puzzle5d2dPointerKey);
    if (selected3d) keys.push(puzzle5d3dPointerKey(selected3d));
    this.pointerFocus.setSelection(keys);
  }

  constructor(commandBus: CommandBus, hostNotify: () => void) {
    super(PUZZLE_5D_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.puzzle5dStoreBridge = new Puzzle5dStoreBridge(this.puzzle5dStore);
    this.provideStore(PUZZLE_5D_PLAY_STORE_ID, this.puzzle5dStoreBridge);
    this.puzzle5dStore.subscribe(() => this.notifyStoreShellIfNeeded());
    this.jackBridge.setJackQueryText(PUZZLE_5D_PLAY_DEFAULT_JACK_QUERY);
    this.jackBridge.setFixtureJson(puzzle5dPlayJackBoardJson(puzzle5dPlayEmptyModel()));
    this.jackBridge.bindPointerFocus(this.pointerFocus);
    this.rebuildShellMode();
    void this.loadFixtureById(this.activeExampleId);
  }

  private notifyStoreShellIfNeeded(): void {
    const snap = this.puzzle5dStore.getSnapshot();
    const selectionKey = `${snap.selection.partIds.join("\u0001")}\u0002${snap.selection.gripIds.join("\u0001")}`;
    if (
      this.lastStoreShellModel === snap.model &&
      this.lastStoreShellSelectionKey === selectionKey &&
      this.lastStoreShellFillCount === snap.fillCount &&
      this.lastStoreShellFillBuildDone === snap.fillBuildDone
    ) {
      return;
    }
    this.lastStoreShellModel = snap.model;
    this.lastStoreShellSelectionKey = selectionKey;
    this.lastStoreShellFillCount = snap.fillCount;
    this.lastStoreShellFillBuildDone = snap.fillBuildDone;
    this.emit();
  }

  setHostBridge(bridge: Puzzle5dPlayHostBridge | null): void {
    this.hostBridge = bridge;
    this.rebuildShellMode();
  }

  getActiveTool(): Puzzle5dActiveTool {
    return this.activeTool;
  }

  getSuggestionOffset(): number {
    return this.suggestionOffset;
  }

  setBrushEngagementPossibles(rows: readonly { readonly id: string; readonly label: string }[]): void {
    const next = [...rows];
    if (next.length === this.brushEngagementPossibles.length && next.every((row, index) => row.id === this.brushEngagementPossibles[index]?.id)) {
      return;
    }
    this.brushEngagementPossibles = next;
    this.rebuildShellMode();
    this.emit();
  }

  private toolbarState(): Puzzle2dPlayToolbarState {
    return (
      this.hostBridge?.getToolbarState() ?? {
        puzzle2dActiveTool: this.activeTool,
        puzzle2dSuggestionOffset: this.suggestionOffset,
        puzzle2dSelectionMethod: this.puzzle2dSelectionMethod,
        puzzle2dSelectionMode: this.puzzle2dSelectionMode,
        puzzle2dSelectionTargets: this.puzzle2dSelectionTargets,
        puzzle2dGridSnapEnabled: this.puzzle2dGridSnapEnabled,
        puzzle2dRedrawPlaying: this.puzzle2dRedrawPlaying,
      }
    );
  }

  private setPlayActiveTool(tool: Puzzle5dActiveTool): void {
    const prev = this.activeTool;
    if (prev === tool) return;
    this.activeTool = tool;
    if (tool !== "brush") {
      this.brushEngagementPossibles = [];
    }
    this.hostBridge?.runHostCommand("setActiveTool", { tool, prevTool: prev });
    this.rebuildShellMode();
    this.emit();
  }

  private rebuildShellMode(): void {
    const relocateTools: ToolLeaf[] = PUZZLE_3D_GUMBALL_GROUPS.map(({ key, label, iconId }, order) => ({
      id: `puzzle5d.gumball.${key}`,
      kind: "toggle" as const,
      iconId,
      text: label,
      order: order + 100,
      pressed: this.gumballConfig[key] !== false,
      controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID,
      command: "setGumballConfigToggle",
      args: { key },
    }));
    const flatTools = buildPuzzle2dPlayToolbarTools(this.toolbarState(), PUZZLE_5D_PLAY_CONTROLLER_ID);
    this.mainMode.tools = flatTools.map((node) => {
      if (node.kind === "collection" && node.id === "actions") {
        return { ...node, children: [...node.children, ...relocateTools] };
      }
      return node;
    }) satisfies ToolNode[];
    this.mainMode.windowKinds = this.getWindowKinds();
  }

  private suggestionMeasuresGroup(windowId: string): WindowMeasure {
    return {
      kind: "group",
      id: `${windowId}-suggestion`,
      label: "Suggestion",
      children: [
        {
          kind: "slider",
          id: `${windowId}-suggestion-offset`,
          label: "Offset",
          value: this.suggestionOffset,
          min: PUZZLE_5D_SUGGESTION_OFFSET_MIN,
          max: PUZZLE_5D_SUGGESTION_OFFSET_MAX,
          step: PUZZLE_5D_SUGGESTION_OFFSET_STEP,
          onChange: puzzle5dPlayCmd("setSuggestionOffset"),
        },
      ],
    };
  }

  private brushMeasuresGroup(windowId: string): WindowMeasure {
    return {
      kind: "group",
      id: `${windowId}-brush`,
      label: "Brush",
      children: [
        {
          kind: "slider",
          id: `${windowId}-brush-overlap-budget`,
          label: `Overlap ${this.brushOverlapBudget.toFixed(2)} m³`,
          value: this.brushOverlapBudget,
          min: 0,
          max: BRUSH_PLACEMENT_OVERLAP_BUDGET_MAX,
          step: BRUSH_PLACEMENT_OVERLAP_BUDGET_STEP,
          onChange: puzzle5dPlayCmd("setBrushOverlapBudget"),
        },
      ],
    };
  }

  private lod2dMeasure(): WindowMeasure {
    return {
      kind: "select",
      id: `${PUZZLE_5D_PLAY_2D_WINDOW_ID}-lod`,
      label: "LOD",
      value: this.lod2dMode,
      items: [{ id: "automatic", label: puzzle2dLodAutomaticSelectLabel(this.lod2dTag), value: PUZZLE_2D_LOD_MODE_AUTOMATIC }, ...PUZZLE_5D_PLAY_LOD_TIERS_2D.map((tier) => ({ id: tier, label: puzzle5dPlayLodTierMenuLabel(tier), value: tier }))],
      onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "set2dLodMode" },
    };
  }

  private lod3dMeasures(): readonly WindowMeasure[] {
    return [
      {
        kind: "toggle",
        id: `${PUZZLE_5D_PLAY_3D_WINDOW_ID}-auto`,
        iconId: "zoom-in",
        text: "Auto zoom",
        pressed: this.automaticLod3d,
        onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "set3dAutoLod" },
      },
      {
        kind: "toggle",
        id: `${PUZZLE_5D_PLAY_3D_WINDOW_ID}-depth`,
        iconId: "layers",
        text: "Depth-variable",
        pressed: this.depthVariableLod3d,
        onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "set3dDepthLod" },
      },
      {
        kind: "slider",
        id: `${PUZZLE_5D_PLAY_3D_WINDOW_ID}-lod`,
        label: formatLod(this.lod3dTag),
        value: this.lod3dSlider,
        min: PUZZLE_3D_LOD_SLIDER_MIN,
        max: PUZZLE_3D_LOD_SLIDER_MAX,
        step: 1,
        onChange: { controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, command: "set3dManualLod" },
      },
    ];
  }

  private windowEngagementFor(windowId: string): WindowEngagement {
    const staticToolPossibles = [
      { id: PUZZLE_5D_ENGAGEMENT_TOOL_BRUSH_ID, label: "Brush", command: puzzle5dPlayCmd("engagementPossibleSelect", { windowId, possibleId: PUZZLE_5D_ENGAGEMENT_TOOL_BRUSH_ID }) },
      { id: PUZZLE_5D_ENGAGEMENT_TOOL_FILL_ID, label: "Fill", command: puzzle5dPlayCmd("engagementPossibleSelect", { windowId, possibleId: PUZZLE_5D_ENGAGEMENT_TOOL_FILL_ID }) },
      { id: PUZZLE_5D_ENGAGEMENT_TOOL_SELECT_ID, label: "Select", command: puzzle5dPlayCmd("engagementPossibleSelect", { windowId, possibleId: PUZZLE_5D_ENGAGEMENT_TOOL_SELECT_ID }) },
    ];
    const toolPossibles =
      this.activeTool === "brush" && this.brushEngagementPossibles.length > 0
        ? this.brushEngagementPossibles.map((row) => ({
            id: row.id,
            label: row.label,
            command: puzzle5dPlayCmd("engagementPossibleSelect", { windowId, possibleId: row.id }),
          }))
        : staticToolPossibles;
    const storeSnap = this.puzzle5dStore.getSnapshot();
    const fillSliderMax = storeSnap.fillBuildDone ? PUZZLE_5D_FILL_COUNT_MAX : Math.max(storeSnap.fillCount, 1);
    const control =
      this.activeTool === "fill"
        ? {
            kind: "slider" as const,
            id: "puzzle5d-fill-count",
            label: `Fill ${storeSnap.fillCount}`,
            value: Math.min(storeSnap.fillCount, fillSliderMax),
            min: 0,
            max: fillSliderMax,
            step: 1,
            onChange: puzzle5dPlayCmd("engagementControlChange", { windowId }),
          }
        : this.activeTool === "brush" && this.brushEngagementPossibles.length > 0
          ? this.brushPlacementEngagementControl(windowId)
          : undefined;
    return {
      sessionActive: this.activeTool === "brush" || this.activeTool === "fill",
      options: [
        { id: PUZZLE_5D_ENGAGEMENT_TOOL_SELECT_ID, label: "Select", pressed: this.activeTool === "select", command: puzzle5dPlayCmd("engagementPossibleSelect", { windowId, possibleId: PUZZLE_5D_ENGAGEMENT_TOOL_SELECT_ID }) },
        { id: PUZZLE_5D_ENGAGEMENT_TOOL_BRUSH_ID, label: "Brush", pressed: this.activeTool === "brush", command: puzzle5dPlayCmd("engagementPossibleSelect", { windowId, possibleId: PUZZLE_5D_ENGAGEMENT_TOOL_BRUSH_ID }) },
        { id: PUZZLE_5D_ENGAGEMENT_TOOL_FILL_ID, label: "Fill", pressed: this.activeTool === "fill", command: puzzle5dPlayCmd("engagementPossibleSelect", { windowId, possibleId: PUZZLE_5D_ENGAGEMENT_TOOL_FILL_ID }) },
      ],
      input: {
        id: "engagement-input",
        value: this.engagementInputByWindow[windowId] ?? "",
        placeholder: this.activeTool === "fill" ? "Fill" : this.activeTool === "brush" ? "Brush" : "Command",
        onChange: puzzle5dPlayCmd("engagementInput", { windowId }),
        onSubmit: puzzle5dPlayCmd("engagementSubmit", { windowId }),
        onAbort: puzzle5dPlayCmd("engagementAbort", { windowId }),
      },
      control,
      possibleEngagements: toolPossibles,
    };
  }

  private brushPlacementEngagementControl(windowId: string): WindowEngagementControl {
    const candidates = this.brushEngagementPossibles;
    const selectedValue = candidates[0]!.id;
    const selectCmd = puzzle5dPlayCmd("engagementControlSelect", { windowId });
    if (candidates.length <= 6) {
      return {
        kind: "toggleGroup",
        id: "puzzle5d-brush-placement",
        label: "Placement",
        value: selectedValue,
        options: candidates.map((row) => ({ id: row.id, label: row.label })),
        onSelect: selectCmd,
      };
    }
    return {
      kind: "select",
      id: "puzzle5d-brush-placement",
      label: "Placement",
      value: selectedValue,
      placeholder: "Placement",
      items: candidates.map((row) => ({ id: row.id, value: row.id, label: row.label })),
      onChange: selectCmd,
    };
  }

  getWindowKinds(): readonly WindowKindRuntime[] {
    const windowKinds = [
      new WindowKindRuntime(
        PUZZLE_5D_PLAY_2D_WINDOW_ID,
        PUZZLE_5D_PLAY_2D_WINDOW_LABEL,
        PUZZLE_5D_PLAY_2D_BODY_KEY,
        undefined,
        [
          this.lod2dMeasure(),
          this.suggestionMeasuresGroup(PUZZLE_5D_PLAY_2D_WINDOW_ID),
          this.brushMeasuresGroup(PUZZLE_5D_PLAY_2D_WINDOW_ID),
        ],
        this.windowEngagementFor(PUZZLE_5D_PLAY_2D_WINDOW_ID),
      ),
      new WindowKindRuntime(
        PUZZLE_5D_PLAY_3D_WINDOW_ID,
        PUZZLE_5D_PLAY_3D_WINDOW_LABEL,
        PUZZLE_5D_PLAY_3D_BODY_KEY,
        undefined,
        [
          { kind: "group", id: `${PUZZLE_5D_PLAY_3D_WINDOW_ID}-lod`, label: "LOD", children: this.lod3dMeasures() },
          this.suggestionMeasuresGroup(PUZZLE_5D_PLAY_3D_WINDOW_ID),
          this.brushMeasuresGroup(PUZZLE_5D_PLAY_3D_WINDOW_ID),
        ],
        this.windowEngagementFor(PUZZLE_5D_PLAY_3D_WINDOW_ID),
      ),
      new WindowKindRuntime(PUZZLE_5D_PLAY_JACK_WINDOW_ID, PUZZLE_5D_PLAY_JACK_WINDOW_LABEL, PUZZLE_5D_PLAY_JACK_BODY_KEY, undefined, undefined, createJackPlayWindowEngagement(PUZZLE_5D_PLAY_JACK_WINDOW_ID, PUZZLE_5D_PLAY_CONTROLLER_ID, this.jackEngagementInput)),
    ];
    for (const windowKind of windowKinds) {
      enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Puzzle 5D play window "${windowKind.id}"`);
    }
    return windowKinds;
  }

  getDocumentVcsStore(): DocumentVcsStore<Puzzle5dModel, Puzzle5dModelEditOp> {
    return this.modelDocStore;
  }

  private applyModelEdit(op: Puzzle5dModelEditOp): void {
    recordProjectionChange(this.modelDocStore, [op]);
    this.puzzle5dStore.replaceModel(this.modelDocStore.projection());
  }

  private commitModel(model: Puzzle5dModel): void {
    this.applyModelEdit({ op: "setDocument", document: model });
    this.syncJackFixtureJson();
  }

  getExampleCatalog(): PlaygroundExampleCatalog | null {
    if (isPlaygroundExampleLocked()) return null;
    return { activeExampleId: this.activeExampleId, options: PUZZLE_5D_PLAY_EXAMPLE_OPTIONS };
  }

  private async loadFixtureById(fixtureId: string): Promise<void> {
    let model = isPlaygroundNoExampleId(fixtureId) ? puzzle5dPlayEmptyModel() : await fetchPuzzle5dPlayModel(fixtureId);
    if (!model) return;
    if (!isPlaygroundNoExampleId(fixtureId)) {
      const camera2d =
        fixtureId === PUZZLE_5D_PLAY_EXAMPLE_CONCRETE_FOREST_ID || fixtureId === PUZZLE_5D_PLAY_EXAMPLE_NAKAGIN_ID
          ? puzzle2dPlayViewportCameraForFixtureId(fixtureId)
          : puzzle2dPlayViewportCameraFromFixture(project2d(model));
      model = { ...model, camera2d };
    }
    this.puzzle5dStore.replaceModel(model);
    this.commitModel(model);
    this.syncJackFixtureJson();
    this.setPointerFocus5dSelection(new Set(), null);
    this.activeTool = "select";
    this.brushEngagementPossibles = [];
    this.hostBridge?.runHostCommand("setActiveTool", { tool: "select", prevTool: "select" });
    const snap = this.puzzle5dStore.read();
    this.camera2d = { ...snap.camera2d };
    this.camera3d = { ...snap.camera3d };
    this.rebuildShellMode();
    this.emit();
  }

  private syncJackFixtureJson(): void {
    this.jackBridge.setFixtureJson(puzzle5dPlayJackBoardJson(this.puzzle5dStore.read()));
  }

  subscribeSnapshot(listener: () => void): () => void {
    this.snapshotListeners.add(listener);
    const unsubJack = this.jackBridge.subscribe(listener);
    return () => {
      this.snapshotListeners.delete(listener);
      unsubJack();
    };
  }

  getJackQueryText(): string {
    return this.jackBridge.getJackQueryText();
  }

  getWriterDocumentJack(): WriterDocument {
    return createWriterDocument({ id: "puzzle-5d-jack", languageId: "jack", text: this.jackBridge.getJackQueryText() });
  }

  getJackHoverOccurrences(): readonly { readonly start: number; readonly end: number }[] {
    return this.jackBridge.getJackHoverOccurrences();
  }

  getJackSelectOccurrences(): readonly { readonly start: number; readonly end: number }[] {
    return this.jackBridge.getJackSelectOccurrences();
  }

  getHoverEpoch(): number {
    return this.jackBridge.getHoverEpoch();
  }

  getSelectEpoch(): number {
    return this.jackBridge.getSelectEpoch();
  }

  getGraphHighlightedNodeIds(): readonly string[] {
    return this.jackBridge.getGraphHoveredNodeIds();
  }

  private notifySnapshot(): void {
    for (const listener of this.snapshotListeners) {
      listener();
    }
  }

  private syncJackGraphSelect(): void {
    const ids = [...this.selected2d];
    if (this.selected3d) {
      ids.push(this.selected3d);
    }
    this.jackBridge.setGraphSelect([...new Set(ids)]);
    this.notifySnapshot();
  }

  override run(command: string, args?: unknown): void {
    if (command === "jackEngagementInput") {
      const value = (args as { value?: string }).value;
      if (typeof value === "string" && value !== this.jackEngagementInput) {
        this.jackEngagementInput = value;
        this.rebuildShellMode();
        this.emit();
      }
      return;
    }
    if (command === "setJackQuery") {
      const text = (args as { text?: string }).text;
      if (typeof text === "string") {
        this.jackBridge.setJackQueryText(text);
        this.notifySnapshot();
        this.emit();
      }
      return;
    }
    if (command === "setJackHover") {
      this.jackBridge.setJackHover((args as { offset?: number | null }).offset ?? null);
      this.notifySnapshot();
      this.emit();
      return;
    }
    if (command === "setJackSelect") {
      this.jackBridge.setJackSelect((args as { start: number; end: number } | null) ?? null);
      this.notifySnapshot();
      this.emit();
      return;
    }
    if (command === "setGraphHover") {
      this.jackBridge.setGraphHover((args as { id?: string | null }).id ?? null);
      this.notifySnapshot();
      this.emit();
      return;
    }
    if (command === "setGraphSelect") {
      const ids = (args as { ids?: readonly string[] }).ids ?? [];
      this.jackBridge.setGraphSelect(ids);
      this.notifySnapshot();
      this.emit();
      return;
    }
    if (command === "runJackQuery") {
      runJackOnBoardFixture(puzzle5dPlayJackBoardJson(this.puzzle5dStore.read()), this.jackBridge.getJackQueryText());
      this.notifySnapshot();
      this.emit();
      return;
    }
    let changed = true;
    switch (command) {
      case "setActiveExample": {
        if (isPlaygroundExampleLocked()) return;
        const fixtureId = (args as { fixtureId?: string }).fixtureId ?? "";
        const nextId = isPlaygroundNoExampleId(fixtureId) ? PLAYGROUND_NO_EXAMPLE_ID : fixtureId;
        if (nextId === this.activeExampleId) {
          changed = false;
          break;
        }
        this.activeExampleId = nextId;
        void this.loadFixtureById(nextId);
        changed = false;
        break;
      }
      case "set2dLodMode": {
        const value = (args as { value?: string }).value;
        if ((value === PUZZLE_2D_LOD_MODE_AUTOMATIC || (typeof value === "string" && isPuzzle2dDrawLodKind(value))) && this.lod2dMode !== value) this.lod2dMode = value as Puzzle2dLodModeKind;
        else changed = false;
        break;
      }
      case "set3dAutoLod": {
        const pressed = (args as { pressed?: boolean }).pressed;
        if (typeof pressed === "boolean" && this.automaticLod3d !== pressed) this.automaticLod3d = pressed;
        else changed = false;
        break;
      }
      case "set3dDepthLod": {
        const pressed = (args as { pressed?: boolean }).pressed;
        if (typeof pressed === "boolean" && this.depthVariableLod3d !== pressed) this.depthVariableLod3d = pressed;
        else changed = false;
        break;
      }
      case "set3dManualLod": {
        const value = (args as { value?: number }).value;
        if (typeof value === "number" && Number.isFinite(value)) {
          this.lod3dSlider = value;
          this.manualLod3d = lodFromSliderValue(value);
        } else changed = false;
        break;
      }
      case "set2dLodTag": {
        const lod = (args as { lod: Puzzle2dDrawLodKind }).lod;
        if (this.lod2dTag !== lod) this.lod2dTag = lod;
        else changed = false;
        break;
      }
      case "set3dLodTag": {
        const lod = (args as { lod: number }).lod;
        if (typeof lod === "number" && Number.isFinite(lod) && lod > 0) {
          this.lod3dTag = lod;
        }
        changed = false;
        break;
      }
      case "set2dSelection": {
        const ids = (args as { ids: readonly string[] }).ids;
        if (ids.length !== this.selected2d.size || ids.some((id) => !this.selected2d.has(id))) this.setPointerFocus5dSelection(new Set(ids), this.selected3d);
        else changed = false;
        this.syncJackGraphSelect();
        break;
      }
      case "hierarchySelectPart": {
        const partId = (args as { partId?: string }).partId;
        if (typeof partId === "string") {
          this.puzzle5dStore.setSelection({ partIds: [partId], gripIds: [] });
          this.setPointerFocus5dSelection(new Set([partId]), partId);
          this.syncJackGraphSelect();
        } else {
          changed = false;
        }
        break;
      }
      case "hierarchySelectGrip": {
        const gripFullId = (args as { gripFullId?: string }).gripFullId;
        if (typeof gripFullId === "string") {
          this.puzzle5dStore.setSelection({ partIds: [], gripIds: [gripFullId] });
          this.setPointerFocus5dSelection(new Set([gripFullId]), null);
          this.syncJackGraphSelect();
        } else {
          changed = false;
        }
        break;
      }
      case "hierarchySelectFastener": {
        const fastenerId = (args as { fastenerId?: string }).fastenerId;
        if (typeof fastenerId === "string") {
          this.puzzle5dStore.setSelection({ partIds: [], gripIds: [] });
          this.setPointerFocus5dSelection(new Set([fastenerId]), null);
          this.syncJackGraphSelect();
        } else {
          changed = false;
        }
        break;
      }
      case "set3dSelection": {
        const selected = (args as { objectIds: readonly string[] }).objectIds[0] ?? null;
        if (this.selected3d !== selected) this.setPointerFocus5dSelection(this.selected2d, selected);
        else changed = false;
        this.syncJackGraphSelect();
        break;
      }
      case "set2dCamera": {
        const camera = (args as { camera: CameraState }).camera;
        if (!sameCamera(this.camera2d, camera)) this.camera2d = { ...camera };
        else changed = false;
        break;
      }
      case "set3dCamera": {
        const camera = (args as { camera: CameraState }).camera;
        if (!sameCamera(this.camera3d, camera)) this.camera3d = { ...camera };
        else changed = false;
        break;
      }
      case "setGumballConfigToggle": {
        const key = (args as { key?: Puzzle3dGumballGroupKey }).key;
        if (!key || !PUZZLE_3D_GUMBALL_GROUPS.some((row) => row.key === key)) {
          changed = false;
          break;
        }
        this.gumballConfig = { ...this.gumballConfig, [key]: this.gumballConfig[key] === false };
        this.rebuildShellMode();
        break;
      }
      case "note2dConnect":
        this.connect2d += 1;
        break;
      case "note3dConnect":
        this.connect3d += 1;
        break;
      case "note2dProximity":
        this.proximity2d += 1;
        break;
      case "note3dProximity":
        this.proximity3d += 1;
        break;
      case "setActiveTool": {
        const tool = (args as { tool?: Puzzle5dActiveTool }).tool;
        if (tool === "select" || tool === "brush" || tool === "fill") {
          this.setPlayActiveTool(tool);
        } else {
          changed = false;
        }
        break;
      }
      case "addBrushPart": {
        const placement = args as Puzzle5dBrushPlacement;
        if (!placement?.partKind) {
          changed = false;
          break;
        }
        const result = applyBrushPlacementToModel(this.modelDocStore.projection(), placement);
        if (result.kind === "placed") {
          this.applyModelEdit({ op: "applyBrushPlacement", placement });
          this.puzzle5dStore.setSelection({ partIds: [result.partId], gripIds: [] });
          this.emit();
        }
        changed = false;
        break;
      }
      case "deleteSelection": {
        const selection = this.puzzle5dStore.read().selection;
        if (selection.partIds.length === 0 && selection.gripIds.length === 0) {
          changed = false;
          break;
        }
        this.applyModelEdit({ op: "deletePartsAndGrips", partIds: selection.partIds, gripIds: selection.gripIds });
        this.puzzle5dStore.setSelection({ partIds: [], gripIds: [] });
        this.setPointerFocus5dSelection(new Set(), null);
        this.emit();
        changed = false;
        break;
      }
      case "patchPuzzle5dParts": {
        const { partIds, field, value } = args as {
          partIds?: readonly string[];
          field?: Puzzle5dPartPatchField;
          value?: unknown;
        };
        if (!partIds?.length || !field) {
          changed = false;
          break;
        }
        this.applyModelEdit({ op: "patchParts", partIds, field, value });
        changed = false;
        break;
      }
      case "patchPuzzle5dGrips": {
        const { gripFullIds, field, value } = args as {
          gripFullIds?: readonly string[];
          field?: Puzzle5dGripPatchField;
          value?: unknown;
        };
        if (!gripFullIds?.length || !field) {
          changed = false;
          break;
        }
        this.applyModelEdit({ op: "patchGrips", gripFullIds, field, value });
        changed = false;
        break;
      }
      case "setFillCount": {
        const count = Number((args as { count?: number }).count);
        if (!Number.isFinite(count)) {
          changed = false;
          break;
        }
        const session = this.puzzle5dStore.getFillSession();
        if (!session) {
          changed = false;
          break;
        }
        this.applyModelEdit({
          op: "applyFillPrefix",
          baseModel: session.baseModel,
          placements: session.sequence,
          count,
        });
        this.puzzle5dStore.syncFillCount(count);
        this.rebuildShellMode();
        this.emit();
        changed = false;
        break;
      }
      case "setSuggestionOffset": {
        const distance = Number((args as { value?: number }).value);
        if (Number.isFinite(distance)) {
          this.suggestionOffset = Math.max(PUZZLE_5D_SUGGESTION_OFFSET_MIN, Math.min(PUZZLE_5D_SUGGESTION_OFFSET_MAX, distance));
          this.hostBridge?.runHostCommand("setSuggestionOffset", { distance: this.suggestionOffset });
        } else {
          changed = false;
        }
        break;
      }
      case "setBrushOverlapBudget": {
        const value = Number((args as { value?: number }).value);
        if (Number.isFinite(value)) {
          this.brushOverlapBudget = Math.max(0, Math.min(BRUSH_PLACEMENT_OVERLAP_BUDGET_MAX, value));
          this.hostBridge?.runHostCommand("setBrushOverlapBudget", { value: this.brushOverlapBudget });
        } else {
          changed = false;
        }
        break;
      }
      case "pickBrushCandidate": {
        this.hostBridge?.runHostCommand(command, args);
        changed = false;
        break;
      }
      case "engagementPossibleSelect": {
        const { possibleId, windowId } = args as { possibleId?: string; windowId?: string };
        if (!possibleId) {
          changed = false;
          break;
        }
        if (possibleId === PUZZLE_5D_ENGAGEMENT_TOOL_BRUSH_ID) {
          this.setPlayActiveTool("brush");
        } else if (possibleId === PUZZLE_5D_ENGAGEMENT_TOOL_FILL_ID) {
          this.setPlayActiveTool("fill");
        } else if (possibleId === PUZZLE_5D_ENGAGEMENT_TOOL_SELECT_ID) {
          this.setPlayActiveTool("select");
        } else if (possibleId.startsWith("puzzle5d.brush.") || possibleId.startsWith("puzzle2d.brush.") || possibleId.startsWith("puzzle3d.brush.")) {
          this.hostBridge?.runHostCommand(command, args);
        } else {
          this.hostBridge?.runHostCommand(command, args);
        }
        if (windowId && windowId in this.engagementInputByWindow) {
          this.engagementInputByWindow = { ...this.engagementInputByWindow, [windowId]: "" };
        }
        changed = false;
        break;
      }
      case "engagementControlChange": {
        if (this.activeTool === "fill") {
          const value = Number((args as { value?: number }).value);
          if (Number.isFinite(value)) {
            const session = this.puzzle5dStore.getFillSession();
            if (session) {
              this.applyModelEdit({
                op: "applyFillPrefix",
                baseModel: session.baseModel,
                placements: session.sequence,
                count: value,
              });
              this.puzzle5dStore.syncFillCount(value);
            }
            this.rebuildShellMode();
            this.emit();
          }
        } else {
          this.hostBridge?.runHostCommand(command, args);
        }
        changed = false;
        break;
      }
      case "engagementControlSelect":
      case "engagementControlCommit":
        this.hostBridge?.runHostCommand(command, args);
        changed = false;
        break;
      case "engagementInput": {
        const { windowId, value } = args as { windowId?: string; value?: string };
        if (!windowId || !(windowId in this.engagementInputByWindow)) {
          changed = false;
          break;
        }
        this.engagementInputByWindow = { ...this.engagementInputByWindow, [windowId]: String(value ?? "") };
        break;
      }
      case "engagementSubmit": {
        const { windowId, value } = args as { windowId?: string; value?: string };
        if (!windowId || !(windowId in this.engagementInputByWindow)) {
          changed = false;
          break;
        }
        const token = String(value ?? this.engagementInputByWindow[windowId] ?? "")
          .trim()
          .toLowerCase();
        if (token === "brush") this.setPlayActiveTool("brush");
        else if (token === "fill") this.setPlayActiveTool("fill");
        else if (token === "select") this.setPlayActiveTool("select");
        else this.hostBridge?.runHostCommand(command, args);
        this.engagementInputByWindow = { ...this.engagementInputByWindow, [windowId]: "" };
        changed = false;
        break;
      }
      case "engagementAbort": {
        const { windowId } = args as { windowId?: string };
        if (!windowId || !(windowId in this.engagementInputByWindow)) {
          changed = false;
          break;
        }
        if (this.activeTool === "brush" || this.activeTool === "fill") {
          this.setPlayActiveTool("select");
        }
        this.hostBridge?.runHostCommand(command, args);
        this.engagementInputByWindow = { ...this.engagementInputByWindow, [windowId]: "" };
        changed = false;
        break;
      }
      default:
        if (command === "setSelectionMethod" || command === "setSelectionMode" || command === "toggleSelectionTarget") {
          this.hostBridge?.runHostCommand(command, args);
          this.rebuildShellMode();
          this.emit();
        } else if (
          command === "clearSelection" ||
          command === "selectAllSelection" ||
          command === "toggleGridSnap" ||
          command === "appendCircle" ||
          command === "appendRectangle" ||
          command === "toggleRedrawPlaying" ||
          command === "redrawHandlesOnce" ||
          command === "setBrushKindWeights" ||
          command === "setNodeKindWeight" ||
          command === "setHandleKindWeight" ||
          command === "setObjectKindWeight" ||
          command === "setVortexKindWeight"
        ) {
          this.hostBridge?.runHostCommand(command, args);
        }
        changed = false;
        break;
    }
    if (changed) {
      this.rebuildShellMode();
      this.emit();
    }
  }

  getSnapshot(): Puzzle5dPlaySnapshot {
    const storeSnap = this.puzzle5dStore.getSnapshot();
    const model = storeSnap.model;
    const fixture2d = project2d(model);
    const fixture3d = project3d(model);
    const toolbar = this.toolbarState();
    return {
      model,
      selection: storeSnap.selection,
      manifestLabel: model.label,
      fixture2d,
      fixture3d,
      selected2d: this.selected2d,
      camera2d: this.camera2d,
      camera3d: this.camera3d,
      selected3d: this.selected3d,
      gumballConfig: this.gumballConfig,
      lod3dTag: this.lod3dTag,
      lod2dTag: this.lod2dTag,
      lod2dProps: puzzle2dLodCanvasProps(this.lod2dMode),
      lod3dProps: puzzle3dLodCanvasProps({
        automaticLod: this.automaticLod3d,
        depthVariableLod: this.depthVariableLod3d,
        manualLod: this.manualLod3d,
      }),
      automaticLod3d: this.automaticLod3d,
      depthVariableLod3d: this.depthVariableLod3d,
      lod3dSlider: this.lod3dSlider,
      sharedKinds: sharedKindsFromMetas({ meta2d: fixture2d.meta, meta3d: fixture3d.meta }),
      kindCatalogs: model.kindCatalogs,
      connect2d: this.connect2d,
      connect3d: this.connect3d,
      proximity2d: this.proximity2d,
      proximity3d: this.proximity3d,
      activeTool: this.activeTool,
      suggestionOffset: this.suggestionOffset,
      brushOverlapBudget: this.brushOverlapBudget,
      fillCount: storeSnap.fillCount,
      fillBuildDone: storeSnap.fillBuildDone,
      selectionMethod: toolbar.puzzle2dSelectionMethod,
      selectionMode: toolbar.puzzle2dSelectionMode,
    };
  }
}
//#endregion 🔖Controller

//#region 🔖Puzzle5dPlayRuntime
export function buildPuzzle5dPlayAppRuntime(controller: Puzzle5dPlayShellController): AppRuntime {
  const app = new AppRuntime(
    PUZZLE_5D_PLAY_APP_ID,
    "Puzzle 5D",
    undefined,
    controller,
    createDefaultLayout(
      [PUZZLE_5D_PLAY_2D_WINDOW_ID, PUZZLE_5D_PLAY_3D_WINDOW_ID, PUZZLE_5D_PLAY_JACK_WINDOW_ID],
      "row",
      [40, 40, 20],
      [PUZZLE_5D_PLAY_2D_WINDOW_LABEL, PUZZLE_5D_PLAY_3D_WINDOW_LABEL, PUZZLE_5D_PLAY_JACK_WINDOW_LABEL],
    ) as never,
    controller.getWindowKinds(),
  );
  app.defaultModeId = controller.mainMode.id;
  app.addMode(controller.mainMode);
  app.panelTabs = [
    { id: PUZZLE_5D_PLAY_HIERARCHY_TAB_ID, iconId: FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, panel: "workbench", order: 0, bodyKey: PUZZLE_5D_PLAY_HIERARCHY_BODY_KEY, label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL },
    { id: PUZZLE_5D_PLAY_KINDS_TAB_ID, iconId: FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, panel: "workbench", order: 1, bodyKey: PUZZLE_5D_PLAY_KINDS_BODY_KEY, label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL },
    { id: PUZZLE_5D_PLAY_INSPECTOR_TAB_ID, iconId: FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, panel: "details", order: 0, bodyKey: PUZZLE_5D_PLAY_INSPECTOR_BODY_KEY, label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL },
  ] satisfies SideTabSpec[];
  return app;
}

export function buildPuzzle5dPlayRuntime(initialPanelVisibility?: { leftSidePanel: boolean; rightSidePanel: boolean }): Platform {
  const runtime = new Platform({ initialPanelVisibility });
  const controller = new Puzzle5dPlayShellController(runtime.commandBus, () => runtime.notify());
  runtime.addApp(buildPuzzle5dPlayAppRuntime(controller));
  return runtime;
}

//#endregion 🔖Puzzle5dPlayRuntime

//#region 🔖DeclarativeBodies
export function buildPuzzle5d2dDeclarativeBody(ctx: WindowBodyViewContext): UiNode {
  const ctrl = puzzle5dControllerFromContext(ctx);
  const snap = ctrl?.getSnapshot();
  if (!snap?.fixture2d) return { type: "text", value: "Invalid 2d fixture" };
  return buildPuzzle2dWindowBody(PUZZLE_5D_PLAY_2D_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID, PUZZLE_5D_PLAY_2D_WINDOW_ID);
}

export function buildPuzzle5d3dDeclarativeBody(ctx: WindowBodyViewContext): UiNode {
  const ctrl = puzzle5dControllerFromContext(ctx);
  const snap = ctrl?.getSnapshot();
  if (!snap?.fixture3d) return { type: "text", value: "Invalid 3d fixture" };
  return buildPuzzle3dWindowBody(PUZZLE_5D_PLAY_3D_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID);
}

export function buildPuzzle5dJackDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
  return buildWriterWindowBody(PUZZLE_5D_PLAY_JACK_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID, PUZZLE_5D_PLAY_JACK_WINDOW_ID);
}

function buildPuzzle5dPlayHierarchyPanelBody(ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext): UiTreeNode {
  return buildControllerTreeSidePanelBody(ctx, (ctrl) => {
    const snap = (ctrl as Puzzle5dPlayShellController).getSnapshot();
    if (!snap) {
      return uiDeclarativeSectionsToTree([
        { type: "section", id: "puzzle-5d-play-hierarchy.loading", label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, children: [{ type: "text", value: "…" }] },
      ]);
    }
    return buildPuzzle5dPlayHierarchySections(snap);
  });
}

function buildPuzzle5dPlayKindsPanelBody(ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext): UiTreeNode {
  return buildControllerTreeSidePanelBody(ctx, (ctrl) => {
    const snap = (ctrl as Puzzle5dPlayShellController).getSnapshot();
    if (!snap) {
      return uiDeclarativeSectionsToTree([
        { type: "section", id: "puzzle-5d-play-kinds.loading", label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, children: [{ type: "text", value: "…" }] },
      ]);
    }
    return buildPuzzle5dPlayKindsTree(snap);
  });
}

function buildPuzzle5dPlayInspectorPanelBody(ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext): UiTreeNode {
  return buildControllerTreeSidePanelBody(ctx, (ctrl) => {
    const snap = (ctrl as Puzzle5dPlayShellController).getSnapshot();
    if (!snap) {
      return uiDeclarativeSectionsToTree([
        { type: "section", id: "puzzle-5d-play-inspector.loading", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "…" }] },
      ]);
    }
    return buildPuzzle5dPlayInspectorTree(snap);
  });
}

export const puzzle5dPlayWindowBodies: Readonly<Record<string, (ctx: WindowBodyViewContext) => UiNode>> = {
  [PUZZLE_5D_PLAY_2D_BODY_KEY]: buildPuzzle5d2dDeclarativeBody,
  [PUZZLE_5D_PLAY_3D_BODY_KEY]: buildPuzzle5d3dDeclarativeBody,
  [PUZZLE_5D_PLAY_JACK_BODY_KEY]: buildPuzzle5dJackDeclarativeBody,
};

export const puzzle5dPlaySidePanelBodies: Readonly<Record<string, (ctx: import("@semio-tech/framework-platform-core").SidePanelBodyViewContext) => UiTreeNode>> = {
  [PUZZLE_5D_PLAY_HIERARCHY_BODY_KEY]: buildPuzzle5dPlayHierarchyPanelBody,
  [PUZZLE_5D_PLAY_KINDS_BODY_KEY]: buildPuzzle5dPlayKindsPanelBody,
  [PUZZLE_5D_PLAY_INSPECTOR_BODY_KEY]: buildPuzzle5dPlayInspectorPanelBody,
};

export function registerPuzzle5dPlayDeclarativeBodies(): void {
  for (const [key, build] of Object.entries(puzzle5dPlayWindowBodies)) registerWindowBody(key, build);
  for (const [key, build] of Object.entries(puzzle5dPlaySidePanelBodies)) registerSidePanelBody(key, build);
}
//#endregion 🔖DeclarativeBodies

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  async function puzzle5dPlaySnapshotWithConcreteForest(controller: Puzzle5dPlayShellController): Promise<Puzzle5dPlaySnapshot> {
    const { default: raw } = await import("../../example/concrete-forest.5d.json");
    const model = parseModel(raw as unknown);
    if (!model) throw new Error("concrete-forest model required for test");
    controller.puzzle5dStore.replaceModel(model);
    return controller.getSnapshot();
  }

  describe("puzzle 5d play hierarchy", () => {
    it("buildPuzzle5dPlayKindsTree exposes draggable part palette rows with flat and volume payloads", async () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      expect(controller).toBeTruthy();
      const snapshot = await puzzle5dPlaySnapshotWithConcreteForest(controller!);
      const tree = buildPuzzle5dPlayKindsTree(snapshot);
      expect(tree.type).toBe("tree");
      const parts = tree.sections.find((section) => section.id === "puzzle-5d-play-kinds.parts");
      expect(parts?.items?.some((row) => row.draggable === true && row.dragData?.[PUZZLE_2D_FIXTURE_DRAG_MIME] && row.dragData?.[FIXTURE_DRAG_MIME])).toBe(true);
    });

    it("puzzle5dFixturePaletteTreeDragController routes flat and volume palette rows", async () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      const snapshot = await puzzle5dPlaySnapshotWithConcreteForest(controller!);
      const tree = buildPuzzle5dPlayKindsTree(snapshot);
      const dragByItemId = collectUiTreeItemDragData(tree.sections);
      const dragController = puzzle5dFixturePaletteTreeDragController(dragByItemId);
      const partRow = tree.sections.find((section) => section.id === "puzzle-5d-play-kinds.parts")?.items?.find((row) => row.dragData);
      expect(partRow?.dragData?.[PUZZLE_2D_FIXTURE_DRAG_MIME]).toBeTruthy();
      expect(partRow?.dragData?.[FIXTURE_DRAG_MIME]).toBeTruthy();
      expect(dragController.pointerPaletteDrag?.readEncodedDragPayload(partRow!.dragData!)).toBeTruthy();
    });

    it("buildPuzzle5dPlayHierarchySections includes Parts and Fasteners sections", async () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      expect(controller).toBeTruthy();
      const snapshot = await puzzle5dPlaySnapshotWithConcreteForest(controller!);
      const tree = buildPuzzle5dPlayHierarchySections(snapshot);
      const sectionLabels = tree.sections.map((section) => section.label);
      expect(sectionLabels).toContain("Parts");
      expect(sectionLabels).toContain("Fasteners");
    });
  });

  describe("puzzle 5d play inspection", () => {
    it("buildPuzzle5dPlayInspectorTree exposes editable part fields for selected parts", async () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      const snapshot = await puzzle5dPlaySnapshotWithConcreteForest(controller!);
      const partId = snapshot.model.parts[0]?.id;
      expect(partId).toBeTruthy();
      controller.puzzle5dStore.setSelection({ partIds: [partId!], gripIds: [] });
      const inspectorSnapshot = controller.getSnapshot();
      const tree = buildPuzzle5dPlayInspectorTree(inspectorSnapshot);
      expect(tree.type).toBe("tree");
      const partSection = tree.sections.find((section) => section.id === "puzzle-5d-play-inspector.parts");
      expect(partSection).toBeDefined();
      const kindField = partSection!.items.find((item) => item.id === "puzzle-5d-play-inspector.part.kind");
      expect(kindField?.control?.type).toBe("select");
      expect(kindField?.control?.onChange?.command).toBe("patchPuzzle5dParts");
    });

    it("buildPuzzle5dPlayInspectorTree tolerates selection ids missing from the model", async () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      const snapshot = await puzzle5dPlaySnapshotWithConcreteForest(controller!);
      const orphanSnapshot = {
        ...snapshot,
        selection: { partIds: ["brush-suggestion-preview"], gripIds: [] },
      };
      const tree = buildPuzzle5dPlayInspectorTree(orphanSnapshot);
      const originField = tree.sections
        .flatMap((section) => section.items)
        .find((item) => item.id === "puzzle-5d-play-inspector.part.origin");
      expect(originField).toBeUndefined();
      const partSection = tree.sections.find((section) => section.id === "puzzle-5d-play-inspector.parts");
      expect(partSection?.items.some((item) => item.label === "Selected parts are not available in the model yet.")).toBe(true);
    });

    it("patchPuzzle5dParts updates part kind on the unified store", async () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      await puzzle5dPlaySnapshotWithConcreteForest(controller!);
      const partId = controller.puzzle5dStore.read().parts[0]?.id;
      expect(partId).toBeTruthy();
      controller.run("patchPuzzle5dParts", { partIds: [partId], field: "partKind", value: "test-kind" });
      expect(controller.puzzle5dStore.read().parts.find((part) => part.id === partId)?.partKind).toBe("test-kind");
    });

    it("buildPuzzle5dPlayInspectorTree exposes batch grip fields for multi grip selection", async () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      const snapshot = await puzzle5dPlaySnapshotWithConcreteForest(controller!);
      const part = snapshot.model.parts.find((row) => row.grips.length >= 2);
      expect(part).toBeTruthy();
      const gripIds = part!.grips.slice(0, 2).map((grip) => `${part!.id}:${grip.id}`);
      controller.puzzle5dStore.setSelection({ partIds: [], gripIds });
      const tree = buildPuzzle5dPlayInspectorTree(controller.getSnapshot());
      const gripSection = tree.sections.find((section) => section.id === "puzzle-5d-play-inspector.grips");
      expect(gripSection?.label).toContain("Grips (2)");
      const kindField = gripSection?.items.find((item) => item.id === "puzzle-5d-play-inspector.grip.kind");
      expect(kindField?.control?.onChange?.command).toBe("patchPuzzle5dGrips");
    });
  });

  describe("puzzle 5d play fixtures", () => {
    it("parses nakagin 2d and 3d fixtures", async () => {
      const [{ default: nakagin2dJson }, { default: nakagin3dJson }] = await Promise.all([
        import("../../../2d/example/nakagin-capsule-tower.2d.json"),
        import("../../../3d/example/nakagin-capsule-tower.3d.json"),
      ]);
      const fixture2d = parsePuzzle2dFixture(nakagin2dJson as unknown);
      const fixture3d = parseFixture(nakagin3dJson as unknown);
      expect(fixture2d?.nodes.length).toBeGreaterThan(0);
      expect(fixture3d?.objects.length).toBeGreaterThan(0);
    });
    it("parses nakagin unified puzzle 5d model", async () => {
      const { default: nakagin5dJson } = await import("../../example/nakagin-capsule-tower.5d.json");
      const model = parseModel(nakagin5dJson as unknown);
      expect(model?.schema).toBe(PUZZLE_5D_SCHEMA);
      expect(model?.parts.length).toBeGreaterThan(0);
    });
    it("parses concrete forest 2d, 3d, and unified puzzle 5d model", async () => {
      const [{ default: concreteForest2dJson }, { default: concreteForest3dJson }, { default: concreteForest5dJson }] = await Promise.all([
        import("../../../2d/example/concrete-forest.2d.json"),
        import("../../../3d/example/concrete-forest.3d.json"),
        import("../../example/concrete-forest.5d.json"),
      ]);
      const fixture2d = parsePuzzle2dFixture(concreteForest2dJson as unknown);
      const fixture3d = parseFixture(concreteForest3dJson as unknown);
      const model = parseModel(concreteForest5dJson as unknown);
      expect(fixture2d?.nodes.some((node) => node.id === "seed-left-001")).toBe(true);
      expect(fixture3d?.objects.some((object) => object.id === "seed-left-001")).toBe(true);
      expect(model?.schema).toBe(PUZZLE_5D_SCHEMA);
      expect(model?.parts.some((part) => part.id === "seed-left-001" && part["2d"] && part["3d"])).toBe(true);
    });
    it("fixture catalog lists concrete forest and nakagin", () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      const catalog = controller.getExampleCatalog();
      expect(catalog.activeExampleId).toBe(PUZZLE_5D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
      expect(catalog.options.map((row) => row.id)).toEqual([PUZZLE_5D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, PUZZLE_5D_PLAY_EXAMPLE_NAKAGIN_ID]);
    });
    it("regenerates nakagin 5d fixture when REGENERATE_NAKAGIN_5D=1", async () => {
      if (process.env.REGENERATE_NAKAGIN_5D !== "1") return;
      const [{ default: nakagin2dJson }, { default: nakagin3dJson }] = await Promise.all([
        import("../../../2d/example/nakagin-capsule-tower.2d.json"),
        import("../../../3d/example/nakagin-capsule-tower.3d.json"),
      ]);
      const fixture2d = parsePuzzle2dFixture(nakagin2dJson as unknown);
      const fixture3d = parseFixture(nakagin3dJson as unknown);
      expect(fixture2d).toBeTruthy();
      expect(fixture3d).toBeTruthy();
      const model = {
        ...compose5d(fixture2d!, fixture3d!),
        label: "Nakagin capsule tower",
        meta: {
          description: "Unified puzzle 5d source for Nakagin play; 2d and 3d views project from this model.",
        },
      };
      const { writeFile } = await import("node:fs/promises");
      const { join } = await import("node:path");
      const outPath = join(process.cwd(), "../../example/nakagin-capsule-tower.5d.json");
      await writeFile(outPath, `${JSON.stringify(model, null, 2)}\n`, "utf8");
      expect(model.parts.length).toBeGreaterThan(0);
    });
    it("regenerates concrete forest 5d fixture when REGENERATE_CONCRETE_FOREST_5D=1", async () => {
      if (process.env.REGENERATE_CONCRETE_FOREST_5D !== "1") return;
      const [{ default: concreteForest2dJson }, { default: concreteForest3dJson }] = await Promise.all([
        import("../../../2d/example/concrete-forest.2d.json"),
        import("../../../3d/example/concrete-forest.3d.json"),
      ]);
      const fixture2d = parsePuzzle2dFixture(concreteForest2dJson as unknown);
      const fixture3d = parseFixture(concreteForest3dJson as unknown);
      expect(fixture2d).toBeTruthy();
      expect(fixture3d).toBeTruthy();
      const model = {
        ...compose5d(fixture2d!, fixture3d!),
        label: "Concrete Forest",
        meta: {
          description: "Unified puzzle 5d source for Concrete Forest play; 2d and 3d views project from this model.",
        },
      };
      const { writeFile } = await import("node:fs/promises");
      const { join } = await import("node:path");
      const outPath = join(process.cwd(), "../../example/concrete-forest.5d.json");
      await writeFile(outPath, `${JSON.stringify(model, null, 2)}\n`, "utf8");
      expect(model.parts.length).toBeGreaterThan(0);
    });
    it("shared kinds merge metas like the play harness", () => {
      const sk = sharedKindsFromMetas({
        meta2d: undefined,
        meta3d: { kindCompatibility: [{ source: "u", target: "v" }] },
      });
      expect(sk.kindCompatibility?.length).toBeGreaterThan(0);
    });
    it("activates brush via engagement submit", () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      expect(controller).toBeTruthy();
      controller.run("engagementSubmit", { windowId: PUZZLE_5D_PLAY_2D_WINDOW_ID, value: "Brush" });
      expect(controller.getActiveTool()).toBe("brush");
    });

    it("addBrushPart grows unified store parts when placement is valid", async () => {
      const runtime = buildPuzzle5dPlayRuntime();
      const controller = runtime.getActiveApp()?.controller as Puzzle5dPlayShellController;
      await puzzle5dPlaySnapshotWithConcreteForest(controller);
      const host = controller.puzzle5dStore.read().parts[0];
      const hostAnchor = host?.grips[0]?.id;
      if (!host?.id || !hostAnchor) return;
      const peerKind = controller.puzzle5dStore.read().parts.find((part) => part.partKind && part.id !== host.id)?.partKind;
      if (!peerKind) return;
      const before = controller.puzzle5dStore.read().parts.length;
      controller.run("addBrushPart", {
        partKind: peerKind,
        sourceGripFullId: `${host.id}:${hostAnchor}`,
        aspect3d: {
          targetVortexFullId: `${host.id}:${hostAnchor}`,
          objectKindId: peerKind,
          sourceVortexIndex: 0,
          origin: [2, 0, 0],
          orientation: [0, 0, 0, 1],
          objectId: "brush-test-part",
        },
      });
      expect(controller.puzzle5dStore.read().parts.length).toBeGreaterThan(before);
      const placed = controller.puzzle5dStore.read().parts.find((part) => part.id === "brush-test-part");
      expect(placed?.["2d"]).toBeTruthy();
      expect(placed?.["3d"]).toBeTruthy();
    });

    it("builds declarative 2d and 3d canvas-only bodies", () => {
      const wb = buildPuzzle5dPlayRuntime();
      const body2d = buildPuzzle5d2dDeclarativeBody({
        runtime: wb,
        windowKindId: PUZZLE_5D_PLAY_2D_WINDOW_ID,
        bodyKey: PUZZLE_5D_PLAY_2D_BODY_KEY,
        activeModeId: "main",
        generation: 0,
      });
      const body3d = buildPuzzle5d3dDeclarativeBody({
        runtime: wb,
        windowKindId: PUZZLE_5D_PLAY_3D_WINDOW_ID,
        bodyKey: PUZZLE_5D_PLAY_3D_BODY_KEY,
        activeModeId: "main",
        generation: 0,
      });
      expect(body2d).toEqual(buildPuzzle2dWindowBody(PUZZLE_5D_PLAY_2D_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID, PUZZLE_5D_PLAY_2D_WINDOW_ID));
      expect(body3d).toEqual(buildPuzzle3dWindowBody(PUZZLE_5D_PLAY_3D_SURFACE_ID, PUZZLE_5D_PLAY_CONTROLLER_ID));
    });
  });
}
//#endregion 🧪Tests

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { createTypedAppVcsHandler } from "@semio-tech/framework-os-core";

function meshUrlFromPuzzle5dModel(model: Puzzle5dModel): string | null {
	const fixture3d = project3d(model);
	for (const object of fixture3d.objects) {
		if (object.meshUrl) return object.meshUrl;
	}
	return null;
}

/** @emoji 🧩 OS app VCS handler for puzzle 5d documents. */
export function createPuzzle5dAppVcsHandler() {
	return createTypedAppVcsHandler<Puzzle5dModel, { readonly op: "setRevision"; readonly revision: number }>(
		"puzzle.5d",
		"puzzle.5d",
		() => ({
			schema: "puzzle.5d",
			domain: "architecture",
			camera2d: { x: 0, y: 0, zoom: 1 },
			camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
			parts: [],
			fasteners: [],
		}),
		(doc) => doc,
		undefined,
		{
			applyInputBindings: (model, inputBindings) => {
				const catalogue = inputBindings.catalogue as Puzzle5dKindCatalogBundle | undefined;
				if (!catalogue) return model;
				return { ...model, kindCatalogs: catalogue };
			},
			projectOutput: (model, portId) => {
				if (portId === "graph2d") return project2d(model);
				if (portId === "mesh3d") {
					const url = meshUrlFromPuzzle5dModel(model);
					return { url: url ?? "/mesh/base.glb" };
				}
				return model;
			},
		},
	);
}

/** @emoji 🧩 S program definition for puzzle 5d. */
export function buildPuzzle5dProgramDefinition(): PlatformDefinition {
	return {
		id: "puzzle.5d",
		name: "Puzzle 5D",
		apiVersion: "1",
		apps: [{ id: "puzzle5d", label: "Puzzle 5D", controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🔖OsProgram
import { createCatalogueKindsAppVcsHandler, mergeOsProgramDefinition, osBaselineResource, registerAppVcsHandler, osInPort, osOutPort } from "@semio-tech/framework-os-core";
import type { OsProgramContribution } from "@semio-tech/framework-platform-core";
import { puzzle5dDefaultManifestCatalogBundle } from "@semio-tech/puzzle-5d-react";

const puzzle5dProgramContributionResources = {
		"puzzle5d": { inputs: [osInPort("catalogue.kinds", "catalogue", "Catalogue")], outputs: [osOutPort("2d.puzzle", "graph2d", "2D Graph"), osOutPort("3d.mesh", "mesh3d", "3D Mesh")], sourceFormat: "puzzle.5d", componentKind: "puzzle5d", modes: [{ id: "edit", label: "Edit" }] },
	};

/** @emoji 🧩 OS program contribution for puzzle.5d. */
export const puzzle5dProgramContribution: OsProgramContribution = {
	programId: "puzzle.5d",
	register() {
		mergeOsProgramDefinition("puzzle.5d", buildPuzzle5dProgramDefinition(), puzzle5dProgramContributionResources);
		registerAppVcsHandler(createCatalogueKindsAppVcsHandler(() => puzzle5dDefaultManifestCatalogBundle() ?? {}));
		registerPuzzle5dMediaExportHandlers();
		registerAppVcsHandler(createPuzzle5dAppVcsHandler());
	},
};
//#endregion 🔖OsProgram

//#region 🔖Play

export const PUZZLE_5D_PLAY_EXAMPLE_NAKAGIN_ID = "nakagin";
export const PUZZLE_5D_PLAY_EXAMPLE_CONCRETE_FOREST_ID = "concrete-forest";

export const PUZZLE_5D_PLAY_EXAMPLE_OPTIONS = [
	{ id: PUZZLE_5D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, label: "Concrete Forest" },
	{ id: PUZZLE_5D_PLAY_EXAMPLE_NAKAGIN_ID, label: "Nakagin capsule tower" },
] as const;

/** @emoji 🔒 Resolves a playground example slug (e.g. `concrete`) to a puzzle 5d example id. */
export function resolvePuzzle5dPlayExampleSlug(slug: string): string | undefined {
	const aliases: Record<string, string> = { concrete: PUZZLE_5D_PLAY_EXAMPLE_CONCRETE_FOREST_ID };
	const normalized = aliases[slug] ?? slug;
	return PUZZLE_5D_PLAY_EXAMPLE_OPTIONS.some((row) => row.id === normalized) ? normalized : undefined;
}

import concreteForest5dExampleJson from "../../example/concrete-forest.5d.json";
import nakagin5dExampleJson from "../../example/nakagin-capsule-tower.5d.json";

const PUZZLE_5D_PLAY_EXAMPLE_JSON_BY_ID: Readonly<Record<string, unknown>> = {
	[PUZZLE_5D_PLAY_EXAMPLE_CONCRETE_FOREST_ID]: concreteForest5dExampleJson,
	[PUZZLE_5D_PLAY_EXAMPLE_NAKAGIN_ID]: nakagin5dExampleJson,
};

/** @emoji 📥 Loads a play sample by catalog id from bundled example JSON. */
export async function fetchPuzzle5dPlayModel(exampleId: string): Promise<Puzzle5dModel | null> {
	const raw = PUZZLE_5D_PLAY_EXAMPLE_JSON_BY_ID[exampleId];
	if (!raw) return null;
	return parseModel(raw as unknown);
}

/** @emoji 📭 Empty puzzle 5d model for the no-example playground catalog entry. */
export function puzzle5dPlayEmptyModel(): Puzzle5dModel {
	return {
		schema: PUZZLE_5D_SCHEMA,
		domain: "architecture",
		camera2d: { x: 0, y: 0, zoom: 1 },
		camera3d: { position: [0, 0, 0], target: [0, 0, 0], zoom: 1 },
		parts: [],
		fasteners: [],
		label: "",
	};
}

export const puzzle5dPlayAppDefinition = createPlaygroundApp({
	id: PUZZLE_5D_PLAY_APP_ID,
	label: "Puzzle 5D",
	controllerId: PUZZLE_5D_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "5d",
		resolveDedupe: ["react", "react-dom", "three", "@semio-tech/puzzle-2d-react", "@semio-tech/puzzle-3d-react", "@semio-tech/puzzle-5d-react"],
		optimizeDeps: {
			include: [
				"react",
				"react-dom",
				"react/jsx-runtime",
				"react/jsx-dev-runtime",
				"three",
				"@react-three/fiber",
				"@react-three/drei",
				"lucide-react",
				"@semio-tech/infinite-world-r3f",
				"@semio-tech/puzzle-2d-react",
				"@semio-tech/puzzle-3d-react",
				"@semio-tech/puzzle-5d-react",
			],
		},
	},
	createRuntime: () => {
		return buildPuzzle5dPlayRuntime();
	},
});
//#endregion 🔖Play

//#region 🔖MediaExport
/** @emoji 💾 Registers puzzle 5d model OBJ/GLB export handlers via {@link project3d}. */
export function registerPuzzle5dMediaExportHandlers(): void {
	registerOsMediaExportHandler("5d.puzzle", "obj", async (doc) => ({
		data: await exportPuzzle3dFixtureObj(project3d(doc as Parameters<typeof project3d>[0])),
		mimeType: "text/plain",
		fileName: "puzzle5d.obj",
	}));
	registerOsMediaExportHandler("5d.puzzle", "glb", async (doc) => ({
		data: await exportPuzzle3dFixtureGlb(project3d(doc as Parameters<typeof project3d>[0])),
		mimeType: "model/gltf-binary",
		fileName: "puzzle5d.glb",
	}));
}
//#endregion 🔖MediaExport
