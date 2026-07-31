import.meta.env = { BASE_URL: "/", DEV: true, MODE: "development", PLAYGROUND_LOCKED_FIXTURE_ID: "hexagonal-mushroom-column", PROD: false, PUZZLE_PLAY_ENTRY: "procedural-3d", SSR: false };
import * as __vite_glob_0_0 from "/@fs/Users/ueli/Documents/semio/procedural/3d/fixture/hexagonal-mushroom-column.procedural.json?import";
import * as __vite_glob_0_1 from "/@fs/Users/ueli/Documents/semio/procedural/3d/fixture/rectangle-extrude-volume.procedural.json?import";
import * as __vite_glob_0_2 from "/@fs/Users/ueli/Documents/semio/procedural/3d/fixture/sphere-cut-with-torus.procedural.json?import";
import { buildFlowPlayCatalogueTree, buildFlowPlayDocumentTree, buildFlowPlayInspectorTree, parseFlowPlayFixtureJson } from "/@fs/Users/ueli/Documents/semio/flow/play/index.ts";
import {
  buildCatalogueKindsTreeSections,
  buildFlowContextMenuItems,
  DAG_LOD_MODE_AUTOMATIC,
  dagLodAutomaticSelectLabel,
  dagPlayLodTierMenuLabel,
  dagPlayLodTiers,
  FLOW_DEFAULT_PROXIMITY_DISTANCE,
  flowPlayCatalogueItemDragData,
  flowSensibleSliderRange,
  isDagDrawLodKind,
} from "/@fs/Users/ueli/Documents/semio/flow/react/index.tsx";
import {
  buildFlowWindowBody,
  buildPuzzle3dWindowBody,
  CommandBus,
  Controller,
  createDefaultLayout,
  createPlayAppRuntime,
  createProductPlaygroundPlatform,
  enforcePlaygroundWindowEngagementInput,
  isPlaygroundFixtureLocked,
  isPlaygroundNoFixtureId,
  ModeRuntime,
  Playground,
  PLAYGROUND_NO_FIXTURE_ID,
  playgroundResolvedFixtureId,
  registerWindowBody,
  WindowKindRuntime,
} from "/@fs/Users/ueli/Documents/semio/framework/product/playground/core/index.ts";
import { meshTransferFromPreviewPayload } from "/@fs/Users/ueli/Documents/semio/geometry/brep/js/index.ts";
import { extractChannelPreviewItems, filterVisiblePreviewItems, PROCEDURAL_DEFAULT_FIXTURE, proceduralExtensionHost, proceduralFixtureToJson, resolveGeometryTargets } from "/@fs/Users/ueli/Documents/semio/procedural/3d/react/index.tsx";
import { bootstrapElementsSurfaceChromeDocument, selectionMergeIds } from "/@fs/Users/ueli/Documents/semio/ui/react/index.tsx";
function previewItemKey(item) {
  return `${item.widgetId}:${item.port}:${item.direction}`;
}
function previewItemsWithMeshes(items, previewMeshes, previous = []) {
  const previousByKey = new Map(previous.map((item) => [previewItemKey(item), item]));
  return items.map((item) => {
    if (item.kind !== "geometry" || item.direction !== "out") return item;
    const meshKey = `${item.widgetId}:${item.port}`;
    const previousItem = previousByKey.get(previewItemKey(item));
    const mesh = meshTransferFromPreviewPayload(previewMeshes?.[meshKey]) ?? (previousItem?.handle === item.handle ? previousItem.mesh : void 0);
    return mesh ? { ...item, mesh } : item;
  });
}
export const PROCEDURAL_3D_PLAY_APP_ID = "procedural-3d-play";
export const PROCEDURAL_3D_PLAY_CONTROLLER_ID = "procedural-3d-play";
export const PROCEDURAL_PLAY_SURFACE_ID = "procedural.play/v1";
export const PROCEDURAL_PLAY_BODY_KEY_MAIN = "procedural.play.main";
export const PROCEDURAL_PLAY_WINDOW_KIND_ID = "procedural-main";
export const PROCEDURAL_PLAY_WINDOW_KIND_PREVIEW = "procedural-preview";
export const PROCEDURAL_PLAY_BODY_KEY_PREVIEW = "procedural.play.preview";
export const PROCEDURAL_PLAY_SURFACE_ID_PREVIEW = "procedural.play.preview/v1";
export const PROCEDURAL_PLAY_DEFAULT_FIXTURE = PROCEDURAL_DEFAULT_FIXTURE;
export const PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON = proceduralFixtureToJson(PROCEDURAL_DEFAULT_FIXTURE);
export const PROCEDURAL_PLAY_LAYOUT = createDefaultLayout([PROCEDURAL_PLAY_WINDOW_KIND_ID, PROCEDURAL_PLAY_WINDOW_KIND_PREVIEW], "row", [55, 45], ["Flow", "Preview"]);
export const PROCEDURAL_PLAY_KINDS_TAB_ID = "procedural-play-kinds";
export const PROCEDURAL_PLAY_EXTENSIONS_TAB_ID = "procedural-play-extensions";
export const PROCEDURAL_PLAY_DOCUMENT_TAB_ID = "framework.panel.document";
export const PROCEDURAL_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const PROCEDURAL_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";
export const PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID = "procedural-default";
import { PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID, resolveProceduralPlayFixtureSlug } from "/fixture-slugs.ts";
export { PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID, resolveProceduralPlayFixtureSlug };
const proceduralFixtureModules = /* #__PURE__ */ Object.assign({
  "../fixture/hexagonal-mushroom-column.procedural.json": __vite_glob_0_0,
  "../fixture/rectangle-extrude-volume.procedural.json": __vite_glob_0_1,
  "../fixture/sphere-cut-with-torus.procedural.json": __vite_glob_0_2,
});
function proceduralFixtureIdFromGlobPath(globPath) {
  const base = globPath.split("/").pop() ?? globPath;
  return base.replace(/\.procedural\.json$/, "");
}
function proceduralFixtureLabelFromId(id) {
  return id
    .split("-")
    .filter(Boolean)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
}
const PROCEDURAL_PLAY_FILE_FIXTURE_JSON_BY_ID = Object.fromEntries(
  Object.entries(proceduralFixtureModules).map(([path, mod]) => {
    const id = proceduralFixtureIdFromGlobPath(path);
    const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
    return [id, json];
  }),
);
export const PROCEDURAL_PLAY_EMPTY_FIXTURE = {
  schema: "flow.fixture/v1",
  camera: { x: 0, y: 0, zoom: 1 },
  widgets: [],
  synapses: [],
};
export const PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON = proceduralFixtureToJson(PROCEDURAL_PLAY_EMPTY_FIXTURE);
export const PROCEDURAL_PLAY_FIXTURE_OPTIONS = [
  { id: PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID, label: "Box fillet move" },
  ...Object.keys(PROCEDURAL_PLAY_FILE_FIXTURE_JSON_BY_ID)
    .sort()
    .map((id) => ({ id, label: proceduralFixtureLabelFromId(id) })),
];
const PROCEDURAL_PLAY_STORE_KEY = "procedural.fixture/v1";
export function createProceduralPlayFixtureStore(storage) {
  const resolved =
    storage ??
    (typeof globalThis.localStorage !== "undefined"
      ? globalThis.localStorage
      : /* @__PURE__ */ (() => {
          const backing = /* @__PURE__ */ new Map();
          return {
            getItem: (key) => backing.get(key) ?? null,
            setItem: (key, value) => {
              backing.set(key, value);
            },
            removeItem: (key) => {
              backing.delete(key);
            },
          };
        })());
  return {
    load() {
      return resolved.getItem(PROCEDURAL_PLAY_STORE_KEY);
    },
    save(fixtureJson) {
      resolved.setItem(PROCEDURAL_PLAY_STORE_KEY, fixtureJson);
    },
    clear() {
      resolved.removeItem(PROCEDURAL_PLAY_STORE_KEY);
    },
  };
}
const DEFAULT_LAYER_SPACING = 120;
const DEFAULT_SIBLING_GAP = 40;
const BREP_XFORM_NEURON_KIND = {
  translate: "brep.xform.translate",
  rotate: "brep.xform.rotate",
  scale: "brep.xform.scale",
};
const GUMBALL_SLIDER_HALF_WIDTH = 42;
const GUMBALL_NEURON_HALF_WIDTH = 48;
const GUMBALL_VECTOR_HALF_WIDTH = 52;
const GUMBALL_SOURCE_HALF_WIDTH = 48;
function gumballColumnEdgeGap(layerSpacing, siblingGap) {
  return Math.max(siblingGap, layerSpacing * 0.2, 28);
}
function gumballColumnAfter(prevCenterX, prevHalfWidth, nextHalfWidth, edgeGap) {
  return prevCenterX + prevHalfWidth + edgeGap + nextHalfWidth;
}
function gumballValueRowGap(siblingGap) {
  return Math.max(siblingGap, 32);
}
function gumballMakeSpaceDx(transformColumnX, transformHalfWidth, sourceX, edgeGap) {
  return transformColumnX + transformHalfWidth + edgeGap - sourceX;
}
function widgetLayoutFromFixture(fixtureJson, widgetId) {
  try {
    const fixture = JSON.parse(fixtureJson);
    return fixture.layout?.[widgetId] ?? { x: 0, y: 0 };
  } catch {
    return { x: 0, y: 0 };
  }
}
function gumballZeroDelta(op) {
  if (op === "translate") return { op: "translate", offset: [0, 0, 0] };
  if (op === "rotate") return { op: "rotate", angle: 0 };
  return { op: "scale", factor: 1 };
}
function copyGumballValues(binding) {
  return {
    offset: [binding.values.offset[0], binding.values.offset[1], binding.values.offset[2]],
    angle: binding.values.angle,
    factor: binding.values.factor,
  };
}
function setGumballBindingValues(binding, values) {
  binding.values.offset = [values.offset[0], values.offset[1], values.offset[2]];
  binding.values.angle = values.angle;
  binding.values.factor = values.factor;
}
function applyGumballDeltaToBase(base, op, delta) {
  if (op === "translate" && delta.op === "translate") {
    return {
      offset: [base.offset[0] + delta.offset[0], base.offset[1] + delta.offset[1], base.offset[2] + delta.offset[2]],
      angle: base.angle,
      factor: base.factor,
    };
  }
  if (op === "rotate" && delta.op === "rotate") {
    return { offset: base.offset, angle: base.angle + delta.angle, factor: base.factor };
  }
  if (op === "scale" && delta.op === "scale") {
    return { offset: base.offset, angle: base.angle, factor: base.factor * delta.factor };
  }
  return base;
}
function gumballBindingNodeIds(binding) {
  return [...binding.valueWidgetIds, ...(binding.vectorId ? [binding.vectorId] : []), binding.transformId];
}
function accumulateGumballDelta(binding, delta) {
  if (delta.op === "translate" && binding.op === "translate") {
    binding.values.offset = [binding.values.offset[0] + delta.offset[0], binding.values.offset[1] + delta.offset[1], binding.values.offset[2] + delta.offset[2]];
    return;
  }
  if (delta.op === "rotate" && binding.op === "rotate") {
    binding.values.angle += delta.angle;
    return;
  }
  if (delta.op === "scale" && binding.op === "scale") {
    binding.values.factor *= delta.factor;
  }
}
function compactNeuronParams(binding) {
  if (binding.op === "translate") {
    const [x, y, z] = binding.values.offset;
    return { offset: [x, y, z] };
  }
  if (binding.op === "rotate") {
    return { angle: binding.values.angle };
  }
  return { factor: binding.values.factor };
}
function sliderDescriptor(id, value) {
  const { min, max, step } = flowSensibleSliderRange(value);
  return JSON.stringify({ kind: "inputSlider", id, value, min, max, step });
}
function neuronDescriptor(id, neuronKind) {
  return JSON.stringify({ kind: "neuron", id, neuronKind });
}
function proceduralPlayCmd(command, args) {
  return { controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command, args };
}
function buildProceduralLayoutOptionsJson(layerSpacing, siblingGap, orientation) {
  return JSON.stringify({ layerSpacing, siblingGap, orientation });
}
export function buildProceduralPlayCanvasContextMenu(ctx, dispatch) {
  const items = [...buildFlowContextMenuItems(ctx, dispatch)];
  if (ctx.hoveredNodeId) {
    items.splice(items.length - 1, 0, {
      id: "procedural.ctx.isolatePreview",
      label: "Isolate in preview",
      icon: "eye",
      onSelect: () => {
        dispatch("setSelection", { ids: [ctx.hoveredNodeId], mode: "default" });
        dispatch("setShowMode", { id: "selected" });
      },
    });
  }
  return items;
}
export function buildProceduralPlayExtensionsTree(entries) {
  if (!entries.length) {
    return {
      type: "tree",
      sections: [
        {
          id: "procedural-play-extensions.empty",
          label: "Extensions",
          defaultOpen: false,
          items: [{ id: "procedural-play-extensions.empty.msg", label: "Loading extensions…" }],
        },
      ],
    };
  }
  const commandItems = proceduralExtensionHost.activeCommands().map((command) => ({
    id: `procedural-play-extensions.command.${command.id}`,
    label: command.title,
    description: command.id,
    command: proceduralPlayCmd("runExtensionCommand", { commandId: command.id }),
  }));
  const sections = [
    {
      id: "procedural-play-extensions.installed",
      label: "Installed",
      defaultOpen: false,
      items: entries.map((entry) => {
        const operators = entry.manifest.contributes.operators ?? [];
        const schemas = entry.manifest.contributes.schemas ?? [];
        const commands = entry.manifest.contributes.commands ?? [];
        return {
          id: `procedural-play-extensions.${entry.id}`,
          label: entry.manifest.name,
          description: `${entry.manifest.version} · ${entry.active ? "enabled" : "disabled"} · ${operators.length} operators · ${schemas.length} schemas · ${commands.length} commands`,
          command: proceduralPlayCmd("toggleExtension", { id: entry.id, enabled: !entry.active }),
        };
      }),
    },
  ];
  if (commandItems.length) {
    sections.push({
      id: "procedural-play-extensions.commands",
      label: "Commands",
      defaultOpen: false,
      items: commandItems,
    });
  }
  return { type: "tree", sections };
}
export function buildProceduralPlayKindsTree(sections) {
  if (!sections.length) {
    return {
      type: "tree",
      sections: [
        {
          id: "procedural-play-kinds.empty",
          label: "Catalogue",
          defaultOpen: false,
          items: [{ id: "procedural-play-kinds.empty.msg", label: "Loading catalogue…" }],
        },
      ],
    };
  }
  const treeSections = buildCatalogueKindsTreeSections(sections, "procedural-play-kinds", flowPlayCatalogueItemDragData);
  return { type: "tree", sections: treeSections };
}
export function buildProceduralPlayDocumentTree(fixtureJson, selectedNodeIds) {
  return buildFlowPlayDocumentTree(fixtureJson, selectedNodeIds, PROCEDURAL_3D_PLAY_CONTROLLER_ID);
}
export function buildProceduralPlayCatalogueTree(sections, extensionEntries) {
  return buildFlowPlayCatalogueTree(sections, extensionEntries);
}
export function buildProceduralPlayInspectorTree(fixtureJson, selectedNodeIds) {
  return buildFlowPlayInspectorTree(fixtureJson, selectedNodeIds, PROCEDURAL_3D_PLAY_CONTROLLER_ID);
}
export function buildProceduralPlayToolbarTools(state, controllerId) {
  const selectionTools = [
    {
      id: "procedural.select.rectangle",
      kind: "toggle",
      iconId: "square",
      text: "Rectangle",
      order: 0,
      pressed: state.selectionMethod === "rectangle",
      controllerId,
      command: "setSelectionMethod",
      args: { method: "rectangle" },
    },
    {
      id: "procedural.select.lasso",
      kind: "toggle",
      iconId: "lasso",
      text: "Lasso",
      order: 1,
      pressed: state.selectionMethod === "lasso",
      controllerId,
      command: "setSelectionMethod",
      args: { method: "lasso" },
    },
    {
      id: "procedural.select.mode.default",
      kind: "toggle",
      iconId: "mouse-pointer-2",
      text: "Default",
      order: 2,
      pressed: state.selectionMode === "default",
      controllerId,
      command: "setSelectionMode",
      args: { mode: "default" },
    },
    {
      id: "procedural.select.mode.additive",
      kind: "toggle",
      iconId: "plus",
      text: "Add",
      order: 3,
      pressed: state.selectionMode === "additive",
      controllerId,
      command: "setSelectionMode",
      args: { mode: "additive" },
    },
    {
      id: "procedural.select.mode.subtractive",
      kind: "toggle",
      iconId: "minus",
      text: "Subtract",
      order: 4,
      pressed: state.selectionMode === "subtractive",
      controllerId,
      command: "setSelectionMode",
      args: { mode: "subtractive" },
    },
    {
      id: "procedural.select.mode.invertive",
      kind: "toggle",
      iconId: "arrow-right-left",
      text: "Invert",
      order: 5,
      pressed: state.selectionMode === "invertive",
      controllerId,
      command: "setSelectionMode",
      args: { mode: "invertive" },
    },
    {
      id: "procedural.selection.clear",
      kind: "button",
      iconId: "x",
      label: "Clear",
      order: 6,
      disabled: state.selectionCount === 0,
      controllerId,
      command: "clearSelection",
    },
  ];
  const saveTools = [
    {
      id: "procedural.save.stored",
      kind: "button",
      iconId: "hard-drive",
      label: "Store",
      order: 0,
      controllerId,
      command: "saveStored",
    },
    {
      id: "procedural.save.download",
      kind: "button",
      iconId: "save",
      label: "Download",
      order: 1,
      controllerId,
      command: "saveDownload",
    },
    {
      id: "procedural.save.load",
      kind: "button",
      iconId: "folder-open",
      label: "Load",
      order: 2,
      controllerId,
      command: "loadRequest",
    },
    {
      id: "procedural.save.loadStored",
      kind: "button",
      iconId: "rotate-ccw",
      label: "Restore",
      order: 3,
      disabled: !state.hasStoredFixture,
      controllerId,
      command: "loadStored",
    },
    {
      id: "procedural.save.reset",
      kind: "button",
      iconId: "refresh-cw",
      label: "Reset",
      order: 4,
      controllerId,
      command: "resetFixture",
    },
  ];
  return {
    selection: selectionTools,
    save: saveTools,
    view: [
      {
        id: "procedural.view.everything",
        kind: "toggle",
        iconId: "layers",
        text: "Everything",
        order: 0,
        pressed: state.showMode === "everything",
        controllerId,
        command: "setShowMode",
        args: { id: "everything" },
      },
      {
        id: "procedural.view.selected",
        kind: "toggle",
        iconId: "eye",
        text: "Selected",
        order: 1,
        pressed: state.showMode === "selected",
        controllerId,
        command: "setShowMode",
        args: { id: "selected" },
      },
    ],
    actions: [
      {
        id: "procedural.action.reorganize",
        kind: "button",
        iconId: "layout-grid",
        label: "Reorganize",
        order: 0,
        controllerId,
        command: "reorganize",
      },
      {
        id: "procedural.action.delete",
        kind: "button",
        iconId: "trash-2",
        label: "Delete",
        order: 1,
        disabled: state.selectionCount === 0,
        controllerId,
        command: "deleteSelection",
      },
    ],
  };
}
function proceduralFixtureJsonForId(fixtureId) {
  if (isPlaygroundNoFixtureId(fixtureId)) {
    return proceduralFixtureToJson(PROCEDURAL_PLAY_EMPTY_FIXTURE);
  }
  if (fixtureId === PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID) {
    return PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON;
  }
  const fileJson = PROCEDURAL_PLAY_FILE_FIXTURE_JSON_BY_ID[fixtureId];
  if (fileJson) return fileJson;
  return PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON;
}
export function proceduralPlayFixtureJson(fixtureId = PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID) {
  return proceduralFixtureJsonForId(fixtureId);
}
export class ProceduralPlayController extends Controller {
  mainMode = new ModeRuntime("main", "Procedural", void 0);
  activeFixtureId = playgroundResolvedFixtureId(PLAYGROUND_NO_FIXTURE_ID);
  fixtureJson = proceduralFixtureJsonForId(playgroundResolvedFixtureId(PLAYGROUND_NO_FIXTURE_ID));
  fixtureStore;
  hostBridge = null;
  previewText = "—";
  catalogueSections = [];
  catalogueRevision = 0;
  snapshotListeners = /* @__PURE__ */ new Set();
  engagementInput = "";
  layerSpacing = DEFAULT_LAYER_SPACING;
  siblingGap = DEFAULT_SIBLING_GAP;
  orientation = "leftRight";
  reorganizeEpoch = 0;
  reorganizeOptionsJson = buildProceduralLayoutOptionsJson(DEFAULT_LAYER_SPACING, DEFAULT_SIBLING_GAP, "leftRight");
  commandRequestEpoch = 0;
  commandRequestPayload = { command: "" };
  extensionRevision = 0;
  previewItems = [];
  selectedNodeIds = [];
  preselectNodeIds = [];
  preselectRemovedNodeIds = [];
  hoveredNodeId = null;
  hoveredChannel = null;
  selectedChannels = [];
  fixtureEdges = [];
  previewOffNodeIds = [];
  showMode = "everything";
  selectionMode = "default";
  selectionMethod = "rectangle";
  interactionRevision = 0;
  transformGranularity = "full";
  gumballBindings = /* @__PURE__ */ new Map();
  gumballBindingByTransformId = /* @__PURE__ */ new Map();
  gumballDragSession = null;
  gumballActiveWidgetIds = [];
  lodMode = DAG_LOD_MODE_AUTOMATIC;
  lodModeByInstance = {};
  effectiveLod = "normal";
  proximityDistance = FLOW_DEFAULT_PROXIMITY_DISTANCE;
  constructor(commandBus, hostNotify, fixtureStore = createProceduralPlayFixtureStore()) {
    super(PROCEDURAL_3D_PLAY_CONTROLLER_ID, commandBus, hostNotify);
    this.fixtureStore = fixtureStore;
    this.fixtureEdges = this.parseFixtureEdges(this.fixtureJson);
    this.rebuildShellMode();
  }
  hasStoredFixture() {
    return this.fixtureStore.load() != null;
  }
  getFixtureCatalog() {
    if (isPlaygroundFixtureLocked()) return null;
    return { activeFixtureId: this.activeFixtureId, options: [...PROCEDURAL_PLAY_FIXTURE_OPTIONS] };
  }
  /** @emoji 🔗️ Attaches the React host bridge for toolbar file IO. */
  setHostBridge(bridge) {
    this.hostBridge = bridge;
    this.rebuildToolbarTools();
  }
  toolbarState() {
    return (
      this.hostBridge?.getToolbarState() ?? {
        selectionMethod: this.selectionMethod,
        selectionMode: this.selectionMode,
        showMode: this.showMode,
        selectionCount: this.selectedNodeIds.length,
        hasStoredFixture: this.hasStoredFixture(),
      }
    );
  }
  /** @emoji 🔄️ Rebuilds {@link ModeRuntime.tools} from the latest toolbar snapshot. */
  rebuildToolbarTools() {
    if (!this.hostBridge) {
      this.mainMode.tools = void 0;
      return;
    }
    this.mainMode.tools = buildProceduralPlayToolbarTools(this.toolbarState(), this.id);
  }
  resetInteractionState() {
    this.selectedNodeIds = [];
    this.preselectNodeIds = [];
    this.preselectRemovedNodeIds = [];
    this.hoveredNodeId = null;
    this.hoveredChannel = null;
    this.selectedChannels = [];
    this.previewOffNodeIds = [];
    this.previewItems = [];
    this.gumballBindings.clear();
    this.gumballBindingByTransformId.clear();
    this.clearGumballDrag();
  }
  parseFixtureEdges(json) {
    try {
      const parsed = JSON.parse(json);
      if (!Array.isArray(parsed.synapses)) return [];
      return parsed.synapses.flatMap((synapse) => {
        if (typeof synapse.from !== "string" || typeof synapse.to !== "string") return [];
        const fromPort = typeof synapse.from_port === "string" ? synapse.from_port : typeof synapse.fromPort === "string" ? synapse.fromPort : "";
        const toPort = typeof synapse.to_port === "string" ? synapse.to_port : typeof synapse.toPort === "string" ? synapse.toPort : "";
        return [{ source: `${synapse.from}:${fromPort}`, target: `${synapse.to}:${toPort}` }];
      });
    } catch {
      return [];
    }
  }
  applyFixtureJson(json, resetInteraction = false) {
    if (!json.includes("flow.fixture/v1")) return;
    const unchanged = json === this.fixtureJson;
    if (unchanged && !resetInteraction) return;
    if (!unchanged) {
      this.fixtureJson = json;
      this.fixtureEdges = this.parseFixtureEdges(json);
    }
    if (resetInteraction) this.resetInteractionState();
    this.interactionRevision += 1;
    this.notifySnapshot();
    this.rebuildShellMode();
    this.emit();
  }
  renameFlowWidget(oldId, newId) {
    const trimmed = newId.trim();
    if (!trimmed || trimmed === oldId) return;
    const fixture = parseFlowPlayFixtureJson(this.fixtureJson);
    if (!fixture || fixture.widgets.some((widget) => widget.id === trimmed)) return;
    const widgets = fixture.widgets.map((widget) => (widget.id === oldId ? { ...widget, id: trimmed } : widget));
    const synapses = fixture.synapses.map((synapse) => ({
      ...synapse,
      from: synapse.from === oldId ? trimmed : synapse.from,
      to: synapse.to === oldId ? trimmed : synapse.to,
    }));
    this.selectedNodeIds = this.selectedNodeIds.map((id) => (id === oldId ? trimmed : id));
    this.applyFixtureJson(proceduralFixtureToJson({ ...fixture, widgets, synapses }));
  }
  patchFlowWidget(widgetId, field, value) {
    const fixture = parseFlowPlayFixtureJson(this.fixtureJson);
    if (!fixture) return;
    const widgets = fixture.widgets.map((widget) => {
      if (widget.id !== widgetId) return widget;
      if (field === "value" || field === "min" || field === "max" || field === "step") {
        const numeric = typeof value === "number" ? value : Number(value);
        if (!Number.isFinite(numeric)) return widget;
        return { ...widget, [field]: numeric };
      }
      if (typeof value !== "string") return widget;
      return { ...widget, [field]: value };
    });
    this.applyFixtureJson(proceduralFixtureToJson({ ...fixture, widgets }));
  }
  loadFixtureById(fixtureId) {
    const nextId = isPlaygroundNoFixtureId(fixtureId) ? PLAYGROUND_NO_FIXTURE_ID : fixtureId;
    const nextJson = proceduralFixtureJsonForId(nextId);
    if (nextId === this.activeFixtureId && nextJson === this.fixtureJson) return;
    this.activeFixtureId = nextId;
    this.applyFixtureJson(nextJson, true);
  }
  getFixtureJson() {
    return this.fixtureJson;
  }
  getPreviewText() {
    return this.previewText;
  }
  getCatalogueSections() {
    return this.catalogueSections;
  }
  getCatalogueRevision() {
    return this.catalogueRevision;
  }
  getExtensionRevision() {
    return this.extensionRevision;
  }
  getExtensionEntries() {
    return proceduralExtensionHost.listEntries();
  }
  getPreviewItems() {
    return this.previewItems;
  }
  getSelectedNodeIds() {
    return this.selectedNodeIds;
  }
  getPreselectNodeIds() {
    return this.preselectNodeIds;
  }
  getPreselectRemovedNodeIds() {
    return this.preselectRemovedNodeIds;
  }
  getSelectionMode() {
    return this.selectionMode;
  }
  getSelectionMethod() {
    return this.selectionMethod;
  }
  getHoveredNodeId() {
    return this.hoveredNodeId;
  }
  getHoveredChannel() {
    return this.hoveredChannel;
  }
  getSelectedChannels() {
    return this.selectedChannels;
  }
  getHoveredGeometryTargets() {
    if (this.hoveredChannel) {
      return resolveGeometryTargets([this.hoveredChannel], null, this.previewItems, this.fixtureEdges);
    }
    if (this.hoveredNodeId) {
      return resolveGeometryTargets([], this.hoveredNodeId, this.previewItems, this.fixtureEdges);
    }
    return [];
  }
  getSelectedGeometryTargets() {
    if (this.selectedChannels.length > 0) {
      return resolveGeometryTargets(this.selectedChannels, null, this.previewItems, this.fixtureEdges);
    }
    if (this.selectedNodeIds.length > 0) {
      const targets = [];
      for (const widgetId of this.selectedNodeIds) {
        targets.push(...resolveGeometryTargets([], widgetId, this.previewItems, this.fixtureEdges));
      }
      return targets;
    }
    return [];
  }
  getPreviewOffNodeIds() {
    return this.previewOffNodeIds;
  }
  getShowMode() {
    return this.showMode;
  }
  getInteractionRevision() {
    return this.interactionRevision;
  }
  getTransformGranularity() {
    return this.transformGranularity;
  }
  getGumballActiveWidgetIds() {
    return this.gumballActiveWidgetIds;
  }
  gumballBindingKey(sourceWidgetId, op) {
    return `${sourceWidgetId}:${op}`;
  }
  registerGumballBinding(binding) {
    this.gumballBindings.set(this.gumballBindingKey(binding.sourceWidgetId, binding.op), binding);
    this.gumballBindingByTransformId.set(binding.transformId, binding);
  }
  findGumballBinding(widgetId, op) {
    const byTransform = this.gumballBindingByTransformId.get(widgetId);
    if (byTransform && byTransform.op === op) return byTransform;
    const bySource = this.gumballBindings.get(this.gumballBindingKey(widgetId, op));
    return bySource ?? null;
  }
  resolveGumballSourceWidgetId(widgetId, op) {
    const byTransform = this.gumballBindingByTransformId.get(widgetId);
    if (byTransform && byTransform.op === op) return byTransform.sourceWidgetId;
    return widgetId;
  }
  clearGumballDrag() {
    this.gumballDragSession = null;
    this.gumballActiveWidgetIds = [];
  }
  syncGumballActiveChrome(binding) {
    const nextActive = [binding.transformId, binding.sourceWidgetId];
    if (JSON.stringify(nextActive) !== JSON.stringify(this.gumballActiveWidgetIds)) {
      this.gumballActiveWidgetIds = nextActive;
      this.interactionRevision += 1;
      this.notifySnapshot();
    }
  }
  dispatchFlowCanvasSelection(ids) {
    this.run("canvasCommand", { command: "setSelection", argsJson: JSON.stringify({ ids: [...ids] }) });
  }
  dispatchGraphEdit(ops, selectTransformId) {
    this.run("canvasCommand", { command: "graphEdit", argsJson: JSON.stringify({ ops }) });
    const binding = this.gumballDragSession?.binding;
    if (binding) {
      this.dispatchFlowCanvasSelection(gumballBindingNodeIds(binding));
      this.syncGumballActiveChrome(binding);
      return;
    }
    if (selectTransformId) {
      this.run("setSelection", { ids: [selectTransformId], mode: "default" });
    }
  }
  applyLiveGumballDrag(request) {
    const session = this.gumballDragSession;
    if (!session) return;
    const values = applyGumballDeltaToBase(session.baseValues, session.binding.op, request.delta);
    setGumballBindingValues(session.binding, values);
    this.dispatchGraphEdit(this.buildGumballUpdateOps(session.binding));
  }
  beginGumballDrag(request) {
    const op = request.delta.op;
    let binding = this.findGumballBinding(request.widgetId, op);
    let insertOps = null;
    if (!binding) {
      const sourceWidgetId = this.resolveGumballSourceWidgetId(request.widgetId, op);
      const created = this.buildGumballInsertOps(sourceWidgetId, op, gumballZeroDelta(op), request.granularity);
      this.registerGumballBinding(created.binding);
      binding = created.binding;
      insertOps = created.ops;
      console.log(`[DEBUG] gumball insert ${binding.transformId} source=${sourceWidgetId} op=${op} granularity=${request.granularity}`);
    }
    this.gumballDragSession = { binding, baseValues: copyGumballValues(binding) };
    const values = applyGumballDeltaToBase(this.gumballDragSession.baseValues, op, request.delta);
    setGumballBindingValues(binding, values);
    if (insertOps) {
      this.dispatchGraphEdit(insertOps);
      return;
    }
    this.dispatchGraphEdit(this.buildGumballUpdateOps(binding));
  }
  finishGumballDrag(request) {
    const session = this.gumballDragSession;
    if (session) {
      const binding = session.binding;
      const values = applyGumballDeltaToBase(session.baseValues, binding.op, request.delta);
      setGumballBindingValues(binding, values);
      console.log(`[DEBUG] gumball end ${binding.transformId} op=${binding.op}`);
      this.clearGumballDrag();
      this.dispatchGraphEdit(this.buildGumballUpdateOps(binding));
      this.run("setSelection", { ids: [binding.transformId], mode: "default" });
      return;
    }
    this.applyGumballTransformCommitted(request);
  }
  applyGumballTransformCommitted(request) {
    const op = request.delta.op;
    const granularity = request.granularity;
    const existing = this.findGumballBinding(request.widgetId, op);
    if (existing) {
      accumulateGumballDelta(existing, request.delta);
      const ops2 = this.buildGumballUpdateOps(existing);
      console.log(`[DEBUG] gumball update ${existing.transformId} op=${op} granularity=${granularity}`);
      this.dispatchGraphEdit(ops2, existing.transformId);
      return;
    }
    const sourceWidgetId = this.resolveGumballSourceWidgetId(request.widgetId, op);
    const { ops, binding } = this.buildGumballInsertOps(sourceWidgetId, op, request.delta, granularity);
    this.registerGumballBinding(binding);
    console.log(`[DEBUG] gumball insert ${binding.transformId} source=${sourceWidgetId} op=${op} granularity=${granularity}`);
    this.dispatchGraphEdit(ops, binding.transformId);
  }
  buildGumballUpdateOps(binding) {
    if (binding.granularity === "compact") {
      return [{ op: "setNeuronParams", id: binding.transformId, paramsJson: JSON.stringify(compactNeuronParams(binding)) }];
    }
    if (binding.op === "translate" && binding.vectorId && binding.valueWidgetIds.length === 3) {
      const [sx, sy, sz] = binding.valueWidgetIds;
      const [x, y, z] = binding.values.offset;
      return [
        { op: "setSliderValue", id: sx, value: x },
        { op: "setSliderValue", id: sy, value: y },
        { op: "setSliderValue", id: sz, value: z },
      ];
    }
    const sliderId = binding.valueWidgetIds[0];
    if (!sliderId) return [];
    if (binding.op === "rotate") {
      return [{ op: "setSliderValue", id: sliderId, value: binding.values.angle }];
    }
    return [{ op: "setSliderValue", id: sliderId, value: binding.values.factor }];
  }
  buildGumballInsertOps(sourceWidgetId, op, delta, granularity) {
    const sourceLayout = widgetLayoutFromFixture(this.fixtureJson, sourceWidgetId);
    const edgeGap = gumballColumnEdgeGap(this.layerSpacing, this.siblingGap);
    const valueRowGap = gumballValueRowGap(this.siblingGap);
    const sourceHalf = GUMBALL_SOURCE_HALF_WIDTH;
    const sliderHalf = GUMBALL_SLIDER_HALF_WIDTH;
    const vectorHalf = GUMBALL_VECTOR_HALF_WIDTH;
    const transformHalf = GUMBALL_NEURON_HALF_WIDTH;
    const transformId = `${sourceWidgetId}_gumball_${op}`;
    const vectorId = `${transformId}_vector`;
    const sliderXId = `${transformId}_sx`;
    const sliderYId = `${transformId}_sy`;
    const sliderZId = `${transformId}_sz`;
    const scalarSliderId = `${transformId}_value`;
    const binding = {
      sourceWidgetId,
      transformId,
      op,
      granularity,
      valueWidgetIds: [],
      vectorId: void 0,
      values: {
        offset: delta.op === "translate" ? [delta.offset[0], delta.offset[1], delta.offset[2]] : [0, 0, 0],
        angle: delta.op === "rotate" ? delta.angle : 0,
        factor: delta.op === "scale" ? delta.factor : 1,
      },
    };
    let transformColumnX = gumballColumnAfter(sourceLayout.x, sourceHalf, transformHalf, edgeGap);
    const ops = [];
    if (granularity === "compact") {
      ops.push({ op: "makeSpace", anchor: sourceWidgetId, dx: gumballMakeSpaceDx(transformColumnX, transformHalf, sourceLayout.x, edgeGap), dy: 0 });
      ops.push({
        op: "addWidget",
        descriptor: neuronDescriptor(transformId, BREP_XFORM_NEURON_KIND[op]),
        x: transformColumnX,
        y: sourceLayout.y,
      });
      ops.push({ op: "setNeuronParams", id: transformId, paramsJson: JSON.stringify(compactNeuronParams(binding)) });
    } else if (op === "translate") {
      binding.valueWidgetIds = [sliderXId, sliderYId, sliderZId];
      binding.vectorId = vectorId;
      const valueColumnX = gumballColumnAfter(sourceLayout.x, sourceHalf, sliderHalf, edgeGap);
      const vectorColumnX = gumballColumnAfter(valueColumnX, sliderHalf, vectorHalf, edgeGap);
      transformColumnX = gumballColumnAfter(vectorColumnX, vectorHalf, transformHalf, edgeGap);
      ops.push({ op: "makeSpace", anchor: sourceWidgetId, dx: gumballMakeSpaceDx(transformColumnX, transformHalf, sourceLayout.x, edgeGap), dy: 0 });
      const [x, y, z] = binding.values.offset;
      ops.push(
        { op: "addWidget", descriptor: sliderDescriptor(sliderXId, x), x: valueColumnX, y: sourceLayout.y - valueRowGap },
        { op: "addWidget", descriptor: sliderDescriptor(sliderYId, y), x: valueColumnX, y: sourceLayout.y },
        { op: "addWidget", descriptor: sliderDescriptor(sliderZId, z), x: valueColumnX, y: sourceLayout.y + valueRowGap },
        { op: "addWidget", descriptor: neuronDescriptor(vectorId, "brep.vector"), x: vectorColumnX, y: sourceLayout.y },
        { op: "addWidget", descriptor: neuronDescriptor(transformId, BREP_XFORM_NEURON_KIND.translate), x: transformColumnX, y: sourceLayout.y },
        { op: "connectPorts", from: sliderXId, fromPort: "number", to: vectorId, toPort: "x" },
        { op: "connectPorts", from: sliderYId, fromPort: "number", to: vectorId, toPort: "y" },
        { op: "connectPorts", from: sliderZId, fromPort: "number", to: vectorId, toPort: "z" },
        { op: "connectPorts", from: vectorId, fromPort: "vector", to: transformId, toPort: "offset" },
      );
    } else {
      binding.valueWidgetIds = [scalarSliderId];
      const valueColumnX = gumballColumnAfter(sourceLayout.x, sourceHalf, sliderHalf, edgeGap);
      transformColumnX = gumballColumnAfter(valueColumnX, sliderHalf, transformHalf, edgeGap);
      ops.push({ op: "makeSpace", anchor: sourceWidgetId, dx: gumballMakeSpaceDx(transformColumnX, transformHalf, sourceLayout.x, edgeGap), dy: 0 });
      const scalarValue = op === "rotate" ? binding.values.angle : binding.values.factor;
      ops.push(
        { op: "addWidget", descriptor: sliderDescriptor(scalarSliderId, scalarValue), x: valueColumnX, y: sourceLayout.y },
        { op: "addWidget", descriptor: neuronDescriptor(transformId, BREP_XFORM_NEURON_KIND[op]), x: transformColumnX, y: sourceLayout.y },
        {
          op: "connectPorts",
          from: scalarSliderId,
          fromPort: "number",
          to: transformId,
          toPort: op === "rotate" ? "angle" : "factor",
        },
      );
    }
    ops.push({
      op: "insertBetween",
      anchor: sourceWidgetId,
      anchorOutPort: "solid",
      mid: transformId,
      midInPort: "geometry",
      midOutPort: "geometry",
    });
    ops.push({ op: "setPreviewOff", ids: [sourceWidgetId] });
    return { ops, binding };
  }
  /** @emoji 🎛️ Inserts or updates gumball-driven transform nodes in the flow graph. */
  applyGumballTransform(request) {
    const phase = request.phase ?? "end";
    if (phase === "start") {
      this.beginGumballDrag(request);
      return;
    }
    if (phase === "live") {
      this.applyLiveGumballDrag(request);
      return;
    }
    this.finishGumballDrag(request);
  }
  lodModeForScope(scopeId) {
    return this.lodModeByInstance[scopeId] ?? this.lodMode;
  }
  proximityDistanceValue() {
    return this.proximityDistance;
  }
  lodMeasure(scopeId) {
    return {
      kind: "select",
      id: `${scopeId}-lod`,
      label: "LOD",
      value: this.lodModeForScope(scopeId),
      items: [{ id: "automatic", value: DAG_LOD_MODE_AUTOMATIC, label: dagLodAutomaticSelectLabel(this.effectiveLod) }, ...dagPlayLodTiers().map((tier) => ({ id: tier, value: tier, label: dagPlayLodTierMenuLabel(tier) }))],
      onChange: { controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "setLodMode", args: { instanceId: scopeId } },
    };
  }
  proximityMeasure() {
    return {
      kind: "slider",
      id: "procedural-flow-proximity-distance",
      label: "Proximity",
      value: this.proximityDistance,
      min: 0,
      max: 240,
      step: 4,
      onChange: { controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "setProximityDistance" },
    };
  }
  flowWindowMeasures() {
    return [this.lodMeasure(PROCEDURAL_PLAY_WINDOW_KIND_ID), this.proximityMeasure()];
  }
  previewWindowMeasures() {
    return [
      {
        kind: "select",
        id: `${PROCEDURAL_PLAY_WINDOW_KIND_PREVIEW}-show`,
        label: "Show",
        value: this.showMode,
        items: [
          { id: "everything", value: "everything", label: "Everything" },
          { id: "selected", value: "selected", label: "Selected" },
        ],
        onChange: { controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "setShowMode" },
      },
      {
        kind: "select",
        id: `${PROCEDURAL_PLAY_WINDOW_KIND_PREVIEW}-transform-granularity`,
        label: "Transform Detail",
        value: this.transformGranularity,
        items: [
          { id: "full", value: "full", label: "Full (sliders + vector)" },
          { id: "compact", value: "compact", label: "Compact (node params)" },
        ],
        onChange: { controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "setTransformGranularity" },
      },
    ];
  }
  /** @emoji 🔔️ Subscribes to catalogue updates for workbench kinds panel refresh. */
  subscribeSnapshot(listener) {
    this.snapshotListeners.add(listener);
    return () => this.snapshotListeners.delete(listener);
  }
  notifySnapshot() {
    for (const listener of this.snapshotListeners) {
      listener();
    }
  }
  getReorganize() {
    return { epoch: this.reorganizeEpoch, optionsJson: this.reorganizeOptionsJson };
  }
  getCommandRequest() {
    return { epoch: this.commandRequestEpoch, ...this.commandRequestPayload };
  }
  syncReorganizeOptionsJson() {
    this.reorganizeOptionsJson = buildProceduralLayoutOptionsJson(this.layerSpacing, this.siblingGap, this.orientation);
  }
  triggerReorganize() {
    this.syncReorganizeOptionsJson();
    this.reorganizeEpoch += 1;
    this.rebuildShellMode();
    this.emit();
  }
  flowWindowEngagement() {
    return {
      sessionActive: false,
      input: {
        id: "engagement-input",
        value: this.engagementInput,
        placeholder: "Reorganize, lr, tb",
        onChange: proceduralPlayCmd("engagementInput"),
        onSubmit: proceduralPlayCmd("engagementSubmit"),
      },
      possibleEngagements: [
        { id: "procedural.tool.reorganize", label: "Reorganize", command: proceduralPlayCmd("reorganize") },
        { id: "procedural.layout.leftRight", label: "Left to Right", command: proceduralPlayCmd("setOrientation", { orientation: "leftRight" }) },
        { id: "procedural.layout.topBottom", label: "Top to Bottom", command: proceduralPlayCmd("setOrientation", { orientation: "topBottom" }) },
      ],
      controls: [
        {
          kind: "slider",
          id: "procedural-layer-spacing",
          label: "Layer spacing",
          value: this.layerSpacing,
          min: 40,
          max: 320,
          step: 10,
          onChange: proceduralPlayCmd("setSpacing", { field: "layerSpacing" }),
        },
        {
          kind: "slider",
          id: "procedural-sibling-gap",
          label: "Sibling gap",
          value: this.siblingGap,
          min: 10,
          max: 160,
          step: 5,
          onChange: proceduralPlayCmd("setSpacing", { field: "siblingGap" }),
        },
      ],
      status: [{ id: "procedural-layout-orientation", text: this.orientation === "leftRight" ? "Left to right" : "Top to bottom" }],
    };
  }
  previewWindowEngagement() {
    return {
      sessionActive: false,
      input: {
        id: "preview-engagement-input",
        value: "",
        placeholder: "Preview",
        onChange: proceduralPlayCmd("previewEngagementInput"),
        onSubmit: proceduralPlayCmd("previewEngagementSubmit"),
      },
      status: [{ id: "procedural-preview-item-count", text: `${this.previewItems.length} preview items` }],
    };
  }
  rebuildShellMode() {
    this.mainMode.windowKinds = [
      new WindowKindRuntime(PROCEDURAL_PLAY_WINDOW_KIND_ID, "Flow", PROCEDURAL_PLAY_BODY_KEY_MAIN, void 0, this.flowWindowMeasures(), this.flowWindowEngagement()),
      new WindowKindRuntime(PROCEDURAL_PLAY_WINDOW_KIND_PREVIEW, "Preview", PROCEDURAL_PLAY_BODY_KEY_PREVIEW, void 0, this.previewWindowMeasures(), this.previewWindowEngagement()),
    ];
    for (const windowKind of this.mainMode.windowKinds) {
      enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Procedural play window "${windowKind.id}"`);
    }
    this.rebuildToolbarTools();
  }
  run(command, args) {
    if (command === "engagementInput") {
      const value = args.value;
      if (typeof value === "string" && value !== this.engagementInput) {
        this.engagementInput = value;
        this.rebuildShellMode();
        this.emit();
      }
      return;
    }
    if (command === "engagementSubmit") {
      const value = args.value ?? this.engagementInput;
      this.applyEngagement(value);
      return;
    }
    if (command === "setSpacing") {
      const field = args.field;
      const value = args.value;
      if (typeof value !== "number") return;
      if (field === "layerSpacing") this.layerSpacing = value;
      else if (field === "siblingGap") this.siblingGap = value;
      else return;
      this.syncReorganizeOptionsJson();
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "setOrientation") {
      const orientation = args.orientation;
      if (orientation !== "leftRight" && orientation !== "topBottom") return;
      this.orientation = orientation;
      this.syncReorganizeOptionsJson();
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "reorganize") {
      this.triggerReorganize();
      return;
    }
    if (command === "canvasCommand") {
      const canvasCommand = args.command;
      if (typeof canvasCommand !== "string" || !canvasCommand) return;
      const argsJson = args.argsJson;
      this.commandRequestPayload = { command: canvasCommand, ...(argsJson !== void 0 ? { argsJson } : {}) };
      this.commandRequestEpoch += 1;
      this.emit();
      return;
    }
    if (command === "setFixtureJson") {
      const { json, resetInteraction } = args;
      if (typeof json === "string") {
        this.applyFixtureJson(json, resetInteraction === true);
      }
      return;
    }
    if (command === "setActiveFixture") {
      if (isPlaygroundFixtureLocked()) return;
      const fixtureId = args.fixtureId ?? "";
      this.loadFixtureById(fixtureId);
      return;
    }
    if (command === "saveStored") {
      this.fixtureStore.save(this.fixtureJson);
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "saveDownload" || command === "loadRequest") {
      this.hostBridge?.runHostCommand(command, args);
      return;
    }
    if (command === "loadStored") {
      const json = this.fixtureStore.load();
      if (json) this.applyFixtureJson(json, true);
      return;
    }
    if (command === "resetFixture") {
      this.fixtureStore.clear();
      this.activeFixtureId = PLAYGROUND_NO_FIXTURE_ID;
      this.applyFixtureJson(PROCEDURAL_PLAY_EMPTY_FIXTURE_JSON, true);
      return;
    }
    if (command === "setLodMode") {
      const { value, instanceId } = args;
      const scopeId = instanceId ?? PROCEDURAL_PLAY_WINDOW_KIND_ID;
      if (typeof value !== "string") return;
      if (value !== DAG_LOD_MODE_AUTOMATIC && !isDagDrawLodKind(value)) return;
      this.lodModeByInstance = { ...this.lodModeByInstance, [scopeId]: value };
      if (scopeId === PROCEDURAL_PLAY_WINDOW_KIND_ID) {
        this.lodMode = value;
      }
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "setEffectiveLod") {
      const { lod, instanceId } = args;
      const scopeId = instanceId ?? PROCEDURAL_PLAY_WINDOW_KIND_ID;
      if (!lod || !isDagDrawLodKind(lod)) return;
      if (scopeId !== PROCEDURAL_PLAY_WINDOW_KIND_ID) return;
      if (this.effectiveLod === lod) return;
      this.effectiveLod = lod;
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "setProximityDistance") {
      const value = args.value;
      if (typeof value !== "number" || !Number.isFinite(value)) return;
      const next = Math.max(0, value);
      if (this.proximityDistance === next) return;
      this.proximityDistance = next;
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "setPreviewText") {
      const text = args.text;
      if (typeof text === "string" && text !== this.previewText) {
        this.previewText = text;
        this.emit();
      }
      return;
    }
    if (command === "setEvalOutputs") {
      const outputsJson = args.outputsJson;
      const previewMeshes = args.previewMeshes;
      if (typeof outputsJson === "string") {
        const nextItems = previewItemsWithMeshes(extractChannelPreviewItems(outputsJson), previewMeshes, this.previewItems);
        this.previewItems = nextItems;
        this.interactionRevision += 1;
        this.notifySnapshot();
        this.rebuildShellMode();
        this.emit();
      }
      return;
    }
    if (command === "setSelection") {
      const ids = args.ids;
      const mode = args.mode ?? "default";
      const fromFlow = args.fromFlow === true;
      if (!Array.isArray(ids)) return;
      if (fromFlow && this.gumballDragSession) return;
      const next = selectionMergeIds(mode, this.selectedNodeIds, ids);
      if (JSON.stringify(next) === JSON.stringify(this.selectedNodeIds)) return;
      this.selectedNodeIds = next;
      this.selectedChannels = [];
      this.preselectNodeIds = [];
      this.preselectRemovedNodeIds = [];
      this.interactionRevision += 1;
      this.notifySnapshot();
      this.rebuildToolbarTools();
      this.emit();
      return;
    }
    if (command === "renameFlowWidget") {
      const oldId = args.oldId;
      const value = args.value;
      if (typeof oldId === "string" && typeof value === "string") {
        this.renameFlowWidget(oldId, value);
      }
      return;
    }
    if (command === "patchFlowWidget") {
      const widgetId = args.widgetId;
      const field = args.field;
      const value = args.value;
      if (typeof widgetId === "string" && typeof field === "string") {
        this.patchFlowWidget(widgetId, field, value);
      }
      return;
    }
    if (command === "setPreselect") {
      const ids = args.ids;
      const removedIds = args.removedIds;
      if (!Array.isArray(ids) || !Array.isArray(removedIds)) return;
      this.preselectNodeIds = [...ids];
      this.preselectRemovedNodeIds = [...removedIds];
      this.interactionRevision += 1;
      this.notifySnapshot();
      return;
    }
    if (command === "setSelectionMode") {
      const mode = args.mode;
      if (mode !== "default" && mode !== "additive" && mode !== "subtractive" && mode !== "invertive") return;
      if (this.selectionMode === mode) return;
      this.selectionMode = mode;
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "setSelectionMethod") {
      const method = args.method;
      if (method !== "rectangle" && method !== "lasso") return;
      if (this.selectionMethod === method) return;
      this.selectionMethod = method;
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "selectAll") {
      const ids = [...new Set(this.previewItems.map((entry) => entry.widgetId))];
      this.selectedNodeIds = [...new Set(ids)];
      this.preselectNodeIds = [];
      this.preselectRemovedNodeIds = [];
      this.interactionRevision += 1;
      this.notifySnapshot();
      this.emit();
      return;
    }
    if (command === "clearSelection") {
      if (!this.selectedNodeIds.length) return;
      this.selectedNodeIds = [];
      this.preselectNodeIds = [];
      this.preselectRemovedNodeIds = [];
      this.interactionRevision += 1;
      this.notifySnapshot();
      this.rebuildToolbarTools();
      this.emit();
      return;
    }
    if (command === "deleteSelection") {
      this.run("canvasCommand", { command: "deleteSelection" });
      return;
    }
    if (command === "setHover") {
      const id = args.id;
      const channel = args.channel ?? null;
      const next = typeof id === "string" ? id : null;
      const channelJson = channel ? JSON.stringify(channel) : "null";
      const currentChannelJson = this.hoveredChannel ? JSON.stringify(this.hoveredChannel) : "null";
      if (next === this.hoveredNodeId && channelJson === currentChannelJson) return;
      this.hoveredNodeId = next;
      this.hoveredChannel = channel;
      this.interactionRevision += 1;
      this.notifySnapshot();
      return;
    }
    if (command === "setSelectedChannels" || command === "setSelectChannels") {
      const channels = args.channels;
      if (!Array.isArray(channels)) return;
      const next = [...channels];
      if (JSON.stringify(next) === JSON.stringify(this.selectedChannels)) return;
      this.selectedChannels = next;
      this.selectedNodeIds = [...new Set(next.map((channel) => channel.widgetId))];
      this.preselectNodeIds = [];
      this.preselectRemovedNodeIds = [];
      this.interactionRevision += 1;
      this.notifySnapshot();
      this.rebuildToolbarTools();
      this.emit();
      return;
    }
    if (command === "setHoverChannel") {
      const channel = args.channel ?? null;
      this.run("setHover", { id: channel?.widgetId ?? null, channel });
      return;
    }
    if (command === "togglePreview") {
      const id = args.id;
      if (typeof id !== "string") return;
      const off = new Set(this.previewOffNodeIds);
      if (off.has(id)) off.delete(id);
      else off.add(id);
      this.previewOffNodeIds = [...off];
      this.interactionRevision += 1;
      this.notifySnapshot();
      this.emit();
      return;
    }
    if (command === "setPreviewOff") {
      const ids = args.ids;
      const fromFlow = args.fromFlow === true;
      if (!Array.isArray(ids)) return;
      if (fromFlow && this.gumballDragSession) {
        const next = [...ids];
        if (JSON.stringify(next) === JSON.stringify(this.previewOffNodeIds)) return;
        this.previewOffNodeIds = next;
        this.interactionRevision += 1;
        this.notifySnapshot();
        return;
      }
      this.previewOffNodeIds = [...ids];
      this.interactionRevision += 1;
      this.notifySnapshot();
      this.emit();
      return;
    }
    if (command === "setShowMode") {
      const id = args.id ?? args.value;
      if (id !== "everything" && id !== "selected") return;
      if (this.showMode === id) return;
      this.showMode = id;
      this.interactionRevision += 1;
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "setTransformGranularity") {
      const granularity = args.granularity ?? args.value;
      if (granularity !== "compact" && granularity !== "full") return;
      if (this.transformGranularity === granularity) return;
      this.transformGranularity = granularity;
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (command === "applyGumballTransform") {
      const widgetId = args.widgetId;
      const delta = args.delta;
      const granularity = args.granularity ?? this.transformGranularity;
      const phase = args.phase;
      if (typeof widgetId !== "string" || !delta) return;
      this.applyGumballTransform({ widgetId, delta, granularity, phase });
      return;
    }
    if (command === "setCatalogueSections") {
      const sections = args.sections;
      if (Array.isArray(sections)) {
        this.catalogueSections = sections;
        this.catalogueRevision += 1;
        this.notifySnapshot();
        this.emit();
      }
      return;
    }
    if (command === "toggleExtension") {
      const id = args.id;
      const enabled = args.enabled;
      if (typeof id !== "string" || typeof enabled !== "boolean") return;
      void proceduralExtensionHost.setActive(id, enabled).then(() => {
        this.extensionRevision += 1;
        this.notifySnapshot();
        this.emit();
      });
      return;
    }
    if (command === "runExtensionCommand") {
      const commandId = args.commandId;
      if (typeof commandId !== "string") return;
      const result = proceduralExtensionHost.executeCommand(commandId);
      console.log(`[DEBUG] procedural extension command ${commandId}: ${result}`);
      this.emit();
      return;
    }
  }
  applyEngagement(value) {
    const trimmed = value.trim().toLowerCase();
    if (!trimmed) return;
    if (trimmed === "reorganize" || trimmed === "layout") {
      this.triggerReorganize();
      return;
    }
    if (trimmed === "lr" || trimmed === "left" || trimmed === "left to right") {
      this.orientation = "leftRight";
      this.syncReorganizeOptionsJson();
      this.rebuildShellMode();
      this.emit();
      return;
    }
    if (trimmed === "tb" || trimmed === "top" || trimmed === "top to bottom") {
      this.orientation = "topBottom";
      this.syncReorganizeOptionsJson();
      this.rebuildShellMode();
      this.emit();
      return;
    }
    this.engagementInput = "";
    this.rebuildShellMode();
    this.emit();
  }
}
export function registerProceduralPlayDeclarativeBodies() {
  registerWindowBody(PROCEDURAL_PLAY_BODY_KEY_MAIN, (_ctx) => buildFlowWindowBody(PROCEDURAL_PLAY_SURFACE_ID, PROCEDURAL_3D_PLAY_CONTROLLER_ID, PROCEDURAL_PLAY_WINDOW_KIND_ID));
  registerWindowBody(PROCEDURAL_PLAY_BODY_KEY_PREVIEW, (_ctx) => buildPuzzle3dWindowBody(PROCEDURAL_PLAY_SURFACE_ID_PREVIEW, PROCEDURAL_3D_PLAY_CONTROLLER_ID));
}
export function buildProceduralPlayAppRuntime(controller) {
  return createPlayAppRuntime(PROCEDURAL_3D_PLAY_APP_ID, "semio · procedural", controller, PROCEDURAL_PLAY_LAYOUT, controller.mainMode);
}
export class PlaygroundProcedural extends Playground {
  id = PROCEDURAL_3D_PLAY_APP_ID;
  keybindings = [
    { key: "ctrl+a,meta+a", controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "selectAll" },
    { key: "Delete", controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
    { key: "Backspace", controllerId: PROCEDURAL_3D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
  ];
  createRuntime() {
    const runtime = createProductPlaygroundPlatform(this.id);
    const ctrl = new ProceduralPlayController(runtime.commandBus, () => runtime.notify());
    runtime.addApp(buildProceduralPlayAppRuntime(ctrl));
    return runtime;
  }
  registerBodies() {
    registerProceduralPlayDeclarativeBodies();
  }
}
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("@semio-tech/procedural-3d-play", () => {
    it("exports default fixture json", () => {
      expect(PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON).toContain("flow.fixture/v1");
    });
    it("starts with no fixture selected", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      expect(ctrl.getFixtureCatalog().activeFixtureId).toBe(PLAYGROUND_NO_FIXTURE_ID);
      expect(ctrl.getFixtureJson()).toContain('"widgets":[]');
    });
    it("does not auto-load stored fixture on startup", () => {
      const backing = /* @__PURE__ */ new Map();
      const store = createProceduralPlayFixtureStore({
        getItem: (k) => backing.get(k) ?? null,
        setItem: (k, v) => {
          backing.set(k, v);
        },
        removeItem: (k) => {
          backing.delete(k);
        },
      });
      store.save(PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON);
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {}, store);
      expect(ctrl.getFixtureJson()).toContain('"widgets":[]');
    });
    it("controller stores fixture json", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setFixtureJson", { json: '{"schema":"flow.fixture/v1"}' });
      expect(ctrl.getFixtureJson()).toContain("flow.fixture/v1");
    });
    it("kinds tree marks nested catalogue rows draggable", () => {
      const tree = buildProceduralPlayKindsTree([
        {
          id: "brep",
          title: "Brep",
          items: [],
          groups: [
            {
              id: "brep.primitives-3d",
              title: "Primitives 3D",
              items: [{ kind: "neuron", neuronKind: "brep.prim3d.box", name: "Box", abbreviation: "Box", icon: "emoji:📦️", summary: "Axis-aligned box" }],
            },
          ],
        },
      ]);
      expect(tree.type).toBe("tree");
      const leaf = tree.sections?.[0]?.items?.[0]?.items?.[0];
      expect(leaf?.draggable).toBe(true);
      expect(leaf?.dragData).toBeDefined();
    });
    it("catalogue snapshot listeners fire when sections arrive", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      let revision = ctrl.getCatalogueRevision();
      const unsubscribe = ctrl.subscribeSnapshot(() => {
        revision = ctrl.getCatalogueRevision();
      });
      ctrl.run("setCatalogueSections", { sections: [{ id: "brep", title: "Brep", items: [] }] });
      unsubscribe();
      expect(revision).toBe(1);
    });
    it("catalogue revision bumps when sections arrive", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      expect(ctrl.getCatalogueRevision()).toBe(0);
      ctrl.run("setCatalogueSections", {
        sections: [
          {
            id: "brep",
            title: "Brep",
            items: [],
            groups: [
              {
                id: "brep.primitives-3d",
                title: "Primitives 3D",
                items: [{ kind: "neuron", neuronKind: "brep.prim3d.box", name: "Box", abbreviation: "Box", icon: "emoji:📦️", summary: "Box" }],
              },
              {
                id: "brep.curves",
                title: "Curves",
                items: [{ kind: "neuron", neuronKind: "brep.curve.line", name: "Line", abbreviation: "Line", icon: "emoji:〰", summary: "Line edge" }],
              },
            ],
          },
        ],
      });
      expect(ctrl.getCatalogueRevision()).toBe(1);
    });
    it("catalogue revision bumps for nested brep groups", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setCatalogueSections", {
        sections: [
          {
            id: "brep",
            title: "Brep",
            items: [],
            groups: [
              { id: "brep.primitives-3d", title: "Primitives 3D", items: [] },
              { id: "brep.solid", title: "Solid", items: [] },
            ],
          },
        ],
      });
      expect(ctrl.getCatalogueSections()[0]?.groups?.length).toBe(2);
    });
    it("controller exposes flow and preview window kinds", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      expect(ctrl.mainMode.windowKinds).toHaveLength(2);
      expect(ctrl.mainMode.windowKinds[1]?.id).toBe(PROCEDURAL_PLAY_WINDOW_KIND_PREVIEW);
    });
    it("flow window exposes inline lod select", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      const measures = ctrl.mainMode.windowKinds[0]?.measures ?? [];
      expect(measures.some((measure) => measure.kind === "select" && measure.label === "LOD")).toBe(true);
    });
    it("flow window proximity measure defaults and updates via command", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      expect(ctrl.proximityDistanceValue()).toBe(FLOW_DEFAULT_PROXIMITY_DISTANCE);
      const measures = ctrl.mainMode.windowKinds[0]?.measures ?? [];
      const proximity = measures.find((measure) => measure.kind === "slider" && measure.label === "Proximity");
      expect(proximity?.kind).toBe("slider");
      if (proximity?.kind === "slider") {
        expect(proximity.value).toBe(FLOW_DEFAULT_PROXIMITY_DISTANCE);
      }
      ctrl.run("setProximityDistance", { value: 0 });
      expect(ctrl.proximityDistanceValue()).toBe(0);
      const updated = ctrl.mainMode.windowKinds[0]?.measures?.find((measure) => measure.kind === "slider" && measure.label === "Proximity");
      expect(updated?.kind).toBe("slider");
      if (updated?.kind === "slider") {
        expect(updated.value).toBe(0);
      }
    });
    it("preview window exposes show mode and transform detail in shell measures", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      const measures = ctrl.mainMode.windowKinds[1]?.measures ?? [];
      const show = measures.find((measure) => measure.kind === "select" && measure.label === "Show");
      expect(show?.kind === "select" && show.value).toBe("everything");
      expect(measures.some((measure) => measure.kind === "select" && measure.label === "Transform Detail")).toBe(true);
    });
    it("setTransformGranularity accepts shell measure value", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setTransformGranularity", { value: "compact" });
      expect(ctrl.getTransformGranularity()).toBe("compact");
    });
    it("setShowMode updates preview filter", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      expect(ctrl.getShowMode()).toBe("everything");
      ctrl.run("setShowMode", { id: "selected" });
      expect(ctrl.getShowMode()).toBe("selected");
    });
    it("setShowMode accepts shell measure value", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setShowMode", { value: "selected" });
      expect(ctrl.getShowMode()).toBe("selected");
      ctrl.run("setShowMode", { value: "everything" });
      expect(ctrl.getShowMode()).toBe("everything");
    });
    it("canvasCommand bumps command request epoch", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("canvasCommand", { command: "deleteSelection" });
      expect(ctrl.getCommandRequest().command).toBe("deleteSelection");
      expect(ctrl.getCommandRequest().epoch).toBe(1);
    });
    it("deleteSelection forwards to flow canvas command request", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setSelection", { ids: ["node-a"] });
      ctrl.run("deleteSelection");
      expect(ctrl.getCommandRequest().command).toBe("deleteSelection");
      expect(ctrl.getSelectedNodeIds()).toEqual(["node-a"]);
    });
    it("setPreviewOff stores preview-off node ids", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setPreviewOff", { ids: ["a", "b"] });
      expect(ctrl.getPreviewOffNodeIds()).toEqual(["a", "b"]);
    });
    it("buildProceduralPlayCanvasContextMenu adds isolate in preview for hovered node", () => {
      const items = buildProceduralPlayCanvasContextMenu(
        {
          hoveredNodeId: "box",
          selectedNodeIds: ["box"],
          clusterNodeIds: [],
          isImageWidget: false,
          isBackground: false,
          previewOffNodeIds: [],
          screen: { x: 0, y: 0 },
          world: { x: 0, y: 0 },
          clientX: 0,
          clientY: 0,
        },
        () => {},
      );
      expect(items.some((item) => item.id === "procedural.ctx.isolatePreview")).toBe(true);
    });
    it("setFixtureJson sync preserves preview items after flow interaction", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setEvalOutputs", {
        outputsJson: JSON.stringify({ box: { in: {}, out: { solid: { geometry: "solid-1" } } } }),
      });
      const base = ctrl.getFixtureJson();
      const interacted = JSON.stringify({
        ...JSON.parse(base),
        camera: { x: 12, y: -4, zoom: 2.5 },
        widgets: [
          { kind: "neuron", id: "sketch", neuronKind: "brep.sketch2d.rectangle" },
          { kind: "neuron", id: "solid", neuronKind: "brep.solid.extrude" },
          { kind: "outputPreview", id: "preview", preview: { geometry: "solid-9" } },
        ],
      });
      ctrl.run("setFixtureJson", { json: interacted });
      expect(ctrl.getPreviewItems()).toEqual([{ widgetId: "box", port: "solid", direction: "out", kind: "geometry", handle: "solid-1" }]);
    });
    it("setFixtureJson with resetInteraction clears preview items", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setEvalOutputs", {
        outputsJson: JSON.stringify({ box: { in: {}, out: { solid: { geometry: "solid-1" } } } }),
      });
      ctrl.run("setFixtureJson", {
        json: '{"schema":"flow.fixture/v1","camera":{"x":0,"y":0,"zoom":1},"widgets":[],"synapses":[]}',
        resetInteraction: true,
      });
      expect(ctrl.getPreviewItems()).toEqual([]);
    });
    it("setEvalOutputs stores preview items per widget", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setEvalOutputs", {
        outputsJson: JSON.stringify({ box: { in: {}, out: { solid: { geometry: "solid-1" } } } }),
      });
      expect(ctrl.getPreviewItems()).toEqual([{ widgetId: "box", port: "solid", direction: "out", kind: "geometry", handle: "solid-1" }]);
    });
    it("setEvalOutputs stores point and vector preview items", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setEvalOutputs", {
        outputsJson: JSON.stringify({
          pt: { in: {}, out: { point: { $schema: "point", x: 1, y: 0, z: 0 } } },
          vec: { in: {}, out: { vector: { $schema: "vector", x: 0, y: 1, z: 0 } } },
        }),
      });
      expect(ctrl.getPreviewItems()).toEqual([
        { widgetId: "pt", port: "point", direction: "out", kind: "point", position: [1, 0, 0] },
        { widgetId: "vec", port: "vector", direction: "out", kind: "vector", directionVec: [0, 1, 0] },
      ]);
    });
    it("selectAll includes widgets with point and vector preview items", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setEvalOutputs", {
        outputsJson: JSON.stringify({
          pt: { in: {}, out: { point: { $schema: "point", x: 0, y: 0, z: 0 } } },
          vec: { in: {}, out: { vector: { $schema: "vector", x: 1, y: 0, z: 0 } } },
        }),
      });
      ctrl.run("selectAll");
      expect(ctrl.getSelectedNodeIds().sort()).toEqual(["pt", "vec"]);
    });
    it("setHoverChannel and geometry target getters resolve upstream output", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setEvalOutputs", {
        outputsJson: JSON.stringify({
          circle: { in: {}, out: { wire: { geometry: "drawing-1" } } },
          offset: { in: { geometry: "drawing-1" }, out: { geometry: { geometry: "wire-2" } } },
        }),
      });
      ctrl.run("setFixtureJson", {
        json: JSON.stringify({
          schema: "flow.fixture/v1",
          camera: { x: 0, y: 0, zoom: 1 },
          widgets: [
            { kind: "neuron", id: "circle", neuronKind: "brep.sketch2d.circle" },
            { kind: "neuron", id: "offset", neuronKind: "brep.xform.offset" },
          ],
          synapses: [{ id: "s1", from: "circle", to: "offset", from_port: "wire", to_port: "geometry" }],
        }),
      });
      ctrl.run("setHoverChannel", {
        channel: { widgetId: "offset", port: "geometry", direction: "in" },
      });
      expect(ctrl.getHoveredChannel()).toEqual({ widgetId: "offset", port: "geometry", direction: "in" });
      expect(ctrl.getHoveredGeometryTargets()).toEqual([{ widgetId: "circle", port: "wire", direction: "out" }]);
    });
    it("parseFixtureEdges reads camelCase flow synapse ports", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setFixtureJson", {
        json: JSON.stringify({
          schema: "flow.fixture/v1",
          camera: { x: 0, y: 0, zoom: 1 },
          widgets: [],
          synapses: [
            { id: "e101", from: "brep_prim3d_sphere_2", to: "brep_bool_cut_5", fromPort: "solid", toPort: "a" },
            { id: "e102", from: "brep_prim3d_torus_4", to: "brep_bool_cut_5", fromPort: "solid", toPort: "b" },
          ],
        }),
      });
      expect(ctrl.getSelectedGeometryTargets()).toEqual([]);
      ctrl.run("setSelectChannels", {
        channels: [{ widgetId: "brep_bool_cut_5", port: "a", direction: "in" }],
      });
      ctrl.run("setEvalOutputs", {
        outputsJson: JSON.stringify({
          brep_prim3d_sphere_2: { in: {}, out: { solid: { geometry: "solid-sphere" } } },
          brep_bool_cut_5: { in: { a: { geometry: "solid-sphere" } }, out: { solid: { geometry: "solid-cut" } } },
        }),
      });
      expect(ctrl.getSelectedGeometryTargets()).toEqual([{ widgetId: "brep_prim3d_sphere_2", port: "solid", direction: "out" }]);
    });
    it("show selected reveals upstream geometry for preview-off input channels", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      const outputsJson = JSON.stringify({
        brep_prim3d_sphere_2: { in: {}, out: { solid: { geometry: "solid-sphere" } } },
        brep_prim3d_torus_4: { in: {}, out: { solid: { geometry: "solid-torus" } } },
        brep_bool_cut_5: {
          in: { a: { geometry: "solid-sphere" }, b: { geometry: "solid-torus" } },
          out: { solid: { geometry: "solid-cut" } },
        },
      });
      ctrl.run("setEvalOutputs", { outputsJson });
      ctrl.run("setFixtureJson", {
        json: JSON.stringify({
          schema: "flow.fixture/v1",
          camera: { x: 0, y: 0, zoom: 1 },
          widgets: [
            { kind: "neuron", id: "brep_prim3d_sphere_2", neuronKind: "brep.prim3d.sphere", preview: false },
            { kind: "neuron", id: "brep_prim3d_torus_4", neuronKind: "brep.prim3d.torus", preview: false },
            { kind: "neuron", id: "brep_bool_cut_5", neuronKind: "brep.bool.cut", preview: true },
          ],
          synapses: [
            { id: "e1", from: "brep_prim3d_sphere_2", to: "brep_bool_cut_5", fromPort: "solid", toPort: "a" },
            { id: "e2", from: "brep_prim3d_torus_4", to: "brep_bool_cut_5", fromPort: "solid", toPort: "b" },
          ],
        }),
      });
      ctrl.run("setPreviewOff", {
        ids: ["brep_prim3d_sphere_2", "brep_prim3d_torus_4"],
        fromFlow: true,
      });
      ctrl.run("setShowMode", { id: "selected" });
      ctrl.run("setSelectChannels", {
        channels: [{ widgetId: "brep_bool_cut_5", port: "a", direction: "in" }],
      });
      const visible = filterVisiblePreviewItems(ctrl.getPreviewItems(), {
        showMode: ctrl.getShowMode(),
        selectedNodeIds: [...ctrl.getSelectedNodeIds()],
        selectedChannels: [...ctrl.getSelectedChannels()],
        selectedGeometryTargets: [...ctrl.getSelectedGeometryTargets()],
        hoveredNodeId: null,
        hoveredChannel: null,
      });
      expect(visible).toEqual([
        {
          widgetId: "brep_prim3d_sphere_2",
          port: "solid",
          direction: "out",
          kind: "geometry",
          handle: "solid-sphere",
        },
      ]);
    });
    it("setSelectChannels stores channel selection and parent nodes", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setSelectChannels", {
        channels: [{ widgetId: "box", port: "solid", direction: "out" }],
      });
      expect(ctrl.getSelectedChannels()).toEqual([{ widgetId: "box", port: "solid", direction: "out" }]);
      expect(ctrl.getSelectedNodeIds()).toEqual(["box"]);
    });
    it("setSelection and setHover update interaction revision", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setSelection", { ids: ["box"] });
      ctrl.run("setHover", { id: "box" });
      expect(ctrl.getSelectedNodeIds()).toEqual(["box"]);
      expect(ctrl.getHoveredNodeId()).toBe("box");
      expect(ctrl.getInteractionRevision()).toBeGreaterThan(0);
    });
    it("setHover stores hovered channel", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setHover", { id: "offset", channel: { widgetId: "offset", port: "geometry", direction: "in" } });
      expect(ctrl.getHoveredChannel()).toEqual({ widgetId: "offset", port: "geometry", direction: "in" });
    });
    it("setSelection merges additively when mode is additive", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setSelection", { ids: ["a"], mode: "default" });
      ctrl.run("setSelection", { ids: ["b"], mode: "additive" });
      expect(ctrl.getSelectedNodeIds()).toEqual(["a", "b"]);
    });
    it("setSelectionMethod updates marquee method", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setSelectionMethod", { method: "lasso" });
      expect(ctrl.getSelectionMethod()).toBe("lasso");
    });
    it("buildProceduralPlayToolbarTools registers selection, save, view, and actions", () => {
      const tools = buildProceduralPlayToolbarTools(
        {
          selectionMethod: "rectangle",
          selectionMode: "default",
          showMode: "everything",
          selectionCount: 0,
          hasStoredFixture: false,
        },
        PROCEDURAL_3D_PLAY_CONTROLLER_ID,
      );
      expect(tools.selection?.some((row) => row.id === "procedural.select.rectangle")).toBe(true);
      expect(tools.save?.map((row) => row.id)).toEqual(["procedural.save.stored", "procedural.save.download", "procedural.save.load", "procedural.save.loadStored", "procedural.save.reset"]);
      expect(tools.save?.[3]?.disabled).toBe(true);
      expect(tools.view?.length).toBe(2);
      expect(tools.actions?.some((row) => row.id === "procedural.action.reorganize")).toBe(true);
    });
    it("controller exposes toolbar tools when host bridge is attached", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      expect(ctrl.mainMode.tools).toBeUndefined();
      ctrl.setHostBridge({
        getToolbarState: () => ({
          selectionMethod: "rectangle",
          selectionMode: "default",
          showMode: "everything",
          selectionCount: 0,
          hasStoredFixture: false,
        }),
        runHostCommand: () => {},
      });
      expect(ctrl.mainMode.tools?.selection?.length).toBeGreaterThan(0);
    });
    it("fixture store round-trips json", () => {
      const backing = /* @__PURE__ */ new Map();
      const store = createProceduralPlayFixtureStore({
        getItem: (k) => backing.get(k) ?? null,
        setItem: (k, v) => {
          backing.set(k, v);
        },
        removeItem: (k) => {
          backing.delete(k);
        },
      });
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {}, store);
      ctrl.run("saveStored");
      expect(ctrl.hasStoredFixture()).toBe(true);
      ctrl.run("setFixtureJson", { json: '{"schema":"flow.fixture/v1","widgets":[],"synapses":[]}' });
      ctrl.run("loadStored");
      expect(ctrl.getFixtureJson()).toContain("flow.fixture/v1");
    });
    it("setActiveFixture loads default and empty fixtures", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setActiveFixture", { fixtureId: PLAYGROUND_NO_FIXTURE_ID });
      expect(ctrl.getFixtureJson()).toContain('"widgets":[]');
      ctrl.run("setActiveFixture", { fixtureId: PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID });
      expect(ctrl.getFixtureJson()).toContain("brep.prim3d.box");
    });
    it("fixture catalog includes procedural/fixture files", () => {
      expect(PROCEDURAL_PLAY_FIXTURE_OPTIONS.some((option) => option.id === "sphere-cut-with-torus")).toBe(true);
      expect(PROCEDURAL_PLAY_FIXTURE_OPTIONS.find((option) => option.id === "sphere-cut-with-torus")?.label).toBe("Sphere Cut With Torus");
      expect(PROCEDURAL_PLAY_FIXTURE_OPTIONS.some((option) => option.id === PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID)).toBe(true);
    });
    it("resolveProceduralPlayFixtureSlug maps hexagonal-column shorthand", async () => {
      const { resolveProceduralPlayFixtureSlug: resolveProceduralPlayFixtureSlug2, PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID: PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID2 } = await import("/fixture-slugs.ts");
      expect(resolveProceduralPlayFixtureSlug2("hexagonal-column")).toBe(PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID2);
      expect(resolveProceduralPlayFixtureSlug2("column")).toBe(PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID2);
    });
    it("getFixtureCatalog returns null when fixture host is locked", () => {
      const prev = import.meta.env.PLAYGROUND_LOCKED_FIXTURE_ID;
      import.meta.env.PLAYGROUND_LOCKED_FIXTURE_ID = PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID;
      try {
        const bus = new CommandBus();
        const ctrl = new ProceduralPlayController(bus, () => {});
        expect(ctrl.getFixtureCatalog()).toBeNull();
        ctrl.run("setActiveFixture", { fixtureId: PROCEDURAL_PLAY_FIXTURE_DEFAULT_ID });
        expect(ctrl.getFixtureCatalog()).toBeNull();
      } finally {
        if (prev === void 0) {
          delete import.meta.env.PLAYGROUND_LOCKED_FIXTURE_ID;
        } else {
          import.meta.env.PLAYGROUND_LOCKED_FIXTURE_ID = prev;
        }
      }
    });
    it("locked fixture host loads file fixture on construct", () => {
      const prev = import.meta.env.PLAYGROUND_LOCKED_FIXTURE_ID;
      import.meta.env.PLAYGROUND_LOCKED_FIXTURE_ID = PROCEDURAL_PLAY_FIXTURE_HEXAGONAL_MUSHROOM_COLUMN_ID;
      try {
        const bus = new CommandBus();
        const ctrl = new ProceduralPlayController(bus, () => {});
        expect(ctrl.getFixtureJson()).toContain("brep.solid.extrude");
        expect(ctrl.getFixtureJson()).toContain("brep_curve_polygon_9");
      } finally {
        if (prev === void 0) {
          delete import.meta.env.PLAYGROUND_LOCKED_FIXTURE_ID;
        } else {
          import.meta.env.PLAYGROUND_LOCKED_FIXTURE_ID = prev;
        }
      }
    });
    it("setActiveFixture loads file fixtures from procedural/fixture", () => {
      const sphereCutId = "sphere-cut-with-torus";
      expect(proceduralPlayFixtureJson(sphereCutId)).toContain("brep.bool.cut");
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setActiveFixture", { fixtureId: sphereCutId });
      expect(ctrl.getFixtureJson()).toContain("brep.bool.cut");
      expect(ctrl.getFixtureJson()).toContain("brep.prim3d.sphere");
    });
    it("extensions tree lists installed modules", () => {
      const tree = buildProceduralPlayExtensionsTree([
        {
          id: "brep",
          active: true,
          manifest: {
            schema: "flow.module/v1",
            id: "brep",
            name: "Brep",
            version: "0.1.0",
            activationEvents: ["onStartup"],
            contributes: {
              neuronKinds: [{ id: "brep.prim3d.box", module: "brep", name: "Box", abbreviation: "Box", icon: "emoji:📦️", summary: "Box", inputs: [], outputs: ["geometry"] }],
              widgets: [],
              commands: [],
              settings: [],
            },
          },
        },
      ]);
      const labels = tree.sections?.flatMap((section) => section.items?.map((item) => item.label) ?? []) ?? [];
      expect(labels).toContain("Brep");
    });
    it("applyGumballTransform dispatches graphEdit insert then update", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setFixtureJson", {
        json: JSON.stringify({
          schema: "flow.fixture/v1",
          camera: { x: 0, y: 0, zoom: 1 },
          widgets: [{ kind: "neuron", id: "solid", neuronKind: "brep.prim3d.box" }],
          synapses: [],
          layout: { solid: { x: 100, y: 50 } },
        }),
      });
      ctrl.applyGumballTransform({
        widgetId: "solid",
        granularity: "compact",
        delta: { op: "translate", offset: [1, 0, 0] },
      });
      const insert = ctrl.getCommandRequest();
      expect(insert.command).toBe("graphEdit");
      const insertOps = JSON.parse(insert.argsJson ?? "{}").ops;
      expect(insertOps.some((op) => op.op === "insertBetween")).toBe(true);
      const makeSpace = insertOps.find((op) => op.op === "makeSpace");
      expect(makeSpace?.op === "makeSpace" && makeSpace.dx).toBeGreaterThan(120);
      ctrl.applyGumballTransform({
        widgetId: "solid_gumball_translate",
        granularity: "compact",
        delta: { op: "translate", offset: [0, 2, 0] },
      });
      const update = ctrl.getCommandRequest();
      const updateOps = JSON.parse(update.argsJson ?? "{}").ops;
      expect(updateOps).toEqual([{ op: "setNeuronParams", id: "solid_gumball_translate", paramsJson: JSON.stringify({ offset: [1, 2, 0] }) }]);
    });
    it("applyGumballTransform live drag updates without accumulating per frame", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setFixtureJson", {
        json: JSON.stringify({
          schema: "flow.fixture/v1",
          camera: { x: 0, y: 0, zoom: 1 },
          widgets: [{ kind: "neuron", id: "solid", neuronKind: "brep.prim3d.box" }],
          synapses: [],
          layout: { solid: { x: 100, y: 50 } },
        }),
      });
      ctrl.applyGumballTransform({
        widgetId: "solid",
        granularity: "compact",
        phase: "start",
        delta: { op: "translate", offset: [0, 0, 0] },
      });
      expect(ctrl.getGumballActiveWidgetIds()).toEqual(["solid_gumball_translate", "solid"]);
      ctrl.applyGumballTransform({
        widgetId: "solid",
        granularity: "compact",
        phase: "live",
        delta: { op: "translate", offset: [2, 0, 0] },
      });
      ctrl.applyGumballTransform({
        widgetId: "solid",
        granularity: "compact",
        phase: "end",
        delta: { op: "translate", offset: [3, 0, 0] },
      });
      const end = ctrl.getCommandRequest();
      const endOps = JSON.parse(end.argsJson ?? "{}").ops;
      expect(endOps).toEqual([{ op: "setNeuronParams", id: "solid_gumball_translate", paramsJson: JSON.stringify({ offset: [3, 0, 0] }) }]);
      expect(ctrl.getGumballActiveWidgetIds()).toEqual([]);
    });
    it("applyGumballTransform full translate lays out value, vector, and transform columns without overlap", () => {
      const bus = new CommandBus();
      const ctrl = new ProceduralPlayController(bus, () => {});
      ctrl.run("setFixtureJson", {
        json: JSON.stringify({
          schema: "flow.fixture/v1",
          camera: { x: 0, y: 0, zoom: 1 },
          widgets: [{ kind: "neuron", id: "solid", neuronKind: "brep.prim3d.box" }],
          synapses: [],
          layout: { solid: { x: 200, y: 0 } },
        }),
      });
      ctrl.applyGumballTransform({
        widgetId: "solid",
        granularity: "full",
        delta: { op: "translate", offset: [1, 2, 3] },
      });
      const insertOps = JSON.parse(ctrl.getCommandRequest().argsJson ?? "{}").ops;
      const positions = insertOps.filter((op) => op.op === "addWidget").map((op) => ({ id: JSON.parse(op.descriptor).id, x: op.x, y: op.y }));
      const byId = Object.fromEntries(positions.map((entry) => [entry.id, entry]));
      expect(byId.solid_gumball_translate_sx.x).toBeLessThan(byId.solid_gumball_translate_vector.x);
      expect(byId.solid_gumball_translate_vector.x).toBeLessThan(byId.solid_gumball_translate.x);
      expect(byId.solid_gumball_translate_sx.x - byId.solid_gumball_translate_sy.x).toBe(0);
      expect(Math.abs(byId.solid_gumball_translate_sx.y - byId.solid_gumball_translate_sy.y)).toBeGreaterThanOrEqual(32);
      const makeSpace = insertOps.find((op) => op.op === "makeSpace");
      expect(makeSpace?.op === "makeSpace" && makeSpace.dx).toBeGreaterThan(240);
      const sliderX = insertOps.find((op) => op.op === "addWidget" && JSON.parse(op.descriptor).id === "solid_gumball_translate_sx");
      expect(sliderX?.op).toBe("addWidget");
      expect(JSON.parse(sliderX.descriptor)).toEqual({ kind: "inputSlider", id: "solid_gumball_translate_sx", value: 1, min: 0, max: 1, step: 1 });
    });
  });
}
if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "procedural-3d") {
  bootstrapElementsSurfaceChromeDocument("system");
  void (async () => {
    await import("/globals.css");
    const { bootProceduralPlay } = await import("/@fs/Users/ueli/Documents/semio/framework/product/playground/renderer/react/index.tsx?playgroundEntry=puzzle-procedural-3d");
    bootProceduralPlay(new PlaygroundProcedural());
  })();
}

//# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbImluZGV4LnRzIl0sInNvdXJjZXNDb250ZW50IjpbIi8vICNyZWdpb24g8J+nskhlYWRlclxuLyoqIEBlbW9qaSDwn5SnIFByb2NlZHVyYWwgcGxheSBoYXJuZXNzIG9uIGBAc2VtaW8tdGVjaC9mcmFtZXdvcmstcGxheWdyb3VuZC1jb3JlYC4gKi9cbi8vICNlbmRyZWdpb24g8J+nskhlYWRlclxuXG5pbXBvcnQge1xuICAgIGJ1aWxkRmxvd1BsYXlDYXRhbG9ndWVUcmVlLFxuICAgIGJ1aWxkRmxvd1BsYXlIaWVyYXJjaHlUcmVlLFxuICAgIGJ1aWxkRmxvd1BsYXlJbnNwZWN0b3JUcmVlLFxuICAgIHBhcnNlRmxvd1BsYXlGaXh0dXJlSnNvbixcbn0gZnJvbSBcIkBzZW1pby10ZWNoL2Zsb3ctcGxheVwiO1xuaW1wb3J0IHtcbiAgICBidWlsZENhdGFsb2d1ZUtpbmRzVHJlZVNlY3Rpb25zLFxuICAgIGJ1aWxkRmxvd0NvbnRleHRNZW51SXRlbXMsXG4gICAgREFHX0xPRF9NT0RFX0FVVE9NQVRJQyxcbiAgICBkYWdMb2RBdXRvbWF0aWNTZWxlY3RMYWJlbCxcbiAgICBkYWdQbGF5TG9kVGllck1lbnVMYWJlbCxcbiAgICBkYWdQbGF5TG9kVGllcnMsXG4gICAgRkxPV19ERUZBVUxUX1BST1hJTUlUWV9ESVNUQU5DRSxcbiAgICBmbG93UGxheUNhdGFsb2d1ZUl0ZW1EcmFnRGF0YSxcbiAgICBmbG93U2Vuc2libGVTbGlkZXJSYW5nZSxcbiAgICBpc0RhZ0RyYXdMb2RLaW5kLFxuICAgIHR5cGUgQ2F0YWxvZ3VlU2VjdGlvbixcbiAgICB0eXBlIERhZ0RyYXdMb2RLaW5kLFxuICAgIHR5cGUgRGFnTG9kTW9kZUtpbmQsXG4gICAgdHlwZSBGbG93Q2FudmFzQ29tbWFuZFJlcXVlc3QsXG4gICAgdHlwZSBGbG93Q2FudmFzQ29udGV4dE1lbnVDb250ZXh0LFxuICAgIHR5cGUgRmxvd0NvbnRleHRNZW51RGlzcGF0Y2gsXG4gICAgdHlwZSBGbG93RXh0ZW5zaW9uRW50cnksXG4gICAgdHlwZSBGbG93R3JhcGhFZGl0T3AsXG4gICAgdHlwZSBGbG93UmVvcmdhbml6ZVJlcXVlc3QsXG59IGZyb20gXCJAc2VtaW8tdGVjaC9mbG93LXJlYWN0XCI7XG5pbXBvcnQgdHlwZSB7IFdpbmRvd01lYXN1cmUgfSBmcm9tIFwiQHNlbWlvLXRlY2gvZnJhbWV3b3JrLXBsYXlncm91bmQtY29yZVwiO1xuaW1wb3J0IHtcbiAgICBBcHBSdW50aW1lLFxuICAgIGJ1aWxkRmxvd1dpbmRvd0JvZHksXG4gICAgYnVpbGRQdXp6bGUzZFdpbmRvd0JvZHksXG4gICAgQ29tbWFuZEJ1cyxcbiAgICBDb250cm9sbGVyLFxuICAgIGNyZWF0ZURlZmF1bHRMYXlvdXQsXG4gICAgY3JlYXRlUGxheUFwcFJ1bnRpbWUsXG4gICAgY3JlYXRlUHJvZHVjdFBsYXlncm91bmRQbGF0Zm9ybSxcbiAgICBlbmZvcmNlUGxheWdyb3VuZFdpbmRvd0VuZ2FnZW1lbnRJbnB1dCxcbiAgICBpc1BsYXlncm91bmRGaXh0dXJlTG9ja2VkLFxuICAgIGlzUGxheWdyb3VuZE5vRml4dHVyZUlkLFxuICAgIE1vZGVSdW50aW1lLFxuICAgIFBsYXRmb3JtLFxuICAgIFBsYXlncm91bmQsXG4gICAgUExBWUdST1VORF9OT19GSVhUVVJFX0lELFxuICAgIHBsYXlncm91bmRSZXNvbHZlZEZpeHR1cmVJZCxcbiAgICByZWdpc3RlcldpbmRvd0JvZHksXG4gICAgV2luZG93S2luZFJ1bnRpbWUsXG4gICAgdHlwZSBBcHBUb29scyxcbiAgICB0eXBlIENvbW1hbmREZXNjcmlwdG9yLFxuICAgIHR5cGUgUGxheWdyb3VuZEZpeHR1cmVDYXRhbG9nLFxuICAgIHR5cGUgUGxheWdyb3VuZEZpeHR1cmVIb3N0LFxuICAgIHR5cGUgVG9vbEl0ZW0sXG4gICAgdHlwZSBVaU5vZGUsXG4gICAgdHlwZSBVaVRyZWVTZWN0aW9uTm9kZSxcbiAgICB0eXBlIFdpbmRvd0JvZHlWaWV3Q29udGV4dCxcbiAgICB0eXBlIFdpbmRvd0VuZ2FnZW1lbnQsXG59IGZyb20gXCJAc2VtaW8tdGVjaC9mcmFtZXdvcmstcGxheWdyb3VuZC1jb3JlXCI7XG5pbXBvcnQgeyBtZXNoVHJhbnNmZXJGcm9tUHJldmlld1BheWxvYWQgfSBmcm9tIFwiQHNlbWlvLXRlY2gvZ2VvbWV0cnktYnJlcC1qc1wiO1xuaW1wb3J0IHtcbiAgICBleHRyYWN0Q2hhbm5lbFByZXZpZXdJdGVtcyxcbiAgICBmaWx0ZXJWaXNpYmxlUHJldmlld0l0ZW1zLFxuICAgIFBST0NFRFVSQUxfREVGQVVMVF9GSVhUVVJFLFxuICAgIHByb2NlZHVyYWxFeHRlbnNpb25Ib3N0LFxuICAgIHByb2NlZHVyYWxGaXh0dXJlVG9Kc29uLFxuICAgIHJlc29sdmVHZW9tZXRyeVRhcmdldHMsXG4gICAgdHlwZSBGbG93Rml4dHVyZVYxLFxuICAgIHR5cGUgUHJvY2VkdXJhbENoYW5uZWxSZWYsXG4gICAgdHlwZSBQcm9jZWR1cmFsRml4dHVyZUVkZ2UsXG4gICAgdHlwZSBQcm9jZWR1cmFsR3VtYmFsbFRyYW5zZm9ybURlbHRhLFxuICAgIHR5cGUgUHJvY2VkdXJhbEd1bWJhbGxUcmFuc2Zvcm1PcCxcbiAgICB0eXBlIFByb2NlZHVyYWxHdW1iYWxsVHJhbnNmb3JtUGhhc2UsXG4gICAgdHlwZSBQcm9jZWR1cmFsR3VtYmFsbFRyYW5zZm9ybVJlcXVlc3QsXG4gICAgdHlwZSBQcm9jZWR1cmFsUHJldmlld0l0ZW0sXG4gICAgdHlwZSBQcm9jZWR1cmFsUHJldmlld1Nob3dNb2RlLFxuICAgIHR5cGUgUHJvY2VkdXJhbFRyYW5zZm9ybUdyYW51bGFyaXR5LFxufSBmcm9tIFwiQHNlbWlvLXRlY2gvcHJvY2VkdXJhbC0zZC1yZWFjdFwiO1xuaW1wb3J0IHR5cGUgeyBDb250ZXh0TWVudUl0ZW0gfSBmcm9tIFwiQHNlbWlvLXRlY2gvdWktcmVhY3RcIjtcbmltcG9ydCB7IGJvb3RzdHJhcEVsZW1lbnRzU3VyZmFjZUNocm9tZURvY3VtZW50LCBzZWxlY3Rpb25NZXJnZUlkcywgdHlwZSBTZWxlY3Rpb25NZXJnZU1vZGUgfSBmcm9tIFwiQHNlbWlvLXRlY2gvdWktcmVhY3RcIjtcblxuZnVuY3Rpb24gcHJldmlld0l0ZW1LZXkoaXRlbTogUHJvY2VkdXJhbFByZXZpZXdJdGVtKTogc3RyaW5nIHtcblx0cmV0dXJuIGAke2l0ZW0ud2lkZ2V0SWR9OiR7aXRlbS5wb3J0fToke2l0ZW0uZGlyZWN0aW9ufWA7XG59XG5cbmZ1bmN0aW9uIHByZXZpZXdJdGVtc1dpdGhNZXNoZXMoXG5cdGl0ZW1zOiBQcm9jZWR1cmFsUHJldmlld0l0ZW1bXSxcblx0cHJldmlld01lc2hlcz86IFJlYWRvbmx5PFJlY29yZDxzdHJpbmcsIHVua25vd24+Pixcblx0cHJldmlvdXM6IHJlYWRvbmx5IFByb2NlZHVyYWxQcmV2aWV3SXRlbVtdID0gW10sXG4pOiBQcm9jZWR1cmFsUHJldmlld0l0ZW1bXSB7XG5cdGNvbnN0IHByZXZpb3VzQnlLZXkgPSBuZXcgTWFwKHByZXZpb3VzLm1hcCgoaXRlbSkgPT4gW3ByZXZpZXdJdGVtS2V5KGl0ZW0pLCBpdGVtXSkpO1xuXHRyZXR1cm4gaXRlbXMubWFwKChpdGVtKSA9PiB7XG5cdFx0aWYgKGl0ZW0ua2luZCAhPT0gXCJnZW9tZXRyeVwiIHx8IGl0ZW0uZGlyZWN0aW9uICE9PSBcIm91dFwiKSByZXR1cm4gaXRlbTtcblx0XHRjb25zdCBtZXNoS2V5ID0gYCR7aXRlbS53aWRnZXRJZH06JHtpdGVtLnBvcnR9YDtcblx0XHRjb25zdCBwcmV2aW91c0l0ZW0gPSBwcmV2aW91c0J5S2V5LmdldChwcmV2aWV3SXRlbUtleShpdGVtKSk7XG5cdFx0Y29uc3QgbWVzaCA9XG5cdFx0XHRtZXNoVHJhbnNmZXJGcm9tUHJldmlld1BheWxvYWQocHJldmlld01lc2hlcz8uW21lc2hLZXldKSA/P1xuXHRcdFx0KHByZXZpb3VzSXRlbT8uaGFuZGxlID09PSBpdGVtLmhhbmRsZSA/IHByZXZpb3VzSXRlbS5tZXNoIDogdW5kZWZpbmVkKTtcblx0XHRyZXR1cm4gbWVzaCA/IHsgLi4uaXRlbSwgbWVzaCB9IDogaXRlbTtcblx0fSk7XG59XG5cbmV4cG9ydCBjb25zdCBQUk9DRURVUkFMXzNEX1BMQVlfQVBQX0lEID0gXCJwcm9jZWR1cmFsLTNkLXBsYXlcIjtcbmV4cG9ydCBjb25zdCBQUk9DRURVUkFMXzNEX1BMQVlfQ09OVFJPTExFUl9JRCA9IFwicHJvY2VkdXJhbC0zZC1wbGF5XCI7XG5leHBvcnQgY29uc3QgUFJPQ0VEVVJBTF9QTEFZX1NVUkZBQ0VfSUQgPSBcInByb2NlZHVyYWwucGxheS92MVwiO1xuZXhwb3J0IGNvbnN0IFBST0NFRFVSQUxfUExBWV9CT0RZX0tFWV9NQUlOID0gXCJwcm9jZWR1cmFsLnBsYXkubWFpblwiO1xuZXhwb3J0IGNvbnN0IFBST0NFRFVSQUxfUExBWV9XSU5ET1dfS0lORF9JRCA9IFwicHJvY2VkdXJhbC1tYWluXCI7XG5leHBvcnQgY29uc3QgUFJPQ0VEVVJBTF9QTEFZX1dJTkRPV19LSU5EX1BSRVZJRVcgPSBcInByb2NlZHVyYWwtcHJldmlld1wiO1xuZXhwb3J0IGNvbnN0IFBST0NFRFVSQUxfUExBWV9CT0RZX0tFWV9QUkVWSUVXID0gXCJwcm9jZWR1cmFsLnBsYXkucHJldmlld1wiO1xuZXhwb3J0IGNvbnN0IFBST0NFRFVSQUxfUExBWV9TVVJGQUNFX0lEX1BSRVZJRVcgPSBcInByb2NlZHVyYWwucGxheS5wcmV2aWV3L3YxXCI7XG5cbmV4cG9ydCBjb25zdCBQUk9DRURVUkFMX1BMQVlfREVGQVVMVF9GSVhUVVJFOiBGbG93Rml4dHVyZVYxID0gUFJPQ0VEVVJBTF9ERUZBVUxUX0ZJWFRVUkU7XG5leHBvcnQgY29uc3QgUFJPQ0VEVVJBTF9QTEFZX0RFRkFVTFRfRklYVFVSRV9KU09OID0gcHJvY2VkdXJhbEZpeHR1cmVUb0pzb24oUFJPQ0VEVVJBTF9ERUZBVUxUX0ZJWFRVUkUpO1xuZXhwb3J0IGNvbnN0IFBST0NFRFVSQUxfUExBWV9MQVlPVVQgPSBjcmVhdGVEZWZhdWx0TGF5b3V0KFxuXHRbUFJPQ0VEVVJBTF9QTEFZX1dJTkRPV19LSU5EX0lELCBQUk9DRURVUkFMX1BMQVlfV0lORE9XX0tJTkRfUFJFVklFV10sXG5cdFwicm93XCIsXG5cdFs1NSwgNDVdLFxuXHRbXCJGbG93XCIsIFwiUHJldmlld1wiXSxcbik7XG5leHBvcnQgY29uc3QgUFJPQ0VEVVJBTF9QTEFZX0tJTkRTX1RBQl9JRCA9IFwicHJvY2VkdXJhbC1wbGF5LWtpbmRzXCI7XG5leHBvcnQgY29uc3QgUFJPQ0VEVVJBTF9QTEFZX0VYVEVOU0lPTlNfVEFCX0lEID0gXCJwcm9jZWR1cmFsLXBsYXktZXh0ZW5zaW9uc1wiO1xuZXhwb3J0IGNvbnN0IFBST0NFRFVSQUxfUExBWV9ISUVSQVJDSFlfVEFCX0lEID0gXCJmcmFtZXdvcmsucGFuZWwuaGllcmFyY2h5XCI7XG5leHBvcnQgY29uc3QgUFJPQ0VEVVJBTF9QTEFZX0NBVEFMT0dVRV9UQUJfSUQgPSBcImZyYW1ld29yay5wYW5lbC5jYXRhbG9ndWVcIjtcbmV4cG9ydCBjb25zdCBQUk9DRURVUkFMX1BMQVlfSU5TUEVDVElPTl9UQUJfSUQgPSBcImZyYW1ld29yay5wYW5lbC5pbnNwZWN0aW9uXCI7XG5leHBvcnQgY29uc3QgUFJPQ0VEVVJBTF9QTEFZX0ZJWFRVUkVfREVGQVVMVF9JRCA9IFwicHJvY2VkdXJhbC1kZWZhdWx0XCI7XG5cbmltcG9ydCB7XG4gICAgUFJPQ0VEVVJBTF9QTEFZX0ZJWFRVUkVfSEVYQUdPTkFMX01VU0hST09NX0NPTFVNTl9JRCxcbiAgICByZXNvbHZlUHJvY2VkdXJhbFBsYXlGaXh0dXJlU2x1Zyxcbn0gZnJvbSBcIi4vZml4dHVyZS1zbHVncy5qc1wiO1xuXG5leHBvcnQgeyBQUk9DRURVUkFMX1BMQVlfRklYVFVSRV9IRVhBR09OQUxfTVVTSFJPT01fQ09MVU1OX0lELCByZXNvbHZlUHJvY2VkdXJhbFBsYXlGaXh0dXJlU2x1ZyB9O1xuXG5jb25zdCBwcm9jZWR1cmFsRml4dHVyZU1vZHVsZXMgPSBpbXBvcnQubWV0YS5nbG9iKFwiLi4vZml4dHVyZS8qLnByb2NlZHVyYWwuanNvblwiLCB7IGVhZ2VyOiB0cnVlIH0pIGFzIFJlY29yZDxcblx0c3RyaW5nLFxuXHR7IGRlZmF1bHQ6IHVua25vd24gfVxuPjtcblxuZnVuY3Rpb24gcHJvY2VkdXJhbEZpeHR1cmVJZEZyb21HbG9iUGF0aChnbG9iUGF0aDogc3RyaW5nKTogc3RyaW5nIHtcblx0Y29uc3QgYmFzZSA9IGdsb2JQYXRoLnNwbGl0KFwiL1wiKS5wb3AoKSA/PyBnbG9iUGF0aDtcblx0cmV0dXJuIGJhc2UucmVwbGFjZSgvXFwucHJvY2VkdXJhbFxcLmpzb24kLywgXCJcIik7XG59XG5cbmZ1bmN0aW9uIHByb2NlZHVyYWxGaXh0dXJlTGFiZWxGcm9tSWQoaWQ6IHN0cmluZyk6IHN0cmluZyB7XG5cdHJldHVybiBpZFxuXHRcdC5zcGxpdChcIi1cIilcblx0XHQuZmlsdGVyKEJvb2xlYW4pXG5cdFx0Lm1hcCgod29yZCkgPT4gd29yZC5jaGFyQXQoMCkudG9VcHBlckNhc2UoKSArIHdvcmQuc2xpY2UoMSkpXG5cdFx0LmpvaW4oXCIgXCIpO1xufVxuXG5jb25zdCBQUk9DRURVUkFMX1BMQVlfRklMRV9GSVhUVVJFX0pTT05fQllfSUQ6IFJlY29yZDxzdHJpbmcsIHN0cmluZz4gPSBPYmplY3QuZnJvbUVudHJpZXMoXG5cdE9iamVjdC5lbnRyaWVzKHByb2NlZHVyYWxGaXh0dXJlTW9kdWxlcykubWFwKChbcGF0aCwgbW9kXSkgPT4ge1xuXHRcdGNvbnN0IGlkID0gcHJvY2VkdXJhbEZpeHR1cmVJZEZyb21HbG9iUGF0aChwYXRoKTtcblx0XHRjb25zdCBqc29uID0gdHlwZW9mIG1vZC5kZWZhdWx0ID09PSBcInN0cmluZ1wiID8gbW9kLmRlZmF1bHQgOiBKU09OLnN0cmluZ2lmeShtb2QuZGVmYXVsdCk7XG5cdFx0cmV0dXJuIFtpZCwganNvbl07XG5cdH0pLFxuKTtcblxuZXhwb3J0IGNvbnN0IFBST0NFRFVSQUxfUExBWV9FTVBUWV9GSVhUVVJFOiBGbG93Rml4dHVyZVYxID0ge1xuXHRzY2hlbWE6IFwiZmxvdy5maXh0dXJlL3YxXCIsXG5cdGNhbWVyYTogeyB4OiAwLCB5OiAwLCB6b29tOiAxIH0sXG5cdHdpZGdldHM6IFtdLFxuXHRzeW5hcHNlczogW10sXG59O1xuXG5leHBvcnQgY29uc3QgUFJPQ0VEVVJBTF9QTEFZX0VNUFRZX0ZJWFRVUkVfSlNPTiA9IHByb2NlZHVyYWxGaXh0dXJlVG9Kc29uKFBST0NFRFVSQUxfUExBWV9FTVBUWV9GSVhUVVJFKTtcblxuZXhwb3J0IGNvbnN0IFBST0NFRFVSQUxfUExBWV9GSVhUVVJFX09QVElPTlM6IFJlYWRvbmx5QXJyYXk8eyByZWFkb25seSBpZDogc3RyaW5nOyByZWFkb25seSBsYWJlbDogc3RyaW5nIH0+ID0gW1xuXHR7IGlkOiBQUk9DRURVUkFMX1BMQVlfRklYVFVSRV9ERUZBVUxUX0lELCBsYWJlbDogXCJCb3ggZmlsbGV0IG1vdmVcIiB9LFxuXHQuLi5PYmplY3Qua2V5cyhQUk9DRURVUkFMX1BMQVlfRklMRV9GSVhUVVJFX0pTT05fQllfSUQpXG5cdFx0LnNvcnQoKVxuXHRcdC5tYXAoKGlkKSA9PiAoeyBpZCwgbGFiZWw6IHByb2NlZHVyYWxGaXh0dXJlTGFiZWxGcm9tSWQoaWQpIH0pKSxcbl07XG5cbmNvbnN0IFBST0NFRFVSQUxfUExBWV9TVE9SRV9LRVkgPSBcInByb2NlZHVyYWwuZml4dHVyZS92MVwiO1xuXG4vKiogQGVtb2ppIPCfkr4gTG9jYWwgcGVyc2lzdGVuY2UgZm9yIHByb2NlZHVyYWwgZmxvdyBmaXh0dXJlcy4gKi9cbmV4cG9ydCBpbnRlcmZhY2UgUHJvY2VkdXJhbFBsYXlGaXh0dXJlU3RvcmUge1xuXHRsb2FkKCk6IHN0cmluZyB8IG51bGw7XG5cdHNhdmUoZml4dHVyZUpzb246IHN0cmluZyk6IHZvaWQ7XG5cdGNsZWFyKCk6IHZvaWQ7XG59XG5cbmV4cG9ydCBmdW5jdGlvbiBjcmVhdGVQcm9jZWR1cmFsUGxheUZpeHR1cmVTdG9yZShzdG9yYWdlPzogUGljazxTdG9yYWdlLCBcImdldEl0ZW1cIiB8IFwic2V0SXRlbVwiIHwgXCJyZW1vdmVJdGVtXCI+KTogUHJvY2VkdXJhbFBsYXlGaXh0dXJlU3RvcmUge1xuXHRjb25zdCByZXNvbHZlZCA9XG5cdFx0c3RvcmFnZSA/P1xuXHRcdCh0eXBlb2YgZ2xvYmFsVGhpcy5sb2NhbFN0b3JhZ2UgIT09IFwidW5kZWZpbmVkXCJcblx0XHRcdD8gZ2xvYmFsVGhpcy5sb2NhbFN0b3JhZ2Vcblx0XHRcdDogKCgpID0+IHtcblx0XHRcdFx0XHRjb25zdCBiYWNraW5nID0gbmV3IE1hcDxzdHJpbmcsIHN0cmluZz4oKTtcblx0XHRcdFx0XHRyZXR1cm4ge1xuXHRcdFx0XHRcdFx0Z2V0SXRlbTogKGtleTogc3RyaW5nKSA9PiBiYWNraW5nLmdldChrZXkpID8/IG51bGwsXG5cdFx0XHRcdFx0XHRzZXRJdGVtOiAoa2V5OiBzdHJpbmcsIHZhbHVlOiBzdHJpbmcpID0+IHtcblx0XHRcdFx0XHRcdFx0YmFja2luZy5zZXQoa2V5LCB2YWx1ZSk7XG5cdFx0XHRcdFx0XHR9LFxuXHRcdFx0XHRcdFx0cmVtb3ZlSXRlbTogKGtleTogc3RyaW5nKSA9PiB7XG5cdFx0XHRcdFx0XHRcdGJhY2tpbmcuZGVsZXRlKGtleSk7XG5cdFx0XHRcdFx0XHR9LFxuXHRcdFx0XHRcdH07XG5cdFx0XHRcdH0pKCkpO1xuXHRyZXR1cm4ge1xuXHRcdGxvYWQoKTogc3RyaW5nIHwgbnVsbCB7XG5cdFx0XHRyZXR1cm4gcmVzb2x2ZWQuZ2V0SXRlbShQUk9DRURVUkFMX1BMQVlfU1RPUkVfS0VZKTtcblx0XHR9LFxuXHRcdHNhdmUoZml4dHVyZUpzb246IHN0cmluZyk6IHZvaWQge1xuXHRcdFx0cmVzb2x2ZWQuc2V0SXRlbShQUk9DRURVUkFMX1BMQVlfU1RPUkVfS0VZLCBmaXh0dXJlSnNvbik7XG5cdFx0fSxcblx0XHRjbGVhcigpOiB2b2lkIHtcblx0XHRcdHJlc29sdmVkLnJlbW92ZUl0ZW0oUFJPQ0VEVVJBTF9QTEFZX1NUT1JFX0tFWSk7XG5cdFx0fSxcblx0fTtcbn1cblxuZXhwb3J0IHR5cGUgUHJvY2VkdXJhbExheW91dE9yaWVudGF0aW9uID0gXCJsZWZ0UmlnaHRcIiB8IFwidG9wQm90dG9tXCI7XG5leHBvcnQgdHlwZSBQcm9jZWR1cmFsUGxheVNlbGVjdGlvbk1vZGUgPSBTZWxlY3Rpb25NZXJnZU1vZGU7XG5leHBvcnQgdHlwZSBQcm9jZWR1cmFsUGxheVNlbGVjdGlvbk1ldGhvZCA9IFwicmVjdGFuZ2xlXCIgfCBcImxhc3NvXCI7XG5cbmNvbnN0IERFRkFVTFRfTEFZRVJfU1BBQ0lORyA9IDEyMDtcbmNvbnN0IERFRkFVTFRfU0lCTElOR19HQVAgPSA0MDtcblxuZXhwb3J0IHR5cGUge1xuICAgIFByb2NlZHVyYWxHdW1iYWxsVHJhbnNmb3JtRGVsdGEsXG4gICAgUHJvY2VkdXJhbEd1bWJhbGxUcmFuc2Zvcm1PcCxcbiAgICBQcm9jZWR1cmFsR3VtYmFsbFRyYW5zZm9ybVBoYXNlLFxuICAgIFByb2NlZHVyYWxHdW1iYWxsVHJhbnNmb3JtUmVxdWVzdCxcbiAgICBQcm9jZWR1cmFsVHJhbnNmb3JtR3JhbnVsYXJpdHlcbn0gZnJvbSBcIkBzZW1pby10ZWNoL3Byb2NlZHVyYWwtM2QtcmVhY3RcIjtcblxuaW50ZXJmYWNlIEd1bWJhbGxUcmFuc2Zvcm1CaW5kaW5nIHtcblx0cmVhZG9ubHkgc291cmNlV2lkZ2V0SWQ6IHN0cmluZztcblx0cmVhZG9ubHkgdHJhbnNmb3JtSWQ6IHN0cmluZztcblx0cmVhZG9ubHkgb3A6IFByb2NlZHVyYWxHdW1iYWxsVHJhbnNmb3JtT3A7XG5cdHJlYWRvbmx5IGdyYW51bGFyaXR5OiBQcm9jZWR1cmFsVHJhbnNmb3JtR3JhbnVsYXJpdHk7XG5cdHJlYWRvbmx5IHZhbHVlV2lkZ2V0SWRzOiBzdHJpbmdbXTtcblx0cmVhZG9ubHkgdmVjdG9ySWQ/OiBzdHJpbmc7XG5cdHJlYWRvbmx5IHZhbHVlczogeyBvZmZzZXQ6IFtudW1iZXIsIG51bWJlciwgbnVtYmVyXTsgYW5nbGU6IG51bWJlcjsgZmFjdG9yOiBudW1iZXIgfTtcbn1cblxuaW50ZXJmYWNlIEd1bWJhbGxEcmFnU2Vzc2lvbiB7XG5cdHJlYWRvbmx5IGJpbmRpbmc6IEd1bWJhbGxUcmFuc2Zvcm1CaW5kaW5nO1xuXHRyZWFkb25seSBiYXNlVmFsdWVzOiB7IG9mZnNldDogW251bWJlciwgbnVtYmVyLCBudW1iZXJdOyBhbmdsZTogbnVtYmVyOyBmYWN0b3I6IG51bWJlciB9O1xufVxuXG5jb25zdCBCUkVQX1hGT1JNX05FVVJPTl9LSU5EOiBSZWNvcmQ8UHJvY2VkdXJhbEd1bWJhbGxUcmFuc2Zvcm1PcCwgc3RyaW5nPiA9IHtcblx0dHJhbnNsYXRlOiBcImJyZXAueGZvcm0udHJhbnNsYXRlXCIsXG5cdHJvdGF0ZTogXCJicmVwLnhmb3JtLnJvdGF0ZVwiLFxuXHRzY2FsZTogXCJicmVwLnhmb3JtLnNjYWxlXCIsXG59O1xuXG5jb25zdCBHVU1CQUxMX1NMSURFUl9IQUxGX1dJRFRIID0gNDI7XG5jb25zdCBHVU1CQUxMX05FVVJPTl9IQUxGX1dJRFRIID0gNDg7XG5jb25zdCBHVU1CQUxMX1ZFQ1RPUl9IQUxGX1dJRFRIID0gNTI7XG5jb25zdCBHVU1CQUxMX1NPVVJDRV9IQUxGX1dJRFRIID0gNDg7XG5cbmZ1bmN0aW9uIGd1bWJhbGxDb2x1bW5FZGdlR2FwKGxheWVyU3BhY2luZzogbnVtYmVyLCBzaWJsaW5nR2FwOiBudW1iZXIpOiBudW1iZXIge1xuXHRyZXR1cm4gTWF0aC5tYXgoc2libGluZ0dhcCwgbGF5ZXJTcGFjaW5nICogMC4yLCAyOCk7XG59XG5cbmZ1bmN0aW9uIGd1bWJhbGxDb2x1bW5BZnRlcihwcmV2Q2VudGVyWDogbnVtYmVyLCBwcmV2SGFsZldpZHRoOiBudW1iZXIsIG5leHRIYWxmV2lkdGg6IG51bWJlciwgZWRnZUdhcDogbnVtYmVyKTogbnVtYmVyIHtcblx0cmV0dXJuIHByZXZDZW50ZXJYICsgcHJldkhhbGZXaWR0aCArIGVkZ2VHYXAgKyBuZXh0SGFsZldpZHRoO1xufVxuXG5mdW5jdGlvbiBndW1iYWxsVmFsdWVSb3dHYXAoc2libGluZ0dhcDogbnVtYmVyKTogbnVtYmVyIHtcblx0cmV0dXJuIE1hdGgubWF4KHNpYmxpbmdHYXAsIDMyKTtcbn1cblxuZnVuY3Rpb24gZ3VtYmFsbE1ha2VTcGFjZUR4KHRyYW5zZm9ybUNvbHVtblg6IG51bWJlciwgdHJhbnNmb3JtSGFsZldpZHRoOiBudW1iZXIsIHNvdXJjZVg6IG51bWJlciwgZWRnZUdhcDogbnVtYmVyKTogbnVtYmVyIHtcblx0cmV0dXJuIHRyYW5zZm9ybUNvbHVtblggKyB0cmFuc2Zvcm1IYWxmV2lkdGggKyBlZGdlR2FwIC0gc291cmNlWDtcbn1cblxuZnVuY3Rpb24gd2lkZ2V0TGF5b3V0RnJvbUZpeHR1cmUoZml4dHVyZUpzb246IHN0cmluZywgd2lkZ2V0SWQ6IHN0cmluZyk6IHsgeDogbnVtYmVyOyB5OiBudW1iZXIgfSB7XG5cdHRyeSB7XG5cdFx0Y29uc3QgZml4dHVyZSA9IEpTT04ucGFyc2UoZml4dHVyZUpzb24pIGFzIEZsb3dGaXh0dXJlVjE7XG5cdFx0cmV0dXJuIGZpeHR1cmUubGF5b3V0Py5bd2lkZ2V0SWRdID8/IHsgeDogMCwgeTogMCB9O1xuXHR9IGNhdGNoIHtcblx0XHRyZXR1cm4geyB4OiAwLCB5OiAwIH07XG5cdH1cbn1cblxuZnVuY3Rpb24gZ3VtYmFsbFplcm9EZWx0YShvcDogUHJvY2VkdXJhbEd1bWJhbGxUcmFuc2Zvcm1PcCk6IFByb2NlZHVyYWxHdW1iYWxsVHJhbnNmb3JtRGVsdGEge1xuXHRpZiAob3AgPT09IFwidHJhbnNsYXRlXCIpIHJldHVybiB7IG9wOiBcInRyYW5zbGF0ZVwiLCBvZmZzZXQ6IFswLCAwLCAwXSB9O1xuXHRpZiAob3AgPT09IFwicm90YXRlXCIpIHJldHVybiB7IG9wOiBcInJvdGF0ZVwiLCBhbmdsZTogMCB9O1xuXHRyZXR1cm4geyBvcDogXCJzY2FsZVwiLCBmYWN0b3I6IDEgfTtcbn1cblxuZnVuY3Rpb24gY29weUd1bWJhbGxWYWx1ZXMoYmluZGluZzogR3VtYmFsbFRyYW5zZm9ybUJpbmRpbmcpOiBHdW1iYWxsRHJhZ1Nlc3Npb25bXCJiYXNlVmFsdWVzXCJdIHtcblx0cmV0dXJuIHtcblx0XHRvZmZzZXQ6IFtiaW5kaW5nLnZhbHVlcy5vZmZzZXRbMF0sIGJpbmRpbmcudmFsdWVzLm9mZnNldFsxXSwgYmluZGluZy52YWx1ZXMub2Zmc2V0WzJdXSxcblx0XHRhbmdsZTogYmluZGluZy52YWx1ZXMuYW5nbGUsXG5cdFx0ZmFjdG9yOiBiaW5kaW5nLnZhbHVlcy5mYWN0b3IsXG5cdH07XG59XG5cbmZ1bmN0aW9uIHNldEd1bWJhbGxCaW5kaW5nVmFsdWVzKGJpbmRpbmc6IEd1bWJhbGxUcmFuc2Zvcm1CaW5kaW5nLCB2YWx1ZXM6IEd1bWJhbGxEcmFnU2Vzc2lvbltcImJhc2VWYWx1ZXNcIl0pOiB2b2lkIHtcblx0YmluZGluZy52YWx1ZXMub2Zmc2V0ID0gW3ZhbHVlcy5vZmZzZXRbMF0sIHZhbHVlcy5vZmZzZXRbMV0sIHZhbHVlcy5vZmZzZXRbMl1dO1xuXHRiaW5kaW5nLnZhbHVlcy5hbmdsZSA9IHZhbHVlcy5hbmdsZTtcblx0YmluZGluZy52YWx1ZXMuZmFjdG9yID0gdmFsdWVzLmZhY3Rvcjtcbn1cblxuZnVuY3Rpb24gYXBwbHlHdW1iYWxsRGVsdGFUb0Jhc2UoXG5cdGJhc2U6IEd1bWJhbGxEcmFnU2Vzc2lvbltcImJhc2VWYWx1ZXNcIl0sXG5cdG9wOiBQcm9jZWR1cmFsR3VtYmFsbFRyYW5zZm9ybU9wLFxuXHRkZWx0YTogUHJvY2VkdXJhbEd1bWJhbGxUcmFuc2Zvcm1EZWx0YSxcbik6IEd1bWJhbGxEcmFnU2Vzc2lvbltcImJhc2VWYWx1ZXNcIl0ge1xuXHRpZiAob3AgPT09IFwidHJhbnNsYXRlXCIgJiYgZGVsdGEub3AgPT09IFwidHJhbnNsYXRlXCIpIHtcblx0XHRyZXR1cm4ge1xuXHRcdFx0b2Zmc2V0OiBbYmFzZS5vZmZzZXRbMF0gKyBkZWx0YS5vZmZzZXRbMF0sIGJhc2Uub2Zmc2V0WzFdICsgZGVsdGEub2Zmc2V0WzFdLCBiYXNlLm9mZnNldFsyXSArIGRlbHRhLm9mZnNldFsyXV0sXG5cdFx0XHRhbmdsZTogYmFzZS5hbmdsZSxcblx0XHRcdGZhY3RvcjogYmFzZS5mYWN0b3IsXG5cdFx0fTtcblx0fVxuXHRpZiAob3AgPT09IFwicm90YXRlXCIgJiYgZGVsdGEub3AgPT09IFwicm90YXRlXCIpIHtcblx0XHRyZXR1cm4geyBvZmZzZXQ6IGJhc2Uub2Zmc2V0LCBhbmdsZTogYmFzZS5hbmdsZSArIGRlbHRhLmFuZ2xlLCBmYWN0b3I6IGJhc2UuZmFjdG9yIH07XG5cdH1cblx0aWYgKG9wID09PSBcInNjYWxlXCIgJiYgZGVsdGEub3AgPT09IFwic2NhbGVcIikge1xuXHRcdHJldHVybiB7IG9mZnNldDogYmFzZS5vZmZzZXQsIGFuZ2xlOiBiYXNlLmFuZ2xlLCBmYWN0b3I6IGJhc2UuZmFjdG9yICogZGVsdGEuZmFjdG9yIH07XG5cdH1cblx0cmV0dXJuIGJhc2U7XG59XG5cbmZ1bmN0aW9uIGd1bWJhbGxCaW5kaW5nTm9kZUlkcyhiaW5kaW5nOiBHdW1iYWxsVHJhbnNmb3JtQmluZGluZyk6IHN0cmluZ1tdIHtcblx0cmV0dXJuIFsuLi5iaW5kaW5nLnZhbHVlV2lkZ2V0SWRzLCAuLi4oYmluZGluZy52ZWN0b3JJZCA/IFtiaW5kaW5nLnZlY3RvcklkXSA6IFtdKSwgYmluZGluZy50cmFuc2Zvcm1JZF07XG59XG5cbmZ1bmN0aW9uIGFjY3VtdWxhdGVHdW1iYWxsRGVsdGEoYmluZGluZzogR3VtYmFsbFRyYW5zZm9ybUJpbmRpbmcsIGRlbHRhOiBQcm9jZWR1cmFsR3VtYmFsbFRyYW5zZm9ybURlbHRhKTogdm9pZCB7XG5cdGlmIChkZWx0YS5vcCA9PT0gXCJ0cmFuc2xhdGVcIiAmJiBiaW5kaW5nLm9wID09PSBcInRyYW5zbGF0ZVwiKSB7XG5cdFx0YmluZGluZy52YWx1ZXMub2Zmc2V0ID0gW1xuXHRcdFx0YmluZGluZy52YWx1ZXMub2Zmc2V0WzBdICsgZGVsdGEub2Zmc2V0WzBdLFxuXHRcdFx0YmluZGluZy52YWx1ZXMub2Zmc2V0WzFdICsgZGVsdGEub2Zmc2V0WzFdLFxuXHRcdFx0YmluZGluZy52YWx1ZXMub2Zmc2V0WzJdICsgZGVsdGEub2Zmc2V0WzJdLFxuXHRcdF07XG5cdFx0cmV0dXJuO1xuXHR9XG5cdGlmIChkZWx0YS5vcCA9PT0gXCJyb3RhdGVcIiAmJiBiaW5kaW5nLm9wID09PSBcInJvdGF0ZVwiKSB7XG5cdFx0YmluZGluZy52YWx1ZXMuYW5nbGUgKz0gZGVsdGEuYW5nbGU7XG5cdFx0cmV0dXJuO1xuXHR9XG5cdGlmIChkZWx0YS5vcCA9PT0gXCJzY2FsZVwiICYmIGJpbmRpbmcub3AgPT09IFwic2NhbGVcIikge1xuXHRcdGJpbmRpbmcudmFsdWVzLmZhY3RvciAqPSBkZWx0YS5mYWN0b3I7XG5cdH1cbn1cblxuZnVuY3Rpb24gY29tcGFjdE5ldXJvblBhcmFtcyhiaW5kaW5nOiBHdW1iYWxsVHJhbnNmb3JtQmluZGluZyk6IFJlY29yZDxzdHJpbmcsIHVua25vd24+IHtcblx0aWYgKGJpbmRpbmcub3AgPT09IFwidHJhbnNsYXRlXCIpIHtcblx0XHRjb25zdCBbeCwgeSwgel0gPSBiaW5kaW5nLnZhbHVlcy5vZmZzZXQ7XG5cdFx0cmV0dXJuIHsgb2Zmc2V0OiBbeCwgeSwgel0gfTtcblx0fVxuXHRpZiAoYmluZGluZy5vcCA9PT0gXCJyb3RhdGVcIikge1xuXHRcdHJldHVybiB7IGFuZ2xlOiBiaW5kaW5nLnZhbHVlcy5hbmdsZSB9O1xuXHR9XG5cdHJldHVybiB7IGZhY3RvcjogYmluZGluZy52YWx1ZXMuZmFjdG9yIH07XG59XG5cbmZ1bmN0aW9uIHNsaWRlckRlc2NyaXB0b3IoaWQ6IHN0cmluZywgdmFsdWU6IG51bWJlcik6IHN0cmluZyB7XG5cdGNvbnN0IHsgbWluLCBtYXgsIHN0ZXAgfSA9IGZsb3dTZW5zaWJsZVNsaWRlclJhbmdlKHZhbHVlKTtcblx0cmV0dXJuIEpTT04uc3RyaW5naWZ5KHsga2luZDogXCJpbnB1dFNsaWRlclwiLCBpZCwgdmFsdWUsIG1pbiwgbWF4LCBzdGVwIH0pO1xufVxuXG5mdW5jdGlvbiBuZXVyb25EZXNjcmlwdG9yKGlkOiBzdHJpbmcsIG5ldXJvbktpbmQ6IHN0cmluZyk6IHN0cmluZyB7XG5cdHJldHVybiBKU09OLnN0cmluZ2lmeSh7IGtpbmQ6IFwibmV1cm9uXCIsIGlkLCBuZXVyb25LaW5kIH0pO1xufVxuXG5mdW5jdGlvbiBwcm9jZWR1cmFsUGxheUNtZChjb21tYW5kOiBzdHJpbmcsIGFyZ3M/OiBSZWNvcmQ8c3RyaW5nLCB1bmtub3duPik6IENvbW1hbmREZXNjcmlwdG9yIHtcblx0cmV0dXJuIHsgY29udHJvbGxlcklkOiBQUk9DRURVUkFMXzNEX1BMQVlfQ09OVFJPTExFUl9JRCwgY29tbWFuZCwgYXJncyB9O1xufVxuXG5mdW5jdGlvbiBidWlsZFByb2NlZHVyYWxMYXlvdXRPcHRpb25zSnNvbihsYXllclNwYWNpbmc6IG51bWJlciwgc2libGluZ0dhcDogbnVtYmVyLCBvcmllbnRhdGlvbjogUHJvY2VkdXJhbExheW91dE9yaWVudGF0aW9uKTogc3RyaW5nIHtcblx0cmV0dXJuIEpTT04uc3RyaW5naWZ5KHsgbGF5ZXJTcGFjaW5nLCBzaWJsaW5nR2FwLCBvcmllbnRhdGlvbiB9KTtcbn1cblxuLyoqIEBlbW9qaSDwn5ax77iPIFByb2NlZHVyYWwgcGxheSBjYW52YXMgcmlnaHQtY2xpY2sgbWVudSB3aXRoIHByZXZpZXcgYWN0aW9ucy4gKi9cbmV4cG9ydCBmdW5jdGlvbiBidWlsZFByb2NlZHVyYWxQbGF5Q2FudmFzQ29udGV4dE1lbnUoY3R4OiBGbG93Q2FudmFzQ29udGV4dE1lbnVDb250ZXh0LCBkaXNwYXRjaDogRmxvd0NvbnRleHRNZW51RGlzcGF0Y2gpOiBDb250ZXh0TWVudUl0ZW1bXSB7XG5cdGNvbnN0IGl0ZW1zID0gWy4uLmJ1aWxkRmxvd0NvbnRleHRNZW51SXRlbXMoY3R4LCBkaXNwYXRjaCldO1xuXHRpZiAoY3R4LmhvdmVyZWROb2RlSWQpIHtcblx0XHRpdGVtcy5zcGxpY2UoaXRlbXMubGVuZ3RoIC0gMSwgMCwge1xuXHRcdFx0aWQ6IFwicHJvY2VkdXJhbC5jdHguaXNvbGF0ZVByZXZpZXdcIixcblx0XHRcdGxhYmVsOiBcIklzb2xhdGUgaW4gcHJldmlld1wiLFxuXHRcdFx0aWNvbjogXCJleWVcIixcblx0XHRcdG9uU2VsZWN0OiAoKSA9PiB7XG5cdFx0XHRcdGRpc3BhdGNoKFwic2V0U2VsZWN0aW9uXCIsIHsgaWRzOiBbY3R4LmhvdmVyZWROb2RlSWRdLCBtb2RlOiBcImRlZmF1bHRcIiB9KTtcblx0XHRcdFx0ZGlzcGF0Y2goXCJzZXRTaG93TW9kZVwiLCB7IGlkOiBcInNlbGVjdGVkXCIgfSk7XG5cdFx0XHR9LFxuXHRcdH0pO1xuXHR9XG5cdHJldHVybiBpdGVtcztcbn1cblxuLyoqIEBlbW9qaSDwn6epIFdvcmtiZW5jaCBleHRlbnNpb25zIHRhYjogaW5zdGFsbGVkIG1vZHVsZXMgd2l0aCBlbmFibGUvZGlzYWJsZSB0b2dnbGVzLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIGJ1aWxkUHJvY2VkdXJhbFBsYXlFeHRlbnNpb25zVHJlZShlbnRyaWVzOiByZWFkb25seSBGbG93RXh0ZW5zaW9uRW50cnlbXSk6IFVpTm9kZSB7XG5cdGlmICghZW50cmllcy5sZW5ndGgpIHtcblx0XHRyZXR1cm4ge1xuXHRcdFx0dHlwZTogXCJ0cmVlXCIsXG5cdFx0XHRzZWN0aW9uczogW1xuXHRcdFx0XHR7XG5cdFx0XHRcdFx0aWQ6IFwicHJvY2VkdXJhbC1wbGF5LWV4dGVuc2lvbnMuZW1wdHlcIixcblx0XHRcdFx0XHRsYWJlbDogXCJFeHRlbnNpb25zXCIsXG5cdFx0XHRcdFx0ZGVmYXVsdE9wZW46IGZhbHNlLFxuXHRcdFx0XHRcdGl0ZW1zOiBbeyBpZDogXCJwcm9jZWR1cmFsLXBsYXktZXh0ZW5zaW9ucy5lbXB0eS5tc2dcIiwgbGFiZWw6IFwiTG9hZGluZyBleHRlbnNpb25z4oCmXCIgfV0sXG5cdFx0XHRcdH0sXG5cdFx0XHRdLFxuXHRcdH07XG5cdH1cblx0Y29uc3QgY29tbWFuZEl0ZW1zID0gcHJvY2VkdXJhbEV4dGVuc2lvbkhvc3QuYWN0aXZlQ29tbWFuZHMoKS5tYXAoKGNvbW1hbmQpID0+ICh7XG5cdFx0aWQ6IGBwcm9jZWR1cmFsLXBsYXktZXh0ZW5zaW9ucy5jb21tYW5kLiR7Y29tbWFuZC5pZH1gLFxuXHRcdGxhYmVsOiBjb21tYW5kLnRpdGxlLFxuXHRcdGRlc2NyaXB0aW9uOiBjb21tYW5kLmlkLFxuXHRcdGNvbW1hbmQ6IHByb2NlZHVyYWxQbGF5Q21kKFwicnVuRXh0ZW5zaW9uQ29tbWFuZFwiLCB7IGNvbW1hbmRJZDogY29tbWFuZC5pZCB9KSxcblx0fSkpO1xuXHRjb25zdCBzZWN0aW9uczogVWlUcmVlU2VjdGlvbk5vZGVbXSA9IFtcblx0XHR7XG5cdFx0XHRpZDogXCJwcm9jZWR1cmFsLXBsYXktZXh0ZW5zaW9ucy5pbnN0YWxsZWRcIixcblx0XHRcdGxhYmVsOiBcIkluc3RhbGxlZFwiLFxuXHRcdFx0ZGVmYXVsdE9wZW46IGZhbHNlLFxuXHRcdFx0aXRlbXM6IGVudHJpZXMubWFwKChlbnRyeSkgPT4ge1xuXHRcdFx0XHRjb25zdCBvcGVyYXRvcnMgPSBlbnRyeS5tYW5pZmVzdC5jb250cmlidXRlcy5vcGVyYXRvcnMgPz8gW107XG5cdFx0XHRcdGNvbnN0IHNjaGVtYXMgPSBlbnRyeS5tYW5pZmVzdC5jb250cmlidXRlcy5zY2hlbWFzID8/IFtdO1xuXHRcdFx0XHRjb25zdCBjb21tYW5kcyA9IGVudHJ5Lm1hbmlmZXN0LmNvbnRyaWJ1dGVzLmNvbW1hbmRzID8/IFtdO1xuXHRcdFx0XHRyZXR1cm4ge1xuXHRcdFx0XHRcdGlkOiBgcHJvY2VkdXJhbC1wbGF5LWV4dGVuc2lvbnMuJHtlbnRyeS5pZH1gLFxuXHRcdFx0XHRcdGxhYmVsOiBlbnRyeS5tYW5pZmVzdC5uYW1lLFxuXHRcdFx0XHRcdGRlc2NyaXB0aW9uOiBgJHtlbnRyeS5tYW5pZmVzdC52ZXJzaW9ufSDCtyAke2VudHJ5LmFjdGl2ZSA/IFwiZW5hYmxlZFwiIDogXCJkaXNhYmxlZFwifSDCtyAke29wZXJhdG9ycy5sZW5ndGh9IG9wZXJhdG9ycyDCtyAke3NjaGVtYXMubGVuZ3RofSBzY2hlbWFzIMK3ICR7Y29tbWFuZHMubGVuZ3RofSBjb21tYW5kc2AsXG5cdFx0XHRcdFx0Y29tbWFuZDogcHJvY2VkdXJhbFBsYXlDbWQoXCJ0b2dnbGVFeHRlbnNpb25cIiwgeyBpZDogZW50cnkuaWQsIGVuYWJsZWQ6ICFlbnRyeS5hY3RpdmUgfSksXG5cdFx0XHRcdH07XG5cdFx0XHR9KSxcblx0XHR9LFxuXHRdO1xuXHRpZiAoY29tbWFuZEl0ZW1zLmxlbmd0aCkge1xuXHRcdHNlY3Rpb25zLnB1c2goe1xuXHRcdFx0aWQ6IFwicHJvY2VkdXJhbC1wbGF5LWV4dGVuc2lvbnMuY29tbWFuZHNcIixcblx0XHRcdGxhYmVsOiBcIkNvbW1hbmRzXCIsXG5cdFx0XHRkZWZhdWx0T3BlbjogZmFsc2UsXG5cdFx0XHRpdGVtczogY29tbWFuZEl0ZW1zLFxuXHRcdH0pO1xuXHR9XG5cdHJldHVybiB7IHR5cGU6IFwidHJlZVwiLCBzZWN0aW9ucyB9O1xufVxuXG4vKiogQGVtb2ppIPCfj7fvuI8gV29ya2JlbmNoIGNhdGFsb2d1ZSB0YWI6IG1vZHVsZSBzZWN0aW9ucyBwbHVzIElucHV0cyBhbmQgT3V0cHV0cy4gKi9cbmV4cG9ydCBmdW5jdGlvbiBidWlsZFByb2NlZHVyYWxQbGF5S2luZHNUcmVlKHNlY3Rpb25zOiByZWFkb25seSBDYXRhbG9ndWVTZWN0aW9uW10pOiBVaU5vZGUge1xuXHRpZiAoIXNlY3Rpb25zLmxlbmd0aCkge1xuXHRcdHJldHVybiB7XG5cdFx0XHR0eXBlOiBcInRyZWVcIixcblx0XHRcdHNlY3Rpb25zOiBbXG5cdFx0XHRcdHtcblx0XHRcdFx0XHRpZDogXCJwcm9jZWR1cmFsLXBsYXkta2luZHMuZW1wdHlcIixcblx0XHRcdFx0XHRsYWJlbDogXCJDYXRhbG9ndWVcIixcblx0XHRcdFx0XHRkZWZhdWx0T3BlbjogZmFsc2UsXG5cdFx0XHRcdFx0aXRlbXM6IFt7IGlkOiBcInByb2NlZHVyYWwtcGxheS1raW5kcy5lbXB0eS5tc2dcIiwgbGFiZWw6IFwiTG9hZGluZyBjYXRhbG9ndWXigKZcIiB9XSxcblx0XHRcdFx0fSxcblx0XHRcdF0sXG5cdFx0fTtcblx0fVxuXHRjb25zdCB0cmVlU2VjdGlvbnM6IFVpVHJlZVNlY3Rpb25Ob2RlW10gPSBidWlsZENhdGFsb2d1ZUtpbmRzVHJlZVNlY3Rpb25zKHNlY3Rpb25zLCBcInByb2NlZHVyYWwtcGxheS1raW5kc1wiLCBmbG93UGxheUNhdGFsb2d1ZUl0ZW1EcmFnRGF0YSk7XG5cdHJldHVybiB7IHR5cGU6IFwidHJlZVwiLCBzZWN0aW9uczogdHJlZVNlY3Rpb25zIH07XG59XG5cbmV4cG9ydCBmdW5jdGlvbiBidWlsZFByb2NlZHVyYWxQbGF5SGllcmFyY2h5VHJlZShmaXh0dXJlSnNvbjogc3RyaW5nLCBzZWxlY3RlZE5vZGVJZHM6IHJlYWRvbmx5IHN0cmluZ1tdKTogVWlOb2RlIHtcblx0cmV0dXJuIGJ1aWxkRmxvd1BsYXlIaWVyYXJjaHlUcmVlKGZpeHR1cmVKc29uLCBzZWxlY3RlZE5vZGVJZHMsIFBST0NFRFVSQUxfM0RfUExBWV9DT05UUk9MTEVSX0lEKTtcbn1cblxuZXhwb3J0IGZ1bmN0aW9uIGJ1aWxkUHJvY2VkdXJhbFBsYXlDYXRhbG9ndWVUcmVlKHNlY3Rpb25zOiByZWFkb25seSBDYXRhbG9ndWVTZWN0aW9uW10sIGV4dGVuc2lvbkVudHJpZXM6IHJlYWRvbmx5IEZsb3dFeHRlbnNpb25FbnRyeVtdKTogVWlOb2RlIHtcblx0cmV0dXJuIGJ1aWxkRmxvd1BsYXlDYXRhbG9ndWVUcmVlKHNlY3Rpb25zLCBleHRlbnNpb25FbnRyaWVzKTtcbn1cblxuZXhwb3J0IGZ1bmN0aW9uIGJ1aWxkUHJvY2VkdXJhbFBsYXlJbnNwZWN0b3JUcmVlKGZpeHR1cmVKc29uOiBzdHJpbmcsIHNlbGVjdGVkTm9kZUlkczogcmVhZG9ubHkgc3RyaW5nW10pOiBVaU5vZGUge1xuXHRyZXR1cm4gYnVpbGRGbG93UGxheUluc3BlY3RvclRyZWUoZml4dHVyZUpzb24sIHNlbGVjdGVkTm9kZUlkcywgUFJPQ0VEVVJBTF8zRF9QTEFZX0NPTlRST0xMRVJfSUQpO1xufVxuXG4vKiogQGVtb2ppIPCfp7AgU25hcHNob3QgcmVhZCBieSB7QGxpbmsgYnVpbGRQcm9jZWR1cmFsUGxheVRvb2xiYXJUb29sc30uICovXG5leHBvcnQgaW50ZXJmYWNlIFByb2NlZHVyYWxQbGF5VG9vbGJhclN0YXRlIHtcblx0cmVhZG9ubHkgc2VsZWN0aW9uTWV0aG9kOiBQcm9jZWR1cmFsUGxheVNlbGVjdGlvbk1ldGhvZDtcblx0cmVhZG9ubHkgc2VsZWN0aW9uTW9kZTogUHJvY2VkdXJhbFBsYXlTZWxlY3Rpb25Nb2RlO1xuXHRyZWFkb25seSBzaG93TW9kZTogUHJvY2VkdXJhbFByZXZpZXdTaG93TW9kZTtcblx0cmVhZG9ubHkgc2VsZWN0aW9uQ291bnQ6IG51bWJlcjtcblx0cmVhZG9ubHkgaGFzU3RvcmVkRml4dHVyZTogYm9vbGVhbjtcbn1cblxuLyoqIEBlbW9qaSDwn5SXIEhvc3QgYnJpZGdlIGZvciB0b29sYmFyIGNvbW1hbmRzIHRoYXQgbmVlZCBSZWFjdCAoZmlsZSBwaWNrZXIsIGRvd25sb2FkKS4gKi9cbmV4cG9ydCBpbnRlcmZhY2UgUHJvY2VkdXJhbFBsYXlIb3N0QnJpZGdlIHtcblx0Z2V0VG9vbGJhclN0YXRlKCk6IFByb2NlZHVyYWxQbGF5VG9vbGJhclN0YXRlO1xuXHRydW5Ib3N0Q29tbWFuZChjb21tYW5kOiBzdHJpbmcsIGFyZ3M/OiB1bmtub3duKTogdm9pZDtcbn1cblxuLyoqIEBlbW9qaSDwn6ewIFBsYXlncm91bmQge0BsaW5rIEFwcFRvb2xzfSBmb3IgcHJvY2VkdXJhbCBwbGF5IChzZWxlY3Rpb24sIHNhdmUsIHZpZXcsIGFjdGlvbnMpLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIGJ1aWxkUHJvY2VkdXJhbFBsYXlUb29sYmFyVG9vbHMoc3RhdGU6IFByb2NlZHVyYWxQbGF5VG9vbGJhclN0YXRlLCBjb250cm9sbGVySWQ6IHN0cmluZyk6IEFwcFRvb2xzIHtcblx0Y29uc3Qgc2VsZWN0aW9uVG9vbHM6IFRvb2xJdGVtW10gPSBbXG5cdFx0e1xuXHRcdFx0aWQ6IFwicHJvY2VkdXJhbC5zZWxlY3QucmVjdGFuZ2xlXCIsXG5cdFx0XHRraW5kOiBcInRvZ2dsZVwiLFxuXHRcdFx0aWNvbklkOiBcInNxdWFyZVwiLFxuXHRcdFx0dGV4dDogXCJSZWN0YW5nbGVcIixcblx0XHRcdG9yZGVyOiAwLFxuXHRcdFx0cHJlc3NlZDogc3RhdGUuc2VsZWN0aW9uTWV0aG9kID09PSBcInJlY3RhbmdsZVwiLFxuXHRcdFx0Y29udHJvbGxlcklkLFxuXHRcdFx0Y29tbWFuZDogXCJzZXRTZWxlY3Rpb25NZXRob2RcIixcblx0XHRcdGFyZ3M6IHsgbWV0aG9kOiBcInJlY3RhbmdsZVwiIH0sXG5cdFx0fSxcblx0XHR7XG5cdFx0XHRpZDogXCJwcm9jZWR1cmFsLnNlbGVjdC5sYXNzb1wiLFxuXHRcdFx0a2luZDogXCJ0b2dnbGVcIixcblx0XHRcdGljb25JZDogXCJsYXNzb1wiLFxuXHRcdFx0dGV4dDogXCJMYXNzb1wiLFxuXHRcdFx0b3JkZXI6IDEsXG5cdFx0XHRwcmVzc2VkOiBzdGF0ZS5zZWxlY3Rpb25NZXRob2QgPT09IFwibGFzc29cIixcblx0XHRcdGNvbnRyb2xsZXJJZCxcblx0XHRcdGNvbW1hbmQ6IFwic2V0U2VsZWN0aW9uTWV0aG9kXCIsXG5cdFx0XHRhcmdzOiB7IG1ldGhvZDogXCJsYXNzb1wiIH0sXG5cdFx0fSxcblx0XHR7XG5cdFx0XHRpZDogXCJwcm9jZWR1cmFsLnNlbGVjdC5tb2RlLmRlZmF1bHRcIixcblx0XHRcdGtpbmQ6IFwidG9nZ2xlXCIsXG5cdFx0XHRpY29uSWQ6IFwibW91c2UtcG9pbnRlci0yXCIsXG5cdFx0XHR0ZXh0OiBcIkRlZmF1bHRcIixcblx0XHRcdG9yZGVyOiAyLFxuXHRcdFx0cHJlc3NlZDogc3RhdGUuc2VsZWN0aW9uTW9kZSA9PT0gXCJkZWZhdWx0XCIsXG5cdFx0XHRjb250cm9sbGVySWQsXG5cdFx0XHRjb21tYW5kOiBcInNldFNlbGVjdGlvbk1vZGVcIixcblx0XHRcdGFyZ3M6IHsgbW9kZTogXCJkZWZhdWx0XCIgfSxcblx0XHR9LFxuXHRcdHtcblx0XHRcdGlkOiBcInByb2NlZHVyYWwuc2VsZWN0Lm1vZGUuYWRkaXRpdmVcIixcblx0XHRcdGtpbmQ6IFwidG9nZ2xlXCIsXG5cdFx0XHRpY29uSWQ6IFwicGx1c1wiLFxuXHRcdFx0dGV4dDogXCJBZGRcIixcblx0XHRcdG9yZGVyOiAzLFxuXHRcdFx0cHJlc3NlZDogc3RhdGUuc2VsZWN0aW9uTW9kZSA9PT0gXCJhZGRpdGl2ZVwiLFxuXHRcdFx0Y29udHJvbGxlcklkLFxuXHRcdFx0Y29tbWFuZDogXCJzZXRTZWxlY3Rpb25Nb2RlXCIsXG5cdFx0XHRhcmdzOiB7IG1vZGU6IFwiYWRkaXRpdmVcIiB9LFxuXHRcdH0sXG5cdFx0e1xuXHRcdFx0aWQ6IFwicHJvY2VkdXJhbC5zZWxlY3QubW9kZS5zdWJ0cmFjdGl2ZVwiLFxuXHRcdFx0a2luZDogXCJ0b2dnbGVcIixcblx0XHRcdGljb25JZDogXCJtaW51c1wiLFxuXHRcdFx0dGV4dDogXCJTdWJ0cmFjdFwiLFxuXHRcdFx0b3JkZXI6IDQsXG5cdFx0XHRwcmVzc2VkOiBzdGF0ZS5zZWxlY3Rpb25Nb2RlID09PSBcInN1YnRyYWN0aXZlXCIsXG5cdFx0XHRjb250cm9sbGVySWQsXG5cdFx0XHRjb21tYW5kOiBcInNldFNlbGVjdGlvbk1vZGVcIixcblx0XHRcdGFyZ3M6IHsgbW9kZTogXCJzdWJ0cmFjdGl2ZVwiIH0sXG5cdFx0fSxcblx0XHR7XG5cdFx0XHRpZDogXCJwcm9jZWR1cmFsLnNlbGVjdC5tb2RlLmludmVydGl2ZVwiLFxuXHRcdFx0a2luZDogXCJ0b2dnbGVcIixcblx0XHRcdGljb25JZDogXCJhcnJvdy1yaWdodC1sZWZ0XCIsXG5cdFx0XHR0ZXh0OiBcIkludmVydFwiLFxuXHRcdFx0b3JkZXI6IDUsXG5cdFx0XHRwcmVzc2VkOiBzdGF0ZS5zZWxlY3Rpb25Nb2RlID09PSBcImludmVydGl2ZVwiLFxuXHRcdFx0Y29udHJvbGxlcklkLFxuXHRcdFx0Y29tbWFuZDogXCJzZXRTZWxlY3Rpb25Nb2RlXCIsXG5cdFx0XHRhcmdzOiB7IG1vZGU6IFwiaW52ZXJ0aXZlXCIgfSxcblx0XHR9LFxuXHRcdHtcblx0XHRcdGlkOiBcInByb2NlZHVyYWwuc2VsZWN0aW9uLmNsZWFyXCIsXG5cdFx0XHRraW5kOiBcImJ1dHRvblwiLFxuXHRcdFx0aWNvbklkOiBcInhcIixcblx0XHRcdGxhYmVsOiBcIkNsZWFyXCIsXG5cdFx0XHRvcmRlcjogNixcblx0XHRcdGRpc2FibGVkOiBzdGF0ZS5zZWxlY3Rpb25Db3VudCA9PT0gMCxcblx0XHRcdGNvbnRyb2xsZXJJZCxcblx0XHRcdGNvbW1hbmQ6IFwiY2xlYXJTZWxlY3Rpb25cIixcblx0XHR9LFxuXHRdO1xuXHRjb25zdCBzYXZlVG9vbHM6IFRvb2xJdGVtW10gPSBbXG5cdFx0e1xuXHRcdFx0aWQ6IFwicHJvY2VkdXJhbC5zYXZlLnN0b3JlZFwiLFxuXHRcdFx0a2luZDogXCJidXR0b25cIixcblx0XHRcdGljb25JZDogXCJoYXJkLWRyaXZlXCIsXG5cdFx0XHRsYWJlbDogXCJTdG9yZVwiLFxuXHRcdFx0b3JkZXI6IDAsXG5cdFx0XHRjb250cm9sbGVySWQsXG5cdFx0XHRjb21tYW5kOiBcInNhdmVTdG9yZWRcIixcblx0XHR9LFxuXHRcdHtcblx0XHRcdGlkOiBcInByb2NlZHVyYWwuc2F2ZS5kb3dubG9hZFwiLFxuXHRcdFx0a2luZDogXCJidXR0b25cIixcblx0XHRcdGljb25JZDogXCJzYXZlXCIsXG5cdFx0XHRsYWJlbDogXCJEb3dubG9hZFwiLFxuXHRcdFx0b3JkZXI6IDEsXG5cdFx0XHRjb250cm9sbGVySWQsXG5cdFx0XHRjb21tYW5kOiBcInNhdmVEb3dubG9hZFwiLFxuXHRcdH0sXG5cdFx0e1xuXHRcdFx0aWQ6IFwicHJvY2VkdXJhbC5zYXZlLmxvYWRcIixcblx0XHRcdGtpbmQ6IFwiYnV0dG9uXCIsXG5cdFx0XHRpY29uSWQ6IFwiZm9sZGVyLW9wZW5cIixcblx0XHRcdGxhYmVsOiBcIkxvYWRcIixcblx0XHRcdG9yZGVyOiAyLFxuXHRcdFx0Y29udHJvbGxlcklkLFxuXHRcdFx0Y29tbWFuZDogXCJsb2FkUmVxdWVzdFwiLFxuXHRcdH0sXG5cdFx0e1xuXHRcdFx0aWQ6IFwicHJvY2VkdXJhbC5zYXZlLmxvYWRTdG9yZWRcIixcblx0XHRcdGtpbmQ6IFwiYnV0dG9uXCIsXG5cdFx0XHRpY29uSWQ6IFwicm90YXRlLWNjd1wiLFxuXHRcdFx0bGFiZWw6IFwiUmVzdG9yZVwiLFxuXHRcdFx0b3JkZXI6IDMsXG5cdFx0XHRkaXNhYmxlZDogIXN0YXRlLmhhc1N0b3JlZEZpeHR1cmUsXG5cdFx0XHRjb250cm9sbGVySWQsXG5cdFx0XHRjb21tYW5kOiBcImxvYWRTdG9yZWRcIixcblx0XHR9LFxuXHRcdHtcblx0XHRcdGlkOiBcInByb2NlZHVyYWwuc2F2ZS5yZXNldFwiLFxuXHRcdFx0a2luZDogXCJidXR0b25cIixcblx0XHRcdGljb25JZDogXCJyZWZyZXNoLWN3XCIsXG5cdFx0XHRsYWJlbDogXCJSZXNldFwiLFxuXHRcdFx0b3JkZXI6IDQsXG5cdFx0XHRjb250cm9sbGVySWQsXG5cdFx0XHRjb21tYW5kOiBcInJlc2V0Rml4dHVyZVwiLFxuXHRcdH0sXG5cdF07XG5cdHJldHVybiB7XG5cdFx0c2VsZWN0aW9uOiBzZWxlY3Rpb25Ub29scyxcblx0XHRzYXZlOiBzYXZlVG9vbHMsXG5cdFx0dmlldzogW1xuXHRcdFx0e1xuXHRcdFx0XHRpZDogXCJwcm9jZWR1cmFsLnZpZXcuZXZlcnl0aGluZ1wiLFxuXHRcdFx0XHRraW5kOiBcInRvZ2dsZVwiLFxuXHRcdFx0XHRpY29uSWQ6IFwibGF5ZXJzXCIsXG5cdFx0XHRcdHRleHQ6IFwiRXZlcnl0aGluZ1wiLFxuXHRcdFx0XHRvcmRlcjogMCxcblx0XHRcdFx0cHJlc3NlZDogc3RhdGUuc2hvd01vZGUgPT09IFwiZXZlcnl0aGluZ1wiLFxuXHRcdFx0XHRjb250cm9sbGVySWQsXG5cdFx0XHRcdGNvbW1hbmQ6IFwic2V0U2hvd01vZGVcIixcblx0XHRcdFx0YXJnczogeyBpZDogXCJldmVyeXRoaW5nXCIgfSxcblx0XHRcdH0sXG5cdFx0XHR7XG5cdFx0XHRcdGlkOiBcInByb2NlZHVyYWwudmlldy5zZWxlY3RlZFwiLFxuXHRcdFx0XHRraW5kOiBcInRvZ2dsZVwiLFxuXHRcdFx0XHRpY29uSWQ6IFwiZXllXCIsXG5cdFx0XHRcdHRleHQ6IFwiU2VsZWN0ZWRcIixcblx0XHRcdFx0b3JkZXI6IDEsXG5cdFx0XHRcdHByZXNzZWQ6IHN0YXRlLnNob3dNb2RlID09PSBcInNlbGVjdGVkXCIsXG5cdFx0XHRcdGNvbnRyb2xsZXJJZCxcblx0XHRcdFx0Y29tbWFuZDogXCJzZXRTaG93TW9kZVwiLFxuXHRcdFx0XHRhcmdzOiB7IGlkOiBcInNlbGVjdGVkXCIgfSxcblx0XHRcdH0sXG5cdFx0XSxcblx0XHRhY3Rpb25zOiBbXG5cdFx0XHR7XG5cdFx0XHRcdGlkOiBcInByb2NlZHVyYWwuYWN0aW9uLnJlb3JnYW5pemVcIixcblx0XHRcdFx0a2luZDogXCJidXR0b25cIixcblx0XHRcdFx0aWNvbklkOiBcImxheW91dC1ncmlkXCIsXG5cdFx0XHRcdGxhYmVsOiBcIlJlb3JnYW5pemVcIixcblx0XHRcdFx0b3JkZXI6IDAsXG5cdFx0XHRcdGNvbnRyb2xsZXJJZCxcblx0XHRcdFx0Y29tbWFuZDogXCJyZW9yZ2FuaXplXCIsXG5cdFx0XHR9LFxuXHRcdFx0e1xuXHRcdFx0XHRpZDogXCJwcm9jZWR1cmFsLmFjdGlvbi5kZWxldGVcIixcblx0XHRcdFx0a2luZDogXCJidXR0b25cIixcblx0XHRcdFx0aWNvbklkOiBcInRyYXNoLTJcIixcblx0XHRcdFx0bGFiZWw6IFwiRGVsZXRlXCIsXG5cdFx0XHRcdG9yZGVyOiAxLFxuXHRcdFx0XHRkaXNhYmxlZDogc3RhdGUuc2VsZWN0aW9uQ291bnQgPT09IDAsXG5cdFx0XHRcdGNvbnRyb2xsZXJJZCxcblx0XHRcdFx0Y29tbWFuZDogXCJkZWxldGVTZWxlY3Rpb25cIixcblx0XHRcdH0sXG5cdFx0XSxcblx0fTtcbn1cblxuZnVuY3Rpb24gcHJvY2VkdXJhbEZpeHR1cmVKc29uRm9ySWQoZml4dHVyZUlkOiBzdHJpbmcpOiBzdHJpbmcge1xuXHRpZiAoaXNQbGF5Z3JvdW5kTm9GaXh0dXJlSWQoZml4dHVyZUlkKSkge1xuXHRcdHJldHVybiBwcm9jZWR1cmFsRml4dHVyZVRvSnNvbihQUk9DRURVUkFMX1BMQVlfRU1QVFlfRklYVFVSRSk7XG5cdH1cblx0aWYgKGZpeHR1cmVJZCA9PT0gUFJPQ0VEVVJBTF9QTEFZX0ZJWFRVUkVfREVGQVVMVF9JRCkge1xuXHRcdHJldHVybiBQUk9DRURVUkFMX1BMQVlfREVGQVVMVF9GSVhUVVJFX0pTT047XG5cdH1cblx0Y29uc3QgZmlsZUpzb24gPSBQUk9DRURVUkFMX1BMQVlfRklMRV9GSVhUVVJFX0pTT05fQllfSURbZml4dHVyZUlkXTtcblx0aWYgKGZpbGVKc29uKSByZXR1cm4gZmlsZUpzb247XG5cdHJldHVybiBQUk9DRURVUkFMX1BMQVlfRU1QVFlfRklYVFVSRV9KU09OO1xufVxuXG4vKiogQGVtb2ppIPCfp6ogUmVzb2x2ZXMgcHJvY2VkdXJhbCBwbGF5IGZpeHR1cmUgSlNPTiBieSBjYXRhbG9nIGlkLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHByb2NlZHVyYWxQbGF5Rml4dHVyZUpzb24oZml4dHVyZUlkOiBzdHJpbmcgPSBQUk9DRURVUkFMX1BMQVlfRklYVFVSRV9ERUZBVUxUX0lEKTogc3RyaW5nIHtcblx0cmV0dXJuIHByb2NlZHVyYWxGaXh0dXJlSnNvbkZvcklkKGZpeHR1cmVJZCk7XG59XG5cbi8qKiBAZW1vamkg8J+OmyBQcm9jZWR1cmFsIHBsYXkgc2hlbGwgY29udHJvbGxlci4gKi9cbmV4cG9ydCBjbGFzcyBQcm9jZWR1cmFsUGxheUNvbnRyb2xsZXIgZXh0ZW5kcyBDb250cm9sbGVyIGltcGxlbWVudHMgUGxheWdyb3VuZEZpeHR1cmVIb3N0IHtcblx0cmVhZG9ubHkgbWFpbk1vZGUgPSBuZXcgTW9kZVJ1bnRpbWUoXCJtYWluXCIsIFwiUHJvY2VkdXJhbFwiLCB1bmRlZmluZWQpO1xuXHRwcml2YXRlIGFjdGl2ZUZpeHR1cmVJZCA9IHBsYXlncm91bmRSZXNvbHZlZEZpeHR1cmVJZChQTEFZR1JPVU5EX05PX0ZJWFRVUkVfSUQpO1xuXHRwcml2YXRlIGZpeHR1cmVKc29uID0gcHJvY2VkdXJhbEZpeHR1cmVKc29uRm9ySWQocGxheWdyb3VuZFJlc29sdmVkRml4dHVyZUlkKFBMQVlHUk9VTkRfTk9fRklYVFVSRV9JRCkpO1xuXHRwcml2YXRlIHJlYWRvbmx5IGZpeHR1cmVTdG9yZTogUHJvY2VkdXJhbFBsYXlGaXh0dXJlU3RvcmU7XG5cdHByaXZhdGUgaG9zdEJyaWRnZTogUHJvY2VkdXJhbFBsYXlIb3N0QnJpZGdlIHwgbnVsbCA9IG51bGw7XG5cdHByaXZhdGUgcHJldmlld1RleHQgPSBcIuKAlFwiO1xuXHRwcml2YXRlIGNhdGFsb2d1ZVNlY3Rpb25zOiBDYXRhbG9ndWVTZWN0aW9uW10gPSBbXTtcblx0cHJpdmF0ZSBjYXRhbG9ndWVSZXZpc2lvbiA9IDA7XG5cdHByaXZhdGUgcmVhZG9ubHkgc25hcHNob3RMaXN0ZW5lcnMgPSBuZXcgU2V0PCgpID0+IHZvaWQ+KCk7XG5cdHByaXZhdGUgZW5nYWdlbWVudElucHV0ID0gXCJcIjtcblx0cHJpdmF0ZSBsYXllclNwYWNpbmcgPSBERUZBVUxUX0xBWUVSX1NQQUNJTkc7XG5cdHByaXZhdGUgc2libGluZ0dhcCA9IERFRkFVTFRfU0lCTElOR19HQVA7XG5cdHByaXZhdGUgb3JpZW50YXRpb246IFByb2NlZHVyYWxMYXlvdXRPcmllbnRhdGlvbiA9IFwibGVmdFJpZ2h0XCI7XG5cdHByaXZhdGUgcmVvcmdhbml6ZUVwb2NoID0gMDtcblx0cHJpdmF0ZSByZW9yZ2FuaXplT3B0aW9uc0pzb24gPSBidWlsZFByb2NlZHVyYWxMYXlvdXRPcHRpb25zSnNvbihERUZBVUxUX0xBWUVSX1NQQUNJTkcsIERFRkFVTFRfU0lCTElOR19HQVAsIFwibGVmdFJpZ2h0XCIpO1xuXHRwcml2YXRlIGNvbW1hbmRSZXF1ZXN0RXBvY2ggPSAwO1xuXHRwcml2YXRlIGNvbW1hbmRSZXF1ZXN0UGF5bG9hZDogT21pdDxGbG93Q2FudmFzQ29tbWFuZFJlcXVlc3QsIFwiZXBvY2hcIj4gPSB7IGNvbW1hbmQ6IFwiXCIgfTtcblx0cHJpdmF0ZSBleHRlbnNpb25SZXZpc2lvbiA9IDA7XG5cdHByaXZhdGUgcHJldmlld0l0ZW1zOiBQcm9jZWR1cmFsUHJldmlld0l0ZW1bXSA9IFtdO1xuXHRwcml2YXRlIHNlbGVjdGVkTm9kZUlkczogc3RyaW5nW10gPSBbXTtcblx0cHJpdmF0ZSBwcmVzZWxlY3ROb2RlSWRzOiBzdHJpbmdbXSA9IFtdO1xuXHRwcml2YXRlIHByZXNlbGVjdFJlbW92ZWROb2RlSWRzOiBzdHJpbmdbXSA9IFtdO1xuXHRwcml2YXRlIGhvdmVyZWROb2RlSWQ6IHN0cmluZyB8IG51bGwgPSBudWxsO1xuXHRwcml2YXRlIGhvdmVyZWRDaGFubmVsOiBQcm9jZWR1cmFsQ2hhbm5lbFJlZiB8IG51bGwgPSBudWxsO1xuXHRwcml2YXRlIHNlbGVjdGVkQ2hhbm5lbHM6IFByb2NlZHVyYWxDaGFubmVsUmVmW10gPSBbXTtcblx0cHJpdmF0ZSBmaXh0dXJlRWRnZXM6IFByb2NlZHVyYWxGaXh0dXJlRWRnZVtdID0gW107XG5cdHByaXZhdGUgcHJldmlld09mZk5vZGVJZHM6IHN0cmluZ1tdID0gW107XG5cdHByaXZhdGUgc2hvd01vZGU6IFByb2NlZHVyYWxQcmV2aWV3U2hvd01vZGUgPSBcImV2ZXJ5dGhpbmdcIjtcblx0cHJpdmF0ZSBzZWxlY3Rpb25Nb2RlOiBQcm9jZWR1cmFsUGxheVNlbGVjdGlvbk1vZGUgPSBcImRlZmF1bHRcIjtcblx0cHJpdmF0ZSBzZWxlY3Rpb25NZXRob2Q6IFByb2NlZHVyYWxQbGF5U2VsZWN0aW9uTWV0aG9kID0gXCJyZWN0YW5nbGVcIjtcblx0cHJpdmF0ZSBpbnRlcmFjdGlvblJldmlzaW9uID0gMDtcblx0cHJpdmF0ZSB0cmFuc2Zvcm1HcmFudWxhcml0eTogUHJvY2VkdXJhbFRyYW5zZm9ybUdyYW51bGFyaXR5ID0gXCJmdWxsXCI7XG5cdHByaXZhdGUgZ3VtYmFsbEJpbmRpbmdzID0gbmV3IE1hcDxzdHJpbmcsIEd1bWJhbGxUcmFuc2Zvcm1CaW5kaW5nPigpO1xuXHRwcml2YXRlIGd1bWJhbGxCaW5kaW5nQnlUcmFuc2Zvcm1JZCA9IG5ldyBNYXA8c3RyaW5nLCBHdW1iYWxsVHJhbnNmb3JtQmluZGluZz4oKTtcblx0cHJpdmF0ZSBndW1iYWxsRHJhZ1Nlc3Npb246IEd1bWJhbGxEcmFnU2Vzc2lvbiB8IG51bGwgPSBudWxsO1xuXHRwcml2YXRlIGd1bWJhbGxBY3RpdmVXaWRnZXRJZHM6IHN0cmluZ1tdID0gW107XG5cdHByaXZhdGUgbG9kTW9kZTogRGFnTG9kTW9kZUtpbmQgPSBEQUdfTE9EX01PREVfQVVUT01BVElDO1xuXHRwcml2YXRlIGxvZE1vZGVCeUluc3RhbmNlOiBSZWNvcmQ8c3RyaW5nLCBEYWdMb2RNb2RlS2luZD4gPSB7fTtcblx0cHJpdmF0ZSBlZmZlY3RpdmVMb2Q6IERhZ0RyYXdMb2RLaW5kID0gXCJub3JtYWxcIjtcblx0cHJpdmF0ZSBwcm94aW1pdHlEaXN0YW5jZSA9IEZMT1dfREVGQVVMVF9QUk9YSU1JVFlfRElTVEFOQ0U7XG5cblx0Y29uc3RydWN0b3IoY29tbWFuZEJ1czogQ29tbWFuZEJ1cywgaG9zdE5vdGlmeTogKCkgPT4gdm9pZCwgZml4dHVyZVN0b3JlOiBQcm9jZWR1cmFsUGxheUZpeHR1cmVTdG9yZSA9IGNyZWF0ZVByb2NlZHVyYWxQbGF5Rml4dHVyZVN0b3JlKCkpIHtcblx0XHRzdXBlcihQUk9DRURVUkFMXzNEX1BMQVlfQ09OVFJPTExFUl9JRCwgY29tbWFuZEJ1cywgaG9zdE5vdGlmeSk7XG5cdFx0dGhpcy5maXh0dXJlU3RvcmUgPSBmaXh0dXJlU3RvcmU7XG5cdFx0dGhpcy5maXh0dXJlRWRnZXMgPSB0aGlzLnBhcnNlRml4dHVyZUVkZ2VzKHRoaXMuZml4dHVyZUpzb24pO1xuXHRcdHRoaXMucmVidWlsZFNoZWxsTW9kZSgpO1xuXHR9XG5cblx0aGFzU3RvcmVkRml4dHVyZSgpOiBib29sZWFuIHtcblx0XHRyZXR1cm4gdGhpcy5maXh0dXJlU3RvcmUubG9hZCgpICE9IG51bGw7XG5cdH1cblxuXHRnZXRGaXh0dXJlQ2F0YWxvZygpOiBQbGF5Z3JvdW5kRml4dHVyZUNhdGFsb2cgfCBudWxsIHtcblx0XHRpZiAoaXNQbGF5Z3JvdW5kRml4dHVyZUxvY2tlZCgpKSByZXR1cm4gbnVsbDtcblx0XHRyZXR1cm4geyBhY3RpdmVGaXh0dXJlSWQ6IHRoaXMuYWN0aXZlRml4dHVyZUlkLCBvcHRpb25zOiBbLi4uUFJPQ0VEVVJBTF9QTEFZX0ZJWFRVUkVfT1BUSU9OU10gfTtcblx0fVxuXG5cdC8qKiBAZW1vamkg8J+UlyBBdHRhY2hlcyB0aGUgUmVhY3QgaG9zdCBicmlkZ2UgZm9yIHRvb2xiYXIgZmlsZSBJTy4gKi9cblx0c2V0SG9zdEJyaWRnZShicmlkZ2U6IFByb2NlZHVyYWxQbGF5SG9zdEJyaWRnZSB8IG51bGwpOiB2b2lkIHtcblx0XHR0aGlzLmhvc3RCcmlkZ2UgPSBicmlkZ2U7XG5cdFx0dGhpcy5yZWJ1aWxkVG9vbGJhclRvb2xzKCk7XG5cdH1cblxuXHRwcml2YXRlIHRvb2xiYXJTdGF0ZSgpOiBQcm9jZWR1cmFsUGxheVRvb2xiYXJTdGF0ZSB7XG5cdFx0cmV0dXJuIChcblx0XHRcdHRoaXMuaG9zdEJyaWRnZT8uZ2V0VG9vbGJhclN0YXRlKCkgPz8ge1xuXHRcdFx0XHRzZWxlY3Rpb25NZXRob2Q6IHRoaXMuc2VsZWN0aW9uTWV0aG9kLFxuXHRcdFx0XHRzZWxlY3Rpb25Nb2RlOiB0aGlzLnNlbGVjdGlvbk1vZGUsXG5cdFx0XHRcdHNob3dNb2RlOiB0aGlzLnNob3dNb2RlLFxuXHRcdFx0XHRzZWxlY3Rpb25Db3VudDogdGhpcy5zZWxlY3RlZE5vZGVJZHMubGVuZ3RoLFxuXHRcdFx0XHRoYXNTdG9yZWRGaXh0dXJlOiB0aGlzLmhhc1N0b3JlZEZpeHR1cmUoKSxcblx0XHRcdH1cblx0XHQpO1xuXHR9XG5cblx0LyoqIEBlbW9qaSDwn5SEIFJlYnVpbGRzIHtAbGluayBNb2RlUnVudGltZS50b29sc30gZnJvbSB0aGUgbGF0ZXN0IHRvb2xiYXIgc25hcHNob3QuICovXG5cdHJlYnVpbGRUb29sYmFyVG9vbHMoKTogdm9pZCB7XG5cdFx0aWYgKCF0aGlzLmhvc3RCcmlkZ2UpIHtcblx0XHRcdHRoaXMubWFpbk1vZGUudG9vbHMgPSB1bmRlZmluZWQ7XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdHRoaXMubWFpbk1vZGUudG9vbHMgPSBidWlsZFByb2NlZHVyYWxQbGF5VG9vbGJhclRvb2xzKHRoaXMudG9vbGJhclN0YXRlKCksIHRoaXMuaWQpO1xuXHR9XG5cblx0cHJpdmF0ZSByZXNldEludGVyYWN0aW9uU3RhdGUoKTogdm9pZCB7XG5cdFx0dGhpcy5zZWxlY3RlZE5vZGVJZHMgPSBbXTtcblx0XHR0aGlzLnByZXNlbGVjdE5vZGVJZHMgPSBbXTtcblx0XHR0aGlzLnByZXNlbGVjdFJlbW92ZWROb2RlSWRzID0gW107XG5cdFx0dGhpcy5ob3ZlcmVkTm9kZUlkID0gbnVsbDtcblx0XHR0aGlzLmhvdmVyZWRDaGFubmVsID0gbnVsbDtcblx0XHR0aGlzLnNlbGVjdGVkQ2hhbm5lbHMgPSBbXTtcblx0XHR0aGlzLnByZXZpZXdPZmZOb2RlSWRzID0gW107XG5cdFx0dGhpcy5wcmV2aWV3SXRlbXMgPSBbXTtcblx0XHR0aGlzLmd1bWJhbGxCaW5kaW5ncy5jbGVhcigpO1xuXHRcdHRoaXMuZ3VtYmFsbEJpbmRpbmdCeVRyYW5zZm9ybUlkLmNsZWFyKCk7XG5cdFx0dGhpcy5jbGVhckd1bWJhbGxEcmFnKCk7XG5cdH1cblxuXHRwcml2YXRlIHBhcnNlRml4dHVyZUVkZ2VzKGpzb246IHN0cmluZyk6IFByb2NlZHVyYWxGaXh0dXJlRWRnZVtdIHtcblx0XHR0cnkge1xuXHRcdFx0Y29uc3QgcGFyc2VkID0gSlNPTi5wYXJzZShqc29uKSBhcyB7XG5cdFx0XHRcdHN5bmFwc2VzPzogQXJyYXk8e1xuXHRcdFx0XHRcdGZyb20/OiBzdHJpbmc7XG5cdFx0XHRcdFx0dG8/OiBzdHJpbmc7XG5cdFx0XHRcdFx0ZnJvbV9wb3J0Pzogc3RyaW5nO1xuXHRcdFx0XHRcdHRvX3BvcnQ/OiBzdHJpbmc7XG5cdFx0XHRcdFx0ZnJvbVBvcnQ/OiBzdHJpbmc7XG5cdFx0XHRcdFx0dG9Qb3J0Pzogc3RyaW5nO1xuXHRcdFx0XHR9Pjtcblx0XHRcdH07XG5cdFx0XHRpZiAoIUFycmF5LmlzQXJyYXkocGFyc2VkLnN5bmFwc2VzKSkgcmV0dXJuIFtdO1xuXHRcdFx0cmV0dXJuIHBhcnNlZC5zeW5hcHNlcy5mbGF0TWFwKChzeW5hcHNlKSA9PiB7XG5cdFx0XHRcdGlmICh0eXBlb2Ygc3luYXBzZS5mcm9tICE9PSBcInN0cmluZ1wiIHx8IHR5cGVvZiBzeW5hcHNlLnRvICE9PSBcInN0cmluZ1wiKSByZXR1cm4gW107XG5cdFx0XHRcdGNvbnN0IGZyb21Qb3J0ID1cblx0XHRcdFx0XHR0eXBlb2Ygc3luYXBzZS5mcm9tX3BvcnQgPT09IFwic3RyaW5nXCJcblx0XHRcdFx0XHRcdD8gc3luYXBzZS5mcm9tX3BvcnRcblx0XHRcdFx0XHRcdDogdHlwZW9mIHN5bmFwc2UuZnJvbVBvcnQgPT09IFwic3RyaW5nXCJcblx0XHRcdFx0XHRcdFx0PyBzeW5hcHNlLmZyb21Qb3J0XG5cdFx0XHRcdFx0XHRcdDogXCJcIjtcblx0XHRcdFx0Y29uc3QgdG9Qb3J0ID1cblx0XHRcdFx0XHR0eXBlb2Ygc3luYXBzZS50b19wb3J0ID09PSBcInN0cmluZ1wiID8gc3luYXBzZS50b19wb3J0IDogdHlwZW9mIHN5bmFwc2UudG9Qb3J0ID09PSBcInN0cmluZ1wiID8gc3luYXBzZS50b1BvcnQgOiBcIlwiO1xuXHRcdFx0XHRyZXR1cm4gW3sgc291cmNlOiBgJHtzeW5hcHNlLmZyb219OiR7ZnJvbVBvcnR9YCwgdGFyZ2V0OiBgJHtzeW5hcHNlLnRvfToke3RvUG9ydH1gIH1dO1xuXHRcdFx0fSk7XG5cdFx0fSBjYXRjaCB7XG5cdFx0XHRyZXR1cm4gW107XG5cdFx0fVxuXHR9XG5cblx0cHJpdmF0ZSBhcHBseUZpeHR1cmVKc29uKGpzb246IHN0cmluZywgcmVzZXRJbnRlcmFjdGlvbiA9IGZhbHNlKTogdm9pZCB7XG5cdFx0aWYgKCFqc29uLmluY2x1ZGVzKFwiZmxvdy5maXh0dXJlL3YxXCIpKSByZXR1cm47XG5cdFx0Y29uc3QgdW5jaGFuZ2VkID0ganNvbiA9PT0gdGhpcy5maXh0dXJlSnNvbjtcblx0XHRpZiAodW5jaGFuZ2VkICYmICFyZXNldEludGVyYWN0aW9uKSByZXR1cm47XG5cdFx0aWYgKCF1bmNoYW5nZWQpIHtcblx0XHRcdHRoaXMuZml4dHVyZUpzb24gPSBqc29uO1xuXHRcdFx0dGhpcy5maXh0dXJlRWRnZXMgPSB0aGlzLnBhcnNlRml4dHVyZUVkZ2VzKGpzb24pO1xuXHRcdH1cblx0XHRpZiAocmVzZXRJbnRlcmFjdGlvbikgdGhpcy5yZXNldEludGVyYWN0aW9uU3RhdGUoKTtcblx0XHR0aGlzLmludGVyYWN0aW9uUmV2aXNpb24gKz0gMTtcblx0XHR0aGlzLm5vdGlmeVNuYXBzaG90KCk7XG5cdFx0dGhpcy5yZWJ1aWxkU2hlbGxNb2RlKCk7XG5cdFx0dGhpcy5lbWl0KCk7XG5cdH1cblxuXHRwcml2YXRlIHJlbmFtZUZsb3dXaWRnZXQob2xkSWQ6IHN0cmluZywgbmV3SWQ6IHN0cmluZyk6IHZvaWQge1xuXHRcdGNvbnN0IHRyaW1tZWQgPSBuZXdJZC50cmltKCk7XG5cdFx0aWYgKCF0cmltbWVkIHx8IHRyaW1tZWQgPT09IG9sZElkKSByZXR1cm47XG5cdFx0Y29uc3QgZml4dHVyZSA9IHBhcnNlRmxvd1BsYXlGaXh0dXJlSnNvbih0aGlzLmZpeHR1cmVKc29uKTtcblx0XHRpZiAoIWZpeHR1cmUgfHwgZml4dHVyZS53aWRnZXRzLnNvbWUoKHdpZGdldCkgPT4gd2lkZ2V0LmlkID09PSB0cmltbWVkKSkgcmV0dXJuO1xuXHRcdGNvbnN0IHdpZGdldHMgPSBmaXh0dXJlLndpZGdldHMubWFwKCh3aWRnZXQpID0+ICh3aWRnZXQuaWQgPT09IG9sZElkID8gKHsgLi4ud2lkZ2V0LCBpZDogdHJpbW1lZCB9IGFzIGltcG9ydChcIkBzZW1pby10ZWNoL2Zsb3ctcmVhY3RcIikuRmxvd1dpZGdldFYxKSA6IHdpZGdldCkpO1xuXHRcdGNvbnN0IHN5bmFwc2VzID0gZml4dHVyZS5zeW5hcHNlcy5tYXAoKHN5bmFwc2UpID0+ICh7XG5cdFx0XHQuLi5zeW5hcHNlLFxuXHRcdFx0ZnJvbTogc3luYXBzZS5mcm9tID09PSBvbGRJZCA/IHRyaW1tZWQgOiBzeW5hcHNlLmZyb20sXG5cdFx0XHR0bzogc3luYXBzZS50byA9PT0gb2xkSWQgPyB0cmltbWVkIDogc3luYXBzZS50byxcblx0XHR9KSk7XG5cdFx0dGhpcy5zZWxlY3RlZE5vZGVJZHMgPSB0aGlzLnNlbGVjdGVkTm9kZUlkcy5tYXAoKGlkKSA9PiAoaWQgPT09IG9sZElkID8gdHJpbW1lZCA6IGlkKSk7XG5cdFx0dGhpcy5hcHBseUZpeHR1cmVKc29uKHByb2NlZHVyYWxGaXh0dXJlVG9Kc29uKHsgLi4uZml4dHVyZSwgd2lkZ2V0cywgc3luYXBzZXMgfSkpO1xuXHR9XG5cblx0cHJpdmF0ZSBwYXRjaEZsb3dXaWRnZXQod2lkZ2V0SWQ6IHN0cmluZywgZmllbGQ6IHN0cmluZywgdmFsdWU6IHVua25vd24pOiB2b2lkIHtcblx0XHRjb25zdCBmaXh0dXJlID0gcGFyc2VGbG93UGxheUZpeHR1cmVKc29uKHRoaXMuZml4dHVyZUpzb24pO1xuXHRcdGlmICghZml4dHVyZSkgcmV0dXJuO1xuXHRcdGNvbnN0IHdpZGdldHMgPSBmaXh0dXJlLndpZGdldHMubWFwKCh3aWRnZXQpID0+IHtcblx0XHRcdGlmICh3aWRnZXQuaWQgIT09IHdpZGdldElkKSByZXR1cm4gd2lkZ2V0O1xuXHRcdFx0aWYgKGZpZWxkID09PSBcInZhbHVlXCIgfHwgZmllbGQgPT09IFwibWluXCIgfHwgZmllbGQgPT09IFwibWF4XCIgfHwgZmllbGQgPT09IFwic3RlcFwiKSB7XG5cdFx0XHRcdGNvbnN0IG51bWVyaWMgPSB0eXBlb2YgdmFsdWUgPT09IFwibnVtYmVyXCIgPyB2YWx1ZSA6IE51bWJlcih2YWx1ZSk7XG5cdFx0XHRcdGlmICghTnVtYmVyLmlzRmluaXRlKG51bWVyaWMpKSByZXR1cm4gd2lkZ2V0O1xuXHRcdFx0XHRyZXR1cm4geyAuLi53aWRnZXQsIFtmaWVsZF06IG51bWVyaWMgfSBhcyBpbXBvcnQoXCJAc2VtaW8tdGVjaC9mbG93LXJlYWN0XCIpLkZsb3dXaWRnZXRWMTtcblx0XHRcdH1cblx0XHRcdGlmICh0eXBlb2YgdmFsdWUgIT09IFwic3RyaW5nXCIpIHJldHVybiB3aWRnZXQ7XG5cdFx0XHRyZXR1cm4geyAuLi53aWRnZXQsIFtmaWVsZF06IHZhbHVlIH0gYXMgaW1wb3J0KFwiQHNlbWlvLXRlY2gvZmxvdy1yZWFjdFwiKS5GbG93V2lkZ2V0VjE7XG5cdFx0fSk7XG5cdFx0dGhpcy5hcHBseUZpeHR1cmVKc29uKHByb2NlZHVyYWxGaXh0dXJlVG9Kc29uKHsgLi4uZml4dHVyZSwgd2lkZ2V0cyB9KSk7XG5cdH1cblxuXHRwcml2YXRlIGxvYWRGaXh0dXJlQnlJZChmaXh0dXJlSWQ6IHN0cmluZyk6IHZvaWQge1xuXHRcdGNvbnN0IG5leHRJZCA9IGlzUGxheWdyb3VuZE5vRml4dHVyZUlkKGZpeHR1cmVJZCkgPyBQTEFZR1JPVU5EX05PX0ZJWFRVUkVfSUQgOiBmaXh0dXJlSWQ7XG5cdFx0Y29uc3QgbmV4dEpzb24gPSBwcm9jZWR1cmFsRml4dHVyZUpzb25Gb3JJZChuZXh0SWQpO1xuXHRcdGlmIChuZXh0SWQgPT09IHRoaXMuYWN0aXZlRml4dHVyZUlkICYmIG5leHRKc29uID09PSB0aGlzLmZpeHR1cmVKc29uKSByZXR1cm47XG5cdFx0dGhpcy5hY3RpdmVGaXh0dXJlSWQgPSBuZXh0SWQ7XG5cdFx0dGhpcy5hcHBseUZpeHR1cmVKc29uKG5leHRKc29uLCB0cnVlKTtcblx0fVxuXG5cdGdldEZpeHR1cmVKc29uKCk6IHN0cmluZyB7XG5cdFx0cmV0dXJuIHRoaXMuZml4dHVyZUpzb247XG5cdH1cblxuXHRnZXRQcmV2aWV3VGV4dCgpOiBzdHJpbmcge1xuXHRcdHJldHVybiB0aGlzLnByZXZpZXdUZXh0O1xuXHR9XG5cblx0Z2V0Q2F0YWxvZ3VlU2VjdGlvbnMoKTogcmVhZG9ubHkgQ2F0YWxvZ3VlU2VjdGlvbltdIHtcblx0XHRyZXR1cm4gdGhpcy5jYXRhbG9ndWVTZWN0aW9ucztcblx0fVxuXG5cdGdldENhdGFsb2d1ZVJldmlzaW9uKCk6IG51bWJlciB7XG5cdFx0cmV0dXJuIHRoaXMuY2F0YWxvZ3VlUmV2aXNpb247XG5cdH1cblxuXHRnZXRFeHRlbnNpb25SZXZpc2lvbigpOiBudW1iZXIge1xuXHRcdHJldHVybiB0aGlzLmV4dGVuc2lvblJldmlzaW9uO1xuXHR9XG5cblx0Z2V0RXh0ZW5zaW9uRW50cmllcygpOiByZWFkb25seSBGbG93RXh0ZW5zaW9uRW50cnlbXSB7XG5cdFx0cmV0dXJuIHByb2NlZHVyYWxFeHRlbnNpb25Ib3N0Lmxpc3RFbnRyaWVzKCk7XG5cdH1cblxuXHRnZXRQcmV2aWV3SXRlbXMoKTogcmVhZG9ubHkgUHJvY2VkdXJhbFByZXZpZXdJdGVtW10ge1xuXHRcdHJldHVybiB0aGlzLnByZXZpZXdJdGVtcztcblx0fVxuXG5cdGdldFNlbGVjdGVkTm9kZUlkcygpOiByZWFkb25seSBzdHJpbmdbXSB7XG5cdFx0cmV0dXJuIHRoaXMuc2VsZWN0ZWROb2RlSWRzO1xuXHR9XG5cblx0Z2V0UHJlc2VsZWN0Tm9kZUlkcygpOiByZWFkb25seSBzdHJpbmdbXSB7XG5cdFx0cmV0dXJuIHRoaXMucHJlc2VsZWN0Tm9kZUlkcztcblx0fVxuXG5cdGdldFByZXNlbGVjdFJlbW92ZWROb2RlSWRzKCk6IHJlYWRvbmx5IHN0cmluZ1tdIHtcblx0XHRyZXR1cm4gdGhpcy5wcmVzZWxlY3RSZW1vdmVkTm9kZUlkcztcblx0fVxuXG5cdGdldFNlbGVjdGlvbk1vZGUoKTogUHJvY2VkdXJhbFBsYXlTZWxlY3Rpb25Nb2RlIHtcblx0XHRyZXR1cm4gdGhpcy5zZWxlY3Rpb25Nb2RlO1xuXHR9XG5cblx0Z2V0U2VsZWN0aW9uTWV0aG9kKCk6IFByb2NlZHVyYWxQbGF5U2VsZWN0aW9uTWV0aG9kIHtcblx0XHRyZXR1cm4gdGhpcy5zZWxlY3Rpb25NZXRob2Q7XG5cdH1cblxuXHRnZXRIb3ZlcmVkTm9kZUlkKCk6IHN0cmluZyB8IG51bGwge1xuXHRcdHJldHVybiB0aGlzLmhvdmVyZWROb2RlSWQ7XG5cdH1cblxuXHRnZXRIb3ZlcmVkQ2hhbm5lbCgpOiBQcm9jZWR1cmFsQ2hhbm5lbFJlZiB8IG51bGwge1xuXHRcdHJldHVybiB0aGlzLmhvdmVyZWRDaGFubmVsO1xuXHR9XG5cblx0Z2V0U2VsZWN0ZWRDaGFubmVscygpOiByZWFkb25seSBQcm9jZWR1cmFsQ2hhbm5lbFJlZltdIHtcblx0XHRyZXR1cm4gdGhpcy5zZWxlY3RlZENoYW5uZWxzO1xuXHR9XG5cblx0Z2V0SG92ZXJlZEdlb21ldHJ5VGFyZ2V0cygpOiByZWFkb25seSBQcm9jZWR1cmFsQ2hhbm5lbFJlZltdIHtcblx0XHRpZiAodGhpcy5ob3ZlcmVkQ2hhbm5lbCkge1xuXHRcdFx0cmV0dXJuIHJlc29sdmVHZW9tZXRyeVRhcmdldHMoW3RoaXMuaG92ZXJlZENoYW5uZWxdLCBudWxsLCB0aGlzLnByZXZpZXdJdGVtcywgdGhpcy5maXh0dXJlRWRnZXMpO1xuXHRcdH1cblx0XHRpZiAodGhpcy5ob3ZlcmVkTm9kZUlkKSB7XG5cdFx0XHRyZXR1cm4gcmVzb2x2ZUdlb21ldHJ5VGFyZ2V0cyhbXSwgdGhpcy5ob3ZlcmVkTm9kZUlkLCB0aGlzLnByZXZpZXdJdGVtcywgdGhpcy5maXh0dXJlRWRnZXMpO1xuXHRcdH1cblx0XHRyZXR1cm4gW107XG5cdH1cblxuXHRnZXRTZWxlY3RlZEdlb21ldHJ5VGFyZ2V0cygpOiByZWFkb25seSBQcm9jZWR1cmFsQ2hhbm5lbFJlZltdIHtcblx0XHRpZiAodGhpcy5zZWxlY3RlZENoYW5uZWxzLmxlbmd0aCA+IDApIHtcblx0XHRcdHJldHVybiByZXNvbHZlR2VvbWV0cnlUYXJnZXRzKHRoaXMuc2VsZWN0ZWRDaGFubmVscywgbnVsbCwgdGhpcy5wcmV2aWV3SXRlbXMsIHRoaXMuZml4dHVyZUVkZ2VzKTtcblx0XHR9XG5cdFx0aWYgKHRoaXMuc2VsZWN0ZWROb2RlSWRzLmxlbmd0aCA+IDApIHtcblx0XHRcdGNvbnN0IHRhcmdldHM6IFByb2NlZHVyYWxDaGFubmVsUmVmW10gPSBbXTtcblx0XHRcdGZvciAoY29uc3Qgd2lkZ2V0SWQgb2YgdGhpcy5zZWxlY3RlZE5vZGVJZHMpIHtcblx0XHRcdFx0dGFyZ2V0cy5wdXNoKC4uLnJlc29sdmVHZW9tZXRyeVRhcmdldHMoW10sIHdpZGdldElkLCB0aGlzLnByZXZpZXdJdGVtcywgdGhpcy5maXh0dXJlRWRnZXMpKTtcblx0XHRcdH1cblx0XHRcdHJldHVybiB0YXJnZXRzO1xuXHRcdH1cblx0XHRyZXR1cm4gW107XG5cdH1cblxuXHRnZXRQcmV2aWV3T2ZmTm9kZUlkcygpOiByZWFkb25seSBzdHJpbmdbXSB7XG5cdFx0cmV0dXJuIHRoaXMucHJldmlld09mZk5vZGVJZHM7XG5cdH1cblxuXHRnZXRTaG93TW9kZSgpOiBQcm9jZWR1cmFsUHJldmlld1Nob3dNb2RlIHtcblx0XHRyZXR1cm4gdGhpcy5zaG93TW9kZTtcblx0fVxuXG5cdGdldEludGVyYWN0aW9uUmV2aXNpb24oKTogbnVtYmVyIHtcblx0XHRyZXR1cm4gdGhpcy5pbnRlcmFjdGlvblJldmlzaW9uO1xuXHR9XG5cblx0Z2V0VHJhbnNmb3JtR3JhbnVsYXJpdHkoKTogUHJvY2VkdXJhbFRyYW5zZm9ybUdyYW51bGFyaXR5IHtcblx0XHRyZXR1cm4gdGhpcy50cmFuc2Zvcm1HcmFudWxhcml0eTtcblx0fVxuXG5cdGdldEd1bWJhbGxBY3RpdmVXaWRnZXRJZHMoKTogcmVhZG9ubHkgc3RyaW5nW10ge1xuXHRcdHJldHVybiB0aGlzLmd1bWJhbGxBY3RpdmVXaWRnZXRJZHM7XG5cdH1cblxuXHRwcml2YXRlIGd1bWJhbGxCaW5kaW5nS2V5KHNvdXJjZVdpZGdldElkOiBzdHJpbmcsIG9wOiBQcm9jZWR1cmFsR3VtYmFsbFRyYW5zZm9ybU9wKTogc3RyaW5nIHtcblx0XHRyZXR1cm4gYCR7c291cmNlV2lkZ2V0SWR9OiR7b3B9YDtcblx0fVxuXG5cdHByaXZhdGUgcmVnaXN0ZXJHdW1iYWxsQmluZGluZyhiaW5kaW5nOiBHdW1iYWxsVHJhbnNmb3JtQmluZGluZyk6IHZvaWQge1xuXHRcdHRoaXMuZ3VtYmFsbEJpbmRpbmdzLnNldCh0aGlzLmd1bWJhbGxCaW5kaW5nS2V5KGJpbmRpbmcuc291cmNlV2lkZ2V0SWQsIGJpbmRpbmcub3ApLCBiaW5kaW5nKTtcblx0XHR0aGlzLmd1bWJhbGxCaW5kaW5nQnlUcmFuc2Zvcm1JZC5zZXQoYmluZGluZy50cmFuc2Zvcm1JZCwgYmluZGluZyk7XG5cdH1cblxuXHRwcml2YXRlIGZpbmRHdW1iYWxsQmluZGluZyh3aWRnZXRJZDogc3RyaW5nLCBvcDogUHJvY2VkdXJhbEd1bWJhbGxUcmFuc2Zvcm1PcCk6IEd1bWJhbGxUcmFuc2Zvcm1CaW5kaW5nIHwgbnVsbCB7XG5cdFx0Y29uc3QgYnlUcmFuc2Zvcm0gPSB0aGlzLmd1bWJhbGxCaW5kaW5nQnlUcmFuc2Zvcm1JZC5nZXQod2lkZ2V0SWQpO1xuXHRcdGlmIChieVRyYW5zZm9ybSAmJiBieVRyYW5zZm9ybS5vcCA9PT0gb3ApIHJldHVybiBieVRyYW5zZm9ybTtcblx0XHRjb25zdCBieVNvdXJjZSA9IHRoaXMuZ3VtYmFsbEJpbmRpbmdzLmdldCh0aGlzLmd1bWJhbGxCaW5kaW5nS2V5KHdpZGdldElkLCBvcCkpO1xuXHRcdHJldHVybiBieVNvdXJjZSA/PyBudWxsO1xuXHR9XG5cblx0cHJpdmF0ZSByZXNvbHZlR3VtYmFsbFNvdXJjZVdpZGdldElkKHdpZGdldElkOiBzdHJpbmcsIG9wOiBQcm9jZWR1cmFsR3VtYmFsbFRyYW5zZm9ybU9wKTogc3RyaW5nIHtcblx0XHRjb25zdCBieVRyYW5zZm9ybSA9IHRoaXMuZ3VtYmFsbEJpbmRpbmdCeVRyYW5zZm9ybUlkLmdldCh3aWRnZXRJZCk7XG5cdFx0aWYgKGJ5VHJhbnNmb3JtICYmIGJ5VHJhbnNmb3JtLm9wID09PSBvcCkgcmV0dXJuIGJ5VHJhbnNmb3JtLnNvdXJjZVdpZGdldElkO1xuXHRcdHJldHVybiB3aWRnZXRJZDtcblx0fVxuXG5cdHByaXZhdGUgY2xlYXJHdW1iYWxsRHJhZygpOiB2b2lkIHtcblx0XHR0aGlzLmd1bWJhbGxEcmFnU2Vzc2lvbiA9IG51bGw7XG5cdFx0dGhpcy5ndW1iYWxsQWN0aXZlV2lkZ2V0SWRzID0gW107XG5cdH1cblxuXHRwcml2YXRlIHN5bmNHdW1iYWxsQWN0aXZlQ2hyb21lKGJpbmRpbmc6IEd1bWJhbGxUcmFuc2Zvcm1CaW5kaW5nKTogdm9pZCB7XG5cdFx0Y29uc3QgbmV4dEFjdGl2ZSA9IFtiaW5kaW5nLnRyYW5zZm9ybUlkLCBiaW5kaW5nLnNvdXJjZVdpZGdldElkXTtcblx0XHRpZiAoSlNPTi5zdHJpbmdpZnkobmV4dEFjdGl2ZSkgIT09IEpTT04uc3RyaW5naWZ5KHRoaXMuZ3VtYmFsbEFjdGl2ZVdpZGdldElkcykpIHtcblx0XHRcdHRoaXMuZ3VtYmFsbEFjdGl2ZVdpZGdldElkcyA9IG5leHRBY3RpdmU7XG5cdFx0XHR0aGlzLmludGVyYWN0aW9uUmV2aXNpb24gKz0gMTtcblx0XHRcdHRoaXMubm90aWZ5U25hcHNob3QoKTtcblx0XHR9XG5cdH1cblxuXHRwcml2YXRlIGRpc3BhdGNoRmxvd0NhbnZhc1NlbGVjdGlvbihpZHM6IHJlYWRvbmx5IHN0cmluZ1tdKTogdm9pZCB7XG5cdFx0dGhpcy5ydW4oXCJjYW52YXNDb21tYW5kXCIsIHsgY29tbWFuZDogXCJzZXRTZWxlY3Rpb25cIiwgYXJnc0pzb246IEpTT04uc3RyaW5naWZ5KHsgaWRzOiBbLi4uaWRzXSB9KSB9KTtcblx0fVxuXG5cdHByaXZhdGUgZGlzcGF0Y2hHcmFwaEVkaXQob3BzOiByZWFkb25seSBGbG93R3JhcGhFZGl0T3BbXSwgc2VsZWN0VHJhbnNmb3JtSWQ/OiBzdHJpbmcpOiB2b2lkIHtcblx0XHR0aGlzLnJ1bihcImNhbnZhc0NvbW1hbmRcIiwgeyBjb21tYW5kOiBcImdyYXBoRWRpdFwiLCBhcmdzSnNvbjogSlNPTi5zdHJpbmdpZnkoeyBvcHMgfSkgfSk7XG5cdFx0Y29uc3QgYmluZGluZyA9IHRoaXMuZ3VtYmFsbERyYWdTZXNzaW9uPy5iaW5kaW5nO1xuXHRcdGlmIChiaW5kaW5nKSB7XG5cdFx0XHR0aGlzLmRpc3BhdGNoRmxvd0NhbnZhc1NlbGVjdGlvbihndW1iYWxsQmluZGluZ05vZGVJZHMoYmluZGluZykpO1xuXHRcdFx0dGhpcy5zeW5jR3VtYmFsbEFjdGl2ZUNocm9tZShiaW5kaW5nKTtcblx0XHRcdHJldHVybjtcblx0XHR9XG5cdFx0aWYgKHNlbGVjdFRyYW5zZm9ybUlkKSB7XG5cdFx0XHR0aGlzLnJ1bihcInNldFNlbGVjdGlvblwiLCB7IGlkczogW3NlbGVjdFRyYW5zZm9ybUlkXSwgbW9kZTogXCJkZWZhdWx0XCIgfSk7XG5cdFx0fVxuXHR9XG5cblx0cHJpdmF0ZSBhcHBseUxpdmVHdW1iYWxsRHJhZyhyZXF1ZXN0OiBQcm9jZWR1cmFsR3VtYmFsbFRyYW5zZm9ybVJlcXVlc3QpOiB2b2lkIHtcblx0XHRjb25zdCBzZXNzaW9uID0gdGhpcy5ndW1iYWxsRHJhZ1Nlc3Npb247XG5cdFx0aWYgKCFzZXNzaW9uKSByZXR1cm47XG5cdFx0Y29uc3QgdmFsdWVzID0gYXBwbHlHdW1iYWxsRGVsdGFUb0Jhc2Uoc2Vzc2lvbi5iYXNlVmFsdWVzLCBzZXNzaW9uLmJpbmRpbmcub3AsIHJlcXVlc3QuZGVsdGEpO1xuXHRcdHNldEd1bWJhbGxCaW5kaW5nVmFsdWVzKHNlc3Npb24uYmluZGluZywgdmFsdWVzKTtcblx0XHR0aGlzLmRpc3BhdGNoR3JhcGhFZGl0KHRoaXMuYnVpbGRHdW1iYWxsVXBkYXRlT3BzKHNlc3Npb24uYmluZGluZykpO1xuXHR9XG5cblx0cHJpdmF0ZSBiZWdpbkd1bWJhbGxEcmFnKHJlcXVlc3Q6IFByb2NlZHVyYWxHdW1iYWxsVHJhbnNmb3JtUmVxdWVzdCk6IHZvaWQge1xuXHRcdGNvbnN0IG9wID0gcmVxdWVzdC5kZWx0YS5vcDtcblx0XHRsZXQgYmluZGluZyA9IHRoaXMuZmluZEd1bWJhbGxCaW5kaW5nKHJlcXVlc3Qud2lkZ2V0SWQsIG9wKTtcblx0XHRsZXQgaW5zZXJ0T3BzOiBGbG93R3JhcGhFZGl0T3BbXSB8IG51bGwgPSBudWxsO1xuXHRcdGlmICghYmluZGluZykge1xuXHRcdFx0Y29uc3Qgc291cmNlV2lkZ2V0SWQgPSB0aGlzLnJlc29sdmVHdW1iYWxsU291cmNlV2lkZ2V0SWQocmVxdWVzdC53aWRnZXRJZCwgb3ApO1xuXHRcdFx0Y29uc3QgY3JlYXRlZCA9IHRoaXMuYnVpbGRHdW1iYWxsSW5zZXJ0T3BzKHNvdXJjZVdpZGdldElkLCBvcCwgZ3VtYmFsbFplcm9EZWx0YShvcCksIHJlcXVlc3QuZ3JhbnVsYXJpdHkpO1xuXHRcdFx0dGhpcy5yZWdpc3Rlckd1bWJhbGxCaW5kaW5nKGNyZWF0ZWQuYmluZGluZyk7XG5cdFx0XHRiaW5kaW5nID0gY3JlYXRlZC5iaW5kaW5nO1xuXHRcdFx0aW5zZXJ0T3BzID0gY3JlYXRlZC5vcHM7XG5cdFx0XHRjb25zb2xlLmxvZyhgW0RFQlVHXSBndW1iYWxsIGluc2VydCAke2JpbmRpbmcudHJhbnNmb3JtSWR9IHNvdXJjZT0ke3NvdXJjZVdpZGdldElkfSBvcD0ke29wfSBncmFudWxhcml0eT0ke3JlcXVlc3QuZ3JhbnVsYXJpdHl9YCk7XG5cdFx0fVxuXHRcdHRoaXMuZ3VtYmFsbERyYWdTZXNzaW9uID0geyBiaW5kaW5nLCBiYXNlVmFsdWVzOiBjb3B5R3VtYmFsbFZhbHVlcyhiaW5kaW5nKSB9O1xuXHRcdGNvbnN0IHZhbHVlcyA9IGFwcGx5R3VtYmFsbERlbHRhVG9CYXNlKHRoaXMuZ3VtYmFsbERyYWdTZXNzaW9uLmJhc2VWYWx1ZXMsIG9wLCByZXF1ZXN0LmRlbHRhKTtcblx0XHRzZXRHdW1iYWxsQmluZGluZ1ZhbHVlcyhiaW5kaW5nLCB2YWx1ZXMpO1xuXHRcdGlmIChpbnNlcnRPcHMpIHtcblx0XHRcdHRoaXMuZGlzcGF0Y2hHcmFwaEVkaXQoaW5zZXJ0T3BzKTtcblx0XHRcdHJldHVybjtcblx0XHR9XG5cdFx0dGhpcy5kaXNwYXRjaEdyYXBoRWRpdCh0aGlzLmJ1aWxkR3VtYmFsbFVwZGF0ZU9wcyhiaW5kaW5nKSk7XG5cdH1cblxuXHRwcml2YXRlIGZpbmlzaEd1bWJhbGxEcmFnKHJlcXVlc3Q6IFByb2NlZHVyYWxHdW1iYWxsVHJhbnNmb3JtUmVxdWVzdCk6IHZvaWQge1xuXHRcdGNvbnN0IHNlc3Npb24gPSB0aGlzLmd1bWJhbGxEcmFnU2Vzc2lvbjtcblx0XHRpZiAoc2Vzc2lvbikge1xuXHRcdFx0Y29uc3QgYmluZGluZyA9IHNlc3Npb24uYmluZGluZztcblx0XHRcdGNvbnN0IHZhbHVlcyA9IGFwcGx5R3VtYmFsbERlbHRhVG9CYXNlKHNlc3Npb24uYmFzZVZhbHVlcywgYmluZGluZy5vcCwgcmVxdWVzdC5kZWx0YSk7XG5cdFx0XHRzZXRHdW1iYWxsQmluZGluZ1ZhbHVlcyhiaW5kaW5nLCB2YWx1ZXMpO1xuXHRcdFx0Y29uc29sZS5sb2coYFtERUJVR10gZ3VtYmFsbCBlbmQgJHtiaW5kaW5nLnRyYW5zZm9ybUlkfSBvcD0ke2JpbmRpbmcub3B9YCk7XG5cdFx0XHR0aGlzLmNsZWFyR3VtYmFsbERyYWcoKTtcblx0XHRcdHRoaXMuZGlzcGF0Y2hHcmFwaEVkaXQodGhpcy5idWlsZEd1bWJhbGxVcGRhdGVPcHMoYmluZGluZykpO1xuXHRcdFx0dGhpcy5ydW4oXCJzZXRTZWxlY3Rpb25cIiwgeyBpZHM6IFtiaW5kaW5nLnRyYW5zZm9ybUlkXSwgbW9kZTogXCJkZWZhdWx0XCIgfSk7XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdHRoaXMuYXBwbHlHdW1iYWxsVHJhbnNmb3JtQ29tbWl0dGVkKHJlcXVlc3QpO1xuXHR9XG5cblx0cHJpdmF0ZSBhcHBseUd1bWJhbGxUcmFuc2Zvcm1Db21taXR0ZWQocmVxdWVzdDogUHJvY2VkdXJhbEd1bWJhbGxUcmFuc2Zvcm1SZXF1ZXN0KTogdm9pZCB7XG5cdFx0Y29uc3Qgb3AgPSByZXF1ZXN0LmRlbHRhLm9wO1xuXHRcdGNvbnN0IGdyYW51bGFyaXR5ID0gcmVxdWVzdC5ncmFudWxhcml0eTtcblx0XHRjb25zdCBleGlzdGluZyA9IHRoaXMuZmluZEd1bWJhbGxCaW5kaW5nKHJlcXVlc3Qud2lkZ2V0SWQsIG9wKTtcblx0XHRpZiAoZXhpc3RpbmcpIHtcblx0XHRcdGFjY3VtdWxhdGVHdW1iYWxsRGVsdGEoZXhpc3RpbmcsIHJlcXVlc3QuZGVsdGEpO1xuXHRcdFx0Y29uc3Qgb3BzID0gdGhpcy5idWlsZEd1bWJhbGxVcGRhdGVPcHMoZXhpc3RpbmcpO1xuXHRcdFx0Y29uc29sZS5sb2coYFtERUJVR10gZ3VtYmFsbCB1cGRhdGUgJHtleGlzdGluZy50cmFuc2Zvcm1JZH0gb3A9JHtvcH0gZ3JhbnVsYXJpdHk9JHtncmFudWxhcml0eX1gKTtcblx0XHRcdHRoaXMuZGlzcGF0Y2hHcmFwaEVkaXQob3BzLCBleGlzdGluZy50cmFuc2Zvcm1JZCk7XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdGNvbnN0IHNvdXJjZVdpZGdldElkID0gdGhpcy5yZXNvbHZlR3VtYmFsbFNvdXJjZVdpZGdldElkKHJlcXVlc3Qud2lkZ2V0SWQsIG9wKTtcblx0XHRjb25zdCB7IG9wcywgYmluZGluZyB9ID0gdGhpcy5idWlsZEd1bWJhbGxJbnNlcnRPcHMoc291cmNlV2lkZ2V0SWQsIG9wLCByZXF1ZXN0LmRlbHRhLCBncmFudWxhcml0eSk7XG5cdFx0dGhpcy5yZWdpc3Rlckd1bWJhbGxCaW5kaW5nKGJpbmRpbmcpO1xuXHRcdGNvbnNvbGUubG9nKGBbREVCVUddIGd1bWJhbGwgaW5zZXJ0ICR7YmluZGluZy50cmFuc2Zvcm1JZH0gc291cmNlPSR7c291cmNlV2lkZ2V0SWR9IG9wPSR7b3B9IGdyYW51bGFyaXR5PSR7Z3JhbnVsYXJpdHl9YCk7XG5cdFx0dGhpcy5kaXNwYXRjaEdyYXBoRWRpdChvcHMsIGJpbmRpbmcudHJhbnNmb3JtSWQpO1xuXHR9XG5cblx0cHJpdmF0ZSBidWlsZEd1bWJhbGxVcGRhdGVPcHMoYmluZGluZzogR3VtYmFsbFRyYW5zZm9ybUJpbmRpbmcpOiBGbG93R3JhcGhFZGl0T3BbXSB7XG5cdFx0aWYgKGJpbmRpbmcuZ3JhbnVsYXJpdHkgPT09IFwiY29tcGFjdFwiKSB7XG5cdFx0XHRyZXR1cm4gW3sgb3A6IFwic2V0TmV1cm9uUGFyYW1zXCIsIGlkOiBiaW5kaW5nLnRyYW5zZm9ybUlkLCBwYXJhbXNKc29uOiBKU09OLnN0cmluZ2lmeShjb21wYWN0TmV1cm9uUGFyYW1zKGJpbmRpbmcpKSB9XTtcblx0XHR9XG5cdFx0aWYgKGJpbmRpbmcub3AgPT09IFwidHJhbnNsYXRlXCIgJiYgYmluZGluZy52ZWN0b3JJZCAmJiBiaW5kaW5nLnZhbHVlV2lkZ2V0SWRzLmxlbmd0aCA9PT0gMykge1xuXHRcdFx0Y29uc3QgW3N4LCBzeSwgc3pdID0gYmluZGluZy52YWx1ZVdpZGdldElkcztcblx0XHRcdGNvbnN0IFt4LCB5LCB6XSA9IGJpbmRpbmcudmFsdWVzLm9mZnNldDtcblx0XHRcdHJldHVybiBbXG5cdFx0XHRcdHsgb3A6IFwic2V0U2xpZGVyVmFsdWVcIiwgaWQ6IHN4LCB2YWx1ZTogeCB9LFxuXHRcdFx0XHR7IG9wOiBcInNldFNsaWRlclZhbHVlXCIsIGlkOiBzeSwgdmFsdWU6IHkgfSxcblx0XHRcdFx0eyBvcDogXCJzZXRTbGlkZXJWYWx1ZVwiLCBpZDogc3osIHZhbHVlOiB6IH0sXG5cdFx0XHRdO1xuXHRcdH1cblx0XHRjb25zdCBzbGlkZXJJZCA9IGJpbmRpbmcudmFsdWVXaWRnZXRJZHNbMF07XG5cdFx0aWYgKCFzbGlkZXJJZCkgcmV0dXJuIFtdO1xuXHRcdGlmIChiaW5kaW5nLm9wID09PSBcInJvdGF0ZVwiKSB7XG5cdFx0XHRyZXR1cm4gW3sgb3A6IFwic2V0U2xpZGVyVmFsdWVcIiwgaWQ6IHNsaWRlcklkLCB2YWx1ZTogYmluZGluZy52YWx1ZXMuYW5nbGUgfV07XG5cdFx0fVxuXHRcdHJldHVybiBbeyBvcDogXCJzZXRTbGlkZXJWYWx1ZVwiLCBpZDogc2xpZGVySWQsIHZhbHVlOiBiaW5kaW5nLnZhbHVlcy5mYWN0b3IgfV07XG5cdH1cblxuXHRwcml2YXRlIGJ1aWxkR3VtYmFsbEluc2VydE9wcyhcblx0XHRzb3VyY2VXaWRnZXRJZDogc3RyaW5nLFxuXHRcdG9wOiBQcm9jZWR1cmFsR3VtYmFsbFRyYW5zZm9ybU9wLFxuXHRcdGRlbHRhOiBQcm9jZWR1cmFsR3VtYmFsbFRyYW5zZm9ybURlbHRhLFxuXHRcdGdyYW51bGFyaXR5OiBQcm9jZWR1cmFsVHJhbnNmb3JtR3JhbnVsYXJpdHksXG5cdCk6IHsgb3BzOiBGbG93R3JhcGhFZGl0T3BbXTsgYmluZGluZzogR3VtYmFsbFRyYW5zZm9ybUJpbmRpbmcgfSB7XG5cdFx0Y29uc3Qgc291cmNlTGF5b3V0ID0gd2lkZ2V0TGF5b3V0RnJvbUZpeHR1cmUodGhpcy5maXh0dXJlSnNvbiwgc291cmNlV2lkZ2V0SWQpO1xuXHRcdGNvbnN0IGVkZ2VHYXAgPSBndW1iYWxsQ29sdW1uRWRnZUdhcCh0aGlzLmxheWVyU3BhY2luZywgdGhpcy5zaWJsaW5nR2FwKTtcblx0XHRjb25zdCB2YWx1ZVJvd0dhcCA9IGd1bWJhbGxWYWx1ZVJvd0dhcCh0aGlzLnNpYmxpbmdHYXApO1xuXHRcdGNvbnN0IHNvdXJjZUhhbGYgPSBHVU1CQUxMX1NPVVJDRV9IQUxGX1dJRFRIO1xuXHRcdGNvbnN0IHNsaWRlckhhbGYgPSBHVU1CQUxMX1NMSURFUl9IQUxGX1dJRFRIO1xuXHRcdGNvbnN0IHZlY3RvckhhbGYgPSBHVU1CQUxMX1ZFQ1RPUl9IQUxGX1dJRFRIO1xuXHRcdGNvbnN0IHRyYW5zZm9ybUhhbGYgPSBHVU1CQUxMX05FVVJPTl9IQUxGX1dJRFRIO1xuXHRcdGNvbnN0IHRyYW5zZm9ybUlkID0gYCR7c291cmNlV2lkZ2V0SWR9X2d1bWJhbGxfJHtvcH1gO1xuXHRcdGNvbnN0IHZlY3RvcklkID0gYCR7dHJhbnNmb3JtSWR9X3ZlY3RvcmA7XG5cdFx0Y29uc3Qgc2xpZGVyWElkID0gYCR7dHJhbnNmb3JtSWR9X3N4YDtcblx0XHRjb25zdCBzbGlkZXJZSWQgPSBgJHt0cmFuc2Zvcm1JZH1fc3lgO1xuXHRcdGNvbnN0IHNsaWRlclpJZCA9IGAke3RyYW5zZm9ybUlkfV9zemA7XG5cdFx0Y29uc3Qgc2NhbGFyU2xpZGVySWQgPSBgJHt0cmFuc2Zvcm1JZH1fdmFsdWVgO1xuXHRcdGNvbnN0IGJpbmRpbmc6IEd1bWJhbGxUcmFuc2Zvcm1CaW5kaW5nID0ge1xuXHRcdFx0c291cmNlV2lkZ2V0SWQsXG5cdFx0XHR0cmFuc2Zvcm1JZCxcblx0XHRcdG9wLFxuXHRcdFx0Z3JhbnVsYXJpdHksXG5cdFx0XHR2YWx1ZVdpZGdldElkczogW10sXG5cdFx0XHR2ZWN0b3JJZDogdW5kZWZpbmVkLFxuXHRcdFx0dmFsdWVzOiB7XG5cdFx0XHRcdG9mZnNldDpcblx0XHRcdFx0XHRkZWx0YS5vcCA9PT0gXCJ0cmFuc2xhdGVcIiA/IFtkZWx0YS5vZmZzZXRbMF0sIGRlbHRhLm9mZnNldFsxXSwgZGVsdGEub2Zmc2V0WzJdXSA6IChbMCwgMCwgMF0gYXMgW251bWJlciwgbnVtYmVyLCBudW1iZXJdKSxcblx0XHRcdFx0YW5nbGU6IGRlbHRhLm9wID09PSBcInJvdGF0ZVwiID8gZGVsdGEuYW5nbGUgOiAwLFxuXHRcdFx0XHRmYWN0b3I6IGRlbHRhLm9wID09PSBcInNjYWxlXCIgPyBkZWx0YS5mYWN0b3IgOiAxLFxuXHRcdFx0fSxcblx0XHR9O1xuXHRcdGxldCB0cmFuc2Zvcm1Db2x1bW5YID0gZ3VtYmFsbENvbHVtbkFmdGVyKHNvdXJjZUxheW91dC54LCBzb3VyY2VIYWxmLCB0cmFuc2Zvcm1IYWxmLCBlZGdlR2FwKTtcblx0XHRjb25zdCBvcHM6IEZsb3dHcmFwaEVkaXRPcFtdID0gW107XG5cdFx0aWYgKGdyYW51bGFyaXR5ID09PSBcImNvbXBhY3RcIikge1xuXHRcdFx0b3BzLnB1c2goeyBvcDogXCJtYWtlU3BhY2VcIiwgYW5jaG9yOiBzb3VyY2VXaWRnZXRJZCwgZHg6IGd1bWJhbGxNYWtlU3BhY2VEeCh0cmFuc2Zvcm1Db2x1bW5YLCB0cmFuc2Zvcm1IYWxmLCBzb3VyY2VMYXlvdXQueCwgZWRnZUdhcCksIGR5OiAwIH0pO1xuXHRcdFx0b3BzLnB1c2goe1xuXHRcdFx0XHRvcDogXCJhZGRXaWRnZXRcIixcblx0XHRcdFx0ZGVzY3JpcHRvcjogbmV1cm9uRGVzY3JpcHRvcih0cmFuc2Zvcm1JZCwgQlJFUF9YRk9STV9ORVVST05fS0lORFtvcF0pLFxuXHRcdFx0XHR4OiB0cmFuc2Zvcm1Db2x1bW5YLFxuXHRcdFx0XHR5OiBzb3VyY2VMYXlvdXQueSxcblx0XHRcdH0pO1xuXHRcdFx0b3BzLnB1c2goeyBvcDogXCJzZXROZXVyb25QYXJhbXNcIiwgaWQ6IHRyYW5zZm9ybUlkLCBwYXJhbXNKc29uOiBKU09OLnN0cmluZ2lmeShjb21wYWN0TmV1cm9uUGFyYW1zKGJpbmRpbmcpKSB9KTtcblx0XHR9IGVsc2UgaWYgKG9wID09PSBcInRyYW5zbGF0ZVwiKSB7XG5cdFx0XHRiaW5kaW5nLnZhbHVlV2lkZ2V0SWRzID0gW3NsaWRlclhJZCwgc2xpZGVyWUlkLCBzbGlkZXJaSWRdO1xuXHRcdFx0YmluZGluZy52ZWN0b3JJZCA9IHZlY3RvcklkO1xuXHRcdFx0Y29uc3QgdmFsdWVDb2x1bW5YID0gZ3VtYmFsbENvbHVtbkFmdGVyKHNvdXJjZUxheW91dC54LCBzb3VyY2VIYWxmLCBzbGlkZXJIYWxmLCBlZGdlR2FwKTtcblx0XHRcdGNvbnN0IHZlY3RvckNvbHVtblggPSBndW1iYWxsQ29sdW1uQWZ0ZXIodmFsdWVDb2x1bW5YLCBzbGlkZXJIYWxmLCB2ZWN0b3JIYWxmLCBlZGdlR2FwKTtcblx0XHRcdHRyYW5zZm9ybUNvbHVtblggPSBndW1iYWxsQ29sdW1uQWZ0ZXIodmVjdG9yQ29sdW1uWCwgdmVjdG9ySGFsZiwgdHJhbnNmb3JtSGFsZiwgZWRnZUdhcCk7XG5cdFx0XHRvcHMucHVzaCh7IG9wOiBcIm1ha2VTcGFjZVwiLCBhbmNob3I6IHNvdXJjZVdpZGdldElkLCBkeDogZ3VtYmFsbE1ha2VTcGFjZUR4KHRyYW5zZm9ybUNvbHVtblgsIHRyYW5zZm9ybUhhbGYsIHNvdXJjZUxheW91dC54LCBlZGdlR2FwKSwgZHk6IDAgfSk7XG5cdFx0XHRjb25zdCBbeCwgeSwgel0gPSBiaW5kaW5nLnZhbHVlcy5vZmZzZXQ7XG5cdFx0XHRvcHMucHVzaChcblx0XHRcdFx0eyBvcDogXCJhZGRXaWRnZXRcIiwgZGVzY3JpcHRvcjogc2xpZGVyRGVzY3JpcHRvcihzbGlkZXJYSWQsIHgpLCB4OiB2YWx1ZUNvbHVtblgsIHk6IHNvdXJjZUxheW91dC55IC0gdmFsdWVSb3dHYXAgfSxcblx0XHRcdFx0eyBvcDogXCJhZGRXaWRnZXRcIiwgZGVzY3JpcHRvcjogc2xpZGVyRGVzY3JpcHRvcihzbGlkZXJZSWQsIHkpLCB4OiB2YWx1ZUNvbHVtblgsIHk6IHNvdXJjZUxheW91dC55IH0sXG5cdFx0XHRcdHsgb3A6IFwiYWRkV2lkZ2V0XCIsIGRlc2NyaXB0b3I6IHNsaWRlckRlc2NyaXB0b3Ioc2xpZGVyWklkLCB6KSwgeDogdmFsdWVDb2x1bW5YLCB5OiBzb3VyY2VMYXlvdXQueSArIHZhbHVlUm93R2FwIH0sXG5cdFx0XHRcdHsgb3A6IFwiYWRkV2lkZ2V0XCIsIGRlc2NyaXB0b3I6IG5ldXJvbkRlc2NyaXB0b3IodmVjdG9ySWQsIFwiYnJlcC52ZWN0b3JcIiksIHg6IHZlY3RvckNvbHVtblgsIHk6IHNvdXJjZUxheW91dC55IH0sXG5cdFx0XHRcdHsgb3A6IFwiYWRkV2lkZ2V0XCIsIGRlc2NyaXB0b3I6IG5ldXJvbkRlc2NyaXB0b3IodHJhbnNmb3JtSWQsIEJSRVBfWEZPUk1fTkVVUk9OX0tJTkQudHJhbnNsYXRlKSwgeDogdHJhbnNmb3JtQ29sdW1uWCwgeTogc291cmNlTGF5b3V0LnkgfSxcblx0XHRcdFx0eyBvcDogXCJjb25uZWN0UG9ydHNcIiwgZnJvbTogc2xpZGVyWElkLCBmcm9tUG9ydDogXCJudW1iZXJcIiwgdG86IHZlY3RvcklkLCB0b1BvcnQ6IFwieFwiIH0sXG5cdFx0XHRcdHsgb3A6IFwiY29ubmVjdFBvcnRzXCIsIGZyb206IHNsaWRlcllJZCwgZnJvbVBvcnQ6IFwibnVtYmVyXCIsIHRvOiB2ZWN0b3JJZCwgdG9Qb3J0OiBcInlcIiB9LFxuXHRcdFx0XHR7IG9wOiBcImNvbm5lY3RQb3J0c1wiLCBmcm9tOiBzbGlkZXJaSWQsIGZyb21Qb3J0OiBcIm51bWJlclwiLCB0bzogdmVjdG9ySWQsIHRvUG9ydDogXCJ6XCIgfSxcblx0XHRcdFx0eyBvcDogXCJjb25uZWN0UG9ydHNcIiwgZnJvbTogdmVjdG9ySWQsIGZyb21Qb3J0OiBcInZlY3RvclwiLCB0bzogdHJhbnNmb3JtSWQsIHRvUG9ydDogXCJvZmZzZXRcIiB9LFxuXHRcdFx0KTtcblx0XHR9IGVsc2Uge1xuXHRcdFx0YmluZGluZy52YWx1ZVdpZGdldElkcyA9IFtzY2FsYXJTbGlkZXJJZF07XG5cdFx0XHRjb25zdCB2YWx1ZUNvbHVtblggPSBndW1iYWxsQ29sdW1uQWZ0ZXIoc291cmNlTGF5b3V0LngsIHNvdXJjZUhhbGYsIHNsaWRlckhhbGYsIGVkZ2VHYXApO1xuXHRcdFx0dHJhbnNmb3JtQ29sdW1uWCA9IGd1bWJhbGxDb2x1bW5BZnRlcih2YWx1ZUNvbHVtblgsIHNsaWRlckhhbGYsIHRyYW5zZm9ybUhhbGYsIGVkZ2VHYXApO1xuXHRcdFx0b3BzLnB1c2goeyBvcDogXCJtYWtlU3BhY2VcIiwgYW5jaG9yOiBzb3VyY2VXaWRnZXRJZCwgZHg6IGd1bWJhbGxNYWtlU3BhY2VEeCh0cmFuc2Zvcm1Db2x1bW5YLCB0cmFuc2Zvcm1IYWxmLCBzb3VyY2VMYXlvdXQueCwgZWRnZUdhcCksIGR5OiAwIH0pO1xuXHRcdFx0Y29uc3Qgc2NhbGFyVmFsdWUgPSBvcCA9PT0gXCJyb3RhdGVcIiA/IGJpbmRpbmcudmFsdWVzLmFuZ2xlIDogYmluZGluZy52YWx1ZXMuZmFjdG9yO1xuXHRcdFx0b3BzLnB1c2goXG5cdFx0XHRcdHsgb3A6IFwiYWRkV2lkZ2V0XCIsIGRlc2NyaXB0b3I6IHNsaWRlckRlc2NyaXB0b3Ioc2NhbGFyU2xpZGVySWQsIHNjYWxhclZhbHVlKSwgeDogdmFsdWVDb2x1bW5YLCB5OiBzb3VyY2VMYXlvdXQueSB9LFxuXHRcdFx0XHR7IG9wOiBcImFkZFdpZGdldFwiLCBkZXNjcmlwdG9yOiBuZXVyb25EZXNjcmlwdG9yKHRyYW5zZm9ybUlkLCBCUkVQX1hGT1JNX05FVVJPTl9LSU5EW29wXSksIHg6IHRyYW5zZm9ybUNvbHVtblgsIHk6IHNvdXJjZUxheW91dC55IH0sXG5cdFx0XHRcdHtcblx0XHRcdFx0XHRvcDogXCJjb25uZWN0UG9ydHNcIixcblx0XHRcdFx0XHRmcm9tOiBzY2FsYXJTbGlkZXJJZCxcblx0XHRcdFx0XHRmcm9tUG9ydDogXCJudW1iZXJcIixcblx0XHRcdFx0XHR0bzogdHJhbnNmb3JtSWQsXG5cdFx0XHRcdFx0dG9Qb3J0OiBvcCA9PT0gXCJyb3RhdGVcIiA/IFwiYW5nbGVcIiA6IFwiZmFjdG9yXCIsXG5cdFx0XHRcdH0sXG5cdFx0XHQpO1xuXHRcdH1cblx0XHRvcHMucHVzaCh7XG5cdFx0XHRvcDogXCJpbnNlcnRCZXR3ZWVuXCIsXG5cdFx0XHRhbmNob3I6IHNvdXJjZVdpZGdldElkLFxuXHRcdFx0YW5jaG9yT3V0UG9ydDogXCJzb2xpZFwiLFxuXHRcdFx0bWlkOiB0cmFuc2Zvcm1JZCxcblx0XHRcdG1pZEluUG9ydDogXCJnZW9tZXRyeVwiLFxuXHRcdFx0bWlkT3V0UG9ydDogXCJnZW9tZXRyeVwiLFxuXHRcdH0pO1xuXHRcdG9wcy5wdXNoKHsgb3A6IFwic2V0UHJldmlld09mZlwiLCBpZHM6IFtzb3VyY2VXaWRnZXRJZF0gfSk7XG5cdFx0cmV0dXJuIHsgb3BzLCBiaW5kaW5nIH07XG5cdH1cblxuXHQvKiogQGVtb2ppIPCfjpsgSW5zZXJ0cyBvciB1cGRhdGVzIGd1bWJhbGwtZHJpdmVuIHRyYW5zZm9ybSBub2RlcyBpbiB0aGUgZmxvdyBncmFwaC4gKi9cblx0YXBwbHlHdW1iYWxsVHJhbnNmb3JtKHJlcXVlc3Q6IFByb2NlZHVyYWxHdW1iYWxsVHJhbnNmb3JtUmVxdWVzdCk6IHZvaWQge1xuXHRcdGNvbnN0IHBoYXNlOiBQcm9jZWR1cmFsR3VtYmFsbFRyYW5zZm9ybVBoYXNlID0gcmVxdWVzdC5waGFzZSA/PyBcImVuZFwiO1xuXHRcdGlmIChwaGFzZSA9PT0gXCJzdGFydFwiKSB7XG5cdFx0XHR0aGlzLmJlZ2luR3VtYmFsbERyYWcocmVxdWVzdCk7XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdGlmIChwaGFzZSA9PT0gXCJsaXZlXCIpIHtcblx0XHRcdHRoaXMuYXBwbHlMaXZlR3VtYmFsbERyYWcocmVxdWVzdCk7XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdHRoaXMuZmluaXNoR3VtYmFsbERyYWcocmVxdWVzdCk7XG5cdH1cblxuXHRsb2RNb2RlRm9yU2NvcGUoc2NvcGVJZDogc3RyaW5nKTogRGFnTG9kTW9kZUtpbmQge1xuXHRcdHJldHVybiB0aGlzLmxvZE1vZGVCeUluc3RhbmNlW3Njb3BlSWRdID8/IHRoaXMubG9kTW9kZTtcblx0fVxuXG5cdHByb3hpbWl0eURpc3RhbmNlVmFsdWUoKTogbnVtYmVyIHtcblx0XHRyZXR1cm4gdGhpcy5wcm94aW1pdHlEaXN0YW5jZTtcblx0fVxuXG5cdHByaXZhdGUgbG9kTWVhc3VyZShzY29wZUlkOiBzdHJpbmcpOiBXaW5kb3dNZWFzdXJlIHtcblx0XHRyZXR1cm4ge1xuXHRcdFx0a2luZDogXCJzZWxlY3RcIixcblx0XHRcdGlkOiBgJHtzY29wZUlkfS1sb2RgLFxuXHRcdFx0bGFiZWw6IFwiTE9EXCIsXG5cdFx0XHR2YWx1ZTogdGhpcy5sb2RNb2RlRm9yU2NvcGUoc2NvcGVJZCksXG5cdFx0XHRpdGVtczogW1xuXHRcdFx0XHR7IGlkOiBcImF1dG9tYXRpY1wiLCB2YWx1ZTogREFHX0xPRF9NT0RFX0FVVE9NQVRJQywgbGFiZWw6IGRhZ0xvZEF1dG9tYXRpY1NlbGVjdExhYmVsKHRoaXMuZWZmZWN0aXZlTG9kKSB9LFxuXHRcdFx0XHQuLi5kYWdQbGF5TG9kVGllcnMoKS5tYXAoKHRpZXIpID0+ICh7IGlkOiB0aWVyLCB2YWx1ZTogdGllciwgbGFiZWw6IGRhZ1BsYXlMb2RUaWVyTWVudUxhYmVsKHRpZXIpIH0pKSxcblx0XHRcdF0sXG5cdFx0XHRvbkNoYW5nZTogeyBjb250cm9sbGVySWQ6IFBST0NFRFVSQUxfM0RfUExBWV9DT05UUk9MTEVSX0lELCBjb21tYW5kOiBcInNldExvZE1vZGVcIiwgYXJnczogeyBpbnN0YW5jZUlkOiBzY29wZUlkIH0gfSxcblx0XHR9O1xuXHR9XG5cblx0cHJpdmF0ZSBwcm94aW1pdHlNZWFzdXJlKCk6IFdpbmRvd01lYXN1cmUge1xuXHRcdHJldHVybiB7XG5cdFx0XHRraW5kOiBcInNsaWRlclwiLFxuXHRcdFx0aWQ6IFwicHJvY2VkdXJhbC1mbG93LXByb3hpbWl0eS1kaXN0YW5jZVwiLFxuXHRcdFx0bGFiZWw6IFwiUHJveGltaXR5XCIsXG5cdFx0XHR2YWx1ZTogdGhpcy5wcm94aW1pdHlEaXN0YW5jZSxcblx0XHRcdG1pbjogMCxcblx0XHRcdG1heDogMjQwLFxuXHRcdFx0c3RlcDogNCxcblx0XHRcdG9uQ2hhbmdlOiB7IGNvbnRyb2xsZXJJZDogUFJPQ0VEVVJBTF8zRF9QTEFZX0NPTlRST0xMRVJfSUQsIGNvbW1hbmQ6IFwic2V0UHJveGltaXR5RGlzdGFuY2VcIiB9LFxuXHRcdH07XG5cdH1cblxuXHRwcml2YXRlIGZsb3dXaW5kb3dNZWFzdXJlcygpOiByZWFkb25seSBXaW5kb3dNZWFzdXJlW10ge1xuXHRcdHJldHVybiBbdGhpcy5sb2RNZWFzdXJlKFBST0NFRFVSQUxfUExBWV9XSU5ET1dfS0lORF9JRCksIHRoaXMucHJveGltaXR5TWVhc3VyZSgpXTtcblx0fVxuXG5cdHByaXZhdGUgcHJldmlld1dpbmRvd01lYXN1cmVzKCk6IHJlYWRvbmx5IFdpbmRvd01lYXN1cmVbXSB7XG5cdFx0cmV0dXJuIFtcblx0XHRcdHtcblx0XHRcdFx0a2luZDogXCJzZWxlY3RcIixcblx0XHRcdFx0aWQ6IGAke1BST0NFRFVSQUxfUExBWV9XSU5ET1dfS0lORF9QUkVWSUVXfS1zaG93YCxcblx0XHRcdFx0bGFiZWw6IFwiU2hvd1wiLFxuXHRcdFx0XHR2YWx1ZTogdGhpcy5zaG93TW9kZSxcblx0XHRcdFx0aXRlbXM6IFtcblx0XHRcdFx0XHR7IGlkOiBcImV2ZXJ5dGhpbmdcIiwgdmFsdWU6IFwiZXZlcnl0aGluZ1wiLCBsYWJlbDogXCJFdmVyeXRoaW5nXCIgfSxcblx0XHRcdFx0XHR7IGlkOiBcInNlbGVjdGVkXCIsIHZhbHVlOiBcInNlbGVjdGVkXCIsIGxhYmVsOiBcIlNlbGVjdGVkXCIgfSxcblx0XHRcdFx0XSxcblx0XHRcdFx0b25DaGFuZ2U6IHsgY29udHJvbGxlcklkOiBQUk9DRURVUkFMXzNEX1BMQVlfQ09OVFJPTExFUl9JRCwgY29tbWFuZDogXCJzZXRTaG93TW9kZVwiIH0sXG5cdFx0XHR9LFxuXHRcdFx0e1xuXHRcdFx0XHRraW5kOiBcInNlbGVjdFwiLFxuXHRcdFx0XHRpZDogYCR7UFJPQ0VEVVJBTF9QTEFZX1dJTkRPV19LSU5EX1BSRVZJRVd9LXRyYW5zZm9ybS1ncmFudWxhcml0eWAsXG5cdFx0XHRcdGxhYmVsOiBcIlRyYW5zZm9ybSBEZXRhaWxcIixcblx0XHRcdFx0dmFsdWU6IHRoaXMudHJhbnNmb3JtR3JhbnVsYXJpdHksXG5cdFx0XHRcdGl0ZW1zOiBbXG5cdFx0XHRcdFx0eyBpZDogXCJmdWxsXCIsIHZhbHVlOiBcImZ1bGxcIiwgbGFiZWw6IFwiRnVsbCAoc2xpZGVycyArIHZlY3RvcilcIiB9LFxuXHRcdFx0XHRcdHsgaWQ6IFwiY29tcGFjdFwiLCB2YWx1ZTogXCJjb21wYWN0XCIsIGxhYmVsOiBcIkNvbXBhY3QgKG5vZGUgcGFyYW1zKVwiIH0sXG5cdFx0XHRcdF0sXG5cdFx0XHRcdG9uQ2hhbmdlOiB7IGNvbnRyb2xsZXJJZDogUFJPQ0VEVVJBTF8zRF9QTEFZX0NPTlRST0xMRVJfSUQsIGNvbW1hbmQ6IFwic2V0VHJhbnNmb3JtR3JhbnVsYXJpdHlcIiB9LFxuXHRcdFx0fSxcblx0XHRdO1xuXHR9XG5cblx0LyoqIEBlbW9qaSDwn5SUIFN1YnNjcmliZXMgdG8gY2F0YWxvZ3VlIHVwZGF0ZXMgZm9yIHdvcmtiZW5jaCBraW5kcyBwYW5lbCByZWZyZXNoLiAqL1xuXHRzdWJzY3JpYmVTbmFwc2hvdChsaXN0ZW5lcjogKCkgPT4gdm9pZCk6ICgpID0+IHZvaWQge1xuXHRcdHRoaXMuc25hcHNob3RMaXN0ZW5lcnMuYWRkKGxpc3RlbmVyKTtcblx0XHRyZXR1cm4gKCkgPT4gdGhpcy5zbmFwc2hvdExpc3RlbmVycy5kZWxldGUobGlzdGVuZXIpO1xuXHR9XG5cblx0cHJpdmF0ZSBub3RpZnlTbmFwc2hvdCgpOiB2b2lkIHtcblx0XHRmb3IgKGNvbnN0IGxpc3RlbmVyIG9mIHRoaXMuc25hcHNob3RMaXN0ZW5lcnMpIHtcblx0XHRcdGxpc3RlbmVyKCk7XG5cdFx0fVxuXHR9XG5cblx0Z2V0UmVvcmdhbml6ZSgpOiBGbG93UmVvcmdhbml6ZVJlcXVlc3Qge1xuXHRcdHJldHVybiB7IGVwb2NoOiB0aGlzLnJlb3JnYW5pemVFcG9jaCwgb3B0aW9uc0pzb246IHRoaXMucmVvcmdhbml6ZU9wdGlvbnNKc29uIH07XG5cdH1cblxuXHRnZXRDb21tYW5kUmVxdWVzdCgpOiBGbG93Q2FudmFzQ29tbWFuZFJlcXVlc3Qge1xuXHRcdHJldHVybiB7IGVwb2NoOiB0aGlzLmNvbW1hbmRSZXF1ZXN0RXBvY2gsIC4uLnRoaXMuY29tbWFuZFJlcXVlc3RQYXlsb2FkIH07XG5cdH1cblxuXHRwcml2YXRlIHN5bmNSZW9yZ2FuaXplT3B0aW9uc0pzb24oKTogdm9pZCB7XG5cdFx0dGhpcy5yZW9yZ2FuaXplT3B0aW9uc0pzb24gPSBidWlsZFByb2NlZHVyYWxMYXlvdXRPcHRpb25zSnNvbih0aGlzLmxheWVyU3BhY2luZywgdGhpcy5zaWJsaW5nR2FwLCB0aGlzLm9yaWVudGF0aW9uKTtcblx0fVxuXG5cdHByaXZhdGUgdHJpZ2dlclJlb3JnYW5pemUoKTogdm9pZCB7XG5cdFx0dGhpcy5zeW5jUmVvcmdhbml6ZU9wdGlvbnNKc29uKCk7XG5cdFx0dGhpcy5yZW9yZ2FuaXplRXBvY2ggKz0gMTtcblx0XHR0aGlzLnJlYnVpbGRTaGVsbE1vZGUoKTtcblx0XHR0aGlzLmVtaXQoKTtcblx0fVxuXG5cdHByaXZhdGUgZmxvd1dpbmRvd0VuZ2FnZW1lbnQoKTogV2luZG93RW5nYWdlbWVudCB7XG5cdFx0cmV0dXJuIHtcblx0XHRcdHNlc3Npb25BY3RpdmU6IGZhbHNlLFxuXHRcdFx0aW5wdXQ6IHtcblx0XHRcdFx0aWQ6IFwiZW5nYWdlbWVudC1pbnB1dFwiLFxuXHRcdFx0XHR2YWx1ZTogdGhpcy5lbmdhZ2VtZW50SW5wdXQsXG5cdFx0XHRcdHBsYWNlaG9sZGVyOiBcIlJlb3JnYW5pemUsIGxyLCB0YlwiLFxuXHRcdFx0XHRvbkNoYW5nZTogcHJvY2VkdXJhbFBsYXlDbWQoXCJlbmdhZ2VtZW50SW5wdXRcIiksXG5cdFx0XHRcdG9uU3VibWl0OiBwcm9jZWR1cmFsUGxheUNtZChcImVuZ2FnZW1lbnRTdWJtaXRcIiksXG5cdFx0XHR9LFxuXHRcdFx0cG9zc2libGVFbmdhZ2VtZW50czogW1xuXHRcdFx0XHR7IGlkOiBcInByb2NlZHVyYWwudG9vbC5yZW9yZ2FuaXplXCIsIGxhYmVsOiBcIlJlb3JnYW5pemVcIiwgY29tbWFuZDogcHJvY2VkdXJhbFBsYXlDbWQoXCJyZW9yZ2FuaXplXCIpIH0sXG5cdFx0XHRcdHsgaWQ6IFwicHJvY2VkdXJhbC5sYXlvdXQubGVmdFJpZ2h0XCIsIGxhYmVsOiBcIkxlZnQgdG8gUmlnaHRcIiwgY29tbWFuZDogcHJvY2VkdXJhbFBsYXlDbWQoXCJzZXRPcmllbnRhdGlvblwiLCB7IG9yaWVudGF0aW9uOiBcImxlZnRSaWdodFwiIH0pIH0sXG5cdFx0XHRcdHsgaWQ6IFwicHJvY2VkdXJhbC5sYXlvdXQudG9wQm90dG9tXCIsIGxhYmVsOiBcIlRvcCB0byBCb3R0b21cIiwgY29tbWFuZDogcHJvY2VkdXJhbFBsYXlDbWQoXCJzZXRPcmllbnRhdGlvblwiLCB7IG9yaWVudGF0aW9uOiBcInRvcEJvdHRvbVwiIH0pIH0sXG5cdFx0XHRdLFxuXHRcdFx0Y29udHJvbHM6IFtcblx0XHRcdFx0e1xuXHRcdFx0XHRcdGtpbmQ6IFwic2xpZGVyXCIsXG5cdFx0XHRcdFx0aWQ6IFwicHJvY2VkdXJhbC1sYXllci1zcGFjaW5nXCIsXG5cdFx0XHRcdFx0bGFiZWw6IFwiTGF5ZXIgc3BhY2luZ1wiLFxuXHRcdFx0XHRcdHZhbHVlOiB0aGlzLmxheWVyU3BhY2luZyxcblx0XHRcdFx0XHRtaW46IDQwLFxuXHRcdFx0XHRcdG1heDogMzIwLFxuXHRcdFx0XHRcdHN0ZXA6IDEwLFxuXHRcdFx0XHRcdG9uQ2hhbmdlOiBwcm9jZWR1cmFsUGxheUNtZChcInNldFNwYWNpbmdcIiwgeyBmaWVsZDogXCJsYXllclNwYWNpbmdcIiB9KSxcblx0XHRcdFx0fSxcblx0XHRcdFx0e1xuXHRcdFx0XHRcdGtpbmQ6IFwic2xpZGVyXCIsXG5cdFx0XHRcdFx0aWQ6IFwicHJvY2VkdXJhbC1zaWJsaW5nLWdhcFwiLFxuXHRcdFx0XHRcdGxhYmVsOiBcIlNpYmxpbmcgZ2FwXCIsXG5cdFx0XHRcdFx0dmFsdWU6IHRoaXMuc2libGluZ0dhcCxcblx0XHRcdFx0XHRtaW46IDEwLFxuXHRcdFx0XHRcdG1heDogMTYwLFxuXHRcdFx0XHRcdHN0ZXA6IDUsXG5cdFx0XHRcdFx0b25DaGFuZ2U6IHByb2NlZHVyYWxQbGF5Q21kKFwic2V0U3BhY2luZ1wiLCB7IGZpZWxkOiBcInNpYmxpbmdHYXBcIiB9KSxcblx0XHRcdFx0fSxcblx0XHRcdF0sXG5cdFx0XHRzdGF0dXM6IFt7IGlkOiBcInByb2NlZHVyYWwtbGF5b3V0LW9yaWVudGF0aW9uXCIsIHRleHQ6IHRoaXMub3JpZW50YXRpb24gPT09IFwibGVmdFJpZ2h0XCIgPyBcIkxlZnQgdG8gcmlnaHRcIiA6IFwiVG9wIHRvIGJvdHRvbVwiIH1dLFxuXHRcdH07XG5cdH1cblxuXHRwcml2YXRlIHByZXZpZXdXaW5kb3dFbmdhZ2VtZW50KCk6IFdpbmRvd0VuZ2FnZW1lbnQge1xuXHRcdHJldHVybiB7XG5cdFx0XHRzZXNzaW9uQWN0aXZlOiBmYWxzZSxcblx0XHRcdGlucHV0OiB7XG5cdFx0XHRcdGlkOiBcInByZXZpZXctZW5nYWdlbWVudC1pbnB1dFwiLFxuXHRcdFx0XHR2YWx1ZTogXCJcIixcblx0XHRcdFx0cGxhY2Vob2xkZXI6IFwiUHJldmlld1wiLFxuXHRcdFx0XHRvbkNoYW5nZTogcHJvY2VkdXJhbFBsYXlDbWQoXCJwcmV2aWV3RW5nYWdlbWVudElucHV0XCIpLFxuXHRcdFx0XHRvblN1Ym1pdDogcHJvY2VkdXJhbFBsYXlDbWQoXCJwcmV2aWV3RW5nYWdlbWVudFN1Ym1pdFwiKSxcblx0XHRcdH0sXG5cdFx0XHRzdGF0dXM6IFt7IGlkOiBcInByb2NlZHVyYWwtcHJldmlldy1pdGVtLWNvdW50XCIsIHRleHQ6IGAke3RoaXMucHJldmlld0l0ZW1zLmxlbmd0aH0gcHJldmlldyBpdGVtc2AgfV0sXG5cdFx0fTtcblx0fVxuXG5cdHByaXZhdGUgcmVidWlsZFNoZWxsTW9kZSgpOiB2b2lkIHtcblx0XHR0aGlzLm1haW5Nb2RlLndpbmRvd0tpbmRzID0gW1xuXHRcdFx0bmV3IFdpbmRvd0tpbmRSdW50aW1lKFBST0NFRFVSQUxfUExBWV9XSU5ET1dfS0lORF9JRCwgXCJGbG93XCIsIFBST0NFRFVSQUxfUExBWV9CT0RZX0tFWV9NQUlOLCB1bmRlZmluZWQsIHRoaXMuZmxvd1dpbmRvd01lYXN1cmVzKCksIHRoaXMuZmxvd1dpbmRvd0VuZ2FnZW1lbnQoKSksXG5cdFx0XHRuZXcgV2luZG93S2luZFJ1bnRpbWUoXG5cdFx0XHRcdFBST0NFRFVSQUxfUExBWV9XSU5ET1dfS0lORF9QUkVWSUVXLFxuXHRcdFx0XHRcIlByZXZpZXdcIixcblx0XHRcdFx0UFJPQ0VEVVJBTF9QTEFZX0JPRFlfS0VZX1BSRVZJRVcsXG5cdFx0XHRcdHVuZGVmaW5lZCxcblx0XHRcdFx0dGhpcy5wcmV2aWV3V2luZG93TWVhc3VyZXMoKSxcblx0XHRcdFx0dGhpcy5wcmV2aWV3V2luZG93RW5nYWdlbWVudCgpLFxuXHRcdFx0KSxcblx0XHRdO1xuXHRcdGZvciAoY29uc3Qgd2luZG93S2luZCBvZiB0aGlzLm1haW5Nb2RlLndpbmRvd0tpbmRzKSB7XG5cdFx0XHRlbmZvcmNlUGxheWdyb3VuZFdpbmRvd0VuZ2FnZW1lbnRJbnB1dCh3aW5kb3dLaW5kLmVuZ2FnZW1lbnQsIGBQcm9jZWR1cmFsIHBsYXkgd2luZG93IFwiJHt3aW5kb3dLaW5kLmlkfVwiYCk7XG5cdFx0fVxuXHRcdHRoaXMucmVidWlsZFRvb2xiYXJUb29scygpO1xuXHR9XG5cblx0b3ZlcnJpZGUgcnVuKGNvbW1hbmQ6IHN0cmluZywgYXJncz86IHVua25vd24pOiB2b2lkIHtcblx0XHRpZiAoY29tbWFuZCA9PT0gXCJlbmdhZ2VtZW50SW5wdXRcIikge1xuXHRcdFx0Y29uc3QgdmFsdWUgPSAoYXJncyBhcyB7IHZhbHVlPzogc3RyaW5nIH0pLnZhbHVlO1xuXHRcdFx0aWYgKHR5cGVvZiB2YWx1ZSA9PT0gXCJzdHJpbmdcIiAmJiB2YWx1ZSAhPT0gdGhpcy5lbmdhZ2VtZW50SW5wdXQpIHtcblx0XHRcdFx0dGhpcy5lbmdhZ2VtZW50SW5wdXQgPSB2YWx1ZTtcblx0XHRcdFx0dGhpcy5yZWJ1aWxkU2hlbGxNb2RlKCk7XG5cdFx0XHRcdHRoaXMuZW1pdCgpO1xuXHRcdFx0fVxuXHRcdFx0cmV0dXJuO1xuXHRcdH1cblx0XHRpZiAoY29tbWFuZCA9PT0gXCJlbmdhZ2VtZW50U3VibWl0XCIpIHtcblx0XHRcdGNvbnN0IHZhbHVlID0gKGFyZ3MgYXMgeyB2YWx1ZT86IHN0cmluZyB9KS52YWx1ZSA/PyB0aGlzLmVuZ2FnZW1lbnRJbnB1dDtcblx0XHRcdHRoaXMuYXBwbHlFbmdhZ2VtZW50KHZhbHVlKTtcblx0XHRcdHJldHVybjtcblx0XHR9XG5cdFx0aWYgKGNvbW1hbmQgPT09IFwic2V0U3BhY2luZ1wiKSB7XG5cdFx0XHRjb25zdCBmaWVsZCA9IChhcmdzIGFzIHsgZmllbGQ/OiBzdHJpbmc7IHZhbHVlPzogbnVtYmVyIH0pLmZpZWxkO1xuXHRcdFx0Y29uc3QgdmFsdWUgPSAoYXJncyBhcyB7IHZhbHVlPzogbnVtYmVyIH0pLnZhbHVlO1xuXHRcdFx0aWYgKHR5cGVvZiB2YWx1ZSAhPT0gXCJudW1iZXJcIikgcmV0dXJuO1xuXHRcdFx0aWYgKGZpZWxkID09PSBcImxheWVyU3BhY2luZ1wiKSB0aGlzLmxheWVyU3BhY2luZyA9IHZhbHVlO1xuXHRcdFx0ZWxzZSBpZiAoZmllbGQgPT09IFwic2libGluZ0dhcFwiKSB0aGlzLnNpYmxpbmdHYXAgPSB2YWx1ZTtcblx0XHRcdGVsc2UgcmV0dXJuO1xuXHRcdFx0dGhpcy5zeW5jUmVvcmdhbml6ZU9wdGlvbnNKc29uKCk7XG5cdFx0XHR0aGlzLnJlYnVpbGRTaGVsbE1vZGUoKTtcblx0XHRcdHRoaXMuZW1pdCgpO1xuXHRcdFx0cmV0dXJuO1xuXHRcdH1cblx0XHRpZiAoY29tbWFuZCA9PT0gXCJzZXRPcmllbnRhdGlvblwiKSB7XG5cdFx0XHRjb25zdCBvcmllbnRhdGlvbiA9IChhcmdzIGFzIHsgb3JpZW50YXRpb24/OiBQcm9jZWR1cmFsTGF5b3V0T3JpZW50YXRpb24gfSkub3JpZW50YXRpb247XG5cdFx0XHRpZiAob3JpZW50YXRpb24gIT09IFwibGVmdFJpZ2h0XCIgJiYgb3JpZW50YXRpb24gIT09IFwidG9wQm90dG9tXCIpIHJldHVybjtcblx0XHRcdHRoaXMub3JpZW50YXRpb24gPSBvcmllbnRhdGlvbjtcblx0XHRcdHRoaXMuc3luY1Jlb3JnYW5pemVPcHRpb25zSnNvbigpO1xuXHRcdFx0dGhpcy5yZWJ1aWxkU2hlbGxNb2RlKCk7XG5cdFx0XHR0aGlzLmVtaXQoKTtcblx0XHRcdHJldHVybjtcblx0XHR9XG5cdFx0aWYgKGNvbW1hbmQgPT09IFwicmVvcmdhbml6ZVwiKSB7XG5cdFx0XHR0aGlzLnRyaWdnZXJSZW9yZ2FuaXplKCk7XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdGlmIChjb21tYW5kID09PSBcImNhbnZhc0NvbW1hbmRcIikge1xuXHRcdFx0Y29uc3QgY2FudmFzQ29tbWFuZCA9IChhcmdzIGFzIHsgY29tbWFuZD86IHN0cmluZzsgYXJnc0pzb24/OiBzdHJpbmcgfSkuY29tbWFuZDtcblx0XHRcdGlmICh0eXBlb2YgY2FudmFzQ29tbWFuZCAhPT0gXCJzdHJpbmdcIiB8fCAhY2FudmFzQ29tbWFuZCkgcmV0dXJuO1xuXHRcdFx0Y29uc3QgYXJnc0pzb24gPSAoYXJncyBhcyB7IGFyZ3NKc29uPzogc3RyaW5nIH0pLmFyZ3NKc29uO1xuXHRcdFx0dGhpcy5jb21tYW5kUmVxdWVzdFBheWxvYWQgPSB7IGNvbW1hbmQ6IGNhbnZhc0NvbW1hbmQsIC4uLihhcmdzSnNvbiAhPT0gdW5kZWZpbmVkID8geyBhcmdzSnNvbiB9IDoge30pIH07XG5cdFx0XHR0aGlzLmNvbW1hbmRSZXF1ZXN0RXBvY2ggKz0gMTtcblx0XHRcdHRoaXMuZW1pdCgpO1xuXHRcdFx0cmV0dXJuO1xuXHRcdH1cblx0XHRpZiAoY29tbWFuZCA9PT0gXCJzZXRGaXh0dXJlSnNvblwiKSB7XG5cdFx0XHRjb25zdCB7IGpzb24sIHJlc2V0SW50ZXJhY3Rpb24gfSA9IGFyZ3MgYXMgeyBqc29uPzogc3RyaW5nOyByZXNldEludGVyYWN0aW9uPzogYm9vbGVhbiB9O1xuXHRcdFx0aWYgKHR5cGVvZiBqc29uID09PSBcInN0cmluZ1wiKSB7XG5cdFx0XHRcdHRoaXMuYXBwbHlGaXh0dXJlSnNvbihqc29uLCByZXNldEludGVyYWN0aW9uID09PSB0cnVlKTtcblx0XHRcdH1cblx0XHRcdHJldHVybjtcblx0XHR9XG5cdFx0aWYgKGNvbW1hbmQgPT09IFwic2V0QWN0aXZlRml4dHVyZVwiKSB7XG5cdFx0XHRpZiAoaXNQbGF5Z3JvdW5kRml4dHVyZUxvY2tlZCgpKSByZXR1cm47XG5cdFx0XHRjb25zdCBmaXh0dXJlSWQgPSAoYXJncyBhcyB7IGZpeHR1cmVJZD86IHN0cmluZyB9KS5maXh0dXJlSWQgPz8gXCJcIjtcblx0XHRcdHRoaXMubG9hZEZpeHR1cmVCeUlkKGZpeHR1cmVJZCk7XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdGlmIChjb21tYW5kID09PSBcInNhdmVTdG9yZWRcIikge1xuXHRcdFx0dGhpcy5maXh0dXJlU3RvcmUuc2F2ZSh0aGlzLmZpeHR1cmVKc29uKTtcblx0XHRcdHRoaXMucmVidWlsZFNoZWxsTW9kZSgpO1xuXHRcdFx0dGhpcy5lbWl0KCk7XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdGlmIChjb21tYW5kID09PSBcInNhdmVEb3dubG9hZFwiIHx8IGNvbW1hbmQgPT09IFwibG9hZFJlcXVlc3RcIikge1xuXHRcdFx0dGhpcy5ob3N0QnJpZGdlPy5ydW5Ib3N0Q29tbWFuZChjb21tYW5kLCBhcmdzKTtcblx0XHRcdHJldHVybjtcblx0XHR9XG5cdFx0aWYgKGNvbW1hbmQgPT09IFwibG9hZFN0b3JlZFwiKSB7XG5cdFx0XHRjb25zdCBqc29uID0gdGhpcy5maXh0dXJlU3RvcmUubG9hZCgpO1xuXHRcdFx0aWYgKGpzb24pIHRoaXMuYXBwbHlGaXh0dXJlSnNvbihqc29uLCB0cnVlKTtcblx0XHRcdHJldHVybjtcblx0XHR9XG5cdFx0aWYgKGNvbW1hbmQgPT09IFwicmVzZXRGaXh0dXJlXCIpIHtcblx0XHRcdHRoaXMuZml4dHVyZVN0b3JlLmNsZWFyKCk7XG5cdFx0XHR0aGlzLmFjdGl2ZUZpeHR1cmVJZCA9IFBMQVlHUk9VTkRfTk9fRklYVFVSRV9JRDtcblx0XHRcdHRoaXMuYXBwbHlGaXh0dXJlSnNvbihQUk9DRURVUkFMX1BMQVlfRU1QVFlfRklYVFVSRV9KU09OLCB0cnVlKTtcblx0XHRcdHJldHVybjtcblx0XHR9XG5cdFx0aWYgKGNvbW1hbmQgPT09IFwic2V0TG9kTW9kZVwiKSB7XG5cdFx0XHRjb25zdCB7IHZhbHVlLCBpbnN0YW5jZUlkIH0gPSBhcmdzIGFzIHsgdmFsdWU/OiBzdHJpbmc7IGluc3RhbmNlSWQ/OiBzdHJpbmcgfTtcblx0XHRcdGNvbnN0IHNjb3BlSWQgPSBpbnN0YW5jZUlkID8/IFBST0NFRFVSQUxfUExBWV9XSU5ET1dfS0lORF9JRDtcblx0XHRcdGlmICh0eXBlb2YgdmFsdWUgIT09IFwic3RyaW5nXCIpIHJldHVybjtcblx0XHRcdGlmICh2YWx1ZSAhPT0gREFHX0xPRF9NT0RFX0FVVE9NQVRJQyAmJiAhaXNEYWdEcmF3TG9kS2luZCh2YWx1ZSkpIHJldHVybjtcblx0XHRcdHRoaXMubG9kTW9kZUJ5SW5zdGFuY2UgPSB7IC4uLnRoaXMubG9kTW9kZUJ5SW5zdGFuY2UsIFtzY29wZUlkXTogdmFsdWUgYXMgRGFnTG9kTW9kZUtpbmQgfTtcblx0XHRcdGlmIChzY29wZUlkID09PSBQUk9DRURVUkFMX1BMQVlfV0lORE9XX0tJTkRfSUQpIHtcblx0XHRcdFx0dGhpcy5sb2RNb2RlID0gdmFsdWUgYXMgRGFnTG9kTW9kZUtpbmQ7XG5cdFx0XHR9XG5cdFx0XHR0aGlzLnJlYnVpbGRTaGVsbE1vZGUoKTtcblx0XHRcdHRoaXMuZW1pdCgpO1xuXHRcdFx0cmV0dXJuO1xuXHRcdH1cblx0XHRpZiAoY29tbWFuZCA9PT0gXCJzZXRFZmZlY3RpdmVMb2RcIikge1xuXHRcdFx0Y29uc3QgeyBsb2QsIGluc3RhbmNlSWQgfSA9IGFyZ3MgYXMgeyBsb2Q/OiBEYWdEcmF3TG9kS2luZDsgaW5zdGFuY2VJZD86IHN0cmluZyB9O1xuXHRcdFx0Y29uc3Qgc2NvcGVJZCA9IGluc3RhbmNlSWQgPz8gUFJPQ0VEVVJBTF9QTEFZX1dJTkRPV19LSU5EX0lEO1xuXHRcdFx0aWYgKCFsb2QgfHwgIWlzRGFnRHJhd0xvZEtpbmQobG9kKSkgcmV0dXJuO1xuXHRcdFx0aWYgKHNjb3BlSWQgIT09IFBST0NFRFVSQUxfUExBWV9XSU5ET1dfS0lORF9JRCkgcmV0dXJuO1xuXHRcdFx0aWYgKHRoaXMuZWZmZWN0aXZlTG9kID09PSBsb2QpIHJldHVybjtcblx0XHRcdHRoaXMuZWZmZWN0aXZlTG9kID0gbG9kO1xuXHRcdFx0dGhpcy5yZWJ1aWxkU2hlbGxNb2RlKCk7XG5cdFx0XHR0aGlzLmVtaXQoKTtcblx0XHRcdHJldHVybjtcblx0XHR9XG5cdFx0aWYgKGNvbW1hbmQgPT09IFwic2V0UHJveGltaXR5RGlzdGFuY2VcIikge1xuXHRcdFx0Y29uc3QgdmFsdWUgPSAoYXJncyBhcyB7IHZhbHVlPzogbnVtYmVyIH0pLnZhbHVlO1xuXHRcdFx0aWYgKHR5cGVvZiB2YWx1ZSAhPT0gXCJudW1iZXJcIiB8fCAhTnVtYmVyLmlzRmluaXRlKHZhbHVlKSkgcmV0dXJuO1xuXHRcdFx0Y29uc3QgbmV4dCA9IE1hdGgubWF4KDAsIHZhbHVlKTtcblx0XHRcdGlmICh0aGlzLnByb3hpbWl0eURpc3RhbmNlID09PSBuZXh0KSByZXR1cm47XG5cdFx0XHR0aGlzLnByb3hpbWl0eURpc3RhbmNlID0gbmV4dDtcblx0XHRcdHRoaXMucmVidWlsZFNoZWxsTW9kZSgpO1xuXHRcdFx0dGhpcy5lbWl0KCk7XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdGlmIChjb21tYW5kID09PSBcInNldFByZXZpZXdUZXh0XCIpIHtcblx0XHRcdGNvbnN0IHRleHQgPSAoYXJncyBhcyB7IHRleHQ/OiBzdHJpbmcgfSkudGV4dDtcblx0XHRcdGlmICh0eXBlb2YgdGV4dCA9PT0gXCJzdHJpbmdcIiAmJiB0ZXh0ICE9PSB0aGlzLnByZXZpZXdUZXh0KSB7XG5cdFx0XHRcdHRoaXMucHJldmlld1RleHQgPSB0ZXh0O1xuXHRcdFx0XHR0aGlzLmVtaXQoKTtcblx0XHRcdH1cblx0XHRcdHJldHVybjtcblx0XHR9XG5cdFx0aWYgKGNvbW1hbmQgPT09IFwic2V0RXZhbE91dHB1dHNcIikge1xuXHRcdFx0Y29uc3Qgb3V0cHV0c0pzb24gPSAoYXJncyBhcyB7IG91dHB1dHNKc29uPzogc3RyaW5nIH0pLm91dHB1dHNKc29uO1xuXHRcdFx0Y29uc3QgcHJldmlld01lc2hlcyA9IChhcmdzIGFzIHsgcHJldmlld01lc2hlcz86IFJlYWRvbmx5PFJlY29yZDxzdHJpbmcsIHVua25vd24+PiB9KS5wcmV2aWV3TWVzaGVzO1xuXHRcdFx0aWYgKHR5cGVvZiBvdXRwdXRzSnNvbiA9PT0gXCJzdHJpbmdcIikge1xuXHRcdFx0XHRjb25zdCBuZXh0SXRlbXMgPSBwcmV2aWV3SXRlbXNXaXRoTWVzaGVzKFxuXHRcdFx0XHRcdGV4dHJhY3RDaGFubmVsUHJldmlld0l0ZW1zKG91dHB1dHNKc29uKSxcblx0XHRcdFx0XHRwcmV2aWV3TWVzaGVzLFxuXHRcdFx0XHRcdHRoaXMucHJldmlld0l0ZW1zLFxuXHRcdFx0XHQpO1xuXHRcdFx0XHR0aGlzLnByZXZpZXdJdGVtcyA9IG5leHRJdGVtcztcblx0XHRcdFx0dGhpcy5pbnRlcmFjdGlvblJldmlzaW9uICs9IDE7XG5cdFx0XHRcdHRoaXMubm90aWZ5U25hcHNob3QoKTtcblx0XHRcdFx0dGhpcy5yZWJ1aWxkU2hlbGxNb2RlKCk7XG5cdFx0XHRcdHRoaXMuZW1pdCgpO1xuXHRcdFx0fVxuXHRcdFx0cmV0dXJuO1xuXHRcdH1cblx0XHRpZiAoY29tbWFuZCA9PT0gXCJzZXRTZWxlY3Rpb25cIikge1xuXHRcdFx0Y29uc3QgaWRzID0gKGFyZ3MgYXMgeyBpZHM/OiBzdHJpbmdbXSB9KS5pZHM7XG5cdFx0XHRjb25zdCBtb2RlID0gKGFyZ3MgYXMgeyBtb2RlPzogUHJvY2VkdXJhbFBsYXlTZWxlY3Rpb25Nb2RlIH0pLm1vZGUgPz8gXCJkZWZhdWx0XCI7XG5cdFx0XHRjb25zdCBmcm9tRmxvdyA9IChhcmdzIGFzIHsgZnJvbUZsb3c/OiBib29sZWFuIH0pLmZyb21GbG93ID09PSB0cnVlO1xuXHRcdFx0aWYgKCFBcnJheS5pc0FycmF5KGlkcykpIHJldHVybjtcblx0XHRcdGlmIChmcm9tRmxvdyAmJiB0aGlzLmd1bWJhbGxEcmFnU2Vzc2lvbikgcmV0dXJuO1xuXHRcdFx0Y29uc3QgbmV4dCA9IHNlbGVjdGlvbk1lcmdlSWRzKG1vZGUsIHRoaXMuc2VsZWN0ZWROb2RlSWRzLCBpZHMpO1xuXHRcdFx0aWYgKEpTT04uc3RyaW5naWZ5KG5leHQpID09PSBKU09OLnN0cmluZ2lmeSh0aGlzLnNlbGVjdGVkTm9kZUlkcykpIHJldHVybjtcblx0XHRcdHRoaXMuc2VsZWN0ZWROb2RlSWRzID0gbmV4dDtcblx0XHRcdHRoaXMuc2VsZWN0ZWRDaGFubmVscyA9IFtdO1xuXHRcdFx0dGhpcy5wcmVzZWxlY3ROb2RlSWRzID0gW107XG5cdFx0XHR0aGlzLnByZXNlbGVjdFJlbW92ZWROb2RlSWRzID0gW107XG5cdFx0XHR0aGlzLmludGVyYWN0aW9uUmV2aXNpb24gKz0gMTtcblx0XHRcdHRoaXMubm90aWZ5U25hcHNob3QoKTtcblx0XHRcdHRoaXMucmVidWlsZFRvb2xiYXJUb29scygpO1xuXHRcdFx0dGhpcy5lbWl0KCk7XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdGlmIChjb21tYW5kID09PSBcInJlbmFtZUZsb3dXaWRnZXRcIikge1xuXHRcdFx0Y29uc3Qgb2xkSWQgPSAoYXJncyBhcyB7IG9sZElkPzogc3RyaW5nIH0pLm9sZElkO1xuXHRcdFx0Y29uc3QgdmFsdWUgPSAoYXJncyBhcyB7IHZhbHVlPzogc3RyaW5nIH0pLnZhbHVlO1xuXHRcdFx0aWYgKHR5cGVvZiBvbGRJZCA9PT0gXCJzdHJpbmdcIiAmJiB0eXBlb2YgdmFsdWUgPT09IFwic3RyaW5nXCIpIHtcblx0XHRcdFx0dGhpcy5yZW5hbWVGbG93V2lkZ2V0KG9sZElkLCB2YWx1ZSk7XG5cdFx0XHR9XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdGlmIChjb21tYW5kID09PSBcInBhdGNoRmxvd1dpZGdldFwiKSB7XG5cdFx0XHRjb25zdCB3aWRnZXRJZCA9IChhcmdzIGFzIHsgd2lkZ2V0SWQ/OiBzdHJpbmcgfSkud2lkZ2V0SWQ7XG5cdFx0XHRjb25zdCBmaWVsZCA9IChhcmdzIGFzIHsgZmllbGQ/OiBzdHJpbmcgfSkuZmllbGQ7XG5cdFx0XHRjb25zdCB2YWx1ZSA9IChhcmdzIGFzIHsgdmFsdWU/OiB1bmtub3duIH0pLnZhbHVlO1xuXHRcdFx0aWYgKHR5cGVvZiB3aWRnZXRJZCA9PT0gXCJzdHJpbmdcIiAmJiB0eXBlb2YgZmllbGQgPT09IFwic3RyaW5nXCIpIHtcblx0XHRcdFx0dGhpcy5wYXRjaEZsb3dXaWRnZXQod2lkZ2V0SWQsIGZpZWxkLCB2YWx1ZSk7XG5cdFx0XHR9XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdGlmIChjb21tYW5kID09PSBcInNldFByZXNlbGVjdFwiKSB7XG5cdFx0XHRjb25zdCBpZHMgPSAoYXJncyBhcyB7IGlkcz86IHN0cmluZ1tdIH0pLmlkcztcblx0XHRcdGNvbnN0IHJlbW92ZWRJZHMgPSAoYXJncyBhcyB7IHJlbW92ZWRJZHM/OiBzdHJpbmdbXSB9KS5yZW1vdmVkSWRzO1xuXHRcdFx0aWYgKCFBcnJheS5pc0FycmF5KGlkcykgfHwgIUFycmF5LmlzQXJyYXkocmVtb3ZlZElkcykpIHJldHVybjtcblx0XHRcdHRoaXMucHJlc2VsZWN0Tm9kZUlkcyA9IFsuLi5pZHNdO1xuXHRcdFx0dGhpcy5wcmVzZWxlY3RSZW1vdmVkTm9kZUlkcyA9IFsuLi5yZW1vdmVkSWRzXTtcblx0XHRcdHRoaXMuaW50ZXJhY3Rpb25SZXZpc2lvbiArPSAxO1xuXHRcdFx0dGhpcy5ub3RpZnlTbmFwc2hvdCgpO1xuXHRcdFx0cmV0dXJuO1xuXHRcdH1cblx0XHRpZiAoY29tbWFuZCA9PT0gXCJzZXRTZWxlY3Rpb25Nb2RlXCIpIHtcblx0XHRcdGNvbnN0IG1vZGUgPSAoYXJncyBhcyB7IG1vZGU/OiBQcm9jZWR1cmFsUGxheVNlbGVjdGlvbk1vZGUgfSkubW9kZTtcblx0XHRcdGlmIChtb2RlICE9PSBcImRlZmF1bHRcIiAmJiBtb2RlICE9PSBcImFkZGl0aXZlXCIgJiYgbW9kZSAhPT0gXCJzdWJ0cmFjdGl2ZVwiICYmIG1vZGUgIT09IFwiaW52ZXJ0aXZlXCIpIHJldHVybjtcblx0XHRcdGlmICh0aGlzLnNlbGVjdGlvbk1vZGUgPT09IG1vZGUpIHJldHVybjtcblx0XHRcdHRoaXMuc2VsZWN0aW9uTW9kZSA9IG1vZGU7XG5cdFx0XHR0aGlzLnJlYnVpbGRTaGVsbE1vZGUoKTtcblx0XHRcdHRoaXMuZW1pdCgpO1xuXHRcdFx0cmV0dXJuO1xuXHRcdH1cblx0XHRpZiAoY29tbWFuZCA9PT0gXCJzZXRTZWxlY3Rpb25NZXRob2RcIikge1xuXHRcdFx0Y29uc3QgbWV0aG9kID0gKGFyZ3MgYXMgeyBtZXRob2Q/OiBQcm9jZWR1cmFsUGxheVNlbGVjdGlvbk1ldGhvZCB9KS5tZXRob2Q7XG5cdFx0XHRpZiAobWV0aG9kICE9PSBcInJlY3RhbmdsZVwiICYmIG1ldGhvZCAhPT0gXCJsYXNzb1wiKSByZXR1cm47XG5cdFx0XHRpZiAodGhpcy5zZWxlY3Rpb25NZXRob2QgPT09IG1ldGhvZCkgcmV0dXJuO1xuXHRcdFx0dGhpcy5zZWxlY3Rpb25NZXRob2QgPSBtZXRob2Q7XG5cdFx0XHR0aGlzLnJlYnVpbGRTaGVsbE1vZGUoKTtcblx0XHRcdHRoaXMuZW1pdCgpO1xuXHRcdFx0cmV0dXJuO1xuXHRcdH1cblx0XHRpZiAoY29tbWFuZCA9PT0gXCJzZWxlY3RBbGxcIikge1xuXHRcdFx0Y29uc3QgaWRzID0gWy4uLm5ldyBTZXQodGhpcy5wcmV2aWV3SXRlbXMubWFwKChlbnRyeSkgPT4gZW50cnkud2lkZ2V0SWQpKV07XG5cdFx0XHR0aGlzLnNlbGVjdGVkTm9kZUlkcyA9IFsuLi5uZXcgU2V0KGlkcyldO1xuXHRcdFx0dGhpcy5wcmVzZWxlY3ROb2RlSWRzID0gW107XG5cdFx0XHR0aGlzLnByZXNlbGVjdFJlbW92ZWROb2RlSWRzID0gW107XG5cdFx0XHR0aGlzLmludGVyYWN0aW9uUmV2aXNpb24gKz0gMTtcblx0XHRcdHRoaXMubm90aWZ5U25hcHNob3QoKTtcblx0XHRcdHRoaXMuZW1pdCgpO1xuXHRcdFx0cmV0dXJuO1xuXHRcdH1cblx0XHRpZiAoY29tbWFuZCA9PT0gXCJjbGVhclNlbGVjdGlvblwiKSB7XG5cdFx0XHRpZiAoIXRoaXMuc2VsZWN0ZWROb2RlSWRzLmxlbmd0aCkgcmV0dXJuO1xuXHRcdFx0dGhpcy5zZWxlY3RlZE5vZGVJZHMgPSBbXTtcblx0XHRcdHRoaXMucHJlc2VsZWN0Tm9kZUlkcyA9IFtdO1xuXHRcdFx0dGhpcy5wcmVzZWxlY3RSZW1vdmVkTm9kZUlkcyA9IFtdO1xuXHRcdFx0dGhpcy5pbnRlcmFjdGlvblJldmlzaW9uICs9IDE7XG5cdFx0XHR0aGlzLm5vdGlmeVNuYXBzaG90KCk7XG5cdFx0XHR0aGlzLnJlYnVpbGRUb29sYmFyVG9vbHMoKTtcblx0XHRcdHRoaXMuZW1pdCgpO1xuXHRcdFx0cmV0dXJuO1xuXHRcdH1cblx0XHRpZiAoY29tbWFuZCA9PT0gXCJkZWxldGVTZWxlY3Rpb25cIikge1xuXHRcdFx0dGhpcy5ydW4oXCJjYW52YXNDb21tYW5kXCIsIHsgY29tbWFuZDogXCJkZWxldGVTZWxlY3Rpb25cIiB9KTtcblx0XHRcdHJldHVybjtcblx0XHR9XG5cdFx0aWYgKGNvbW1hbmQgPT09IFwic2V0SG92ZXJcIikge1xuXHRcdFx0Y29uc3QgaWQgPSAoYXJncyBhcyB7IGlkPzogc3RyaW5nIHwgbnVsbCB9KS5pZDtcblx0XHRcdGNvbnN0IGNoYW5uZWwgPSAoYXJncyBhcyB7IGNoYW5uZWw/OiBQcm9jZWR1cmFsQ2hhbm5lbFJlZiB8IG51bGwgfSkuY2hhbm5lbCA/PyBudWxsO1xuXHRcdFx0Y29uc3QgbmV4dCA9IHR5cGVvZiBpZCA9PT0gXCJzdHJpbmdcIiA/IGlkIDogbnVsbDtcblx0XHRcdGNvbnN0IGNoYW5uZWxKc29uID0gY2hhbm5lbCA/IEpTT04uc3RyaW5naWZ5KGNoYW5uZWwpIDogXCJudWxsXCI7XG5cdFx0XHRjb25zdCBjdXJyZW50Q2hhbm5lbEpzb24gPSB0aGlzLmhvdmVyZWRDaGFubmVsID8gSlNPTi5zdHJpbmdpZnkodGhpcy5ob3ZlcmVkQ2hhbm5lbCkgOiBcIm51bGxcIjtcblx0XHRcdGlmIChuZXh0ID09PSB0aGlzLmhvdmVyZWROb2RlSWQgJiYgY2hhbm5lbEpzb24gPT09IGN1cnJlbnRDaGFubmVsSnNvbikgcmV0dXJuO1xuXHRcdFx0dGhpcy5ob3ZlcmVkTm9kZUlkID0gbmV4dDtcblx0XHRcdHRoaXMuaG92ZXJlZENoYW5uZWwgPSBjaGFubmVsO1xuXHRcdFx0dGhpcy5pbnRlcmFjdGlvblJldmlzaW9uICs9IDE7XG5cdFx0XHR0aGlzLm5vdGlmeVNuYXBzaG90KCk7XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdGlmIChjb21tYW5kID09PSBcInNldFNlbGVjdGVkQ2hhbm5lbHNcIiB8fCBjb21tYW5kID09PSBcInNldFNlbGVjdENoYW5uZWxzXCIpIHtcblx0XHRcdGNvbnN0IGNoYW5uZWxzID0gKGFyZ3MgYXMgeyBjaGFubmVscz86IFByb2NlZHVyYWxDaGFubmVsUmVmW10gfSkuY2hhbm5lbHM7XG5cdFx0XHRpZiAoIUFycmF5LmlzQXJyYXkoY2hhbm5lbHMpKSByZXR1cm47XG5cdFx0XHRjb25zdCBuZXh0ID0gWy4uLmNoYW5uZWxzXTtcblx0XHRcdGlmIChKU09OLnN0cmluZ2lmeShuZXh0KSA9PT0gSlNPTi5zdHJpbmdpZnkodGhpcy5zZWxlY3RlZENoYW5uZWxzKSkgcmV0dXJuO1xuXHRcdFx0dGhpcy5zZWxlY3RlZENoYW5uZWxzID0gbmV4dDtcblx0XHRcdHRoaXMuc2VsZWN0ZWROb2RlSWRzID0gWy4uLm5ldyBTZXQobmV4dC5tYXAoKGNoYW5uZWwpID0+IGNoYW5uZWwud2lkZ2V0SWQpKV07XG5cdFx0XHR0aGlzLnByZXNlbGVjdE5vZGVJZHMgPSBbXTtcblx0XHRcdHRoaXMucHJlc2VsZWN0UmVtb3ZlZE5vZGVJZHMgPSBbXTtcblx0XHRcdHRoaXMuaW50ZXJhY3Rpb25SZXZpc2lvbiArPSAxO1xuXHRcdFx0dGhpcy5ub3RpZnlTbmFwc2hvdCgpO1xuXHRcdFx0dGhpcy5yZWJ1aWxkVG9vbGJhclRvb2xzKCk7XG5cdFx0XHR0aGlzLmVtaXQoKTtcblx0XHRcdHJldHVybjtcblx0XHR9XG5cdFx0aWYgKGNvbW1hbmQgPT09IFwic2V0SG92ZXJDaGFubmVsXCIpIHtcblx0XHRcdGNvbnN0IGNoYW5uZWwgPSAoYXJncyBhcyB7IGNoYW5uZWw/OiBQcm9jZWR1cmFsQ2hhbm5lbFJlZiB8IG51bGwgfSkuY2hhbm5lbCA/PyBudWxsO1xuXHRcdFx0dGhpcy5ydW4oXCJzZXRIb3ZlclwiLCB7IGlkOiBjaGFubmVsPy53aWRnZXRJZCA/PyBudWxsLCBjaGFubmVsIH0pO1xuXHRcdFx0cmV0dXJuO1xuXHRcdH1cblx0XHRpZiAoY29tbWFuZCA9PT0gXCJ0b2dnbGVQcmV2aWV3XCIpIHtcblx0XHRcdGNvbnN0IGlkID0gKGFyZ3MgYXMgeyBpZD86IHN0cmluZyB9KS5pZDtcblx0XHRcdGlmICh0eXBlb2YgaWQgIT09IFwic3RyaW5nXCIpIHJldHVybjtcblx0XHRcdGNvbnN0IG9mZiA9IG5ldyBTZXQodGhpcy5wcmV2aWV3T2ZmTm9kZUlkcyk7XG5cdFx0XHRpZiAob2ZmLmhhcyhpZCkpIG9mZi5kZWxldGUoaWQpO1xuXHRcdFx0ZWxzZSBvZmYuYWRkKGlkKTtcblx0XHRcdHRoaXMucHJldmlld09mZk5vZGVJZHMgPSBbLi4ub2ZmXTtcblx0XHRcdHRoaXMuaW50ZXJhY3Rpb25SZXZpc2lvbiArPSAxO1xuXHRcdFx0dGhpcy5ub3RpZnlTbmFwc2hvdCgpO1xuXHRcdFx0dGhpcy5lbWl0KCk7XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdGlmIChjb21tYW5kID09PSBcInNldFByZXZpZXdPZmZcIikge1xuXHRcdFx0Y29uc3QgaWRzID0gKGFyZ3MgYXMgeyBpZHM/OiBzdHJpbmdbXSB9KS5pZHM7XG5cdFx0XHRjb25zdCBmcm9tRmxvdyA9IChhcmdzIGFzIHsgZnJvbUZsb3c/OiBib29sZWFuIH0pLmZyb21GbG93ID09PSB0cnVlO1xuXHRcdFx0aWYgKCFBcnJheS5pc0FycmF5KGlkcykpIHJldHVybjtcblx0XHRcdGlmIChmcm9tRmxvdyAmJiB0aGlzLmd1bWJhbGxEcmFnU2Vzc2lvbikge1xuXHRcdFx0XHRjb25zdCBuZXh0ID0gWy4uLmlkc107XG5cdFx0XHRcdGlmIChKU09OLnN0cmluZ2lmeShuZXh0KSA9PT0gSlNPTi5zdHJpbmdpZnkodGhpcy5wcmV2aWV3T2ZmTm9kZUlkcykpIHJldHVybjtcblx0XHRcdFx0dGhpcy5wcmV2aWV3T2ZmTm9kZUlkcyA9IG5leHQ7XG5cdFx0XHRcdHRoaXMuaW50ZXJhY3Rpb25SZXZpc2lvbiArPSAxO1xuXHRcdFx0XHR0aGlzLm5vdGlmeVNuYXBzaG90KCk7XG5cdFx0XHRcdHJldHVybjtcblx0XHRcdH1cblx0XHRcdHRoaXMucHJldmlld09mZk5vZGVJZHMgPSBbLi4uaWRzXTtcblx0XHRcdHRoaXMuaW50ZXJhY3Rpb25SZXZpc2lvbiArPSAxO1xuXHRcdFx0dGhpcy5ub3RpZnlTbmFwc2hvdCgpO1xuXHRcdFx0dGhpcy5lbWl0KCk7XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdGlmIChjb21tYW5kID09PSBcInNldFNob3dNb2RlXCIpIHtcblx0XHRcdGNvbnN0IGlkID0gKGFyZ3MgYXMgeyBpZD86IHN0cmluZyB9KS5pZCA/PyAoYXJncyBhcyB7IHZhbHVlPzogc3RyaW5nIH0pLnZhbHVlO1xuXHRcdFx0aWYgKGlkICE9PSBcImV2ZXJ5dGhpbmdcIiAmJiBpZCAhPT0gXCJzZWxlY3RlZFwiKSByZXR1cm47XG5cdFx0XHRpZiAodGhpcy5zaG93TW9kZSA9PT0gaWQpIHJldHVybjtcblx0XHRcdHRoaXMuc2hvd01vZGUgPSBpZDtcblx0XHRcdHRoaXMuaW50ZXJhY3Rpb25SZXZpc2lvbiArPSAxO1xuXHRcdFx0dGhpcy5yZWJ1aWxkU2hlbGxNb2RlKCk7XG5cdFx0XHR0aGlzLmVtaXQoKTtcblx0XHRcdHJldHVybjtcblx0XHR9XG5cdFx0aWYgKGNvbW1hbmQgPT09IFwic2V0VHJhbnNmb3JtR3JhbnVsYXJpdHlcIikge1xuXHRcdFx0Y29uc3QgZ3JhbnVsYXJpdHkgPVxuXHRcdFx0XHQoYXJncyBhcyB7IGdyYW51bGFyaXR5PzogUHJvY2VkdXJhbFRyYW5zZm9ybUdyYW51bGFyaXR5IH0pLmdyYW51bGFyaXR5ID8/XG5cdFx0XHRcdChhcmdzIGFzIHsgdmFsdWU/OiBzdHJpbmcgfSkudmFsdWU7XG5cdFx0XHRpZiAoZ3JhbnVsYXJpdHkgIT09IFwiY29tcGFjdFwiICYmIGdyYW51bGFyaXR5ICE9PSBcImZ1bGxcIikgcmV0dXJuO1xuXHRcdFx0aWYgKHRoaXMudHJhbnNmb3JtR3JhbnVsYXJpdHkgPT09IGdyYW51bGFyaXR5KSByZXR1cm47XG5cdFx0XHR0aGlzLnRyYW5zZm9ybUdyYW51bGFyaXR5ID0gZ3JhbnVsYXJpdHk7XG5cdFx0XHR0aGlzLnJlYnVpbGRTaGVsbE1vZGUoKTtcblx0XHRcdHRoaXMuZW1pdCgpO1xuXHRcdFx0cmV0dXJuO1xuXHRcdH1cblx0XHRpZiAoY29tbWFuZCA9PT0gXCJhcHBseUd1bWJhbGxUcmFuc2Zvcm1cIikge1xuXHRcdFx0Y29uc3Qgd2lkZ2V0SWQgPSAoYXJncyBhcyB7IHdpZGdldElkPzogc3RyaW5nIH0pLndpZGdldElkO1xuXHRcdFx0Y29uc3QgZGVsdGEgPSAoYXJncyBhcyB7IGRlbHRhPzogUHJvY2VkdXJhbEd1bWJhbGxUcmFuc2Zvcm1EZWx0YSB9KS5kZWx0YTtcblx0XHRcdGNvbnN0IGdyYW51bGFyaXR5ID0gKGFyZ3MgYXMgeyBncmFudWxhcml0eT86IFByb2NlZHVyYWxUcmFuc2Zvcm1HcmFudWxhcml0eSB9KS5ncmFudWxhcml0eSA/PyB0aGlzLnRyYW5zZm9ybUdyYW51bGFyaXR5O1xuXHRcdFx0Y29uc3QgcGhhc2UgPSAoYXJncyBhcyB7IHBoYXNlPzogUHJvY2VkdXJhbEd1bWJhbGxUcmFuc2Zvcm1QaGFzZSB9KS5waGFzZTtcblx0XHRcdGlmICh0eXBlb2Ygd2lkZ2V0SWQgIT09IFwic3RyaW5nXCIgfHwgIWRlbHRhKSByZXR1cm47XG5cdFx0XHR0aGlzLmFwcGx5R3VtYmFsbFRyYW5zZm9ybSh7IHdpZGdldElkLCBkZWx0YSwgZ3JhbnVsYXJpdHksIHBoYXNlIH0pO1xuXHRcdFx0cmV0dXJuO1xuXHRcdH1cblx0XHRpZiAoY29tbWFuZCA9PT0gXCJzZXRDYXRhbG9ndWVTZWN0aW9uc1wiKSB7XG5cdFx0XHRjb25zdCBzZWN0aW9ucyA9IChhcmdzIGFzIHsgc2VjdGlvbnM/OiBDYXRhbG9ndWVTZWN0aW9uW10gfSkuc2VjdGlvbnM7XG5cdFx0XHRpZiAoQXJyYXkuaXNBcnJheShzZWN0aW9ucykpIHtcblx0XHRcdFx0dGhpcy5jYXRhbG9ndWVTZWN0aW9ucyA9IHNlY3Rpb25zO1xuXHRcdFx0XHR0aGlzLmNhdGFsb2d1ZVJldmlzaW9uICs9IDE7XG5cdFx0XHRcdHRoaXMubm90aWZ5U25hcHNob3QoKTtcblx0XHRcdFx0dGhpcy5lbWl0KCk7XG5cdFx0XHR9XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdGlmIChjb21tYW5kID09PSBcInRvZ2dsZUV4dGVuc2lvblwiKSB7XG5cdFx0XHRjb25zdCBpZCA9IChhcmdzIGFzIHsgaWQ/OiBzdHJpbmcgfSkuaWQ7XG5cdFx0XHRjb25zdCBlbmFibGVkID0gKGFyZ3MgYXMgeyBlbmFibGVkPzogYm9vbGVhbiB9KS5lbmFibGVkO1xuXHRcdFx0aWYgKHR5cGVvZiBpZCAhPT0gXCJzdHJpbmdcIiB8fCB0eXBlb2YgZW5hYmxlZCAhPT0gXCJib29sZWFuXCIpIHJldHVybjtcblx0XHRcdHZvaWQgcHJvY2VkdXJhbEV4dGVuc2lvbkhvc3Quc2V0QWN0aXZlKGlkLCBlbmFibGVkKS50aGVuKCgpID0+IHtcblx0XHRcdFx0dGhpcy5leHRlbnNpb25SZXZpc2lvbiArPSAxO1xuXHRcdFx0XHR0aGlzLm5vdGlmeVNuYXBzaG90KCk7XG5cdFx0XHRcdHRoaXMuZW1pdCgpO1xuXHRcdFx0fSk7XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdGlmIChjb21tYW5kID09PSBcInJ1bkV4dGVuc2lvbkNvbW1hbmRcIikge1xuXHRcdFx0Y29uc3QgY29tbWFuZElkID0gKGFyZ3MgYXMgeyBjb21tYW5kSWQ/OiBzdHJpbmcgfSkuY29tbWFuZElkO1xuXHRcdFx0aWYgKHR5cGVvZiBjb21tYW5kSWQgIT09IFwic3RyaW5nXCIpIHJldHVybjtcblx0XHRcdGNvbnN0IHJlc3VsdCA9IHByb2NlZHVyYWxFeHRlbnNpb25Ib3N0LmV4ZWN1dGVDb21tYW5kKGNvbW1hbmRJZCk7XG5cdFx0XHRjb25zb2xlLmxvZyhgW0RFQlVHXSBwcm9jZWR1cmFsIGV4dGVuc2lvbiBjb21tYW5kICR7Y29tbWFuZElkfTogJHtyZXN1bHR9YCk7XG5cdFx0XHR0aGlzLmVtaXQoKTtcblx0XHRcdHJldHVybjtcblx0XHR9XG5cdH1cblxuXHRwcml2YXRlIGFwcGx5RW5nYWdlbWVudCh2YWx1ZTogc3RyaW5nKTogdm9pZCB7XG5cdFx0Y29uc3QgdHJpbW1lZCA9IHZhbHVlLnRyaW0oKS50b0xvd2VyQ2FzZSgpO1xuXHRcdGlmICghdHJpbW1lZCkgcmV0dXJuO1xuXHRcdGlmICh0cmltbWVkID09PSBcInJlb3JnYW5pemVcIiB8fCB0cmltbWVkID09PSBcImxheW91dFwiKSB7XG5cdFx0XHR0aGlzLnRyaWdnZXJSZW9yZ2FuaXplKCk7XG5cdFx0XHRyZXR1cm47XG5cdFx0fVxuXHRcdGlmICh0cmltbWVkID09PSBcImxyXCIgfHwgdHJpbW1lZCA9PT0gXCJsZWZ0XCIgfHwgdHJpbW1lZCA9PT0gXCJsZWZ0IHRvIHJpZ2h0XCIpIHtcblx0XHRcdHRoaXMub3JpZW50YXRpb24gPSBcImxlZnRSaWdodFwiO1xuXHRcdFx0dGhpcy5zeW5jUmVvcmdhbml6ZU9wdGlvbnNKc29uKCk7XG5cdFx0XHR0aGlzLnJlYnVpbGRTaGVsbE1vZGUoKTtcblx0XHRcdHRoaXMuZW1pdCgpO1xuXHRcdFx0cmV0dXJuO1xuXHRcdH1cblx0XHRpZiAodHJpbW1lZCA9PT0gXCJ0YlwiIHx8IHRyaW1tZWQgPT09IFwidG9wXCIgfHwgdHJpbW1lZCA9PT0gXCJ0b3AgdG8gYm90dG9tXCIpIHtcblx0XHRcdHRoaXMub3JpZW50YXRpb24gPSBcInRvcEJvdHRvbVwiO1xuXHRcdFx0dGhpcy5zeW5jUmVvcmdhbml6ZU9wdGlvbnNKc29uKCk7XG5cdFx0XHR0aGlzLnJlYnVpbGRTaGVsbE1vZGUoKTtcblx0XHRcdHRoaXMuZW1pdCgpO1xuXHRcdFx0cmV0dXJuO1xuXHRcdH1cblx0XHR0aGlzLmVuZ2FnZW1lbnRJbnB1dCA9IFwiXCI7XG5cdFx0dGhpcy5yZWJ1aWxkU2hlbGxNb2RlKCk7XG5cdFx0dGhpcy5lbWl0KCk7XG5cdH1cblxufVxuXG5leHBvcnQgZnVuY3Rpb24gcmVnaXN0ZXJQcm9jZWR1cmFsUGxheURlY2xhcmF0aXZlQm9kaWVzKCk6IHZvaWQge1xuXHRyZWdpc3RlcldpbmRvd0JvZHkoUFJPQ0VEVVJBTF9QTEFZX0JPRFlfS0VZX01BSU4sIChfY3R4OiBXaW5kb3dCb2R5Vmlld0NvbnRleHQpID0+XG5cdFx0YnVpbGRGbG93V2luZG93Qm9keShQUk9DRURVUkFMX1BMQVlfU1VSRkFDRV9JRCwgUFJPQ0VEVVJBTF8zRF9QTEFZX0NPTlRST0xMRVJfSUQsIFBST0NFRFVSQUxfUExBWV9XSU5ET1dfS0lORF9JRCkpO1xuXHRyZWdpc3RlcldpbmRvd0JvZHkoUFJPQ0VEVVJBTF9QTEFZX0JPRFlfS0VZX1BSRVZJRVcsIChfY3R4OiBXaW5kb3dCb2R5Vmlld0NvbnRleHQpID0+XG5cdFx0YnVpbGRQdXp6bGUzZFdpbmRvd0JvZHkoUFJPQ0VEVVJBTF9QTEFZX1NVUkZBQ0VfSURfUFJFVklFVywgUFJPQ0VEVVJBTF8zRF9QTEFZX0NPTlRST0xMRVJfSUQpKTtcbn1cblxuZXhwb3J0IGZ1bmN0aW9uIGJ1aWxkUHJvY2VkdXJhbFBsYXlBcHBSdW50aW1lKGNvbnRyb2xsZXI6IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcik6IEFwcFJ1bnRpbWUge1xuXHRyZXR1cm4gY3JlYXRlUGxheUFwcFJ1bnRpbWUoUFJPQ0VEVVJBTF8zRF9QTEFZX0FQUF9JRCwgXCJzZW1pbyDCtyBwcm9jZWR1cmFsXCIsIGNvbnRyb2xsZXIsIFBST0NFRFVSQUxfUExBWV9MQVlPVVQsIGNvbnRyb2xsZXIubWFpbk1vZGUpO1xufVxuXG4vKiogQGVtb2ppIPCfm50gUHJvY2VkdXJhbCBwbGF5Z3JvdW5kIGFwcC4gKi9cbmV4cG9ydCBjbGFzcyBQbGF5Z3JvdW5kUHJvY2VkdXJhbCBleHRlbmRzIFBsYXlncm91bmQge1xuXHRyZWFkb25seSBpZCA9IFBST0NFRFVSQUxfM0RfUExBWV9BUFBfSUQ7XG5cdHJlYWRvbmx5IGtleWJpbmRpbmdzID0gW1xuXHRcdHsga2V5OiBcImN0cmwrYSxtZXRhK2FcIiwgY29udHJvbGxlcklkOiBQUk9DRURVUkFMXzNEX1BMQVlfQ09OVFJPTExFUl9JRCwgY29tbWFuZDogXCJzZWxlY3RBbGxcIiB9LFxuXHRcdHsga2V5OiBcIkRlbGV0ZVwiLCBjb250cm9sbGVySWQ6IFBST0NFRFVSQUxfM0RfUExBWV9DT05UUk9MTEVSX0lELCBjb21tYW5kOiBcImRlbGV0ZVNlbGVjdGlvblwiIH0sXG5cdFx0eyBrZXk6IFwiQmFja3NwYWNlXCIsIGNvbnRyb2xsZXJJZDogUFJPQ0VEVVJBTF8zRF9QTEFZX0NPTlRST0xMRVJfSUQsIGNvbW1hbmQ6IFwiZGVsZXRlU2VsZWN0aW9uXCIgfSxcblx0XTtcblxuXHRjcmVhdGVSdW50aW1lKCk6IFBsYXRmb3JtIHtcblx0XHRjb25zdCBydW50aW1lID0gY3JlYXRlUHJvZHVjdFBsYXlncm91bmRQbGF0Zm9ybSh0aGlzLmlkKTtcblx0XHRjb25zdCBjdHJsID0gbmV3IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcihydW50aW1lLmNvbW1hbmRCdXMsICgpID0+IHJ1bnRpbWUubm90aWZ5KCkpO1xuXHRcdHJ1bnRpbWUuYWRkQXBwKGJ1aWxkUHJvY2VkdXJhbFBsYXlBcHBSdW50aW1lKGN0cmwpKTtcblx0XHRyZXR1cm4gcnVudGltZTtcblx0fVxuXG5cdHJlZ2lzdGVyQm9kaWVzKCk6IHZvaWQge1xuXHRcdHJlZ2lzdGVyUHJvY2VkdXJhbFBsYXlEZWNsYXJhdGl2ZUJvZGllcygpO1xuXHR9XG59XG5cbi8vICNyZWdpb24g8J+nqlRlc3RzXG5pZiAoaW1wb3J0Lm1ldGEudml0ZXN0KSB7XG5cdGNvbnN0IHsgZGVzY3JpYmUsIGV4cGVjdCwgaXQgfSA9IGltcG9ydC5tZXRhLnZpdGVzdDtcblxuXHRkZXNjcmliZShcIkBzZW1pby10ZWNoL3Byb2NlZHVyYWwtM2QtcGxheVwiLCAoKSA9PiB7XG5cdFx0aXQoXCJleHBvcnRzIGRlZmF1bHQgZml4dHVyZSBqc29uXCIsICgpID0+IHtcblx0XHRcdGV4cGVjdChQUk9DRURVUkFMX1BMQVlfREVGQVVMVF9GSVhUVVJFX0pTT04pLnRvQ29udGFpbihcImZsb3cuZml4dHVyZS92MVwiKTtcblx0XHR9KTtcblxuXHRcdGl0KFwic3RhcnRzIHdpdGggbm8gZml4dHVyZSBzZWxlY3RlZFwiLCAoKSA9PiB7XG5cdFx0XHRjb25zdCBidXMgPSBuZXcgQ29tbWFuZEJ1cygpO1xuXHRcdFx0Y29uc3QgY3RybCA9IG5ldyBQcm9jZWR1cmFsUGxheUNvbnRyb2xsZXIoYnVzLCAoKSA9PiB7fSk7XG5cdFx0XHRleHBlY3QoY3RybC5nZXRGaXh0dXJlQ2F0YWxvZygpLmFjdGl2ZUZpeHR1cmVJZCkudG9CZShQTEFZR1JPVU5EX05PX0ZJWFRVUkVfSUQpO1xuXHRcdFx0ZXhwZWN0KGN0cmwuZ2V0Rml4dHVyZUpzb24oKSkudG9Db250YWluKCdcIndpZGdldHNcIjpbXScpO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJkb2VzIG5vdCBhdXRvLWxvYWQgc3RvcmVkIGZpeHR1cmUgb24gc3RhcnR1cFwiLCAoKSA9PiB7XG5cdFx0XHRjb25zdCBiYWNraW5nID0gbmV3IE1hcDxzdHJpbmcsIHN0cmluZz4oKTtcblx0XHRcdGNvbnN0IHN0b3JlID0gY3JlYXRlUHJvY2VkdXJhbFBsYXlGaXh0dXJlU3RvcmUoe1xuXHRcdFx0XHRnZXRJdGVtOiAoaykgPT4gYmFja2luZy5nZXQoaykgPz8gbnVsbCxcblx0XHRcdFx0c2V0SXRlbTogKGssIHYpID0+IHtcblx0XHRcdFx0XHRiYWNraW5nLnNldChrLCB2KTtcblx0XHRcdFx0fSxcblx0XHRcdFx0cmVtb3ZlSXRlbTogKGspID0+IHtcblx0XHRcdFx0XHRiYWNraW5nLmRlbGV0ZShrKTtcblx0XHRcdFx0fSxcblx0XHRcdH0pO1xuXHRcdFx0c3RvcmUuc2F2ZShQUk9DRURVUkFMX1BMQVlfREVGQVVMVF9GSVhUVVJFX0pTT04pO1xuXHRcdFx0Y29uc3QgYnVzID0gbmV3IENvbW1hbmRCdXMoKTtcblx0XHRcdGNvbnN0IGN0cmwgPSBuZXcgUHJvY2VkdXJhbFBsYXlDb250cm9sbGVyKGJ1cywgKCkgPT4ge30sIHN0b3JlKTtcblx0XHRcdGV4cGVjdChjdHJsLmdldEZpeHR1cmVKc29uKCkpLnRvQ29udGFpbignXCJ3aWRnZXRzXCI6W10nKTtcblx0XHR9KTtcblxuXHRcdGl0KFwiY29udHJvbGxlciBzdG9yZXMgZml4dHVyZSBqc29uXCIsICgpID0+IHtcblx0XHRcdGNvbnN0IGJ1cyA9IG5ldyBDb21tYW5kQnVzKCk7XG5cdFx0XHRjb25zdCBjdHJsID0gbmV3IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcihidXMsICgpID0+IHt9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0Rml4dHVyZUpzb25cIiwgeyBqc29uOiAne1wic2NoZW1hXCI6XCJmbG93LmZpeHR1cmUvdjFcIn0nIH0pO1xuXHRcdFx0ZXhwZWN0KGN0cmwuZ2V0Rml4dHVyZUpzb24oKSkudG9Db250YWluKFwiZmxvdy5maXh0dXJlL3YxXCIpO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJraW5kcyB0cmVlIG1hcmtzIG5lc3RlZCBjYXRhbG9ndWUgcm93cyBkcmFnZ2FibGVcIiwgKCkgPT4ge1xuXHRcdFx0Y29uc3QgdHJlZSA9IGJ1aWxkUHJvY2VkdXJhbFBsYXlLaW5kc1RyZWUoW1xuXHRcdFx0XHR7XG5cdFx0XHRcdFx0aWQ6IFwiYnJlcFwiLFxuXHRcdFx0XHRcdHRpdGxlOiBcIkJyZXBcIixcblx0XHRcdFx0XHRpdGVtczogW10sXG5cdFx0XHRcdFx0Z3JvdXBzOiBbXG5cdFx0XHRcdFx0XHR7XG5cdFx0XHRcdFx0XHRcdGlkOiBcImJyZXAucHJpbWl0aXZlcy0zZFwiLFxuXHRcdFx0XHRcdFx0XHR0aXRsZTogXCJQcmltaXRpdmVzIDNEXCIsXG5cdFx0XHRcdFx0XHRcdGl0ZW1zOiBbeyBraW5kOiBcIm5ldXJvblwiLCBuZXVyb25LaW5kOiBcImJyZXAucHJpbTNkLmJveFwiLCBuYW1lOiBcIkJveFwiLCBhYmJyZXZpYXRpb246IFwiQm94XCIsIGljb246IFwiZW1vamk68J+TplwiLCBzdW1tYXJ5OiBcIkF4aXMtYWxpZ25lZCBib3hcIiB9XSxcblx0XHRcdFx0XHRcdH0sXG5cdFx0XHRcdFx0XSxcblx0XHRcdFx0fSxcblx0XHRcdF0pO1xuXHRcdFx0ZXhwZWN0KHRyZWUudHlwZSkudG9CZShcInRyZWVcIik7XG5cdFx0XHRjb25zdCBsZWFmID0gdHJlZS5zZWN0aW9ucz8uWzBdPy5pdGVtcz8uWzBdPy5pdGVtcz8uWzBdO1xuXHRcdFx0ZXhwZWN0KGxlYWY/LmRyYWdnYWJsZSkudG9CZSh0cnVlKTtcblx0XHRcdGV4cGVjdChsZWFmPy5kcmFnRGF0YSkudG9CZURlZmluZWQoKTtcblx0XHR9KTtcblxuXHRcdGl0KFwiY2F0YWxvZ3VlIHNuYXBzaG90IGxpc3RlbmVycyBmaXJlIHdoZW4gc2VjdGlvbnMgYXJyaXZlXCIsICgpID0+IHtcblx0XHRcdGNvbnN0IGJ1cyA9IG5ldyBDb21tYW5kQnVzKCk7XG5cdFx0XHRjb25zdCBjdHJsID0gbmV3IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcihidXMsICgpID0+IHt9KTtcblx0XHRcdGxldCByZXZpc2lvbiA9IGN0cmwuZ2V0Q2F0YWxvZ3VlUmV2aXNpb24oKTtcblx0XHRcdGNvbnN0IHVuc3Vic2NyaWJlID0gY3RybC5zdWJzY3JpYmVTbmFwc2hvdCgoKSA9PiB7XG5cdFx0XHRcdHJldmlzaW9uID0gY3RybC5nZXRDYXRhbG9ndWVSZXZpc2lvbigpO1xuXHRcdFx0fSk7XG5cdFx0XHRjdHJsLnJ1bihcInNldENhdGFsb2d1ZVNlY3Rpb25zXCIsIHsgc2VjdGlvbnM6IFt7IGlkOiBcImJyZXBcIiwgdGl0bGU6IFwiQnJlcFwiLCBpdGVtczogW10gfV0gfSk7XG5cdFx0XHR1bnN1YnNjcmliZSgpO1xuXHRcdFx0ZXhwZWN0KHJldmlzaW9uKS50b0JlKDEpO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJjYXRhbG9ndWUgcmV2aXNpb24gYnVtcHMgd2hlbiBzZWN0aW9ucyBhcnJpdmVcIiwgKCkgPT4ge1xuXHRcdFx0Y29uc3QgYnVzID0gbmV3IENvbW1hbmRCdXMoKTtcblx0XHRcdGNvbnN0IGN0cmwgPSBuZXcgUHJvY2VkdXJhbFBsYXlDb250cm9sbGVyKGJ1cywgKCkgPT4ge30pO1xuXHRcdFx0ZXhwZWN0KGN0cmwuZ2V0Q2F0YWxvZ3VlUmV2aXNpb24oKSkudG9CZSgwKTtcblx0XHRcdGN0cmwucnVuKFwic2V0Q2F0YWxvZ3VlU2VjdGlvbnNcIiwge1xuXHRcdFx0XHRzZWN0aW9uczogW1xuXHRcdFx0XHRcdHtcblx0XHRcdFx0XHRcdGlkOiBcImJyZXBcIixcblx0XHRcdFx0XHRcdHRpdGxlOiBcIkJyZXBcIixcblx0XHRcdFx0XHRcdGl0ZW1zOiBbXSxcblx0XHRcdFx0XHRcdGdyb3VwczogW1xuXHRcdFx0XHRcdFx0XHR7XG5cdFx0XHRcdFx0XHRcdFx0aWQ6IFwiYnJlcC5wcmltaXRpdmVzLTNkXCIsXG5cdFx0XHRcdFx0XHRcdFx0dGl0bGU6IFwiUHJpbWl0aXZlcyAzRFwiLFxuXHRcdFx0XHRcdFx0XHRcdGl0ZW1zOiBbeyBraW5kOiBcIm5ldXJvblwiLCBuZXVyb25LaW5kOiBcImJyZXAucHJpbTNkLmJveFwiLCBuYW1lOiBcIkJveFwiLCBhYmJyZXZpYXRpb246IFwiQm94XCIsIGljb246IFwiZW1vamk68J+TplwiLCBzdW1tYXJ5OiBcIkJveFwiIH1dLFxuXHRcdFx0XHRcdFx0XHR9LFxuXHRcdFx0XHRcdFx0XHR7XG5cdFx0XHRcdFx0XHRcdFx0aWQ6IFwiYnJlcC5jdXJ2ZXNcIixcblx0XHRcdFx0XHRcdFx0XHR0aXRsZTogXCJDdXJ2ZXNcIixcblx0XHRcdFx0XHRcdFx0XHRpdGVtczogW3sga2luZDogXCJuZXVyb25cIiwgbmV1cm9uS2luZDogXCJicmVwLmN1cnZlLmxpbmVcIiwgbmFtZTogXCJMaW5lXCIsIGFiYnJldmlhdGlvbjogXCJMaW5lXCIsIGljb246IFwiZW1vamk644Cw77iPXCIsIHN1bW1hcnk6IFwiTGluZSBlZGdlXCIgfV0sXG5cdFx0XHRcdFx0XHRcdH0sXG5cdFx0XHRcdFx0XHRdLFxuXHRcdFx0XHRcdH0sXG5cdFx0XHRcdF0sXG5cdFx0XHR9KTtcblx0XHRcdGV4cGVjdChjdHJsLmdldENhdGFsb2d1ZVJldmlzaW9uKCkpLnRvQmUoMSk7XG5cdFx0fSk7XG5cblx0XHRpdChcImNhdGFsb2d1ZSByZXZpc2lvbiBidW1wcyBmb3IgbmVzdGVkIGJyZXAgZ3JvdXBzXCIsICgpID0+IHtcblx0XHRcdGNvbnN0IGJ1cyA9IG5ldyBDb21tYW5kQnVzKCk7XG5cdFx0XHRjb25zdCBjdHJsID0gbmV3IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcihidXMsICgpID0+IHt9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0Q2F0YWxvZ3VlU2VjdGlvbnNcIiwge1xuXHRcdFx0XHRzZWN0aW9uczogW1xuXHRcdFx0XHRcdHtcblx0XHRcdFx0XHRcdGlkOiBcImJyZXBcIixcblx0XHRcdFx0XHRcdHRpdGxlOiBcIkJyZXBcIixcblx0XHRcdFx0XHRcdGl0ZW1zOiBbXSxcblx0XHRcdFx0XHRcdGdyb3VwczogW1xuXHRcdFx0XHRcdFx0XHR7IGlkOiBcImJyZXAucHJpbWl0aXZlcy0zZFwiLCB0aXRsZTogXCJQcmltaXRpdmVzIDNEXCIsIGl0ZW1zOiBbXSB9LFxuXHRcdFx0XHRcdFx0XHR7IGlkOiBcImJyZXAuc29saWRcIiwgdGl0bGU6IFwiU29saWRcIiwgaXRlbXM6IFtdIH0sXG5cdFx0XHRcdFx0XHRdLFxuXHRcdFx0XHRcdH0sXG5cdFx0XHRcdF0sXG5cdFx0XHR9KTtcblx0XHRcdGV4cGVjdChjdHJsLmdldENhdGFsb2d1ZVNlY3Rpb25zKClbMF0/Lmdyb3Vwcz8ubGVuZ3RoKS50b0JlKDIpO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJjb250cm9sbGVyIGV4cG9zZXMgZmxvdyBhbmQgcHJldmlldyB3aW5kb3cga2luZHNcIiwgKCkgPT4ge1xuXHRcdFx0Y29uc3QgYnVzID0gbmV3IENvbW1hbmRCdXMoKTtcblx0XHRcdGNvbnN0IGN0cmwgPSBuZXcgUHJvY2VkdXJhbFBsYXlDb250cm9sbGVyKGJ1cywgKCkgPT4ge30pO1xuXHRcdFx0ZXhwZWN0KGN0cmwubWFpbk1vZGUud2luZG93S2luZHMpLnRvSGF2ZUxlbmd0aCgyKTtcblx0XHRcdGV4cGVjdChjdHJsLm1haW5Nb2RlLndpbmRvd0tpbmRzWzFdPy5pZCkudG9CZShQUk9DRURVUkFMX1BMQVlfV0lORE9XX0tJTkRfUFJFVklFVyk7XG5cdFx0fSk7XG5cblx0XHRpdChcImZsb3cgd2luZG93IGV4cG9zZXMgaW5saW5lIGxvZCBzZWxlY3RcIiwgKCkgPT4ge1xuXHRcdFx0Y29uc3QgYnVzID0gbmV3IENvbW1hbmRCdXMoKTtcblx0XHRcdGNvbnN0IGN0cmwgPSBuZXcgUHJvY2VkdXJhbFBsYXlDb250cm9sbGVyKGJ1cywgKCkgPT4ge30pO1xuXHRcdFx0Y29uc3QgbWVhc3VyZXMgPSBjdHJsLm1haW5Nb2RlLndpbmRvd0tpbmRzWzBdPy5tZWFzdXJlcyA/PyBbXTtcblx0XHRcdGV4cGVjdChtZWFzdXJlcy5zb21lKChtZWFzdXJlKSA9PiBtZWFzdXJlLmtpbmQgPT09IFwic2VsZWN0XCIgJiYgbWVhc3VyZS5sYWJlbCA9PT0gXCJMT0RcIikpLnRvQmUodHJ1ZSk7XG5cdFx0fSk7XG5cblx0XHRpdChcImZsb3cgd2luZG93IHByb3hpbWl0eSBtZWFzdXJlIGRlZmF1bHRzIGFuZCB1cGRhdGVzIHZpYSBjb21tYW5kXCIsICgpID0+IHtcblx0XHRcdGNvbnN0IGJ1cyA9IG5ldyBDb21tYW5kQnVzKCk7XG5cdFx0XHRjb25zdCBjdHJsID0gbmV3IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcihidXMsICgpID0+IHt9KTtcblx0XHRcdGV4cGVjdChjdHJsLnByb3hpbWl0eURpc3RhbmNlVmFsdWUoKSkudG9CZShGTE9XX0RFRkFVTFRfUFJPWElNSVRZX0RJU1RBTkNFKTtcblx0XHRcdGNvbnN0IG1lYXN1cmVzID0gY3RybC5tYWluTW9kZS53aW5kb3dLaW5kc1swXT8ubWVhc3VyZXMgPz8gW107XG5cdFx0XHRjb25zdCBwcm94aW1pdHkgPSBtZWFzdXJlcy5maW5kKChtZWFzdXJlKSA9PiBtZWFzdXJlLmtpbmQgPT09IFwic2xpZGVyXCIgJiYgbWVhc3VyZS5sYWJlbCA9PT0gXCJQcm94aW1pdHlcIik7XG5cdFx0XHRleHBlY3QocHJveGltaXR5Py5raW5kKS50b0JlKFwic2xpZGVyXCIpO1xuXHRcdFx0aWYgKHByb3hpbWl0eT8ua2luZCA9PT0gXCJzbGlkZXJcIikge1xuXHRcdFx0XHRleHBlY3QocHJveGltaXR5LnZhbHVlKS50b0JlKEZMT1dfREVGQVVMVF9QUk9YSU1JVFlfRElTVEFOQ0UpO1xuXHRcdFx0fVxuXHRcdFx0Y3RybC5ydW4oXCJzZXRQcm94aW1pdHlEaXN0YW5jZVwiLCB7IHZhbHVlOiAwIH0pO1xuXHRcdFx0ZXhwZWN0KGN0cmwucHJveGltaXR5RGlzdGFuY2VWYWx1ZSgpKS50b0JlKDApO1xuXHRcdFx0Y29uc3QgdXBkYXRlZCA9IGN0cmwubWFpbk1vZGUud2luZG93S2luZHNbMF0/Lm1lYXN1cmVzPy5maW5kKChtZWFzdXJlKSA9PiBtZWFzdXJlLmtpbmQgPT09IFwic2xpZGVyXCIgJiYgbWVhc3VyZS5sYWJlbCA9PT0gXCJQcm94aW1pdHlcIik7XG5cdFx0XHRleHBlY3QodXBkYXRlZD8ua2luZCkudG9CZShcInNsaWRlclwiKTtcblx0XHRcdGlmICh1cGRhdGVkPy5raW5kID09PSBcInNsaWRlclwiKSB7XG5cdFx0XHRcdGV4cGVjdCh1cGRhdGVkLnZhbHVlKS50b0JlKDApO1xuXHRcdFx0fVxuXHRcdH0pO1xuXG5cdFx0aXQoXCJwcmV2aWV3IHdpbmRvdyBleHBvc2VzIHNob3cgbW9kZSBhbmQgdHJhbnNmb3JtIGRldGFpbCBpbiBzaGVsbCBtZWFzdXJlc1wiLCAoKSA9PiB7XG5cdFx0XHRjb25zdCBidXMgPSBuZXcgQ29tbWFuZEJ1cygpO1xuXHRcdFx0Y29uc3QgY3RybCA9IG5ldyBQcm9jZWR1cmFsUGxheUNvbnRyb2xsZXIoYnVzLCAoKSA9PiB7fSk7XG5cdFx0XHRjb25zdCBtZWFzdXJlcyA9IGN0cmwubWFpbk1vZGUud2luZG93S2luZHNbMV0/Lm1lYXN1cmVzID8/IFtdO1xuXHRcdFx0Y29uc3Qgc2hvdyA9IG1lYXN1cmVzLmZpbmQoKG1lYXN1cmUpID0+IG1lYXN1cmUua2luZCA9PT0gXCJzZWxlY3RcIiAmJiBtZWFzdXJlLmxhYmVsID09PSBcIlNob3dcIik7XG5cdFx0XHRleHBlY3Qoc2hvdz8ua2luZCA9PT0gXCJzZWxlY3RcIiAmJiBzaG93LnZhbHVlKS50b0JlKFwiZXZlcnl0aGluZ1wiKTtcblx0XHRcdGV4cGVjdChtZWFzdXJlcy5zb21lKChtZWFzdXJlKSA9PiBtZWFzdXJlLmtpbmQgPT09IFwic2VsZWN0XCIgJiYgbWVhc3VyZS5sYWJlbCA9PT0gXCJUcmFuc2Zvcm0gRGV0YWlsXCIpKS50b0JlKHRydWUpO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJzZXRUcmFuc2Zvcm1HcmFudWxhcml0eSBhY2NlcHRzIHNoZWxsIG1lYXN1cmUgdmFsdWVcIiwgKCkgPT4ge1xuXHRcdFx0Y29uc3QgYnVzID0gbmV3IENvbW1hbmRCdXMoKTtcblx0XHRcdGNvbnN0IGN0cmwgPSBuZXcgUHJvY2VkdXJhbFBsYXlDb250cm9sbGVyKGJ1cywgKCkgPT4ge30pO1xuXHRcdFx0Y3RybC5ydW4oXCJzZXRUcmFuc2Zvcm1HcmFudWxhcml0eVwiLCB7IHZhbHVlOiBcImNvbXBhY3RcIiB9KTtcblx0XHRcdGV4cGVjdChjdHJsLmdldFRyYW5zZm9ybUdyYW51bGFyaXR5KCkpLnRvQmUoXCJjb21wYWN0XCIpO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJzZXRTaG93TW9kZSB1cGRhdGVzIHByZXZpZXcgZmlsdGVyXCIsICgpID0+IHtcblx0XHRcdGNvbnN0IGJ1cyA9IG5ldyBDb21tYW5kQnVzKCk7XG5cdFx0XHRjb25zdCBjdHJsID0gbmV3IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcihidXMsICgpID0+IHt9KTtcblx0XHRcdGV4cGVjdChjdHJsLmdldFNob3dNb2RlKCkpLnRvQmUoXCJldmVyeXRoaW5nXCIpO1xuXHRcdFx0Y3RybC5ydW4oXCJzZXRTaG93TW9kZVwiLCB7IGlkOiBcInNlbGVjdGVkXCIgfSk7XG5cdFx0XHRleHBlY3QoY3RybC5nZXRTaG93TW9kZSgpKS50b0JlKFwic2VsZWN0ZWRcIik7XG5cdFx0fSk7XG5cblx0XHRpdChcInNldFNob3dNb2RlIGFjY2VwdHMgc2hlbGwgbWVhc3VyZSB2YWx1ZVwiLCAoKSA9PiB7XG5cdFx0XHRjb25zdCBidXMgPSBuZXcgQ29tbWFuZEJ1cygpO1xuXHRcdFx0Y29uc3QgY3RybCA9IG5ldyBQcm9jZWR1cmFsUGxheUNvbnRyb2xsZXIoYnVzLCAoKSA9PiB7fSk7XG5cdFx0XHRjdHJsLnJ1bihcInNldFNob3dNb2RlXCIsIHsgdmFsdWU6IFwic2VsZWN0ZWRcIiB9KTtcblx0XHRcdGV4cGVjdChjdHJsLmdldFNob3dNb2RlKCkpLnRvQmUoXCJzZWxlY3RlZFwiKTtcblx0XHRcdGN0cmwucnVuKFwic2V0U2hvd01vZGVcIiwgeyB2YWx1ZTogXCJldmVyeXRoaW5nXCIgfSk7XG5cdFx0XHRleHBlY3QoY3RybC5nZXRTaG93TW9kZSgpKS50b0JlKFwiZXZlcnl0aGluZ1wiKTtcblx0XHR9KTtcblxuXHRcdGl0KFwiY2FudmFzQ29tbWFuZCBidW1wcyBjb21tYW5kIHJlcXVlc3QgZXBvY2hcIiwgKCkgPT4ge1xuXHRcdFx0Y29uc3QgYnVzID0gbmV3IENvbW1hbmRCdXMoKTtcblx0XHRcdGNvbnN0IGN0cmwgPSBuZXcgUHJvY2VkdXJhbFBsYXlDb250cm9sbGVyKGJ1cywgKCkgPT4ge30pO1xuXHRcdFx0Y3RybC5ydW4oXCJjYW52YXNDb21tYW5kXCIsIHsgY29tbWFuZDogXCJkZWxldGVTZWxlY3Rpb25cIiB9KTtcblx0XHRcdGV4cGVjdChjdHJsLmdldENvbW1hbmRSZXF1ZXN0KCkuY29tbWFuZCkudG9CZShcImRlbGV0ZVNlbGVjdGlvblwiKTtcblx0XHRcdGV4cGVjdChjdHJsLmdldENvbW1hbmRSZXF1ZXN0KCkuZXBvY2gpLnRvQmUoMSk7XG5cdFx0fSk7XG5cblx0XHRpdChcImRlbGV0ZVNlbGVjdGlvbiBmb3J3YXJkcyB0byBmbG93IGNhbnZhcyBjb21tYW5kIHJlcXVlc3RcIiwgKCkgPT4ge1xuXHRcdFx0Y29uc3QgYnVzID0gbmV3IENvbW1hbmRCdXMoKTtcblx0XHRcdGNvbnN0IGN0cmwgPSBuZXcgUHJvY2VkdXJhbFBsYXlDb250cm9sbGVyKGJ1cywgKCkgPT4ge30pO1xuXHRcdFx0Y3RybC5ydW4oXCJzZXRTZWxlY3Rpb25cIiwgeyBpZHM6IFtcIm5vZGUtYVwiXSB9KTtcblx0XHRcdGN0cmwucnVuKFwiZGVsZXRlU2VsZWN0aW9uXCIpO1xuXHRcdFx0ZXhwZWN0KGN0cmwuZ2V0Q29tbWFuZFJlcXVlc3QoKS5jb21tYW5kKS50b0JlKFwiZGVsZXRlU2VsZWN0aW9uXCIpO1xuXHRcdFx0ZXhwZWN0KGN0cmwuZ2V0U2VsZWN0ZWROb2RlSWRzKCkpLnRvRXF1YWwoW1wibm9kZS1hXCJdKTtcblx0XHR9KTtcblxuXHRcdGl0KFwic2V0UHJldmlld09mZiBzdG9yZXMgcHJldmlldy1vZmYgbm9kZSBpZHNcIiwgKCkgPT4ge1xuXHRcdFx0Y29uc3QgYnVzID0gbmV3IENvbW1hbmRCdXMoKTtcblx0XHRcdGNvbnN0IGN0cmwgPSBuZXcgUHJvY2VkdXJhbFBsYXlDb250cm9sbGVyKGJ1cywgKCkgPT4ge30pO1xuXHRcdFx0Y3RybC5ydW4oXCJzZXRQcmV2aWV3T2ZmXCIsIHsgaWRzOiBbXCJhXCIsIFwiYlwiXSB9KTtcblx0XHRcdGV4cGVjdChjdHJsLmdldFByZXZpZXdPZmZOb2RlSWRzKCkpLnRvRXF1YWwoW1wiYVwiLCBcImJcIl0pO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJidWlsZFByb2NlZHVyYWxQbGF5Q2FudmFzQ29udGV4dE1lbnUgYWRkcyBpc29sYXRlIGluIHByZXZpZXcgZm9yIGhvdmVyZWQgbm9kZVwiLCAoKSA9PiB7XG5cdFx0XHRjb25zdCBpdGVtcyA9IGJ1aWxkUHJvY2VkdXJhbFBsYXlDYW52YXNDb250ZXh0TWVudShcblx0XHRcdFx0e1xuXHRcdFx0XHRcdGhvdmVyZWROb2RlSWQ6IFwiYm94XCIsXG5cdFx0XHRcdFx0c2VsZWN0ZWROb2RlSWRzOiBbXCJib3hcIl0sXG5cdFx0XHRcdFx0Y2x1c3Rlck5vZGVJZHM6IFtdLFxuXHRcdFx0XHRcdGlzSW1hZ2VXaWRnZXQ6IGZhbHNlLFxuXHRcdFx0XHRcdGlzQmFja2dyb3VuZDogZmFsc2UsXG5cdFx0XHRcdFx0cHJldmlld09mZk5vZGVJZHM6IFtdLFxuXHRcdFx0XHRcdHNjcmVlbjogeyB4OiAwLCB5OiAwIH0sXG5cdFx0XHRcdFx0d29ybGQ6IHsgeDogMCwgeTogMCB9LFxuXHRcdFx0XHRcdGNsaWVudFg6IDAsXG5cdFx0XHRcdFx0Y2xpZW50WTogMCxcblx0XHRcdFx0fSxcblx0XHRcdFx0KCkgPT4ge30sXG5cdFx0XHQpO1xuXHRcdFx0ZXhwZWN0KGl0ZW1zLnNvbWUoKGl0ZW0pID0+IGl0ZW0uaWQgPT09IFwicHJvY2VkdXJhbC5jdHguaXNvbGF0ZVByZXZpZXdcIikpLnRvQmUodHJ1ZSk7XG5cdFx0fSk7XG5cblx0XHRpdChcInNldEZpeHR1cmVKc29uIHN5bmMgcHJlc2VydmVzIHByZXZpZXcgaXRlbXMgYWZ0ZXIgZmxvdyBpbnRlcmFjdGlvblwiLCAoKSA9PiB7XG5cdFx0XHRjb25zdCBidXMgPSBuZXcgQ29tbWFuZEJ1cygpO1xuXHRcdFx0Y29uc3QgY3RybCA9IG5ldyBQcm9jZWR1cmFsUGxheUNvbnRyb2xsZXIoYnVzLCAoKSA9PiB7fSk7XG5cdFx0XHRjdHJsLnJ1bihcInNldEV2YWxPdXRwdXRzXCIsIHtcblx0XHRcdFx0b3V0cHV0c0pzb246IEpTT04uc3RyaW5naWZ5KHsgYm94OiB7IGluOiB7fSwgb3V0OiB7IHNvbGlkOiB7IGdlb21ldHJ5OiBcInNvbGlkLTFcIiB9IH0gfSB9KSxcblx0XHRcdH0pO1xuXHRcdFx0Y29uc3QgYmFzZSA9IGN0cmwuZ2V0Rml4dHVyZUpzb24oKTtcblx0XHRcdGNvbnN0IGludGVyYWN0ZWQgPSBKU09OLnN0cmluZ2lmeSh7XG5cdFx0XHRcdC4uLkpTT04ucGFyc2UoYmFzZSksXG5cdFx0XHRcdGNhbWVyYTogeyB4OiAxMiwgeTogLTQsIHpvb206IDIuNSB9LFxuXHRcdFx0XHR3aWRnZXRzOiBbXG5cdFx0XHRcdFx0eyBraW5kOiBcIm5ldXJvblwiLCBpZDogXCJza2V0Y2hcIiwgbmV1cm9uS2luZDogXCJicmVwLnNrZXRjaDJkLnJlY3RhbmdsZVwiIH0sXG5cdFx0XHRcdFx0eyBraW5kOiBcIm5ldXJvblwiLCBpZDogXCJzb2xpZFwiLCBuZXVyb25LaW5kOiBcImJyZXAuc29saWQuZXh0cnVkZVwiIH0sXG5cdFx0XHRcdFx0eyBraW5kOiBcIm91dHB1dFByZXZpZXdcIiwgaWQ6IFwicHJldmlld1wiLCBwcmV2aWV3OiB7IGdlb21ldHJ5OiBcInNvbGlkLTlcIiB9IH0sXG5cdFx0XHRcdF0sXG5cdFx0XHR9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0Rml4dHVyZUpzb25cIiwgeyBqc29uOiBpbnRlcmFjdGVkIH0pO1xuXHRcdFx0ZXhwZWN0KGN0cmwuZ2V0UHJldmlld0l0ZW1zKCkpLnRvRXF1YWwoW1xuXHRcdFx0XHR7IHdpZGdldElkOiBcImJveFwiLCBwb3J0OiBcInNvbGlkXCIsIGRpcmVjdGlvbjogXCJvdXRcIiwga2luZDogXCJnZW9tZXRyeVwiLCBoYW5kbGU6IFwic29saWQtMVwiIH0sXG5cdFx0XHRdKTtcblx0XHR9KTtcblxuXHRcdGl0KFwic2V0Rml4dHVyZUpzb24gd2l0aCByZXNldEludGVyYWN0aW9uIGNsZWFycyBwcmV2aWV3IGl0ZW1zXCIsICgpID0+IHtcblx0XHRcdGNvbnN0IGJ1cyA9IG5ldyBDb21tYW5kQnVzKCk7XG5cdFx0XHRjb25zdCBjdHJsID0gbmV3IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcihidXMsICgpID0+IHt9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0RXZhbE91dHB1dHNcIiwge1xuXHRcdFx0XHRvdXRwdXRzSnNvbjogSlNPTi5zdHJpbmdpZnkoeyBib3g6IHsgaW46IHt9LCBvdXQ6IHsgc29saWQ6IHsgZ2VvbWV0cnk6IFwic29saWQtMVwiIH0gfSB9IH0pLFxuXHRcdFx0fSk7XG5cdFx0XHRjdHJsLnJ1bihcInNldEZpeHR1cmVKc29uXCIsIHtcblx0XHRcdFx0anNvbjogJ3tcInNjaGVtYVwiOlwiZmxvdy5maXh0dXJlL3YxXCIsXCJjYW1lcmFcIjp7XCJ4XCI6MCxcInlcIjowLFwiem9vbVwiOjF9LFwid2lkZ2V0c1wiOltdLFwic3luYXBzZXNcIjpbXX0nLFxuXHRcdFx0XHRyZXNldEludGVyYWN0aW9uOiB0cnVlLFxuXHRcdFx0fSk7XG5cdFx0XHRleHBlY3QoY3RybC5nZXRQcmV2aWV3SXRlbXMoKSkudG9FcXVhbChbXSk7XG5cdFx0fSk7XG5cblx0XHRpdChcInNldEV2YWxPdXRwdXRzIHN0b3JlcyBwcmV2aWV3IGl0ZW1zIHBlciB3aWRnZXRcIiwgKCkgPT4ge1xuXHRcdFx0Y29uc3QgYnVzID0gbmV3IENvbW1hbmRCdXMoKTtcblx0XHRcdGNvbnN0IGN0cmwgPSBuZXcgUHJvY2VkdXJhbFBsYXlDb250cm9sbGVyKGJ1cywgKCkgPT4ge30pO1xuXHRcdFx0Y3RybC5ydW4oXCJzZXRFdmFsT3V0cHV0c1wiLCB7XG5cdFx0XHRcdG91dHB1dHNKc29uOiBKU09OLnN0cmluZ2lmeSh7IGJveDogeyBpbjoge30sIG91dDogeyBzb2xpZDogeyBnZW9tZXRyeTogXCJzb2xpZC0xXCIgfSB9IH0gfSksXG5cdFx0XHR9KTtcblx0XHRcdGV4cGVjdChjdHJsLmdldFByZXZpZXdJdGVtcygpKS50b0VxdWFsKFtcblx0XHRcdFx0eyB3aWRnZXRJZDogXCJib3hcIiwgcG9ydDogXCJzb2xpZFwiLCBkaXJlY3Rpb246IFwib3V0XCIsIGtpbmQ6IFwiZ2VvbWV0cnlcIiwgaGFuZGxlOiBcInNvbGlkLTFcIiB9LFxuXHRcdFx0XSk7XG5cdFx0fSk7XG5cblx0XHRpdChcInNldEV2YWxPdXRwdXRzIHN0b3JlcyBwb2ludCBhbmQgdmVjdG9yIHByZXZpZXcgaXRlbXNcIiwgKCkgPT4ge1xuXHRcdFx0Y29uc3QgYnVzID0gbmV3IENvbW1hbmRCdXMoKTtcblx0XHRcdGNvbnN0IGN0cmwgPSBuZXcgUHJvY2VkdXJhbFBsYXlDb250cm9sbGVyKGJ1cywgKCkgPT4ge30pO1xuXHRcdFx0Y3RybC5ydW4oXCJzZXRFdmFsT3V0cHV0c1wiLCB7XG5cdFx0XHRcdG91dHB1dHNKc29uOiBKU09OLnN0cmluZ2lmeSh7XG5cdFx0XHRcdFx0cHQ6IHsgaW46IHt9LCBvdXQ6IHsgcG9pbnQ6IHsgJHNjaGVtYTogXCJwb2ludFwiLCB4OiAxLCB5OiAwLCB6OiAwIH0gfSB9LFxuXHRcdFx0XHRcdHZlYzogeyBpbjoge30sIG91dDogeyB2ZWN0b3I6IHsgJHNjaGVtYTogXCJ2ZWN0b3JcIiwgeDogMCwgeTogMSwgejogMCB9IH0gfSxcblx0XHRcdFx0fSksXG5cdFx0XHR9KTtcblx0XHRcdGV4cGVjdChjdHJsLmdldFByZXZpZXdJdGVtcygpKS50b0VxdWFsKFtcblx0XHRcdFx0eyB3aWRnZXRJZDogXCJwdFwiLCBwb3J0OiBcInBvaW50XCIsIGRpcmVjdGlvbjogXCJvdXRcIiwga2luZDogXCJwb2ludFwiLCBwb3NpdGlvbjogWzEsIDAsIDBdIH0sXG5cdFx0XHRcdHsgd2lkZ2V0SWQ6IFwidmVjXCIsIHBvcnQ6IFwidmVjdG9yXCIsIGRpcmVjdGlvbjogXCJvdXRcIiwga2luZDogXCJ2ZWN0b3JcIiwgZGlyZWN0aW9uVmVjOiBbMCwgMSwgMF0gfSxcblx0XHRcdF0pO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJzZWxlY3RBbGwgaW5jbHVkZXMgd2lkZ2V0cyB3aXRoIHBvaW50IGFuZCB2ZWN0b3IgcHJldmlldyBpdGVtc1wiLCAoKSA9PiB7XG5cdFx0XHRjb25zdCBidXMgPSBuZXcgQ29tbWFuZEJ1cygpO1xuXHRcdFx0Y29uc3QgY3RybCA9IG5ldyBQcm9jZWR1cmFsUGxheUNvbnRyb2xsZXIoYnVzLCAoKSA9PiB7fSk7XG5cdFx0XHRjdHJsLnJ1bihcInNldEV2YWxPdXRwdXRzXCIsIHtcblx0XHRcdFx0b3V0cHV0c0pzb246IEpTT04uc3RyaW5naWZ5KHtcblx0XHRcdFx0XHRwdDogeyBpbjoge30sIG91dDogeyBwb2ludDogeyAkc2NoZW1hOiBcInBvaW50XCIsIHg6IDAsIHk6IDAsIHo6IDAgfSB9IH0sXG5cdFx0XHRcdFx0dmVjOiB7IGluOiB7fSwgb3V0OiB7IHZlY3RvcjogeyAkc2NoZW1hOiBcInZlY3RvclwiLCB4OiAxLCB5OiAwLCB6OiAwIH0gfSB9LFxuXHRcdFx0XHR9KSxcblx0XHRcdH0pO1xuXHRcdFx0Y3RybC5ydW4oXCJzZWxlY3RBbGxcIik7XG5cdFx0XHRleHBlY3QoY3RybC5nZXRTZWxlY3RlZE5vZGVJZHMoKS5zb3J0KCkpLnRvRXF1YWwoW1wicHRcIiwgXCJ2ZWNcIl0pO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJzZXRIb3ZlckNoYW5uZWwgYW5kIGdlb21ldHJ5IHRhcmdldCBnZXR0ZXJzIHJlc29sdmUgdXBzdHJlYW0gb3V0cHV0XCIsICgpID0+IHtcblx0XHRcdGNvbnN0IGJ1cyA9IG5ldyBDb21tYW5kQnVzKCk7XG5cdFx0XHRjb25zdCBjdHJsID0gbmV3IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcihidXMsICgpID0+IHt9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0RXZhbE91dHB1dHNcIiwge1xuXHRcdFx0XHRvdXRwdXRzSnNvbjogSlNPTi5zdHJpbmdpZnkoe1xuXHRcdFx0XHRcdGNpcmNsZTogeyBpbjoge30sIG91dDogeyB3aXJlOiB7IGdlb21ldHJ5OiBcImRyYXdpbmctMVwiIH0gfSB9LFxuXHRcdFx0XHRcdG9mZnNldDogeyBpbjogeyBnZW9tZXRyeTogXCJkcmF3aW5nLTFcIiB9LCBvdXQ6IHsgZ2VvbWV0cnk6IHsgZ2VvbWV0cnk6IFwid2lyZS0yXCIgfSB9IH0sXG5cdFx0XHRcdH0pLFxuXHRcdFx0fSk7XG5cdFx0XHRjdHJsLnJ1bihcInNldEZpeHR1cmVKc29uXCIsIHtcblx0XHRcdFx0anNvbjogSlNPTi5zdHJpbmdpZnkoe1xuXHRcdFx0XHRcdHNjaGVtYTogXCJmbG93LmZpeHR1cmUvdjFcIixcblx0XHRcdFx0XHRjYW1lcmE6IHsgeDogMCwgeTogMCwgem9vbTogMSB9LFxuXHRcdFx0XHRcdHdpZGdldHM6IFtcblx0XHRcdFx0XHRcdHsga2luZDogXCJuZXVyb25cIiwgaWQ6IFwiY2lyY2xlXCIsIG5ldXJvbktpbmQ6IFwiYnJlcC5za2V0Y2gyZC5jaXJjbGVcIiB9LFxuXHRcdFx0XHRcdFx0eyBraW5kOiBcIm5ldXJvblwiLCBpZDogXCJvZmZzZXRcIiwgbmV1cm9uS2luZDogXCJicmVwLnhmb3JtLm9mZnNldFwiIH0sXG5cdFx0XHRcdFx0XSxcblx0XHRcdFx0XHRzeW5hcHNlczogW3sgaWQ6IFwiczFcIiwgZnJvbTogXCJjaXJjbGVcIiwgdG86IFwib2Zmc2V0XCIsIGZyb21fcG9ydDogXCJ3aXJlXCIsIHRvX3BvcnQ6IFwiZ2VvbWV0cnlcIiB9XSxcblx0XHRcdFx0fSksXG5cdFx0XHR9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0SG92ZXJDaGFubmVsXCIsIHtcblx0XHRcdFx0Y2hhbm5lbDogeyB3aWRnZXRJZDogXCJvZmZzZXRcIiwgcG9ydDogXCJnZW9tZXRyeVwiLCBkaXJlY3Rpb246IFwiaW5cIiB9LFxuXHRcdFx0fSk7XG5cdFx0XHRleHBlY3QoY3RybC5nZXRIb3ZlcmVkQ2hhbm5lbCgpKS50b0VxdWFsKHsgd2lkZ2V0SWQ6IFwib2Zmc2V0XCIsIHBvcnQ6IFwiZ2VvbWV0cnlcIiwgZGlyZWN0aW9uOiBcImluXCIgfSk7XG5cdFx0XHRleHBlY3QoY3RybC5nZXRIb3ZlcmVkR2VvbWV0cnlUYXJnZXRzKCkpLnRvRXF1YWwoW3sgd2lkZ2V0SWQ6IFwiY2lyY2xlXCIsIHBvcnQ6IFwid2lyZVwiLCBkaXJlY3Rpb246IFwib3V0XCIgfV0pO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJwYXJzZUZpeHR1cmVFZGdlcyByZWFkcyBjYW1lbENhc2UgZmxvdyBzeW5hcHNlIHBvcnRzXCIsICgpID0+IHtcblx0XHRcdGNvbnN0IGJ1cyA9IG5ldyBDb21tYW5kQnVzKCk7XG5cdFx0XHRjb25zdCBjdHJsID0gbmV3IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcihidXMsICgpID0+IHt9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0Rml4dHVyZUpzb25cIiwge1xuXHRcdFx0XHRqc29uOiBKU09OLnN0cmluZ2lmeSh7XG5cdFx0XHRcdFx0c2NoZW1hOiBcImZsb3cuZml4dHVyZS92MVwiLFxuXHRcdFx0XHRcdGNhbWVyYTogeyB4OiAwLCB5OiAwLCB6b29tOiAxIH0sXG5cdFx0XHRcdFx0d2lkZ2V0czogW10sXG5cdFx0XHRcdFx0c3luYXBzZXM6IFtcblx0XHRcdFx0XHRcdHsgaWQ6IFwiZTEwMVwiLCBmcm9tOiBcImJyZXBfcHJpbTNkX3NwaGVyZV8yXCIsIHRvOiBcImJyZXBfYm9vbF9jdXRfNVwiLCBmcm9tUG9ydDogXCJzb2xpZFwiLCB0b1BvcnQ6IFwiYVwiIH0sXG5cdFx0XHRcdFx0XHR7IGlkOiBcImUxMDJcIiwgZnJvbTogXCJicmVwX3ByaW0zZF90b3J1c180XCIsIHRvOiBcImJyZXBfYm9vbF9jdXRfNVwiLCBmcm9tUG9ydDogXCJzb2xpZFwiLCB0b1BvcnQ6IFwiYlwiIH0sXG5cdFx0XHRcdFx0XSxcblx0XHRcdFx0fSksXG5cdFx0XHR9KTtcblx0XHRcdGV4cGVjdChjdHJsLmdldFNlbGVjdGVkR2VvbWV0cnlUYXJnZXRzKCkpLnRvRXF1YWwoW10pO1xuXHRcdFx0Y3RybC5ydW4oXCJzZXRTZWxlY3RDaGFubmVsc1wiLCB7XG5cdFx0XHRcdGNoYW5uZWxzOiBbeyB3aWRnZXRJZDogXCJicmVwX2Jvb2xfY3V0XzVcIiwgcG9ydDogXCJhXCIsIGRpcmVjdGlvbjogXCJpblwiIH1dLFxuXHRcdFx0fSk7XG5cdFx0XHRjdHJsLnJ1bihcInNldEV2YWxPdXRwdXRzXCIsIHtcblx0XHRcdFx0b3V0cHV0c0pzb246IEpTT04uc3RyaW5naWZ5KHtcblx0XHRcdFx0XHRicmVwX3ByaW0zZF9zcGhlcmVfMjogeyBpbjoge30sIG91dDogeyBzb2xpZDogeyBnZW9tZXRyeTogXCJzb2xpZC1zcGhlcmVcIiB9IH0gfSxcblx0XHRcdFx0XHRicmVwX2Jvb2xfY3V0XzU6IHsgaW46IHsgYTogeyBnZW9tZXRyeTogXCJzb2xpZC1zcGhlcmVcIiB9IH0sIG91dDogeyBzb2xpZDogeyBnZW9tZXRyeTogXCJzb2xpZC1jdXRcIiB9IH0gfSxcblx0XHRcdFx0fSksXG5cdFx0XHR9KTtcblx0XHRcdGV4cGVjdChjdHJsLmdldFNlbGVjdGVkR2VvbWV0cnlUYXJnZXRzKCkpLnRvRXF1YWwoW1xuXHRcdFx0XHR7IHdpZGdldElkOiBcImJyZXBfcHJpbTNkX3NwaGVyZV8yXCIsIHBvcnQ6IFwic29saWRcIiwgZGlyZWN0aW9uOiBcIm91dFwiIH0sXG5cdFx0XHRdKTtcblx0XHR9KTtcblxuXHRcdGl0KFwic2hvdyBzZWxlY3RlZCByZXZlYWxzIHVwc3RyZWFtIGdlb21ldHJ5IGZvciBwcmV2aWV3LW9mZiBpbnB1dCBjaGFubmVsc1wiLCAoKSA9PiB7XG5cdFx0XHRjb25zdCBidXMgPSBuZXcgQ29tbWFuZEJ1cygpO1xuXHRcdFx0Y29uc3QgY3RybCA9IG5ldyBQcm9jZWR1cmFsUGxheUNvbnRyb2xsZXIoYnVzLCAoKSA9PiB7fSk7XG5cdFx0XHRjb25zdCBvdXRwdXRzSnNvbiA9IEpTT04uc3RyaW5naWZ5KHtcblx0XHRcdFx0YnJlcF9wcmltM2Rfc3BoZXJlXzI6IHsgaW46IHt9LCBvdXQ6IHsgc29saWQ6IHsgZ2VvbWV0cnk6IFwic29saWQtc3BoZXJlXCIgfSB9IH0sXG5cdFx0XHRcdGJyZXBfcHJpbTNkX3RvcnVzXzQ6IHsgaW46IHt9LCBvdXQ6IHsgc29saWQ6IHsgZ2VvbWV0cnk6IFwic29saWQtdG9ydXNcIiB9IH0gfSxcblx0XHRcdFx0YnJlcF9ib29sX2N1dF81OiB7XG5cdFx0XHRcdFx0aW46IHsgYTogeyBnZW9tZXRyeTogXCJzb2xpZC1zcGhlcmVcIiB9LCBiOiB7IGdlb21ldHJ5OiBcInNvbGlkLXRvcnVzXCIgfSB9LFxuXHRcdFx0XHRcdG91dDogeyBzb2xpZDogeyBnZW9tZXRyeTogXCJzb2xpZC1jdXRcIiB9IH0sXG5cdFx0XHRcdH0sXG5cdFx0XHR9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0RXZhbE91dHB1dHNcIiwgeyBvdXRwdXRzSnNvbiB9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0Rml4dHVyZUpzb25cIiwge1xuXHRcdFx0XHRqc29uOiBKU09OLnN0cmluZ2lmeSh7XG5cdFx0XHRcdFx0c2NoZW1hOiBcImZsb3cuZml4dHVyZS92MVwiLFxuXHRcdFx0XHRcdGNhbWVyYTogeyB4OiAwLCB5OiAwLCB6b29tOiAxIH0sXG5cdFx0XHRcdFx0d2lkZ2V0czogW1xuXHRcdFx0XHRcdFx0eyBraW5kOiBcIm5ldXJvblwiLCBpZDogXCJicmVwX3ByaW0zZF9zcGhlcmVfMlwiLCBuZXVyb25LaW5kOiBcImJyZXAucHJpbTNkLnNwaGVyZVwiLCBwcmV2aWV3OiBmYWxzZSB9LFxuXHRcdFx0XHRcdFx0eyBraW5kOiBcIm5ldXJvblwiLCBpZDogXCJicmVwX3ByaW0zZF90b3J1c180XCIsIG5ldXJvbktpbmQ6IFwiYnJlcC5wcmltM2QudG9ydXNcIiwgcHJldmlldzogZmFsc2UgfSxcblx0XHRcdFx0XHRcdHsga2luZDogXCJuZXVyb25cIiwgaWQ6IFwiYnJlcF9ib29sX2N1dF81XCIsIG5ldXJvbktpbmQ6IFwiYnJlcC5ib29sLmN1dFwiLCBwcmV2aWV3OiB0cnVlIH0sXG5cdFx0XHRcdFx0XSxcblx0XHRcdFx0XHRzeW5hcHNlczogW1xuXHRcdFx0XHRcdFx0eyBpZDogXCJlMVwiLCBmcm9tOiBcImJyZXBfcHJpbTNkX3NwaGVyZV8yXCIsIHRvOiBcImJyZXBfYm9vbF9jdXRfNVwiLCBmcm9tUG9ydDogXCJzb2xpZFwiLCB0b1BvcnQ6IFwiYVwiIH0sXG5cdFx0XHRcdFx0XHR7IGlkOiBcImUyXCIsIGZyb206IFwiYnJlcF9wcmltM2RfdG9ydXNfNFwiLCB0bzogXCJicmVwX2Jvb2xfY3V0XzVcIiwgZnJvbVBvcnQ6IFwic29saWRcIiwgdG9Qb3J0OiBcImJcIiB9LFxuXHRcdFx0XHRcdF0sXG5cdFx0XHRcdH0pLFxuXHRcdFx0fSk7XG5cdFx0XHRjdHJsLnJ1bihcInNldFByZXZpZXdPZmZcIiwge1xuXHRcdFx0XHRpZHM6IFtcImJyZXBfcHJpbTNkX3NwaGVyZV8yXCIsIFwiYnJlcF9wcmltM2RfdG9ydXNfNFwiXSxcblx0XHRcdFx0ZnJvbUZsb3c6IHRydWUsXG5cdFx0XHR9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0U2hvd01vZGVcIiwgeyBpZDogXCJzZWxlY3RlZFwiIH0pO1xuXHRcdFx0Y3RybC5ydW4oXCJzZXRTZWxlY3RDaGFubmVsc1wiLCB7XG5cdFx0XHRcdGNoYW5uZWxzOiBbeyB3aWRnZXRJZDogXCJicmVwX2Jvb2xfY3V0XzVcIiwgcG9ydDogXCJhXCIsIGRpcmVjdGlvbjogXCJpblwiIH1dLFxuXHRcdFx0fSk7XG5cdFx0XHRjb25zdCB2aXNpYmxlID0gZmlsdGVyVmlzaWJsZVByZXZpZXdJdGVtcyhjdHJsLmdldFByZXZpZXdJdGVtcygpLCB7XG5cdFx0XHRcdHNob3dNb2RlOiBjdHJsLmdldFNob3dNb2RlKCksXG5cdFx0XHRcdHNlbGVjdGVkTm9kZUlkczogWy4uLmN0cmwuZ2V0U2VsZWN0ZWROb2RlSWRzKCldLFxuXHRcdFx0XHRzZWxlY3RlZENoYW5uZWxzOiBbLi4uY3RybC5nZXRTZWxlY3RlZENoYW5uZWxzKCldLFxuXHRcdFx0XHRzZWxlY3RlZEdlb21ldHJ5VGFyZ2V0czogWy4uLmN0cmwuZ2V0U2VsZWN0ZWRHZW9tZXRyeVRhcmdldHMoKV0sXG5cdFx0XHRcdGhvdmVyZWROb2RlSWQ6IG51bGwsXG5cdFx0XHRcdGhvdmVyZWRDaGFubmVsOiBudWxsLFxuXHRcdFx0fSk7XG5cdFx0XHRleHBlY3QodmlzaWJsZSkudG9FcXVhbChbXG5cdFx0XHRcdHtcblx0XHRcdFx0XHR3aWRnZXRJZDogXCJicmVwX3ByaW0zZF9zcGhlcmVfMlwiLFxuXHRcdFx0XHRcdHBvcnQ6IFwic29saWRcIixcblx0XHRcdFx0XHRkaXJlY3Rpb246IFwib3V0XCIsXG5cdFx0XHRcdFx0a2luZDogXCJnZW9tZXRyeVwiLFxuXHRcdFx0XHRcdGhhbmRsZTogXCJzb2xpZC1zcGhlcmVcIixcblx0XHRcdFx0fSxcblx0XHRcdF0pO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJzZXRTZWxlY3RDaGFubmVscyBzdG9yZXMgY2hhbm5lbCBzZWxlY3Rpb24gYW5kIHBhcmVudCBub2Rlc1wiLCAoKSA9PiB7XG5cdFx0XHRjb25zdCBidXMgPSBuZXcgQ29tbWFuZEJ1cygpO1xuXHRcdFx0Y29uc3QgY3RybCA9IG5ldyBQcm9jZWR1cmFsUGxheUNvbnRyb2xsZXIoYnVzLCAoKSA9PiB7fSk7XG5cdFx0XHRjdHJsLnJ1bihcInNldFNlbGVjdENoYW5uZWxzXCIsIHtcblx0XHRcdFx0Y2hhbm5lbHM6IFt7IHdpZGdldElkOiBcImJveFwiLCBwb3J0OiBcInNvbGlkXCIsIGRpcmVjdGlvbjogXCJvdXRcIiB9XSxcblx0XHRcdH0pO1xuXHRcdFx0ZXhwZWN0KGN0cmwuZ2V0U2VsZWN0ZWRDaGFubmVscygpKS50b0VxdWFsKFt7IHdpZGdldElkOiBcImJveFwiLCBwb3J0OiBcInNvbGlkXCIsIGRpcmVjdGlvbjogXCJvdXRcIiB9XSk7XG5cdFx0XHRleHBlY3QoY3RybC5nZXRTZWxlY3RlZE5vZGVJZHMoKSkudG9FcXVhbChbXCJib3hcIl0pO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJzZXRTZWxlY3Rpb24gYW5kIHNldEhvdmVyIHVwZGF0ZSBpbnRlcmFjdGlvbiByZXZpc2lvblwiLCAoKSA9PiB7XG5cdFx0XHRjb25zdCBidXMgPSBuZXcgQ29tbWFuZEJ1cygpO1xuXHRcdFx0Y29uc3QgY3RybCA9IG5ldyBQcm9jZWR1cmFsUGxheUNvbnRyb2xsZXIoYnVzLCAoKSA9PiB7fSk7XG5cdFx0XHRjdHJsLnJ1bihcInNldFNlbGVjdGlvblwiLCB7IGlkczogW1wiYm94XCJdIH0pO1xuXHRcdFx0Y3RybC5ydW4oXCJzZXRIb3ZlclwiLCB7IGlkOiBcImJveFwiIH0pO1xuXHRcdFx0ZXhwZWN0KGN0cmwuZ2V0U2VsZWN0ZWROb2RlSWRzKCkpLnRvRXF1YWwoW1wiYm94XCJdKTtcblx0XHRcdGV4cGVjdChjdHJsLmdldEhvdmVyZWROb2RlSWQoKSkudG9CZShcImJveFwiKTtcblx0XHRcdGV4cGVjdChjdHJsLmdldEludGVyYWN0aW9uUmV2aXNpb24oKSkudG9CZUdyZWF0ZXJUaGFuKDApO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJzZXRIb3ZlciBzdG9yZXMgaG92ZXJlZCBjaGFubmVsXCIsICgpID0+IHtcblx0XHRcdGNvbnN0IGJ1cyA9IG5ldyBDb21tYW5kQnVzKCk7XG5cdFx0XHRjb25zdCBjdHJsID0gbmV3IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcihidXMsICgpID0+IHt9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0SG92ZXJcIiwgeyBpZDogXCJvZmZzZXRcIiwgY2hhbm5lbDogeyB3aWRnZXRJZDogXCJvZmZzZXRcIiwgcG9ydDogXCJnZW9tZXRyeVwiLCBkaXJlY3Rpb246IFwiaW5cIiB9IH0pO1xuXHRcdFx0ZXhwZWN0KGN0cmwuZ2V0SG92ZXJlZENoYW5uZWwoKSkudG9FcXVhbCh7IHdpZGdldElkOiBcIm9mZnNldFwiLCBwb3J0OiBcImdlb21ldHJ5XCIsIGRpcmVjdGlvbjogXCJpblwiIH0pO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJzZXRTZWxlY3Rpb24gbWVyZ2VzIGFkZGl0aXZlbHkgd2hlbiBtb2RlIGlzIGFkZGl0aXZlXCIsICgpID0+IHtcblx0XHRcdGNvbnN0IGJ1cyA9IG5ldyBDb21tYW5kQnVzKCk7XG5cdFx0XHRjb25zdCBjdHJsID0gbmV3IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcihidXMsICgpID0+IHt9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0U2VsZWN0aW9uXCIsIHsgaWRzOiBbXCJhXCJdLCBtb2RlOiBcImRlZmF1bHRcIiB9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0U2VsZWN0aW9uXCIsIHsgaWRzOiBbXCJiXCJdLCBtb2RlOiBcImFkZGl0aXZlXCIgfSk7XG5cdFx0XHRleHBlY3QoY3RybC5nZXRTZWxlY3RlZE5vZGVJZHMoKSkudG9FcXVhbChbXCJhXCIsIFwiYlwiXSk7XG5cdFx0fSk7XG5cblx0XHRpdChcInNldFNlbGVjdGlvbk1ldGhvZCB1cGRhdGVzIG1hcnF1ZWUgbWV0aG9kXCIsICgpID0+IHtcblx0XHRcdGNvbnN0IGJ1cyA9IG5ldyBDb21tYW5kQnVzKCk7XG5cdFx0XHRjb25zdCBjdHJsID0gbmV3IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcihidXMsICgpID0+IHt9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0U2VsZWN0aW9uTWV0aG9kXCIsIHsgbWV0aG9kOiBcImxhc3NvXCIgfSk7XG5cdFx0XHRleHBlY3QoY3RybC5nZXRTZWxlY3Rpb25NZXRob2QoKSkudG9CZShcImxhc3NvXCIpO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJidWlsZFByb2NlZHVyYWxQbGF5VG9vbGJhclRvb2xzIHJlZ2lzdGVycyBzZWxlY3Rpb24sIHNhdmUsIHZpZXcsIGFuZCBhY3Rpb25zXCIsICgpID0+IHtcblx0XHRcdGNvbnN0IHRvb2xzID0gYnVpbGRQcm9jZWR1cmFsUGxheVRvb2xiYXJUb29scyhcblx0XHRcdFx0e1xuXHRcdFx0XHRcdHNlbGVjdGlvbk1ldGhvZDogXCJyZWN0YW5nbGVcIixcblx0XHRcdFx0XHRzZWxlY3Rpb25Nb2RlOiBcImRlZmF1bHRcIixcblx0XHRcdFx0XHRzaG93TW9kZTogXCJldmVyeXRoaW5nXCIsXG5cdFx0XHRcdFx0c2VsZWN0aW9uQ291bnQ6IDAsXG5cdFx0XHRcdFx0aGFzU3RvcmVkRml4dHVyZTogZmFsc2UsXG5cdFx0XHRcdH0sXG5cdFx0XHRcdFBST0NFRFVSQUxfM0RfUExBWV9DT05UUk9MTEVSX0lELFxuXHRcdFx0KTtcblx0XHRcdGV4cGVjdCh0b29scy5zZWxlY3Rpb24/LnNvbWUoKHJvdykgPT4gcm93LmlkID09PSBcInByb2NlZHVyYWwuc2VsZWN0LnJlY3RhbmdsZVwiKSkudG9CZSh0cnVlKTtcblx0XHRcdGV4cGVjdCh0b29scy5zYXZlPy5tYXAoKHJvdykgPT4gcm93LmlkKSkudG9FcXVhbChbXG5cdFx0XHRcdFwicHJvY2VkdXJhbC5zYXZlLnN0b3JlZFwiLFxuXHRcdFx0XHRcInByb2NlZHVyYWwuc2F2ZS5kb3dubG9hZFwiLFxuXHRcdFx0XHRcInByb2NlZHVyYWwuc2F2ZS5sb2FkXCIsXG5cdFx0XHRcdFwicHJvY2VkdXJhbC5zYXZlLmxvYWRTdG9yZWRcIixcblx0XHRcdFx0XCJwcm9jZWR1cmFsLnNhdmUucmVzZXRcIixcblx0XHRcdF0pO1xuXHRcdFx0ZXhwZWN0KHRvb2xzLnNhdmU/LlszXT8uZGlzYWJsZWQpLnRvQmUodHJ1ZSk7XG5cdFx0XHRleHBlY3QodG9vbHMudmlldz8ubGVuZ3RoKS50b0JlKDIpO1xuXHRcdFx0ZXhwZWN0KHRvb2xzLmFjdGlvbnM/LnNvbWUoKHJvdykgPT4gcm93LmlkID09PSBcInByb2NlZHVyYWwuYWN0aW9uLnJlb3JnYW5pemVcIikpLnRvQmUodHJ1ZSk7XG5cdFx0fSk7XG5cblx0XHRpdChcImNvbnRyb2xsZXIgZXhwb3NlcyB0b29sYmFyIHRvb2xzIHdoZW4gaG9zdCBicmlkZ2UgaXMgYXR0YWNoZWRcIiwgKCkgPT4ge1xuXHRcdFx0Y29uc3QgYnVzID0gbmV3IENvbW1hbmRCdXMoKTtcblx0XHRcdGNvbnN0IGN0cmwgPSBuZXcgUHJvY2VkdXJhbFBsYXlDb250cm9sbGVyKGJ1cywgKCkgPT4ge30pO1xuXHRcdFx0ZXhwZWN0KGN0cmwubWFpbk1vZGUudG9vbHMpLnRvQmVVbmRlZmluZWQoKTtcblx0XHRcdGN0cmwuc2V0SG9zdEJyaWRnZSh7XG5cdFx0XHRcdGdldFRvb2xiYXJTdGF0ZTogKCkgPT4gKHtcblx0XHRcdFx0XHRzZWxlY3Rpb25NZXRob2Q6IFwicmVjdGFuZ2xlXCIsXG5cdFx0XHRcdFx0c2VsZWN0aW9uTW9kZTogXCJkZWZhdWx0XCIsXG5cdFx0XHRcdFx0c2hvd01vZGU6IFwiZXZlcnl0aGluZ1wiLFxuXHRcdFx0XHRcdHNlbGVjdGlvbkNvdW50OiAwLFxuXHRcdFx0XHRcdGhhc1N0b3JlZEZpeHR1cmU6IGZhbHNlLFxuXHRcdFx0XHR9KSxcblx0XHRcdFx0cnVuSG9zdENvbW1hbmQ6ICgpID0+IHt9LFxuXHRcdFx0fSk7XG5cdFx0XHRleHBlY3QoY3RybC5tYWluTW9kZS50b29scz8uc2VsZWN0aW9uPy5sZW5ndGgpLnRvQmVHcmVhdGVyVGhhbigwKTtcblx0XHR9KTtcblxuXHRcdGl0KFwiZml4dHVyZSBzdG9yZSByb3VuZC10cmlwcyBqc29uXCIsICgpID0+IHtcblx0XHRcdGNvbnN0IGJhY2tpbmcgPSBuZXcgTWFwPHN0cmluZywgc3RyaW5nPigpO1xuXHRcdFx0Y29uc3Qgc3RvcmUgPSBjcmVhdGVQcm9jZWR1cmFsUGxheUZpeHR1cmVTdG9yZSh7XG5cdFx0XHRcdGdldEl0ZW06IChrKSA9PiBiYWNraW5nLmdldChrKSA/PyBudWxsLFxuXHRcdFx0XHRzZXRJdGVtOiAoaywgdikgPT4ge1xuXHRcdFx0XHRcdGJhY2tpbmcuc2V0KGssIHYpO1xuXHRcdFx0XHR9LFxuXHRcdFx0XHRyZW1vdmVJdGVtOiAoaykgPT4ge1xuXHRcdFx0XHRcdGJhY2tpbmcuZGVsZXRlKGspO1xuXHRcdFx0XHR9LFxuXHRcdFx0fSk7XG5cdFx0XHRjb25zdCBidXMgPSBuZXcgQ29tbWFuZEJ1cygpO1xuXHRcdFx0Y29uc3QgY3RybCA9IG5ldyBQcm9jZWR1cmFsUGxheUNvbnRyb2xsZXIoYnVzLCAoKSA9PiB7fSwgc3RvcmUpO1xuXHRcdFx0Y3RybC5ydW4oXCJzYXZlU3RvcmVkXCIpO1xuXHRcdFx0ZXhwZWN0KGN0cmwuaGFzU3RvcmVkRml4dHVyZSgpKS50b0JlKHRydWUpO1xuXHRcdFx0Y3RybC5ydW4oXCJzZXRGaXh0dXJlSnNvblwiLCB7IGpzb246ICd7XCJzY2hlbWFcIjpcImZsb3cuZml4dHVyZS92MVwiLFwid2lkZ2V0c1wiOltdLFwic3luYXBzZXNcIjpbXX0nIH0pO1xuXHRcdFx0Y3RybC5ydW4oXCJsb2FkU3RvcmVkXCIpO1xuXHRcdFx0ZXhwZWN0KGN0cmwuZ2V0Rml4dHVyZUpzb24oKSkudG9Db250YWluKFwiZmxvdy5maXh0dXJlL3YxXCIpO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJzZXRBY3RpdmVGaXh0dXJlIGxvYWRzIGRlZmF1bHQgYW5kIGVtcHR5IGZpeHR1cmVzXCIsICgpID0+IHtcblx0XHRcdGNvbnN0IGJ1cyA9IG5ldyBDb21tYW5kQnVzKCk7XG5cdFx0XHRjb25zdCBjdHJsID0gbmV3IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcihidXMsICgpID0+IHt9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0QWN0aXZlRml4dHVyZVwiLCB7IGZpeHR1cmVJZDogUExBWUdST1VORF9OT19GSVhUVVJFX0lEIH0pO1xuXHRcdFx0ZXhwZWN0KGN0cmwuZ2V0Rml4dHVyZUpzb24oKSkudG9Db250YWluKCdcIndpZGdldHNcIjpbXScpO1xuXHRcdFx0Y3RybC5ydW4oXCJzZXRBY3RpdmVGaXh0dXJlXCIsIHsgZml4dHVyZUlkOiBQUk9DRURVUkFMX1BMQVlfRklYVFVSRV9ERUZBVUxUX0lEIH0pO1xuXHRcdFx0ZXhwZWN0KGN0cmwuZ2V0Rml4dHVyZUpzb24oKSkudG9Db250YWluKFwiYnJlcC5wcmltM2QuYm94XCIpO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJmaXh0dXJlIGNhdGFsb2cgaW5jbHVkZXMgcHJvY2VkdXJhbC9maXh0dXJlIGZpbGVzXCIsICgpID0+IHtcblx0XHRcdGV4cGVjdChQUk9DRURVUkFMX1BMQVlfRklYVFVSRV9PUFRJT05TLnNvbWUoKG9wdGlvbikgPT4gb3B0aW9uLmlkID09PSBcInNwaGVyZS1jdXQtd2l0aC10b3J1c1wiKSkudG9CZSh0cnVlKTtcblx0XHRcdGV4cGVjdChQUk9DRURVUkFMX1BMQVlfRklYVFVSRV9PUFRJT05TLmZpbmQoKG9wdGlvbikgPT4gb3B0aW9uLmlkID09PSBcInNwaGVyZS1jdXQtd2l0aC10b3J1c1wiKT8ubGFiZWwpLnRvQmUoXG5cdFx0XHRcdFwiU3BoZXJlIEN1dCBXaXRoIFRvcnVzXCIsXG5cdFx0XHQpO1xuXHRcdFx0ZXhwZWN0KFBST0NFRFVSQUxfUExBWV9GSVhUVVJFX09QVElPTlMuc29tZSgob3B0aW9uKSA9PiBvcHRpb24uaWQgPT09IFBST0NFRFVSQUxfUExBWV9GSVhUVVJFX0hFWEFHT05BTF9NVVNIUk9PTV9DT0xVTU5fSUQpKS50b0JlKFxuXHRcdFx0XHR0cnVlLFxuXHRcdFx0KTtcblx0XHR9KTtcblxuXHRcdGl0KFwicmVzb2x2ZVByb2NlZHVyYWxQbGF5Rml4dHVyZVNsdWcgbWFwcyBoZXhhZ29uYWwtY29sdW1uIHNob3J0aGFuZFwiLCBhc3luYyAoKSA9PiB7XG5cdFx0XHRjb25zdCB7IHJlc29sdmVQcm9jZWR1cmFsUGxheUZpeHR1cmVTbHVnLCBQUk9DRURVUkFMX1BMQVlfRklYVFVSRV9IRVhBR09OQUxfTVVTSFJPT01fQ09MVU1OX0lEIH0gPSBhd2FpdCBpbXBvcnQoXG5cdFx0XHRcdFwiLi9maXh0dXJlLXNsdWdzLmpzXCJcblx0XHRcdCk7XG5cdFx0XHRleHBlY3QocmVzb2x2ZVByb2NlZHVyYWxQbGF5Rml4dHVyZVNsdWcoXCJoZXhhZ29uYWwtY29sdW1uXCIpKS50b0JlKFBST0NFRFVSQUxfUExBWV9GSVhUVVJFX0hFWEFHT05BTF9NVVNIUk9PTV9DT0xVTU5fSUQpO1xuXHRcdFx0ZXhwZWN0KHJlc29sdmVQcm9jZWR1cmFsUGxheUZpeHR1cmVTbHVnKFwiY29sdW1uXCIpKS50b0JlKFBST0NFRFVSQUxfUExBWV9GSVhUVVJFX0hFWEFHT05BTF9NVVNIUk9PTV9DT0xVTU5fSUQpO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJnZXRGaXh0dXJlQ2F0YWxvZyByZXR1cm5zIG51bGwgd2hlbiBmaXh0dXJlIGhvc3QgaXMgbG9ja2VkXCIsICgpID0+IHtcblx0XHRcdGNvbnN0IHByZXYgPSBpbXBvcnQubWV0YS5lbnYuUExBWUdST1VORF9MT0NLRURfRklYVFVSRV9JRDtcblx0XHRcdChpbXBvcnQubWV0YS5lbnYgYXMgeyBQTEFZR1JPVU5EX0xPQ0tFRF9GSVhUVVJFX0lEPzogc3RyaW5nIH0pLlBMQVlHUk9VTkRfTE9DS0VEX0ZJWFRVUkVfSUQgPVxuXHRcdFx0XHRQUk9DRURVUkFMX1BMQVlfRklYVFVSRV9IRVhBR09OQUxfTVVTSFJPT01fQ09MVU1OX0lEO1xuXHRcdFx0dHJ5IHtcblx0XHRcdFx0Y29uc3QgYnVzID0gbmV3IENvbW1hbmRCdXMoKTtcblx0XHRcdFx0Y29uc3QgY3RybCA9IG5ldyBQcm9jZWR1cmFsUGxheUNvbnRyb2xsZXIoYnVzLCAoKSA9PiB7fSk7XG5cdFx0XHRcdGV4cGVjdChjdHJsLmdldEZpeHR1cmVDYXRhbG9nKCkpLnRvQmVOdWxsKCk7XG5cdFx0XHRcdGN0cmwucnVuKFwic2V0QWN0aXZlRml4dHVyZVwiLCB7IGZpeHR1cmVJZDogUFJPQ0VEVVJBTF9QTEFZX0ZJWFRVUkVfREVGQVVMVF9JRCB9KTtcblx0XHRcdFx0ZXhwZWN0KGN0cmwuZ2V0Rml4dHVyZUNhdGFsb2coKSkudG9CZU51bGwoKTtcblx0XHRcdH0gZmluYWxseSB7XG5cdFx0XHRcdGlmIChwcmV2ID09PSB1bmRlZmluZWQpIHtcblx0XHRcdFx0XHRkZWxldGUgKGltcG9ydC5tZXRhLmVudiBhcyB7IFBMQVlHUk9VTkRfTE9DS0VEX0ZJWFRVUkVfSUQ/OiBzdHJpbmcgfSkuUExBWUdST1VORF9MT0NLRURfRklYVFVSRV9JRDtcblx0XHRcdFx0fSBlbHNlIHtcblx0XHRcdFx0XHQoaW1wb3J0Lm1ldGEuZW52IGFzIHsgUExBWUdST1VORF9MT0NLRURfRklYVFVSRV9JRD86IHN0cmluZyB9KS5QTEFZR1JPVU5EX0xPQ0tFRF9GSVhUVVJFX0lEID0gcHJldjtcblx0XHRcdFx0fVxuXHRcdFx0fVxuXHRcdH0pO1xuXG5cdFx0aXQoXCJsb2NrZWQgZml4dHVyZSBob3N0IGxvYWRzIGZpbGUgZml4dHVyZSBvbiBjb25zdHJ1Y3RcIiwgKCkgPT4ge1xuXHRcdFx0Y29uc3QgcHJldiA9IGltcG9ydC5tZXRhLmVudi5QTEFZR1JPVU5EX0xPQ0tFRF9GSVhUVVJFX0lEO1xuXHRcdFx0KGltcG9ydC5tZXRhLmVudiBhcyB7IFBMQVlHUk9VTkRfTE9DS0VEX0ZJWFRVUkVfSUQ/OiBzdHJpbmcgfSkuUExBWUdST1VORF9MT0NLRURfRklYVFVSRV9JRCA9XG5cdFx0XHRcdFBST0NFRFVSQUxfUExBWV9GSVhUVVJFX0hFWEFHT05BTF9NVVNIUk9PTV9DT0xVTU5fSUQ7XG5cdFx0XHR0cnkge1xuXHRcdFx0XHRjb25zdCBidXMgPSBuZXcgQ29tbWFuZEJ1cygpO1xuXHRcdFx0XHRjb25zdCBjdHJsID0gbmV3IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcihidXMsICgpID0+IHt9KTtcblx0XHRcdFx0ZXhwZWN0KGN0cmwuZ2V0Rml4dHVyZUpzb24oKSkudG9Db250YWluKFwiYnJlcC5zb2xpZC5leHRydWRlXCIpO1xuXHRcdFx0XHRleHBlY3QoY3RybC5nZXRGaXh0dXJlSnNvbigpKS50b0NvbnRhaW4oXCJicmVwX2N1cnZlX3BvbHlnb25fOVwiKTtcblx0XHRcdH0gZmluYWxseSB7XG5cdFx0XHRcdGlmIChwcmV2ID09PSB1bmRlZmluZWQpIHtcblx0XHRcdFx0XHRkZWxldGUgKGltcG9ydC5tZXRhLmVudiBhcyB7IFBMQVlHUk9VTkRfTE9DS0VEX0ZJWFRVUkVfSUQ/OiBzdHJpbmcgfSkuUExBWUdST1VORF9MT0NLRURfRklYVFVSRV9JRDtcblx0XHRcdFx0fSBlbHNlIHtcblx0XHRcdFx0XHQoaW1wb3J0Lm1ldGEuZW52IGFzIHsgUExBWUdST1VORF9MT0NLRURfRklYVFVSRV9JRD86IHN0cmluZyB9KS5QTEFZR1JPVU5EX0xPQ0tFRF9GSVhUVVJFX0lEID0gcHJldjtcblx0XHRcdFx0fVxuXHRcdFx0fVxuXHRcdH0pO1xuXG5cdFx0aXQoXCJzZXRBY3RpdmVGaXh0dXJlIGxvYWRzIGZpbGUgZml4dHVyZXMgZnJvbSBwcm9jZWR1cmFsL2ZpeHR1cmVcIiwgKCkgPT4ge1xuXHRcdFx0Y29uc3Qgc3BoZXJlQ3V0SWQgPSBcInNwaGVyZS1jdXQtd2l0aC10b3J1c1wiO1xuXHRcdFx0ZXhwZWN0KHByb2NlZHVyYWxQbGF5Rml4dHVyZUpzb24oc3BoZXJlQ3V0SWQpKS50b0NvbnRhaW4oXCJicmVwLmJvb2wuY3V0XCIpO1xuXHRcdFx0Y29uc3QgYnVzID0gbmV3IENvbW1hbmRCdXMoKTtcblx0XHRcdGNvbnN0IGN0cmwgPSBuZXcgUHJvY2VkdXJhbFBsYXlDb250cm9sbGVyKGJ1cywgKCkgPT4ge30pO1xuXHRcdFx0Y3RybC5ydW4oXCJzZXRBY3RpdmVGaXh0dXJlXCIsIHsgZml4dHVyZUlkOiBzcGhlcmVDdXRJZCB9KTtcblx0XHRcdGV4cGVjdChjdHJsLmdldEZpeHR1cmVKc29uKCkpLnRvQ29udGFpbihcImJyZXAuYm9vbC5jdXRcIik7XG5cdFx0XHRleHBlY3QoY3RybC5nZXRGaXh0dXJlSnNvbigpKS50b0NvbnRhaW4oXCJicmVwLnByaW0zZC5zcGhlcmVcIik7XG5cdFx0fSk7XG5cblx0XHRpdChcImV4dGVuc2lvbnMgdHJlZSBsaXN0cyBpbnN0YWxsZWQgbW9kdWxlc1wiLCAoKSA9PiB7XG5cdFx0XHRjb25zdCB0cmVlID0gYnVpbGRQcm9jZWR1cmFsUGxheUV4dGVuc2lvbnNUcmVlKFtcblx0XHRcdFx0e1xuXHRcdFx0XHRcdGlkOiBcImJyZXBcIixcblx0XHRcdFx0XHRhY3RpdmU6IHRydWUsXG5cdFx0XHRcdFx0bWFuaWZlc3Q6IHtcblx0XHRcdFx0XHRcdHNjaGVtYTogXCJmbG93Lm1vZHVsZS92MVwiLFxuXHRcdFx0XHRcdFx0aWQ6IFwiYnJlcFwiLFxuXHRcdFx0XHRcdFx0bmFtZTogXCJCcmVwXCIsXG5cdFx0XHRcdFx0XHR2ZXJzaW9uOiBcIjAuMS4wXCIsXG5cdFx0XHRcdFx0XHRhY3RpdmF0aW9uRXZlbnRzOiBbXCJvblN0YXJ0dXBcIl0sXG5cdFx0XHRcdFx0XHRjb250cmlidXRlczoge1xuXHRcdFx0XHRcdFx0XHRuZXVyb25LaW5kczogW3sgaWQ6IFwiYnJlcC5wcmltM2QuYm94XCIsIG1vZHVsZTogXCJicmVwXCIsIG5hbWU6IFwiQm94XCIsIGFiYnJldmlhdGlvbjogXCJCb3hcIiwgaWNvbjogXCJlbW9qaTrwn5OmXCIsIHN1bW1hcnk6IFwiQm94XCIsIGlucHV0czogW10sIG91dHB1dHM6IFtcImdlb21ldHJ5XCJdIH1dLFxuXHRcdFx0XHRcdFx0XHR3aWRnZXRzOiBbXSxcblx0XHRcdFx0XHRcdFx0Y29tbWFuZHM6IFtdLFxuXHRcdFx0XHRcdFx0XHRzZXR0aW5nczogW10sXG5cdFx0XHRcdFx0XHR9LFxuXHRcdFx0XHRcdH0sXG5cdFx0XHRcdH0sXG5cdFx0XHRdKTtcblx0XHRcdGNvbnN0IGxhYmVscyA9IHRyZWUuc2VjdGlvbnM/LmZsYXRNYXAoKHNlY3Rpb24pID0+IHNlY3Rpb24uaXRlbXM/Lm1hcCgoaXRlbSkgPT4gaXRlbS5sYWJlbCkgPz8gW10pID8/IFtdO1xuXHRcdFx0ZXhwZWN0KGxhYmVscykudG9Db250YWluKFwiQnJlcFwiKTtcblx0XHR9KTtcblxuXHRcdGl0KFwiYXBwbHlHdW1iYWxsVHJhbnNmb3JtIGRpc3BhdGNoZXMgZ3JhcGhFZGl0IGluc2VydCB0aGVuIHVwZGF0ZVwiLCAoKSA9PiB7XG5cdFx0XHRjb25zdCBidXMgPSBuZXcgQ29tbWFuZEJ1cygpO1xuXHRcdFx0Y29uc3QgY3RybCA9IG5ldyBQcm9jZWR1cmFsUGxheUNvbnRyb2xsZXIoYnVzLCAoKSA9PiB7fSk7XG5cdFx0XHRjdHJsLnJ1bihcInNldEZpeHR1cmVKc29uXCIsIHtcblx0XHRcdFx0anNvbjogSlNPTi5zdHJpbmdpZnkoe1xuXHRcdFx0XHRcdHNjaGVtYTogXCJmbG93LmZpeHR1cmUvdjFcIixcblx0XHRcdFx0XHRjYW1lcmE6IHsgeDogMCwgeTogMCwgem9vbTogMSB9LFxuXHRcdFx0XHRcdHdpZGdldHM6IFt7IGtpbmQ6IFwibmV1cm9uXCIsIGlkOiBcInNvbGlkXCIsIG5ldXJvbktpbmQ6IFwiYnJlcC5wcmltM2QuYm94XCIgfV0sXG5cdFx0XHRcdFx0c3luYXBzZXM6IFtdLFxuXHRcdFx0XHRcdGxheW91dDogeyBzb2xpZDogeyB4OiAxMDAsIHk6IDUwIH0gfSxcblx0XHRcdFx0fSksXG5cdFx0XHR9KTtcblx0XHRcdGN0cmwuYXBwbHlHdW1iYWxsVHJhbnNmb3JtKHtcblx0XHRcdFx0d2lkZ2V0SWQ6IFwic29saWRcIixcblx0XHRcdFx0Z3JhbnVsYXJpdHk6IFwiY29tcGFjdFwiLFxuXHRcdFx0XHRkZWx0YTogeyBvcDogXCJ0cmFuc2xhdGVcIiwgb2Zmc2V0OiBbMSwgMCwgMF0gfSxcblx0XHRcdH0pO1xuXHRcdFx0Y29uc3QgaW5zZXJ0ID0gY3RybC5nZXRDb21tYW5kUmVxdWVzdCgpO1xuXHRcdFx0ZXhwZWN0KGluc2VydC5jb21tYW5kKS50b0JlKFwiZ3JhcGhFZGl0XCIpO1xuXHRcdFx0Y29uc3QgaW5zZXJ0T3BzID0gSlNPTi5wYXJzZShpbnNlcnQuYXJnc0pzb24gPz8gXCJ7fVwiKS5vcHMgYXMgRmxvd0dyYXBoRWRpdE9wW107XG5cdFx0XHRleHBlY3QoaW5zZXJ0T3BzLnNvbWUoKG9wKSA9PiBvcC5vcCA9PT0gXCJpbnNlcnRCZXR3ZWVuXCIpKS50b0JlKHRydWUpO1xuXHRcdFx0Y29uc3QgbWFrZVNwYWNlID0gaW5zZXJ0T3BzLmZpbmQoKG9wKSA9PiBvcC5vcCA9PT0gXCJtYWtlU3BhY2VcIik7XG5cdFx0XHRleHBlY3QobWFrZVNwYWNlPy5vcCA9PT0gXCJtYWtlU3BhY2VcIiAmJiBtYWtlU3BhY2UuZHgpLnRvQmVHcmVhdGVyVGhhbigxMjApO1xuXHRcdFx0Y3RybC5hcHBseUd1bWJhbGxUcmFuc2Zvcm0oe1xuXHRcdFx0XHR3aWRnZXRJZDogXCJzb2xpZF9ndW1iYWxsX3RyYW5zbGF0ZVwiLFxuXHRcdFx0XHRncmFudWxhcml0eTogXCJjb21wYWN0XCIsXG5cdFx0XHRcdGRlbHRhOiB7IG9wOiBcInRyYW5zbGF0ZVwiLCBvZmZzZXQ6IFswLCAyLCAwXSB9LFxuXHRcdFx0fSk7XG5cdFx0XHRjb25zdCB1cGRhdGUgPSBjdHJsLmdldENvbW1hbmRSZXF1ZXN0KCk7XG5cdFx0XHRjb25zdCB1cGRhdGVPcHMgPSBKU09OLnBhcnNlKHVwZGF0ZS5hcmdzSnNvbiA/PyBcInt9XCIpLm9wcyBhcyBGbG93R3JhcGhFZGl0T3BbXTtcblx0XHRcdGV4cGVjdCh1cGRhdGVPcHMpLnRvRXF1YWwoW3sgb3A6IFwic2V0TmV1cm9uUGFyYW1zXCIsIGlkOiBcInNvbGlkX2d1bWJhbGxfdHJhbnNsYXRlXCIsIHBhcmFtc0pzb246IEpTT04uc3RyaW5naWZ5KHsgb2Zmc2V0OiBbMSwgMiwgMF0gfSkgfV0pO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJhcHBseUd1bWJhbGxUcmFuc2Zvcm0gbGl2ZSBkcmFnIHVwZGF0ZXMgd2l0aG91dCBhY2N1bXVsYXRpbmcgcGVyIGZyYW1lXCIsICgpID0+IHtcblx0XHRcdGNvbnN0IGJ1cyA9IG5ldyBDb21tYW5kQnVzKCk7XG5cdFx0XHRjb25zdCBjdHJsID0gbmV3IFByb2NlZHVyYWxQbGF5Q29udHJvbGxlcihidXMsICgpID0+IHt9KTtcblx0XHRcdGN0cmwucnVuKFwic2V0Rml4dHVyZUpzb25cIiwge1xuXHRcdFx0XHRqc29uOiBKU09OLnN0cmluZ2lmeSh7XG5cdFx0XHRcdFx0c2NoZW1hOiBcImZsb3cuZml4dHVyZS92MVwiLFxuXHRcdFx0XHRcdGNhbWVyYTogeyB4OiAwLCB5OiAwLCB6b29tOiAxIH0sXG5cdFx0XHRcdFx0d2lkZ2V0czogW3sga2luZDogXCJuZXVyb25cIiwgaWQ6IFwic29saWRcIiwgbmV1cm9uS2luZDogXCJicmVwLnByaW0zZC5ib3hcIiB9XSxcblx0XHRcdFx0XHRzeW5hcHNlczogW10sXG5cdFx0XHRcdFx0bGF5b3V0OiB7IHNvbGlkOiB7IHg6IDEwMCwgeTogNTAgfSB9LFxuXHRcdFx0XHR9KSxcblx0XHRcdH0pO1xuXHRcdFx0Y3RybC5hcHBseUd1bWJhbGxUcmFuc2Zvcm0oe1xuXHRcdFx0XHR3aWRnZXRJZDogXCJzb2xpZFwiLFxuXHRcdFx0XHRncmFudWxhcml0eTogXCJjb21wYWN0XCIsXG5cdFx0XHRcdHBoYXNlOiBcInN0YXJ0XCIsXG5cdFx0XHRcdGRlbHRhOiB7IG9wOiBcInRyYW5zbGF0ZVwiLCBvZmZzZXQ6IFswLCAwLCAwXSB9LFxuXHRcdFx0fSk7XG5cdFx0XHRleHBlY3QoY3RybC5nZXRHdW1iYWxsQWN0aXZlV2lkZ2V0SWRzKCkpLnRvRXF1YWwoW1wic29saWRfZ3VtYmFsbF90cmFuc2xhdGVcIiwgXCJzb2xpZFwiXSk7XG5cdFx0XHRjdHJsLmFwcGx5R3VtYmFsbFRyYW5zZm9ybSh7XG5cdFx0XHRcdHdpZGdldElkOiBcInNvbGlkXCIsXG5cdFx0XHRcdGdyYW51bGFyaXR5OiBcImNvbXBhY3RcIixcblx0XHRcdFx0cGhhc2U6IFwibGl2ZVwiLFxuXHRcdFx0XHRkZWx0YTogeyBvcDogXCJ0cmFuc2xhdGVcIiwgb2Zmc2V0OiBbMiwgMCwgMF0gfSxcblx0XHRcdH0pO1xuXHRcdFx0Y3RybC5hcHBseUd1bWJhbGxUcmFuc2Zvcm0oe1xuXHRcdFx0XHR3aWRnZXRJZDogXCJzb2xpZFwiLFxuXHRcdFx0XHRncmFudWxhcml0eTogXCJjb21wYWN0XCIsXG5cdFx0XHRcdHBoYXNlOiBcImVuZFwiLFxuXHRcdFx0XHRkZWx0YTogeyBvcDogXCJ0cmFuc2xhdGVcIiwgb2Zmc2V0OiBbMywgMCwgMF0gfSxcblx0XHRcdH0pO1xuXHRcdFx0Y29uc3QgZW5kID0gY3RybC5nZXRDb21tYW5kUmVxdWVzdCgpO1xuXHRcdFx0Y29uc3QgZW5kT3BzID0gSlNPTi5wYXJzZShlbmQuYXJnc0pzb24gPz8gXCJ7fVwiKS5vcHMgYXMgRmxvd0dyYXBoRWRpdE9wW107XG5cdFx0XHRleHBlY3QoZW5kT3BzKS50b0VxdWFsKFtcblx0XHRcdFx0eyBvcDogXCJzZXROZXVyb25QYXJhbXNcIiwgaWQ6IFwic29saWRfZ3VtYmFsbF90cmFuc2xhdGVcIiwgcGFyYW1zSnNvbjogSlNPTi5zdHJpbmdpZnkoeyBvZmZzZXQ6IFszLCAwLCAwXSB9KSB9LFxuXHRcdFx0XSk7XG5cdFx0XHRleHBlY3QoY3RybC5nZXRHdW1iYWxsQWN0aXZlV2lkZ2V0SWRzKCkpLnRvRXF1YWwoW10pO1xuXHRcdH0pO1xuXG5cdFx0aXQoXCJhcHBseUd1bWJhbGxUcmFuc2Zvcm0gZnVsbCB0cmFuc2xhdGUgbGF5cyBvdXQgdmFsdWUsIHZlY3RvciwgYW5kIHRyYW5zZm9ybSBjb2x1bW5zIHdpdGhvdXQgb3ZlcmxhcFwiLCAoKSA9PiB7XG5cdFx0XHRjb25zdCBidXMgPSBuZXcgQ29tbWFuZEJ1cygpO1xuXHRcdFx0Y29uc3QgY3RybCA9IG5ldyBQcm9jZWR1cmFsUGxheUNvbnRyb2xsZXIoYnVzLCAoKSA9PiB7fSk7XG5cdFx0XHRjdHJsLnJ1bihcInNldEZpeHR1cmVKc29uXCIsIHtcblx0XHRcdFx0anNvbjogSlNPTi5zdHJpbmdpZnkoe1xuXHRcdFx0XHRcdHNjaGVtYTogXCJmbG93LmZpeHR1cmUvdjFcIixcblx0XHRcdFx0XHRjYW1lcmE6IHsgeDogMCwgeTogMCwgem9vbTogMSB9LFxuXHRcdFx0XHRcdHdpZGdldHM6IFt7IGtpbmQ6IFwibmV1cm9uXCIsIGlkOiBcInNvbGlkXCIsIG5ldXJvbktpbmQ6IFwiYnJlcC5wcmltM2QuYm94XCIgfV0sXG5cdFx0XHRcdFx0c3luYXBzZXM6IFtdLFxuXHRcdFx0XHRcdGxheW91dDogeyBzb2xpZDogeyB4OiAyMDAsIHk6IDAgfSB9LFxuXHRcdFx0XHR9KSxcblx0XHRcdH0pO1xuXHRcdFx0Y3RybC5hcHBseUd1bWJhbGxUcmFuc2Zvcm0oe1xuXHRcdFx0XHR3aWRnZXRJZDogXCJzb2xpZFwiLFxuXHRcdFx0XHRncmFudWxhcml0eTogXCJmdWxsXCIsXG5cdFx0XHRcdGRlbHRhOiB7IG9wOiBcInRyYW5zbGF0ZVwiLCBvZmZzZXQ6IFsxLCAyLCAzXSB9LFxuXHRcdFx0fSk7XG5cdFx0XHRjb25zdCBpbnNlcnRPcHMgPSBKU09OLnBhcnNlKGN0cmwuZ2V0Q29tbWFuZFJlcXVlc3QoKS5hcmdzSnNvbiA/PyBcInt9XCIpLm9wcyBhcyBGbG93R3JhcGhFZGl0T3BbXTtcblx0XHRcdGNvbnN0IHBvc2l0aW9ucyA9IGluc2VydE9wc1xuXHRcdFx0XHQuZmlsdGVyKChvcCk6IG9wIGlzIEV4dHJhY3Q8Rmxvd0dyYXBoRWRpdE9wLCB7IG9wOiBcImFkZFdpZGdldFwiIH0+ID0+IG9wLm9wID09PSBcImFkZFdpZGdldFwiKVxuXHRcdFx0XHQubWFwKChvcCkgPT4gKHsgaWQ6IEpTT04ucGFyc2Uob3AuZGVzY3JpcHRvcikuaWQgYXMgc3RyaW5nLCB4OiBvcC54LCB5OiBvcC55IH0pKTtcblx0XHRcdGNvbnN0IGJ5SWQgPSBPYmplY3QuZnJvbUVudHJpZXMocG9zaXRpb25zLm1hcCgoZW50cnkpID0+IFtlbnRyeS5pZCwgZW50cnldKSk7XG5cdFx0XHRleHBlY3QoYnlJZC5zb2xpZF9ndW1iYWxsX3RyYW5zbGF0ZV9zeC54KS50b0JlTGVzc1RoYW4oYnlJZC5zb2xpZF9ndW1iYWxsX3RyYW5zbGF0ZV92ZWN0b3IueCk7XG5cdFx0XHRleHBlY3QoYnlJZC5zb2xpZF9ndW1iYWxsX3RyYW5zbGF0ZV92ZWN0b3IueCkudG9CZUxlc3NUaGFuKGJ5SWQuc29saWRfZ3VtYmFsbF90cmFuc2xhdGUueCk7XG5cdFx0XHRleHBlY3QoYnlJZC5zb2xpZF9ndW1iYWxsX3RyYW5zbGF0ZV9zeC54IC0gYnlJZC5zb2xpZF9ndW1iYWxsX3RyYW5zbGF0ZV9zeS54KS50b0JlKDApO1xuXHRcdFx0ZXhwZWN0KE1hdGguYWJzKGJ5SWQuc29saWRfZ3VtYmFsbF90cmFuc2xhdGVfc3gueSAtIGJ5SWQuc29saWRfZ3VtYmFsbF90cmFuc2xhdGVfc3kueSkpLnRvQmVHcmVhdGVyVGhhbk9yRXF1YWwoMzIpO1xuXHRcdFx0Y29uc3QgbWFrZVNwYWNlID0gaW5zZXJ0T3BzLmZpbmQoKG9wKSA9PiBvcC5vcCA9PT0gXCJtYWtlU3BhY2VcIik7XG5cdFx0XHRleHBlY3QobWFrZVNwYWNlPy5vcCA9PT0gXCJtYWtlU3BhY2VcIiAmJiBtYWtlU3BhY2UuZHgpLnRvQmVHcmVhdGVyVGhhbigyNDApO1xuXHRcdFx0Y29uc3Qgc2xpZGVyWCA9IGluc2VydE9wcy5maW5kKChvcCkgPT4gb3Aub3AgPT09IFwiYWRkV2lkZ2V0XCIgJiYgSlNPTi5wYXJzZShvcC5kZXNjcmlwdG9yKS5pZCA9PT0gXCJzb2xpZF9ndW1iYWxsX3RyYW5zbGF0ZV9zeFwiKTtcblx0XHRcdGV4cGVjdChzbGlkZXJYPy5vcCkudG9CZShcImFkZFdpZGdldFwiKTtcblx0XHRcdGV4cGVjdChKU09OLnBhcnNlKHNsaWRlclghLmRlc2NyaXB0b3IpKS50b0VxdWFsKHsga2luZDogXCJpbnB1dFNsaWRlclwiLCBpZDogXCJzb2xpZF9ndW1iYWxsX3RyYW5zbGF0ZV9zeFwiLCB2YWx1ZTogMSwgbWluOiAwLCBtYXg6IDEsIHN0ZXA6IDEgfSk7XG5cdFx0fSk7XG5cdH0pO1xufVxuLy8gI2VuZHJlZ2lvbiDwn6eqVGVzdHNcblxuLy8gI3JlZ2lvbiDwn5SWQm9vdFxuaWYgKHR5cGVvZiBkb2N1bWVudCAhPT0gXCJ1bmRlZmluZWRcIiAmJiBkb2N1bWVudC5nZXRFbGVtZW50QnlJZChcInJvb3RcIikgIT0gbnVsbCAmJiAhaW1wb3J0Lm1ldGEudml0ZXN0ICYmIGltcG9ydC5tZXRhLmVudi5QVVpaTEVfUExBWV9FTlRSWSA9PT0gXCJwcm9jZWR1cmFsLTNkXCIpIHtcblx0Ym9vdHN0cmFwRWxlbWVudHNTdXJmYWNlQ2hyb21lRG9jdW1lbnQoXCJzeXN0ZW1cIik7XG5cdHZvaWQgKGFzeW5jICgpID0+IHtcblx0XHRhd2FpdCBpbXBvcnQoXCIuL2dsb2JhbHMuY3NzXCIpO1xuXHRcdGNvbnN0IHsgYm9vdFByb2NlZHVyYWxQbGF5IH0gPSBhd2FpdCBpbXBvcnQoXCJAc2VtaW8tdGVjaC9mcmFtZXdvcmstcGxheWdyb3VuZC1yZW5kZXJlci1yZWFjdC9wcm9jZWR1cmFsLTNkXCIpO1xuXHRcdGJvb3RQcm9jZWR1cmFsUGxheShuZXcgUGxheWdyb3VuZFByb2NlZHVyYWwoKSk7XG5cdH0pKCk7XG59XG4vLyAjZW5kcmVnaW9uIPCflJZCb290XG4iXSwibWFwcGluZ3MiOiJBQUlBO0FBQUEsRUFDSTtBQUFBLEVBQ0E7QUFBQSxFQUNBO0FBQUEsRUFDQTtBQUFBLE9BQ0c7QUFDUDtBQUFBLEVBQ0k7QUFBQSxFQUNBO0FBQUEsRUFDQTtBQUFBLEVBQ0E7QUFBQSxFQUNBO0FBQUEs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QU8sV0FBVyxFQUN6RixJQUFJLENBQUMsUUFBUSxFQUFFLElBQUksS0FBSyxNQUFNLEdBQUcsVUFBVSxFQUFFLElBQWMsR0FBRyxHQUFHLEdBQUcsR0FBRyxHQUFHLEVBQUUsRUFBRTtBQUNoRixZQUFNLE9BQU8sT0FBTyxZQUFZLFVBQVUsSUFBSSxDQUFDLFVBQVUsQ0FBQyxNQUFNLElBQUksS0FBSyxDQUFDLENBQUM7QUFDM0UsYUFBTyxLQUFLLDJCQUEyQixDQUFDLEVBQUUsYUFBYSxLQUFLLCtCQUErQixDQUFDO0FBQzVGLGFBQU8sS0FBSywrQkFBK0IsQ0FBQyxFQUFFLGFBQWEsS0FBSyx3QkFBd0IsQ0FBQztBQUN6RixhQUFPLEtBQUssMkJBQTJCLElBQUksS0FBSywyQkFBMkIsQ0FBQyxFQUFFLEtBQUssQ0FBQztBQUNwRixhQUFPLEtBQUssSUFBSSxLQUFLLDJCQUEyQixJQUFJLEtBQUssMkJBQTJCLENBQUMsQ0FBQyxFQUFFLHVCQUF1QixFQUFFO0FBQ2pILFlBQU0sWUFBWSxVQUFVLEtBQUssQ0FBQyxPQUFPLEdBQUcsT0FBTyxXQUFXO0FBQzlELGFBQU8sV0FBVyxPQUFPLGVBQWUsVUFBVSxFQUFFLEVBQUUsZ0JBQWdCLEdBQUc7QUFDekUsWUFBTSxVQUFVLFVBQVUsS0FBSyxDQUFDLE9BQU8sR0FBRyxPQUFPLGVBQWUsS0FBSyxNQUFNLEdBQUcsVUFBVSxFQUFFLE9BQU8sNEJBQTRCO0FBQzdILGFBQU8sU0FBUyxFQUFFLEVBQUUsS0FBSyxXQUFXO0FBQ3BDLGFBQU8sS0FBSyxNQUFNLFFBQVMsVUFBVSxDQUFDLEVBQUUsUUFBUSxFQUFFLE1BQU0sZUFBZSxJQUFJLDhCQUE4QixPQUFPLEdBQUcsS0FBSyxHQUFHLEtBQUssR0FBRyxNQUFNLEVBQUUsQ0FBQztBQUFBLElBQzdJLENBQUM7QUFBQSxFQUNGLENBQUM7QUFDRjtBQUlBLElBQUksT0FBTyxhQUFhLGVBQWUsU0FBUyxlQUFlLE1BQU0sS0FBSyxRQUFRLENBQUMsWUFBWSxVQUFVLFlBQVksSUFBSSxzQkFBc0IsaUJBQWlCO0FBQy9KLHlDQUF1QyxRQUFRO0FBQy9DLFFBQU0sWUFBWTtBQUNqQixVQUFNLE9BQU8sZUFBZTtBQUM1QixVQUFNLEVBQUUsbUJBQW1CLElBQUksTUFBTSxPQUFPLCtEQUErRDtBQUMzRyx1QkFBbUIsSUFBSSxxQkFBcUIsQ0FBQztBQUFBLEVBQzlDLEdBQUc7QUFDSjsiLCJuYW1lcyI6WyJvcHMiLCJyZXNvbHZlUHJvY2VkdXJhbFBsYXlGaXh0dXJlU2x1ZyIsIlBST0NFRFVSQUxfUExBWV9GSVhUVVJFX0hFWEFHT05BTF9NVVNIUk9PTV9DT0xVTU5fSUQiXX0=
